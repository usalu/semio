//! 🧪️ `create-object` law — `🌱️appends-a-box-to-the-shape-pane`.
//!
//! ⚠️ Rust-constructed rather than a committed JSON quintet — see `move-objects`'s law for why the
//! pane child's local materialization cannot appear on the wire.

use crate::mutations::{cad_object_primitives_of, cad_object_spec_of, create_object::CreateObject, CadMutation};
use crate::sample_scene_fixture::{materialized_objects, materialized_shape_scene, sample_object};
use crate::CadPaneId;
use protocol::{Mutation, MutationDiff};

fn base() -> crate::CadSnapshot {
    materialized_shape_scene(vec![sample_object("object-a", [0.0, 0.0, 0.0])])
}

fn create_b(index: u32) -> CadMutation {
    CadMutation::CreateObject(CreateObject { pane: CadPaneId::Shape, index, object: cad_object_spec_of(&sample_object("object-b", [2.0, 0.0, 0.0])), primitives: cad_object_primitives_of(&sample_object("object-b", [2.0, 0.0, 0.0])) })
}

/// ▶️ A created object becomes a real rendered instance of the pane — it is in the re-materialized
/// working scene the world-3d builder reads.
#[semio_framework_async_macros::async_test]
async fn creates_a_new_rendered_instance() {
    let base = base();
    let after = create_b(1).diff(&base).diff().apply(&base).expect("create-object applies");
    let objects = materialized_objects(&after, CadPaneId::Shape);
    assert_eq!(objects.len(), 2, "the pane must materialize one more object");
    assert_eq!(objects[1].id, "object-b", "the new object lands at its declared index");
    assert_eq!(objects[1].origin, [2.0, 0.0, 0.0], "the created object keeps its authored placement");
    assert_eq!(objects[1].primitives.len(), 1, "the primitive slot list is rebuilt from the solid handle");
}

/// 🕳️ A pane with no materialized child yet still admits its first object.
#[semio_framework_async_macros::async_test]
async fn first_object_materializes_an_empty_pane() {
    let base = crate::sample_scene_fixture::sample_scene();
    let create = CadMutation::CreateObject(CreateObject { pane: CadPaneId::Energy, index: 0, object: cad_object_spec_of(&sample_object("object-e", [0.0, 0.0, 0.0])), primitives: Vec::new() });
    let after = create.diff(&base).diff().apply(&base).expect("create-object applies to an empty pane");
    assert_eq!(materialized_objects(&after, CadPaneId::Energy).len(), 1, "the first object mints the pane's composed child");
}

/// 🚫️ A duplicate id never applies.
#[semio_framework_async_macros::async_test]
async fn duplicate_id_never_applies() {
    let base = base();
    let duplicate = CadMutation::CreateObject(CreateObject { pane: CadPaneId::Shape, index: 0, object: cad_object_spec_of(&sample_object("object-a", [9.0, 9.0, 9.0])), primitives: Vec::new() });
    store::os_spr::protocol_laws::assert_fatal_never_applies(&duplicate.diff(&base)).await;
}

/// ↩️ The inverse deletes the created id and restores the exact base document.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_created_object() {
    let base = base();
    let mutation = create_b(1);
    let mut snapshot = mutation.diff(&base).diff().apply(&base).expect("create-object applies");
    for step in mutation.inverse(&base) {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-object inverse did not restore the before-snapshot");
}
