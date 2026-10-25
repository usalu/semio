//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::mutations::*;
use crate::Generation3dViewCamera;
use crate::Generation3dViewConfig;

impl protocol::OpBinary for Generation3dViewConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
semio_framework_os_kernel::original_ordinal_operation_receiving!(Generation3dViewConfigMutation);
}
