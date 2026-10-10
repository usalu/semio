//! 🔍️ BIM properties panel: the authored parameters of the selected entities as one field table, plus the values inferred for them. An authored row with a `set-*` mutation is an input
//! that commits through `setField`; one without is read-only. When several entities of one kind are selected a row shows the common value or `Mixed`, and a commit sets all of them
//! at once. Inferred rows (elevation, height, length, area, volume) are read-only text: nothing derived is ever stored or edited.
//!
//! Beyond the kind's own parameters a placed element has its classification (`System Code Title`, empty removes it), its property sets (one input per property, a remove row each, and an
//! add row that takes `Set.Property = value`), a place-at row that puts it where the author types, and a wall has its flip and split rows. With nothing selected the panel edits the
//! project record next to the per-kind summary. Every one of these is reachable from the keyboard: they are labelled inputs and rows, not gestures.

use crate::editor::bim::commands::attach_walls;
use crate::editor::bim::entities::wall_sweeps::top_attach_text;
use crate::editor::bim::entities::{fields_of, kind_holding, property_text, EntityKind, FieldRow, PROJECT_FIELDS, PROJECT_ID};
use crate::editor::bim::kit::{bim_action, tree_item_with_icon, ui_capacity_error, ui_label, ui_text, ui_value_list, ui_value_map, ui_value_text};
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
use semio_framework_plugin::ActionId;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_framework_ui_contract as ui;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;

#[path = "🏷️data/🦀️.rs"]
mod data;

#[path = "🎚️overrides/🦀️.rs"]
pub mod overrides;

//#region 🔖️Constants
pub const BODY_KEY: &str = "bim.edit.properties";
const ROOT: &str = "bim-properties";
/// 🔽️ A reference row with fewer choices than this is a dropdown of them; a longer library is typed by id (a dropdown holds at most 32 items, one of them is "none").
const SELECT_ITEMS: usize = 31;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: BimLabels::localized(|labels| labels.panel_properties),
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
    let choices = field.choices.map(|choices| choices(snapshot, labels)).filter(|choices| choices.len() < SELECT_ITEMS);
    if let Some(choices) = choices {
        let mut select = ui::select(ui_text(value.as_deref().unwrap_or(""))?).try_id(format!("{id}.input")).map_err(|_| ui_capacity_error())?.try_label(label.as_str()).map_err(|_| ui_capacity_error())?;
        if value.is_none() {
            select = select.placeholder(ui::Label(ui_text(labels.field_mixed.as_str())?));
        }
        select = select.try_item(ui_text("")?, ui::Label(ui_text(labels.choice_none.as_str())?)).map_err(|_| ui_capacity_error())?;
        for (choice, name) in &choices {
            select = select.try_item(ui_text(choice)?, ui::Label(ui_text(&format!("{name} ({choice})"))?)).map_err(|_| ui_capacity_error())?;
        }
        let control = select.try_on_with(Trigger::Change, action, args).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())?;
        return control_row(&id, label.as_str(), control);
    }
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

//#region 🔖️Actions
fn ids_args(ids: &[&str]) -> UiAssemblyResult<UiValue> {
    ui_value_map([("ids", ui_value_list(ids.iter().map(|id| ui_value_text(id)).collect::<UiAssemblyResult<Vec<_>>>()?)?)])
}

pub fn input_row(id: &str, label: &str, value: &str, placeholder: Option<&str>, action: UiAssemblyResult<(ActionId, Option<UiValue>)>) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = action?;
    let args = args.ok_or_else(ui_capacity_error)?;
    let mut input = ui::input(InputKind::Text).value(ui_text(value)?).try_id(format!("{id}.input")).map_err(|_| ui_capacity_error())?.try_label(label).map_err(|_| ui_capacity_error())?.commit(ui_text("blur")?);
    if let Some(placeholder) = placeholder {
        input = input.placeholder(ui::Label(ui_text(placeholder)?));
    }
    let control = input.try_on_with(Trigger::Commit, action, args).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())?;
    control_row(id, label, control)
}

/// 🧱️ The flip and split rows of a wall selection, the attach row when the selection also holds a roof, slab or ceiling, and the row that frees an attached top.
fn wall_rows(snapshot: &ModelSnapshot, ids: &[&str], selected: &[String], labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    if !ids.iter().all(|id| snapshot.walls.contains_key(*id)) {
        return Vec::new();
    }
    let mut rows = vec![
        ids_args(ids).and_then(|args| tree_item_with_icon(format!("{ROOT}.action.flip"), Label::data(labels.action_flip_wall.as_str().to_string()), "flip-horizontal", bim_action("flipWalls", Some(args)))),
        ids_args(ids).and_then(|args| input_row(&format!("{ROOT}.action.split"), labels.action_split_wall_at.as_str(), "0.5", Some("0.5"), bim_action("splitWall", Some(args)))),
    ];
    if let Some(target) = attach_walls::target_of(snapshot, "", &[], selected) {
        let args = || ui_value_map([("ids", ui_value_list(ids.iter().map(|id| ui_value_text(id)).collect::<UiAssemblyResult<Vec<_>>>()?)?), ("target", ui_value_text(target)?)]);
        rows.push(args().and_then(|args| tree_item_with_icon(format!("{ROOT}.action.attach"), Label::data(labels.action_attach_walls.as_str().to_string()), "arrow-up-to-line", bim_action("attachWalls", Some(args)))));
    }
    if ids.iter().any(|id| top_attach_text(snapshot, id).is_some_and(|attached| !attached.is_empty())) {
        let args = || ui_value_map([("field", ui_value_text("top_attach")?), ("value", ui_value_text("")?), ("ids", ui_value_list(ids.iter().map(|id| ui_value_text(id)).collect::<UiAssemblyResult<Vec<_>>>()?)?)]);
        rows.push(args().and_then(|args| tree_item_with_icon(format!("{ROOT}.action.free-top"), Label::data(labels.action_free_top.as_str().to_string()), "unlink", bim_action("setField", Some(args)))));
    }
    rows
}

/// 📍️ The place-at row of a placed selection: the reference point of the first element, to be typed over.
fn place_rows(snapshot: &ModelSnapshot, ids: &[&str], labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let reference = ids.first().and_then(|id| crate::mutations::elements::placement(snapshot, id)).and_then(|placement| crate::editor::bim::commands::place_elements::reference_of(&placement));
    let Some(reference) = reference else { return Vec::new() };
    vec![ids_args(ids).and_then(|args| input_row(&format!("{ROOT}.action.place"), labels.action_place_at.as_str(), &format!("{}, {}", reference.x, reference.y), Some("0, 0"), bim_action("placeElements", Some(args))))]
}
/// 🌡️ The rows of a selection of spaces that state conditions: remove them, and with several spaces selected copy the first one's conditions to the others.
fn conditions_rows(snapshot: &ModelSnapshot, ids: &[&str], labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    if ids.is_empty() || !ids.iter().all(|id| snapshot.spaces.contains_key(*id)) || !ids.iter().any(|id| snapshot.space_conditions.contains_key(*id)) {
        return Vec::new();
    }
    let mut rows = vec![ids_args(ids).and_then(|args| tree_item_with_icon(format!("{ROOT}.action.clear-conditions"), Label::data(labels.action_clear_conditions.as_str().to_string()), "trash", bim_action("clearConditions", Some(args))))];
    if ids.len() > 1 {
        rows.push(ids_args(ids).and_then(|args| tree_item_with_icon(format!("{ROOT}.action.copy-conditions"), Label::data(labels.action_copy_conditions.as_str().to_string()), "copy", bim_action("applyConditions", Some(args)))));
    }
    rows
}
//#endregion 🔖️Actions

//#region 🔖️PropertySets
/// 🧾️ The input and the remove row of one property of one element.
fn property_pair(id: &str, pset: &str, name: &str, value: &crate::PropertyValue, labels: &BimLabels) -> UiAssemblyResult<Vec<BuiltNode>> {
    let row_id = format!("{ROOT}.property.{pset}.{name}");
    let args = || ui_value_map([("ids", ui_value_list([ui_value_text(id)?])?), ("pset", ui_value_text(pset)?), ("property", ui_value_text(name)?)]);
    let label = format!("{pset} · {name} ({})", crate::editor::bim::entities::property_type(value));
    let set = input_row(&row_id, &label, &property_text(value), None, bim_action("setProperty", Some(args()?)))?;
    let remove = tree_item_with_icon(format!("{row_id}.remove"), Label::data(BimLabels::named(labels.action_remove_property, &format!("{pset}.{name}"))), "trash", bim_action("removeProperty", Some(args()?)))?;
    Ok(vec![set, remove])
}

/// 🧾️ The rows of the property sets of one element: an input and a remove row per property, then the add row. A property whose names overflow the fixed UI capacity is left out rather than
/// failing the panel.
fn property_rows(snapshot: &ModelSnapshot, id: &str, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let mut rows: Vec<UiAssemblyResult<BuiltNode>> = Vec::new();
    for (pset, properties) in snapshot.properties.get(id).into_iter().flatten() {
        for (name, value) in properties {
            if let Ok(pair) = property_pair(id, pset, name, value, labels) {
                rows.extend(pair.into_iter().map(Ok));
            }
        }
    }
    rows.push(ids_args(&[id]).and_then(|args| input_row(&format!("{ROOT}.property.add"), labels.action_add_property.as_str(), "", None, bim_action("setProperty", Some(args)))));
    rows
}
//#endregion 🔖️PropertySets

//#region 🔖️Render
/// 🔍️ Renders the properties panel for the given selections.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, elements: &[String], library: &[String], labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    let builder = PanelTreeBuilder::new(ROOT)?;
    let Some((row, ids)) = subject(snapshot, elements, library) else {
        let project = ui_node_list(PROJECT_FIELDS.iter().map(|field| authored_row(snapshot, field, &[PROJECT_ID], labels)))?;
        return builder
            .section(format!("{ROOT}.project"), Some(ui_label(labels.section_project.as_str())?), true, project)?
            .section(format!("{ROOT}.summary"), Some(ui_label(labels.section_summary.as_str())?), true, ui_node_list(summary_rows(snapshot, labels))?)?
            .build();
    };
    let heading = format!("{} {}", (row.label)(labels).as_str(), if ids.len() > 1 { format!("× {}", ids.len()) } else { (row.name)(snapshot, ids[0]).unwrap_or_default() });
    let authored = ui_node_list(fields_of(row).map(|field| authored_row(snapshot, field, &ids, labels)))?;
    let mut builder = builder.section(format!("{ROOT}.authored"), Some(ui_label(&heading)?), true, authored)?;
    let inferred = inferred_rows(snapshot, inference, row, &ids, labels);
    if !inferred.is_empty() {
        builder = builder.section(format!("{ROOT}.inferred"), Some(ui_label(labels.section_inferred.as_str())?), true, ui_node_list(inferred)?)?;
    }
    if !row.library {
        let mut actions = wall_rows(snapshot, &ids, elements, labels);
        actions.extend(place_rows(snapshot, &ids, labels));
        actions.extend(conditions_rows(snapshot, &ids, labels));
        if !actions.is_empty() {
            builder = builder.section(format!("{ROOT}.actions"), Some(ui_label(labels.section_actions.as_str())?), true, ui_node_list(actions)?)?;
        }
    }
    if let [only] = ids.as_slice() {
        if snapshot.components.contains_key(*only) {
            builder = builder.section(format!("{ROOT}.overrides"), Some(ui_label(labels.section_overrides.as_str())?), true, ui_node_list(overrides::render_rows(snapshot, ROOT, only, labels))?)?;
        }
        if crate::mutations::elements::holds_data(snapshot, only) {
            builder = builder.section(format!("{ROOT}.properties"), Some(ui_label(labels.section_property_sets.as_str())?), true, ui_node_list(property_rows(snapshot, only, labels))?)?;
            let effective = data::effective_rows(snapshot, inference, only, labels);
            if !effective.is_empty() {
                builder = builder.section(format!("{ROOT}.effective"), Some(ui_label(labels.section_effective.as_str())?), true, ui_node_list(effective)?)?;
            }
            let applying = data::apply_rows(snapshot, only, labels);
            if !applying.is_empty() {
                builder = builder.section(format!("{ROOT}.templates"), Some(ui_label(labels.section_templates.as_str())?), true, ui_node_list(applying)?)?;
            }
            if !snapshot.classification_systems.is_empty() {
                builder = builder.section(format!("{ROOT}.classification"), Some(ui_label(labels.section_classification.as_str())?), true, ui_node_list(data::classification_rows(snapshot, only, labels))?)?;
            }
        }
        if row.kind == "property-template" {
            for (position, (title, rows)) in data::definition_groups(snapshot, only, labels).into_iter().enumerate() {
                builder = builder.section(format!("{ROOT}.definitions.{position}"), Some(ui_label(&title)?), true, ui_node_list(rows)?)?;
            }
        }
        if row.kind == "classification-system" {
            builder = builder.section(format!("{ROOT}.entries"), Some(ui_label(labels.section_entries.as_str())?), true, ui_node_list(data::entry_rows(snapshot, only, labels))?)?;
        }
    }
    builder.build()
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
