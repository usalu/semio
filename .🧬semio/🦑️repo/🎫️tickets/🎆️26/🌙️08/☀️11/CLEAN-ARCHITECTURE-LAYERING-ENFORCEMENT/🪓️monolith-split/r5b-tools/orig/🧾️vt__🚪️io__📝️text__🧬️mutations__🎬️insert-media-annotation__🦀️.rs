//! 🎬️ Direct text codec for `insert-media-annotation`.

use crate::standards::v1_7::subsets::vt::schema::mutations::InsertMediaAnnotation;

//#region 🔖️Identity
pub const OPCODE: &str = "insert-media-annotation";
pub const TEXT_OPCODE: &str = OPCODE;
//#endregion 🔖️Identity

//#region 🔖️Codec
/// 🖨️ Prints the owned payload as schema JSON.
pub fn print(payload: &InsertMediaAnnotation) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(payload))
}

/// 📥️ Parses the owned payload from schema JSON.
pub fn parse(text: &str) -> Result<InsertMediaAnnotation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
