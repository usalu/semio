//! 📷️ Generation3d viewer command — `set-camera`. Read-only by construction: the emitted
//! `ViewEmit` carries config operations only, never an artifact or draft mutation.

use crate::viewer::generation3d::config;
use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation, Generation3dViewCamera};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
