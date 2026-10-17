//! 🗺️ Patch round-trip tests of the context registers: site, organizational, service, infrastructure, information, communication, wayfinding, schedule, flexibility, growth, sustainability, resilience, cost and delivery records plus risks and conflicts.

#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::kernel::*;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::registers::kernel::*;
#[allow(unused_imports)]
use protocol::Identified;
#[allow(unused_imports)]
use protocol::list_delta::RowPatch;
use crate::standards::v1::subsets::any::schema::registers::context_requirements::*;

macro_rules! patch_through_program {
    ($item:ident, $patch:expr, $field:ident, $delta:ty, $id:expr) => {{
        let mut program = crate::standards::v1::subsets::any::schema::snapshot::empty_plugin();
        program.$field.push($item.clone());
        let diff = crate::standards::v1::subsets::any::schema::diff::ProgramDiff { $field: Some(<$delta>::modification($id, $patch.clone())), ..Default::default() };
        $item = protocol::apply_diff(&diff, &program).expect("the patch commits through the central applier").$field.remove(0);
    }};
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
