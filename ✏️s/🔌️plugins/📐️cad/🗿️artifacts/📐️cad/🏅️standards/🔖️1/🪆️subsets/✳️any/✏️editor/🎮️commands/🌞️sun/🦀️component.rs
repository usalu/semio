//! 🌞️ CAD play app commands — the shared sun/environment controls. Config-only and coalesced, so a slider drag is one undo step.

use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{runtime_of, snapshot_of};
use semio_framework_plugin::apply_world3d_sun_action;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️ToggleSun
pub mod toggle_sun {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-sun")]
    pub struct ToggleSun {}

    pub fn handle(_payload: &ToggleSun, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        apply_world3d_sun_action(&mut runtime.sun, "toggleSun", None);
        Ok(Emit::amend_config(vec![snapshot_of(&runtime, cfg.snapshot)?], "sun"))
    }
}
//#endregion 🔖️ToggleSun

//#region 🔖️SetSunAzimuth
pub mod set_sun_azimuth {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-azimuth")]
    pub struct SetSunAzimuth {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunAzimuth, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let args_value = json!({ "value": payload.value });
        apply_world3d_sun_action(&mut runtime.sun, "setSunAzimuth", Some(&args_value));
        Ok(Emit::amend_config(vec![snapshot_of(&runtime, cfg.snapshot)?], "sun"))
    }
}
//#endregion 🔖️SetSunAzimuth

//#region 🔖️SetSunElevation
pub mod set_sun_elevation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-elevation")]
    pub struct SetSunElevation {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunElevation, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let args_value = json!({ "value": payload.value });
        apply_world3d_sun_action(&mut runtime.sun, "setSunElevation", Some(&args_value));
        Ok(Emit::amend_config(vec![snapshot_of(&runtime, cfg.snapshot)?], "sun"))
    }
}
//#endregion 🔖️SetSunElevation

//#region 🔖️SetSunIntensity
pub mod set_sun_intensity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sun-intensity")]
    pub struct SetSunIntensity {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunIntensity, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let args_value = json!({ "value": payload.value });
        apply_world3d_sun_action(&mut runtime.sun, "setSunIntensity", Some(&args_value));
        Ok(Emit::amend_config(vec![snapshot_of(&runtime, cfg.snapshot)?], "sun"))
    }
}
//#endregion 🔖️SetSunIntensity
