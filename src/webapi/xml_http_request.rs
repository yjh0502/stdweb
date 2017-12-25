use webapi::event_target::{IEventTarget, EventTarget};
use webcore::unsafe_typed_array::UnsafeTypedArray;
use webcore::value::{
    Reference,
    Value,
};
use webcore::try_from::TryInto;

/// Use XMLHttpRequest (XHR) objects to interact with servers.
/// You can retrieve data from a URL without having to do a full page refresh.
/// This enables a Web page to update just part of a page without disrupting
/// what the user is doing. XMLHttpRequest is used heavily in Ajax programming.
/// 
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest)
pub struct XMLHttpRequest( Reference );

reference_boilerplate! {
    XMLHttpRequest,
    instanceof XMLHttpRequest
    convertible to EventTarget
}

/// An enum indicating the state of the `XMLHttpRequest`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ReadyState {
    /// Client has been created. [open()](struct.XMLHttpRequest.html#method.open) not called yet.
    Unsent,
    /// [open()](struct.XMLHttpRequest.html#method.open) has been called.
    Opened,
    /// [send()](struct.XMLHttpRequest.html#method.send) has been called, and headers and [status()](struct.XMLHttpRequest.html#method.status) are available.
    HeadersReceived,
    /// Downloading; [reponse_text()](struct.XMLHttpRequest.html#method.reponse_text) holds partial data.
    Loading,
    /// The operation is complete.
    Done,
}

impl IEventTarget for XMLHttpRequest {}


impl XMLHttpRequest {
    /// Creates new `XMLHttpRequest`.
    pub fn new() -> XMLHttpRequest {
        js!( return new XMLHttpRequest(); ).try_into().unwrap()
    }

    /// Returns the current state of the request as a [ReadyState](enum.ReadyState.html). 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)
    pub fn ready_state(&self) -> ReadyState {
        use self::ReadyState::*;
        let state: u16 = js!( return @{self}.readyState; ).try_into().unwrap();
        match state {
            0 => Unsent,
            1 => Opened,
            2 => HeadersReceived,
            3 => Loading,
            4 => Done,
            _ => unreachable!( "Unexpected value of XMLHttpRequest::readyState: {}", state )
        }
    }

    /// Returns a string that contains the response to the request as text, or None
    /// if the request was unsuccessful or has not yet been sent.
    ///
    ///[(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseText)
    pub fn response_text(&self) -> Option<String> {
        let response = js!(return @{self}.responseText;);
        match response {
            Value::Null => None,
            Value::String(resp) => Some(resp),
            _ => unreachable!(),
        }
    }

    /// Returns an unsigned short with the status of the response of the request.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/status)
    pub fn status(&self) -> u16 {
        js!(return @{self}.status;).try_into().unwrap()
    }

    /// Open connection with given method (ie GET or POST), and the url to hit
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)
    pub fn open(&self, method: &str, url: &str) {
        js! {
            @{self}.open(@{method}, @{url}, true);
        };
    }

    /// Send request on an open connection with no data
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    pub fn send(&self) {
        js! {
            @{self}.send();
        };
    }

    /// Send request on an open connection with string body
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    pub fn send_with_string(&self, body: &str) {
        js! {
            @{self}.send(@{body});
        };
    }

    /// Send request on an open connection with a byte array body
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    pub fn send_with_bytes(&self, body: &[u8]) {
        js! {
            @{self}.send(@{UnsafeTypedArray(body)});
        };
    }

    /// Aborts the request if it has already been sent.
    /// When a request is aborted, its [ready_state](struct.XMLHttpRequest.html#method.ready_state) is changed to [Done](enum.ReadyState.html#variant.Done)
    /// and the [status](struct.XMLHttpRequest.html#method.status) code is set to
    /// [Unsent](enum.ReadyState.html#variant.Unsent).
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/abort)
    pub fn abort(&self) {
        js! {
            @{self}.abort();
        };
    }
}