//! 📄️ Forms play app panel — the document tree: steps and their questions.

use crate::schema::forms_play_step_tree_id;
use crate::{forms_steps, FormsSnapshot};
use crate::editor::forms::terminology::FormsLabels;
use crate::editor::forms::{forms_action, ui_node_list, FORMS_INTERACTION_FIELDS};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_DOCUMENT: &str = "forms.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(FORMS_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `fields` domain target ids `FormsPlayApp::interaction_topology` declares — steps at the "section"
/// granularity via `forms_play_step_tree_id`, questions at the "field" granularity via their own raw
/// id — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item click
/// action is declared here anymore (clicks are translated into `interactionSelect` generically)?.
pub fn render(spec: &FormsSnapshot, labels: &FormsLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let step_items = ui_node_list(forms_steps(spec).iter().map(|step| {
        let question_items = ui_node_list(step.blocks.iter().map(|question| {
            let mut node = tree_item_desc(question.id.clone(), question.label.as_str(), Some(question.kind.clone()))?;
            if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
                props.icon = Some(UiText::try_from_str("help-circle").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "forms question icon admission failed"))?);
                props.draggable = Some(true);
            }
            Ok(node)
        }))?;
        let mut node = tree_item_desc(forms_play_step_tree_id(&step.id), step.title.as_str(), Some(format!("{} questions", step.blocks.len())))?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
            props.icon = Some(UiText::try_from_str("list-tree").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "forms step icon admission failed"))?);
            props.default_open = Some(true);
            props.draggable = Some(true);
        }
        node = node.try_with_children(question_items).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "forms question children admission failed"))?;
        Ok(node)
    }))?;
    let (drop_action, drop_args) = forms_action("dropQuestionKind", None)?;
    if drop_args.is_some() {
        return Err(PluginAssemblyError::new("ui.action-argument", "forms drop action must not carry arguments"));
    }
    PanelTreeBuilder::new("forms-play-document")?
        .section_or_placeholder("forms-play-document.steps", Some(crate::editor::forms::ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?), true, step_items, labels.no_steps_tree_item.as_str())?
        .interaction_domain(FORMS_INTERACTION_FIELDS)?
        .drop_action(drop_action)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
