//! 💾️ Native value transport for owned Equation field edits.

use crate::EquationDiff;
use semio_framework_value::{FromValue,ToValue};

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

impl protocol::DiffBinary for EquationDiff {
    fn encode_diff(&self) -> Result<Vec<u8>,protocol::ProtocolError> { Ok(protocol::pack_rt::encode_wire_value(&self.to_value())) }
    fn decode_diff(bytes: &[u8]) -> Result<Self,protocol::ProtocolError> {
        let malformed = |detail:String| protocol::ProtocolError::Malformed { what:"Equation diff",offset:0,detail };
        let value = protocol::pack_rt::decode_wire_value(bytes).map_err(|error|malformed(error.to_string()))?;
        Self::from_value(value).map_err(|error|malformed(error.to_string()))
    }
}
