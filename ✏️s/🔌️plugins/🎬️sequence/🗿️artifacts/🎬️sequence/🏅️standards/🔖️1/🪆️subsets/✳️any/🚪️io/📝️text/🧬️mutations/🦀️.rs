//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::SequenceDiff;
use crate::SequenceSnapshot;
use crate::schema::operations::*;

/// 📝️ No parent operation line exists.
impl protocol::OpText for SequenceMutation {
    fn parse_op(_line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "a sequence has no parent-lane mutation; content edits are child-lane leaves", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::operations::*;
use crate::schema::mutations::SequenceMutation;
use crate::SequenceSnapshot;

/// ⚖️ The SEMANTIC PROJECTION a parent document is compared through: its own fields (`schema` and the composed `content`
/// handle). The content lives in the child's store and is compared there, never read off the parent.
pub fn encode_sequence_projection_json(snapshot: &SequenceSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
}
pub use mutations_wire_codec::*;
