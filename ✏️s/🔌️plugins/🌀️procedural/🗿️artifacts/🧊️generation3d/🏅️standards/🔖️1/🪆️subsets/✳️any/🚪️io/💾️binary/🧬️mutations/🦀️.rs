//! ⚖️ Generation3d artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

#[cfg(test)] use semio_s_artifact_procedural_3d_mut_document::change_schema::ChangeSchema;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_synapses::connect_synapse::ConnectSynapse;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_generations::create_generation::CreateGeneration;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_widgets::create_widget::CreateWidget;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_widgets::delete_widget::DeleteWidget;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_widgets::delete_widget_position::DeleteWidgetPosition;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_widgets::move_widget::MoveWidget;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_document::update_camera::UpdateCamera;
#[cfg(test)] use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use protocol::OpBinary;

//#region 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs


impl OpBinary for Generation3dOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(COMPONENT_PROTOCOL_SEMIO, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs







/// ⚡️ Binary mirror of the `OpText` bridge above.
impl OpBinary for Generation3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        generation3d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Generation3dOperationDsl::decode_op(bytes)?;
        generation3d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "generation3d mutation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️OpTextMirror

/// 📦️ Encodes a `Generation3dMutation` to its binary state-patch form.
pub fn encode_op(operation: &Generation3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Generation3dMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Generation3dMutation, protocol::ProtocolError> {
    Generation3dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests


use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dOperationDsl;
use semio_s_artifact_procedural_3d_io_text::mutations::generation3d_operation_to_dsl;
use semio_s_artifact_procedural_3d_io_text::mutations::generation3d_operation_from_dsl;
