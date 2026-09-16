//! 🧪️ `delete-object` law — `🚫️removes-the-shape-pane-object`.
//!
//! ⚠️ Rust-constructed rather than a committed JSON quintet — see `move-objects`'s law for why.

use crate::mutations::{delete_object::DeleteObject, CadMutation};
use crate::sample_scene_fixture::{materialized_objects, materialized_shape_scene, sample_object};
use crate::CadPaneId;
use protocol::{Mutation, MutationDiff};

fn base() -> crate::CadSnapshot {
    materialized_shape_scene(vec![sample_object("object-a", [0.0, 0.0, 0.0]), sample_object("object-b", [4.0, 0.0, 0.0]), sample_object("object-c", [8.0, 0.0, 0.0])])
}

fn delete_middle() -> CadMutation {
    CadMutation::DeleteObject(DeleteObject { pane: CadPaneId::Shape, object_id: "object-b".into() })
}

/// ▶️ The removed object leaves the pane's re-materialized working scene; its siblings keep their order.
#[semio_framework_async_macros::async_test]
async fn removes_the_addressed_object() {
    let base = base();
    let after = delete_middle().diff(&base).diff().apply(&base).expect("delete-object applies");
    let ids: Vec<String> = materialized_objects(&after, CadPaneId::Shape).into_iter().map(|object| object.id).collect();
    assert_eq!(ids, vec!["object-a".to_string(), "object-c".to_string()], "only the addressed object is removed and order is preserved");
}

/// 🕳️ Emptying a pane vacates its composed child slot rather than minting an empty child.
#[semio_framework_async_macros::async_test]
async fn deleting_the_last_object_vacates_the_slot() {
    let base = materialized_shape_scene(vec![sample_object("object-a", [0.0, 0.0, 0.0])]);
    let delete = CadMutation::DeleteObject(DeleteObject { pane: CadPaneId::Shape, object_id: "object-a".into() });
    let after = delete.diff(&base).diff().apply(&base).expect("delete-object applies");
    assert!(after.shape_model.is_none(), "an emptied pane vacates its child slot");
}

/// 🚫️ Deleting an object that does not exist is a target-missing error.
#[semio_framework_async_macros::async_test]
async fn missing_object_is_a_target_missing_error() {
    let base = base();
    store::os_spr::protocol_laws::assert_missing_target_is_error(&base, &CadMutation::DeleteObject(DeleteObject { pane: CadPaneId::Shape, object_id: "does-not-exist".into() })).await;
}

/// ↩️ The inverse recreates the object at its original slot and restores the exact base document.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_object_at_its_slot() {
    let base = base();
    let mutation = delete_middle();
    let inverse = mutation.inverse(&base);
    match &inverse[0] {
        CadMutation::CreateObject(step) => assert_eq!(step.index, 1, "the inverse must restore the removed object at its original index"),
        other => panic!("delete-object must invert to create-object, got {other:?}"),
    }
    let mut snapshot = mutation.diff(&base).diff().apply(&base).expect("delete-object applies");
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-object inverse did not restore the before-snapshot");
}
