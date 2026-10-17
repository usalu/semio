//! ⚠️ Delivery constraints, risks and conflicts.

use super::*;

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
