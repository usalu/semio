//! 🧭️ Architect program structure registers: meta, project, stakeholders, users, activities, functions, elements, quantities, relationships and adjacencies; the shared enums live in `kernel` and the patch macros in [`super`].

use super::*;

// #region 🔖️ProgramMeta
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramMeta {
    pub schema: String,
    pub document_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub purpose: TextField,
    pub terminology: Vec<String>,
    pub classification: Vec<String>,
    pub industry_sector: String,
    pub project_type: String,
    pub locale: String,
    pub revision: String,
    pub author_ids: Vec<EntityId>,
    pub source_system: Option<String>,
    pub export_profile: Option<String>,
    pub timestamps: TimestampMeta,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramMetaPatch {
    pub schema: Option<String>,
    pub document_id: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub purpose: Option<TextField>,
    pub terminology: Option<Vec<String>>,
    pub classification: Option<Vec<String>>,
    pub industry_sector: Option<String>,
    pub project_type: Option<String>,
    pub locale: Option<String>,
    pub revision: Option<String>,
    pub author_ids: Option<Vec<EntityId>>,
    pub source_system: Option<String>,
    pub export_profile: Option<String>,
    pub timestamps: Option<TimestampMeta>,
}

impl_patchable!(
    ProgramMeta,
    ProgramMetaPatch,
    {
        [schema] => schema,
        [document_id] => document_id,
        [title] => title,
        [subtitle] => subtitle,
        [purpose] => purpose,
        [terminology] => terminology,
        [classification] => classification,
        [industry_sector] => industry_sector,
        [project_type] => project_type,
        [locale] => locale,
        [revision] => revision,
        [author_ids] => author_ids,
        [source_system] => source_system,
        [export_profile] => export_profile,
        [timestamps] => timestamps,
    }
);
// #endregion

// #region 🔖️ProjectDefinition
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProjectDefinition {
    pub id: EntityId,
    pub code: String,
    pub client_name: String,
    pub owner_organization: String,
    pub brief_summary: TextField,
    pub problem_statement: TextField,
    pub vision: TextField,
    pub mission: TextField,
    pub objectives: Vec<String>,
    pub success_criteria: Vec<String>,
    pub project_priorities: Vec<Priority>,
    pub completion_criteria: Vec<String>,
    pub decision_criteria: Vec<String>,
    pub scope_inclusions: Vec<String>,
    pub scope_exclusions: Vec<String>,
    pub assumptions: Vec<String>,
    pub constraints_summary: Vec<String>,
    pub dependencies: Vec<String>,
    pub deliverables: Vec<String>,
    pub phases: Vec<String>,
    pub geographic_context: TextField,
    pub development_context: TextField,
    pub operational_context: TextField,
    pub regulatory_context: Vec<String>,
    pub funding_model: String,
    pub ownership: Ownership,
    pub timestamps: TimestampMeta,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProjectDefinitionPatch {
    pub id: Option<EntityId>,
    pub code: Option<String>,
    pub client_name: Option<String>,
    pub owner_organization: Option<String>,
    pub brief_summary: Option<TextField>,
    pub problem_statement: Option<TextField>,
    pub vision: Option<TextField>,
    pub mission: Option<TextField>,
    pub objectives: Option<Vec<String>>,
    pub success_criteria: Option<Vec<String>>,
    pub project_priorities: Option<Vec<Priority>>,
    pub completion_criteria: Option<Vec<String>>,
    pub decision_criteria: Option<Vec<String>>,
    pub scope_inclusions: Option<Vec<String>>,
    pub scope_exclusions: Option<Vec<String>>,
    pub assumptions: Option<Vec<String>>,
    pub constraints_summary: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub deliverables: Option<Vec<String>>,
    pub phases: Option<Vec<String>>,
    pub geographic_context: Option<TextField>,
    pub development_context: Option<TextField>,
    pub operational_context: Option<TextField>,
    pub regulatory_context: Option<Vec<String>>,
    pub funding_model: Option<String>,
    pub ownership: Option<Ownership>,
    pub timestamps: Option<TimestampMeta>,
}

impl Identified<EntityId> for ProjectDefinition {
    fn id(&self) -> &EntityId {
        &self.id
    }
}

impl_patchable!(
    ProjectDefinition,
    ProjectDefinitionPatch,
    {
        [id] => id,
        [code] => code,
        [client_name] => client_name,
        [owner_organization] => owner_organization,
        [brief_summary] => brief_summary,
        [problem_statement] => problem_statement,
        [vision] => vision,
        [mission] => mission,
        [objectives] => objectives,
        [success_criteria] => success_criteria,
        [project_priorities] => project_priorities,
        [completion_criteria] => completion_criteria,
        [decision_criteria] => decision_criteria,
        [scope_inclusions] => scope_inclusions,
        [scope_exclusions] => scope_exclusions,
        [assumptions] => assumptions,
        [constraints_summary] => constraints_summary,
        [dependencies] => dependencies,
        [deliverables] => deliverables,
        [phases] => phases,
        [geographic_context] => geographic_context,
        [development_context] => development_context,
        [operational_context] => operational_context,
        [regulatory_context] => regulatory_context,
        [funding_model] => funding_model,
        [ownership] => ownership,
        [timestamps] => timestamps,
    }
);
// #endregion

// #region 🔖️Stakeholder
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Stakeholder {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub role: String,
    pub organization: String,
    pub department: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub influence: InfluenceLevel,
    pub interest: InfluenceLevel,
    pub engagement: EngagementLevel,
    pub expectations: Vec<String>,
    pub concerns: Vec<String>,
    pub requirement_ids: Vec<EntityId>,
    pub decision_authority: bool,
    pub communication_preferences: Vec<String>,
    pub reporting_frequency: Option<String>,
    pub involvement_phases: Vec<String>,
    pub availability: Option<String>,
    pub representative_of: Option<EntityId>,
    pub delegated_to: Option<EntityId>,
    pub relationship_to_client: Option<String>,
    pub power_interest_notes: Vec<TaggedNote>,
    pub stakeholder_type: String,
    pub influence_strategy: Option<String>,
    pub communication_channels: Vec<String>,
    pub success_metrics: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct StakeholderPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub role: Option<String>,
    pub organization: Option<String>,
    pub department: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub influence: Option<InfluenceLevel>,
    pub interest: Option<InfluenceLevel>,
    pub engagement: Option<EngagementLevel>,
    pub expectations: Option<Vec<String>>,
    pub concerns: Option<Vec<String>>,
    pub requirement_ids: Option<Vec<EntityId>>,
    pub decision_authority: Option<bool>,
    pub communication_preferences: Option<Vec<String>>,
    pub reporting_frequency: Option<String>,
    pub involvement_phases: Option<Vec<String>>,
    pub availability: Option<String>,
    pub representative_of: Option<EntityId>,
    pub delegated_to: Option<EntityId>,
    pub relationship_to_client: Option<String>,
    pub power_interest_notes: Option<Vec<TaggedNote>>,
    pub stakeholder_type: Option<String>,
    pub influence_strategy: Option<String>,
    pub communication_channels: Option<Vec<String>>,
    pub success_metrics: Option<Vec<String>>,
}

impl_identified_header!(Stakeholder);

impl_patchable!(
    Stakeholder,
    StakeholderPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [role] => role,
        [organization] => organization,
        [department] => department,
        [contact_email] => contact_email,
        [contact_phone] => contact_phone,
        [influence] => influence,
        [interest] => interest,
        [engagement] => engagement,
        [expectations] => expectations,
        [concerns] => concerns,
        [requirement_ids] => requirement_ids,
        [decision_authority] => decision_authority,
        [communication_preferences] => communication_preferences,
        [reporting_frequency] => reporting_frequency,
        [involvement_phases] => involvement_phases,
        [availability] => availability,
        [representative_of] => representative_of,
        [delegated_to] => delegated_to,
        [relationship_to_client] => relationship_to_client,
        [power_interest_notes] => power_interest_notes,
        [stakeholder_type] => stakeholder_type,
        [influence_strategy] => influence_strategy,
        [communication_channels] => communication_channels,
        [success_metrics] => success_metrics,
    }
);
// #endregion

// #region 🔖️UserProfile
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct UserProfile {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub category: UserCategory,
    pub demographic: Option<String>,
    pub age_range: Option<String>,
    pub abilities: Vec<String>,
    pub disabilities: Vec<String>,
    pub occupation: Option<String>,
    pub role_title: Option<String>,
    pub department: Option<String>,
    pub mobility_profile: Vec<String>,
    pub sensory_profile: Vec<String>,
    pub cognitive_profile: Vec<String>,
    pub behavioral_patterns: Vec<String>,
    pub usage_frequency: Option<String>,
    pub usage_duration: Option<String>,
    pub peak_usage_times: Vec<String>,
    pub technology_proficiency: Option<String>,
    pub preferences: Vec<String>,
    pub pain_points: Vec<String>,
    pub goals: Vec<String>,
    pub activity_ids: Vec<EntityId>,
    pub research_method: Option<String>,
    pub persona_archetype: Option<String>,
    pub validated: bool,
    pub stakeholder_ids: Vec<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct UserProfilePatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub category: Option<UserCategory>,
    pub demographic: Option<String>,
    pub age_range: Option<String>,
    pub abilities: Option<Vec<String>>,
    pub disabilities: Option<Vec<String>>,
    pub occupation: Option<String>,
    pub role_title: Option<String>,
    pub department: Option<String>,
    pub mobility_profile: Option<Vec<String>>,
    pub sensory_profile: Option<Vec<String>>,
    pub cognitive_profile: Option<Vec<String>>,
    pub behavioral_patterns: Option<Vec<String>>,
    pub usage_frequency: Option<String>,
    pub usage_duration: Option<String>,
    pub peak_usage_times: Option<Vec<String>>,
    pub technology_proficiency: Option<String>,
    pub preferences: Option<Vec<String>>,
    pub pain_points: Option<Vec<String>>,
    pub goals: Option<Vec<String>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub research_method: Option<String>,
    pub persona_archetype: Option<String>,
    pub validated: Option<bool>,
    pub stakeholder_ids: Option<Vec<EntityId>>,
}

impl_identified_header!(UserProfile);

impl_patchable!(
    UserProfile,
    UserProfilePatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [category] => category,
        [demographic] => demographic,
        [age_range] => age_range,
        [abilities] => abilities,
        [disabilities] => disabilities,
        [occupation] => occupation,
        [role_title] => role_title,
        [department] => department,
        [mobility_profile] => mobility_profile,
        [sensory_profile] => sensory_profile,
        [cognitive_profile] => cognitive_profile,
        [behavioral_patterns] => behavioral_patterns,
        [usage_frequency] => usage_frequency,
        [usage_duration] => usage_duration,
        [peak_usage_times] => peak_usage_times,
        [technology_proficiency] => technology_proficiency,
        [preferences] => preferences,
        [pain_points] => pain_points,
        [goals] => goals,
        [activity_ids] => activity_ids,
        [research_method] => research_method,
        [persona_archetype] => persona_archetype,
        [validated] => validated,
        [stakeholder_ids] => stakeholder_ids,
    }
);
// #endregion

// #region 🔖️Activity
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Activity {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub code: String,
    pub category: String,
    pub frequency: Option<String>,
    pub duration: Option<String>,
    pub intensity: Option<String>,
    pub participants: QuantitySpec,
    pub equipment_ids: Vec<EntityId>,
    pub space_requirements: Vec<String>,
    pub environmental_needs: Vec<String>,
    pub privacy_needs: Vec<String>,
    pub accessibility_needs: Vec<String>,
    pub adjacent_activities: Vec<EntityId>,
    pub sequencing: Vec<String>,
    pub peak_periods: Vec<String>,
    pub workflow_steps: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub user_profile_ids: Vec<EntityId>,
    pub function_ids: Vec<EntityId>,
    pub performance_indicators: Vec<String>,
    pub activity_type: String,
    pub location_context: Option<String>,
    pub temporal_pattern: Option<String>,
    pub supervision_level: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ActivityPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub code: Option<String>,
    pub category: Option<String>,
    pub frequency: Option<String>,
    pub duration: Option<String>,
    pub intensity: Option<String>,
    pub participants: Option<QuantitySpec>,
    pub equipment_ids: Option<Vec<EntityId>>,
    pub space_requirements: Option<Vec<String>>,
    pub environmental_needs: Option<Vec<String>>,
    pub privacy_needs: Option<Vec<String>>,
    pub accessibility_needs: Option<Vec<String>>,
    pub adjacent_activities: Option<Vec<EntityId>>,
    pub sequencing: Option<Vec<String>>,
    pub peak_periods: Option<Vec<String>>,
    pub workflow_steps: Option<Vec<String>>,
    pub inputs: Option<Vec<String>>,
    pub outputs: Option<Vec<String>>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub function_ids: Option<Vec<EntityId>>,
    pub performance_indicators: Option<Vec<String>>,
    pub activity_type: Option<String>,
    pub location_context: Option<String>,
    pub temporal_pattern: Option<String>,
    pub supervision_level: Option<String>,
}

impl_identified_header!(Activity);

impl_patchable!(
    Activity,
    ActivityPatch,
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
        [category] => category,
        [frequency] => frequency,
        [duration] => duration,
        [intensity] => intensity,
        [participants] => participants,
        [equipment_ids] => equipment_ids,
        [space_requirements] => space_requirements,
        [environmental_needs] => environmental_needs,
        [privacy_needs] => privacy_needs,
        [accessibility_needs] => accessibility_needs,
        [adjacent_activities] => adjacent_activities,
        [sequencing] => sequencing,
        [peak_periods] => peak_periods,
        [workflow_steps] => workflow_steps,
        [inputs] => inputs,
        [outputs] => outputs,
        [user_profile_ids] => user_profile_ids,
        [function_ids] => function_ids,
        [performance_indicators] => performance_indicators,
        [activity_type] => activity_type,
        [location_context] => location_context,
        [temporal_pattern] => temporal_pattern,
        [supervision_level] => supervision_level,
    }
);
// #endregion

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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
