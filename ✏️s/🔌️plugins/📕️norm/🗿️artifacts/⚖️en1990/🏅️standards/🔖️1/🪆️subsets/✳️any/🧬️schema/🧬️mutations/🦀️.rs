//! 🧬️ En1990 closed semantic mutation vocabulary for the basis-of-design subject.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_os_kernel as vcs;

use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES, ID_REMOVALS};

//#region 🔖️Leaves
use semio_s_artifact_norm_en1990_mut_design_basis::change_annex;
use semio_s_artifact_norm_en1990_mut_design_basis::change_project_id;
use semio_s_artifact_norm_en1990_mut_design_basis::change_altitude_m;
use semio_s_artifact_norm_en1990_mut_reliability::change_consequence_class;
use semio_s_artifact_norm_en1990_mut_reliability::change_reliability_class;
use semio_s_artifact_norm_en1990_mut_design_basis::change_design_working_life_category;
use semio_s_artifact_norm_en1990_mut_design_basis::change_design_working_life_years;
use semio_s_artifact_norm_en1990_mut_design_basis::change_reference_period_years;
use semio_s_artifact_norm_en1990_mut_reliability::change_supervision_level;
use semio_s_artifact_norm_en1990_mut_reliability::change_inspection_level;
use semio_s_artifact_norm_en1990_mut_reliability::change_beta_computed;
use semio_s_artifact_norm_en1990_mut_actions::insert_permanent;
use semio_s_artifact_norm_en1990_mut_actions::insert_variable;
use semio_s_artifact_norm_en1990_mut_accidental::insert_accidental;
use semio_s_artifact_norm_en1990_mut_accidental::insert_seismic;
use semio_s_artifact_norm_en1990_mut_members::insert_member;
use semio_s_artifact_norm_en1990_mut_members::insert_effect;
use semio_s_artifact_norm_en1990_mut_actions::remove_permanent;
use semio_s_artifact_norm_en1990_mut_actions::remove_variable;
use semio_s_artifact_norm_en1990_mut_accidental::remove_accidental;
use semio_s_artifact_norm_en1990_mut_accidental::remove_seismic;
use semio_s_artifact_norm_en1990_mut_members::remove_member;
use semio_s_artifact_norm_en1990_mut_members::remove_effect;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_permanent_kind;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_permanent_gk;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_variable_category;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_variable_qk;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_variable_group;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_accidental_ad;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_seismic_a_ek;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_seismic_importance_class;
use semio_s_artifact_norm_en1990_mut_members_identity::change_member_label_en;
use semio_s_artifact_norm_en1990_mut_members_identity::change_member_label_de;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_str;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_geo;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_equ_stab;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_equ_destab;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_fat;
use semio_s_artifact_norm_en1990_mut_members_identity::change_member_span;
use semio_s_artifact_norm_en1990_mut_members_identity::change_member_precamber;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_deflection_w1;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_deflection_w2;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_deflection_w3;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_defl_limit_ratio;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_defl_limit_ratio_w3;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_sls_limit_char;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_sls_limit_freq;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_sls_limit_qp;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_vibration_frequency;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_vib_freq_min;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_member_id;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deck_acceleration;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deck_accel_limit;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deck_twist;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deck_twist_limit;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deflection;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deflection_limit;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_effect_member_id;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_effect_action_id;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_effect_influence;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_fatigue_member_id;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_fatigue_detail_category;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_fatigue_stress_range;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_fatigue_cycles;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_resistance_dist;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_resistance_mean;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_resistance_sd;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_action_dist;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_action_mean;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_action_sd;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_quantity;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_pairs;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_distribution;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_sample_size;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_mean;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_std_dev;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_variation_known;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_conversion_factor;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_required;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_theory_variation;
use semio_s_artifact_norm_en1990_mut_placements::change_placement_x;
use semio_s_artifact_norm_en1990_mut_placements::change_placement_y;
use semio_s_artifact_norm_en1990_mut_placements::change_placement_z;
use semio_s_artifact_norm_en1990_mut_placements::change_placement_yaw;
use semio_s_artifact_norm_en1990_mut_design_basis_structure::change_structure_kind;
use semio_s_artifact_norm_en1990_mut_bridge_fatigue_rows::insert_bridge_sls;
use semio_s_artifact_norm_en1990_mut_bridge_fatigue_rows::remove_bridge_sls;
use semio_s_artifact_norm_en1990_mut_bridge_fatigue_rows::insert_fatigue_spectrum;
use semio_s_artifact_norm_en1990_mut_bridge_fatigue_rows::remove_fatigue_spectrum;
use semio_s_artifact_norm_en1990_mut_analysis_rows::insert_reliability_analysis;
use semio_s_artifact_norm_en1990_mut_analysis_rows::remove_reliability_analysis;
use semio_s_artifact_norm_en1990_mut_analysis_rows::insert_test_series;
use semio_s_artifact_norm_en1990_mut_analysis_rows::remove_test_series;
use semio_s_artifact_norm_en1990_mut_analysis_rows::insert_placement;
use semio_s_artifact_norm_en1990_mut_analysis_rows::remove_placement;
use semio_s_artifact_norm_en1990_mut_design_cases::insert_case;
use semio_s_artifact_norm_en1990_mut_design_cases::remove_case;
use semio_s_artifact_norm_en1990_mut_design_cases::change_case_number;
use semio_s_artifact_norm_en1990_mut_design_cases::change_case_text;
use semio_s_artifact_norm_en1990_mut_design_cases::change_case_label_en;
use semio_s_artifact_norm_en1990_mut_design_cases::change_case_label_de;
use semio_s_artifact_norm_en1990_mut_design_cases::change_case_entry_id;
use semio_s_artifact_norm_en1990_mut_members_horizontal::change_member_frame_height;
use semio_s_artifact_norm_en1990_mut_members_horizontal::change_member_frame_drift;
use semio_s_artifact_norm_en1990_mut_members_horizontal::change_member_storey_height;
use semio_s_artifact_norm_en1990_mut_members_horizontal::change_member_storey_drift;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_track_gauge;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_line_speed;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_ballast_depth;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_deck_mass;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_natural_frequency;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_end_rotation;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_basic_wind_speed;
use semio_s_artifact_norm_en1990_mut_members_material::change_member_material_strength_k;
use semio_s_artifact_norm_en1990_mut_members_material::change_member_material_conv_factor;
use semio_s_artifact_norm_en1990_mut_members_material::change_member_mat_partial_factor;
use semio_s_artifact_norm_en1990_mut_members_material::change_member_mat_design_strength;
use semio_s_artifact_norm_en1990_mut_members_geometry::change_member_nominal_dimension;
use semio_s_artifact_norm_en1990_mut_members_geometry::change_member_dimension_tolerance;
use semio_s_artifact_norm_en1990_mut_members_geometry::change_member_design_dimension;
use semio_s_artifact_norm_en1990_mut_members_analysis::change_member_member_alpha_cr;
use semio_s_artifact_norm_en1990_mut_members_analysis::change_member_second_order_analysis;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_damping_ratio;
use semio_s_artifact_norm_en1990_mut_factor_overrides::change_factor_override_factor;
use semio_s_artifact_norm_en1990_mut_factor_overrides::change_factor_override_target;
use semio_s_artifact_norm_en1990_mut_factor_overrides::change_factor_override_value;
use semio_s_artifact_norm_en1990_mut_factor_overrides::change_factor_override_justif;
use semio_s_artifact_norm_en1990_mut_factor_overrides_rows::insert_factor_override;
use semio_s_artifact_norm_en1990_mut_factor_overrides_rows::remove_factor_override;
use semio_s_artifact_norm_en1990_mut_traffic_groups::change_traffic_group_group;
use semio_s_artifact_norm_en1990_mut_traffic_groups::change_traffic_group_variable_id;
use semio_s_artifact_norm_en1990_mut_traffic_groups::change_reliability_member_id;
use semio_s_artifact_norm_en1990_mut_traffic_groups_rows::insert_traffic_group;
use semio_s_artifact_norm_en1990_mut_traffic_groups_rows::remove_traffic_group;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1990Snapshot, diff = En1990Diff, schema = "s.norm.en1990", reference = En1990Snapshot, reference_diff = En1990Diff)]
pub enum En1990Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeProjectId(change_project_id::ChangeProjectId),
    ChangeAltitudeM(change_altitude_m::ChangeAltitudeM),
    ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass),
    ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass),
    ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory),
    ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears),
    ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears),
    ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel),
    ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel),
    ChangeBetaComputed(change_beta_computed::ChangeBetaComputed),
    RemoveEffect(remove_effect::RemoveEffect),
    RemoveMember(remove_member::RemoveMember),
    RemoveSeismic(remove_seismic::RemoveSeismic),
    RemoveAccidental(remove_accidental::RemoveAccidental),
    RemoveVariable(remove_variable::RemoveVariable),
    RemovePermanent(remove_permanent::RemovePermanent),
    InsertEffect(insert_effect::InsertEffect),
    InsertMember(insert_member::InsertMember),
    InsertSeismic(insert_seismic::InsertSeismic),
    InsertAccidental(insert_accidental::InsertAccidental),
    InsertVariable(insert_variable::InsertVariable),
    InsertPermanent(insert_permanent::InsertPermanent),
    ChangePermanentKind(change_permanent_kind::ChangePermanentKind),
    ChangePermanentGk(change_permanent_gk::ChangePermanentGk),
    ChangeVariableCategory(change_variable_category::ChangeVariableCategory),
    ChangeVariableQk(change_variable_qk::ChangeVariableQk),
    ChangeVariableGroup(change_variable_group::ChangeVariableGroup),
    ChangeAccidentalAd(change_accidental_ad::ChangeAccidentalAd),
    ChangeSeismicAEk(change_seismic_a_ek::ChangeSeismicAEk),
    ChangeSeismicImportanceClass(change_seismic_importance_class::ChangeSeismicImportanceClass),
    ChangeMemberLabelEn(change_member_label_en::ChangeMemberLabelEn),
    ChangeMemberLabelDe(change_member_label_de::ChangeMemberLabelDe),
    ChangeMemberRdStr(change_member_rd_str::ChangeMemberRdStr),
    ChangeMemberRdGeo(change_member_rd_geo::ChangeMemberRdGeo),
    ChangeMemberRdEquStab(change_member_rd_equ_stab::ChangeMemberRdEquStab),
    ChangeMemberRdEquDestab(change_member_rd_equ_destab::ChangeMemberRdEquDestab),
    ChangeMemberRdFat(change_member_rd_fat::ChangeMemberRdFat),
    ChangeMemberSpan(change_member_span::ChangeMemberSpan),
    ChangeMemberPrecamber(change_member_precamber::ChangeMemberPrecamber),
    ChangeMemberDeflectionW1(change_member_deflection_w1::ChangeMemberDeflectionW1),
    ChangeMemberDeflectionW2(change_member_deflection_w2::ChangeMemberDeflectionW2),
    ChangeMemberDeflectionW3(change_member_deflection_w3::ChangeMemberDeflectionW3),
    ChangeMemberDeflLimitRatio(change_member_defl_limit_ratio::ChangeMemberDeflLimitRatio),
    ChangeMemberDeflLimitRatioW3(change_member_defl_limit_ratio_w3::ChangeMemberDeflLimitRatioW3),
    ChangeMemberSlsLimitChar(change_member_sls_limit_char::ChangeMemberSlsLimitChar),
    ChangeMemberSlsLimitFreq(change_member_sls_limit_freq::ChangeMemberSlsLimitFreq),
    ChangeMemberSlsLimitQp(change_member_sls_limit_qp::ChangeMemberSlsLimitQp),
    ChangeMemberVibrationFrequency(change_member_vibration_frequency::ChangeMemberVibrationFrequency),
    ChangeMemberVibFreqMin(change_member_vib_freq_min::ChangeMemberVibFreqMin),
    ChangeBridgeMemberId(change_bridge_member_id::ChangeBridgeMemberId),
    ChangeBridgeDeckAcceleration(change_bridge_deck_acceleration::ChangeBridgeDeckAcceleration),
    ChangeBridgeDeckAccelLimit(change_bridge_deck_accel_limit::ChangeBridgeDeckAccelLimit),
    ChangeBridgeDeckTwist(change_bridge_deck_twist::ChangeBridgeDeckTwist),
    ChangeBridgeDeckTwistLimit(change_bridge_deck_twist_limit::ChangeBridgeDeckTwistLimit),
    ChangeBridgeDeflection(change_bridge_deflection::ChangeBridgeDeflection),
    ChangeBridgeDeflectionLimit(change_bridge_deflection_limit::ChangeBridgeDeflectionLimit),
    ChangeEffectMemberId(change_effect_member_id::ChangeEffectMemberId),
    ChangeEffectActionId(change_effect_action_id::ChangeEffectActionId),
    ChangeEffectInfluence(change_effect_influence::ChangeEffectInfluence),
    ChangeFatigueMemberId(change_fatigue_member_id::ChangeFatigueMemberId),
    ChangeFatigueDetailCategory(change_fatigue_detail_category::ChangeFatigueDetailCategory),
    ChangeFatigueStressRange(change_fatigue_stress_range::ChangeFatigueStressRange),
    ChangeFatigueCycles(change_fatigue_cycles::ChangeFatigueCycles),
    ChangeReliabilityResistanceDist(change_reliability_resistance_dist::ChangeReliabilityResistanceDist),
    ChangeReliabilityResistanceMean(change_reliability_resistance_mean::ChangeReliabilityResistanceMean),
    ChangeReliabilityResistanceSd(change_reliability_resistance_sd::ChangeReliabilityResistanceSd),
    ChangeReliabilityActionDist(change_reliability_action_dist::ChangeReliabilityActionDist),
    ChangeReliabilityActionMean(change_reliability_action_mean::ChangeReliabilityActionMean),
    ChangeReliabilityActionSd(change_reliability_action_sd::ChangeReliabilityActionSd),
    ChangeTestQuantity(change_test_quantity::ChangeTestQuantity),
    ChangeTestPairs(change_test_pairs::ChangeTestPairs),
    ChangeTestDistribution(change_test_distribution::ChangeTestDistribution),
    ChangeTestSampleSize(change_test_sample_size::ChangeTestSampleSize),
    ChangeTestMean(change_test_mean::ChangeTestMean),
    ChangeTestStdDev(change_test_std_dev::ChangeTestStdDev),
    ChangeTestVariationKnown(change_test_variation_known::ChangeTestVariationKnown),
    ChangeTestConversionFactor(change_test_conversion_factor::ChangeTestConversionFactor),
    ChangeTestRequired(change_test_required::ChangeTestRequired),
    ChangeTestTheoryVariation(change_test_theory_variation::ChangeTestTheoryVariation),
    ChangePlacementX(change_placement_x::ChangePlacementX),
    ChangePlacementY(change_placement_y::ChangePlacementY),
    ChangePlacementZ(change_placement_z::ChangePlacementZ),
    ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw),
    ChangeStructureKind(change_structure_kind::ChangeStructureKind),
    InsertBridgeSls(insert_bridge_sls::InsertBridgeSls),
    RemoveBridgeSls(remove_bridge_sls::RemoveBridgeSls),
    InsertFatigueSpectrum(insert_fatigue_spectrum::InsertFatigueSpectrum),
    RemoveFatigueSpectrum(remove_fatigue_spectrum::RemoveFatigueSpectrum),
    InsertReliabilityAnalysis(insert_reliability_analysis::InsertReliabilityAnalysis),
    RemoveReliabilityAnalysis(remove_reliability_analysis::RemoveReliabilityAnalysis),
    InsertTestSeries(insert_test_series::InsertTestSeries),
    RemoveTestSeries(remove_test_series::RemoveTestSeries),
    InsertPlacement(insert_placement::InsertPlacement),
    RemovePlacement(remove_placement::RemovePlacement),
    InsertCase(insert_case::InsertCase),
    RemoveCase(remove_case::RemoveCase),
    ChangeCaseNumber(change_case_number::ChangeCaseNumber),
    ChangeCaseText(change_case_text::ChangeCaseText),
    ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn),
    ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe),
    ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId),
    ChangeMemberFrameHeight(change_member_frame_height::ChangeMemberFrameHeight),
    ChangeMemberFrameDrift(change_member_frame_drift::ChangeMemberFrameDrift),
    ChangeMemberStoreyHeight(change_member_storey_height::ChangeMemberStoreyHeight),
    ChangeMemberStoreyDrift(change_member_storey_drift::ChangeMemberStoreyDrift),
    ChangeBridgeTrackGauge(change_bridge_track_gauge::ChangeBridgeTrackGauge),
    ChangeBridgeLineSpeed(change_bridge_line_speed::ChangeBridgeLineSpeed),
    ChangeBridgeBallastDepth(change_bridge_ballast_depth::ChangeBridgeBallastDepth),
    ChangeBridgeDeckMass(change_bridge_deck_mass::ChangeBridgeDeckMass),
    ChangeBridgeNaturalFrequency(change_bridge_natural_frequency::ChangeBridgeNaturalFrequency),
    ChangeBridgeEndRotation(change_bridge_end_rotation::ChangeBridgeEndRotation),
    ChangeBridgeBasicWindSpeed(change_bridge_basic_wind_speed::ChangeBridgeBasicWindSpeed),
    ChangeMemberMaterialStrengthK(change_member_material_strength_k::ChangeMemberMaterialStrengthK),
    ChangeMemberMaterialConvFactor(change_member_material_conv_factor::ChangeMemberMaterialConvFactor),
    ChangeMemberMatPartialFactor(change_member_mat_partial_factor::ChangeMemberMatPartialFactor),
    ChangeMemberMatDesignStrength(change_member_mat_design_strength::ChangeMemberMatDesignStrength),
    ChangeMemberNominalDimension(change_member_nominal_dimension::ChangeMemberNominalDimension),
    ChangeMemberDimensionTolerance(change_member_dimension_tolerance::ChangeMemberDimensionTolerance),
    ChangeMemberDesignDimension(change_member_design_dimension::ChangeMemberDesignDimension),
    ChangeMemberMemberAlphaCr(change_member_member_alpha_cr::ChangeMemberMemberAlphaCr),
    ChangeMemberSecondOrderAnalysis(change_member_second_order_analysis::ChangeMemberSecondOrderAnalysis),
    ChangeBridgeDampingRatio(change_bridge_damping_ratio::ChangeBridgeDampingRatio),
    ChangeFactorOverrideFactor(change_factor_override_factor::ChangeFactorOverrideFactor),
    ChangeFactorOverrideTarget(change_factor_override_target::ChangeFactorOverrideTarget),
    ChangeFactorOverrideValue(change_factor_override_value::ChangeFactorOverrideValue),
    ChangeFactorOverrideJustif(change_factor_override_justif::ChangeFactorOverrideJustif),
    InsertFactorOverride(insert_factor_override::InsertFactorOverride),
    RemoveFactorOverride(remove_factor_override::RemoveFactorOverride),
    ChangeTrafficGroupGroup(change_traffic_group_group::ChangeTrafficGroupGroup),
    ChangeTrafficGroupVariableId(change_traffic_group_variable_id::ChangeTrafficGroupVariableId),
    ChangeReliabilityMemberId(change_reliability_member_id::ChangeReliabilityMemberId),
    InsertTrafficGroup(insert_traffic_group::InsertTrafficGroup),
    RemoveTrafficGroup(remove_traffic_group::RemoveTrafficGroup),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-project-id",
    "change-altitude-m",
    "change-consequence-class",
    "change-reliability-class",
    "change-design-working-life-category",
    "change-design-working-life-years",
    "change-reference-period-years",
    "change-supervision-level",
    "change-inspection-level",
    "change-beta-computed",
    "remove-effect",
    "remove-member",
    "remove-seismic",
    "remove-accidental",
    "remove-variable",
    "remove-permanent",
    "insert-effect",
    "insert-member",
    "insert-seismic",
    "insert-accidental",
    "insert-variable",
    "insert-permanent",
    "change-permanent-kind",
    "change-permanent-gk",
    "change-variable-category",
    "change-variable-qk",
    "change-variable-group",
    "change-accidental-ad",
    "change-seismic-a-ek",
    "change-seismic-importance-class",
    "change-member-label-en",
    "change-member-label-de",
    "change-member-rd-str",
    "change-member-rd-geo",
    "change-member-rd-equ-stab",
    "change-member-rd-equ-destab",
    "change-member-rd-fat",
    "change-member-span",
    "change-member-precamber",
    "change-member-deflection-w1",
    "change-member-deflection-w2",
    "change-member-deflection-w3",
    "change-member-defl-limit-ratio",
    "change-member-defl-limit-ratio-w3",
    "change-member-sls-limit-char",
    "change-member-sls-limit-freq",
    "change-member-sls-limit-qp",
    "change-member-vibration-frequency",
    "change-member-vib-freq-min",
    "change-bridge-member-id",
    "change-bridge-deck-acceleration",
    "change-bridge-deck-accel-limit",
    "change-bridge-deck-twist",
    "change-bridge-deck-twist-limit",
    "change-bridge-deflection",
    "change-bridge-deflection-limit",
    "change-effect-member-id",
    "change-effect-action-id",
    "change-effect-influence",
    "change-fatigue-member-id",
    "change-fatigue-detail-category",
    "change-fatigue-stress-range",
    "change-fatigue-cycles",
    "change-reliability-resistance-dist",
    "change-reliability-resistance-mean",
    "change-reliability-resistance-sd",
    "change-reliability-action-dist",
    "change-reliability-action-mean",
    "change-reliability-action-sd",
    "change-test-quantity",
    "change-test-pairs",
    "change-test-distribution",
    "change-test-sample-size",
    "change-test-mean",
    "change-test-std-dev",
    "change-test-variation-known",
    "change-test-conversion-factor",
    "change-test-required",
    "change-test-theory-variation",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "change-structure-kind",
    "insert-bridge-sls",
    "remove-bridge-sls",
    "insert-fatigue-spectrum",
    "remove-fatigue-spectrum",
    "insert-reliability-analysis",
    "remove-reliability-analysis",
    "insert-test-series",
    "remove-test-series",
    "insert-placement",
    "remove-placement",
    "insert-case",
    "remove-case",
    "change-case-number",
    "change-case-text",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
    "change-member-frame-height",
    "change-member-frame-drift",
    "change-member-storey-height",
    "change-member-storey-drift",
    "change-bridge-track-gauge",
    "change-bridge-line-speed",
    "change-bridge-ballast-depth",
    "change-bridge-deck-mass",
    "change-bridge-natural-frequency",
    "change-bridge-end-rotation",
    "change-bridge-basic-wind-speed",
    "change-member-material-strength-k",
    "change-member-material-conv-factor",
    "change-member-mat-partial-factor",
    "change-member-mat-design-strength",
    "change-member-nominal-dimension",
    "change-member-dimension-tolerance",
    "change-member-design-dimension",
    "change-member-member-alpha-cr",
    "change-member-second-order-analysis",
    "change-bridge-damping-ratio",
    "change-factor-override-factor",
    "change-factor-override-target",
    "change-factor-override-value",
    "change-factor-override-justif",
    "insert-factor-override",
    "remove-factor-override",
    "change-traffic-group-group",
    "change-traffic-group-variable-id",
    "change-reliability-member-id",
    "insert-traffic-group",
    "remove-traffic-group",
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff and inverse through the central applier.
pub mod mutation_bridge {
/// 🌉️ Applies one mutation to `base` through the central applier, returning the next snapshot and the raised `<Level>:<code>` messages.
pub fn apply_en1990_mutation(base: &semio_s_artifact_norm_en1990_model::En1990Snapshot, mutation: &crate::En1990Mutation) -> Result<(semio_s_artifact_norm_en1990_model::En1990Snapshot, Vec<String>), String> {
    let raised = <crate::En1990Mutation as protocol::ColdMutation<semio_s_artifact_norm_en1990_model::En1990Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ The mutation rows that undo `mutation` applied to `base`, in store order.
pub fn inverse_en1990_mutation(mutation: &crate::En1990Mutation, base: &semio_s_artifact_norm_en1990_model::En1990Snapshot) -> Result<Vec<crate::En1990Mutation>, semio_framework_value::ValueError> {
    <crate::En1990Mutation as protocol::ColdMutation<semio_s_artifact_norm_en1990_model::En1990Snapshot>>::inverse(mutation, base)
}
}
pub use crate::mutation_bridge::*;

#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
