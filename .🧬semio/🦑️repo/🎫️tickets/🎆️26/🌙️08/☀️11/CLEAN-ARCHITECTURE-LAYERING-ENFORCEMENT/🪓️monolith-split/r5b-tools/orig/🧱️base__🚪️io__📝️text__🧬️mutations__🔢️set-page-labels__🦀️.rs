//! 🔢️ Direct text identity for `set-page-labels`.

pub const OPCODE: &str = "set-page-labels";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::base::schema::mutations::SetPageLabels;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageLabels) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageLabels, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
