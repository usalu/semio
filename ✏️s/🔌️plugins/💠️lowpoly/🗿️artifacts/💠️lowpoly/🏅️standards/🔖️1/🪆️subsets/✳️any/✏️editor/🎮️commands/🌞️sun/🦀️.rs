//! 🌞️ Lowpoly play app commands — the world-3d sun toggle/azimuth/elevation/intensity, reusing the
//! framework's shared `WorldSunConfig`-shaped action logic. Config-only.

use crate::op::LowpolyMutation;
use crate::LowpolySnapshot;
use crate::editor::lowpoly::config::{lowpoly_sun_config, LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 🌞️ Reuses the framework's shared sun toggle/slider logic (`apply_world3d_sun_action`), threading it
/// through `LowpolyConfig`'s flattened sun fields and returning the resulting `SetSun` config op.
/// `apply_world3d_sun_action` is a framework fn (`🧰️framework/…/🔌️plugin/🦀️.rs`, `world3d_host` module)
/// whose `args` parameter is `Option<&dsl::os_pack::json::Value>`, so the single-entry object is
/// built through the unconditional `DslValue` bridge (`🌱️value/🦀️.rs`) and converted only at the
/// call boundary via `dsl::os_pack::json::from_dsl_value`, never via `serde_json::json!`.
fn apply_sun_command(config: &LowpolyConfig, action_id: &str, value: Option<f64>) -> LowpolyConfigMutation {
    let mut sun = lowpoly_sun_config(config);
    let args = value.map(|value| dsl::os_pack::json::from_dsl_value(&dsl::DslValue::object([("value".to_string(), dsl::DslValue::float(value))])));
    apply_world3d_sun_action(&mut sun, action_id, args.as_ref());
    LowpolyConfigMutation::SetSun { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color }
}

//#region 🔖️ToggleSun
pub mod toggle_sun {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "toggle-sun")]
    pub struct ToggleSun {}

    pub fn handle(_payload: &ToggleSun, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![apply_sun_command(cfg.snapshot, "toggleSun", None)]))
    }
}
//#endregion 🔖️ToggleSun

//#region 🔖️SetSunAzimuth
pub mod set_sun_azimuth {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-sun-azimuth")]
    pub struct SetSunAzimuth {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunAzimuth, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![apply_sun_command(cfg.snapshot, "setSunAzimuth", Some(payload.value))]))
    }
}
//#endregion 🔖️SetSunAzimuth

//#region 🔖️SetSunElevation
pub mod set_sun_elevation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-sun-elevation")]
    pub struct SetSunElevation {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunElevation, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![apply_sun_command(cfg.snapshot, "setSunElevation", Some(payload.value))]))
    }
}
//#endregion 🔖️SetSunElevation

//#region 🔖️SetSunIntensity
pub mod set_sun_intensity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-sun-intensity")]
    pub struct SetSunIntensity {
        pub value: f64,
    }

    pub fn handle(payload: &SetSunIntensity, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![apply_sun_command(cfg.snapshot, "setSunIntensity", Some(payload.value))]))
    }
}
//#endregion 🔖️SetSunIntensity

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
