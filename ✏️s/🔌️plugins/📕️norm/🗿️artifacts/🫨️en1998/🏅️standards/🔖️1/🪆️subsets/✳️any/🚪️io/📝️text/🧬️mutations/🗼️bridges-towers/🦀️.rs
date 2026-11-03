//! 🗼️ EN 1998 bridges towers mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_bridges_towers::insert_bridge::InsertBridge;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_v_rd_n::ChangeBridgeVRdN;
use semio_s_artifact_norm_en1998_mut_bridges_towers::insert_tower::InsertTower;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_m_rd_nm::ChangeTowerMRdNm;
use semio_s_artifact_norm_en1998_mut_bridges_towers::remove_bridge::RemoveBridge;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_period_ratio::ChangeBridgePeriodRatio;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_fundamental_period_s::ChangeBridgeFundamentalPeriodS;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_bearing_d_rd_m::ChangeBridgeBearingDRdM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_permanent_gk_n::ChangeBridgePermanentGkN;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_correlated_occupancy::ChangeBridgeCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_height_m::ChangeTowerHeightM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_is_chimney::ChangeTowerIsChimney;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_q_nominal::ChangeTowerQNominal;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_permanent_gk_n::ChangeTowerPermanentGkN;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_correlated_occupancy::ChangeTowerCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_pier_height_m::ChangeBridgePierHeightM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_pier_moment_rd_nm::ChangeBridgePierMomentRdNm;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_pier_shear_rd_n::ChangeBridgePierShearRdN;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_importance_class::ChangeBridgeImportanceClass;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_vertical_period_s::ChangeBridgeVerticalPeriodS;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_pier_cracked_inertia_ratio::ChangeBridgePierCrackedInertiaRatio;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_deck_length_m::ChangeBridgeDeckLengthM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_deck_ei_nm2::ChangeBridgeDeckEiNm2;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_ei_nm2::ChangeTowerEiNm2;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_variable_category::ChangeBridgeVariableCategory;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_variable_qk_n::ChangeBridgeVariableQkN;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_variable_category::ChangeTowerVariableCategory;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_variable_qk_n::ChangeTowerVariableQkN;
use semio_s_artifact_norm_en1998_mut_bridges_towers::insert_bridge_variable::InsertBridgeVariable;
use semio_s_artifact_norm_en1998_mut_bridges_towers::remove_bridge_variable::RemoveBridgeVariable;
use semio_s_artifact_norm_en1998_mut_bridges_towers::insert_tower_variable::InsertTowerVariable;
use semio_s_artifact_norm_en1998_mut_bridges_towers::remove_tower_variable::RemoveTowerVariable;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_base_rot_stiff_nm::ChangeTowerBaseRotStiffNm;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_bridge_id::ChangePierBridgeId;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_stiffness_basis::ChangePierStiffnessBasis;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_x_m::ChangePierXM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_height_m::ChangePierHeightM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_section_b_m::ChangePierSectionBM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_section_h_m::ChangePierSectionHM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_modulus_mpa::ChangePierModulusMpa;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_moment_rd_nm::ChangePierMomentRdNm;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_shear_rd_n::ChangePierShearRdN;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_reinforcement_ratio::ChangePierReinforcementRatio;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_cover_m::ChangePierCoverM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_concrete_fck_mpa::ChangePierConcreteFckMpa;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_steel_fyk_mpa::ChangePierSteelFykMpa;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_pier_axial_load_n::ChangePierAxialLoadN;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bearing_bridge_id::ChangeBearingBridgeId;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bearing_pier_id::ChangeBearingPierId;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bearing_device::ChangeBearingDevice;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bearing_x_m::ChangeBearingXM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bearing_stiffness_n_per_m::ChangeBearingStiffnessNPerM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bearing_d_rd_m::ChangeBearingDRdM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bearing_seat_length_m::ChangeBearingSeatLengthM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bearing_min_support_m::ChangeBearingMinSupportM;
use semio_s_artifact_norm_en1998_mut_bridges_towers::insert_pier::InsertPier;
use semio_s_artifact_norm_en1998_mut_bridges_towers::remove_pier::RemovePier;
use semio_s_artifact_norm_en1998_mut_bridges_towers::insert_bearing::InsertBearing;
use semio_s_artifact_norm_en1998_mut_bridges_towers::remove_bearing::RemoveBearing;
use semio_s_artifact_norm_en1998_mut_bridges_towers::remove_tower::RemoveTower;

/// ✍️ Prints `insert-bridge` as an op-text line.
pub fn print_insert_bridge(p: &InsertBridge) -> String {
    format!("insert-bridge index={} bridge={}", enc_json(&p.index), enc_json(&p.bridge))
}

/// 📖 Parses the arguments of `insert-bridge`.
pub fn parse_insert_bridge(args: &Args) -> Result<InsertBridge, String> {
    let arg = |k: &str| required(args, "insert-bridge", k);
    Ok(InsertBridge { index: opt_index(args)?, bridge: dec_json(&arg("bridge")?)? })
}

/// ✍️ Prints `change-bridge-v-rd-n` as an op-text line.
pub fn print_change_bridge_v_rd_n(p: &ChangeBridgeVRdN) -> String {
    format!("change-bridge-v-rd-n id={} new-v-rd-n={}", enc_json(&p.id), enc_json(&p.new_v_rd_n))
}

/// 📖 Parses the arguments of `change-bridge-v-rd-n`.
pub fn parse_change_bridge_v_rd_n(args: &Args) -> Result<ChangeBridgeVRdN, String> {
    let arg = |k: &str| required(args, "change-bridge-v-rd-n", k);
    Ok(ChangeBridgeVRdN { id: dec_json(&arg("id")?)?, new_v_rd_n: dec_json(&arg("new-v-rd-n")?)? })
}

/// ✍️ Prints `insert-tower` as an op-text line.
pub fn print_insert_tower(p: &InsertTower) -> String {
    format!("insert-tower index={} tower={}", enc_json(&p.index), enc_json(&p.tower))
}

/// 📖 Parses the arguments of `insert-tower`.
pub fn parse_insert_tower(args: &Args) -> Result<InsertTower, String> {
    let arg = |k: &str| required(args, "insert-tower", k);
    Ok(InsertTower { index: opt_index(args)?, tower: dec_json(&arg("tower")?)? })
}

/// ✍️ Prints `change-tower-m-rd-nm` as an op-text line.
pub fn print_change_tower_m_rd_nm(p: &ChangeTowerMRdNm) -> String {
    format!("change-tower-m-rd-nm id={} new-m-rd-nm={}", enc_json(&p.id), enc_json(&p.new_m_rd_nm))
}

/// 📖 Parses the arguments of `change-tower-m-rd-nm`.
pub fn parse_change_tower_m_rd_nm(args: &Args) -> Result<ChangeTowerMRdNm, String> {
    let arg = |k: &str| required(args, "change-tower-m-rd-nm", k);
    Ok(ChangeTowerMRdNm { id: dec_json(&arg("id")?)?, new_m_rd_nm: dec_json(&arg("new-m-rd-nm")?)? })
}

/// ✍️ Prints `remove-bridge` as an op-text line.
pub fn print_remove_bridge(p: &RemoveBridge) -> String {
    format!("remove-bridge id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-bridge`.
pub fn parse_remove_bridge(args: &Args) -> Result<RemoveBridge, String> {
    let arg = |k: &str| required(args, "remove-bridge", k);
    Ok(RemoveBridge { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-bridge-period-ratio` as an op-text line.
pub fn print_change_bridge_period_ratio(p: &ChangeBridgePeriodRatio) -> String {
    format!("change-bridge-period-ratio id={} new-period-ratio={}", enc_json(&p.id), enc_json(&p.new_period_ratio))
}

/// 📖 Parses the arguments of `change-bridge-period-ratio`.
pub fn parse_change_bridge_period_ratio(args: &Args) -> Result<ChangeBridgePeriodRatio, String> {
    let arg = |k: &str| required(args, "change-bridge-period-ratio", k);
    Ok(ChangeBridgePeriodRatio { id: dec_json(&arg("id")?)?, new_period_ratio: dec_json(&arg("new-period-ratio")?)? })
}

/// ✍️ Prints `change-bridge-fundamental-period-s` as an op-text line.
pub fn print_change_bridge_fundamental_period_s(p: &ChangeBridgeFundamentalPeriodS) -> String {
    format!("change-bridge-fundamental-period-s id={} new-fundamental-period-s={}", enc_json(&p.id), enc_json(&p.new_fundamental_period_s))
}

/// 📖 Parses the arguments of `change-bridge-fundamental-period-s`.
pub fn parse_change_bridge_fundamental_period_s(args: &Args) -> Result<ChangeBridgeFundamentalPeriodS, String> {
    let arg = |k: &str| required(args, "change-bridge-fundamental-period-s", k);
    Ok(ChangeBridgeFundamentalPeriodS { id: dec_json(&arg("id")?)?, new_fundamental_period_s: dec_json(&arg("new-fundamental-period-s")?)? })
}

/// ✍️ Prints `change-bridge-bearing-d-rd-m` as an op-text line.
pub fn print_change_bridge_bearing_d_rd_m(p: &ChangeBridgeBearingDRdM) -> String {
    format!("change-bridge-bearing-d-rd-m id={} new-bearing-d-rd-m={}", enc_json(&p.id), enc_json(&p.new_bearing_d_rd_m))
}

/// 📖 Parses the arguments of `change-bridge-bearing-d-rd-m`.
pub fn parse_change_bridge_bearing_d_rd_m(args: &Args) -> Result<ChangeBridgeBearingDRdM, String> {
    let arg = |k: &str| required(args, "change-bridge-bearing-d-rd-m", k);
    Ok(ChangeBridgeBearingDRdM { id: dec_json(&arg("id")?)?, new_bearing_d_rd_m: dec_json(&arg("new-bearing-d-rd-m")?)? })
}

/// ✍️ Prints `change-bridge-permanent-gk-n` as an op-text line.
pub fn print_change_bridge_permanent_gk_n(p: &ChangeBridgePermanentGkN) -> String {
    format!("change-bridge-permanent-gk-n id={} new-permanent-gk-n={}", enc_json(&p.id), enc_json(&p.new_permanent_gk_n))
}

/// 📖 Parses the arguments of `change-bridge-permanent-gk-n`.
pub fn parse_change_bridge_permanent_gk_n(args: &Args) -> Result<ChangeBridgePermanentGkN, String> {
    let arg = |k: &str| required(args, "change-bridge-permanent-gk-n", k);
    Ok(ChangeBridgePermanentGkN { id: dec_json(&arg("id")?)?, new_permanent_gk_n: dec_json(&arg("new-permanent-gk-n")?)? })
}

/// ✍️ Prints `change-bridge-correlated-occupancy` as an op-text line.
pub fn print_change_bridge_correlated_occupancy(p: &ChangeBridgeCorrelatedOccupancy) -> String {
    format!("change-bridge-correlated-occupancy id={} new-correlated-occupancy={}", enc_json(&p.id), enc_json(&p.new_correlated_occupancy))
}

/// 📖 Parses the arguments of `change-bridge-correlated-occupancy`.
pub fn parse_change_bridge_correlated_occupancy(args: &Args) -> Result<ChangeBridgeCorrelatedOccupancy, String> {
    let arg = |k: &str| required(args, "change-bridge-correlated-occupancy", k);
    Ok(ChangeBridgeCorrelatedOccupancy { id: dec_json(&arg("id")?)?, new_correlated_occupancy: dec_json(&arg("new-correlated-occupancy")?)? })
}

/// ✍️ Prints `change-tower-height-m` as an op-text line.
pub fn print_change_tower_height_m(p: &ChangeTowerHeightM) -> String {
    format!("change-tower-height-m id={} new-height-m={}", enc_json(&p.id), enc_json(&p.new_height_m))
}

/// 📖 Parses the arguments of `change-tower-height-m`.
pub fn parse_change_tower_height_m(args: &Args) -> Result<ChangeTowerHeightM, String> {
    let arg = |k: &str| required(args, "change-tower-height-m", k);
    Ok(ChangeTowerHeightM { id: dec_json(&arg("id")?)?, new_height_m: dec_json(&arg("new-height-m")?)? })
}

/// ✍️ Prints `change-tower-is-chimney` as an op-text line.
pub fn print_change_tower_is_chimney(p: &ChangeTowerIsChimney) -> String {
    format!("change-tower-is-chimney id={} new-is-chimney={}", enc_json(&p.id), enc_json(&p.new_is_chimney))
}

/// 📖 Parses the arguments of `change-tower-is-chimney`.
pub fn parse_change_tower_is_chimney(args: &Args) -> Result<ChangeTowerIsChimney, String> {
    let arg = |k: &str| required(args, "change-tower-is-chimney", k);
    Ok(ChangeTowerIsChimney { id: dec_json(&arg("id")?)?, new_is_chimney: dec_json(&arg("new-is-chimney")?)? })
}

/// ✍️ Prints `change-tower-q-nominal` as an op-text line.
pub fn print_change_tower_q_nominal(p: &ChangeTowerQNominal) -> String {
    format!("change-tower-q-nominal id={} new-q-nominal={}", enc_json(&p.id), enc_json(&p.new_q_nominal))
}

/// 📖 Parses the arguments of `change-tower-q-nominal`.
pub fn parse_change_tower_q_nominal(args: &Args) -> Result<ChangeTowerQNominal, String> {
    let arg = |k: &str| required(args, "change-tower-q-nominal", k);
    Ok(ChangeTowerQNominal { id: dec_json(&arg("id")?)?, new_q_nominal: dec_json(&arg("new-q-nominal")?)? })
}

/// ✍️ Prints `change-tower-permanent-gk-n` as an op-text line.
pub fn print_change_tower_permanent_gk_n(p: &ChangeTowerPermanentGkN) -> String {
    format!("change-tower-permanent-gk-n id={} new-permanent-gk-n={}", enc_json(&p.id), enc_json(&p.new_permanent_gk_n))
}

/// 📖 Parses the arguments of `change-tower-permanent-gk-n`.
pub fn parse_change_tower_permanent_gk_n(args: &Args) -> Result<ChangeTowerPermanentGkN, String> {
    let arg = |k: &str| required(args, "change-tower-permanent-gk-n", k);
    Ok(ChangeTowerPermanentGkN { id: dec_json(&arg("id")?)?, new_permanent_gk_n: dec_json(&arg("new-permanent-gk-n")?)? })
}

/// ✍️ Prints `change-tower-correlated-occupancy` as an op-text line.
pub fn print_change_tower_correlated_occupancy(p: &ChangeTowerCorrelatedOccupancy) -> String {
    format!("change-tower-correlated-occupancy id={} new-correlated-occupancy={}", enc_json(&p.id), enc_json(&p.new_correlated_occupancy))
}

/// 📖 Parses the arguments of `change-tower-correlated-occupancy`.
pub fn parse_change_tower_correlated_occupancy(args: &Args) -> Result<ChangeTowerCorrelatedOccupancy, String> {
    let arg = |k: &str| required(args, "change-tower-correlated-occupancy", k);
    Ok(ChangeTowerCorrelatedOccupancy { id: dec_json(&arg("id")?)?, new_correlated_occupancy: dec_json(&arg("new-correlated-occupancy")?)? })
}

/// ✍️ Prints `change-bridge-pier-height-m` as an op-text line.
pub fn print_change_bridge_pier_height_m(p: &ChangeBridgePierHeightM) -> String {
    format!("change-bridge-pier-height-m id={} new-pier-height-m={}", enc_json(&p.id), enc_json(&p.new_pier_height_m))
}

/// 📖 Parses the arguments of `change-bridge-pier-height-m`.
pub fn parse_change_bridge_pier_height_m(args: &Args) -> Result<ChangeBridgePierHeightM, String> {
    let arg = |k: &str| required(args, "change-bridge-pier-height-m", k);
    Ok(ChangeBridgePierHeightM { id: dec_json(&arg("id")?)?, new_pier_height_m: dec_json(&arg("new-pier-height-m")?)? })
}

/// ✍️ Prints `change-bridge-pier-moment-rd-nm` as an op-text line.
pub fn print_change_bridge_pier_moment_rd_nm(p: &ChangeBridgePierMomentRdNm) -> String {
    format!("change-bridge-pier-moment-rd-nm id={} new-pier-moment-rd-nm={}", enc_json(&p.id), enc_json(&p.new_pier_moment_rd_nm))
}

/// 📖 Parses the arguments of `change-bridge-pier-moment-rd-nm`.
pub fn parse_change_bridge_pier_moment_rd_nm(args: &Args) -> Result<ChangeBridgePierMomentRdNm, String> {
    let arg = |k: &str| required(args, "change-bridge-pier-moment-rd-nm", k);
    Ok(ChangeBridgePierMomentRdNm { id: dec_json(&arg("id")?)?, new_pier_moment_rd_nm: dec_json(&arg("new-pier-moment-rd-nm")?)? })
}

/// ✍️ Prints `change-bridge-pier-shear-rd-n` as an op-text line.
pub fn print_change_bridge_pier_shear_rd_n(p: &ChangeBridgePierShearRdN) -> String {
    format!("change-bridge-pier-shear-rd-n id={} new-pier-shear-rd-n={}", enc_json(&p.id), enc_json(&p.new_pier_shear_rd_n))
}

/// 📖 Parses the arguments of `change-bridge-pier-shear-rd-n`.
pub fn parse_change_bridge_pier_shear_rd_n(args: &Args) -> Result<ChangeBridgePierShearRdN, String> {
    let arg = |k: &str| required(args, "change-bridge-pier-shear-rd-n", k);
    Ok(ChangeBridgePierShearRdN { id: dec_json(&arg("id")?)?, new_pier_shear_rd_n: dec_json(&arg("new-pier-shear-rd-n")?)? })
}

/// ✍️ Prints `change-bridge-importance-class` as an op-text line.
pub fn print_change_bridge_importance_class(p: &ChangeBridgeImportanceClass) -> String {
    format!("change-bridge-importance-class id={} new-importance-class={}", enc_json(&p.id), enc_json(&p.new_importance_class))
}

/// 📖 Parses the arguments of `change-bridge-importance-class`.
pub fn parse_change_bridge_importance_class(args: &Args) -> Result<ChangeBridgeImportanceClass, String> {
    let arg = |k: &str| required(args, "change-bridge-importance-class", k);
    Ok(ChangeBridgeImportanceClass { id: dec_json(&arg("id")?)?, new_importance_class: dec_json(&arg("new-importance-class")?)? })
}

/// ✍️ Prints `change-bridge-vertical-period-s` as an op-text line.
pub fn print_change_bridge_vertical_period_s(p: &ChangeBridgeVerticalPeriodS) -> String {
    format!("change-bridge-vertical-period-s id={} new-vertical-period-s={}", enc_json(&p.id), enc_json(&p.new_vertical_period_s))
}

/// 📖 Parses the arguments of `change-bridge-vertical-period-s`.
pub fn parse_change_bridge_vertical_period_s(args: &Args) -> Result<ChangeBridgeVerticalPeriodS, String> {
    let arg = |k: &str| required(args, "change-bridge-vertical-period-s", k);
    Ok(ChangeBridgeVerticalPeriodS { id: dec_json(&arg("id")?)?, new_vertical_period_s: dec_json(&arg("new-vertical-period-s")?)? })
}

/// ✍️ Prints `change-bridge-pier-cracked-inertia-ratio` as an op-text line.
pub fn print_change_bridge_pier_cracked_inertia_ratio(p: &ChangeBridgePierCrackedInertiaRatio) -> String {
    format!("change-bridge-pier-cracked-inertia-ratio id={} new-pier-cracked-inertia-ratio={}", enc_json(&p.id), enc_json(&p.new_pier_cracked_inertia_ratio))
}

/// 📖 Parses the arguments of `change-bridge-pier-cracked-inertia-ratio`.
pub fn parse_change_bridge_pier_cracked_inertia_ratio(args: &Args) -> Result<ChangeBridgePierCrackedInertiaRatio, String> {
    let arg = |k: &str| required(args, "change-bridge-pier-cracked-inertia-ratio", k);
    Ok(ChangeBridgePierCrackedInertiaRatio { id: dec_json(&arg("id")?)?, new_pier_cracked_inertia_ratio: dec_json(&arg("new-pier-cracked-inertia-ratio")?)? })
}

/// ✍️ Prints `change-bridge-deck-length-m` as an op-text line.
pub fn print_change_bridge_deck_length_m(p: &ChangeBridgeDeckLengthM) -> String {
    format!("change-bridge-deck-length-m id={} new-deck-length-m={}", enc_json(&p.id), enc_json(&p.new_deck_length_m))
}

/// 📖 Parses the arguments of `change-bridge-deck-length-m`.
pub fn parse_change_bridge_deck_length_m(args: &Args) -> Result<ChangeBridgeDeckLengthM, String> {
    let arg = |k: &str| required(args, "change-bridge-deck-length-m", k);
    Ok(ChangeBridgeDeckLengthM { id: dec_json(&arg("id")?)?, new_deck_length_m: dec_json(&arg("new-deck-length-m")?)? })
}

/// ✍️ Prints `change-bridge-deck-ei-nm2` as an op-text line.
pub fn print_change_bridge_deck_ei_nm2(p: &ChangeBridgeDeckEiNm2) -> String {
    format!("change-bridge-deck-ei-nm2 id={} new-deck-ei-nm2={}", enc_json(&p.id), enc_json(&p.new_deck_ei_nm2))
}

/// 📖 Parses the arguments of `change-bridge-deck-ei-nm2`.
pub fn parse_change_bridge_deck_ei_nm2(args: &Args) -> Result<ChangeBridgeDeckEiNm2, String> {
    let arg = |k: &str| required(args, "change-bridge-deck-ei-nm2", k);
    Ok(ChangeBridgeDeckEiNm2 { id: dec_json(&arg("id")?)?, new_deck_ei_nm2: dec_json(&arg("new-deck-ei-nm2")?)? })
}

/// ✍️ Prints `change-tower-ei-nm2` as an op-text line.
pub fn print_change_tower_ei_nm2(p: &ChangeTowerEiNm2) -> String {
    format!("change-tower-ei-nm2 id={} new-ei-nm2={}", enc_json(&p.id), enc_json(&p.new_ei_nm2))
}

/// 📖 Parses the arguments of `change-tower-ei-nm2`.
pub fn parse_change_tower_ei_nm2(args: &Args) -> Result<ChangeTowerEiNm2, String> {
    let arg = |k: &str| required(args, "change-tower-ei-nm2", k);
    Ok(ChangeTowerEiNm2 { id: dec_json(&arg("id")?)?, new_ei_nm2: dec_json(&arg("new-ei-nm2")?)? })
}

/// ✍️ Prints `change-bridge-variable-category` as an op-text line.
pub fn print_change_bridge_variable_category(p: &ChangeBridgeVariableCategory) -> String {
    format!("change-bridge-variable-category bridge-id={} id={} new-category={}", enc_json(&p.bridge_id), enc_json(&p.id), enc_json(&p.new_category))
}

/// 📖 Parses the arguments of `change-bridge-variable-category`.
pub fn parse_change_bridge_variable_category(args: &Args) -> Result<ChangeBridgeVariableCategory, String> {
    let arg = |k: &str| required(args, "change-bridge-variable-category", k);
    Ok(ChangeBridgeVariableCategory { bridge_id: dec_json(&arg("bridge-id")?)?, id: dec_json(&arg("id")?)?, new_category: dec_json(&arg("new-category")?)? })
}

/// ✍️ Prints `change-bridge-variable-qk-n` as an op-text line.
pub fn print_change_bridge_variable_qk_n(p: &ChangeBridgeVariableQkN) -> String {
    format!("change-bridge-variable-qk-n bridge-id={} id={} new-qk-n={}", enc_json(&p.bridge_id), enc_json(&p.id), enc_json(&p.new_qk_n))
}

/// 📖 Parses the arguments of `change-bridge-variable-qk-n`.
pub fn parse_change_bridge_variable_qk_n(args: &Args) -> Result<ChangeBridgeVariableQkN, String> {
    let arg = |k: &str| required(args, "change-bridge-variable-qk-n", k);
    Ok(ChangeBridgeVariableQkN { bridge_id: dec_json(&arg("bridge-id")?)?, id: dec_json(&arg("id")?)?, new_qk_n: dec_json(&arg("new-qk-n")?)? })
}

/// ✍️ Prints `change-tower-variable-category` as an op-text line.
pub fn print_change_tower_variable_category(p: &ChangeTowerVariableCategory) -> String {
    format!("change-tower-variable-category tower-id={} id={} new-category={}", enc_json(&p.tower_id), enc_json(&p.id), enc_json(&p.new_category))
}

/// 📖 Parses the arguments of `change-tower-variable-category`.
pub fn parse_change_tower_variable_category(args: &Args) -> Result<ChangeTowerVariableCategory, String> {
    let arg = |k: &str| required(args, "change-tower-variable-category", k);
    Ok(ChangeTowerVariableCategory { tower_id: dec_json(&arg("tower-id")?)?, id: dec_json(&arg("id")?)?, new_category: dec_json(&arg("new-category")?)? })
}

/// ✍️ Prints `change-tower-variable-qk-n` as an op-text line.
pub fn print_change_tower_variable_qk_n(p: &ChangeTowerVariableQkN) -> String {
    format!("change-tower-variable-qk-n tower-id={} id={} new-qk-n={}", enc_json(&p.tower_id), enc_json(&p.id), enc_json(&p.new_qk_n))
}

/// 📖 Parses the arguments of `change-tower-variable-qk-n`.
pub fn parse_change_tower_variable_qk_n(args: &Args) -> Result<ChangeTowerVariableQkN, String> {
    let arg = |k: &str| required(args, "change-tower-variable-qk-n", k);
    Ok(ChangeTowerVariableQkN { tower_id: dec_json(&arg("tower-id")?)?, id: dec_json(&arg("id")?)?, new_qk_n: dec_json(&arg("new-qk-n")?)? })
}

/// ✍️ Prints `insert-bridge-variable` as an op-text line.
pub fn print_insert_bridge_variable(p: &InsertBridgeVariable) -> String {
    format!("insert-bridge-variable bridge-id={} index={} variable={}", enc_json(&p.bridge_id), enc_json(&p.index), enc_json(&p.variable))
}

/// 📖 Parses the arguments of `insert-bridge-variable`.
pub fn parse_insert_bridge_variable(args: &Args) -> Result<InsertBridgeVariable, String> {
    let arg = |k: &str| required(args, "insert-bridge-variable", k);
    Ok(InsertBridgeVariable { bridge_id: dec_json(&arg("bridge-id")?)?, index: opt_index(args)?, variable: dec_json(&arg("variable")?)? })
}

/// ✍️ Prints `remove-bridge-variable` as an op-text line.
pub fn print_remove_bridge_variable(p: &RemoveBridgeVariable) -> String {
    format!("remove-bridge-variable bridge-id={} id={}", enc_json(&p.bridge_id), enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-bridge-variable`.
pub fn parse_remove_bridge_variable(args: &Args) -> Result<RemoveBridgeVariable, String> {
    let arg = |k: &str| required(args, "remove-bridge-variable", k);
    Ok(RemoveBridgeVariable { bridge_id: dec_json(&arg("bridge-id")?)?, id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `insert-tower-variable` as an op-text line.
pub fn print_insert_tower_variable(p: &InsertTowerVariable) -> String {
    format!("insert-tower-variable tower-id={} index={} variable={}", enc_json(&p.tower_id), enc_json(&p.index), enc_json(&p.variable))
}

/// 📖 Parses the arguments of `insert-tower-variable`.
pub fn parse_insert_tower_variable(args: &Args) -> Result<InsertTowerVariable, String> {
    let arg = |k: &str| required(args, "insert-tower-variable", k);
    Ok(InsertTowerVariable { tower_id: dec_json(&arg("tower-id")?)?, index: opt_index(args)?, variable: dec_json(&arg("variable")?)? })
}

/// ✍️ Prints `remove-tower-variable` as an op-text line.
pub fn print_remove_tower_variable(p: &RemoveTowerVariable) -> String {
    format!("remove-tower-variable tower-id={} id={}", enc_json(&p.tower_id), enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-tower-variable`.
pub fn parse_remove_tower_variable(args: &Args) -> Result<RemoveTowerVariable, String> {
    let arg = |k: &str| required(args, "remove-tower-variable", k);
    Ok(RemoveTowerVariable { tower_id: dec_json(&arg("tower-id")?)?, id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `change-tower-base-rot-stiff-nm` as an op-text line.
pub fn print_change_tower_base_rot_stiff_nm(p: &ChangeTowerBaseRotStiffNm) -> String {
    format!("change-tower-base-rot-stiff-nm id={} new-base-rotational-stiffness-nm={}", enc_json(&p.id), enc_json(&p.new_base_rotational_stiffness_nm))
}

/// 📖 Parses the arguments of `change-tower-base-rot-stiff-nm`.
pub fn parse_change_tower_base_rot_stiff_nm(args: &Args) -> Result<ChangeTowerBaseRotStiffNm, String> {
    let arg = |k: &str| required(args, "change-tower-base-rot-stiff-nm", k);
    Ok(ChangeTowerBaseRotStiffNm { id: dec_json(&arg("id")?)?, new_base_rotational_stiffness_nm: dec_json(&arg("new-base-rotational-stiffness-nm")?)? })
}

/// ✍️ Prints `change-pier-bridge-id` as an op-text line.
pub fn print_change_pier_bridge_id(p: &ChangePierBridgeId) -> String {
    format!("change-pier-bridge-id id={} new-bridge-id={}", enc_json(&p.id), enc_json(&p.new_bridge_id))
}

/// 📖 Parses the arguments of `change-pier-bridge-id`.
pub fn parse_change_pier_bridge_id(args: &Args) -> Result<ChangePierBridgeId, String> {
    let arg = |k: &str| required(args, "change-pier-bridge-id", k);
    Ok(ChangePierBridgeId { id: dec_json(&arg("id")?)?, new_bridge_id: dec_json(&arg("new-bridge-id")?)? })
}

/// ✍️ Prints `change-pier-stiffness-basis` as an op-text line.
pub fn print_change_pier_stiffness_basis(p: &ChangePierStiffnessBasis) -> String {
    format!("change-pier-stiffness-basis id={} new-stiffness-basis={}", enc_json(&p.id), enc_json(&p.new_stiffness_basis))
}

/// 📖 Parses the arguments of `change-pier-stiffness-basis`.
pub fn parse_change_pier_stiffness_basis(args: &Args) -> Result<ChangePierStiffnessBasis, String> {
    let arg = |k: &str| required(args, "change-pier-stiffness-basis", k);
    Ok(ChangePierStiffnessBasis { id: dec_json(&arg("id")?)?, new_stiffness_basis: dec_json(&arg("new-stiffness-basis")?)? })
}

/// ✍️ Prints `change-pier-xm` as an op-text line.
pub fn print_change_pier_x_m(p: &ChangePierXM) -> String {
    format!("change-pier-xm id={} new-x-m={}", enc_json(&p.id), enc_json(&p.new_x_m))
}

/// 📖 Parses the arguments of `change-pier-xm`.
pub fn parse_change_pier_x_m(args: &Args) -> Result<ChangePierXM, String> {
    let arg = |k: &str| required(args, "change-pier-xm", k);
    Ok(ChangePierXM { id: dec_json(&arg("id")?)?, new_x_m: dec_json(&arg("new-x-m")?)? })
}

/// ✍️ Prints `change-pier-height-m` as an op-text line.
pub fn print_change_pier_height_m(p: &ChangePierHeightM) -> String {
    format!("change-pier-height-m id={} new-height-m={}", enc_json(&p.id), enc_json(&p.new_height_m))
}

/// 📖 Parses the arguments of `change-pier-height-m`.
pub fn parse_change_pier_height_m(args: &Args) -> Result<ChangePierHeightM, String> {
    let arg = |k: &str| required(args, "change-pier-height-m", k);
    Ok(ChangePierHeightM { id: dec_json(&arg("id")?)?, new_height_m: dec_json(&arg("new-height-m")?)? })
}

/// ✍️ Prints `change-pier-section-bm` as an op-text line.
pub fn print_change_pier_section_b_m(p: &ChangePierSectionBM) -> String {
    format!("change-pier-section-bm id={} new-section-b-m={}", enc_json(&p.id), enc_json(&p.new_section_b_m))
}

/// 📖 Parses the arguments of `change-pier-section-bm`.
pub fn parse_change_pier_section_b_m(args: &Args) -> Result<ChangePierSectionBM, String> {
    let arg = |k: &str| required(args, "change-pier-section-bm", k);
    Ok(ChangePierSectionBM { id: dec_json(&arg("id")?)?, new_section_b_m: dec_json(&arg("new-section-b-m")?)? })
}

/// ✍️ Prints `change-pier-section-hm` as an op-text line.
pub fn print_change_pier_section_h_m(p: &ChangePierSectionHM) -> String {
    format!("change-pier-section-hm id={} new-section-h-m={}", enc_json(&p.id), enc_json(&p.new_section_h_m))
}

/// 📖 Parses the arguments of `change-pier-section-hm`.
pub fn parse_change_pier_section_h_m(args: &Args) -> Result<ChangePierSectionHM, String> {
    let arg = |k: &str| required(args, "change-pier-section-hm", k);
    Ok(ChangePierSectionHM { id: dec_json(&arg("id")?)?, new_section_h_m: dec_json(&arg("new-section-h-m")?)? })
}

/// ✍️ Prints `change-pier-modulus-mpa` as an op-text line.
pub fn print_change_pier_modulus_mpa(p: &ChangePierModulusMpa) -> String {
    format!("change-pier-modulus-mpa id={} new-modulus-mpa={}", enc_json(&p.id), enc_json(&p.new_modulus_mpa))
}

/// 📖 Parses the arguments of `change-pier-modulus-mpa`.
pub fn parse_change_pier_modulus_mpa(args: &Args) -> Result<ChangePierModulusMpa, String> {
    let arg = |k: &str| required(args, "change-pier-modulus-mpa", k);
    Ok(ChangePierModulusMpa { id: dec_json(&arg("id")?)?, new_modulus_mpa: dec_json(&arg("new-modulus-mpa")?)? })
}

/// ✍️ Prints `change-pier-moment-rd-nm` as an op-text line.
pub fn print_change_pier_moment_rd_nm(p: &ChangePierMomentRdNm) -> String {
    format!("change-pier-moment-rd-nm id={} new-moment-rd-nm={}", enc_json(&p.id), enc_json(&p.new_moment_rd_nm))
}

/// 📖 Parses the arguments of `change-pier-moment-rd-nm`.
pub fn parse_change_pier_moment_rd_nm(args: &Args) -> Result<ChangePierMomentRdNm, String> {
    let arg = |k: &str| required(args, "change-pier-moment-rd-nm", k);
    Ok(ChangePierMomentRdNm { id: dec_json(&arg("id")?)?, new_moment_rd_nm: dec_json(&arg("new-moment-rd-nm")?)? })
}

/// ✍️ Prints `change-pier-shear-rd-n` as an op-text line.
pub fn print_change_pier_shear_rd_n(p: &ChangePierShearRdN) -> String {
    format!("change-pier-shear-rd-n id={} new-shear-rd-n={}", enc_json(&p.id), enc_json(&p.new_shear_rd_n))
}

/// 📖 Parses the arguments of `change-pier-shear-rd-n`.
pub fn parse_change_pier_shear_rd_n(args: &Args) -> Result<ChangePierShearRdN, String> {
    let arg = |k: &str| required(args, "change-pier-shear-rd-n", k);
    Ok(ChangePierShearRdN { id: dec_json(&arg("id")?)?, new_shear_rd_n: dec_json(&arg("new-shear-rd-n")?)? })
}

/// ✍️ Prints `change-pier-reinforcement-ratio` as an op-text line.
pub fn print_change_pier_reinforcement_ratio(p: &ChangePierReinforcementRatio) -> String {
    format!("change-pier-reinforcement-ratio id={} new-reinforcement-ratio={}", enc_json(&p.id), enc_json(&p.new_reinforcement_ratio))
}

/// 📖 Parses the arguments of `change-pier-reinforcement-ratio`.
pub fn parse_change_pier_reinforcement_ratio(args: &Args) -> Result<ChangePierReinforcementRatio, String> {
    let arg = |k: &str| required(args, "change-pier-reinforcement-ratio", k);
    Ok(ChangePierReinforcementRatio { id: dec_json(&arg("id")?)?, new_reinforcement_ratio: dec_json(&arg("new-reinforcement-ratio")?)? })
}

/// ✍️ Prints `change-pier-cover-m` as an op-text line.
pub fn print_change_pier_cover_m(p: &ChangePierCoverM) -> String {
    format!("change-pier-cover-m id={} new-cover-m={}", enc_json(&p.id), enc_json(&p.new_cover_m))
}

/// 📖 Parses the arguments of `change-pier-cover-m`.
pub fn parse_change_pier_cover_m(args: &Args) -> Result<ChangePierCoverM, String> {
    let arg = |k: &str| required(args, "change-pier-cover-m", k);
    Ok(ChangePierCoverM { id: dec_json(&arg("id")?)?, new_cover_m: dec_json(&arg("new-cover-m")?)? })
}

/// ✍️ Prints `change-pier-concrete-fck-mpa` as an op-text line.
pub fn print_change_pier_concrete_fck_mpa(p: &ChangePierConcreteFckMpa) -> String {
    format!("change-pier-concrete-fck-mpa id={} new-concrete-fck-mpa={}", enc_json(&p.id), enc_json(&p.new_concrete_fck_mpa))
}

/// 📖 Parses the arguments of `change-pier-concrete-fck-mpa`.
pub fn parse_change_pier_concrete_fck_mpa(args: &Args) -> Result<ChangePierConcreteFckMpa, String> {
    let arg = |k: &str| required(args, "change-pier-concrete-fck-mpa", k);
    Ok(ChangePierConcreteFckMpa { id: dec_json(&arg("id")?)?, new_concrete_fck_mpa: dec_json(&arg("new-concrete-fck-mpa")?)? })
}

/// ✍️ Prints `change-pier-steel-fyk-mpa` as an op-text line.
pub fn print_change_pier_steel_fyk_mpa(p: &ChangePierSteelFykMpa) -> String {
    format!("change-pier-steel-fyk-mpa id={} new-steel-fyk-mpa={}", enc_json(&p.id), enc_json(&p.new_steel_fyk_mpa))
}

/// 📖 Parses the arguments of `change-pier-steel-fyk-mpa`.
pub fn parse_change_pier_steel_fyk_mpa(args: &Args) -> Result<ChangePierSteelFykMpa, String> {
    let arg = |k: &str| required(args, "change-pier-steel-fyk-mpa", k);
    Ok(ChangePierSteelFykMpa { id: dec_json(&arg("id")?)?, new_steel_fyk_mpa: dec_json(&arg("new-steel-fyk-mpa")?)? })
}

/// ✍️ Prints `change-pier-axial-load-n` as an op-text line.
pub fn print_change_pier_axial_load_n(p: &ChangePierAxialLoadN) -> String {
    format!("change-pier-axial-load-n id={} new-axial-load-n={}", enc_json(&p.id), enc_json(&p.new_axial_load_n))
}

/// 📖 Parses the arguments of `change-pier-axial-load-n`.
pub fn parse_change_pier_axial_load_n(args: &Args) -> Result<ChangePierAxialLoadN, String> {
    let arg = |k: &str| required(args, "change-pier-axial-load-n", k);
    Ok(ChangePierAxialLoadN { id: dec_json(&arg("id")?)?, new_axial_load_n: dec_json(&arg("new-axial-load-n")?)? })
}

/// ✍️ Prints `change-bearing-bridge-id` as an op-text line.
pub fn print_change_bearing_bridge_id(p: &ChangeBearingBridgeId) -> String {
    format!("change-bearing-bridge-id id={} new-bridge-id={}", enc_json(&p.id), enc_json(&p.new_bridge_id))
}

/// 📖 Parses the arguments of `change-bearing-bridge-id`.
pub fn parse_change_bearing_bridge_id(args: &Args) -> Result<ChangeBearingBridgeId, String> {
    let arg = |k: &str| required(args, "change-bearing-bridge-id", k);
    Ok(ChangeBearingBridgeId { id: dec_json(&arg("id")?)?, new_bridge_id: dec_json(&arg("new-bridge-id")?)? })
}

/// ✍️ Prints `change-bearing-pier-id` as an op-text line.
pub fn print_change_bearing_pier_id(p: &ChangeBearingPierId) -> String {
    format!("change-bearing-pier-id id={} new-pier-id={}", enc_json(&p.id), enc_json(&p.new_pier_id))
}

/// 📖 Parses the arguments of `change-bearing-pier-id`.
pub fn parse_change_bearing_pier_id(args: &Args) -> Result<ChangeBearingPierId, String> {
    let arg = |k: &str| required(args, "change-bearing-pier-id", k);
    Ok(ChangeBearingPierId { id: dec_json(&arg("id")?)?, new_pier_id: dec_json(&arg("new-pier-id")?)? })
}

/// ✍️ Prints `change-bearing-device` as an op-text line.
pub fn print_change_bearing_device(p: &ChangeBearingDevice) -> String {
    format!("change-bearing-device id={} new-device={}", enc_json(&p.id), enc_json(&p.new_device))
}

/// 📖 Parses the arguments of `change-bearing-device`.
pub fn parse_change_bearing_device(args: &Args) -> Result<ChangeBearingDevice, String> {
    let arg = |k: &str| required(args, "change-bearing-device", k);
    Ok(ChangeBearingDevice { id: dec_json(&arg("id")?)?, new_device: dec_json(&arg("new-device")?)? })
}

/// ✍️ Prints `change-bearing-xm` as an op-text line.
pub fn print_change_bearing_x_m(p: &ChangeBearingXM) -> String {
    format!("change-bearing-xm id={} new-x-m={}", enc_json(&p.id), enc_json(&p.new_x_m))
}

/// 📖 Parses the arguments of `change-bearing-xm`.
pub fn parse_change_bearing_x_m(args: &Args) -> Result<ChangeBearingXM, String> {
    let arg = |k: &str| required(args, "change-bearing-xm", k);
    Ok(ChangeBearingXM { id: dec_json(&arg("id")?)?, new_x_m: dec_json(&arg("new-x-m")?)? })
}

/// ✍️ Prints `change-bearing-stiffness-n-per-m` as an op-text line.
pub fn print_change_bearing_stiffness_n_per_m(p: &ChangeBearingStiffnessNPerM) -> String {
    format!("change-bearing-stiffness-n-per-m id={} new-stiffness-n-per-m={}", enc_json(&p.id), enc_json(&p.new_stiffness_n_per_m))
}

/// 📖 Parses the arguments of `change-bearing-stiffness-n-per-m`.
pub fn parse_change_bearing_stiffness_n_per_m(args: &Args) -> Result<ChangeBearingStiffnessNPerM, String> {
    let arg = |k: &str| required(args, "change-bearing-stiffness-n-per-m", k);
    Ok(ChangeBearingStiffnessNPerM { id: dec_json(&arg("id")?)?, new_stiffness_n_per_m: dec_json(&arg("new-stiffness-n-per-m")?)? })
}

/// ✍️ Prints `change-bearing-d-rd-m` as an op-text line.
pub fn print_change_bearing_d_rd_m(p: &ChangeBearingDRdM) -> String {
    format!("change-bearing-d-rd-m id={} new-d-rd-m={}", enc_json(&p.id), enc_json(&p.new_d_rd_m))
}

/// 📖 Parses the arguments of `change-bearing-d-rd-m`.
pub fn parse_change_bearing_d_rd_m(args: &Args) -> Result<ChangeBearingDRdM, String> {
    let arg = |k: &str| required(args, "change-bearing-d-rd-m", k);
    Ok(ChangeBearingDRdM { id: dec_json(&arg("id")?)?, new_d_rd_m: dec_json(&arg("new-d-rd-m")?)? })
}

/// ✍️ Prints `change-bearing-seat-length-m` as an op-text line.
pub fn print_change_bearing_seat_length_m(p: &ChangeBearingSeatLengthM) -> String {
    format!("change-bearing-seat-length-m id={} new-seat-length-m={}", enc_json(&p.id), enc_json(&p.new_seat_length_m))
}

/// 📖 Parses the arguments of `change-bearing-seat-length-m`.
pub fn parse_change_bearing_seat_length_m(args: &Args) -> Result<ChangeBearingSeatLengthM, String> {
    let arg = |k: &str| required(args, "change-bearing-seat-length-m", k);
    Ok(ChangeBearingSeatLengthM { id: dec_json(&arg("id")?)?, new_seat_length_m: dec_json(&arg("new-seat-length-m")?)? })
}

/// ✍️ Prints `change-bearing-min-support-m` as an op-text line.
pub fn print_change_bearing_min_support_m(p: &ChangeBearingMinSupportM) -> String {
    format!("change-bearing-min-support-m id={} new-min-support-length-m={}", enc_json(&p.id), enc_json(&p.new_min_support_length_m))
}

/// 📖 Parses the arguments of `change-bearing-min-support-m`.
pub fn parse_change_bearing_min_support_m(args: &Args) -> Result<ChangeBearingMinSupportM, String> {
    let arg = |k: &str| required(args, "change-bearing-min-support-m", k);
    Ok(ChangeBearingMinSupportM { id: dec_json(&arg("id")?)?, new_min_support_length_m: dec_json(&arg("new-min-support-length-m")?)? })
}

/// ✍️ Prints `insert-pier` as an op-text line.
pub fn print_insert_pier(p: &InsertPier) -> String {
    format!("insert-pier index={} pier={}", enc_json(&p.index), enc_json(&p.pier))
}

/// 📖 Parses the arguments of `insert-pier`.
pub fn parse_insert_pier(args: &Args) -> Result<InsertPier, String> {
    let arg = |k: &str| required(args, "insert-pier", k);
    Ok(InsertPier { index: opt_index(args)?, pier: dec_json(&arg("pier")?)? })
}

/// ✍️ Prints `remove-pier` as an op-text line.
pub fn print_remove_pier(p: &RemovePier) -> String {
    format!("remove-pier id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-pier`.
pub fn parse_remove_pier(args: &Args) -> Result<RemovePier, String> {
    let arg = |k: &str| required(args, "remove-pier", k);
    Ok(RemovePier { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `insert-bearing` as an op-text line.
pub fn print_insert_bearing(p: &InsertBearing) -> String {
    format!("insert-bearing index={} bearing={}", enc_json(&p.index), enc_json(&p.bearing))
}

/// 📖 Parses the arguments of `insert-bearing`.
pub fn parse_insert_bearing(args: &Args) -> Result<InsertBearing, String> {
    let arg = |k: &str| required(args, "insert-bearing", k);
    Ok(InsertBearing { index: opt_index(args)?, bearing: dec_json(&arg("bearing")?)? })
}

/// ✍️ Prints `remove-bearing` as an op-text line.
pub fn print_remove_bearing(p: &RemoveBearing) -> String {
    format!("remove-bearing id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-bearing`.
pub fn parse_remove_bearing(args: &Args) -> Result<RemoveBearing, String> {
    let arg = |k: &str| required(args, "remove-bearing", k);
    Ok(RemoveBearing { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `remove-tower` as an op-text line.
pub fn print_remove_tower(p: &RemoveTower) -> String {
    format!("remove-tower id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-tower`.
pub fn parse_remove_tower(args: &Args) -> Result<RemoveTower, String> {
    let arg = |k: &str| required(args, "remove-tower", k);
    Ok(RemoveTower { id: dec_json(&arg("id")?)? })
}
