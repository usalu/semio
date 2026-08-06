//! 📡️ CAD artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `CadOperation`'s binary wire form, plus the `DocumentEnvelope`/`DocumentStore` aliases every
//! cad host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

use crate::artifacts::cad::op::CadOperation;
use crate::artifacts::cad::CadProjection;
use protocol::OpBinary;
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `CadOperation` to its binary command form.
pub fn encode_op(operation: &CadOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `CadOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<CadOperation, protocol::ProtocolError> {
    CadOperation::decode_op(bytes)
}

//#region 🔖️Store
pub type CadEnvelope = DocumentEnvelope<CadProjection, CadOperation>;
pub type CadStore = DocumentStore<CadProjection, CadOperation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::{empty_cad_projection, CadNode, CadObject, CadPaneId, CadPrimitiveSlot, CAD_DOCUMENT_SCHEMA};
    use store::{create_document_envelope, DocumentCommand};

    #[test]
    fn encode_decode_op_round_trips_a_representative_operation() {
        let operation = CadOperation::TranslateObjects { object_ids: vec!["object-1".into()], dx: 1.0, dy: -2.0, dz: 3.5 };
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn cad_projection_defaults() {
        let store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        assert_eq!(store.projection().expect("projection").id, "cad");
    }

    #[test]
    fn add_object_round_trips_through_store() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
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
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::AddObject { pane: CadPaneId::Shape, object }], description: None }).expect("apply");
        let scene = store.projection().expect("projection");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].primitives[0].kind, "solid");
    }

    #[test]
    fn translate_objects_updates_origin() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![CadOperation::AddObject {
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
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::TranslateObjects { object_ids: vec!["object-1".into()], dx: 1.0, dy: -1.0, dz: 0.5 }], description: None }).expect("translate");
        let scene = store.projection().expect("projection");
        assert_eq!(scene.objects[0].origin, [2.0, 1.0, 3.5]);
    }

    #[test]
    fn set_scene_replaces_projection_and_inverts() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        let mut replacement = empty_cad_projection();
        replacement.id = "replaced".into();
        replacement.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::SetScene { scene: Box::new(replacement) }], description: None }).expect("set scene");
        assert_eq!(store.projection().expect("projection").id, "replaced");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").id, "cad");
        assert!(store.projection().expect("projection").nodes.is_empty());
    }
}
//#endregion 🧪️Tests
