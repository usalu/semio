//! 🔍️ Architect inspection panel — the document-wide register summary.

use crate::editor::architect::{ui_children, ui_label, ui_node};
use crate::ProgramSnapshot;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{column, field, section, text, BuiltNode};

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
pub fn render(program: &ProgramSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let values = [
        ("schema", "Schema", program.schema.clone()),
        ("elements", "Elements", program.elements.len().to_string()),
        ("stakeholders", "Stakeholders", program.stakeholders.len().to_string()),
        ("adjacencies", "Adjacencies", program.adjacencies.len().to_string()),
        ("requirements", "Requirements", program.requirements.len().to_string()),
        ("risks", "Risks", program.risks.len().to_string()),
    ];
    let mut fields = Vec::with_capacity(values.len());
    for (key, label, value) in values {
        let id = format!("architect-inspection.summary.{key}");
        let value = ui_node(text(ui_label(value)?), &format!("{id}.value"))?;
        fields.push(ui_node(ui_children(field(ui_label(label)?), [value])?, &id)?);
    }
    let summary = ui_node(ui_children(section(ui_label("ProgramSnapshot")?).default_open(true), fields)?, "architect-inspection.summary")?;
    ui_node(ui_children(column(), [summary])?, "architect-inspection")
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
