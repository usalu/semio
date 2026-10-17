//! 🧱️ Functions, program elements, quantity requirements, relationships and adjacencies.

use super::*;

// #region 🔖️Function
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Function {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub code: String,
    pub kind: FunctionKind,
    pub purpose: TextField,
    pub criticality: Priority,
    pub performance_targets: Vec<String>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub staffing: QuantitySpec,
    pub equipment_ids: Vec<EntityId>,
    pub resource_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub dependencies: Vec<EntityId>,
    pub interfaces: Vec<String>,
    pub constraints: Vec<String>,
    pub quality_criteria: Vec<String>,
    pub regulatory_refs: Vec<String>,
    pub future_changes: Vec<String>,
    pub owner_stakeholder_id: Option<EntityId>,
    pub success_metrics: Vec<String>,
    pub hierarchy_parent_id: Option<EntityId>,
    pub conflict_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FunctionPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub kind: Option<FunctionKind>,
    pub purpose: Option<TextField>,
    pub criticality: Option<Priority>,
    pub performance_targets: Option<Vec<String>>,
    pub service_level: Option<String>,
    pub operating_hours: Option<String>,
    pub staffing: Option<QuantitySpec>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub resource_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub dependencies: Option<Vec<EntityId>>,
    pub interfaces: Option<Vec<String>>,
    pub constraints: Option<Vec<String>>,
    pub quality_criteria: Option<Vec<String>>,
    pub regulatory_refs: Option<Vec<String>>,
    pub future_changes: Option<Vec<String>>,
    pub owner_stakeholder_id: Option<EntityId>,
    pub success_metrics: Option<Vec<String>>,
    pub hierarchy_parent_id: Option<EntityId>,
    pub conflict_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(Function);

impl_patchable!(
    Function,
    FunctionPatch,
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
        [kind] => kind,
        [purpose] => purpose,
        [criticality] => criticality,
        [performance_targets] => performance_targets,
        [service_level] => service_level,
        [operating_hours] => operating_hours,
        [staffing] => staffing,
        [equipment_ids] => equipment_ids,
        [resource_ids] => resource_ids,
        [activity_ids] => activity_ids,
        [element_ids] => element_ids,
        [dependencies] => dependencies,
        [interfaces] => interfaces,
        [constraints] => constraints,
        [quality_criteria] => quality_criteria,
        [regulatory_refs] => regulatory_refs,
        [future_changes] => future_changes,
        [owner_stakeholder_id] => owner_stakeholder_id,
        [success_metrics] => success_metrics,
        [hierarchy_parent_id] => hierarchy_parent_id,
        [conflict_ids] => conflict_ids,
    }
);
// #endregion

// #region 🔖️ProgramElement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramElement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub code: String,
    pub kind: ProgramElementKind,
    pub parent_id: Option<EntityId>,
    pub level: Option<String>,
    pub area: QuantitySpec,
    pub volume: QuantitySpec,
    pub height: QuantitySpec,
    pub occupancy: QuantitySpec,
    pub function_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub user_profile_ids: Vec<EntityId>,
    pub adjacency_ids: Vec<EntityId>,
    pub quantity_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub location_hint: Option<String>,
    pub orientation: Option<String>,
    pub daylight_requirement: Option<String>,
    pub acoustic_class: Option<String>,
    pub security_zone: Option<String>,
    pub flexibility_notes: Vec<String>,
    pub growth_allocation: Option<String>,
    pub circulation_role: Option<String>,
    pub visibility_level: Option<String>,
    pub adjacency_preferences: Vec<EntityId>,
    pub environmental_zone: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramElementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub kind: Option<ProgramElementKind>,
    pub parent_id: Option<EntityId>,
    pub level: Option<String>,
    pub area: Option<QuantitySpec>,
    pub volume: Option<QuantitySpec>,
    pub height: Option<QuantitySpec>,
    pub occupancy: Option<QuantitySpec>,
    pub function_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub adjacency_ids: Option<Vec<EntityId>>,
    pub quantity_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub location_hint: Option<String>,
    pub orientation: Option<String>,
    pub daylight_requirement: Option<String>,
    pub acoustic_class: Option<String>,
    pub security_zone: Option<String>,
    pub flexibility_notes: Option<Vec<String>>,
    pub growth_allocation: Option<String>,
    pub circulation_role: Option<String>,
    pub visibility_level: Option<String>,
    pub adjacency_preferences: Option<Vec<EntityId>>,
    pub environmental_zone: Option<String>,
}

impl_identified_header!(ProgramElement);

impl_patchable!(
    ProgramElement,
    ProgramElementPatch,
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
        [kind] => kind,
        [parent_id] => parent_id,
        [level] => level,
        [area] => area,
        [volume] => volume,
        [height] => height,
        [occupancy] => occupancy,
        [function_ids] => function_ids,
        [activity_ids] => activity_ids,
        [user_profile_ids] => user_profile_ids,
        [adjacency_ids] => adjacency_ids,
        [quantity_ids] => quantity_ids,
        [requirement_ids] => requirement_ids,
        [location_hint] => location_hint,
        [orientation] => orientation,
        [daylight_requirement] => daylight_requirement,
        [acoustic_class] => acoustic_class,
        [security_zone] => security_zone,
        [flexibility_notes] => flexibility_notes,
        [growth_allocation] => growth_allocation,
        [circulation_role] => circulation_role,
        [visibility_level] => visibility_level,
        [adjacency_preferences] => adjacency_preferences,
        [environmental_zone] => environmental_zone,
    }
);
// #endregion

// #region 🔖️QuantityRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct QuantityRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub target_element_id: EntityId,
    pub metric: String,
    pub quantity: QuantitySpec,
    pub basis: Option<String>,
    pub calculation_method: Option<String>,
    pub source: Option<String>,
    pub benchmark_ref: Option<EntityId>,
    #[dsl(unit = "pct")]
    pub tolerance_percent: Option<f64>,
    pub peak_factor: Option<f64>,
    pub growth_factor: Option<f64>,
    pub unit_cost: Option<f64>,
    pub currency: Option<String>,
    pub verification_method: Option<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub assumptions: Vec<String>,
    pub constraints: Vec<String>,
    pub schedule_phase: Option<String>,
    pub responsible_party: Option<EntityId>,
    pub last_verified: Option<String>,
    pub variance_notes: Vec<TaggedNote>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct QuantityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub target_element_id: Option<EntityId>,
    pub metric: Option<String>,
    pub quantity: Option<QuantitySpec>,
    pub basis: Option<String>,
    pub calculation_method: Option<String>,
    pub source: Option<String>,
    pub benchmark_ref: Option<EntityId>,
    pub tolerance_percent: Option<f64>,
    pub peak_factor: Option<f64>,
    pub growth_factor: Option<f64>,
    pub unit_cost: Option<f64>,
    pub currency: Option<String>,
    pub verification_method: Option<String>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub assumptions: Option<Vec<String>>,
    pub constraints: Option<Vec<String>>,
    pub schedule_phase: Option<String>,
    pub responsible_party: Option<EntityId>,
    pub last_verified: Option<String>,
    pub variance_notes: Option<Vec<TaggedNote>>,
}

impl_identified_header!(QuantityRequirement);

impl_patchable!(
    QuantityRequirement,
    QuantityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [target_element_id] => target_element_id,
        [metric] => metric,
        [quantity] => quantity,
        [basis] => basis,
        [calculation_method] => calculation_method,
        [source] => source,
        [benchmark_ref] => benchmark_ref,
        [tolerance_percent] => tolerance_percent,
        [peak_factor] => peak_factor,
        [growth_factor] => growth_factor,
        [unit_cost] => unit_cost,
        [currency] => currency,
        [verification_method] => verification_method,
        [related_requirement_ids] => related_requirement_ids,
        [assumptions] => assumptions,
        [constraints] => constraints,
        [schedule_phase] => schedule_phase,
        [responsible_party] => responsible_party,
        [last_verified] => last_verified,
        [variance_notes] => variance_notes,
    }
);
// #endregion

// #region 🔖️Relationship
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Relationship {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub source_id: EntityId,
    pub target_id: EntityId,
    pub kind: RelationshipKind,
    pub strength: Option<f64>,
    pub directional: bool,
    pub rationale: Option<TextField>,
    pub constraints: Vec<String>,
    pub conditions: Vec<String>,
    pub relationship_priority: Priority,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub evidence: Vec<String>,
    pub conflict_ids: Vec<EntityId>,
    pub trace_links: Vec<TraceLink>,
    pub bidirectional: bool,
    #[dsl(unit = "m")]
    pub distance_constraint_m: Option<f64>,
    pub capacity_constraint: Option<String>,
    pub regulatory_basis: Vec<String>,
    pub review_cycle: Option<String>,
    pub owner_id: Option<EntityId>,
    pub proximity_requirement: Option<TextField>,
    pub compatibility_requirement: Option<TextField>,
    pub incompatibility_requirement: Option<TextField>,
    pub separation_requirements: Vec<SeparationKind>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct RelationshipPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub source_id: Option<EntityId>,
    pub target_id: Option<EntityId>,
    pub kind: Option<RelationshipKind>,
    pub strength: Option<f64>,
    pub directional: Option<bool>,
    pub rationale: Option<TextField>,
    pub constraints: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
    pub relationship_priority: Option<Priority>,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub evidence: Option<Vec<String>>,
    pub conflict_ids: Option<Vec<EntityId>>,
    pub trace_links: Option<Vec<TraceLink>>,
    pub bidirectional: Option<bool>,
    pub distance_constraint_m: Option<f64>,
    pub capacity_constraint: Option<String>,
    pub regulatory_basis: Option<Vec<String>>,
    pub review_cycle: Option<String>,
    pub owner_id: Option<EntityId>,
    pub proximity_requirement: Option<TextField>,
    pub compatibility_requirement: Option<TextField>,
    pub incompatibility_requirement: Option<TextField>,
    pub separation_requirements: Option<Vec<SeparationKind>>,
}

impl_identified_header!(Relationship);

impl_patchable!(
    Relationship,
    RelationshipPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [source_id] => source_id,
        [target_id] => target_id,
        [kind] => kind,
        [strength] => strength,
        [directional] => directional,
        [rationale] => rationale,
        [constraints] => constraints,
        [conditions] => conditions,
        [relationship_priority] => relationship_priority,
        [valid_from] => valid_from,
        [valid_until] => valid_until,
        [evidence] => evidence,
        [conflict_ids] => conflict_ids,
        [trace_links] => trace_links,
        [bidirectional] => bidirectional,
        [distance_constraint_m] => distance_constraint_m,
        [capacity_constraint] => capacity_constraint,
        [regulatory_basis] => regulatory_basis,
        [review_cycle] => review_cycle,
        [owner_id] => owner_id,
        [proximity_requirement] => proximity_requirement,
        [compatibility_requirement] => compatibility_requirement,
        [incompatibility_requirement] => incompatibility_requirement,
        [separation_requirements] => separation_requirements,
    }
);
// #endregion

// #region 🔖️Adjacency
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Adjacency {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub element_a_id: EntityId,
    pub element_b_id: EntityId,
    pub kind: AdjacencyKind,
    pub connection: ConnectionKind,
    pub separations: Vec<SeparationKind>,
    pub weight: f64,
    pub rationale: Option<TextField>,
    #[dsl(unit = "m")]
    pub distance_max_m: Option<f64>,
    #[dsl(unit = "m")]
    pub distance_min_m: Option<f64>,
    pub level_constraint: Option<String>,
    pub access_path: Option<String>,
    pub shared_wall: bool,
    pub shared_entry: bool,
    pub traffic_isolation: bool,
    pub circulation_overlap: bool,
    pub conflict_ids: Vec<EntityId>,
    pub normalized: bool,
    pub verification_status: ValidationStatus,
    pub source_relationship_id: Option<EntityId>,
    pub internal_external_access: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AdjacencyPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub element_a_id: Option<EntityId>,
    pub element_b_id: Option<EntityId>,
    pub kind: Option<AdjacencyKind>,
    pub connection: Option<ConnectionKind>,
    pub separations: Option<Vec<SeparationKind>>,
    pub weight: Option<f64>,
    pub rationale: Option<TextField>,
    pub distance_max_m: Option<f64>,
    pub distance_min_m: Option<f64>,
    pub level_constraint: Option<String>,
    pub access_path: Option<String>,
    pub shared_wall: Option<bool>,
    pub shared_entry: Option<bool>,
    pub traffic_isolation: Option<bool>,
    pub circulation_overlap: Option<bool>,
    pub conflict_ids: Option<Vec<EntityId>>,
    pub normalized: Option<bool>,
    pub verification_status: Option<ValidationStatus>,
    pub source_relationship_id: Option<EntityId>,
    pub internal_external_access: Option<String>,
}

impl_identified_header!(Adjacency);

impl_patchable!(
    Adjacency,
    AdjacencyPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [element_a_id] => element_a_id,
        [element_b_id] => element_b_id,
        [kind] => kind,
        [connection] => connection,
        [separations] => separations,
        [weight] => weight,
        [rationale] => rationale,
        [distance_max_m] => distance_max_m,
        [distance_min_m] => distance_min_m,
        [level_constraint] => level_constraint,
        [access_path] => access_path,
        [shared_wall] => shared_wall,
        [shared_entry] => shared_entry,
        [traffic_isolation] => traffic_isolation,
        [circulation_overlap] => circulation_overlap,
        [conflict_ids] => conflict_ids,
        [normalized] => normalized,
        [verification_status] => verification_status,
        [source_relationship_id] => source_relationship_id,
        [internal_external_access] => internal_external_access,
    }
);
// #endregion
