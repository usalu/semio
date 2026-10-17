//! Direct text identity for `insert-media-annotation`.

pub const OPCODE: &str = "insert-media-annotation";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::e::schema::mutations::InsertMediaAnnotation;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &InsertMediaAnnotation) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<InsertMediaAnnotation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
