//! 📝️ insert-page text payload owner.

use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
use crate::standards::v1_4::subsets::base::io::text::mutations::{hex, unhex};
use crate::standards::v1_4::subsets::base::schema::mutations::InsertPage;

//#region 🔖️Codec
pub const OPCODE: &str = "insert-page";

pub fn print(mutation: &PdfMutation) -> Option<String> {
    let PdfMutation::InsertPage(payload) = mutation else {
        return None;
    };
    Some(hex(&semio_framework_pack_json::to_json_string(payload).into_bytes()))
}

pub fn parse(payload: &str) -> Result<PdfMutation, semio_framework_diagnostic::TextError> {
    let bytes = unhex(payload).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    <InsertPage as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map(PdfMutation::InsertPage).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
//#endregion 🔖️Codec
