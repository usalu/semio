//! 🧪️ `move-objects` law — `📍️moves-the-shape-pane-object`.
//!
//! ⚠️ No committed JSON quintet: the state this verb reads is the pane child's
//! `ArtifactChild::local_owner`, which every wire codec skips by design (see `🏪️store`'s
//! `ArtifactChild` doc: "intentionally absent from equality, debug, DSL, pack, and JSON identity").
//! A committed `before` snapshot would therefore describe a pane with no objects and the vector would
//! be a no-op — so the law is Rust-constructed instead, and the manifest records the same reason.

use crate::mutations::{move_objects::MoveObjects, CadMutation, CadObjectOrigin};
use crate::sample_scene_fixture::{materialized_objects, materialized_shape_scene, sample_object};
use crate::CadPaneId;
use protocol::{Mutation, MutationDiff};

fn base() -> crate::CadSnapshot {
    materialized_shape_scene(vec![sample_object("object-a", [0.0, 0.0, 0.0]), sample_object("object-b", [4.0, 0.0, 0.0])])
}

fn move_a() -> CadMutation {
    CadMutation::MoveObjects(MoveObjects { pane: CadPaneId::Shape, placements: vec![CadObjectOrigin { object_id: "object-a".into(), new_origin: [1.5, -2.25, 0.5] }] })
}

/// ▶️ The moved object's origin lands in the pane child's re-materialized working scene, and the
/// untouched sibling keeps its own pose.
#[semio_framework_async_macros::async_test]
async fn moves_the_addressed_object_in_the_composed_child() {
    let base = base();
    let after = move_a().diff(&base).diff().apply(&base).expect("move-objects applies");
    let objects = materialized_objects(&after, CadPaneId::Shape);
    assert_eq!(objects.iter().find(|object| object.id == "object-a").expect("object-a survives").origin, [1.5, -2.25, 0.5], "the addressed object must carry its new origin");
    assert_eq!(objects.iter().find(|object| object.id == "object-b").expect("object-b survives").origin, [4.0, 0.0, 0.0], "an unaddressed sibling must keep its pose");
}

/// 🪆️ The pane's composed child HANDLE is re-minted — that content-address change is what makes the
/// edit observable to the parent document (and therefore to history and to every renderer).
#[semio_framework_async_macros::async_test]
async fn remints_the_pane_child_handle() {
    let base = base();
    let after = move_a().diff(&base).diff().apply(&base).expect("move-objects applies");
    let before_id = base.shape_model.as_ref().expect("base shape child").child_id.clone();
    let after_id = after.shape_model.as_ref().expect("after shape child").child_id.clone();
    assert_ne!(before_id, after_id, "a moved object must content-address to a different composed child handle");
    assert_eq!(after.building_model, base.building_model, "only the edited pane's child is re-minted");
}

/// ↩️ The inverse carries the PRE-move origin and restores the exact base document.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_pre_move_pose() {
    let base = base();
    let mutation = move_a();
    let inverse = mutation.inverse(&base);
    assert_eq!(inverse.len(), 1, "move-objects inverts to exactly one step");
    match &inverse[0] {
        CadMutation::MoveObjects(step) => assert_eq!(step.placements[0].new_origin, [0.0, 0.0, 0.0], "the inverse must carry the pre-move origin"),
        other => panic!("move-objects must invert to move-objects, got {other:?}"),
    }
    let mut snapshot = mutation.diff(&base).diff().apply(&base).expect("move-objects applies");
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "move-objects inverse did not restore the before-snapshot");
    assert_eq!(materialized_objects(&snapshot, CadPaneId::Shape)[0].origin, [0.0, 0.0, 0.0], "undo must restore the materialized pose too");
}

/// 🚦️ A pane with no materialized child, and a re-declaration of the current pose, are both no-ops.
#[semio_framework_async_macros::async_test]
async fn unmaterialized_and_unchanged_poses_are_no_ops() {
    let base = base();
    let unchanged = CadMutation::MoveObjects(MoveObjects { pane: CadPaneId::Shape, placements: vec![CadObjectOrigin { object_id: "object-a".into(), new_origin: [0.0, 0.0, 0.0] }] });
    assert!(unchanged.diff(&base).messages().iter().any(|message| message.code.0 == "mutation.no-op"), "re-declaring the current origin must be a no-op");
    let elsewhere = CadMutation::MoveObjects(MoveObjects { pane: CadPaneId::Energy, placements: vec![CadObjectOrigin { object_id: "object-a".into(), new_origin: [1.0, 1.0, 1.0] }] });
    assert!(elsewhere.diff(&base).messages().iter().any(|message| message.code.0 == "mutation.no-op"), "a pane with no materialized child must be a no-op");
    assert!(elsewhere.inverse(&base).is_empty(), "a no-op move has no inverse step");
}
