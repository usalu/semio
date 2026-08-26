//! ⚡️ En1994 artifact — hand-rolled `OpText`/`OpBinary` for `En1994Mutation`. Every field on this
//! artifact is a document-root scalar (no id-keyed or ordered collections), so each variant's wire
//! form is a single `keyword key=value` token — no JSON-per-field wrapping needed except for the
//! one enum-valued field (`annex`). `#[derive(dsl::Mutations)]` only generates `Mutation`/
//! `SemanticMutation` (see `../🦀️component.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary
//! codecs stay handcrafted here.

pub use crate::artifacts::en1994::schema::mutations::En1994Mutation;

use crate::artifacts::en1994::schema::mutations::{
    change_annex::mutation::ChangeAnnex, change_d_mm::mutation::ChangeDMm, change_deck_type::mutation::ChangeDeckType, change_delta_sigma_mpa::mutation::ChangeDeltaSigmaMpa, change_delta_tau_stud_mpa::mutation::ChangeDeltaTauStudMpa,
    change_e_cm_mpa::mutation::ChangeECmMpa, change_eta::mutation::ChangeEta, change_f_ck_mpa::mutation::ChangeFCkMpa, change_f_u_mpa::mutation::ChangeFUMpa, change_f_y_mpa::mutation::ChangeFYMpa,
    change_fatigue_detail::mutation::ChangeFatigueDetail, change_fire_rating::mutation::ChangeFireRating, change_h_sc_mm::mutation::ChangeHScMm, change_insulation_thickness_mm::mutation::ChangeInsulationThicknessMm,
    change_m_ed_knm::mutation::ChangeMEdKnm, change_m_pl_rd::mutation::ChangeMPlRd, change_m_pla::mutation::ChangeMPla, change_n_cycles_stud::mutation::ChangeNCyclesStud, change_span_m::mutation::ChangeSpanM, change_v_ed_kn::mutation::ChangeVEdKn,
    change_v_ed_per_stud_kn::mutation::ChangeVEdPerStudKn, change_v_l_rd::mutation::ChangeVLRd,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space.
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
fn enc_f64(v: f64) -> String {
    v.to_string()
}
fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
/// 🧬️ `annex` is the only non-scalar-primitive field (an `AnnexChoice` enum) — a quoted JSON
/// string reuses its existing `Serialize`/`Deserialize` losslessly instead of a bespoke grammar.
fn enc_json<T: serde::Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("en1994 mutation payload field always serializes"))
}
fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
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
fn print_en1994_mutation(mutation: &En1994Mutation) -> String {
    match mutation {
        En1994Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1994Mutation::ChangeMEdKnm(p) => format!("change-m-ed-knm new-m-ed-knm={}", enc_f64(p.new_m_ed_knm)),
        En1994Mutation::ChangeVEdKn(p) => format!("change-v-ed-kn new-v-ed-kn={}", enc_f64(p.new_v_ed_kn)),
        En1994Mutation::ChangeMPla(p) => format!("change-m-pla new-m-pla={}", enc_f64(p.new_m_pla)),
        En1994Mutation::ChangeMPlRd(p) => format!("change-m-pl-rd new-m-pl-rd={}", enc_f64(p.new_m_pl_rd)),
        En1994Mutation::ChangeEta(p) => format!("change-eta new-eta={}", enc_f64(p.new_eta)),
        En1994Mutation::ChangeVLRd(p) => format!("change-vl-rd new-v-l-rd={}", enc_f64(p.new_v_l_rd)),
        En1994Mutation::ChangeInsulationThicknessMm(p) => format!("change-insulation-thickness-mm new-insulation-thickness-mm={}", enc_f64(p.new_insulation_thickness_mm)),
        En1994Mutation::ChangeFireRating(p) => format!("change-fire-rating new-fire-rating={}", enc_str(&p.new_fire_rating)),
        En1994Mutation::ChangeDeckType(p) => format!("change-deck-type new-deck-type={}", enc_str(&p.new_deck_type)),
        En1994Mutation::ChangeDeltaSigmaMpa(p) => format!("change-delta-sigma-mpa new-delta-sigma-mpa={}", enc_f64(p.new_delta_sigma_mpa)),
        En1994Mutation::ChangeFatigueDetail(p) => format!("change-fatigue-detail new-fatigue-detail={}", enc_str(&p.new_fatigue_detail)),
        En1994Mutation::ChangeDMm(p) => format!("change-d-mm new-d-mm={}", enc_f64(p.new_d_mm)),
        En1994Mutation::ChangeHScMm(p) => format!("change-h-sc-mm new-h-sc-mm={}", enc_f64(p.new_h_sc_mm)),
        En1994Mutation::ChangeFCkMpa(p) => format!("change-f-ck-mpa new-f-ck-mpa={}", enc_f64(p.new_f_ck_mpa)),
        En1994Mutation::ChangeFUMpa(p) => format!("change-fu-mpa new-f-u-mpa={}", enc_f64(p.new_f_u_mpa)),
        En1994Mutation::ChangeECmMpa(p) => format!("change-e-cm-mpa new-e-cm-mpa={}", enc_f64(p.new_e_cm_mpa)),
        En1994Mutation::ChangeVEdPerStudKn(p) => format!("change-v-ed-per-stud-kn new-v-ed-per-stud-kn={}", enc_f64(p.new_v_ed_per_stud_kn)),
        En1994Mutation::ChangeSpanM(p) => format!("change-span-m new-span-m={}", enc_f64(p.new_span_m)),
        En1994Mutation::ChangeFYMpa(p) => format!("change-fy-mpa new-f-y-mpa={}", enc_f64(p.new_f_y_mpa)),
        En1994Mutation::ChangeNCyclesStud(p) => format!("change-n-cycles-stud new-n-cycles-stud={}", enc_f64(p.new_n_cycles_stud)),
        En1994Mutation::ChangeDeltaTauStudMpa(p) => format!("change-delta-tau-stud-mpa new-delta-tau-stud-mpa={}", enc_f64(p.new_delta_tau_stud_mpa)),
    }
}

fn parse_en1994_mutation(line: &str) -> Result<En1994Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("en1994 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-annex" => Ok(En1994Mutation::ChangeAnnex(ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })),
        "change-m-ed-knm" => Ok(En1994Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: dec_f64(&arg("new-m-ed-knm")?)? })),
        "change-v-ed-kn" => Ok(En1994Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: dec_f64(&arg("new-v-ed-kn")?)? })),
        "change-m-pla" => Ok(En1994Mutation::ChangeMPla(ChangeMPla { new_m_pla: dec_f64(&arg("new-m-pla")?)? })),
        "change-m-pl-rd" => Ok(En1994Mutation::ChangeMPlRd(ChangeMPlRd { new_m_pl_rd: dec_f64(&arg("new-m-pl-rd")?)? })),
        "change-eta" => Ok(En1994Mutation::ChangeEta(ChangeEta { new_eta: dec_f64(&arg("new-eta")?)? })),
        "change-vl-rd" => Ok(En1994Mutation::ChangeVLRd(ChangeVLRd { new_v_l_rd: dec_f64(&arg("new-v-l-rd")?)? })),
        "change-insulation-thickness-mm" => Ok(En1994Mutation::ChangeInsulationThicknessMm(ChangeInsulationThicknessMm { new_insulation_thickness_mm: dec_f64(&arg("new-insulation-thickness-mm")?)? })),
        "change-fire-rating" => Ok(En1994Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: dec_str(&arg("new-fire-rating")?)? })),
        "change-deck-type" => Ok(En1994Mutation::ChangeDeckType(ChangeDeckType { new_deck_type: dec_str(&arg("new-deck-type")?)? })),
        "change-delta-sigma-mpa" => Ok(En1994Mutation::ChangeDeltaSigmaMpa(ChangeDeltaSigmaMpa { new_delta_sigma_mpa: dec_f64(&arg("new-delta-sigma-mpa")?)? })),
        "change-fatigue-detail" => Ok(En1994Mutation::ChangeFatigueDetail(ChangeFatigueDetail { new_fatigue_detail: dec_str(&arg("new-fatigue-detail")?)? })),
        "change-d-mm" => Ok(En1994Mutation::ChangeDMm(ChangeDMm { new_d_mm: dec_f64(&arg("new-d-mm")?)? })),
        "change-h-sc-mm" => Ok(En1994Mutation::ChangeHScMm(ChangeHScMm { new_h_sc_mm: dec_f64(&arg("new-h-sc-mm")?)? })),
        "change-f-ck-mpa" => Ok(En1994Mutation::ChangeFCkMpa(ChangeFCkMpa { new_f_ck_mpa: dec_f64(&arg("new-f-ck-mpa")?)? })),
        "change-fu-mpa" => Ok(En1994Mutation::ChangeFUMpa(ChangeFUMpa { new_f_u_mpa: dec_f64(&arg("new-f-u-mpa")?)? })),
        "change-e-cm-mpa" => Ok(En1994Mutation::ChangeECmMpa(ChangeECmMpa { new_e_cm_mpa: dec_f64(&arg("new-e-cm-mpa")?)? })),
        "change-v-ed-per-stud-kn" => Ok(En1994Mutation::ChangeVEdPerStudKn(ChangeVEdPerStudKn { new_v_ed_per_stud_kn: dec_f64(&arg("new-v-ed-per-stud-kn")?)? })),
        "change-span-m" => Ok(En1994Mutation::ChangeSpanM(ChangeSpanM { new_span_m: dec_f64(&arg("new-span-m")?)? })),
        "change-fy-mpa" => Ok(En1994Mutation::ChangeFYMpa(ChangeFYMpa { new_f_y_mpa: dec_f64(&arg("new-f-y-mpa")?)? })),
        "change-n-cycles-stud" => Ok(En1994Mutation::ChangeNCyclesStud(ChangeNCyclesStud { new_n_cycles_stud: dec_f64(&arg("new-n-cycles-stud")?)? })),
        "change-delta-tau-stud-mpa" => Ok(En1994Mutation::ChangeDeltaTauStudMpa(ChangeDeltaTauStudMpa { new_delta_tau_stud_mpa: dec_f64(&arg("new-delta-tau-stud-mpa")?)? })),
        other => Err(format!("en1994 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for En1994Mutation {
    fn print_op(&self) -> String {
        print_en1994_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_en1994_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | one field encoding` (f64 as fixed 8 bytes, `String`
/// length-prefixed utf8, `annex` as a length-prefixed JSON string).
fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}
fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}
fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    let bytes = reader.read_bytes(8).map_err(|e| e.to_string())?;
    let array: [u8; 8] = bytes.try_into().map_err(|_| "expected 8 bytes for f64".to_string())?;
    Ok(f64::from_le_bytes(array))
}
fn write_json_bin<T: serde::Serialize>(out: &mut Vec<u8>, value: &T) {
    write_str_bin(out, &serde_json::to_string(value).expect("en1994 mutation payload field always serializes"));
}
fn read_json_bin<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    serde_json::from_str(&read_str_bin(reader)?).map_err(|e| e.to_string())
}

impl protocol::OpBinary for En1994Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1994Mutation::ChangeAnnex(_) => 0,
            En1994Mutation::ChangeMEdKnm(_) => 1,
            En1994Mutation::ChangeVEdKn(_) => 2,
            En1994Mutation::ChangeMPla(_) => 3,
            En1994Mutation::ChangeMPlRd(_) => 4,
            En1994Mutation::ChangeEta(_) => 5,
            En1994Mutation::ChangeVLRd(_) => 6,
            En1994Mutation::ChangeInsulationThicknessMm(_) => 7,
            En1994Mutation::ChangeFireRating(_) => 8,
            En1994Mutation::ChangeDeckType(_) => 9,
            En1994Mutation::ChangeDeltaSigmaMpa(_) => 10,
            En1994Mutation::ChangeFatigueDetail(_) => 11,
            En1994Mutation::ChangeDMm(_) => 12,
            En1994Mutation::ChangeHScMm(_) => 13,
            En1994Mutation::ChangeFCkMpa(_) => 14,
            En1994Mutation::ChangeFUMpa(_) => 15,
            En1994Mutation::ChangeECmMpa(_) => 16,
            En1994Mutation::ChangeVEdPerStudKn(_) => 17,
            En1994Mutation::ChangeSpanM(_) => 18,
            En1994Mutation::ChangeFYMpa(_) => 19,
            En1994Mutation::ChangeNCyclesStud(_) => 20,
            En1994Mutation::ChangeDeltaTauStudMpa(_) => 21,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            En1994Mutation::ChangeAnnex(p) => write_json_bin(&mut out, &p.new_annex),
            En1994Mutation::ChangeMEdKnm(p) => write_f64_bin(&mut out, p.new_m_ed_knm),
            En1994Mutation::ChangeVEdKn(p) => write_f64_bin(&mut out, p.new_v_ed_kn),
            En1994Mutation::ChangeMPla(p) => write_f64_bin(&mut out, p.new_m_pla),
            En1994Mutation::ChangeMPlRd(p) => write_f64_bin(&mut out, p.new_m_pl_rd),
            En1994Mutation::ChangeEta(p) => write_f64_bin(&mut out, p.new_eta),
            En1994Mutation::ChangeVLRd(p) => write_f64_bin(&mut out, p.new_v_l_rd),
            En1994Mutation::ChangeInsulationThicknessMm(p) => write_f64_bin(&mut out, p.new_insulation_thickness_mm),
            En1994Mutation::ChangeFireRating(p) => write_str_bin(&mut out, &p.new_fire_rating),
            En1994Mutation::ChangeDeckType(p) => write_str_bin(&mut out, &p.new_deck_type),
            En1994Mutation::ChangeDeltaSigmaMpa(p) => write_f64_bin(&mut out, p.new_delta_sigma_mpa),
            En1994Mutation::ChangeFatigueDetail(p) => write_str_bin(&mut out, &p.new_fatigue_detail),
            En1994Mutation::ChangeDMm(p) => write_f64_bin(&mut out, p.new_d_mm),
            En1994Mutation::ChangeHScMm(p) => write_f64_bin(&mut out, p.new_h_sc_mm),
            En1994Mutation::ChangeFCkMpa(p) => write_f64_bin(&mut out, p.new_f_ck_mpa),
            En1994Mutation::ChangeFUMpa(p) => write_f64_bin(&mut out, p.new_f_u_mpa),
            En1994Mutation::ChangeECmMpa(p) => write_f64_bin(&mut out, p.new_e_cm_mpa),
            En1994Mutation::ChangeVEdPerStudKn(p) => write_f64_bin(&mut out, p.new_v_ed_per_stud_kn),
            En1994Mutation::ChangeSpanM(p) => write_f64_bin(&mut out, p.new_span_m),
            En1994Mutation::ChangeFYMpa(p) => write_f64_bin(&mut out, p.new_f_y_mpa),
            En1994Mutation::ChangeNCyclesStud(p) => write_f64_bin(&mut out, p.new_n_cycles_stud),
            En1994Mutation::ChangeDeltaTauStudMpa(p) => write_f64_bin(&mut out, p.new_delta_tau_stud_mpa),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(En1994Mutation::ChangeAnnex(ChangeAnnex { new_annex: read_json_bin(&mut reader).map_err(|e| malformed("new_annex", reader.position(), e))? })),
            1 => Ok(En1994Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: read_f64_bin(&mut reader).map_err(|e| malformed("new_m_ed_knm", reader.position(), e))? })),
            2 => Ok(En1994Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: read_f64_bin(&mut reader).map_err(|e| malformed("new_v_ed_kn", reader.position(), e))? })),
            3 => Ok(En1994Mutation::ChangeMPla(ChangeMPla { new_m_pla: read_f64_bin(&mut reader).map_err(|e| malformed("new_m_pla", reader.position(), e))? })),
            4 => Ok(En1994Mutation::ChangeMPlRd(ChangeMPlRd { new_m_pl_rd: read_f64_bin(&mut reader).map_err(|e| malformed("new_m_pl_rd", reader.position(), e))? })),
            5 => Ok(En1994Mutation::ChangeEta(ChangeEta { new_eta: read_f64_bin(&mut reader).map_err(|e| malformed("new_eta", reader.position(), e))? })),
            6 => Ok(En1994Mutation::ChangeVLRd(ChangeVLRd { new_v_l_rd: read_f64_bin(&mut reader).map_err(|e| malformed("new_v_l_rd", reader.position(), e))? })),
            7 => Ok(En1994Mutation::ChangeInsulationThicknessMm(ChangeInsulationThicknessMm { new_insulation_thickness_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_insulation_thickness_mm", reader.position(), e))? })),
            8 => Ok(En1994Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: read_str_bin(&mut reader).map_err(|e| malformed("new_fire_rating", reader.position(), e))? })),
            9 => Ok(En1994Mutation::ChangeDeckType(ChangeDeckType { new_deck_type: read_str_bin(&mut reader).map_err(|e| malformed("new_deck_type", reader.position(), e))? })),
            10 => Ok(En1994Mutation::ChangeDeltaSigmaMpa(ChangeDeltaSigmaMpa { new_delta_sigma_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_delta_sigma_mpa", reader.position(), e))? })),
            11 => Ok(En1994Mutation::ChangeFatigueDetail(ChangeFatigueDetail { new_fatigue_detail: read_str_bin(&mut reader).map_err(|e| malformed("new_fatigue_detail", reader.position(), e))? })),
            12 => Ok(En1994Mutation::ChangeDMm(ChangeDMm { new_d_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_d_mm", reader.position(), e))? })),
            13 => Ok(En1994Mutation::ChangeHScMm(ChangeHScMm { new_h_sc_mm: read_f64_bin(&mut reader).map_err(|e| malformed("new_h_sc_mm", reader.position(), e))? })),
            14 => Ok(En1994Mutation::ChangeFCkMpa(ChangeFCkMpa { new_f_ck_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_f_ck_mpa", reader.position(), e))? })),
            15 => Ok(En1994Mutation::ChangeFUMpa(ChangeFUMpa { new_f_u_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_f_u_mpa", reader.position(), e))? })),
            16 => Ok(En1994Mutation::ChangeECmMpa(ChangeECmMpa { new_e_cm_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_e_cm_mpa", reader.position(), e))? })),
            17 => Ok(En1994Mutation::ChangeVEdPerStudKn(ChangeVEdPerStudKn { new_v_ed_per_stud_kn: read_f64_bin(&mut reader).map_err(|e| malformed("new_v_ed_per_stud_kn", reader.position(), e))? })),
            18 => Ok(En1994Mutation::ChangeSpanM(ChangeSpanM { new_span_m: read_f64_bin(&mut reader).map_err(|e| malformed("new_span_m", reader.position(), e))? })),
            19 => Ok(En1994Mutation::ChangeFYMpa(ChangeFYMpa { new_f_y_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_f_y_mpa", reader.position(), e))? })),
            20 => Ok(En1994Mutation::ChangeNCyclesStud(ChangeNCyclesStud { new_n_cycles_stud: read_f64_bin(&mut reader).map_err(|e| malformed("new_n_cycles_stud", reader.position(), e))? })),
            21 => Ok(En1994Mutation::ChangeDeltaTauStudMpa(ChangeDeltaTauStudMpa { new_delta_tau_stud_mpa: read_f64_bin(&mut reader).map_err(|e| malformed("new_delta_tau_stud_mpa", reader.position(), e))? })),
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1994Mutation> {
    vec![
        En1994Mutation::ChangeAnnex(ChangeAnnex { new_annex: AnnexChoice::En }),
        En1994Mutation::ChangeMEdKnm(ChangeMEdKnm { new_m_ed_knm: 42.75_f64 }),
        En1994Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: 42.75_f64 }),
        En1994Mutation::ChangeMPla(ChangeMPla { new_m_pla: 42.75_f64 }),
        En1994Mutation::ChangeMPlRd(ChangeMPlRd { new_m_pl_rd: 42.75_f64 }),
        En1994Mutation::ChangeEta(ChangeEta { new_eta: 42.75_f64 }),
        En1994Mutation::ChangeVLRd(ChangeVLRd { new_v_l_rd: 42.75_f64 }),
        En1994Mutation::ChangeInsulationThicknessMm(ChangeInsulationThicknessMm { new_insulation_thickness_mm: 42.75_f64 }),
        En1994Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: "demo fire_rating".to_string() }),
        En1994Mutation::ChangeDeckType(ChangeDeckType { new_deck_type: "demo deck_type".to_string() }),
        En1994Mutation::ChangeDeltaSigmaMpa(ChangeDeltaSigmaMpa { new_delta_sigma_mpa: 42.75_f64 }),
        En1994Mutation::ChangeFatigueDetail(ChangeFatigueDetail { new_fatigue_detail: "demo fatigue_detail".to_string() }),
        En1994Mutation::ChangeDMm(ChangeDMm { new_d_mm: 42.75_f64 }),
        En1994Mutation::ChangeHScMm(ChangeHScMm { new_h_sc_mm: 42.75_f64 }),
        En1994Mutation::ChangeFCkMpa(ChangeFCkMpa { new_f_ck_mpa: 42.75_f64 }),
        En1994Mutation::ChangeFUMpa(ChangeFUMpa { new_f_u_mpa: 42.75_f64 }),
        En1994Mutation::ChangeECmMpa(ChangeECmMpa { new_e_cm_mpa: 42.75_f64 }),
        En1994Mutation::ChangeVEdPerStudKn(ChangeVEdPerStudKn { new_v_ed_per_stud_kn: 42.75_f64 }),
        En1994Mutation::ChangeSpanM(ChangeSpanM { new_span_m: 42.75_f64 }),
        En1994Mutation::ChangeFYMpa(ChangeFYMpa { new_f_y_mpa: 42.75_f64 }),
        En1994Mutation::ChangeNCyclesStud(ChangeNCyclesStud { new_n_cycles_stud: 42.75_f64 }),
        En1994Mutation::ChangeDeltaTauStudMpa(ChangeDeltaTauStudMpa { new_delta_tau_stud_mpa: 42.75_f64 }),
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
            let parsed = <En1994Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <En1994Mutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
