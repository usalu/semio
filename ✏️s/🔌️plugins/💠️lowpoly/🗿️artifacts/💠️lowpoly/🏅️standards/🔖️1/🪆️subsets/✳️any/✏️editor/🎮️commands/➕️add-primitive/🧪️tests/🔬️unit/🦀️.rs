use super::*;
use crate::editor::lowpoly::testkit::{app, dispatch};
use crate::editor::lowpoly::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn add_primitive_emits_objects_add_operation() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::AddPrimitive(AddPrimitive { kind: Some("box".into()) })).await;
    let projection = a.snapshot().expect("projection");
    assert_eq!(projection.objects.len(), 2);
    assert!(projection.objects.iter().any(|object| object.name == "box"));
}

#[semio_framework_async_macros::async_test]
async fn add_primitive_supports_every_known_kind() {
    let mut a = app().await;
    for kind in ["plane", "cylinder", "cone", "ico_sphere"] {
        dispatch(&mut a, LowpolyCommand::AddPrimitive(AddPrimitive { kind: Some(kind.into()) })).await;
    }
    assert_eq!(a.snapshot().expect("projection").objects.len(), 5);
}

/// 🐛 ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS: `handle` above reaches
/// `session::build_doc` (a read) AND `ctx.set_mesh_workspace_map` (a write) of the session-local
/// `mesh_workspace` cache, exactly like the mesh-edit commands `lowpoly_retained_reduce`'s
/// `threaded!` macro rehydrates from the live persisted `LowpolyTransient`. A blank
/// `LowpolyScratch::default()` only ever covers the fresh single default object, so
/// `LowpolyDocument::reload_meshes` rejects it as stale for every object a prior mesh edit already
/// touched, `build_doc` returns `None`, and `handle` silently no-ops. This dispatches a real mesh
/// edit (extrude) first, THEN `addPrimitive`, and asserts both the new object actually lands AND
/// the extruded mesh is not reverted/lost in the process.
#[semio_framework_async_macros::async_test]
async fn add_primitive_after_mesh_edit_adds_object_and_preserves_the_edit() {
    let mut a = crate::editor::lowpoly::testkit::app_with_registry().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    crate::editor::lowpoly::testkit::select_face(&mut a, &object_id, 0).await;
    dispatch(&mut a, LowpolyCommand::Extrude(crate::editor::lowpoly::commands::mesh_edit::extrude::Extrude { extrude_distance: None })).await;
    let extruded_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
    assert!(extruded_mesh.is_some(), "extrude must have produced a mesh handle to guard");
    dispatch(&mut a, LowpolyCommand::AddPrimitive(AddPrimitive { kind: Some("cone".into()) })).await;
    let projection = a.snapshot().expect("projection");
    assert_eq!(projection.objects.len(), 2, "addPrimitive after a mesh edit must still add the new object, not silently no-op");
    assert!(projection.objects.iter().any(|object| object.name == "cone"), "the new primitive must actually be present");
    assert_eq!(projection.objects[0].mesh, extruded_mesh, "the pre-existing extrude edit must not be lost or reverted");
}
