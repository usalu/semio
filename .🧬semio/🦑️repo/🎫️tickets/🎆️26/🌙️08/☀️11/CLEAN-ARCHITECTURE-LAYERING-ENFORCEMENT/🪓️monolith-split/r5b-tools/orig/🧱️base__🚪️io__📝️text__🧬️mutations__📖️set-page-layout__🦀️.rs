//! 📖️ Direct text identity for `set-page-layout`.

pub const OPCODE: &str = "set-page-layout";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::base::schema::mutations::SetPageLayout;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &SetPageLayout) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<SetPageLayout, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
