//! 🧭️ Accessible workset management and per-window option switcher; authored changes reuse entity commands.
use crate::{ModelSnapshot, ModelInference};
use crate::editor::bim::{kit::{bim_action, tree_item_with_icon, ui_label, ui_value_map, ui_value_text, ui_capacity_error}, terminology::BimLabels};
use semio_framework_plugin::{BuiltNode, PanelTreeBuilder, PanelTabDefinition, PanelTabKind, PanelGroup, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, Buildable, HasChildren};
use semio_framework_ui_locale::Label;
use std::collections::BTreeMap;
pub const BODY_KEY: &str = "bim.edit.options";
/// 🧱️ One shared management panel, registered with the artifact editor.
pub fn definition() -> PanelTabDefinition { PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: BimLabels::localized(|labels| labels.panel_options), group: PanelGroup::Details, body_key: Some(BODY_KEY.into()), children: Vec::new() } }
fn create(kind: &str, parent: &str, name: &str) -> UiAssemblyResult<BuiltNode> {
    tree_item_with_icon(format!("bim.options.create.{kind}.{parent}"), Label::data(name), "plus", bim_action("createEntity", Some(ui_value_map([("kind", ui_value_text(kind)?), ("parent", ui_value_text(parent)?), ("name", ui_value_text(name)?)])?)))
}
fn entity(snapshot: &ModelSnapshot, kind: &str, id: &str, description: &str) -> UiAssemblyResult<BuiltNode> {
    let name = crate::editor::bim::entities::kind_of(kind).and_then(|row| (row.name)(snapshot, id)).unwrap_or_else(|| id.to_owned());
    crate::editor::bim::panels::outliner::leaf(crate::editor::bim::panels::outliner::item(id, &name, "layers", Some(kind), Some(description))?)
}
/// 🧭️ Groups, options and worksets are selectable authored records; properties edits their parameters.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, labels: &BimLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let adds = semio_framework_plugin::ui_node_list([create("option-group", "", labels.kind_option_group.as_str()), create("workset", "", labels.kind_workset.as_str())])?;
    let mut builder = PanelTreeBuilder::new("bim.options")?.section("bim.options.add", Some(ui_label(labels.panel_options.as_str())?), true, adds)?;
    for (id, group) in &snapshot.option_groups {
        let options: Vec<_> = snapshot.design_options.iter().filter(|(_, option)| option.group == *id).collect();
        builder = builder.section(format!("bim.options.group.{id}"), Some(ui_label(&group.name)?), true, semio_framework_plugin::ui_node_list([entity(snapshot, "option-group", id, ""), create("design-option", id, labels.kind_design_option.as_str())])?)?;
        builder = builder.window_section(windows, &format!("bim.options.choices.{id}"), None, false, &options, |(id, option)| entity(snapshot, "design-option", id, if option.primary { labels.field_primary_option.as_str() } else { "" }))?;
    }
    let worksets: Vec<_> = snapshot.worksets.iter().collect();
    builder.window_section(windows, "bim.options.worksets", Some(ui_label(labels.group_worksets.as_str())?), true, &worksets, |(id, _)| entity(snapshot, "workset", id, &inference.option_scope.worksets.get(*id).map_or(0, Vec::len).to_string()))?.build()
}
fn view_row(id: String, label: &str, field: &str, value: &str) -> UiAssemblyResult<BuiltNode> { tree_item_with_icon(id, Label::data(label), "eye", bim_action("setView", Some(ui_value_map([("field", ui_value_text(field)?), ("value", ui_value_text(value)?)])?))) }
/// 🎚️ Choice controls publish only window-local config diffs and wrap the drawing surface.
pub fn with_switcher(snapshot: &ModelSnapshot, inference: &ModelInference, selected: &BTreeMap<String, String>, visibility: &BTreeMap<String, bool>, labels: &BimLabels, windows: &TreeWindows<'_>, body: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    if snapshot.option_groups.is_empty() && snapshot.worksets.is_empty() { return Ok(body); }
    let mut builder = PanelTreeBuilder::new("bim.options.switcher")?;
    for (group_id, group) in &snapshot.option_groups {
        let current = selected.get(group_id).or_else(|| inference.option_scope.defaults.get(group_id)).and_then(|id| snapshot.design_options.get(id)).map(|option| option.name.as_str()).unwrap_or("");
        let heading = format!("{}: {}", group.name, current);
        let options: Vec<_> = snapshot.design_options.iter().filter(|(_, option)| option.group == *group_id).collect();
        builder = builder.window_section(windows, &format!("bim.options.switcher.{group_id}"), Some(ui_label(&heading)?), false, &options, |(id, option)| view_row(format!("bim.options.activate.{id}"), &option.name, &format!("option:{group_id}"), id))?;
    }
    let worksets: Vec<_> = snapshot.worksets.iter().collect();
    builder = builder.window_section(windows, "bim.options.switcher.worksets", Some(ui_label(labels.group_worksets.as_str())?), false, &worksets, |(id, workset)| {
        let shown = visibility.get(*id).copied().unwrap_or(workset.default_visible);
        view_row(format!("bim.options.visibility.{id}"), &format!("{} · {}", workset.name, if shown { labels.option_hide.as_str() } else { labels.option_visible.as_str() }), &format!("workset:{id}"), if shown { "false" } else { "true" })
    })?;
    semio_framework_ui_contract::column().try_child(builder.build()?).map_err(|_| ui_capacity_error())?.try_child(body).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())
}
