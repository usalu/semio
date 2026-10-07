//! 💾️ En1991 mutation binary — the protocol-tagged payload frame of `📡️.protocol.semio` that the text facet's OpBinary writes.

use crate::standards::v1::subsets::any::io::text::mutations::*;

//#region 📡️Protocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️Protocol

mod native_codec {
use super::*;
use crate::artifact_schema::mutations::En1991Mutation;

impl protocol::OpBinary for En1991Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::encode::<crate::En1991Snapshot, _>(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::decode::<crate::En1991Snapshot, _>(include_str!("📡️.protocol.semio"), bytes)
    }
}
}
