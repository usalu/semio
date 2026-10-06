//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::schema::diff::WiresDiff;
use crate::WiresSnapshot;

/// 📝️ No parent operation line exists.
impl protocol::OpText for WiresMutation {
    fn parse_op(_line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "a wires board has no parent-lane mutation; board edits are child-lane graph leaves", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}
}
pub use mutations_codec::*;
