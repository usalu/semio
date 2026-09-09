use super::*;
use crate::kernel::EntityHeader;
use crate::registers::Requirement;
use crate::{empty_plugin, sample_plugin};

#[semio_framework_async_macros::async_test]
async fn sample_plugin_passes_validation() {
    let diagnostics = validate_plugin(&sample_plugin());
    assert!(diagnostics.iter().all(|d| d.severity != DiagnosticSeverity::Error));
}

#[semio_framework_async_macros::async_test]
async fn empty_plugin_warns_on_title() {
    let diagnostics = validate_plugin(&empty_plugin());
    assert!(diagnostics.iter().any(|d| d.code == "meta.empty_title"));
}

#[semio_framework_async_macros::async_test]
async fn detects_orphan_requirement() {
    let mut program = sample_plugin();
    program.requirements.push(Requirement {
        header: EntityHeader::new(EntityId::new_serial("requirement", "Orphan"), "Orphan"),
        code: "OR-1".into(),
        kind: crate::registers::RequirementKind::Functional,
        statement: crate::kernel::TextField::plain("orphan req"),
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

#[semio_framework_async_macros::async_test]
async fn detects_broken_relationship_target() {
    let mut program = sample_plugin();
    program.relationships.push(crate::registers::Relationship {
        header: EntityHeader::new(EntityId::new_serial("relationship", "broken"), "broken"),
        source_id: program.elements[0].header.id.clone(),
        target_id: EntityId("missing-target".into()),
        kind: RelationshipKind::DependsOn,
        strength: Some(1.0),
        directional: true,
        rationale: None,
        constraints: Vec::new(),
        conditions: Vec::new(),
        relationship_priority: Priority::Preferred,
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
