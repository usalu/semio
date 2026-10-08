//! 🔍️ BIM properties panel: the authored parameters of the selected entities as one field table, plus the values inferred for them. An authored row with a `set-*` mutation is an input
//! that commits through `setField`; one without is read-only. When several entities of one kind are selected a row shows the common value or `Mixed`, and a commit sets all of them
//! at once. Inferred rows (elevation, height, length, area, volume) are read-only text: nothing derived is ever stored or edited.

use crate::editor::bim::entities::{kind_holding, EntityKind, FieldRow};
use crate::editor::bim::kit::{bim_action, ui_capacity_error, ui_label, ui_text, ui_value_list, ui_value_map, ui_value_text};
use crate::editor::bim::terminology::BimLabels;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::ui_node_list;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_framework_ui_contract as ui;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const BODY_KEY: &str = "bim.edit.properties";
const ROOT: &str = "bim-properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.panel_properties.as_str(), BimLabels::NATIVE_DE.panel_properties.as_str()),
        group: PanelGroup::Details,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Selection
/// 🎯️ The ids the panel shows: the element selection, else the library selection; only the ids of the first selected id's kind, so one table describes them all.
pub fn subject<'a>(snapshot: &ModelSnapshot, elements: &'a [String], library: &'a [String]) -> Option<(&'static EntityKind, Vec<&'a str>)> {
    let ids = if elements.is_empty() { library } else { elements };
    let row = ids.iter().find_map(|id| kind_holding(snapshot, id))?;
    Some((row, ids.iter().filter(|id| kind_holding(snapshot, id).is_some_and(|candidate| candidate.kind == row.kind)).map(String::as_str).collect()))
}

/// 🧮️ The value of a field across the subjects: the common value, or `None` when they differ.
fn common(snapshot: &ModelSnapshot, read: fn(&ModelSnapshot, &str) -> Option<String>, ids: &[&str]) -> Option<Option<String>> {
    let mut values = ids.iter().map(|id| read(snapshot, id));
    let first = values.next()?;
    Some(if values.all(|value| value == first) { first } else { None })
}
//#endregion 🔖️Selection

//#region 🔖️Rows
fn control_row(row_id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    ui::tree_item(ui_label(label)?).try_id(row_id).map_err(|_| ui_capacity_error())?.try_child(control).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())
}

fn authored_row(snapshot: &ModelSnapshot, field: &FieldRow, ids: &[&str], labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    let id = format!("{ROOT}.{}", field.key);
    let label = (field.label)(labels);
    let value = common(snapshot, field.read, ids).flatten();
    if field.write.is_none() {
        let shown = value.unwrap_or_else(|| if ids.len() > 1 { labels.field_mixed.as_str().to_string() } else { String::new() });
        return tree_item_desc(&id, ui_label(label.as_str())?, Some(shown));
    }
    let args = ui_value_map([("field", ui_value_text(field.key)?), ("ids", ui_value_list(ids.iter().map(|id| ui_value_text(id)).collect::<UiAssemblyResult<Vec<_>>>()?)?)])?;
    let (action, args) = bim_action("setField", Some(args))?;
    let args = args.ok_or_else(ui_capacity_error)?;
    let mut input = ui::input(field.input).value(ui_text(value.as_deref().unwrap_or(""))?).try_id(format!("{id}.input")).map_err(|_| ui_capacity_error())?.try_label(label.as_str()).map_err(|_| ui_capacity_error())?.commit(ui_text("blur")?);
    if value.is_none() {
        input = input.placeholder(ui::Label(ui_text(labels.field_mixed.as_str())?));
    }
    if field.input == InputKind::Number {
        input = input.step(0.1);
    }
    let control = input.try_on_with(Trigger::Commit, action, args).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())?;
    control_row(&id, label.as_str(), control)
}

fn inferred_rows(snapshot: &ModelSnapshot, inference: &ModelInference, row: &EntityKind, ids: &[&str], labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let Some(id) = ids.first().filter(|_| ids.len() == 1) else { return Vec::new() };
    row.inferred
        .iter()
        .filter_map(|inferred| (inferred.read)(snapshot, inference, id).map(|value| tree_item_desc(format!("{ROOT}.inferred.{}", inferred.key), ui_label((inferred.label)(labels).as_str())?, Some(value))))
        .collect()
}

fn summary_rows(snapshot: &ModelSnapshot, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let mut rows = vec![
        ui_label(labels.empty_properties.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.summary.empty"), label, None)),
        ui_label(labels.summary_schema.as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.summary.schema"), label, Some(snapshot.schema.clone()))),
    ];
    rows.extend(
        crate::editor::bim::entities::ENTITIES
            .iter()
            .filter(|row| !(row.ids)(snapshot).is_empty())
            .map(|row| ui_label((row.group)(labels).as_str()).and_then(|label| tree_item_desc(format!("{ROOT}.summary.{}", row.kind), label, Some((row.ids)(snapshot).len().to_string())))),
    );
    rows
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🔍️ Renders the properties panel for the given selections.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, elements: &[String], library: &[String], labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    let builder = PanelTreeBuilder::new(ROOT)?;
    let Some((row, ids)) = subject(snapshot, elements, library) else {
        return builder.section(format!("{ROOT}.summary"), Some(ui_label(labels.section_summary.as_str())?), true, ui_node_list(summary_rows(snapshot, labels))?)?.build();
    };
    let heading = format!("{} {}", (row.label)(labels).as_str(), if ids.len() > 1 { format!("× {}", ids.len()) } else { (row.name)(snapshot, ids[0]).unwrap_or_default() });
    let authored = ui_node_list(row.fields.iter().map(|field| authored_row(snapshot, field, &ids, labels)))?;
    let mut builder = builder.section(format!("{ROOT}.authored"), Some(ui_label(&heading)?), true, authored)?;
    let inferred = inferred_rows(snapshot, inference, row, &ids, labels);
    if !inferred.is_empty() {
        builder = builder.section(format!("{ROOT}.inferred"), Some(ui_label(labels.section_inferred.as_str())?), true, ui_node_list(inferred)?)?;
    }
    builder.build()
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
