//! 🏭️ EN 1998 silos tanks mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_silos_tanks::insert_silo::InsertSilo;
use semio_s_artifact_norm_en1998_mut_silos_tanks::insert_tank::InsertTank;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_height_m::ChangeSiloHeightM;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_radius_m::ChangeSiloRadiusM;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_permanent_gk_n::ChangeSiloPermanentGkN;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_content_qk_n::ChangeSiloContentQkN;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_content_category::ChangeSiloContentCategory;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_filling_ratio::ChangeSiloFillingRatio;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_n_rd_n::ChangeSiloNRdN;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_v_rd_n::ChangeSiloVRdN;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_silo_q_nominal::ChangeSiloQNominal;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_height_m::ChangeTankHeightM;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_radius_m::ChangeTankRadiusM;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_permanent_gk_n::ChangeTankPermanentGkN;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_content_qk_n::ChangeTankContentQkN;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_content_category::ChangeTankContentCategory;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_filling_ratio::ChangeTankFillingRatio;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_v_rd_n::ChangeTankVRdN;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_shell_t_m::ChangeTankShellTM;
use semio_s_artifact_norm_en1998_mut_silos_tanks::change_tank_shell_str_mpa::ChangeTankShellStrMpa;
use semio_s_artifact_norm_en1998_mut_silos_tanks::remove_silo::RemoveSilo;
use semio_s_artifact_norm_en1998_mut_silos_tanks::remove_tank::RemoveTank;

/// ✍️ Prints `insert-silo` as an op-text line.
pub fn print_insert_silo(p: &InsertSilo) -> String {
    format!("insert-silo index={} silo={}", enc_json(&p.index), enc_json(&p.silo))
}

/// 📖 Parses the arguments of `insert-silo`.
pub fn parse_insert_silo(args: &Args) -> Result<InsertSilo, String> {
    let arg = |k: &str| required(args, "insert-silo", k);
    Ok(InsertSilo { index: opt_index(args)?, silo: dec_json(&arg("silo")?)? })
}

/// ✍️ Prints `insert-tank` as an op-text line.
pub fn print_insert_tank(p: &InsertTank) -> String {
    format!("insert-tank index={} tank={}", enc_json(&p.index), enc_json(&p.tank))
}

/// 📖 Parses the arguments of `insert-tank`.
pub fn parse_insert_tank(args: &Args) -> Result<InsertTank, String> {
    let arg = |k: &str| required(args, "insert-tank", k);
    Ok(InsertTank { index: opt_index(args)?, tank: dec_json(&arg("tank")?)? })
}

/// ✍️ Prints `change-silo-height-m` as an op-text line.
pub fn print_change_silo_height_m(p: &ChangeSiloHeightM) -> String {
    format!("change-silo-height-m id={} new-height-m={}", enc_json(&p.id), enc_json(&p.new_height_m))
}

/// 📖 Parses the arguments of `change-silo-height-m`.
pub fn parse_change_silo_height_m(args: &Args) -> Result<ChangeSiloHeightM, String> {
    let arg = |k: &str| required(args, "change-silo-height-m", k);
    Ok(ChangeSiloHeightM { id: dec_json(&arg("id")?)?, new_height_m: dec_json(&arg("new-height-m")?)? })
}

/// ✍️ Prints `change-silo-radius-m` as an op-text line.
pub fn print_change_silo_radius_m(p: &ChangeSiloRadiusM) -> String {
    format!("change-silo-radius-m id={} new-radius-m={}", enc_json(&p.id), enc_json(&p.new_radius_m))
}

/// 📖 Parses the arguments of `change-silo-radius-m`.
pub fn parse_change_silo_radius_m(args: &Args) -> Result<ChangeSiloRadiusM, String> {
    let arg = |k: &str| required(args, "change-silo-radius-m", k);
    Ok(ChangeSiloRadiusM { id: dec_json(&arg("id")?)?, new_radius_m: dec_json(&arg("new-radius-m")?)? })
}

/// ✍️ Prints `change-silo-permanent-gk-n` as an op-text line.
pub fn print_change_silo_permanent_gk_n(p: &ChangeSiloPermanentGkN) -> String {
    format!("change-silo-permanent-gk-n id={} new-permanent-gk-n={}", enc_json(&p.id), enc_json(&p.new_permanent_gk_n))
}

/// 📖 Parses the arguments of `change-silo-permanent-gk-n`.
pub fn parse_change_silo_permanent_gk_n(args: &Args) -> Result<ChangeSiloPermanentGkN, String> {
    let arg = |k: &str| required(args, "change-silo-permanent-gk-n", k);
    Ok(ChangeSiloPermanentGkN { id: dec_json(&arg("id")?)?, new_permanent_gk_n: dec_json(&arg("new-permanent-gk-n")?)? })
}

/// ✍️ Prints `change-silo-content-qk-n` as an op-text line.
pub fn print_change_silo_content_qk_n(p: &ChangeSiloContentQkN) -> String {
    format!("change-silo-content-qk-n id={} new-content-qk-n={}", enc_json(&p.id), enc_json(&p.new_content_qk_n))
}

/// 📖 Parses the arguments of `change-silo-content-qk-n`.
pub fn parse_change_silo_content_qk_n(args: &Args) -> Result<ChangeSiloContentQkN, String> {
    let arg = |k: &str| required(args, "change-silo-content-qk-n", k);
    Ok(ChangeSiloContentQkN { id: dec_json(&arg("id")?)?, new_content_qk_n: dec_json(&arg("new-content-qk-n")?)? })
}

/// ✍️ Prints `change-silo-content-category` as an op-text line.
pub fn print_change_silo_content_category(p: &ChangeSiloContentCategory) -> String {
    format!("change-silo-content-category id={} new-content-category={}", enc_json(&p.id), enc_json(&p.new_content_category))
}

/// 📖 Parses the arguments of `change-silo-content-category`.
pub fn parse_change_silo_content_category(args: &Args) -> Result<ChangeSiloContentCategory, String> {
    let arg = |k: &str| required(args, "change-silo-content-category", k);
    Ok(ChangeSiloContentCategory { id: dec_json(&arg("id")?)?, new_content_category: dec_json(&arg("new-content-category")?)? })
}

/// ✍️ Prints `change-silo-filling-ratio` as an op-text line.
pub fn print_change_silo_filling_ratio(p: &ChangeSiloFillingRatio) -> String {
    format!("change-silo-filling-ratio id={} new-filling-ratio={}", enc_json(&p.id), enc_json(&p.new_filling_ratio))
}

/// 📖 Parses the arguments of `change-silo-filling-ratio`.
pub fn parse_change_silo_filling_ratio(args: &Args) -> Result<ChangeSiloFillingRatio, String> {
    let arg = |k: &str| required(args, "change-silo-filling-ratio", k);
    Ok(ChangeSiloFillingRatio { id: dec_json(&arg("id")?)?, new_filling_ratio: dec_json(&arg("new-filling-ratio")?)? })
}

/// ✍️ Prints `change-silo-n-rd-n` as an op-text line.
pub fn print_change_silo_n_rd_n(p: &ChangeSiloNRdN) -> String {
    format!("change-silo-n-rd-n id={} new-n-rd-n={}", enc_json(&p.id), enc_json(&p.new_n_rd_n))
}

/// 📖 Parses the arguments of `change-silo-n-rd-n`.
pub fn parse_change_silo_n_rd_n(args: &Args) -> Result<ChangeSiloNRdN, String> {
    let arg = |k: &str| required(args, "change-silo-n-rd-n", k);
    Ok(ChangeSiloNRdN { id: dec_json(&arg("id")?)?, new_n_rd_n: dec_json(&arg("new-n-rd-n")?)? })
}

/// ✍️ Prints `change-silo-v-rd-n` as an op-text line.
pub fn print_change_silo_v_rd_n(p: &ChangeSiloVRdN) -> String {
    format!("change-silo-v-rd-n id={} new-v-rd-n={}", enc_json(&p.id), enc_json(&p.new_v_rd_n))
}

/// 📖 Parses the arguments of `change-silo-v-rd-n`.
pub fn parse_change_silo_v_rd_n(args: &Args) -> Result<ChangeSiloVRdN, String> {
    let arg = |k: &str| required(args, "change-silo-v-rd-n", k);
    Ok(ChangeSiloVRdN { id: dec_json(&arg("id")?)?, new_v_rd_n: dec_json(&arg("new-v-rd-n")?)? })
}

/// ✍️ Prints `change-silo-q-nominal` as an op-text line.
pub fn print_change_silo_q_nominal(p: &ChangeSiloQNominal) -> String {
    format!("change-silo-q-nominal id={} new-q-nominal={}", enc_json(&p.id), enc_json(&p.new_q_nominal))
}

/// 📖 Parses the arguments of `change-silo-q-nominal`.
pub fn parse_change_silo_q_nominal(args: &Args) -> Result<ChangeSiloQNominal, String> {
    let arg = |k: &str| required(args, "change-silo-q-nominal", k);
    Ok(ChangeSiloQNominal { id: dec_json(&arg("id")?)?, new_q_nominal: dec_json(&arg("new-q-nominal")?)? })
}

/// ✍️ Prints `change-tank-height-m` as an op-text line.
pub fn print_change_tank_height_m(p: &ChangeTankHeightM) -> String {
    format!("change-tank-height-m id={} new-height-m={}", enc_json(&p.id), enc_json(&p.new_height_m))
}

/// 📖 Parses the arguments of `change-tank-height-m`.
pub fn parse_change_tank_height_m(args: &Args) -> Result<ChangeTankHeightM, String> {
    let arg = |k: &str| required(args, "change-tank-height-m", k);
    Ok(ChangeTankHeightM { id: dec_json(&arg("id")?)?, new_height_m: dec_json(&arg("new-height-m")?)? })
}

/// ✍️ Prints `change-tank-radius-m` as an op-text line.
pub fn print_change_tank_radius_m(p: &ChangeTankRadiusM) -> String {
    format!("change-tank-radius-m id={} new-radius-m={}", enc_json(&p.id), enc_json(&p.new_radius_m))
}

/// 📖 Parses the arguments of `change-tank-radius-m`.
pub fn parse_change_tank_radius_m(args: &Args) -> Result<ChangeTankRadiusM, String> {
    let arg = |k: &str| required(args, "change-tank-radius-m", k);
    Ok(ChangeTankRadiusM { id: dec_json(&arg("id")?)?, new_radius_m: dec_json(&arg("new-radius-m")?)? })
}

/// ✍️ Prints `change-tank-permanent-gk-n` as an op-text line.
pub fn print_change_tank_permanent_gk_n(p: &ChangeTankPermanentGkN) -> String {
    format!("change-tank-permanent-gk-n id={} new-permanent-gk-n={}", enc_json(&p.id), enc_json(&p.new_permanent_gk_n))
}

/// 📖 Parses the arguments of `change-tank-permanent-gk-n`.
pub fn parse_change_tank_permanent_gk_n(args: &Args) -> Result<ChangeTankPermanentGkN, String> {
    let arg = |k: &str| required(args, "change-tank-permanent-gk-n", k);
    Ok(ChangeTankPermanentGkN { id: dec_json(&arg("id")?)?, new_permanent_gk_n: dec_json(&arg("new-permanent-gk-n")?)? })
}

/// ✍️ Prints `change-tank-content-qk-n` as an op-text line.
pub fn print_change_tank_content_qk_n(p: &ChangeTankContentQkN) -> String {
    format!("change-tank-content-qk-n id={} new-content-qk-n={}", enc_json(&p.id), enc_json(&p.new_content_qk_n))
}

/// 📖 Parses the arguments of `change-tank-content-qk-n`.
pub fn parse_change_tank_content_qk_n(args: &Args) -> Result<ChangeTankContentQkN, String> {
    let arg = |k: &str| required(args, "change-tank-content-qk-n", k);
    Ok(ChangeTankContentQkN { id: dec_json(&arg("id")?)?, new_content_qk_n: dec_json(&arg("new-content-qk-n")?)? })
}

/// ✍️ Prints `change-tank-content-category` as an op-text line.
pub fn print_change_tank_content_category(p: &ChangeTankContentCategory) -> String {
    format!("change-tank-content-category id={} new-content-category={}", enc_json(&p.id), enc_json(&p.new_content_category))
}

/// 📖 Parses the arguments of `change-tank-content-category`.
pub fn parse_change_tank_content_category(args: &Args) -> Result<ChangeTankContentCategory, String> {
    let arg = |k: &str| required(args, "change-tank-content-category", k);
    Ok(ChangeTankContentCategory { id: dec_json(&arg("id")?)?, new_content_category: dec_json(&arg("new-content-category")?)? })
}

/// ✍️ Prints `change-tank-filling-ratio` as an op-text line.
pub fn print_change_tank_filling_ratio(p: &ChangeTankFillingRatio) -> String {
    format!("change-tank-filling-ratio id={} new-filling-ratio={}", enc_json(&p.id), enc_json(&p.new_filling_ratio))
}

/// 📖 Parses the arguments of `change-tank-filling-ratio`.
pub fn parse_change_tank_filling_ratio(args: &Args) -> Result<ChangeTankFillingRatio, String> {
    let arg = |k: &str| required(args, "change-tank-filling-ratio", k);
    Ok(ChangeTankFillingRatio { id: dec_json(&arg("id")?)?, new_filling_ratio: dec_json(&arg("new-filling-ratio")?)? })
}

/// ✍️ Prints `change-tank-v-rd-n` as an op-text line.
pub fn print_change_tank_v_rd_n(p: &ChangeTankVRdN) -> String {
    format!("change-tank-v-rd-n id={} new-v-rd-n={}", enc_json(&p.id), enc_json(&p.new_v_rd_n))
}

/// 📖 Parses the arguments of `change-tank-v-rd-n`.
pub fn parse_change_tank_v_rd_n(args: &Args) -> Result<ChangeTankVRdN, String> {
    let arg = |k: &str| required(args, "change-tank-v-rd-n", k);
    Ok(ChangeTankVRdN { id: dec_json(&arg("id")?)?, new_v_rd_n: dec_json(&arg("new-v-rd-n")?)? })
}

/// ✍️ Prints `change-tank-shell-tm` as an op-text line.
pub fn print_change_tank_shell_t_m(p: &ChangeTankShellTM) -> String {
    format!("change-tank-shell-tm id={} new-shell-thickness-m={}", enc_json(&p.id), enc_json(&p.new_shell_thickness_m))
}

/// 📖 Parses the arguments of `change-tank-shell-tm`.
pub fn parse_change_tank_shell_t_m(args: &Args) -> Result<ChangeTankShellTM, String> {
    let arg = |k: &str| required(args, "change-tank-shell-tm", k);
    Ok(ChangeTankShellTM { id: dec_json(&arg("id")?)?, new_shell_thickness_m: dec_json(&arg("new-shell-thickness-m")?)? })
}

/// ✍️ Prints `change-tank-shell-str-mpa` as an op-text line.
pub fn print_change_tank_shell_str_mpa(p: &ChangeTankShellStrMpa) -> String {
    format!("change-tank-shell-str-mpa id={} new-shell-strength-mpa={}", enc_json(&p.id), enc_json(&p.new_shell_strength_mpa))
}

/// 📖 Parses the arguments of `change-tank-shell-str-mpa`.
pub fn parse_change_tank_shell_str_mpa(args: &Args) -> Result<ChangeTankShellStrMpa, String> {
    let arg = |k: &str| required(args, "change-tank-shell-str-mpa", k);
    Ok(ChangeTankShellStrMpa { id: dec_json(&arg("id")?)?, new_shell_strength_mpa: dec_json(&arg("new-shell-strength-mpa")?)? })
}

/// ✍️ Prints `remove-silo` as an op-text line.
pub fn print_remove_silo(p: &RemoveSilo) -> String {
    format!("remove-silo id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-silo`.
pub fn parse_remove_silo(args: &Args) -> Result<RemoveSilo, String> {
    let arg = |k: &str| required(args, "remove-silo", k);
    Ok(RemoveSilo { id: dec_json(&arg("id")?)? })
}

/// ✍️ Prints `remove-tank` as an op-text line.
pub fn print_remove_tank(p: &RemoveTank) -> String {
    format!("remove-tank id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-tank`.
pub fn parse_remove_tank(args: &Args) -> Result<RemoveTank, String> {
    let arg = |k: &str| required(args, "remove-tank", k);
    Ok(RemoveTank { id: dec_json(&arg("id")?)? })
}
