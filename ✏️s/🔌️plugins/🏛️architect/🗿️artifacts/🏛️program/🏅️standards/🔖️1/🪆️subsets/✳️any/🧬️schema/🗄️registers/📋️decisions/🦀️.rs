//! 📋️ Architect decision registers: requirements, priorities, scenarios, option evaluations, decisions, validations, performance, quality, artifacts, changes, collaboration, analyses, reports and search filters; the shared enums live in `kernel` and the patch macros in [`super`].

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

// #region 🔖️QualityRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct QualityRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub quality_topic: String,
    pub standard: Option<String>,
    pub target_level: Option<String>,
    pub inspection_points: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub testing_requirements: Vec<String>,
    pub sample_rate: Option<String>,
    pub defect_categories: Vec<String>,
    pub corrective_action_process: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub supplier_requirements: Vec<String>,
    pub documentation_requirements: Vec<String>,
    pub training_requirements: Vec<String>,
    pub audit_schedule: Option<String>,
    pub kpis: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub certification_targets: Vec<String>,
    pub continuous_improvement: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct QualityRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub quality_topic: Option<String>,
    pub standard: Option<String>,
    pub target_level: Option<String>,
    pub inspection_points: Option<Vec<String>>,
    pub acceptance_criteria: Option<Vec<String>>,
    pub testing_requirements: Option<Vec<String>>,
    pub sample_rate: Option<String>,
    pub defect_categories: Option<Vec<String>>,
    pub corrective_action_process: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub supplier_requirements: Option<Vec<String>>,
    pub documentation_requirements: Option<Vec<String>>,
    pub training_requirements: Option<Vec<String>>,
    pub audit_schedule: Option<String>,
    pub kpis: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub certification_targets: Option<Vec<String>>,
    pub continuous_improvement: Option<Vec<String>>,
}

impl_identified_header!(QualityRecord);

impl_patchable!(
    QualityRecord,
    QualityRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [quality_topic] => quality_topic,
        [standard] => standard,
        [target_level] => target_level,
        [inspection_points] => inspection_points,
        [acceptance_criteria] => acceptance_criteria,
        [testing_requirements] => testing_requirements,
        [sample_rate] => sample_rate,
        [defect_categories] => defect_categories,
        [corrective_action_process] => corrective_action_process,
        [element_ids] => element_ids,
        [requirement_ids] => requirement_ids,
        [supplier_requirements] => supplier_requirements,
        [documentation_requirements] => documentation_requirements,
        [training_requirements] => training_requirements,
        [audit_schedule] => audit_schedule,
        [kpis] => kpis,
        [owner_id] => owner_id,
        [certification_targets] => certification_targets,
        [continuous_improvement] => continuous_improvement,
    }
);
// #endregion

// #region 🔖️ArtifactRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ArtifactRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub document_type: String,
    pub title: String,
    pub version: String,
    pub file_ref: Option<String>,
    pub format: Option<String>,
    pub author_ids: Vec<EntityId>,
    pub reviewer_ids: Vec<EntityId>,
    pub approver_ids: Vec<EntityId>,
    pub issue_date: Option<String>,
    pub revision_date: Option<String>,
    pub distribution_list: Vec<EntityId>,
    pub related_entity_ids: Vec<EntityId>,
    pub classification: Option<String>,
    pub retention_period: Option<String>,
    pub access_controls: Vec<String>,
    pub supersedes: Option<EntityId>,
    pub document_status: LifecycleStatus,
    pub checksum: Option<String>,
    pub source_system: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ArtifactRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub document_type: Option<String>,
    pub title: Option<String>,
    pub version: Option<String>,
    pub file_ref: Option<String>,
    pub format: Option<String>,
    pub author_ids: Option<Vec<EntityId>>,
    pub reviewer_ids: Option<Vec<EntityId>>,
    pub approver_ids: Option<Vec<EntityId>>,
    pub issue_date: Option<String>,
    pub revision_date: Option<String>,
    pub distribution_list: Option<Vec<EntityId>>,
    pub related_entity_ids: Option<Vec<EntityId>>,
    pub classification: Option<String>,
    pub retention_period: Option<String>,
    pub access_controls: Option<Vec<String>>,
    pub supersedes: Option<EntityId>,
    pub document_status: Option<LifecycleStatus>,
    pub checksum: Option<String>,
    pub source_system: Option<String>,
}

impl_identified_header!(ArtifactRecord);

impl_patchable!(
    ArtifactRecord,
    ArtifactRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [document_type] => document_type,
        [title] => title,
        [version] => version,
        [file_ref] => file_ref,
        [format] => format,
        [author_ids] => author_ids,
        [reviewer_ids] => reviewer_ids,
        [approver_ids] => approver_ids,
        [issue_date] => issue_date,
        [revision_date] => revision_date,
        [distribution_list] => distribution_list,
        [related_entity_ids] => related_entity_ids,
        [classification] => classification,
        [retention_period] => retention_period,
        [access_controls] => access_controls,
        [supersedes] => supersedes,
        [document_status] => document_status,
        [checksum] => checksum,
        [source_system] => source_system,
    }
);
// #endregion

// #region 🔖️ChangeRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ChangeRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub change_type: String,
    pub summary: TextField,
    pub reason: TextField,
    pub requested_by: Option<EntityId>,
    pub approved_by: Option<EntityId>,
    pub change_date: Option<String>,
    pub effective_date: Option<String>,
    pub impacted_entity_ids: Vec<EntityId>,
    pub before_snapshot: Option<String>,
    pub after_snapshot: Option<String>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub risk_impact: Vec<String>,
    pub approval_status: ValidationStatus,
    pub rollback_plan: Vec<String>,
    pub communication_plan: Vec<String>,
    pub version_from: Option<String>,
    pub version_to: Option<String>,
    pub audit_event_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ChangeRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub change_type: Option<String>,
    pub summary: Option<TextField>,
    pub reason: Option<TextField>,
    pub requested_by: Option<EntityId>,
    pub approved_by: Option<EntityId>,
    pub change_date: Option<String>,
    pub effective_date: Option<String>,
    pub impacted_entity_ids: Option<Vec<EntityId>>,
    pub before_snapshot: Option<String>,
    pub after_snapshot: Option<String>,
    pub cost_impact: Option<f64>,
    pub schedule_impact: Option<String>,
    pub risk_impact: Option<Vec<String>>,
    pub approval_status: Option<ValidationStatus>,
    pub rollback_plan: Option<Vec<String>>,
    pub communication_plan: Option<Vec<String>>,
    pub version_from: Option<String>,
    pub version_to: Option<String>,
    pub audit_event_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(ChangeRecord);

impl_patchable!(
    ChangeRecord,
    ChangeRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [change_type] => change_type,
        [summary] => summary,
        [reason] => reason,
        [requested_by] => requested_by,
        [approved_by] => approved_by,
        [change_date] => change_date,
        [effective_date] => effective_date,
        [impacted_entity_ids] => impacted_entity_ids,
        [before_snapshot] => before_snapshot,
        [after_snapshot] => after_snapshot,
        [cost_impact] => cost_impact,
        [schedule_impact] => schedule_impact,
        [risk_impact] => risk_impact,
        [approval_status] => approval_status,
        [rollback_plan] => rollback_plan,
        [communication_plan] => communication_plan,
        [version_from] => version_from,
        [version_to] => version_to,
        [audit_event_ids] => audit_event_ids,
    }
);
// #endregion

// #region 🔖️CollaborationRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CollaborationRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub session_type: String,
    pub title: String,
    pub participants: Vec<EntityId>,
    pub facilitator_id: Option<EntityId>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub agenda: Vec<String>,
    pub outcomes: Vec<String>,
    pub action_items: Vec<String>,
    pub decision_ids: Vec<EntityId>,
    pub issue_ids: Vec<EntityId>,
    pub document_ids: Vec<EntityId>,
    pub recording_ref: Option<String>,
    pub feedback: Vec<TaggedNote>,
    pub follow_up_date: Option<String>,
    pub workshop_id: Option<EntityId>,
    pub survey_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CollaborationRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub session_type: Option<String>,
    pub title: Option<String>,
    pub participants: Option<Vec<EntityId>>,
    pub facilitator_id: Option<EntityId>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub agenda: Option<Vec<String>>,
    pub outcomes: Option<Vec<String>>,
    pub action_items: Option<Vec<String>>,
    pub decision_ids: Option<Vec<EntityId>>,
    pub issue_ids: Option<Vec<EntityId>>,
    pub document_ids: Option<Vec<EntityId>>,
    pub recording_ref: Option<String>,
    pub feedback: Option<Vec<TaggedNote>>,
    pub follow_up_date: Option<String>,
    pub workshop_id: Option<EntityId>,
    pub survey_id: Option<EntityId>,
}

impl_identified_header!(CollaborationRecord);

impl_patchable!(
    CollaborationRecord,
    CollaborationRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [session_type] => session_type,
        [title] => title,
        [participants] => participants,
        [facilitator_id] => facilitator_id,
        [start_time] => start_time,
        [end_time] => end_time,
        [location] => location,
        [agenda] => agenda,
        [outcomes] => outcomes,
        [action_items] => action_items,
        [decision_ids] => decision_ids,
        [issue_ids] => issue_ids,
        [document_ids] => document_ids,
        [recording_ref] => recording_ref,
        [feedback] => feedback,
        [follow_up_date] => follow_up_date,
        [workshop_id] => workshop_id,
        [survey_id] => survey_id,
    }
);
// #endregion

// #region 🔖️AnalysisRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AnalysisRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub kind: AnalysisKind,
    pub title: String,
    pub parameters: Vec<String>,
    pub input_entity_ids: Vec<EntityId>,
    pub output_summary: TextField,
    pub findings: Vec<String>,
    pub metrics: Vec<String>,
    pub charts: Vec<String>,
    pub run_by: Option<EntityId>,
    pub run_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub tool_version: Option<String>,
    pub scenario_id: Option<EntityId>,
    pub report_id: Option<EntityId>,
    pub confidence: Option<String>,
    pub limitations: Vec<String>,
    pub recommendations: Vec<String>,
    pub raw_result_ref: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AnalysisRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub kind: Option<AnalysisKind>,
    pub title: Option<String>,
    pub parameters: Option<Vec<String>>,
    pub input_entity_ids: Option<Vec<EntityId>>,
    pub output_summary: Option<TextField>,
    pub findings: Option<Vec<String>>,
    pub metrics: Option<Vec<String>>,
    pub charts: Option<Vec<String>>,
    pub run_by: Option<EntityId>,
    pub run_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub tool_version: Option<String>,
    pub scenario_id: Option<EntityId>,
    pub report_id: Option<EntityId>,
    pub confidence: Option<String>,
    pub limitations: Option<Vec<String>>,
    pub recommendations: Option<Vec<String>>,
    pub raw_result_ref: Option<String>,
}

impl_identified_header!(AnalysisRecord);

impl_patchable!(
    AnalysisRecord,
    AnalysisRecordPatch,
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
        [title] => title,
        [parameters] => parameters,
        [input_entity_ids] => input_entity_ids,
        [output_summary] => output_summary,
        [findings] => findings,
        [metrics] => metrics,
        [charts] => charts,
        [run_by] => run_by,
        [run_at] => run_at,
        [duration_ms] => duration_ms,
        [tool_version] => tool_version,
        [scenario_id] => scenario_id,
        [report_id] => report_id,
        [confidence] => confidence,
        [limitations] => limitations,
        [recommendations] => recommendations,
        [raw_result_ref] => raw_result_ref,
    }
);
// #endregion

// #region 🔖️ReportRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReportRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub kind: ReportKind,
    pub title: String,
    pub audience: Vec<String>,
    pub sections: Vec<String>,
    pub generated_at: Option<String>,
    pub generated_by: Option<EntityId>,
    pub analysis_ids: Vec<EntityId>,
    pub format: Option<String>,
    pub file_ref: Option<String>,
    pub distribution_list: Vec<EntityId>,
    pub approval_status: ValidationStatus,
    pub approver_id: Option<EntityId>,
    pub version: String,
    pub template_id: Option<EntityId>,
    pub parameters: Vec<String>,
    pub confidentiality: Option<String>,
    pub expiry_date: Option<String>,
    pub related_decision_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReportRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub kind: Option<ReportKind>,
    pub title: Option<String>,
    pub audience: Option<Vec<String>>,
    pub sections: Option<Vec<String>>,
    pub generated_at: Option<String>,
    pub generated_by: Option<EntityId>,
    pub analysis_ids: Option<Vec<EntityId>>,
    pub format: Option<String>,
    pub file_ref: Option<String>,
    pub distribution_list: Option<Vec<EntityId>>,
    pub approval_status: Option<ValidationStatus>,
    pub approver_id: Option<EntityId>,
    pub version: Option<String>,
    pub template_id: Option<EntityId>,
    pub parameters: Option<Vec<String>>,
    pub confidentiality: Option<String>,
    pub expiry_date: Option<String>,
    pub related_decision_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(ReportRecord);

impl_patchable!(
    ReportRecord,
    ReportRecordPatch,
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
        [title] => title,
        [audience] => audience,
        [sections] => sections,
        [generated_at] => generated_at,
        [generated_by] => generated_by,
        [analysis_ids] => analysis_ids,
        [format] => format,
        [file_ref] => file_ref,
        [distribution_list] => distribution_list,
        [approval_status] => approval_status,
        [approver_id] => approver_id,
        [version] => version,
        [template_id] => template_id,
        [parameters] => parameters,
        [confidentiality] => confidentiality,
        [expiry_date] => expiry_date,
        [related_decision_ids] => related_decision_ids,
    }
);
// #endregion

// #region 🔖️SearchFilter
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SearchFilter {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub filter_name: String,
    pub filter_description: Option<TextField>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub owner_ids: Vec<EntityId>,
    pub statuses: Vec<LifecycleStatus>,
    pub priorities: Vec<Priority>,
    pub sources: Vec<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub entity_kinds: Vec<String>,
    pub tag_filters: Vec<String>,
    pub sort_field: Option<String>,
    pub sort_direction: Option<String>,
    pub is_public: bool,
    pub created_by: Option<EntityId>,
    pub last_used: Option<String>,
    pub use_count: u64,
    pub pinned: bool,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SearchFilterPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub filter_name: Option<String>,
    pub filter_description: Option<TextField>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub owner_ids: Option<Vec<EntityId>>,
    pub statuses: Option<Vec<LifecycleStatus>>,
    pub priorities: Option<Vec<Priority>>,
    pub sources: Option<Vec<String>>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub entity_kinds: Option<Vec<String>>,
    pub tag_filters: Option<Vec<String>>,
    pub sort_field: Option<String>,
    pub sort_direction: Option<String>,
    pub is_public: Option<bool>,
    pub created_by: Option<EntityId>,
    pub last_used: Option<String>,
    pub use_count: Option<u64>,
    pub pinned: Option<bool>,
}

impl_identified_header!(SearchFilter);

impl_patchable!(
    SearchFilter,
    SearchFilterPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [filter_name] => filter_name,
        [filter_description] => filter_description,
        [keywords] => keywords,
        [categories] => categories,
        [owner_ids] => owner_ids,
        [statuses] => statuses,
        [priorities] => priorities,
        [sources] => sources,
        [date_from] => date_from,
        [date_to] => date_to,
        [entity_kinds] => entity_kinds,
        [tag_filters] => tag_filters,
        [sort_field] => sort_field,
        [sort_direction] => sort_direction,
        [is_public] => is_public,
        [created_by] => created_by,
        [last_used] => last_used,
        [use_count] => use_count,
        [pinned] => pinned,
    }
);
// #endregion

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
