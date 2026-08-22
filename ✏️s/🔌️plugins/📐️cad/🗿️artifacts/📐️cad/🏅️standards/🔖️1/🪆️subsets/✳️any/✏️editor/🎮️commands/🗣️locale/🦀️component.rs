//! 🗣️ CAD play app commands — the host-pushed locale and terminology switches.
//!
//! Neither is a user-facing palette action, which is why their wire keywords stay the bare
//! `"locale"`/`"terminology"` rather than the kebab-cased forms their command ids would suggest —
//! see the `as` literals in `crate::editor::cad`'s `app_commands!` invocation.

use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{runtime_of, snapshot_of};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.locale = payload.value.clone();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)?]))
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

    pub async fn handle(payload: &SetTerminology, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.terminology = payload.value.clone();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)?]))
    }
}
//#endregion 🔖️SetTerminology
