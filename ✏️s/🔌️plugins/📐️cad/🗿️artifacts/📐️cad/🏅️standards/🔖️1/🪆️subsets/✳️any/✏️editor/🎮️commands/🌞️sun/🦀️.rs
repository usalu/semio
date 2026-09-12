//! 🌞️ CAD sun controls persisted by the exact invoking world window.

use crate::editor::cad::config::{cad_sun_config_from_world, cad_sun_config_to_world, CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::modes::edit::windows::config as window_config;
use crate::op::CadMutation;
use crate::CadSnapshot;
use protocol::DslValue;
use semio_framework_plugin::apply_world3d_sun_action;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ToggleSun
pub mod toggle_sun {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "toggle-sun")]
    pub struct ToggleSun {}

    pub fn handle(_payload: &ToggleSun, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut config = window_config::current(cfg);
        let mut sun = cad_sun_config_to_world(&config.sun);
        apply_world3d_sun_action(&mut sun, "toggleSun", None);
        config.sun = cad_sun_config_from_world(&sun);
        Ok(Emit { window_config_mutations: vec![window_config::addressed_from_context(ctx, config)?], ..Default::default() })
    }
}
//#endregion 🔖️ToggleSun

//#region 🔖️SetSunAzimuth
pub mod set_sun_azimuth {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "sun-azimuth")]
    pub struct SetSunAzimuth {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunAzimuth, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut config = window_config::current(cfg);
        let mut sun = cad_sun_config_to_world(&config.sun);
        // 🌉️ `apply_world3d_sun_action` (framework `🔌️plugin/🦀️.rs`) takes `Option<&dsl::os_pack::json::Value>` — a genuine framework boundary, bridged once here from a `DslValue` built the normal way.
        let args_value = protocol::json::from_dsl_value(&DslValue::object([("value".to_string(), DslValue::float(payload.value))]));
        apply_world3d_sun_action(&mut sun, "setSunAzimuth", Some(&args_value));
        config.sun = cad_sun_config_from_world(&sun);
        Ok(Emit { window_config_mutations: vec![window_config::addressed_from_context(ctx, config)?], ..Default::default() })
    }
}
//#endregion 🔖️SetSunAzimuth

//#region 🔖️SetSunElevation
pub mod set_sun_elevation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "sun-elevation")]
    pub struct SetSunElevation {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunElevation, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut config = window_config::current(cfg);
        let mut sun = cad_sun_config_to_world(&config.sun);
        let args_value = protocol::json::from_dsl_value(&DslValue::object([("value".to_string(), DslValue::float(payload.value))]));
        apply_world3d_sun_action(&mut sun, "setSunElevation", Some(&args_value));
        config.sun = cad_sun_config_from_world(&sun);
        Ok(Emit { window_config_mutations: vec![window_config::addressed_from_context(ctx, config)?], ..Default::default() })
    }
}
//#endregion 🔖️SetSunElevation

//#region 🔖️SetSunIntensity
pub mod set_sun_intensity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "sun-intensity")]
    pub struct SetSunIntensity {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunIntensity, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut config = window_config::current(cfg);
        let mut sun = cad_sun_config_to_world(&config.sun);
        let args_value = protocol::json::from_dsl_value(&DslValue::object([("value".to_string(), DslValue::float(payload.value))]));
        apply_world3d_sun_action(&mut sun, "setSunIntensity", Some(&args_value));
        config.sun = cad_sun_config_from_world(&sun);
        Ok(Emit { window_config_mutations: vec![window_config::addressed_from_context(ctx, config)?], ..Default::default() })
    }
}
//#endregion 🔖️SetSunIntensity
