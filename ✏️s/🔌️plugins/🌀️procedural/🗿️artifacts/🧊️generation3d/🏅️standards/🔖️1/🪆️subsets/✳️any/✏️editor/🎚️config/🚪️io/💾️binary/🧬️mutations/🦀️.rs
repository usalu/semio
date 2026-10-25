//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::mutations::*;
use crate::CameraJson;
use crate::Generation3dConfig;
use crate::Generation3dPreviewCamera;

impl protocol::OpBinary for Generation3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
semio_framework_os_kernel::original_ordinal_operation_receiving!(Generation3dConfigMutation);
}
