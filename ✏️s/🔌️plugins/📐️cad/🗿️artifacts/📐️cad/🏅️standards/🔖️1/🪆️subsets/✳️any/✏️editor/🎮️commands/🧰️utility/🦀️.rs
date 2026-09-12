//! 🧰️ CAD play app commands — per-pane options for the window-scoped Dislocate utility.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::modes::edit::windows::config as window_config;
use crate::editor::cad::cad_pane_id_from_suffix;
use crate::op::CadMutation;
use crate::CadPaneId;
use crate::CadSnapshot;
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

    pub fn handle(payload: &SetDislocateOption, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let _surface = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_suffix);
        let mut config = window_config::current(cfg);
        let options = &mut config.dislocate_options;
        match payload.option.as_str() {
            "move" => options.move_enabled = payload.pressed.unwrap_or(!options.move_enabled),
            "rotate" => options.rotate_enabled = payload.pressed.unwrap_or(!options.rotate_enabled),
            _ => {}
        }
        Ok(Emit { window_config_mutations: vec![window_config::addressed_from_context(ctx, config)?], ..Default::default() })
    }
}
//#endregion 🔖️SetDislocateOption
