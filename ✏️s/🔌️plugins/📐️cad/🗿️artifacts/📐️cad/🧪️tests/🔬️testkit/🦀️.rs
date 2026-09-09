use super::*;

/// 🧩️ A sample composed `s.stdio.semio.model` CHILD HANDLE — `child_id` + `target` only, per
/// `🔖️Composition`'s "a child handle is two strings" rule; never the resolved model content
/// (that lives in the child's own document, out of `CadSnapshot`'s reach).
pub fn sample_model_child(child_id: &str) -> CadModelChild {
    store::ArtifactChild::new(child_id.into(), store::os_io::ArtifactRef { artifact_id: format!("crate-{child_id}"), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() } })
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
    scene.active_model_definition_id = CadPaneId::Shape.model_definition_id().to_string();
    scene
}
