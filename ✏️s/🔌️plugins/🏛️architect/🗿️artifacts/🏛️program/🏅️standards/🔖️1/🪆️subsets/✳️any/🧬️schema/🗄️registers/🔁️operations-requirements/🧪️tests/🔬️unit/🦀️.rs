//! ⚙️ Patch round-trip tests of the operations and requirement registers: processes, flows, access rules, equipment, resources and the operational, storage, environmental, human factor, accessibility, privacy, safety, security and regulatory requirements.

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
    patch_through_program!(item, patch, processes, crate::standards::v1::subsets::any::schema::diff::ProgramProcessesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Process");
    patch_through_program!(item, inverse, processes, crate::standards::v1::subsets::any::schema::diff::ProgramProcessesDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, flows, crate::standards::v1::subsets::any::schema::diff::ProgramFlowsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched FlowRequirement");
    patch_through_program!(item, inverse, flows, crate::standards::v1::subsets::any::schema::diff::ProgramFlowsDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, access_rules, crate::standards::v1::subsets::any::schema::diff::ProgramAccessRulesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched AccessRule");
    patch_through_program!(item, inverse, access_rules, crate::standards::v1::subsets::any::schema::diff::ProgramAccessRulesDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, operations, crate::standards::v1::subsets::any::schema::diff::ProgramOperationsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched OperationalRequirement");
    patch_through_program!(item, inverse, operations, crate::standards::v1::subsets::any::schema::diff::ProgramOperationsDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, equipment, crate::standards::v1::subsets::any::schema::diff::ProgramEquipmentDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Equipment");
    patch_through_program!(item, inverse, equipment, crate::standards::v1::subsets::any::schema::diff::ProgramEquipmentDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, resources, crate::standards::v1::subsets::any::schema::diff::ProgramResourcesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Resource");
    patch_through_program!(item, inverse, resources, crate::standards::v1::subsets::any::schema::diff::ProgramResourcesDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, storage, crate::standards::v1::subsets::any::schema::diff::ProgramStorageDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched StorageRequirement");
    patch_through_program!(item, inverse, storage, crate::standards::v1::subsets::any::schema::diff::ProgramStorageDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, environmental, crate::standards::v1::subsets::any::schema::diff::ProgramEnvironmentalDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched EnvironmentalRequirement");
    patch_through_program!(item, inverse, environmental, crate::standards::v1::subsets::any::schema::diff::ProgramEnvironmentalDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, human_factors, crate::standards::v1::subsets::any::schema::diff::ProgramHumanFactorsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched HumanFactorRequirement");
    patch_through_program!(item, inverse, human_factors, crate::standards::v1::subsets::any::schema::diff::ProgramHumanFactorsDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, accessibility, crate::standards::v1::subsets::any::schema::diff::ProgramAccessibilityDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched AccessibilityRequirement");
    patch_through_program!(item, inverse, accessibility, crate::standards::v1::subsets::any::schema::diff::ProgramAccessibilityDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, privacy, crate::standards::v1::subsets::any::schema::diff::ProgramPrivacyDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched PrivacyRequirement");
    patch_through_program!(item, inverse, privacy, crate::standards::v1::subsets::any::schema::diff::ProgramPrivacyDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, safety, crate::standards::v1::subsets::any::schema::diff::ProgramSafetyDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SafetyRequirement");
    patch_through_program!(item, inverse, safety, crate::standards::v1::subsets::any::schema::diff::ProgramSafetyDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, security, crate::standards::v1::subsets::any::schema::diff::ProgramSecurityDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SecurityRequirement");
    patch_through_program!(item, inverse, security, crate::standards::v1::subsets::any::schema::diff::ProgramSecurityDelta, item.header.id.0.clone());
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
    patch_through_program!(item, patch, regulatory, crate::standards::v1::subsets::any::schema::diff::ProgramRegulatoryDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched RegulatoryRequirement");
    patch_through_program!(item, inverse, regulatory, crate::standards::v1::subsets::any::schema::diff::ProgramRegulatoryDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}
