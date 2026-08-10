//! 🗣️ CAD play app commands — the host-pushed locale and terminology switches.
//!
//! Neither is a user-facing palette action, which is why their wire keywords stay the bare
//! `"locale"`/`"terminology"` rather than the kebab-cased forms their command ids would suggest —
//! see the `as` literals in `crate::apps::cad`'s `app_commands!` invocation.

use crate::apps::cad::config::{CadConfig, CadConfigMutation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{cad_config_from_runtime, runtime_of};


//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut config = cad_config_from_runtime(&runtime_of(cfg), cfg.snapshot);
        config.locale = payload.value.clone();
        Ok(Emit::config(vec![CadConfigMutation::Snapshot { config }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🔖️SetTerminology
pub mod set_terminology {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "terminology")]
    pub struct SetTerminology {
        pub value: String,
    }

    pub fn handle(payload: &SetTerminology, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut config = cad_config_from_runtime(&runtime_of(cfg), cfg.snapshot);
        config.terminology = payload.value.clone();
        Ok(Emit::config(vec![CadConfigMutation::Snapshot { config }]))
    }
}
//#endregion 🔖️SetTerminology
