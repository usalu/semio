//! 👁️ Generation3d viewer command — `set-show-mode`. Read-only by construction: the emitted
//! `ViewEmit` carries config operations only, never an artifact or draft mutation.

use crate::viewer::generation3d::config;
use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "show-mode")]
#[value(rename_all = "camelCase")]
pub struct SetShowMode {
    pub value: String,
}

/// 👁️ Switches the read-only preview between shaded / shaded+edges / wireframe / points.
pub fn handle(payload: &SetShowMode, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    Ok(ViewEmit::config(vec![Generation3dViewConfigMutation::SetShowMode(config::SetShowMode { value: payload.value.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
