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
    use crate::artifacts::cad::{empty_cad_snapshot, CadNode, CadObject, CadPaneId, CadPrimitiveSlot, CAD_DOCUMENT_SCHEMA};
    use store::{create_document_envelope, ArtifactCommand};

    #[test]
    fn encode_decode_op_round_trips_a_representative_operation() {
        let operation = CadMutation::TranslateObjects { object_ids: vec!["object-1".into()], dx: 1.0, dy: -2.0, dz: 3.5 };
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn cad_projection_defaults() {
        let store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None));
        assert_eq!(store.snapshot().expect("projection").id, "cad");
    }

    #[test]
    fn add_object_round_trips_through_store() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None));
        let object = CadObject {
            id: "object-1".into(),
            label: "Box".into(),
            typology: "spatial.shape.box".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: None,
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }],
        };
        store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::AddObject { pane: CadPaneId::Shape, object }], description: None }).expect("apply");
        let scene = store.snapshot().expect("projection");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].primitives[0].kind, "solid");
    }

    #[test]
    fn translate_objects_updates_origin() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None));
        store
            .dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::AddObject {
                    pane: CadPaneId::Shape,
                    object: CadObject {
                        id: "object-1".into(),
                        label: "Box".into(),
                        typology: "spatial.shape.box".into(),
                        visible: true,
                        locked: false,
                        origin: [1.0, 2.0, 3.0],
                        orientation: None,
                        scale: None,
                        mesh_url: None,
                        extent: None,
                        solid_handle: None,
                        primitives: Vec::new(),
                    },
                }],
                description: None,
            })
            .expect("apply");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::TranslateObjects { object_ids: vec!["object-1".into()], dx: 1.0, dy: -1.0, dz: 0.5 }], description: None }).expect("translate");
        let scene = store.snapshot().expect("projection");
        assert_eq!(scene.objects[0].origin, [2.0, 1.0, 3.5]);
    }

    #[test]
    fn set_scene_replaces_projection_and_inverts() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None));
        let mut replacement = empty_cad_snapshot();
        replacement.id = "replaced".into();
        replacement.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::SetSnapshot { snapshot: Box::new(replacement) }], description: None }).expect("set scene");
        assert_eq!(store.snapshot().expect("projection").id, "replaced");
        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("projection").id, "cad");
        assert!(store.snapshot().expect("projection").nodes.is_empty());
    }
}
//#endregion 🧪️Tests
