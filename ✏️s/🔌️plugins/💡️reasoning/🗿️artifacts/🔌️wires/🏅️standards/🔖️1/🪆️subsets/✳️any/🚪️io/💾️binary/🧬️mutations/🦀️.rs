//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::schema::diff::WiresDiff;
use crate::WiresSnapshot;

/// 💾️ No parent operation record exists.
impl protocol::OpBinary for WiresMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Err(protocol::ProtocolError::Malformed { what: "wires-mutation", offset: 0, detail: "a wires board has no parent-lane mutation; board edits are child-lane graph leaves".into() })
    }
}
}
pub use mutations_codec::*;
