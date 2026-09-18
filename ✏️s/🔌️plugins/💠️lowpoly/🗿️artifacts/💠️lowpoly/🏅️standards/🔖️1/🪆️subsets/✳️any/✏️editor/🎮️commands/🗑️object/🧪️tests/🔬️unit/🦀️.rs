use super::*;
use crate::editor::lowpoly::unit_tests::context::{act, app, dispatch, dispatch_refused, select, select_face};
use crate::editor::lowpoly::LowpolyCommand;
use semio_framework_3d::mesh::HalfedgeMesh;

fn faces(object: &LowpolyObject) -> usize {
    HalfedgeMesh::from_json(&object.mesh_content).expect("mesh content parses").face_count()
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
    dispatch(&mut a, LowpolyCommand::AddPrimitive(crate::editor::lowpoly::commands::add_primitive::AddPrimitive { kind: Some("box".into()) })).await;
    let snapshot = a.snapshot().expect("projection");
    assert_eq!(snapshot.objects.len(), 2);
    let added = snapshot.objects[1].id.clone();
    select(&mut a, &[("object", &crate::editor::lowpoly::view::document_object_row_id(&added))]).await;
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
    let source = a.snapshot().expect("projection").objects[0].clone();
    dispatch(&mut a, LowpolyCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: None })).await;
    let snapshot = a.snapshot().expect("projection");
    assert_eq!(snapshot.objects.len(), 2);
    let copy = &snapshot.objects[1];
    assert_ne!(copy.id, source.id);
    assert_eq!(copy.name, format!("{} copy", source.name));
    assert_eq!(faces(copy), faces(&source));
    assert_eq!(copy.mesh_content, source.mesh_content);
    assert_eq!(copy.transform.position[0], source.transform.position[0] + 1.0);
    assert_eq!(copy.mesh, Some(crate::mesh_child_handle(&copy.id, &copy.mesh_content)), "the copy's handle is its own");
}
