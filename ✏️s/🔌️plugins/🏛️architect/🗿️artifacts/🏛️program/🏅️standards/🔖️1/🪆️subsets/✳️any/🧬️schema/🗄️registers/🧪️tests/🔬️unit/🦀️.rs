
use super::*;

#[semio_framework_async_macros::async_test]
async fn stakeholder_patch_round_trips() {
    let mut item = Stakeholder {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("stakeholder", "Base Stakeholder"), "Base Stakeholder") },
        role: String::new(),
        organization: String::new(),
        department: Some(String::new()),
        contact_email: Some(String::new()),
        contact_phone: Some(String::new()),
        influence: InfluenceLevel::Low,
        interest: InfluenceLevel::Low,
        engagement: EngagementLevel::Unaware,
        expectations: Vec::new(),
        concerns: Vec::new(),
        requirement_ids: Vec::new(),
        decision_authority: false,
        communication_preferences: Vec::new(),
        reporting_frequency: Some(String::new()),
        involvement_phases: Vec::new(),
        availability: Some(String::new()),
        representative_of: Some(EntityId::new_serial("base0", "base0")),
        delegated_to: Some(EntityId::new_serial("base0", "base0")),
        relationship_to_client: Some(String::new()),
        power_interest_notes: Vec::new(),
        stakeholder_type: String::new(),
        influence_strategy: Some(String::new()),
        communication_channels: Vec::new(),
        success_metrics: Vec::new(),
    };
    let original = item.clone();
    let patch = StakeholderPatch {
        name: Some("Patched Stakeholder".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        role: Some("patched-0".to_string()),
        organization: Some("patched-0".to_string()),
        department: Some("patched-0".to_string()),
        contact_email: Some("patched-0".to_string()),
        contact_phone: Some("patched-0".to_string()),
        influence: Some(InfluenceLevel::Medium),
        interest: Some(InfluenceLevel::Medium),
        engagement: Some(EngagementLevel::Resistant),
        expectations: Some(vec!["patched-0".to_string()]),
        concerns: Some(vec!["patched-0".to_string()]),
        requirement_ids: Some(vec![EntityId::new_serial("new0", "new0")]),
        decision_authority: Some(true),
        communication_preferences: Some(vec!["patched-0".to_string()]),
        reporting_frequency: Some("patched-0".to_string()),
        involvement_phases: Some(vec!["patched-0".to_string()]),
        availability: Some("patched-0".to_string()),
        representative_of: Some(EntityId::new_serial("new0", "new0")),
        delegated_to: Some(EntityId::new_serial("new0", "new0")),
        relationship_to_client: Some("patched-0".to_string()),
        power_interest_notes: Some(vec![TaggedNote { tag: "new0".into(), text: "new-note0".into() }]),
        stakeholder_type: Some("patched-0".to_string()),
        influence_strategy: Some("patched-0".to_string()),
        communication_channels: Some(vec!["patched-0".to_string()]),
        success_metrics: Some(vec!["patched-0".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Stakeholder");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn user_profile_patch_round_trips() {
    let mut item = UserProfile {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("userprofile", "Base UserProfile"), "Base UserProfile") },
        category: UserCategory::Primary,
        demographic: Some(String::new()),
        age_range: Some(String::new()),
        abilities: Vec::new(),
        disabilities: Vec::new(),
        occupation: Some(String::new()),
        role_title: Some(String::new()),
        department: Some(String::new()),
        mobility_profile: Vec::new(),
        sensory_profile: Vec::new(),
        cognitive_profile: Vec::new(),
        behavioral_patterns: Vec::new(),
        usage_frequency: Some(String::new()),
        usage_duration: Some(String::new()),
        peak_usage_times: Vec::new(),
        technology_proficiency: Some(String::new()),
        preferences: Vec::new(),
        pain_points: Vec::new(),
        goals: Vec::new(),
        activity_ids: Vec::new(),
        research_method: Some(String::new()),
        persona_archetype: Some(String::new()),
        validated: false,
        stakeholder_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = UserProfilePatch {
        name: Some("Patched UserProfile".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        category: Some(UserCategory::Secondary),
        demographic: Some("patched-1".to_string()),
        age_range: Some("patched-1".to_string()),
        abilities: Some(vec!["patched-1".to_string()]),
        disabilities: Some(vec!["patched-1".to_string()]),
        occupation: Some("patched-1".to_string()),
        role_title: Some("patched-1".to_string()),
        department: Some("patched-1".to_string()),
        mobility_profile: Some(vec!["patched-1".to_string()]),
        sensory_profile: Some(vec!["patched-1".to_string()]),
        cognitive_profile: Some(vec!["patched-1".to_string()]),
        behavioral_patterns: Some(vec!["patched-1".to_string()]),
        usage_frequency: Some("patched-1".to_string()),
        usage_duration: Some("patched-1".to_string()),
        peak_usage_times: Some(vec!["patched-1".to_string()]),
        technology_proficiency: Some("patched-1".to_string()),
        preferences: Some(vec!["patched-1".to_string()]),
        pain_points: Some(vec!["patched-1".to_string()]),
        goals: Some(vec!["patched-1".to_string()]),
        activity_ids: Some(vec![EntityId::new_serial("new1", "new1")]),
        research_method: Some("patched-1".to_string()),
        persona_archetype: Some("patched-1".to_string()),
        validated: Some(true),
        stakeholder_ids: Some(vec![EntityId::new_serial("new1", "new1")]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched UserProfile");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn activity_patch_round_trips() {
    let mut item = Activity {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("activity", "Base Activity"), "Base Activity") },
        code: String::new(),
        category: String::new(),
        frequency: Some(String::new()),
        duration: Some(String::new()),
        intensity: Some(String::new()),
        participants: QuantitySpec::default(),
        equipment_ids: Vec::new(),
        space_requirements: Vec::new(),
        environmental_needs: Vec::new(),
        privacy_needs: Vec::new(),
        accessibility_needs: Vec::new(),
        adjacent_activities: Vec::new(),
        sequencing: Vec::new(),
        peak_periods: Vec::new(),
        workflow_steps: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
        user_profile_ids: Vec::new(),
        function_ids: Vec::new(),
        performance_indicators: Vec::new(),
        activity_type: String::new(),
        location_context: Some(String::new()),
        temporal_pattern: Some(String::new()),
        supervision_level: Some(String::new()),
    };
    let original = item.clone();
    let patch = ActivityPatch {
        name: Some("Patched Activity".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-2".to_string()),
        category: Some("patched-2".to_string()),
        frequency: Some("patched-2".to_string()),
        duration: Some("patched-2".to_string()),
        intensity: Some("patched-2".to_string()),
        participants: Some(QuantitySpec::target_unit(42.0, "m2")),
        equipment_ids: Some(vec![EntityId::new_serial("new2", "new2")]),
        space_requirements: Some(vec!["patched-2".to_string()]),
        environmental_needs: Some(vec!["patched-2".to_string()]),
        privacy_needs: Some(vec!["patched-2".to_string()]),
        accessibility_needs: Some(vec!["patched-2".to_string()]),
        adjacent_activities: Some(vec![EntityId::new_serial("new2", "new2")]),
        sequencing: Some(vec!["patched-2".to_string()]),
        peak_periods: Some(vec!["patched-2".to_string()]),
        workflow_steps: Some(vec!["patched-2".to_string()]),
        inputs: Some(vec!["patched-2".to_string()]),
        outputs: Some(vec!["patched-2".to_string()]),
        user_profile_ids: Some(vec![EntityId::new_serial("new2", "new2")]),
        function_ids: Some(vec![EntityId::new_serial("new2", "new2")]),
        performance_indicators: Some(vec!["patched-2".to_string()]),
        activity_type: Some("patched-2".to_string()),
        location_context: Some("patched-2".to_string()),
        temporal_pattern: Some("patched-2".to_string()),
        supervision_level: Some("patched-2".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Activity");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn function_patch_round_trips() {
    let mut item = Function {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("function", "Base Function"), "Base Function") },
        code: String::new(),
        kind: FunctionKind::Primary,
        purpose: TextField::default(),
        criticality: Priority::Mandatory,
        performance_targets: Vec::new(),
        service_level: Some(String::new()),
        operating_hours: Some(String::new()),
        staffing: QuantitySpec::default(),
        equipment_ids: Vec::new(),
        resource_ids: Vec::new(),
        activity_ids: Vec::new(),
        element_ids: Vec::new(),
        dependencies: Vec::new(),
        interfaces: Vec::new(),
        constraints: Vec::new(),
        quality_criteria: Vec::new(),
        regulatory_refs: Vec::new(),
        future_changes: Vec::new(),
        owner_stakeholder_id: Some(EntityId::new_serial("base3", "base3")),
        success_metrics: Vec::new(),
        hierarchy_parent_id: Some(EntityId::new_serial("base3", "base3")),
        conflict_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = FunctionPatch {
        name: Some("Patched Function".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-3".to_string()),
        kind: Some(FunctionKind::Secondary),
        purpose: Some(TextField::plain("patched-3")),
        criticality: Some(Priority::Essential),
        performance_targets: Some(vec!["patched-3".to_string()]),
        service_level: Some("patched-3".to_string()),
        operating_hours: Some("patched-3".to_string()),
        staffing: Some(QuantitySpec::target_unit(42.0, "m2")),
        equipment_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
        resource_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
        activity_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
        element_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
        dependencies: Some(vec![EntityId::new_serial("new3", "new3")]),
        interfaces: Some(vec!["patched-3".to_string()]),
        constraints: Some(vec!["patched-3".to_string()]),
        quality_criteria: Some(vec!["patched-3".to_string()]),
        regulatory_refs: Some(vec!["patched-3".to_string()]),
        future_changes: Some(vec!["patched-3".to_string()]),
        owner_stakeholder_id: Some(EntityId::new_serial("new3", "new3")),
        success_metrics: Some(vec!["patched-3".to_string()]),
        hierarchy_parent_id: Some(EntityId::new_serial("new3", "new3")),
        conflict_ids: Some(vec![EntityId::new_serial("new3", "new3")]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Function");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn program_element_patch_round_trips() {
    let mut item = ProgramElement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("programelement", "Base ProgramElement"), "Base ProgramElement") },
        code: String::new(),
        kind: ProgramElementKind::Building,
        parent_id: Some(EntityId::new_serial("base4", "base4")),
        level: Some(String::new()),
        area: QuantitySpec::default(),
        volume: QuantitySpec::default(),
        height: QuantitySpec::default(),
        occupancy: QuantitySpec::default(),
        function_ids: Vec::new(),
        activity_ids: Vec::new(),
        user_profile_ids: Vec::new(),
        adjacency_ids: Vec::new(),
        quantity_ids: Vec::new(),
        requirement_ids: Vec::new(),
        location_hint: Some(String::new()),
        orientation: Some(String::new()),
        daylight_requirement: Some(String::new()),
        acoustic_class: Some(String::new()),
        security_zone: Some(String::new()),
        flexibility_notes: Vec::new(),
        growth_allocation: Some(String::new()),
        circulation_role: Some(String::new()),
        visibility_level: Some(String::new()),
        adjacency_preferences: Vec::new(),
        environmental_zone: Some(String::new()),
    };
    let original = item.clone();
    let patch = ProgramElementPatch {
        name: Some("Patched ProgramElement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-4".to_string()),
        kind: Some(ProgramElementKind::Campus),
        parent_id: Some(EntityId::new_serial("new4", "new4")),
        level: Some("patched-4".to_string()),
        area: Some(QuantitySpec::target_unit(42.0, "m2")),
        volume: Some(QuantitySpec::target_unit(42.0, "m2")),
        height: Some(QuantitySpec::target_unit(42.0, "m2")),
        occupancy: Some(QuantitySpec::target_unit(42.0, "m2")),
        function_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
        activity_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
        user_profile_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
        adjacency_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
        quantity_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
        requirement_ids: Some(vec![EntityId::new_serial("new4", "new4")]),
        location_hint: Some("patched-4".to_string()),
        orientation: Some("patched-4".to_string()),
        daylight_requirement: Some("patched-4".to_string()),
        acoustic_class: Some("patched-4".to_string()),
        security_zone: Some("patched-4".to_string()),
        flexibility_notes: Some(vec!["patched-4".to_string()]),
        growth_allocation: Some("patched-4".to_string()),
        circulation_role: Some("patched-4".to_string()),
        visibility_level: Some("patched-4".to_string()),
        adjacency_preferences: Some(vec![EntityId::new_serial("new4", "new4")]),
        environmental_zone: Some("patched-4".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ProgramElement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn quantity_requirement_patch_round_trips() {
    let mut item = QuantityRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("quantityrequirement", "Base QuantityRequirement"), "Base QuantityRequirement") },
        target_element_id: EntityId::new_serial("base5", "base5"),
        metric: String::new(),
        quantity: QuantitySpec::default(),
        basis: Some(String::new()),
        calculation_method: Some(String::new()),
        source: Some(String::new()),
        benchmark_ref: Some(EntityId::new_serial("base5", "base5")),
        tolerance_percent: Some(0.0),
        peak_factor: Some(0.0),
        growth_factor: Some(0.0),
        unit_cost: Some(0.0),
        currency: Some(String::new()),
        verification_method: Some(String::new()),
        related_requirement_ids: Vec::new(),
        assumptions: Vec::new(),
        constraints: Vec::new(),
        schedule_phase: Some(String::new()),
        responsible_party: Some(EntityId::new_serial("base5", "base5")),
        last_verified: Some(String::new()),
        variance_notes: Vec::new(),
    };
    let original = item.clone();
    let patch = QuantityRequirementPatch {
        name: Some("Patched QuantityRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        target_element_id: Some(EntityId::new_serial("new5", "new5")),
        metric: Some("patched-5".to_string()),
        quantity: Some(QuantitySpec::target_unit(42.0, "m2")),
        basis: Some("patched-5".to_string()),
        calculation_method: Some("patched-5".to_string()),
        source: Some("patched-5".to_string()),
        benchmark_ref: Some(EntityId::new_serial("new5", "new5")),
        tolerance_percent: Some(42.0),
        peak_factor: Some(42.0),
        growth_factor: Some(42.0),
        unit_cost: Some(42.0),
        currency: Some("patched-5".to_string()),
        verification_method: Some("patched-5".to_string()),
        related_requirement_ids: Some(vec![EntityId::new_serial("new5", "new5")]),
        assumptions: Some(vec!["patched-5".to_string()]),
        constraints: Some(vec!["patched-5".to_string()]),
        schedule_phase: Some("patched-5".to_string()),
        responsible_party: Some(EntityId::new_serial("new5", "new5")),
        last_verified: Some("patched-5".to_string()),
        variance_notes: Some(vec![TaggedNote { tag: "new5".into(), text: "new-note5".into() }]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched QuantityRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn relationship_patch_round_trips() {
    let mut item = Relationship {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("relationship", "Base Relationship"), "Base Relationship") },
        source_id: EntityId::new_serial("base6", "base6"),
        target_id: EntityId::new_serial("base6", "base6"),
        kind: RelationshipKind::Contains,
        strength: Some(0.0),
        directional: false,
        rationale: Some(TextField::default()),
        constraints: Vec::new(),
        conditions: Vec::new(),
        relationship_priority: Priority::Mandatory,
        valid_from: Some(String::new()),
        valid_until: Some(String::new()),
        evidence: Vec::new(),
        conflict_ids: Vec::new(),
        trace_links: Vec::new(),
        bidirectional: false,
        distance_constraint_m: Some(0.0),
        capacity_constraint: Some(String::new()),
        regulatory_basis: Vec::new(),
        review_cycle: Some(String::new()),
        owner_id: Some(EntityId::new_serial("base6", "base6")),
        proximity_requirement: Some(TextField::default()),
        compatibility_requirement: Some(TextField::default()),
        incompatibility_requirement: Some(TextField::default()),
        separation_requirements: Vec::new(),
    };
    let original = item.clone();
    let patch = RelationshipPatch {
        name: Some("Patched Relationship".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        source_id: Some(EntityId::new_serial("new6", "new6")),
        target_id: Some(EntityId::new_serial("new6", "new6")),
        kind: Some(RelationshipKind::Serves),
        strength: Some(42.0),
        directional: Some(true),
        rationale: Some(TextField::plain("patched-6")),
        constraints: Some(vec!["patched-6".to_string()]),
        conditions: Some(vec!["patched-6".to_string()]),
        relationship_priority: Some(Priority::Essential),
        valid_from: Some("patched-6".to_string()),
        valid_until: Some("patched-6".to_string()),
        evidence: Some(vec!["patched-6".to_string()]),
        conflict_ids: Some(vec![EntityId::new_serial("new6", "new6")]),
        trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom6n", "tfrom6n"), EntityId::new_serial("tto6n", "tto6n"), TraceKind::FullAuditTrail)]),
        bidirectional: Some(true),
        distance_constraint_m: Some(42.0),
        capacity_constraint: Some("patched-6".to_string()),
        regulatory_basis: Some(vec!["patched-6".to_string()]),
        review_cycle: Some("patched-6".to_string()),
        owner_id: Some(EntityId::new_serial("new6", "new6")),
        proximity_requirement: Some(TextField::plain("patched-6")),
        compatibility_requirement: Some(TextField::plain("patched-6")),
        incompatibility_requirement: Some(TextField::plain("patched-6")),
        separation_requirements: Some(vec![SeparationKind::Visual]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Relationship");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn adjacency_patch_round_trips() {
    let mut item = Adjacency {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("adjacency", "Base Adjacency"), "Base Adjacency") },
        element_a_id: EntityId::new_serial("base7", "base7"),
        element_b_id: EntityId::new_serial("base7", "base7"),
        kind: AdjacencyKind::Required,
        connection: ConnectionKind::Direct,
        separations: Vec::new(),
        weight: 0.0,
        rationale: Some(TextField::default()),
        distance_max_m: Some(0.0),
        distance_min_m: Some(0.0),
        level_constraint: Some(String::new()),
        access_path: Some(String::new()),
        shared_wall: false,
        shared_entry: false,
        traffic_isolation: false,
        circulation_overlap: false,
        conflict_ids: Vec::new(),
        normalized: false,
        verification_status: ValidationStatus::Pending,
        source_relationship_id: Some(EntityId::new_serial("base7", "base7")),
        internal_external_access: Some(String::new()),
    };
    let original = item.clone();
    let patch = AdjacencyPatch {
        name: Some("Patched Adjacency".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        element_a_id: Some(EntityId::new_serial("new7", "new7")),
        element_b_id: Some(EntityId::new_serial("new7", "new7")),
        kind: Some(AdjacencyKind::Preferred),
        connection: Some(ConnectionKind::Indirect),
        separations: Some(vec![SeparationKind::Visual]),
        weight: Some(42.0),
        rationale: Some(TextField::plain("patched-7")),
        distance_max_m: Some(42.0),
        distance_min_m: Some(42.0),
        level_constraint: Some("patched-7".to_string()),
        access_path: Some("patched-7".to_string()),
        shared_wall: Some(true),
        shared_entry: Some(true),
        traffic_isolation: Some(true),
        circulation_overlap: Some(true),
        conflict_ids: Some(vec![EntityId::new_serial("new7", "new7")]),
        normalized: Some(true),
        verification_status: Some(ValidationStatus::Passed),
        source_relationship_id: Some(EntityId::new_serial("new7", "new7")),
        internal_external_access: Some("patched-7".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Adjacency");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn process_patch_round_trips() {
    let mut item = Process {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("process", "Base Process"), "Base Process") },
        code: String::new(),
        category: String::new(),
        trigger: Some(String::new()),
        inputs: Vec::new(),
        outputs: Vec::new(),
        steps: Vec::new(),
        actors: Vec::new(),
        equipment_ids: Vec::new(),
        element_ids: Vec::new(),
        duration: Some(String::new()),
        frequency: Some(String::new()),
        critical_path: false,
        bottlenecks: Vec::new(),
        dependencies: Vec::new(),
        kpis: Vec::new(),
        automation_level: Some(String::new()),
        failure_modes: Vec::new(),
        improvement_opportunities: Vec::new(),
        regulatory_refs: Vec::new(),
        owner_id: Some(EntityId::new_serial("base8", "base8")),
        workflow_type: Some(String::new()),
        handoff_points: Vec::new(),
        quality_gates: Vec::new(),
    };
    let original = item.clone();
    let patch = ProcessPatch {
        name: Some("Patched Process".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-8".to_string()),
        category: Some("patched-8".to_string()),
        trigger: Some("patched-8".to_string()),
        inputs: Some(vec!["patched-8".to_string()]),
        outputs: Some(vec!["patched-8".to_string()]),
        steps: Some(vec!["patched-8".to_string()]),
        actors: Some(vec![EntityId::new_serial("new8", "new8")]),
        equipment_ids: Some(vec![EntityId::new_serial("new8", "new8")]),
        element_ids: Some(vec![EntityId::new_serial("new8", "new8")]),
        duration: Some("patched-8".to_string()),
        frequency: Some("patched-8".to_string()),
        critical_path: Some(true),
        bottlenecks: Some(vec!["patched-8".to_string()]),
        dependencies: Some(vec![EntityId::new_serial("new8", "new8")]),
        kpis: Some(vec!["patched-8".to_string()]),
        automation_level: Some("patched-8".to_string()),
        failure_modes: Some(vec!["patched-8".to_string()]),
        improvement_opportunities: Some(vec!["patched-8".to_string()]),
        regulatory_refs: Some(vec!["patched-8".to_string()]),
        owner_id: Some(EntityId::new_serial("new8", "new8")),
        workflow_type: Some("patched-8".to_string()),
        handoff_points: Some(vec!["patched-8".to_string()]),
        quality_gates: Some(vec!["patched-8".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Process");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn flow_requirement_patch_round_trips() {
    let mut item = FlowRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("flowrequirement", "Base FlowRequirement"), "Base FlowRequirement") },
        from_element_id: EntityId::new_serial("base9", "base9"),
        to_element_id: EntityId::new_serial("base9", "base9"),
        kind: FlowKind::People,
        flow_type: String::new(),
        direction: FlowDirection::OneWay,
        volume: QuantitySpec::default(),
        peak_rate: Some(0.0),
        clear_width_m: Some(0.0),
        clear_height_m: Some(0.0),
        separation_requirements: Vec::new(),
        access_level: AccessLevel::Public,
        time_windows: Vec::new(),
        equipment_clearance: Some(String::new()),
        signage_required: false,
        escort_required: false,
        emergency_route: false,
        barrier_free: false,
        monitoring_required: false,
        process_id: Some(EntityId::new_serial("base9", "base9")),
        conflict_ids: Vec::new(),
        verification_method: Some(String::new()),
    };
    let original = item.clone();
    let patch = FlowRequirementPatch {
        name: Some("Patched FlowRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        from_element_id: Some(EntityId::new_serial("new9", "new9")),
        to_element_id: Some(EntityId::new_serial("new9", "new9")),
        kind: Some(FlowKind::Material),
        flow_type: Some("patched-9".to_string()),
        direction: Some(FlowDirection::TwoWay),
        volume: Some(QuantitySpec::target_unit(42.0, "m2")),
        peak_rate: Some(42.0),
        clear_width_m: Some(42.0),
        clear_height_m: Some(42.0),
        separation_requirements: Some(vec![SeparationKind::Visual]),
        access_level: Some(AccessLevel::Restricted),
        time_windows: Some(vec!["patched-9".to_string()]),
        equipment_clearance: Some("patched-9".to_string()),
        signage_required: Some(true),
        escort_required: Some(true),
        emergency_route: Some(true),
        barrier_free: Some(true),
        monitoring_required: Some(true),
        process_id: Some(EntityId::new_serial("new9", "new9")),
        conflict_ids: Some(vec![EntityId::new_serial("new9", "new9")]),
        verification_method: Some("patched-9".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched FlowRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn access_rule_patch_round_trips() {
    let mut item = AccessRule {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("accessrule", "Base AccessRule"), "Base AccessRule") },
        subject_ids: Vec::new(),
        resource_ids: Vec::new(),
        access_level: AccessLevel::Public,
        access_mode: AccessMode::Unrestricted,
        authentication: Vec::new(),
        authorization: Vec::new(),
        time_restrictions: Vec::new(),
        escort_policy: Some(String::new()),
        visitor_policy: Some(String::new()),
        emergency_override: false,
        audit_required: false,
        badge_required: false,
        biometric_required: false,
        zone_ids: Vec::new(),
        exceptions: Vec::new(),
        regulatory_basis: Vec::new(),
        enforcement_method: Some(String::new()),
        revocation_policy: Some(String::new()),
        training_required: false,
        owner_id: Some(EntityId::new_serial("base10", "base10")),
    };
    let original = item.clone();
    let patch = AccessRulePatch {
        name: Some("Patched AccessRule".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        subject_ids: Some(vec![EntityId::new_serial("new10", "new10")]),
        resource_ids: Some(vec![EntityId::new_serial("new10", "new10")]),
        access_level: Some(AccessLevel::Restricted),
        access_mode: Some(AccessMode::CardControlled),
        authentication: Some(vec!["patched-10".to_string()]),
        authorization: Some(vec!["patched-10".to_string()]),
        time_restrictions: Some(vec!["patched-10".to_string()]),
        escort_policy: Some("patched-10".to_string()),
        visitor_policy: Some("patched-10".to_string()),
        emergency_override: Some(true),
        audit_required: Some(true),
        badge_required: Some(true),
        biometric_required: Some(true),
        zone_ids: Some(vec![EntityId::new_serial("new10", "new10")]),
        exceptions: Some(vec!["patched-10".to_string()]),
        regulatory_basis: Some(vec!["patched-10".to_string()]),
        enforcement_method: Some("patched-10".to_string()),
        revocation_policy: Some("patched-10".to_string()),
        training_required: Some(true),
        owner_id: Some(EntityId::new_serial("new10", "new10")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched AccessRule");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn operational_requirement_patch_round_trips() {
    let mut item = OperationalRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("operationalrequirement", "Base OperationalRequirement"), "Base OperationalRequirement") },
        operation: String::new(),
        service_level: Some(String::new()),
        operating_hours: Some(String::new()),
        staffing: QuantitySpec::default(),
        maintenance_interval: Some(String::new()),
        cleaning_regime: Some(String::new()),
        turnaround_time: Some(String::new()),
        redundancy: Some(String::new()),
        uptime_target: Some(0.0),
        response_time: Some(String::new()),
        equipment_ids: Vec::new(),
        element_ids: Vec::new(),
        process_ids: Vec::new(),
        utilities: Vec::new(),
        waste_streams: Vec::new(),
        contingency_plan: Vec::new(),
        training_requirements: Vec::new(),
        sop_references: Vec::new(),
        kpi_targets: Vec::new(),
        owner_id: Some(EntityId::new_serial("base11", "base11")),
        service_category: Some(String::new()),
        shift_pattern: Some(String::new()),
        sla_target: Some(String::new()),
        escalation_contact_id: Some(EntityId::new_serial("base11", "base11")),
    };
    let original = item.clone();
    let patch = OperationalRequirementPatch {
        name: Some("Patched OperationalRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        operation: Some("patched-11".to_string()),
        service_level: Some("patched-11".to_string()),
        operating_hours: Some("patched-11".to_string()),
        staffing: Some(QuantitySpec::target_unit(42.0, "m2")),
        maintenance_interval: Some("patched-11".to_string()),
        cleaning_regime: Some("patched-11".to_string()),
        turnaround_time: Some("patched-11".to_string()),
        redundancy: Some("patched-11".to_string()),
        uptime_target: Some(42.0),
        response_time: Some("patched-11".to_string()),
        equipment_ids: Some(vec![EntityId::new_serial("new11", "new11")]),
        element_ids: Some(vec![EntityId::new_serial("new11", "new11")]),
        process_ids: Some(vec![EntityId::new_serial("new11", "new11")]),
        utilities: Some(vec!["patched-11".to_string()]),
        waste_streams: Some(vec!["patched-11".to_string()]),
        contingency_plan: Some(vec!["patched-11".to_string()]),
        training_requirements: Some(vec!["patched-11".to_string()]),
        sop_references: Some(vec!["patched-11".to_string()]),
        kpi_targets: Some(vec!["patched-11".to_string()]),
        owner_id: Some(EntityId::new_serial("new11", "new11")),
        service_category: Some("patched-11".to_string()),
        shift_pattern: Some("patched-11".to_string()),
        sla_target: Some("patched-11".to_string()),
        escalation_contact_id: Some(EntityId::new_serial("new11", "new11")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched OperationalRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn equipment_patch_round_trips() {
    let mut item = Equipment {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("equipment", "Base Equipment"), "Base Equipment") },
        code: String::new(),
        category: String::new(),
        manufacturer: Some(String::new()),
        model: Some(String::new()),
        quantity: QuantitySpec::default(),
        dimensions: Some(String::new()),
        weight_kg: Some(0.0),
        power_kw: Some(0.0),
        utility_connections: Vec::new(),
        ventilation: Some(String::new()),
        noise_level_db: Some(0.0),
        clearance: Some(String::new()),
        mounting: Some(String::new()),
        element_ids: Vec::new(),
        activity_ids: Vec::new(),
        maintenance_access: Vec::new(),
        lifecycle_years: Some(0),
        replacement_cost: Some(0.0),
        standards: Vec::new(),
        supplier: Some(String::new()),
        activity_link_ids: Vec::new(),
        installation_requirements: Vec::new(),
        commissioning_notes: Vec::new(),
        spare_parts: Vec::new(),
    };
    let original = item.clone();
    let patch = EquipmentPatch {
        name: Some("Patched Equipment".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-12".to_string()),
        category: Some("patched-12".to_string()),
        manufacturer: Some("patched-12".to_string()),
        model: Some("patched-12".to_string()),
        quantity: Some(QuantitySpec::target_unit(42.0, "m2")),
        dimensions: Some("patched-12".to_string()),
        weight_kg: Some(42.0),
        power_kw: Some(42.0),
        utility_connections: Some(vec!["patched-12".to_string()]),
        ventilation: Some("patched-12".to_string()),
        noise_level_db: Some(42.0),
        clearance: Some("patched-12".to_string()),
        mounting: Some("patched-12".to_string()),
        element_ids: Some(vec![EntityId::new_serial("new12", "new12")]),
        activity_ids: Some(vec![EntityId::new_serial("new12", "new12")]),
        maintenance_access: Some(vec!["patched-12".to_string()]),
        lifecycle_years: Some(7),
        replacement_cost: Some(42.0),
        standards: Some(vec!["patched-12".to_string()]),
        supplier: Some("patched-12".to_string()),
        activity_link_ids: Some(vec![EntityId::new_serial("new12", "new12")]),
        installation_requirements: Some(vec!["patched-12".to_string()]),
        commissioning_notes: Some(vec!["patched-12".to_string()]),
        spare_parts: Some(vec!["patched-12".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Equipment");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn resource_patch_round_trips() {
    let mut item = Resource {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("resource", "Base Resource"), "Base Resource") },
        code: String::new(),
        category: String::new(),
        resource_type: String::new(),
        quantity: QuantitySpec::default(),
        mobility: Some(String::new()),
        sharing_model: Some(String::new()),
        allocation: Some(String::new()),
        element_ids: Vec::new(),
        activity_ids: Vec::new(),
        user_profile_ids: Vec::new(),
        storage_requirement_id: Some(EntityId::new_serial("base13", "base13")),
        durability: Some(String::new()),
        cleaning_requirements: Vec::new(),
        replacement_cycle: Some(String::new()),
        cost_per_unit: Some(0.0),
        supplier: Some(String::new()),
        standards: Vec::new(),
        ergonomic_notes: Vec::new(),
        customization: Vec::new(),
        disposal_notes: Vec::new(),
        furniture_class: Some(String::new()),
        ergonomics_rating: Some(String::new()),
        sharing_ratio: Some(0.0),
    };
    let original = item.clone();
    let patch = ResourcePatch {
        name: Some("Patched Resource".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-13".to_string()),
        category: Some("patched-13".to_string()),
        resource_type: Some("patched-13".to_string()),
        quantity: Some(QuantitySpec::target_unit(42.0, "m2")),
        mobility: Some("patched-13".to_string()),
        sharing_model: Some("patched-13".to_string()),
        allocation: Some("patched-13".to_string()),
        element_ids: Some(vec![EntityId::new_serial("new13", "new13")]),
        activity_ids: Some(vec![EntityId::new_serial("new13", "new13")]),
        user_profile_ids: Some(vec![EntityId::new_serial("new13", "new13")]),
        storage_requirement_id: Some(EntityId::new_serial("new13", "new13")),
        durability: Some("patched-13".to_string()),
        cleaning_requirements: Some(vec!["patched-13".to_string()]),
        replacement_cycle: Some("patched-13".to_string()),
        cost_per_unit: Some(42.0),
        supplier: Some("patched-13".to_string()),
        standards: Some(vec!["patched-13".to_string()]),
        ergonomic_notes: Some(vec!["patched-13".to_string()]),
        customization: Some(vec!["patched-13".to_string()]),
        disposal_notes: Some(vec!["patched-13".to_string()]),
        furniture_class: Some("patched-13".to_string()),
        ergonomics_rating: Some("patched-13".to_string()),
        sharing_ratio: Some(42.0),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Resource");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn storage_requirement_patch_round_trips() {
    let mut item = StorageRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("storagerequirement", "Base StorageRequirement"), "Base StorageRequirement") },
        stored_item: String::new(),
        storage_class: StorageClass::General,
        quantity: QuantitySpec::default(),
        volume_m3: Some(0.0),
        weight_kg: Some(0.0),
        temperature_range: Some(String::new()),
        humidity_range: Some(String::new()),
        security_level: AccessLevel::Public,
        hazard_class: Some(String::new()),
        retention_period: Some(String::new()),
        access_frequency: Some(String::new()),
        element_ids: Vec::new(),
        equipment_ids: Vec::new(),
        handling_equipment: Vec::new(),
        fire_protection: Vec::new(),
        ventilation: Some(String::new()),
        organization_system: Some(String::new()),
        growth_allowance: Some(0.0),
        regulatory_refs: Vec::new(),
        owner_id: Some(EntityId::new_serial("base14", "base14")),
    };
    let original = item.clone();
    let patch = StorageRequirementPatch {
        name: Some("Patched StorageRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        stored_item: Some("patched-14".to_string()),
        storage_class: Some(StorageClass::Secure),
        quantity: Some(QuantitySpec::target_unit(42.0, "m2")),
        volume_m3: Some(42.0),
        weight_kg: Some(42.0),
        temperature_range: Some("patched-14".to_string()),
        humidity_range: Some("patched-14".to_string()),
        security_level: Some(AccessLevel::Restricted),
        hazard_class: Some("patched-14".to_string()),
        retention_period: Some("patched-14".to_string()),
        access_frequency: Some("patched-14".to_string()),
        element_ids: Some(vec![EntityId::new_serial("new14", "new14")]),
        equipment_ids: Some(vec![EntityId::new_serial("new14", "new14")]),
        handling_equipment: Some(vec!["patched-14".to_string()]),
        fire_protection: Some(vec!["patched-14".to_string()]),
        ventilation: Some("patched-14".to_string()),
        organization_system: Some("patched-14".to_string()),
        growth_allowance: Some(42.0),
        regulatory_refs: Some(vec!["patched-14".to_string()]),
        owner_id: Some(EntityId::new_serial("new14", "new14")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched StorageRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn environmental_requirement_patch_round_trips() {
    let mut item = EnvironmentalRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("environmentalrequirement", "Base EnvironmentalRequirement"), "Base EnvironmentalRequirement") },
        parameter_kind: EnvironmentalParameter::Temperature,
        parameter: String::new(),
        target_value: Some(0.0),
        unit: Some(String::new()),
        min_value: Some(0.0),
        max_value: Some(0.0),
        comfort_band: Some(String::new()),
        measurement_method: Some(String::new()),
        monitoring_frequency: Some(String::new()),
        element_ids: Vec::new(),
        occupancy_basis: Some(String::new()),
        seasonal_variation: Vec::new(),
        energy_implications: Vec::new(),
        standards: Vec::new(),
        certification_targets: Vec::new(),
        outdoor_conditions: Vec::new(),
        ventilation_strategy: Some(String::new()),
        daylight_target: Some(String::new()),
        acoustic_target: Some(String::new()),
        iaq_target: Some(String::new()),
        verification_plan: Some(String::new()),
    };
    let original = item.clone();
    let patch = EnvironmentalRequirementPatch {
        name: Some("Patched EnvironmentalRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        parameter_kind: Some(EnvironmentalParameter::Humidity),
        parameter: Some("patched-15".to_string()),
        target_value: Some(42.0),
        unit: Some("patched-15".to_string()),
        min_value: Some(42.0),
        max_value: Some(42.0),
        comfort_band: Some("patched-15".to_string()),
        measurement_method: Some("patched-15".to_string()),
        monitoring_frequency: Some("patched-15".to_string()),
        element_ids: Some(vec![EntityId::new_serial("new15", "new15")]),
        occupancy_basis: Some("patched-15".to_string()),
        seasonal_variation: Some(vec!["patched-15".to_string()]),
        energy_implications: Some(vec!["patched-15".to_string()]),
        standards: Some(vec!["patched-15".to_string()]),
        certification_targets: Some(vec!["patched-15".to_string()]),
        outdoor_conditions: Some(vec!["patched-15".to_string()]),
        ventilation_strategy: Some("patched-15".to_string()),
        daylight_target: Some("patched-15".to_string()),
        acoustic_target: Some("patched-15".to_string()),
        iaq_target: Some("patched-15".to_string()),
        verification_plan: Some("patched-15".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched EnvironmentalRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn human_factor_requirement_patch_round_trips() {
    let mut item = HumanFactorRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("humanfactorrequirement", "Base HumanFactorRequirement"), "Base HumanFactorRequirement") },
        aspect: HumanFactorAspect::Ergonomics,
        factor: String::new(),
        user_profile_ids: Vec::new(),
        activity_ids: Vec::new(),
        ergonomic_criteria: Vec::new(),
        cognitive_load: Some(String::new()),
        visual_demands: Vec::new(),
        auditory_demands: Vec::new(),
        posture_requirements: Vec::new(),
        reach_envelope: Some(String::new()),
        lighting_for_tasks: Vec::new(),
        thermal_comfort: Vec::new(),
        privacy_needs: Vec::new(),
        social_interaction: Vec::new(),
        stress_factors: Vec::new(),
        mitigation_measures: Vec::new(),
        training_needs: Vec::new(),
        standards: Vec::new(),
        research_basis: Vec::new(),
        element_ids: Vec::new(),
        verification_method: Some(String::new()),
    };
    let original = item.clone();
    let patch = HumanFactorRequirementPatch {
        name: Some("Patched HumanFactorRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        aspect: Some(HumanFactorAspect::Cognition),
        factor: Some("patched-16".to_string()),
        user_profile_ids: Some(vec![EntityId::new_serial("new16", "new16")]),
        activity_ids: Some(vec![EntityId::new_serial("new16", "new16")]),
        ergonomic_criteria: Some(vec!["patched-16".to_string()]),
        cognitive_load: Some("patched-16".to_string()),
        visual_demands: Some(vec!["patched-16".to_string()]),
        auditory_demands: Some(vec!["patched-16".to_string()]),
        posture_requirements: Some(vec!["patched-16".to_string()]),
        reach_envelope: Some("patched-16".to_string()),
        lighting_for_tasks: Some(vec!["patched-16".to_string()]),
        thermal_comfort: Some(vec!["patched-16".to_string()]),
        privacy_needs: Some(vec!["patched-16".to_string()]),
        social_interaction: Some(vec!["patched-16".to_string()]),
        stress_factors: Some(vec!["patched-16".to_string()]),
        mitigation_measures: Some(vec!["patched-16".to_string()]),
        training_needs: Some(vec!["patched-16".to_string()]),
        standards: Some(vec!["patched-16".to_string()]),
        research_basis: Some(vec!["patched-16".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new16", "new16")]),
        verification_method: Some("patched-16".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched HumanFactorRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn accessibility_requirement_patch_round_trips() {
    let mut item = AccessibilityRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("accessibilityrequirement", "Base AccessibilityRequirement"), "Base AccessibilityRequirement") },
        standard: String::new(),
        level: Some(String::new()),
        user_profile_ids: Vec::new(),
        element_ids: Vec::new(),
        route_ids: Vec::new(),
        clear_width_m: Some(0.0),
        clear_height_m: Some(0.0),
        turning_circle_m: Some(0.0),
        ramp_slope: Some(0.0),
        lift_required: false,
        tactile_guidance: false,
        hearing_loop: false,
        visual_contrast: false,
        signage_requirements: Vec::new(),
        controls_height: Some(String::new()),
        emergency_evacuation: Vec::new(),
        service_animal_policy: Some(String::new()),
        companion_seating: false,
        verification_plan: Some(String::new()),
        exceptions: Vec::new(),
        wcag_conformance: Some(String::new()),
        universal_design_principles: Vec::new(),
    };
    let original = item.clone();
    let patch = AccessibilityRequirementPatch {
        name: Some("Patched AccessibilityRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        standard: Some("patched-17".to_string()),
        level: Some("patched-17".to_string()),
        user_profile_ids: Some(vec![EntityId::new_serial("new17", "new17")]),
        element_ids: Some(vec![EntityId::new_serial("new17", "new17")]),
        route_ids: Some(vec![EntityId::new_serial("new17", "new17")]),
        clear_width_m: Some(42.0),
        clear_height_m: Some(42.0),
        turning_circle_m: Some(42.0),
        ramp_slope: Some(42.0),
        lift_required: Some(true),
        tactile_guidance: Some(true),
        hearing_loop: Some(true),
        visual_contrast: Some(true),
        signage_requirements: Some(vec!["patched-17".to_string()]),
        controls_height: Some("patched-17".to_string()),
        emergency_evacuation: Some(vec!["patched-17".to_string()]),
        service_animal_policy: Some("patched-17".to_string()),
        companion_seating: Some(true),
        verification_plan: Some("patched-17".to_string()),
        exceptions: Some(vec!["patched-17".to_string()]),
        wcag_conformance: Some("patched-17".to_string()),
        universal_design_principles: Some(vec!["patched-17".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched AccessibilityRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn privacy_requirement_patch_round_trips() {
    let mut item = PrivacyRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("privacyrequirement", "Base PrivacyRequirement"), "Base PrivacyRequirement") },
        privacy_kind: PrivacyKind::Public,
        privacy_type: String::new(),
        level: Some(String::new()),
        subject_ids: Vec::new(),
        element_ids: Vec::new(),
        visual_privacy: Vec::new(),
        acoustic_privacy: Vec::new(),
        data_privacy: Vec::new(),
        screening_required: false,
        enclosure_required: false,
        access_restrictions: Vec::new(),
        observation_risk: Some(String::new()),
        regulatory_basis: Vec::new(),
        cultural_considerations: Vec::new(),
        technology_controls: Vec::new(),
        signage: Vec::new(),
        monitoring_restrictions: Vec::new(),
        retention_policy: Some(String::new()),
        breach_response: Vec::new(),
        owner_id: Some(EntityId::new_serial("base18", "base18")),
    };
    let original = item.clone();
    let patch = PrivacyRequirementPatch {
        name: Some("Patched PrivacyRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        privacy_kind: Some(PrivacyKind::SemiPublic),
        privacy_type: Some("patched-18".to_string()),
        level: Some("patched-18".to_string()),
        subject_ids: Some(vec![EntityId::new_serial("new18", "new18")]),
        element_ids: Some(vec![EntityId::new_serial("new18", "new18")]),
        visual_privacy: Some(vec!["patched-18".to_string()]),
        acoustic_privacy: Some(vec!["patched-18".to_string()]),
        data_privacy: Some(vec!["patched-18".to_string()]),
        screening_required: Some(true),
        enclosure_required: Some(true),
        access_restrictions: Some(vec!["patched-18".to_string()]),
        observation_risk: Some("patched-18".to_string()),
        regulatory_basis: Some(vec!["patched-18".to_string()]),
        cultural_considerations: Some(vec!["patched-18".to_string()]),
        technology_controls: Some(vec!["patched-18".to_string()]),
        signage: Some(vec!["patched-18".to_string()]),
        monitoring_restrictions: Some(vec!["patched-18".to_string()]),
        retention_policy: Some("patched-18".to_string()),
        breach_response: Some(vec!["patched-18".to_string()]),
        owner_id: Some(EntityId::new_serial("new18", "new18")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched PrivacyRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn safety_requirement_patch_round_trips() {
    let mut item = SafetyRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("safetyrequirement", "Base SafetyRequirement"), "Base SafetyRequirement") },
        safety_domain: SafetyDomain::LifeSafety,
        hazard: String::new(),
        risk_level: RiskLevel::Negligible,
        affected_element_ids: Vec::new(),
        affected_user_ids: Vec::new(),
        mitigation_measures: Vec::new(),
        ppe_requirements: Vec::new(),
        emergency_procedures: Vec::new(),
        evacuation_requirements: Vec::new(),
        fire_protection: Vec::new(),
        structural_safety: Vec::new(),
        slip_trip_fall: Vec::new(),
        chemical_safety: Vec::new(),
        electrical_safety: Vec::new(),
        machinery_safety: Vec::new(),
        standards: Vec::new(),
        inspection_frequency: Some(String::new()),
        training_requirements: Vec::new(),
        incident_reporting: Vec::new(),
        residual_risk: Some(String::new()),
    };
    let original = item.clone();
    let patch = SafetyRequirementPatch {
        name: Some("Patched SafetyRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        safety_domain: Some(SafetyDomain::OccupationalHealth),
        hazard: Some("patched-19".to_string()),
        risk_level: Some(RiskLevel::Low),
        affected_element_ids: Some(vec![EntityId::new_serial("new19", "new19")]),
        affected_user_ids: Some(vec![EntityId::new_serial("new19", "new19")]),
        mitigation_measures: Some(vec!["patched-19".to_string()]),
        ppe_requirements: Some(vec!["patched-19".to_string()]),
        emergency_procedures: Some(vec!["patched-19".to_string()]),
        evacuation_requirements: Some(vec!["patched-19".to_string()]),
        fire_protection: Some(vec!["patched-19".to_string()]),
        structural_safety: Some(vec!["patched-19".to_string()]),
        slip_trip_fall: Some(vec!["patched-19".to_string()]),
        chemical_safety: Some(vec!["patched-19".to_string()]),
        electrical_safety: Some(vec!["patched-19".to_string()]),
        machinery_safety: Some(vec!["patched-19".to_string()]),
        standards: Some(vec!["patched-19".to_string()]),
        inspection_frequency: Some("patched-19".to_string()),
        training_requirements: Some(vec!["patched-19".to_string()]),
        incident_reporting: Some(vec!["patched-19".to_string()]),
        residual_risk: Some("patched-19".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SafetyRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn security_requirement_patch_round_trips() {
    let mut item = SecurityRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("securityrequirement", "Base SecurityRequirement"), "Base SecurityRequirement") },
        control_kind: SecurityControlKind::AccessControl,
        threat: String::new(),
        risk_level: RiskLevel::Negligible,
        asset_ids: Vec::new(),
        zone_ids: Vec::new(),
        access_level: AccessLevel::Public,
        perimeter_controls: Vec::new(),
        surveillance: Vec::new(),
        intrusion_detection: Vec::new(),
        cybersecurity: Vec::new(),
        screening: Vec::new(),
        visitor_management: Vec::new(),
        key_management: Vec::new(),
        standards: Vec::new(),
        response_procedures: Vec::new(),
        drill_frequency: Some(String::new()),
        liaison_contacts: Vec::new(),
        classified_level: Some(String::new()),
        redundancy: Vec::new(),
        audit_requirements: Vec::new(),
    };
    let original = item.clone();
    let patch = SecurityRequirementPatch {
        name: Some("Patched SecurityRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        control_kind: Some(SecurityControlKind::Surveillance),
        threat: Some("patched-20".to_string()),
        risk_level: Some(RiskLevel::Low),
        asset_ids: Some(vec![EntityId::new_serial("new20", "new20")]),
        zone_ids: Some(vec![EntityId::new_serial("new20", "new20")]),
        access_level: Some(AccessLevel::Restricted),
        perimeter_controls: Some(vec!["patched-20".to_string()]),
        surveillance: Some(vec!["patched-20".to_string()]),
        intrusion_detection: Some(vec!["patched-20".to_string()]),
        cybersecurity: Some(vec!["patched-20".to_string()]),
        screening: Some(vec!["patched-20".to_string()]),
        visitor_management: Some(vec!["patched-20".to_string()]),
        key_management: Some(vec!["patched-20".to_string()]),
        standards: Some(vec!["patched-20".to_string()]),
        response_procedures: Some(vec!["patched-20".to_string()]),
        drill_frequency: Some("patched-20".to_string()),
        liaison_contacts: Some(vec!["patched-20".to_string()]),
        classified_level: Some("patched-20".to_string()),
        redundancy: Some(vec!["patched-20".to_string()]),
        audit_requirements: Some(vec!["patched-20".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SecurityRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn regulatory_requirement_patch_round_trips() {
    let mut item = RegulatoryRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("regulatoryrequirement", "Base RegulatoryRequirement"), "Base RegulatoryRequirement") },
        jurisdiction: String::new(),
        code: String::new(),
        clause: Some(String::new()),
        title: String::new(),
        requirement_text: TextField::default(),
        applicability: Vec::new(),
        element_ids: Vec::new(),
        compliance_method: Some(String::new()),
        evidence_required: Vec::new(),
        authority: Some(String::new()),
        effective_date: Some(String::new()),
        expiry_date: Some(String::new()),
        penalties: Vec::new(),
        exemptions: Vec::new(),
        related_requirement_ids: Vec::new(),
        interpretation_notes: Vec::new(),
        verification_status: ValidationStatus::Pending,
        consultant_refs: Vec::new(),
        update_source: Some(String::new()),
    };
    let original = item.clone();
    let patch = RegulatoryRequirementPatch {
        name: Some("Patched RegulatoryRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        jurisdiction: Some("patched-21".to_string()),
        code: Some("patched-21".to_string()),
        clause: Some("patched-21".to_string()),
        title: Some("patched-21".to_string()),
        requirement_text: Some(TextField::plain("patched-21")),
        applicability: Some(vec!["patched-21".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new21", "new21")]),
        compliance_method: Some("patched-21".to_string()),
        evidence_required: Some(vec!["patched-21".to_string()]),
        authority: Some("patched-21".to_string()),
        effective_date: Some("patched-21".to_string()),
        expiry_date: Some("patched-21".to_string()),
        penalties: Some(vec!["patched-21".to_string()]),
        exemptions: Some(vec!["patched-21".to_string()]),
        related_requirement_ids: Some(vec![EntityId::new_serial("new21", "new21")]),
        interpretation_notes: Some(vec![TaggedNote { tag: "new21".into(), text: "new-note21".into() }]),
        verification_status: Some(ValidationStatus::Passed),
        consultant_refs: Some(vec![EntityId::new_serial("new21", "new21")]),
        update_source: Some("patched-21".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched RegulatoryRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn site_context_patch_round_trips() {
    let mut item = SiteContext {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("sitecontext", "Base SiteContext"), "Base SiteContext") },
        site_name: String::new(),
        address: Some(String::new()),
        latitude: Some(0.0),
        longitude: Some(0.0),
        elevation_m: Some(0.0),
        climate_zone: Some(String::new()),
        seismic_zone: Some(String::new()),
        flood_risk: Some(String::new()),
        soil_conditions: Vec::new(),
        utilities_available: Vec::new(),
        access_roads: Vec::new(),
        public_transit: Vec::new(),
        neighbors: Vec::new(),
        views: Vec::new(),
        noise_sources: Vec::new(),
        environmental_constraints: Vec::new(),
        heritage_constraints: Vec::new(),
        zoning: Some(String::new()),
        max_height_m: Some(0.0),
        max_coverage: Some(0.0),
    };
    let original = item.clone();
    let patch = SiteContextPatch {
        name: Some("Patched SiteContext".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        site_name: Some("patched-22".to_string()),
        address: Some("patched-22".to_string()),
        latitude: Some(42.0),
        longitude: Some(42.0),
        elevation_m: Some(42.0),
        climate_zone: Some("patched-22".to_string()),
        seismic_zone: Some("patched-22".to_string()),
        flood_risk: Some("patched-22".to_string()),
        soil_conditions: Some(vec!["patched-22".to_string()]),
        utilities_available: Some(vec!["patched-22".to_string()]),
        access_roads: Some(vec!["patched-22".to_string()]),
        public_transit: Some(vec!["patched-22".to_string()]),
        neighbors: Some(vec!["patched-22".to_string()]),
        views: Some(vec!["patched-22".to_string()]),
        noise_sources: Some(vec!["patched-22".to_string()]),
        environmental_constraints: Some(vec!["patched-22".to_string()]),
        heritage_constraints: Some(vec!["patched-22".to_string()]),
        zoning: Some("patched-22".to_string()),
        max_height_m: Some(42.0),
        max_coverage: Some(42.0),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SiteContext");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn organizational_requirement_patch_round_trips() {
    let mut item = OrganizationalRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("organizationalrequirement", "Base OrganizationalRequirement"), "Base OrganizationalRequirement") },
        department: String::new(),
        reporting_line: Some(String::new()),
        headcount: QuantitySpec::default(),
        growth_plan_id: Some(EntityId::new_serial("base23", "base23")),
        work_patterns: Vec::new(),
        collaboration_model: Some(String::new()),
        hierarchy_levels: Vec::new(),
        decision_making: Vec::new(),
        culture_notes: Vec::new(),
        change_readiness: Some(String::new()),
        union_considerations: Vec::new(),
        training_needs: Vec::new(),
        element_ids: Vec::new(),
        stakeholder_ids: Vec::new(),
        service_requirement_ids: Vec::new(),
        branding_requirements: Vec::new(),
        wellness_plugins: Vec::new(),
        diversity_goals: Vec::new(),
        owner_id: Some(EntityId::new_serial("base23", "base23")),
    };
    let original = item.clone();
    let patch = OrganizationalRequirementPatch {
        name: Some("Patched OrganizationalRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        department: Some("patched-23".to_string()),
        reporting_line: Some("patched-23".to_string()),
        headcount: Some(QuantitySpec::target_unit(42.0, "m2")),
        growth_plan_id: Some(EntityId::new_serial("new23", "new23")),
        work_patterns: Some(vec!["patched-23".to_string()]),
        collaboration_model: Some("patched-23".to_string()),
        hierarchy_levels: Some(vec!["patched-23".to_string()]),
        decision_making: Some(vec!["patched-23".to_string()]),
        culture_notes: Some(vec!["patched-23".to_string()]),
        change_readiness: Some("patched-23".to_string()),
        union_considerations: Some(vec!["patched-23".to_string()]),
        training_needs: Some(vec!["patched-23".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new23", "new23")]),
        stakeholder_ids: Some(vec![EntityId::new_serial("new23", "new23")]),
        service_requirement_ids: Some(vec![EntityId::new_serial("new23", "new23")]),
        branding_requirements: Some(vec!["patched-23".to_string()]),
        wellness_plugins: Some(vec!["patched-23".to_string()]),
        diversity_goals: Some(vec!["patched-23".to_string()]),
        owner_id: Some(EntityId::new_serial("new23", "new23")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched OrganizationalRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn service_requirement_patch_round_trips() {
    let mut item = ServiceRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("servicerequirement", "Base ServiceRequirement"), "Base ServiceRequirement") },
        service_name: String::new(),
        service_type: String::new(),
        provider: Some(String::new()),
        service_level: Some(String::new()),
        operating_hours: Some(String::new()),
        capacity: QuantitySpec::default(),
        response_time: Some(String::new()),
        queue_management: Vec::new(),
        customer_profiles: Vec::new(),
        element_ids: Vec::new(),
        equipment_ids: Vec::new(),
        staffing: QuantitySpec::default(),
        quality_metrics: Vec::new(),
        cost_model: Some(String::new()),
        contract_refs: Vec::new(),
        dependencies: Vec::new(),
        failure_impact: Some(String::new()),
        backup_service: Vec::new(),
        feedback_channels: Vec::new(),
    };
    let original = item.clone();
    let patch = ServiceRequirementPatch {
        name: Some("Patched ServiceRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        service_name: Some("patched-24".to_string()),
        service_type: Some("patched-24".to_string()),
        provider: Some("patched-24".to_string()),
        service_level: Some("patched-24".to_string()),
        operating_hours: Some("patched-24".to_string()),
        capacity: Some(QuantitySpec::target_unit(42.0, "m2")),
        response_time: Some("patched-24".to_string()),
        queue_management: Some(vec!["patched-24".to_string()]),
        customer_profiles: Some(vec![EntityId::new_serial("new24", "new24")]),
        element_ids: Some(vec![EntityId::new_serial("new24", "new24")]),
        equipment_ids: Some(vec![EntityId::new_serial("new24", "new24")]),
        staffing: Some(QuantitySpec::target_unit(42.0, "m2")),
        quality_metrics: Some(vec!["patched-24".to_string()]),
        cost_model: Some("patched-24".to_string()),
        contract_refs: Some(vec!["patched-24".to_string()]),
        dependencies: Some(vec![EntityId::new_serial("new24", "new24")]),
        failure_impact: Some("patched-24".to_string()),
        backup_service: Some(vec!["patched-24".to_string()]),
        feedback_channels: Some(vec!["patched-24".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ServiceRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn infrastructure_requirement_patch_round_trips() {
    let mut item = InfrastructureRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("infrastructurerequirement", "Base InfrastructureRequirement"), "Base InfrastructureRequirement") },
        system: String::new(),
        category: String::new(),
        capacity: QuantitySpec::default(),
        redundancy: Some(String::new()),
        distribution: Vec::new(),
        entry_points: Vec::new(),
        utility_source: Some(String::new()),
        standby_power: false,
        monitoring: Vec::new(),
        maintenance_access: Vec::new(),
        standards: Vec::new(),
        element_ids: Vec::new(),
        peak_demand: Some(0.0),
        diversity_factor: Some(0.0),
        future_expansion: Vec::new(),
        interface_requirements: Vec::new(),
        commissioning: Vec::new(),
        lifecycle_cost: Some(0.0),
        owner_id: Some(EntityId::new_serial("base25", "base25")),
    };
    let original = item.clone();
    let patch = InfrastructureRequirementPatch {
        name: Some("Patched InfrastructureRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        system: Some("patched-25".to_string()),
        category: Some("patched-25".to_string()),
        capacity: Some(QuantitySpec::target_unit(42.0, "m2")),
        redundancy: Some("patched-25".to_string()),
        distribution: Some(vec!["patched-25".to_string()]),
        entry_points: Some(vec!["patched-25".to_string()]),
        utility_source: Some("patched-25".to_string()),
        standby_power: Some(true),
        monitoring: Some(vec!["patched-25".to_string()]),
        maintenance_access: Some(vec!["patched-25".to_string()]),
        standards: Some(vec!["patched-25".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new25", "new25")]),
        peak_demand: Some(42.0),
        diversity_factor: Some(42.0),
        future_expansion: Some(vec!["patched-25".to_string()]),
        interface_requirements: Some(vec!["patched-25".to_string()]),
        commissioning: Some(vec!["patched-25".to_string()]),
        lifecycle_cost: Some(42.0),
        owner_id: Some(EntityId::new_serial("new25", "new25")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched InfrastructureRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn information_requirement_patch_round_trips() {
    let mut item = InformationRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("informationrequirement", "Base InformationRequirement"), "Base InformationRequirement") },
        information_type: String::new(),
        format: Some(String::new()),
        source_system: Some(String::new()),
        destination_systems: Vec::new(),
        update_frequency: Some(String::new()),
        retention_period: Some(String::new()),
        access_controls: Vec::new(),
        classification: Some(String::new()),
        quality_criteria: Vec::new(),
        metadata_requirements: Vec::new(),
        integration_points: Vec::new(),
        backup_requirements: Vec::new(),
        disaster_recovery: Vec::new(),
        privacy_controls: Vec::new(),
        audit_trail: false,
        element_ids: Vec::new(),
        stakeholder_ids: Vec::new(),
        standards: Vec::new(),
        owner_id: Some(EntityId::new_serial("base26", "base26")),
    };
    let original = item.clone();
    let patch = InformationRequirementPatch {
        name: Some("Patched InformationRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        information_type: Some("patched-26".to_string()),
        format: Some("patched-26".to_string()),
        source_system: Some("patched-26".to_string()),
        destination_systems: Some(vec!["patched-26".to_string()]),
        update_frequency: Some("patched-26".to_string()),
        retention_period: Some("patched-26".to_string()),
        access_controls: Some(vec!["patched-26".to_string()]),
        classification: Some("patched-26".to_string()),
        quality_criteria: Some(vec!["patched-26".to_string()]),
        metadata_requirements: Some(vec!["patched-26".to_string()]),
        integration_points: Some(vec!["patched-26".to_string()]),
        backup_requirements: Some(vec!["patched-26".to_string()]),
        disaster_recovery: Some(vec!["patched-26".to_string()]),
        privacy_controls: Some(vec!["patched-26".to_string()]),
        audit_trail: Some(true),
        element_ids: Some(vec![EntityId::new_serial("new26", "new26")]),
        stakeholder_ids: Some(vec![EntityId::new_serial("new26", "new26")]),
        standards: Some(vec!["patched-26".to_string()]),
        owner_id: Some(EntityId::new_serial("new26", "new26")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched InformationRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn communication_requirement_patch_round_trips() {
    let mut item = CommunicationRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("communicationrequirement", "Base CommunicationRequirement"), "Base CommunicationRequirement") },
        channel: String::new(),
        audience_ids: Vec::new(),
        message_types: Vec::new(),
        frequency: Some(String::new()),
        medium: Vec::new(),
        language: Vec::new(),
        accessibility: Vec::new(),
        emergency_use: false,
        two_way: false,
        recording_policy: Some(String::new()),
        signage_locations: Vec::new(),
        technology: Vec::new(),
        escalation_path: Vec::new(),
        feedback_loop: false,
        privacy_controls: Vec::new(),
        element_ids: Vec::new(),
        standards: Vec::new(),
        owner_id: Some(EntityId::new_serial("base27", "base27")),
        templates: Vec::new(),
    };
    let original = item.clone();
    let patch = CommunicationRequirementPatch {
        name: Some("Patched CommunicationRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        channel: Some("patched-27".to_string()),
        audience_ids: Some(vec![EntityId::new_serial("new27", "new27")]),
        message_types: Some(vec!["patched-27".to_string()]),
        frequency: Some("patched-27".to_string()),
        medium: Some(vec!["patched-27".to_string()]),
        language: Some(vec!["patched-27".to_string()]),
        accessibility: Some(vec!["patched-27".to_string()]),
        emergency_use: Some(true),
        two_way: Some(true),
        recording_policy: Some("patched-27".to_string()),
        signage_locations: Some(vec!["patched-27".to_string()]),
        technology: Some(vec!["patched-27".to_string()]),
        escalation_path: Some(vec!["patched-27".to_string()]),
        feedback_loop: Some(true),
        privacy_controls: Some(vec!["patched-27".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new27", "new27")]),
        standards: Some(vec!["patched-27".to_string()]),
        owner_id: Some(EntityId::new_serial("new27", "new27")),
        templates: Some(vec![EntityId::new_serial("new27", "new27")]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched CommunicationRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn wayfinding_requirement_patch_round_trips() {
    let mut item = WayfindingRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("wayfindingrequirement", "Base WayfindingRequirement"), "Base WayfindingRequirement") },
        user_profile_ids: Vec::new(),
        element_ids: Vec::new(),
        destination_types: Vec::new(),
        signage_types: Vec::new(),
        languages: Vec::new(),
        tactile_required: false,
        audio_required: false,
        digital_wayfinding: false,
        landmark_strategy: Vec::new(),
        color_coding: Vec::new(),
        symbol_standards: Vec::new(),
        decision_points: Vec::new(),
        maximum_signage_distance_m: Some(0.0),
        lighting_requirements: Vec::new(),
        maintenance_plan: Some(String::new()),
        emergency_egress: Vec::new(),
        visitor_journey: Vec::new(),
        staff_journey: Vec::new(),
        brand_integration: Vec::new(),
    };
    let original = item.clone();
    let patch = WayfindingRequirementPatch {
        name: Some("Patched WayfindingRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        user_profile_ids: Some(vec![EntityId::new_serial("new28", "new28")]),
        element_ids: Some(vec![EntityId::new_serial("new28", "new28")]),
        destination_types: Some(vec!["patched-28".to_string()]),
        signage_types: Some(vec!["patched-28".to_string()]),
        languages: Some(vec!["patched-28".to_string()]),
        tactile_required: Some(true),
        audio_required: Some(true),
        digital_wayfinding: Some(true),
        landmark_strategy: Some(vec!["patched-28".to_string()]),
        color_coding: Some(vec!["patched-28".to_string()]),
        symbol_standards: Some(vec!["patched-28".to_string()]),
        decision_points: Some(vec!["patched-28".to_string()]),
        maximum_signage_distance_m: Some(42.0),
        lighting_requirements: Some(vec!["patched-28".to_string()]),
        maintenance_plan: Some("patched-28".to_string()),
        emergency_egress: Some(vec!["patched-28".to_string()]),
        visitor_journey: Some(vec!["patched-28".to_string()]),
        staff_journey: Some(vec!["patched-28".to_string()]),
        brand_integration: Some(vec!["patched-28".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched WayfindingRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn schedule_requirement_patch_round_trips() {
    let mut item = ScheduleRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("schedulerequirement", "Base ScheduleRequirement"), "Base ScheduleRequirement") },
        milestone: String::new(),
        phase: DeliveryPhase::Concept,
        start_date: Some(String::new()),
        end_date: Some(String::new()),
        duration: Some(String::new()),
        dependencies: Vec::new(),
        predecessors: Vec::new(),
        successors: Vec::new(),
        critical: false,
        float_days: Some(0),
        resource_requirements: Vec::new(),
        occupancy_impact: Vec::new(),
        phasing_strategy: Some(String::new()),
        decant_requirements: Vec::new(),
        commissioning_window: Some(String::new()),
        stakeholder_ids: Vec::new(),
        risk_ids: Vec::new(),
        contingency_days: Some(0),
        reporting_cadence: Some(String::new()),
        owner_id: Some(EntityId::new_serial("base29", "base29")),
    };
    let original = item.clone();
    let patch = ScheduleRequirementPatch {
        name: Some("Patched ScheduleRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        milestone: Some("patched-29".to_string()),
        phase: Some(DeliveryPhase::Schematic),
        start_date: Some("patched-29".to_string()),
        end_date: Some("patched-29".to_string()),
        duration: Some("patched-29".to_string()),
        dependencies: Some(vec![EntityId::new_serial("new29", "new29")]),
        predecessors: Some(vec![EntityId::new_serial("new29", "new29")]),
        successors: Some(vec![EntityId::new_serial("new29", "new29")]),
        critical: Some(true),
        float_days: Some(7),
        resource_requirements: Some(vec!["patched-29".to_string()]),
        occupancy_impact: Some(vec!["patched-29".to_string()]),
        phasing_strategy: Some("patched-29".to_string()),
        decant_requirements: Some(vec!["patched-29".to_string()]),
        commissioning_window: Some("patched-29".to_string()),
        stakeholder_ids: Some(vec![EntityId::new_serial("new29", "new29")]),
        risk_ids: Some(vec![EntityId::new_serial("new29", "new29")]),
        contingency_days: Some(7),
        reporting_cadence: Some("patched-29".to_string()),
        owner_id: Some(EntityId::new_serial("new29", "new29")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ScheduleRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn flexibility_requirement_patch_round_trips() {
    let mut item = FlexibilityRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("flexibilityrequirement", "Base FlexibilityRequirement"), "Base FlexibilityRequirement") },
        flexibility_type: String::new(),
        element_ids: Vec::new(),
        adaptation_scenarios: Vec::new(),
        modularity_level: Some(String::new()),
        reconfiguration_time: Some(String::new()),
        cost_of_change: Some(0.0),
        technology_readiness: Some(String::new()),
        future_function_ids: Vec::new(),
        demountable_partitions: false,
        raised_floor: false,
        overhead_services: false,
        expansion_direction: Vec::new(),
        contraction_scenario: Vec::new(),
        multi_use_potential: Vec::new(),
        furniture_strategy: Vec::new(),
        infrastructure_spare_capacity: Vec::new(),
        lease_implications: Vec::new(),
        owner_id: Some(EntityId::new_serial("base30", "base30")),
    };
    let original = item.clone();
    let patch = FlexibilityRequirementPatch {
        name: Some("Patched FlexibilityRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        flexibility_type: Some("patched-30".to_string()),
        element_ids: Some(vec![EntityId::new_serial("new30", "new30")]),
        adaptation_scenarios: Some(vec!["patched-30".to_string()]),
        modularity_level: Some("patched-30".to_string()),
        reconfiguration_time: Some("patched-30".to_string()),
        cost_of_change: Some(42.0),
        technology_readiness: Some("patched-30".to_string()),
        future_function_ids: Some(vec![EntityId::new_serial("new30", "new30")]),
        demountable_partitions: Some(true),
        raised_floor: Some(true),
        overhead_services: Some(true),
        expansion_direction: Some(vec!["patched-30".to_string()]),
        contraction_scenario: Some(vec!["patched-30".to_string()]),
        multi_use_potential: Some(vec!["patched-30".to_string()]),
        furniture_strategy: Some(vec!["patched-30".to_string()]),
        infrastructure_spare_capacity: Some(vec!["patched-30".to_string()]),
        lease_implications: Some(vec!["patched-30".to_string()]),
        owner_id: Some(EntityId::new_serial("new30", "new30")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched FlexibilityRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn growth_plan_patch_round_trips() {
    let mut item = GrowthPlan {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("growthplan", "Base GrowthPlan"), "Base GrowthPlan") },
        horizon_years: 0,
        growth_rate: Some(0.0),
        headcount_growth: QuantitySpec::default(),
        area_growth: QuantitySpec::default(),
        phases: Vec::new(),
        trigger_events: Vec::new(),
        expansion_element_ids: Vec::new(),
        reserve_areas: Vec::new(),
        infrastructure_headroom: Vec::new(),
        budget_envelope: Some(0.0),
        funding_sources: Vec::new(),
        risk_factors: Vec::new(),
        decision_points: Vec::new(),
        scenario_ids: Vec::new(),
        decommission_plan: Vec::new(),
        relocation_strategy: Vec::new(),
        stakeholder_impact: Vec::new(),
        regulatory_considerations: Vec::new(),
        owner_id: Some(EntityId::new_serial("base31", "base31")),
    };
    let original = item.clone();
    let patch = GrowthPlanPatch {
        name: Some("Patched GrowthPlan".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        horizon_years: Some(7),
        growth_rate: Some(42.0),
        headcount_growth: Some(QuantitySpec::target_unit(42.0, "m2")),
        area_growth: Some(QuantitySpec::target_unit(42.0, "m2")),
        phases: Some(vec!["patched-31".to_string()]),
        trigger_events: Some(vec!["patched-31".to_string()]),
        expansion_element_ids: Some(vec![EntityId::new_serial("new31", "new31")]),
        reserve_areas: Some(vec!["patched-31".to_string()]),
        infrastructure_headroom: Some(vec!["patched-31".to_string()]),
        budget_envelope: Some(42.0),
        funding_sources: Some(vec!["patched-31".to_string()]),
        risk_factors: Some(vec![EntityId::new_serial("new31", "new31")]),
        decision_points: Some(vec![EntityId::new_serial("new31", "new31")]),
        scenario_ids: Some(vec![EntityId::new_serial("new31", "new31")]),
        decommission_plan: Some(vec!["patched-31".to_string()]),
        relocation_strategy: Some(vec!["patched-31".to_string()]),
        stakeholder_impact: Some(vec!["patched-31".to_string()]),
        regulatory_considerations: Some(vec!["patched-31".to_string()]),
        owner_id: Some(EntityId::new_serial("new31", "new31")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched GrowthPlan");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn sustainability_requirement_patch_round_trips() {
    let mut item = SustainabilityRequirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("sustainabilityrequirement", "Base SustainabilityRequirement"), "Base SustainabilityRequirement") },
        topic: String::new(),
        target: Some(String::new()),
        metric: Some(String::new()),
        baseline: Some(0.0),
        target_value: Some(0.0),
        unit: Some(String::new()),
        certification: Vec::new(),
        standards: Vec::new(),
        element_ids: Vec::new(),
        strategies: Vec::new(),
        materials_preferences: Vec::new(),
        energy_strategy: Vec::new(),
        water_strategy: Vec::new(),
        waste_strategy: Vec::new(),
        biodiversity: Vec::new(),
        embodied_carbon: Some(0.0),
        operational_carbon: Some(0.0),
        reporting_requirements: Vec::new(),
        verification_plan: Some(String::new()),
        owner_id: Some(EntityId::new_serial("base32", "base32")),
    };
    let original = item.clone();
    let patch = SustainabilityRequirementPatch {
        name: Some("Patched SustainabilityRequirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        topic: Some("patched-32".to_string()),
        target: Some("patched-32".to_string()),
        metric: Some("patched-32".to_string()),
        baseline: Some(42.0),
        target_value: Some(42.0),
        unit: Some("patched-32".to_string()),
        certification: Some(vec!["patched-32".to_string()]),
        standards: Some(vec!["patched-32".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new32", "new32")]),
        strategies: Some(vec!["patched-32".to_string()]),
        materials_preferences: Some(vec!["patched-32".to_string()]),
        energy_strategy: Some(vec!["patched-32".to_string()]),
        water_strategy: Some(vec!["patched-32".to_string()]),
        waste_strategy: Some(vec!["patched-32".to_string()]),
        biodiversity: Some(vec!["patched-32".to_string()]),
        embodied_carbon: Some(42.0),
        operational_carbon: Some(42.0),
        reporting_requirements: Some(vec!["patched-32".to_string()]),
        verification_plan: Some("patched-32".to_string()),
        owner_id: Some(EntityId::new_serial("new32", "new32")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SustainabilityRequirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

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
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ResilienceRequirement");
    item.apply_patch(&inverse);
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
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched CostRequirement");
    item.apply_patch(&inverse);
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
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched DeliveryConstraint");
    item.apply_patch(&inverse);
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
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Risk");
    item.apply_patch(&inverse);
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
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Conflict");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn requirement_patch_round_trips() {
    let mut item = Requirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("requirement", "Base Requirement"), "Base Requirement") },
        code: String::new(),
        kind: RequirementKind::Functional,
        statement: TextField::default(),
        rationale: Some(TextField::default()),
        source: Some(String::new()),
        stakeholder_ids: Vec::new(),
        element_ids: Vec::new(),
        function_ids: Vec::new(),
        parent_requirement_id: Some(EntityId::new_serial("base38", "base38")),
        child_requirement_ids: Vec::new(),
        acceptance_criteria: Vec::new(),
        verification_method: Some(String::new()),
        validation_status: ValidationStatus::Pending,
        conflict_ids: Vec::new(),
        risk_ids: Vec::new(),
        cost_estimate: Some(0.0),
        schedule_constraint: Some(String::new()),
        regulatory_refs: Vec::new(),
        trace_links: Vec::new(),
        superseded_by: Some(EntityId::new_serial("base38", "base38")),
    };
    let original = item.clone();
    let patch = RequirementPatch {
        name: Some("Patched Requirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-38".to_string()),
        kind: Some(RequirementKind::Spatial),
        statement: Some(TextField::plain("patched-38")),
        rationale: Some(TextField::plain("patched-38")),
        source: Some("patched-38".to_string()),
        stakeholder_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        element_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        function_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        parent_requirement_id: Some(EntityId::new_serial("new38", "new38")),
        child_requirement_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        acceptance_criteria: Some(vec!["patched-38".to_string()]),
        verification_method: Some("patched-38".to_string()),
        validation_status: Some(ValidationStatus::Passed),
        conflict_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        risk_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        cost_estimate: Some(42.0),
        schedule_constraint: Some("patched-38".to_string()),
        regulatory_refs: Some(vec!["patched-38".to_string()]),
        trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom38n", "tfrom38n"), EntityId::new_serial("tto38n", "tto38n"), TraceKind::FullAuditTrail)]),
        superseded_by: Some(EntityId::new_serial("new38", "new38")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Requirement");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn priority_record_patch_round_trips() {
    let mut item = PriorityRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("priorityrecord", "Base PriorityRecord"), "Base PriorityRecord") },
        subject_id: EntityId::new_serial("base39", "base39"),
        subject_kind: String::new(),
        ranked_priority: Priority::Mandatory,
        rank: Some(0),
        weight: Some(0.0),
        rationale: Some(TextField::default()),
        decision_id: Some(EntityId::new_serial("base39", "base39")),
        stakeholder_ids: Vec::new(),
        effective_from: Some(String::new()),
        effective_until: Some(String::new()),
        review_cycle: Some(String::new()),
        dependencies: Vec::new(),
        conflicts: Vec::new(),
        scoring_method: Some(String::new()),
        score: Some(0.0),
        criteria: Vec::new(),
        approved_by: Some(EntityId::new_serial("base39", "base39")),
        approval_date: Some(String::new()),
        ranking_notes: Vec::new(),
    };
    let original = item.clone();
    let patch = PriorityRecordPatch {
        name: Some("Patched PriorityRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        subject_id: Some(EntityId::new_serial("new39", "new39")),
        subject_kind: Some("patched-39".to_string()),
        ranked_priority: Some(Priority::Essential),
        rank: Some(7),
        weight: Some(42.0),
        rationale: Some(TextField::plain("patched-39")),
        decision_id: Some(EntityId::new_serial("new39", "new39")),
        stakeholder_ids: Some(vec![EntityId::new_serial("new39", "new39")]),
        effective_from: Some("patched-39".to_string()),
        effective_until: Some("patched-39".to_string()),
        review_cycle: Some("patched-39".to_string()),
        dependencies: Some(vec![EntityId::new_serial("new39", "new39")]),
        conflicts: Some(vec![EntityId::new_serial("new39", "new39")]),
        scoring_method: Some("patched-39".to_string()),
        score: Some(42.0),
        criteria: Some(vec!["patched-39".to_string()]),
        approved_by: Some(EntityId::new_serial("new39", "new39")),
        approval_date: Some("patched-39".to_string()),
        ranking_notes: Some(vec![TaggedNote { tag: "new39".into(), text: "new-note39".into() }]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched PriorityRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn scenario_patch_round_trips() {
    let mut item = Scenario {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("scenario", "Base Scenario"), "Base Scenario") },
        code: String::new(),
        hypothesis: TextField::default(),
        assumptions: Vec::new(),
        variables: Vec::new(),
        element_ids: Vec::new(),
        requirement_ids: Vec::new(),
        growth_plan_id: Some(EntityId::new_serial("base40", "base40")),
        probability: Some(0.0),
        impact_summary: Some(TextField::default()),
        cost_delta: Some(0.0),
        area_delta: Some(0.0),
        headcount_delta: Some(0.0),
        schedule_delta: Some(String::new()),
        risk_ids: Vec::new(),
        option_ids: Vec::new(),
        baseline: false,
        preferred: false,
        analysis_ids: Vec::new(),
        owner_id: Some(EntityId::new_serial("base40", "base40")),
    };
    let original = item.clone();
    let patch = ScenarioPatch {
        name: Some("Patched Scenario".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-40".to_string()),
        hypothesis: Some(TextField::plain("patched-40")),
        assumptions: Some(vec!["patched-40".to_string()]),
        variables: Some(vec!["patched-40".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        requirement_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        growth_plan_id: Some(EntityId::new_serial("new40", "new40")),
        probability: Some(42.0),
        impact_summary: Some(TextField::plain("patched-40")),
        cost_delta: Some(42.0),
        area_delta: Some(42.0),
        headcount_delta: Some(42.0),
        schedule_delta: Some("patched-40".to_string()),
        risk_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        option_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        baseline: Some(true),
        preferred: Some(true),
        analysis_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        owner_id: Some(EntityId::new_serial("new40", "new40")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Scenario");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn option_evaluation_patch_round_trips() {
    let mut item = OptionEvaluation {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("optionevaluation", "Base OptionEvaluation"), "Base OptionEvaluation") },
        option_name: String::new(),
        option_description: TextField::default(),
        scenario_id: Some(EntityId::new_serial("base41", "base41")),
        criteria_ids: Vec::new(),
        scores: Vec::new(),
        weighted_score: Some(0.0),
        cost_estimate: Some(0.0),
        schedule_estimate: Some(String::new()),
        risk_summary: Vec::new(),
        benefits: Vec::new(),
        drawbacks: Vec::new(),
        assumptions: Vec::new(),
        dependencies: Vec::new(),
        stakeholder_feedback: Vec::new(),
        recommendation: Some(String::new()),
        decision_id: Some(EntityId::new_serial("base41", "base41")),
        evaluation_status: ValidationStatus::Pending,
        evaluator_ids: Vec::new(),
        evaluation_date: Some(String::new()),
    };
    let original = item.clone();
    let patch = OptionEvaluationPatch {
        name: Some("Patched OptionEvaluation".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        option_name: Some("patched-41".to_string()),
        option_description: Some(TextField::plain("patched-41")),
        scenario_id: Some(EntityId::new_serial("new41", "new41")),
        criteria_ids: Some(vec![EntityId::new_serial("new41", "new41")]),
        scores: Some(vec![42.0]),
        weighted_score: Some(42.0),
        cost_estimate: Some(42.0),
        schedule_estimate: Some("patched-41".to_string()),
        risk_summary: Some(vec!["patched-41".to_string()]),
        benefits: Some(vec!["patched-41".to_string()]),
        drawbacks: Some(vec!["patched-41".to_string()]),
        assumptions: Some(vec!["patched-41".to_string()]),
        dependencies: Some(vec![EntityId::new_serial("new41", "new41")]),
        stakeholder_feedback: Some(vec![TaggedNote { tag: "new41".into(), text: "new-note41".into() }]),
        recommendation: Some("patched-41".to_string()),
        decision_id: Some(EntityId::new_serial("new41", "new41")),
        evaluation_status: Some(ValidationStatus::Passed),
        evaluator_ids: Some(vec![EntityId::new_serial("new41", "new41")]),
        evaluation_date: Some("patched-41".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched OptionEvaluation");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn decision_patch_round_trips() {
    let mut item = Decision {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("decision", "Base Decision"), "Base Decision") },
        decision_statement: TextField::default(),
        context: TextField::default(),
        options_considered: Vec::new(),
        selected_option_id: Some(EntityId::new_serial("base42", "base42")),
        rationale: TextField::default(),
        decision_maker_ids: Vec::new(),
        consulted_ids: Vec::new(),
        informed_ids: Vec::new(),
        decision_date: Some(String::new()),
        effective_date: Some(String::new()),
        reversal_conditions: Vec::new(),
        impacted_requirement_ids: Vec::new(),
        impacted_element_ids: Vec::new(),
        cost_impact: Some(0.0),
        schedule_impact: Some(String::new()),
        risk_impact: Vec::new(),
        approval_status: ValidationStatus::Pending,
        meeting_ref: Some(EntityId::new_serial("base42", "base42")),
        artifact_refs: Vec::new(),
    };
    let original = item.clone();
    let patch = DecisionPatch {
        name: Some("Patched Decision".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        decision_statement: Some(TextField::plain("patched-42")),
        context: Some(TextField::plain("patched-42")),
        options_considered: Some(vec![EntityId::new_serial("new42", "new42")]),
        selected_option_id: Some(EntityId::new_serial("new42", "new42")),
        rationale: Some(TextField::plain("patched-42")),
        decision_maker_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        consulted_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        informed_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        decision_date: Some("patched-42".to_string()),
        effective_date: Some("patched-42".to_string()),
        reversal_conditions: Some(vec!["patched-42".to_string()]),
        impacted_requirement_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        impacted_element_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        cost_impact: Some(42.0),
        schedule_impact: Some("patched-42".to_string()),
        risk_impact: Some(vec!["patched-42".to_string()]),
        approval_status: Some(ValidationStatus::Passed),
        meeting_ref: Some(EntityId::new_serial("new42", "new42")),
        artifact_refs: Some(vec![EntityId::new_serial("new42", "new42")]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Decision");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn validation_record_patch_round_trips() {
    let mut item = ValidationRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("validationrecord", "Base ValidationRecord"), "Base ValidationRecord") },
        subject_id: EntityId::new_serial("base43", "base43"),
        subject_kind: String::new(),
        validation_type: String::new(),
        method: Some(String::new()),
        criteria: Vec::new(),
        result: ValidationStatus::Pending,
        evidence: Vec::new(),
        validator_ids: Vec::new(),
        validation_date: Some(String::new()),
        next_review_date: Some(String::new()),
        findings: Vec::new(),
        non_conformities: Vec::new(),
        corrective_actions: Vec::new(),
        waivers: Vec::new(),
        standards: Vec::new(),
        trace_links: Vec::new(),
        report_id: Some(EntityId::new_serial("base43", "base43")),
        confidence_level: Some(String::new()),
        validation_notes: Vec::new(),
    };
    let original = item.clone();
    let patch = ValidationRecordPatch {
        name: Some("Patched ValidationRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        subject_id: Some(EntityId::new_serial("new43", "new43")),
        subject_kind: Some("patched-43".to_string()),
        validation_type: Some("patched-43".to_string()),
        method: Some("patched-43".to_string()),
        criteria: Some(vec!["patched-43".to_string()]),
        result: Some(ValidationStatus::Passed),
        evidence: Some(vec!["patched-43".to_string()]),
        validator_ids: Some(vec![EntityId::new_serial("new43", "new43")]),
        validation_date: Some("patched-43".to_string()),
        next_review_date: Some("patched-43".to_string()),
        findings: Some(vec!["patched-43".to_string()]),
        non_conformities: Some(vec!["patched-43".to_string()]),
        corrective_actions: Some(vec!["patched-43".to_string()]),
        waivers: Some(vec!["patched-43".to_string()]),
        standards: Some(vec!["patched-43".to_string()]),
        trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom43n", "tfrom43n"), EntityId::new_serial("tto43n", "tto43n"), TraceKind::FullAuditTrail)]),
        report_id: Some(EntityId::new_serial("new43", "new43")),
        confidence_level: Some("patched-43".to_string()),
        validation_notes: Some(vec![TaggedNote { tag: "new43".into(), text: "new-note43".into() }]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ValidationRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn performance_criterion_patch_round_trips() {
    let mut item = PerformanceCriterion {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("performancecriterion", "Base PerformanceCriterion"), "Base PerformanceCriterion") },
        criterion: String::new(),
        metric: String::new(),
        target: Some(0.0),
        unit: Some(String::new()),
        minimum: Some(0.0),
        maximum: Some(0.0),
        measurement_method: Some(String::new()),
        frequency: Some(String::new()),
        requirement_ids: Vec::new(),
        element_ids: Vec::new(),
        baseline: Some(0.0),
        benchmark_ref: Some(EntityId::new_serial("base44", "base44")),
        weight: Some(0.0),
        data_source: Some(String::new()),
        reporting_cadence: Some(String::new()),
        owner_id: Some(EntityId::new_serial("base44", "base44")),
        verification_plan: Some(String::new()),
        penalty_threshold: Some(0.0),
        incentive_threshold: Some(0.0),
    };
    let original = item.clone();
    let patch = PerformanceCriterionPatch {
        name: Some("Patched PerformanceCriterion".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        criterion: Some("patched-44".to_string()),
        metric: Some("patched-44".to_string()),
        target: Some(42.0),
        unit: Some("patched-44".to_string()),
        minimum: Some(42.0),
        maximum: Some(42.0),
        measurement_method: Some("patched-44".to_string()),
        frequency: Some("patched-44".to_string()),
        requirement_ids: Some(vec![EntityId::new_serial("new44", "new44")]),
        element_ids: Some(vec![EntityId::new_serial("new44", "new44")]),
        baseline: Some(42.0),
        benchmark_ref: Some(EntityId::new_serial("new44", "new44")),
        weight: Some(42.0),
        data_source: Some("patched-44".to_string()),
        reporting_cadence: Some("patched-44".to_string()),
        owner_id: Some(EntityId::new_serial("new44", "new44")),
        verification_plan: Some("patched-44".to_string()),
        penalty_threshold: Some(42.0),
        incentive_threshold: Some(42.0),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched PerformanceCriterion");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn quality_record_patch_round_trips() {
    let mut item = QualityRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("qualityrecord", "Base QualityRecord"), "Base QualityRecord") },
        quality_topic: String::new(),
        standard: Some(String::new()),
        target_level: Some(String::new()),
        inspection_points: Vec::new(),
        acceptance_criteria: Vec::new(),
        testing_requirements: Vec::new(),
        sample_rate: Some(String::new()),
        defect_categories: Vec::new(),
        corrective_action_process: Vec::new(),
        element_ids: Vec::new(),
        requirement_ids: Vec::new(),
        supplier_requirements: Vec::new(),
        documentation_requirements: Vec::new(),
        training_requirements: Vec::new(),
        audit_schedule: Some(String::new()),
        kpis: Vec::new(),
        owner_id: Some(EntityId::new_serial("base45", "base45")),
        certification_targets: Vec::new(),
        continuous_improvement: Vec::new(),
    };
    let original = item.clone();
    let patch = QualityRecordPatch {
        name: Some("Patched QualityRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        quality_topic: Some("patched-45".to_string()),
        standard: Some("patched-45".to_string()),
        target_level: Some("patched-45".to_string()),
        inspection_points: Some(vec!["patched-45".to_string()]),
        acceptance_criteria: Some(vec!["patched-45".to_string()]),
        testing_requirements: Some(vec!["patched-45".to_string()]),
        sample_rate: Some("patched-45".to_string()),
        defect_categories: Some(vec!["patched-45".to_string()]),
        corrective_action_process: Some(vec!["patched-45".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new45", "new45")]),
        requirement_ids: Some(vec![EntityId::new_serial("new45", "new45")]),
        supplier_requirements: Some(vec!["patched-45".to_string()]),
        documentation_requirements: Some(vec!["patched-45".to_string()]),
        training_requirements: Some(vec!["patched-45".to_string()]),
        audit_schedule: Some("patched-45".to_string()),
        kpis: Some(vec!["patched-45".to_string()]),
        owner_id: Some(EntityId::new_serial("new45", "new45")),
        certification_targets: Some(vec!["patched-45".to_string()]),
        continuous_improvement: Some(vec!["patched-45".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched QualityRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn document_record_patch_round_trips() {
    let mut item = ArtifactRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("documentrecord", "Base ArtifactRecord"), "Base ArtifactRecord") },
        document_type: String::new(),
        title: String::new(),
        version: String::new(),
        file_ref: Some(String::new()),
        format: Some(String::new()),
        author_ids: Vec::new(),
        reviewer_ids: Vec::new(),
        approver_ids: Vec::new(),
        issue_date: Some(String::new()),
        revision_date: Some(String::new()),
        distribution_list: Vec::new(),
        related_entity_ids: Vec::new(),
        classification: Some(String::new()),
        retention_period: Some(String::new()),
        access_controls: Vec::new(),
        supersedes: Some(EntityId::new_serial("base46", "base46")),
        document_status: LifecycleStatus::Draft,
        checksum: Some(String::new()),
        source_system: Some(String::new()),
    };
    let original = item.clone();
    let patch = ArtifactRecordPatch {
        name: Some("Patched ArtifactRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        document_type: Some("patched-46".to_string()),
        title: Some("patched-46".to_string()),
        version: Some("patched-46".to_string()),
        file_ref: Some("patched-46".to_string()),
        format: Some("patched-46".to_string()),
        author_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
        reviewer_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
        approver_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
        issue_date: Some("patched-46".to_string()),
        revision_date: Some("patched-46".to_string()),
        distribution_list: Some(vec![EntityId::new_serial("new46", "new46")]),
        related_entity_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
        classification: Some("patched-46".to_string()),
        retention_period: Some("patched-46".to_string()),
        access_controls: Some(vec!["patched-46".to_string()]),
        supersedes: Some(EntityId::new_serial("new46", "new46")),
        document_status: Some(LifecycleStatus::Proposed),
        checksum: Some("patched-46".to_string()),
        source_system: Some("patched-46".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ArtifactRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn change_record_patch_round_trips() {
    let mut item = ChangeRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("changerecord", "Base ChangeRecord"), "Base ChangeRecord") },
        change_type: String::new(),
        summary: TextField::default(),
        reason: TextField::default(),
        requested_by: Some(EntityId::new_serial("base47", "base47")),
        approved_by: Some(EntityId::new_serial("base47", "base47")),
        change_date: Some(String::new()),
        effective_date: Some(String::new()),
        impacted_entity_ids: Vec::new(),
        before_snapshot: Some(String::new()),
        after_snapshot: Some(String::new()),
        cost_impact: Some(0.0),
        schedule_impact: Some(String::new()),
        risk_impact: Vec::new(),
        approval_status: ValidationStatus::Pending,
        rollback_plan: Vec::new(),
        communication_plan: Vec::new(),
        version_from: Some(String::new()),
        version_to: Some(String::new()),
        audit_event_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = ChangeRecordPatch {
        name: Some("Patched ChangeRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        change_type: Some("patched-47".to_string()),
        summary: Some(TextField::plain("patched-47")),
        reason: Some(TextField::plain("patched-47")),
        requested_by: Some(EntityId::new_serial("new47", "new47")),
        approved_by: Some(EntityId::new_serial("new47", "new47")),
        change_date: Some("patched-47".to_string()),
        effective_date: Some("patched-47".to_string()),
        impacted_entity_ids: Some(vec![EntityId::new_serial("new47", "new47")]),
        before_snapshot: Some("patched-47".to_string()),
        after_snapshot: Some("patched-47".to_string()),
        cost_impact: Some(42.0),
        schedule_impact: Some("patched-47".to_string()),
        risk_impact: Some(vec!["patched-47".to_string()]),
        approval_status: Some(ValidationStatus::Passed),
        rollback_plan: Some(vec!["patched-47".to_string()]),
        communication_plan: Some(vec!["patched-47".to_string()]),
        version_from: Some("patched-47".to_string()),
        version_to: Some("patched-47".to_string()),
        audit_event_ids: Some(vec![EntityId::new_serial("new47", "new47")]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ChangeRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn collaboration_record_patch_round_trips() {
    let mut item = CollaborationRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("collaborationrecord", "Base CollaborationRecord"), "Base CollaborationRecord") },
        session_type: String::new(),
        title: String::new(),
        participants: Vec::new(),
        facilitator_id: Some(EntityId::new_serial("base48", "base48")),
        start_time: Some(String::new()),
        end_time: Some(String::new()),
        location: Some(String::new()),
        agenda: Vec::new(),
        outcomes: Vec::new(),
        action_items: Vec::new(),
        decision_ids: Vec::new(),
        issue_ids: Vec::new(),
        document_ids: Vec::new(),
        recording_ref: Some(String::new()),
        feedback: Vec::new(),
        follow_up_date: Some(String::new()),
        workshop_id: Some(EntityId::new_serial("base48", "base48")),
        survey_id: Some(EntityId::new_serial("base48", "base48")),
    };
    let original = item.clone();
    let patch = CollaborationRecordPatch {
        name: Some("Patched CollaborationRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        session_type: Some("patched-48".to_string()),
        title: Some("patched-48".to_string()),
        participants: Some(vec![EntityId::new_serial("new48", "new48")]),
        facilitator_id: Some(EntityId::new_serial("new48", "new48")),
        start_time: Some("patched-48".to_string()),
        end_time: Some("patched-48".to_string()),
        location: Some("patched-48".to_string()),
        agenda: Some(vec!["patched-48".to_string()]),
        outcomes: Some(vec!["patched-48".to_string()]),
        action_items: Some(vec!["patched-48".to_string()]),
        decision_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
        issue_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
        document_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
        recording_ref: Some("patched-48".to_string()),
        feedback: Some(vec![TaggedNote { tag: "new48".into(), text: "new-note48".into() }]),
        follow_up_date: Some("patched-48".to_string()),
        workshop_id: Some(EntityId::new_serial("new48", "new48")),
        survey_id: Some(EntityId::new_serial("new48", "new48")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched CollaborationRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn analysis_record_patch_round_trips() {
    let mut item = AnalysisRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("analysisrecord", "Base AnalysisRecord"), "Base AnalysisRecord") },
        kind: AnalysisKind::Gap,
        title: String::new(),
        parameters: Vec::new(),
        input_entity_ids: Vec::new(),
        output_summary: TextField::default(),
        findings: Vec::new(),
        metrics: Vec::new(),
        charts: Vec::new(),
        run_by: Some(EntityId::new_serial("base49", "base49")),
        run_at: Some(String::new()),
        duration_ms: Some(0),
        tool_version: Some(String::new()),
        scenario_id: Some(EntityId::new_serial("base49", "base49")),
        report_id: Some(EntityId::new_serial("base49", "base49")),
        confidence: Some(String::new()),
        limitations: Vec::new(),
        recommendations: Vec::new(),
        raw_result_ref: Some(String::new()),
    };
    let original = item.clone();
    let patch = AnalysisRecordPatch {
        name: Some("Patched AnalysisRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        kind: Some(AnalysisKind::Conflict),
        title: Some("patched-49".to_string()),
        parameters: Some(vec!["patched-49".to_string()]),
        input_entity_ids: Some(vec![EntityId::new_serial("new49", "new49")]),
        output_summary: Some(TextField::plain("patched-49")),
        findings: Some(vec!["patched-49".to_string()]),
        metrics: Some(vec!["patched-49".to_string()]),
        charts: Some(vec!["patched-49".to_string()]),
        run_by: Some(EntityId::new_serial("new49", "new49")),
        run_at: Some("patched-49".to_string()),
        duration_ms: Some(7),
        tool_version: Some("patched-49".to_string()),
        scenario_id: Some(EntityId::new_serial("new49", "new49")),
        report_id: Some(EntityId::new_serial("new49", "new49")),
        confidence: Some("patched-49".to_string()),
        limitations: Some(vec!["patched-49".to_string()]),
        recommendations: Some(vec!["patched-49".to_string()]),
        raw_result_ref: Some("patched-49".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched AnalysisRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn report_record_patch_round_trips() {
    let mut item = ReportRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("reportrecord", "Base ReportRecord"), "Base ReportRecord") },
        kind: ReportKind::ExecutiveSummary,
        title: String::new(),
        audience: Vec::new(),
        sections: Vec::new(),
        generated_at: Some(String::new()),
        generated_by: Some(EntityId::new_serial("base50", "base50")),
        analysis_ids: Vec::new(),
        format: Some(String::new()),
        file_ref: Some(String::new()),
        distribution_list: Vec::new(),
        approval_status: ValidationStatus::Pending,
        approver_id: Some(EntityId::new_serial("base50", "base50")),
        version: String::new(),
        template_id: Some(EntityId::new_serial("base50", "base50")),
        parameters: Vec::new(),
        confidentiality: Some(String::new()),
        expiry_date: Some(String::new()),
        related_decision_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = ReportRecordPatch {
        name: Some("Patched ReportRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        kind: Some(ReportKind::ProgramOverview),
        title: Some("patched-50".to_string()),
        audience: Some(vec!["patched-50".to_string()]),
        sections: Some(vec!["patched-50".to_string()]),
        generated_at: Some("patched-50".to_string()),
        generated_by: Some(EntityId::new_serial("new50", "new50")),
        analysis_ids: Some(vec![EntityId::new_serial("new50", "new50")]),
        format: Some("patched-50".to_string()),
        file_ref: Some("patched-50".to_string()),
        distribution_list: Some(vec![EntityId::new_serial("new50", "new50")]),
        approval_status: Some(ValidationStatus::Passed),
        approver_id: Some(EntityId::new_serial("new50", "new50")),
        version: Some("patched-50".to_string()),
        template_id: Some(EntityId::new_serial("new50", "new50")),
        parameters: Some(vec!["patched-50".to_string()]),
        confidentiality: Some("patched-50".to_string()),
        expiry_date: Some("patched-50".to_string()),
        related_decision_ids: Some(vec![EntityId::new_serial("new50", "new50")]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ReportRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn search_filter_patch_round_trips() {
    let mut item = SearchFilter {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("searchfilter", "Base SearchFilter"), "Base SearchFilter") },
        filter_name: String::new(),
        filter_description: Some(TextField::default()),
        keywords: Vec::new(),
        categories: Vec::new(),
        owner_ids: Vec::new(),
        statuses: Vec::new(),
        priorities: Vec::new(),
        sources: Vec::new(),
        date_from: Some(String::new()),
        date_to: Some(String::new()),
        entity_kinds: Vec::new(),
        tag_filters: Vec::new(),
        sort_field: Some(String::new()),
        sort_direction: Some(String::new()),
        is_public: false,
        created_by: Some(EntityId::new_serial("base51", "base51")),
        last_used: Some(String::new()),
        use_count: 0,
        pinned: false,
    };
    let original = item.clone();
    let patch = SearchFilterPatch {
        name: Some("Patched SearchFilter".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        filter_name: Some("patched-51".to_string()),
        filter_description: Some(TextField::plain("patched-51")),
        keywords: Some(vec!["patched-51".to_string()]),
        categories: Some(vec!["patched-51".to_string()]),
        owner_ids: Some(vec![EntityId::new_serial("new51", "new51")]),
        statuses: Some(vec![LifecycleStatus::Proposed]),
        priorities: Some(vec![Priority::Essential]),
        sources: Some(vec!["patched-51".to_string()]),
        date_from: Some("patched-51".to_string()),
        date_to: Some("patched-51".to_string()),
        entity_kinds: Some(vec!["patched-51".to_string()]),
        tag_filters: Some(vec!["patched-51".to_string()]),
        sort_field: Some("patched-51".to_string()),
        sort_direction: Some("patched-51".to_string()),
        is_public: Some(true),
        created_by: Some(EntityId::new_serial("new51", "new51")),
        last_used: Some("patched-51".to_string()),
        use_count: Some(7),
        pinned: Some(true),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SearchFilter");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn status_record_patch_round_trips() {
    let mut item = StatusRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("statusrecord", "Base StatusRecord"), "Base StatusRecord") },
        subject_id: EntityId::new_serial("base52", "base52"),
        subject_kind: String::new(),
        record_status: LifecycleStatus::Draft,
        previous_status: Some(LifecycleStatus::Draft),
        changed_by: Some(EntityId::new_serial("base52", "base52")),
        changed_at: Some(String::new()),
        reason: Some(TextField::default()),
        blockers: Vec::new(),
        next_actions: Vec::new(),
        due_date: Some(String::new()),
        progress_percent: Some(0.0),
        health: Some(String::new()),
        escalation_level: Some(String::new()),
        related_issue_ids: Vec::new(),
        related_risk_ids: Vec::new(),
        milestone_id: Some(EntityId::new_serial("base52", "base52")),
        reporting_period: Some(String::new()),
        status_notes: Vec::new(),
    };
    let original = item.clone();
    let patch = StatusRecordPatch {
        name: Some("Patched StatusRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        subject_id: Some(EntityId::new_serial("new52", "new52")),
        subject_kind: Some("patched-52".to_string()),
        record_status: Some(LifecycleStatus::Proposed),
        previous_status: Some(LifecycleStatus::Proposed),
        changed_by: Some(EntityId::new_serial("new52", "new52")),
        changed_at: Some("patched-52".to_string()),
        reason: Some(TextField::plain("patched-52")),
        blockers: Some(vec!["patched-52".to_string()]),
        next_actions: Some(vec!["patched-52".to_string()]),
        due_date: Some("patched-52".to_string()),
        progress_percent: Some(42.0),
        health: Some("patched-52".to_string()),
        escalation_level: Some("patched-52".to_string()),
        related_issue_ids: Some(vec![EntityId::new_serial("new52", "new52")]),
        related_risk_ids: Some(vec![EntityId::new_serial("new52", "new52")]),
        milestone_id: Some(EntityId::new_serial("new52", "new52")),
        reporting_period: Some("patched-52".to_string()),
        status_notes: Some(vec![TaggedNote { tag: "new52".into(), text: "new-note52".into() }]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched StatusRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn workshop_patch_round_trips() {
    let mut item = Workshop {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("workshop", "Base Workshop"), "Base Workshop") },
        workshop_type: String::new(),
        objectives: Vec::new(),
        agenda: Vec::new(),
        facilitator_id: Some(EntityId::new_serial("base53", "base53")),
        participants: Vec::new(),
        scheduled_start: Some(String::new()),
        scheduled_end: Some(String::new()),
        location: Some(String::new()),
        materials: Vec::new(),
        methods: Vec::new(),
        outputs: Vec::new(),
        decisions: Vec::new(),
        issues: Vec::new(),
        follow_up_actions: Vec::new(),
        feedback: Vec::new(),
        recording_ref: Some(String::new()),
        budget: Some(0.0),
        workshop_status: LifecycleStatus::Draft,
        survey_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = WorkshopPatch {
        name: Some("Patched Workshop".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        workshop_type: Some("patched-53".to_string()),
        objectives: Some(vec!["patched-53".to_string()]),
        agenda: Some(vec!["patched-53".to_string()]),
        facilitator_id: Some(EntityId::new_serial("new53", "new53")),
        participants: Some(vec![EntityId::new_serial("new53", "new53")]),
        scheduled_start: Some("patched-53".to_string()),
        scheduled_end: Some("patched-53".to_string()),
        location: Some("patched-53".to_string()),
        materials: Some(vec!["patched-53".to_string()]),
        methods: Some(vec!["patched-53".to_string()]),
        outputs: Some(vec!["patched-53".to_string()]),
        decisions: Some(vec![EntityId::new_serial("new53", "new53")]),
        issues: Some(vec![EntityId::new_serial("new53", "new53")]),
        follow_up_actions: Some(vec!["patched-53".to_string()]),
        feedback: Some(vec![TaggedNote { tag: "new53".into(), text: "new-note53".into() }]),
        recording_ref: Some("patched-53".to_string()),
        budget: Some(42.0),
        workshop_status: Some(LifecycleStatus::Proposed),
        survey_ids: Some(vec![EntityId::new_serial("new53", "new53")]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Workshop");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn survey_patch_round_trips() {
    let mut item = Survey {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("survey", "Base Survey"), "Base Survey") },
        survey_type: String::new(),
        title: String::new(),
        objectives: Vec::new(),
        questions: Vec::new(),
        target_audience: Vec::new(),
        distribution_channels: Vec::new(),
        launch_date: Some(String::new()),
        close_date: Some(String::new()),
        response_count: 0,
        response_rate: Some(0.0),
        findings: Vec::new(),
        themes: Vec::new(),
        recommendations: Vec::new(),
        confidentiality: Some(String::new()),
        consent_process: Vec::new(),
        analysis_id: Some(EntityId::new_serial("base54", "base54")),
        workshop_id: Some(EntityId::new_serial("base54", "base54")),
        owner_id: Some(EntityId::new_serial("base54", "base54")),
        survey_status: LifecycleStatus::Draft,
    };
    let original = item.clone();
    let patch = SurveyPatch {
        name: Some("Patched Survey".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        survey_type: Some("patched-54".to_string()),
        title: Some("patched-54".to_string()),
        objectives: Some(vec!["patched-54".to_string()]),
        questions: Some(vec!["patched-54".to_string()]),
        target_audience: Some(vec![EntityId::new_serial("new54", "new54")]),
        distribution_channels: Some(vec!["patched-54".to_string()]),
        launch_date: Some("patched-54".to_string()),
        close_date: Some("patched-54".to_string()),
        response_count: Some(7),
        response_rate: Some(42.0),
        findings: Some(vec!["patched-54".to_string()]),
        themes: Some(vec!["patched-54".to_string()]),
        recommendations: Some(vec!["patched-54".to_string()]),
        confidentiality: Some("patched-54".to_string()),
        consent_process: Some(vec!["patched-54".to_string()]),
        analysis_id: Some(EntityId::new_serial("new54", "new54")),
        workshop_id: Some(EntityId::new_serial("new54", "new54")),
        owner_id: Some(EntityId::new_serial("new54", "new54")),
        survey_status: Some(LifecycleStatus::Proposed),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Survey");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn issue_patch_round_trips() {
    let mut item = Issue {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("issue", "Base Issue"), "Base Issue") },
        issue_type: String::new(),
        summary: TextField::default(),
        issue_description: TextField::default(),
        severity: IssueSeverity::Cosmetic,
        issue_priority: Priority::Mandatory,
        reporter_id: Some(EntityId::new_serial("base55", "base55")),
        assignee_id: Some(EntityId::new_serial("base55", "base55")),
        affected_entity_ids: Vec::new(),
        root_cause: Some(TextField::default()),
        resolution: Some(TextField::default()),
        workaround: Some(TextField::default()),
        due_date: Some(String::new()),
        resolved_date: Some(String::new()),
        related_conflict_ids: Vec::new(),
        related_risk_ids: Vec::new(),
        decision_id: Some(EntityId::new_serial("base55", "base55")),
        comments: Vec::new(),
        attachments: Vec::new(),
        escalation_level: Some(String::new()),
    };
    let original = item.clone();
    let patch = IssuePatch {
        name: Some("Patched Issue".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        issue_type: Some("patched-55".to_string()),
        summary: Some(TextField::plain("patched-55")),
        issue_description: Some(TextField::plain("patched-55")),
        severity: Some(IssueSeverity::Minor),
        issue_priority: Some(Priority::Essential),
        reporter_id: Some(EntityId::new_serial("new55", "new55")),
        assignee_id: Some(EntityId::new_serial("new55", "new55")),
        affected_entity_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
        root_cause: Some(TextField::plain("patched-55")),
        resolution: Some(TextField::plain("patched-55")),
        workaround: Some(TextField::plain("patched-55")),
        due_date: Some("patched-55".to_string()),
        resolved_date: Some("patched-55".to_string()),
        related_conflict_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
        related_risk_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
        decision_id: Some(EntityId::new_serial("new55", "new55")),
        comments: Some(vec![TaggedNote { tag: "new55".into(), text: "new-note55".into() }]),
        attachments: Some(vec![EntityId::new_serial("new55", "new55")]),
        escalation_level: Some("patched-55".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Issue");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn audit_event_patch_round_trips() {
    let mut item = AuditEvent {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("auditevent", "Base AuditEvent"), "Base AuditEvent") },
        action: AuditAction::Created,
        actor_id: Some(EntityId::new_serial("base56", "base56")),
        subject_id: EntityId::new_serial("base56", "base56"),
        subject_kind: String::new(),
        timestamp: String::new(),
        details: TextField::default(),
        before_state: Some(String::new()),
        after_state: Some(String::new()),
        ip_address: Some(String::new()),
        client: Some(String::new()),
        session_id: Some(String::new()),
        change_record_id: Some(EntityId::new_serial("base56", "base56")),
        trace_link: Some(TraceLink::new(EntityId::new_serial("tfrom56", "tfrom56"), EntityId::new_serial("tto56", "tto56"), TraceKind::FullAuditTrail)),
        success: false,
        error_message: Some(String::new()),
        correlation_id: Some(String::new()),
        compliance_tags: Vec::new(),
        retention_until: Some(String::new()),
    };
    let original = item.clone();
    let patch = AuditEventPatch {
        name: Some("Patched AuditEvent".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        action: Some(AuditAction::Updated),
        actor_id: Some(EntityId::new_serial("new56", "new56")),
        subject_id: Some(EntityId::new_serial("new56", "new56")),
        subject_kind: Some("patched-56".to_string()),
        timestamp: Some("patched-56".to_string()),
        details: Some(TextField::plain("patched-56")),
        before_state: Some("patched-56".to_string()),
        after_state: Some("patched-56".to_string()),
        ip_address: Some("patched-56".to_string()),
        client: Some("patched-56".to_string()),
        session_id: Some("patched-56".to_string()),
        change_record_id: Some(EntityId::new_serial("new56", "new56")),
        trace_link: Some(TraceLink::new(EntityId::new_serial("tfrom56n", "tfrom56n"), EntityId::new_serial("tto56n", "tto56n"), TraceKind::FullAuditTrail)),
        success: Some(true),
        error_message: Some("patched-56".to_string()),
        correlation_id: Some("patched-56".to_string()),
        compliance_tags: Some(vec!["patched-56".to_string()]),
        retention_until: Some("patched-56".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched AuditEvent");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn template_record_patch_round_trips() {
    let mut item = TemplateRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("templaterecord", "Base TemplateRecord"), "Base TemplateRecord") },
        template_type: String::new(),
        sector: Some(String::new()),
        project_type: Some(String::new()),
        version: String::new(),
        content_ref: Some(String::new()),
        entity_kinds: Vec::new(),
        default_fields: Vec::new(),
        checklists: Vec::new(),
        standards: Vec::new(),
        applicability: Vec::new(),
        author_id: Some(EntityId::new_serial("base57", "base57")),
        approval_status: ValidationStatus::Pending,
        usage_count: 0,
        last_applied: Some(String::new()),
        customization_notes: Vec::new(),
        related_knowledge_ids: Vec::new(),
        benchmark_ids: Vec::new(),
        license: Some(String::new()),
        source_organization: Some(String::new()),
    };
    let original = item.clone();
    let patch = TemplateRecordPatch {
        name: Some("Patched TemplateRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        template_type: Some("patched-57".to_string()),
        sector: Some("patched-57".to_string()),
        project_type: Some("patched-57".to_string()),
        version: Some("patched-57".to_string()),
        content_ref: Some("patched-57".to_string()),
        entity_kinds: Some(vec!["patched-57".to_string()]),
        default_fields: Some(vec!["patched-57".to_string()]),
        checklists: Some(vec!["patched-57".to_string()]),
        standards: Some(vec!["patched-57".to_string()]),
        applicability: Some(vec!["patched-57".to_string()]),
        author_id: Some(EntityId::new_serial("new57", "new57")),
        approval_status: Some(ValidationStatus::Passed),
        usage_count: Some(7),
        last_applied: Some("patched-57".to_string()),
        customization_notes: Some(vec!["patched-57".to_string()]),
        related_knowledge_ids: Some(vec![EntityId::new_serial("new57", "new57")]),
        benchmark_ids: Some(vec![EntityId::new_serial("new57", "new57")]),
        license: Some("patched-57".to_string()),
        source_organization: Some("patched-57".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched TemplateRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn knowledge_record_patch_round_trips() {
    let mut item = KnowledgeRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("knowledgerecord", "Base KnowledgeRecord"), "Base KnowledgeRecord") },
        topic: String::new(),
        category: String::new(),
        summary: TextField::default(),
        content: TextField::default(),
        sources: Vec::new(),
        references: Vec::new(),
        lessons_learned: Vec::new(),
        best_practices: Vec::new(),
        applicable_sectors: Vec::new(),
        related_entity_kinds: Vec::new(),
        author_ids: Vec::new(),
        expertise_level: Some(String::new()),
        validation_status: ValidationStatus::Pending,
        last_reviewed: Some(String::new()),
        keywords: Vec::new(),
        attachments: Vec::new(),
        citations: Vec::new(),
        usage_count: 0,
    };
    let original = item.clone();
    let patch = KnowledgeRecordPatch {
        name: Some("Patched KnowledgeRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        topic: Some("patched-58".to_string()),
        category: Some("patched-58".to_string()),
        summary: Some(TextField::plain("patched-58")),
        content: Some(TextField::plain("patched-58")),
        sources: Some(vec!["patched-58".to_string()]),
        references: Some(vec!["patched-58".to_string()]),
        lessons_learned: Some(vec!["patched-58".to_string()]),
        best_practices: Some(vec!["patched-58".to_string()]),
        applicable_sectors: Some(vec!["patched-58".to_string()]),
        related_entity_kinds: Some(vec!["patched-58".to_string()]),
        author_ids: Some(vec![EntityId::new_serial("new58", "new58")]),
        expertise_level: Some("patched-58".to_string()),
        validation_status: Some(ValidationStatus::Passed),
        last_reviewed: Some("patched-58".to_string()),
        keywords: Some(vec!["patched-58".to_string()]),
        attachments: Some(vec![EntityId::new_serial("new58", "new58")]),
        citations: Some(vec!["patched-58".to_string()]),
        usage_count: Some(7),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched KnowledgeRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn benchmark_record_patch_round_trips() {
    let mut item = BenchmarkRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("benchmarkrecord", "Base BenchmarkRecord"), "Base BenchmarkRecord") },
        benchmark_name: String::new(),
        sector: String::new(),
        metric: String::new(),
        value: 0.0,
        unit: String::new(),
        sample_size: Some(0),
        source: Some(String::new()),
        collection_year: Some(0),
        geography: Some(String::new()),
        building_type: Some(String::new()),
        confidence: Some(String::new()),
        methodology: Some(String::new()),
        applicable_element_kinds: Vec::new(),
        related_requirement_ids: Vec::new(),
        comparison_notes: Vec::new(),
        limitations: Vec::new(),
        license: Some(String::new()),
        knowledge_id: Some(EntityId::new_serial("base59", "base59")),
        last_verified: Some(String::new()),
    };
    let original = item.clone();
    let patch = BenchmarkRecordPatch {
        name: Some("Patched BenchmarkRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        benchmark_name: Some("patched-59".to_string()),
        sector: Some("patched-59".to_string()),
        metric: Some("patched-59".to_string()),
        value: Some(42.0),
        unit: Some("patched-59".to_string()),
        sample_size: Some(7),
        source: Some("patched-59".to_string()),
        collection_year: Some(7),
        geography: Some("patched-59".to_string()),
        building_type: Some("patched-59".to_string()),
        confidence: Some("patched-59".to_string()),
        methodology: Some("patched-59".to_string()),
        applicable_element_kinds: Some(vec!["patched-59".to_string()]),
        related_requirement_ids: Some(vec![EntityId::new_serial("new59", "new59")]),
        comparison_notes: Some(vec!["patched-59".to_string()]),
        limitations: Some(vec!["patched-59".to_string()]),
        license: Some("patched-59".to_string()),
        knowledge_id: Some(EntityId::new_serial("new59", "new59")),
        last_verified: Some("patched-59".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched BenchmarkRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn assumption_patch_round_trips() {
    let mut item = Assumption {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("assumption", "Base Assumption"), "Base Assumption") },
        statement: TextField::default(),
        basis: Some(TextField::default()),
        confidence_level: Some(String::new()),
        impact_if_false: Some(TextField::default()),
        related_entity_ids: Vec::new(),
        validation_status: ValidationStatus::Pending,
        validated_by: Some(EntityId::new_serial("base60", "base60")),
        validation_date: Some(String::new()),
        owner_id: Some(EntityId::new_serial("base60", "base60")),
        review_cycle: Some(String::new()),
        source: Some(String::new()),
        category: Some(String::new()),
        dependencies: Vec::new(),
        mitigation: Vec::new(),
        linked_requirement_ids: Vec::new(),
        linked_risk_ids: Vec::new(),
        expiration_date: Some(String::new()),
        status_notes: Vec::new(),
        artifact_refs: Vec::new(),
    };
    let original = item.clone();
    let patch = AssumptionPatch {
        name: Some("Patched Assumption".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        statement: Some(TextField::plain("patched-60")),
        basis: Some(TextField::plain("patched-60")),
        confidence_level: Some("patched-60".to_string()),
        impact_if_false: Some(TextField::plain("patched-60")),
        related_entity_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
        validation_status: Some(ValidationStatus::Passed),
        validated_by: Some(EntityId::new_serial("new60", "new60")),
        validation_date: Some("patched-60".to_string()),
        owner_id: Some(EntityId::new_serial("new60", "new60")),
        review_cycle: Some("patched-60".to_string()),
        source: Some("patched-60".to_string()),
        category: Some("patched-60".to_string()),
        dependencies: Some(vec!["patched-60".to_string()]),
        mitigation: Some(vec!["patched-60".to_string()]),
        linked_requirement_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
        linked_risk_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
        expiration_date: Some("patched-60".to_string()),
        status_notes: Some(vec![TaggedNote { tag: "new60".into(), text: "new-note60".into() }]),
        artifact_refs: Some(vec!["patched-60".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Assumption");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn constraint_record_patch_round_trips() {
    let mut item = ConstraintRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("constraintrecord", "Base ConstraintRecord"), "Base ConstraintRecord") },
        constraint_type: String::new(),
        summary: TextField::default(),
        severity: RiskLevel::Negligible,
        affected_entity_ids: Vec::new(),
        source: Some(String::new()),
        regulatory_basis: Vec::new(),
        mitigation_options: Vec::new(),
        owner_id: Some(EntityId::new_serial("base61", "base61")),
        effective_date: Some(String::new()),
        expiry_date: Some(String::new()),
        waiver_status: Some(String::new()),
        waiver_approver: Some(EntityId::new_serial("base61", "base61")),
        impact_assessment: Some(TextField::default()),
        resolution_plan: Vec::new(),
        related_requirement_ids: Vec::new(),
        related_decision_ids: Vec::new(),
        monitoring_frequency: Some(String::new()),
        compliance_status: ValidationStatus::Pending,
        exceptions: Vec::new(),
        trace_links: Vec::new(),
        escalation_contact_id: Some(EntityId::new_serial("base61", "base61")),
    };
    let original = item.clone();
    let patch = ConstraintRecordPatch {
        name: Some("Patched ConstraintRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        constraint_type: Some("patched-61".to_string()),
        summary: Some(TextField::plain("patched-61")),
        severity: Some(RiskLevel::Low),
        affected_entity_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
        source: Some("patched-61".to_string()),
        regulatory_basis: Some(vec!["patched-61".to_string()]),
        mitigation_options: Some(vec!["patched-61".to_string()]),
        owner_id: Some(EntityId::new_serial("new61", "new61")),
        effective_date: Some("patched-61".to_string()),
        expiry_date: Some("patched-61".to_string()),
        waiver_status: Some("patched-61".to_string()),
        waiver_approver: Some(EntityId::new_serial("new61", "new61")),
        impact_assessment: Some(TextField::plain("patched-61")),
        resolution_plan: Some(vec!["patched-61".to_string()]),
        related_requirement_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
        related_decision_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
        monitoring_frequency: Some("patched-61".to_string()),
        compliance_status: Some(ValidationStatus::Passed),
        exceptions: Some(vec!["patched-61".to_string()]),
        trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom61n", "tfrom61n"), EntityId::new_serial("tto61n", "tto61n"), TraceKind::FullAuditTrail)]),
        escalation_contact_id: Some(EntityId::new_serial("new61", "new61")),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ConstraintRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn compliance_record_patch_round_trips() {
    let mut item = ComplianceRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("compliancerecord", "Base ComplianceRecord"), "Base ComplianceRecord") },
        standard_ref: String::new(),
        obligation: TextField::default(),
        compliance_status: ValidationStatus::Pending,
        evidence_refs: Vec::new(),
        auditor_id: Some(EntityId::new_serial("base62", "base62")),
        audit_date: Some(String::new()),
        next_review: Some(String::new()),
        affected_entity_ids: Vec::new(),
        gap_analysis: Vec::new(),
        remediation_plan: Vec::new(),
        owner_id: Some(EntityId::new_serial("base62", "base62")),
        severity: RiskLevel::Negligible,
        regulatory_body: Some(String::new()),
        certification_target: Some(String::new()),
        waiver_status: Some(String::new()),
        related_requirement_ids: Vec::new(),
        monitoring_method: Some(String::new()),
        reporting_frequency: Some(String::new()),
        penalties: Vec::new(),
        corrective_actions: Vec::new(),
        artifact_refs: Vec::new(),
    };
    let original = item.clone();
    let patch = ComplianceRecordPatch {
        name: Some("Patched ComplianceRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        standard_ref: Some("patched-62".to_string()),
        obligation: Some(TextField::plain("patched-62")),
        compliance_status: Some(ValidationStatus::Passed),
        evidence_refs: Some(vec!["patched-62".to_string()]),
        auditor_id: Some(EntityId::new_serial("new62", "new62")),
        audit_date: Some("patched-62".to_string()),
        next_review: Some("patched-62".to_string()),
        affected_entity_ids: Some(vec![EntityId::new_serial("new62", "new62")]),
        gap_analysis: Some(vec!["patched-62".to_string()]),
        remediation_plan: Some(vec!["patched-62".to_string()]),
        owner_id: Some(EntityId::new_serial("new62", "new62")),
        severity: Some(RiskLevel::Low),
        regulatory_body: Some("patched-62".to_string()),
        certification_target: Some("patched-62".to_string()),
        waiver_status: Some("patched-62".to_string()),
        related_requirement_ids: Some(vec![EntityId::new_serial("new62", "new62")]),
        monitoring_method: Some("patched-62".to_string()),
        reporting_frequency: Some("patched-62".to_string()),
        penalties: Some(vec!["patched-62".to_string()]),
        corrective_actions: Some(vec!["patched-62".to_string()]),
        artifact_refs: Some(vec!["patched-62".to_string()]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ComplianceRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn approval_record_patch_round_trips() {
    let mut item = ApprovalRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("approvalrecord", "Base ApprovalRecord"), "Base ApprovalRecord") },
        approval_type: String::new(),
        subject_id: EntityId::new_serial("base63", "base63"),
        approver_ids: Vec::new(),
        approval_date: Some(String::new()),
        conditions: Vec::new(),
        approval_status: LifecycleStatus::Draft,
        expiry_date: Some(String::new()),
        delegation_chain: Vec::new(),
        evidence_refs: Vec::new(),
        related_decision_id: Some(EntityId::new_serial("base63", "base63")),
        related_change_id: Some(EntityId::new_serial("base63", "base63")),
        authority_basis: Vec::new(),
        signature_method: Some(String::new()),
        rejection_reason: Some(TextField::default()),
        resubmission_date: Some(String::new()),
        notification_list: Vec::new(),
        workflow_step: Some(String::new()),
        version: Some(String::new()),
        audit_trail_ref: Some(String::new()),
    };
    let original = item.clone();
    let patch = ApprovalRecordPatch {
        name: Some("Patched ApprovalRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        approval_type: Some("patched-63".to_string()),
        subject_id: Some(EntityId::new_serial("new63", "new63")),
        approver_ids: Some(vec![EntityId::new_serial("new63", "new63")]),
        approval_date: Some("patched-63".to_string()),
        conditions: Some(vec!["patched-63".to_string()]),
        approval_status: Some(LifecycleStatus::Proposed),
        expiry_date: Some("patched-63".to_string()),
        delegation_chain: Some(vec![EntityId::new_serial("new63", "new63")]),
        evidence_refs: Some(vec!["patched-63".to_string()]),
        related_decision_id: Some(EntityId::new_serial("new63", "new63")),
        related_change_id: Some(EntityId::new_serial("new63", "new63")),
        authority_basis: Some(vec!["patched-63".to_string()]),
        signature_method: Some("patched-63".to_string()),
        rejection_reason: Some(TextField::plain("patched-63")),
        resubmission_date: Some("patched-63".to_string()),
        notification_list: Some(vec![EntityId::new_serial("new63", "new63")]),
        workflow_step: Some("patched-63".to_string()),
        version: Some("patched-63".to_string()),
        audit_trail_ref: Some("patched-63".to_string()),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ApprovalRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn meeting_record_patch_round_trips() {
    let mut item = MeetingRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("meetingrecord", "Base MeetingRecord"), "Base MeetingRecord") },
        meeting_type: String::new(),
        scheduled_date: Some(String::new()),
        duration: Some(String::new()),
        location: Some(String::new()),
        chair_id: Some(EntityId::new_serial("base64", "base64")),
        attendee_ids: Vec::new(),
        agenda_items: Vec::new(),
        minutes: Some(TextField::default()),
        action_items: Vec::new(),
        decisions_made: Vec::new(),
        artifact_refs: Vec::new(),
        follow_up_date: Some(String::new()),
        recording_ref: Some(String::new()),
        quorum_met: false,
        meeting_status: LifecycleStatus::Draft,
        workshop_id: Some(EntityId::new_serial("base64", "base64")),
        stakeholder_ids: Vec::new(),
        requirement_ids: Vec::new(),
        issue_ids: Vec::new(),
        approval_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = MeetingRecordPatch {
        name: Some("Patched MeetingRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        meeting_type: Some("patched-64".to_string()),
        scheduled_date: Some("patched-64".to_string()),
        duration: Some("patched-64".to_string()),
        location: Some("patched-64".to_string()),
        chair_id: Some(EntityId::new_serial("new64", "new64")),
        attendee_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        agenda_items: Some(vec!["patched-64".to_string()]),
        minutes: Some(TextField::plain("patched-64")),
        action_items: Some(vec!["patched-64".to_string()]),
        decisions_made: Some(vec![EntityId::new_serial("new64", "new64")]),
        artifact_refs: Some(vec!["patched-64".to_string()]),
        follow_up_date: Some("patched-64".to_string()),
        recording_ref: Some("patched-64".to_string()),
        quorum_met: Some(true),
        meeting_status: Some(LifecycleStatus::Proposed),
        workshop_id: Some(EntityId::new_serial("new64", "new64")),
        stakeholder_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        requirement_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        issue_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        approval_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
    };
    item.apply_patch(&patch);
    let inverse = item.diff_patch(&original).expect("diff_patch always produces a snapshot patch");
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched MeetingRecord");
    item.apply_patch(&inverse);
    assert_eq!(item, original);
}
