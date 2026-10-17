//! 🧾️ Benchmark, assumption, constraint, compliance, approval and meeting records plus governance.

use super::*;

// #region 🔖️BenchmarkRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BenchmarkRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub benchmark_name: String,
    pub sector: String,
    pub metric: String,
    pub value: f64,
    pub unit: String,
    pub sample_size: Option<u32>,
    pub source: Option<String>,
    pub collection_year: Option<u32>,
    pub geography: Option<String>,
    pub building_type: Option<String>,
    pub confidence: Option<String>,
    pub methodology: Option<String>,
    pub applicable_element_kinds: Vec<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub comparison_notes: Vec<String>,
    pub limitations: Vec<String>,
    pub license: Option<String>,
    pub knowledge_id: Option<EntityId>,
    pub last_verified: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BenchmarkRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub benchmark_name: Option<String>,
    pub sector: Option<String>,
    pub metric: Option<String>,
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub sample_size: Option<u32>,
    pub source: Option<String>,
    pub collection_year: Option<u32>,
    pub geography: Option<String>,
    pub building_type: Option<String>,
    pub confidence: Option<String>,
    pub methodology: Option<String>,
    pub applicable_element_kinds: Option<Vec<String>>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub comparison_notes: Option<Vec<String>>,
    pub limitations: Option<Vec<String>>,
    pub license: Option<String>,
    pub knowledge_id: Option<EntityId>,
    pub last_verified: Option<String>,
}

impl_identified_header!(BenchmarkRecord);

impl_patchable!(
    BenchmarkRecord,
    BenchmarkRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [benchmark_name] => benchmark_name,
        [sector] => sector,
        [metric] => metric,
        [value] => value,
        [unit] => unit,
        [sample_size] => sample_size,
        [source] => source,
        [collection_year] => collection_year,
        [geography] => geography,
        [building_type] => building_type,
        [confidence] => confidence,
        [methodology] => methodology,
        [applicable_element_kinds] => applicable_element_kinds,
        [related_requirement_ids] => related_requirement_ids,
        [comparison_notes] => comparison_notes,
        [limitations] => limitations,
        [license] => license,
        [knowledge_id] => knowledge_id,
        [last_verified] => last_verified,
    }
);
// #endregion

// #region 🔖️Assumption
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Assumption {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub statement: TextField,
    pub basis: Option<TextField>,
    pub confidence_level: Option<String>,
    pub impact_if_false: Option<TextField>,
    pub related_entity_ids: Vec<EntityId>,
    pub validation_status: ValidationStatus,
    pub validated_by: Option<EntityId>,
    pub validation_date: Option<String>,
    pub owner_id: Option<EntityId>,
    pub review_cycle: Option<String>,
    pub source: Option<String>,
    pub category: Option<String>,
    pub dependencies: Vec<String>,
    pub mitigation: Vec<String>,
    pub linked_requirement_ids: Vec<EntityId>,
    pub linked_risk_ids: Vec<EntityId>,
    pub expiration_date: Option<String>,
    pub status_notes: Vec<TaggedNote>,
    pub artifact_refs: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AssumptionPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub statement: Option<TextField>,
    pub basis: Option<TextField>,
    pub confidence_level: Option<String>,
    pub impact_if_false: Option<TextField>,
    pub related_entity_ids: Option<Vec<EntityId>>,
    pub validation_status: Option<ValidationStatus>,
    pub validated_by: Option<EntityId>,
    pub validation_date: Option<String>,
    pub owner_id: Option<EntityId>,
    pub review_cycle: Option<String>,
    pub source: Option<String>,
    pub category: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub mitigation: Option<Vec<String>>,
    pub linked_requirement_ids: Option<Vec<EntityId>>,
    pub linked_risk_ids: Option<Vec<EntityId>>,
    pub expiration_date: Option<String>,
    pub status_notes: Option<Vec<TaggedNote>>,
    pub artifact_refs: Option<Vec<String>>,
}

impl_identified_header!(Assumption);

impl_patchable!(
    Assumption,
    AssumptionPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [statement] => statement,
        [basis] => basis,
        [confidence_level] => confidence_level,
        [impact_if_false] => impact_if_false,
        [related_entity_ids] => related_entity_ids,
        [validation_status] => validation_status,
        [validated_by] => validated_by,
        [validation_date] => validation_date,
        [owner_id] => owner_id,
        [review_cycle] => review_cycle,
        [source] => source,
        [category] => category,
        [dependencies] => dependencies,
        [mitigation] => mitigation,
        [linked_requirement_ids] => linked_requirement_ids,
        [linked_risk_ids] => linked_risk_ids,
        [expiration_date] => expiration_date,
        [status_notes] => status_notes,
        [artifact_refs] => artifact_refs,
    }
);
// #endregion

// #region 🔖️ConstraintRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ConstraintRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub constraint_type: String,
    pub summary: TextField,
    pub severity: RiskLevel,
    pub affected_entity_ids: Vec<EntityId>,
    pub source: Option<String>,
    pub regulatory_basis: Vec<String>,
    pub mitigation_options: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub effective_date: Option<String>,
    pub expiry_date: Option<String>,
    pub waiver_status: Option<String>,
    pub waiver_approver: Option<EntityId>,
    pub impact_assessment: Option<TextField>,
    pub resolution_plan: Vec<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub related_decision_ids: Vec<EntityId>,
    pub monitoring_frequency: Option<String>,
    pub compliance_status: ValidationStatus,
    pub exceptions: Vec<String>,
    pub trace_links: Vec<TraceLink>,
    pub escalation_contact_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ConstraintRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub constraint_type: Option<String>,
    pub summary: Option<TextField>,
    pub severity: Option<RiskLevel>,
    pub affected_entity_ids: Option<Vec<EntityId>>,
    pub source: Option<String>,
    pub regulatory_basis: Option<Vec<String>>,
    pub mitigation_options: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub effective_date: Option<String>,
    pub expiry_date: Option<String>,
    pub waiver_status: Option<String>,
    pub waiver_approver: Option<EntityId>,
    pub impact_assessment: Option<TextField>,
    pub resolution_plan: Option<Vec<String>>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub related_decision_ids: Option<Vec<EntityId>>,
    pub monitoring_frequency: Option<String>,
    pub compliance_status: Option<ValidationStatus>,
    pub exceptions: Option<Vec<String>>,
    pub trace_links: Option<Vec<TraceLink>>,
    pub escalation_contact_id: Option<EntityId>,
}

impl_identified_header!(ConstraintRecord);

impl_patchable!(
    ConstraintRecord,
    ConstraintRecordPatch,
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
        [summary] => summary,
        [severity] => severity,
        [affected_entity_ids] => affected_entity_ids,
        [source] => source,
        [regulatory_basis] => regulatory_basis,
        [mitigation_options] => mitigation_options,
        [owner_id] => owner_id,
        [effective_date] => effective_date,
        [expiry_date] => expiry_date,
        [waiver_status] => waiver_status,
        [waiver_approver] => waiver_approver,
        [impact_assessment] => impact_assessment,
        [resolution_plan] => resolution_plan,
        [related_requirement_ids] => related_requirement_ids,
        [related_decision_ids] => related_decision_ids,
        [monitoring_frequency] => monitoring_frequency,
        [compliance_status] => compliance_status,
        [exceptions] => exceptions,
        [trace_links] => trace_links,
        [escalation_contact_id] => escalation_contact_id,
    }
);
// #endregion

// #region 🔖️ComplianceRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ComplianceRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub standard_ref: String,
    pub obligation: TextField,
    pub compliance_status: ValidationStatus,
    pub evidence_refs: Vec<String>,
    pub auditor_id: Option<EntityId>,
    pub audit_date: Option<String>,
    pub next_review: Option<String>,
    pub affected_entity_ids: Vec<EntityId>,
    pub gap_analysis: Vec<String>,
    pub remediation_plan: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub severity: RiskLevel,
    pub regulatory_body: Option<String>,
    pub certification_target: Option<String>,
    pub waiver_status: Option<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub monitoring_method: Option<String>,
    pub reporting_frequency: Option<String>,
    pub penalties: Vec<String>,
    pub corrective_actions: Vec<String>,
    pub artifact_refs: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ComplianceRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub standard_ref: Option<String>,
    pub obligation: Option<TextField>,
    pub compliance_status: Option<ValidationStatus>,
    pub evidence_refs: Option<Vec<String>>,
    pub auditor_id: Option<EntityId>,
    pub audit_date: Option<String>,
    pub next_review: Option<String>,
    pub affected_entity_ids: Option<Vec<EntityId>>,
    pub gap_analysis: Option<Vec<String>>,
    pub remediation_plan: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub severity: Option<RiskLevel>,
    pub regulatory_body: Option<String>,
    pub certification_target: Option<String>,
    pub waiver_status: Option<String>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub monitoring_method: Option<String>,
    pub reporting_frequency: Option<String>,
    pub penalties: Option<Vec<String>>,
    pub corrective_actions: Option<Vec<String>>,
    pub artifact_refs: Option<Vec<String>>,
}

impl_identified_header!(ComplianceRecord);

impl_patchable!(
    ComplianceRecord,
    ComplianceRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [standard_ref] => standard_ref,
        [obligation] => obligation,
        [compliance_status] => compliance_status,
        [evidence_refs] => evidence_refs,
        [auditor_id] => auditor_id,
        [audit_date] => audit_date,
        [next_review] => next_review,
        [affected_entity_ids] => affected_entity_ids,
        [gap_analysis] => gap_analysis,
        [remediation_plan] => remediation_plan,
        [owner_id] => owner_id,
        [severity] => severity,
        [regulatory_body] => regulatory_body,
        [certification_target] => certification_target,
        [waiver_status] => waiver_status,
        [related_requirement_ids] => related_requirement_ids,
        [monitoring_method] => monitoring_method,
        [reporting_frequency] => reporting_frequency,
        [penalties] => penalties,
        [corrective_actions] => corrective_actions,
        [artifact_refs] => artifact_refs,
    }
);
// #endregion

// #region 🔖️ApprovalRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ApprovalRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub approval_type: String,
    pub subject_id: EntityId,
    pub approver_ids: Vec<EntityId>,
    pub approval_date: Option<String>,
    pub conditions: Vec<String>,
    pub approval_status: LifecycleStatus,
    pub expiry_date: Option<String>,
    pub delegation_chain: Vec<EntityId>,
    pub evidence_refs: Vec<String>,
    pub related_decision_id: Option<EntityId>,
    pub related_change_id: Option<EntityId>,
    pub authority_basis: Vec<String>,
    pub signature_method: Option<String>,
    pub rejection_reason: Option<TextField>,
    pub resubmission_date: Option<String>,
    pub notification_list: Vec<EntityId>,
    pub workflow_step: Option<String>,
    pub version: Option<String>,
    pub audit_trail_ref: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ApprovalRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub approval_type: Option<String>,
    pub subject_id: Option<EntityId>,
    pub approver_ids: Option<Vec<EntityId>>,
    pub approval_date: Option<String>,
    pub conditions: Option<Vec<String>>,
    pub approval_status: Option<LifecycleStatus>,
    pub expiry_date: Option<String>,
    pub delegation_chain: Option<Vec<EntityId>>,
    pub evidence_refs: Option<Vec<String>>,
    pub related_decision_id: Option<EntityId>,
    pub related_change_id: Option<EntityId>,
    pub authority_basis: Option<Vec<String>>,
    pub signature_method: Option<String>,
    pub rejection_reason: Option<TextField>,
    pub resubmission_date: Option<String>,
    pub notification_list: Option<Vec<EntityId>>,
    pub workflow_step: Option<String>,
    pub version: Option<String>,
    pub audit_trail_ref: Option<String>,
}

impl_identified_header!(ApprovalRecord);

impl_patchable!(
    ApprovalRecord,
    ApprovalRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [approval_type] => approval_type,
        [subject_id] => subject_id,
        [approver_ids] => approver_ids,
        [approval_date] => approval_date,
        [conditions] => conditions,
        [approval_status] => approval_status,
        [expiry_date] => expiry_date,
        [delegation_chain] => delegation_chain,
        [evidence_refs] => evidence_refs,
        [related_decision_id] => related_decision_id,
        [related_change_id] => related_change_id,
        [authority_basis] => authority_basis,
        [signature_method] => signature_method,
        [rejection_reason] => rejection_reason,
        [resubmission_date] => resubmission_date,
        [notification_list] => notification_list,
        [workflow_step] => workflow_step,
        [version] => version,
        [audit_trail_ref] => audit_trail_ref,
    }
);
// #endregion

// #region 🔖️MeetingRecord
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct MeetingRecord {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub meeting_type: String,
    pub scheduled_date: Option<String>,
    pub duration: Option<String>,
    pub location: Option<String>,
    pub chair_id: Option<EntityId>,
    pub attendee_ids: Vec<EntityId>,
    pub agenda_items: Vec<String>,
    pub minutes: Option<TextField>,
    pub action_items: Vec<String>,
    pub decisions_made: Vec<EntityId>,
    pub artifact_refs: Vec<String>,
    pub follow_up_date: Option<String>,
    pub recording_ref: Option<String>,
    pub quorum_met: bool,
    pub meeting_status: LifecycleStatus,
    pub workshop_id: Option<EntityId>,
    pub stakeholder_ids: Vec<EntityId>,
    pub requirement_ids: Vec<EntityId>,
    pub issue_ids: Vec<EntityId>,
    pub approval_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct MeetingRecordPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub meeting_type: Option<String>,
    pub scheduled_date: Option<String>,
    pub duration: Option<String>,
    pub location: Option<String>,
    pub chair_id: Option<EntityId>,
    pub attendee_ids: Option<Vec<EntityId>>,
    pub agenda_items: Option<Vec<String>>,
    pub minutes: Option<TextField>,
    pub action_items: Option<Vec<String>>,
    pub decisions_made: Option<Vec<EntityId>>,
    pub artifact_refs: Option<Vec<String>>,
    pub follow_up_date: Option<String>,
    pub recording_ref: Option<String>,
    pub quorum_met: Option<bool>,
    pub meeting_status: Option<LifecycleStatus>,
    pub workshop_id: Option<EntityId>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub issue_ids: Option<Vec<EntityId>>,
    pub approval_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(MeetingRecord);

impl_patchable!(
    MeetingRecord,
    MeetingRecordPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [meeting_type] => meeting_type,
        [scheduled_date] => scheduled_date,
        [duration] => duration,
        [location] => location,
        [chair_id] => chair_id,
        [attendee_ids] => attendee_ids,
        [agenda_items] => agenda_items,
        [minutes] => minutes,
        [action_items] => action_items,
        [decisions_made] => decisions_made,
        [artifact_refs] => artifact_refs,
        [follow_up_date] => follow_up_date,
        [recording_ref] => recording_ref,
        [quorum_met] => quorum_met,
        [meeting_status] => meeting_status,
        [workshop_id] => workshop_id,
        [stakeholder_ids] => stakeholder_ids,
        [requirement_ids] => requirement_ids,
        [issue_ids] => issue_ids,
        [approval_ids] => approval_ids,
    }
);
// #endregion

// #region 🔖️Governance
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Governance {
    pub id: EntityId,
    pub framework: String,
    pub roles: Vec<String>,
    pub responsibilities: Vec<String>,
    pub approval_matrix: Vec<String>,
    pub escalation_paths: Vec<String>,
    pub meeting_cadence: Vec<String>,
    pub decision_rights: Vec<String>,
    pub change_control_process: Vec<String>,
    pub quality_policy: TextField,
    pub risk_appetite: Option<String>,
    pub compliance_obligations: Vec<String>,
    pub audit_schedule: Option<String>,
    pub document_control: Vec<String>,
    pub stakeholder_engagement_plan: Vec<String>,
    pub ethics_policy: Vec<String>,
    pub data_governance: Vec<String>,
    pub owner_id: Option<EntityId>,
    pub review_cycle: Option<String>,
    pub review_hierarchy: Vec<String>,
    pub policy_ownership_id: Option<EntityId>,
    pub requirement_ownership_id: Option<EntityId>,
    pub risk_ownership_id: Option<EntityId>,
    pub reporting_frequency: Option<String>,
    pub accountability_rules: Vec<String>,
    pub exception_management: Vec<String>,
    pub governance_performance: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct GovernancePatch {
    pub id: Option<EntityId>,
    pub framework: Option<String>,
    pub roles: Option<Vec<String>>,
    pub responsibilities: Option<Vec<String>>,
    pub approval_matrix: Option<Vec<String>>,
    pub escalation_paths: Option<Vec<String>>,
    pub meeting_cadence: Option<Vec<String>>,
    pub decision_rights: Option<Vec<String>>,
    pub change_control_process: Option<Vec<String>>,
    pub quality_policy: Option<TextField>,
    pub risk_appetite: Option<String>,
    pub compliance_obligations: Option<Vec<String>>,
    pub audit_schedule: Option<String>,
    pub document_control: Option<Vec<String>>,
    pub stakeholder_engagement_plan: Option<Vec<String>>,
    pub ethics_policy: Option<Vec<String>>,
    pub data_governance: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
    pub review_cycle: Option<String>,
    pub review_hierarchy: Option<Vec<String>>,
    pub policy_ownership_id: Option<EntityId>,
    pub requirement_ownership_id: Option<EntityId>,
    pub risk_ownership_id: Option<EntityId>,
    pub reporting_frequency: Option<String>,
    pub accountability_rules: Option<Vec<String>>,
    pub exception_management: Option<Vec<String>>,
    pub governance_performance: Option<Vec<String>>,
}

impl Identified<EntityId> for Governance {
    fn id(&self) -> &EntityId {
        &self.id
    }
}

impl_patchable!(
    Governance,
    GovernancePatch,
    {
        [id] => id,
        [framework] => framework,
        [roles] => roles,
        [responsibilities] => responsibilities,
        [approval_matrix] => approval_matrix,
        [escalation_paths] => escalation_paths,
        [meeting_cadence] => meeting_cadence,
        [decision_rights] => decision_rights,
        [change_control_process] => change_control_process,
        [quality_policy] => quality_policy,
        [risk_appetite] => risk_appetite,
        [compliance_obligations] => compliance_obligations,
        [audit_schedule] => audit_schedule,
        [document_control] => document_control,
        [stakeholder_engagement_plan] => stakeholder_engagement_plan,
        [ethics_policy] => ethics_policy,
        [data_governance] => data_governance,
        [owner_id] => owner_id,
        [review_cycle] => review_cycle,
        [review_hierarchy] => review_hierarchy,
        [policy_ownership_id] => policy_ownership_id,
        [requirement_ownership_id] => requirement_ownership_id,
        [risk_ownership_id] => risk_ownership_id,
        [reporting_frequency] => reporting_frequency,
        [accountability_rules] => accountability_rules,
        [exception_management] => exception_management,
        [governance_performance] => governance_performance,
    }
);
// #endregion
