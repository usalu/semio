//! 🛡️ Environmental, human factor, accessibility, privacy, safety, security and regulatory requirements.

use super::*;

// #region 🔖️EnvironmentalRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct EnvironmentalRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub parameter_kind: EnvironmentalParameter,
    pub parameter: String,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub comfort_band: Option<String>,
    pub measurement_method: Option<String>,
    pub monitoring_frequency: Option<String>,
    pub element_ids: Vec<EntityId>,
    pub occupancy_basis: Option<String>,
    pub seasonal_variation: Vec<String>,
    pub energy_implications: Vec<String>,
    pub standards: Vec<String>,
    pub certification_targets: Vec<String>,
    pub outdoor_conditions: Vec<String>,
    pub ventilation_strategy: Option<String>,
    pub daylight_target: Option<String>,
    pub acoustic_target: Option<String>,
    pub iaq_target: Option<String>,
    pub verification_plan: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct EnvironmentalRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub parameter_kind: Option<EnvironmentalParameter>,
    pub parameter: Option<String>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub comfort_band: Option<String>,
    pub measurement_method: Option<String>,
    pub monitoring_frequency: Option<String>,
    pub element_ids: Option<Vec<EntityId>>,
    pub occupancy_basis: Option<String>,
    pub seasonal_variation: Option<Vec<String>>,
    pub energy_implications: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub certification_targets: Option<Vec<String>>,
    pub outdoor_conditions: Option<Vec<String>>,
    pub ventilation_strategy: Option<String>,
    pub daylight_target: Option<String>,
    pub acoustic_target: Option<String>,
    pub iaq_target: Option<String>,
    pub verification_plan: Option<String>,
}

impl_identified_header!(EnvironmentalRequirement);

impl_patchable!(
    EnvironmentalRequirement,
    EnvironmentalRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [parameter_kind] => parameter_kind,
        [parameter] => parameter,
        [target_value] => target_value,
        [unit] => unit,
        [min_value] => min_value,
        [max_value] => max_value,
        [comfort_band] => comfort_band,
        [measurement_method] => measurement_method,
        [monitoring_frequency] => monitoring_frequency,
        [element_ids] => element_ids,
        [occupancy_basis] => occupancy_basis,
        [seasonal_variation] => seasonal_variation,
        [energy_implications] => energy_implications,
        [standards] => standards,
        [certification_targets] => certification_targets,
        [outdoor_conditions] => outdoor_conditions,
        [ventilation_strategy] => ventilation_strategy,
        [daylight_target] => daylight_target,
        [acoustic_target] => acoustic_target,
        [iaq_target] => iaq_target,
        [verification_plan] => verification_plan,
    }
);
// #endregion

// #region 🔖️HumanFactorRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct HumanFactorRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub aspect: HumanFactorAspect,
    pub factor: String,
    pub user_profile_ids: Vec<EntityId>,
    pub activity_ids: Vec<EntityId>,
    pub ergonomic_criteria: Vec<String>,
    pub cognitive_load: Option<String>,
    pub visual_demands: Vec<String>,
    pub auditory_demands: Vec<String>,
    pub posture_requirements: Vec<String>,
    pub reach_envelope: Option<String>,
    pub lighting_for_tasks: Vec<String>,
    pub thermal_comfort: Vec<String>,
    pub privacy_needs: Vec<String>,
    pub social_interaction: Vec<String>,
    pub stress_factors: Vec<String>,
    pub mitigation_measures: Vec<String>,
    pub training_needs: Vec<String>,
    pub standards: Vec<String>,
    pub research_basis: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub verification_method: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct HumanFactorRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub aspect: Option<HumanFactorAspect>,
    pub factor: Option<String>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub activity_ids: Option<Vec<EntityId>>,
    pub ergonomic_criteria: Option<Vec<String>>,
    pub cognitive_load: Option<String>,
    pub visual_demands: Option<Vec<String>>,
    pub auditory_demands: Option<Vec<String>>,
    pub posture_requirements: Option<Vec<String>>,
    pub reach_envelope: Option<String>,
    pub lighting_for_tasks: Option<Vec<String>>,
    pub thermal_comfort: Option<Vec<String>>,
    pub privacy_needs: Option<Vec<String>>,
    pub social_interaction: Option<Vec<String>>,
    pub stress_factors: Option<Vec<String>>,
    pub mitigation_measures: Option<Vec<String>>,
    pub training_needs: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub research_basis: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub verification_method: Option<String>,
}

impl_identified_header!(HumanFactorRequirement);

impl_patchable!(
    HumanFactorRequirement,
    HumanFactorRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [aspect] => aspect,
        [factor] => factor,
        [user_profile_ids] => user_profile_ids,
        [activity_ids] => activity_ids,
        [ergonomic_criteria] => ergonomic_criteria,
        [cognitive_load] => cognitive_load,
        [visual_demands] => visual_demands,
        [auditory_demands] => auditory_demands,
        [posture_requirements] => posture_requirements,
        [reach_envelope] => reach_envelope,
        [lighting_for_tasks] => lighting_for_tasks,
        [thermal_comfort] => thermal_comfort,
        [privacy_needs] => privacy_needs,
        [social_interaction] => social_interaction,
        [stress_factors] => stress_factors,
        [mitigation_measures] => mitigation_measures,
        [training_needs] => training_needs,
        [standards] => standards,
        [research_basis] => research_basis,
        [element_ids] => element_ids,
        [verification_method] => verification_method,
    }
);
// #endregion

// #region 🔖️AccessibilityRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AccessibilityRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub standard: String,
    pub level: Option<String>,
    pub user_profile_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub route_ids: Vec<EntityId>,
    #[dsl(unit = "m")]
    pub clear_width_m: Option<f64>,
    #[dsl(unit = "m")]
    pub clear_height_m: Option<f64>,
    #[dsl(unit = "m")]
    pub turning_circle_m: Option<f64>,
    pub ramp_slope: Option<f64>,
    pub lift_required: bool,
    pub tactile_guidance: bool,
    pub hearing_loop: bool,
    pub visual_contrast: bool,
    pub signage_requirements: Vec<String>,
    pub controls_height: Option<String>,
    pub emergency_evacuation: Vec<String>,
    pub service_animal_policy: Option<String>,
    pub companion_seating: bool,
    pub verification_plan: Option<String>,
    pub exceptions: Vec<String>,
    pub wcag_conformance: Option<String>,
    pub universal_design_principles: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AccessibilityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub standard: Option<String>,
    pub level: Option<String>,
    pub user_profile_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub route_ids: Option<Vec<EntityId>>,
    pub clear_width_m: Option<f64>,
    pub clear_height_m: Option<f64>,
    pub turning_circle_m: Option<f64>,
    pub ramp_slope: Option<f64>,
    pub lift_required: Option<bool>,
    pub tactile_guidance: Option<bool>,
    pub hearing_loop: Option<bool>,
    pub visual_contrast: Option<bool>,
    pub signage_requirements: Option<Vec<String>>,
    pub controls_height: Option<String>,
    pub emergency_evacuation: Option<Vec<String>>,
    pub service_animal_policy: Option<String>,
    pub companion_seating: Option<bool>,
    pub verification_plan: Option<String>,
    pub exceptions: Option<Vec<String>>,
    pub wcag_conformance: Option<String>,
    pub universal_design_principles: Option<Vec<String>>,
}

impl_identified_header!(AccessibilityRequirement);

impl_patchable!(
    AccessibilityRequirement,
    AccessibilityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [standard] => standard,
        [level] => level,
        [user_profile_ids] => user_profile_ids,
        [element_ids] => element_ids,
        [route_ids] => route_ids,
        [clear_width_m] => clear_width_m,
        [clear_height_m] => clear_height_m,
        [turning_circle_m] => turning_circle_m,
        [ramp_slope] => ramp_slope,
        [lift_required] => lift_required,
        [tactile_guidance] => tactile_guidance,
        [hearing_loop] => hearing_loop,
        [visual_contrast] => visual_contrast,
        [signage_requirements] => signage_requirements,
        [controls_height] => controls_height,
        [emergency_evacuation] => emergency_evacuation,
        [service_animal_policy] => service_animal_policy,
        [companion_seating] => companion_seating,
        [verification_plan] => verification_plan,
        [exceptions] => exceptions,
        [wcag_conformance] => wcag_conformance,
        [universal_design_principles] => universal_design_principles,
    }
);
// #endregion

// #region 🔖️PrivacyRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PrivacyRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub privacy_kind: PrivacyKind,
    pub privacy_type: String,
    pub level: Option<String>,
    pub subject_ids: Vec<EntityId>,
    pub element_ids: Vec<EntityId>,
    pub visual_privacy: Vec<String>,
    pub acoustic_privacy: Vec<String>,
    pub data_privacy: Vec<String>,
    pub screening_required: bool,
    pub enclosure_required: bool,
    pub access_restrictions: Vec<String>,
    pub observation_risk: Option<String>,
    pub regulatory_basis: Vec<String>,
    pub cultural_considerations: Vec<String>,
    pub technology_controls: Vec<String>,
    pub signage: Vec<String>,
    pub monitoring_restrictions: Vec<String>,
    pub retention_policy: Option<String>,
    pub breach_response: Vec<String>,
    pub owner_id: Option<EntityId>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PrivacyRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub privacy_kind: Option<PrivacyKind>,
    pub privacy_type: Option<String>,
    pub level: Option<String>,
    pub subject_ids: Option<Vec<EntityId>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub visual_privacy: Option<Vec<String>>,
    pub acoustic_privacy: Option<Vec<String>>,
    pub data_privacy: Option<Vec<String>>,
    pub screening_required: Option<bool>,
    pub enclosure_required: Option<bool>,
    pub access_restrictions: Option<Vec<String>>,
    pub observation_risk: Option<String>,
    pub regulatory_basis: Option<Vec<String>>,
    pub cultural_considerations: Option<Vec<String>>,
    pub technology_controls: Option<Vec<String>>,
    pub signage: Option<Vec<String>>,
    pub monitoring_restrictions: Option<Vec<String>>,
    pub retention_policy: Option<String>,
    pub breach_response: Option<Vec<String>>,
    pub owner_id: Option<EntityId>,
}

impl_identified_header!(PrivacyRequirement);

impl_patchable!(
    PrivacyRequirement,
    PrivacyRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [privacy_kind] => privacy_kind,
        [privacy_type] => privacy_type,
        [level] => level,
        [subject_ids] => subject_ids,
        [element_ids] => element_ids,
        [visual_privacy] => visual_privacy,
        [acoustic_privacy] => acoustic_privacy,
        [data_privacy] => data_privacy,
        [screening_required] => screening_required,
        [enclosure_required] => enclosure_required,
        [access_restrictions] => access_restrictions,
        [observation_risk] => observation_risk,
        [regulatory_basis] => regulatory_basis,
        [cultural_considerations] => cultural_considerations,
        [technology_controls] => technology_controls,
        [signage] => signage,
        [monitoring_restrictions] => monitoring_restrictions,
        [retention_policy] => retention_policy,
        [breach_response] => breach_response,
        [owner_id] => owner_id,
    }
);
// #endregion

// #region 🔖️SafetyRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SafetyRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub safety_domain: SafetyDomain,
    pub hazard: String,
    pub risk_level: RiskLevel,
    pub affected_element_ids: Vec<EntityId>,
    pub affected_user_ids: Vec<EntityId>,
    pub mitigation_measures: Vec<String>,
    pub ppe_requirements: Vec<String>,
    pub emergency_procedures: Vec<String>,
    pub evacuation_requirements: Vec<String>,
    pub fire_protection: Vec<String>,
    pub structural_safety: Vec<String>,
    pub slip_trip_fall: Vec<String>,
    pub chemical_safety: Vec<String>,
    pub electrical_safety: Vec<String>,
    pub machinery_safety: Vec<String>,
    pub standards: Vec<String>,
    pub inspection_frequency: Option<String>,
    pub training_requirements: Vec<String>,
    pub incident_reporting: Vec<String>,
    pub residual_risk: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SafetyRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub safety_domain: Option<SafetyDomain>,
    pub hazard: Option<String>,
    pub risk_level: Option<RiskLevel>,
    pub affected_element_ids: Option<Vec<EntityId>>,
    pub affected_user_ids: Option<Vec<EntityId>>,
    pub mitigation_measures: Option<Vec<String>>,
    pub ppe_requirements: Option<Vec<String>>,
    pub emergency_procedures: Option<Vec<String>>,
    pub evacuation_requirements: Option<Vec<String>>,
    pub fire_protection: Option<Vec<String>>,
    pub structural_safety: Option<Vec<String>>,
    pub slip_trip_fall: Option<Vec<String>>,
    pub chemical_safety: Option<Vec<String>>,
    pub electrical_safety: Option<Vec<String>>,
    pub machinery_safety: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub inspection_frequency: Option<String>,
    pub training_requirements: Option<Vec<String>>,
    pub incident_reporting: Option<Vec<String>>,
    pub residual_risk: Option<String>,
}

impl_identified_header!(SafetyRequirement);

impl_patchable!(
    SafetyRequirement,
    SafetyRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [safety_domain] => safety_domain,
        [hazard] => hazard,
        [risk_level] => risk_level,
        [affected_element_ids] => affected_element_ids,
        [affected_user_ids] => affected_user_ids,
        [mitigation_measures] => mitigation_measures,
        [ppe_requirements] => ppe_requirements,
        [emergency_procedures] => emergency_procedures,
        [evacuation_requirements] => evacuation_requirements,
        [fire_protection] => fire_protection,
        [structural_safety] => structural_safety,
        [slip_trip_fall] => slip_trip_fall,
        [chemical_safety] => chemical_safety,
        [electrical_safety] => electrical_safety,
        [machinery_safety] => machinery_safety,
        [standards] => standards,
        [inspection_frequency] => inspection_frequency,
        [training_requirements] => training_requirements,
        [incident_reporting] => incident_reporting,
        [residual_risk] => residual_risk,
    }
);
// #endregion

// #region 🔖️SecurityRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SecurityRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub control_kind: SecurityControlKind,
    pub threat: String,
    pub risk_level: RiskLevel,
    pub asset_ids: Vec<EntityId>,
    pub zone_ids: Vec<EntityId>,
    pub access_level: AccessLevel,
    pub perimeter_controls: Vec<String>,
    pub surveillance: Vec<String>,
    pub intrusion_detection: Vec<String>,
    pub cybersecurity: Vec<String>,
    pub screening: Vec<String>,
    pub visitor_management: Vec<String>,
    pub key_management: Vec<String>,
    pub standards: Vec<String>,
    pub response_procedures: Vec<String>,
    pub drill_frequency: Option<String>,
    pub liaison_contacts: Vec<String>,
    pub classified_level: Option<String>,
    pub redundancy: Vec<String>,
    pub audit_requirements: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SecurityRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub control_kind: Option<SecurityControlKind>,
    pub threat: Option<String>,
    pub risk_level: Option<RiskLevel>,
    pub asset_ids: Option<Vec<EntityId>>,
    pub zone_ids: Option<Vec<EntityId>>,
    pub access_level: Option<AccessLevel>,
    pub perimeter_controls: Option<Vec<String>>,
    pub surveillance: Option<Vec<String>>,
    pub intrusion_detection: Option<Vec<String>>,
    pub cybersecurity: Option<Vec<String>>,
    pub screening: Option<Vec<String>>,
    pub visitor_management: Option<Vec<String>>,
    pub key_management: Option<Vec<String>>,
    pub standards: Option<Vec<String>>,
    pub response_procedures: Option<Vec<String>>,
    pub drill_frequency: Option<String>,
    pub liaison_contacts: Option<Vec<String>>,
    pub classified_level: Option<String>,
    pub redundancy: Option<Vec<String>>,
    pub audit_requirements: Option<Vec<String>>,
}

impl_identified_header!(SecurityRequirement);

impl_patchable!(
    SecurityRequirement,
    SecurityRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [control_kind] => control_kind,
        [threat] => threat,
        [risk_level] => risk_level,
        [asset_ids] => asset_ids,
        [zone_ids] => zone_ids,
        [access_level] => access_level,
        [perimeter_controls] => perimeter_controls,
        [surveillance] => surveillance,
        [intrusion_detection] => intrusion_detection,
        [cybersecurity] => cybersecurity,
        [screening] => screening,
        [visitor_management] => visitor_management,
        [key_management] => key_management,
        [standards] => standards,
        [response_procedures] => response_procedures,
        [drill_frequency] => drill_frequency,
        [liaison_contacts] => liaison_contacts,
        [classified_level] => classified_level,
        [redundancy] => redundancy,
        [audit_requirements] => audit_requirements,
    }
);
// #endregion

// #region 🔖️RegulatoryRequirement
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct RegulatoryRequirement {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    pub header: EntityHeader,
    pub jurisdiction: String,
    pub code: String,
    pub clause: Option<String>,
    pub title: String,
    pub requirement_text: TextField,
    pub applicability: Vec<String>,
    pub element_ids: Vec<EntityId>,
    pub compliance_method: Option<String>,
    pub evidence_required: Vec<String>,
    pub authority: Option<String>,
    pub effective_date: Option<String>,
    pub expiry_date: Option<String>,
    pub penalties: Vec<String>,
    pub exemptions: Vec<String>,
    pub related_requirement_ids: Vec<EntityId>,
    pub interpretation_notes: Vec<TaggedNote>,
    pub verification_status: ValidationStatus,
    pub consultant_refs: Vec<EntityId>,
    pub update_source: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct RegulatoryRequirementPatch {
    pub name: Option<String>,
    pub description: Option<TextField>,
    pub status: Option<LifecycleStatus>,
    pub priority: Option<Priority>,
    pub ownership: Option<Ownership>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<TaggedNote>>,
    pub timestamps: Option<TimestampMeta>,
    pub jurisdiction: Option<String>,
    pub code: Option<String>,
    pub clause: Option<String>,
    pub title: Option<String>,
    pub requirement_text: Option<TextField>,
    pub applicability: Option<Vec<String>>,
    pub element_ids: Option<Vec<EntityId>>,
    pub compliance_method: Option<String>,
    pub evidence_required: Option<Vec<String>>,
    pub authority: Option<String>,
    pub effective_date: Option<String>,
    pub expiry_date: Option<String>,
    pub penalties: Option<Vec<String>>,
    pub exemptions: Option<Vec<String>>,
    pub related_requirement_ids: Option<Vec<EntityId>>,
    pub interpretation_notes: Option<Vec<TaggedNote>>,
    pub verification_status: Option<ValidationStatus>,
    pub consultant_refs: Option<Vec<EntityId>>,
    pub update_source: Option<String>,
}

impl_identified_header!(RegulatoryRequirement);

impl_patchable!(
    RegulatoryRequirement,
    RegulatoryRequirementPatch,
    {
        [header.name] => name,
        [header.description] => description,
        [header.status] => status,
        [header.priority] => priority,
        [header.ownership] => ownership,
        [header.tags] => tags,
        [header.notes] => notes,
        [header.timestamps] => timestamps,
        [jurisdiction] => jurisdiction,
        [code] => code,
        [clause] => clause,
        [title] => title,
        [requirement_text] => requirement_text,
        [applicability] => applicability,
        [element_ids] => element_ids,
        [compliance_method] => compliance_method,
        [evidence_required] => evidence_required,
        [authority] => authority,
        [effective_date] => effective_date,
        [expiry_date] => expiry_date,
        [penalties] => penalties,
        [exemptions] => exemptions,
        [related_requirement_ids] => related_requirement_ids,
        [interpretation_notes] => interpretation_notes,
        [verification_status] => verification_status,
        [consultant_refs] => consultant_refs,
        [update_source] => update_source,
    }
);
// #endregion
