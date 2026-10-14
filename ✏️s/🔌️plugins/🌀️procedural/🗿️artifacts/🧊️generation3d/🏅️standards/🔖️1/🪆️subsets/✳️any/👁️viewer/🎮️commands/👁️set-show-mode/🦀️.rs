//! 👁️ Generation3d viewer command — `set-show-mode`. Read-only by construction: the emitted
//! `ViewEmit` carries config operations only, never an artifact or draft mutation.

use semio_s_artifact_procedural_3d_viewer_config as config;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "show-mode")]
#[value(rename_all = "camelCase")]
pub struct SetShowMode {
    pub value: String,
}

/// 👁️ Switches the read-only preview between shaded / shaded+edges / wireframe / points.
pub fn handle(payload: &SetShowMode, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::config(vec![Generation3dViewConfigMutation::SetShowMode(config::SetShowMode { value: payload.value.clone() })]))
}
