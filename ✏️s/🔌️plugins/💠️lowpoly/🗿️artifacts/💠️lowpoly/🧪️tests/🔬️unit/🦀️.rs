
use super::*;

#[semio_framework_async_macros::async_test]
async fn object_patch_apply_mutates_and_inverse_restores_all_fields() {
    let mesh_workspace = "{}".to_string();
    let original_mesh = mesh_child_handle("obj-1", &mesh_workspace);
    let mut object = LowpolyObject { id: "obj-1".into(), name: "Original".into(), transform: LowpolyTransform::default(), smooth_shading: false, mesh: Some(original_mesh), paint_layers: vec![LowpolyPaintLayer::new("Base")] };
    let original = object.clone();
    let new_mesh_workspace = "{\"changed\":true}".to_string();
    let new_mesh = mesh_child_handle("obj-1", &new_mesh_workspace);
    let patch = LowpolyObjectPatch { name: Some("Renamed".into()), smooth_shading: Some(true), transform: Some(LowpolyTransform { position: [1.0, 2.0, 3.0], ..LowpolyTransform::default() }), mesh: Some(Some(new_mesh.clone())) };
    object.apply_patch(&patch);
    assert_eq!(object.name, "Renamed");
    assert!(object.smooth_shading);
    assert_eq!(object.transform.position, [1.0, 2.0, 3.0]);
    assert_eq!(object.mesh, Some(new_mesh));
    let inverse = object.diff_patch(&original).expect("patch changed state");
    object.apply_patch(&inverse);
    assert_eq!(object, original);
}

#[semio_framework_async_macros::async_test]
async fn snapshot_from_mesh_json_builds_single_object_with_base_layer() {
    let mesh_json = "{}".to_string();
    let snapshot = snapshot_from_mesh_json(&mesh_json, "obj-42", "Widget");
    assert_eq!(snapshot.schema, LOWPOLY_DOCUMENT_SCHEMA);
    assert_eq!(snapshot.objects.len(), 1);
    assert_eq!(snapshot.objects[0].id, "obj-42");
    assert_eq!(snapshot.objects[0].name, "Widget");
    assert_eq!(snapshot.objects[0].mesh, Some(mesh_child_handle("obj-42", &mesh_json)));
    assert_eq!(snapshot.objects[0].paint_layers.len(), 1);
    assert_eq!(snapshot.objects[0].paint_layers[0].name, "Base");
}

#[semio_framework_async_macros::async_test]
async fn lowpoly_selection_defaults_target_whole_mesh() {
    let targets = LowpolySelectionTargets::default();
    assert!(targets.mesh);
    assert!(!targets.vertex && !targets.edge && !targets.face);
    let selection = LowpolySelection::default();
    assert_eq!(selection.mode, "mesh");
    assert!(selection.ids.is_empty());
}
