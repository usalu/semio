//! ⚡️ EN 1992 design of concrete structures — hand-rolled `OpText`/`OpBinary` for
//! `En1992Mutation`. `#[derive(dsl_derive::Mutations)]` only generates `Mutation`/
//! `SemanticMutation` (see `../🦀️.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary
//! codecs stay handcrafted here, one keyword per semantic verb, grammar
//! `change-<field> new-<field>=<value>`. The three enum-typed fields (`annex`, `fire_rating`,
//! `tightness_class`) round-trip through a quoted JSON string — they already derive
//! `Serialize`/`Deserialize`, so a second handcrafted grammar per enum would just duplicate that
//! losslessly.

pub use crate::artifacts::en1992::schema::mutations::En1992Mutation;

use crate::artifacts::en1992::schema::mutations::set_snapshot::ChangeAnnex;
use crate::artifacts::en1992::schema::mutations::{
    change_a_c_mm2::ChangeACMm2, change_a_s_mm2::ChangeASMm2, change_anchor_a_s_mm2::ChangeAnchorASMm2, change_anchor_c1_mm::ChangeAnchorC1Mm, change_anchor_cracked::ChangeAnchorCracked,
    change_anchor_d_mm::ChangeAnchorDMm, change_anchor_f_uk_mpa::ChangeAnchorFUkMpa, change_anchor_f_yk_mpa::ChangeAnchorFYkMpa, change_anchor_h_ef_mm::ChangeAnchorHEfMm,
    change_anchor_n_ed_kn::ChangeAnchorNEdKn, change_anchor_v_ed_kn::ChangeAnchorVEdKn, change_b_mm::ChangeBMm, change_bridge_delta_sigma_s_mpa::ChangeBridgeDeltaSigmaSMpa,
    change_bridge_sigma_c_mpa::ChangeBridgeSigmaCMpa, change_d_mm::ChangeDMm, change_f_ck::ChangeFCk, change_f_yk::ChangeFYk, change_fire_rating::ChangeFireRating,
    change_hd_over_h::ChangeHdOverH, change_liquid_e_s_mpa::ChangeLiquidESMpa, change_liquid_f_ct_eff_mpa::ChangeLiquidFCtEffMpa, change_liquid_rho_p_eff::ChangeLiquidRhoPEff,
    change_liquid_s_r_max_mm::ChangeLiquidSRMaxMm, change_liquid_sigma_s_mpa::ChangeLiquidSigmaSMpa, change_m_ed_knm::ChangeMEdKnm, change_n_ed_kn::ChangeNEdKn, change_p_kn::ChangePKn,
    change_provided_axis_distance_mm::ChangeProvidedAxisDistanceMm, change_rho_l::ChangeRhoL, change_span_m::ChangeSpanM, change_tightness_class::ChangeTightnessClass, change_udl_kn_m::ChangeUdlKnM,
    change_use_fem::ChangeUseFem, change_v_ed_kn::ChangeVEdKn,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, used to
/// wrap the JSON form of the three enum-typed fields.
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
fn enc_json<T: serde::Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("en1992 mutation payload field always serializes"))
}
fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
fn dec_bool(s: &str) -> Result<bool, String> {
    s.parse().map_err(|e: std::str::ParseBoolError| e.to_string())
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value.
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
fn print_en1992_mutation(mutation: &En1992Mutation) -> String {
    match mutation {
        En1992Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1992Mutation::ChangeMEdKnm(p) => format!("change-m-ed-knm new-m_ed_knm={}", p.new_m_ed_knm),
        En1992Mutation::ChangeVEdKn(p) => format!("change-v-ed-kn new-v_ed_kn={}", p.new_v_ed_kn),
        En1992Mutation::ChangeFCk(p) => format!("change-f-ck new-f_ck={}", p.new_f_ck),
        En1992Mutation::ChangeBMm(p) => format!("change-b-mm new-b_mm={}", p.new_b_mm),
        En1992Mutation::ChangeDMm(p) => format!("change-d-mm new-d_mm={}", p.new_d_mm),
        En1992Mutation::ChangeASMm2(p) => format!("change-as-mm2 new-a_s_mm2={}", p.new_a_s_mm2),
        En1992Mutation::ChangeFYk(p) => format!("change-f-yk new-f_yk={}", p.new_f_yk),
        En1992Mutation::ChangeRhoL(p) => format!("change-rho-l new-rho_l={}", p.new_rho_l),
        En1992Mutation::ChangeNEdKn(p) => format!("change-n-ed-kn new-n_ed_kn={}", p.new_n_ed_kn),
        En1992Mutation::ChangePKn(p) => format!("change-p-kn new-p_kn={}", p.new_p_kn),
        En1992Mutation::ChangeACMm2(p) => format!("change-ac-mm2 new-a_c_mm2={}", p.new_a_c_mm2),
        En1992Mutation::ChangeUseFem(p) => format!("change-use-fem new-use_fem={}", p.new_use_fem),
        En1992Mutation::ChangeSpanM(p) => format!("change-span-m new-span_m={}", p.new_span_m),
        En1992Mutation::ChangeUdlKnM(p) => format!("change-udl-kn-m new-udl_kn_m={}", p.new_udl_kn_m),
        En1992Mutation::ChangeFireRating(p) => format!("change-fire-rating new-fire_rating={}", enc_json(&p.new_fire_rating)),
        En1992Mutation::ChangeProvidedAxisDistanceMm(p) => format!("change-provided-axis-distance-mm new-provided_axis_distance_mm={}", p.new_provided_axis_distance_mm),
        En1992Mutation::ChangeBridgeSigmaCMpa(p) => format!("change-bridge-sigma-c-mpa new-bridge_sigma_c_mpa={}", p.new_bridge_sigma_c_mpa),
        En1992Mutation::ChangeBridgeDeltaSigmaSMpa(p) => format!("change-bridge-delta-sigma-s-mpa new-bridge_delta_sigma_s_mpa={}", p.new_bridge_delta_sigma_s_mpa),
        En1992Mutation::ChangeTightnessClass(p) => format!("change-tightness-class new-tightness_class={}", enc_json(&p.new_tightness_class)),
        En1992Mutation::ChangeHdOverH(p) => format!("change-hd-over-h new-hd_over_h={}", p.new_hd_over_h),
        En1992Mutation::ChangeLiquidSigmaSMpa(p) => format!("change-liquid-sigma-s-mpa new-liquid_sigma_s_mpa={}", p.new_liquid_sigma_s_mpa),
        En1992Mutation::ChangeLiquidRhoPEff(p) => format!("change-liquid-rho-p-eff new-liquid_rho_p_eff={}", p.new_liquid_rho_p_eff),
        En1992Mutation::ChangeLiquidFCtEffMpa(p) => format!("change-liquid-f-ct-eff-mpa new-liquid_f_ct_eff_mpa={}", p.new_liquid_f_ct_eff_mpa),
        En1992Mutation::ChangeLiquidESMpa(p) => format!("change-liquid-es-mpa new-liquid_e_s_mpa={}", p.new_liquid_e_s_mpa),
        En1992Mutation::ChangeLiquidSRMaxMm(p) => format!("change-liquid-sr-max-mm new-liquid_s_r_max_mm={}", p.new_liquid_s_r_max_mm),
        En1992Mutation::ChangeAnchorHEfMm(p) => format!("change-anchor-h-ef-mm new-anchor_h_ef_mm={}", p.new_anchor_h_ef_mm),
        En1992Mutation::ChangeAnchorCracked(p) => format!("change-anchor-cracked new-anchor_cracked={}", p.new_anchor_cracked),
        En1992Mutation::ChangeAnchorFUkMpa(p) => format!("change-anchor-f-uk-mpa new-anchor_f_uk_mpa={}", p.new_anchor_f_uk_mpa),
        En1992Mutation::ChangeAnchorFYkMpa(p) => format!("change-anchor-f-yk-mpa new-anchor_f_yk_mpa={}", p.new_anchor_f_yk_mpa),
        En1992Mutation::ChangeAnchorASMm2(p) => format!("change-anchor-as-mm2 new-anchor_a_s_mm2={}", p.new_anchor_a_s_mm2),
        En1992Mutation::ChangeAnchorDMm(p) => format!("change-anchor-d-mm new-anchor_d_mm={}", p.new_anchor_d_mm),
        En1992Mutation::ChangeAnchorC1Mm(p) => format!("change-anchor-c1-mm new-anchor_c1_mm={}", p.new_anchor_c1_mm),
        En1992Mutation::ChangeAnchorNEdKn(p) => format!("change-anchor-n-ed-kn new-anchor_n_ed_kn={}", p.new_anchor_n_ed_kn),
        En1992Mutation::ChangeAnchorVEdKn(p) => format!("change-anchor-v-ed-kn new-anchor_v_ed_kn={}", p.new_anchor_v_ed_kn),
    }
}

fn parse_en1992_mutation(line: &str) -> Result<En1992Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("en1992 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-annex" => Ok(En1992Mutation::ChangeAnnex(ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })),
        "change-m-ed-knm" => Ok(En1992Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: dec_f64(&arg("new-m_ed_knm")?)? })),
        "change-v-ed-kn" => Ok(En1992Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: dec_f64(&arg("new-v_ed_kn")?)? })),
        "change-f-ck" => Ok(En1992Mutation::ChangeFCk(ChangeFCk { new_f_ck: dec_f64(&arg("new-f_ck")?)? })),
        "change-b-mm" => Ok(En1992Mutation::ChangeBMm(ChangeBMm { new_b_mm: dec_f64(&arg("new-b_mm")?)? })),
        "change-d-mm" => Ok(En1992Mutation::ChangeDMm(ChangeDMm { new_d_mm: dec_f64(&arg("new-d_mm")?)? })),
        "change-as-mm2" => Ok(En1992Mutation::ChangeASMm2(ChangeASMm2 { new_a_s_mm2: dec_f64(&arg("new-a_s_mm2")?)? })),
        "change-f-yk" => Ok(En1992Mutation::ChangeFYk(ChangeFYk { new_f_yk: dec_f64(&arg("new-f_yk")?)? })),
        "change-rho-l" => Ok(En1992Mutation::ChangeRhoL(ChangeRhoL { new_rho_l: dec_f64(&arg("new-rho_l")?)? })),
        "change-n-ed-kn" => Ok(En1992Mutation::ChangeNEdKn(ChangeNEdKn { new_n_ed_kn: dec_f64(&arg("new-n_ed_kn")?)? })),
        "change-p-kn" => Ok(En1992Mutation::ChangePKn(ChangePKn { new_p_kn: dec_f64(&arg("new-p_kn")?)? })),
        "change-ac-mm2" => Ok(En1992Mutation::ChangeACMm2(ChangeACMm2 { new_a_c_mm2: dec_f64(&arg("new-a_c_mm2")?)? })),
        "change-use-fem" => Ok(En1992Mutation::ChangeUseFem(ChangeUseFem { new_use_fem: dec_bool(&arg("new-use_fem")?)? })),
        "change-span-m" => Ok(En1992Mutation::ChangeSpanM(ChangeSpanM { new_span_m: dec_f64(&arg("new-span_m")?)? })),
        "change-udl-kn-m" => Ok(En1992Mutation::ChangeUdlKnM(ChangeUdlKnM { new_udl_kn_m: dec_f64(&arg("new-udl_kn_m")?)? })),
        "change-fire-rating" => Ok(En1992Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: dec_json(&arg("new-fire_rating")?)? })),
        "change-provided-axis-distance-mm" => Ok(En1992Mutation::ChangeProvidedAxisDistanceMm(ChangeProvidedAxisDistanceMm { new_provided_axis_distance_mm: dec_f64(&arg("new-provided_axis_distance_mm")?)? })),
        "change-bridge-sigma-c-mpa" => Ok(En1992Mutation::ChangeBridgeSigmaCMpa(ChangeBridgeSigmaCMpa { new_bridge_sigma_c_mpa: dec_f64(&arg("new-bridge_sigma_c_mpa")?)? })),
        "change-bridge-delta-sigma-s-mpa" => Ok(En1992Mutation::ChangeBridgeDeltaSigmaSMpa(ChangeBridgeDeltaSigmaSMpa { new_bridge_delta_sigma_s_mpa: dec_f64(&arg("new-bridge_delta_sigma_s_mpa")?)? })),
        "change-tightness-class" => Ok(En1992Mutation::ChangeTightnessClass(ChangeTightnessClass { new_tightness_class: dec_json(&arg("new-tightness_class")?)? })),
        "change-hd-over-h" => Ok(En1992Mutation::ChangeHdOverH(ChangeHdOverH { new_hd_over_h: dec_f64(&arg("new-hd_over_h")?)? })),
        "change-liquid-sigma-s-mpa" => Ok(En1992Mutation::ChangeLiquidSigmaSMpa(ChangeLiquidSigmaSMpa { new_liquid_sigma_s_mpa: dec_f64(&arg("new-liquid_sigma_s_mpa")?)? })),
        "change-liquid-rho-p-eff" => Ok(En1992Mutation::ChangeLiquidRhoPEff(ChangeLiquidRhoPEff { new_liquid_rho_p_eff: dec_f64(&arg("new-liquid_rho_p_eff")?)? })),
        "change-liquid-f-ct-eff-mpa" => Ok(En1992Mutation::ChangeLiquidFCtEffMpa(ChangeLiquidFCtEffMpa { new_liquid_f_ct_eff_mpa: dec_f64(&arg("new-liquid_f_ct_eff_mpa")?)? })),
        "change-liquid-es-mpa" => Ok(En1992Mutation::ChangeLiquidESMpa(ChangeLiquidESMpa { new_liquid_e_s_mpa: dec_f64(&arg("new-liquid_e_s_mpa")?)? })),
        "change-liquid-sr-max-mm" => Ok(En1992Mutation::ChangeLiquidSRMaxMm(ChangeLiquidSRMaxMm { new_liquid_s_r_max_mm: dec_f64(&arg("new-liquid_s_r_max_mm")?)? })),
        "change-anchor-h-ef-mm" => Ok(En1992Mutation::ChangeAnchorHEfMm(ChangeAnchorHEfMm { new_anchor_h_ef_mm: dec_f64(&arg("new-anchor_h_ef_mm")?)? })),
        "change-anchor-cracked" => Ok(En1992Mutation::ChangeAnchorCracked(ChangeAnchorCracked { new_anchor_cracked: dec_bool(&arg("new-anchor_cracked")?)? })),
        "change-anchor-f-uk-mpa" => Ok(En1992Mutation::ChangeAnchorFUkMpa(ChangeAnchorFUkMpa { new_anchor_f_uk_mpa: dec_f64(&arg("new-anchor_f_uk_mpa")?)? })),
        "change-anchor-f-yk-mpa" => Ok(En1992Mutation::ChangeAnchorFYkMpa(ChangeAnchorFYkMpa { new_anchor_f_yk_mpa: dec_f64(&arg("new-anchor_f_yk_mpa")?)? })),
        "change-anchor-as-mm2" => Ok(En1992Mutation::ChangeAnchorASMm2(ChangeAnchorASMm2 { new_anchor_a_s_mm2: dec_f64(&arg("new-anchor_a_s_mm2")?)? })),
        "change-anchor-d-mm" => Ok(En1992Mutation::ChangeAnchorDMm(ChangeAnchorDMm { new_anchor_d_mm: dec_f64(&arg("new-anchor_d_mm")?)? })),
        "change-anchor-c1-mm" => Ok(En1992Mutation::ChangeAnchorC1Mm(ChangeAnchorC1Mm { new_anchor_c1_mm: dec_f64(&arg("new-anchor_c1_mm")?)? })),
        "change-anchor-n-ed-kn" => Ok(En1992Mutation::ChangeAnchorNEdKn(ChangeAnchorNEdKn { new_anchor_n_ed_kn: dec_f64(&arg("new-anchor_n_ed_kn")?)? })),
        "change-anchor-v-ed-kn" => Ok(En1992Mutation::ChangeAnchorVEdKn(ChangeAnchorVEdKn { new_anchor_v_ed_kn: dec_f64(&arg("new-anchor_v_ed_kn")?)? })),
        other => Err(format!("en1992 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for En1992Mutation {
    fn print_op(&self) -> String {
        print_en1992_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_en1992_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | value`; scalar fields write their native binary
/// form directly, the three enum-typed fields go through the same JSON bridge as `OpText` above.
fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}
fn write_json_bin<T: serde::Serialize>(out: &mut Vec<u8>, value: &T) {
    write_str_bin(out, &serde_json::to_string(value).expect("en1992 mutation payload field always serializes"));
}
fn read_json_bin<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    serde_json::from_str(&read_str_bin(reader)?).map_err(|e| e.to_string())
}
fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}
fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}
fn write_bool_bin(out: &mut Vec<u8>, v: bool) {
    out.push(if v { 1 } else { 0 });
}
fn read_bool_bin(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}

impl protocol::OpBinary for En1992Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1992Mutation::ChangeAnnex(_) => 0,
            En1992Mutation::ChangeMEdKnm(_) => 1,
            En1992Mutation::ChangeVEdKn(_) => 2,
            En1992Mutation::ChangeFCk(_) => 3,
            En1992Mutation::ChangeBMm(_) => 4,
            En1992Mutation::ChangeDMm(_) => 5,
            En1992Mutation::ChangeASMm2(_) => 6,
            En1992Mutation::ChangeFYk(_) => 7,
            En1992Mutation::ChangeRhoL(_) => 8,
            En1992Mutation::ChangeNEdKn(_) => 9,
            En1992Mutation::ChangePKn(_) => 10,
            En1992Mutation::ChangeACMm2(_) => 11,
            En1992Mutation::ChangeUseFem(_) => 12,
            En1992Mutation::ChangeSpanM(_) => 13,
            En1992Mutation::ChangeUdlKnM(_) => 14,
            En1992Mutation::ChangeFireRating(_) => 15,
            En1992Mutation::ChangeProvidedAxisDistanceMm(_) => 16,
            En1992Mutation::ChangeBridgeSigmaCMpa(_) => 17,
            En1992Mutation::ChangeBridgeDeltaSigmaSMpa(_) => 18,
            En1992Mutation::ChangeTightnessClass(_) => 19,
            En1992Mutation::ChangeHdOverH(_) => 20,
            En1992Mutation::ChangeLiquidSigmaSMpa(_) => 21,
            En1992Mutation::ChangeLiquidRhoPEff(_) => 22,
            En1992Mutation::ChangeLiquidFCtEffMpa(_) => 23,
            En1992Mutation::ChangeLiquidESMpa(_) => 24,
            En1992Mutation::ChangeLiquidSRMaxMm(_) => 25,
            En1992Mutation::ChangeAnchorHEfMm(_) => 26,
            En1992Mutation::ChangeAnchorCracked(_) => 27,
            En1992Mutation::ChangeAnchorFUkMpa(_) => 28,
            En1992Mutation::ChangeAnchorFYkMpa(_) => 29,
            En1992Mutation::ChangeAnchorASMm2(_) => 30,
            En1992Mutation::ChangeAnchorDMm(_) => 31,
            En1992Mutation::ChangeAnchorC1Mm(_) => 32,
            En1992Mutation::ChangeAnchorNEdKn(_) => 33,
            En1992Mutation::ChangeAnchorVEdKn(_) => 34,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            En1992Mutation::ChangeAnnex(p) => write_json_bin(&mut out, &p.new_annex),
            En1992Mutation::ChangeMEdKnm(p) => write_f64_bin(&mut out, p.new_m_ed_knm),
            En1992Mutation::ChangeVEdKn(p) => write_f64_bin(&mut out, p.new_v_ed_kn),
            En1992Mutation::ChangeFCk(p) => write_f64_bin(&mut out, p.new_f_ck),
            En1992Mutation::ChangeBMm(p) => write_f64_bin(&mut out, p.new_b_mm),
            En1992Mutation::ChangeDMm(p) => write_f64_bin(&mut out, p.new_d_mm),
            En1992Mutation::ChangeASMm2(p) => write_f64_bin(&mut out, p.new_a_s_mm2),
            En1992Mutation::ChangeFYk(p) => write_f64_bin(&mut out, p.new_f_yk),
            En1992Mutation::ChangeRhoL(p) => write_f64_bin(&mut out, p.new_rho_l),
            En1992Mutation::ChangeNEdKn(p) => write_f64_bin(&mut out, p.new_n_ed_kn),
            En1992Mutation::ChangePKn(p) => write_f64_bin(&mut out, p.new_p_kn),
            En1992Mutation::ChangeACMm2(p) => write_f64_bin(&mut out, p.new_a_c_mm2),
            En1992Mutation::ChangeUseFem(p) => write_bool_bin(&mut out, p.new_use_fem),
            En1992Mutation::ChangeSpanM(p) => write_f64_bin(&mut out, p.new_span_m),
            En1992Mutation::ChangeUdlKnM(p) => write_f64_bin(&mut out, p.new_udl_kn_m),
            En1992Mutation::ChangeFireRating(p) => write_json_bin(&mut out, &p.new_fire_rating),
            En1992Mutation::ChangeProvidedAxisDistanceMm(p) => write_f64_bin(&mut out, p.new_provided_axis_distance_mm),
            En1992Mutation::ChangeBridgeSigmaCMpa(p) => write_f64_bin(&mut out, p.new_bridge_sigma_c_mpa),
            En1992Mutation::ChangeBridgeDeltaSigmaSMpa(p) => write_f64_bin(&mut out, p.new_bridge_delta_sigma_s_mpa),
            En1992Mutation::ChangeTightnessClass(p) => write_json_bin(&mut out, &p.new_tightness_class),
            En1992Mutation::ChangeHdOverH(p) => write_f64_bin(&mut out, p.new_hd_over_h),
            En1992Mutation::ChangeLiquidSigmaSMpa(p) => write_f64_bin(&mut out, p.new_liquid_sigma_s_mpa),
            En1992Mutation::ChangeLiquidRhoPEff(p) => write_f64_bin(&mut out, p.new_liquid_rho_p_eff),
            En1992Mutation::ChangeLiquidFCtEffMpa(p) => write_f64_bin(&mut out, p.new_liquid_f_ct_eff_mpa),
            En1992Mutation::ChangeLiquidESMpa(p) => write_f64_bin(&mut out, p.new_liquid_e_s_mpa),
            En1992Mutation::ChangeLiquidSRMaxMm(p) => write_f64_bin(&mut out, p.new_liquid_s_r_max_mm),
            En1992Mutation::ChangeAnchorHEfMm(p) => write_f64_bin(&mut out, p.new_anchor_h_ef_mm),
            En1992Mutation::ChangeAnchorCracked(p) => write_bool_bin(&mut out, p.new_anchor_cracked),
            En1992Mutation::ChangeAnchorFUkMpa(p) => write_f64_bin(&mut out, p.new_anchor_f_uk_mpa),
            En1992Mutation::ChangeAnchorFYkMpa(p) => write_f64_bin(&mut out, p.new_anchor_f_yk_mpa),
            En1992Mutation::ChangeAnchorASMm2(p) => write_f64_bin(&mut out, p.new_anchor_a_s_mm2),
            En1992Mutation::ChangeAnchorDMm(p) => write_f64_bin(&mut out, p.new_anchor_d_mm),
            En1992Mutation::ChangeAnchorC1Mm(p) => write_f64_bin(&mut out, p.new_anchor_c1_mm),
            En1992Mutation::ChangeAnchorNEdKn(p) => write_f64_bin(&mut out, p.new_anchor_n_ed_kn),
            En1992Mutation::ChangeAnchorVEdKn(p) => write_f64_bin(&mut out, p.new_anchor_v_ed_kn),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(En1992Mutation::ChangeAnnex(ChangeAnnex { new_annex: read_json_bin(&mut reader).map_err(|e| malformed("new_annex", reader.position(), e))? })),
            1 => Ok(En1992Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: read_f64_bin(&mut reader).map_err(|e| malformed("new_m_ed_knm", reader.position(), e))? })),
            2 => Ok(En1992Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: read_f64_bin(&mut reader).map_err(|e| malformed("new_v_ed_kn", reader.position(), e))? })),
            3 => Ok(En1992Mutation::ChangeFCk(ChangeFCk { new_f_ck: read_f64_bin(&mut reader).map_err(|e| malformed("new_f_ck", reader.position(), e))? })),
            4 => Ok(En1992Mutation::ChangeBMm(ChangeBMm { new_b_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_b_mm", reader.position(), e))? })),
            5 => Ok(En1992Mutation::ChangeDMm(ChangeDMm { new_d_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_d_mm", reader.position(), e))? })),
            6 => Ok(En1992Mutation::ChangeASMm2(ChangeASMm2 { new_a_s_mm2: read_f64_bin(&mut reader).map_err(|e| malformed("new_a_s_mm2", reader.position(), e))? })),
            7 => Ok(En1992Mutation::ChangeFYk(ChangeFYk { new_f_yk: read_f64_bin(&mut reader).map_err(|e| malformed("new_f_yk", reader.position(), e))? })),
            8 => Ok(En1992Mutation::ChangeRhoL(ChangeRhoL { new_rho_l: read_f64_bin(&mut reader).map_err(|e| malformed("new_rho_l", reader.position(), e))? })),
            9 => Ok(En1992Mutation::ChangeNEdKn(ChangeNEdKn { new_n_ed_kn: read_f64_bin(&mut reader).map_err(|e| malformed("new_n_ed_kn", reader.position(), e))? })),
            10 => Ok(En1992Mutation::ChangePKn(ChangePKn { new_p_kn: read_f64_bin(&mut reader).map_err(|e| malformed("new_p_kn", reader.position(), e))? })),
            11 => Ok(En1992Mutation::ChangeACMm2(ChangeACMm2 { new_a_c_mm2: read_f64_bin(&mut reader).map_err(|e| malformed("new_a_c_mm2", reader.position(), e))? })),
            12 => Ok(En1992Mutation::ChangeUseFem(ChangeUseFem { new_use_fem: read_bool_bin(&mut reader).map_err(|e| malformed("new_use_fem", reader.position(), e))? })),
            13 => Ok(En1992Mutation::ChangeSpanM(ChangeSpanM { new_span_m: read_f64_bin(&mut reader).map_err(|e| malformed("new_span_m", reader.position(), e))? })),
            14 => Ok(En1992Mutation::ChangeUdlKnM(ChangeUdlKnM { new_udl_kn_m: read_f64_bin(&mut reader).map_err(|e| malformed("new_udl_kn_m", reader.position(), e))? })),
            15 => Ok(En1992Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: read_json_bin(&mut reader).map_err(|e| malformed("new_fire_rating", reader.position(), e))? })),
            16 => Ok(En1992Mutation::ChangeProvidedAxisDistanceMm(ChangeProvidedAxisDistanceMm { new_provided_axis_distance_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_provided_axis_distance_mm", reader.position(), e))? })),
            17 => Ok(En1992Mutation::ChangeBridgeSigmaCMpa(ChangeBridgeSigmaCMpa { new_bridge_sigma_c_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_bridge_sigma_c_mpa", reader.position(), e))? })),
            18 => Ok(En1992Mutation::ChangeBridgeDeltaSigmaSMpa(ChangeBridgeDeltaSigmaSMpa { new_bridge_delta_sigma_s_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_bridge_delta_sigma_s_mpa", reader.position(), e))? })),
            19 => Ok(En1992Mutation::ChangeTightnessClass(ChangeTightnessClass { new_tightness_class: read_json_bin(&mut reader).map_err(|e| malformed("new_tightness_class", reader.position(), e))? })),
            20 => Ok(En1992Mutation::ChangeHdOverH(ChangeHdOverH { new_hd_over_h: read_f64_bin(&mut reader).map_err(|e| malformed("new_hd_over_h", reader.position(), e))? })),
            21 => Ok(En1992Mutation::ChangeLiquidSigmaSMpa(ChangeLiquidSigmaSMpa { new_liquid_sigma_s_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_liquid_sigma_s_mpa", reader.position(), e))? })),
            22 => Ok(En1992Mutation::ChangeLiquidRhoPEff(ChangeLiquidRhoPEff { new_liquid_rho_p_eff: read_f64_bin(&mut reader).map_err(|e| malformed("new_liquid_rho_p_eff", reader.position(), e))? })),
            23 => Ok(En1992Mutation::ChangeLiquidFCtEffMpa(ChangeLiquidFCtEffMpa { new_liquid_f_ct_eff_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_liquid_f_ct_eff_mpa", reader.position(), e))? })),
            24 => Ok(En1992Mutation::ChangeLiquidESMpa(ChangeLiquidESMpa { new_liquid_e_s_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_liquid_e_s_mpa", reader.position(), e))? })),
            25 => Ok(En1992Mutation::ChangeLiquidSRMaxMm(ChangeLiquidSRMaxMm { new_liquid_s_r_max_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_liquid_s_r_max_mm", reader.position(), e))? })),
            26 => Ok(En1992Mutation::ChangeAnchorHEfMm(ChangeAnchorHEfMm { new_anchor_h_ef_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_anchor_h_ef_mm", reader.position(), e))? })),
            27 => Ok(En1992Mutation::ChangeAnchorCracked(ChangeAnchorCracked { new_anchor_cracked: read_bool_bin(&mut reader).map_err(|e| malformed("new_anchor_cracked", reader.position(), e))? })),
            28 => Ok(En1992Mutation::ChangeAnchorFUkMpa(ChangeAnchorFUkMpa { new_anchor_f_uk_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_anchor_f_uk_mpa", reader.position(), e))? })),
            29 => Ok(En1992Mutation::ChangeAnchorFYkMpa(ChangeAnchorFYkMpa { new_anchor_f_yk_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_anchor_f_yk_mpa", reader.position(), e))? })),
            30 => Ok(En1992Mutation::ChangeAnchorASMm2(ChangeAnchorASMm2 { new_anchor_a_s_mm2: read_f64_bin(&mut reader).map_err(|e| malformed("new_anchor_a_s_mm2", reader.position(), e))? })),
            31 => Ok(En1992Mutation::ChangeAnchorDMm(ChangeAnchorDMm { new_anchor_d_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_anchor_d_mm", reader.position(), e))? })),
            32 => Ok(En1992Mutation::ChangeAnchorC1Mm(ChangeAnchorC1Mm { new_anchor_c1_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_anchor_c1_mm", reader.position(), e))? })),
            33 => Ok(En1992Mutation::ChangeAnchorNEdKn(ChangeAnchorNEdKn { new_anchor_n_ed_kn: read_f64_bin(&mut reader).map_err(|e| malformed("new_anchor_n_ed_kn", reader.position(), e))? })),
            34 => Ok(En1992Mutation::ChangeAnchorVEdKn(ChangeAnchorVEdKn { new_anchor_v_ed_kn: read_f64_bin(&mut reader).map_err(|e| malformed("new_anchor_v_ed_kn", reader.position(), e))? })),
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1992Mutation> {
    vec![
        En1992Mutation::ChangeAnnex(ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1992Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: 150.0 }),
        En1992Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: 95.0 }),
        En1992Mutation::ChangeFCk(ChangeFCk { new_f_ck: 35.0 }),
        En1992Mutation::ChangeBMm(ChangeBMm { new_b_mm: 350.0 }),
        En1992Mutation::ChangeDMm(ChangeDMm { new_d_mm: 500.0 }),
        En1992Mutation::ChangeASMm2(ChangeASMm2 { new_a_s_mm2: 1400.0 }),
        En1992Mutation::ChangeFYk(ChangeFYk { new_f_yk: 550.0 }),
        En1992Mutation::ChangeRhoL(ChangeRhoL { new_rho_l: 0.015 }),
        En1992Mutation::ChangeNEdKn(ChangeNEdKn { new_n_ed_kn: 25.0 }),
        En1992Mutation::ChangePKn(ChangePKn { new_p_kn: 50.0 }),
        En1992Mutation::ChangeACMm2(ChangeACMm2 { new_a_c_mm2: 150000.0 }),
        En1992Mutation::ChangeUseFem(ChangeUseFem { new_use_fem: true }),
        En1992Mutation::ChangeSpanM(ChangeSpanM { new_span_m: 7.5 }),
        En1992Mutation::ChangeUdlKnM(ChangeUdlKnM { new_udl_kn_m: 24.0 }),
        En1992Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: crate::artifacts::en1992::part_1_2::FireRating::R90 }),
        En1992Mutation::ChangeProvidedAxisDistanceMm(ChangeProvidedAxisDistanceMm { new_provided_axis_distance_mm: 40.0 }),
        En1992Mutation::ChangeBridgeSigmaCMpa(ChangeBridgeSigmaCMpa { new_bridge_sigma_c_mpa: 14.0 }),
        En1992Mutation::ChangeBridgeDeltaSigmaSMpa(ChangeBridgeDeltaSigmaSMpa { new_bridge_delta_sigma_s_mpa: 120.0 }),
        En1992Mutation::ChangeTightnessClass(ChangeTightnessClass { new_tightness_class: crate::artifacts::en1992::part_3::TightnessClass::Tc2 }),
        En1992Mutation::ChangeHdOverH(ChangeHdOverH { new_hd_over_h: 12.0 }),
        En1992Mutation::ChangeLiquidSigmaSMpa(ChangeLiquidSigmaSMpa { new_liquid_sigma_s_mpa: 220.0 }),
        En1992Mutation::ChangeLiquidRhoPEff(ChangeLiquidRhoPEff { new_liquid_rho_p_eff: 0.012 }),
        En1992Mutation::ChangeLiquidFCtEffMpa(ChangeLiquidFCtEffMpa { new_liquid_f_ct_eff_mpa: 3.1 }),
        En1992Mutation::ChangeLiquidESMpa(ChangeLiquidESMpa { new_liquid_e_s_mpa: 205000.0 }),
        En1992Mutation::ChangeLiquidSRMaxMm(ChangeLiquidSRMaxMm { new_liquid_s_r_max_mm: 275.0 }),
        En1992Mutation::ChangeAnchorHEfMm(ChangeAnchorHEfMm { new_anchor_h_ef_mm: 90.0 }),
        En1992Mutation::ChangeAnchorCracked(ChangeAnchorCracked { new_anchor_cracked: true }),
        En1992Mutation::ChangeAnchorFUkMpa(ChangeAnchorFUkMpa { new_anchor_f_uk_mpa: 850.0 }),
        En1992Mutation::ChangeAnchorFYkMpa(ChangeAnchorFYkMpa { new_anchor_f_yk_mpa: 680.0 }),
        En1992Mutation::ChangeAnchorASMm2(ChangeAnchorASMm2 { new_anchor_a_s_mm2: 94.3 }),
        En1992Mutation::ChangeAnchorDMm(ChangeAnchorDMm { new_anchor_d_mm: 14.0 }),
        En1992Mutation::ChangeAnchorC1Mm(ChangeAnchorC1Mm { new_anchor_c1_mm: 120.0 }),
        En1992Mutation::ChangeAnchorNEdKn(ChangeAnchorNEdKn { new_anchor_n_ed_kn: 15.0 }),
        En1992Mutation::ChangeAnchorVEdKn(ChangeAnchorVEdKn { new_anchor_v_ed_kn: 8.0 }),
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
            let parsed = <En1992Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <En1992Mutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
