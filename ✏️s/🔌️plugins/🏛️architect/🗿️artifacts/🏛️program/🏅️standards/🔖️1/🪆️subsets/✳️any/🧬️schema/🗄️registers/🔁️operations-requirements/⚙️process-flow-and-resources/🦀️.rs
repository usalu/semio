//! ⚙️ Processes, flow requirements, access rules, operational requirements, equipment, resources and storage.

use super::*;

// #region 🔖️Process
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Process {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub code: String,
    pub category: String,
    pub trigger: Option<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub steps: Vec<String>,
    pub actors: Vec<EntityId>,
    pub equipment_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub duration: Option<String>,
    pub frequency: Option<String>,
    pub critical_path: bool,
    pub bottlenecks: Vec<String>,
    pub dependencies: Vec<EntityId>,
    pub kpis: Vec<String>,
    pub automation_level: Option<String>,
    pub failure_modes: Vec<String>,
    pub improvement_opportunities: Vec<String>,
    pub regulatory_refs: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub workflow_type: Option<String>,
    pub handoff_points: Vec<String>,
    pub quality_gates: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProcessPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub category: Option<String>,
    pub trigger: Option<String>,
    pub inputs: Option<Vec<String>>,
    pub outputs: Option<Vec<String>>,
    pub steps: Option<Vec<String>>,
    pub actors: Option<Vec<EntityId>>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub duration: Option<String>,
    pub frequency: Option<String>,
    pub critical_path: Option<bool>,
    pub bottlenecks: Option<Vec<String>>,
    pub dependencies: Option<Vec<EntityId>>,
    pub kpis: Option<Vec<String>>,
    pub automation_level: Option<String>,
    pub failure_modes: Option<Vec<String>>,
    pub improvement_opportunities: Option<Vec<String>>,
    pub regulatory_refs: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub workflow_type: Option<String>,
    pub handoff_points: Option<Vec<String>>,
    pub quality_gates: Option<Vec<String>>,
}

impl_identified_header!(Process);

impl_patchable!(
    Process,
    ProcessPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [category] => category,
        [trigger] => trigger,
        [inputs] => inputs,
        [outputs] => outputs,
        [steps] => steps,
        [actors] => actors,
        [equipment_ids] => equipment_ids,
        [element_ids] => element_ids,
        [duration] => duration,
        [frequency] => frequency,
        [critical_path] => critical_path,
        [bottlenecks] => bottlenecks,
        [dependencies] => dependencies,
        [kpis] => kpis,
        [automation_level] => automation_level,
        [failure_modes] => failure_modes,
        [improvement_opportunities] => improvement_opportunities,
        [regulatory_refs] => regulatory_refs,
        [owner_id] => owner_id,
        [workflow_type] => workflow_type,
        [handoff_points] => handoff_points,
        [quality_gates] => quality_gates,
    }
);
// #endregion

// #region 🔖️FlowRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FlowRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub from_element_id: EntityId,
    pub to_element_id: EntityId,
    pub kind: FlowKind,
    pub flow_type: String,
    pub direction: FlowDirection,
    pub volume: QuantitySpec,
    pub peak_rate: Option<f64>,
    #[dsl(unit = "m")]
    pub clear_width_m: Option<f64>,
    #[dsl(unit = "m")]
    pub clear_height_m: Option<f64>,
    pub separation_requirements: Vec<SeparationKind>,
    pub access_level: AccessLevel,
    pub time_windows: Vec<String>,
    pub equipment_clearance: Option<String>,
    pub signage_required: bool,
    pub escort_required: bool,
    pub emergency_route: bool,
    pub barrier_free: bool,
    pub monitoring_required: bool,
    pub process_id: Option<EntityId>,
    pub conflict_ids: Vec<EntityId>,
    pub verification_method: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FlowRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub from_element_id: Option<EntityId>,
    pub to_element_id: Option<EntityId>,
    pub kind: Option<FlowKind>,
    pub flow_type: Option<String>,
    pub direction: Option<FlowDirection>,
    pub volume: Option<QuantitySpec>,
    pub peak_rate: Option<f64>,
    pub clear_width_m: Option<f64>,
    pub clear_height_m: Option<f64>,
    pub separation_requirements: Option<Vec<SeparationKind>>,
    pub access_level: Option<AccessLevel>,
    pub time_windows: Option<Vec<String>>,
    pub equipment_clearance: Option<String>,
    pub signage_required: Option<bool>,
    pub escort_required: Option<bool>,
    pub emergency_route: Option<bool>,
    pub barrier_free: Option<bool>,
    pub monitoring_required: Option<bool>,
    pub process_id: Option<EntityId>,
    pub conflict_ids: Option<Vec<EntityId>>,
    pub verification_method: Option<String>,
}

impl_identified_header!(FlowRequirement);

impl_patchable!(
    FlowRequirement,
    FlowRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [from_element_id] => from_element_id,
        [to_element_id] => to_element_id,
        [kind] => kind,
        [flow_type] => flow_type,
        [direction] => direction,
        [volume] => volume,
        [peak_rate] => peak_rate,
        [clear_width_m] => clear_width_m,
        [clear_height_m] => clear_height_m,
        [separation_requirements] => separation_requirements,
        [access_level] => access_level,
        [time_windows] => time_windows,
        [equipment_clearance] => equipment_clearance,
        [signage_required] => signage_required,
        [escort_required] => escort_required,
        [emergency_route] => emergency_route,
        [barrier_free] => barrier_free,
        [monitoring_required] => monitoring_required,
        [process_id] => process_id,
        [conflict_ids] => conflict_ids,
        [verification_method] => verification_method,
    }
);
// #endregion

// #region 🔖️AccessRule
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AccessRule {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub subject_ids: Vec<EntityId>,
    pub resource_ids: Vec<EntityId>,
    pub access_level: AccessLevel,
    pub access_mode: AccessMode,
    pub authentication: Vec<String>,
    pub authorization: Vec<String>,
    pub time_restrictions: Vec<String>,
    pub escort_policy: Option<String>,
    pub visitor_policy: Option<String>,
    pub emergency_override: bool,
    pub audit_required: bool,
    pub badge_required: bool,
    pub biometric_required: bool,
    pub zone_ids: Vec<EntityId>,
    pub exceptions: Vec<String>,
    pub regulatory_basis: Vec<String>,
    pub enforcement_method: Option<String>,
    pub revocation_policy: Option<String>,
    pub training_required: bool,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AccessRulePatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub subject_ids: Option<Vec<EntityId>>,
    pub resource_ids: Option<Vec<EntityId>>,
    pub access_level: Option<AccessLevel>,
    pub access_mode: Option<AccessMode>,
    pub authentication: Option<Vec<String>>,
    pub authorization: Option<Vec<String>>,
    pub time_restrictions: Option<Vec<String>>,
    pub escort_policy: Option<String>,
    pub visitor_policy: Option<String>,
    pub emergency_override: Option<bool>,
    pub audit_required: Option<bool>,
    pub badge_required: Option<bool>,
    pub biometric_required: Option<bool>,
    pub zone_ids: Option<Vec<EntityId>>,
    pub exceptions: Option<Vec<String>>,
    pub regulatory_basis: Option<Vec<String>>,
    pub enforcement_method: Option<String>,
    pub revocation_policy: Option<String>,
    pub training_required: Option<bool>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(AccessRule);

impl_patchable!(
    AccessRule,
    AccessRulePatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [subject_ids] => subject_ids,
        [resource_ids] => resource_ids,
        [access_level] => access_level,
        [access_mode] => access_mode,
        [authentication] => authentication,
        [authorization] => authorization,
        [time_restrictions] => time_restrictions,
        [escort_policy] => escort_policy,
        [visitor_policy] => visitor_policy,
        [emergency_override] => emergency_override,
        [audit_required] => audit_required,
        [badge_required] => badge_required,
        [biometric_required] => biometric_required,
        [zone_ids] => zone_ids,
        [exceptions] => exceptions,
        [regulatory_basis] => regulatory_basis,
        [enforcement_method] => enforcement_method,
        [revocation_policy] => revocation_policy,
        [training_required] => training_required,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️OperationalRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct OperationalRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub operation: String,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub staffing: QuantitySpec,
    pub maintenance_interval: Option<String>,
    pub cleaning_regime: Option<String>,
    pub turnaround_time: Option<String>,
    pub redundancy: Option<String>,
    pub uptime_target: Option<f64>,
    pub response_time: Option<String>,
    pub equipment_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub process_ids: Vec<EntityId>,
    pub utilities: Vec<String>,
    pub waste_streams: Vec<String>,
    pub contingency_plan: Vec<String>,
    pub training_requirements: Vec<String>,
    pub sop_references: Vec<String>,
    pub kpi_targets: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub service_category: Option<String>,
    pub shift_pattern: Option<String>,
    pub sla_target: Option<String>,
    pub escalation_contact_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct OperationalRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub operation: Option<String>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub staffing: Option<QuantitySpec>,
    pub maintenance_interval: Option<String>,
    pub cleaning_regime: Option<String>,
    pub turnaround_time: Option<String>,
    pub redundancy: Option<String>,
    pub uptime_target: Option<f64>,
    pub response_time: Option<String>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub process_ids: Option<Vec<EntityId>>,
    pub utilities: Option<Vec<String>>,
    pub waste_streams: Option<Vec<String>>,
    pub contingency_plan: Option<Vec<String>>,
    pub training_requirements: Option<Vec<String>>,
    pub sop_references: Option<Vec<String>>,
    pub kpi_targets: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub service_category: Option<String>,
    pub shift_pattern: Option<String>,
    pub sla_target: Option<String>,
    pub escalation_contact_id: Option<EntityId>,
}

impl_identified_header!(OperationalRequirement);

impl_patchable!(
    OperationalRequirement,
    OperationalRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [operation] => operation,
        [service_level] => service_level,
        [operating_hours] => operating_hours,
        [staffing] => staffing,
        [maintenance_interval] => maintenance_interval,
        [cleaning_regime] => cleaning_regime,
        [turnaround_time] => turnaround_time,
        [redundancy] => redundancy,
        [uptime_target] => uptime_target,
        [response_time] => response_time,
        [equipment_ids] => equipment_ids,
        [element_ids] => element_ids,
        [process_ids] => process_ids,
        [utilities] => utilities,
        [waste_streams] => waste_streams,
        [contingency_plan] => contingency_plan,
        [training_requirements] => training_requirements,
        [sop_references] => sop_references,
        [kpi_targets] => kpi_targets,
        [owner_id] => owner_id,
        [service_category] => service_category,
        [shift_pattern] => shift_pattern,
        [sla_target] => sla_target,
        [escalation_contact_id] => escalation_contact_id,
    }
);
// #endregion

// #region 🔖️Equipment
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Equipment {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub code: String,
    pub category: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub quantity: QuantitySpec,
    pub dimensions: Option<String>,
    #[dsl(unit = "kg")]
    pub weight_kg: Option<f64>,
    #[dsl(unit = "kW")]
    pub power_kw: Option<f64>,
    pub utility_connections: Vec<String>,
    pub ventilation: Option<String>,
    pub noise_level_db: Option<f64>,
    pub clearance: Option<String>,
    pub mounting: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub maintenance_access: Vec<String>,
    pub lifecycle_years: Option<u32>,
    pub replacement_cost: Option<f64>,
    pub standards: Vec<String>,
    pub supplier: Option<String>,
    pub activity_link_ids: Vec<EntityId>,
    pub installation_requirements: Vec<String>,
    pub commissioning_notes: Vec<String>,
    pub spare_parts: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct EquipmentPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub category: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub quantity: Option<QuantitySpec>,
    pub dimensions: Option<String>,
    pub weight_kg: Option<f64>,
    pub power_kw: Option<f64>,
    pub utility_connections: Option<Vec<String>>,
    pub ventilation: Option<String>,
    pub noise_level_db: Option<f64>,
    pub clearance: Option<String>,
    pub mounting: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub maintenance_access: Option<Vec<String>>,
    pub lifecycle_years: Option<u32>,
    pub replacement_cost: Option<f64>,
    pub standards: Option<Vec<String>>,
    pub supplier: Option<String>,
    pub activity_link_ids: Option<Vec<EntityId>>,
    pub installation_requirements: Option<Vec<String>>,
    pub commissioning_notes: Option<Vec<String>>,
    pub spare_parts: Option<Vec<String>>,
}

impl_identified_header!(Equipment);

impl_patchable!(
    Equipment,
    EquipmentPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [category] => category,
        [manufacturer] => manufacturer,
        [model] => model,
        [quantity] => quantity,
        [dimensions] => dimensions,
        [weight_kg] => weight_kg,
        [power_kw] => power_kw,
        [utility_connections] => utility_connections,
        [ventilation] => ventilation,
        [noise_level_db] => noise_level_db,
        [clearance] => clearance,
        [mounting] => mounting,
        [element_ids] => element_ids,
        [activity_ids] => activity_ids,
        [maintenance_access] => maintenance_access,
        [lifecycle_years] => lifecycle_years,
        [replacement_cost] => replacement_cost,
        [standards] => standards,
        [supplier] => supplier,
        [activity_link_ids] => activity_link_ids,
        [installation_requirements] => installation_requirements,
        [commissioning_notes] => commissioning_notes,
        [spare_parts] => spare_parts,
    }
);
// #endregion

// #region 🔖️Resource
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Resource {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub code: String,
    pub category: String,
    pub resource_type: String,
    pub quantity: QuantitySpec,
    pub mobility: Option<String>,
    pub sharing_model: Option<String>,
    pub allocation: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub user_profile_ids: Vec<EntityId>,
    pub storage_requirement_id: Option<EntityId>,
    pub durability: Option<String>,
    pub cleaning_requirements: Vec<String>,
    pub replacement_cycle: Option<String>,
    pub cost_per_unit: Option<f64>,
    pub supplier: Option<String>,
    pub standards: Vec<String>,
    pub ergonomic_notes: Vec<String>,
    pub customization: Vec<String>,
    pub disposal_notes: Vec<String>,
    pub furniture_class: Option<String>,
    pub ergonomics_rating: Option<String>,
    pub sharing_ratio: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ResourcePatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub category: Option<String>,
    pub resource_type: Option<String>,
    pub quantity: Option<QuantitySpec>,
    pub mobility: Option<String>,
    pub sharing_model: Option<String>,
    pub allocation: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub storage_requirement_id: Option<EntityId>,
    pub durability: Option<String>,
    pub cleaning_requirements: Option<Vec<String>>,
    pub replacement_cycle: Option<String>,
    pub cost_per_unit: Option<f64>,
    pub supplier: Option<String>,
    pub standards: Option<Vec<String>>,
    pub ergonomic_notes: Option<Vec<String>>,
    pub customization: Option<Vec<String>>,
    pub disposal_notes: Option<Vec<String>>,
    pub furniture_class: Option<String>,
    pub ergonomics_rating: Option<String>,
    pub sharing_ratio: Option<f64>,
}

impl_identified_header!(Resource);

impl_patchable!(
    Resource,
    ResourcePatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [code] => code,
        [category] => category,
        [resource_type] => resource_type,
        [quantity] => quantity,
        [mobility] => mobility,
        [sharing_model] => sharing_model,
        [allocation] => allocation,
        [element_ids] => element_ids,
        [activity_ids] => activity_ids,
        [user_profile_ids] => user_profile_ids,
        [storage_requirement_id] => storage_requirement_id,
        [durability] => durability,
        [cleaning_requirements] => cleaning_requirements,
        [replacement_cycle] => replacement_cycle,
        [cost_per_unit] => cost_per_unit,
        [supplier] => supplier,
        [standards] => standards,
        [ergonomic_notes] => ergonomic_notes,
        [customization] => customization,
        [disposal_notes] => disposal_notes,
        [furniture_class] => furniture_class,
        [ergonomics_rating] => ergonomics_rating,
        [sharing_ratio] => sharing_ratio,
    }
);
// #endregion

// #region 🔖️StorageRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct StorageRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub stored_item: String,
    pub storage_class: StorageClass,
    pub quantity: QuantitySpec,
    #[dsl(unit = "m3")]
    pub volume_m3: Option<f64>,
    #[dsl(unit = "kg")]
    pub weight_kg: Option<f64>,
    pub temperature_range: Option<String>,
    pub humidity_range: Option<String>,
    pub security_level: AccessLevel,
    pub hazard_class: Option<String>,
    pub retention_period: Option<String>,
    pub access_frequency: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub equipment_ids: Vec<EntityId>,
    pub handling_equipment: Vec<String>,
    pub fire_protection: Vec<String>,
    pub ventilation: Option<String>,
    pub organization_system: Option<String>,
    pub growth_allowance: Option<f64>,
    pub regulatory_refs: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct StorageRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub stored_item: Option<String>,
    pub storage_class: Option<StorageClass>,
    pub quantity: Option<QuantitySpec>,
    pub volume_m3: Option<f64>,
    pub weight_kg: Option<f64>,
    pub temperature_range: Option<String>,
    pub humidity_range: Option<String>,
    pub security_level: Option<AccessLevel>,
    pub hazard_class: Option<String>,
    pub retention_period: Option<String>,
    pub access_frequency: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub handling_equipment: Option<Vec<String>>,
    pub fire_protection: Option<Vec<String>>,
    pub ventilation: Option<String>,
    pub organization_system: Option<String>,
    pub growth_allowance: Option<f64>,
    pub regulatory_refs: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(StorageRequirement);

impl_patchable!(
    StorageRequirement,
    StorageRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [stored_item] => stored_item,
        [storage_class] => storage_class,
        [quantity] => quantity,
        [volume_m3] => volume_m3,
        [weight_kg] => weight_kg,
        [temperature_range] => temperature_range,
        [humidity_range] => humidity_range,
        [security_level] => security_level,
        [hazard_class] => hazard_class,
        [retention_period] => retention_period,
        [access_frequency] => access_frequency,
        [element_ids] => element_ids,
        [equipment_ids] => equipment_ids,
        [handling_equipment] => handling_equipment,
        [fire_protection] => fire_protection,
        [ventilation] => ventilation,
        [organization_system] => organization_system,
        [growth_allowance] => growth_allowance,
        [regulatory_refs] => regulatory_refs,
        [owner_id] => owner_id,
    }
);
// #endregion
