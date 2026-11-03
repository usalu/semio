//! 🏢️ EN 1998 buildings mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_buildings::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1998_mut_buildings::insert_building::InsertBuilding;
use semio_s_artifact_norm_en1998_mut_buildings::remove_building::RemoveBuilding;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_v_rd_n::ChangeSystemVRdN;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_permanent_gk_n::ChangeStoreyPermanentGkN;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_stiffness_x::ChangeStoreyStiffnessX;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_drift_xm::ChangeStoreyDriftXM;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_plan_regular::ChangeBuildingPlanRegular;
use semio_s_artifact_norm_en1998_mut_buildings::change_elevation_regular::ChangeElevationRegular;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_s_ap_r::ChangeSiteSApR;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_a_gr::ChangeSiteAGr;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_de_ground_combo::ChangeSiteDeGroundCombo;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_en_ground_type::ChangeSiteEnGroundType;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_en_spectrum_type::ChangeSiteEnSpectrumType;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_importance_class::ChangeSiteImportanceClass;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_name::ChangeBuildingName;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_plan_width_m::ChangeBuildingPlanWidthM;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_plan_length_m::ChangeBuildingPlanLengthM;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_t1_method::ChangeBuildingT1Method;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_t1_given_s::ChangeBuildingT1GivenS;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_ct::ChangeBuildingCt;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_drift_limit_class::ChangeBuildingDriftLimitClass;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_nu::ChangeBuildingNu;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_multi_resisting_systems::ChangeBuildingMultiResistingSystems;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_claims_simple_masonry::ChangeBuildingClaimsSimpleMasonry;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_accidental_ecc_ratio::ChangeBuildingAccidentalEccRatio;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_direction::ChangeSystemDirection;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_system_type::ChangeSystemSystemType;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_material::ChangeSystemMaterial;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_ductility_class::ChangeSystemDuctilityClass;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_q0::ChangeSystemQ0;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_alpha_u_over_alpha_1::ChangeSystemAlphaUOverAlpha1;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_k_w::ChangeSystemKW;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_height_m::ChangeStoreyHeightM;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_correlated_occupancy::ChangeStoreyCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_stiffness_y::ChangeStoreyStiffnessY;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_centre_of_mass_x_m::ChangeStoreyCentreOfMassXM;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_centre_of_mass_y_m::ChangeStoreyCentreOfMassYM;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_centre_of_stiffness_x_m::ChangeStoreyCentreOfStiffnessXM;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_centre_of_stiffness_y_m::ChangeStoreyCentreOfStiffnessYM;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_drift_y_m::ChangeStoreyDriftYM;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_shear_resistance_n::ChangeStoreyShearResistanceN;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_v_s30_m_s::ChangeSiteVS30MS;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_n_spt::ChangeSiteNSpt;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_c_u_kpa::ChangeSiteCUKpa;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_soft_clay_thickness_m::ChangeSiteSoftClayThicknessM;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_soft_clay_pi::ChangeSiteSoftClayPi;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_liquefiable_layer::ChangeSiteLiquefiableLayer;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_alluvium_thickness_m::ChangeSiteAlluviumThicknessM;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_alluvium_v_s_m_s::ChangeSiteAlluviumVSMS;
use semio_s_artifact_norm_en1998_mut_buildings::change_site_below_alluvium_v_s_m_s::ChangeSiteBelowAlluviumVSMS;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_setback_ratio::ChangeBuildingSetbackRatio;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_storey_stiffness_ratio::ChangeBuildingStoreyStiffnessRatio;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_storey_stiffness_basis::ChangeBuildingStoreyStiffnessBasis;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_plan_env_gap_ratio::ChangeBuildingPlanEnvGapRatio;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_joint_gap_m::ChangeBuildingJointGapM;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_ns_weight_n::ChangeBuildingNsWeightN;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_ns_height_m::ChangeBuildingNsHeightM;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_ns_period_s::ChangeBuildingNsPeriodS;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_ns_q::ChangeBuildingNsQ;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_ns_resistance_n::ChangeBuildingNsResistanceN;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_col_moment_sum_nm::ChangeBuildingColMomentSumNm;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_beam_moment_sum_nm::ChangeBuildingBeamMomentSumNm;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_variable_category::ChangeStoreyVariableCategory;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_variable_qk_n::ChangeStoreyVariableQkN;
use semio_s_artifact_norm_en1998_mut_buildings::insert_storey_variable::InsertStoreyVariable;
use semio_s_artifact_norm_en1998_mut_buildings::remove_storey_variable::RemoveStoreyVariable;
use semio_s_artifact_norm_en1998_mut_buildings::insert_member::InsertMember;
use semio_s_artifact_norm_en1998_mut_buildings::remove_member::RemoveMember;
use semio_s_artifact_norm_en1998_mut_buildings::insert_storey::InsertStorey;
use semio_s_artifact_norm_en1998_mut_buildings::remove_storey::RemoveStorey;
use semio_s_artifact_norm_en1998_mut_buildings::insert_system::InsertSystem;
use semio_s_artifact_norm_en1998_mut_buildings::remove_system::RemoveSystem;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_ot_base_x_m::ChangeBuildingOtBaseXM;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_ot_base_y_m::ChangeBuildingOtBaseYM;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_tiedown_nm::ChangeBuildingTiedownNm;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_building_id::ChangeDiaphragmBuildingId;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_storey_id::ChangeDiaphragmStoreyId;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_material::ChangeDiaphragmMaterial;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_thickness_m::ChangeDiaphragmThicknessM;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_span_m::ChangeDiaphragmSpanM;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_depth_m::ChangeDiaphragmDepthM;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_opening_width_m::ChangeDiaphragmOpeningWidthM;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_shear_str_mpa::ChangeDiaphragmShearStrMpa;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_chord_area_m2::ChangeDiaphragmChordAreaM2;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_chord_str_mpa::ChangeDiaphragmChordStrMpa;
use semio_s_artifact_norm_en1998_mut_buildings::change_diaphragm_gamma_d::ChangeDiaphragmGammaD;
use semio_s_artifact_norm_en1998_mut_buildings::change_accelerogram_building_id::ChangeAccelerogramBuildingId;
use semio_s_artifact_norm_en1998_mut_buildings::change_accelerogram_label::ChangeAccelerogramLabel;
use semio_s_artifact_norm_en1998_mut_buildings::change_accelerogram_samples::ChangeAccelerogramSamples;
use semio_s_artifact_norm_en1998_mut_buildings::change_accelerogram_dt_s::ChangeAccelerogramDtS;
use semio_s_artifact_norm_en1998_mut_buildings::change_accelerogram_scale_factor::ChangeAccelerogramScaleFactor;
use semio_s_artifact_norm_en1998_mut_buildings::insert_diaphragm::InsertDiaphragm;
use semio_s_artifact_norm_en1998_mut_buildings::remove_diaphragm::RemoveDiaphragm;
use semio_s_artifact_norm_en1998_mut_buildings::insert_accelerogram::InsertAccelerogram;
use semio_s_artifact_norm_en1998_mut_buildings::remove_accelerogram::RemoveAccelerogram;

/// ✍️ Prints `change-annex` as an op-text line.
pub fn print_change_annex(p: &ChangeAnnex) -> String {
    format!("change-annex new-annex={}", enc_json(&p.new_annex))
}

/// 📖 Parses the arguments of `change-annex`.
pub fn parse_change_annex(args: &Args) -> Result<ChangeAnnex, String> {
    let arg = |k: &str| required(args, "change-annex", k);
    Ok(ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })
}

/// ✍️ Prints `insert-building` as an op-text line.
pub fn print_insert_building(p: &InsertBuilding) -> String {
    format!("insert-building index={} building={}", enc_json(&p.index), enc_json(&p.building))
}

/// 📖 Parses the arguments of `insert-building`.
pub fn parse_insert_building(args: &Args) -> Result<InsertBuilding, String> {
    let arg = |k: &str| required(args, "insert-building", k);
    Ok(InsertBuilding { index: opt_index(args)?, building: dec_json(&arg("building")?)? })
}

/// ✍️ Prints `remove-building` as an op-text line.
pub fn print_remove_building(p: &RemoveBuilding) -> String {
    format!("remove-building id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-building`.
pub fn parse_remove_building(args: &Args) -> Result<RemoveBuilding, String> {
    let arg = |k: &str| required(args, "remove-building", k);
    Ok(RemoveBuilding { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-system-v-rd-n` as an op-text line.
pub fn print_change_system_v_rd_n(p: &ChangeSystemVRdN) -> String {
    format!("change-system-v-rd-n building-id={} id={} new-base-shear-resistance-n={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_base_shear_resistance_n))
}

/// 📖 Parses the arguments of `change-system-v-rd-n`.
pub fn parse_change_system_v_rd_n(args: &Args) -> Result<ChangeSystemVRdN, String> {
    let arg = |k: &str| required(args, "change-system-v-rd-n", k);
    Ok(ChangeSystemVRdN { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_base_shear_resistance_n: dec_json(&arg("new-base-shear-resistance-n")?)? })
}

/// ✍️ Prints `change-storey-permanent-gk-n` as an op-text line.
pub fn print_change_storey_permanent_gk_n(p: &ChangeStoreyPermanentGkN) -> String {
    format!("change-storey-permanent-gk-n building-id={} id={} new-permanent-gk-n={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_permanent_gk_n))
}

/// 📖 Parses the arguments of `change-storey-permanent-gk-n`.
pub fn parse_change_storey_permanent_gk_n(args: &Args) -> Result<ChangeStoreyPermanentGkN, String> {
    let arg = |k: &str| required(args, "change-storey-permanent-gk-n", k);
    Ok(ChangeStoreyPermanentGkN { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_permanent_gk_n: dec_json(&arg("new-permanent-gk-n")?)? })
}

/// ✍️ Prints `change-storey-stiffness-x` as an op-text line.
pub fn print_change_storey_stiffness_x(p: &ChangeStoreyStiffnessX) -> String {
    format!("change-storey-stiffness-x building-id={} id={} new-stiffness-x={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_stiffness_x))
}

/// 📖 Parses the arguments of `change-storey-stiffness-x`.
pub fn parse_change_storey_stiffness_x(args: &Args) -> Result<ChangeStoreyStiffnessX, String> {
    let arg = |k: &str| required(args, "change-storey-stiffness-x", k);
    Ok(ChangeStoreyStiffnessX { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_stiffness_x: dec_json(&arg("new-stiffness-x")?)? })
}

/// ✍️ Prints `change-storey-drift-xm` as an op-text line.
pub fn print_change_storey_drift_xm(p: &ChangeStoreyDriftXM) -> String {
    format!("change-storey-drift-xm building-id={} id={} new-drift-x-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_drift_x_m))
}

/// 📖 Parses the arguments of `change-storey-drift-xm`.
pub fn parse_change_storey_drift_xm(args: &Args) -> Result<ChangeStoreyDriftXM, String> {
    let arg = |k: &str| required(args, "change-storey-drift-xm", k);
    Ok(ChangeStoreyDriftXM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_drift_x_m: dec_json(&arg("new-drift-x-m")?)? })
}

/// ✍️ Prints `change-building-plan-regular` as an op-text line.
pub fn print_change_building_plan_regular(p: &ChangeBuildingPlanRegular) -> String {
    format!("change-building-plan-regular id={} new-plan-regular={}", enc_json(&p.id), enc_json(&p.new_plan_regular))
}

/// 📖 Parses the arguments of `change-building-plan-regular`.
pub fn parse_change_building_plan_regular(args: &Args) -> Result<ChangeBuildingPlanRegular, String> {
    let arg = |k: &str| required(args, "change-building-plan-regular", k);
    Ok(ChangeBuildingPlanRegular { id: dec_json(&arg("id")?)?, new_plan_regular: dec_json(&arg("new-plan-regular")?)? })
}

/// ✍️ Prints `change-elevation-regular` as an op-text line.
pub fn print_change_elevation_regular(p: &ChangeElevationRegular) -> String {
    format!("change-elevation-regular id={} new-elevation-regular={}", enc_json(&p.id), enc_json(&p.new_elevation_regular))
}

/// 📖 Parses the arguments of `change-elevation-regular`.
pub fn parse_change_elevation_regular(args: &Args) -> Result<ChangeElevationRegular, String> {
    let arg = |k: &str| required(args, "change-elevation-regular", k);
    Ok(ChangeElevationRegular { id: dec_json(&arg("id")?)?, new_elevation_regular: dec_json(&arg("new-elevation-regular")?)? })
}

/// ✍️ Prints `change-site-s-ap-r` as an op-text line.
pub fn print_change_site_s_ap_r(p: &ChangeSiteSApR) -> String {
    format!("change-site-s-ap-r new-s-ap-r={}", enc_json(&p.new_s_ap_r))
}

/// 📖 Parses the arguments of `change-site-s-ap-r`.
pub fn parse_change_site_s_ap_r(args: &Args) -> Result<ChangeSiteSApR, String> {
    let arg = |k: &str| required(args, "change-site-s-ap-r", k);
    Ok(ChangeSiteSApR { new_s_ap_r: dec_json(&arg("new-s-ap-r")?)? })
}

/// ✍️ Prints `change-site-a-gr` as an op-text line.
pub fn print_change_site_a_gr(p: &ChangeSiteAGr) -> String {
    format!("change-site-a-gr new-a-gr={}", enc_json(&p.new_a_gr))
}

/// 📖 Parses the arguments of `change-site-a-gr`.
pub fn parse_change_site_a_gr(args: &Args) -> Result<ChangeSiteAGr, String> {
    let arg = |k: &str| required(args, "change-site-a-gr", k);
    Ok(ChangeSiteAGr { new_a_gr: dec_json(&arg("new-a-gr")?)? })
}

/// ✍️ Prints `change-site-de-ground-combo` as an op-text line.
pub fn print_change_site_de_ground_combo(p: &ChangeSiteDeGroundCombo) -> String {
    format!("change-site-de-ground-combo new-de-ground-combo={}", enc_json(&p.new_de_ground_combo))
}

/// 📖 Parses the arguments of `change-site-de-ground-combo`.
pub fn parse_change_site_de_ground_combo(args: &Args) -> Result<ChangeSiteDeGroundCombo, String> {
    let arg = |k: &str| required(args, "change-site-de-ground-combo", k);
    Ok(ChangeSiteDeGroundCombo { new_de_ground_combo: dec_json(&arg("new-de-ground-combo")?)? })
}

/// ✍️ Prints `change-site-en-ground-type` as an op-text line.
pub fn print_change_site_en_ground_type(p: &ChangeSiteEnGroundType) -> String {
    format!("change-site-en-ground-type new-en-ground-type={}", enc_json(&p.new_en_ground_type))
}

/// 📖 Parses the arguments of `change-site-en-ground-type`.
pub fn parse_change_site_en_ground_type(args: &Args) -> Result<ChangeSiteEnGroundType, String> {
    let arg = |k: &str| required(args, "change-site-en-ground-type", k);
    Ok(ChangeSiteEnGroundType { new_en_ground_type: dec_json(&arg("new-en-ground-type")?)? })
}

/// ✍️ Prints `change-site-en-spectrum-type` as an op-text line.
pub fn print_change_site_en_spectrum_type(p: &ChangeSiteEnSpectrumType) -> String {
    format!("change-site-en-spectrum-type new-en-spectrum-type={}", enc_json(&p.new_en_spectrum_type))
}

/// 📖 Parses the arguments of `change-site-en-spectrum-type`.
pub fn parse_change_site_en_spectrum_type(args: &Args) -> Result<ChangeSiteEnSpectrumType, String> {
    let arg = |k: &str| required(args, "change-site-en-spectrum-type", k);
    Ok(ChangeSiteEnSpectrumType { new_en_spectrum_type: dec_json(&arg("new-en-spectrum-type")?)? })
}

/// ✍️ Prints `change-site-importance-class` as an op-text line.
pub fn print_change_site_importance_class(p: &ChangeSiteImportanceClass) -> String {
    format!("change-site-importance-class new-importance-class={}", enc_json(&p.new_importance_class))
}

/// 📖 Parses the arguments of `change-site-importance-class`.
pub fn parse_change_site_importance_class(args: &Args) -> Result<ChangeSiteImportanceClass, String> {
    let arg = |k: &str| required(args, "change-site-importance-class", k);
    Ok(ChangeSiteImportanceClass { new_importance_class: dec_json(&arg("new-importance-class")?)? })
}

/// ✍️ Prints `change-building-name` as an op-text line.
pub fn print_change_building_name(p: &ChangeBuildingName) -> String {
    format!("change-building-name id={} new-name={}", enc_json(&p.id), enc_json(&p.new_name))
}

/// 📖 Parses the arguments of `change-building-name`.
pub fn parse_change_building_name(args: &Args) -> Result<ChangeBuildingName, String> {
    let arg = |k: &str| required(args, "change-building-name", k);
    Ok(ChangeBuildingName { id: dec_json(&arg("id")?)?, new_name: dec_json(&arg("new-name")?)? })
}

/// ✍️ Prints `change-building-plan-width-m` as an op-text line.
pub fn print_change_building_plan_width_m(p: &ChangeBuildingPlanWidthM) -> String {
    format!("change-building-plan-width-m id={} new-plan-width-m={}", enc_json(&p.id), enc_json(&p.new_plan_width_m))
}

/// 📖 Parses the arguments of `change-building-plan-width-m`.
pub fn parse_change_building_plan_width_m(args: &Args) -> Result<ChangeBuildingPlanWidthM, String> {
    let arg = |k: &str| required(args, "change-building-plan-width-m", k);
    Ok(ChangeBuildingPlanWidthM { id: dec_json(&arg("id")?)?, new_plan_width_m: dec_json(&arg("new-plan-width-m")?)? })
}

/// ✍️ Prints `change-building-plan-length-m` as an op-text line.
pub fn print_change_building_plan_length_m(p: &ChangeBuildingPlanLengthM) -> String {
    format!("change-building-plan-length-m id={} new-plan-length-m={}", enc_json(&p.id), enc_json(&p.new_plan_length_m))
}

/// 📖 Parses the arguments of `change-building-plan-length-m`.
pub fn parse_change_building_plan_length_m(args: &Args) -> Result<ChangeBuildingPlanLengthM, String> {
    let arg = |k: &str| required(args, "change-building-plan-length-m", k);
    Ok(ChangeBuildingPlanLengthM { id: dec_json(&arg("id")?)?, new_plan_length_m: dec_json(&arg("new-plan-length-m")?)? })
}

/// ✍️ Prints `change-building-t1-method` as an op-text line.
pub fn print_change_building_t1_method(p: &ChangeBuildingT1Method) -> String {
    format!("change-building-t1-method id={} new-t1-method={}", enc_json(&p.id), enc_json(&p.new_t1_method))
}

/// 📖 Parses the arguments of `change-building-t1-method`.
pub fn parse_change_building_t1_method(args: &Args) -> Result<ChangeBuildingT1Method, String> {
    let arg = |k: &str| required(args, "change-building-t1-method", k);
    Ok(ChangeBuildingT1Method { id: dec_json(&arg("id")?)?, new_t1_method: dec_json(&arg("new-t1-method")?)? })
}

/// ✍️ Prints `change-building-t1-given-s` as an op-text line.
pub fn print_change_building_t1_given_s(p: &ChangeBuildingT1GivenS) -> String {
    format!("change-building-t1-given-s id={} new-t1-given-s={}", enc_json(&p.id), enc_json(&p.new_t1_given_s))
}

/// 📖 Parses the arguments of `change-building-t1-given-s`.
pub fn parse_change_building_t1_given_s(args: &Args) -> Result<ChangeBuildingT1GivenS, String> {
    let arg = |k: &str| required(args, "change-building-t1-given-s", k);
    Ok(ChangeBuildingT1GivenS { id: dec_json(&arg("id")?)?, new_t1_given_s: dec_json(&arg("new-t1-given-s")?)? })
}

/// ✍️ Prints `change-building-ct` as an op-text line.
pub fn print_change_building_ct(p: &ChangeBuildingCt) -> String {
    format!("change-building-ct id={} new-ct={}", enc_json(&p.id), enc_json(&p.new_ct))
}

/// 📖 Parses the arguments of `change-building-ct`.
pub fn parse_change_building_ct(args: &Args) -> Result<ChangeBuildingCt, String> {
    let arg = |k: &str| required(args, "change-building-ct", k);
    Ok(ChangeBuildingCt { id: dec_json(&arg("id")?)?, new_ct: dec_json(&arg("new-ct")?)? })
}

/// ✍️ Prints `change-building-drift-limit-class` as an op-text line.
pub fn print_change_building_drift_limit_class(p: &ChangeBuildingDriftLimitClass) -> String {
    format!("change-building-drift-limit-class id={} new-drift-limit-class={}", enc_json(&p.id), enc_json(&p.new_drift_limit_class))
}

/// 📖 Parses the arguments of `change-building-drift-limit-class`.
pub fn parse_change_building_drift_limit_class(args: &Args) -> Result<ChangeBuildingDriftLimitClass, String> {
    let arg = |k: &str| required(args, "change-building-drift-limit-class", k);
    Ok(ChangeBuildingDriftLimitClass { id: dec_json(&arg("id")?)?, new_drift_limit_class: dec_json(&arg("new-drift-limit-class")?)? })
}

/// ✍️ Prints `change-building-nu` as an op-text line.
pub fn print_change_building_nu(p: &ChangeBuildingNu) -> String {
    format!("change-building-nu id={} new-nu={}", enc_json(&p.id), enc_json(&p.new_nu))
}

/// 📖 Parses the arguments of `change-building-nu`.
pub fn parse_change_building_nu(args: &Args) -> Result<ChangeBuildingNu, String> {
    let arg = |k: &str| required(args, "change-building-nu", k);
    Ok(ChangeBuildingNu { id: dec_json(&arg("id")?)?, new_nu: dec_json(&arg("new-nu")?)? })
}

/// ✍️ Prints `change-building-multi-resisting-systems` as an op-text line.
pub fn print_change_building_multi_resisting_systems(p: &ChangeBuildingMultiResistingSystems) -> String {
    format!("change-building-multi-resisting-systems id={} new-multiple-resisting-systems={}", enc_json(&p.id), enc_json(&p.new_multiple_resisting_systems))
}

/// 📖 Parses the arguments of `change-building-multi-resisting-systems`.
pub fn parse_change_building_multi_resisting_systems(args: &Args) -> Result<ChangeBuildingMultiResistingSystems, String> {
    let arg = |k: &str| required(args, "change-building-multi-resisting-systems", k);
    Ok(ChangeBuildingMultiResistingSystems { id: dec_json(&arg("id")?)?, new_multiple_resisting_systems: dec_json(&arg("new-multiple-resisting-systems")?)? })
}

/// ✍️ Prints `change-building-claims-simple-masonry` as an op-text line.
pub fn print_change_building_claims_simple_masonry(p: &ChangeBuildingClaimsSimpleMasonry) -> String {
    format!("change-building-claims-simple-masonry id={} new-claims-simple-masonry={}", enc_json(&p.id), enc_json(&p.new_claims_simple_masonry))
}

/// 📖 Parses the arguments of `change-building-claims-simple-masonry`.
pub fn parse_change_building_claims_simple_masonry(args: &Args) -> Result<ChangeBuildingClaimsSimpleMasonry, String> {
    let arg = |k: &str| required(args, "change-building-claims-simple-masonry", k);
    Ok(ChangeBuildingClaimsSimpleMasonry { id: dec_json(&arg("id")?)?, new_claims_simple_masonry: dec_json(&arg("new-claims-simple-masonry")?)? })
}

/// ✍️ Prints `change-building-accidental-ecc-ratio` as an op-text line.
pub fn print_change_building_accidental_ecc_ratio(p: &ChangeBuildingAccidentalEccRatio) -> String {
    format!("change-building-accidental-ecc-ratio id={} new-accidental-eccentricity-ratio={}", enc_json(&p.id), enc_json(&p.new_accidental_eccentricity_ratio))
}

/// 📖 Parses the arguments of `change-building-accidental-ecc-ratio`.
pub fn parse_change_building_accidental_ecc_ratio(args: &Args) -> Result<ChangeBuildingAccidentalEccRatio, String> {
    let arg = |k: &str| required(args, "change-building-accidental-ecc-ratio", k);
    Ok(ChangeBuildingAccidentalEccRatio { id: dec_json(&arg("id")?)?, new_accidental_eccentricity_ratio: dec_json(&arg("new-accidental-eccentricity-ratio")?)? })
}

/// ✍️ Prints `change-system-direction` as an op-text line.
pub fn print_change_system_direction(p: &ChangeSystemDirection) -> String {
    format!("change-system-direction building-id={} id={} new-direction={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_direction))
}

/// 📖 Parses the arguments of `change-system-direction`.
pub fn parse_change_system_direction(args: &Args) -> Result<ChangeSystemDirection, String> {
    let arg = |k: &str| required(args, "change-system-direction", k);
    Ok(ChangeSystemDirection { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_direction: dec_json(&arg("new-direction")?)? })
}

/// ✍️ Prints `change-system-system-type` as an op-text line.
pub fn print_change_system_system_type(p: &ChangeSystemSystemType) -> String {
    format!("change-system-system-type building-id={} id={} new-system-type={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_system_type))
}

/// 📖 Parses the arguments of `change-system-system-type`.
pub fn parse_change_system_system_type(args: &Args) -> Result<ChangeSystemSystemType, String> {
    let arg = |k: &str| required(args, "change-system-system-type", k);
    Ok(ChangeSystemSystemType { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_system_type: dec_json(&arg("new-system-type")?)? })
}

/// ✍️ Prints `change-system-material` as an op-text line.
pub fn print_change_system_material(p: &ChangeSystemMaterial) -> String {
    format!("change-system-material building-id={} id={} new-material={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_material))
}

/// 📖 Parses the arguments of `change-system-material`.
pub fn parse_change_system_material(args: &Args) -> Result<ChangeSystemMaterial, String> {
    let arg = |k: &str| required(args, "change-system-material", k);
    Ok(ChangeSystemMaterial { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_material: dec_json(&arg("new-material")?)? })
}

/// ✍️ Prints `change-system-ductility-class` as an op-text line.
pub fn print_change_system_ductility_class(p: &ChangeSystemDuctilityClass) -> String {
    format!("change-system-ductility-class building-id={} id={} new-ductility-class={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_ductility_class))
}

/// 📖 Parses the arguments of `change-system-ductility-class`.
pub fn parse_change_system_ductility_class(args: &Args) -> Result<ChangeSystemDuctilityClass, String> {
    let arg = |k: &str| required(args, "change-system-ductility-class", k);
    Ok(ChangeSystemDuctilityClass { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_ductility_class: dec_json(&arg("new-ductility-class")?)? })
}

/// ✍️ Prints `change-system-q0` as an op-text line.
pub fn print_change_system_q0(p: &ChangeSystemQ0) -> String {
    format!("change-system-q0 building-id={} id={} new-q0={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_q0))
}

/// 📖 Parses the arguments of `change-system-q0`.
pub fn parse_change_system_q0(args: &Args) -> Result<ChangeSystemQ0, String> {
    let arg = |k: &str| required(args, "change-system-q0", k);
    Ok(ChangeSystemQ0 { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_q0: dec_json(&arg("new-q0")?)? })
}

/// ✍️ Prints `change-system-alpha-u-over-alpha1` as an op-text line.
pub fn print_change_system_alpha_u_over_alpha_1(p: &ChangeSystemAlphaUOverAlpha1) -> String {
    format!("change-system-alpha-u-over-alpha1 building-id={} id={} new-alpha-u-over-alpha-1={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_alpha_u_over_alpha_1))
}

/// 📖 Parses the arguments of `change-system-alpha-u-over-alpha1`.
pub fn parse_change_system_alpha_u_over_alpha_1(args: &Args) -> Result<ChangeSystemAlphaUOverAlpha1, String> {
    let arg = |k: &str| required(args, "change-system-alpha-u-over-alpha1", k);
    Ok(ChangeSystemAlphaUOverAlpha1 { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_alpha_u_over_alpha_1: dec_json(&arg("new-alpha-u-over-alpha-1")?)? })
}

/// ✍️ Prints `change-system-kw` as an op-text line.
pub fn print_change_system_k_w(p: &ChangeSystemKW) -> String {
    format!("change-system-kw building-id={} id={} new-k-w={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_k_w))
}

/// 📖 Parses the arguments of `change-system-kw`.
pub fn parse_change_system_k_w(args: &Args) -> Result<ChangeSystemKW, String> {
    let arg = |k: &str| required(args, "change-system-kw", k);
    Ok(ChangeSystemKW { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_k_w: dec_json(&arg("new-k-w")?)? })
}

/// ✍️ Prints `change-storey-height-m` as an op-text line.
pub fn print_change_storey_height_m(p: &ChangeStoreyHeightM) -> String {
    format!("change-storey-height-m building-id={} id={} new-height-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_height_m))
}

/// 📖 Parses the arguments of `change-storey-height-m`.
pub fn parse_change_storey_height_m(args: &Args) -> Result<ChangeStoreyHeightM, String> {
    let arg = |k: &str| required(args, "change-storey-height-m", k);
    Ok(ChangeStoreyHeightM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_height_m: dec_json(&arg("new-height-m")?)? })
}

/// ✍️ Prints `change-storey-correlated-occupancy` as an op-text line.
pub fn print_change_storey_correlated_occupancy(p: &ChangeStoreyCorrelatedOccupancy) -> String {
    format!("change-storey-correlated-occupancy building-id={} id={} new-correlated-occupancy={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_correlated_occupancy))
}

/// 📖 Parses the arguments of `change-storey-correlated-occupancy`.
pub fn parse_change_storey_correlated_occupancy(args: &Args) -> Result<ChangeStoreyCorrelatedOccupancy, String> {
    let arg = |k: &str| required(args, "change-storey-correlated-occupancy", k);
    Ok(ChangeStoreyCorrelatedOccupancy { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_correlated_occupancy: dec_json(&arg("new-correlated-occupancy")?)? })
}

/// ✍️ Prints `change-storey-stiffness-y` as an op-text line.
pub fn print_change_storey_stiffness_y(p: &ChangeStoreyStiffnessY) -> String {
    format!("change-storey-stiffness-y building-id={} id={} new-stiffness-y={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_stiffness_y))
}

/// 📖 Parses the arguments of `change-storey-stiffness-y`.
pub fn parse_change_storey_stiffness_y(args: &Args) -> Result<ChangeStoreyStiffnessY, String> {
    let arg = |k: &str| required(args, "change-storey-stiffness-y", k);
    Ok(ChangeStoreyStiffnessY { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_stiffness_y: dec_json(&arg("new-stiffness-y")?)? })
}

/// ✍️ Prints `change-storey-centre-of-mass-xm` as an op-text line.
pub fn print_change_storey_centre_of_mass_x_m(p: &ChangeStoreyCentreOfMassXM) -> String {
    format!("change-storey-centre-of-mass-xm building-id={} id={} new-centre-of-mass-x-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_centre_of_mass_x_m))
}

/// 📖 Parses the arguments of `change-storey-centre-of-mass-xm`.
pub fn parse_change_storey_centre_of_mass_x_m(args: &Args) -> Result<ChangeStoreyCentreOfMassXM, String> {
    let arg = |k: &str| required(args, "change-storey-centre-of-mass-xm", k);
    Ok(ChangeStoreyCentreOfMassXM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_centre_of_mass_x_m: dec_json(&arg("new-centre-of-mass-x-m")?)? })
}

/// ✍️ Prints `change-storey-centre-of-mass-ym` as an op-text line.
pub fn print_change_storey_centre_of_mass_y_m(p: &ChangeStoreyCentreOfMassYM) -> String {
    format!("change-storey-centre-of-mass-ym building-id={} id={} new-centre-of-mass-y-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_centre_of_mass_y_m))
}

/// 📖 Parses the arguments of `change-storey-centre-of-mass-ym`.
pub fn parse_change_storey_centre_of_mass_y_m(args: &Args) -> Result<ChangeStoreyCentreOfMassYM, String> {
    let arg = |k: &str| required(args, "change-storey-centre-of-mass-ym", k);
    Ok(ChangeStoreyCentreOfMassYM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_centre_of_mass_y_m: dec_json(&arg("new-centre-of-mass-y-m")?)? })
}

/// ✍️ Prints `change-storey-centre-of-stiffness-xm` as an op-text line.
pub fn print_change_storey_centre_of_stiffness_x_m(p: &ChangeStoreyCentreOfStiffnessXM) -> String {
    format!("change-storey-centre-of-stiffness-xm building-id={} id={} new-centre-of-stiffness-x-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_centre_of_stiffness_x_m))
}

/// 📖 Parses the arguments of `change-storey-centre-of-stiffness-xm`.
pub fn parse_change_storey_centre_of_stiffness_x_m(args: &Args) -> Result<ChangeStoreyCentreOfStiffnessXM, String> {
    let arg = |k: &str| required(args, "change-storey-centre-of-stiffness-xm", k);
    Ok(ChangeStoreyCentreOfStiffnessXM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_centre_of_stiffness_x_m: dec_json(&arg("new-centre-of-stiffness-x-m")?)? })
}

/// ✍️ Prints `change-storey-centre-of-stiffness-ym` as an op-text line.
pub fn print_change_storey_centre_of_stiffness_y_m(p: &ChangeStoreyCentreOfStiffnessYM) -> String {
    format!("change-storey-centre-of-stiffness-ym building-id={} id={} new-centre-of-stiffness-y-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_centre_of_stiffness_y_m))
}

/// 📖 Parses the arguments of `change-storey-centre-of-stiffness-ym`.
pub fn parse_change_storey_centre_of_stiffness_y_m(args: &Args) -> Result<ChangeStoreyCentreOfStiffnessYM, String> {
    let arg = |k: &str| required(args, "change-storey-centre-of-stiffness-ym", k);
    Ok(ChangeStoreyCentreOfStiffnessYM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_centre_of_stiffness_y_m: dec_json(&arg("new-centre-of-stiffness-y-m")?)? })
}

/// ✍️ Prints `change-storey-drift-ym` as an op-text line.
pub fn print_change_storey_drift_y_m(p: &ChangeStoreyDriftYM) -> String {
    format!("change-storey-drift-ym building-id={} id={} new-drift-y-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_drift_y_m))
}

/// 📖 Parses the arguments of `change-storey-drift-ym`.
pub fn parse_change_storey_drift_y_m(args: &Args) -> Result<ChangeStoreyDriftYM, String> {
    let arg = |k: &str| required(args, "change-storey-drift-ym", k);
    Ok(ChangeStoreyDriftYM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_drift_y_m: dec_json(&arg("new-drift-y-m")?)? })
}

/// ✍️ Prints `change-storey-shear-resistance-n` as an op-text line.
pub fn print_change_storey_shear_resistance_n(p: &ChangeStoreyShearResistanceN) -> String {
    format!("change-storey-shear-resistance-n building-id={} id={} new-shear-resistance-n={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_shear_resistance_n))
}

/// 📖 Parses the arguments of `change-storey-shear-resistance-n`.
pub fn parse_change_storey_shear_resistance_n(args: &Args) -> Result<ChangeStoreyShearResistanceN, String> {
    let arg = |k: &str| required(args, "change-storey-shear-resistance-n", k);
    Ok(ChangeStoreyShearResistanceN { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_shear_resistance_n: dec_json(&arg("new-shear-resistance-n")?)? })
}

/// ✍️ Prints `change-site-vs30-ms` as an op-text line.
pub fn print_change_site_v_s30_m_s(p: &ChangeSiteVS30MS) -> String {
    format!("change-site-vs30-ms new-v-s30-m-s={}", enc_json(&p.new_v_s30_m_s))
}

/// 📖 Parses the arguments of `change-site-vs30-ms`.
pub fn parse_change_site_v_s30_m_s(args: &Args) -> Result<ChangeSiteVS30MS, String> {
    let arg = |k: &str| required(args, "change-site-vs30-ms", k);
    Ok(ChangeSiteVS30MS { new_v_s30_m_s: dec_json(&arg("new-v-s30-m-s")?)? })
}

/// ✍️ Prints `change-site-n-spt` as an op-text line.
pub fn print_change_site_n_spt(p: &ChangeSiteNSpt) -> String {
    format!("change-site-n-spt new-n-spt={}", enc_json(&p.new_n_spt))
}

/// 📖 Parses the arguments of `change-site-n-spt`.
pub fn parse_change_site_n_spt(args: &Args) -> Result<ChangeSiteNSpt, String> {
    let arg = |k: &str| required(args, "change-site-n-spt", k);
    Ok(ChangeSiteNSpt { new_n_spt: dec_json(&arg("new-n-spt")?)? })
}

/// ✍️ Prints `change-site-cu-kpa` as an op-text line.
pub fn print_change_site_c_u_kpa(p: &ChangeSiteCUKpa) -> String {
    format!("change-site-cu-kpa new-c-u-kpa={}", enc_json(&p.new_c_u_kpa))
}

/// 📖 Parses the arguments of `change-site-cu-kpa`.
pub fn parse_change_site_c_u_kpa(args: &Args) -> Result<ChangeSiteCUKpa, String> {
    let arg = |k: &str| required(args, "change-site-cu-kpa", k);
    Ok(ChangeSiteCUKpa { new_c_u_kpa: dec_json(&arg("new-c-u-kpa")?)? })
}

/// ✍️ Prints `change-site-soft-clay-thickness-m` as an op-text line.
pub fn print_change_site_soft_clay_thickness_m(p: &ChangeSiteSoftClayThicknessM) -> String {
    format!("change-site-soft-clay-thickness-m new-soft-clay-thickness-m={}", enc_json(&p.new_soft_clay_thickness_m))
}

/// 📖 Parses the arguments of `change-site-soft-clay-thickness-m`.
pub fn parse_change_site_soft_clay_thickness_m(args: &Args) -> Result<ChangeSiteSoftClayThicknessM, String> {
    let arg = |k: &str| required(args, "change-site-soft-clay-thickness-m", k);
    Ok(ChangeSiteSoftClayThicknessM { new_soft_clay_thickness_m: dec_json(&arg("new-soft-clay-thickness-m")?)? })
}

/// ✍️ Prints `change-site-soft-clay-pi` as an op-text line.
pub fn print_change_site_soft_clay_pi(p: &ChangeSiteSoftClayPi) -> String {
    format!("change-site-soft-clay-pi new-soft-clay-pi={}", enc_json(&p.new_soft_clay_pi))
}

/// 📖 Parses the arguments of `change-site-soft-clay-pi`.
pub fn parse_change_site_soft_clay_pi(args: &Args) -> Result<ChangeSiteSoftClayPi, String> {
    let arg = |k: &str| required(args, "change-site-soft-clay-pi", k);
    Ok(ChangeSiteSoftClayPi { new_soft_clay_pi: dec_json(&arg("new-soft-clay-pi")?)? })
}

/// ✍️ Prints `change-site-liquefiable-layer` as an op-text line.
pub fn print_change_site_liquefiable_layer(p: &ChangeSiteLiquefiableLayer) -> String {
    format!("change-site-liquefiable-layer new-liquefiable-layer={}", enc_json(&p.new_liquefiable_layer))
}

/// 📖 Parses the arguments of `change-site-liquefiable-layer`.
pub fn parse_change_site_liquefiable_layer(args: &Args) -> Result<ChangeSiteLiquefiableLayer, String> {
    let arg = |k: &str| required(args, "change-site-liquefiable-layer", k);
    Ok(ChangeSiteLiquefiableLayer { new_liquefiable_layer: dec_json(&arg("new-liquefiable-layer")?)? })
}

/// ✍️ Prints `change-site-alluvium-thickness-m` as an op-text line.
pub fn print_change_site_alluvium_thickness_m(p: &ChangeSiteAlluviumThicknessM) -> String {
    format!("change-site-alluvium-thickness-m new-alluvium-thickness-m={}", enc_json(&p.new_alluvium_thickness_m))
}

/// 📖 Parses the arguments of `change-site-alluvium-thickness-m`.
pub fn parse_change_site_alluvium_thickness_m(args: &Args) -> Result<ChangeSiteAlluviumThicknessM, String> {
    let arg = |k: &str| required(args, "change-site-alluvium-thickness-m", k);
    Ok(ChangeSiteAlluviumThicknessM { new_alluvium_thickness_m: dec_json(&arg("new-alluvium-thickness-m")?)? })
}

/// ✍️ Prints `change-site-alluvium-vsms` as an op-text line.
pub fn print_change_site_alluvium_v_s_m_s(p: &ChangeSiteAlluviumVSMS) -> String {
    format!("change-site-alluvium-vsms new-alluvium-v-s-m-s={}", enc_json(&p.new_alluvium_v_s_m_s))
}

/// 📖 Parses the arguments of `change-site-alluvium-vsms`.
pub fn parse_change_site_alluvium_v_s_m_s(args: &Args) -> Result<ChangeSiteAlluviumVSMS, String> {
    let arg = |k: &str| required(args, "change-site-alluvium-vsms", k);
    Ok(ChangeSiteAlluviumVSMS { new_alluvium_v_s_m_s: dec_json(&arg("new-alluvium-v-s-m-s")?)? })
}

/// ✍️ Prints `change-site-below-alluvium-vsms` as an op-text line.
pub fn print_change_site_below_alluvium_v_s_m_s(p: &ChangeSiteBelowAlluviumVSMS) -> String {
    format!("change-site-below-alluvium-vsms new-below-alluvium-v-s-m-s={}", enc_json(&p.new_below_alluvium_v_s_m_s))
}

/// 📖 Parses the arguments of `change-site-below-alluvium-vsms`.
pub fn parse_change_site_below_alluvium_v_s_m_s(args: &Args) -> Result<ChangeSiteBelowAlluviumVSMS, String> {
    let arg = |k: &str| required(args, "change-site-below-alluvium-vsms", k);
    Ok(ChangeSiteBelowAlluviumVSMS { new_below_alluvium_v_s_m_s: dec_json(&arg("new-below-alluvium-v-s-m-s")?)? })
}

/// ✍️ Prints `change-building-setback-ratio` as an op-text line.
pub fn print_change_building_setback_ratio(p: &ChangeBuildingSetbackRatio) -> String {
    format!("change-building-setback-ratio id={} new-setback-ratio={}", enc_json(&p.id), enc_json(&p.new_setback_ratio))
}

/// 📖 Parses the arguments of `change-building-setback-ratio`.
pub fn parse_change_building_setback_ratio(args: &Args) -> Result<ChangeBuildingSetbackRatio, String> {
    let arg = |k: &str| required(args, "change-building-setback-ratio", k);
    Ok(ChangeBuildingSetbackRatio { id: dec_json(&arg("id")?)?, new_setback_ratio: dec_json(&arg("new-setback-ratio")?)? })
}

/// ✍️ Prints `change-building-storey-stiffness-ratio` as an op-text line.
pub fn print_change_building_storey_stiffness_ratio(p: &ChangeBuildingStoreyStiffnessRatio) -> String {
    format!("change-building-storey-stiffness-ratio id={} new-storey-stiffness-ratio={}", enc_json(&p.id), enc_json(&p.new_storey_stiffness_ratio))
}

/// 📖 Parses the arguments of `change-building-storey-stiffness-ratio`.
pub fn parse_change_building_storey_stiffness_ratio(args: &Args) -> Result<ChangeBuildingStoreyStiffnessRatio, String> {
    let arg = |k: &str| required(args, "change-building-storey-stiffness-ratio", k);
    Ok(ChangeBuildingStoreyStiffnessRatio { id: dec_json(&arg("id")?)?, new_storey_stiffness_ratio: dec_json(&arg("new-storey-stiffness-ratio")?)? })
}

/// ✍️ Prints `change-building-storey-stiffness-basis` as an op-text line.
pub fn print_change_building_storey_stiffness_basis(p: &ChangeBuildingStoreyStiffnessBasis) -> String {
    format!("change-building-storey-stiffness-basis id={} new-storey-stiffness-basis={}", enc_json(&p.id), enc_json(&p.new_storey_stiffness_basis))
}

/// 📖 Parses the arguments of `change-building-storey-stiffness-basis`.
pub fn parse_change_building_storey_stiffness_basis(args: &Args) -> Result<ChangeBuildingStoreyStiffnessBasis, String> {
    let arg = |k: &str| required(args, "change-building-storey-stiffness-basis", k);
    Ok(ChangeBuildingStoreyStiffnessBasis { id: dec_json(&arg("id")?)?, new_storey_stiffness_basis: dec_json(&arg("new-storey-stiffness-basis")?)? })
}

/// ✍️ Prints `change-building-plan-env-gap-ratio` as an op-text line.
pub fn print_change_building_plan_env_gap_ratio(p: &ChangeBuildingPlanEnvGapRatio) -> String {
    format!("change-building-plan-env-gap-ratio id={} new-plan-envelope-gap-ratio={}", enc_json(&p.id), enc_json(&p.new_plan_envelope_gap_ratio))
}

/// 📖 Parses the arguments of `change-building-plan-env-gap-ratio`.
pub fn parse_change_building_plan_env_gap_ratio(args: &Args) -> Result<ChangeBuildingPlanEnvGapRatio, String> {
    let arg = |k: &str| required(args, "change-building-plan-env-gap-ratio", k);
    Ok(ChangeBuildingPlanEnvGapRatio { id: dec_json(&arg("id")?)?, new_plan_envelope_gap_ratio: dec_json(&arg("new-plan-envelope-gap-ratio")?)? })
}

/// ✍️ Prints `change-building-joint-gap-m` as an op-text line.
pub fn print_change_building_joint_gap_m(p: &ChangeBuildingJointGapM) -> String {
    format!("change-building-joint-gap-m id={} new-joint-gap-m={}", enc_json(&p.id), enc_json(&p.new_joint_gap_m))
}

/// 📖 Parses the arguments of `change-building-joint-gap-m`.
pub fn parse_change_building_joint_gap_m(args: &Args) -> Result<ChangeBuildingJointGapM, String> {
    let arg = |k: &str| required(args, "change-building-joint-gap-m", k);
    Ok(ChangeBuildingJointGapM { id: dec_json(&arg("id")?)?, new_joint_gap_m: dec_json(&arg("new-joint-gap-m")?)? })
}

/// ✍️ Prints `change-building-ns-weight-n` as an op-text line.
pub fn print_change_building_ns_weight_n(p: &ChangeBuildingNsWeightN) -> String {
    format!("change-building-ns-weight-n id={} new-nonstructural-weight-n={}", enc_json(&p.id), enc_json(&p.new_nonstructural_weight_n))
}

/// 📖 Parses the arguments of `change-building-ns-weight-n`.
pub fn parse_change_building_ns_weight_n(args: &Args) -> Result<ChangeBuildingNsWeightN, String> {
    let arg = |k: &str| required(args, "change-building-ns-weight-n", k);
    Ok(ChangeBuildingNsWeightN { id: dec_json(&arg("id")?)?, new_nonstructural_weight_n: dec_json(&arg("new-nonstructural-weight-n")?)? })
}

/// ✍️ Prints `change-building-ns-height-m` as an op-text line.
pub fn print_change_building_ns_height_m(p: &ChangeBuildingNsHeightM) -> String {
    format!("change-building-ns-height-m id={} new-nonstructural-height-m={}", enc_json(&p.id), enc_json(&p.new_nonstructural_height_m))
}

/// 📖 Parses the arguments of `change-building-ns-height-m`.
pub fn parse_change_building_ns_height_m(args: &Args) -> Result<ChangeBuildingNsHeightM, String> {
    let arg = |k: &str| required(args, "change-building-ns-height-m", k);
    Ok(ChangeBuildingNsHeightM { id: dec_json(&arg("id")?)?, new_nonstructural_height_m: dec_json(&arg("new-nonstructural-height-m")?)? })
}

/// ✍️ Prints `change-building-ns-period-s` as an op-text line.
pub fn print_change_building_ns_period_s(p: &ChangeBuildingNsPeriodS) -> String {
    format!("change-building-ns-period-s id={} new-nonstructural-period-s={}", enc_json(&p.id), enc_json(&p.new_nonstructural_period_s))
}

/// 📖 Parses the arguments of `change-building-ns-period-s`.
pub fn parse_change_building_ns_period_s(args: &Args) -> Result<ChangeBuildingNsPeriodS, String> {
    let arg = |k: &str| required(args, "change-building-ns-period-s", k);
    Ok(ChangeBuildingNsPeriodS { id: dec_json(&arg("id")?)?, new_nonstructural_period_s: dec_json(&arg("new-nonstructural-period-s")?)? })
}

/// ✍️ Prints `change-building-ns-q` as an op-text line.
pub fn print_change_building_ns_q(p: &ChangeBuildingNsQ) -> String {
    format!("change-building-ns-q id={} new-nonstructural-q={}", enc_json(&p.id), enc_json(&p.new_nonstructural_q))
}

/// 📖 Parses the arguments of `change-building-ns-q`.
pub fn parse_change_building_ns_q(args: &Args) -> Result<ChangeBuildingNsQ, String> {
    let arg = |k: &str| required(args, "change-building-ns-q", k);
    Ok(ChangeBuildingNsQ { id: dec_json(&arg("id")?)?, new_nonstructural_q: dec_json(&arg("new-nonstructural-q")?)? })
}

/// ✍️ Prints `change-building-ns-resistance-n` as an op-text line.
pub fn print_change_building_ns_resistance_n(p: &ChangeBuildingNsResistanceN) -> String {
    format!("change-building-ns-resistance-n id={} new-nonstructural-resistance-n={}", enc_json(&p.id), enc_json(&p.new_nonstructural_resistance_n))
}

/// 📖 Parses the arguments of `change-building-ns-resistance-n`.
pub fn parse_change_building_ns_resistance_n(args: &Args) -> Result<ChangeBuildingNsResistanceN, String> {
    let arg = |k: &str| required(args, "change-building-ns-resistance-n", k);
    Ok(ChangeBuildingNsResistanceN { id: dec_json(&arg("id")?)?, new_nonstructural_resistance_n: dec_json(&arg("new-nonstructural-resistance-n")?)? })
}

/// ✍️ Prints `change-building-col-moment-sum-nm` as an op-text line.
pub fn print_change_building_col_moment_sum_nm(p: &ChangeBuildingColMomentSumNm) -> String {
    format!("change-building-col-moment-sum-nm id={} new-column-moment-sum-nm={}", enc_json(&p.id), enc_json(&p.new_column_moment_sum_nm))
}

/// 📖 Parses the arguments of `change-building-col-moment-sum-nm`.
pub fn parse_change_building_col_moment_sum_nm(args: &Args) -> Result<ChangeBuildingColMomentSumNm, String> {
    let arg = |k: &str| required(args, "change-building-col-moment-sum-nm", k);
    Ok(ChangeBuildingColMomentSumNm { id: dec_json(&arg("id")?)?, new_column_moment_sum_nm: dec_json(&arg("new-column-moment-sum-nm")?)? })
}

/// ✍️ Prints `change-building-beam-moment-sum-nm` as an op-text line.
pub fn print_change_building_beam_moment_sum_nm(p: &ChangeBuildingBeamMomentSumNm) -> String {
    format!("change-building-beam-moment-sum-nm id={} new-beam-moment-sum-nm={}", enc_json(&p.id), enc_json(&p.new_beam_moment_sum_nm))
}

/// 📖 Parses the arguments of `change-building-beam-moment-sum-nm`.
pub fn parse_change_building_beam_moment_sum_nm(args: &Args) -> Result<ChangeBuildingBeamMomentSumNm, String> {
    let arg = |k: &str| required(args, "change-building-beam-moment-sum-nm", k);
    Ok(ChangeBuildingBeamMomentSumNm { id: dec_json(&arg("id")?)?, new_beam_moment_sum_nm: dec_json(&arg("new-beam-moment-sum-nm")?)? })
}

/// ✍️ Prints `change-storey-variable-category` as an op-text line.
pub fn print_change_storey_variable_category(p: &ChangeStoreyVariableCategory) -> String {
    format!("change-storey-variable-category building-id={} storey-id={} id={} new-category={}", enc_json(&p.building_id), enc_json(&p.storey_id), enc_json(&p.id), enc_json(&p.new_category))
}

/// 📖 Parses the arguments of `change-storey-variable-category`.
pub fn parse_change_storey_variable_category(args: &Args) -> Result<ChangeStoreyVariableCategory, String> {
    let arg = |k: &str| required(args, "change-storey-variable-category", k);
    Ok(ChangeStoreyVariableCategory { building_id: dec_json(&arg("building-id")?)?, storey_id: dec_json(&arg("storey-id")?)?, id: dec_json(&arg("id")?)?, new_category: dec_json(&arg("new-category")?)? })
}

/// ✍️ Prints `change-storey-variable-qk-n` as an op-text line.
pub fn print_change_storey_variable_qk_n(p: &ChangeStoreyVariableQkN) -> String {
    format!("change-storey-variable-qk-n building-id={} storey-id={} id={} new-qk-n={}", enc_json(&p.building_id), enc_json(&p.storey_id), enc_json(&p.id), enc_json(&p.new_qk_n))
}

/// 📖 Parses the arguments of `change-storey-variable-qk-n`.
pub fn parse_change_storey_variable_qk_n(args: &Args) -> Result<ChangeStoreyVariableQkN, String> {
    let arg = |k: &str| required(args, "change-storey-variable-qk-n", k);
    Ok(ChangeStoreyVariableQkN { building_id: dec_json(&arg("building-id")?)?, storey_id: dec_json(&arg("storey-id")?)?, id: dec_json(&arg("id")?)?, new_qk_n: dec_json(&arg("new-qk-n")?)? })
}

/// ✍️ Prints `insert-storey-variable` as an op-text line.
pub fn print_insert_storey_variable(p: &InsertStoreyVariable) -> String {
    format!("insert-storey-variable building-id={} storey-id={} index={} variable={}", enc_json(&p.building_id), enc_json(&p.storey_id), enc_json(&p.index), enc_json(&p.variable))
}

/// 📖 Parses the arguments of `insert-storey-variable`.
pub fn parse_insert_storey_variable(args: &Args) -> Result<InsertStoreyVariable, String> {
    let arg = |k: &str| required(args, "insert-storey-variable", k);
    Ok(InsertStoreyVariable { building_id: dec_json(&arg("building-id")?)?, storey_id: dec_json(&arg("storey-id")?)?, index: opt_index(args)?, variable: dec_json(&arg("variable")?)? })
}

/// ✍️ Prints `remove-storey-variable` as an op-text line.
pub fn print_remove_storey_variable(p: &RemoveStoreyVariable) -> String {
    format!("remove-storey-variable building-id={} storey-id={} id={}", enc_json(&p.building_id), enc_json(&p.storey_id), enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-storey-variable`.
pub fn parse_remove_storey_variable(args: &Args) -> Result<RemoveStoreyVariable, String> {
    let arg = |k: &str| required(args, "remove-storey-variable", k);
    Ok(RemoveStoreyVariable { building_id: dec_json(&arg("building-id")?)?, storey_id: dec_json(&arg("storey-id")?)?, id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `insert-member` as an op-text line.
pub fn print_insert_member(p: &InsertMember) -> String {
    format!("insert-member building-id={} index={} member={}", enc_json(&p.building_id), enc_json(&p.index), enc_json(&p.member))
}

/// 📖 Parses the arguments of `insert-member`.
pub fn parse_insert_member(args: &Args) -> Result<InsertMember, String> {
    let arg = |k: &str| required(args, "insert-member", k);
    Ok(InsertMember { building_id: dec_json(&arg("building-id")?)?, index: opt_index(args)?, member: dec_json(&arg("member")?)? })
}

/// ✍️ Prints `remove-member` as an op-text line.
pub fn print_remove_member(p: &RemoveMember) -> String {
    format!("remove-member building-id={} id={}", enc_json(&p.building_id), enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-member`.
pub fn parse_remove_member(args: &Args) -> Result<RemoveMember, String> {
    let arg = |k: &str| required(args, "remove-member", k);
    Ok(RemoveMember { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `insert-storey` as an op-text line.
pub fn print_insert_storey(p: &InsertStorey) -> String {
    format!("insert-storey building-id={} index={} storey={}", enc_json(&p.building_id), enc_json(&p.index), enc_json(&p.storey))
}

/// 📖 Parses the arguments of `insert-storey`.
pub fn parse_insert_storey(args: &Args) -> Result<InsertStorey, String> {
    let arg = |k: &str| required(args, "insert-storey", k);
    Ok(InsertStorey { building_id: dec_json(&arg("building-id")?)?, index: opt_index(args)?, storey: dec_json(&arg("storey")?)? })
}

/// ✍️ Prints `remove-storey` as an op-text line.
pub fn print_remove_storey(p: &RemoveStorey) -> String {
    format!("remove-storey building-id={} id={}", enc_json(&p.building_id), enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-storey`.
pub fn parse_remove_storey(args: &Args) -> Result<RemoveStorey, String> {
    let arg = |k: &str| required(args, "remove-storey", k);
    Ok(RemoveStorey { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `insert-system` as an op-text line.
pub fn print_insert_system(p: &InsertSystem) -> String {
    format!("insert-system building-id={} index={} system={}", enc_json(&p.building_id), enc_json(&p.index), enc_json(&p.system))
}

/// 📖 Parses the arguments of `insert-system`.
pub fn parse_insert_system(args: &Args) -> Result<InsertSystem, String> {
    let arg = |k: &str| required(args, "insert-system", k);
    Ok(InsertSystem { building_id: dec_json(&arg("building-id")?)?, index: opt_index(args)?, system: dec_json(&arg("system")?)? })
}

/// ✍️ Prints `remove-system` as an op-text line.
pub fn print_remove_system(p: &RemoveSystem) -> String {
    format!("remove-system building-id={} id={}", enc_json(&p.building_id), enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-system`.
pub fn parse_remove_system(args: &Args) -> Result<RemoveSystem, String> {
    let arg = |k: &str| required(args, "remove-system", k);
    Ok(RemoveSystem { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-building-ot-base-xm` as an op-text line.
pub fn print_change_building_ot_base_x_m(p: &ChangeBuildingOtBaseXM) -> String {
    format!("change-building-ot-base-xm id={} new-overturning-base-x-m={}", enc_json(&p.id), enc_json(&p.new_overturning_base_x_m))
}

/// 📖 Parses the arguments of `change-building-ot-base-xm`.
pub fn parse_change_building_ot_base_x_m(args: &Args) -> Result<ChangeBuildingOtBaseXM, String> {
    let arg = |k: &str| required(args, "change-building-ot-base-xm", k);
    Ok(ChangeBuildingOtBaseXM { id: dec_json(&arg("id")?)?, new_overturning_base_x_m: dec_json(&arg("new-overturning-base-x-m")?)? })
}

/// ✍️ Prints `change-building-ot-base-ym` as an op-text line.
pub fn print_change_building_ot_base_y_m(p: &ChangeBuildingOtBaseYM) -> String {
    format!("change-building-ot-base-ym id={} new-overturning-base-y-m={}", enc_json(&p.id), enc_json(&p.new_overturning_base_y_m))
}

/// 📖 Parses the arguments of `change-building-ot-base-ym`.
pub fn parse_change_building_ot_base_y_m(args: &Args) -> Result<ChangeBuildingOtBaseYM, String> {
    let arg = |k: &str| required(args, "change-building-ot-base-ym", k);
    Ok(ChangeBuildingOtBaseYM { id: dec_json(&arg("id")?)?, new_overturning_base_y_m: dec_json(&arg("new-overturning-base-y-m")?)? })
}

/// ✍️ Prints `change-building-tiedown-nm` as an op-text line.
pub fn print_change_building_tiedown_nm(p: &ChangeBuildingTiedownNm) -> String {
    format!("change-building-tiedown-nm id={} new-tie-down-moment-nm={}", enc_json(&p.id), enc_json(&p.new_tie_down_moment_nm))
}

/// 📖 Parses the arguments of `change-building-tiedown-nm`.
pub fn parse_change_building_tiedown_nm(args: &Args) -> Result<ChangeBuildingTiedownNm, String> {
    let arg = |k: &str| required(args, "change-building-tiedown-nm", k);
    Ok(ChangeBuildingTiedownNm { id: dec_json(&arg("id")?)?, new_tie_down_moment_nm: dec_json(&arg("new-tie-down-moment-nm")?)? })
}

/// ✍️ Prints `change-diaphragm-building-id` as an op-text line.
pub fn print_change_diaphragm_building_id(p: &ChangeDiaphragmBuildingId) -> String {
    format!("change-diaphragm-building-id id={} new-building-id={}", enc_json(&p.id), enc_json(&p.new_building_id))
}

/// 📖 Parses the arguments of `change-diaphragm-building-id`.
pub fn parse_change_diaphragm_building_id(args: &Args) -> Result<ChangeDiaphragmBuildingId, String> {
    let arg = |k: &str| required(args, "change-diaphragm-building-id", k);
    Ok(ChangeDiaphragmBuildingId { id: dec_json(&arg("id")?)?, new_building_id: dec_json(&arg("new-building-id")?)? })
}

/// ✍️ Prints `change-diaphragm-storey-id` as an op-text line.
pub fn print_change_diaphragm_storey_id(p: &ChangeDiaphragmStoreyId) -> String {
    format!("change-diaphragm-storey-id id={} new-storey-id={}", enc_json(&p.id), enc_json(&p.new_storey_id))
}

/// 📖 Parses the arguments of `change-diaphragm-storey-id`.
pub fn parse_change_diaphragm_storey_id(args: &Args) -> Result<ChangeDiaphragmStoreyId, String> {
    let arg = |k: &str| required(args, "change-diaphragm-storey-id", k);
    Ok(ChangeDiaphragmStoreyId { id: dec_json(&arg("id")?)?, new_storey_id: dec_json(&arg("new-storey-id")?)? })
}

/// ✍️ Prints `change-diaphragm-material` as an op-text line.
pub fn print_change_diaphragm_material(p: &ChangeDiaphragmMaterial) -> String {
    format!("change-diaphragm-material id={} new-material={}", enc_json(&p.id), enc_json(&p.new_material))
}

/// 📖 Parses the arguments of `change-diaphragm-material`.
pub fn parse_change_diaphragm_material(args: &Args) -> Result<ChangeDiaphragmMaterial, String> {
    let arg = |k: &str| required(args, "change-diaphragm-material", k);
    Ok(ChangeDiaphragmMaterial { id: dec_json(&arg("id")?)?, new_material: dec_json(&arg("new-material")?)? })
}

/// ✍️ Prints `change-diaphragm-thickness-m` as an op-text line.
pub fn print_change_diaphragm_thickness_m(p: &ChangeDiaphragmThicknessM) -> String {
    format!("change-diaphragm-thickness-m id={} new-thickness-m={}", enc_json(&p.id), enc_json(&p.new_thickness_m))
}

/// 📖 Parses the arguments of `change-diaphragm-thickness-m`.
pub fn parse_change_diaphragm_thickness_m(args: &Args) -> Result<ChangeDiaphragmThicknessM, String> {
    let arg = |k: &str| required(args, "change-diaphragm-thickness-m", k);
    Ok(ChangeDiaphragmThicknessM { id: dec_json(&arg("id")?)?, new_thickness_m: dec_json(&arg("new-thickness-m")?)? })
}

/// ✍️ Prints `change-diaphragm-span-m` as an op-text line.
pub fn print_change_diaphragm_span_m(p: &ChangeDiaphragmSpanM) -> String {
    format!("change-diaphragm-span-m id={} new-span-m={}", enc_json(&p.id), enc_json(&p.new_span_m))
}

/// 📖 Parses the arguments of `change-diaphragm-span-m`.
pub fn parse_change_diaphragm_span_m(args: &Args) -> Result<ChangeDiaphragmSpanM, String> {
    let arg = |k: &str| required(args, "change-diaphragm-span-m", k);
    Ok(ChangeDiaphragmSpanM { id: dec_json(&arg("id")?)?, new_span_m: dec_json(&arg("new-span-m")?)? })
}

/// ✍️ Prints `change-diaphragm-depth-m` as an op-text line.
pub fn print_change_diaphragm_depth_m(p: &ChangeDiaphragmDepthM) -> String {
    format!("change-diaphragm-depth-m id={} new-depth-m={}", enc_json(&p.id), enc_json(&p.new_depth_m))
}

/// 📖 Parses the arguments of `change-diaphragm-depth-m`.
pub fn parse_change_diaphragm_depth_m(args: &Args) -> Result<ChangeDiaphragmDepthM, String> {
    let arg = |k: &str| required(args, "change-diaphragm-depth-m", k);
    Ok(ChangeDiaphragmDepthM { id: dec_json(&arg("id")?)?, new_depth_m: dec_json(&arg("new-depth-m")?)? })
}

/// ✍️ Prints `change-diaphragm-opening-width-m` as an op-text line.
pub fn print_change_diaphragm_opening_width_m(p: &ChangeDiaphragmOpeningWidthM) -> String {
    format!("change-diaphragm-opening-width-m id={} new-opening-width-m={}", enc_json(&p.id), enc_json(&p.new_opening_width_m))
}

/// 📖 Parses the arguments of `change-diaphragm-opening-width-m`.
pub fn parse_change_diaphragm_opening_width_m(args: &Args) -> Result<ChangeDiaphragmOpeningWidthM, String> {
    let arg = |k: &str| required(args, "change-diaphragm-opening-width-m", k);
    Ok(ChangeDiaphragmOpeningWidthM { id: dec_json(&arg("id")?)?, new_opening_width_m: dec_json(&arg("new-opening-width-m")?)? })
}

/// ✍️ Prints `change-diaphragm-shear-str-mpa` as an op-text line.
pub fn print_change_diaphragm_shear_str_mpa(p: &ChangeDiaphragmShearStrMpa) -> String {
    format!("change-diaphragm-shear-str-mpa id={} new-shear-strength-mpa={}", enc_json(&p.id), enc_json(&p.new_shear_strength_mpa))
}

/// 📖 Parses the arguments of `change-diaphragm-shear-str-mpa`.
pub fn parse_change_diaphragm_shear_str_mpa(args: &Args) -> Result<ChangeDiaphragmShearStrMpa, String> {
    let arg = |k: &str| required(args, "change-diaphragm-shear-str-mpa", k);
    Ok(ChangeDiaphragmShearStrMpa { id: dec_json(&arg("id")?)?, new_shear_strength_mpa: dec_json(&arg("new-shear-strength-mpa")?)? })
}

/// ✍️ Prints `change-diaphragm-chord-area-m2` as an op-text line.
pub fn print_change_diaphragm_chord_area_m2(p: &ChangeDiaphragmChordAreaM2) -> String {
    format!("change-diaphragm-chord-area-m2 id={} new-chord-area-m2={}", enc_json(&p.id), enc_json(&p.new_chord_area_m2))
}

/// 📖 Parses the arguments of `change-diaphragm-chord-area-m2`.
pub fn parse_change_diaphragm_chord_area_m2(args: &Args) -> Result<ChangeDiaphragmChordAreaM2, String> {
    let arg = |k: &str| required(args, "change-diaphragm-chord-area-m2", k);
    Ok(ChangeDiaphragmChordAreaM2 { id: dec_json(&arg("id")?)?, new_chord_area_m2: dec_json(&arg("new-chord-area-m2")?)? })
}

/// ✍️ Prints `change-diaphragm-chord-str-mpa` as an op-text line.
pub fn print_change_diaphragm_chord_str_mpa(p: &ChangeDiaphragmChordStrMpa) -> String {
    format!("change-diaphragm-chord-str-mpa id={} new-chord-strength-mpa={}", enc_json(&p.id), enc_json(&p.new_chord_strength_mpa))
}

/// 📖 Parses the arguments of `change-diaphragm-chord-str-mpa`.
pub fn parse_change_diaphragm_chord_str_mpa(args: &Args) -> Result<ChangeDiaphragmChordStrMpa, String> {
    let arg = |k: &str| required(args, "change-diaphragm-chord-str-mpa", k);
    Ok(ChangeDiaphragmChordStrMpa { id: dec_json(&arg("id")?)?, new_chord_strength_mpa: dec_json(&arg("new-chord-strength-mpa")?)? })
}

/// ✍️ Prints `change-diaphragm-gamma-d` as an op-text line.
pub fn print_change_diaphragm_gamma_d(p: &ChangeDiaphragmGammaD) -> String {
    format!("change-diaphragm-gamma-d id={} new-gamma-d={}", enc_json(&p.id), enc_json(&p.new_gamma_d))
}

/// 📖 Parses the arguments of `change-diaphragm-gamma-d`.
pub fn parse_change_diaphragm_gamma_d(args: &Args) -> Result<ChangeDiaphragmGammaD, String> {
    let arg = |k: &str| required(args, "change-diaphragm-gamma-d", k);
    Ok(ChangeDiaphragmGammaD { id: dec_json(&arg("id")?)?, new_gamma_d: dec_json(&arg("new-gamma-d")?)? })
}

/// ✍️ Prints `change-accelerogram-building-id` as an op-text line.
pub fn print_change_accelerogram_building_id(p: &ChangeAccelerogramBuildingId) -> String {
    format!("change-accelerogram-building-id id={} new-building-id={}", enc_json(&p.id), enc_json(&p.new_building_id))
}

/// 📖 Parses the arguments of `change-accelerogram-building-id`.
pub fn parse_change_accelerogram_building_id(args: &Args) -> Result<ChangeAccelerogramBuildingId, String> {
    let arg = |k: &str| required(args, "change-accelerogram-building-id", k);
    Ok(ChangeAccelerogramBuildingId { id: dec_json(&arg("id")?)?, new_building_id: dec_json(&arg("new-building-id")?)? })
}

/// ✍️ Prints `change-accelerogram-label` as an op-text line.
pub fn print_change_accelerogram_label(p: &ChangeAccelerogramLabel) -> String {
    format!("change-accelerogram-label id={} new-label={}", enc_json(&p.id), enc_json(&p.new_label))
}

/// 📖 Parses the arguments of `change-accelerogram-label`.
pub fn parse_change_accelerogram_label(args: &Args) -> Result<ChangeAccelerogramLabel, String> {
    let arg = |k: &str| required(args, "change-accelerogram-label", k);
    Ok(ChangeAccelerogramLabel { id: dec_json(&arg("id")?)?, new_label: dec_json(&arg("new-label")?)? })
}

/// ✍️ Prints `change-accelerogram-samples` as an op-text line.
pub fn print_change_accelerogram_samples(p: &ChangeAccelerogramSamples) -> String {
    format!("change-accelerogram-samples id={} new-samples={}", enc_json(&p.id), enc_json(&p.new_samples))
}

/// 📖 Parses the arguments of `change-accelerogram-samples`.
pub fn parse_change_accelerogram_samples(args: &Args) -> Result<ChangeAccelerogramSamples, String> {
    let arg = |k: &str| required(args, "change-accelerogram-samples", k);
    Ok(ChangeAccelerogramSamples { id: dec_json(&arg("id")?)?, new_samples: dec_json(&arg("new-samples")?)? })
}

/// ✍️ Prints `change-accelerogram-dt-s` as an op-text line.
pub fn print_change_accelerogram_dt_s(p: &ChangeAccelerogramDtS) -> String {
    format!("change-accelerogram-dt-s id={} new-dt-s={}", enc_json(&p.id), enc_json(&p.new_dt_s))
}

/// 📖 Parses the arguments of `change-accelerogram-dt-s`.
pub fn parse_change_accelerogram_dt_s(args: &Args) -> Result<ChangeAccelerogramDtS, String> {
    let arg = |k: &str| required(args, "change-accelerogram-dt-s", k);
    Ok(ChangeAccelerogramDtS { id: dec_json(&arg("id")?)?, new_dt_s: dec_json(&arg("new-dt-s")?)? })
}

/// ✍️ Prints `change-accelerogram-scale-factor` as an op-text line.
pub fn print_change_accelerogram_scale_factor(p: &ChangeAccelerogramScaleFactor) -> String {
    format!("change-accelerogram-scale-factor id={} new-scale-factor={}", enc_json(&p.id), enc_json(&p.new_scale_factor))
}

/// 📖 Parses the arguments of `change-accelerogram-scale-factor`.
pub fn parse_change_accelerogram_scale_factor(args: &Args) -> Result<ChangeAccelerogramScaleFactor, String> {
    let arg = |k: &str| required(args, "change-accelerogram-scale-factor", k);
    Ok(ChangeAccelerogramScaleFactor { id: dec_json(&arg("id")?)?, new_scale_factor: dec_json(&arg("new-scale-factor")?)? })
}

/// ✍️ Prints `insert-diaphragm` as an op-text line.
pub fn print_insert_diaphragm(p: &InsertDiaphragm) -> String {
    format!("insert-diaphragm index={} diaphragm={}", enc_json(&p.index), enc_json(&p.diaphragm))
}

/// 📖 Parses the arguments of `insert-diaphragm`.
pub fn parse_insert_diaphragm(args: &Args) -> Result<InsertDiaphragm, String> {
    let arg = |k: &str| required(args, "insert-diaphragm", k);
    Ok(InsertDiaphragm { index: opt_index(args)?, diaphragm: dec_json(&arg("diaphragm")?)? })
}

/// ✍️ Prints `remove-diaphragm` as an op-text line.
pub fn print_remove_diaphragm(p: &RemoveDiaphragm) -> String {
    format!("remove-diaphragm id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-diaphragm`.
pub fn parse_remove_diaphragm(args: &Args) -> Result<RemoveDiaphragm, String> {
    let arg = |k: &str| required(args, "remove-diaphragm", k);
    Ok(RemoveDiaphragm { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `insert-accelerogram` as an op-text line.
pub fn print_insert_accelerogram(p: &InsertAccelerogram) -> String {
    format!("insert-accelerogram index={} accelerogram={}", enc_json(&p.index), enc_json(&p.accelerogram))
}

/// 📖 Parses the arguments of `insert-accelerogram`.
pub fn parse_insert_accelerogram(args: &Args) -> Result<InsertAccelerogram, String> {
    let arg = |k: &str| required(args, "insert-accelerogram", k);
    Ok(InsertAccelerogram { index: opt_index(args)?, accelerogram: dec_json(&arg("accelerogram")?)? })
}

/// ✍️ Prints `remove-accelerogram` as an op-text line.
pub fn print_remove_accelerogram(p: &RemoveAccelerogram) -> String {
    format!("remove-accelerogram id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-accelerogram`.
pub fn parse_remove_accelerogram(args: &Args) -> Result<RemoveAccelerogram, String> {
    let arg = |k: &str| required(args, "remove-accelerogram", k);
    Ok(RemoveAccelerogram { id: dec_json(&arg("id")?)? })
}
