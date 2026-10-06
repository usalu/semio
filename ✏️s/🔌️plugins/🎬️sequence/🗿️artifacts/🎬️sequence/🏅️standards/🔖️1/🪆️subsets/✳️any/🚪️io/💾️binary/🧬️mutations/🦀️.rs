//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::SequenceDiff;
use crate::SequenceSnapshot;
use crate::schema::operations::*;

/// 💾️ No parent operation record exists.
impl protocol::OpBinary for SequenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Err(protocol::ProtocolError::Malformed { what: "sequence-mutation", offset: 0, detail: "a sequence has no parent-lane mutation; content edits are child-lane leaves".into() })
    }
}
}
pub use mutations_codec::*;
