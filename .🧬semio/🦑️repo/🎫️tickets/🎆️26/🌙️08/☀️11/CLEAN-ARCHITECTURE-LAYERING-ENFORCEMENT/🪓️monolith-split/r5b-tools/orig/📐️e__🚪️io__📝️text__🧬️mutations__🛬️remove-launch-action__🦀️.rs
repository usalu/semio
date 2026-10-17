//! Direct text identity for `remove-launch-action`.

pub const OPCODE: &str = "remove-launch-action";
pub const TEXT_OPCODE: &str = OPCODE;

use crate::standards::v1_7::subsets::e::schema::mutations::RemoveLaunchAction;

/// 🖨️ Prints this direct payload through its schema-derived JSON representation.
pub fn print(payload: &RemoveLaunchAction) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses this direct payload through its schema-derived JSON representation.
pub fn parse(text: &str) -> Result<RemoveLaunchAction, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
