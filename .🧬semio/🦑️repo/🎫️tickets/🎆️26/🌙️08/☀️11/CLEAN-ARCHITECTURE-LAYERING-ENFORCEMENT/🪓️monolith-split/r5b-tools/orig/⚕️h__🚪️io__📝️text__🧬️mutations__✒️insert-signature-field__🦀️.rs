//! Direct text identity for `insert-signature-field`.

pub const OPCODE: &str = "insert-signature-field";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::h::schema::mutations::InsertSignatureField;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertSignatureField) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertSignatureField, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
