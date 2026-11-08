//! 🔗️ EN 1998 tie beams mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_displacement_limit::ChangeTieBeamDisplacementLimit;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_relative_displacement::ChangeTieBeamRelativeDisplacement;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_steel_design_strength::ChangeTieBeamSteelDesignStrength;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_steel_area::ChangeTieBeamSteelArea;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_depth::ChangeTieBeamDepth;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_width::ChangeTieBeamWidth;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_span::ChangeTieBeamSpan;
use semio_s_artifact_norm_en1998_mut_tie_beams::insert_isolation_system::InsertIsolationSystem;
use semio_s_artifact_norm_en1998_mut_tie_beams::remove_isolation_system::RemoveIsolationSystem;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_isolation_system_bridge_id::ChangeIsolationSystemBridgeId;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_isolation_system_building_id::ChangeIsolationSystemBuildingId;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_isolation_system_strength_n::ChangeIsolationSystemStrengthN;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_isolation_system_stiffness::ChangeIsolationSystemStiffnessNPerM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_isolation_system_yield_disp_m::ChangeIsolationSystemYieldDisplacementM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_isolation_system_capacity_m::ChangeIsolationSystemCapacityM;
use semio_s_artifact_norm_en1998_mut_tie_beams::insert_pier_base::InsertPierBase;
use semio_s_artifact_norm_en1998_mut_tie_beams::remove_pier_base::RemovePierBase;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_pier_base_bridge_id::ChangePierBaseBridgeId;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_pier_base_pier_id::ChangePierBasePierId;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_pier_base_radius_m::ChangePierBaseRadiusM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_pier_base_soil_vs_m_s::ChangePierBaseSoilVsMS;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_pier_base_soil_unit_weight_n_m3::ChangePierBaseSoilUnitWeightNM3;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_pier_base_poisson_ratio::ChangePierBasePoissonRatio;
use semio_s_artifact_norm_en1998_mut_tie_beams::insert_critical_region::InsertCriticalRegion;
use semio_s_artifact_norm_en1998_mut_tie_beams::remove_critical_region::RemoveCriticalRegion;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_building_id::ChangeCriticalRegionBuildingId;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_member_id::ChangeCriticalRegionMemberId;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_section_width_m::ChangeCriticalRegionSectionWidthM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_section_depth_m::ChangeCriticalRegionSectionDepthM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_clear_length_m::ChangeCriticalRegionClearLengthM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_hooped_length_m::ChangeCriticalRegionHoopedLengthM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_core_width_m::ChangeCriticalRegionCoreWidthM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_core_depth_m::ChangeCriticalRegionCoreDepthM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_axial_force_n::ChangeCriticalRegionAxialForceN;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_concrete_fcd::ChangeCriticalRegionConcreteDesignStrengthMpa;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_steel_fyd::ChangeCriticalRegionSteelDesignYieldMpa;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_hoop_fywd::ChangeCriticalRegionHoopDesignYieldMpa;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_long_steel_area::ChangeCriticalRegionLongitudinalSteelAreaM2;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_long_bar_dia::ChangeCriticalRegionLongitudinalBarDiameterM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_long_bar_max::ChangeCriticalRegionLongitudinalBarMaxDiameterM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_hoop_diameter_m::ChangeCriticalRegionHoopDiameterM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_hoop_spacing_m::ChangeCriticalRegionHoopSpacingM;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_hoop_legs_x::ChangeCriticalRegionHoopLegsX;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_hoop_legs_y::ChangeCriticalRegionHoopLegsY;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_restr_bar_count::ChangeCriticalRegionRestrainedBarCount;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_critical_region_restr_bar_space::ChangeCriticalRegionRestrainedBarSpacingM;

/// ✍️ Prints `change-tie-beam-displacement-limit` as an op-text line.
pub fn print_change_tie_beam_displacement_limit(p: &ChangeTieBeamDisplacementLimit) -> String {
    format!("change-tie-beam-displacement-limit id={} new-displacement-limit={}", enc_json(&p.id), enc_json(&p.new_displacement_limit))
}

/// 📖 Parses the arguments of `change-tie-beam-displacement-limit`.
pub fn parse_change_tie_beam_displacement_limit(args: &Args) -> Result<ChangeTieBeamDisplacementLimit, String> {
    let arg = |k: &str| required(args, "change-tie-beam-displacement-limit", k);
    Ok(ChangeTieBeamDisplacementLimit { id: dec_json(&arg("id")?)?, new_displacement_limit: dec_json(&arg("new-displacement-limit")?)? })
}

/// ✍️ Prints `change-tie-beam-relative-displacement` as an op-text line.
pub fn print_change_tie_beam_relative_displacement(p: &ChangeTieBeamRelativeDisplacement) -> String {
    format!("change-tie-beam-relative-displacement id={} new-relative-displacement={}", enc_json(&p.id), enc_json(&p.new_relative_displacement))
}

/// 📖 Parses the arguments of `change-tie-beam-relative-displacement`.
pub fn parse_change_tie_beam_relative_displacement(args: &Args) -> Result<ChangeTieBeamRelativeDisplacement, String> {
    let arg = |k: &str| required(args, "change-tie-beam-relative-displacement", k);
    Ok(ChangeTieBeamRelativeDisplacement { id: dec_json(&arg("id")?)?, new_relative_displacement: dec_json(&arg("new-relative-displacement")?)? })
}

/// ✍️ Prints `change-tie-beam-steel-design-strength` as an op-text line.
pub fn print_change_tie_beam_steel_design_strength(p: &ChangeTieBeamSteelDesignStrength) -> String {
    format!("change-tie-beam-steel-design-strength id={} new-steel-design-strength={}", enc_json(&p.id), enc_json(&p.new_steel_design_strength))
}

/// 📖 Parses the arguments of `change-tie-beam-steel-design-strength`.
pub fn parse_change_tie_beam_steel_design_strength(args: &Args) -> Result<ChangeTieBeamSteelDesignStrength, String> {
    let arg = |k: &str| required(args, "change-tie-beam-steel-design-strength", k);
    Ok(ChangeTieBeamSteelDesignStrength { id: dec_json(&arg("id")?)?, new_steel_design_strength: dec_json(&arg("new-steel-design-strength")?)? })
}

/// ✍️ Prints `change-tie-beam-steel-area` as an op-text line.
pub fn print_change_tie_beam_steel_area(p: &ChangeTieBeamSteelArea) -> String {
    format!("change-tie-beam-steel-area id={} new-steel-area={}", enc_json(&p.id), enc_json(&p.new_steel_area))
}

/// 📖 Parses the arguments of `change-tie-beam-steel-area`.
pub fn parse_change_tie_beam_steel_area(args: &Args) -> Result<ChangeTieBeamSteelArea, String> {
    let arg = |k: &str| required(args, "change-tie-beam-steel-area", k);
    Ok(ChangeTieBeamSteelArea { id: dec_json(&arg("id")?)?, new_steel_area: dec_json(&arg("new-steel-area")?)? })
}

/// ✍️ Prints `change-tie-beam-depth` as an op-text line.
pub fn print_change_tie_beam_depth(p: &ChangeTieBeamDepth) -> String {
    format!("change-tie-beam-depth id={} new-depth={}", enc_json(&p.id), enc_json(&p.new_depth))
}

/// 📖 Parses the arguments of `change-tie-beam-depth`.
pub fn parse_change_tie_beam_depth(args: &Args) -> Result<ChangeTieBeamDepth, String> {
    let arg = |k: &str| required(args, "change-tie-beam-depth", k);
    Ok(ChangeTieBeamDepth { id: dec_json(&arg("id")?)?, new_depth: dec_json(&arg("new-depth")?)? })
}

/// ✍️ Prints `change-tie-beam-width` as an op-text line.
pub fn print_change_tie_beam_width(p: &ChangeTieBeamWidth) -> String {
    format!("change-tie-beam-width id={} new-width={}", enc_json(&p.id), enc_json(&p.new_width))
}

/// 📖 Parses the arguments of `change-tie-beam-width`.
pub fn parse_change_tie_beam_width(args: &Args) -> Result<ChangeTieBeamWidth, String> {
    let arg = |k: &str| required(args, "change-tie-beam-width", k);
    Ok(ChangeTieBeamWidth { id: dec_json(&arg("id")?)?, new_width: dec_json(&arg("new-width")?)? })
}

/// ✍️ Prints `change-tie-beam-span` as an op-text line.
pub fn print_change_tie_beam_span(p: &ChangeTieBeamSpan) -> String {
    format!("change-tie-beam-span id={} new-span={}", enc_json(&p.id), enc_json(&p.new_span))
}

/// 📖 Parses the arguments of `change-tie-beam-span`.
pub fn parse_change_tie_beam_span(args: &Args) -> Result<ChangeTieBeamSpan, String> {
    let arg = |k: &str| required(args, "change-tie-beam-span", k);
    Ok(ChangeTieBeamSpan { id: dec_json(&arg("id")?)?, new_span: dec_json(&arg("new-span")?)? })
}

/// ✍️ Prints `insert-isolation-system` as an op-text line.
pub fn print_insert_isolation_system(p: &InsertIsolationSystem) -> String {
    format!("insert-isolation-system index={} isolation_system={}", enc_json(&p.index), enc_json(&p.isolation_system))
}

/// 📖 Parses the arguments of `insert-isolation-system`.
pub fn parse_insert_isolation_system(args: &Args) -> Result<InsertIsolationSystem, String> {
    let arg = |k: &str| required(args, "insert-isolation-system", k);
    Ok(InsertIsolationSystem { index: opt_index(args)?, isolation_system: dec_json(&arg("isolation_system")?)? })
}

/// ✍️ Prints `remove-isolation-system` as an op-text line.
pub fn print_remove_isolation_system(p: &RemoveIsolationSystem) -> String {
    format!("remove-isolation-system id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-isolation-system`.
pub fn parse_remove_isolation_system(args: &Args) -> Result<RemoveIsolationSystem, String> {
    let arg = |k: &str| required(args, "remove-isolation-system", k);
    Ok(RemoveIsolationSystem { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-isolation-system-bridge-id` as an op-text line.
pub fn print_change_isolation_system_bridge_id(p: &ChangeIsolationSystemBridgeId) -> String {
    format!("change-isolation-system-bridge-id id={} new-bridge-id={}", enc_json(&p.id), enc_json(&p.new_bridge_id))
}

/// 📖 Parses the arguments of `change-isolation-system-bridge-id`.
pub fn parse_change_isolation_system_bridge_id(args: &Args) -> Result<ChangeIsolationSystemBridgeId, String> {
    let arg = |k: &str| required(args, "change-isolation-system-bridge-id", k);
    Ok(ChangeIsolationSystemBridgeId { id: dec_json(&arg("id")?)?, new_bridge_id: dec_json(&arg("new-bridge-id")?)? })
}

/// ✍️ Prints `change-isolation-system-building-id` as an op-text line.
pub fn print_change_isolation_system_building_id(p: &ChangeIsolationSystemBuildingId) -> String {
    format!("change-isolation-system-building-id id={} new-building-id={}", enc_json(&p.id), enc_json(&p.new_building_id))
}

/// 📖 Parses the arguments of `change-isolation-system-building-id`.
pub fn parse_change_isolation_system_building_id(args: &Args) -> Result<ChangeIsolationSystemBuildingId, String> {
    let arg = |k: &str| required(args, "change-isolation-system-building-id", k);
    Ok(ChangeIsolationSystemBuildingId { id: dec_json(&arg("id")?)?, new_building_id: dec_json(&arg("new-building-id")?)? })
}

/// ✍️ Prints `change-isolation-system-strength-n` as an op-text line.
pub fn print_change_isolation_system_strength_n(p: &ChangeIsolationSystemStrengthN) -> String {
    format!("change-isolation-system-strength-n id={} new-strength-n={}", enc_json(&p.id), enc_json(&p.new_strength_n))
}

/// 📖 Parses the arguments of `change-isolation-system-strength-n`.
pub fn parse_change_isolation_system_strength_n(args: &Args) -> Result<ChangeIsolationSystemStrengthN, String> {
    let arg = |k: &str| required(args, "change-isolation-system-strength-n", k);
    Ok(ChangeIsolationSystemStrengthN { id: dec_json(&arg("id")?)?, new_strength_n: dec_json(&arg("new-strength-n")?)? })
}

/// ✍️ Prints `change-isolation-system-stiffness` as an op-text line.
pub fn print_change_isolation_system_stiffness(p: &ChangeIsolationSystemStiffnessNPerM) -> String {
    format!("change-isolation-system-stiffness id={} new-stiffness-n-per-m={}", enc_json(&p.id), enc_json(&p.new_stiffness_n_per_m))
}

/// 📖 Parses the arguments of `change-isolation-system-stiffness`.
pub fn parse_change_isolation_system_stiffness(args: &Args) -> Result<ChangeIsolationSystemStiffnessNPerM, String> {
    let arg = |k: &str| required(args, "change-isolation-system-stiffness", k);
    Ok(ChangeIsolationSystemStiffnessNPerM { id: dec_json(&arg("id")?)?, new_stiffness_n_per_m: dec_json(&arg("new-stiffness-n-per-m")?)? })
}

/// ✍️ Prints `change-isolation-system-yield-disp-m` as an op-text line.
pub fn print_change_isolation_system_yield_disp_m(p: &ChangeIsolationSystemYieldDisplacementM) -> String {
    format!("change-isolation-system-yield-disp-m id={} new-yield-displacement-m={}", enc_json(&p.id), enc_json(&p.new_yield_displacement_m))
}

/// 📖 Parses the arguments of `change-isolation-system-yield-disp-m`.
pub fn parse_change_isolation_system_yield_disp_m(args: &Args) -> Result<ChangeIsolationSystemYieldDisplacementM, String> {
    let arg = |k: &str| required(args, "change-isolation-system-yield-disp-m", k);
    Ok(ChangeIsolationSystemYieldDisplacementM { id: dec_json(&arg("id")?)?, new_yield_displacement_m: dec_json(&arg("new-yield-displacement-m")?)? })
}

/// ✍️ Prints `change-isolation-system-capacity-m` as an op-text line.
pub fn print_change_isolation_system_capacity_m(p: &ChangeIsolationSystemCapacityM) -> String {
    format!("change-isolation-system-capacity-m id={} new-capacity-m={}", enc_json(&p.id), enc_json(&p.new_capacity_m))
}

/// 📖 Parses the arguments of `change-isolation-system-capacity-m`.
pub fn parse_change_isolation_system_capacity_m(args: &Args) -> Result<ChangeIsolationSystemCapacityM, String> {
    let arg = |k: &str| required(args, "change-isolation-system-capacity-m", k);
    Ok(ChangeIsolationSystemCapacityM { id: dec_json(&arg("id")?)?, new_capacity_m: dec_json(&arg("new-capacity-m")?)? })
}

/// ✍️ Prints `insert-pier-base` as an op-text line.
pub fn print_insert_pier_base(p: &InsertPierBase) -> String {
    format!("insert-pier-base index={} pier_base={}", enc_json(&p.index), enc_json(&p.pier_base))
}

/// 📖 Parses the arguments of `insert-pier-base`.
pub fn parse_insert_pier_base(args: &Args) -> Result<InsertPierBase, String> {
    let arg = |k: &str| required(args, "insert-pier-base", k);
    Ok(InsertPierBase { index: opt_index(args)?, pier_base: dec_json(&arg("pier_base")?)? })
}

/// ✍️ Prints `remove-pier-base` as an op-text line.
pub fn print_remove_pier_base(p: &RemovePierBase) -> String {
    format!("remove-pier-base id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-pier-base`.
pub fn parse_remove_pier_base(args: &Args) -> Result<RemovePierBase, String> {
    let arg = |k: &str| required(args, "remove-pier-base", k);
    Ok(RemovePierBase { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-pier-base-bridge-id` as an op-text line.
pub fn print_change_pier_base_bridge_id(p: &ChangePierBaseBridgeId) -> String {
    format!("change-pier-base-bridge-id id={} new-bridge-id={}", enc_json(&p.id), enc_json(&p.new_bridge_id))
}

/// 📖 Parses the arguments of `change-pier-base-bridge-id`.
pub fn parse_change_pier_base_bridge_id(args: &Args) -> Result<ChangePierBaseBridgeId, String> {
    let arg = |k: &str| required(args, "change-pier-base-bridge-id", k);
    Ok(ChangePierBaseBridgeId { id: dec_json(&arg("id")?)?, new_bridge_id: dec_json(&arg("new-bridge-id")?)? })
}

/// ✍️ Prints `change-pier-base-pier-id` as an op-text line.
pub fn print_change_pier_base_pier_id(p: &ChangePierBasePierId) -> String {
    format!("change-pier-base-pier-id id={} new-pier-id={}", enc_json(&p.id), enc_json(&p.new_pier_id))
}

/// 📖 Parses the arguments of `change-pier-base-pier-id`.
pub fn parse_change_pier_base_pier_id(args: &Args) -> Result<ChangePierBasePierId, String> {
    let arg = |k: &str| required(args, "change-pier-base-pier-id", k);
    Ok(ChangePierBasePierId { id: dec_json(&arg("id")?)?, new_pier_id: dec_json(&arg("new-pier-id")?)? })
}

/// ✍️ Prints `change-pier-base-radius-m` as an op-text line.
pub fn print_change_pier_base_radius_m(p: &ChangePierBaseRadiusM) -> String {
    format!("change-pier-base-radius-m id={} new-radius-m={}", enc_json(&p.id), enc_json(&p.new_radius_m))
}

/// 📖 Parses the arguments of `change-pier-base-radius-m`.
pub fn parse_change_pier_base_radius_m(args: &Args) -> Result<ChangePierBaseRadiusM, String> {
    let arg = |k: &str| required(args, "change-pier-base-radius-m", k);
    Ok(ChangePierBaseRadiusM { id: dec_json(&arg("id")?)?, new_radius_m: dec_json(&arg("new-radius-m")?)? })
}

/// ✍️ Prints `change-pier-base-soil-vs-m-s` as an op-text line.
pub fn print_change_pier_base_soil_vs_m_s(p: &ChangePierBaseSoilVsMS) -> String {
    format!("change-pier-base-soil-vs-m-s id={} new-soil-vs-m-s={}", enc_json(&p.id), enc_json(&p.new_soil_vs_m_s))
}

/// 📖 Parses the arguments of `change-pier-base-soil-vs-m-s`.
pub fn parse_change_pier_base_soil_vs_m_s(args: &Args) -> Result<ChangePierBaseSoilVsMS, String> {
    let arg = |k: &str| required(args, "change-pier-base-soil-vs-m-s", k);
    Ok(ChangePierBaseSoilVsMS { id: dec_json(&arg("id")?)?, new_soil_vs_m_s: dec_json(&arg("new-soil-vs-m-s")?)? })
}

/// ✍️ Prints `change-pier-base-soil-unit-weight-n-m3` as an op-text line.
pub fn print_change_pier_base_soil_unit_weight_n_m3(p: &ChangePierBaseSoilUnitWeightNM3) -> String {
    format!("change-pier-base-soil-unit-weight-n-m3 id={} new-soil-unit-weight-n-m3={}", enc_json(&p.id), enc_json(&p.new_soil_unit_weight_n_m3))
}

/// 📖 Parses the arguments of `change-pier-base-soil-unit-weight-n-m3`.
pub fn parse_change_pier_base_soil_unit_weight_n_m3(args: &Args) -> Result<ChangePierBaseSoilUnitWeightNM3, String> {
    let arg = |k: &str| required(args, "change-pier-base-soil-unit-weight-n-m3", k);
    Ok(ChangePierBaseSoilUnitWeightNM3 { id: dec_json(&arg("id")?)?, new_soil_unit_weight_n_m3: dec_json(&arg("new-soil-unit-weight-n-m3")?)? })
}

/// ✍️ Prints `change-pier-base-poisson-ratio` as an op-text line.
pub fn print_change_pier_base_poisson_ratio(p: &ChangePierBasePoissonRatio) -> String {
    format!("change-pier-base-poisson-ratio id={} new-poisson-ratio={}", enc_json(&p.id), enc_json(&p.new_poisson_ratio))
}

/// 📖 Parses the arguments of `change-pier-base-poisson-ratio`.
pub fn parse_change_pier_base_poisson_ratio(args: &Args) -> Result<ChangePierBasePoissonRatio, String> {
    let arg = |k: &str| required(args, "change-pier-base-poisson-ratio", k);
    Ok(ChangePierBasePoissonRatio { id: dec_json(&arg("id")?)?, new_poisson_ratio: dec_json(&arg("new-poisson-ratio")?)? })
}

/// ✍️ Prints `insert-critical-region` as an op-text line.
pub fn print_insert_critical_region(p: &InsertCriticalRegion) -> String {
    format!("insert-critical-region index={} critical_region={}", enc_json(&p.index), enc_json(&p.critical_region))
}

/// 📖 Parses the arguments of `insert-critical-region`.
pub fn parse_insert_critical_region(args: &Args) -> Result<InsertCriticalRegion, String> {
    let arg = |k: &str| required(args, "insert-critical-region", k);
    Ok(InsertCriticalRegion { index: opt_index(args)?, critical_region: dec_json(&arg("critical_region")?)? })
}

/// ✍️ Prints `remove-critical-region` as an op-text line.
pub fn print_remove_critical_region(p: &RemoveCriticalRegion) -> String {
    format!("remove-critical-region id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-critical-region`.
pub fn parse_remove_critical_region(args: &Args) -> Result<RemoveCriticalRegion, String> {
    let arg = |k: &str| required(args, "remove-critical-region", k);
    Ok(RemoveCriticalRegion { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-critical-region-building-id` as an op-text line.
pub fn print_change_critical_region_building_id(p: &ChangeCriticalRegionBuildingId) -> String {
    format!("change-critical-region-building-id id={} new-building-id={}", enc_json(&p.id), enc_json(&p.new_building_id))
}

/// 📖 Parses the arguments of `change-critical-region-building-id`.
pub fn parse_change_critical_region_building_id(args: &Args) -> Result<ChangeCriticalRegionBuildingId, String> {
    let arg = |k: &str| required(args, "change-critical-region-building-id", k);
    Ok(ChangeCriticalRegionBuildingId { id: dec_json(&arg("id")?)?, new_building_id: dec_json(&arg("new-building-id")?)? })
}

/// ✍️ Prints `change-critical-region-member-id` as an op-text line.
pub fn print_change_critical_region_member_id(p: &ChangeCriticalRegionMemberId) -> String {
    format!("change-critical-region-member-id id={} new-member-id={}", enc_json(&p.id), enc_json(&p.new_member_id))
}

/// 📖 Parses the arguments of `change-critical-region-member-id`.
pub fn parse_change_critical_region_member_id(args: &Args) -> Result<ChangeCriticalRegionMemberId, String> {
    let arg = |k: &str| required(args, "change-critical-region-member-id", k);
    Ok(ChangeCriticalRegionMemberId { id: dec_json(&arg("id")?)?, new_member_id: dec_json(&arg("new-member-id")?)? })
}

/// ✍️ Prints `change-critical-region-section-width-m` as an op-text line.
pub fn print_change_critical_region_section_width_m(p: &ChangeCriticalRegionSectionWidthM) -> String {
    format!("change-critical-region-section-width-m id={} new-section-width-m={}", enc_json(&p.id), enc_json(&p.new_section_width_m))
}

/// 📖 Parses the arguments of `change-critical-region-section-width-m`.
pub fn parse_change_critical_region_section_width_m(args: &Args) -> Result<ChangeCriticalRegionSectionWidthM, String> {
    let arg = |k: &str| required(args, "change-critical-region-section-width-m", k);
    Ok(ChangeCriticalRegionSectionWidthM { id: dec_json(&arg("id")?)?, new_section_width_m: dec_json(&arg("new-section-width-m")?)? })
}

/// ✍️ Prints `change-critical-region-section-depth-m` as an op-text line.
pub fn print_change_critical_region_section_depth_m(p: &ChangeCriticalRegionSectionDepthM) -> String {
    format!("change-critical-region-section-depth-m id={} new-section-depth-m={}", enc_json(&p.id), enc_json(&p.new_section_depth_m))
}

/// 📖 Parses the arguments of `change-critical-region-section-depth-m`.
pub fn parse_change_critical_region_section_depth_m(args: &Args) -> Result<ChangeCriticalRegionSectionDepthM, String> {
    let arg = |k: &str| required(args, "change-critical-region-section-depth-m", k);
    Ok(ChangeCriticalRegionSectionDepthM { id: dec_json(&arg("id")?)?, new_section_depth_m: dec_json(&arg("new-section-depth-m")?)? })
}

/// ✍️ Prints `change-critical-region-clear-length-m` as an op-text line.
pub fn print_change_critical_region_clear_length_m(p: &ChangeCriticalRegionClearLengthM) -> String {
    format!("change-critical-region-clear-length-m id={} new-clear-length-m={}", enc_json(&p.id), enc_json(&p.new_clear_length_m))
}

/// 📖 Parses the arguments of `change-critical-region-clear-length-m`.
pub fn parse_change_critical_region_clear_length_m(args: &Args) -> Result<ChangeCriticalRegionClearLengthM, String> {
    let arg = |k: &str| required(args, "change-critical-region-clear-length-m", k);
    Ok(ChangeCriticalRegionClearLengthM { id: dec_json(&arg("id")?)?, new_clear_length_m: dec_json(&arg("new-clear-length-m")?)? })
}

/// ✍️ Prints `change-critical-region-hooped-length-m` as an op-text line.
pub fn print_change_critical_region_hooped_length_m(p: &ChangeCriticalRegionHoopedLengthM) -> String {
    format!("change-critical-region-hooped-length-m id={} new-hooped-length-m={}", enc_json(&p.id), enc_json(&p.new_hooped_length_m))
}

/// 📖 Parses the arguments of `change-critical-region-hooped-length-m`.
pub fn parse_change_critical_region_hooped_length_m(args: &Args) -> Result<ChangeCriticalRegionHoopedLengthM, String> {
    let arg = |k: &str| required(args, "change-critical-region-hooped-length-m", k);
    Ok(ChangeCriticalRegionHoopedLengthM { id: dec_json(&arg("id")?)?, new_hooped_length_m: dec_json(&arg("new-hooped-length-m")?)? })
}

/// ✍️ Prints `change-critical-region-core-width-m` as an op-text line.
pub fn print_change_critical_region_core_width_m(p: &ChangeCriticalRegionCoreWidthM) -> String {
    format!("change-critical-region-core-width-m id={} new-core-width-m={}", enc_json(&p.id), enc_json(&p.new_core_width_m))
}

/// 📖 Parses the arguments of `change-critical-region-core-width-m`.
pub fn parse_change_critical_region_core_width_m(args: &Args) -> Result<ChangeCriticalRegionCoreWidthM, String> {
    let arg = |k: &str| required(args, "change-critical-region-core-width-m", k);
    Ok(ChangeCriticalRegionCoreWidthM { id: dec_json(&arg("id")?)?, new_core_width_m: dec_json(&arg("new-core-width-m")?)? })
}

/// ✍️ Prints `change-critical-region-core-depth-m` as an op-text line.
pub fn print_change_critical_region_core_depth_m(p: &ChangeCriticalRegionCoreDepthM) -> String {
    format!("change-critical-region-core-depth-m id={} new-core-depth-m={}", enc_json(&p.id), enc_json(&p.new_core_depth_m))
}

/// 📖 Parses the arguments of `change-critical-region-core-depth-m`.
pub fn parse_change_critical_region_core_depth_m(args: &Args) -> Result<ChangeCriticalRegionCoreDepthM, String> {
    let arg = |k: &str| required(args, "change-critical-region-core-depth-m", k);
    Ok(ChangeCriticalRegionCoreDepthM { id: dec_json(&arg("id")?)?, new_core_depth_m: dec_json(&arg("new-core-depth-m")?)? })
}

/// ✍️ Prints `change-critical-region-axial-force-n` as an op-text line.
pub fn print_change_critical_region_axial_force_n(p: &ChangeCriticalRegionAxialForceN) -> String {
    format!("change-critical-region-axial-force-n id={} new-axial-force-n={}", enc_json(&p.id), enc_json(&p.new_axial_force_n))
}

/// 📖 Parses the arguments of `change-critical-region-axial-force-n`.
pub fn parse_change_critical_region_axial_force_n(args: &Args) -> Result<ChangeCriticalRegionAxialForceN, String> {
    let arg = |k: &str| required(args, "change-critical-region-axial-force-n", k);
    Ok(ChangeCriticalRegionAxialForceN { id: dec_json(&arg("id")?)?, new_axial_force_n: dec_json(&arg("new-axial-force-n")?)? })
}

/// ✍️ Prints `change-critical-region-concrete-fcd` as an op-text line.
pub fn print_change_critical_region_concrete_fcd(p: &ChangeCriticalRegionConcreteDesignStrengthMpa) -> String {
    format!("change-critical-region-concrete-fcd id={} new-concrete-design-strength-mpa={}", enc_json(&p.id), enc_json(&p.new_concrete_design_strength_mpa))
}

/// 📖 Parses the arguments of `change-critical-region-concrete-fcd`.
pub fn parse_change_critical_region_concrete_fcd(args: &Args) -> Result<ChangeCriticalRegionConcreteDesignStrengthMpa, String> {
    let arg = |k: &str| required(args, "change-critical-region-concrete-fcd", k);
    Ok(ChangeCriticalRegionConcreteDesignStrengthMpa { id: dec_json(&arg("id")?)?, new_concrete_design_strength_mpa: dec_json(&arg("new-concrete-design-strength-mpa")?)? })
}

/// ✍️ Prints `change-critical-region-steel-fyd` as an op-text line.
pub fn print_change_critical_region_steel_fyd(p: &ChangeCriticalRegionSteelDesignYieldMpa) -> String {
    format!("change-critical-region-steel-fyd id={} new-steel-design-yield-mpa={}", enc_json(&p.id), enc_json(&p.new_steel_design_yield_mpa))
}

/// 📖 Parses the arguments of `change-critical-region-steel-fyd`.
pub fn parse_change_critical_region_steel_fyd(args: &Args) -> Result<ChangeCriticalRegionSteelDesignYieldMpa, String> {
    let arg = |k: &str| required(args, "change-critical-region-steel-fyd", k);
    Ok(ChangeCriticalRegionSteelDesignYieldMpa { id: dec_json(&arg("id")?)?, new_steel_design_yield_mpa: dec_json(&arg("new-steel-design-yield-mpa")?)? })
}

/// ✍️ Prints `change-critical-region-hoop-fywd` as an op-text line.
pub fn print_change_critical_region_hoop_fywd(p: &ChangeCriticalRegionHoopDesignYieldMpa) -> String {
    format!("change-critical-region-hoop-fywd id={} new-hoop-design-yield-mpa={}", enc_json(&p.id), enc_json(&p.new_hoop_design_yield_mpa))
}

/// 📖 Parses the arguments of `change-critical-region-hoop-fywd`.
pub fn parse_change_critical_region_hoop_fywd(args: &Args) -> Result<ChangeCriticalRegionHoopDesignYieldMpa, String> {
    let arg = |k: &str| required(args, "change-critical-region-hoop-fywd", k);
    Ok(ChangeCriticalRegionHoopDesignYieldMpa { id: dec_json(&arg("id")?)?, new_hoop_design_yield_mpa: dec_json(&arg("new-hoop-design-yield-mpa")?)? })
}

/// ✍️ Prints `change-critical-region-long-steel-area` as an op-text line.
pub fn print_change_critical_region_long_steel_area(p: &ChangeCriticalRegionLongitudinalSteelAreaM2) -> String {
    format!("change-critical-region-long-steel-area id={} new-longitudinal-steel-area-m2={}", enc_json(&p.id), enc_json(&p.new_longitudinal_steel_area_m2))
}

/// 📖 Parses the arguments of `change-critical-region-long-steel-area`.
pub fn parse_change_critical_region_long_steel_area(args: &Args) -> Result<ChangeCriticalRegionLongitudinalSteelAreaM2, String> {
    let arg = |k: &str| required(args, "change-critical-region-long-steel-area", k);
    Ok(ChangeCriticalRegionLongitudinalSteelAreaM2 { id: dec_json(&arg("id")?)?, new_longitudinal_steel_area_m2: dec_json(&arg("new-longitudinal-steel-area-m2")?)? })
}

/// ✍️ Prints `change-critical-region-long-bar-dia` as an op-text line.
pub fn print_change_critical_region_long_bar_dia(p: &ChangeCriticalRegionLongitudinalBarDiameterM) -> String {
    format!("change-critical-region-long-bar-dia id={} new-longitudinal-bar-diameter-m={}", enc_json(&p.id), enc_json(&p.new_longitudinal_bar_diameter_m))
}

/// 📖 Parses the arguments of `change-critical-region-long-bar-dia`.
pub fn parse_change_critical_region_long_bar_dia(args: &Args) -> Result<ChangeCriticalRegionLongitudinalBarDiameterM, String> {
    let arg = |k: &str| required(args, "change-critical-region-long-bar-dia", k);
    Ok(ChangeCriticalRegionLongitudinalBarDiameterM { id: dec_json(&arg("id")?)?, new_longitudinal_bar_diameter_m: dec_json(&arg("new-longitudinal-bar-diameter-m")?)? })
}

/// ✍️ Prints `change-critical-region-long-bar-max` as an op-text line.
pub fn print_change_critical_region_long_bar_max(p: &ChangeCriticalRegionLongitudinalBarMaxDiameterM) -> String {
    format!("change-critical-region-long-bar-max id={} new-longitudinal-bar-max-diameter-m={}", enc_json(&p.id), enc_json(&p.new_longitudinal_bar_max_diameter_m))
}

/// 📖 Parses the arguments of `change-critical-region-long-bar-max`.
pub fn parse_change_critical_region_long_bar_max(args: &Args) -> Result<ChangeCriticalRegionLongitudinalBarMaxDiameterM, String> {
    let arg = |k: &str| required(args, "change-critical-region-long-bar-max", k);
    Ok(ChangeCriticalRegionLongitudinalBarMaxDiameterM { id: dec_json(&arg("id")?)?, new_longitudinal_bar_max_diameter_m: dec_json(&arg("new-longitudinal-bar-max-diameter-m")?)? })
}

/// ✍️ Prints `change-critical-region-hoop-diameter-m` as an op-text line.
pub fn print_change_critical_region_hoop_diameter_m(p: &ChangeCriticalRegionHoopDiameterM) -> String {
    format!("change-critical-region-hoop-diameter-m id={} new-hoop-diameter-m={}", enc_json(&p.id), enc_json(&p.new_hoop_diameter_m))
}

/// 📖 Parses the arguments of `change-critical-region-hoop-diameter-m`.
pub fn parse_change_critical_region_hoop_diameter_m(args: &Args) -> Result<ChangeCriticalRegionHoopDiameterM, String> {
    let arg = |k: &str| required(args, "change-critical-region-hoop-diameter-m", k);
    Ok(ChangeCriticalRegionHoopDiameterM { id: dec_json(&arg("id")?)?, new_hoop_diameter_m: dec_json(&arg("new-hoop-diameter-m")?)? })
}

/// ✍️ Prints `change-critical-region-hoop-spacing-m` as an op-text line.
pub fn print_change_critical_region_hoop_spacing_m(p: &ChangeCriticalRegionHoopSpacingM) -> String {
    format!("change-critical-region-hoop-spacing-m id={} new-hoop-spacing-m={}", enc_json(&p.id), enc_json(&p.new_hoop_spacing_m))
}

/// 📖 Parses the arguments of `change-critical-region-hoop-spacing-m`.
pub fn parse_change_critical_region_hoop_spacing_m(args: &Args) -> Result<ChangeCriticalRegionHoopSpacingM, String> {
    let arg = |k: &str| required(args, "change-critical-region-hoop-spacing-m", k);
    Ok(ChangeCriticalRegionHoopSpacingM { id: dec_json(&arg("id")?)?, new_hoop_spacing_m: dec_json(&arg("new-hoop-spacing-m")?)? })
}

/// ✍️ Prints `change-critical-region-hoop-legs-x` as an op-text line.
pub fn print_change_critical_region_hoop_legs_x(p: &ChangeCriticalRegionHoopLegsX) -> String {
    format!("change-critical-region-hoop-legs-x id={} new-hoop-legs-x={}", enc_json(&p.id), enc_json(&p.new_hoop_legs_x))
}

/// 📖 Parses the arguments of `change-critical-region-hoop-legs-x`.
pub fn parse_change_critical_region_hoop_legs_x(args: &Args) -> Result<ChangeCriticalRegionHoopLegsX, String> {
    let arg = |k: &str| required(args, "change-critical-region-hoop-legs-x", k);
    Ok(ChangeCriticalRegionHoopLegsX { id: dec_json(&arg("id")?)?, new_hoop_legs_x: dec_json(&arg("new-hoop-legs-x")?)? })
}

/// ✍️ Prints `change-critical-region-hoop-legs-y` as an op-text line.
pub fn print_change_critical_region_hoop_legs_y(p: &ChangeCriticalRegionHoopLegsY) -> String {
    format!("change-critical-region-hoop-legs-y id={} new-hoop-legs-y={}", enc_json(&p.id), enc_json(&p.new_hoop_legs_y))
}

/// 📖 Parses the arguments of `change-critical-region-hoop-legs-y`.
pub fn parse_change_critical_region_hoop_legs_y(args: &Args) -> Result<ChangeCriticalRegionHoopLegsY, String> {
    let arg = |k: &str| required(args, "change-critical-region-hoop-legs-y", k);
    Ok(ChangeCriticalRegionHoopLegsY { id: dec_json(&arg("id")?)?, new_hoop_legs_y: dec_json(&arg("new-hoop-legs-y")?)? })
}

/// ✍️ Prints `change-critical-region-restr-bar-count` as an op-text line.
pub fn print_change_critical_region_restr_bar_count(p: &ChangeCriticalRegionRestrainedBarCount) -> String {
    format!("change-critical-region-restr-bar-count id={} new-restrained-bar-count={}", enc_json(&p.id), enc_json(&p.new_restrained_bar_count))
}

/// 📖 Parses the arguments of `change-critical-region-restr-bar-count`.
pub fn parse_change_critical_region_restr_bar_count(args: &Args) -> Result<ChangeCriticalRegionRestrainedBarCount, String> {
    let arg = |k: &str| required(args, "change-critical-region-restr-bar-count", k);
    Ok(ChangeCriticalRegionRestrainedBarCount { id: dec_json(&arg("id")?)?, new_restrained_bar_count: dec_json(&arg("new-restrained-bar-count")?)? })
}

/// ✍️ Prints `change-critical-region-restr-bar-space` as an op-text line.
pub fn print_change_critical_region_restr_bar_space(p: &ChangeCriticalRegionRestrainedBarSpacingM) -> String {
    format!("change-critical-region-restr-bar-space id={} new-restrained-bar-spacing-m={}", enc_json(&p.id), enc_json(&p.new_restrained_bar_spacing_m))
}

/// 📖 Parses the arguments of `change-critical-region-restr-bar-space`.
pub fn parse_change_critical_region_restr_bar_space(args: &Args) -> Result<ChangeCriticalRegionRestrainedBarSpacingM, String> {
    let arg = |k: &str| required(args, "change-critical-region-restr-bar-space", k);
    Ok(ChangeCriticalRegionRestrainedBarSpacingM { id: dec_json(&arg("id")?)?, new_restrained_bar_spacing_m: dec_json(&arg("new-restrained-bar-spacing-m")?)? })
}
