/// 🏞️ Site, organization, service, infrastructure, information and communication patches: the patch round trips of the site context and of the organizational, service, infrastructure, information and communication requirements through the central applier.

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
    patch_through_program!(item, patch, site_context, crate::standards::v1::subsets::any::schema::diff::ProgramSiteContextDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SiteContext");
    patch_through_program!(item, inverse, site_context, crate::standards::v1::subsets::any::schema::diff::ProgramSiteContextDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, organizational, crate::standards::v1::subsets::any::schema::diff::ProgramOrganizationalDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched OrganizationalRequirement");
    patch_through_program!(item, inverse, organizational, crate::standards::v1::subsets::any::schema::diff::ProgramOrganizationalDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, services, crate::standards::v1::subsets::any::schema::diff::ProgramServicesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ServiceRequirement");
    patch_through_program!(item, inverse, services, crate::standards::v1::subsets::any::schema::diff::ProgramServicesDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, infrastructure, crate::standards::v1::subsets::any::schema::diff::ProgramInfrastructureDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched InfrastructureRequirement");
    patch_through_program!(item, inverse, infrastructure, crate::standards::v1::subsets::any::schema::diff::ProgramInfrastructureDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, information, crate::standards::v1::subsets::any::schema::diff::ProgramInformationDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched InformationRequirement");
    patch_through_program!(item, inverse, information, crate::standards::v1::subsets::any::schema::diff::ProgramInformationDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, communication, crate::standards::v1::subsets::any::schema::diff::ProgramCommunicationDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched CommunicationRequirement");
    patch_through_program!(item, inverse, communication, crate::standards::v1::subsets::any::schema::diff::ProgramCommunicationDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}
