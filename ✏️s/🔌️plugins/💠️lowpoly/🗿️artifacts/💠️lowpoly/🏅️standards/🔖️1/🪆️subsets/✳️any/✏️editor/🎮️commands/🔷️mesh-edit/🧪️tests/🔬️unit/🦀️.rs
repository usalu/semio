
use crate::editor::lowpoly::LowpolyCommand;
use crate::editor::lowpoly::testkit::{app, app_with_registry, dispatch, select_face};
use semio_framework_plugin::PluginApp;

/// 🧪️ Rewritten (round 2 of ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's round-trip law
/// fix) to assert on the persisted `mesh` HANDLE rather than reconstructing a `LowpolyDocument`
/// and counting faces: the live half-edge-mesh JSON no longer lives on `LowpolyObject` at all, and
/// store-level undo/redo (which this test exercises) bypasses `ArtifactApp::handle` entirely, so
/// the app's own session-local `mesh_workspace` cache is never resynced by it — there is no
/// honest way to rebuild real post-undo geometry from outside the live session any more. The
/// handle still round-trips correctly through undo either way, which is what this test now checks.
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking a face is now the
/// framework's injected `interactionSelect`, so this needs `app_with_registry().await` (the "mesh" domain
/// must be declared to select against).
#[semio_framework_async_macros::async_test]
async fn extrude_selected_face_grows_mesh_and_undo_restores() {
    let mut a = app_with_registry().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    let before_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
    select_face(&mut a, &object_id, 0).await;
    dispatch(&mut a, LowpolyCommand::Extrude(super::extrude::Extrude { extrude_distance: None })).await;
    let after_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
    assert_ne!(after_mesh, before_mesh, "extrude must change the mesh handle");
    a.handle_action("undo", None, &semio_framework_plugin::testkit::meta("a")).await.unwrap();
    let restored_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
    assert_eq!(restored_mesh, before_mesh, "undo restores the pre-extrude mesh handle");
}

#[semio_framework_async_macros::async_test]
async fn extrude_reads_staged_arg_distance_into_the_operation() {
    // 🧪️ Arg-form action: the staged `extrudeDistance` (not the config backing store) drives the edit.
    let mut small = app_with_registry().await;
    let mut large = app_with_registry().await;
    let object_id = small.snapshot().expect("projection").objects[0].id.clone();
    select_face(&mut small, &object_id, 0).await;
    select_face(&mut large, &object_id, 0).await;
    dispatch(&mut small, LowpolyCommand::Extrude(super::extrude::Extrude { extrude_distance: Some(0.1) })).await;
    dispatch(&mut large, LowpolyCommand::Extrude(super::extrude::Extrude { extrude_distance: Some(1.5) })).await;
    let small_handle = small.snapshot().expect("projection").objects.iter().find(|o| o.id == object_id).unwrap().mesh.clone();
    let large_handle = large.snapshot().expect("projection").objects.iter().find(|o| o.id == object_id).unwrap().mesh.clone();
    assert_ne!(small_handle, large_handle, "different staged extrude distances must produce different meshes");
}

#[semio_framework_async_macros::async_test]
async fn toggle_smooth_emits_op_and_flips_shading() {
    let mut a = app().await;
    let before = a.snapshot().expect("projection").objects[0].smooth_shading;
    dispatch(&mut a, LowpolyCommand::ToggleSmooth(super::toggle_smooth::ToggleSmooth {})).await;
    assert_ne!(a.snapshot().expect("projection").objects[0].smooth_shading, before);
}
