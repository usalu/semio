//! 🏗️ Patch round-trip tests of the program structure registers: meta, project, stakeholders, users, activities, functions, elements, quantities, relationships and adjacencies.

use super::*;

macro_rules! patch_through_program {
    ($item:ident, $patch:expr, $field:ident, $delta:ty, $id:expr) => {{
        let mut program = crate::standards::v1::subsets::any::schema::snapshot::empty_plugin();
        program.$field.push($item.clone());
        let diff = crate::standards::v1::subsets::any::schema::diff::ProgramDiff { $field: Some(<$delta>::modification($id, $patch.clone())), ..Default::default() };
        $item = protocol::apply_diff(&diff, &program).expect("the patch commits through the central applier").$field.remove(0);
    }};
}

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
    patch_through_program!(item, patch, stakeholders, crate::standards::v1::subsets::any::schema::diff::ProgramStakeholdersDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Stakeholder");
    patch_through_program!(item, inverse, stakeholders, crate::standards::v1::subsets::any::schema::diff::ProgramStakeholdersDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, users, crate::standards::v1::subsets::any::schema::diff::ProgramUsersDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched UserProfile");
    patch_through_program!(item, inverse, users, crate::standards::v1::subsets::any::schema::diff::ProgramUsersDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, activities, crate::standards::v1::subsets::any::schema::diff::ProgramActivitiesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Activity");
    patch_through_program!(item, inverse, activities, crate::standards::v1::subsets::any::schema::diff::ProgramActivitiesDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, functions, crate::standards::v1::subsets::any::schema::diff::ProgramFunctionsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Function");
    patch_through_program!(item, inverse, functions, crate::standards::v1::subsets::any::schema::diff::ProgramFunctionsDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, elements, crate::standards::v1::subsets::any::schema::diff::ProgramElementsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ProgramElement");
    patch_through_program!(item, inverse, elements, crate::standards::v1::subsets::any::schema::diff::ProgramElementsDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, quantities, crate::standards::v1::subsets::any::schema::diff::ProgramQuantitiesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched QuantityRequirement");
    patch_through_program!(item, inverse, quantities, crate::standards::v1::subsets::any::schema::diff::ProgramQuantitiesDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, relationships, crate::standards::v1::subsets::any::schema::diff::ProgramRelationshipsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Relationship");
    patch_through_program!(item, inverse, relationships, crate::standards::v1::subsets::any::schema::diff::ProgramRelationshipsDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, adjacencies, crate::standards::v1::subsets::any::schema::diff::ProgramAdjacenciesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Adjacency");
    patch_through_program!(item, inverse, adjacencies, crate::standards::v1::subsets::any::schema::diff::ProgramAdjacenciesDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}
