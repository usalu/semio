//! 💾️ En1991 mutation binary — delegates to JSON OpBinary in text facet.

pub use crate::artifact_schema::mutations::text::*;

//#region 📡️Protocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️Protocol
