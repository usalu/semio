//! 👁️ Fem2d play app commands command — `set-result-display`.
//!
//! Two vocabularies, one command (the same split `set-result-animation` documents): the staged form
//! sends the whole typed triple, while a persistent `select`/stepper in the results panel sends
//! `{field, value}` — the host merges a control's own scalar under the single key `value`, so one
//! control cannot name the field it just changed any other way.

use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️SetResultDisplay
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
        "sourceId" => {
            let source = Some(value.to_string()).filter(|id| !id.is_empty());
            if let Some(source) = source.as_deref() { semio_s_fem_analyses_model::decode_result_source_identity(source).map_err(|error| Fault::from(error.to_string()))?; }
            config.result_source_id = source;
        }
        "mode" => config.result_mode = semio_s_fem_app_surface::ResultMode::try_from(value).map_err(Fault::from)?,
        "modeIndex" => {
            let index = value.trim().parse::<f64>().map_err(|_| Fault::from(format!("fem2d.results.value: '{value}' is not a number")))?;
            config.result_mode_index = index.max(0.0) as u32;
        }
        other => return Err(Fault::from(format!("fem2d.results.field: '{other}' is not a display field"))),
    }
    Ok(())
}

/// 🔎️ The canonical result-source identity of a bare load case or combination id or name, so the palette's free-text Source accepts what the document calls its cases; an identity, an empty value or an unknown name stays as typed for validation to judge.
fn canonical_source(raw: &str, doc: &Fem2dSnapshot) -> Option<String> {
    if raw.is_empty() || semio_s_fem_analyses_model::decode_result_source_identity(raw).is_ok() {
        return None;
    }
    let wanted = raw.trim().to_lowercase();
    let named = |id: &str, name: &str| id.to_lowercase() == wanted || name.to_lowercase() == wanted;
    let (kind, id) = doc.load_cases.iter().find(|case| named(&case.id, &case.name)).map(|case| ("loadCase", case.id.as_str())).or_else(|| doc.combinations.iter().find(|combination| named(&combination.id, &combination.name)).map(|combination| ("combination", combination.id.as_str())))?;
    semio_s_fem_analyses_model::encode_result_source_parts(kind, id).ok()
}

/// 🔎️ The payload with its source spelled as a canonical identity (see [`canonical_source`]).
pub fn resolved(payload: &SetResultDisplay, doc: &ArtifactView<'_, Fem2dSnapshot>) -> SetResultDisplay {
    let mut next = payload.clone();
    if next.field.as_deref() == Some("sourceId") {
        next.value = next.value.as_deref().and_then(|value| canonical_source(value, doc.snapshot)).or(next.value);
    } else {
        next.source_id = next.source_id.as_deref().and_then(|value| canonical_source(value, doc.snapshot)).or(next.source_id);
    }
    next
}

pub fn handle(_payload: &SetResultDisplay, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.results.window-context-required"))
}

pub fn handle_window(payload: &SetResultDisplay, cfg: &ConfigView<'_, NoConfig>, view: &semio_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let window_id = results::config::addressed_window_id(cfg, view, payload.window_id.as_deref())?;
    let live = results::config::captured(cfg).is_some();
    let mut next = if live { results::config::current(cfg) } else { results::config::remembered(&window_id).unwrap_or_default() };
    let field = payload.field.as_deref().filter(|field| !field.is_empty());
    match field {
        Some(field) => apply_field(&mut next, field, payload.value.as_deref().unwrap_or_default())?,
        None => {
            next.result_source_id = payload.source_id.clone().filter(|id| !id.is_empty());
            next.result_mode = semio_s_fem_app_surface::ResultMode::try_from(payload.mode.as_str()).map_err(Fault::from)?;
            next.result_mode_index = payload.mode_index;
        }
    }
    if let Some(source) = next.result_source_id.as_deref() { semio_s_fem_analyses_model::decode_result_source_identity(source).map_err(|error| Fault::from(error.to_string()))?; }
    let mutation = if live { results::config::addressed_to(&window_id, next.clone()) } else { results::config::addressed_fields(&window_id, &next, field) };
    results::config::remember(&window_id, &next);
    Ok(Emit { window_config_mutations: vec![mutation], ui_scope: semio_framework::kernel::UiDirtyScope::Partial { window_bodies: vec![results::BODY_KEY.to_owned()], panel_bodies: vec![semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::results::BODY_KEY.to_owned()], utilities: false, tools: false, engagements: false, measures: false, labels: false }, ..Default::default() })
}
//#endregion 🔖️SetResultDisplay

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
