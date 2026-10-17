//! 🏗️ Program meta, project definition, stakeholders, user profiles and activities.

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
