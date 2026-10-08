//! 📦️ Binary representation of the model diff: the framework wire value of its `ToValue` tree.

/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::ModelDiff;

impl protocol::DiffBinary for ModelDiff {
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "diff record", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error| protocol::ProtocolError::Malformed { what: "diff record", offset: 0, detail: error.to_string() })
    }
}
