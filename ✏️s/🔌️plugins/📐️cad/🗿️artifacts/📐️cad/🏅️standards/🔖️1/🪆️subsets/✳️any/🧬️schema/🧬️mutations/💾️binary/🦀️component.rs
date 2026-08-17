//! 📡️ CAD artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `CadMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! cad host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `CadMutation` to its binary command form.
pub fn encode_op(operation: &CadMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `CadMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<CadMutation, protocol::ProtocolError> {
    CadMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type CadEnvelope = ArtifactEnvelope<CadSnapshot, CadMutation>;
pub type CadStore = ArtifactStore<CadSnapshot, CadMutation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::mutations::create_shape_model::mutation::CreateShapeModel;
    use crate::artifacts::cad::{empty_cad_snapshot, testkit::sample_model_child, CAD_DOCUMENT_SCHEMA};
    use store::{create_document_envelope, ArtifactCommand};

    #[test]
    fn encode_decode_op_round_trips_a_representative_operation() {
        let sample = sample_model_child("op-round-trip-1");
        let operation = CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() });
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn cad_projection_defaults() {
        let store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None)).expect("store");
        assert_eq!(store.snapshot().expect("projection").id, "cad");
    }

    #[test]
    fn create_shape_model_round_trips_through_store() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None)).expect("store");
        let sample = sample_model_child("store-round-trip-1");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() })], description: None }).expect("apply");
        let scene = store.snapshot().expect("projection");
        assert_eq!(scene.shape_model.expect("shape_model set").child_id, sample.child_id);
    }
}
//#endregion 🧪️Tests
