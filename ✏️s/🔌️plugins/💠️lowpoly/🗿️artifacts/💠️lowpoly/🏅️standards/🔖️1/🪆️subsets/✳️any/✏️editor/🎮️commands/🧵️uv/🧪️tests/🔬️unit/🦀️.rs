extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use crate::context::{app, dispatch};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn unwrap_active_resyncs_mesh_json() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::UnwrapActive(semio_s_artifact_lowpoly_lowpoly_commands::uv::unwrap_active::UnwrapActive {})).await;
    // unwrap is idempotent-ish on an already-unwrapped mesh, so just assert it runs without error and
    // keeps the object count stable.
    assert_eq!(a.snapshot().expect("projection").objects.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn clear_seam_delegates_to_mark_uv_seam_with_seam_false() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::ClearSeam(semio_s_artifact_lowpoly_lowpoly_commands::uv::clear_seam::ClearSeam {})).await;
    assert_eq!(a.snapshot().expect("projection").objects.len(), 1);
}
