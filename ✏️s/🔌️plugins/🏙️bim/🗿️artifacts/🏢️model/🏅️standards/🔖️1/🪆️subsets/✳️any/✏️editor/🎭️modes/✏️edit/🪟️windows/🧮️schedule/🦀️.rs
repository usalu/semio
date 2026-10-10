//! 🧮️ BIM schedule window: a user-defined schedule as a `Table` surface in the `elements` interaction domain, so a row pick selects its element everywhere. The window is bound to one schedule by its
//! config. Without one it lists the schedules of the model with the presets that create new ones; with one it shows the inferred table (items, group subtotals, grand total) or, in editing mode, the
//! definition as a second table: the column picker, sort keys, filters, grouping and scope as rows whose buttons and inputs emit the `editSchedule` command. The table stores and computes nothing:
//! every cell is read from the `schedules` inference.

#[path = "🎚️config/🦀️.rs"]
pub mod config;
#[path = "✏️edit/🦀️.rs"]
pub mod edit;
#[path = "🏷️vocabulary/🦀️.rs"]
pub mod vocabulary;

use self::config::BimScheduleWindowConfig;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::bim_window_action;
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::inferences::schedules::{RowKind, ScheduleCell};
use crate::{ModelInference, ModelSnapshot, Phase, Schedule, ScheduleField, ScheduleKey, ScheduleOp};
use semio_framework_plugin::DslValue;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::TableCell;
use semio_framework_plugin::UiTreeActionPlacement;
use semio_framework_plugin::UiTreeItemAction;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_locale::{Label, LocalizedLabel};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-edit-schedule";
pub const BODY_KEY: &str = "bim.edit.schedule";
const SURFACE_ID: &str = "bim.edit.schedule/table";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: BimLabels::localized(|labels| labels.window_schedule),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "table".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: vec![semio_framework_plugin::InteractionRef::new(BIM_ELEMENT_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Cells
fn text(value: impl Into<String>) -> TableCell {
    TableCell::Text { value: value.into() }
}

fn number(value: f64) -> TableCell {
    TableCell::Number { value }
}

fn object<'a>(entries: impl IntoIterator<Item = (&'a str, &'a str)>) -> DslValue {
    DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), DslValue::String(value.to_string()))))
}

fn button(icon: &str, label: &str, action: &str, args: DslValue) -> UiTreeItemAction {
    UiTreeItemAction { icon_id: icon.into(), label: Some(Label::data(label.to_string())), action: bim_window_action(action, Some(args)), placement: Some(UiTreeActionPlacement::Row), disabled: false, reason: None }
}

fn edit_button(id: &str, icon: &str, label: &str, part: &str, op: &str, key: &str, value: &str) -> UiTreeItemAction {
    let strings = [("id", id), ("part", part), ("op", op), ("key", key), ("value", value)];
    button(icon, label, "editSchedule", object(strings))
}

fn view_button(icon: &str, label: &str, field: &str, value: &str) -> UiTreeItemAction {
    button(icon, label, "setView", object([("field", field), ("value", value)]))
}

fn row(id: &str, cells: Vec<(String, TableCell)>) -> DslValue {
    let mut entries = vec![("id".to_string(), DslValue::String(id.to_string()))];
    entries.extend(cells.into_iter().map(|(column, cell)| (column, semio_framework_value::ToValue::to_value(&cell))));
    DslValue::object(entries)
}

fn column_value(id: &str, label: &str) -> DslValue {
    DslValue::object([("id".to_string(), DslValue::String(id.into())), ("label".to_string(), DslValue::String(label.into())), ("sortable".to_string(), DslValue::Bool(false))])
}

fn table(columns: Vec<DslValue>, rows: Vec<DslValue>, granularity: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut scene = semio_framework_plugin::TableScene::base(semio_framework_pack_json::to_json_string(&DslValue::Array(columns)), semio_framework_pack_json::to_json_string(&DslValue::Array(rows)));
    if let Some(granularity) = granularity {
        scene.domain_id = Some(BIM_ELEMENT_DOMAIN.into());
        scene.domain_granularity_id = Some(granularity.into());
    }
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Table, &scene)
}
//#endregion 🔖️Cells

//#region 🔖️List
/// 📋️ The list of the schedules of the model and the presets that create one: a row per schedule (name, category, number of rows) with show and delete buttons, then a row per preset with a create button.
pub fn list_rows(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels) -> Vec<DslValue> {
    let mut rows: Vec<DslValue> = snapshot
        .schedules
        .iter()
        .map(|(id, schedule)| {
            let items = inference.schedules.get(id).map_or(0, |found| found.items);
            row(
                id,
                vec![
                    ("name".into(), text(&schedule.name)),
                    ("detail".into(), text(vocabulary::category_label(labels, schedule.category))),
                    ("rows".into(), number(f64::from(items))),
                    ("actions".into(), TableCell::Buttons { buttons: vec![view_button("table", labels.sch_show.as_str(), "schedule", id), button("download", labels.sch_export_csv.as_str(), "exportScheduleCsv", object([("id", id.as_str())])), button("trash", &BimLabels::named(labels.act_delete_named, &schedule.name), "deleteSelection", DslValue::object([("ids".to_string(), DslValue::Array(vec![DslValue::String(id.clone())]))]))] }),
                ],
            )
        })
        .collect();
    for key in edit::presets() {
        let name = preset_label(labels, key);
        let args = object([("kind", "schedule"), ("parent", key), ("name", name.as_str())]);
        rows.push(row(&format!("preset:{key}"), vec![("name".into(), text(&name)), ("detail".into(), text("")), ("rows".into(), text("")), ("actions".into(), TableCell::Buttons { buttons: vec![button("plus", &BimLabels::named(labels.act_new_named, &name), "createEntity", args)] })]));
    }
    rows
}

/// 🏷️ The localized name of the preset `key`.
pub fn preset_label(labels: &BimLabels, key: &str) -> String {
    match key {
        "door" => labels.sp_door,
        "window" => labels.sp_window,
        "room" => labels.sp_room,
        "finish" => labels.sp_finish,
        "envelope" => labels.sp_envelope,
        "wall" => labels.sp_wall,
        _ => labels.sp_material,
    }
    .as_str()
    .to_string()
}

fn list_surface(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let columns = vec![column_value("name", labels.column_name.as_str()), column_value("detail", labels.field_category.as_str()), column_value("rows", labels.sch_rows.as_str()), column_value("actions", labels.sch_actions.as_str())];
    table(columns, list_rows(snapshot, inference, labels), None)
}
//#endregion 🔖️List

//#region 🔖️Table
/// 🧾️ The columns and rows of the inferred table of `schedule`, texts localized: an item row is keyed by its element id (a pick selects it), a group row `group:<n>`, the grand total `total`.
pub fn table_parts(schedule: &Schedule, inference: &ModelInference, id: &str, labels: &BimLabels) -> (Vec<DslValue>, Vec<DslValue>) {
    let columns: Vec<DslValue> = schedule.columns.iter().enumerate().map(|(index, column)| column_value(&format!("c{index}"), &vocabulary::heading(labels, column))).collect();
    let Some(found) = inference.schedules.get(id) else { return (columns, Vec::new()) };
    let rows = found
        .rows
        .iter()
        .enumerate()
        .map(|(position, found_row)| {
            let row_id = match found_row.kind {
                RowKind::Item => found_row.elements.first().cloned().unwrap_or_else(|| format!("item:{position}")),
                RowKind::Group => format!("group:{position}"),
                RowKind::Total => "total".to_string(),
            };
            let mut shown_total = found_row.kind != RowKind::Total;
            let cells = found_row
                .cells
                .iter()
                .zip(&schedule.columns)
                .enumerate()
                .map(|(index, (cell, column))| {
                    let cell = match cell {
                        ScheduleCell::Number { value } => number(*value),
                        ScheduleCell::Empty if !shown_total => {
                            shown_total = true;
                            text(labels.row_total.as_str())
                        }
                        other => text(vocabulary::cell_text(labels, &column.key, other)),
                    };
                    (format!("c{index}"), cell)
                })
                .collect();
            row(&row_id, cells)
        })
        .collect();
    (columns, rows)
}

/// 🎯️ The granularity a row of the table is picked under: the kind of the elements its item rows stand for, none when they are not placed elements (a material take-off picks nothing).
pub fn row_kind(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str) -> Option<String> {
    let found = inference.schedules.get(id)?;
    let element = found.rows.iter().filter(|row| row.kind == RowKind::Item).find_map(|row| row.elements.first())?;
    crate::editor::bim::entities::kind_holding(snapshot, element).filter(|row| !row.library).map(|row| row.kind.to_string())
}

fn table_surface(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimScheduleWindowConfig, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(schedule) = snapshot.schedules.get(&config.schedule) else { return list_surface(snapshot, inference, labels) };
    let (columns, rows) = table_parts(schedule, inference, &config.schedule, labels);
    table(columns, rows, row_kind(snapshot, inference, &config.schedule).as_deref())
}
//#endregion 🔖️Table

//#region 🔖️Editor
fn push(rows: &mut Vec<DslValue>, part: &str, item: String, value: TableCell, buttons: Vec<UiTreeItemAction>) {
    let position = rows.len();
    rows.push(editor_row(position, part, item, value, buttons));
}

fn editor_row(position: usize, part: &str, item: String, value: TableCell, buttons: Vec<UiTreeItemAction>) -> DslValue {
    row(&format!("edit:{position}"), vec![("part".into(), text(part)), ("item".into(), text(item)), ("value".into(), value), ("actions".into(), TableCell::Buttons { buttons })])
}

fn field_pool(schedule: &Schedule) -> Vec<ScheduleKey> {
    ScheduleField::ALL.into_iter().filter(|field| field.available(schedule.category)).map(ScheduleKey::field).collect()
}

fn property_keys(schedule: &Schedule) -> Vec<ScheduleKey> {
    schedule.columns.iter().map(|column| &column.key).chain(schedule.sort.iter().map(|sort| &sort.key)).chain(schedule.filter.iter().map(|filter| &filter.key)).chain(schedule.group.iter().map(|group| &group.key)).filter(|key| matches!(key, ScheduleKey::Property { .. })).cloned().collect()
}

/// 🧾️ The rows of the definition editor of schedule `id`: name, category, the column picker (every available key once, with add or remove, move and sum buttons), the sort keys, the filters, the grouping, the
/// itemization and the storey and phase scope.
pub fn editor_rows(snapshot: &ModelSnapshot, id: &str, labels: &BimLabels) -> Vec<DslValue> {
    let Some(schedule) = snapshot.schedules.get(id) else { return Vec::new() };
    let mut rows: Vec<DslValue> = Vec::new();
    let name_action = bim_window_action("editSchedule", Some(object([("id", id), ("part", "name"), ("op", "set"), ("key", "")])));
    push(&mut rows, labels.sf_name.as_str(), String::new(), TableCell::EditableText { value: schedule.name.clone(), action: name_action }, Vec::new());
    let categories: Vec<UiTreeItemAction> = crate::ScheduleCategory::ALL.into_iter().filter(|category| *category != schedule.category).map(|category| edit_button(id, "table", &vocabulary::category_label(labels, category), "category", "set", "", category.token())).collect();
    push(&mut rows, labels.field_category.as_str(), vocabulary::category_label(labels, schedule.category), text(schedule.category.token()), categories);
    let mut keys = field_pool(schedule);
    keys.extend(property_keys(schedule));
    keys.dedup();
    for key in &keys {
        let token = key.token();
        let shown = schedule.columns.iter().position(|column| column.key == *key);
        let mut buttons = Vec::new();
        match shown {
            Some(index) => {
                buttons.push(edit_button(id, "minus", labels.act_remove.as_str(), "column", "remove", &token, ""));
                if index > 0 {
                    buttons.push(edit_button(id, "arrow-up", labels.act_up.as_str(), "column", "up", &token, ""));
                }
                if index + 1 < schedule.columns.len() {
                    buttons.push(edit_button(id, "arrow-down", labels.act_down.as_str(), "column", "down", &token, ""));
                }
                buttons.push(edit_button(id, "sigma", labels.act_total.as_str(), "column", "total", &token, ""));
            }
            None => buttons.push(edit_button(id, "plus", labels.act_add.as_str(), "column", "add", &token, "")),
        }
        let state = shown.map_or_else(String::new, |index| format!("{} {}{}", labels.sch_shown.as_str(), index + 1, if schedule.columns[index].total { " Σ" } else { "" }));
        push(&mut rows, labels.field_columns.as_str(), vocabulary::key_label(labels, key), text(state), buttons);
    }
    for (index, sort) in schedule.sort.iter().enumerate() {
        let token = sort.key.token();
        let mut buttons = vec![edit_button(id, "minus", labels.act_remove.as_str(), "sort", "remove", &token, ""), edit_button(id, "arrow-up-down", labels.act_descending.as_str(), "sort", "descending", &token, "")];
        if index > 0 {
            buttons.push(edit_button(id, "arrow-up", labels.act_up.as_str(), "sort", "up", &token, ""));
        }
        if index + 1 < schedule.sort.len() {
            buttons.push(edit_button(id, "arrow-down", labels.act_down.as_str(), "sort", "down", &token, ""));
        }
        push(&mut rows, labels.field_sort.as_str(), vocabulary::key_label(labels, &sort.key), text(if sort.descending { "↓" } else { "↑" }), buttons);
    }
    for (index, group) in schedule.group.iter().enumerate() {
        let token = group.key.token();
        let mut buttons = vec![edit_button(id, "minus", labels.act_remove.as_str(), "group", "remove", &token, "")];
        if index > 0 {
            buttons.push(edit_button(id, "arrow-up", labels.act_up.as_str(), "group", "up", &token, ""));
        }
        if index + 1 < schedule.group.len() {
            buttons.push(edit_button(id, "arrow-down", labels.act_down.as_str(), "group", "down", &token, ""));
        }
        push(&mut rows, labels.field_group.as_str(), vocabulary::key_label(labels, &group.key), text(format!("{}", index + 1)), buttons);
    }
    for (index, filter) in schedule.filter.iter().enumerate() {
        let position = index.to_string();
        let buttons = vec![edit_button(id, "minus", labels.act_remove.as_str(), "filter", "remove", &position, ""), edit_button(id, "refresh-cw", labels.act_next_comparison.as_str(), "filter", "op", &position, "")];
        let value = if filter.op == ScheduleOp::Empty || filter.op == ScheduleOp::NotEmpty {
            text(vocabulary::op_label(labels, filter.op))
        } else {
            TableCell::EditableText { value: filter.value.clone(), action: bim_window_action("editSchedule", Some(object([("id", id), ("part", "filter"), ("op", "value"), ("key", position.as_str()), ("value", "")]))) }
        };
        push(&mut rows, labels.field_filter.as_str(), format!("{} · {}", vocabulary::key_label(labels, &filter.key), vocabulary::op_label(labels, filter.op)), value, buttons);
    }
    let pool: Vec<ScheduleKey> = keys.iter().filter(|key| !schedule.sort.iter().any(|sort| sort.key == **key)).cloned().collect();
    let sort_adds: Vec<UiTreeItemAction> = pool.iter().take(6).map(|key| edit_button(id, "plus", &format!("{} {}", labels.field_sort.as_str(), vocabulary::key_label(labels, key)), "sort", "add", &key.token(), "")).collect();
    push(&mut rows, labels.field_sort.as_str(), labels.act_add.as_str().to_string(), text(""), sort_adds);
    let group_adds: Vec<UiTreeItemAction> = keys.iter().filter(|key| !schedule.group.iter().any(|group| group.key == **key)).take(6).map(|key| edit_button(id, "plus", &format!("{} {}", labels.field_group.as_str(), vocabulary::key_label(labels, key)), "group", "add", &key.token(), "")).collect();
    push(&mut rows, labels.field_group.as_str(), labels.act_add.as_str().to_string(), text(""), group_adds);
    let filter_adds: Vec<UiTreeItemAction> = keys.iter().take(6).map(|key| edit_button(id, "plus", &format!("{} {}", labels.field_filter.as_str(), vocabulary::key_label(labels, key)), "filter", "add", &key.token(), "")).collect();
    push(&mut rows, labels.field_filter.as_str(), labels.act_add.as_str().to_string(), text(""), filter_adds);
    push(&mut rows, labels.field_itemize.as_str(), String::new(), text(schedule.itemize.to_string()), vec![edit_button(id, "list", labels.act_toggle.as_str(), "itemize", "toggle", "", "")]);
    for (storey_id, storey) in &snapshot.storeys {
        let on = schedule.storeys.contains(storey_id);
        push(&mut rows, labels.field_scope_storeys.as_str(), storey.name.clone(), text(on.to_string()), vec![edit_button(id, "layers", labels.act_toggle.as_str(), "storey", "toggle", storey_id, "")]);
    }
    for phase in [Phase::Existing, Phase::New, Phase::Demolished, Phase::Temporary] {
        let token = format!("{phase:?}");
        push(&mut rows, labels.field_scope_phases.as_str(), vocabulary::phase_label(labels, phase), text(schedule.phases.contains(&phase).to_string()), vec![edit_button(id, "clock", labels.act_toggle.as_str(), "phase", "toggle", &token, "")]);
    }
    rows
}

fn editor_surface(snapshot: &ModelSnapshot, config: &BimScheduleWindowConfig, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let columns = vec![column_value("part", labels.sch_part.as_str()), column_value("item", labels.sch_item.as_str()), column_value("value", labels.sch_value.as_str()), column_value("actions", labels.sch_actions.as_str())];
    table(columns, editor_rows(snapshot, &config.schedule, labels), None)
}
//#endregion 🔖️Editor

//#region 🔖️Render
/// 🧮️ Renders the schedule window: the list without a chosen schedule, else its table or its definition editor.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimScheduleWindowConfig, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if !snapshot.schedules.contains_key(&config.schedule) {
        return list_surface(snapshot, inference, labels);
    }
    if config.editing {
        editor_surface(snapshot, config, labels)
    } else {
        table_surface(snapshot, inference, config, labels)
    }
}

//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
