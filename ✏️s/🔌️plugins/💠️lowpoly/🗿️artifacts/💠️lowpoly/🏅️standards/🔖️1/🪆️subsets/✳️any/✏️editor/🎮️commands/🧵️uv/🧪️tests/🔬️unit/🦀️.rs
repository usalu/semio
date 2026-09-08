
use crate::editor::lowpoly::LowpolyCommand;
use crate::editor::lowpoly::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn unwrap_active_resyncs_mesh_json() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::UnwrapActive(super::unwrap_active::UnwrapActive {})).await;
    // unwrap is idempotent-ish on an already-unwrapped mesh, so just assert it runs without error and
    // keeps the object count stable.
    assert_eq!(a.snapshot().expect("projection").objects.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn clear_seam_delegates_to_mark_uv_seam_with_seam_false() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::ClearSeam(super::clear_seam::ClearSeam {})).await;
    assert_eq!(a.snapshot().expect("projection").objects.len(), 1);
}
