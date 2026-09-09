use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn fem2d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_fem_2d::viewer::fem2d::Fem2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn fem2d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_fem_2d::editor::fem2d::Fem2dPlayApp, semio_s_artifact_fem_2d::viewer::fem2d::Fem2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn fem3d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_fem_3d::viewer::fem3d::Fem3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn fem3d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_fem_3d::editor::fem3d::Fem3dPlayApp, semio_s_artifact_fem_3d::viewer::fem3d::Fem3dViewer>().await;
}
