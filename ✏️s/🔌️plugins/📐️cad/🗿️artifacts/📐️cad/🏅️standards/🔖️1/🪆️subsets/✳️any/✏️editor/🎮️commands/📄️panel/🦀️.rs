//! 📄️ CAD play app commands — virtualised-panel paging.
//!
//! 📄️ `setPanelPage` is the navigation half of the framework's `paged_panel_section` convention: the
//! builder stops a section at its row quota and closes it with a `+N` continuation row, and THIS verb
//! is what that row dispatches to advance the section's cursor. The cursor is app view-state, so it
//! rides the config lane (`CadConfig::panel_pages_json`), never the document.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{runtime_of, snapshot_of};
use crate::op::CadMutation;
use crate::CadSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetPanelPage
pub mod set_panel_page {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-panel-page")]
    pub struct SetPanelPage {
        pub section: String,
        pub page: f64,
    }

    pub fn handle(payload: &SetPanelPage, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        if payload.section.is_empty() || !payload.page.is_finite() || payload.page < 0.0 {
            return Ok(Emit::default());
        }
        let mut runtime = runtime_of(cfg);
        let page = payload.page as u32;
        if runtime.panel_pages.get(&payload.section).copied() == Some(page) {
            return Ok(Emit::default());
        }
        runtime.panel_pages.insert(payload.section.clone(), page);
        Ok(Emit { config_mutations: vec![snapshot_of(&runtime, cfg.snapshot)?], ..Default::default() })
    }
}
//#endregion 🔖️SetPanelPage
