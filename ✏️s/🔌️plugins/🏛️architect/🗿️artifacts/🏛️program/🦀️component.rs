//! 🏛️ Architect program artifact — the root program document: all 65 feature-area registers plus
//! meta, project, and governance (constitutional: general).
//!
//! Domain row types live under `🧬️schema/🗄️registers`; shared entity primitives under
//! `🧬️schema/🧱️kernel`. The persisted snapshot type is `ProgramSnapshot`.

pub use crate::artifacts::program::kernel::*;
pub use crate::artifacts::program::registers::*;
pub use crate::artifacts::program::snapshot::schema::ProgramSnapshot;

#[cfg(test)]
use store::DocumentDsl;

/// @emoji 📜️ Persisted architect program document schema identifier.
pub const ARCHITECT_PROGRAM_SCHEMA: &str = "architect.program";


pub fn empty_plugin() -> ProgramSnapshot {
    let project_id = EntityId::new_serial("project", "project");
    let governance_id = EntityId::new_serial("governance", "governance");
    ProgramSnapshot {
        schema: ARCHITECT_PROGRAM_SCHEMA.into(),
        meta: ProgramMeta {
            schema: ARCHITECT_PROGRAM_SCHEMA.into(),
            document_id: EntityId::new_serial("document", "document").0,
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

/// @emoji 🧪️ Sample program for tests with elements, stakeholders, and one adjacency.
pub fn sample_plugin() -> ProgramSnapshot {
    let mut program = empty_plugin();
    program.meta.title = "Sample Clinic".into();
    program.meta.industry_sector = "healthcare".into();
    program.project.code = "CLN-001".into();
    program.project.client_name = "Sample Health".into();

    let reception_id = EntityId::new_serial("element", "element");
    let waiting_id = EntityId::new_serial("element", "element");
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

    let stakeholder_id = EntityId::new_serial("stakeholder", "stakeholder");
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

    let (a, b) = crate::artifacts::program::engine::adjacency::normalize_pair(&reception_id, &waiting_id);
    program.adjacencies.push(Adjacency {
        header: EntityHeader::new(EntityId::new_serial("adjacency", "Reception ↔ Waiting"), "Reception ↔ Waiting"),
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
    fn empty_plugin_has_schema() {
        let program = empty_plugin();
        assert_eq!(program.schema, ARCHITECT_PROGRAM_SCHEMA);
        assert_eq!(program.meta.schema, ARCHITECT_PROGRAM_SCHEMA);
    }

    #[test]
    fn sample_plugin_round_trips_json() {
        let program = sample_plugin();
        let json = serde_json::to_string(&program).expect("serialize");
        let decoded: ProgramSnapshot = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.elements.len(), 2);
        assert_eq!(decoded.adjacencies.len(), 1);
    }

    // #region 🔖️DslDocument
    #[test]
    fn empty_plugin_dsl_round_trips() {
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&empty_plugin());
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&empty_plugin());
    }

    #[test]
    fn sample_plugin_dsl_round_trips() {
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&sample_plugin());
    }

    #[test]
    // 🪲️ Blocked on a confirmed upstream `pack` crate bug, NOT an architect defect: table
    // rows (`#[dsl(table)] Vec<Stakeholder>` etc.) decode via `pack::value`'s self-describing
    fn sample_plugin_dsl_pack_equivalence() {
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&sample_plugin());
    }

    #[test]
    fn sample_plugin_dsl_text_is_parseable_and_reflects_registers() {
        let printed = sample_plugin().print_dsl();
        assert!(printed.contains("Sample Clinic"), "printed dsl text must contain program title: {printed}");
        assert!(printed.contains("REC"), "printed dsl text must contain the reception element code: {printed}");
    }

    /// @emoji 🧪️ The bundled `.architect` fixture (a static transcription of `sample_plugin()`)
    /// parses and round-trips — the compile-time validation ground truth for
    /// `ARCHITECT_EXAMPLE_TEXT`. Compared field-by-field rather than via `PartialEq` against a
    /// freshly called `sample_plugin()`, because `EntityId::new_serial` draws from a
    /// process-wide counter shared with every other test in this binary, so the serial ids a
    /// fresh call mints depend on test execution order and never match the fixture's baked-in ids.
    #[test]
    fn architect_example_text_parses_to_sample_plugin_and_round_trips() {
        let parsed = ProgramSnapshot::parse_dsl(crate::artifacts::program::dsl::ARCHITECT_EXAMPLE_TEXT).expect("parse bundled .architect example");
        let expected = sample_plugin();
        assert_eq!(parsed.meta.title, expected.meta.title);
        assert_eq!(parsed.meta.industry_sector, expected.meta.industry_sector);
        assert_eq!(parsed.project.code, expected.project.code);
        assert_eq!(parsed.project.client_name, expected.project.client_name);
        assert_eq!(parsed.stakeholders.len(), expected.stakeholders.len());
        assert_eq!(parsed.stakeholders[0].header.name, expected.stakeholders[0].header.name);
        assert_eq!(parsed.elements.len(), expected.elements.len());
        assert_eq!(parsed.elements[0].code, expected.elements[0].code);
        assert_eq!(parsed.elements[1].code, expected.elements[1].code);
        assert_eq!(parsed.adjacencies.len(), expected.adjacencies.len());
        assert_eq!(parsed.adjacencies[0].kind, expected.adjacencies[0].kind);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&parsed);
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&parsed);
    }
    // #endregion 🔖️DslDocument
}