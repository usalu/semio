//! 🔍️ Architect inspection panel — typed inspectors for the selected entity, with a generic
//! id/name fallback for registers that have no bespoke inspector yet.

use crate::apps::architect::catalog::{find_register_for_entity, register_entities};
use crate::apps::architect::chrome::{adjacency_kind_label, element_label, entity_id_from_json, entity_name_from_json, inspector_number_field, inspector_text_field, inspector_toggle_field};
use crate::apps::architect::config::ArchitectConfig;
use crate::artifacts::program::{EntityId, ProgramSnapshot};
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const ARCHITECT_BODY_INSPECTION: &str = "architect.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::apps::architect::create_architect_app`.
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
pub fn render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode {
    if cfg.selected_ids.is_empty() {
        return ui_stack_vertical(vec![ui_text(Label::data("Select an entity in the document or register view."))]);
    }
    let id = EntityId(cfg.selected_ids[0].clone());
    let register = find_register_for_entity(program, &id).unwrap_or("elements");
    let entity_id = id.to_string();
    if let Some(element) = program.elements.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.element.id", Label::data("Id"), entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.element.name", "Name", std::slice::from_ref(&element.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.element.code", "Code", std::slice::from_ref(&element.code), "code"),
            inspector_text_field(register, &entity_id, "architect-inspection.element.level", "Level", &[element.level.clone().unwrap_or_default()], "level"),
            ui_inspector_readonly_field("architect-inspection.element.kind", Label::data("Kind"), format!("{:?}", element.kind)),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.element".into(), label: Label::data("Element"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    if let Some(stakeholder) = program.stakeholders.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.stakeholder.id", Label::data("Id"), entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.name", "Name", std::slice::from_ref(&stakeholder.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.role", "Role", std::slice::from_ref(&stakeholder.role), "role"),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.organization", "Organization", std::slice::from_ref(&stakeholder.organization), "organization"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.stakeholder".into(), label: Label::data("Stakeholder"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    if let Some(adjacency) = program.adjacencies.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.adjacency.id", Label::data("Id"), entity_id.clone()),
            ui_inspector_readonly_field("architect-inspection.adjacency.pair", Label::data("Pair"), format!("{} ↔ {}", element_label(program, &adjacency.element_a_id), element_label(program, &adjacency.element_b_id))),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.kind", "Kind", &[adjacency_kind_label(&adjacency.kind).to_string()], "kind"),
            inspector_number_field(register, &entity_id, "architect-inspection.adjacency.weight", "Weight", &[adjacency.weight], "weight"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.connection", "Connection", &[format!("{:?}", adjacency.connection)], "connection"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.separations", "Separations", &[adjacency.separations.iter().map(|separation| format!("{separation:?}")).collect::<Vec<_>>().join(", ")], "separations"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.internalExternalAccess", "Internal/External Access", &[adjacency.internal_external_access.clone().unwrap_or_default()], "internalExternalAccess"),
            inspector_toggle_field(register, &entity_id, "architect-inspection.adjacency.sharedWall", "Shared Wall", &[adjacency.shared_wall], "sharedWall"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.adjacency".into(), label: Label::data("Adjacency"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    if let Some(requirement) = program.requirements.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.requirement.id", Label::data("Id"), entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.name", "Name", std::slice::from_ref(&requirement.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.code", "Code", std::slice::from_ref(&requirement.code), "code"),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.statement", "Statement", std::slice::from_ref(&requirement.statement.text), "statement"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.requirement".into(), label: Label::data("Requirement"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    if let Some(risk) = program.risks.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.risk.id", Label::data("Id"), entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.risk.name", "Name", std::slice::from_ref(&risk.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.risk.statement", "Statement", std::slice::from_ref(&risk.risk_statement.text), "riskStatement"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.risk".into(), label: Label::data("Risk"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    let generic_name = register_entities(program, register).into_iter().find(|entity| entity_id_from_json(entity).as_deref() == Some(entity_id.as_str())).map_or_else(|| entity_id.clone(), |entity| entity_name_from_json(&entity));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "architect-inspection.generic".into(),
        label: Label::data(format!("{register} entity")),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![ui_inspector_readonly_field("architect-inspection.generic.id", Label::data("Id"), entity_id.clone()), inspector_text_field(register, &entity_id, "architect-inspection.generic.name", "Name", &[generic_name], "name")],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn the_tab_is_the_framework_inspection_tab_bound_to_this_apps_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_INSPECTION));
        assert!(matches!(definition.group, PanelGroup::Details));
    }

    #[test]
    fn no_selection_renders_the_placeholder() {
        let json = serde_json::to_string(&render(&sample_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("Select an entity in the document"));
    }

    #[test]
    fn a_selected_element_renders_the_element_inspector_group() {
        let program = sample_plugin();
        let cfg = ArchitectConfig { selected_ids: vec![program.elements[0].header.id.to_string()], ..ArchitectConfig::default() };
        let json = serde_json::to_string(&render(&program, &cfg)).expect("json");
        assert!(json.contains("architect-inspection.element.name"));
    }

    #[test]
    fn a_selected_adjacency_renders_the_adjacency_inspector_group() {
        let program = sample_plugin();
        let cfg = ArchitectConfig { selected_ids: vec![program.adjacencies[0].header.id.to_string()], ..ArchitectConfig::default() };
        let json = serde_json::to_string(&render(&program, &cfg)).expect("json");
        assert!(json.contains("architect-inspection.adjacency.weight"));
    }

    #[test]
    fn an_unknown_id_falls_back_to_the_generic_inspector() {
        let cfg = ArchitectConfig { selected_ids: vec!["nope".into()], ..ArchitectConfig::default() };
        let json = serde_json::to_string(&render(&sample_plugin(), &cfg)).expect("json");
        assert!(json.contains("architect-inspection.generic"));
    }
}
//#endregion 🧪️Tests
