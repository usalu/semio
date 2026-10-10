//! 🎛️ BIM window chrome: the status line of every window and the measures that set its view parameters (the plan's and the section's view, the world's projection, storey isolation
//! and section plane). Both are keyed by the window instance the view addresses, read that window's own config, and dispatch `setView`.

use crate::editor::bim::entities::{ordered_storeys, storey_of, ENTITIES};
use crate::editor::bim::kit::bim_window_action;
use crate::editor::bim::modes::edit::windows::{plan, schedule, section, sheet, world};
use crate::editor::bim::terminology::{bim_labels, BimLabels};
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::SeverityCounts;
use crate::ModelSnapshot;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::MeasureSelectItem;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementInput;
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
    ENTITIES.iter().filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid" | "view" | "curtain-panel-override")).map(|row| (row.ids)(snapshot).iter().filter(|id| storey_of(snapshot, id).as_deref() == Some(storey)).count()).sum()
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

/// 🚨️ The problems line of the status: the findings of the diagnostics by severity, or that there are none. The same line in every window, so the author sees the health of the model wherever they work.
pub fn problems(labels: &BimLabels, counts: &SeverityCounts) -> String {
    if counts.total() == 0 {
        return labels.status_no_problems.as_str().to_string();
    }
    [(counts.error, labels.status_errors), (counts.warning, labels.status_warnings), (counts.info, labels.status_notes)].into_iter().filter(|(count, _)| *count > 0).map(|(count, label)| BimLabels::counted(label, count as usize)).collect::<Vec<_>>().join(" · ")
}

/// ⌨️ The entry field of a window: the keyboard twin of the pointer. Typing keeps the line (window transient and presence), Enter gives it to the armed utility as a click, Escape cancels
/// the gesture and empties the field. The host holds the line while it is typed, so the field publishes no value of its own.
pub fn entry(window: &str, labels: &BimLabels) -> WindowEngagementInput {
    WindowEngagementInput {
        id: Some(format!("{window}.entry")),
        value: None,
        placeholder: Some(labels.entry_placeholder.as_str().to_string()),
        disabled: None,
        on_change: Some(bim_window_action("engagementInput", None)),
        on_submit: Some(bim_window_action("engagementSubmit", None)),
        on_repeat_last: None,
        on_abort: Some(bim_window_action("canvasEscape", None)),
    }
}

/// 📟️ The engagement of the addressed window: its status line, the problems line and, for every window that has gestures, the entry field.
pub fn engagements(snapshot: &ModelSnapshot, view_state: &ViewModel, plan_storey: Option<&str>, selected: usize, found: &SeverityCounts) -> HashMap<String, WindowEngagement> {
    let Some(window) = view_state.window_id.clone() else { return HashMap::new() };
    let text = status(snapshot, view_state, plan_storey, selected);
    let found = problems(bim_labels(view_state), found);
    let kind = addressed_kind(view_state);
    let input = matches!(kind, Some(plan::WINDOW_KIND_ID | world::WINDOW_KIND_ID | section::WINDOW_KIND_ID | sheet::WINDOW_KIND_ID)).then(|| entry(&window, bim_labels(view_state)));
    HashMap::from([(
        window.clone(),
        WindowEngagement {
            session_active: Some(kind == Some(world::WINDOW_KIND_ID) && crate::editor::bim::utilities::draws(crate::editor::bim::utilities::active(view_state))),
            options: None,
            input,
            control: None,
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: format!("{window}.status"), text }, WindowEngagementStatus { id: format!("{window}.problems"), text: found }]),
            possible_engagements: None,
        },
    )])
}
//#endregion 🔖️Status

//#region 🔖️Measures
fn view_action(field: &str) -> semio_framework_plugin::ActionDescriptor {
    bim_window_action("setView", Some(DslValue::object([("field".to_string(), DslValue::String(field.to_string()))])))
}

fn view_action_with(field: &str, value: &str) -> semio_framework_plugin::ActionDescriptor {
    bim_window_action("setView", Some(DslValue::object([("field".to_string(), DslValue::String(field.to_string())), ("value".to_string(), DslValue::String(value.to_string()))])))
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

fn view_items(snapshot: &ModelSnapshot, ids: Vec<String>) -> Vec<(String, String)> {
    ids.into_iter().map(|id| (snapshot.views.get(&id).map_or_else(|| id.clone(), |row| row.name.clone()), id)).map(|(name, id)| (id, name)).collect()
}

fn plan_measures(snapshot: &ModelSnapshot, config: &plan::config::BimPlanWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let view = plan::active_view(snapshot, config).unwrap_or_default();
    vec![select("bim.measure.plan.view", labels.measure_view.as_str(), &view, view_items(snapshot, plan::plan_views(snapshot)), "view")]
}

/// 📤️ One download toggle per export format: each runs the stepped, cancellable `exportModel` job.
fn export_toggles(labels: &BimLabels) -> Vec<WindowMeasure> {
    [("ifc2x3", labels.export_ifc2x3.as_str()), ("ifc4", labels.export_ifc4.as_str()), ("glb", labels.export_glb.as_str()), ("svg", labels.export_svg.as_str()), ("csv", labels.export_csv.as_str())]
        .into_iter()
        .map(|(format, label)| WindowMeasure::Toggle { id: format!("bim.measure.world.export.{format}"), icon_id: "download".into(), label: Some(label.to_string()), pressed: false, text: None, on_change: bim_window_action("exportModel", Some(DslValue::object([("format".to_string(), DslValue::String(format.to_string()))]))) })
        .collect()
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
    let phases = vec![
        ("all".to_string(), labels.phase_all.as_str().to_string()),
        ("existing".to_string(), labels.phase_existing.as_str().to_string()),
        ("new".to_string(), labels.phase_new.as_str().to_string()),
        ("demolished".to_string(), labels.phase_demolished.as_str().to_string()),
        ("temporary".to_string(), labels.phase_temporary.as_str().to_string()),
    ];
    let energy_modes = vec![("u_value".to_string(), labels.env_mode_u_value.as_str().to_string()), ("boundary".to_string(), labels.env_mode_boundary.as_str().to_string())];
    let axes = vec![("x".to_string(), labels.axis_x.as_str().to_string()), ("y".to_string(), labels.axis_y.as_str().to_string()), ("z".to_string(), labels.axis_z.as_str().to_string())];
    let storeys = storey_items(snapshot)
        .into_iter()
        .map(|(storey, name)| WindowMeasure::Toggle { id: format!("bim.measure.world.hide.{storey}"), icon_id: "eye-off".into(), label: Some(BimLabels::named(labels.measure_hide_storey, &name)), pressed: config.hidden_storeys.contains(&storey), text: None, on_change: view_action_with("hidden_storey", &storey) })
        .collect();
    vec![
        select("bim.measure.world.projection", labels.measure_projection.as_str(), &config.projection.kind, projection, "projection"),
        select("bim.measure.world.isolate", labels.measure_isolate.as_str(), &config.isolated_storey, isolate, "isolated_storey"),
        select("bim.measure.world.phase", labels.measure_view_phase.as_str(), world::view_phase(config).key(), phases, "view_phase"),
        WindowMeasure::measure_group("bim.measure.world.storeys", labels.measure_visible_storeys.as_str(), storeys),
        WindowMeasure::Toggle { id: "bim.measure.world.section".into(), icon_id: "scissors".into(), label: Some(labels.measure_section_enabled.as_str().to_string()), pressed: config.section_enabled, text: None, on_change: view_action("section_enabled") },
        select("bim.measure.world.section-axis", labels.measure_section_axis.as_str(), &config.section_axis, axes, "section_axis"),
        number("bim.measure.world.section-offset", labels.measure_section_offset.as_str(), config.section_offset, None, 0.1, "section_offset"),
        WindowMeasure::Toggle { id: "bim.measure.world.energy".into(), icon_id: "thermometer".into(), label: Some(labels.measure_env_overlay.as_str().to_string()), pressed: config.energy_overlay, text: Some(world::envelope::legend_text(labels, labels.locale().as_str().starts_with("de"), world::envelope::mode(config))), on_change: view_action("energy_overlay") },
        select("bim.measure.world.energy-mode", labels.measure_env_mode.as_str(), world::envelope::mode(config).key(), energy_modes, "energy_mode"),
        WindowMeasure::Toggle { id: "bim.measure.world.show-all".into(), icon_id: "eye".into(), label: Some(labels.clash_show_all.as_str().to_string()), pressed: !(config.isolated_elements.is_empty() && config.section_box.is_empty()), text: None, on_change: bim_window_action("viewClash", Some(DslValue::object([("first".to_string(), DslValue::String(String::new())), ("second".to_string(), DslValue::String(String::new())), ("mode".to_string(), DslValue::String("clear".to_string()))]))) },
        WindowMeasure::Toggle { id: "bim.measure.world.analyse".into(), icon_id: "refresh".into(), label: Some(labels.measure_analyse.as_str().to_string()), pressed: false, text: None, on_change: bim_window_action("analyseModel", None) },
        WindowMeasure::measure_group("bim.measure.world.export", labels.measure_export.as_str(), export_toggles(labels)),
    ]
}

fn section_measures(snapshot: &ModelSnapshot, config: &section::config::BimSectionWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let view = section::active_view(snapshot, config).unwrap_or_default();
    vec![select("bim.measure.section.view", labels.measure_view.as_str(), &view, view_items(snapshot, section::vertical_views(snapshot)), "view")]
}

fn schedule_measures(snapshot: &ModelSnapshot, config: &schedule::config::BimScheduleWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let mut items = vec![(String::new(), labels.sch_back.as_str().to_string())];
    items.extend(snapshot.schedules.iter().map(|(id, row)| (id.clone(), row.name.clone())));
    let mut measures = vec![select("bim.measure.schedule.shown", labels.kind_schedule.as_str(), &config.schedule, items, "schedule")];
    if snapshot.schedules.contains_key(&config.schedule) {
        measures.push(WindowMeasure::Toggle { id: "bim.measure.schedule.export".into(), icon_id: "download".into(), label: Some(labels.sch_export_csv.as_str().to_string()), pressed: false, text: None, on_change: bim_window_action("exportScheduleCsv", Some(DslValue::object([("id".to_string(), DslValue::String(config.schedule.clone()))]))) });
        measures.push(WindowMeasure::Toggle { id: "bim.measure.schedule.editing".into(), icon_id: "pencil".into(), label: Some(labels.sch_edit.as_str().to_string()), pressed: config.editing, text: None, on_change: view_action("editing") });
    }
    measures
}

fn sheet_measures(snapshot: &ModelSnapshot, config: &sheet::config::BimSheetWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let shown = sheet::active_sheet(snapshot, config).unwrap_or_default();
    let items = crate::editor::bim::entities::sheets::sheet_choices(snapshot, labels);
    let export = |format: &str, only: Option<&str>, label: &str, id: &str| {
        let mut args = vec![("format".to_string(), DslValue::String(format.to_string())), ("locale".to_string(), DslValue::String(labels.locale().as_str().to_string()))];
        if let Some(sheet) = only {
            args.push(("sheet".to_string(), DslValue::String(sheet.to_string())));
        }
        WindowMeasure::Toggle { id: format!("bim.measure.sheet.export.{id}"), icon_id: "download".into(), label: Some(label.to_string()), pressed: false, text: None, on_change: bim_window_action("exportSheets", Some(DslValue::object(args))) }
    };
    let mut measures = vec![select("bim.measure.sheet.shown", labels.kind_sheet.as_str(), &shown, items, "sheet")];
    if !shown.is_empty() {
        measures.push(WindowMeasure::measure_group("bim.measure.sheet.export", labels.measure_export_sheets.as_str(), vec![export("svg", Some(shown.as_str()), labels.export_sheet_svg.as_str(), "svg"), export("pdf", None, labels.export_sheets_pdf.as_str(), "pdf")]));
    }
    measures
}

/// 🎚️ The measures of the addressed window, from that window's own config.
pub fn measures<C>(snapshot: &ModelSnapshot, cfg: &ConfigView<'_, C>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
    let (Some(window), Some(kind)) = (view_state.window_id.clone(), addressed_kind(view_state)) else { return HashMap::new() };
    let labels = bim_labels(view_state);
    let measures = match kind {
        plan::WINDOW_KIND_ID => plan_measures(snapshot, &plan::config::current(cfg), labels),
        world::WINDOW_KIND_ID => world_measures(snapshot, &world::config::current(cfg), labels),
        section::WINDOW_KIND_ID => section_measures(snapshot, &section::config::current(cfg), labels),
        schedule::WINDOW_KIND_ID => schedule_measures(snapshot, &schedule::config::current(cfg), labels),
        sheet::WINDOW_KIND_ID => sheet_measures(snapshot, &sheet::config::current(cfg), labels),
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
