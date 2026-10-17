//! 🏢️ Site context and the organizational, service, infrastructure, information, communication and wayfinding requirements.

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
