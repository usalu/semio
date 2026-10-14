//! 🧰️ CAD play app commands — per-pane options for the window-scoped Dislocate utility.

use semio_s_artifact_cad_cad_config::{CadConfig, CadConfigMutation};
use semio_s_artifact_cad_cad_kit::CadDispatchCtx;
use semio_s_artifact_cad_cad_modes::windows::config as window_config;
use semio_s_artifact_cad_cad_kit::cad_pane_id_from_suffix;
use semio_s_artifact_cad_cad_mutations::CadMutation;
use semio_s_artifact_cad_cad_model::snapshot::CadPaneId;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetDislocateOption
pub mod set_dislocate_option {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
