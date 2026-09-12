//! 👁️ 👁️ Fem2d play app commands command — `set-result-display`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::fem2d::modes::edit::windows::results;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️SetResultDisplay
//#endregion 🔖️SetResultDisplay

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "result-display")]
pub struct SetResultDisplay {
    pub source_id: Option<String>,
    pub mode: String,
    pub mode_index: u32,
}

pub fn handle(_payload: &SetResultDisplay, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.results.window-context-required"))
}

pub fn handle_window(payload: &SetResultDisplay, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let mut next = results::config::current(cfg);
    next.result_source_id = payload.source_id.clone();
    next.result_mode = crate::app_surface::ResultMode::try_from(payload.mode.as_str()).map_err(Fault::from)?;
    next.result_mode_index = payload.mode_index;
    Ok(Emit { window_config_mutations: vec![results::config::addressed(view, next)?], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
