//! 🏛️ Root program document — all 65 feature-area registers plus meta, project, and governance.

use crate::kernel::*;
use crate::registers::*;
use serde::{Deserialize, Serialize};

/// @emoji 📜 Persisted architect program document schema identifier.
pub const ARCHITECT_PROGRAM_SCHEMA: &str = "architect.program";

// #region 🔖Program
/// @emoji 🗂️ Full architectural program document with every typed register collection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    pub schema: String,
    pub meta: ProgramMeta,
    pub project: ProjectDefinition,
    pub stakeholders: Vec<Stakeholder>,
    pub users: Vec<UserProfile>,
    pub activities: Vec<Activity>,
    pub functions: Vec<Function>,
    pub elements: Vec<ProgramElement>,
    pub quantities: Vec<QuantityRequirement>,
    pub relationships: Vec<Relationship>,
    pub adjacencies: Vec<Adjacency>,
    pub processes: Vec<Process>,
    pub flows: Vec<FlowRequirement>,
    pub access_rules: Vec<AccessRule>,
    pub operations: Vec<OperationalRequirement>,
    pub equipment: Vec<Equipment>,
    pub resources: Vec<Resource>,
    pub storage: Vec<StorageRequirement>,
    pub environmental: Vec<EnvironmentalRequirement>,
    pub human_factors: Vec<HumanFactorRequirement>,
    pub accessibility: Vec<AccessibilityRequirement>,
    pub privacy: Vec<PrivacyRequirement>,
    pub safety: Vec<SafetyRequirement>,
    pub security: Vec<SecurityRequirement>,
    pub regulatory: Vec<RegulatoryRequirement>,
    pub site_context: Vec<SiteContext>,
    pub organizational: Vec<OrganizationalRequirement>,
    pub services: Vec<ServiceRequirement>,
    pub infrastructure: Vec<InfrastructureRequirement>,
    pub information: Vec<InformationRequirement>,
    pub communication: Vec<CommunicationRequirement>,
    pub wayfinding: Vec<WayfindingRequirement>,
    pub schedules: Vec<ScheduleRequirement>,
    pub flexibility: Vec<FlexibilityRequirement>,
    pub growth: Vec<GrowthPlan>,
    pub sustainability: Vec<SustainabilityRequirement>,
    pub resilience: Vec<ResilienceRequirement>,
    pub costs: Vec<CostRequirement>,
    pub delivery: Vec<DeliveryConstraint>,
    pub risks: Vec<Risk>,
    pub conflicts: Vec<Conflict>,
    pub requirements: Vec<Requirement>,
    pub priorities: Vec<PriorityRecord>,
    pub scenarios: Vec<Scenario>,
    pub options: Vec<OptionEvaluation>,
    pub decisions: Vec<Decision>,
    pub validations: Vec<ValidationRecord>,
    pub performance: Vec<PerformanceCriterion>,
    pub quality: Vec<QualityRecord>,
    pub documents: Vec<DocumentRecord>,
    pub assumptions: Vec<Assumption>,
    pub constraints: Vec<ConstraintRecord>,
    pub compliance_records: Vec<ComplianceRecord>,
    pub approvals: Vec<ApprovalRecord>,
    pub meetings: Vec<MeetingRecord>,
    pub changes: Vec<ChangeRecord>,
    pub collaboration: Vec<CollaborationRecord>,
    pub analyses: Vec<AnalysisRecord>,
    pub reports: Vec<ReportRecord>,
    pub search_filters: Vec<SearchFilter>,
    pub status_records: Vec<StatusRecord>,
    pub workshops: Vec<Workshop>,
    pub surveys: Vec<Survey>,
    pub issues: Vec<Issue>,
    pub audit_events: Vec<AuditEvent>,
    pub templates: Vec<TemplateRecord>,
    pub knowledge: Vec<KnowledgeRecord>,
    pub benchmarks: Vec<BenchmarkRecord>,
    pub governance: Governance,
    pub traces: Vec<TraceLink>,
}
// #endregion

// #region 🔖Factories
/// @emoji 📭 Empty program with schema, meta, project, and governance initialized.
pub fn empty_program() -> Program {
    let project_id = EntityId::new_serial("project");
    let governance_id = EntityId::new_serial("governance");
    Program {
        schema: ARCHITECT_PROGRAM_SCHEMA.into(),
        meta: ProgramMeta {
            schema: ARCHITECT_PROGRAM_SCHEMA.into(),
            document_id: EntityId::new_serial("document").0,
            title: String::new(),
            subtitle: None,
            purpose: TextField::plain(""),
            terminology: Vec::new(),
            classification: Vec::new(),
            industry_sector: String::new(),
            project_type: String::new(),
            locale: "en".into(),
            revision: "0".into(),
            author_ids: Vec::new(),
            source_system: None,
            export_profile: None,
            timestamps: TimestampMeta::default(),
        },
        project: ProjectDefinition {
            id: project_id,
            code: String::new(),
            client_name: String::new(),
            owner_organization: String::new(),
            brief_summary: TextField::plain(""),
            problem_statement: TextField::plain(""),
            vision: TextField::plain(""),
            mission: TextField::plain(""),
            objectives: Vec::new(),
            success_criteria: Vec::new(),
            project_priorities: Vec::new(),
            completion_criteria: Vec::new(),
            decision_criteria: Vec::new(),
            scope_inclusions: Vec::new(),
            scope_exclusions: Vec::new(),
            assumptions: Vec::new(),
            constraints_summary: Vec::new(),
            dependencies: Vec::new(),
            deliverables: Vec::new(),
            phases: Vec::new(),
            geographic_context: TextField::plain(""),
            development_context: TextField::plain(""),
            operational_context: TextField::plain(""),
            regulatory_context: Vec::new(),
            funding_model: String::new(),
            ownership: Ownership::default(),
            timestamps: TimestampMeta::default(),
        },
        stakeholders: Vec::new(),
        users: Vec::new(),
        activities: Vec::new(),
        functions: Vec::new(),
        elements: Vec::new(),
        quantities: Vec::new(),
        relationships: Vec::new(),
        adjacencies: Vec::new(),
        processes: Vec::new(),
        flows: Vec::new(),
        access_rules: Vec::new(),
        operations: Vec::new(),
        equipment: Vec::new(),
        resources: Vec::new(),
        storage: Vec::new(),
        environmental: Vec::new(),
        human_factors: Vec::new(),
        accessibility: Vec::new(),
        privacy: Vec::new(),
        safety: Vec::new(),
        security: Vec::new(),
        regulatory: Vec::new(),
        site_context: Vec::new(),
        organizational: Vec::new(),
        services: Vec::new(),
        infrastructure: Vec::new(),
        information: Vec::new(),
        communication: Vec::new(),
        wayfinding: Vec::new(),
        schedules: Vec::new(),
        flexibility: Vec::new(),
        growth: Vec::new(),
        sustainability: Vec::new(),
        resilience: Vec::new(),
        costs: Vec::new(),
        delivery: Vec::new(),
        risks: Vec::new(),
        conflicts: Vec::new(),
        requirements: Vec::new(),
        priorities: Vec::new(),
        scenarios: Vec::new(),
        options: Vec::new(),
        decisions: Vec::new(),
        validations: Vec::new(),
        performance: Vec::new(),
        quality: Vec::new(),
        documents: Vec::new(),
        assumptions: Vec::new(),
        constraints: Vec::new(),
        compliance_records: Vec::new(),
        approvals: Vec::new(),
        meetings: Vec::new(),
        changes: Vec::new(),
        collaboration: Vec::new(),
        analyses: Vec::new(),
        reports: Vec::new(),
        search_filters: Vec::new(),
        status_records: Vec::new(),
        workshops: Vec::new(),
        surveys: Vec::new(),
        issues: Vec::new(),
        audit_events: Vec::new(),
        templates: Vec::new(),
        knowledge: Vec::new(),
        benchmarks: Vec::new(),
        governance: Governance {
            id: governance_id,
            framework: String::new(),
            roles: Vec::new(),
            responsibilities: Vec::new(),
            approval_matrix: Vec::new(),
            escalation_paths: Vec::new(),
            meeting_cadence: Vec::new(),
            decision_rights: Vec::new(),
            change_control_process: Vec::new(),
            quality_policy: TextField::plain(""),
            risk_appetite: None,
            compliance_obligations: Vec::new(),
            audit_schedule: None,
            document_control: Vec::new(),
            stakeholder_engagement_plan: Vec::new(),
            ethics_policy: Vec::new(),
            data_governance: Vec::new(),
            owner_id: None,
            review_cycle: None,
            review_hierarchy: Vec::new(),
            policy_ownership_id: None,
            requirement_ownership_id: None,
            risk_ownership_id: None,
            reporting_frequency: None,
            accountability_rules: Vec::new(),
            exception_management: Vec::new(),
            governance_performance: Vec::new(),
        },
        traces: Vec::new(),
    }
}

/// @emoji 🧪 Sample program for tests with elements, stakeholders, and one adjacency.
pub fn sample_program() -> Program {
    let mut program = empty_program();
    program.meta.title = "Sample Clinic".into();
    program.meta.industry_sector = "healthcare".into();
    program.project.code = "CLN-001".into();
    program.project.client_name = "Sample Health".into();

    let reception_id = EntityId::new_serial("element");
    let waiting_id = EntityId::new_serial("element");
    program.elements.push(ProgramElement {
        header: EntityHeader::new(reception_id.clone(), "Reception"),
        code: "REC".into(),
        kind: ProgramElementKind::Room,
        parent_id: None,
        level: Some("L1".into()),
        area: QuantitySpec::target_unit(25.0, "m2"),
        volume: QuantitySpec::default(),
        height: QuantitySpec::default(),
        occupancy: QuantitySpec::target_unit(4.0, "persons"),
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
    });
    program.elements.push(ProgramElement {
        header: EntityHeader::new(waiting_id.clone(), "Waiting"),
        code: "WAI".into(),
        kind: ProgramElementKind::Room,
        parent_id: None,
        level: Some("L1".into()),
        area: QuantitySpec::target_unit(40.0, "m2"),
        volume: QuantitySpec::default(),
        height: QuantitySpec::default(),
        occupancy: QuantitySpec::target_unit(12.0, "persons"),
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
    });

    let stakeholder_id = EntityId::new_serial("stakeholder");
    program.stakeholders.push(Stakeholder {
        header: EntityHeader::new(stakeholder_id, "Facilities Director"),
        role: "Owner".into(),
        organization: "Sample Health".into(),
        department: None,
        contact_email: None,
        contact_phone: None,
        influence: InfluenceLevel::High,
        interest: InfluenceLevel::High,
        engagement: EngagementLevel::Leading,
        expectations: vec!["On-time delivery".into()],
        concerns: Vec::new(),
        requirement_ids: Vec::new(),
        decision_authority: true,
        communication_preferences: Vec::new(),
        reporting_frequency: None,
        involvement_phases: Vec::new(),
        availability: None,
        representative_of: None,
        delegated_to: None,
        relationship_to_client: None,
        power_interest_notes: Vec::new(),
        stakeholder_type: "Internal".into(),
        influence_strategy: None,
        communication_channels: Vec::new(),
        success_metrics: Vec::new(),
    });

    let (a, b) = crate::adjacency::normalize_pair(reception_id.clone(), waiting_id.clone());
    program.adjacencies.push(Adjacency {
        header: EntityHeader::new(EntityId::new_serial("adjacency"), "Reception ↔ Waiting"),
        element_a_id: a,
        element_b_id: b,
        kind: AdjacencyKind::Required,
        connection: ConnectionKind::Direct,
        separations: Vec::new(),
        weight: 1.0,
        rationale: None,
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
    });

    program
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_program_has_schema() {
        let program = empty_program();
        assert_eq!(program.schema, ARCHITECT_PROGRAM_SCHEMA);
        assert_eq!(program.meta.schema, ARCHITECT_PROGRAM_SCHEMA);
    }

    #[test]
    fn sample_program_round_trips_json() {
        let program = sample_program();
        let json = serde_json::to_string(&program).expect("serialize");
        let decoded: Program = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.elements.len(), 2);
        assert_eq!(decoded.adjacencies.len(), 1);
    }
}
