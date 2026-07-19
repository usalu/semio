//! 📋 Template application — sector and project templates into program registers.

use crate::adjacency::{normalize_pair, set_adjacency};
use crate::kernel::{EntityHeader, EntityId, TextField};
use crate::ops::ProgramOp;
use crate::program::Program;
use crate::registers::{
    Activity, Adjacency, AdjacencyKind, ConnectionKind, Equipment, Function, FunctionKind, Process,
    ProgramElement, ProgramElementKind, Requirement, RequirementKind, Risk, RiskLevel, Stakeholder,
    TemplateRecord, UserCategory, UserProfile, ValidationStatus,
};
use serde::{Deserialize, Serialize};
use vcs::CollectionOp;

// #region 🔖TemplateApply
/// @emoji 📋 Result of applying a template to a program.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateApplyResult {
    pub template_id: EntityId,
    pub created_entity_ids: Vec<EntityId>,
    pub messages: Vec<String>,
}

/// @emoji 🧩 Applies a template record and returns replayable `ProgramOp`s.
pub fn apply_template(program: &mut Program, template: &TemplateRecord) -> Vec<ProgramOp> {
    let mut ops = Vec::new();
    let mut element_ids = Vec::new();
    for field in &template.default_fields {
        let id = EntityId::new_serial("template-entity");
        match field.as_str() {
            "stakeholder" => {
                let item = Stakeholder {
                    header: EntityHeader::new(id.clone(), format!("{} Stakeholder", template.header.name)),
                    role: "Template".into(),
                    organization: template.source_organization.clone().unwrap_or_default(),
                    department: None,
                    contact_email: None,
                    contact_phone: None,
                    influence: crate::registers::InfluenceLevel::Medium,
                    interest: crate::registers::InfluenceLevel::Medium,
                    engagement: crate::registers::EngagementLevel::Neutral,
                    expectations: template.checklists.clone(),
                    concerns: Vec::new(),
                    requirement_ids: Vec::new(),
                    decision_authority: false,
                    communication_preferences: Vec::new(),
                    reporting_frequency: None,
                    involvement_phases: Vec::new(),
                    availability: None,
                    representative_of: None,
                    delegated_to: None,
                    relationship_to_client: None,
                    power_interest_notes: Vec::new(),
                    stakeholder_type: "Template".into(),
                    influence_strategy: None,
                    communication_channels: Vec::new(),
                    success_metrics: Vec::new(),
                };
                ops.push(ProgramOp::Stakeholders(CollectionOp::Add {
                    index: program.stakeholders.len(),
                    item: item.clone(),
                }));
                program.stakeholders.push(item);
            }
            "user" => {
                let item = UserProfile {
                    header: EntityHeader::new(id.clone(), format!("{} User", template.header.name)),
                    category: UserCategory::Primary,
                    demographic: None,
                    age_range: None,
                    abilities: Vec::new(),
                    disabilities: Vec::new(),
                    occupation: None,
                    role_title: None,
                    department: None,
                    mobility_profile: Vec::new(),
                    sensory_profile: Vec::new(),
                    cognitive_profile: Vec::new(),
                    behavioral_patterns: Vec::new(),
                    usage_frequency: None,
                    usage_duration: None,
                    peak_usage_times: Vec::new(),
                    technology_proficiency: None,
                    preferences: Vec::new(),
                    pain_points: Vec::new(),
                    goals: template.checklists.clone(),
                    activity_ids: Vec::new(),
                    research_method: None,
                    persona_archetype: None,
                    validated: false,
                    stakeholder_ids: Vec::new(),
                };
                ops.push(ProgramOp::Users(CollectionOp::Add {
                    index: program.users.len(),
                    item: item.clone(),
                }));
                program.users.push(item);
            }
            "activity" => {
                let item = Activity {
                    header: EntityHeader::new(id.clone(), format!("{} Activity", template.header.name)),
                    code: "ACT".into(),
                    category: template.sector.clone().unwrap_or_else(|| "general".into()),
                    frequency: None,
                    duration: None,
                    intensity: None,
                    participants: crate::kernel::QuantitySpec::default(),
                    equipment_ids: Vec::new(),
                    space_requirements: Vec::new(),
                    environmental_needs: Vec::new(),
                    privacy_needs: Vec::new(),
                    accessibility_needs: Vec::new(),
                    adjacent_activities: Vec::new(),
                    sequencing: Vec::new(),
                    peak_periods: Vec::new(),
                    workflow_steps: template.checklists.clone(),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    user_profile_ids: Vec::new(),
                    function_ids: Vec::new(),
                    performance_indicators: Vec::new(),
                    activity_type: "template".into(),
                    location_context: None,
                    temporal_pattern: None,
                    supervision_level: None,
                };
                ops.push(ProgramOp::Activities(CollectionOp::Add {
                    index: program.activities.len(),
                    item: item.clone(),
                }));
                program.activities.push(item);
            }
            "function" => {
                let item = Function {
                    header: EntityHeader::new(id.clone(), format!("{} Function", template.header.name)),
                    code: "FN".into(),
                    kind: FunctionKind::Primary,
                    purpose: TextField::plain(template.standards.join(", ")),
                    criticality: crate::kernel::Priority::Preferred,
                    performance_targets: Vec::new(),
                    service_level: None,
                    operating_hours: None,
                    staffing: crate::kernel::QuantitySpec::default(),
                    equipment_ids: Vec::new(),
                    resource_ids: Vec::new(),
                    activity_ids: Vec::new(),
                    element_ids: Vec::new(),
                    dependencies: Vec::new(),
                    interfaces: Vec::new(),
                    constraints: Vec::new(),
                    quality_criteria: Vec::new(),
                    regulatory_refs: template.standards.clone(),
                    future_changes: Vec::new(),
                    owner_stakeholder_id: None,
                    success_metrics: Vec::new(),
                    hierarchy_parent_id: None,
                    conflict_ids: Vec::new(),
                };
                ops.push(ProgramOp::Functions(CollectionOp::Add {
                    index: program.functions.len(),
                    item: item.clone(),
                }));
                program.functions.push(item);
            }
            "element" | "room" => {
                let item = ProgramElement {
                    header: EntityHeader::new(id.clone(), format!("{} Space", template.header.name)),
                    code: "TPL".into(),
                    kind: ProgramElementKind::Room,
                    parent_id: None,
                    level: None,
                    area: crate::kernel::QuantitySpec::default(),
                    volume: crate::kernel::QuantitySpec::default(),
                    height: crate::kernel::QuantitySpec::default(),
                    occupancy: crate::kernel::QuantitySpec::default(),
                    function_ids: Vec::new(),
                    activity_ids: Vec::new(),
                    user_profile_ids: Vec::new(),
                    adjacency_ids: Vec::new(),
                    quantity_ids: Vec::new(),
                    requirement_ids: Vec::new(),
                    location_hint: None,
                    orientation: None,
                    daylight_requirement: None,
                    acoustic_class: None,
                    security_zone: None,
                    flexibility_notes: Vec::new(),
                    growth_allocation: None,
                    circulation_role: None,
                    visibility_level: None,
                    adjacency_preferences: Vec::new(),
                    environmental_zone: None,
                };
                ops.push(ProgramOp::Elements(CollectionOp::Add {
                    index: program.elements.len(),
                    item: item.clone(),
                }));
                program.elements.push(item);
                element_ids.push(id.clone());
            }
            "requirement" => {
                let item = Requirement {
                    header: EntityHeader::new(id.clone(), format!("{} Requirement", template.header.name)),
                    code: String::new(),
                    kind: RequirementKind::Functional,
                    statement: TextField::plain(template.standards.join(", ")),
                    rationale: None,
                    source: template.source_organization.clone(),
                    stakeholder_ids: Vec::new(),
                    element_ids: Vec::new(),
                    function_ids: Vec::new(),
                    parent_requirement_id: None,
                    child_requirement_ids: Vec::new(),
                    acceptance_criteria: template.checklists.clone(),
                    verification_method: None,
                    validation_status: ValidationStatus::Pending,
                    conflict_ids: Vec::new(),
                    risk_ids: Vec::new(),
                    cost_estimate: None,
                    schedule_constraint: None,
                    regulatory_refs: template.standards.clone(),
                    trace_links: Vec::new(),
                    superseded_by: None,
                };
                ops.push(ProgramOp::Requirements(CollectionOp::Add {
                    index: program.requirements.len(),
                    item: item.clone(),
                }));
                program.requirements.push(item);
            }
            "risk" => {
                let item = Risk {
                    header: EntityHeader::new(id.clone(), format!("{} Risk", template.header.name)),
                    risk_statement: TextField::plain(template.checklists.join("; ")),
                    category: "template".into(),
                    probability: RiskLevel::Medium,
                    impact: RiskLevel::Medium,
                    risk_score: None,
                    causes: Vec::new(),
                    effects: Vec::new(),
                    affected_element_ids: Vec::new(),
                    affected_requirement_ids: Vec::new(),
                    mitigation: Vec::new(),
                    contingency: Vec::new(),
                    owner_id: None,
                    review_date: None,
                    trigger_indicators: Vec::new(),
                    residual_probability: None,
                    residual_impact: None,
                    related_conflict_ids: Vec::new(),
                    escalation_path: Vec::new(),
                    monitoring_plan: None,
                };
                ops.push(ProgramOp::Risks(CollectionOp::Add {
                    index: program.risks.len(),
                    item: item.clone(),
                }));
                program.risks.push(item);
            }
            "process" => {
                let item = Process {
                    header: EntityHeader::new(id.clone(), format!("{} Process", template.header.name)),
                    code: "PRC".into(),
                    category: template.sector.clone().unwrap_or_else(|| "general".into()),
                    trigger: None,
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    steps: template.checklists.clone(),
                    actors: Vec::new(),
                    equipment_ids: Vec::new(),
                    element_ids: Vec::new(),
                    duration: None,
                    frequency: None,
                    critical_path: false,
                    bottlenecks: Vec::new(),
                    dependencies: Vec::new(),
                    kpis: Vec::new(),
                    automation_level: None,
                    failure_modes: Vec::new(),
                    improvement_opportunities: Vec::new(),
                    regulatory_refs: template.standards.clone(),
                    owner_id: None,
                    workflow_type: Some("template".into()),
                    handoff_points: Vec::new(),
                    quality_gates: Vec::new(),
                };
                ops.push(ProgramOp::Processes(CollectionOp::Add {
                    index: program.processes.len(),
                    item: item.clone(),
                }));
                program.processes.push(item);
            }
            "equipment" => {
                let item = Equipment {
                    header: EntityHeader::new(id.clone(), format!("{} Equipment", template.header.name)),
                    code: "EQ".into(),
                    category: template.sector.clone().unwrap_or_else(|| "general".into()),
                    manufacturer: None,
                    model: None,
                    quantity: crate::kernel::QuantitySpec::default(),
                    dimensions: None,
                    weight_kg: None,
                    power_kw: None,
                    utility_connections: Vec::new(),
                    ventilation: None,
                    noise_level_db: None,
                    clearance: None,
                    mounting: None,
                    element_ids: Vec::new(),
                    activity_ids: Vec::new(),
                    maintenance_access: Vec::new(),
                    lifecycle_years: None,
                    replacement_cost: None,
                    standards: template.standards.clone(),
                    supplier: None,
                    activity_link_ids: Vec::new(),
                    installation_requirements: Vec::new(),
                    commissioning_notes: Vec::new(),
                    spare_parts: Vec::new(),
                };
                ops.push(ProgramOp::Equipment(CollectionOp::Add {
                    index: program.equipment.len(),
                    item: item.clone(),
                }));
                program.equipment.push(item);
            }
            "adjacency" | "adjacency_bundle" => {
                if element_ids.len() >= 2 {
                    let (a, b) = normalize_pair(element_ids[0].clone(), element_ids[1].clone());
                    let adjacency = Adjacency {
                        header: EntityHeader::new(id.clone(), format!("{} Adjacency", template.header.name)),
                        element_a_id: a,
                        element_b_id: b,
                        kind: AdjacencyKind::Preferred,
                        connection: ConnectionKind::Direct,
                        separations: Vec::new(),
                        weight: 1.0,
                        rationale: Some(TextField::plain("template bundle")),
                        distance_max_m: None,
                        distance_min_m: None,
                        level_constraint: None,
                        access_path: None,
                        shared_wall: true,
                        shared_entry: false,
                        traffic_isolation: false,
                        circulation_overlap: true,
                        conflict_ids: Vec::new(),
                        normalized: true,
                        verification_status: ValidationStatus::Pending,
                        source_relationship_id: None,
                        internal_external_access: None,
                    };
                    ops.push(ProgramOp::SetAdjacency {
                        adjacency: adjacency.clone(),
                    });
                    set_adjacency(program, adjacency);
                }
            }
            _ => {}
        }
    }
    if let Some(existing) = program.templates.iter_mut().find(|t| t.header.id == template.header.id) {
        existing.usage_count += 1;
        existing.last_applied = Some(program.meta.timestamps.updated.clone());
    }
    ops
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::apply_program_op;
    use crate::program::empty_program;
    use crate::registers::TemplateRecord;

    #[test]
    fn apply_template_returns_program_ops() {
        let mut program = empty_program();
        let template = TemplateRecord {
            header: EntityHeader::new(EntityId::new_serial("template"), "Clinic Starter"),
            template_type: "sector".into(),
            sector: Some("healthcare".into()),
            project_type: None,
            version: "1".into(),
            content_ref: None,
            entity_kinds: vec!["stakeholder".into(), "element".into()],
            default_fields: vec!["stakeholder".into(), "room".into(), "requirement".into()],
            checklists: vec!["intake".into()],
            standards: vec!["ISO-41001".into()],
            applicability: Vec::new(),
            author_id: None,
            approval_status: ValidationStatus::Passed,
            usage_count: 0,
            last_applied: None,
            customization_notes: Vec::new(),
            related_knowledge_ids: Vec::new(),
            benchmark_ids: Vec::new(),
            license: None,
            source_organization: Some("Semio".into()),
        };
        let ops = apply_template(&mut program, &template);
        assert!(!ops.is_empty());
        assert_eq!(program.stakeholders.len(), 1);
        assert_eq!(program.elements.len(), 1);
        assert_eq!(program.requirements.len(), 1);
    }

    #[test]
    fn template_ops_replay_on_empty_program() {
        let mut source = empty_program();
        let template = TemplateRecord {
            header: EntityHeader::new(EntityId::new_serial("template"), "Replay"),
            template_type: "sector".into(),
            sector: None,
            project_type: None,
            version: "1".into(),
            content_ref: None,
            entity_kinds: vec!["function".into()],
            default_fields: vec!["function".into()],
            checklists: Vec::new(),
            standards: Vec::new(),
            applicability: Vec::new(),
            author_id: None,
            approval_status: ValidationStatus::Passed,
            usage_count: 0,
            last_applied: None,
            customization_notes: Vec::new(),
            related_knowledge_ids: Vec::new(),
            benchmark_ids: Vec::new(),
            license: None,
            source_organization: None,
        };
        let ops = apply_template(&mut source, &template);
        let mut target = empty_program();
        for op in &ops {
            apply_program_op(&mut target, op);
        }
        assert_eq!(target.functions.len(), 1);
    }
}
