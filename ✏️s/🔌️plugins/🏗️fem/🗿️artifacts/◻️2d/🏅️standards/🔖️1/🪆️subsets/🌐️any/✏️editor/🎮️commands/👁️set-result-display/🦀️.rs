//! 👁️ Fem2d play app commands command — `set-result-display`.
//!
//! Two vocabularies, one command (the same split `set-result-animation` documents): the staged form
//! sends the whole typed triple, while a persistent `select`/stepper in the results panel sends
//! `{field, value}` — the host merges a control's own scalar under the single key `value`, so one
//! control cannot name the field it just changed any other way.

use crate::editor::fem2d::modes::edit::windows::results;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️SetResultDisplay
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "result-display")]
pub struct SetResultDisplay {
    pub source_id: Option<String>,
    pub mode: String,
    pub mode_index: u32,
    pub field: Option<String>,
    pub value: Option<String>,
    /// 🪟️ The results window this gesture speaks for, as the panel control tagged it. A panel
    /// projection carries no `window_id` of its own, so the tag is the ONLY thing that keeps a split
    /// layout from retuning the pane the user is not looking at.
    pub window_id: Option<String>,
}

/// 👁️ Applies ONE named display field, leaving the rest of the window's state alone.
fn apply_field(config: &mut results::config::Fem2dResultsWindowConfig, field: &str, value: &str) -> Result<(), Fault> {
    match field {
        "sourceId" => config.result_source_id = Some(value.to_string()).filter(|id| !id.is_empty()),
        "mode" => config.result_mode = crate::app_surface::ResultMode::try_from(value).map_err(Fault::from)?,
        "modeIndex" => {
            let index = value.trim().parse::<f64>().map_err(|_| Fault::from(format!("fem2d.results.value: '{value}' is not a number")))?;
            config.result_mode_index = index.max(0.0) as u32;
        }
        other => return Err(Fault::from(format!("fem2d.results.field: '{other}' is not a display field"))),
    }
    Ok(())
}

pub fn handle(_payload: &SetResultDisplay, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.results.window-context-required"))
}

pub fn handle_window(payload: &SetResultDisplay, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let window_id = results::config::addressed_window_id(cfg, view, payload.window_id.as_deref())?;
    let mut next = results::config::current(cfg);
    match payload.field.as_deref().filter(|field| !field.is_empty()) {
        Some(field) => apply_field(&mut next, field, payload.value.as_deref().unwrap_or_default())?,
        None => {
            next.result_source_id = payload.source_id.clone();
            next.result_mode = crate::app_surface::ResultMode::try_from(payload.mode.as_str()).map_err(Fault::from)?;
            next.result_mode_index = payload.mode_index;
        }
    }
    Ok(Emit { window_config_mutations: vec![results::config::addressed_to(&window_id, next)], ui_scope: semio_framework::kernel::UiDirtyScope::Partial { window_bodies: vec![results::BODY_KEY.to_owned()], panel_bodies: vec![crate::editor::fem2d::panels::results::BODY_KEY.to_owned()], utilities: false, tools: false, engagements: false, measures: false, labels: false }, ..Default::default() })
}
//#endregion 🔖️SetResultDisplay

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
