extern crate semio_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use semio_s_artifact_lowpoly_lowpoly_commands::object::*;
use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation, SetActiveObjectEdit};
use semio_s_artifact_lowpoly_lowpoly_view::{editor::lowpoly::session::mesh_edit, editor::lowpoly::session::LowpolyScratch};
use semio_s_artifact_lowpoly_lowpoly_view::resolve_active_object_id;
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::{snapshot::LowpolyObject, snapshot::LowpolySnapshot};
use semio_framework_3d::mesh::{EdgeId, FaceId, VertexId};
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::context::{act, app, dispatch, dispatch_refused, select, select_face};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;

fn paint_json(object: &LowpolyObject) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&object.paint_layers))).unwrap()
}

fn faces(object: &LowpolyObject) -> usize {
    object.mesh_state.clone().expect("managed mesh state").into_mesh().expect("valid managed mesh").face_count()
}

/// 🗑️ Delete at face granularity removes exactly the picked face and undo brings it back.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_picked_face_and_undo_restores_it() {
    let mut a = app().await;
    let before = a.snapshot().expect("projection").objects[0].clone();
    select_face(&mut a, &before.id, 0).await;
    dispatch(&mut a, LowpolyCommand::DeleteSelection(delete_selection::DeleteSelection {})).await;
    let after = a.snapshot().expect("projection").objects[0].clone();
    assert_eq!(faces(&after), faces(&before) - 1, "one face gone");
    assert_ne!(after.mesh, before.mesh);
    act(&mut a, "undo", serde_json::json!({})).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].mesh, before.mesh, "undo restores the face");
}

/// 🗑️ Delete at object granularity removes the selected object, and never the last one.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_a_selected_object_but_keeps_the_last() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::AddPrimitive(semio_s_artifact_lowpoly_lowpoly_commands::add_primitive::AddPrimitive { kind: Some("box".into()) })).await;
    let snapshot = a.snapshot().expect("projection");
    assert_eq!(snapshot.objects.len(), 2);
    let added = snapshot.objects[1].id.clone();
    select(&mut a, &[("object", &semio_s_artifact_lowpoly_lowpoly_view::document_object_row_id(&added))]).await;
    dispatch(&mut a, LowpolyCommand::DeleteSelection(delete_selection::DeleteSelection {})).await;
    let snapshot = a.snapshot().expect("projection");
    assert_eq!(snapshot.objects.len(), 1, "the selected object is gone");
    assert_ne!(snapshot.objects[0].id, added);
    let refusal = dispatch_refused(&mut a, LowpolyCommand::DeleteSelection(delete_selection::DeleteSelection {})).await;
    assert!(refusal.contains("at least one object"), "the last object is refused loudly: {refusal}");
    assert_eq!(a.snapshot().expect("projection").objects.len(), 1, "the last object survives");
}

/// 🧬️ Duplicate copies the active object with its geometry beside it and makes the copy active.
#[semio_framework_async_macros::async_test]
async fn duplicate_object_copies_geometry_beside_the_source() {
    let mut a = app().await;
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧬️managed-duplicate/🔣️.json")).unwrap();
    dispatch(&mut a, LowpolyCommand::AddPrimitive(semio_s_artifact_lowpoly_lowpoly_commands::add_primitive::AddPrimitive { kind: Some("box".into()) })).await;
    let source = a.snapshot().expect("projection").objects[1].clone();
    assert_eq!(source.mesh_content, contract["precondition"]["literalMeshContent"].as_str().unwrap());
    assert!(source.mesh_state.is_some());
    dispatch(&mut a, LowpolyCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: Some(source.id.clone()) })).await;
    let snapshot = a.snapshot().expect("projection");
    assert_eq!(snapshot.objects.len(), contract["expected"]["objectCount"].as_u64().unwrap() as usize);
    let copy = &snapshot.objects[2];
    assert_ne!(copy.id, source.id);
    assert_eq!(copy.name, format!("{} copy", source.name));
    assert_eq!(faces(copy), faces(&source));
    assert_eq!(copy.mesh_content, source.mesh_content);
    assert_eq!(copy.mesh_state, source.mesh_state);
    assert!(copy.mesh_content.is_empty(), "managed geometry is independent of literal source");
    for axis in 0..3 {
        assert_eq!(copy.transform.position[axis], source.transform.position[axis] + contract["expected"]["positionOffset"][axis].as_f64().unwrap() as f32);
    }
    assert_eq!(paint_json(copy), paint_json(&source));
    assert_eq!(copy.mesh, Some(semio_s_artifact_lowpoly_lowpoly_model::snapshot::managed_mesh_child_handle(&copy.id, copy.mesh_state.as_ref().unwrap())), "the copy's handle is its own");
    let copied = copy.clone();
    act(&mut a, "undo", serde_json::json!({})).await;
    assert_eq!(a.snapshot().unwrap().objects.len(), contract["expected"]["undoObjectCount"].as_u64().unwrap() as usize);
    act(&mut a, "redo", serde_json::json!({})).await;
    let restored = a.snapshot().unwrap();
    assert_eq!(restored.objects.len(), contract["expected"]["redoObjectCount"].as_u64().unwrap() as usize);
    assert_eq!(restored.objects[2], copied);
}
