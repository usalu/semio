//! 🔍️ Forms play app panel — the document-wide summary (schema, step count, question count).

use crate::{forms_steps, FormsSnapshot};
use crate::editor::forms::{ui_label, ui_node_list};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_INSPECTION: &str = "forms.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(FORMS_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ `ArtifactEditor::render` carries no `InteractionView` (a known SDK gap — matches `gis2d`'s and
/// `note`'s inspection panel precedent, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so
/// this panel can no longer tell which question(s) are selected — it always shows the document-wide
/// summary now; the per-selected-question kind editor (label/kind/required/options/vector fields/…,
/// driven by `patchQuestions`/`patchQuestionOptions`/`patchVectorField`) that used to read
/// `cfg.selected_ids` is gone with it.
pub fn render(spec: &FormsSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let children = ui_node_list([
        tree_item_desc("forms-play-inspector.schema", format!("Schema: {}", crate::FORMS_DOCUMENT_SCHEMA), None),
        tree_item_desc("forms-play-inspector.steps", format!("Steps: {}", forms_steps(spec).len()), None),
        tree_item_desc("forms-play-inspector.questions", format!("Questions: {}", crate::schema::flatten_questions(spec).len()), None),
    ])?;
    PanelTreeBuilder::new("forms-play-inspector")?.section("forms-play-inspector.summary", Some(ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, children)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
