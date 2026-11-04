//! 🧱️ EN 1998 members mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_members::change_member_detailing::ChangeMemberDetailing;
use semio_s_artifact_norm_en1998_mut_members::change_masonry_wall_ratio::ChangeMasonryWallRatio;
use semio_s_artifact_norm_en1998_mut_members::insert_assessment::InsertAssessment;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_rkn::ChangeAssessmentRKN;
use semio_s_artifact_norm_en1998_mut_members::remove_assessment::RemoveAssessment;
use semio_s_artifact_norm_en1998_mut_members::change_member_material::ChangeMemberMaterial;
use semio_s_artifact_norm_en1998_mut_members::change_member_role::ChangeMemberRole;
use semio_s_artifact_norm_en1998_mut_members::change_member_min_dimension_m::ChangeMemberMinDimensionM;
use semio_s_artifact_norm_en1998_mut_members::change_member_rho::ChangeMemberRho;
use semio_s_artifact_norm_en1998_mut_members::change_member_rho_prime::ChangeMemberRhoPrime;
use semio_s_artifact_norm_en1998_mut_members::change_member_omega_wd::ChangeMemberOmegaWd;
use semio_s_artifact_norm_en1998_mut_members::change_member_steel_section_class::ChangeMemberSteelSectionClass;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_knowledge_level::ChangeAssessmentKnowledgeLevel;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_limit_state::ChangeAssessmentLimitState;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_supported_building_id::ChangeAssessmentSupportedBuildingId;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_gamma_el::ChangeAssessmentGammaEl;
use semio_s_artifact_norm_en1998_mut_members::change_member_fastener_diameter_mm::ChangeMemberFastenerDiameterMm;
use semio_s_artifact_norm_en1998_mut_members::change_member_timber_thickness_mm::ChangeMemberTimberThicknessMm;
use semio_s_artifact_norm_en1998_mut_members::change_member_wall_thickness_m::ChangeMemberWallThicknessM;
use semio_s_artifact_norm_en1998_mut_members::change_member_effective_stiffness_ratio::ChangeMemberEffectiveStiffnessRatio;
use semio_s_artifact_norm_en1998_mut_members::change_member_effective_stiffness_basis::ChangeMemberEffectiveStiffnessBasis;
use semio_s_artifact_norm_en1998_mut_members::change_member_wall_poisson_ratio::ChangeMemberWallPoissonRatio;
use semio_s_artifact_norm_en1998_mut_members::change_member_wall_start_x_m::ChangeMemberWallStartXM;
use semio_s_artifact_norm_en1998_mut_members::change_member_wall_start_y_m::ChangeMemberWallStartYM;
use semio_s_artifact_norm_en1998_mut_members::change_member_wall_length_m::ChangeMemberWallLengthM;
use semio_s_artifact_norm_en1998_mut_members::change_member_wall_direction_rad::ChangeMemberWallDirectionRad;
use semio_s_artifact_norm_en1998_mut_members::change_member_wall_storey_id::ChangeMemberWallStoreyId;
use semio_s_artifact_norm_en1998_mut_members::change_member_wall_eff_height_m::ChangeMemberWallEffHeightM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_chord_demand_rad::ChangeAssessmentChordDemandRad;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_chord_yield_rad::ChangeAssessmentChordYieldRad;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_chord_ult_rad::ChangeAssessmentChordUltRad;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_section_depth_m::ChangeAssessmentSectionDepthM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_section_width_m::ChangeAssessmentSectionWidthM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_eff_depth_m::ChangeAssessmentEffDepthM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_shear_span_m::ChangeAssessmentShearSpanM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_axial_force_n::ChangeAssessmentAxialForceN;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_shear_force_demand_n::ChangeAssessmentShearForceDemandN;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_as1_area_m2::ChangeAssessmentAs1AreaM2;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_as2_area_m2::ChangeAssessmentAs2AreaM2;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_bar_diameter_mm::ChangeAssessmentBarDiameterMm;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_steel_yield_mpa::ChangeAssessmentSteelYieldMpa;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_fc_mpa::ChangeAssessmentFcMpa;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_strp_leg_area_m2::ChangeAssessmentStrpLegAreaM2;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_strp_spacing_m::ChangeAssessmentStrpSpacingM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_strp_yield_mpa::ChangeAssessmentStrpYieldMpa;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_core_width_m::ChangeAssessmentCoreWidthM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_core_depth_m::ChangeAssessmentCoreDepthM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_rbar_count::ChangeAssessmentRbarCount;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_rbar_spacing_m::ChangeAssessmentRbarSpacingM;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_diag_reinf_ratio::ChangeAssessmentDiagReinfRatio;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_shear_crack::ChangeAssessmentShearCrack;
use semio_s_artifact_norm_en1998_mut_members::change_joint_building_id::ChangeJointBuildingId;
use semio_s_artifact_norm_en1998_mut_members::change_joint_storey_id::ChangeJointStoreyId;
use semio_s_artifact_norm_en1998_mut_members::change_joint_column_id::ChangeJointColumnId;
use semio_s_artifact_norm_en1998_mut_members::change_joint_beam_id::ChangeJointBeamId;
use semio_s_artifact_norm_en1998_mut_members::change_joint_x_m::ChangeJointXM;
use semio_s_artifact_norm_en1998_mut_members::change_joint_y_m::ChangeJointYM;
use semio_s_artifact_norm_en1998_mut_members::change_joint_column_moment_rd_nm::ChangeJointColumnMomentRdNm;
use semio_s_artifact_norm_en1998_mut_members::change_joint_beam_moment_rd_nm::ChangeJointBeamMomentRdNm;
use semio_s_artifact_norm_en1998_mut_members::insert_joint::InsertJoint;
use semio_s_artifact_norm_en1998_mut_members::insert_tie_beam::InsertTieBeam;
use semio_s_artifact_norm_en1998_mut_members::insert_seismic_slope::InsertSeismicSlope;
use semio_s_artifact_norm_en1998_mut_members::remove_joint::RemoveJoint;
use semio_s_artifact_norm_en1998_mut_members::remove_tie_beam::RemoveTieBeam;
use semio_s_artifact_norm_en1998_mut_members::remove_seismic_slope::RemoveSeismicSlope;
use semio_s_artifact_norm_en1998_mut_members::change_joint_col_moment_rd_nm::ChangeJointColMomentRdNm;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_mid_steel_area_m2::ChangeAssessmentMidSteelAreaM2;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_analysis_method::ChangeAssessmentAnalysisMethod;

/// ✍️ Prints `change-member-detailing` as an op-text line.
pub fn print_change_member_detailing(p: &ChangeMemberDetailing) -> String {
    format!("change-member-detailing building-id={} id={} new-detailing-compatible-with-q={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_detailing_compatible_with_q))
}

/// 📖 Parses the arguments of `change-member-detailing`.
pub fn parse_change_member_detailing(args: &Args) -> Result<ChangeMemberDetailing, String> {
    let arg = |k: &str| required(args, "change-member-detailing", k);
    Ok(ChangeMemberDetailing { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_detailing_compatible_with_q: dec_json(&arg("new-detailing-compatible-with-q")?)? })
}

/// ✍️ Prints `change-masonry-wall-ratio` as an op-text line.
pub fn print_change_masonry_wall_ratio(p: &ChangeMasonryWallRatio) -> String {
    format!("change-masonry-wall-ratio id={} new-masonry-wall-area-ratio={}", enc_json(&p.id), enc_json(&p.new_masonry_wall_area_ratio))
}

/// 📖 Parses the arguments of `change-masonry-wall-ratio`.
pub fn parse_change_masonry_wall_ratio(args: &Args) -> Result<ChangeMasonryWallRatio, String> {
    let arg = |k: &str| required(args, "change-masonry-wall-ratio", k);
    Ok(ChangeMasonryWallRatio { id: dec_json(&arg("id")?)?, new_masonry_wall_area_ratio: dec_json(&arg("new-masonry-wall-area-ratio")?)? })
}

/// ✍️ Prints `insert-assessment` as an op-text line.
pub fn print_insert_assessment(p: &InsertAssessment) -> String {
    format!("insert-assessment index={} assessment={}", enc_json(&p.index), enc_json(&p.assessment))
}

/// 📖 Parses the arguments of `insert-assessment`.
pub fn parse_insert_assessment(args: &Args) -> Result<InsertAssessment, String> {
    let arg = |k: &str| required(args, "insert-assessment", k);
    Ok(InsertAssessment { index: opt_index(args)?, assessment: dec_json(&arg("assessment")?)? })
}

/// ✍️ Prints `change-assessment-rkn` as an op-text line.
pub fn print_change_assessment_rkn(p: &ChangeAssessmentRKN) -> String {
    format!("change-assessment-rkn id={} new-r-k-n={}", enc_json(&p.id), enc_json(&p.new_r_k_n))
}

/// 📖 Parses the arguments of `change-assessment-rkn`.
pub fn parse_change_assessment_rkn(args: &Args) -> Result<ChangeAssessmentRKN, String> {
    let arg = |k: &str| required(args, "change-assessment-rkn", k);
    Ok(ChangeAssessmentRKN { id: dec_json(&arg("id")?)?, new_r_k_n: dec_json(&arg("new-r-k-n")?)? })
}

/// ✍️ Prints `remove-assessment` as an op-text line.
pub fn print_remove_assessment(p: &RemoveAssessment) -> String {
    format!("remove-assessment id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-assessment`.
pub fn parse_remove_assessment(args: &Args) -> Result<RemoveAssessment, String> {
    let arg = |k: &str| required(args, "remove-assessment", k);
    Ok(RemoveAssessment { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-member-material` as an op-text line.
pub fn print_change_member_material(p: &ChangeMemberMaterial) -> String {
    format!("change-member-material building-id={} id={} new-material={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_material))
}

/// 📖 Parses the arguments of `change-member-material`.
pub fn parse_change_member_material(args: &Args) -> Result<ChangeMemberMaterial, String> {
    let arg = |k: &str| required(args, "change-member-material", k);
    Ok(ChangeMemberMaterial { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_material: dec_json(&arg("new-material")?)? })
}

/// ✍️ Prints `change-member-role` as an op-text line.
pub fn print_change_member_role(p: &ChangeMemberRole) -> String {
    format!("change-member-role building-id={} id={} new-role={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_role))
}

/// 📖 Parses the arguments of `change-member-role`.
pub fn parse_change_member_role(args: &Args) -> Result<ChangeMemberRole, String> {
    let arg = |k: &str| required(args, "change-member-role", k);
    Ok(ChangeMemberRole { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_role: dec_json(&arg("new-role")?)? })
}

/// ✍️ Prints `change-member-min-dimension-m` as an op-text line.
pub fn print_change_member_min_dimension_m(p: &ChangeMemberMinDimensionM) -> String {
    format!("change-member-min-dimension-m building-id={} id={} new-min-dimension-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_min_dimension_m))
}

/// 📖 Parses the arguments of `change-member-min-dimension-m`.
pub fn parse_change_member_min_dimension_m(args: &Args) -> Result<ChangeMemberMinDimensionM, String> {
    let arg = |k: &str| required(args, "change-member-min-dimension-m", k);
    Ok(ChangeMemberMinDimensionM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_min_dimension_m: dec_json(&arg("new-min-dimension-m")?)? })
}

/// ✍️ Prints `change-member-rho` as an op-text line.
pub fn print_change_member_rho(p: &ChangeMemberRho) -> String {
    format!("change-member-rho building-id={} id={} new-rho={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_rho))
}

/// 📖 Parses the arguments of `change-member-rho`.
pub fn parse_change_member_rho(args: &Args) -> Result<ChangeMemberRho, String> {
    let arg = |k: &str| required(args, "change-member-rho", k);
    Ok(ChangeMemberRho { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_rho: dec_json(&arg("new-rho")?)? })
}

/// ✍️ Prints `change-member-rho-prime` as an op-text line.
pub fn print_change_member_rho_prime(p: &ChangeMemberRhoPrime) -> String {
    format!("change-member-rho-prime building-id={} id={} new-rho-prime={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_rho_prime))
}

/// 📖 Parses the arguments of `change-member-rho-prime`.
pub fn parse_change_member_rho_prime(args: &Args) -> Result<ChangeMemberRhoPrime, String> {
    let arg = |k: &str| required(args, "change-member-rho-prime", k);
    Ok(ChangeMemberRhoPrime { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_rho_prime: dec_json(&arg("new-rho-prime")?)? })
}

/// ✍️ Prints `change-member-omega-wd` as an op-text line.
pub fn print_change_member_omega_wd(p: &ChangeMemberOmegaWd) -> String {
    format!("change-member-omega-wd building-id={} id={} new-omega-wd={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_omega_wd))
}

/// 📖 Parses the arguments of `change-member-omega-wd`.
pub fn parse_change_member_omega_wd(args: &Args) -> Result<ChangeMemberOmegaWd, String> {
    let arg = |k: &str| required(args, "change-member-omega-wd", k);
    Ok(ChangeMemberOmegaWd { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_omega_wd: dec_json(&arg("new-omega-wd")?)? })
}

/// ✍️ Prints `change-member-steel-section-class` as an op-text line.
pub fn print_change_member_steel_section_class(p: &ChangeMemberSteelSectionClass) -> String {
    format!("change-member-steel-section-class building-id={} id={} new-steel-section-class={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_steel_section_class))
}

/// 📖 Parses the arguments of `change-member-steel-section-class`.
pub fn parse_change_member_steel_section_class(args: &Args) -> Result<ChangeMemberSteelSectionClass, String> {
    let arg = |k: &str| required(args, "change-member-steel-section-class", k);
    Ok(ChangeMemberSteelSectionClass { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_steel_section_class: dec_json(&arg("new-steel-section-class")?)? })
}

/// ✍️ Prints `change-assessment-knowledge-level` as an op-text line.
pub fn print_change_assessment_knowledge_level(p: &ChangeAssessmentKnowledgeLevel) -> String {
    format!("change-assessment-knowledge-level id={} new-knowledge-level={}", enc_json(&p.id), enc_json(&p.new_knowledge_level))
}

/// 📖 Parses the arguments of `change-assessment-knowledge-level`.
pub fn parse_change_assessment_knowledge_level(args: &Args) -> Result<ChangeAssessmentKnowledgeLevel, String> {
    let arg = |k: &str| required(args, "change-assessment-knowledge-level", k);
    Ok(ChangeAssessmentKnowledgeLevel { id: dec_json(&arg("id")?)?, new_knowledge_level: dec_json(&arg("new-knowledge-level")?)? })
}

/// ✍️ Prints `change-assessment-limit-state` as an op-text line.
pub fn print_change_assessment_limit_state(p: &ChangeAssessmentLimitState) -> String {
    format!("change-assessment-limit-state id={} new-limit-state={}", enc_json(&p.id), enc_json(&p.new_limit_state))
}

/// 📖 Parses the arguments of `change-assessment-limit-state`.
pub fn parse_change_assessment_limit_state(args: &Args) -> Result<ChangeAssessmentLimitState, String> {
    let arg = |k: &str| required(args, "change-assessment-limit-state", k);
    Ok(ChangeAssessmentLimitState { id: dec_json(&arg("id")?)?, new_limit_state: dec_json(&arg("new-limit-state")?)? })
}

/// ✍️ Prints `change-assessment-supported-building-id` as an op-text line.
pub fn print_change_assessment_supported_building_id(p: &ChangeAssessmentSupportedBuildingId) -> String {
    format!("change-assessment-supported-building-id id={} new-supported-building-id={}", enc_json(&p.id), enc_json(&p.new_supported_building_id))
}

/// 📖 Parses the arguments of `change-assessment-supported-building-id`.
pub fn parse_change_assessment_supported_building_id(args: &Args) -> Result<ChangeAssessmentSupportedBuildingId, String> {
    let arg = |k: &str| required(args, "change-assessment-supported-building-id", k);
    Ok(ChangeAssessmentSupportedBuildingId { id: dec_json(&arg("id")?)?, new_supported_building_id: dec_json(&arg("new-supported-building-id")?)? })
}

/// ✍️ Prints `change-assessment-gamma-el` as an op-text line.
pub fn print_change_assessment_gamma_el(p: &ChangeAssessmentGammaEl) -> String {
    format!("change-assessment-gamma-el id={} new-gamma-el={}", enc_json(&p.id), enc_json(&p.new_gamma_el))
}

/// 📖 Parses the arguments of `change-assessment-gamma-el`.
pub fn parse_change_assessment_gamma_el(args: &Args) -> Result<ChangeAssessmentGammaEl, String> {
    let arg = |k: &str| required(args, "change-assessment-gamma-el", k);
    Ok(ChangeAssessmentGammaEl { id: dec_json(&arg("id")?)?, new_gamma_el: dec_json(&arg("new-gamma-el")?)? })
}

/// ✍️ Prints `change-member-fastener-diameter-mm` as an op-text line.
pub fn print_change_member_fastener_diameter_mm(p: &ChangeMemberFastenerDiameterMm) -> String {
    format!("change-member-fastener-diameter-mm building-id={} id={} new-fastener-diameter-mm={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_fastener_diameter_mm))
}

/// 📖 Parses the arguments of `change-member-fastener-diameter-mm`.
pub fn parse_change_member_fastener_diameter_mm(args: &Args) -> Result<ChangeMemberFastenerDiameterMm, String> {
    let arg = |k: &str| required(args, "change-member-fastener-diameter-mm", k);
    Ok(ChangeMemberFastenerDiameterMm { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_fastener_diameter_mm: dec_json(&arg("new-fastener-diameter-mm")?)? })
}

/// ✍️ Prints `change-member-timber-thickness-mm` as an op-text line.
pub fn print_change_member_timber_thickness_mm(p: &ChangeMemberTimberThicknessMm) -> String {
    format!("change-member-timber-thickness-mm building-id={} id={} new-timber-thickness-mm={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_timber_thickness_mm))
}

/// 📖 Parses the arguments of `change-member-timber-thickness-mm`.
pub fn parse_change_member_timber_thickness_mm(args: &Args) -> Result<ChangeMemberTimberThicknessMm, String> {
    let arg = |k: &str| required(args, "change-member-timber-thickness-mm", k);
    Ok(ChangeMemberTimberThicknessMm { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_timber_thickness_mm: dec_json(&arg("new-timber-thickness-mm")?)? })
}

/// ✍️ Prints `change-member-wall-thickness-m` as an op-text line.
pub fn print_change_member_wall_thickness_m(p: &ChangeMemberWallThicknessM) -> String {
    format!("change-member-wall-thickness-m building-id={} id={} new-wall-thickness-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_wall_thickness_m))
}

/// 📖 Parses the arguments of `change-member-wall-thickness-m`.
pub fn parse_change_member_wall_thickness_m(args: &Args) -> Result<ChangeMemberWallThicknessM, String> {
    let arg = |k: &str| required(args, "change-member-wall-thickness-m", k);
    Ok(ChangeMemberWallThicknessM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_wall_thickness_m: dec_json(&arg("new-wall-thickness-m")?)? })
}

/// ✍️ Prints `change-member-effective-stiffness-ratio` as an op-text line.
pub fn print_change_member_effective_stiffness_ratio(p: &ChangeMemberEffectiveStiffnessRatio) -> String {
    format!("change-member-effective-stiffness-ratio building-id={} id={} new-effective-stiffness-ratio={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_effective_stiffness_ratio))
}

/// 📖 Parses the arguments of `change-member-effective-stiffness-ratio`.
pub fn parse_change_member_effective_stiffness_ratio(args: &Args) -> Result<ChangeMemberEffectiveStiffnessRatio, String> {
    let arg = |k: &str| required(args, "change-member-effective-stiffness-ratio", k);
    Ok(ChangeMemberEffectiveStiffnessRatio { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_effective_stiffness_ratio: dec_json(&arg("new-effective-stiffness-ratio")?)? })
}

/// ✍️ Prints `change-member-effective-stiffness-basis` as an op-text line.
pub fn print_change_member_effective_stiffness_basis(p: &ChangeMemberEffectiveStiffnessBasis) -> String {
    format!("change-member-effective-stiffness-basis building-id={} id={} new-effective-stiffness-basis={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_effective_stiffness_basis))
}

/// 📖 Parses the arguments of `change-member-effective-stiffness-basis`.
pub fn parse_change_member_effective_stiffness_basis(args: &Args) -> Result<ChangeMemberEffectiveStiffnessBasis, String> {
    let arg = |k: &str| required(args, "change-member-effective-stiffness-basis", k);
    Ok(ChangeMemberEffectiveStiffnessBasis { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_effective_stiffness_basis: dec_json(&arg("new-effective-stiffness-basis")?)? })
}

/// ✍️ Prints `change-member-wall-poisson-ratio` as an op-text line.
pub fn print_change_member_wall_poisson_ratio(p: &ChangeMemberWallPoissonRatio) -> String {
    format!("change-member-wall-poisson-ratio building-id={} id={} new-wall-poisson-ratio={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_wall_poisson_ratio))
}

/// 📖 Parses the arguments of `change-member-wall-poisson-ratio`.
pub fn parse_change_member_wall_poisson_ratio(args: &Args) -> Result<ChangeMemberWallPoissonRatio, String> {
    let arg = |k: &str| required(args, "change-member-wall-poisson-ratio", k);
    Ok(ChangeMemberWallPoissonRatio { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_wall_poisson_ratio: dec_json(&arg("new-wall-poisson-ratio")?)? })
}

/// ✍️ Prints `change-member-wall-start-xm` as an op-text line.
pub fn print_change_member_wall_start_x_m(p: &ChangeMemberWallStartXM) -> String {
    format!("change-member-wall-start-xm building-id={} id={} new-wall-start-x-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_wall_start_x_m))
}

/// 📖 Parses the arguments of `change-member-wall-start-xm`.
pub fn parse_change_member_wall_start_x_m(args: &Args) -> Result<ChangeMemberWallStartXM, String> {
    let arg = |k: &str| required(args, "change-member-wall-start-xm", k);
    Ok(ChangeMemberWallStartXM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_wall_start_x_m: dec_json(&arg("new-wall-start-x-m")?)? })
}

/// ✍️ Prints `change-member-wall-start-ym` as an op-text line.
pub fn print_change_member_wall_start_y_m(p: &ChangeMemberWallStartYM) -> String {
    format!("change-member-wall-start-ym building-id={} id={} new-wall-start-y-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_wall_start_y_m))
}

/// 📖 Parses the arguments of `change-member-wall-start-ym`.
pub fn parse_change_member_wall_start_y_m(args: &Args) -> Result<ChangeMemberWallStartYM, String> {
    let arg = |k: &str| required(args, "change-member-wall-start-ym", k);
    Ok(ChangeMemberWallStartYM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_wall_start_y_m: dec_json(&arg("new-wall-start-y-m")?)? })
}

/// ✍️ Prints `change-member-wall-length-m` as an op-text line.
pub fn print_change_member_wall_length_m(p: &ChangeMemberWallLengthM) -> String {
    format!("change-member-wall-length-m building-id={} id={} new-wall-length-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_wall_length_m))
}

/// 📖 Parses the arguments of `change-member-wall-length-m`.
pub fn parse_change_member_wall_length_m(args: &Args) -> Result<ChangeMemberWallLengthM, String> {
    let arg = |k: &str| required(args, "change-member-wall-length-m", k);
    Ok(ChangeMemberWallLengthM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_wall_length_m: dec_json(&arg("new-wall-length-m")?)? })
}

/// ✍️ Prints `change-member-wall-direction-rad` as an op-text line.
pub fn print_change_member_wall_direction_rad(p: &ChangeMemberWallDirectionRad) -> String {
    format!("change-member-wall-direction-rad building-id={} id={} new-wall-direction-rad={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_wall_direction_rad))
}

/// 📖 Parses the arguments of `change-member-wall-direction-rad`.
pub fn parse_change_member_wall_direction_rad(args: &Args) -> Result<ChangeMemberWallDirectionRad, String> {
    let arg = |k: &str| required(args, "change-member-wall-direction-rad", k);
    Ok(ChangeMemberWallDirectionRad { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_wall_direction_rad: dec_json(&arg("new-wall-direction-rad")?)? })
}

/// ✍️ Prints `change-member-wall-storey-id` as an op-text line.
pub fn print_change_member_wall_storey_id(p: &ChangeMemberWallStoreyId) -> String {
    format!("change-member-wall-storey-id building-id={} id={} new-wall-storey-id={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_wall_storey_id))
}

/// 📖 Parses the arguments of `change-member-wall-storey-id`.
pub fn parse_change_member_wall_storey_id(args: &Args) -> Result<ChangeMemberWallStoreyId, String> {
    let arg = |k: &str| required(args, "change-member-wall-storey-id", k);
    Ok(ChangeMemberWallStoreyId { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_wall_storey_id: dec_json(&arg("new-wall-storey-id")?)? })
}

/// ✍️ Prints `change-member-wall-eff-height-m` as an op-text line.
pub fn print_change_member_wall_eff_height_m(p: &ChangeMemberWallEffHeightM) -> String {
    format!("change-member-wall-eff-height-m building-id={} id={} new-wall-effective-height-m={}", enc_json(&p.building_id), enc_json(&p.id), enc_json(&p.new_wall_effective_height_m))
}

/// 📖 Parses the arguments of `change-member-wall-eff-height-m`.
pub fn parse_change_member_wall_eff_height_m(args: &Args) -> Result<ChangeMemberWallEffHeightM, String> {
    let arg = |k: &str| required(args, "change-member-wall-eff-height-m", k);
    Ok(ChangeMemberWallEffHeightM { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)?, new_wall_effective_height_m: dec_json(&arg("new-wall-effective-height-m")?)? })
}

/// ✍️ Prints `change-assessment-chord-demand-rad` as an op-text line.
pub fn print_change_assessment_chord_demand_rad(p: &ChangeAssessmentChordDemandRad) -> String {
    format!("change-assessment-chord-demand-rad id={} new-chord-rotation-demand-rad={}", enc_json(&p.id), enc_json(&p.new_chord_rotation_demand_rad))
}

/// 📖 Parses the arguments of `change-assessment-chord-demand-rad`.
pub fn parse_change_assessment_chord_demand_rad(args: &Args) -> Result<ChangeAssessmentChordDemandRad, String> {
    let arg = |k: &str| required(args, "change-assessment-chord-demand-rad", k);
    Ok(ChangeAssessmentChordDemandRad { id: dec_json(&arg("id")?)?, new_chord_rotation_demand_rad: dec_json(&arg("new-chord-rotation-demand-rad")?)? })
}

/// ✍️ Prints `change-assessment-chord-yield-rad` as an op-text line.
pub fn print_change_assessment_chord_yield_rad(p: &ChangeAssessmentChordYieldRad) -> String {
    format!("change-assessment-chord-yield-rad id={} new-chord-rotation-yield-rad={}", enc_json(&p.id), enc_json(&p.new_chord_rotation_yield_rad))
}

/// 📖 Parses the arguments of `change-assessment-chord-yield-rad`.
pub fn parse_change_assessment_chord_yield_rad(args: &Args) -> Result<ChangeAssessmentChordYieldRad, String> {
    let arg = |k: &str| required(args, "change-assessment-chord-yield-rad", k);
    Ok(ChangeAssessmentChordYieldRad { id: dec_json(&arg("id")?)?, new_chord_rotation_yield_rad: dec_json(&arg("new-chord-rotation-yield-rad")?)? })
}

/// ✍️ Prints `change-assessment-chord-ult-rad` as an op-text line.
pub fn print_change_assessment_chord_ult_rad(p: &ChangeAssessmentChordUltRad) -> String {
    format!("change-assessment-chord-ult-rad id={} new-chord-rotation-ultimate-rad={}", enc_json(&p.id), enc_json(&p.new_chord_rotation_ultimate_rad))
}

/// 📖 Parses the arguments of `change-assessment-chord-ult-rad`.
pub fn parse_change_assessment_chord_ult_rad(args: &Args) -> Result<ChangeAssessmentChordUltRad, String> {
    let arg = |k: &str| required(args, "change-assessment-chord-ult-rad", k);
    Ok(ChangeAssessmentChordUltRad { id: dec_json(&arg("id")?)?, new_chord_rotation_ultimate_rad: dec_json(&arg("new-chord-rotation-ultimate-rad")?)? })
}

/// ✍️ Prints `change-assessment-section-depth-m` as an op-text line.
pub fn print_change_assessment_section_depth_m(p: &ChangeAssessmentSectionDepthM) -> String {
    format!("change-assessment-section-depth-m id={} new-section-depth-m={}", enc_json(&p.id), enc_json(&p.new_section_depth_m))
}

/// 📖 Parses the arguments of `change-assessment-section-depth-m`.
pub fn parse_change_assessment_section_depth_m(args: &Args) -> Result<ChangeAssessmentSectionDepthM, String> {
    let arg = |k: &str| required(args, "change-assessment-section-depth-m", k);
    Ok(ChangeAssessmentSectionDepthM { id: dec_json(&arg("id")?)?, new_section_depth_m: dec_json(&arg("new-section-depth-m")?)? })
}

/// ✍️ Prints `change-assessment-section-width-m` as an op-text line.
pub fn print_change_assessment_section_width_m(p: &ChangeAssessmentSectionWidthM) -> String {
    format!("change-assessment-section-width-m id={} new-section-width-m={}", enc_json(&p.id), enc_json(&p.new_section_width_m))
}

/// 📖 Parses the arguments of `change-assessment-section-width-m`.
pub fn parse_change_assessment_section_width_m(args: &Args) -> Result<ChangeAssessmentSectionWidthM, String> {
    let arg = |k: &str| required(args, "change-assessment-section-width-m", k);
    Ok(ChangeAssessmentSectionWidthM { id: dec_json(&arg("id")?)?, new_section_width_m: dec_json(&arg("new-section-width-m")?)? })
}

/// ✍️ Prints `change-assessment-eff-depth-m` as an op-text line.
pub fn print_change_assessment_eff_depth_m(p: &ChangeAssessmentEffDepthM) -> String {
    format!("change-assessment-eff-depth-m id={} new-effective-depth-m={}", enc_json(&p.id), enc_json(&p.new_effective_depth_m))
}

/// 📖 Parses the arguments of `change-assessment-eff-depth-m`.
pub fn parse_change_assessment_eff_depth_m(args: &Args) -> Result<ChangeAssessmentEffDepthM, String> {
    let arg = |k: &str| required(args, "change-assessment-eff-depth-m", k);
    Ok(ChangeAssessmentEffDepthM { id: dec_json(&arg("id")?)?, new_effective_depth_m: dec_json(&arg("new-effective-depth-m")?)? })
}

/// ✍️ Prints `change-assessment-shear-span-m` as an op-text line.
pub fn print_change_assessment_shear_span_m(p: &ChangeAssessmentShearSpanM) -> String {
    format!("change-assessment-shear-span-m id={} new-shear-span-m={}", enc_json(&p.id), enc_json(&p.new_shear_span_m))
}

/// 📖 Parses the arguments of `change-assessment-shear-span-m`.
pub fn parse_change_assessment_shear_span_m(args: &Args) -> Result<ChangeAssessmentShearSpanM, String> {
    let arg = |k: &str| required(args, "change-assessment-shear-span-m", k);
    Ok(ChangeAssessmentShearSpanM { id: dec_json(&arg("id")?)?, new_shear_span_m: dec_json(&arg("new-shear-span-m")?)? })
}

/// ✍️ Prints `change-assessment-axial-force-n` as an op-text line.
pub fn print_change_assessment_axial_force_n(p: &ChangeAssessmentAxialForceN) -> String {
    format!("change-assessment-axial-force-n id={} new-axial-force-n={}", enc_json(&p.id), enc_json(&p.new_axial_force_n))
}

/// 📖 Parses the arguments of `change-assessment-axial-force-n`.
pub fn parse_change_assessment_axial_force_n(args: &Args) -> Result<ChangeAssessmentAxialForceN, String> {
    let arg = |k: &str| required(args, "change-assessment-axial-force-n", k);
    Ok(ChangeAssessmentAxialForceN { id: dec_json(&arg("id")?)?, new_axial_force_n: dec_json(&arg("new-axial-force-n")?)? })
}

/// ✍️ Prints `change-assessment-shear-force-demand-n` as an op-text line.
pub fn print_change_assessment_shear_force_demand_n(p: &ChangeAssessmentShearForceDemandN) -> String {
    format!("change-assessment-shear-force-demand-n id={} new-shear-force-demand-n={}", enc_json(&p.id), enc_json(&p.new_shear_force_demand_n))
}

/// 📖 Parses the arguments of `change-assessment-shear-force-demand-n`.
pub fn parse_change_assessment_shear_force_demand_n(args: &Args) -> Result<ChangeAssessmentShearForceDemandN, String> {
    let arg = |k: &str| required(args, "change-assessment-shear-force-demand-n", k);
    Ok(ChangeAssessmentShearForceDemandN { id: dec_json(&arg("id")?)?, new_shear_force_demand_n: dec_json(&arg("new-shear-force-demand-n")?)? })
}

/// ✍️ Prints `change-assessment-as1-area-m2` as an op-text line.
pub fn print_change_assessment_as1_area_m2(p: &ChangeAssessmentAs1AreaM2) -> String {
    format!("change-assessment-as1-area-m2 id={} new-tension-steel-area-m2={}", enc_json(&p.id), enc_json(&p.new_tension_steel_area_m2))
}

/// 📖 Parses the arguments of `change-assessment-as1-area-m2`.
pub fn parse_change_assessment_as1_area_m2(args: &Args) -> Result<ChangeAssessmentAs1AreaM2, String> {
    let arg = |k: &str| required(args, "change-assessment-as1-area-m2", k);
    Ok(ChangeAssessmentAs1AreaM2 { id: dec_json(&arg("id")?)?, new_tension_steel_area_m2: dec_json(&arg("new-tension-steel-area-m2")?)? })
}

/// ✍️ Prints `change-assessment-as2-area-m2` as an op-text line.
pub fn print_change_assessment_as2_area_m2(p: &ChangeAssessmentAs2AreaM2) -> String {
    format!("change-assessment-as2-area-m2 id={} new-compression-steel-area-m2={}", enc_json(&p.id), enc_json(&p.new_compression_steel_area_m2))
}

/// 📖 Parses the arguments of `change-assessment-as2-area-m2`.
pub fn parse_change_assessment_as2_area_m2(args: &Args) -> Result<ChangeAssessmentAs2AreaM2, String> {
    let arg = |k: &str| required(args, "change-assessment-as2-area-m2", k);
    Ok(ChangeAssessmentAs2AreaM2 { id: dec_json(&arg("id")?)?, new_compression_steel_area_m2: dec_json(&arg("new-compression-steel-area-m2")?)? })
}

/// ✍️ Prints `change-assessment-bar-diameter-mm` as an op-text line.
pub fn print_change_assessment_bar_diameter_mm(p: &ChangeAssessmentBarDiameterMm) -> String {
    format!("change-assessment-bar-diameter-mm id={} new-bar-diameter-mm={}", enc_json(&p.id), enc_json(&p.new_bar_diameter_mm))
}

/// 📖 Parses the arguments of `change-assessment-bar-diameter-mm`.
pub fn parse_change_assessment_bar_diameter_mm(args: &Args) -> Result<ChangeAssessmentBarDiameterMm, String> {
    let arg = |k: &str| required(args, "change-assessment-bar-diameter-mm", k);
    Ok(ChangeAssessmentBarDiameterMm { id: dec_json(&arg("id")?)?, new_bar_diameter_mm: dec_json(&arg("new-bar-diameter-mm")?)? })
}

/// ✍️ Prints `change-assessment-steel-yield-mpa` as an op-text line.
pub fn print_change_assessment_steel_yield_mpa(p: &ChangeAssessmentSteelYieldMpa) -> String {
    format!("change-assessment-steel-yield-mpa id={} new-steel-yield-mpa={}", enc_json(&p.id), enc_json(&p.new_steel_yield_mpa))
}

/// 📖 Parses the arguments of `change-assessment-steel-yield-mpa`.
pub fn parse_change_assessment_steel_yield_mpa(args: &Args) -> Result<ChangeAssessmentSteelYieldMpa, String> {
    let arg = |k: &str| required(args, "change-assessment-steel-yield-mpa", k);
    Ok(ChangeAssessmentSteelYieldMpa { id: dec_json(&arg("id")?)?, new_steel_yield_mpa: dec_json(&arg("new-steel-yield-mpa")?)? })
}

/// ✍️ Prints `change-assessment-fc-mpa` as an op-text line.
pub fn print_change_assessment_fc_mpa(p: &ChangeAssessmentFcMpa) -> String {
    format!("change-assessment-fc-mpa id={} new-concrete-strength-mpa={}", enc_json(&p.id), enc_json(&p.new_concrete_strength_mpa))
}

/// 📖 Parses the arguments of `change-assessment-fc-mpa`.
pub fn parse_change_assessment_fc_mpa(args: &Args) -> Result<ChangeAssessmentFcMpa, String> {
    let arg = |k: &str| required(args, "change-assessment-fc-mpa", k);
    Ok(ChangeAssessmentFcMpa { id: dec_json(&arg("id")?)?, new_concrete_strength_mpa: dec_json(&arg("new-concrete-strength-mpa")?)? })
}

/// ✍️ Prints `change-assessment-strp-leg-area-m2` as an op-text line.
pub fn print_change_assessment_strp_leg_area_m2(p: &ChangeAssessmentStrpLegAreaM2) -> String {
    format!("change-assessment-strp-leg-area-m2 id={} new-stirrup-leg-area-m2={}", enc_json(&p.id), enc_json(&p.new_stirrup_leg_area_m2))
}

/// 📖 Parses the arguments of `change-assessment-strp-leg-area-m2`.
pub fn parse_change_assessment_strp_leg_area_m2(args: &Args) -> Result<ChangeAssessmentStrpLegAreaM2, String> {
    let arg = |k: &str| required(args, "change-assessment-strp-leg-area-m2", k);
    Ok(ChangeAssessmentStrpLegAreaM2 { id: dec_json(&arg("id")?)?, new_stirrup_leg_area_m2: dec_json(&arg("new-stirrup-leg-area-m2")?)? })
}

/// ✍️ Prints `change-assessment-strp-spacing-m` as an op-text line.
pub fn print_change_assessment_strp_spacing_m(p: &ChangeAssessmentStrpSpacingM) -> String {
    format!("change-assessment-strp-spacing-m id={} new-stirrup-spacing-m={}", enc_json(&p.id), enc_json(&p.new_stirrup_spacing_m))
}

/// 📖 Parses the arguments of `change-assessment-strp-spacing-m`.
pub fn parse_change_assessment_strp_spacing_m(args: &Args) -> Result<ChangeAssessmentStrpSpacingM, String> {
    let arg = |k: &str| required(args, "change-assessment-strp-spacing-m", k);
    Ok(ChangeAssessmentStrpSpacingM { id: dec_json(&arg("id")?)?, new_stirrup_spacing_m: dec_json(&arg("new-stirrup-spacing-m")?)? })
}

/// ✍️ Prints `change-assessment-strp-yield-mpa` as an op-text line.
pub fn print_change_assessment_strp_yield_mpa(p: &ChangeAssessmentStrpYieldMpa) -> String {
    format!("change-assessment-strp-yield-mpa id={} new-stirrup-yield-mpa={}", enc_json(&p.id), enc_json(&p.new_stirrup_yield_mpa))
}

/// 📖 Parses the arguments of `change-assessment-strp-yield-mpa`.
pub fn parse_change_assessment_strp_yield_mpa(args: &Args) -> Result<ChangeAssessmentStrpYieldMpa, String> {
    let arg = |k: &str| required(args, "change-assessment-strp-yield-mpa", k);
    Ok(ChangeAssessmentStrpYieldMpa { id: dec_json(&arg("id")?)?, new_stirrup_yield_mpa: dec_json(&arg("new-stirrup-yield-mpa")?)? })
}

/// ✍️ Prints `change-assessment-core-width-m` as an op-text line.
pub fn print_change_assessment_core_width_m(p: &ChangeAssessmentCoreWidthM) -> String {
    format!("change-assessment-core-width-m id={} new-core-width-m={}", enc_json(&p.id), enc_json(&p.new_core_width_m))
}

/// 📖 Parses the arguments of `change-assessment-core-width-m`.
pub fn parse_change_assessment_core_width_m(args: &Args) -> Result<ChangeAssessmentCoreWidthM, String> {
    let arg = |k: &str| required(args, "change-assessment-core-width-m", k);
    Ok(ChangeAssessmentCoreWidthM { id: dec_json(&arg("id")?)?, new_core_width_m: dec_json(&arg("new-core-width-m")?)? })
}

/// ✍️ Prints `change-assessment-core-depth-m` as an op-text line.
pub fn print_change_assessment_core_depth_m(p: &ChangeAssessmentCoreDepthM) -> String {
    format!("change-assessment-core-depth-m id={} new-core-depth-m={}", enc_json(&p.id), enc_json(&p.new_core_depth_m))
}

/// 📖 Parses the arguments of `change-assessment-core-depth-m`.
pub fn parse_change_assessment_core_depth_m(args: &Args) -> Result<ChangeAssessmentCoreDepthM, String> {
    let arg = |k: &str| required(args, "change-assessment-core-depth-m", k);
    Ok(ChangeAssessmentCoreDepthM { id: dec_json(&arg("id")?)?, new_core_depth_m: dec_json(&arg("new-core-depth-m")?)? })
}

/// ✍️ Prints `change-assessment-rbar-count` as an op-text line.
pub fn print_change_assessment_rbar_count(p: &ChangeAssessmentRbarCount) -> String {
    format!("change-assessment-rbar-count id={} new-restrained-bar-count={}", enc_json(&p.id), enc_json(&p.new_restrained_bar_count))
}

/// 📖 Parses the arguments of `change-assessment-rbar-count`.
pub fn parse_change_assessment_rbar_count(args: &Args) -> Result<ChangeAssessmentRbarCount, String> {
    let arg = |k: &str| required(args, "change-assessment-rbar-count", k);
    Ok(ChangeAssessmentRbarCount { id: dec_json(&arg("id")?)?, new_restrained_bar_count: dec_json(&arg("new-restrained-bar-count")?)? })
}

/// ✍️ Prints `change-assessment-rbar-spacing-m` as an op-text line.
pub fn print_change_assessment_rbar_spacing_m(p: &ChangeAssessmentRbarSpacingM) -> String {
    format!("change-assessment-rbar-spacing-m id={} new-restrained-bar-spacing-m={}", enc_json(&p.id), enc_json(&p.new_restrained_bar_spacing_m))
}

/// 📖 Parses the arguments of `change-assessment-rbar-spacing-m`.
pub fn parse_change_assessment_rbar_spacing_m(args: &Args) -> Result<ChangeAssessmentRbarSpacingM, String> {
    let arg = |k: &str| required(args, "change-assessment-rbar-spacing-m", k);
    Ok(ChangeAssessmentRbarSpacingM { id: dec_json(&arg("id")?)?, new_restrained_bar_spacing_m: dec_json(&arg("new-restrained-bar-spacing-m")?)? })
}

/// ✍️ Prints `change-assessment-diag-reinf-ratio` as an op-text line.
pub fn print_change_assessment_diag_reinf_ratio(p: &ChangeAssessmentDiagReinfRatio) -> String {
    format!("change-assessment-diag-reinf-ratio id={} new-diagonal-reinforcement-ratio={}", enc_json(&p.id), enc_json(&p.new_diagonal_reinforcement_ratio))
}

/// 📖 Parses the arguments of `change-assessment-diag-reinf-ratio`.
pub fn parse_change_assessment_diag_reinf_ratio(args: &Args) -> Result<ChangeAssessmentDiagReinfRatio, String> {
    let arg = |k: &str| required(args, "change-assessment-diag-reinf-ratio", k);
    Ok(ChangeAssessmentDiagReinfRatio { id: dec_json(&arg("id")?)?, new_diagonal_reinforcement_ratio: dec_json(&arg("new-diagonal-reinforcement-ratio")?)? })
}

/// ✍️ Prints `change-assessment-shear-crack` as an op-text line.
pub fn print_change_assessment_shear_crack(p: &ChangeAssessmentShearCrack) -> String {
    format!("change-assessment-shear-crack id={} new-shear-cracking-before-yield={}", enc_json(&p.id), enc_json(&p.new_shear_cracking_before_yield))
}

/// 📖 Parses the arguments of `change-assessment-shear-crack`.
pub fn parse_change_assessment_shear_crack(args: &Args) -> Result<ChangeAssessmentShearCrack, String> {
    let arg = |k: &str| required(args, "change-assessment-shear-crack", k);
    Ok(ChangeAssessmentShearCrack { id: dec_json(&arg("id")?)?, new_shear_cracking_before_yield: dec_json(&arg("new-shear-cracking-before-yield")?)? })
}

/// ✍️ Prints `change-joint-building-id` as an op-text line.
pub fn print_change_joint_building_id(p: &ChangeJointBuildingId) -> String {
    format!("change-joint-building-id id={} new-building-id={}", enc_json(&p.id), enc_json(&p.new_building_id))
}

/// 📖 Parses the arguments of `change-joint-building-id`.
pub fn parse_change_joint_building_id(args: &Args) -> Result<ChangeJointBuildingId, String> {
    let arg = |k: &str| required(args, "change-joint-building-id", k);
    Ok(ChangeJointBuildingId { id: dec_json(&arg("id")?)?, new_building_id: dec_json(&arg("new-building-id")?)? })
}

/// ✍️ Prints `change-joint-storey-id` as an op-text line.
pub fn print_change_joint_storey_id(p: &ChangeJointStoreyId) -> String {
    format!("change-joint-storey-id id={} new-storey-id={}", enc_json(&p.id), enc_json(&p.new_storey_id))
}

/// 📖 Parses the arguments of `change-joint-storey-id`.
pub fn parse_change_joint_storey_id(args: &Args) -> Result<ChangeJointStoreyId, String> {
    let arg = |k: &str| required(args, "change-joint-storey-id", k);
    Ok(ChangeJointStoreyId { id: dec_json(&arg("id")?)?, new_storey_id: dec_json(&arg("new-storey-id")?)? })
}

/// ✍️ Prints `change-joint-column-id` as an op-text line.
pub fn print_change_joint_column_id(p: &ChangeJointColumnId) -> String {
    format!("change-joint-column-id id={} new-column-id={}", enc_json(&p.id), enc_json(&p.new_column_id))
}

/// 📖 Parses the arguments of `change-joint-column-id`.
pub fn parse_change_joint_column_id(args: &Args) -> Result<ChangeJointColumnId, String> {
    let arg = |k: &str| required(args, "change-joint-column-id", k);
    Ok(ChangeJointColumnId { id: dec_json(&arg("id")?)?, new_column_id: dec_json(&arg("new-column-id")?)? })
}

/// ✍️ Prints `change-joint-beam-id` as an op-text line.
pub fn print_change_joint_beam_id(p: &ChangeJointBeamId) -> String {
    format!("change-joint-beam-id id={} new-beam-id={}", enc_json(&p.id), enc_json(&p.new_beam_id))
}

/// 📖 Parses the arguments of `change-joint-beam-id`.
pub fn parse_change_joint_beam_id(args: &Args) -> Result<ChangeJointBeamId, String> {
    let arg = |k: &str| required(args, "change-joint-beam-id", k);
    Ok(ChangeJointBeamId { id: dec_json(&arg("id")?)?, new_beam_id: dec_json(&arg("new-beam-id")?)? })
}

/// ✍️ Prints `change-joint-xm` as an op-text line.
pub fn print_change_joint_x_m(p: &ChangeJointXM) -> String {
    format!("change-joint-xm id={} new-x-m={}", enc_json(&p.id), enc_json(&p.new_x_m))
}

/// 📖 Parses the arguments of `change-joint-xm`.
pub fn parse_change_joint_x_m(args: &Args) -> Result<ChangeJointXM, String> {
    let arg = |k: &str| required(args, "change-joint-xm", k);
    Ok(ChangeJointXM { id: dec_json(&arg("id")?)?, new_x_m: dec_json(&arg("new-x-m")?)? })
}

/// ✍️ Prints `change-joint-ym` as an op-text line.
pub fn print_change_joint_y_m(p: &ChangeJointYM) -> String {
    format!("change-joint-ym id={} new-y-m={}", enc_json(&p.id), enc_json(&p.new_y_m))
}

/// 📖 Parses the arguments of `change-joint-ym`.
pub fn parse_change_joint_y_m(args: &Args) -> Result<ChangeJointYM, String> {
    let arg = |k: &str| required(args, "change-joint-ym", k);
    Ok(ChangeJointYM { id: dec_json(&arg("id")?)?, new_y_m: dec_json(&arg("new-y-m")?)? })
}

/// ✍️ Prints `change-joint-column-moment-rd-nm` as an op-text line.
pub fn print_change_joint_column_moment_rd_nm(p: &ChangeJointColumnMomentRdNm) -> String {
    format!("change-joint-column-moment-rd-nm id={} new-column-moment-rd-nm={}", enc_json(&p.id), enc_json(&p.new_column_moment_rd_nm))
}

/// 📖 Parses the arguments of `change-joint-column-moment-rd-nm`.
pub fn parse_change_joint_column_moment_rd_nm(args: &Args) -> Result<ChangeJointColumnMomentRdNm, String> {
    let arg = |k: &str| required(args, "change-joint-column-moment-rd-nm", k);
    Ok(ChangeJointColumnMomentRdNm { id: dec_json(&arg("id")?)?, new_column_moment_rd_nm: dec_json(&arg("new-column-moment-rd-nm")?)? })
}

/// ✍️ Prints `change-joint-beam-moment-rd-nm` as an op-text line.
pub fn print_change_joint_beam_moment_rd_nm(p: &ChangeJointBeamMomentRdNm) -> String {
    format!("change-joint-beam-moment-rd-nm id={} new-beam-moment-rd-nm={}", enc_json(&p.id), enc_json(&p.new_beam_moment_rd_nm))
}

/// 📖 Parses the arguments of `change-joint-beam-moment-rd-nm`.
pub fn parse_change_joint_beam_moment_rd_nm(args: &Args) -> Result<ChangeJointBeamMomentRdNm, String> {
    let arg = |k: &str| required(args, "change-joint-beam-moment-rd-nm", k);
    Ok(ChangeJointBeamMomentRdNm { id: dec_json(&arg("id")?)?, new_beam_moment_rd_nm: dec_json(&arg("new-beam-moment-rd-nm")?)? })
}

/// ✍️ Prints `insert-joint` as an op-text line.
pub fn print_insert_joint(p: &InsertJoint) -> String {
    format!("insert-joint index={} joint={}", enc_json(&p.index), enc_json(&p.joint))
}

/// 📖 Parses the arguments of `insert-joint`.
pub fn parse_insert_joint(args: &Args) -> Result<InsertJoint, String> {
    let arg = |k: &str| required(args, "insert-joint", k);
    Ok(InsertJoint { index: opt_index(args)?, joint: dec_json(&arg("joint")?)? })
}

/// ✍️ Prints `insert-tie-beam` as an op-text line.
pub fn print_insert_tie_beam(p: &InsertTieBeam) -> String {
    format!("insert-tie-beam index={} tie_beam={}", enc_json(&p.index), enc_json(&p.tie_beam))
}

/// 📖 Parses the arguments of `insert-tie-beam`.
pub fn parse_insert_tie_beam(args: &Args) -> Result<InsertTieBeam, String> {
    let arg = |k: &str| required(args, "insert-tie-beam", k);
    Ok(InsertTieBeam { index: opt_index(args)?, tie_beam: dec_json(&arg("tie_beam")?)? })
}

/// ✍️ Prints `insert-seismic-slope` as an op-text line.
pub fn print_insert_seismic_slope(p: &InsertSeismicSlope) -> String {
    format!("insert-seismic-slope index={} seismic_slope={}", enc_json(&p.index), enc_json(&p.seismic_slope))
}

/// 📖 Parses the arguments of `insert-seismic-slope`.
pub fn parse_insert_seismic_slope(args: &Args) -> Result<InsertSeismicSlope, String> {
    let arg = |k: &str| required(args, "insert-seismic-slope", k);
    Ok(InsertSeismicSlope { index: opt_index(args)?, seismic_slope: dec_json(&arg("seismic_slope")?)? })
}

/// ✍️ Prints `remove-joint` as an op-text line.
pub fn print_remove_joint(p: &RemoveJoint) -> String {
    format!("remove-joint id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-joint`.
pub fn parse_remove_joint(args: &Args) -> Result<RemoveJoint, String> {
    let arg = |k: &str| required(args, "remove-joint", k);
    Ok(RemoveJoint { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `remove-tie-beam` as an op-text line.
pub fn print_remove_tie_beam(p: &RemoveTieBeam) -> String {
    format!("remove-tie-beam id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-tie-beam`.
pub fn parse_remove_tie_beam(args: &Args) -> Result<RemoveTieBeam, String> {
    let arg = |k: &str| required(args, "remove-tie-beam", k);
    Ok(RemoveTieBeam { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `remove-seismic-slope` as an op-text line.
pub fn print_remove_seismic_slope(p: &RemoveSeismicSlope) -> String {
    format!("remove-seismic-slope id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-seismic-slope`.
pub fn parse_remove_seismic_slope(args: &Args) -> Result<RemoveSeismicSlope, String> {
    let arg = |k: &str| required(args, "remove-seismic-slope", k);
    Ok(RemoveSeismicSlope { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-joint-col-moment-rd-nm` as an op-text line.
pub fn print_change_joint_col_moment_rd_nm(p: &ChangeJointColMomentRdNm) -> String {
    format!("change-joint-col-moment-rd-nm index={} new-column-moment-rd-nm={}", enc_json(&p.index), enc_json(&p.new_column_moment_rd_nm))
}

/// 📖 Parses the arguments of `change-joint-col-moment-rd-nm`.
pub fn parse_change_joint_col_moment_rd_nm(args: &Args) -> Result<ChangeJointColMomentRdNm, String> {
    let arg = |k: &str| required(args, "change-joint-col-moment-rd-nm", k);
    Ok(ChangeJointColMomentRdNm { index: opt_index(args)?, new_column_moment_rd_nm: dec_json(&arg("new-column-moment-rd-nm")?)? })
}

/// ✍️ Prints `change-assessment-mid-steel-area-m2` as an op-text line.
pub fn print_change_assessment_mid_steel_area_m2(p: &ChangeAssessmentMidSteelAreaM2) -> String {
    format!("change-assessment-mid-steel-area-m2 id={} new-intermediate-steel-area-m2={}", enc_json(&p.id), enc_json(&p.new_intermediate_steel_area_m2))
}

/// 📖 Parses the arguments of `change-assessment-mid-steel-area-m2`.
pub fn parse_change_assessment_mid_steel_area_m2(args: &Args) -> Result<ChangeAssessmentMidSteelAreaM2, String> {
    let arg = |k: &str| required(args, "change-assessment-mid-steel-area-m2", k);
    Ok(ChangeAssessmentMidSteelAreaM2 { id: dec_json(&arg("id")?)?, new_intermediate_steel_area_m2: dec_json(&arg("new-intermediate-steel-area-m2")?)? })
}

/// ✍️ Prints `change-assessment-analysis-method` as an op-text line.
pub fn print_change_assessment_analysis_method(p: &ChangeAssessmentAnalysisMethod) -> String {
    format!("change-assessment-analysis-method id={} new-analysis-method={}", enc_json(&p.id), enc_json(&p.new_analysis_method_choice))
}

/// 📖 Parses the arguments of `change-assessment-analysis-method`.
pub fn parse_change_assessment_analysis_method(args: &Args) -> Result<ChangeAssessmentAnalysisMethod, String> {
    let arg = |k: &str| required(args, "change-assessment-analysis-method", k);
    Ok(ChangeAssessmentAnalysisMethod { id: dec_json(&arg("id")?)?, new_analysis_method_choice: dec_json(&arg("new-analysis-method")?)? })
}
