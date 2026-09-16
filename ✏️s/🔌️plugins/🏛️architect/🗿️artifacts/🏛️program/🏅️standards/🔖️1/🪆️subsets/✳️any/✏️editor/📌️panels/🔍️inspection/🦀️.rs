//! 🔍️ Architect inspection panel — the document-wide register summary.

use crate::editor::architect::ui_label;
use crate::ProgramSnapshot;
use semio_framework_plugin::{tree_item_desc, ui_node_list, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const ARCHITECT_BODY_INSPECTION: &str = "architect.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(ARCHITECT_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
/// `InteractionView` (matches `gis2d`'s inspection panel precedent), so this panel can no longer
/// tell which entity is currently selected — it always shows the document-wide register summary
/// now; the per-selected-entity typed inspector branches (element/stakeholder/adjacency/
/// requirement/risk/generic, keyed off the deleted `cfg.selected_ids`) are gone with it.
pub fn render(program: &ProgramSnapshot) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let rows = ui_node_list([
        tree_item_desc("architect-inspection.summary.schema", ui_label("Schema")?, Some(program.schema.clone())),
        tree_item_desc("architect-inspection.summary.elements", ui_label("Elements")?, Some(program.elements.len().to_string())),
        tree_item_desc("architect-inspection.summary.stakeholders", ui_label("Stakeholders")?, Some(program.stakeholders.len().to_string())),
        tree_item_desc("architect-inspection.summary.adjacencies", ui_label("Adjacencies")?, Some(program.adjacencies.len().to_string())),
        tree_item_desc("architect-inspection.summary.requirements", ui_label("Requirements")?, Some(program.requirements.len().to_string())),
        tree_item_desc("architect-inspection.summary.risks", ui_label("Risks")?, Some(program.risks.len().to_string())),
    ])?;
    PanelTreeBuilder::new("architect-inspection")?.section("architect-inspection.summary", Some(ui_label("ProgramSnapshot")?), true, rows)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semantic-contract/🦀️.rs"]
mod semantic_contract;
