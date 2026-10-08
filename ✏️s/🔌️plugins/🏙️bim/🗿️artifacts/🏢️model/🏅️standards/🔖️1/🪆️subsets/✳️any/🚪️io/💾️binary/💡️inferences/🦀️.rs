//! 📡️ Inference binary protocol: inference values are computed from a snapshot and never decoded from an authored binary document,
//! so this facet declares the wire protocol only.

/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
