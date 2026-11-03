//! 🏗️ EN 1998 foundations mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_foundations::insert_foundation::InsertFoundation;
use semio_s_artifact_norm_en1998_mut_foundations::insert_retaining_wall::InsertRetainingWall;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_supported_building_id::ChangeFoundationSupportedBuildingId;
use semio_s_artifact_norm_en1998_mut_foundations::change_tie_beam_foundation_b_id::ChangeTieBeamFoundationBId;
use semio_s_artifact_norm_en1998_mut_foundations::change_tie_beam_foundation_a_id::ChangeTieBeamFoundationAId;
use semio_s_artifact_norm_en1998_mut_foundations::change_seismic_slope_governing_layer_id::ChangeSeismicSlopeGoverningLayerId;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_area_m2::ChangeFoundationAreaM2;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_p_rd_pa::ChangeFoundationPRdPa;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_passive_resistance_n::ChangeFoundationPassiveResistanceN;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_k_foundation::ChangeFoundationKFoundation;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_k_soil::ChangeFoundationKSoil;
use semio_s_artifact_norm_en1998_mut_foundations::change_retaining_wall_height_m::ChangeRetainingWallHeightM;
use semio_s_artifact_norm_en1998_mut_foundations::change_retaining_wall_phi_deg::ChangeRetainingWallPhiDeg;
use semio_s_artifact_norm_en1998_mut_foundations::change_retaining_wall_soil_gamma::ChangeRetainingWallSoilGamma;
use semio_s_artifact_norm_en1998_mut_foundations::change_retaining_wall_r::ChangeRetainingWallR;
use semio_s_artifact_norm_en1998_mut_foundations::change_retaining_wall_h_rd_n_per_m::ChangeRetainingWallHRdNPerM;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_liq_n160::ChangeFoundationLiqN160;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_liq_fines_percent::ChangeFoundationLiqFinesPercent;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_liq_depth_m::ChangeFoundationLiqDepthM;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_liq_water_depth_m::ChangeFoundationLiqWaterDepthM;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_liq_unit_wt_n_m3::ChangeFoundationLiqUnitWtNM3;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_liq_magnitude::ChangeFoundationLiqMagnitude;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_liq_layer_m::ChangeFoundationLiqLayerM;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_footing_width_m::ChangeFoundationFootingWidthM;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_bear_axial_n::ChangeFoundationBearAxialN;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_bear_shear_n::ChangeFoundationBearShearN;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_bear_moment_nm::ChangeFoundationBearMomentNm;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_bearing_n_max_n::ChangeFoundationBearingNMaxN;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_soil_phi_deg::ChangeFoundationSoilPhiDeg;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_diameter_m::ChangeFoundationPileDiameterM;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_length_m::ChangeFoundationPileLengthM;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_modulus_pa::ChangeFoundationPileModulusPa;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_ks_n_m3::ChangeFoundationPileKsNM3;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_head_shear_n::ChangeFoundationPileHeadShearN;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_head_fixed::ChangeFoundationPileHeadFixed;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_iface_m::ChangeFoundationPileIfaceM;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_up_vs_m_s::ChangeFoundationPileUpVsMS;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_low_vs_m_s::ChangeFoundationPileLowVsMS;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_pile_moment_rd_nm::ChangeFoundationPileMomentRdNm;
use semio_s_artifact_norm_en1998_mut_foundations::change_soil_layer_foundation_id::ChangeSoilLayerFoundationId;
use semio_s_artifact_norm_en1998_mut_foundations::change_soil_layer_thickness_m::ChangeSoilLayerThicknessM;
use semio_s_artifact_norm_en1998_mut_foundations::change_soil_unit_wt_n_m3::ChangeSoilUnitWtNM3;
use semio_s_artifact_norm_en1998_mut_foundations::change_soil_layer_vs_m_s::ChangeSoilLayerVsMS;
use semio_s_artifact_norm_en1998_mut_foundations::change_soil_layer_n1_60::ChangeSoilLayerN160;
use semio_s_artifact_norm_en1998_mut_foundations::change_soil_layer_fines_percent::ChangeSoilLayerFinesPercent;
use semio_s_artifact_norm_en1998_mut_foundations::insert_soil_layer::InsertSoilLayer;
use semio_s_artifact_norm_en1998_mut_foundations::remove_soil_layer::RemoveSoilLayer;
use semio_s_artifact_norm_en1998_mut_foundations::change_retaining_wall_water_height_m::ChangeRetainingWallWaterHeightM;
use semio_s_artifact_norm_en1998_mut_foundations::change_retaining_wall_saturated_gamma::ChangeRetainingWallSaturatedGamma;
use semio_s_artifact_norm_en1998_mut_foundations::change_retaining_wall_permeability_m_s::ChangeRetainingWallPermeabilityMS;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_base_friction_deg::ChangeFoundationBaseFrictionDeg;
use semio_s_artifact_norm_en1998_mut_foundations::change_foundation_base_pore_pressure_pa::ChangeFoundationBasePorePressurePa;
use semio_s_artifact_norm_en1998_mut_foundations::remove_foundation::RemoveFoundation;
use semio_s_artifact_norm_en1998_mut_foundations::remove_retaining_wall::RemoveRetainingWall;

/// ✍️ Prints `insert-foundation` as an op-text line.
pub fn print_insert_foundation(p: &InsertFoundation) -> String {
    format!("insert-foundation index={} foundation={}", enc_json(&p.index), enc_json(&p.foundation))
}

/// 📖 Parses the arguments of `insert-foundation`.
pub fn parse_insert_foundation(args: &Args) -> Result<InsertFoundation, String> {
    let arg = |k: &str| required(args, "insert-foundation", k);
    Ok(InsertFoundation { index: opt_index(args)?, foundation: dec_json(&arg("foundation")?)? })
}

/// ✍️ Prints `insert-retaining-wall` as an op-text line.
pub fn print_insert_retaining_wall(p: &InsertRetainingWall) -> String {
    format!("insert-retaining-wall index={} wall={}", enc_json(&p.index), enc_json(&p.wall))
}

/// 📖 Parses the arguments of `insert-retaining-wall`.
pub fn parse_insert_retaining_wall(args: &Args) -> Result<InsertRetainingWall, String> {
    let arg = |k: &str| required(args, "insert-retaining-wall", k);
    Ok(InsertRetainingWall { index: opt_index(args)?, wall: dec_json(&arg("wall")?)? })
}

/// ✍️ Prints `change-foundation-supported-building-id` as an op-text line.
pub fn print_change_foundation_supported_building_id(p: &ChangeFoundationSupportedBuildingId) -> String {
    format!("change-foundation-supported-building-id id={} new-supported-building-id={}", enc_json(&p.id), enc_json(&p.new_supported_building_id))
}

/// 📖 Parses the arguments of `change-foundation-supported-building-id`.
pub fn parse_change_foundation_supported_building_id(args: &Args) -> Result<ChangeFoundationSupportedBuildingId, String> {
    let arg = |k: &str| required(args, "change-foundation-supported-building-id", k);
    Ok(ChangeFoundationSupportedBuildingId { id: dec_json(&arg("id")?)?, new_supported_building_id: dec_json(&arg("new-supported-building-id")?)? })
}

/// ✍️ Prints `change-tie-beam-foundation-b-id` as an op-text line.
pub fn print_change_tie_beam_foundation_b_id(p: &ChangeTieBeamFoundationBId) -> String {
    format!("change-tie-beam-foundation-b-id id={} new-foundation-b-id={}", enc_json(&p.id), enc_json(&p.new_foundation_b_id))
}

/// 📖 Parses the arguments of `change-tie-beam-foundation-b-id`.
pub fn parse_change_tie_beam_foundation_b_id(args: &Args) -> Result<ChangeTieBeamFoundationBId, String> {
    let arg = |k: &str| required(args, "change-tie-beam-foundation-b-id", k);
    Ok(ChangeTieBeamFoundationBId { id: dec_json(&arg("id")?)?, new_foundation_b_id: dec_json(&arg("new-foundation-b-id")?)? })
}

/// ✍️ Prints `change-tie-beam-foundation-a-id` as an op-text line.
pub fn print_change_tie_beam_foundation_a_id(p: &ChangeTieBeamFoundationAId) -> String {
    format!("change-tie-beam-foundation-a-id id={} new-foundation-a-id={}", enc_json(&p.id), enc_json(&p.new_foundation_a_id))
}

/// 📖 Parses the arguments of `change-tie-beam-foundation-a-id`.
pub fn parse_change_tie_beam_foundation_a_id(args: &Args) -> Result<ChangeTieBeamFoundationAId, String> {
    let arg = |k: &str| required(args, "change-tie-beam-foundation-a-id", k);
    Ok(ChangeTieBeamFoundationAId { id: dec_json(&arg("id")?)?, new_foundation_a_id: dec_json(&arg("new-foundation-a-id")?)? })
}

/// ✍️ Prints `change-seismic-slope-governing-layer-id` as an op-text line.
pub fn print_change_seismic_slope_governing_layer_id(p: &ChangeSeismicSlopeGoverningLayerId) -> String {
    format!("change-seismic-slope-governing-layer-id id={} new-governing-layer-id={}", enc_json(&p.id), enc_json(&p.new_governing_layer_id))
}

/// 📖 Parses the arguments of `change-seismic-slope-governing-layer-id`.
pub fn parse_change_seismic_slope_governing_layer_id(args: &Args) -> Result<ChangeSeismicSlopeGoverningLayerId, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-governing-layer-id", k);
    Ok(ChangeSeismicSlopeGoverningLayerId { id: dec_json(&arg("id")?)?, new_governing_layer_id: dec_json(&arg("new-governing-layer-id")?)? })
}

/// ✍️ Prints `change-foundation-area-m2` as an op-text line.
pub fn print_change_foundation_area_m2(p: &ChangeFoundationAreaM2) -> String {
    format!("change-foundation-area-m2 id={} new-area-m2={}", enc_json(&p.id), enc_json(&p.new_area_m2))
}

/// 📖 Parses the arguments of `change-foundation-area-m2`.
pub fn parse_change_foundation_area_m2(args: &Args) -> Result<ChangeFoundationAreaM2, String> {
    let arg = |k: &str| required(args, "change-foundation-area-m2", k);
    Ok(ChangeFoundationAreaM2 { id: dec_json(&arg("id")?)?, new_area_m2: dec_json(&arg("new-area-m2")?)? })
}

/// ✍️ Prints `change-foundation-p-rd-pa` as an op-text line.
pub fn print_change_foundation_p_rd_pa(p: &ChangeFoundationPRdPa) -> String {
    format!("change-foundation-p-rd-pa id={} new-p-rd-pa={}", enc_json(&p.id), enc_json(&p.new_p_rd_pa))
}

/// 📖 Parses the arguments of `change-foundation-p-rd-pa`.
pub fn parse_change_foundation_p_rd_pa(args: &Args) -> Result<ChangeFoundationPRdPa, String> {
    let arg = |k: &str| required(args, "change-foundation-p-rd-pa", k);
    Ok(ChangeFoundationPRdPa { id: dec_json(&arg("id")?)?, new_p_rd_pa: dec_json(&arg("new-p-rd-pa")?)? })
}

/// ✍️ Prints `change-foundation-passive-resistance-n` as an op-text line.
pub fn print_change_foundation_passive_resistance_n(p: &ChangeFoundationPassiveResistanceN) -> String {
    format!("change-foundation-passive-resistance-n id={} new-passive-resistance-n={}", enc_json(&p.id), enc_json(&p.new_passive_resistance_n))
}

/// 📖 Parses the arguments of `change-foundation-passive-resistance-n`.
pub fn parse_change_foundation_passive_resistance_n(args: &Args) -> Result<ChangeFoundationPassiveResistanceN, String> {
    let arg = |k: &str| required(args, "change-foundation-passive-resistance-n", k);
    Ok(ChangeFoundationPassiveResistanceN { id: dec_json(&arg("id")?)?, new_passive_resistance_n: dec_json(&arg("new-passive-resistance-n")?)? })
}

/// ✍️ Prints `change-foundation-k-foundation` as an op-text line.
pub fn print_change_foundation_k_foundation(p: &ChangeFoundationKFoundation) -> String {
    format!("change-foundation-k-foundation id={} new-k-foundation={}", enc_json(&p.id), enc_json(&p.new_k_foundation))
}

/// 📖 Parses the arguments of `change-foundation-k-foundation`.
pub fn parse_change_foundation_k_foundation(args: &Args) -> Result<ChangeFoundationKFoundation, String> {
    let arg = |k: &str| required(args, "change-foundation-k-foundation", k);
    Ok(ChangeFoundationKFoundation { id: dec_json(&arg("id")?)?, new_k_foundation: dec_json(&arg("new-k-foundation")?)? })
}

/// ✍️ Prints `change-foundation-k-soil` as an op-text line.
pub fn print_change_foundation_k_soil(p: &ChangeFoundationKSoil) -> String {
    format!("change-foundation-k-soil id={} new-k-soil={}", enc_json(&p.id), enc_json(&p.new_k_soil))
}

/// 📖 Parses the arguments of `change-foundation-k-soil`.
pub fn parse_change_foundation_k_soil(args: &Args) -> Result<ChangeFoundationKSoil, String> {
    let arg = |k: &str| required(args, "change-foundation-k-soil", k);
    Ok(ChangeFoundationKSoil { id: dec_json(&arg("id")?)?, new_k_soil: dec_json(&arg("new-k-soil")?)? })
}

/// ✍️ Prints `change-retaining-wall-height-m` as an op-text line.
pub fn print_change_retaining_wall_height_m(p: &ChangeRetainingWallHeightM) -> String {
    format!("change-retaining-wall-height-m id={} new-height-m={}", enc_json(&p.id), enc_json(&p.new_height_m))
}

/// 📖 Parses the arguments of `change-retaining-wall-height-m`.
pub fn parse_change_retaining_wall_height_m(args: &Args) -> Result<ChangeRetainingWallHeightM, String> {
    let arg = |k: &str| required(args, "change-retaining-wall-height-m", k);
    Ok(ChangeRetainingWallHeightM { id: dec_json(&arg("id")?)?, new_height_m: dec_json(&arg("new-height-m")?)? })
}

/// ✍️ Prints `change-retaining-wall-phi-deg` as an op-text line.
pub fn print_change_retaining_wall_phi_deg(p: &ChangeRetainingWallPhiDeg) -> String {
    format!("change-retaining-wall-phi-deg id={} new-phi-deg={}", enc_json(&p.id), enc_json(&p.new_phi_deg))
}

/// 📖 Parses the arguments of `change-retaining-wall-phi-deg`.
pub fn parse_change_retaining_wall_phi_deg(args: &Args) -> Result<ChangeRetainingWallPhiDeg, String> {
    let arg = |k: &str| required(args, "change-retaining-wall-phi-deg", k);
    Ok(ChangeRetainingWallPhiDeg { id: dec_json(&arg("id")?)?, new_phi_deg: dec_json(&arg("new-phi-deg")?)? })
}

/// ✍️ Prints `change-retaining-wall-soil-gamma` as an op-text line.
pub fn print_change_retaining_wall_soil_gamma(p: &ChangeRetainingWallSoilGamma) -> String {
    format!("change-retaining-wall-soil-gamma id={} new-soil-gamma={}", enc_json(&p.id), enc_json(&p.new_soil_gamma))
}

/// 📖 Parses the arguments of `change-retaining-wall-soil-gamma`.
pub fn parse_change_retaining_wall_soil_gamma(args: &Args) -> Result<ChangeRetainingWallSoilGamma, String> {
    let arg = |k: &str| required(args, "change-retaining-wall-soil-gamma", k);
    Ok(ChangeRetainingWallSoilGamma { id: dec_json(&arg("id")?)?, new_soil_gamma: dec_json(&arg("new-soil-gamma")?)? })
}

/// ✍️ Prints `change-retaining-wall-r` as an op-text line.
pub fn print_change_retaining_wall_r(p: &ChangeRetainingWallR) -> String {
    format!("change-retaining-wall-r id={} new-r={}", enc_json(&p.id), enc_json(&p.new_r))
}

/// 📖 Parses the arguments of `change-retaining-wall-r`.
pub fn parse_change_retaining_wall_r(args: &Args) -> Result<ChangeRetainingWallR, String> {
    let arg = |k: &str| required(args, "change-retaining-wall-r", k);
    Ok(ChangeRetainingWallR { id: dec_json(&arg("id")?)?, new_r: dec_json(&arg("new-r")?)? })
}

/// ✍️ Prints `change-retaining-wall-h-rd-n-per-m` as an op-text line.
pub fn print_change_retaining_wall_h_rd_n_per_m(p: &ChangeRetainingWallHRdNPerM) -> String {
    format!("change-retaining-wall-h-rd-n-per-m id={} new-h-rd-n-per-m={}", enc_json(&p.id), enc_json(&p.new_h_rd_n_per_m))
}

/// 📖 Parses the arguments of `change-retaining-wall-h-rd-n-per-m`.
pub fn parse_change_retaining_wall_h_rd_n_per_m(args: &Args) -> Result<ChangeRetainingWallHRdNPerM, String> {
    let arg = |k: &str| required(args, "change-retaining-wall-h-rd-n-per-m", k);
    Ok(ChangeRetainingWallHRdNPerM { id: dec_json(&arg("id")?)?, new_h_rd_n_per_m: dec_json(&arg("new-h-rd-n-per-m")?)? })
}

/// ✍️ Prints `change-foundation-liq-n160` as an op-text line.
pub fn print_change_foundation_liq_n160(p: &ChangeFoundationLiqN160) -> String {
    format!("change-foundation-liq-n160 id={} new-liq-n160={}", enc_json(&p.id), enc_json(&p.new_liq_n160))
}

/// 📖 Parses the arguments of `change-foundation-liq-n160`.
pub fn parse_change_foundation_liq_n160(args: &Args) -> Result<ChangeFoundationLiqN160, String> {
    let arg = |k: &str| required(args, "change-foundation-liq-n160", k);
    Ok(ChangeFoundationLiqN160 { id: dec_json(&arg("id")?)?, new_liq_n160: dec_json(&arg("new-liq-n160")?)? })
}

/// ✍️ Prints `change-foundation-liq-fines-percent` as an op-text line.
pub fn print_change_foundation_liq_fines_percent(p: &ChangeFoundationLiqFinesPercent) -> String {
    format!("change-foundation-liq-fines-percent id={} new-liq-fines-percent={}", enc_json(&p.id), enc_json(&p.new_liq_fines_percent))
}

/// 📖 Parses the arguments of `change-foundation-liq-fines-percent`.
pub fn parse_change_foundation_liq_fines_percent(args: &Args) -> Result<ChangeFoundationLiqFinesPercent, String> {
    let arg = |k: &str| required(args, "change-foundation-liq-fines-percent", k);
    Ok(ChangeFoundationLiqFinesPercent { id: dec_json(&arg("id")?)?, new_liq_fines_percent: dec_json(&arg("new-liq-fines-percent")?)? })
}

/// ✍️ Prints `change-foundation-liq-depth-m` as an op-text line.
pub fn print_change_foundation_liq_depth_m(p: &ChangeFoundationLiqDepthM) -> String {
    format!("change-foundation-liq-depth-m id={} new-liq-depth-m={}", enc_json(&p.id), enc_json(&p.new_liq_depth_m))
}

/// 📖 Parses the arguments of `change-foundation-liq-depth-m`.
pub fn parse_change_foundation_liq_depth_m(args: &Args) -> Result<ChangeFoundationLiqDepthM, String> {
    let arg = |k: &str| required(args, "change-foundation-liq-depth-m", k);
    Ok(ChangeFoundationLiqDepthM { id: dec_json(&arg("id")?)?, new_liq_depth_m: dec_json(&arg("new-liq-depth-m")?)? })
}

/// ✍️ Prints `change-foundation-liq-water-depth-m` as an op-text line.
pub fn print_change_foundation_liq_water_depth_m(p: &ChangeFoundationLiqWaterDepthM) -> String {
    format!("change-foundation-liq-water-depth-m id={} new-liq-water-depth-m={}", enc_json(&p.id), enc_json(&p.new_liq_water_depth_m))
}

/// 📖 Parses the arguments of `change-foundation-liq-water-depth-m`.
pub fn parse_change_foundation_liq_water_depth_m(args: &Args) -> Result<ChangeFoundationLiqWaterDepthM, String> {
    let arg = |k: &str| required(args, "change-foundation-liq-water-depth-m", k);
    Ok(ChangeFoundationLiqWaterDepthM { id: dec_json(&arg("id")?)?, new_liq_water_depth_m: dec_json(&arg("new-liq-water-depth-m")?)? })
}

/// ✍️ Prints `change-foundation-liq-unit-wt-nm3` as an op-text line.
pub fn print_change_foundation_liq_unit_wt_n_m3(p: &ChangeFoundationLiqUnitWtNM3) -> String {
    format!("change-foundation-liq-unit-wt-nm3 id={} new-liq-unit-weight-n-m3={}", enc_json(&p.id), enc_json(&p.new_liq_unit_weight_n_m3))
}

/// 📖 Parses the arguments of `change-foundation-liq-unit-wt-nm3`.
pub fn parse_change_foundation_liq_unit_wt_n_m3(args: &Args) -> Result<ChangeFoundationLiqUnitWtNM3, String> {
    let arg = |k: &str| required(args, "change-foundation-liq-unit-wt-nm3", k);
    Ok(ChangeFoundationLiqUnitWtNM3 { id: dec_json(&arg("id")?)?, new_liq_unit_weight_n_m3: dec_json(&arg("new-liq-unit-weight-n-m3")?)? })
}

/// ✍️ Prints `change-foundation-liq-magnitude` as an op-text line.
pub fn print_change_foundation_liq_magnitude(p: &ChangeFoundationLiqMagnitude) -> String {
    format!("change-foundation-liq-magnitude id={} new-liq-magnitude={}", enc_json(&p.id), enc_json(&p.new_liq_magnitude))
}

/// 📖 Parses the arguments of `change-foundation-liq-magnitude`.
pub fn parse_change_foundation_liq_magnitude(args: &Args) -> Result<ChangeFoundationLiqMagnitude, String> {
    let arg = |k: &str| required(args, "change-foundation-liq-magnitude", k);
    Ok(ChangeFoundationLiqMagnitude { id: dec_json(&arg("id")?)?, new_liq_magnitude: dec_json(&arg("new-liq-magnitude")?)? })
}

/// ✍️ Prints `change-foundation-liq-layer-m` as an op-text line.
pub fn print_change_foundation_liq_layer_m(p: &ChangeFoundationLiqLayerM) -> String {
    format!("change-foundation-liq-layer-m id={} new-liq-layer-thickness-m={}", enc_json(&p.id), enc_json(&p.new_liq_layer_thickness_m))
}

/// 📖 Parses the arguments of `change-foundation-liq-layer-m`.
pub fn parse_change_foundation_liq_layer_m(args: &Args) -> Result<ChangeFoundationLiqLayerM, String> {
    let arg = |k: &str| required(args, "change-foundation-liq-layer-m", k);
    Ok(ChangeFoundationLiqLayerM { id: dec_json(&arg("id")?)?, new_liq_layer_thickness_m: dec_json(&arg("new-liq-layer-thickness-m")?)? })
}

/// ✍️ Prints `change-foundation-footing-width-m` as an op-text line.
pub fn print_change_foundation_footing_width_m(p: &ChangeFoundationFootingWidthM) -> String {
    format!("change-foundation-footing-width-m id={} new-footing-width-m={}", enc_json(&p.id), enc_json(&p.new_footing_width_m))
}

/// 📖 Parses the arguments of `change-foundation-footing-width-m`.
pub fn parse_change_foundation_footing_width_m(args: &Args) -> Result<ChangeFoundationFootingWidthM, String> {
    let arg = |k: &str| required(args, "change-foundation-footing-width-m", k);
    Ok(ChangeFoundationFootingWidthM { id: dec_json(&arg("id")?)?, new_footing_width_m: dec_json(&arg("new-footing-width-m")?)? })
}

/// ✍️ Prints `change-foundation-bear-axial-n` as an op-text line.
pub fn print_change_foundation_bear_axial_n(p: &ChangeFoundationBearAxialN) -> String {
    format!("change-foundation-bear-axial-n id={} new-bearing-axial-n={}", enc_json(&p.id), enc_json(&p.new_bearing_axial_n))
}

/// 📖 Parses the arguments of `change-foundation-bear-axial-n`.
pub fn parse_change_foundation_bear_axial_n(args: &Args) -> Result<ChangeFoundationBearAxialN, String> {
    let arg = |k: &str| required(args, "change-foundation-bear-axial-n", k);
    Ok(ChangeFoundationBearAxialN { id: dec_json(&arg("id")?)?, new_bearing_axial_n: dec_json(&arg("new-bearing-axial-n")?)? })
}

/// ✍️ Prints `change-foundation-bear-shear-n` as an op-text line.
pub fn print_change_foundation_bear_shear_n(p: &ChangeFoundationBearShearN) -> String {
    format!("change-foundation-bear-shear-n id={} new-bearing-shear-n={}", enc_json(&p.id), enc_json(&p.new_bearing_shear_n))
}

/// 📖 Parses the arguments of `change-foundation-bear-shear-n`.
pub fn parse_change_foundation_bear_shear_n(args: &Args) -> Result<ChangeFoundationBearShearN, String> {
    let arg = |k: &str| required(args, "change-foundation-bear-shear-n", k);
    Ok(ChangeFoundationBearShearN { id: dec_json(&arg("id")?)?, new_bearing_shear_n: dec_json(&arg("new-bearing-shear-n")?)? })
}

/// ✍️ Prints `change-foundation-bear-moment-nm` as an op-text line.
pub fn print_change_foundation_bear_moment_nm(p: &ChangeFoundationBearMomentNm) -> String {
    format!("change-foundation-bear-moment-nm id={} new-bearing-moment-nm={}", enc_json(&p.id), enc_json(&p.new_bearing_moment_nm))
}

/// 📖 Parses the arguments of `change-foundation-bear-moment-nm`.
pub fn parse_change_foundation_bear_moment_nm(args: &Args) -> Result<ChangeFoundationBearMomentNm, String> {
    let arg = |k: &str| required(args, "change-foundation-bear-moment-nm", k);
    Ok(ChangeFoundationBearMomentNm { id: dec_json(&arg("id")?)?, new_bearing_moment_nm: dec_json(&arg("new-bearing-moment-nm")?)? })
}

/// ✍️ Prints `change-foundation-bearing-n-max-n` as an op-text line.
pub fn print_change_foundation_bearing_n_max_n(p: &ChangeFoundationBearingNMaxN) -> String {
    format!("change-foundation-bearing-n-max-n id={} new-bearing-n-max-n={}", enc_json(&p.id), enc_json(&p.new_bearing_n_max_n))
}

/// 📖 Parses the arguments of `change-foundation-bearing-n-max-n`.
pub fn parse_change_foundation_bearing_n_max_n(args: &Args) -> Result<ChangeFoundationBearingNMaxN, String> {
    let arg = |k: &str| required(args, "change-foundation-bearing-n-max-n", k);
    Ok(ChangeFoundationBearingNMaxN { id: dec_json(&arg("id")?)?, new_bearing_n_max_n: dec_json(&arg("new-bearing-n-max-n")?)? })
}

/// ✍️ Prints `change-foundation-soil-phi-deg` as an op-text line.
pub fn print_change_foundation_soil_phi_deg(p: &ChangeFoundationSoilPhiDeg) -> String {
    format!("change-foundation-soil-phi-deg id={} new-soil-friction-angle-deg={}", enc_json(&p.id), enc_json(&p.new_soil_friction_angle_deg))
}

/// 📖 Parses the arguments of `change-foundation-soil-phi-deg`.
pub fn parse_change_foundation_soil_phi_deg(args: &Args) -> Result<ChangeFoundationSoilPhiDeg, String> {
    let arg = |k: &str| required(args, "change-foundation-soil-phi-deg", k);
    Ok(ChangeFoundationSoilPhiDeg { id: dec_json(&arg("id")?)?, new_soil_friction_angle_deg: dec_json(&arg("new-soil-friction-angle-deg")?)? })
}

/// ✍️ Prints `change-foundation-pile-diameter-m` as an op-text line.
pub fn print_change_foundation_pile_diameter_m(p: &ChangeFoundationPileDiameterM) -> String {
    format!("change-foundation-pile-diameter-m id={} new-pile-diameter-m={}", enc_json(&p.id), enc_json(&p.new_pile_diameter_m))
}

/// 📖 Parses the arguments of `change-foundation-pile-diameter-m`.
pub fn parse_change_foundation_pile_diameter_m(args: &Args) -> Result<ChangeFoundationPileDiameterM, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-diameter-m", k);
    Ok(ChangeFoundationPileDiameterM { id: dec_json(&arg("id")?)?, new_pile_diameter_m: dec_json(&arg("new-pile-diameter-m")?)? })
}

/// ✍️ Prints `change-foundation-pile-length-m` as an op-text line.
pub fn print_change_foundation_pile_length_m(p: &ChangeFoundationPileLengthM) -> String {
    format!("change-foundation-pile-length-m id={} new-pile-length-m={}", enc_json(&p.id), enc_json(&p.new_pile_length_m))
}

/// 📖 Parses the arguments of `change-foundation-pile-length-m`.
pub fn parse_change_foundation_pile_length_m(args: &Args) -> Result<ChangeFoundationPileLengthM, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-length-m", k);
    Ok(ChangeFoundationPileLengthM { id: dec_json(&arg("id")?)?, new_pile_length_m: dec_json(&arg("new-pile-length-m")?)? })
}

/// ✍️ Prints `change-foundation-pile-modulus-pa` as an op-text line.
pub fn print_change_foundation_pile_modulus_pa(p: &ChangeFoundationPileModulusPa) -> String {
    format!("change-foundation-pile-modulus-pa id={} new-pile-modulus-pa={}", enc_json(&p.id), enc_json(&p.new_pile_modulus_pa))
}

/// 📖 Parses the arguments of `change-foundation-pile-modulus-pa`.
pub fn parse_change_foundation_pile_modulus_pa(args: &Args) -> Result<ChangeFoundationPileModulusPa, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-modulus-pa", k);
    Ok(ChangeFoundationPileModulusPa { id: dec_json(&arg("id")?)?, new_pile_modulus_pa: dec_json(&arg("new-pile-modulus-pa")?)? })
}

/// ✍️ Prints `change-foundation-pile-ks-nm3` as an op-text line.
pub fn print_change_foundation_pile_ks_n_m3(p: &ChangeFoundationPileKsNM3) -> String {
    format!("change-foundation-pile-ks-nm3 id={} new-pile-subgrade-modulus-n-m3={}", enc_json(&p.id), enc_json(&p.new_pile_subgrade_modulus_n_m3))
}

/// 📖 Parses the arguments of `change-foundation-pile-ks-nm3`.
pub fn parse_change_foundation_pile_ks_n_m3(args: &Args) -> Result<ChangeFoundationPileKsNM3, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-ks-nm3", k);
    Ok(ChangeFoundationPileKsNM3 { id: dec_json(&arg("id")?)?, new_pile_subgrade_modulus_n_m3: dec_json(&arg("new-pile-subgrade-modulus-n-m3")?)? })
}

/// ✍️ Prints `change-foundation-pile-head-shear-n` as an op-text line.
pub fn print_change_foundation_pile_head_shear_n(p: &ChangeFoundationPileHeadShearN) -> String {
    format!("change-foundation-pile-head-shear-n id={} new-pile-head-shear-n={}", enc_json(&p.id), enc_json(&p.new_pile_head_shear_n))
}

/// 📖 Parses the arguments of `change-foundation-pile-head-shear-n`.
pub fn parse_change_foundation_pile_head_shear_n(args: &Args) -> Result<ChangeFoundationPileHeadShearN, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-head-shear-n", k);
    Ok(ChangeFoundationPileHeadShearN { id: dec_json(&arg("id")?)?, new_pile_head_shear_n: dec_json(&arg("new-pile-head-shear-n")?)? })
}

/// ✍️ Prints `change-foundation-pile-head-fixed` as an op-text line.
pub fn print_change_foundation_pile_head_fixed(p: &ChangeFoundationPileHeadFixed) -> String {
    format!("change-foundation-pile-head-fixed id={} new-pile-head-fixed={}", enc_json(&p.id), enc_json(&p.new_pile_head_fixed))
}

/// 📖 Parses the arguments of `change-foundation-pile-head-fixed`.
pub fn parse_change_foundation_pile_head_fixed(args: &Args) -> Result<ChangeFoundationPileHeadFixed, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-head-fixed", k);
    Ok(ChangeFoundationPileHeadFixed { id: dec_json(&arg("id")?)?, new_pile_head_fixed: dec_json(&arg("new-pile-head-fixed")?)? })
}

/// ✍️ Prints `change-foundation-pile-iface-m` as an op-text line.
pub fn print_change_foundation_pile_iface_m(p: &ChangeFoundationPileIfaceM) -> String {
    format!("change-foundation-pile-iface-m id={} new-pile-interface-depth-m={}", enc_json(&p.id), enc_json(&p.new_pile_interface_depth_m))
}

/// 📖 Parses the arguments of `change-foundation-pile-iface-m`.
pub fn parse_change_foundation_pile_iface_m(args: &Args) -> Result<ChangeFoundationPileIfaceM, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-iface-m", k);
    Ok(ChangeFoundationPileIfaceM { id: dec_json(&arg("id")?)?, new_pile_interface_depth_m: dec_json(&arg("new-pile-interface-depth-m")?)? })
}

/// ✍️ Prints `change-foundation-pile-up-vs-ms` as an op-text line.
pub fn print_change_foundation_pile_up_vs_m_s(p: &ChangeFoundationPileUpVsMS) -> String {
    format!("change-foundation-pile-up-vs-ms id={} new-pile-upper-vs-m-s={}", enc_json(&p.id), enc_json(&p.new_pile_upper_vs_m_s))
}

/// 📖 Parses the arguments of `change-foundation-pile-up-vs-ms`.
pub fn parse_change_foundation_pile_up_vs_m_s(args: &Args) -> Result<ChangeFoundationPileUpVsMS, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-up-vs-ms", k);
    Ok(ChangeFoundationPileUpVsMS { id: dec_json(&arg("id")?)?, new_pile_upper_vs_m_s: dec_json(&arg("new-pile-upper-vs-m-s")?)? })
}

/// ✍️ Prints `change-foundation-pile-low-vs-ms` as an op-text line.
pub fn print_change_foundation_pile_low_vs_m_s(p: &ChangeFoundationPileLowVsMS) -> String {
    format!("change-foundation-pile-low-vs-ms id={} new-pile-lower-vs-m-s={}", enc_json(&p.id), enc_json(&p.new_pile_lower_vs_m_s))
}

/// 📖 Parses the arguments of `change-foundation-pile-low-vs-ms`.
pub fn parse_change_foundation_pile_low_vs_m_s(args: &Args) -> Result<ChangeFoundationPileLowVsMS, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-low-vs-ms", k);
    Ok(ChangeFoundationPileLowVsMS { id: dec_json(&arg("id")?)?, new_pile_lower_vs_m_s: dec_json(&arg("new-pile-lower-vs-m-s")?)? })
}

/// ✍️ Prints `change-foundation-pile-moment-rd-nm` as an op-text line.
pub fn print_change_foundation_pile_moment_rd_nm(p: &ChangeFoundationPileMomentRdNm) -> String {
    format!("change-foundation-pile-moment-rd-nm id={} new-pile-moment-rd-nm={}", enc_json(&p.id), enc_json(&p.new_pile_moment_rd_nm))
}

/// 📖 Parses the arguments of `change-foundation-pile-moment-rd-nm`.
pub fn parse_change_foundation_pile_moment_rd_nm(args: &Args) -> Result<ChangeFoundationPileMomentRdNm, String> {
    let arg = |k: &str| required(args, "change-foundation-pile-moment-rd-nm", k);
    Ok(ChangeFoundationPileMomentRdNm { id: dec_json(&arg("id")?)?, new_pile_moment_rd_nm: dec_json(&arg("new-pile-moment-rd-nm")?)? })
}

/// ✍️ Prints `change-soil-layer-foundation-id` as an op-text line.
pub fn print_change_soil_layer_foundation_id(p: &ChangeSoilLayerFoundationId) -> String {
    format!("change-soil-layer-foundation-id id={} new-foundation-id={}", enc_json(&p.id), enc_json(&p.new_foundation_id))
}

/// 📖 Parses the arguments of `change-soil-layer-foundation-id`.
pub fn parse_change_soil_layer_foundation_id(args: &Args) -> Result<ChangeSoilLayerFoundationId, String> {
    let arg = |k: &str| required(args, "change-soil-layer-foundation-id", k);
    Ok(ChangeSoilLayerFoundationId { id: dec_json(&arg("id")?)?, new_foundation_id: dec_json(&arg("new-foundation-id")?)? })
}

/// ✍️ Prints `change-soil-layer-thickness-m` as an op-text line.
pub fn print_change_soil_layer_thickness_m(p: &ChangeSoilLayerThicknessM) -> String {
    format!("change-soil-layer-thickness-m id={} new-thickness-m={}", enc_json(&p.id), enc_json(&p.new_thickness_m))
}

/// 📖 Parses the arguments of `change-soil-layer-thickness-m`.
pub fn parse_change_soil_layer_thickness_m(args: &Args) -> Result<ChangeSoilLayerThicknessM, String> {
    let arg = |k: &str| required(args, "change-soil-layer-thickness-m", k);
    Ok(ChangeSoilLayerThicknessM { id: dec_json(&arg("id")?)?, new_thickness_m: dec_json(&arg("new-thickness-m")?)? })
}

/// ✍️ Prints `change-soil-unit-wt-nm3` as an op-text line.
pub fn print_change_soil_unit_wt_n_m3(p: &ChangeSoilUnitWtNM3) -> String {
    format!("change-soil-unit-wt-nm3 id={} new-unit-weight-n-m3={}", enc_json(&p.id), enc_json(&p.new_unit_weight_n_m3))
}

/// 📖 Parses the arguments of `change-soil-unit-wt-nm3`.
pub fn parse_change_soil_unit_wt_n_m3(args: &Args) -> Result<ChangeSoilUnitWtNM3, String> {
    let arg = |k: &str| required(args, "change-soil-unit-wt-nm3", k);
    Ok(ChangeSoilUnitWtNM3 { id: dec_json(&arg("id")?)?, new_unit_weight_n_m3: dec_json(&arg("new-unit-weight-n-m3")?)? })
}

/// ✍️ Prints `change-soil-layer-vs-ms` as an op-text line.
pub fn print_change_soil_layer_vs_m_s(p: &ChangeSoilLayerVsMS) -> String {
    format!("change-soil-layer-vs-ms id={} new-shear-wave-velocity-m-s={}", enc_json(&p.id), enc_json(&p.new_shear_wave_velocity_m_s))
}

/// 📖 Parses the arguments of `change-soil-layer-vs-ms`.
pub fn parse_change_soil_layer_vs_m_s(args: &Args) -> Result<ChangeSoilLayerVsMS, String> {
    let arg = |k: &str| required(args, "change-soil-layer-vs-ms", k);
    Ok(ChangeSoilLayerVsMS { id: dec_json(&arg("id")?)?, new_shear_wave_velocity_m_s: dec_json(&arg("new-shear-wave-velocity-m-s")?)? })
}

/// ✍️ Prints `change-soil-layer-n160` as an op-text line.
pub fn print_change_soil_layer_n1_60(p: &ChangeSoilLayerN160) -> String {
    format!("change-soil-layer-n160 id={} new-n1-60={}", enc_json(&p.id), enc_json(&p.new_n1_60))
}

/// 📖 Parses the arguments of `change-soil-layer-n160`.
pub fn parse_change_soil_layer_n1_60(args: &Args) -> Result<ChangeSoilLayerN160, String> {
    let arg = |k: &str| required(args, "change-soil-layer-n160", k);
    Ok(ChangeSoilLayerN160 { id: dec_json(&arg("id")?)?, new_n1_60: dec_json(&arg("new-n1-60")?)? })
}

/// ✍️ Prints `change-soil-layer-fines-percent` as an op-text line.
pub fn print_change_soil_layer_fines_percent(p: &ChangeSoilLayerFinesPercent) -> String {
    format!("change-soil-layer-fines-percent id={} new-fines-percent={}", enc_json(&p.id), enc_json(&p.new_fines_percent))
}

/// 📖 Parses the arguments of `change-soil-layer-fines-percent`.
pub fn parse_change_soil_layer_fines_percent(args: &Args) -> Result<ChangeSoilLayerFinesPercent, String> {
    let arg = |k: &str| required(args, "change-soil-layer-fines-percent", k);
    Ok(ChangeSoilLayerFinesPercent { id: dec_json(&arg("id")?)?, new_fines_percent: dec_json(&arg("new-fines-percent")?)? })
}

/// ✍️ Prints `insert-soil-layer` as an op-text line.
pub fn print_insert_soil_layer(p: &InsertSoilLayer) -> String {
    format!("insert-soil-layer index={} layer={}", enc_json(&p.index), enc_json(&p.layer))
}

/// 📖 Parses the arguments of `insert-soil-layer`.
pub fn parse_insert_soil_layer(args: &Args) -> Result<InsertSoilLayer, String> {
    let arg = |k: &str| required(args, "insert-soil-layer", k);
    Ok(InsertSoilLayer { index: opt_index(args)?, layer: dec_json(&arg("layer")?)? })
}

/// ✍️ Prints `remove-soil-layer` as an op-text line.
pub fn print_remove_soil_layer(p: &RemoveSoilLayer) -> String {
    format!("remove-soil-layer id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-soil-layer`.
pub fn parse_remove_soil_layer(args: &Args) -> Result<RemoveSoilLayer, String> {
    let arg = |k: &str| required(args, "remove-soil-layer", k);
    Ok(RemoveSoilLayer { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-retaining-wall-water-height-m` as an op-text line.
pub fn print_change_retaining_wall_water_height_m(p: &ChangeRetainingWallWaterHeightM) -> String {
    format!("change-retaining-wall-water-height-m index={} new-water-height-m={}", enc_json(&p.index), enc_json(&p.new_water_height_m))
}

/// 📖 Parses the arguments of `change-retaining-wall-water-height-m`.
pub fn parse_change_retaining_wall_water_height_m(args: &Args) -> Result<ChangeRetainingWallWaterHeightM, String> {
    let arg = |k: &str| required(args, "change-retaining-wall-water-height-m", k);
    Ok(ChangeRetainingWallWaterHeightM { index: opt_index(args)?, new_water_height_m: dec_json(&arg("new-water-height-m")?)? })
}

/// ✍️ Prints `change-retaining-wall-saturated-gamma` as an op-text line.
pub fn print_change_retaining_wall_saturated_gamma(p: &ChangeRetainingWallSaturatedGamma) -> String {
    format!("change-retaining-wall-saturated-gamma index={} new-saturated-gamma={}", enc_json(&p.index), enc_json(&p.new_saturated_gamma))
}

/// 📖 Parses the arguments of `change-retaining-wall-saturated-gamma`.
pub fn parse_change_retaining_wall_saturated_gamma(args: &Args) -> Result<ChangeRetainingWallSaturatedGamma, String> {
    let arg = |k: &str| required(args, "change-retaining-wall-saturated-gamma", k);
    Ok(ChangeRetainingWallSaturatedGamma { index: opt_index(args)?, new_saturated_gamma: dec_json(&arg("new-saturated-gamma")?)? })
}

/// ✍️ Prints `change-retaining-wall-permeability-ms` as an op-text line.
pub fn print_change_retaining_wall_permeability_m_s(p: &ChangeRetainingWallPermeabilityMS) -> String {
    format!("change-retaining-wall-permeability-ms index={} new-permeability-m-s={}", enc_json(&p.index), enc_json(&p.new_permeability_m_s))
}

/// 📖 Parses the arguments of `change-retaining-wall-permeability-ms`.
pub fn parse_change_retaining_wall_permeability_m_s(args: &Args) -> Result<ChangeRetainingWallPermeabilityMS, String> {
    let arg = |k: &str| required(args, "change-retaining-wall-permeability-ms", k);
    Ok(ChangeRetainingWallPermeabilityMS { index: opt_index(args)?, new_permeability_m_s: dec_json(&arg("new-permeability-m-s")?)? })
}

/// ✍️ Prints `change-foundation-base-friction-deg` as an op-text line.
pub fn print_change_foundation_base_friction_deg(p: &ChangeFoundationBaseFrictionDeg) -> String {
    format!("change-foundation-base-friction-deg index={} new-base-friction-angle-deg={}", enc_json(&p.index), enc_json(&p.new_base_friction_angle_deg))
}

/// 📖 Parses the arguments of `change-foundation-base-friction-deg`.
pub fn parse_change_foundation_base_friction_deg(args: &Args) -> Result<ChangeFoundationBaseFrictionDeg, String> {
    let arg = |k: &str| required(args, "change-foundation-base-friction-deg", k);
    Ok(ChangeFoundationBaseFrictionDeg { index: opt_index(args)?, new_base_friction_angle_deg: dec_json(&arg("new-base-friction-angle-deg")?)? })
}

/// ✍️ Prints `change-foundation-base-pore-pressure-pa` as an op-text line.
pub fn print_change_foundation_base_pore_pressure_pa(p: &ChangeFoundationBasePorePressurePa) -> String {
    format!("change-foundation-base-pore-pressure-pa index={} new-base-pore-pressure-pa={}", enc_json(&p.index), enc_json(&p.new_base_pore_pressure_pa))
}

/// 📖 Parses the arguments of `change-foundation-base-pore-pressure-pa`.
pub fn parse_change_foundation_base_pore_pressure_pa(args: &Args) -> Result<ChangeFoundationBasePorePressurePa, String> {
    let arg = |k: &str| required(args, "change-foundation-base-pore-pressure-pa", k);
    Ok(ChangeFoundationBasePorePressurePa { index: opt_index(args)?, new_base_pore_pressure_pa: dec_json(&arg("new-base-pore-pressure-pa")?)? })
}

/// ✍️ Prints `remove-foundation` as an op-text line.
pub fn print_remove_foundation(p: &RemoveFoundation) -> String {
    format!("remove-foundation id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-foundation`.
pub fn parse_remove_foundation(args: &Args) -> Result<RemoveFoundation, String> {
    let arg = |k: &str| required(args, "remove-foundation", k);
    Ok(RemoveFoundation { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `remove-retaining-wall` as an op-text line.
pub fn print_remove_retaining_wall(p: &RemoveRetainingWall) -> String {
    format!("remove-retaining-wall id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-retaining-wall`.
pub fn parse_remove_retaining_wall(args: &Args) -> Result<RemoveRetainingWall, String> {
    let arg = |k: &str| required(args, "remove-retaining-wall", k);
    Ok(RemoveRetainingWall { id: dec_json(&arg("id")?)? })
}
