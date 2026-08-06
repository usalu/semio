//! ⚙️ Architect program artifact engine — the `validate` topic.

//! ✅️ Program validation — schema, references, and adjacency integrity.

use crate::artifacts::program::engine::adjacency::detect_adjacency_conflicts;
use crate::artifacts::program::kernel::{DiagnosticSeverity, EntityId, ProgramDiagnostic};
use crate::artifacts::program::{Program, ARCHITECT_PROGRAM_SCHEMA};
use crate::artifacts::program::registers::ValidationStatus;
use std::collections::{HashMap, HashSet};

// #region 🔖️EntityIndex
struct EntityIndex {
    locations: HashMap<EntityId, (String, String)>,
    duplicates: Vec<(EntityId, String, String)>,
}

fn build_entity_index(program: &Program) -> EntityIndex {
    let mut locations: HashMap<EntityId, (String, String)> = HashMap::new();
    let mut duplicates = Vec::new();
    let mut register = |name: &str, id: &EntityId, label: &str| {
        if let Some((prev_reg, _)) = locations.get(id) {
            duplicates.push((id.clone(), prev_reg.clone(), name.to_string()));
        } else {
            locations.insert(id.clone(), (name.to_string(), label.to_string()));
        }
    };
    for e in &program.stakeholders {
        register("stakeholders", &e.header.id, &e.header.name);
    }
    for e in &program.users {
        register("users", &e.header.id, &e.header.name);
    }
    for e in &program.activities {
        register("activities", &e.header.id, &e.header.name);
    }
    for e in &program.functions {
        register("functions", &e.header.id, &e.header.name);
    }
    for e in &program.elements {
        register("elements", &e.header.id, &e.header.name);
    }
    for e in &program.quantities {
        register("quantities", &e.header.id, &e.header.name);
    }
    for e in &program.relationships {
        register("relationships", &e.header.id, &e.header.name);
    }
    for e in &program.adjacencies {
        register("adjacencies", &e.header.id, &e.header.name);
    }
    for e in &program.processes {
        register("processes", &e.header.id, &e.header.name);
    }
    for e in &program.flows {
        register("flows", &e.header.id, &e.header.name);
    }
    for e in &program.access_rules {
        register("access_rules", &e.header.id, &e.header.name);
    }
    for e in &program.operations {
        register("operations", &e.header.id, &e.header.name);
    }
    for e in &program.equipment {
        register("equipment", &e.header.id, &e.header.name);
    }
    for e in &program.resources {
        register("resources", &e.header.id, &e.header.name);
    }
    for e in &program.storage {
        register("storage", &e.header.id, &e.header.name);
    }
    for e in &program.environmental {
        register("environmental", &e.header.id, &e.header.name);
    }
    for e in &program.human_factors {
        register("human_factors", &e.header.id, &e.header.name);
    }
    for e in &program.accessibility {
        register("accessibility", &e.header.id, &e.header.name);
    }
    for e in &program.privacy {
        register("privacy", &e.header.id, &e.header.name);
    }
    for e in &program.safety {
        register("safety", &e.header.id, &e.header.name);
    }
    for e in &program.security {
        register("security", &e.header.id, &e.header.name);
    }
    for e in &program.regulatory {
        register("regulatory", &e.header.id, &e.header.name);
    }
    for e in &program.site_context {
        register("site_context", &e.header.id, &e.header.name);
    }
    for e in &program.organizational {
        register("organizational", &e.header.id, &e.header.name);
    }
    for e in &program.services {
        register("services", &e.header.id, &e.header.name);
    }
    for e in &program.infrastructure {
        register("infrastructure", &e.header.id, &e.header.name);
    }
    for e in &program.information {
        register("information", &e.header.id, &e.header.name);
    }
    for e in &program.communication {
        register("communication", &e.header.id, &e.header.name);
    }
    for e in &program.wayfinding {
        register("wayfinding", &e.header.id, &e.header.name);
    }
    for e in &program.schedules {
        register("schedules", &e.header.id, &e.header.name);
    }
    for e in &program.flexibility {
        register("flexibility", &e.header.id, &e.header.name);
    }
    for e in &program.growth {
        register("growth", &e.header.id, &e.header.name);
    }
    for e in &program.sustainability {
        register("sustainability", &e.header.id, &e.header.name);
    }
    for e in &program.resilience {
        register("resilience", &e.header.id, &e.header.name);
    }
    for e in &program.costs {
        register("costs", &e.header.id, &e.header.name);
    }
    for e in &program.delivery {
        register("delivery", &e.header.id, &e.header.name);
    }
    for e in &program.risks {
        register("risks", &e.header.id, &e.header.name);
    }
    for e in &program.conflicts {
        register("conflicts", &e.header.id, &e.header.name);
    }
    for e in &program.requirements {
        register("requirements", &e.header.id, &e.header.name);
    }
    for e in &program.priorities {
        register("priorities", &e.header.id, &e.header.name);
    }
    for e in &program.scenarios {
        register("scenarios", &e.header.id, &e.header.name);
    }
    for e in &program.options {
        register("options", &e.header.id, &e.header.name);
    }
    for e in &program.decisions {
        register("decisions", &e.header.id, &e.header.name);
    }
    for e in &program.validations {
        register("validations", &e.header.id, &e.header.name);
    }
    for e in &program.performance {
        register("performance", &e.header.id, &e.header.name);
    }
    for e in &program.quality {
        register("quality", &e.header.id, &e.header.name);
    }
    for e in &program.documents {
        register("documents", &e.header.id, &e.header.name);
    }
    for e in &program.changes {
        register("changes", &e.header.id, &e.header.name);
    }
    for e in &program.collaboration {
        register("collaboration", &e.header.id, &e.header.name);
    }
    for e in &program.analyses {
        register("analyses", &e.header.id, &e.header.name);
    }
    for e in &program.reports {
        register("reports", &e.header.id, &e.header.name);
    }
    for e in &program.search_filters {
        register("search_filters", &e.header.id, &e.header.name);
    }
    for e in &program.status_records {
        register("status_records", &e.header.id, &e.header.name);
    }
    for e in &program.workshops {
        register("workshops", &e.header.id, &e.header.name);
    }
    for e in &program.surveys {
        register("surveys", &e.header.id, &e.header.name);
    }
    for e in &program.issues {
        register("issues", &e.header.id, &e.header.name);
    }
    for e in &program.audit_events {
        register("audit_events", &e.header.id, &e.header.name);
    }
    for e in &program.templates {
        register("templates", &e.header.id, &e.header.name);
    }
    for e in &program.knowledge {
        register("knowledge", &e.header.id, &e.header.name);
    }
    for e in &program.benchmarks {
        register("benchmarks", &e.header.id, &e.header.name);
    }
    register("project", &program.project.id, &program.project.code);
    register("governance", &program.governance.id, "governance");
    for link in &program.traces {
        register("traces", &link.id, &format!("{}→{}", link.from_id, link.to_id));
    }
    EntityIndex { locations, duplicates }
}

fn check_ref(diagnostics: &mut Vec<ProgramDiagnostic>, index: &EntityIndex, target: &EntityId, source_id: &EntityId, register: &str, code: &str) {
    if !index.locations.contains_key(target) {
        diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: code.into(), message: format!("{register} references missing entity {target}"), entity_id: Some(source_id.clone()), register: Some(register.into()) });
    }
}
// #endregion

// #region 🔖️Validate
/// @emoji 🩺️ Validates a plugin document and returns all diagnostics (non-fatal).
pub fn validate_plugin(program: &Program) -> Vec<ProgramDiagnostic> {
    let mut diagnostics = Vec::new();
    if program.schema != ARCHITECT_PROGRAM_SCHEMA {
        diagnostics.push(ProgramDiagnostic {
            severity: DiagnosticSeverity::Error,
            code: "schema.mismatch".into(),
            message: format!("expected schema {ARCHITECT_PROGRAM_SCHEMA}, got {}", program.schema),
            entity_id: None,
            register: Some("meta".into()),
        });
    }
    if program.meta.title.trim().is_empty() {
        diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Warning, code: "meta.empty_title".into(), message: "program title is empty".into(), entity_id: None, register: Some("meta".into()) });
    }

    let index = build_entity_index(program);
    for (id, first, second) in &index.duplicates {
        diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: "entity.duplicate_id".into(), message: format!("entity id {id} appears in both {first} and {second}"), entity_id: Some(id.clone()), register: None });
    }

    let element_ids: HashSet<EntityId> = program.elements.iter().map(|e| e.header.id.clone()).collect();

    for element in &program.elements {
        if let Some(parent) = &element.parent_id {
            check_ref(&mut diagnostics, &index, parent, &element.header.id, "elements", "element.missing_parent");
        }
        for id in &element.function_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_function");
        }
        for id in &element.activity_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_activity");
        }
        for id in &element.user_profile_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_user");
        }
        for id in &element.adjacency_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_adjacency");
        }
        for id in &element.quantity_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_quantity");
        }
        for id in &element.requirement_ids {
            check_ref(&mut diagnostics, &index, id, &element.header.id, "elements", "element.missing_requirement");
        }
    }

    for function in &program.functions {
        for id in &function.activity_ids {
            check_ref(&mut diagnostics, &index, id, &function.header.id, "functions", "function.missing_activity");
        }
        for id in &function.element_ids {
            check_ref(&mut diagnostics, &index, id, &function.header.id, "functions", "function.missing_element");
        }
        for id in &function.dependencies {
            check_ref(&mut diagnostics, &index, id, &function.header.id, "functions", "function.missing_dependency");
        }
        if let Some(parent) = &function.hierarchy_parent_id {
            check_ref(&mut diagnostics, &index, parent, &function.header.id, "functions", "function.missing_parent");
        }
    }

    for activity in &program.activities {
        for id in &activity.function_ids {
            check_ref(&mut diagnostics, &index, id, &activity.header.id, "activities", "activity.missing_function");
        }
        for id in &activity.adjacent_activities {
            check_ref(&mut diagnostics, &index, id, &activity.header.id, "activities", "activity.missing_adjacent_activity");
        }
        for id in &activity.user_profile_ids {
            check_ref(&mut diagnostics, &index, id, &activity.header.id, "activities", "activity.missing_user");
        }
        for id in &activity.equipment_ids {
            check_ref(&mut diagnostics, &index, id, &activity.header.id, "activities", "activity.missing_equipment");
        }
    }

    for requirement in &program.requirements {
        if requirement.element_ids.is_empty() && requirement.function_ids.is_empty() {
            diagnostics.push(ProgramDiagnostic {
                severity: DiagnosticSeverity::Warning,
                code: "requirement.orphan".into(),
                message: format!("requirement {} is not linked to elements or functions", requirement.header.id),
                entity_id: Some(requirement.header.id.clone()),
                register: Some("requirements".into()),
            });
        }
        if let Some(parent) = &requirement.parent_requirement_id {
            check_ref(&mut diagnostics, &index, parent, &requirement.header.id, "requirements", "requirement.missing_parent");
        }
        for id in &requirement.child_requirement_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_child");
        }
        for id in &requirement.stakeholder_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_stakeholder");
        }
        for id in &requirement.element_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_element");
        }
        for id in &requirement.function_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_function");
        }
        for id in &requirement.conflict_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_conflict");
        }
        for id in &requirement.risk_ids {
            check_ref(&mut diagnostics, &index, id, &requirement.header.id, "requirements", "requirement.missing_risk");
        }
        if let Some(superseded) = &requirement.superseded_by {
            check_ref(&mut diagnostics, &index, superseded, &requirement.header.id, "requirements", "requirement.missing_superseded_by");
        }
    }

    for relationship in &program.relationships {
        check_ref(&mut diagnostics, &index, &relationship.source_id, &relationship.header.id, "relationships", "relationship.missing_source");
        check_ref(&mut diagnostics, &index, &relationship.target_id, &relationship.header.id, "relationships", "relationship.missing_target");
    }

    for adjacency in &program.adjacencies {
        for endpoint in [&adjacency.element_a_id, &adjacency.element_b_id] {
            if !element_ids.contains(endpoint) {
                diagnostics.push(ProgramDiagnostic {
                    severity: DiagnosticSeverity::Error,
                    code: "adjacency.missing_element".into(),
                    message: format!("adjacency references missing element {endpoint}"),
                    entity_id: Some(adjacency.header.id.clone()),
                    register: Some("adjacencies".into()),
                });
            }
        }
        if let Some(rel_id) = &adjacency.source_relationship_id {
            check_ref(&mut diagnostics, &index, rel_id, &adjacency.header.id, "adjacencies", "adjacency.missing_relationship");
        }
    }

    for process in &program.processes {
        for id in &process.actors {
            check_ref(&mut diagnostics, &index, id, &process.header.id, "processes", "process.missing_actor");
        }
        for id in &process.equipment_ids {
            check_ref(&mut diagnostics, &index, id, &process.header.id, "processes", "process.missing_equipment");
        }
        for id in &process.element_ids {
            check_ref(&mut diagnostics, &index, id, &process.header.id, "processes", "process.missing_element");
        }
        for id in &process.dependencies {
            check_ref(&mut diagnostics, &index, id, &process.header.id, "processes", "process.missing_dependency");
        }
    }

    for equipment in &program.equipment {
        for id in &equipment.element_ids {
            check_ref(&mut diagnostics, &index, id, &equipment.header.id, "equipment", "equipment.missing_element");
        }
        for id in &equipment.activity_ids {
            check_ref(&mut diagnostics, &index, id, &equipment.header.id, "equipment", "equipment.missing_activity");
        }
    }

    for quantity in &program.quantities {
        check_ref(&mut diagnostics, &index, &quantity.target_element_id, &quantity.header.id, "quantities", "quantity.missing_element");
        for id in &quantity.related_requirement_ids {
            check_ref(&mut diagnostics, &index, id, &quantity.header.id, "quantities", "quantity.missing_requirement");
        }
    }

    for conflict in &program.conflicts {
        check_ref(&mut diagnostics, &index, &conflict.entity_a_id, &conflict.header.id, "conflicts", "conflict.missing_entity_a");
        check_ref(&mut diagnostics, &index, &conflict.entity_b_id, &conflict.header.id, "conflicts", "conflict.missing_entity_b");
        if conflict.entity_a_id == conflict.entity_b_id {
            diagnostics.push(ProgramDiagnostic {
                severity: DiagnosticSeverity::Error,
                code: "conflict.self_reference".into(),
                message: "conflict references the same entity for both sides".into(),
                entity_id: Some(conflict.header.id.clone()),
                register: Some("conflicts".into()),
            });
        }
        if let Some(decision_id) = &conflict.decision_id {
            check_ref(&mut diagnostics, &index, decision_id, &conflict.header.id, "conflicts", "conflict.missing_decision");
        }
    }

    for status in &program.status_records {
        check_ref(&mut diagnostics, &index, &status.subject_id, &status.header.id, "status_records", "status.missing_subject");
    }

    for validation in &program.validations {
        check_ref(&mut diagnostics, &index, &validation.subject_id, &validation.header.id, "validations", "validation.missing_subject");
        if validation.result == ValidationStatus::Failed && validation.corrective_actions.is_empty() {
            diagnostics.push(ProgramDiagnostic {
                severity: DiagnosticSeverity::Warning,
                code: "validation.failed_without_actions".into(),
                message: format!("validation {} failed without corrective actions", validation.header.id),
                entity_id: Some(validation.header.id.clone()),
                register: Some("validations".into()),
            });
        }
    }

    for conflict in detect_adjacency_conflicts(program) {
        diagnostics.push(ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: "adjacency.conflict".into(), message: conflict.message, entity_id: Some(conflict.adjacency_a_id), register: Some("adjacencies".into()) });
    }

    diagnostics
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::EntityHeader;
    use crate::artifacts::program::{empty_plugin, sample_plugin};
    use crate::artifacts::program::registers::Requirement;

    #[test]
    fn sample_plugin_passes_validation() {
        let diagnostics = validate_plugin(&sample_plugin());
        assert!(diagnostics.iter().all(|d| d.severity != DiagnosticSeverity::Error));
    }

    #[test]
    fn empty_plugin_warns_on_title() {
        let diagnostics = validate_plugin(&empty_plugin());
        assert!(diagnostics.iter().any(|d| d.code == "meta.empty_title"));
    }

    #[test]
    fn detects_orphan_requirement() {
        let mut program = sample_plugin();
        program.requirements.push(Requirement {
            header: EntityHeader::new(EntityId::new_serial("requirement", "Orphan"), "Orphan"),
            code: "OR-1".into(),
            kind: crate::artifacts::program::registers::RequirementKind::Functional,
            statement: crate::artifacts::program::kernel::TextField::plain("orphan req"),
            rationale: None,
            source: None,
            stakeholder_ids: Vec::new(),
            element_ids: Vec::new(),
            function_ids: Vec::new(),
            parent_requirement_id: None,
            child_requirement_ids: Vec::new(),
            acceptance_criteria: Vec::new(),
            verification_method: None,
            validation_status: ValidationStatus::Pending,
            conflict_ids: Vec::new(),
            risk_ids: Vec::new(),
            cost_estimate: None,
            schedule_constraint: None,
            regulatory_refs: Vec::new(),
            trace_links: Vec::new(),
            superseded_by: None,
        });
        let diagnostics = validate_plugin(&program);
        assert!(diagnostics.iter().any(|d| d.code == "requirement.orphan"));
    }

    #[test]
    fn detects_broken_relationship_target() {
        let mut program = sample_plugin();
        program.relationships.push(crate::artifacts::program::registers::Relationship {
            header: EntityHeader::new(EntityId::new_serial("relationship", "broken"), "broken"),
            source_id: program.elements[0].header.id.clone(),
            target_id: EntityId("missing-target".into()),
            kind: crate::artifacts::program::registers::RelationshipKind::DependsOn,
            strength: Some(1.0),
            directional: true,
            rationale: None,
            constraints: Vec::new(),
            conditions: Vec::new(),
            relationship_priority: crate::artifacts::program::kernel::Priority::Preferred,
            valid_from: None,
            valid_until: None,
            evidence: Vec::new(),
            conflict_ids: Vec::new(),
            trace_links: Vec::new(),
            bidirectional: false,
            distance_constraint_m: None,
            capacity_constraint: None,
            regulatory_basis: Vec::new(),
            review_cycle: None,
            owner_id: None,
            proximity_requirement: None,
            compatibility_requirement: None,
            incompatibility_requirement: None,
            separation_requirements: Vec::new(),
        });
        let diagnostics = validate_plugin(&program);
        assert!(diagnostics.iter().any(|d| d.code == "relationship.missing_target"));
    }
}