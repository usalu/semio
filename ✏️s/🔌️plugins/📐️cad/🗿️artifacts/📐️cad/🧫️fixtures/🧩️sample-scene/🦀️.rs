use super::*;

/// 🧩️ A sample composed `s.stdio.semio.model` CHILD HANDLE — `child_id` + `target` only, per
/// `🔖️Composition`'s "a child handle is two strings" rule; never the resolved model content
/// (that lives in the child's own document, out of `CadSnapshot`'s reach).
pub fn sample_model_child(child_id: &str) -> CadModelChild {
    store::ArtifactChild::new(child_id.into(), store::os_io::ArtifactRef { artifact_id: child_id.to_string(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() } })
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
pub fn sample_object(id: &str, origin: [f64; 3]) -> crate::standards::v1::subsets::any::io::geometry_import::CadObject {
    crate::standards::v1::subsets::any::io::geometry_import::CadObject {
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
        primitives: vec![crate::standards::v1::subsets::any::io::geometry_import::CadPrimitiveSlot { slot: "solid".into(), primitive_id: format!("solid-{id}"), kind: "solid".into() }],
    }
}

/// 🪆️ A snapshot whose `shape_model` child carries a REAL in-process materialization — the only state
/// in which an object mutation is anything but a no-op, and (by design) a state no wire codec can
/// express: `ArtifactChild::local_owner` is skipped by every serializer, so these laws are Rust-
/// constructed rather than committed JSON quintets like the reference/node/child-slot kinds.
pub fn materialized_shape_scene(objects: Vec<crate::standards::v1::subsets::any::io::geometry_import::CadObject>) -> CadSnapshot {
    let mut scene = sample_scene();
    let working = crate::CadWorkingScene { objects: objects.clone(), ..Default::default() };
    scene.shape_model = crate::cad_pane_rematerialized_child(&working, CadPaneId::Shape, objects);
    scene
}

/// 🧱️ The objects `pane` currently materializes, for a law that has to read them back.
pub fn materialized_objects(document: &CadSnapshot, pane: CadPaneId) -> Vec<crate::standards::v1::subsets::any::io::geometry_import::CadObject> {
    crate::cad_pane_local_scene(document, pane).map(|scene| crate::cad_scene_pane_objects(&scene, pane).to_vec()).unwrap_or_default()
}
