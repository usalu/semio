//! 🌐️ Architect context registers: site, organizational, service, infrastructure, information, communication, wayfinding, schedule, flexibility, growth, sustainability, resilience, cost and delivery records plus risks and conflicts; the shared enums live in `kernel` and the patch macros in [`super`].

use super::*;

// #region 🔖️SiteContext
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SiteContext {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub site_name: String,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[dsl(unit = "m")]
    pub elevation_m: Option<f64>,
    pub climate_zone: Option<String>,
    pub seismic_zone: Option<String>,
    pub flood_risk: Option<String>,
    pub soil_conditions: Vec<String>,
    pub utilities_available: Vec<String>,
    pub access_roads: Vec<String>,
    pub public_transit: Vec<String>,
    pub neighbors: Vec<String>,
    pub views: Vec<String>,
    pub noise_sources: Vec<String>,
    pub environmental_constraints: Vec<String>,
    pub heritage_constraints: Vec<String>,
    pub zoning: Option<String>,
    #[dsl(unit = "m")]
    pub max_height_m: Option<f64>,
    pub max_coverage: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SiteContextPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub site_name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation_m: Option<f64>,
    pub climate_zone: Option<String>,
    pub seismic_zone: Option<String>,
    pub flood_risk: Option<String>,
    pub soil_conditions: Option<Vec<String>>,
    pub utilities_available: Option<Vec<String>>,
    pub access_roads: Option<Vec<String>>,
    pub public_transit: Option<Vec<String>>,
    pub neighbors: Option<Vec<String>>,
    pub views: Option<Vec<String>>,
    pub noise_sources: Option<Vec<String>>,
    pub environmental_constraints: Option<Vec<String>>,
    pub heritage_constraints: Option<Vec<String>>,
    pub zoning: Option<String>,
    pub max_height_m: Option<f64>,
    pub max_coverage: Option<f64>,
}

impl_identified_header!(SiteContext);

impl_patchable!(
    SiteContext,
    SiteContextPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [site_name] => site_name,
        [address] => address,
        [latitude] => latitude,
        [longitude] => longitude,
        [elevation_m] => elevation_m,
        [climate_zone] => climate_zone,
        [seismic_zone] => seismic_zone,
        [flood_risk] => flood_risk,
        [soil_conditions] => soil_conditions,
        [utilities_available] => utilities_available,
        [access_roads] => access_roads,
        [public_transit] => public_transit,
        [neighbors] => neighbors,
        [views] => views,
        [noise_sources] => noise_sources,
        [environmental_constraints] => environmental_constraints,
        [heritage_constraints] => heritage_constraints,
        [zoning] => zoning,
        [max_height_m] => max_height_m,
        [max_coverage] => max_coverage,
    }
);
// #endregion

// #region 🔖️OrganizationalRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct OrganizationalRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub department: String,
    pub reporting_line: Option<String>,
    pub headcount: QuantitySpec,
    pub growth_plan_id: Option<EntityId>,
    pub work_patterns: Vec<String>,
    pub collaboration_model: Option<String>,
    pub hierarchy_levels: Vec<String>,
    pub decision_making: Vec<String>,
    pub culture_notes: Vec<String>,
    pub change_readiness: Option<String>,
    pub union_considerations: Vec<String>,
    pub training_needs: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub service_requirement_ids: Vec<EntityId>,
    pub branding_requirements: Vec<String>,
    pub wellness_plugins: Vec<String>,
    pub diversity_goals: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct OrganizationalRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub department: Option<String>,
    pub reporting_line: Option<String>,
    pub headcount: Option<QuantitySpec>,
    pub growth_plan_id: Option<EntityId>,
    pub work_patterns: Option<Vec<String>>,
    pub collaboration_model: Option<String>,
    pub hierarchy_levels: Option<Vec<String>>,
    pub decision_making: Option<Vec<String>>,
    pub culture_notes: Option<Vec<String>>,
    pub change_readiness: Option<String>,
    pub union_considerations: Option<Vec<String>>,
    pub training_needs: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub service_requirement_ids: Option<Vec<EntityId>>,
    pub branding_requirements: Option<Vec<String>>,
    pub wellness_plugins: Option<Vec<String>>,
    pub diversity_goals: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(OrganizationalRequirement);

impl_patchable!(
    OrganizationalRequirement,
    OrganizationalRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [department] => department,
        [reporting_line] => reporting_line,
        [headcount] => headcount,
        [growth_plan_id] => growth_plan_id,
        [work_patterns] => work_patterns,
        [collaboration_model] => collaboration_model,
        [hierarchy_levels] => hierarchy_levels,
        [decision_making] => decision_making,
        [culture_notes] => culture_notes,
        [change_readiness] => change_readiness,
        [union_considerations] => union_considerations,
        [training_needs] => training_needs,
        [element_ids] => element_ids,
        [stakeholder_ids] => stakeholder_ids,
        [service_requirement_ids] => service_requirement_ids,
        [branding_requirements] => branding_requirements,
        [wellness_plugins] => wellness_plugins,
        [diversity_goals] => diversity_goals,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️ServiceRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ServiceRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub service_name: String,
    pub service_type: String,
    pub provider: Option<String>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub capacity: QuantitySpec,
    pub response_time: Option<String>,
    pub queue_management: Vec<String>,
    pub customer_profiles: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub equipment_ids: Vec<EntityId>,
    pub staffing: QuantitySpec,
    pub quality_metrics: Vec<String>,
    pub cost_model: Option<String>,
    pub contract_refs: Vec<String>,
    pub dependencies: Vec<EntityId>,
    pub failure_impact: Option<String>,
    pub backup_service: Vec<String>,
    pub feedback_channels: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ServiceRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub service_name: Option<String>,
    pub service_type: Option<String>,
    pub provider: Option<String>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub capacity: Option<QuantitySpec>,
    pub response_time: Option<String>,
    pub queue_management: Option<Vec<String>>,
    pub customer_profiles: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub staffing: Option<QuantitySpec>,
    pub quality_metrics: Option<Vec<String>>,
    pub cost_model: Option<String>,
    pub contract_refs: Option<Vec<String>>,
    pub dependencies: Option<Vec<EntityId>>,
    pub failure_impact: Option<String>,
    pub backup_service: Option<Vec<String>>,
    pub feedback_channels: Option<Vec<String>>,
}

impl_identified_header!(ServiceRequirement);

impl_patchable!(
    ServiceRequirement,
    ServiceRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [service_name] => service_name,
        [service_type] => service_type,
        [provider] => provider,
        [service_level] => service_level,
        [operating_hours] => operating_hours,
        [capacity] => capacity,
        [response_time] => response_time,
        [queue_management] => queue_management,
        [customer_profiles] => customer_profiles,
        [element_ids] => element_ids,
        [equipment_ids] => equipment_ids,
        [staffing] => staffing,
        [quality_metrics] => quality_metrics,
        [cost_model] => cost_model,
        [contract_refs] => contract_refs,
        [dependencies] => dependencies,
        [failure_impact] => failure_impact,
        [backup_service] => backup_service,
        [feedback_channels] => feedback_channels,
    }
);
// #endregion

// #region 🔖️InfrastructureRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct InfrastructureRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub system: String,
    pub category: String,
    pub capacity: QuantitySpec,
    pub redundancy: Option<String>,
    pub distribution: Vec<String>,
    pub entry_points: Vec<String>,
    pub utility_source: Option<String>,
    pub standby_power: bool,
    pub monitoring: Vec<String>,
    pub maintenance_access: Vec<String>,
    pub standards: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub peak_demand: Option<f64>,
    pub diversity_factor: Option<f64>,
    pub future_expansion: Vec<String>,
    pub interface_requirements: Vec<String>,
    pub commissioning: Vec<String>,
    pub lifecycle_cost: Option<f64>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct InfrastructureRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub system: Option<String>,
    pub category: Option<String>,
    pub capacity: Option<QuantitySpec>,
    pub redundancy: Option<String>,
    pub distribution: Option<Vec<String>>,
    pub entry_points: Option<Vec<String>>,
    pub utility_source: Option<String>,
    pub standby_power: Option<bool>,
    pub monitoring: Option<Vec<String>>,
    pub maintenance_access: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub peak_demand: Option<f64>,
    pub diversity_factor: Option<f64>,
    pub future_expansion: Option<Vec<String>>,
    pub interface_requirements: Option<Vec<String>>,
    pub commissioning: Option<Vec<String>>,
    pub lifecycle_cost: Option<f64>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(InfrastructureRequirement);

impl_patchable!(
    InfrastructureRequirement,
    InfrastructureRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [system] => system,
        [category] => category,
        [capacity] => capacity,
        [redundancy] => redundancy,
        [distribution] => distribution,
        [entry_points] => entry_points,
        [utility_source] => utility_source,
        [standby_power] => standby_power,
        [monitoring] => monitoring,
        [maintenance_access] => maintenance_access,
        [standards] => standards,
        [element_ids] => element_ids,
        [peak_demand] => peak_demand,
        [diversity_factor] => diversity_factor,
        [future_expansion] => future_expansion,
        [interface_requirements] => interface_requirements,
        [commissioning] => commissioning,
        [lifecycle_cost] => lifecycle_cost,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️InformationRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct InformationRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub information_type: String,
    pub format: Option<String>,
    pub source_system: Option<String>,
    pub destination_systems: Vec<String>,
    pub update_frequency: Option<String>,
    pub retention_period: Option<String>,
    pub access_controls: Vec<String>,
    pub classification: Option<String>,
    pub quality_criteria: Vec<String>,
    pub metadata_requirements: Vec<String>,
    pub integration_points: Vec<String>,
    pub backup_requirements: Vec<String>,
    pub disaster_recovery: Vec<String>,
    pub privacy_controls: Vec<String>,
    pub audit_trail: bool,
    pub element_ids: Vec<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub standards: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct InformationRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub information_type: Option<String>,
    pub format: Option<String>,
    pub source_system: Option<String>,
    pub destination_systems: Option<Vec<String>>,
    pub update_frequency: Option<String>,
    pub retention_period: Option<String>,
    pub access_controls: Option<Vec<String>>,
    pub classification: Option<String>,
    pub quality_criteria: Option<Vec<String>>,
    pub metadata_requirements: Option<Vec<String>>,
    pub integration_points: Option<Vec<String>>,
    pub backup_requirements: Option<Vec<String>>,
    pub disaster_recovery: Option<Vec<String>>,
    pub privacy_controls: Option<Vec<String>>,
    pub audit_trail: Option<bool>,
    pub element_ids: Option<Vec<EntityId>>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub standards: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(InformationRequirement);

impl_patchable!(
    InformationRequirement,
    InformationRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [information_type] => information_type,
        [format] => format,
        [source_system] => source_system,
        [destination_systems] => destination_systems,
        [update_frequency] => update_frequency,
        [retention_period] => retention_period,
        [access_controls] => access_controls,
        [classification] => classification,
        [quality_criteria] => quality_criteria,
        [metadata_requirements] => metadata_requirements,
        [integration_points] => integration_points,
        [backup_requirements] => backup_requirements,
        [disaster_recovery] => disaster_recovery,
        [privacy_controls] => privacy_controls,
        [audit_trail] => audit_trail,
        [element_ids] => element_ids,
        [stakeholder_ids] => stakeholder_ids,
        [standards] => standards,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️CommunicationRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CommunicationRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub channel: String,
    pub audience_ids: Vec<EntityId>,
    pub message_types: Vec<String>,
    pub frequency: Option<String>,
    pub medium: Vec<String>,
    pub language: Vec<String>,
    pub accessibility: Vec<String>,
    pub emergency_use: bool,
    pub two_way: bool,
    pub recording_policy: Option<String>,
    pub signage_locations: Vec<String>,
    pub technology: Vec<String>,
    pub escalation_path: Vec<String>,
    pub feedback_loop: bool,
    pub privacy_controls: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub standards: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub templates: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CommunicationRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub channel: Option<String>,
    pub audience_ids: Option<Vec<EntityId>>,
    pub message_types: Option<Vec<String>>,
    pub frequency: Option<String>,
    pub medium: Option<Vec<String>>,
    pub language: Option<Vec<String>>,
    pub accessibility: Option<Vec<String>>,
    pub emergency_use: Option<bool>,
    pub two_way: Option<bool>,
    pub recording_policy: Option<String>,
    pub signage_locations: Option<Vec<String>>,
    pub technology: Option<Vec<String>>,
    pub escalation_path: Option<Vec<String>>,
    pub feedback_loop: Option<bool>,
    pub privacy_controls: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub standards: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub templates: Option<Vec<EntityId>>,
}

impl_identified_header!(CommunicationRequirement);

impl_patchable!(
    CommunicationRequirement,
    CommunicationRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [channel] => channel,
        [audience_ids] => audience_ids,
        [message_types] => message_types,
        [frequency] => frequency,
        [medium] => medium,
        [language] => language,
        [accessibility] => accessibility,
        [emergency_use] => emergency_use,
        [two_way] => two_way,
        [recording_policy] => recording_policy,
        [signage_locations] => signage_locations,
        [technology] => technology,
        [escalation_path] => escalation_path,
        [feedback_loop] => feedback_loop,
        [privacy_controls] => privacy_controls,
        [element_ids] => element_ids,
        [standards] => standards,
        [owner_id] => owner_id,
        [templates] => templates,
    }
);
// #endregion

// #region 🔖️WayfindingRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct WayfindingRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub user_profile_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub destination_types: Vec<String>,
    pub signage_types: Vec<String>,
    pub languages: Vec<String>,
    pub tactile_required: bool,
    pub audio_required: bool,
    pub digital_wayfinding: bool,
    pub landmark_strategy: Vec<String>,
    pub color_coding: Vec<String>,
    pub symbol_standards: Vec<String>,
    pub decision_points: Vec<String>,
    #[dsl(unit = "m")]
    pub maximum_signage_distance_m: Option<f64>,
    pub lighting_requirements: Vec<String>,
    pub maintenance_plan: Option<String>,
    pub emergency_egress: Vec<String>,
    pub visitor_journey: Vec<String>,
    pub staff_journey: Vec<String>,
    pub brand_integration: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct WayfindingRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub destination_types: Option<Vec<String>>,
    pub signage_types: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub tactile_required: Option<bool>,
    pub audio_required: Option<bool>,
    pub digital_wayfinding: Option<bool>,
    pub landmark_strategy: Option<Vec<String>>,
    pub color_coding: Option<Vec<String>>,
    pub symbol_standards: Option<Vec<String>>,
    pub decision_points: Option<Vec<String>>,
    pub maximum_signage_distance_m: Option<f64>,
    pub lighting_requirements: Option<Vec<String>>,
    pub maintenance_plan: Option<String>,
    pub emergency_egress: Option<Vec<String>>,
    pub visitor_journey: Option<Vec<String>>,
    pub staff_journey: Option<Vec<String>>,
    pub brand_integration: Option<Vec<String>>,
}

impl_identified_header!(WayfindingRequirement);

impl_patchable!(
    WayfindingRequirement,
    WayfindingRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [user_profile_ids] => user_profile_ids,
        [element_ids] => element_ids,
        [destination_types] => destination_types,
        [signage_types] => signage_types,
        [languages] => languages,
        [tactile_required] => tactile_required,
        [audio_required] => audio_required,
        [digital_wayfinding] => digital_wayfinding,
        [landmark_strategy] => landmark_strategy,
        [color_coding] => color_coding,
        [symbol_standards] => symbol_standards,
        [decision_points] => decision_points,
        [maximum_signage_distance_m] => maximum_signage_distance_m,
        [lighting_requirements] => lighting_requirements,
        [maintenance_plan] => maintenance_plan,
        [emergency_egress] => emergency_egress,
        [visitor_journey] => visitor_journey,
        [staff_journey] => staff_journey,
        [brand_integration] => brand_integration,
    }
);
// #endregion

// #region 🔖️ScheduleRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ScheduleRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub milestone: String,
    pub phase: DeliveryPhase,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub duration: Option<String>,
    pub dependencies: Vec<EntityId>,
    pub predecessors: Vec<EntityId>,
    pub successors: Vec<EntityId>,
    pub critical: bool,
    pub float_days: Option<u32>,
    pub resource_requirements: Vec<String>,
    pub occupancy_impact: Vec<String>,
    pub phasing_strategy: Option<String>,
    pub decant_requirements: Vec<String>,
    pub commissioning_window: Option<String>,
    pub stakeholder_ids: Vec<EntityId>,
    pub risk_ids: Vec<EntityId>,
    pub contingency_days: Option<u32>,
    pub reporting_cadence: Option<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ScheduleRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub milestone: Option<String>,
    pub phase: Option<DeliveryPhase>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub duration: Option<String>,
    pub dependencies: Option<Vec<EntityId>>,
    pub predecessors: Option<Vec<EntityId>>,
    pub successors: Option<Vec<EntityId>>,
    pub critical: Option<bool>,
    pub float_days: Option<u32>,
    pub resource_requirements: Option<Vec<String>>,
    pub occupancy_impact: Option<Vec<String>>,
    pub phasing_strategy: Option<String>,
    pub decant_requirements: Option<Vec<String>>,
    pub commissioning_window: Option<String>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub risk_ids: Option<Vec<EntityId>>,
    pub contingency_days: Option<u32>,
    pub reporting_cadence: Option<String>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(ScheduleRequirement);

impl_patchable!(
    ScheduleRequirement,
    ScheduleRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [milestone] => milestone,
        [phase] => phase,
        [start_date] => start_date,
        [end_date] => end_date,
        [duration] => duration,
        [dependencies] => dependencies,
        [predecessors] => predecessors,
        [successors] => successors,
        [critical] => critical,
        [float_days] => float_days,
        [resource_requirements] => resource_requirements,
        [occupancy_impact] => occupancy_impact,
        [phasing_strategy] => phasing_strategy,
        [decant_requirements] => decant_requirements,
        [commissioning_window] => commissioning_window,
        [stakeholder_ids] => stakeholder_ids,
        [risk_ids] => risk_ids,
        [contingency_days] => contingency_days,
        [reporting_cadence] => reporting_cadence,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️FlexibilityRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FlexibilityRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub flexibility_type: String,
    pub element_ids: Vec<EntityId>,
    pub adaptation_scenarios: Vec<String>,
    pub modularity_level: Option<String>,
    pub reconfiguration_time: Option<String>,
    pub cost_of_change: Option<f64>,
    pub technology_readiness: Option<String>,
    pub future_function_ids: Vec<EntityId>,
    pub demountable_partitions: bool,
    pub raised_floor: bool,
    pub overhead_services: bool,
    pub expansion_direction: Vec<String>,
    pub contraction_scenario: Vec<String>,
    pub multi_use_potential: Vec<String>,
    pub furniture_strategy: Vec<String>,
    pub infrastructure_spare_capacity: Vec<String>,
    pub lease_implications: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FlexibilityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub flexibility_type: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub adaptation_scenarios: Option<Vec<String>>,
    pub modularity_level: Option<String>,
    pub reconfiguration_time: Option<String>,
    pub cost_of_change: Option<f64>,
    pub technology_readiness: Option<String>,
    pub future_function_ids: Option<Vec<EntityId>>,
    pub demountable_partitions: Option<bool>,
    pub raised_floor: Option<bool>,
    pub overhead_services: Option<bool>,
    pub expansion_direction: Option<Vec<String>>,
    pub contraction_scenario: Option<Vec<String>>,
    pub multi_use_potential: Option<Vec<String>>,
    pub furniture_strategy: Option<Vec<String>>,
    pub infrastructure_spare_capacity: Option<Vec<String>>,
    pub lease_implications: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(FlexibilityRequirement);

impl_patchable!(
    FlexibilityRequirement,
    FlexibilityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [flexibility_type] => flexibility_type,
        [element_ids] => element_ids,
        [adaptation_scenarios] => adaptation_scenarios,
        [modularity_level] => modularity_level,
        [reconfiguration_time] => reconfiguration_time,
        [cost_of_change] => cost_of_change,
        [technology_readiness] => technology_readiness,
        [future_function_ids] => future_function_ids,
        [demountable_partitions] => demountable_partitions,
        [raised_floor] => raised_floor,
        [overhead_services] => overhead_services,
        [expansion_direction] => expansion_direction,
        [contraction_scenario] => contraction_scenario,
        [multi_use_potential] => multi_use_potential,
        [furniture_strategy] => furniture_strategy,
        [infrastructure_spare_capacity] => infrastructure_spare_capacity,
        [lease_implications] => lease_implications,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️GrowthPlan
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct GrowthPlan {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub horizon_years: u32,
    pub growth_rate: Option<f64>,
    pub headcount_growth: QuantitySpec,
    pub area_growth: QuantitySpec,
    pub phases: Vec<String>,
    pub trigger_events: Vec<String>,
    pub expansion_element_ids: Vec<EntityId>,
    pub reserve_areas: Vec<String>,
    pub infrastructure_headroom: Vec<String>,
    pub budget_envelope: Option<f64>,
    pub funding_sources: Vec<String>,
    pub risk_factors: Vec<EntityId>,
    pub decision_points: Vec<EntityId>,
    pub scenario_ids: Vec<EntityId>,
    pub decommission_plan: Vec<String>,
    pub relocation_strategy: Vec<String>,
    pub stakeholder_impact: Vec<String>,
    pub regulatory_considerations: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct GrowthPlanPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub horizon_years: Option<u32>,
    pub growth_rate: Option<f64>,
    pub headcount_growth: Option<QuantitySpec>,
    pub area_growth: Option<QuantitySpec>,
    pub phases: Option<Vec<String>>,
    pub trigger_events: Option<Vec<String>>,
    pub expansion_element_ids: Option<Vec<EntityId>>,
    pub reserve_areas: Option<Vec<String>>,
    pub infrastructure_headroom: Option<Vec<String>>,
    pub budget_envelope: Option<f64>,
    pub funding_sources: Option<Vec<String>>,
    pub risk_factors: Option<Vec<EntityId>>,
    pub decision_points: Option<Vec<EntityId>>,
    pub scenario_ids: Option<Vec<EntityId>>,
    pub decommission_plan: Option<Vec<String>>,
    pub relocation_strategy: Option<Vec<String>>,
    pub stakeholder_impact: Option<Vec<String>>,
    pub regulatory_considerations: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(GrowthPlan);

impl_patchable!(
    GrowthPlan,
    GrowthPlanPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [horizon_years] => horizon_years,
        [growth_rate] => growth_rate,
        [headcount_growth] => headcount_growth,
        [area_growth] => area_growth,
        [phases] => phases,
        [trigger_events] => trigger_events,
        [expansion_element_ids] => expansion_element_ids,
        [reserve_areas] => reserve_areas,
        [infrastructure_headroom] => infrastructure_headroom,
        [budget_envelope] => budget_envelope,
        [funding_sources] => funding_sources,
        [risk_factors] => risk_factors,
        [decision_points] => decision_points,
        [scenario_ids] => scenario_ids,
        [decommission_plan] => decommission_plan,
        [relocation_strategy] => relocation_strategy,
        [stakeholder_impact] => stakeholder_impact,
        [regulatory_considerations] => regulatory_considerations,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️SustainabilityRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SustainabilityRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub topic: String,
    pub target: Option<String>,
    pub metric: Option<String>,
    pub baseline: Option<f64>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub certification: Vec<String>,
    pub standards: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub strategies: Vec<String>,
    pub materials_preferences: Vec<String>,
    pub energy_strategy: Vec<String>,
    pub water_strategy: Vec<String>,
    pub waste_strategy: Vec<String>,
    pub biodiversity: Vec<String>,
    pub embodied_carbon: Option<f64>,
    pub operational_carbon: Option<f64>,
    pub reporting_requirements: Vec<String>,
    pub verification_plan: Option<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SustainabilityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub topic: Option<String>,
    pub target: Option<String>,
    pub metric: Option<String>,
    pub baseline: Option<f64>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub certification: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub strategies: Option<Vec<String>>,
    pub materials_preferences: Option<Vec<String>>,
    pub energy_strategy: Option<Vec<String>>,
    pub water_strategy: Option<Vec<String>>,
    pub waste_strategy: Option<Vec<String>>,
    pub biodiversity: Option<Vec<String>>,
    pub embodied_carbon: Option<f64>,
    pub operational_carbon: Option<f64>,
    pub reporting_requirements: Option<Vec<String>>,
    pub verification_plan: Option<String>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(SustainabilityRequirement);

impl_patchable!(
    SustainabilityRequirement,
    SustainabilityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [topic] => topic,
        [target] => target,
        [metric] => metric,
        [baseline] => baseline,
        [target_value] => target_value,
        [unit] => unit,
        [certification] => certification,
        [standards] => standards,
        [element_ids] => element_ids,
        [strategies] => strategies,
        [materials_preferences] => materials_preferences,
        [energy_strategy] => energy_strategy,
        [water_strategy] => water_strategy,
        [waste_strategy] => waste_strategy,
        [biodiversity] => biodiversity,
        [embodied_carbon] => embodied_carbon,
        [operational_carbon] => operational_carbon,
        [reporting_requirements] => reporting_requirements,
        [verification_plan] => verification_plan,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️ResilienceRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ResilienceRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub hazard: String,
    pub risk_level: RiskLevel,
    pub scenario: Option<String>,
    pub recovery_time: Option<String>,
    pub recovery_point: Option<String>,
    pub redundancy: Vec<String>,
    pub hardening_measures: Vec<String>,
    pub backup_systems: Vec<String>,
    pub alternate_sites: Vec<String>,
    pub supply_chain: Vec<String>,
    pub communication_plan: Vec<String>,
    pub drill_requirements: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub infrastructure_ids: Vec<EntityId>,
    pub standards: Vec<String>,
    pub insurance_implications: Vec<String>,
    pub climate_adaptation: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub verification_plan: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ResilienceRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub hazard: Option<String>,
    pub risk_level: Option<RiskLevel>,
    pub scenario: Option<String>,
    pub recovery_time: Option<String>,
    pub recovery_point: Option<String>,
    pub redundancy: Option<Vec<String>>,
    pub hardening_measures: Option<Vec<String>>,
    pub backup_systems: Option<Vec<String>>,
    pub alternate_sites: Option<Vec<String>>,
    pub supply_chain: Option<Vec<String>>,
    pub communication_plan: Option<Vec<String>>,
    pub drill_requirements: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub infrastructure_ids: Option<Vec<EntityId>>,
    pub standards: Option<Vec<String>>,
    pub insurance_implications: Option<Vec<String>>,
    pub climate_adaptation: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub verification_plan: Option<String>,
}

impl_identified_header!(ResilienceRequirement);

impl_patchable!(
    ResilienceRequirement,
    ResilienceRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [hazard] => hazard,
        [risk_level] => risk_level,
        [scenario] => scenario,
        [recovery_time] => recovery_time,
        [recovery_point] => recovery_point,
        [redundancy] => redundancy,
        [hardening_measures] => hardening_measures,
        [backup_systems] => backup_systems,
        [alternate_sites] => alternate_sites,
        [supply_chain] => supply_chain,
        [communication_plan] => communication_plan,
        [drill_requirements] => drill_requirements,
        [element_ids] => element_ids,
        [infrastructure_ids] => infrastructure_ids,
        [standards] => standards,
        [insurance_implications] => insurance_implications,
        [climate_adaptation] => climate_adaptation,
        [owner_id] => owner_id,
        [verification_plan] => verification_plan,
    }
);
// #endregion

// #region 🔖️CostRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CostRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub cost_item: String,
    pub basis: CostBasis,
    pub amount: Option<f64>,
    pub currency: String,
    pub quantity_basis: Option<String>,
    pub unit_cost: Option<f64>,
    #[dsl(unit = "pct")]
    pub contingency_percent: Option<f64>,
    pub escalation_rate: Option<f64>,
    pub funding_source: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub phase: Option<DeliveryPhase>,
    pub cash_flow_profile: Vec<String>,
    pub value_engineering_notes: Vec<String>,
    pub benchmark_ref: Option<EntityId>,
    pub approval_status: ValidationStatus,
    pub owner_id: Option<EntityId>,
    pub assumptions: Vec<String>,
    pub sensitivity_factors: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CostRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub cost_item: Option<String>,
    pub basis: Option<CostBasis>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub quantity_basis: Option<String>,
    pub unit_cost: Option<f64>,
    pub contingency_percent: Option<f64>,
    pub escalation_rate: Option<f64>,
    pub funding_source: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub phase: Option<DeliveryPhase>,
    pub cash_flow_profile: Option<Vec<String>>,
    pub value_engineering_notes: Option<Vec<String>>,
    pub benchmark_ref: Option<EntityId>,
    pub approval_status: Option<ValidationStatus>,
    pub owner_id: Option<EntityId>,
    pub assumptions: Option<Vec<String>>,
    pub sensitivity_factors: Option<Vec<String>>,
}

impl_identified_header!(CostRequirement);

impl_patchable!(
    CostRequirement,
    CostRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [cost_item] => cost_item,
        [basis] => basis,
        [amount] => amount,
        [currency] => currency,
        [quantity_basis] => quantity_basis,
        [unit_cost] => unit_cost,
        [contingency_percent] => contingency_percent,
        [escalation_rate] => escalation_rate,
        [funding_source] => funding_source,
        [element_ids] => element_ids,
        [requirement_ids] => requirement_ids,
        [phase] => phase,
        [cash_flow_profile] => cash_flow_profile,
        [value_engineering_notes] => value_engineering_notes,
        [benchmark_ref] => benchmark_ref,
        [approval_status] => approval_status,
        [owner_id] => owner_id,
        [assumptions] => assumptions,
        [sensitivity_factors] => sensitivity_factors,
    }
);
// #endregion

// #region 🔖️DeliveryConstraint
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DeliveryConstraint {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub constraint_type: String,
    pub constraint_details: TextField,
    pub phase: DeliveryPhase,
    pub hard_deadline: Option<String>,
    pub soft_deadline: Option<String>,
    pub impacted_element_ids: Vec<EntityId>,
    pub impacted_requirement_ids: Vec<EntityId>,
    pub work_hours: Option<String>,
    pub noise_restrictions: Vec<String>,
    pub access_restrictions: Vec<String>,
    pub site_logistics: Vec<String>,
    pub procurement_lead_time: Option<String>,
    pub approval_gates: Vec<String>,
    pub occupancy_constraints: Vec<String>,
    pub weather_windows: Vec<String>,
    pub penalty_clauses: Vec<String>,
    pub mitigation_options: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub risk_ids: Vec<EntityId>,
    pub constraint_status: LifecycleStatus,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DeliveryConstraintPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub constraint_type: Option<String>,
    pub constraint_details: Option<TextField>,
    pub phase: Option<DeliveryPhase>,
    pub hard_deadline: Option<String>,
    pub soft_deadline: Option<String>,
    pub impacted_element_ids: Option<Vec<EntityId>>,
    pub impacted_requirement_ids: Option<Vec<EntityId>>,
    pub work_hours: Option<String>,
    pub noise_restrictions: Option<Vec<String>>,
    pub access_restrictions: Option<Vec<String>>,
    pub site_logistics: Option<Vec<String>>,
    pub procurement_lead_time: Option<String>,
    pub approval_gates: Option<Vec<String>>,
    pub occupancy_constraints: Option<Vec<String>>,
    pub weather_windows: Option<Vec<String>>,
    pub penalty_clauses: Option<Vec<String>>,
    pub mitigation_options: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub risk_ids: Option<Vec<EntityId>>,
    pub constraint_status: Option<LifecycleStatus>,
}

impl_identified_header!(DeliveryConstraint);

impl_patchable!(
    DeliveryConstraint,
    DeliveryConstraintPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [constraint_type] => constraint_type,
        [constraint_details] => constraint_details,
        [phase] => phase,
        [hard_deadline] => hard_deadline,
        [soft_deadline] => soft_deadline,
        [impacted_element_ids] => impacted_element_ids,
        [impacted_requirement_ids] => impacted_requirement_ids,
        [work_hours] => work_hours,
        [noise_restrictions] => noise_restrictions,
        [access_restrictions] => access_restrictions,
        [site_logistics] => site_logistics,
        [procurement_lead_time] => procurement_lead_time,
        [approval_gates] => approval_gates,
        [occupancy_constraints] => occupancy_constraints,
        [weather_windows] => weather_windows,
        [penalty_clauses] => penalty_clauses,
        [mitigation_options] => mitigation_options,
        [owner_id] => owner_id,
        [risk_ids] => risk_ids,
        [constraint_status] => constraint_status,
    }
);
// #endregion

// #region 🔖️Risk
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Risk {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub risk_statement: TextField,
    pub category: String,
    pub probability: RiskLevel,
    pub impact: RiskLevel,
    pub risk_score: Option<f64>,
    pub causes: Vec<String>,
    pub effects: Vec<String>,
    pub affected_element_ids: Vec<EntityId>,
    pub affected_requirement_ids: Vec<EntityId>,
    pub mitigation: Vec<String>,
    pub contingency: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub review_date: Option<String>,
    pub trigger_indicators: Vec<String>,
    pub residual_probability: Option<RiskLevel>,
    pub residual_impact: Option<RiskLevel>,
    pub related_conflict_ids: Vec<EntityId>,
    pub escalation_path: Vec<String>,
    pub monitoring_plan: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct RiskPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub risk_statement: Option<TextField>,
    pub category: Option<String>,
    pub probability: Option<RiskLevel>,
    pub impact: Option<RiskLevel>,
    pub risk_score: Option<f64>,
    pub causes: Option<Vec<String>>,
    pub effects: Option<Vec<String>>,
    pub affected_element_ids: Option<Vec<EntityId>>,
    pub affected_requirement_ids: Option<Vec<EntityId>>,
    pub mitigation: Option<Vec<String>>,
    pub contingency: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub review_date: Option<String>,
    pub trigger_indicators: Option<Vec<String>>,
    pub residual_probability: Option<RiskLevel>,
    pub residual_impact: Option<RiskLevel>,
    pub related_conflict_ids: Option<Vec<EntityId>>,
    pub escalation_path: Option<Vec<String>>,
    pub monitoring_plan: Option<String>,
}

impl_identified_header!(Risk);

impl_patchable!(
    Risk,
    RiskPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [risk_statement] => risk_statement,
        [category] => category,
        [probability] => probability,
        [impact] => impact,
        [risk_score] => risk_score,
        [causes] => causes,
        [effects] => effects,
        [affected_element_ids] => affected_element_ids,
        [affected_requirement_ids] => affected_requirement_ids,
        [mitigation] => mitigation,
        [contingency] => contingency,
        [owner_id] => owner_id,
        [review_date] => review_date,
        [trigger_indicators] => trigger_indicators,
        [residual_probability] => residual_probability,
        [residual_impact] => residual_impact,
        [related_conflict_ids] => related_conflict_ids,
        [escalation_path] => escalation_path,
        [monitoring_plan] => monitoring_plan,
    }
);
// #endregion

// #region 🔖️Conflict
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Conflict {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub kind: ConflictKind,
    pub summary: TextField,
    pub entity_a_id: EntityId,
    pub entity_b_id: EntityId,
    pub severity: IssueSeverity,
    pub detected_by: Option<String>,
    pub detection_date: Option<String>,
    pub trade_off_options: Vec<String>,
    pub recommended_resolution: Option<TextField>,
    pub decision_id: Option<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub quality_impact: Vec<String>,
    pub resolution_status: ValidationStatus,
    pub owner_id: Option<EntityId>,
    pub escalation_level: Option<String>,
    pub related_risk_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ConflictPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub kind: Option<ConflictKind>,
    pub summary: Option<TextField>,
    pub entity_a_id: Option<EntityId>,
    pub entity_b_id: Option<EntityId>,
    pub severity: Option<IssueSeverity>,
    pub detected_by: Option<String>,
    pub detection_date: Option<String>,
    pub trade_off_options: Option<Vec<String>>,
    pub recommended_resolution: Option<TextField>,
    pub decision_id: Option<EntityId>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub quality_impact: Option<Vec<String>>,
    pub resolution_status: Option<ValidationStatus>,
    pub owner_id: Option<EntityId>,
    pub escalation_level: Option<String>,
    pub related_risk_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(Conflict);

impl_patchable!(
    Conflict,
    ConflictPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [kind] => kind,
        [summary] => summary,
        [entity_a_id] => entity_a_id,
        [entity_b_id] => entity_b_id,
        [severity] => severity,
        [detected_by] => detected_by,
        [detection_date] => detection_date,
        [trade_off_options] => trade_off_options,
        [recommended_resolution] => recommended_resolution,
        [decision_id] => decision_id,
        [stakeholder_ids] => stakeholder_ids,
        [requirement_ids] => requirement_ids,
        [cost_impact] => cost_impact,
        [schedule_impact] => schedule_impact,
        [quality_impact] => quality_impact,
        [resolution_status] => resolution_status,
        [owner_id] => owner_id,
        [escalation_level] => escalation_level,
        [related_risk_ids] => related_risk_ids,
    }
);
// #endregion

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
