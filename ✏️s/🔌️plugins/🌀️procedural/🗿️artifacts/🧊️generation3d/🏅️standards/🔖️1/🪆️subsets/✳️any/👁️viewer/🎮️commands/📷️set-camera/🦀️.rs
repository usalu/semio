//! 📷️ Generation3d viewer command — `set-camera`. Read-only by construction: the emitted
//! `ViewEmit` carries config operations only, never an artifact or draft mutation.

use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewCamera;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "camera")]
#[value(rename_all = "camelCase")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: Generation3dViewCamera,
}

/// 📷️ Orbit / pan / zoom: the host reports the settled viewport camera as one whole facet.
pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::config(vec![Generation3dViewConfigMutation::SetPreviewCamera(config::SetPreviewCamera { camera: payload.camera.clone() })]))
}
