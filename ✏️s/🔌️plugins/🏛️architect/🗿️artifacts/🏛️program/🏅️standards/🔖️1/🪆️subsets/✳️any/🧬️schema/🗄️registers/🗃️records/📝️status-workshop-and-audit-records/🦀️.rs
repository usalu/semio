//! 📝️ Status, workshop, survey, issue, audit, template and knowledge records.

use super::*;

// #region 🔖️StatusRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct StatusRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub subject_id: EntityId,
    pub subject_kind: String,
    pub record_status: LifecycleStatus,
    pub previous_status: Option<LifecycleStatus>,
    pub changed_by: Option<EntityId>,
    pub changed_at: Option<String>,
    pub reason: Option<TextField>,
    pub blockers: Vec<String>,
    pub next_actions: Vec<String>,
    pub due_date: Option<String>,
    #[dsl(unit = "pct")]
    pub progress_percent: Option<f64>,
    pub health: Option<String>,
    pub escalation_level: Option<String>,
    pub related_issue_ids: Vec<EntityId>,
    pub related_risk_ids: Vec<EntityId>,
    pub milestone_id: Option<EntityId>,
    pub reporting_period: Option<String>,
    pub status_notes: Vec<TaggedNote>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct StatusRecordPatch {
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
    pub record_status: Option<LifecycleStatus>,
    pub previous_status: Option<LifecycleStatus>,
    pub changed_by: Option<EntityId>,
    pub changed_at: Option<String>,
    pub reason: Option<TextField>,
    pub blockers: Option<Vec<String>>,
    pub next_actions: Option<Vec<String>>,
    pub due_date: Option<String>,
    pub progress_percent: Option<f64>,
    pub health: Option<String>,
    pub escalation_level: Option<String>,
    pub related_issue_ids: Option<Vec<EntityId>>,
    pub related_risk_ids: Option<Vec<EntityId>>,
    pub milestone_id: Option<EntityId>,
    pub reporting_period: Option<String>,
    pub status_notes: Option<Vec<TaggedNote>>,
}

impl_identified_header!(StatusRecord);

impl_patchable!(
    StatusRecord,
    StatusRecordPatch,
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
        [record_status] => record_status,
        [previous_status] => previous_status,
        [changed_by] => changed_by,
        [changed_at] => changed_at,
        [reason] => reason,
        [blockers] => blockers,
        [next_actions] => next_actions,
        [due_date] => due_date,
        [progress_percent] => progress_percent,
        [health] => health,
        [escalation_level] => escalation_level,
        [related_issue_ids] => related_issue_ids,
        [related_risk_ids] => related_risk_ids,
        [milestone_id] => milestone_id,
        [reporting_period] => reporting_period,
        [status_notes] => status_notes,
    }
);
// #endregion

// #region 🔖️Workshop
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Workshop {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub workshop_type: String,
    pub objectives: Vec<String>,
    pub agenda: Vec<String>,
    pub facilitator_id: Option<EntityId>,
    pub participants: Vec<EntityId>,
    pub scheduled_start: Option<String>,
    pub scheduled_end: Option<String>,
    pub location: Option<String>,
    pub materials: Vec<String>,
    pub methods: Vec<String>,
    pub outputs: Vec<String>,
    pub decisions: Vec<EntityId>,
    pub issues: Vec<EntityId>,
    pub follow_up_actions: Vec<String>,
    pub feedback: Vec<TaggedNote>,
    pub recording_ref: Option<String>,
    pub budget: Option<f64>,
    pub workshop_status: LifecycleStatus,
    pub survey_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct WorkshopPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub workshop_type: Option<String>,
    pub objectives: Option<Vec<String>>,
    pub agenda: Option<Vec<String>>,
    pub facilitator_id: Option<EntityId>,
    pub participants: Option<Vec<EntityId>>,
    pub scheduled_start: Option<String>,
    pub scheduled_end: Option<String>,
    pub location: Option<String>,
    pub materials: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
    pub outputs: Option<Vec<String>>,
    pub decisions: Option<Vec<EntityId>>,
    pub issues: Option<Vec<EntityId>>,
    pub follow_up_actions: Option<Vec<String>>,
    pub feedback: Option<Vec<TaggedNote>>,
    pub recording_ref: Option<String>,
    pub budget: Option<f64>,
    pub workshop_status: Option<LifecycleStatus>,
    pub survey_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(Workshop);

impl_patchable!(
    Workshop,
    WorkshopPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [workshop_type] => workshop_type,
        [objectives] => objectives,
        [agenda] => agenda,
        [facilitator_id] => facilitator_id,
        [participants] => participants,
        [scheduled_start] => scheduled_start,
        [scheduled_end] => scheduled_end,
        [location] => location,
        [materials] => materials,
        [methods] => methods,
        [outputs] => outputs,
        [decisions] => decisions,
        [issues] => issues,
        [follow_up_actions] => follow_up_actions,
        [feedback] => feedback,
        [recording_ref] => recording_ref,
        [budget] => budget,
        [workshop_status] => workshop_status,
        [survey_ids] => survey_ids,
    }
);
// #endregion

// #region 🔖️Survey
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Survey {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub survey_type: String,
    pub title: String,
    pub objectives: Vec<String>,
    pub questions: Vec<String>,
    pub target_audience: Vec<EntityId>,
    pub distribution_channels: Vec<String>,
    pub launch_date: Option<String>,
    pub close_date: Option<String>,
    pub response_count: u32,
    pub response_rate: Option<f64>,
    pub findings: Vec<String>,
    pub themes: Vec<String>,
    pub recommendations: Vec<String>,
    pub confidentiality: Option<String>,
    pub consent_process: Vec<String>,
    pub analysis_id: Option<EntityId>,
    pub workshop_id: Option<EntityId>,
    pub owner_id: Option<EntityId>,
    pub survey_status: LifecycleStatus,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SurveyPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub survey_type: Option<String>,
    pub title: Option<String>,
    pub objectives: Option<Vec<String>>,
    pub questions: Option<Vec<String>>,
    pub target_audience: Option<Vec<EntityId>>,
    pub distribution_channels: Option<Vec<String>>,
    pub launch_date: Option<String>,
    pub close_date: Option<String>,
    pub response_count: Option<u32>,
    pub response_rate: Option<f64>,
    pub findings: Option<Vec<String>>,
    pub themes: Option<Vec<String>>,
    pub recommendations: Option<Vec<String>>,
    pub confidentiality: Option<String>,
    pub consent_process: Option<Vec<String>>,
    pub analysis_id: Option<EntityId>,
    pub workshop_id: Option<EntityId>,
    pub owner_id: Option<EntityId>,
    pub survey_status: Option<LifecycleStatus>,
}

impl_identified_header!(Survey);

impl_patchable!(
    Survey,
    SurveyPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [survey_type] => survey_type,
        [title] => title,
        [objectives] => objectives,
        [questions] => questions,
        [target_audience] => target_audience,
        [distribution_channels] => distribution_channels,
        [launch_date] => launch_date,
        [close_date] => close_date,
        [response_count] => response_count,
        [response_rate] => response_rate,
        [findings] => findings,
        [themes] => themes,
        [recommendations] => recommendations,
        [confidentiality] => confidentiality,
        [consent_process] => consent_process,
        [analysis_id] => analysis_id,
        [workshop_id] => workshop_id,
        [owner_id] => owner_id,
        [survey_status] => survey_status,
    }
);
// #endregion

// #region 🔖️Issue
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Issue {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub issue_type: String,
    pub summary: TextField,
    pub issue_description: TextField,
    pub severity: IssueSeverity,
    pub issue_priority: Priority,
    pub reporter_id: Option<EntityId>,
    pub assignee_id: Option<EntityId>,
    pub affected_entity_ids: Vec<EntityId>,
    pub root_cause: Option<TextField>,
    pub resolution: Option<TextField>,
    pub workaround: Option<TextField>,
    pub due_date: Option<String>,
    pub resolved_date: Option<String>,
    pub related_conflict_ids: Vec<EntityId>,
    pub related_risk_ids: Vec<EntityId>,
    pub decision_id: Option<EntityId>,
    pub comments: Vec<TaggedNote>,
    pub attachments: Vec<EntityId>,
    pub escalation_level: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct IssuePatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub issue_type: Option<String>,
    pub summary: Option<TextField>,
    pub issue_description: Option<TextField>,
    pub severity: Option<IssueSeverity>,
    pub issue_priority: Option<Priority>,
    pub reporter_id: Option<EntityId>,
    pub assignee_id: Option<EntityId>,
    pub affected_entity_ids: Option<Vec<EntityId>>,
    pub root_cause: Option<TextField>,
    pub resolution: Option<TextField>,
    pub workaround: Option<TextField>,
    pub due_date: Option<String>,
    pub resolved_date: Option<String>,
    pub related_conflict_ids: Option<Vec<EntityId>>,
    pub related_risk_ids: Option<Vec<EntityId>>,
    pub decision_id: Option<EntityId>,
    pub comments: Option<Vec<TaggedNote>>,
    pub attachments: Option<Vec<EntityId>>,
    pub escalation_level: Option<String>,
}

impl_identified_header!(Issue);

impl_patchable!(
    Issue,
    IssuePatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [issue_type] => issue_type,
        [summary] => summary,
        [issue_description] => issue_description,
        [severity] => severity,
        [issue_priority] => issue_priority,
        [reporter_id] => reporter_id,
        [assignee_id] => assignee_id,
        [affected_entity_ids] => affected_entity_ids,
        [root_cause] => root_cause,
        [resolution] => resolution,
        [workaround] => workaround,
        [due_date] => due_date,
        [resolved_date] => resolved_date,
        [related_conflict_ids] => related_conflict_ids,
        [related_risk_ids] => related_risk_ids,
        [decision_id] => decision_id,
        [comments] => comments,
        [attachments] => attachments,
        [escalation_level] => escalation_level,
    }
);
// #endregion

// #region 🔖️AuditEvent
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AuditEvent {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub action: AuditAction,
    pub actor_id: Option<EntityId>,
    pub subject_id: EntityId,
    pub subject_kind: String,
    pub timestamp: String,
    pub details: TextField,
    pub before_state: Option<String>,
    pub after_state: Option<String>,
    pub ip_address: Option<String>,
    pub client: Option<String>,
    pub session_id: Option<String>,
    pub change_record_id: Option<EntityId>,
    pub trace_link: Option<TraceLink>,
    pub success: bool,
    pub error_message: Option<String>,
    pub correlation_id: Option<String>,
    pub compliance_tags: Vec<String>,
    pub retention_until: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AuditEventPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub action: Option<AuditAction>,
    pub actor_id: Option<EntityId>,
    pub subject_id: Option<EntityId>,
    pub subject_kind: Option<String>,
    pub timestamp: Option<String>,
    pub details: Option<TextField>,
    pub before_state: Option<String>,
    pub after_state: Option<String>,
    pub ip_address: Option<String>,
    pub client: Option<String>,
    pub session_id: Option<String>,
    pub change_record_id: Option<EntityId>,
    pub trace_link: Option<TraceLink>,
    pub success: Option<bool>,
    pub error_message: Option<String>,
    pub correlation_id: Option<String>,
    pub compliance_tags: Option<Vec<String>>,
    pub retention_until: Option<String>,
}

impl_identified_header!(AuditEvent);

impl_patchable!(
    AuditEvent,
    AuditEventPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [action] => action,
        [actor_id] => actor_id,
        [subject_id] => subject_id,
        [subject_kind] => subject_kind,
        [timestamp] => timestamp,
        [details] => details,
        [before_state] => before_state,
        [after_state] => after_state,
        [ip_address] => ip_address,
        [client] => client,
        [session_id] => session_id,
        [change_record_id] => change_record_id,
        [trace_link] => trace_link,
        [success] => success,
        [error_message] => error_message,
        [correlation_id] => correlation_id,
        [compliance_tags] => compliance_tags,
        [retention_until] => retention_until,
    }
);
// #endregion

// #region 🔖️TemplateRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TemplateRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub template_type: String,
    pub sector: Option<String>,
    pub project_type: Option<String>,
    pub version: String,
    pub content_ref: Option<String>,
    pub entity_kinds: Vec<String>,
    pub default_fields: Vec<String>,
    pub checklists: Vec<String>,
    pub standards: Vec<String>,
    pub applicability: Vec<String>,
    pub author_id: Option<EntityId>,
    pub approval_status: ValidationStatus,
    pub usage_count: u64,
    pub last_applied: Option<String>,
    pub customization_notes: Vec<String>,
    pub related_knowledge_ids: Vec<EntityId>,
    pub benchmark_ids: Vec<EntityId>,
    pub license: Option<String>,
    pub source_organization: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TemplateRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub template_type: Option<String>,
    pub sector: Option<String>,
    pub project_type: Option<String>,
    pub version: Option<String>,
    pub content_ref: Option<String>,
    pub entity_kinds: Option<Vec<String>>,
    pub default_fields: Option<Vec<String>>,
    pub checklists: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub applicability: Option<Vec<String>>,
    pub author_id: Option<EntityId>,
    pub approval_status: Option<ValidationStatus>,
    pub usage_count: Option<u64>,
    pub last_applied: Option<String>,
    pub customization_notes: Option<Vec<String>>,
    pub related_knowledge_ids: Option<Vec<EntityId>>,
    pub benchmark_ids: Option<Vec<EntityId>>,
    pub license: Option<String>,
    pub source_organization: Option<String>,
}

impl_identified_header!(TemplateRecord);

impl_patchable!(
    TemplateRecord,
    TemplateRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [template_type] => template_type,
        [sector] => sector,
        [project_type] => project_type,
        [version] => version,
        [content_ref] => content_ref,
        [entity_kinds] => entity_kinds,
        [default_fields] => default_fields,
        [checklists] => checklists,
        [standards] => standards,
        [applicability] => applicability,
        [author_id] => author_id,
        [approval_status] => approval_status,
        [usage_count] => usage_count,
        [last_applied] => last_applied,
        [customization_notes] => customization_notes,
        [related_knowledge_ids] => related_knowledge_ids,
        [benchmark_ids] => benchmark_ids,
        [license] => license,
        [source_organization] => source_organization,
    }
);
// #endregion

// #region 🔖️KnowledgeRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct KnowledgeRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub topic: String,
    pub category: String,
    pub summary: TextField,
    pub content: TextField,
    pub sources: Vec<String>,
    pub references: Vec<String>,
    pub lessons_learned: Vec<String>,
    pub best_practices: Vec<String>,
    pub applicable_sectors: Vec<String>,
    pub related_entity_kinds: Vec<String>,
    pub author_ids: Vec<EntityId>,
    pub expertise_level: Option<String>,
    pub validation_status: ValidationStatus,
    pub last_reviewed: Option<String>,
    pub keywords: Vec<String>,
    pub attachments: Vec<EntityId>,
    pub citations: Vec<String>,
    pub usage_count: u64,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct KnowledgeRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub topic: Option<String>,
    pub category: Option<String>,
    pub summary: Option<TextField>,
    pub content: Option<TextField>,
    pub sources: Option<Vec<String>>,
    pub references: Option<Vec<String>>,
    pub lessons_learned: Option<Vec<String>>,
    pub best_practices: Option<Vec<String>>,
    pub applicable_sectors: Option<Vec<String>>,
    pub related_entity_kinds: Option<Vec<String>>,
    pub author_ids: Option<Vec<EntityId>>,
    pub expertise_level: Option<String>,
    pub validation_status: Option<ValidationStatus>,
    pub last_reviewed: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub attachments: Option<Vec<EntityId>>,
    pub citations: Option<Vec<String>>,
    pub usage_count: Option<u64>,
}

impl_identified_header!(KnowledgeRecord);

impl_patchable!(
    KnowledgeRecord,
    KnowledgeRecordPatch,
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
        [category] => category,
        [summary] => summary,
        [content] => content,
        [sources] => sources,
        [references] => references,
        [lessons_learned] => lessons_learned,
        [best_practices] => best_practices,
        [applicable_sectors] => applicable_sectors,
        [related_entity_kinds] => related_entity_kinds,
        [author_ids] => author_ids,
        [expertise_level] => expertise_level,
        [validation_status] => validation_status,
        [last_reviewed] => last_reviewed,
        [keywords] => keywords,
        [attachments] => attachments,
        [citations] => citations,
        [usage_count] => usage_count,
    }
);
// #endregion
