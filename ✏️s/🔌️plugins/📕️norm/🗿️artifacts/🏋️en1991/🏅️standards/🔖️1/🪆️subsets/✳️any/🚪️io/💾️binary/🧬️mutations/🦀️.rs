//! 💾️ En1991 mutation binary — the protocol-tagged payload frame of `📡️.protocol.semio` that the text facet's OpBinary writes.

use crate::standards::v1::subsets::any::io::text::mutations::*;

//#region 📡️Protocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️Protocol
