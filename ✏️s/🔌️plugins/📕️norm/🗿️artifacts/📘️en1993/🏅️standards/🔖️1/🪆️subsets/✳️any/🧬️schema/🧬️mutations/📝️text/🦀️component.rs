//! ⚡️ En1993 artifact — hand-rolled `OpText`/`OpBinary` for `En1993Mutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` (see `../🦀️component.rs`'s
//! `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted here, one keyword per
//! semantic verb, grammar `keyword key1=value1 key2=value2 ...`. Every payload field (even plain
//! `f64`/`u32`/`u8` scalars) round-trips through a quoted-JSON token — every one of them already
//! derives `Serialize`/`Deserialize`, so this reuses that losslessly instead of a hand-rolled
//! per-type encoder, matching the `iso16757` sibling facet's precedent for structured fields
//! (applied here to scalars too, since this facet has none of iso16757's nested entity records).

pub use crate::artifacts::en1993::schema::mutations::En1993Mutation;

use crate::artifacts::en1993::schema::mutations::{
    change_annex::ChangeAnnex, update_bolt_inputs::UpdateBoltInputs, update_bridge_inputs::UpdateBridgeInputs, update_cold_formed_inputs::UpdateColdFormedInputs,
    update_crane_inputs::UpdateCraneInputs, update_fatigue_inputs::UpdateFatigueInputs, update_fire_inputs::UpdateFireInputs, update_hss_inputs::UpdateHssInputs,
    update_member_properties::UpdateMemberProperties, update_pile_inputs::UpdatePileInputs, update_plated_inputs::UpdatePlatedInputs, update_silo_shell_inputs::UpdateSiloShellInputs,
    update_stainless_inputs::UpdateStainlessInputs, update_tension_component_inputs::UpdateTensionComponentInputs, update_through_thickness_inputs::UpdateThroughThicknessInputs,
    update_tower_inputs::UpdateTowerInputs, update_weld_inputs::UpdateWeldInputs,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🧬️ Every payload field already derives `Serialize`/`Deserialize` — a quoted JSON string reuses
/// that losslessly instead of a per-type handcrafted grammar.
fn enc_json<T: serde::Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("en1993 mutation payload field always serializes"))
}
fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other scalar's JSON text form stays tokenizable by [`tokenize_args`].
fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value — every
/// value here is itself a JSON string, which may legitimately contain spaces (e.g. `weld_steel_grade`).
fn tokenize_args(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                current.push(c);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️OpText
fn print_en1993_mutation(mutation: &En1993Mutation) -> String {
    match mutation {
        En1993Mutation::ChangeAnnex(p) => format!("change-annex annex={}", enc_json(&p.new_annex)),
        En1993Mutation::UpdateMemberProperties(p) => format!(
            "update-member-properties n-ed-kn={} m-ed-knm={} v-ed-kn={} a-mm2={} a-v-mm2={} w-pl-mm3={} f-y-mpa={} f-u-mpa={} chi={} a-net-mm2={} tension-n-ed-kn={}",
            enc_json(&p.new_n_ed_kn),
            enc_json(&p.new_m_ed_knm),
            enc_json(&p.new_v_ed_kn),
            enc_json(&p.new_a_mm2),
            enc_json(&p.new_a_v_mm2),
            enc_json(&p.new_w_pl_mm3),
            enc_json(&p.new_f_y_mpa),
            enc_json(&p.new_f_u_mpa),
            enc_json(&p.new_chi),
            enc_json(&p.new_a_net_mm2),
            enc_json(&p.new_tension_n_ed_kn)
        ),
        En1993Mutation::UpdateFireInputs(p) => format!(
            "update-fire-inputs fire-thickness-mm={} fire-rating={} fire-massivity={} fire-mu-0={} fire-design-temperature-c={}",
            enc_json(&p.new_fire_thickness_mm),
            enc_json(&p.new_fire_rating),
            enc_json(&p.new_fire_massivity),
            enc_json(&p.new_fire_mu_0),
            enc_json(&p.new_fire_design_temperature_c)
        ),
        En1993Mutation::UpdateColdFormedInputs(p) => format!(
            "update-cold-formed-inputs cf-b-bar-mm={} cf-t-mm={} cf-k-sigma={} cf-psi={} cf-n-ed-kn={} cf-gross-resistance-kn={}",
            enc_json(&p.new_cf_b_bar_mm),
            enc_json(&p.new_cf_t_mm),
            enc_json(&p.new_cf_k_sigma),
            enc_json(&p.new_cf_psi),
            enc_json(&p.new_cf_n_ed_kn),
            enc_json(&p.new_cf_gross_resistance_kn)
        ),
        En1993Mutation::UpdateStainlessInputs(p) => {
            format!("update-stainless-inputs stainless-m-ed-knm={} stainless-w-pl-mm3={} stainless-f-y-mpa={}", enc_json(&p.new_stainless_m_ed_knm), enc_json(&p.new_stainless_w_pl_mm3), enc_json(&p.new_stainless_f_y_mpa))
        }
        En1993Mutation::UpdatePlatedInputs(p) => format!("update-plated-inputs plated-lambda-p={} plated-sigma-ed-mpa={}", enc_json(&p.new_plated_lambda_p), enc_json(&p.new_plated_sigma_ed_mpa)),
        En1993Mutation::UpdateSiloShellInputs(p) => format!(
            "update-silo-shell-inputs silo-t-mm={} silo-r-mm={} shell-sigma-x-ed-mpa={} silo-k={} silo-gamma-kn-m3={} silo-depth-m={}",
            enc_json(&p.new_silo_t_mm),
            enc_json(&p.new_silo_r_mm),
            enc_json(&p.new_shell_sigma_x_ed_mpa),
            enc_json(&p.new_silo_k),
            enc_json(&p.new_silo_gamma_kn_m3),
            enc_json(&p.new_silo_depth_m)
        ),
        En1993Mutation::UpdateBoltInputs(p) => format!(
            "update-bolt-inputs bolt-f-ed-kn={} bolt-n-bolts={} bolt-a-s-mm2={} bolt-e1-mm={} bolt-e2-mm={} bolt-d0-mm={} bolt-d-mm={} bolt-t-mm={} bolt-f-u-mpa={} bolt-f-ub-mpa={}",
            enc_json(&p.new_bolt_f_ed_kn),
            enc_json(&p.new_bolt_n_bolts),
            enc_json(&p.new_bolt_a_s_mm2),
            enc_json(&p.new_bolt_e1_mm),
            enc_json(&p.new_bolt_e2_mm),
            enc_json(&p.new_bolt_d0_mm),
            enc_json(&p.new_bolt_d_mm),
            enc_json(&p.new_bolt_t_mm),
            enc_json(&p.new_bolt_f_u_mpa),
            enc_json(&p.new_bolt_f_ub_mpa)
        ),
        En1993Mutation::UpdateWeldInputs(p) => format!(
            "update-weld-inputs weld-a-mm={} weld-l-mm={} weld-f-u-mpa={} weld-steel-grade={} weld-f-ed-kn={}",
            enc_json(&p.new_weld_a_mm),
            enc_json(&p.new_weld_l_mm),
            enc_json(&p.new_weld_f_u_mpa),
            enc_json(&p.new_weld_steel_grade),
            enc_json(&p.new_weld_f_ed_kn)
        ),
        En1993Mutation::UpdateFatigueInputs(p) => format!("update-fatigue-inputs delta-sigma-mpa={} fatigue-category={} fatigue-method={}", enc_json(&p.new_delta_sigma_mpa), enc_json(&p.new_fatigue_category), enc_json(&p.new_fatigue_method)),
        En1993Mutation::UpdateThroughThicknessInputs(p) => {
            format!("update-through-thickness-inputs t10-steel-subgrade={} t10-actual-thickness-mm={} t10-t-ed-c={}", enc_json(&p.new_t10_steel_subgrade), enc_json(&p.new_t10_actual_thickness_mm), enc_json(&p.new_t10_t_ed_c))
        }
        En1993Mutation::UpdateTensionComponentInputs(p) => format!(
            "update-tension-component-inputs tension-component-f-uk-kn={} tension-component-f-k-kn={} tension-component-n-ed-kn={}",
            enc_json(&p.new_tension_component_f_uk_kn),
            enc_json(&p.new_tension_component_f_k_kn),
            enc_json(&p.new_tension_component_n_ed_kn)
        ),
        En1993Mutation::UpdateHssInputs(p) => {
            format!("update-hss-inputs hss-w-el-mm3={} hss-f-y-mpa={} hss-section-class={} hss-m-ed-knm={}", enc_json(&p.new_hss_w_el_mm3), enc_json(&p.new_hss_f_y_mpa), enc_json(&p.new_hss_section_class), enc_json(&p.new_hss_m_ed_knm))
        }
        En1993Mutation::UpdateBridgeInputs(p) => format!("update-bridge-inputs bridge-lambda={} bridge-phi-2={} bridge-delta-sigma-p-mpa={}", enc_json(&p.new_bridge_lambda), enc_json(&p.new_bridge_phi_2), enc_json(&p.new_bridge_delta_sigma_p_mpa)),
        En1993Mutation::UpdateTowerInputs(p) => format!("update-tower-inputs tower-wind-factor={} tower-n-ed-kn={}", enc_json(&p.new_tower_wind_factor), enc_json(&p.new_tower_n_ed_kn)),
        En1993Mutation::UpdatePileInputs(p) => format!("update-pile-inputs pile-sigma-mpa={} pile-k-red={} pile-n-ed-kn={}", enc_json(&p.new_pile_sigma_mpa), enc_json(&p.new_pile_k_red), enc_json(&p.new_pile_n_ed_kn)),
        En1993Mutation::UpdateCraneInputs(p) => format!(
            "update-crane-inputs crane-f-z-ed-kn={} crane-wheel-contact-length-mm={} crane-dispersion-mm={} crane-t-w-mm={}",
            enc_json(&p.new_crane_f_z_ed_kn),
            enc_json(&p.new_crane_wheel_contact_length_mm),
            enc_json(&p.new_crane_dispersion_mm),
            enc_json(&p.new_crane_t_w_mm)
        ),
    }
}

fn parse_en1993_mutation(line: &str) -> Result<En1993Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("en1993 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-annex" => Ok(En1993Mutation::ChangeAnnex(ChangeAnnex { new_annex: dec_json(&arg("annex")?)? })),
        "update-member-properties" => Ok(En1993Mutation::UpdateMemberProperties(UpdateMemberProperties {
            new_n_ed_kn: dec_json(&arg("n-ed-kn")?)?,
            new_m_ed_knm: dec_json(&arg("m-ed-knm")?)?,
            new_v_ed_kn: dec_json(&arg("v-ed-kn")?)?,
            new_a_mm2: dec_json(&arg("a-mm2")?)?,
            new_a_v_mm2: dec_json(&arg("a-v-mm2")?)?,
            new_w_pl_mm3: dec_json(&arg("w-pl-mm3")?)?,
            new_f_y_mpa: dec_json(&arg("f-y-mpa")?)?,
            new_f_u_mpa: dec_json(&arg("f-u-mpa")?)?,
            new_chi: dec_json(&arg("chi")?)?,
            new_a_net_mm2: dec_json(&arg("a-net-mm2")?)?,
            new_tension_n_ed_kn: dec_json(&arg("tension-n-ed-kn")?)?,
        })),
        "update-fire-inputs" => Ok(En1993Mutation::UpdateFireInputs(UpdateFireInputs {
            new_fire_thickness_mm: dec_json(&arg("fire-thickness-mm")?)?,
            new_fire_rating: dec_json(&arg("fire-rating")?)?,
            new_fire_massivity: dec_json(&arg("fire-massivity")?)?,
            new_fire_mu_0: dec_json(&arg("fire-mu-0")?)?,
            new_fire_design_temperature_c: dec_json(&arg("fire-design-temperature-c")?)?,
        })),
        "update-cold-formed-inputs" => Ok(En1993Mutation::UpdateColdFormedInputs(UpdateColdFormedInputs {
            new_cf_b_bar_mm: dec_json(&arg("cf-b-bar-mm")?)?,
            new_cf_t_mm: dec_json(&arg("cf-t-mm")?)?,
            new_cf_k_sigma: dec_json(&arg("cf-k-sigma")?)?,
            new_cf_psi: dec_json(&arg("cf-psi")?)?,
            new_cf_n_ed_kn: dec_json(&arg("cf-n-ed-kn")?)?,
            new_cf_gross_resistance_kn: dec_json(&arg("cf-gross-resistance-kn")?)?,
        })),
        "update-stainless-inputs" => Ok(En1993Mutation::UpdateStainlessInputs(UpdateStainlessInputs {
            new_stainless_m_ed_knm: dec_json(&arg("stainless-m-ed-knm")?)?,
            new_stainless_w_pl_mm3: dec_json(&arg("stainless-w-pl-mm3")?)?,
            new_stainless_f_y_mpa: dec_json(&arg("stainless-f-y-mpa")?)?,
        })),
        "update-plated-inputs" => Ok(En1993Mutation::UpdatePlatedInputs(UpdatePlatedInputs { new_plated_lambda_p: dec_json(&arg("plated-lambda-p")?)?, new_plated_sigma_ed_mpa: dec_json(&arg("plated-sigma-ed-mpa")?)? })),
        "update-silo-shell-inputs" => Ok(En1993Mutation::UpdateSiloShellInputs(UpdateSiloShellInputs {
            new_silo_t_mm: dec_json(&arg("silo-t-mm")?)?,
            new_silo_r_mm: dec_json(&arg("silo-r-mm")?)?,
            new_shell_sigma_x_ed_mpa: dec_json(&arg("shell-sigma-x-ed-mpa")?)?,
            new_silo_k: dec_json(&arg("silo-k")?)?,
            new_silo_gamma_kn_m3: dec_json(&arg("silo-gamma-kn-m3")?)?,
            new_silo_depth_m: dec_json(&arg("silo-depth-m")?)?,
        })),
        "update-bolt-inputs" => Ok(En1993Mutation::UpdateBoltInputs(UpdateBoltInputs {
            new_bolt_f_ed_kn: dec_json(&arg("bolt-f-ed-kn")?)?,
            new_bolt_n_bolts: dec_json(&arg("bolt-n-bolts")?)?,
            new_bolt_a_s_mm2: dec_json(&arg("bolt-a-s-mm2")?)?,
            new_bolt_e1_mm: dec_json(&arg("bolt-e1-mm")?)?,
            new_bolt_e2_mm: dec_json(&arg("bolt-e2-mm")?)?,
            new_bolt_d0_mm: dec_json(&arg("bolt-d0-mm")?)?,
            new_bolt_d_mm: dec_json(&arg("bolt-d-mm")?)?,
            new_bolt_t_mm: dec_json(&arg("bolt-t-mm")?)?,
            new_bolt_f_u_mpa: dec_json(&arg("bolt-f-u-mpa")?)?,
            new_bolt_f_ub_mpa: dec_json(&arg("bolt-f-ub-mpa")?)?,
        })),
        "update-weld-inputs" => Ok(En1993Mutation::UpdateWeldInputs(UpdateWeldInputs {
            new_weld_a_mm: dec_json(&arg("weld-a-mm")?)?,
            new_weld_l_mm: dec_json(&arg("weld-l-mm")?)?,
            new_weld_f_u_mpa: dec_json(&arg("weld-f-u-mpa")?)?,
            new_weld_steel_grade: dec_json(&arg("weld-steel-grade")?)?,
            new_weld_f_ed_kn: dec_json(&arg("weld-f-ed-kn")?)?,
        })),
        "update-fatigue-inputs" => {
            Ok(En1993Mutation::UpdateFatigueInputs(UpdateFatigueInputs { new_delta_sigma_mpa: dec_json(&arg("delta-sigma-mpa")?)?, new_fatigue_category: dec_json(&arg("fatigue-category")?)?, new_fatigue_method: dec_json(&arg("fatigue-method")?)? }))
        }
        "update-through-thickness-inputs" => Ok(En1993Mutation::UpdateThroughThicknessInputs(UpdateThroughThicknessInputs {
            new_t10_steel_subgrade: dec_json(&arg("t10-steel-subgrade")?)?,
            new_t10_actual_thickness_mm: dec_json(&arg("t10-actual-thickness-mm")?)?,
            new_t10_t_ed_c: dec_json(&arg("t10-t-ed-c")?)?,
        })),
        "update-tension-component-inputs" => Ok(En1993Mutation::UpdateTensionComponentInputs(UpdateTensionComponentInputs {
            new_tension_component_f_uk_kn: dec_json(&arg("tension-component-f-uk-kn")?)?,
            new_tension_component_f_k_kn: dec_json(&arg("tension-component-f-k-kn")?)?,
            new_tension_component_n_ed_kn: dec_json(&arg("tension-component-n-ed-kn")?)?,
        })),
        "update-hss-inputs" => Ok(En1993Mutation::UpdateHssInputs(UpdateHssInputs {
            new_hss_w_el_mm3: dec_json(&arg("hss-w-el-mm3")?)?,
            new_hss_f_y_mpa: dec_json(&arg("hss-f-y-mpa")?)?,
            new_hss_section_class: dec_json(&arg("hss-section-class")?)?,
            new_hss_m_ed_knm: dec_json(&arg("hss-m-ed-knm")?)?,
        })),
        "update-bridge-inputs" => Ok(En1993Mutation::UpdateBridgeInputs(UpdateBridgeInputs {
            new_bridge_lambda: dec_json(&arg("bridge-lambda")?)?,
            new_bridge_phi_2: dec_json(&arg("bridge-phi-2")?)?,
            new_bridge_delta_sigma_p_mpa: dec_json(&arg("bridge-delta-sigma-p-mpa")?)?,
        })),
        "update-tower-inputs" => Ok(En1993Mutation::UpdateTowerInputs(UpdateTowerInputs { new_tower_wind_factor: dec_json(&arg("tower-wind-factor")?)?, new_tower_n_ed_kn: dec_json(&arg("tower-n-ed-kn")?)? })),
        "update-pile-inputs" => Ok(En1993Mutation::UpdatePileInputs(UpdatePileInputs { new_pile_sigma_mpa: dec_json(&arg("pile-sigma-mpa")?)?, new_pile_k_red: dec_json(&arg("pile-k-red")?)?, new_pile_n_ed_kn: dec_json(&arg("pile-n-ed-kn")?)? })),
        "update-crane-inputs" => Ok(En1993Mutation::UpdateCraneInputs(UpdateCraneInputs {
            new_crane_f_z_ed_kn: dec_json(&arg("crane-f-z-ed-kn")?)?,
            new_crane_wheel_contact_length_mm: dec_json(&arg("crane-wheel-contact-length-mm")?)?,
            new_crane_dispersion_mm: dec_json(&arg("crane-dispersion-mm")?)?,
            new_crane_t_w_mm: dec_json(&arg("crane-t-w-mm")?)?,
        })),
        other => Err(format!("en1993 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for En1993Mutation {
    fn print_op(&self) -> String {
        print_en1993_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_en1993_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1993Mutation> {
    vec![
        En1993Mutation::ChangeAnnex(ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1993Mutation::UpdateMemberProperties(UpdateMemberProperties {
            new_n_ed_kn: 999.0,
            new_m_ed_knm: 999.0,
            new_v_ed_kn: 999.0,
            new_a_mm2: 999.0,
            new_a_v_mm2: 999.0,
            new_w_pl_mm3: 999.0,
            new_f_y_mpa: 999.0,
            new_f_u_mpa: 999.0,
            new_chi: 999.0,
            new_a_net_mm2: 999.0,
            new_tension_n_ed_kn: 999.0,
        }),
        En1993Mutation::UpdateFireInputs(UpdateFireInputs { new_fire_thickness_mm: 999.0, new_fire_rating: "changed".to_string(), new_fire_massivity: 999.0, new_fire_mu_0: 999.0, new_fire_design_temperature_c: 999.0 }),
        En1993Mutation::UpdateColdFormedInputs(UpdateColdFormedInputs { new_cf_b_bar_mm: 999.0, new_cf_t_mm: 999.0, new_cf_k_sigma: 999.0, new_cf_psi: 999.0, new_cf_n_ed_kn: 999.0, new_cf_gross_resistance_kn: 999.0 }),
        En1993Mutation::UpdateStainlessInputs(UpdateStainlessInputs { new_stainless_m_ed_knm: 999.0, new_stainless_w_pl_mm3: 999.0, new_stainless_f_y_mpa: 999.0 }),
        En1993Mutation::UpdatePlatedInputs(UpdatePlatedInputs { new_plated_lambda_p: 999.0, new_plated_sigma_ed_mpa: 999.0 }),
        En1993Mutation::UpdateSiloShellInputs(UpdateSiloShellInputs { new_silo_t_mm: 999.0, new_silo_r_mm: 999.0, new_shell_sigma_x_ed_mpa: 999.0, new_silo_k: 999.0, new_silo_gamma_kn_m3: 999.0, new_silo_depth_m: 999.0 }),
        En1993Mutation::UpdateBoltInputs(UpdateBoltInputs {
            new_bolt_f_ed_kn: 999.0,
            new_bolt_n_bolts: 9,
            new_bolt_a_s_mm2: 999.0,
            new_bolt_e1_mm: 999.0,
            new_bolt_e2_mm: 999.0,
            new_bolt_d0_mm: 999.0,
            new_bolt_d_mm: 999.0,
            new_bolt_t_mm: 999.0,
            new_bolt_f_u_mpa: 999.0,
            new_bolt_f_ub_mpa: 999.0,
        }),
        En1993Mutation::UpdateWeldInputs(UpdateWeldInputs { new_weld_a_mm: 999.0, new_weld_l_mm: 999.0, new_weld_f_u_mpa: 999.0, new_weld_steel_grade: "changed".to_string(), new_weld_f_ed_kn: 999.0 }),
        En1993Mutation::UpdateFatigueInputs(UpdateFatigueInputs { new_delta_sigma_mpa: 999.0, new_fatigue_category: 9, new_fatigue_method: "changed".to_string() }),
        En1993Mutation::UpdateThroughThicknessInputs(UpdateThroughThicknessInputs { new_t10_steel_subgrade: "changed".to_string(), new_t10_actual_thickness_mm: 999.0, new_t10_t_ed_c: 999.0 }),
        En1993Mutation::UpdateTensionComponentInputs(UpdateTensionComponentInputs { new_tension_component_f_uk_kn: 999.0, new_tension_component_f_k_kn: 999.0, new_tension_component_n_ed_kn: 999.0 }),
        En1993Mutation::UpdateHssInputs(UpdateHssInputs { new_hss_w_el_mm3: 999.0, new_hss_f_y_mpa: 999.0, new_hss_section_class: 9, new_hss_m_ed_knm: 999.0 }),
        En1993Mutation::UpdateBridgeInputs(UpdateBridgeInputs { new_bridge_lambda: 999.0, new_bridge_phi_2: 999.0, new_bridge_delta_sigma_p_mpa: 999.0 }),
        En1993Mutation::UpdateTowerInputs(UpdateTowerInputs { new_tower_wind_factor: 999.0, new_tower_n_ed_kn: 999.0 }),
        En1993Mutation::UpdatePileInputs(UpdatePileInputs { new_pile_sigma_mpa: 999.0, new_pile_k_red: 999.0, new_pile_n_ed_kn: 999.0 }),
        En1993Mutation::UpdateCraneInputs(UpdateCraneInputs { new_crane_f_z_ed_kn: 999.0, new_crane_wheel_contact_length_mm: 999.0, new_crane_dispersion_mm: 999.0, new_crane_t_w_mm: 999.0 }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{OpBinary, OpText};

    #[semio_framework_async_macros::async_test]
    fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <En1993Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <En1993Mutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
