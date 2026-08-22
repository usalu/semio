//! 🧰️ CAD play app commands — the window-scoped Dislocate utility: activation and its per-pane handle options.

use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadPaneId;
use crate::artifacts::cad::CadSnapshot;
use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{cad_pane_id_from_suffix, cad_window_id_for_pane, preview_transition_snapshot_of, runtime_of, snapshot_of};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    pub async fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        // 🧰️ Switching the active utility is config-only: it never mutates the document. Clear
        // any in-progress engagement session / rubber-band scratch so a stale preview cannot
        // leak across a utility switch.
        let mut runtime = runtime_of(cfg);
        runtime.engagement_input.clear();
        runtime.engagement_session = None;
        runtime.engagement_step = "Idle".into();
        runtime.active_utility_id = payload.utility_id.clone();
        Ok(Emit::config(vec![preview_transition_snapshot_of(&runtime, cfg.snapshot, ctx)?]))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🔖️SetDislocateOption
pub mod set_dislocate_option {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "dislocate-option")]
    pub struct SetDislocateOption {
        pub pane: Option<String>,
        pub option: String,
        pub pressed: Option<bool>,
    }

    pub async fn handle(payload: &SetDislocateOption, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        let pane = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_suffix);
        let window_id = cad_window_id_for_pane(pane);
        let options = runtime.dislocate_options_by_window_id.entry(window_id.into()).or_default();
        match payload.option.as_str() {
            "move" => options.move_enabled = payload.pressed.unwrap_or(!options.move_enabled),
            "rotate" => options.rotate_enabled = payload.pressed.unwrap_or(!options.rotate_enabled),
            _ => {}
        }
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)?]))
    }
}
//#endregion 🔖️SetDislocateOption
