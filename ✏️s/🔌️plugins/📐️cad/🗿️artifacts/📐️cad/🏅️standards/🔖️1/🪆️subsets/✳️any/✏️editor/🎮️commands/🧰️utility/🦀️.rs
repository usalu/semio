//! 🧰️ CAD play app commands — per-pane options for the window-scoped Dislocate utility.

use crate::op::CadMutation;
use crate::CadPaneId;
use crate::CadSnapshot;
use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{cad_pane_id_from_suffix, cad_window_id_for_pane, preview_transition_snapshot_of, runtime_of, snapshot_of};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetDislocateOption
pub mod set_dislocate_option {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "dislocate-option")]
    pub struct SetDislocateOption {
        pub pane: Option<String>,
        pub option: String,
        pub pressed: Option<bool>,
    }

    pub fn handle(payload: &SetDislocateOption, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
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
