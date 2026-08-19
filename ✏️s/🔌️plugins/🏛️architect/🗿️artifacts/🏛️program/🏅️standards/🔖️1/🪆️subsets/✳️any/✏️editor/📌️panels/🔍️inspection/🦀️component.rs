//! 🔍️ Architect inspection panel — the document-wide register summary.

use crate::editor::architect::config::{active_register, ArchitectConfig};
use crate::artifacts::program::ProgramSnapshot;
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const ARCHITECT_BODY_INSPECTION: &str = "architect.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub async fn definition() -> PanelTabDefinition {
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
pub async fn render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode {
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "architect-inspection.summary".into(),
        label: Label::data("ProgramSnapshot"),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("architect-inspection.summary.schema", Label::data("Schema"), program.schema.clone()),
            ui_inspector_readonly_field("architect-inspection.summary.active-register", Label::data("Active Register"), active_register(cfg).to_string()),
            ui_inspector_readonly_field("architect-inspection.summary.elements", Label::data("Elements"), program.elements.len().to_string()),
            ui_inspector_readonly_field("architect-inspection.summary.stakeholders", Label::data("Stakeholders"), program.stakeholders.len().to_string()),
            ui_inspector_readonly_field("architect-inspection.summary.adjacencies", Label::data("Adjacencies"), program.adjacencies.len().to_string()),
            ui_inspector_readonly_field("architect-inspection.summary.requirements", Label::data("Requirements"), program.requirements.len().to_string()),
            ui_inspector_readonly_field("architect-inspection.summary.risks", Label::data("Risks"), program.risks.len().to_string()),
        ],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[semio_framework_async_macros::async_test]
    async fn the_tab_is_the_framework_inspection_tab_bound_to_this_apps_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_INSPECTION));
        assert!(matches!(definition.group, PanelGroup::Details));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_inspector_always_summarises_the_document_wide_register_counts() {
        let program = sample_plugin();
        let cfg = ArchitectConfig::default();
        let json = serde_json::to_string(&render(&program, &cfg)).expect("json");
        assert!(json.contains("architect-inspection.summary.schema"));
        assert!(json.contains(&program.elements.len().to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_summary_reflects_the_active_register() {
        let program = sample_plugin();
        let cfg = ArchitectConfig { active_register: "risks".into(), ..ArchitectConfig::default() };
        let json = serde_json::to_string(&render(&program, &cfg)).expect("json");
        assert!(json.contains("\"value\":\"risks\""));
    }
}
//#endregion 🧪️Tests
