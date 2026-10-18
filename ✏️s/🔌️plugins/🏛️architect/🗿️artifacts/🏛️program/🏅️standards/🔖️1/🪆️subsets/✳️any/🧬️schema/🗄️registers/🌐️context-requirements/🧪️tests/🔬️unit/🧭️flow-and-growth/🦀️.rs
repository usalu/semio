/// 🧭️ Wayfinding, schedule, flexibility, growth and sustainability patches: the patch round trips of the wayfinding, schedule and flexibility requirements, the growth plan and the sustainability requirement through the central applier.

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
    patch_through_program!(item, patch, wayfinding, crate::standards::v1::subsets::any::schema::diff::ProgramWayfindingDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched WayfindingRequirement");
    patch_through_program!(item, inverse, wayfinding, crate::standards::v1::subsets::any::schema::diff::ProgramWayfindingDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, schedules, crate::standards::v1::subsets::any::schema::diff::ProgramSchedulesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ScheduleRequirement");
    patch_through_program!(item, inverse, schedules, crate::standards::v1::subsets::any::schema::diff::ProgramSchedulesDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, flexibility, crate::standards::v1::subsets::any::schema::diff::ProgramFlexibilityDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched FlexibilityRequirement");
    patch_through_program!(item, inverse, flexibility, crate::standards::v1::subsets::any::schema::diff::ProgramFlexibilityDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, growth, crate::standards::v1::subsets::any::schema::diff::ProgramGrowthDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched GrowthPlan");
    patch_through_program!(item, inverse, growth, crate::standards::v1::subsets::any::schema::diff::ProgramGrowthDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, sustainability, crate::standards::v1::subsets::any::schema::diff::ProgramSustainabilityDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SustainabilityRequirement");
    patch_through_program!(item, inverse, sustainability, crate::standards::v1::subsets::any::schema::diff::ProgramSustainabilityDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}
