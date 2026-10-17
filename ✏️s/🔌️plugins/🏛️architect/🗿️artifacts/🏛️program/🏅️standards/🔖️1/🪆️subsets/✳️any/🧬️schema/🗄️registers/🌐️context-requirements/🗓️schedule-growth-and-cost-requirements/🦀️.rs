//! 🗓️ Schedule, flexibility, growth, sustainability, resilience and cost requirements.

use super::*;

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
