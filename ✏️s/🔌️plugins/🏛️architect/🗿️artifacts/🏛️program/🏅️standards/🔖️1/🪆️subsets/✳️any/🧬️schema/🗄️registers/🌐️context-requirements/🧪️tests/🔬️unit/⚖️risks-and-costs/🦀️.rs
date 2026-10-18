/// ⚖️ Resilience, cost, delivery, risk and conflict patches: the patch round trips of the resilience and cost requirements, the delivery constraint, the risk and the conflict through the central applier.

#[semio_framework_async_macros::async_test]
async fn resilience_requirement_patch_round_trips() {
    let mut item = ResilienceRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("resiliencerequirement", "Base ResilienceRequirement"), "Base ResilienceRequirement") },
        hazard: String::new(),
        risk_level: RiskLevel::Negligible,
        scenario: Some(String::new()),
        recovery_time: Some(String::new()),
        recovery_point: Some(String::new()),
        redundancy: Vec::new(),
        hardening_measures: Vec::new(),
        backup_systems: Vec::new(),
        alternate_sites: Vec::new(),
        supply_chain: Vec::new(),
        communication_plan: Vec::new(),
        drill_requirements: Vec::new(),
        element_ids: Vec::new(),
        infrastructure_ids: Vec::new(),
        standards: Vec::new(),
        insurance_implications: Vec::new(),
        climate_adaptation: Vec::new(),
        owner_id: Some(EntityId::new_serial("base33", "base33")),
        verification_plan: Some(String::new()),
    };
    let original = item.clone();
    let patch = ResilienceRequirementPatch {
        name: Some("Patched ResilienceRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        hazard: Some("patched-33".to_string()),
        risk_level: Some(RiskLevel::Low),
        scenario: Some("patched-33".to_string()),
        recovery_time: Some("patched-33".to_string()),
        recovery_point: Some("patched-33".to_string()),
        redundancy: Some(vec!["patched-33".to_string()]),
        hardening_measures: Some(vec!["patched-33".to_string()]),
        backup_systems: Some(vec!["patched-33".to_string()]),
        alternate_sites: Some(vec!["patched-33".to_string()]),
        supply_chain: Some(vec!["patched-33".to_string()]),
        communication_plan: Some(vec!["patched-33".to_string()]),
        drill_requirements: Some(vec!["patched-33".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new33", "new33")]),
        infrastructure_ids: Some(vec![EntityId::new_serial("new33", "new33")]),
        standards: Some(vec!["patched-33".to_string()]),
        insurance_implications: Some(vec!["patched-33".to_string()]),
        climate_adaptation: Some(vec!["patched-33".to_string()]),
        owner_id: Some(EntityId::new_serial("new33", "new33")),
        verification_plan: Some("patched-33".to_string()),
    };
    patch_through_program!(item, patch, resilience, crate::standards::v1::subsets::any::schema::diff::ProgramResilienceDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ResilienceRequirement");
    patch_through_program!(item, inverse, resilience, crate::standards::v1::subsets::any::schema::diff::ProgramResilienceDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn cost_requirement_patch_round_trips() {
    let mut item = CostRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("costrequirement", "Base CostRequirement"), "Base CostRequirement") },
        cost_item: String::new(),
        basis: CostBasis::Capital,
        amount: Some(0.0),
        currency: String::new(),
        quantity_basis: Some(String::new()),
        unit_cost: Some(0.0),
        contingency_percent: Some(0.0),
        escalation_rate: Some(0.0),
        funding_source: Some(String::new()),
        element_ids: Vec::new(),
        requirement_ids: Vec::new(),
        phase: Some(DeliveryPhase::Concept),
        cash_flow_profile: Vec::new(),
        value_engineering_notes: Vec::new(),
        benchmark_ref: Some(EntityId::new_serial("base34", "base34")),
        approval_status: ValidationStatus::Pending,
        owner_id: Some(EntityId::new_serial("base34", "base34")),
        assumptions: Vec::new(),
        sensitivity_factors: Vec::new(),
    };
    let original = item.clone();
    let patch = CostRequirementPatch {
        name: Some("Patched CostRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        cost_item: Some("patched-34".to_string()),
        basis: Some(CostBasis::Operational),
        amount: Some(42.0),
        currency: Some("patched-34".to_string()),
        quantity_basis: Some("patched-34".to_string()),
        unit_cost: Some(42.0),
        contingency_percent: Some(42.0),
        escalation_rate: Some(42.0),
        funding_source: Some("patched-34".to_string()),
        element_ids: Some(vec![EntityId::new_serial("new34", "new34")]),
        requirement_ids: Some(vec![EntityId::new_serial("new34", "new34")]),
        phase: Some(DeliveryPhase::Schematic),
        cash_flow_profile: Some(vec!["patched-34".to_string()]),
        value_engineering_notes: Some(vec!["patched-34".to_string()]),
        benchmark_ref: Some(EntityId::new_serial("new34", "new34")),
        approval_status: Some(ValidationStatus::Passed),
        owner_id: Some(EntityId::new_serial("new34", "new34")),
        assumptions: Some(vec!["patched-34".to_string()]),
        sensitivity_factors: Some(vec!["patched-34".to_string()]),
    };
    patch_through_program!(item, patch, costs, crate::standards::v1::subsets::any::schema::diff::ProgramCostsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched CostRequirement");
    patch_through_program!(item, inverse, costs, crate::standards::v1::subsets::any::schema::diff::ProgramCostsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn delivery_constraint_patch_round_trips() {
    let mut item = DeliveryConstraint {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("deliveryconstraint", "Base DeliveryConstraint"), "Base DeliveryConstraint") },
        constraint_type: String::new(),
        constraint_details: TextField::default(),
        phase: DeliveryPhase::Concept,
        hard_deadline: Some(String::new()),
        soft_deadline: Some(String::new()),
        impacted_element_ids: Vec::new(),
        impacted_requirement_ids: Vec::new(),
        work_hours: Some(String::new()),
        noise_restrictions: Vec::new(),
        access_restrictions: Vec::new(),
        site_logistics: Vec::new(),
        procurement_lead_time: Some(String::new()),
        approval_gates: Vec::new(),
        occupancy_constraints: Vec::new(),
        weather_windows: Vec::new(),
        penalty_clauses: Vec::new(),
        mitigation_options: Vec::new(),
        owner_id: Some(EntityId::new_serial("base35", "base35")),
        risk_ids: Vec::new(),
        constraint_status: LifecycleStatus::Draft,
    };
    let original = item.clone();
    let patch = DeliveryConstraintPatch {
        name: Some("Patched DeliveryConstraint".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        constraint_type: Some("patched-35".to_string()),
        constraint_details: Some(TextField::plain("patched-35")),
        phase: Some(DeliveryPhase::Schematic),
        hard_deadline: Some("patched-35".to_string()),
        soft_deadline: Some("patched-35".to_string()),
        impacted_element_ids: Some(vec![EntityId::new_serial("new35", "new35")]),
        impacted_requirement_ids: Some(vec![EntityId::new_serial("new35", "new35")]),
        work_hours: Some("patched-35".to_string()),
        noise_restrictions: Some(vec!["patched-35".to_string()]),
        access_restrictions: Some(vec!["patched-35".to_string()]),
        site_logistics: Some(vec!["patched-35".to_string()]),
        procurement_lead_time: Some("patched-35".to_string()),
        approval_gates: Some(vec!["patched-35".to_string()]),
        occupancy_constraints: Some(vec!["patched-35".to_string()]),
        weather_windows: Some(vec!["patched-35".to_string()]),
        penalty_clauses: Some(vec!["patched-35".to_string()]),
        mitigation_options: Some(vec!["patched-35".to_string()]),
        owner_id: Some(EntityId::new_serial("new35", "new35")),
        risk_ids: Some(vec![EntityId::new_serial("new35", "new35")]),
        constraint_status: Some(LifecycleStatus::Proposed),
    };
    patch_through_program!(item, patch, delivery, crate::standards::v1::subsets::any::schema::diff::ProgramDeliveryDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched DeliveryConstraint");
    patch_through_program!(item, inverse, delivery, crate::standards::v1::subsets::any::schema::diff::ProgramDeliveryDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn risk_patch_round_trips() {
    let mut item = Risk {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("risk", "Base Risk"), "Base Risk") },
        risk_statement: TextField::default(),
        category: String::new(),
        probability: RiskLevel::Negligible,
        impact: RiskLevel::Negligible,
        risk_score: Some(0.0),
        causes: Vec::new(),
        effects: Vec::new(),
        affected_element_ids: Vec::new(),
        affected_requirement_ids: Vec::new(),
        mitigation: Vec::new(),
        contingency: Vec::new(),
        owner_id: Some(EntityId::new_serial("base36", "base36")),
        review_date: Some(String::new()),
        trigger_indicators: Vec::new(),
        residual_probability: Some(RiskLevel::Negligible),
        residual_impact: Some(RiskLevel::Negligible),
        related_conflict_ids: Vec::new(),
        escalation_path: Vec::new(),
        monitoring_plan: Some(String::new()),
    };
    let original = item.clone();
    let patch = RiskPatch {
        name: Some("Patched Risk".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        risk_statement: Some(TextField::plain("patched-36")),
        category: Some("patched-36".to_string()),
        probability: Some(RiskLevel::Low),
        impact: Some(RiskLevel::Low),
        risk_score: Some(42.0),
        causes: Some(vec!["patched-36".to_string()]),
        effects: Some(vec!["patched-36".to_string()]),
        affected_element_ids: Some(vec![EntityId::new_serial("new36", "new36")]),
        affected_requirement_ids: Some(vec![EntityId::new_serial("new36", "new36")]),
        mitigation: Some(vec!["patched-36".to_string()]),
        contingency: Some(vec!["patched-36".to_string()]),
        owner_id: Some(EntityId::new_serial("new36", "new36")),
        review_date: Some("patched-36".to_string()),
        trigger_indicators: Some(vec!["patched-36".to_string()]),
        residual_probability: Some(RiskLevel::Low),
        residual_impact: Some(RiskLevel::Low),
        related_conflict_ids: Some(vec![EntityId::new_serial("new36", "new36")]),
        escalation_path: Some(vec!["patched-36".to_string()]),
        monitoring_plan: Some("patched-36".to_string()),
    };
    patch_through_program!(item, patch, risks, crate::standards::v1::subsets::any::schema::diff::ProgramRisksDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Risk");
    patch_through_program!(item, inverse, risks, crate::standards::v1::subsets::any::schema::diff::ProgramRisksDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn conflict_patch_round_trips() {
    let mut item = Conflict {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("conflict", "Base Conflict"), "Base Conflict") },
        kind: ConflictKind::Adjacency,
        summary: TextField::default(),
        entity_a_id: EntityId::new_serial("base37", "base37"),
        entity_b_id: EntityId::new_serial("base37", "base37"),
        severity: IssueSeverity::Cosmetic,
        detected_by: Some(String::new()),
        detection_date: Some(String::new()),
        trade_off_options: Vec::new(),
        recommended_resolution: Some(TextField::default()),
        decision_id: Some(EntityId::new_serial("base37", "base37")),
        stakeholder_ids: Vec::new(),
        requirement_ids: Vec::new(),
        cost_impact: Some(0.0),
        schedule_impact: Some(String::new()),
        quality_impact: Vec::new(),
        resolution_status: ValidationStatus::Pending,
        owner_id: Some(EntityId::new_serial("base37", "base37")),
        escalation_level: Some(String::new()),
        related_risk_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = ConflictPatch {
        name: Some("Patched Conflict".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        kind: Some(ConflictKind::Capacity),
        summary: Some(TextField::plain("patched-37")),
        entity_a_id: Some(EntityId::new_serial("new37", "new37")),
        entity_b_id: Some(EntityId::new_serial("new37", "new37")),
        severity: Some(IssueSeverity::Minor),
        detected_by: Some("patched-37".to_string()),
        detection_date: Some("patched-37".to_string()),
        trade_off_options: Some(vec!["patched-37".to_string()]),
        recommended_resolution: Some(TextField::plain("patched-37")),
        decision_id: Some(EntityId::new_serial("new37", "new37")),
        stakeholder_ids: Some(vec![EntityId::new_serial("new37", "new37")]),
        requirement_ids: Some(vec![EntityId::new_serial("new37", "new37")]),
        cost_impact: Some(42.0),
        schedule_impact: Some("patched-37".to_string()),
        quality_impact: Some(vec!["patched-37".to_string()]),
        resolution_status: Some(ValidationStatus::Passed),
        owner_id: Some(EntityId::new_serial("new37", "new37")),
        escalation_level: Some("patched-37".to_string()),
        related_risk_ids: Some(vec![EntityId::new_serial("new37", "new37")]),
    };
    patch_through_program!(item, patch, conflicts, crate::standards::v1::subsets::any::schema::diff::ProgramConflictsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Conflict");
    patch_through_program!(item, inverse, conflicts, crate::standards::v1::subsets::any::schema::diff::ProgramConflictsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}
