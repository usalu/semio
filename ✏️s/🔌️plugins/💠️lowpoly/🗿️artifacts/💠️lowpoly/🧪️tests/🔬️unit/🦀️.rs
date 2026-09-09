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

#[semio_framework_async_macros::async_test]
async fn artifact_schema_descriptor_leaves_parse_and_field_states_match_snapshot_json() {
    use framework_schema::{parse_state_class_kebab, ArtifactSchemaFields};
    let descriptor = schema::lowpoly_artifact_schema_descriptor();
    assert_eq!(descriptor.id, "s.lowpoly.lowpoly");
    let schema: dsl::os_pack::json::Value = dsl::os_pack::json::from_json_str(descriptor.snapshot.json_schema).expect("snapshot json");
    assert_eq!(schema["title"], "LowpolySnapshot");
    let properties = schema["properties"].as_object().expect("properties");
    let mut json_states: Vec<(String, _)> = properties
        .iter()
        .map(|(name, prop)| {
            let raw = prop["x-semio-state"].as_str().expect("state");
            (name.to_string(), parse_state_class_kebab(raw).expect("parse"))
        })
        .collect();
    json_states.sort_by(|a, b| a.0.cmp(&b.0));
    let mut derived: Vec<(String, _)> = LowpolySnapshot::field_states().await.iter().map(|(n, c)| ((*n).to_string(), *c)).collect();
    derived.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(derived, json_states);
    assert_eq!(schema::LowpolyArtifact::artifact_schema_id().await, "s.lowpoly.lowpoly");
}
