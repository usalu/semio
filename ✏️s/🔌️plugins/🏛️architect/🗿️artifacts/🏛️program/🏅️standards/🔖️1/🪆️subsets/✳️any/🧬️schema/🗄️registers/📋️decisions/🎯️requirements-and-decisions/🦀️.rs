//! 🎯️ Requirements, priorities, scenarios, option evaluations, decisions, validations and performance criteria.

use super::*;

// #region 🔖️Requirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Requirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub code: String,
    pub kind: RequirementKind,
    pub statement: TextField,
    pub rationale: Option<TextField>,
    pub source: Option<String>,
    pub stakeholder_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub function_ids: Vec<EntityId>,
    pub parent_requirement_id: Option<EntityId>,
    pub child_requirement_ids: Vec<EntityId>,
    pub acceptance_criteria: Vec<String>,
    pub verification_method: Option<String>,
    pub validation_status: ValidationStatus,
    pub conflict_ids: Vec<EntityId>,
    pub risk_ids: Vec<EntityId>,
    pub cost_estimate: Option<f64>,
    pub schedule_constraint: Option<String>,
    pub regulatory_refs: Vec<String>,
    pub trace_links: Vec<TraceLink>,
    pub superseded_by: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct RequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub kind: Option<RequirementKind>,
    pub statement: Option<TextField>,
    pub rationale: Option<TextField>,
    pub source: Option<String>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub function_ids: Option<Vec<EntityId>>,
    pub parent_requirement_id: Option<EntityId>,
    pub child_requirement_ids: Option<Vec<EntityId>>,
    pub acceptance_criteria: Option<Vec<String>>,
    pub verification_method: Option<String>,
    pub validation_status: Option<ValidationStatus>,
    pub conflict_ids: Option<Vec<EntityId>>,
    pub risk_ids: Option<Vec<EntityId>>,
    pub cost_estimate: Option<f64>,
    pub schedule_constraint: Option<String>,
    pub regulatory_refs: Option<Vec<String>>,
    pub trace_links: Option<Vec<TraceLink>>,
    pub superseded_by: Option<EntityId>,
}

impl_identified_header!(Requirement);

impl_patchable!(
    Requirement,
    RequirementPatch,
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
        [statement] => statement,
        [rationale] => rationale,
        [source] => source,
        [stakeholder_ids] => stakeholder_ids,
        [element_ids] => element_ids,
        [function_ids] => function_ids,
        [parent_requirement_id] => parent_requirement_id,
        [child_requirement_ids] => child_requirement_ids,
        [acceptance_criteria] => acceptance_criteria,
        [verification_method] => verification_method,
        [validation_status] => validation_status,
        [conflict_ids] => conflict_ids,
        [risk_ids] => risk_ids,
        [cost_estimate] => cost_estimate,
        [schedule_constraint] => schedule_constraint,
        [regulatory_refs] => regulatory_refs,
        [trace_links] => trace_links,
        [superseded_by] => superseded_by,
    }
);
// #endregion

// #region 🔖️PriorityRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PriorityRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub subject_id: EntityId,
    pub subject_kind: String,
    pub ranked_priority: Priority,
    pub rank: Option<u32>,
    pub weight: Option<f64>,
    pub rationale: Option<TextField>,
    pub decision_id: Option<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub effective_from: Option<String>,
    pub effective_until: Option<String>,
    pub review_cycle: Option<String>,
    pub dependencies: Vec<EntityId>,
    pub conflicts: Vec<EntityId>,
    pub scoring_method: Option<String>,
    pub score: Option<f64>,
    pub criteria: Vec<String>,
    pub approved_by: Option<EntityId>,
    pub approval_date: Option<String>,
    pub ranking_notes: Vec<TaggedNote>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PriorityRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub subject_id: Option<EntityId>,
    pub subject_kind: Option<String>,
    pub ranked_priority: Option<Priority>,
    pub rank: Option<u32>,
    pub weight: Option<f64>,
    pub rationale: Option<TextField>,
    pub decision_id: Option<EntityId>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub effective_from: Option<String>,
    pub effective_until: Option<String>,
    pub review_cycle: Option<String>,
    pub dependencies: Option<Vec<EntityId>>,
    pub conflicts: Option<Vec<EntityId>>,
    pub scoring_method: Option<String>,
    pub score: Option<f64>,
    pub criteria: Option<Vec<String>>,
    pub approved_by: Option<EntityId>,
    pub approval_date: Option<String>,
    pub ranking_notes: Option<Vec<TaggedNote>>,
}

impl_identified_header!(PriorityRecord);

impl_patchable!(
    PriorityRecord,
    PriorityRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [subject_id] => subject_id,
        [subject_kind] => subject_kind,
        [ranked_priority] => ranked_priority,
        [rank] => rank,
        [weight] => weight,
        [rationale] => rationale,
        [decision_id] => decision_id,
        [stakeholder_ids] => stakeholder_ids,
        [effective_from] => effective_from,
        [effective_until] => effective_until,
        [review_cycle] => review_cycle,
        [dependencies] => dependencies,
        [conflicts] => conflicts,
        [scoring_method] => scoring_method,
        [score] => score,
        [criteria] => criteria,
        [approved_by] => approved_by,
        [approval_date] => approval_date,
        [ranking_notes] => ranking_notes,
    }
);
// #endregion

// #region 🔖️Scenario
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Scenario {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub code: String,
    pub hypothesis: TextField,
    pub assumptions: Vec<String>,
    pub variables: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub growth_plan_id: Option<EntityId>,
    pub probability: Option<f64>,
    pub impact_summary: Option<TextField>,
    pub cost_delta: Option<f64>,
    pub area_delta: Option<f64>,
    pub headcount_delta: Option<f64>,
    pub schedule_delta: Option<String>,
    pub risk_ids: Vec<EntityId>,
    pub option_ids: Vec<EntityId>,
    pub baseline: bool,
    pub preferred: bool,
    pub analysis_ids: Vec<EntityId>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ScenarioPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub hypothesis: Option<TextField>,
    pub assumptions: Option<Vec<String>>,
    pub variables: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub growth_plan_id: Option<EntityId>,
    pub probability: Option<f64>,
    pub impact_summary: Option<TextField>,
    pub cost_delta: Option<f64>,
    pub area_delta: Option<f64>,
    pub headcount_delta: Option<f64>,
    pub schedule_delta: Option<String>,
    pub risk_ids: Option<Vec<EntityId>>,
    pub option_ids: Option<Vec<EntityId>>,
    pub baseline: Option<bool>,
    pub preferred: Option<bool>,
    pub analysis_ids: Option<Vec<EntityId>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(Scenario);

impl_patchable!(
    Scenario,
    ScenarioPatch,
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
        [hypothesis] => hypothesis,
        [assumptions] => assumptions,
        [variables] => variables,
        [element_ids] => element_ids,
        [requirement_ids] => requirement_ids,
        [growth_plan_id] => growth_plan_id,
        [probability] => probability,
        [impact_summary] => impact_summary,
        [cost_delta] => cost_delta,
        [area_delta] => area_delta,
        [headcount_delta] => headcount_delta,
        [schedule_delta] => schedule_delta,
        [risk_ids] => risk_ids,
        [option_ids] => option_ids,
        [baseline] => baseline,
        [preferred] => preferred,
        [analysis_ids] => analysis_ids,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️OptionEvaluation
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct OptionEvaluation {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub option_name: String,
    pub option_description: TextField,
    pub scenario_id: Option<EntityId>,
    pub criteria_ids: Vec<EntityId>,
    pub scores: Vec<f64>,
    pub weighted_score: Option<f64>,
    pub cost_estimate: Option<f64>,
    pub schedule_estimate: Option<String>,
    pub risk_summary: Vec<String>,
    pub benefits: Vec<String>,
    pub drawbacks: Vec<String>,
    pub assumptions: Vec<String>,
    pub dependencies: Vec<EntityId>,
    pub stakeholder_feedback: Vec<TaggedNote>,
    pub recommendation: Option<String>,
    pub decision_id: Option<EntityId>,
    pub evaluation_status: ValidationStatus,
    pub evaluator_ids: Vec<EntityId>,
    pub evaluation_date: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct OptionEvaluationPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub option_name: Option<String>,
    pub option_description: Option<TextField>,
    pub scenario_id: Option<EntityId>,
    pub criteria_ids: Option<Vec<EntityId>>,
    pub scores: Option<Vec<f64>>,
    pub weighted_score: Option<f64>,
    pub cost_estimate: Option<f64>,
    pub schedule_estimate: Option<String>,
    pub risk_summary: Option<Vec<String>>,
    pub benefits: Option<Vec<String>>,
    pub drawbacks: Option<Vec<String>>,
    pub assumptions: Option<Vec<String>>,
    pub dependencies: Option<Vec<EntityId>>,
    pub stakeholder_feedback: Option<Vec<TaggedNote>>,
    pub recommendation: Option<String>,
    pub decision_id: Option<EntityId>,
    pub evaluation_status: Option<ValidationStatus>,
    pub evaluator_ids: Option<Vec<EntityId>>,
    pub evaluation_date: Option<String>,
}

impl_identified_header!(OptionEvaluation);

impl_patchable!(
    OptionEvaluation,
    OptionEvaluationPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [option_name] => option_name,
        [option_description] => option_description,
        [scenario_id] => scenario_id,
        [criteria_ids] => criteria_ids,
        [scores] => scores,
        [weighted_score] => weighted_score,
        [cost_estimate] => cost_estimate,
        [schedule_estimate] => schedule_estimate,
        [risk_summary] => risk_summary,
        [benefits] => benefits,
        [drawbacks] => drawbacks,
        [assumptions] => assumptions,
        [dependencies] => dependencies,
        [stakeholder_feedback] => stakeholder_feedback,
        [recommendation] => recommendation,
        [decision_id] => decision_id,
        [evaluation_status] => evaluation_status,
        [evaluator_ids] => evaluator_ids,
        [evaluation_date] => evaluation_date,
    }
);
// #endregion

// #region 🔖️Decision
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Decision {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub decision_statement: TextField,
    pub context: TextField,
    pub options_considered: Vec<EntityId>,
    pub selected_option_id: Option<EntityId>,
    pub rationale: TextField,
    pub decision_maker_ids: Vec<EntityId>,
    pub consulted_ids: Vec<EntityId>,
    pub informed_ids: Vec<EntityId>,
    pub decision_date: Option<String>,
    pub effective_date: Option<String>,
    pub reversal_conditions: Vec<String>,
    pub impacted_requirement_ids: Vec<EntityId>,
    pub impacted_element_ids: Vec<EntityId>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub risk_impact: Vec<String>,
    pub approval_status: ValidationStatus,
    pub meeting_ref: Option<EntityId>,
    pub artifact_refs: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DecisionPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub decision_statement: Option<TextField>,
    pub context: Option<TextField>,
    pub options_considered: Option<Vec<EntityId>>,
    pub selected_option_id: Option<EntityId>,
    pub rationale: Option<TextField>,
    pub decision_maker_ids: Option<Vec<EntityId>>,
    pub consulted_ids: Option<Vec<EntityId>>,
    pub informed_ids: Option<Vec<EntityId>>,
    pub decision_date: Option<String>,
    pub effective_date: Option<String>,
    pub reversal_conditions: Option<Vec<String>>,
    pub impacted_requirement_ids: Option<Vec<EntityId>>,
    pub impacted_element_ids: Option<Vec<EntityId>>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub risk_impact: Option<Vec<String>>,
    pub approval_status: Option<ValidationStatus>,
    pub meeting_ref: Option<EntityId>,
    pub artifact_refs: Option<Vec<EntityId>>,
}

impl_identified_header!(Decision);

impl_patchable!(
    Decision,
    DecisionPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [decision_statement] => decision_statement,
        [context] => context,
        [options_considered] => options_considered,
        [selected_option_id] => selected_option_id,
        [rationale] => rationale,
        [decision_maker_ids] => decision_maker_ids,
        [consulted_ids] => consulted_ids,
        [informed_ids] => informed_ids,
        [decision_date] => decision_date,
        [effective_date] => effective_date,
        [reversal_conditions] => reversal_conditions,
        [impacted_requirement_ids] => impacted_requirement_ids,
        [impacted_element_ids] => impacted_element_ids,
        [cost_impact] => cost_impact,
        [schedule_impact] => schedule_impact,
        [risk_impact] => risk_impact,
        [approval_status] => approval_status,
        [meeting_ref] => meeting_ref,
        [artifact_refs] => artifact_refs,
    }
);
// #endregion

// #region 🔖️ValidationRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ValidationRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub subject_id: EntityId,
    pub subject_kind: String,
    pub validation_type: String,
    pub method: Option<String>,
    pub criteria: Vec<String>,
    pub result: ValidationStatus,
    pub evidence: Vec<String>,
    pub validator_ids: Vec<EntityId>,
    pub validation_date: Option<String>,
    pub next_review_date: Option<String>,
    pub findings: Vec<String>,
    pub non_conformities: Vec<String>,
    pub corrective_actions: Vec<String>,
    pub waivers: Vec<String>,
    pub standards: Vec<String>,
    pub trace_links: Vec<TraceLink>,
    pub report_id: Option<EntityId>,
    pub confidence_level: Option<String>,
    pub validation_notes: Vec<TaggedNote>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ValidationRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub subject_id: Option<EntityId>,
    pub subject_kind: Option<String>,
    pub validation_type: Option<String>,
    pub method: Option<String>,
    pub criteria: Option<Vec<String>>,
    pub result: Option<ValidationStatus>,
    pub evidence: Option<Vec<String>>,
    pub validator_ids: Option<Vec<EntityId>>,
    pub validation_date: Option<String>,
    pub next_review_date: Option<String>,
    pub findings: Option<Vec<String>>,
    pub non_conformities: Option<Vec<String>>,
    pub corrective_actions: Option<Vec<String>>,
    pub waivers: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub trace_links: Option<Vec<TraceLink>>,
    pub report_id: Option<EntityId>,
    pub confidence_level: Option<String>,
    pub validation_notes: Option<Vec<TaggedNote>>,
}

impl_identified_header!(ValidationRecord);

impl_patchable!(
    ValidationRecord,
    ValidationRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [subject_id] => subject_id,
        [subject_kind] => subject_kind,
        [validation_type] => validation_type,
        [method] => method,
        [criteria] => criteria,
        [result] => result,
        [evidence] => evidence,
        [validator_ids] => validator_ids,
        [validation_date] => validation_date,
        [next_review_date] => next_review_date,
        [findings] => findings,
        [non_conformities] => non_conformities,
        [corrective_actions] => corrective_actions,
        [waivers] => waivers,
        [standards] => standards,
        [trace_links] => trace_links,
        [report_id] => report_id,
        [confidence_level] => confidence_level,
        [validation_notes] => validation_notes,
    }
);
// #endregion

// #region 🔖️PerformanceCriterion
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PerformanceCriterion {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub criterion: String,
    pub metric: String,
    pub target: Option<f64>,
    pub unit: Option<String>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub measurement_method: Option<String>,
    pub frequency: Option<String>,
    pub requirement_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub baseline: Option<f64>,
    pub benchmark_ref: Option<EntityId>,
    pub weight: Option<f64>,
    pub data_source: Option<String>,
    pub reporting_cadence: Option<String>,
    pub owner_id: Option<EntityId>,
    pub verification_plan: Option<String>,
    pub penalty_threshold: Option<f64>,
    pub incentive_threshold: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PerformanceCriterionPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub criterion: Option<String>,
    pub metric: Option<String>,
    pub target: Option<f64>,
    pub unit: Option<String>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub measurement_method: Option<String>,
    pub frequency: Option<String>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub baseline: Option<f64>,
    pub benchmark_ref: Option<EntityId>,
    pub weight: Option<f64>,
    pub data_source: Option<String>,
    pub reporting_cadence: Option<String>,
    pub owner_id: Option<EntityId>,
    pub verification_plan: Option<String>,
    pub penalty_threshold: Option<f64>,
    pub incentive_threshold: Option<f64>,
}

impl_identified_header!(PerformanceCriterion);

impl_patchable!(
    PerformanceCriterion,
    PerformanceCriterionPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [criterion] => criterion,
        [metric] => metric,
        [target] => target,
        [unit] => unit,
        [minimum] => minimum,
        [maximum] => maximum,
        [measurement_method] => measurement_method,
        [frequency] => frequency,
        [requirement_ids] => requirement_ids,
        [element_ids] => element_ids,
        [baseline] => baseline,
        [benchmark_ref] => benchmark_ref,
        [weight] => weight,
        [data_source] => data_source,
        [reporting_cadence] => reporting_cadence,
        [owner_id] => owner_id,
        [verification_plan] => verification_plan,
        [penalty_threshold] => penalty_threshold,
        [incentive_threshold] => incentive_threshold,
    }
);
// #endregion
