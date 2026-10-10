//! 🏷️ The data rows of the properties panel: the effective properties of a holder (own, inherited from its type or by default, with the findings against the templates that apply), the templates that can be applied to it,
//! its classification per system, the editable property definitions of a property set template and the entry table commands of a classification system. Every row is a labelled input or button: nothing needs a pointer.

use super::{input_row, ROOT};
use crate::editor::bim::entities::property_text;
use crate::editor::bim::kit::{bim_action, tree_item_with_icon, ui_label, ui_value_list, ui_value_map, ui_value_text};
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::inferences::effective_properties::{self as effective, Issue, Source};
use crate::{ModelInference, ModelSnapshot, PropertyDef};
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiValue;
use semio_framework_ui_locale::Label;

/// 🧱️ The most rows one group of the panel shows: the fixed capacity of a container is 128 nodes.
const ROWS: usize = 96;

fn source_label(source: Source, labels: &BimLabels) -> &str {
    match source {
        Source::Own => labels.source_own.as_str(),
        Source::Type => labels.source_type.as_str(),
        Source::Default => labels.source_default.as_str(),
    }
}

fn issue_label(issue: Issue, labels: &BimLabels) -> &str {
    match issue {
        Issue::Missing => labels.issue_missing.as_str(),
        Issue::KindMismatch => labels.issue_kind.as_str(),
        Issue::BelowMinimum => labels.issue_below.as_str(),
        Issue::AboveMaximum => labels.issue_above.as_str(),
        Issue::NotAllowed => labels.issue_not_allowed.as_str(),
    }
}

fn unit_of<'a>(snapshot: &'a ModelSnapshot, template: Option<&str>, set: &str, name: &str) -> &'a str {
    template.and_then(|id| snapshot.property_templates.get(id)).filter(|template| template.name == set).and_then(|template| template.properties.iter().find(|definition| definition.name == name)).and_then(|definition| definition.unit.as_deref()).unwrap_or("")
}

fn holder_args(id: &str, system: &str) -> UiAssemblyResult<UiValue> {
    ui_value_map([("ids", ui_value_list(vec![ui_value_text(id)?])?), ("system", ui_value_text(system)?)])
}

//#region 🔖️Effective
/// 🏷️ The effective properties of one holder as read-only rows, `Set · Name` with the value, its unit and where it comes from, then one row per finding.
pub fn effective_rows(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let Some(properties) = inference.effective_properties.get(id) else { return Vec::new() };
    let mut rows: Vec<UiAssemblyResult<BuiltNode>> = Vec::new();
    for (set, values) in &properties.values {
        for (name, value) in values {
            let unit = unit_of(snapshot, value.template.as_deref(), set, name);
            let shown = format!("{} {unit} ({})", property_text(&value.value), source_label(value.source, labels)).replace("  ", " ");
            rows.push(ui_label(&format!("{set} · {name}")).and_then(|label| tree_item_desc(format!("{ROOT}.effective.{set}.{name}"), label, Some(shown))));
        }
    }
    for finding in &properties.findings {
        let issue = issue_label(finding.issue, labels).to_string();
        rows.push(ui_label(&format!("{} · {}", finding.set, finding.property)).and_then(|label| tree_item_desc(format!("{ROOT}.effective.issue.{}.{}", finding.set, finding.property), label, Some(issue))));
    }
    rows.truncate(ROWS);
    rows
}
//#endregion 🔖️Effective

//#region 🔖️Apply
/// 🧰️ One button per template that applies to the kind of the holder and defines a default: it gives the holder the defaults the template defines.
pub fn apply_rows(snapshot: &ModelSnapshot, id: &str, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let Some(target) = effective::target_of(snapshot, id) else { return Vec::new() };
    snapshot
        .property_templates
        .iter()
        .filter(|(_, template)| template.applies_to.contains(&target) && template.properties.iter().any(|definition| definition.default_value.is_some()))
        .take(ROWS)
        .map(|(template_id, template)| {
            let args = ui_value_map([("ids", ui_value_list(vec![ui_value_text(id)?])?), ("template", ui_value_text(template_id)?)])?;
            tree_item_with_icon(format!("{ROOT}.apply.{template_id}"), Label::data(BimLabels::named(labels.action_apply_template, &template.name)), "list-checks", bim_action("applyTemplate", Some(args)))
        })
        .collect()
}
//#endregion 🔖️Apply

//#region 🔖️Classification
fn classification_pair(snapshot: &ModelSnapshot, id: &str, system_id: &str, labels: &BimLabels) -> UiAssemblyResult<Vec<BuiltNode>> {
    let Some(system) = snapshot.classification_systems.get(system_id) else { return Ok(Vec::new()) };
    let code = snapshot.classifications.get(id).and_then(|set| set.get(system_id));
    let title = code.and_then(|code| system.entry(code)).map(|entry| entry.title.as_str()).unwrap_or("");
    let label = if title.is_empty() { system.name.clone() } else { format!("{} ({title})", system.name) };
    let mut rows = vec![input_row(&format!("{ROOT}.classification.{system_id}"), &label, code.map_or("", String::as_str), None, bim_action("setClassification", Some(holder_args(id, system_id)?)))?];
    if code.is_some() {
        rows.push(tree_item_with_icon(format!("{ROOT}.classification.{system_id}.remove"), Label::data(BimLabels::named(labels.action_remove_classification, &system.name)), "trash", bim_action("removeClassification", Some(holder_args(id, system_id)?)))?);
    }
    Ok(rows)
}

/// 🗂️ The classification of one holder per system: an input for the code (the title of the entry beside the system name) and a remove button where it has one.
pub fn classification_rows(snapshot: &ModelSnapshot, id: &str, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    snapshot.classification_systems.keys().take(ROWS / 2).flat_map(|system_id| match classification_pair(snapshot, id, system_id, labels) {
        Ok(rows) => rows.into_iter().map(Ok).collect::<Vec<_>>(),
        Err(error) => vec![Err(error)],
    }).collect()
}
//#endregion 🔖️Classification

//#region 🔖️Definitions
fn definition_args(id: &str, op: &str, index: &str, field: &str) -> UiAssemblyResult<UiValue> {
    ui_value_map([("id", ui_value_text(id)?), ("op", ui_value_text(op)?), ("index", ui_value_text(index)?), ("field", ui_value_text(field)?)])
}

fn bound(value: Option<f64>) -> String {
    value.map_or_else(String::new, |value| format!("{value}"))
}

/// 🧾️ The rows of one property definition of a template: every field is an input committing one `editTemplate`, then the move and remove buttons.
fn definition_group(template: &str, index: usize, definition: &PropertyDef, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let at = index.to_string();
    let field = |key: &str, label: &str, value: String| definition_args(template, "set", &at, key).and_then(|args| input_row(&format!("{ROOT}.definition.{index}.{key}"), &format!("{}. {} · {label}", index + 1, definition.name), &value, None, bim_action("editTemplate", Some(args))));
    let button = |op: &str, icon: &str, label: String| definition_args(template, op, &at, "").and_then(|args| tree_item_with_icon(format!("{ROOT}.definition.{index}.{op}"), Label::data(label), icon, bim_action("editTemplate", Some(args))));
    let allowed = definition.allowed.iter().map(property_text).collect::<Vec<_>>().join("; ");
    vec![
        field("name", labels.def_name.as_str(), definition.name.clone()),
        field("kind", labels.def_kind.as_str(), definition.kind.name().to_string()),
        field("unit", labels.def_unit.as_str(), definition.unit.clone().unwrap_or_default()),
        field("description", labels.def_description.as_str(), definition.description.clone().unwrap_or_default()),
        field("required", labels.def_required.as_str(), definition.required.to_string()),
        field("default_value", labels.def_default.as_str(), definition.default_value.as_ref().map(property_text).unwrap_or_default()),
        field("minimum", labels.def_minimum.as_str(), bound(definition.minimum)),
        field("maximum", labels.def_maximum.as_str(), bound(definition.maximum)),
        field("allowed", labels.def_allowed.as_str(), allowed),
        button("up", "arrow-up", BimLabels::named(labels.action_move_up, &definition.name)),
        button("down", "arrow-down", BimLabels::named(labels.action_move_down, &definition.name)),
        button("remove", "trash", BimLabels::named(labels.action_remove_definition, &definition.name)),
    ]
}

fn add_definition_row(id: &str, labels: &BimLabels) -> UiAssemblyResult<BuiltNode> {
    input_row(&format!("{ROOT}.definition.add"), labels.action_add_definition.as_str(), "", Some("Name:length"), bim_action("editTemplate", Some(definition_args(id, "add", "", "")?)))
}

/// 🧾️ The groups of a template's definitions, one container each (title and rows), the last one holding the row that adds a definition (`Name` or `Name:kind`).
pub fn definition_groups(snapshot: &ModelSnapshot, id: &str, labels: &BimLabels) -> Vec<(String, Vec<UiAssemblyResult<BuiltNode>>)> {
    let Some(template) = snapshot.property_templates.get(id) else { return Vec::new() };
    let mut groups: Vec<(String, Vec<UiAssemblyResult<BuiltNode>>)> = template.properties.iter().enumerate().take(24).map(|(index, definition)| (format!("{}. {}", index + 1, definition.name), definition_group(id, index, definition, labels))).collect();
    groups.push((labels.section_definitions.as_str().to_string(), vec![add_definition_row(id, labels)]));
    groups
}
//#endregion 🔖️Definitions

//#region 🔖️Entries
fn entry_row(id: &str, op: &str, label: &str, placeholder: &str) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("id", ui_value_text(id)?), ("op", ui_value_text(op)?)])?;
    input_row(&format!("{ROOT}.entries.{op}"), label, "", Some(placeholder), bim_action("editClassification", Some(args)))
}

/// 📚️ The entry table commands of a classification system: add (`code | title | parent`), retitle (`code | title`), move (`code | parent`) and remove (`code`) an entry. The browser lists the table.
pub fn entry_rows(snapshot: &ModelSnapshot, id: &str, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    if !snapshot.classification_systems.contains_key(id) {
        return Vec::new();
    }
    vec![
        entry_row(id, "add", labels.entry_add.as_str(), "Pr_20 | Title | Pr"),
        entry_row(id, "retitle", labels.entry_retitle.as_str(), "Pr_20 | Title"),
        entry_row(id, "reparent", labels.entry_reparent.as_str(), "Pr_20 | Pr"),
        entry_row(id, "remove", labels.entry_remove.as_str(), "Pr_20"),
    ]
}
//#endregion 🔖️Entries

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
