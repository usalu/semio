//! 🛍️ Forms play app panel — the catalogue: the draggable question-kind palette plus quick actions.
//!
//! 🪟️ The question-kind roster grows with every program contribution, so it is a windowed section:
//! it stamps its FULL extent through `TreeWindow { total, offset }` and materialises only the host's
//! slice (`ViewModel::tree_windows`), never a `+N` row. The two-row `actions` section is a pair of
//! hand-written heterogeneous rows with no entry slice behind it, so it stays a plain section.
//!
//! 🕹️ Deliberately UNBOUND to an interaction domain: a catalogue row is not a pick of the `fields`
//! domain, it is its own `addQuestion`/`addStep` command, so the rows keep their action and their
//! drag payload and stamp no `granularity`.

use crate::editor::forms::config::FormsConfig;
use crate::editor::forms::terminology::FormsLabels;
use crate::editor::forms::{catalogue_kinds, forms_action, parse_contributions};
use dsl::os_pack::json::{object, Value};
use semio_framework_plugin::{
    tree_item_with_action, tree_item_with_action_draggable, ui_node_list, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_CATALOGUE: &str = "forms.play.catalogue";
pub const FORMS_PLAY_CATALOGUE_ROOT: &str = "forms-play-catalogue";
pub const FORMS_PLAY_CATALOGUE_KINDS: &str = "forms-play-catalogue.kinds";
pub const FORMS_PLAY_CATALOGUE_ACTIONS: &str = "forms-play-catalogue.actions";
const FORMS_QUESTION_DRAG_MIME: &str = "application/x-semio-forms-question-kind";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(FORMS_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🛍️ One draggable question-kind row — its own `addQuestion` command, not a domain pick.
fn kind_row(kind: &str, label: &str, icon: &semio_framework_plugin::IconName) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::editor::forms::ui_value_map([("kind", crate::editor::forms::ui_value_text(kind)?)])?;
    let drag_data = object([(FORMS_QUESTION_DRAG_MIME.to_string(), Value::String(object([("kind".to_string(), Value::String(kind.to_string()))]).to_string()))]);
    let mut item = tree_item_with_action_draggable(format!("{FORMS_PLAY_CATALOGUE_ROOT}.{kind}"), label, Some(kind.to_string()), forms_action("addQuestion", Some(args))?, &drag_data)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(crate::editor::forms::ui_text_value(icon.as_str())?);
    }
    Ok(item)
}

fn action_row(id: &str, label: &str, icon: &str, action: (semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = tree_item_with_action(id, label, None, action)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.icon", "fixed catalogue icon admission failed"))?);
    }
    Ok(item)
}

pub fn render(config: &FormsConfig, labels: &FormsLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let contributions = parse_contributions(config);
    let kinds = catalogue_kinds(&contributions, labels);
    let text_args = crate::editor::forms::ui_value_map([("kind", crate::editor::forms::ui_value_text("text")?)])?;
    let action_items = ui_node_list([
        action_row("forms-play-catalogue.add-step", labels.add_step.as_str(), "plus", forms_action("addStep", None)?),
        action_row("forms-play-catalogue.add-question", labels.add_text_question.as_str(), "type", forms_action("addQuestion", Some(text_args))?),
    ])?;
    PanelTreeBuilder::new(FORMS_PLAY_CATALOGUE_ROOT)?
        .window_section(windows, FORMS_PLAY_CATALOGUE_KINDS, Some(crate::editor::forms::ui_label(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)?), true, &kinds, |(kind, label, icon)| kind_row(kind, label, icon))?
        .section(FORMS_PLAY_CATALOGUE_ACTIONS, Some(crate::editor::forms::ui_label(labels.actions.as_str())?), true, action_items)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
