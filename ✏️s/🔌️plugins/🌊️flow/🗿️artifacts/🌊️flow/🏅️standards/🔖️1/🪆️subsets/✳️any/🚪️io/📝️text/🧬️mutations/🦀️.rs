//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::diff::FlowDiff;
use crate::FlowSnapshot;

/// 📝️ No parent operation line exists.
impl protocol::OpText for FlowMutation {
    fn parse_op(_line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "a flow has no parent-lane mutation; content edits are child-lane leaves", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}
}
pub use mutations_codec::*;
