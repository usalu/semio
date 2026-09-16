//! 📄️ Forms play app panel — the document tree: steps and their questions.
//!
//! 🪟️ BOTH levels are virtualised the framework way: the steps section is a windowed container and
//! every step row is a windowed container over its own questions
//! (`semio_framework_plugin::tree_window_item`). Each publishes its FULL extent through
//! `TreeWindow { total, offset }` and materialises only the slice the host asked for
//! (`ViewModel::tree_windows`, read once per render as [`TreeWindows`]), so a real form — many steps
//! × many questions per step — scrolls as one document, with no page cursor and no `+N` row.
//!
//! 🎯️ Rows carry a `granularity` and no binding of their own: the tree root owns the single
//! `interactionSelect` binding [`PanelTreeBuilder::interaction_domain`] stamps.

use crate::editor::forms::terminology::FormsLabels;
use crate::editor::forms::{forms_action, FORMS_INTERACTION_FIELDS, FORMS_INTERACTION_GRANULARITY_FIELD, FORMS_INTERACTION_GRANULARITY_SECTION, FORMS_PLAY_APP_ID};
use crate::schema::forms_play_step_tree_id;
use crate::{forms_steps, FormQuestion, FormStep, FormsSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{
    tree_window_item, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_ARTIFACT: &str = "forms.play.artifact";
pub const FORMS_PLAY_DOCUMENT_ROOT: &str = "forms-play-document";
pub const FORMS_PLAY_DOCUMENT_STEPS: &str = "forms-play-document.steps";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(FORMS_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
fn ui_text(value: &str, code: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", code))
}

/// 🎯️ One question row: a `"field"` pick target keyed by its own raw id, draggable, binding nothing.
fn question_row(question: &FormQuestion) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::tree_item(Label::data(question.label.clone()))
        .try_id(&question.id)
        .map_err(|_| PluginAssemblyError::new("ui.document", "forms question id admission failed"))?
        .description(ui_text(&question.kind, "forms question kind admission failed")?)
        .icon(ui_text("help-circle", "forms question icon admission failed")?)
        .draggable(true)
        .granularity(ui_text(FORMS_INTERACTION_GRANULARITY_FIELD, "forms field granularity admission failed")?);
    builder.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "forms question row admission failed"))
}

/// 🎯️ One step row: a `"section"` pick target keyed by its canonical tree id, draggable, and itself
/// a windowed container over the questions it owns.
fn step_row(step: &FormStep, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let id = forms_play_step_tree_id(&step.id);
    let item = ui::tree_item(Label::data(step.title.clone()))
        .try_id(&id)
        .map_err(|_| PluginAssemblyError::new("ui.document", "forms step id admission failed"))?
        .description(UiText::clipped(&format!("{} questions", step.blocks.len())))
        .icon(ui_text("list-tree", "forms step icon admission failed")?)
        .draggable(true)
        .granularity(ui_text(FORMS_INTERACTION_GRANULARITY_SECTION, "forms section granularity admission failed")?);
    tree_window_item(windows, item, &id, true, &step.blocks, question_row)
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `fields` domain target ids `FormsPlayApp::interaction_topology` declares — steps at the "section"
/// granularity via `forms_play_step_tree_id`, questions at the "field" granularity via their own raw
/// id — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item click
/// action is declared here anymore (clicks are translated into `interactionSelect` generically)?.
pub fn render(spec: &FormsSnapshot, labels: &FormsLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let steps = forms_steps(spec);
    let (drop_action, drop_args) = forms_action("dropQuestionKind", None)?;
    if drop_args.is_some() {
        return Err(PluginAssemblyError::new("ui.action-argument", "forms drop action must not carry arguments"));
    }
    PanelTreeBuilder::new(FORMS_PLAY_DOCUMENT_ROOT)?
        .window_section_or_placeholder(
            windows,
            FORMS_PLAY_DOCUMENT_STEPS,
            Some(crate::editor::forms::ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?),
            true,
            &steps,
            |step| step_row(step, windows),
            labels.no_steps_tree_item.as_str(),
        )?
        .interaction_domain(FORMS_PLAY_APP_ID, FORMS_INTERACTION_FIELDS)?
        .drop_action(drop_action)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
