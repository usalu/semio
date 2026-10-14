
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_cad_cad_model::{snapshot::empty_cad_snapshot, snapshot::CadModelChild, snapshot::CadNode, snapshot::CadPaneId, snapshot::CadReference, snapshot::CadSnapshot};

/// 🧩️ A sample composed `s.stdio.semio.model` CHILD HANDLE — `child_id` + `target` only, per
/// `🔖️Composition`'s "a child handle is two strings" rule; never the resolved model content
/// (that lives in the child's own document, out of `CadSnapshot`'s reach).
pub fn sample_model_child(child_id: &str) -> CadModelChild {
    store::os_store::artifact_child::ArtifactChild::new(child_id.into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.to_string(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() } })
}

pub fn sample_reference() -> CadReference {
    CadReference { id: "ref-1".into(), source_url: "https://example.test/plan.png".into(), media_kind: "image".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(1.5), width_world: 8.0, hidden: false, locked: true, opacity: Some(0.8) }
}

pub fn sample_scene() -> CadSnapshot {
    let mut scene = empty_cad_snapshot();
    scene.shape_model = Some(sample_model_child("shape-model-1"));
    scene.building_model = Some(sample_model_child("building-model-1"));
    scene.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
    scene.references_by_model_definition_id.insert(CadPaneId::Shape.model_definition_id().to_string(), vec![sample_reference()]);
    scene
}

/// 🧱️ A minimal ephemeral working object — the shape every object-mutation law starts from.
pub fn sample_object(id: &str, origin: [f64; 3]) -> semio_s_artifact_cad_cad_model::geometry::CadObject {
    semio_s_artifact_cad_cad_model::geometry::CadObject {
        id: id.into(),
        label: format!("Object {id}"),
        typology: "spatial.shape.primitive.box".into(),
        visible: true,
        locked: false,
        origin,
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: Some([1.0, 1.0, 1.0]),
        mesh_url: None,
        extent: Some([1.0, 1.0, 1.0]),
        solid_handle: Some(format!("solid-{id}")),
        primitives: vec![semio_s_artifact_cad_cad_model::geometry::CadPrimitiveSlot { slot: "solid".into(), primitive_id: format!("solid-{id}"), kind: "solid".into() }],
    }
}
