use crate::GISTERRAIN_DIALECT;
use semio_plugin::{ArtifactViewer, Viewer};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document;
#[allow(unused_imports)]
use crate::viewer::gisterrain::modes::view;
#[allow(unused_imports)]
use crate::viewer::gisterrain::modes::view::windows::terrain;
#[allow(unused_imports)]
use crate::{GisTerrainSnapshot, GIS_3D_TERRAIN_SCHEMA};
#[allow(unused_imports)]
use {semio_plugin::ArtifactView, semio_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_plugin::Fault, semio_plugin::NoConfig, semio_plugin::NoConfigMutation, semio_plugin::NoPresence, semio_plugin::NoPresenceMutation, semio_plugin::NoTransient, semio_plugin::NoTransientMutation, semio_plugin::ViewEmit};
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::viewer::gisterrain::*;

#[semio_framework_async_macros::async_test]
async fn create_gisterrain_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_gisterrain_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, GISTERRAIN_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<GisTerrainViewer as ArtifactViewer>::DIALECT, GISTERRAIN_DIALECT);
}
