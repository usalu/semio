//! 🎚️ The parameter override rows of a selected component in the properties panel: one row per parameter of its family with the formula of the family as the placeholder, the override formula of the instance as the input,
//! the value the parameter evaluates to under that override and the issue the evaluation found, plus a reset row while the parameter is overridden. A commit is the `setOverride` command; the view here is a pure function of the
//! snapshot, so the rows read the same in every locale and nothing derived is ever stored.

use crate::editor::bim::kit::{bim_action, tree_item_with_icon, ui_label, ui_value_map, ui_value_text};
use crate::editor::bim::modes::edit::windows::family::vocabulary::{kind_label, value_text};
use crate::editor::bim::panels::properties::input_row;
use crate::editor::bim::terminology::BimLabels;
use crate::mutations::family_rules::parameters_in_order;
use crate::standards::v1::subsets::any::schema::inferences::families::issues::message;
use crate::standards::v1::subsets::any::schema::inferences::families::{parameters, IssueOwner};
use crate::{ModelSnapshot, ParameterKind};
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_ui_locale::Label;
use std::collections::BTreeMap;

//#region 🔖️Rows
/// 🎚️ One parameter of a component as the panel shows it.
#[derive(Clone, Debug, PartialEq)]
pub struct OverrideRow {
    pub name: String,
    pub kind: ParameterKind,
    pub family_formula: String,
    pub formula: Option<String>,
    pub value: String,
    pub issue: String,
}

impl OverrideRow {
    /// 🎚️ Whether the component gives the parameter a formula of its own.
    pub fn overridden(&self) -> bool {
        self.formula.is_some()
    }
}

/// 🎚️ The overrides of `component` by parameter name, as the canonical formula texts the family evaluation takes.
pub fn overrides_of(snapshot: &ModelSnapshot, component: &str) -> BTreeMap<String, String> {
    snapshot.component_overrides.values().filter(|row| row.component == component).map(|row| (row.name.clone(), row.value.clone())).collect()
}

/// 🎚️ The rows of component `component`: every parameter of its family in dependency order, evaluated under the overrides of the instance; none for a component that does not exist.
pub fn rows(snapshot: &ModelSnapshot, component: &str, labels: &BimLabels) -> Vec<OverrideRow> {
    let Some(family) = snapshot.components.get(component).map(|row| row.family.as_str()) else { return Vec::new() };
    let overrides = overrides_of(snapshot, component);
    let resolution = parameters::resolve(snapshot, family, &overrides);
    parameters_in_order(snapshot, family)
        .into_iter()
        .map(|parameter| {
            let value = resolution.parameters.get(&parameter.name).and_then(|resolved| resolved.value.as_ref()).map(|found| value_text(labels, found)).unwrap_or_default();
            let issue = resolution.issues.iter().filter(|issue| issue.owner == IssueOwner::Parameter && issue.subject == parameter.name).map(|issue| message(issue, labels.locale().as_str())).collect::<Vec<_>>().join(" · ");
            OverrideRow { name: parameter.name.clone(), kind: parameter.kind, family_formula: parameter.value.clone(), formula: overrides.get(&parameter.name).cloned(), value, issue }
        })
        .collect()
}
//#endregion 🔖️Rows

//#region 🔖️Panel
fn args(component: &str, name: &str, value: Option<&str>) -> UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut entries = vec![("component", ui_value_text(component)?), ("name", ui_value_text(name)?)];
    if let Some(value) = value {
        entries.push(("value", ui_value_text(value)?));
    }
    ui_value_map(entries)
}

fn parameter_rows(root: &str, component: &str, row: &OverrideRow, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    let id = format!("{root}.override.{}", row.name);
    let state = if row.overridden() { labels.override_overridden } else { labels.override_inherited };
    let title = format!("{} · {} · {}", row.name, kind_label(labels, row.kind), state.as_str());
    let mut nodes = vec![
        ui_label(&title).and_then(|label| tree_item_desc(format!("{id}.value"), label, Some(row.value.clone()))),
        input_row(&id, &BimLabels::named(labels.override_input_named, &row.name), row.formula.as_deref().unwrap_or(""), Some(&row.family_formula), args(component, &row.name, None).and_then(|args| bim_action("setOverride", Some(args)))),
    ];
    if !row.issue.is_empty() {
        nodes.push(ui_label(labels.fam_issue.as_str()).and_then(|label| tree_item_desc(format!("{id}.issue"), label, Some(row.issue.clone()))));
    }
    if row.overridden() {
        nodes.push(args(component, &row.name, Some("")).and_then(|args| tree_item_with_icon(format!("{id}.reset"), Label::data(BimLabels::named(labels.override_reset_named, &row.name)), "rotate-ccw", bim_action("setOverride", Some(args)))));
    }
    nodes
}

/// 🎚️ The panel rows of the overrides of component `component`: the rows of every parameter, each as value, input, issue and reset.
pub fn render_rows(snapshot: &ModelSnapshot, root: &str, component: &str, labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    rows(snapshot, component, labels).iter().flat_map(|row| parameter_rows(root, component, row, labels)).collect()
}
//#endregion 🔖️Panel

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
