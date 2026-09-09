//! 🌞️ Generation3d viewer command — `set-sun-azimuth`. Read-only by construction: the emitted
//! `ViewEmit` carries config operations only, never an artifact or draft mutation.

use crate::viewer::generation3d::config;
use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Fault, ViewEmit};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "sun-azimuth")]
#[value(rename_all = "camelCase")]
pub struct SetSunAzimuth {
    pub value: f64,
}

/// 🧭️ Swings the read-only preview's sun around the horizon.
pub fn handle(payload: &SetSunAzimuth, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    let mut sun = cfg.snapshot.sun();
    apply_world3d_sun_action(&mut sun, "setSunAzimuth", Some(&dsl::json!({ "value": payload.value })));
    Ok(ViewEmit::config(vec![Generation3dViewConfigMutation::SetSun(config::SetSun { json: dsl::json::to_json_string(&sun) })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
