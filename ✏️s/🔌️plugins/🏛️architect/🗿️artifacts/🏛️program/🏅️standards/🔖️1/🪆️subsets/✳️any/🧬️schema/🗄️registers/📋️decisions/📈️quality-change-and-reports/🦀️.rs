//! 📈️ Quality, artifact, change, collaboration, analysis and report records plus search filters.

use super::*;

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
