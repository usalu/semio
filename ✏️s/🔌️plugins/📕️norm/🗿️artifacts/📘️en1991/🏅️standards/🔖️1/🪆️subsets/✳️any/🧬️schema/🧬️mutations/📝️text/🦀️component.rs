//! ⚡️ En1991 artifact — hand-rolled `OpText`/`OpBinary` for `En1991Mutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../🦀️component.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted
//! here, one keyword per semantic verb, grammar `keyword key=value`. Every variant carries exactly
//! one scalar/enum field, so each arm is a single key.

pub use crate::artifacts::en1991::schema::mutations::En1991Mutation;

use crate::artifacts::en1991::schema::mutations::{
    change_accidental_mass_t::mutation::ChangeAccidentalMassT, change_accidental_speed_km_h::mutation::ChangeAccidentalSpeedKmH, change_annex::mutation::ChangeAnnex, change_area_m2::mutation::ChangeAreaM2,
    change_assumed_gk_kn_m2::mutation::ChangeAssumedGKKnM2, change_bridge_lane::mutation::ChangeBridgeLane, change_bridge_lane_width_m::mutation::ChangeBridgeLaneWidthM, change_bridge_moment_resistance_knm::mutation::ChangeBridgeMomentResistanceKnm,
    change_bridge_span_m::mutation::ChangeBridgeSpanM, change_category::mutation::ChangeCategory, change_cd::mutation::ChangeCD, change_construction_activity::mutation::ChangeConstructionActivity, change_crane_class::mutation::ChangeCraneClass,
    change_cs::mutation::ChangeCS, change_delta_tk::mutation::ChangeDeltaTK, change_en_sk_kn_m2::mutation::ChangeEnSKKnM2, change_en_vbms::mutation::ChangeEnVBMS, change_fire_curve::mutation::ChangeFireCurve,
    change_fire_member_capacity_c::mutation::ChangeFireMemberCapacityC, change_fire_resistance_min::mutation::ChangeFireResistanceMin, change_hoist_class::mutation::ChangeHoistClass, change_hoisting_speed_ms::mutation::ChangeHoistingSpeedMS,
    change_self_weight_material::mutation::ChangeSelfWeightMaterial, change_self_weight_thickness_m::mutation::ChangeSelfWeightThicknessM, change_silo_bulk_density_kn_m3::mutation::ChangeSiloBulkDensityKnM3,
    change_silo_height_m::mutation::ChangeSiloHeightM, change_silo_hydraulic_radius_m::mutation::ChangeSiloHydraulicRadiusM, change_silo_k::mutation::ChangeSiloK, change_silo_mu::mutation::ChangeSiloMu,
    change_snow_altitude_m::mutation::ChangeSnowAltitudeM, change_snow_zone::mutation::ChangeSnowZone, change_wind_zone::mutation::ChangeWindZone,
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
/// 🧬️ Enum-valued fields (`category`/`annex`/`fire_curve`) already derive
/// `Serialize`/`Deserialize` — a quoted JSON string reuses that losslessly instead of a second
/// handcrafted grammar per enum type.
fn enc_json<T: serde::Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("en1991 mutation payload field always serializes"))
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
fn print_en1991_mutation(mutation: &En1991Mutation) -> String {
    match mutation {
        En1991Mutation::ChangeAreaM2(p) => format!("change-area-m2 new-area-m2={}", p.new_area_m2),
        En1991Mutation::ChangeCategory(p) => format!("change-category new-category={}", enc_json(&p.new_category)),
        En1991Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1991Mutation::ChangeSelfWeightMaterial(p) => format!("change-self-weight-material new-self-weight-material={}", enc_str(&p.new_self_weight_material)),
        En1991Mutation::ChangeSelfWeightThicknessM(p) => format!("change-self-weight-thickness-m new-self-weight-thickness-m={}", p.new_self_weight_thickness_m),
        En1991Mutation::ChangeAssumedGKKnM2(p) => format!("change-assumed-gk-kn-m2 new-assumed-g-k-kn-m2={}", p.new_assumed_g_k_kn_m2),
        En1991Mutation::ChangeFireCurve(p) => format!("change-fire-curve new-fire-curve={}", enc_json(&p.new_fire_curve)),
        En1991Mutation::ChangeFireResistanceMin(p) => format!("change-fire-resistance-min new-fire-resistance-min={}", p.new_fire_resistance_min),
        En1991Mutation::ChangeFireMemberCapacityC(p) => format!("change-fire-member-capacity-c new-fire-member-capacity-c={}", p.new_fire_member_capacity_c),
        En1991Mutation::ChangeSnowZone(p) => format!("change-snow-zone new-snow-zone={}", p.new_snow_zone),
        En1991Mutation::ChangeSnowAltitudeM(p) => format!("change-snow-altitude-m new-snow-altitude-m={}", p.new_snow_altitude_m),
        En1991Mutation::ChangeEnSKKnM2(p) => format!("change-en-sk-kn-m2 new-en-s-k-kn-m2={}", p.new_en_s_k_kn_m2),
        En1991Mutation::ChangeWindZone(p) => format!("change-wind-zone new-wind-zone={}", p.new_wind_zone),
        En1991Mutation::ChangeEnVBMS(p) => format!("change-en-vbms new-en-v-b-m-s={}", p.new_en_v_b_m_s),
        En1991Mutation::ChangeDeltaTK(p) => format!("change-delta-tk new-delta-t-k={}", p.new_delta_t_k),
        En1991Mutation::ChangeConstructionActivity(p) => format!("change-construction-activity new-construction-activity={}", enc_str(&p.new_construction_activity)),
        En1991Mutation::ChangeAccidentalMassT(p) => format!("change-accidental-mass-t new-accidental-mass-t={}", p.new_accidental_mass_t),
        En1991Mutation::ChangeAccidentalSpeedKmH(p) => format!("change-accidental-speed-km-h new-accidental-speed-km-h={}", p.new_accidental_speed_km_h),
        En1991Mutation::ChangeBridgeLane(p) => format!("change-bridge-lane new-bridge-lane={}", p.new_bridge_lane),
        En1991Mutation::ChangeBridgeSpanM(p) => format!("change-bridge-span-m new-bridge-span-m={}", p.new_bridge_span_m),
        En1991Mutation::ChangeBridgeLaneWidthM(p) => format!("change-bridge-lane-width-m new-bridge-lane-width-m={}", p.new_bridge_lane_width_m),
        En1991Mutation::ChangeBridgeMomentResistanceKnm(p) => format!("change-bridge-moment-resistance-knm new-bridge-moment-resistance-knm={}", p.new_bridge_moment_resistance_knm),
        En1991Mutation::ChangeCraneClass(p) => format!("change-crane-class new-crane-class={}", enc_str(&p.new_crane_class)),
        En1991Mutation::ChangeHoistClass(p) => format!("change-hoist-class new-hoist-class={}", enc_str(&p.new_hoist_class)),
        En1991Mutation::ChangeHoistingSpeedMS(p) => format!("change-hoisting-speed-ms new-hoisting-speed-m-s={}", p.new_hoisting_speed_m_s),
        En1991Mutation::ChangeSiloBulkDensityKnM3(p) => format!("change-silo-bulk-density-kn-m3 new-silo-bulk-density-kn-m3={}", p.new_silo_bulk_density_kn_m3),
        En1991Mutation::ChangeSiloHeightM(p) => format!("change-silo-height-m new-silo-height-m={}", p.new_silo_height_m),
        En1991Mutation::ChangeSiloHydraulicRadiusM(p) => format!("change-silo-hydraulic-radius-m new-silo-hydraulic-radius-m={}", p.new_silo_hydraulic_radius_m),
        En1991Mutation::ChangeSiloMu(p) => format!("change-silo-mu new-silo-mu={}", p.new_silo_mu),
        En1991Mutation::ChangeSiloK(p) => format!("change-silo-k new-silo-k={}", p.new_silo_k),
        En1991Mutation::ChangeCS(p) => format!("change-cs new-c-s={}", p.new_c_s),
        En1991Mutation::ChangeCD(p) => format!("change-cd new-c-d={}", p.new_c_d),
    }
}

fn parse_en1991_mutation(line: &str) -> Result<En1991Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("en1991 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-area-m2" => Ok(En1991Mutation::ChangeAreaM2(ChangeAreaM2 { new_area_m2: arg("new-area-m2")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-category" => Ok(En1991Mutation::ChangeCategory(ChangeCategory { new_category: dec_json(&arg("new-category")?)? })),
        "change-annex" => Ok(En1991Mutation::ChangeAnnex(ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })),
        "change-self-weight-material" => Ok(En1991Mutation::ChangeSelfWeightMaterial(ChangeSelfWeightMaterial { new_self_weight_material: dec_str(&arg("new-self-weight-material")?)? })),
        "change-self-weight-thickness-m" => {
            Ok(En1991Mutation::ChangeSelfWeightThicknessM(ChangeSelfWeightThicknessM { new_self_weight_thickness_m: arg("new-self-weight-thickness-m")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? }))
        }
        "change-assumed-gk-kn-m2" => Ok(En1991Mutation::ChangeAssumedGKKnM2(ChangeAssumedGKKnM2 { new_assumed_g_k_kn_m2: arg("new-assumed-g-k-kn-m2")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-fire-curve" => Ok(En1991Mutation::ChangeFireCurve(ChangeFireCurve { new_fire_curve: dec_json(&arg("new-fire-curve")?)? })),
        "change-fire-resistance-min" => Ok(En1991Mutation::ChangeFireResistanceMin(ChangeFireResistanceMin { new_fire_resistance_min: arg("new-fire-resistance-min")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-fire-member-capacity-c" => Ok(En1991Mutation::ChangeFireMemberCapacityC(ChangeFireMemberCapacityC { new_fire_member_capacity_c: arg("new-fire-member-capacity-c")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-snow-zone" => Ok(En1991Mutation::ChangeSnowZone(ChangeSnowZone { new_snow_zone: arg("new-snow-zone")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-snow-altitude-m" => Ok(En1991Mutation::ChangeSnowAltitudeM(ChangeSnowAltitudeM { new_snow_altitude_m: arg("new-snow-altitude-m")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-en-sk-kn-m2" => Ok(En1991Mutation::ChangeEnSKKnM2(ChangeEnSKKnM2 { new_en_s_k_kn_m2: arg("new-en-s-k-kn-m2")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-wind-zone" => Ok(En1991Mutation::ChangeWindZone(ChangeWindZone { new_wind_zone: arg("new-wind-zone")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-en-vbms" => Ok(En1991Mutation::ChangeEnVBMS(ChangeEnVBMS { new_en_v_b_m_s: arg("new-en-v-b-m-s")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-delta-tk" => Ok(En1991Mutation::ChangeDeltaTK(ChangeDeltaTK { new_delta_t_k: arg("new-delta-t-k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-construction-activity" => Ok(En1991Mutation::ChangeConstructionActivity(ChangeConstructionActivity { new_construction_activity: dec_str(&arg("new-construction-activity")?)? })),
        "change-accidental-mass-t" => Ok(En1991Mutation::ChangeAccidentalMassT(ChangeAccidentalMassT { new_accidental_mass_t: arg("new-accidental-mass-t")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-accidental-speed-km-h" => Ok(En1991Mutation::ChangeAccidentalSpeedKmH(ChangeAccidentalSpeedKmH { new_accidental_speed_km_h: arg("new-accidental-speed-km-h")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bridge-lane" => Ok(En1991Mutation::ChangeBridgeLane(ChangeBridgeLane { new_bridge_lane: arg("new-bridge-lane")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-bridge-span-m" => Ok(En1991Mutation::ChangeBridgeSpanM(ChangeBridgeSpanM { new_bridge_span_m: arg("new-bridge-span-m")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bridge-lane-width-m" => Ok(En1991Mutation::ChangeBridgeLaneWidthM(ChangeBridgeLaneWidthM { new_bridge_lane_width_m: arg("new-bridge-lane-width-m")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bridge-moment-resistance-knm" => {
            Ok(En1991Mutation::ChangeBridgeMomentResistanceKnm(ChangeBridgeMomentResistanceKnm { new_bridge_moment_resistance_knm: arg("new-bridge-moment-resistance-knm")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? }))
        }
        "change-crane-class" => Ok(En1991Mutation::ChangeCraneClass(ChangeCraneClass { new_crane_class: dec_str(&arg("new-crane-class")?)? })),
        "change-hoist-class" => Ok(En1991Mutation::ChangeHoistClass(ChangeHoistClass { new_hoist_class: dec_str(&arg("new-hoist-class")?)? })),
        "change-hoisting-speed-ms" => Ok(En1991Mutation::ChangeHoistingSpeedMS(ChangeHoistingSpeedMS { new_hoisting_speed_m_s: arg("new-hoisting-speed-m-s")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-silo-bulk-density-kn-m3" => Ok(En1991Mutation::ChangeSiloBulkDensityKnM3(ChangeSiloBulkDensityKnM3 { new_silo_bulk_density_kn_m3: arg("new-silo-bulk-density-kn-m3")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-silo-height-m" => Ok(En1991Mutation::ChangeSiloHeightM(ChangeSiloHeightM { new_silo_height_m: arg("new-silo-height-m")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-silo-hydraulic-radius-m" => {
            Ok(En1991Mutation::ChangeSiloHydraulicRadiusM(ChangeSiloHydraulicRadiusM { new_silo_hydraulic_radius_m: arg("new-silo-hydraulic-radius-m")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? }))
        }
        "change-silo-mu" => Ok(En1991Mutation::ChangeSiloMu(ChangeSiloMu { new_silo_mu: arg("new-silo-mu")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-silo-k" => Ok(En1991Mutation::ChangeSiloK(ChangeSiloK { new_silo_k: arg("new-silo-k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-cs" => Ok(En1991Mutation::ChangeCS(ChangeCS { new_c_s: arg("new-c-s")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-cd" => Ok(En1991Mutation::ChangeCD(ChangeCD { new_c_d: arg("new-c-d")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        other => Err(format!("en1991 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for En1991Mutation {
    fn print_op(&self) -> String {
        print_en1991_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_en1991_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | field bytes` (native little-endian for
/// `f64`/`u8`, length-prefixed UTF-8 for `String`, length-prefixed JSON for the three enums).
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
    write_str_bin(out, &serde_json::to_string(value).expect("en1991 mutation payload field always serializes"));
}
fn read_json_bin<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    serde_json::from_str(&read_str_bin(reader)?).map_err(|e| e.to_string())
}

impl protocol::OpBinary for En1991Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1991Mutation::ChangeAreaM2(_) => 0,
            En1991Mutation::ChangeCategory(_) => 1,
            En1991Mutation::ChangeAnnex(_) => 2,
            En1991Mutation::ChangeSelfWeightMaterial(_) => 3,
            En1991Mutation::ChangeSelfWeightThicknessM(_) => 4,
            En1991Mutation::ChangeAssumedGKKnM2(_) => 5,
            En1991Mutation::ChangeFireCurve(_) => 6,
            En1991Mutation::ChangeFireResistanceMin(_) => 7,
            En1991Mutation::ChangeFireMemberCapacityC(_) => 8,
            En1991Mutation::ChangeSnowZone(_) => 9,
            En1991Mutation::ChangeSnowAltitudeM(_) => 10,
            En1991Mutation::ChangeEnSKKnM2(_) => 11,
            En1991Mutation::ChangeWindZone(_) => 12,
            En1991Mutation::ChangeEnVBMS(_) => 13,
            En1991Mutation::ChangeDeltaTK(_) => 14,
            En1991Mutation::ChangeConstructionActivity(_) => 15,
            En1991Mutation::ChangeAccidentalMassT(_) => 16,
            En1991Mutation::ChangeAccidentalSpeedKmH(_) => 17,
            En1991Mutation::ChangeBridgeLane(_) => 18,
            En1991Mutation::ChangeBridgeSpanM(_) => 19,
            En1991Mutation::ChangeBridgeLaneWidthM(_) => 20,
            En1991Mutation::ChangeBridgeMomentResistanceKnm(_) => 21,
            En1991Mutation::ChangeCraneClass(_) => 22,
            En1991Mutation::ChangeHoistClass(_) => 23,
            En1991Mutation::ChangeHoistingSpeedMS(_) => 24,
            En1991Mutation::ChangeSiloBulkDensityKnM3(_) => 25,
            En1991Mutation::ChangeSiloHeightM(_) => 26,
            En1991Mutation::ChangeSiloHydraulicRadiusM(_) => 27,
            En1991Mutation::ChangeSiloMu(_) => 28,
            En1991Mutation::ChangeSiloK(_) => 29,
            En1991Mutation::ChangeCS(_) => 30,
            En1991Mutation::ChangeCD(_) => 31,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            En1991Mutation::ChangeAreaM2(p) => out.extend_from_slice(&p.new_area_m2.to_le_bytes()),
            En1991Mutation::ChangeCategory(p) => write_json_bin(&mut out, &p.new_category),
            En1991Mutation::ChangeAnnex(p) => write_json_bin(&mut out, &p.new_annex),
            En1991Mutation::ChangeSelfWeightMaterial(p) => write_str_bin(&mut out, &p.new_self_weight_material),
            En1991Mutation::ChangeSelfWeightThicknessM(p) => out.extend_from_slice(&p.new_self_weight_thickness_m.to_le_bytes()),
            En1991Mutation::ChangeAssumedGKKnM2(p) => out.extend_from_slice(&p.new_assumed_g_k_kn_m2.to_le_bytes()),
            En1991Mutation::ChangeFireCurve(p) => write_json_bin(&mut out, &p.new_fire_curve),
            En1991Mutation::ChangeFireResistanceMin(p) => out.extend_from_slice(&p.new_fire_resistance_min.to_le_bytes()),
            En1991Mutation::ChangeFireMemberCapacityC(p) => out.extend_from_slice(&p.new_fire_member_capacity_c.to_le_bytes()),
            En1991Mutation::ChangeSnowZone(p) => out.push(p.new_snow_zone),
            En1991Mutation::ChangeSnowAltitudeM(p) => out.extend_from_slice(&p.new_snow_altitude_m.to_le_bytes()),
            En1991Mutation::ChangeEnSKKnM2(p) => out.extend_from_slice(&p.new_en_s_k_kn_m2.to_le_bytes()),
            En1991Mutation::ChangeWindZone(p) => out.push(p.new_wind_zone),
            En1991Mutation::ChangeEnVBMS(p) => out.extend_from_slice(&p.new_en_v_b_m_s.to_le_bytes()),
            En1991Mutation::ChangeDeltaTK(p) => out.extend_from_slice(&p.new_delta_t_k.to_le_bytes()),
            En1991Mutation::ChangeConstructionActivity(p) => write_str_bin(&mut out, &p.new_construction_activity),
            En1991Mutation::ChangeAccidentalMassT(p) => out.extend_from_slice(&p.new_accidental_mass_t.to_le_bytes()),
            En1991Mutation::ChangeAccidentalSpeedKmH(p) => out.extend_from_slice(&p.new_accidental_speed_km_h.to_le_bytes()),
            En1991Mutation::ChangeBridgeLane(p) => out.push(p.new_bridge_lane),
            En1991Mutation::ChangeBridgeSpanM(p) => out.extend_from_slice(&p.new_bridge_span_m.to_le_bytes()),
            En1991Mutation::ChangeBridgeLaneWidthM(p) => out.extend_from_slice(&p.new_bridge_lane_width_m.to_le_bytes()),
            En1991Mutation::ChangeBridgeMomentResistanceKnm(p) => out.extend_from_slice(&p.new_bridge_moment_resistance_knm.to_le_bytes()),
            En1991Mutation::ChangeCraneClass(p) => write_str_bin(&mut out, &p.new_crane_class),
            En1991Mutation::ChangeHoistClass(p) => write_str_bin(&mut out, &p.new_hoist_class),
            En1991Mutation::ChangeHoistingSpeedMS(p) => out.extend_from_slice(&p.new_hoisting_speed_m_s.to_le_bytes()),
            En1991Mutation::ChangeSiloBulkDensityKnM3(p) => out.extend_from_slice(&p.new_silo_bulk_density_kn_m3.to_le_bytes()),
            En1991Mutation::ChangeSiloHeightM(p) => out.extend_from_slice(&p.new_silo_height_m.to_le_bytes()),
            En1991Mutation::ChangeSiloHydraulicRadiusM(p) => out.extend_from_slice(&p.new_silo_hydraulic_radius_m.to_le_bytes()),
            En1991Mutation::ChangeSiloMu(p) => out.extend_from_slice(&p.new_silo_mu.to_le_bytes()),
            En1991Mutation::ChangeSiloK(p) => out.extend_from_slice(&p.new_silo_k.to_le_bytes()),
            En1991Mutation::ChangeCS(p) => out.extend_from_slice(&p.new_c_s.to_le_bytes()),
            En1991Mutation::ChangeCD(p) => out.extend_from_slice(&p.new_c_d.to_le_bytes()),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(En1991Mutation::ChangeAreaM2(ChangeAreaM2 { new_area_m2: reader.read_f64_le().map_err(|e| malformed("new_area_m2", reader.position(), e.to_string()))? })),
            1 => Ok(En1991Mutation::ChangeCategory(ChangeCategory { new_category: read_json_bin(&mut reader).map_err(|e| malformed("new_category", reader.position(), e))? })),
            2 => Ok(En1991Mutation::ChangeAnnex(ChangeAnnex { new_annex: read_json_bin(&mut reader).map_err(|e| malformed("new_annex", reader.position(), e))? })),
            3 => Ok(En1991Mutation::ChangeSelfWeightMaterial(ChangeSelfWeightMaterial { new_self_weight_material: read_str_bin(&mut reader).map_err(|e| malformed("new_self_weight_material", reader.position(), e))? })),
            4 => Ok(En1991Mutation::ChangeSelfWeightThicknessM(ChangeSelfWeightThicknessM { new_self_weight_thickness_m: reader.read_f64_le().map_err(|e| malformed("new_self_weight_thickness_m", reader.position(), e.to_string()))? })),
            5 => Ok(En1991Mutation::ChangeAssumedGKKnM2(ChangeAssumedGKKnM2 { new_assumed_g_k_kn_m2: reader.read_f64_le().map_err(|e| malformed("new_assumed_g_k_kn_m2", reader.position(), e.to_string()))? })),
            6 => Ok(En1991Mutation::ChangeFireCurve(ChangeFireCurve { new_fire_curve: read_json_bin(&mut reader).map_err(|e| malformed("new_fire_curve", reader.position(), e))? })),
            7 => Ok(En1991Mutation::ChangeFireResistanceMin(ChangeFireResistanceMin { new_fire_resistance_min: reader.read_f64_le().map_err(|e| malformed("new_fire_resistance_min", reader.position(), e.to_string()))? })),
            8 => Ok(En1991Mutation::ChangeFireMemberCapacityC(ChangeFireMemberCapacityC { new_fire_member_capacity_c: reader.read_f64_le().map_err(|e| malformed("new_fire_member_capacity_c", reader.position(), e.to_string()))? })),
            9 => Ok(En1991Mutation::ChangeSnowZone(ChangeSnowZone { new_snow_zone: reader.read_u8().map_err(|e| malformed("new_snow_zone", reader.position(), e.to_string()))? })),
            10 => Ok(En1991Mutation::ChangeSnowAltitudeM(ChangeSnowAltitudeM { new_snow_altitude_m: reader.read_f64_le().map_err(|e| malformed("new_snow_altitude_m", reader.position(), e.to_string()))? })),
            11 => Ok(En1991Mutation::ChangeEnSKKnM2(ChangeEnSKKnM2 { new_en_s_k_kn_m2: reader.read_f64_le().map_err(|e| malformed("new_en_s_k_kn_m2", reader.position(), e.to_string()))? })),
            12 => Ok(En1991Mutation::ChangeWindZone(ChangeWindZone { new_wind_zone: reader.read_u8().map_err(|e| malformed("new_wind_zone", reader.position(), e.to_string()))? })),
            13 => Ok(En1991Mutation::ChangeEnVBMS(ChangeEnVBMS { new_en_v_b_m_s: reader.read_f64_le().map_err(|e| malformed("new_en_v_b_m_s", reader.position(), e.to_string()))? })),
            14 => Ok(En1991Mutation::ChangeDeltaTK(ChangeDeltaTK { new_delta_t_k: reader.read_f64_le().map_err(|e| malformed("new_delta_t_k", reader.position(), e.to_string()))? })),
            15 => Ok(En1991Mutation::ChangeConstructionActivity(ChangeConstructionActivity { new_construction_activity: read_str_bin(&mut reader).map_err(|e| malformed("new_construction_activity", reader.position(), e))? })),
            16 => Ok(En1991Mutation::ChangeAccidentalMassT(ChangeAccidentalMassT { new_accidental_mass_t: reader.read_f64_le().map_err(|e| malformed("new_accidental_mass_t", reader.position(), e.to_string()))? })),
            17 => Ok(En1991Mutation::ChangeAccidentalSpeedKmH(ChangeAccidentalSpeedKmH { new_accidental_speed_km_h: reader.read_f64_le().map_err(|e| malformed("new_accidental_speed_km_h", reader.position(), e.to_string()))? })),
            18 => Ok(En1991Mutation::ChangeBridgeLane(ChangeBridgeLane { new_bridge_lane: reader.read_u8().map_err(|e| malformed("new_bridge_lane", reader.position(), e.to_string()))? })),
            19 => Ok(En1991Mutation::ChangeBridgeSpanM(ChangeBridgeSpanM { new_bridge_span_m: reader.read_f64_le().map_err(|e| malformed("new_bridge_span_m", reader.position(), e.to_string()))? })),
            20 => Ok(En1991Mutation::ChangeBridgeLaneWidthM(ChangeBridgeLaneWidthM { new_bridge_lane_width_m: reader.read_f64_le().map_err(|e| malformed("new_bridge_lane_width_m", reader.position(), e.to_string()))? })),
            21 => Ok(En1991Mutation::ChangeBridgeMomentResistanceKnm(ChangeBridgeMomentResistanceKnm {
                new_bridge_moment_resistance_knm: reader.read_f64_le().map_err(|e| malformed("new_bridge_moment_resistance_knm", reader.position(), e.to_string()))?,
            })),
            22 => Ok(En1991Mutation::ChangeCraneClass(ChangeCraneClass { new_crane_class: read_str_bin(&mut reader).map_err(|e| malformed("new_crane_class", reader.position(), e))? })),
            23 => Ok(En1991Mutation::ChangeHoistClass(ChangeHoistClass { new_hoist_class: read_str_bin(&mut reader).map_err(|e| malformed("new_hoist_class", reader.position(), e))? })),
            24 => Ok(En1991Mutation::ChangeHoistingSpeedMS(ChangeHoistingSpeedMS { new_hoisting_speed_m_s: reader.read_f64_le().map_err(|e| malformed("new_hoisting_speed_m_s", reader.position(), e.to_string()))? })),
            25 => Ok(En1991Mutation::ChangeSiloBulkDensityKnM3(ChangeSiloBulkDensityKnM3 { new_silo_bulk_density_kn_m3: reader.read_f64_le().map_err(|e| malformed("new_silo_bulk_density_kn_m3", reader.position(), e.to_string()))? })),
            26 => Ok(En1991Mutation::ChangeSiloHeightM(ChangeSiloHeightM { new_silo_height_m: reader.read_f64_le().map_err(|e| malformed("new_silo_height_m", reader.position(), e.to_string()))? })),
            27 => Ok(En1991Mutation::ChangeSiloHydraulicRadiusM(ChangeSiloHydraulicRadiusM { new_silo_hydraulic_radius_m: reader.read_f64_le().map_err(|e| malformed("new_silo_hydraulic_radius_m", reader.position(), e.to_string()))? })),
            28 => Ok(En1991Mutation::ChangeSiloMu(ChangeSiloMu { new_silo_mu: reader.read_f64_le().map_err(|e| malformed("new_silo_mu", reader.position(), e.to_string()))? })),
            29 => Ok(En1991Mutation::ChangeSiloK(ChangeSiloK { new_silo_k: reader.read_f64_le().map_err(|e| malformed("new_silo_k", reader.position(), e.to_string()))? })),
            30 => Ok(En1991Mutation::ChangeCS(ChangeCS { new_c_s: reader.read_f64_le().map_err(|e| malformed("new_c_s", reader.position(), e.to_string()))? })),
            31 => Ok(En1991Mutation::ChangeCD(ChangeCD { new_c_d: reader.read_f64_le().map_err(|e| malformed("new_c_d", reader.position(), e.to_string()))? })),
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1991Mutation> {
    vec![
        En1991Mutation::ChangeAreaM2(ChangeAreaM2 { new_area_m2: 50.0 }),
        En1991Mutation::ChangeCategory(ChangeCategory { new_category: crate::document::ImposedCategory::B }),
        En1991Mutation::ChangeAnnex(ChangeAnnex { new_annex: crate::document::AnnexChoice::De }),
        En1991Mutation::ChangeSelfWeightMaterial(ChangeSelfWeightMaterial { new_self_weight_material: "reinforced_concrete".to_string() }),
        En1991Mutation::ChangeSelfWeightThicknessM(ChangeSelfWeightThicknessM { new_self_weight_thickness_m: 0.2 }),
        En1991Mutation::ChangeAssumedGKKnM2(ChangeAssumedGKKnM2 { new_assumed_g_k_kn_m2: 6.0 }),
        En1991Mutation::ChangeFireCurve(ChangeFireCurve { new_fire_curve: crate::artifacts::en1991::part_1_2::FireCurve::Standard }),
        En1991Mutation::ChangeFireResistanceMin(ChangeFireResistanceMin { new_fire_resistance_min: 30.0 }),
        En1991Mutation::ChangeFireMemberCapacityC(ChangeFireMemberCapacityC { new_fire_member_capacity_c: 900.0 }),
        En1991Mutation::ChangeSnowZone(ChangeSnowZone { new_snow_zone: 2 }),
        En1991Mutation::ChangeSnowAltitudeM(ChangeSnowAltitudeM { new_snow_altitude_m: 150.0 }),
        En1991Mutation::ChangeEnSKKnM2(ChangeEnSKKnM2 { new_en_s_k_kn_m2: 0.85 }),
        En1991Mutation::ChangeWindZone(ChangeWindZone { new_wind_zone: 2 }),
        En1991Mutation::ChangeEnVBMS(ChangeEnVBMS { new_en_v_b_m_s: 25.0 }),
        En1991Mutation::ChangeDeltaTK(ChangeDeltaTK { new_delta_t_k: 30.0 }),
        En1991Mutation::ChangeConstructionActivity(ChangeConstructionActivity { new_construction_activity: "scaffolding".to_string() }),
        En1991Mutation::ChangeAccidentalMassT(ChangeAccidentalMassT { new_accidental_mass_t: 30.0 }),
        En1991Mutation::ChangeAccidentalSpeedKmH(ChangeAccidentalSpeedKmH { new_accidental_speed_km_h: 80.0 }),
        En1991Mutation::ChangeBridgeLane(ChangeBridgeLane { new_bridge_lane: 1 }),
        En1991Mutation::ChangeBridgeSpanM(ChangeBridgeSpanM { new_bridge_span_m: 20.0 }),
        En1991Mutation::ChangeBridgeLaneWidthM(ChangeBridgeLaneWidthM { new_bridge_lane_width_m: 3.0 }),
        En1991Mutation::ChangeBridgeMomentResistanceKnm(ChangeBridgeMomentResistanceKnm { new_bridge_moment_resistance_knm: 3000.0 }),
        En1991Mutation::ChangeCraneClass(ChangeCraneClass { new_crane_class: "HC2".to_string() }),
        En1991Mutation::ChangeHoistClass(ChangeHoistClass { new_hoist_class: "HC2".to_string() }),
        En1991Mutation::ChangeHoistingSpeedMS(ChangeHoistingSpeedMS { new_hoisting_speed_m_s: 0.5 }),
        En1991Mutation::ChangeSiloBulkDensityKnM3(ChangeSiloBulkDensityKnM3 { new_silo_bulk_density_kn_m3: 8.0 }),
        En1991Mutation::ChangeSiloHeightM(ChangeSiloHeightM { new_silo_height_m: 12.0 }),
        En1991Mutation::ChangeSiloHydraulicRadiusM(ChangeSiloHydraulicRadiusM { new_silo_hydraulic_radius_m: 1.5 }),
        En1991Mutation::ChangeSiloMu(ChangeSiloMu { new_silo_mu: 0.4 }),
        En1991Mutation::ChangeSiloK(ChangeSiloK { new_silo_k: 0.4 }),
        En1991Mutation::ChangeCS(ChangeCS { new_c_s: 1.0 }),
        En1991Mutation::ChangeCD(ChangeCD { new_c_d: 1.0 }),
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
            let parsed = <En1991Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <En1991Mutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
