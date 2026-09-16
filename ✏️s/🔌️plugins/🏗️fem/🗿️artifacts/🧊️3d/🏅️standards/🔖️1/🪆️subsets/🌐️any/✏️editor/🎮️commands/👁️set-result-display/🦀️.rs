//! 👁️ Fem3d play app command — `result-display`: the results window's source/mode selection —
//! window config only. The staged form speaks the TYPED fields; a results-panel control speaks
//! `{field, value, windowId}`, because the host merges a control's own scalar under `value`.

use crate::editor::fem3d::modes::edit::windows::results;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "result-display")]
pub struct SetResultDisplay {
    pub source_id: Option<String>,
    pub mode: String,
    pub mode_index: u32,
    pub field: Option<String>,
    pub value: Option<String>,
    /// 🪟️ The results window this gesture speaks for, as the panel control tagged it.
    pub window_id: Option<String>,
}

/// 👁️ Applies ONE named display field, leaving the rest of the window's state alone.
fn apply_field(config: &mut results::config::Fem3dResultsWindowConfig, field: &str, value: &str) -> Result<(), Fault> {
    match field {
        "sourceId" => config.result_source_id = Some(value.to_string()).filter(|id| !id.is_empty()),
        "mode" => config.result_mode = crate::app_surface::ResultMode::try_from(value).map_err(Fault::from)?,
        "modeIndex" => {
            let index = value.trim().parse::<f64>().map_err(|_| Fault::from(format!("fem3d.results.value: '{value}' is not a number")))?;
            config.result_mode_index = index.max(0.0) as u32;
        }
        other => return Err(Fault::from(format!("fem3d.results.field: '{other}' is not a display field"))),
    }
    Ok(())
}

pub fn handle(_payload: &SetResultDisplay, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem3d.results.window-context-required"))
}

pub fn handle_window(payload: &SetResultDisplay, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
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
    Ok(Emit { window_config_mutations: vec![results::config::addressed_to(&window_id, next)], ui_scope: crate::editor::fem3d::commands::set_result_animation::playback_dirty_scope(), ..Default::default() })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
