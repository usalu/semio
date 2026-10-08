//! 🧮️ BIM schedule window: the quantity take-off of the model as a `Table` surface, one row per element that has quantities followed by one total row per element kind, in the
//! `elements` interaction domain so a row pick selects the element everywhere. Every number is read from the `quantities` inference; the table stores and computes nothing.

use crate::editor::bim::entities::{kind_holding, kind_of};
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::terminology::BimLabels;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::DslValue;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::TableCell;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_locale::LocalizedLabel;

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
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.window_schedule.as_str(), BimLabels::NATIVE_DE.window_schedule.as_str()),
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

//#region 🔖️Rows
/// 🧾️ One schedule row: an element, or the total of one element kind.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub storey: String,
    pub type_name: String,
    pub count: u32,
    pub length: f64,
    pub area: f64,
    pub volume: f64,
    pub mass: f64,
    pub total: bool,
}

/// 🏷️ The entity kind a quantity kind belongs to: windows, doors and voids are all openings.
fn entity_kind(quantity: &str) -> &str {
    match quantity {
        "window" | "door" | "void" => "opening",
        other => other,
    }
}

fn kind_label(labels: &BimLabels, quantity: &str) -> String {
    kind_of(entity_kind(quantity)).map_or_else(|| quantity.to_string(), |row| (row.label)(labels).as_str().to_string())
}

fn type_name(snapshot: &ModelSnapshot, type_id: &str) -> String {
    kind_holding(snapshot, type_id).and_then(|row| (row.name)(snapshot, type_id)).unwrap_or_else(|| type_id.to_string())
}

/// 🧮️ The rows of the schedule: every element in id order, then the total of each kind in kind order.
pub fn rows(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels) -> Vec<Row> {
    let quantities = &inference.quantities;
    let mut rows: Vec<Row> = quantities
        .elements
        .iter()
        .map(|(id, quantity)| Row {
            id: id.clone(),
            name: kind_holding(snapshot, id).and_then(|row| (row.name)(snapshot, id)).unwrap_or_else(|| id.clone()),
            kind: kind_label(labels, quantity.kind.key()),
            storey: snapshot.storeys.get(&quantity.storey).map_or_else(|| quantity.storey.clone(), |storey| storey.name.clone()),
            type_name: type_name(snapshot, &quantity.type_id),
            count: quantity.count,
            length: quantity.length,
            area: quantity.area(),
            volume: quantity.net_volume,
            mass: quantity.mass,
            total: false,
        })
        .collect();
    rows.extend(quantities.project.kinds.iter().map(|(kind, totals)| Row {
        id: format!("total:{kind}"),
        name: format!("{} · {}", labels.row_total.as_str(), kind_label(labels, kind)),
        kind: kind_label(labels, kind),
        storey: String::new(),
        type_name: String::new(),
        count: totals.count,
        length: totals.length,
        area: totals.area,
        volume: totals.volume,
        mass: totals.mass,
        total: true,
    }));
    rows
}

fn text(value: &str) -> TableCell {
    TableCell::Text { value: value.to_string() }
}

fn number(value: f64) -> TableCell {
    TableCell::Number { value }
}

fn row_value(row: &Row) -> DslValue {
    let mut entries = vec![("id".to_string(), DslValue::String(row.id.clone()))];
    let cells = [
        ("name", text(&row.name)),
        ("kind", text(&row.kind)),
        ("storey", text(&row.storey)),
        ("type", text(&row.type_name)),
        ("count", number(f64::from(row.count))),
        ("length", number(row.length)),
        ("area", number(row.area)),
        ("volume", number(row.volume)),
        ("mass", number(row.mass)),
    ];
    entries.extend(cells.into_iter().map(|(column, cell)| (column.to_string(), semio_framework_value::ToValue::to_value(&cell))));
    DslValue::object(entries)
}

fn column_value(id: &str, label: &str) -> DslValue {
    DslValue::object([("id".to_string(), DslValue::String(id.into())), ("label".to_string(), DslValue::String(label.into())), ("sortable".to_string(), DslValue::Bool(true))])
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🧮️ Renders the schedule table.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let columns = DslValue::Array(vec![
        column_value("name", labels.column_name.as_str()),
        column_value("kind", labels.field_kind.as_str()),
        column_value("storey", labels.column_storey.as_str()),
        column_value("type", labels.column_type.as_str()),
        column_value("count", labels.column_count.as_str()),
        column_value("length", labels.column_length.as_str()),
        column_value("area", labels.column_area.as_str()),
        column_value("volume", labels.column_volume.as_str()),
        column_value("mass", labels.column_mass.as_str()),
    ]);
    let rows = DslValue::Array(rows(snapshot, inference, labels).iter().map(row_value).collect());
    let mut scene = semio_framework_plugin::TableScene::base(semio_framework_pack_json::to_json_string(&columns), semio_framework_pack_json::to_json_string(&rows));
    scene.domain_id = Some(BIM_ELEMENT_DOMAIN.into());
    scene.domain_granularity_id = Some("wall".into());
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Table, &scene)
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
