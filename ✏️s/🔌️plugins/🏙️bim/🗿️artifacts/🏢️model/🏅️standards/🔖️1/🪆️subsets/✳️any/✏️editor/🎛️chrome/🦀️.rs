//! 🎛️ BIM window chrome: the status line of every window and the measures that set its view parameters (the plan's storey and cut height, the world's projection, storey isolation
//! and section plane, the section's depth). Both are keyed by the window instance the view addresses, read that window's own config, and dispatch `setView`.

use crate::editor::bim::entities::{ordered_storeys, storey_of, ENTITIES};
use crate::editor::bim::kit::bim_window_action;
use crate::editor::bim::modes::edit::windows::{plan, section, world};
use crate::editor::bim::terminology::{bim_labels, BimLabels};
use crate::ModelSnapshot;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::MeasureSelectItem;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementStatus;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;

/// 🪟️ The kind of the window instance a view addresses.
pub fn addressed_kind(view_state: &ViewModel) -> Option<&str> {
    let id = view_state.window_id.as_deref()?;
    view_state.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str())
}

//#region 🔖️Status
fn count_on(snapshot: &ModelSnapshot, storey: &str) -> usize {
    ENTITIES.iter().filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid")).map(|row| (row.ids)(snapshot).iter().filter(|id| storey_of(snapshot, id).as_deref() == Some(storey)).count()).sum()
}

/// 📟️ The status line of the addressed window: the storey it shows (or all), how many elements that is and how many are selected.
pub fn status(snapshot: &ModelSnapshot, view_state: &ViewModel, plan_storey: Option<&str>, selected: usize) -> String {
    let labels = bim_labels(view_state);
    let mut parts = Vec::new();
    match plan_storey.and_then(|storey| snapshot.storeys.get(storey).map(|row| (storey, row))) {
        Some((storey, row)) => {
            parts.push(BimLabels::named(labels.status_storey, &row.name));
            parts.push(BimLabels::counted(labels.status_elements, count_on(snapshot, storey)));
        }
        None => parts.push(labels.status_all_storeys.as_str().to_string()),
    }
    parts.push(BimLabels::counted(labels.status_selected, selected));
    parts.join(" · ")
}

/// 📟️ One engagement (status only, no input) for the addressed window.
pub fn engagements(snapshot: &ModelSnapshot, view_state: &ViewModel, plan_storey: Option<&str>, selected: usize) -> HashMap<String, WindowEngagement> {
    let Some(window) = view_state.window_id.clone() else { return HashMap::new() };
    let text = status(snapshot, view_state, plan_storey, selected);
    HashMap::from([(window.clone(), WindowEngagement { session_active: Some(addressed_kind(view_state) == Some(world::WINDOW_KIND_ID) && crate::editor::bim::utilities::draws(crate::editor::bim::utilities::active(view_state))), options: None, input: None, control: None, controls: None, status: Some(vec![WindowEngagementStatus { id: format!("{window}.status"), text }]), possible_engagements: None })])
}
//#endregion 🔖️Status

//#region 🔖️Measures
fn view_action(field: &str) -> semio_framework_plugin::ActionDescriptor {
    bim_window_action("setView", Some(DslValue::object([("field".to_string(), DslValue::String(field.to_string()))])))
}

fn select(id: &str, label: &str, value: &str, items: Vec<(String, String)>, field: &str) -> WindowMeasure {
    WindowMeasure::Select { id: id.to_string(), label: Some(label.to_string()), value: value.to_string(), items: items.into_iter().map(|(value, label)| MeasureSelectItem { id: value.clone(), value, label }).collect(), on_change: view_action(field) }
}

fn number(id: &str, label: &str, value: f64, min: Option<f64>, step: f64, field: &str) -> WindowMeasure {
    WindowMeasure::Number { id: id.to_string(), label: Some(label.to_string()), value, min, max: None, step: Some(step), ready: None, loading: None, waiting: None, disabled: None, on_change: view_action(field) }
}

fn storey_items(snapshot: &ModelSnapshot) -> Vec<(String, String)> {
    snapshot.buildings.keys().flat_map(|building| ordered_storeys(snapshot, building)).map(|storey| (storey.clone(), snapshot.storeys.get(&storey).map_or(storey.clone(), |row| row.name.clone()))).collect()
}

fn plan_measures(snapshot: &ModelSnapshot, config: &plan::config::BimPlanWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let storey = plan::active_storey(snapshot, config).unwrap_or_default();
    vec![select("bim.measure.plan.storey", labels.measure_storey.as_str(), &storey, storey_items(snapshot), "storey"), number("bim.measure.plan.cut-height", labels.measure_cut_height.as_str(), config.cut_height, Some(0.0), 0.1, "cut_height")]
}

fn world_measures(snapshot: &ModelSnapshot, config: &world::config::BimWorldWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let projection = vec![
        ("threePoint".to_string(), labels.projection_three_point.as_str().to_string()),
        ("orthographic".to_string(), labels.projection_orthographic.as_str().to_string()),
        ("axonometric".to_string(), labels.projection_axonometric.as_str().to_string()),
        ("onePoint".to_string(), labels.projection_one_point.as_str().to_string()),
        ("twoPoint".to_string(), labels.projection_two_point.as_str().to_string()),
    ];
    let mut isolate = vec![(String::new(), labels.measure_all_storeys.as_str().to_string())];
    isolate.extend(storey_items(snapshot));
    let axes = ["x", "y", "z"].iter().map(|axis| (axis.to_string(), axis.to_uppercase())).collect();
    vec![
        select("bim.measure.world.projection", labels.measure_projection.as_str(), &config.projection.kind, projection, "projection"),
        select("bim.measure.world.isolate", labels.measure_isolate.as_str(), &config.isolated_storey, isolate, "isolated_storey"),
        WindowMeasure::Toggle { id: "bim.measure.world.section".into(), icon_id: "scissors".into(), label: Some(labels.measure_section_enabled.as_str().to_string()), pressed: config.section_enabled, text: None, on_change: view_action("section_enabled") },
        select("bim.measure.world.section-axis", labels.measure_section_axis.as_str(), &config.section_axis, axes, "section_axis"),
        number("bim.measure.world.section-offset", labels.measure_section_offset.as_str(), config.section_offset, None, 0.1, "section_offset"),
    ]
}

fn section_measures(config: &section::config::BimSectionWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    vec![number("bim.measure.section.depth", labels.measure_section_depth.as_str(), config.depth, Some(0.0), 0.5, "depth")]
}

/// 🎚️ The measures of the addressed window, from that window's own config.
pub fn measures<C>(snapshot: &ModelSnapshot, cfg: &ConfigView<'_, C>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
    let (Some(window), Some(kind)) = (view_state.window_id.clone(), addressed_kind(view_state)) else { return HashMap::new() };
    let labels = bim_labels(view_state);
    let measures = match kind {
        plan::WINDOW_KIND_ID => plan_measures(snapshot, &plan::config::current(cfg), labels),
        world::WINDOW_KIND_ID => world_measures(snapshot, &world::config::current(cfg), labels),
        section::WINDOW_KIND_ID => section_measures(&section::config::current(cfg), labels),
        _ => Vec::new(),
    };
    HashMap::from([(window, measures)])
}

/// 🪜️ The storey the addressed plan window shows, for its status line.
pub fn plan_storey<C>(snapshot: &ModelSnapshot, cfg: &ConfigView<'_, C>, view_state: &ViewModel) -> Option<String> {
    (addressed_kind(view_state) == Some(plan::WINDOW_KIND_ID)).then(|| plan::active_storey(snapshot, &plan::config::current(cfg))).flatten()
}
//#endregion 🔖️Measures

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
