//! ⚡️ DIN EN 16798 — hand-rolled OpText/OpBinary for `Din16798Mutation`.

pub use crate::artifact_schema::mutations::Din16798Mutation;

use crate::artifact_schema::mutations::{
    change_annex::ChangeAnnex,
    change_theta_rm::ChangeThetaRm,
    change_outdoor_co2::ChangeOutdoorCo2,
    change_envelope_n50::ChangeEnvelopeN50,
    change_envelope_volume::ChangeEnvelopeVolume,
    change_cellar_area::ChangeCellarArea,
    change_cellar_ventilation::ChangeCellarVentilation,
    change_night_setback::ChangeNightSetback,
    insert_zone::InsertZone,
    remove_zone::RemoveZone,
    change_zone_usage_type::ChangeZoneUsageType,
    change_zone_floor_area::ChangeZoneFloorArea,
    change_zone_occupants::ChangeZoneOccupants,
    change_zone_comfort_category::ChangeZoneComfortCategory,
    change_zone_pollution_class::ChangeZonePollutionClass,
    change_zone_comfort_model::ChangeZoneComfortModel,
    change_zone_t_op_winter::ChangeZoneTOpWinter,
    change_zone_t_op_summer::ChangeZoneTOpSummer,
    change_zone_air_speed::ChangeZoneAirSpeed,
    change_zone_clothing::ChangeZoneClothing,
    change_zone_metabolic_rate::ChangeZoneMetabolicRate,
    change_zone_rh::ChangeZoneRh,
    change_zone_outdoor_air::ChangeZoneOutdoorAir,
    change_zone_co2::ChangeZoneCo2,
    change_zone_illuminance::ChangeZoneIlluminance,
    change_zone_noise::ChangeZoneNoise,
    change_zone_vent_system_id::ChangeZoneVentSystemId,
    change_zone_turbulence::ChangeZoneTurbulence,
    change_zone_vent_method::ChangeZoneVentMethod,
    insert_vent_system::InsertVentSystem,
    remove_vent_system::RemoveVentSystem,
    change_vent_system_type::ChangeVentSystemType,
    change_vent_sfp::ChangeVentSfp,
    change_vent_sfp_class::ChangeVentSfpClass,
    change_vent_heat_recovery::ChangeVentHeatRecovery,
    change_vent_oda_class::ChangeVentOdaClass,
    change_vent_filter_sup::ChangeVentFilterSup,
    change_vent_inspection::ChangeVentInspection,
    change_vent_duct_class::ChangeVentDuctClass,
    change_vent_duct_leakage::ChangeVentDuctLeakage,
    change_vent_design_airflow::ChangeVentDesignAirflow,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
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
fn enc_json<T: dsl::ToValue>(value: &T) -> String {
    enc_str(&pack::json::to_json_string(value))
}
fn dec_json<T: dsl::FromValue>(s: &str) -> Result<T, String> {
    pack::json::from_json_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
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
    tokenize_args(rest)
        .into_iter()
        .map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}")))
        .collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️OpText
fn print_din16798_mutation(mutation: &Din16798Mutation) -> String {
    match mutation {
        Din16798Mutation::ChangeAnnex(p) => format!("change-annex {}", [format!("new-annex={}", enc_json(&p.new_annex))].join(" ")),
        Din16798Mutation::ChangeThetaRm(p) => format!("change-theta-rm {}", [format!("new-theta-rm-c={}", enc_json(&p.new_theta_rm_c))].join(" ")),
        Din16798Mutation::ChangeOutdoorCo2(p) => format!("change-outdoor-co2 {}", [format!("new-outdoor-co2-ppm={}", enc_json(&p.new_outdoor_co2_ppm))].join(" ")),
        Din16798Mutation::ChangeEnvelopeN50(p) => format!("change-envelope-n50 {}", [format!("new-envelope-n50-h-inv={}", enc_json(&p.new_envelope_n50_h_inv))].join(" ")),
        Din16798Mutation::ChangeEnvelopeVolume(p) => format!("change-envelope-volume {}", [format!("new-envelope-volume-m3={}", enc_json(&p.new_envelope_volume_m3))].join(" ")),
        Din16798Mutation::ChangeCellarArea(p) => format!("change-cellar-area {}", [format!("new-cellar-area-m2={}", enc_json(&p.new_cellar_area_m2))].join(" ")),
        Din16798Mutation::ChangeCellarVentilation(p) => format!("change-cellar-ventilation {}", [format!("new-cellar-ventilation-m3-h={}", enc_json(&p.new_cellar_ventilation_m3_h))].join(" ")),
        Din16798Mutation::ChangeNightSetback(p) => format!("change-night-setback {}", [format!("new-night-setback-k={}", enc_json(&p.new_night_setback_k))].join(" ")),
        Din16798Mutation::InsertZone(p) => format!("insert-zone {}", [format!("index={}", enc_json(&p.index)), format!("zone={}", enc_json(&p.zone))].join(" ")),
        Din16798Mutation::RemoveZone(p) => format!("remove-zone {}", [format!("zone-id={}", enc_json(&p.zone_id))].join(" ")),
        Din16798Mutation::ChangeZoneUsageType(p) => format!("change-zone-usage-type {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-usage-type={}", enc_json(&p.new_usage_type))].join(" ")),
        Din16798Mutation::ChangeZoneFloorArea(p) => format!("change-zone-floor-area {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-floor-area-m2={}", enc_json(&p.new_floor_area_m2))].join(" ")),
        Din16798Mutation::ChangeZoneOccupants(p) => format!("change-zone-occupants {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-occupants={}", enc_json(&p.new_occupants))].join(" ")),
        Din16798Mutation::ChangeZoneComfortCategory(p) => format!("change-zone-comfort-category {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-comfort-category={}", enc_json(&p.new_comfort_category))].join(" ")),
        Din16798Mutation::ChangeZonePollutionClass(p) => format!("change-zone-pollution-class {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-pollution-class={}", enc_json(&p.new_pollution_class))].join(" ")),
        Din16798Mutation::ChangeZoneComfortModel(p) => format!("change-zone-comfort-model {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-comfort-model={}", enc_json(&p.new_comfort_model))].join(" ")),
        Din16798Mutation::ChangeZoneTOpWinter(p) => format!("change-zone-t-op-winter {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-t-op-winter-c={}", enc_json(&p.new_t_op_winter_c))].join(" ")),
        Din16798Mutation::ChangeZoneTOpSummer(p) => format!("change-zone-t-op-summer {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-t-op-summer-c={}", enc_json(&p.new_t_op_summer_c))].join(" ")),
        Din16798Mutation::ChangeZoneAirSpeed(p) => format!("change-zone-air-speed {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-air-speed-m-s={}", enc_json(&p.new_air_speed_m_s))].join(" ")),
        Din16798Mutation::ChangeZoneClothing(p) => format!("change-zone-clothing {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-clothing-clo={}", enc_json(&p.new_clothing_clo))].join(" ")),
        Din16798Mutation::ChangeZoneMetabolicRate(p) => format!("change-zone-metabolic-rate {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-metabolic-rate-met={}", enc_json(&p.new_metabolic_rate_met))].join(" ")),
        Din16798Mutation::ChangeZoneRh(p) => format!("change-zone-rh {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-rh-percent={}", enc_json(&p.new_rh_percent))].join(" ")),
        Din16798Mutation::ChangeZoneOutdoorAir(p) => format!("change-zone-outdoor-air {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-outdoor-air-supplied-m3-h={}", enc_json(&p.new_outdoor_air_supplied_m3_h))].join(" ")),
        Din16798Mutation::ChangeZoneCo2(p) => format!("change-zone-co2 {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-co2-ppm={}", enc_json(&p.new_co2_ppm))].join(" ")),
        Din16798Mutation::ChangeZoneIlluminance(p) => format!("change-zone-illuminance {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-illuminance-lx={}", enc_json(&p.new_illuminance_lx))].join(" ")),
        Din16798Mutation::ChangeZoneNoise(p) => format!("change-zone-noise {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-noise-db={}", enc_json(&p.new_noise_db))].join(" ")),
        Din16798Mutation::ChangeZoneVentSystemId(p) => format!("change-zone-vent-system-id {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-vent-system-id={}", enc_json(&p.new_vent_system_id))].join(" ")),
        Din16798Mutation::ChangeZoneTurbulence(p) => format!("change-zone-turbulence {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-turbulence-intensity-percent={}", enc_json(&p.new_turbulence_intensity_percent))].join(" ")),
        Din16798Mutation::ChangeZoneVentMethod(p) => format!("change-zone-vent-method {}", [format!("zone-id={}", enc_json(&p.zone_id)), format!("new-vent-method={}", enc_json(&p.new_vent_method))].join(" ")),
        Din16798Mutation::InsertVentSystem(p) => format!("insert-vent-system {}", [format!("index={}", enc_json(&p.index)), format!("vent={}", enc_json(&p.vent))].join(" ")),
        Din16798Mutation::RemoveVentSystem(p) => format!("remove-vent-system {}", [format!("vent-id={}", enc_json(&p.vent_id))].join(" ")),
        Din16798Mutation::ChangeVentSystemType(p) => format!("change-vent-system-type {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-system-type={}", enc_json(&p.new_system_type))].join(" ")),
        Din16798Mutation::ChangeVentSfp(p) => format!("change-vent-sfp {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-sfp-w-m3-s={}", enc_json(&p.new_sfp_w_m3_s))].join(" ")),
        Din16798Mutation::ChangeVentSfpClass(p) => format!("change-vent-sfp-class {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-sfp-required-class={}", enc_json(&p.new_sfp_required_class))].join(" ")),
        Din16798Mutation::ChangeVentHeatRecovery(p) => format!("change-vent-heat-recovery {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-heat-recovery-eta={}", enc_json(&p.new_heat_recovery_eta))].join(" ")),
        Din16798Mutation::ChangeVentOdaClass(p) => format!("change-vent-oda-class {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-oda-class={}", enc_json(&p.new_oda_class))].join(" ")),
        Din16798Mutation::ChangeVentFilterSup(p) => format!("change-vent-filter-sup {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-filter-sup-class={}", enc_json(&p.new_filter_sup_class))].join(" ")),
        Din16798Mutation::ChangeVentInspection(p) => format!("change-vent-inspection {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-years-since-inspection={}", enc_json(&p.new_years_since_inspection))].join(" ")),
        Din16798Mutation::ChangeVentDuctClass(p) => format!("change-vent-duct-class {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-duct-class={}", enc_json(&p.new_duct_class))].join(" ")),
        Din16798Mutation::ChangeVentDuctLeakage(p) => format!("change-vent-duct-leakage {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-duct-leakage-m3-s-m2={}", enc_json(&p.new_duct_leakage_m3_s_m2))].join(" ")),
        Din16798Mutation::ChangeVentDesignAirflow(p) => format!("change-vent-design-airflow {}", [format!("vent-id={}", enc_json(&p.vent_id)), format!("new-design-airflow-m3-h={}", enc_json(&p.new_design_airflow_m3_h))].join(" ")),
    }
}

fn parse_din16798_mutation(line: &str) -> Result<Din16798Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("din16798 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-annex" => Ok(Din16798Mutation::ChangeAnnex(ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })),
        "change-theta-rm" => Ok(Din16798Mutation::ChangeThetaRm(ChangeThetaRm { new_theta_rm_c: dec_json(&arg("new-theta-rm-c")?)? })),
        "change-outdoor-co2" => Ok(Din16798Mutation::ChangeOutdoorCo2(ChangeOutdoorCo2 { new_outdoor_co2_ppm: dec_json(&arg("new-outdoor-co2-ppm")?)? })),
        "change-envelope-n50" => Ok(Din16798Mutation::ChangeEnvelopeN50(ChangeEnvelopeN50 { new_envelope_n50_h_inv: dec_json(&arg("new-envelope-n50-h-inv")?)? })),
        "change-envelope-volume" => Ok(Din16798Mutation::ChangeEnvelopeVolume(ChangeEnvelopeVolume { new_envelope_volume_m3: dec_json(&arg("new-envelope-volume-m3")?)? })),
        "change-cellar-area" => Ok(Din16798Mutation::ChangeCellarArea(ChangeCellarArea { new_cellar_area_m2: dec_json(&arg("new-cellar-area-m2")?)? })),
        "change-cellar-ventilation" => Ok(Din16798Mutation::ChangeCellarVentilation(ChangeCellarVentilation { new_cellar_ventilation_m3_h: dec_json(&arg("new-cellar-ventilation-m3-h")?)? })),
        "change-night-setback" => Ok(Din16798Mutation::ChangeNightSetback(ChangeNightSetback { new_night_setback_k: dec_json(&arg("new-night-setback-k")?)? })),
        "insert-zone" => Ok(Din16798Mutation::InsertZone(InsertZone { index: dec_json(&arg("index")?)?, zone: dec_json(&arg("zone")?)? })),
        "remove-zone" => Ok(Din16798Mutation::RemoveZone(RemoveZone { zone_id: dec_json(&arg("zone-id")?)? })),
        "change-zone-usage-type" => Ok(Din16798Mutation::ChangeZoneUsageType(ChangeZoneUsageType { zone_id: dec_json(&arg("zone-id")?)?, new_usage_type: dec_json(&arg("new-usage-type")?)? })),
        "change-zone-floor-area" => Ok(Din16798Mutation::ChangeZoneFloorArea(ChangeZoneFloorArea { zone_id: dec_json(&arg("zone-id")?)?, new_floor_area_m2: dec_json(&arg("new-floor-area-m2")?)? })),
        "change-zone-occupants" => Ok(Din16798Mutation::ChangeZoneOccupants(ChangeZoneOccupants { zone_id: dec_json(&arg("zone-id")?)?, new_occupants: dec_json(&arg("new-occupants")?)? })),
        "change-zone-comfort-category" => Ok(Din16798Mutation::ChangeZoneComfortCategory(ChangeZoneComfortCategory { zone_id: dec_json(&arg("zone-id")?)?, new_comfort_category: dec_json(&arg("new-comfort-category")?)? })),
        "change-zone-pollution-class" => Ok(Din16798Mutation::ChangeZonePollutionClass(ChangeZonePollutionClass { zone_id: dec_json(&arg("zone-id")?)?, new_pollution_class: dec_json(&arg("new-pollution-class")?)? })),
        "change-zone-comfort-model" => Ok(Din16798Mutation::ChangeZoneComfortModel(ChangeZoneComfortModel { zone_id: dec_json(&arg("zone-id")?)?, new_comfort_model: dec_json(&arg("new-comfort-model")?)? })),
        "change-zone-t-op-winter" => Ok(Din16798Mutation::ChangeZoneTOpWinter(ChangeZoneTOpWinter { zone_id: dec_json(&arg("zone-id")?)?, new_t_op_winter_c: dec_json(&arg("new-t-op-winter-c")?)? })),
        "change-zone-t-op-summer" => Ok(Din16798Mutation::ChangeZoneTOpSummer(ChangeZoneTOpSummer { zone_id: dec_json(&arg("zone-id")?)?, new_t_op_summer_c: dec_json(&arg("new-t-op-summer-c")?)? })),
        "change-zone-air-speed" => Ok(Din16798Mutation::ChangeZoneAirSpeed(ChangeZoneAirSpeed { zone_id: dec_json(&arg("zone-id")?)?, new_air_speed_m_s: dec_json(&arg("new-air-speed-m-s")?)? })),
        "change-zone-clothing" => Ok(Din16798Mutation::ChangeZoneClothing(ChangeZoneClothing { zone_id: dec_json(&arg("zone-id")?)?, new_clothing_clo: dec_json(&arg("new-clothing-clo")?)? })),
        "change-zone-metabolic-rate" => Ok(Din16798Mutation::ChangeZoneMetabolicRate(ChangeZoneMetabolicRate { zone_id: dec_json(&arg("zone-id")?)?, new_metabolic_rate_met: dec_json(&arg("new-metabolic-rate-met")?)? })),
        "change-zone-rh" => Ok(Din16798Mutation::ChangeZoneRh(ChangeZoneRh { zone_id: dec_json(&arg("zone-id")?)?, new_rh_percent: dec_json(&arg("new-rh-percent")?)? })),
        "change-zone-outdoor-air" => Ok(Din16798Mutation::ChangeZoneOutdoorAir(ChangeZoneOutdoorAir { zone_id: dec_json(&arg("zone-id")?)?, new_outdoor_air_supplied_m3_h: dec_json(&arg("new-outdoor-air-supplied-m3-h")?)? })),
        "change-zone-co2" => Ok(Din16798Mutation::ChangeZoneCo2(ChangeZoneCo2 { zone_id: dec_json(&arg("zone-id")?)?, new_co2_ppm: dec_json(&arg("new-co2-ppm")?)? })),
        "change-zone-illuminance" => Ok(Din16798Mutation::ChangeZoneIlluminance(ChangeZoneIlluminance { zone_id: dec_json(&arg("zone-id")?)?, new_illuminance_lx: dec_json(&arg("new-illuminance-lx")?)? })),
        "change-zone-noise" => Ok(Din16798Mutation::ChangeZoneNoise(ChangeZoneNoise { zone_id: dec_json(&arg("zone-id")?)?, new_noise_db: dec_json(&arg("new-noise-db")?)? })),
        "change-zone-vent-system-id" => Ok(Din16798Mutation::ChangeZoneVentSystemId(ChangeZoneVentSystemId { zone_id: dec_json(&arg("zone-id")?)?, new_vent_system_id: dec_json(&arg("new-vent-system-id")?)? })),
        "change-zone-turbulence" => Ok(Din16798Mutation::ChangeZoneTurbulence(ChangeZoneTurbulence { zone_id: dec_json(&arg("zone-id")?)?, new_turbulence_intensity_percent: dec_json(&arg("new-turbulence-intensity-percent")?)? })),
        "change-zone-vent-method" => Ok(Din16798Mutation::ChangeZoneVentMethod(ChangeZoneVentMethod { zone_id: dec_json(&arg("zone-id")?)?, new_vent_method: dec_json(&arg("new-vent-method")?)? })),
        "insert-vent-system" => Ok(Din16798Mutation::InsertVentSystem(InsertVentSystem { index: dec_json(&arg("index")?)?, vent: dec_json(&arg("vent")?)? })),
        "remove-vent-system" => Ok(Din16798Mutation::RemoveVentSystem(RemoveVentSystem { vent_id: dec_json(&arg("vent-id")?)? })),
        "change-vent-system-type" => Ok(Din16798Mutation::ChangeVentSystemType(ChangeVentSystemType { vent_id: dec_json(&arg("vent-id")?)?, new_system_type: dec_json(&arg("new-system-type")?)? })),
        "change-vent-sfp" => Ok(Din16798Mutation::ChangeVentSfp(ChangeVentSfp { vent_id: dec_json(&arg("vent-id")?)?, new_sfp_w_m3_s: dec_json(&arg("new-sfp-w-m3-s")?)? })),
        "change-vent-sfp-class" => Ok(Din16798Mutation::ChangeVentSfpClass(ChangeVentSfpClass { vent_id: dec_json(&arg("vent-id")?)?, new_sfp_required_class: dec_json(&arg("new-sfp-required-class")?)? })),
        "change-vent-heat-recovery" => Ok(Din16798Mutation::ChangeVentHeatRecovery(ChangeVentHeatRecovery { vent_id: dec_json(&arg("vent-id")?)?, new_heat_recovery_eta: dec_json(&arg("new-heat-recovery-eta")?)? })),
        "change-vent-oda-class" => Ok(Din16798Mutation::ChangeVentOdaClass(ChangeVentOdaClass { vent_id: dec_json(&arg("vent-id")?)?, new_oda_class: dec_json(&arg("new-oda-class")?)? })),
        "change-vent-filter-sup" => Ok(Din16798Mutation::ChangeVentFilterSup(ChangeVentFilterSup { vent_id: dec_json(&arg("vent-id")?)?, new_filter_sup_class: dec_json(&arg("new-filter-sup-class")?)? })),
        "change-vent-inspection" => Ok(Din16798Mutation::ChangeVentInspection(ChangeVentInspection { vent_id: dec_json(&arg("vent-id")?)?, new_years_since_inspection: dec_json(&arg("new-years-since-inspection")?)? })),
        "change-vent-duct-class" => Ok(Din16798Mutation::ChangeVentDuctClass(ChangeVentDuctClass { vent_id: dec_json(&arg("vent-id")?)?, new_duct_class: dec_json(&arg("new-duct-class")?)? })),
        "change-vent-duct-leakage" => Ok(Din16798Mutation::ChangeVentDuctLeakage(ChangeVentDuctLeakage { vent_id: dec_json(&arg("vent-id")?)?, new_duct_leakage_m3_s_m2: dec_json(&arg("new-duct-leakage-m3-s-m2")?)? })),
        "change-vent-design-airflow" => Ok(Din16798Mutation::ChangeVentDesignAirflow(ChangeVentDesignAirflow { vent_id: dec_json(&arg("vent-id")?)?, new_design_airflow_m3_h: dec_json(&arg("new-design-airflow-m3-h")?)? })),
        other => Err(format!("unknown din16798 mutation keyword '{other}'")),
    }
}

impl protocol::OpText for Din16798Mutation {
    fn print_op(&self) -> String {
        print_din16798_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_din16798_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
fn write_json_bin<T: dsl::ToValue>(out: &mut Vec<u8>, value: &T) {
    let bytes = pack::json::to_json_string(value);
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes.as_bytes());
}
fn read_json_bin<T: dsl::FromValue>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    pack::json::from_json_str(text).map_err(|e| e.to_string())
}

const WIRE_PROTOCOL: &str = include_str!("../💾️binary/📡️.protocol.semio");
const TAG_CHANGE_ANNEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-annex");
const TAG_CHANGE_THETA_RM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-theta-rm");
const TAG_CHANGE_OUTDOOR_CO2: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-outdoor-co2");
const TAG_CHANGE_ENVELOPE_N50: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-envelope-n50");
const TAG_CHANGE_ENVELOPE_VOLUME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-envelope-volume");
const TAG_CHANGE_CELLAR_AREA: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-cellar-area");
const TAG_CHANGE_CELLAR_VENTILATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-cellar-ventilation");
const TAG_CHANGE_NIGHT_SETBACK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-night-setback");
const TAG_INSERT_ZONE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-zone");
const TAG_REMOVE_ZONE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-zone");
const TAG_CHANGE_ZONE_USAGE_TYPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-usage-type");
const TAG_CHANGE_ZONE_FLOOR_AREA: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-floor-area");
const TAG_CHANGE_ZONE_OCCUPANTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-occupants");
const TAG_CHANGE_ZONE_COMFORT_CATEGORY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-comfort-category");
const TAG_CHANGE_ZONE_POLLUTION_CLASS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-pollution-class");
const TAG_CHANGE_ZONE_COMFORT_MODEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-comfort-model");
const TAG_CHANGE_ZONE_T_OP_WINTER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-t-op-winter");
const TAG_CHANGE_ZONE_T_OP_SUMMER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-t-op-summer");
const TAG_CHANGE_ZONE_AIR_SPEED: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-air-speed");
const TAG_CHANGE_ZONE_CLOTHING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-clothing");
const TAG_CHANGE_ZONE_METABOLIC_RATE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-metabolic-rate");
const TAG_CHANGE_ZONE_RH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-rh");
const TAG_CHANGE_ZONE_OUTDOOR_AIR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-outdoor-air");
const TAG_CHANGE_ZONE_CO2: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-co2");
const TAG_CHANGE_ZONE_ILLUMINANCE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-illuminance");
const TAG_CHANGE_ZONE_NOISE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-noise");
const TAG_CHANGE_ZONE_VENT_SYSTEM_ID: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-vent-system-id");
const TAG_CHANGE_ZONE_TURBULENCE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-turbulence");
const TAG_CHANGE_ZONE_VENT_METHOD: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-zone-vent-method");
const TAG_INSERT_VENT_SYSTEM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-vent-system");
const TAG_REMOVE_VENT_SYSTEM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-vent-system");
const TAG_CHANGE_VENT_SYSTEM_TYPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-system-type");
const TAG_CHANGE_VENT_SFP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-sfp");
const TAG_CHANGE_VENT_SFP_CLASS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-sfp-class");
const TAG_CHANGE_VENT_HEAT_RECOVERY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-heat-recovery");
const TAG_CHANGE_VENT_ODA_CLASS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-oda-class");
const TAG_CHANGE_VENT_FILTER_SUP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-filter-sup");
const TAG_CHANGE_VENT_INSPECTION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-inspection");
const TAG_CHANGE_VENT_DUCT_CLASS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-duct-class");
const TAG_CHANGE_VENT_DUCT_LEAKAGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-duct-leakage");
const TAG_CHANGE_VENT_DESIGN_AIRFLOW: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-vent-design-airflow");

impl protocol::OpBinary for Din16798Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            Din16798Mutation::ChangeAnnex(_) => TAG_CHANGE_ANNEX,
            Din16798Mutation::ChangeThetaRm(_) => TAG_CHANGE_THETA_RM,
            Din16798Mutation::ChangeOutdoorCo2(_) => TAG_CHANGE_OUTDOOR_CO2,
            Din16798Mutation::ChangeEnvelopeN50(_) => TAG_CHANGE_ENVELOPE_N50,
            Din16798Mutation::ChangeEnvelopeVolume(_) => TAG_CHANGE_ENVELOPE_VOLUME,
            Din16798Mutation::ChangeCellarArea(_) => TAG_CHANGE_CELLAR_AREA,
            Din16798Mutation::ChangeCellarVentilation(_) => TAG_CHANGE_CELLAR_VENTILATION,
            Din16798Mutation::ChangeNightSetback(_) => TAG_CHANGE_NIGHT_SETBACK,
            Din16798Mutation::InsertZone(_) => TAG_INSERT_ZONE,
            Din16798Mutation::RemoveZone(_) => TAG_REMOVE_ZONE,
            Din16798Mutation::ChangeZoneUsageType(_) => TAG_CHANGE_ZONE_USAGE_TYPE,
            Din16798Mutation::ChangeZoneFloorArea(_) => TAG_CHANGE_ZONE_FLOOR_AREA,
            Din16798Mutation::ChangeZoneOccupants(_) => TAG_CHANGE_ZONE_OCCUPANTS,
            Din16798Mutation::ChangeZoneComfortCategory(_) => TAG_CHANGE_ZONE_COMFORT_CATEGORY,
            Din16798Mutation::ChangeZonePollutionClass(_) => TAG_CHANGE_ZONE_POLLUTION_CLASS,
            Din16798Mutation::ChangeZoneComfortModel(_) => TAG_CHANGE_ZONE_COMFORT_MODEL,
            Din16798Mutation::ChangeZoneTOpWinter(_) => TAG_CHANGE_ZONE_T_OP_WINTER,
            Din16798Mutation::ChangeZoneTOpSummer(_) => TAG_CHANGE_ZONE_T_OP_SUMMER,
            Din16798Mutation::ChangeZoneAirSpeed(_) => TAG_CHANGE_ZONE_AIR_SPEED,
            Din16798Mutation::ChangeZoneClothing(_) => TAG_CHANGE_ZONE_CLOTHING,
            Din16798Mutation::ChangeZoneMetabolicRate(_) => TAG_CHANGE_ZONE_METABOLIC_RATE,
            Din16798Mutation::ChangeZoneRh(_) => TAG_CHANGE_ZONE_RH,
            Din16798Mutation::ChangeZoneOutdoorAir(_) => TAG_CHANGE_ZONE_OUTDOOR_AIR,
            Din16798Mutation::ChangeZoneCo2(_) => TAG_CHANGE_ZONE_CO2,
            Din16798Mutation::ChangeZoneIlluminance(_) => TAG_CHANGE_ZONE_ILLUMINANCE,
            Din16798Mutation::ChangeZoneNoise(_) => TAG_CHANGE_ZONE_NOISE,
            Din16798Mutation::ChangeZoneVentSystemId(_) => TAG_CHANGE_ZONE_VENT_SYSTEM_ID,
            Din16798Mutation::ChangeZoneTurbulence(_) => TAG_CHANGE_ZONE_TURBULENCE,
            Din16798Mutation::ChangeZoneVentMethod(_) => TAG_CHANGE_ZONE_VENT_METHOD,
            Din16798Mutation::InsertVentSystem(_) => TAG_INSERT_VENT_SYSTEM,
            Din16798Mutation::RemoveVentSystem(_) => TAG_REMOVE_VENT_SYSTEM,
            Din16798Mutation::ChangeVentSystemType(_) => TAG_CHANGE_VENT_SYSTEM_TYPE,
            Din16798Mutation::ChangeVentSfp(_) => TAG_CHANGE_VENT_SFP,
            Din16798Mutation::ChangeVentSfpClass(_) => TAG_CHANGE_VENT_SFP_CLASS,
            Din16798Mutation::ChangeVentHeatRecovery(_) => TAG_CHANGE_VENT_HEAT_RECOVERY,
            Din16798Mutation::ChangeVentOdaClass(_) => TAG_CHANGE_VENT_ODA_CLASS,
            Din16798Mutation::ChangeVentFilterSup(_) => TAG_CHANGE_VENT_FILTER_SUP,
            Din16798Mutation::ChangeVentInspection(_) => TAG_CHANGE_VENT_INSPECTION,
            Din16798Mutation::ChangeVentDuctClass(_) => TAG_CHANGE_VENT_DUCT_CLASS,
            Din16798Mutation::ChangeVentDuctLeakage(_) => TAG_CHANGE_VENT_DUCT_LEAKAGE,
            Din16798Mutation::ChangeVentDesignAirflow(_) => TAG_CHANGE_VENT_DESIGN_AIRFLOW,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Din16798Mutation::ChangeAnnex(p) => {
                write_json_bin(&mut out, &p.new_annex);
            }
            Din16798Mutation::ChangeThetaRm(p) => {
                write_json_bin(&mut out, &p.new_theta_rm_c);
            }
            Din16798Mutation::ChangeOutdoorCo2(p) => {
                write_json_bin(&mut out, &p.new_outdoor_co2_ppm);
            }
            Din16798Mutation::ChangeEnvelopeN50(p) => {
                write_json_bin(&mut out, &p.new_envelope_n50_h_inv);
            }
            Din16798Mutation::ChangeEnvelopeVolume(p) => {
                write_json_bin(&mut out, &p.new_envelope_volume_m3);
            }
            Din16798Mutation::ChangeCellarArea(p) => {
                write_json_bin(&mut out, &p.new_cellar_area_m2);
            }
            Din16798Mutation::ChangeCellarVentilation(p) => {
                write_json_bin(&mut out, &p.new_cellar_ventilation_m3_h);
            }
            Din16798Mutation::ChangeNightSetback(p) => {
                write_json_bin(&mut out, &p.new_night_setback_k);
            }
            Din16798Mutation::InsertZone(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.zone);
            }
            Din16798Mutation::RemoveZone(p) => {
                write_json_bin(&mut out, &p.zone_id);
            }
            Din16798Mutation::ChangeZoneUsageType(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_usage_type);
            }
            Din16798Mutation::ChangeZoneFloorArea(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_floor_area_m2);
            }
            Din16798Mutation::ChangeZoneOccupants(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_occupants);
            }
            Din16798Mutation::ChangeZoneComfortCategory(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_comfort_category);
            }
            Din16798Mutation::ChangeZonePollutionClass(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_pollution_class);
            }
            Din16798Mutation::ChangeZoneComfortModel(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_comfort_model);
            }
            Din16798Mutation::ChangeZoneTOpWinter(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_t_op_winter_c);
            }
            Din16798Mutation::ChangeZoneTOpSummer(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_t_op_summer_c);
            }
            Din16798Mutation::ChangeZoneAirSpeed(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_air_speed_m_s);
            }
            Din16798Mutation::ChangeZoneClothing(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_clothing_clo);
            }
            Din16798Mutation::ChangeZoneMetabolicRate(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_metabolic_rate_met);
            }
            Din16798Mutation::ChangeZoneRh(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_rh_percent);
            }
            Din16798Mutation::ChangeZoneOutdoorAir(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_outdoor_air_supplied_m3_h);
            }
            Din16798Mutation::ChangeZoneCo2(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_co2_ppm);
            }
            Din16798Mutation::ChangeZoneIlluminance(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_illuminance_lx);
            }
            Din16798Mutation::ChangeZoneNoise(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_noise_db);
            }
            Din16798Mutation::ChangeZoneVentSystemId(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_vent_system_id);
            }
            Din16798Mutation::ChangeZoneTurbulence(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_turbulence_intensity_percent);
            }
            Din16798Mutation::ChangeZoneVentMethod(p) => {
                write_json_bin(&mut out, &p.zone_id);
                write_json_bin(&mut out, &p.new_vent_method);
            }
            Din16798Mutation::InsertVentSystem(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.vent);
            }
            Din16798Mutation::RemoveVentSystem(p) => {
                write_json_bin(&mut out, &p.vent_id);
            }
            Din16798Mutation::ChangeVentSystemType(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_system_type);
            }
            Din16798Mutation::ChangeVentSfp(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_sfp_w_m3_s);
            }
            Din16798Mutation::ChangeVentSfpClass(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_sfp_required_class);
            }
            Din16798Mutation::ChangeVentHeatRecovery(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_heat_recovery_eta);
            }
            Din16798Mutation::ChangeVentOdaClass(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_oda_class);
            }
            Din16798Mutation::ChangeVentFilterSup(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_filter_sup_class);
            }
            Din16798Mutation::ChangeVentInspection(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_years_since_inspection);
            }
            Din16798Mutation::ChangeVentDuctClass(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_duct_class);
            }
            Din16798Mutation::ChangeVentDuctLeakage(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_duct_leakage_m3_s_m2);
            }
            Din16798Mutation::ChangeVentDesignAirflow(p) => {
                write_json_bin(&mut out, &p.vent_id);
                write_json_bin(&mut out, &p.new_design_airflow_m3_h);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_CHANGE_ANNEX => {
                let new_annex = read_json_bin(&mut reader).map_err(|e| malformed("new_annex", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeAnnex(ChangeAnnex { new_annex }))
            }
            TAG_CHANGE_THETA_RM => {
                let new_theta_rm_c = read_json_bin(&mut reader).map_err(|e| malformed("new_theta_rm_c", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeThetaRm(ChangeThetaRm { new_theta_rm_c }))
            }
            TAG_CHANGE_OUTDOOR_CO2 => {
                let new_outdoor_co2_ppm = read_json_bin(&mut reader).map_err(|e| malformed("new_outdoor_co2_ppm", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeOutdoorCo2(ChangeOutdoorCo2 { new_outdoor_co2_ppm }))
            }
            TAG_CHANGE_ENVELOPE_N50 => {
                let new_envelope_n50_h_inv = read_json_bin(&mut reader).map_err(|e| malformed("new_envelope_n50_h_inv", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeEnvelopeN50(ChangeEnvelopeN50 { new_envelope_n50_h_inv }))
            }
            TAG_CHANGE_ENVELOPE_VOLUME => {
                let new_envelope_volume_m3 = read_json_bin(&mut reader).map_err(|e| malformed("new_envelope_volume_m3", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeEnvelopeVolume(ChangeEnvelopeVolume { new_envelope_volume_m3 }))
            }
            TAG_CHANGE_CELLAR_AREA => {
                let new_cellar_area_m2 = read_json_bin(&mut reader).map_err(|e| malformed("new_cellar_area_m2", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeCellarArea(ChangeCellarArea { new_cellar_area_m2 }))
            }
            TAG_CHANGE_CELLAR_VENTILATION => {
                let new_cellar_ventilation_m3_h = read_json_bin(&mut reader).map_err(|e| malformed("new_cellar_ventilation_m3_h", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeCellarVentilation(ChangeCellarVentilation { new_cellar_ventilation_m3_h }))
            }
            TAG_CHANGE_NIGHT_SETBACK => {
                let new_night_setback_k = read_json_bin(&mut reader).map_err(|e| malformed("new_night_setback_k", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeNightSetback(ChangeNightSetback { new_night_setback_k }))
            }
            TAG_INSERT_ZONE => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let zone = read_json_bin(&mut reader).map_err(|e| malformed("zone", reader.position(), e))?;
                Ok(Din16798Mutation::InsertZone(InsertZone { index, zone }))
            }
            TAG_REMOVE_ZONE => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                Ok(Din16798Mutation::RemoveZone(RemoveZone { zone_id }))
            }
            TAG_CHANGE_ZONE_USAGE_TYPE => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_usage_type = read_json_bin(&mut reader).map_err(|e| malformed("new_usage_type", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneUsageType(ChangeZoneUsageType { zone_id, new_usage_type }))
            }
            TAG_CHANGE_ZONE_FLOOR_AREA => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_floor_area_m2 = read_json_bin(&mut reader).map_err(|e| malformed("new_floor_area_m2", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneFloorArea(ChangeZoneFloorArea { zone_id, new_floor_area_m2 }))
            }
            TAG_CHANGE_ZONE_OCCUPANTS => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_occupants = read_json_bin(&mut reader).map_err(|e| malformed("new_occupants", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneOccupants(ChangeZoneOccupants { zone_id, new_occupants }))
            }
            TAG_CHANGE_ZONE_COMFORT_CATEGORY => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_comfort_category = read_json_bin(&mut reader).map_err(|e| malformed("new_comfort_category", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneComfortCategory(ChangeZoneComfortCategory { zone_id, new_comfort_category }))
            }
            TAG_CHANGE_ZONE_POLLUTION_CLASS => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_pollution_class = read_json_bin(&mut reader).map_err(|e| malformed("new_pollution_class", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZonePollutionClass(ChangeZonePollutionClass { zone_id, new_pollution_class }))
            }
            TAG_CHANGE_ZONE_COMFORT_MODEL => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_comfort_model = read_json_bin(&mut reader).map_err(|e| malformed("new_comfort_model", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneComfortModel(ChangeZoneComfortModel { zone_id, new_comfort_model }))
            }
            TAG_CHANGE_ZONE_T_OP_WINTER => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_t_op_winter_c = read_json_bin(&mut reader).map_err(|e| malformed("new_t_op_winter_c", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneTOpWinter(ChangeZoneTOpWinter { zone_id, new_t_op_winter_c }))
            }
            TAG_CHANGE_ZONE_T_OP_SUMMER => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_t_op_summer_c = read_json_bin(&mut reader).map_err(|e| malformed("new_t_op_summer_c", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneTOpSummer(ChangeZoneTOpSummer { zone_id, new_t_op_summer_c }))
            }
            TAG_CHANGE_ZONE_AIR_SPEED => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_air_speed_m_s = read_json_bin(&mut reader).map_err(|e| malformed("new_air_speed_m_s", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneAirSpeed(ChangeZoneAirSpeed { zone_id, new_air_speed_m_s }))
            }
            TAG_CHANGE_ZONE_CLOTHING => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_clothing_clo = read_json_bin(&mut reader).map_err(|e| malformed("new_clothing_clo", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneClothing(ChangeZoneClothing { zone_id, new_clothing_clo }))
            }
            TAG_CHANGE_ZONE_METABOLIC_RATE => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_metabolic_rate_met = read_json_bin(&mut reader).map_err(|e| malformed("new_metabolic_rate_met", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneMetabolicRate(ChangeZoneMetabolicRate { zone_id, new_metabolic_rate_met }))
            }
            TAG_CHANGE_ZONE_RH => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_rh_percent = read_json_bin(&mut reader).map_err(|e| malformed("new_rh_percent", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneRh(ChangeZoneRh { zone_id, new_rh_percent }))
            }
            TAG_CHANGE_ZONE_OUTDOOR_AIR => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_outdoor_air_supplied_m3_h = read_json_bin(&mut reader).map_err(|e| malformed("new_outdoor_air_supplied_m3_h", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneOutdoorAir(ChangeZoneOutdoorAir { zone_id, new_outdoor_air_supplied_m3_h }))
            }
            TAG_CHANGE_ZONE_CO2 => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_co2_ppm = read_json_bin(&mut reader).map_err(|e| malformed("new_co2_ppm", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneCo2(ChangeZoneCo2 { zone_id, new_co2_ppm }))
            }
            TAG_CHANGE_ZONE_ILLUMINANCE => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_illuminance_lx = read_json_bin(&mut reader).map_err(|e| malformed("new_illuminance_lx", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneIlluminance(ChangeZoneIlluminance { zone_id, new_illuminance_lx }))
            }
            TAG_CHANGE_ZONE_NOISE => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_noise_db = read_json_bin(&mut reader).map_err(|e| malformed("new_noise_db", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneNoise(ChangeZoneNoise { zone_id, new_noise_db }))
            }
            TAG_CHANGE_ZONE_VENT_SYSTEM_ID => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_vent_system_id = read_json_bin(&mut reader).map_err(|e| malformed("new_vent_system_id", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneVentSystemId(ChangeZoneVentSystemId { zone_id, new_vent_system_id }))
            }
            TAG_CHANGE_ZONE_TURBULENCE => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_turbulence_intensity_percent = read_json_bin(&mut reader).map_err(|e| malformed("new_turbulence_intensity_percent", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneTurbulence(ChangeZoneTurbulence { zone_id, new_turbulence_intensity_percent }))
            }
            TAG_CHANGE_ZONE_VENT_METHOD => {
                let zone_id = read_json_bin(&mut reader).map_err(|e| malformed("zone_id", reader.position(), e))?;
                let new_vent_method = read_json_bin(&mut reader).map_err(|e| malformed("new_vent_method", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeZoneVentMethod(ChangeZoneVentMethod { zone_id, new_vent_method }))
            }
            TAG_INSERT_VENT_SYSTEM => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let vent = read_json_bin(&mut reader).map_err(|e| malformed("vent", reader.position(), e))?;
                Ok(Din16798Mutation::InsertVentSystem(InsertVentSystem { index, vent }))
            }
            TAG_REMOVE_VENT_SYSTEM => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                Ok(Din16798Mutation::RemoveVentSystem(RemoveVentSystem { vent_id }))
            }
            TAG_CHANGE_VENT_SYSTEM_TYPE => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_system_type = read_json_bin(&mut reader).map_err(|e| malformed("new_system_type", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentSystemType(ChangeVentSystemType { vent_id, new_system_type }))
            }
            TAG_CHANGE_VENT_SFP => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_sfp_w_m3_s = read_json_bin(&mut reader).map_err(|e| malformed("new_sfp_w_m3_s", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentSfp(ChangeVentSfp { vent_id, new_sfp_w_m3_s }))
            }
            TAG_CHANGE_VENT_SFP_CLASS => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_sfp_required_class = read_json_bin(&mut reader).map_err(|e| malformed("new_sfp_required_class", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentSfpClass(ChangeVentSfpClass { vent_id, new_sfp_required_class }))
            }
            TAG_CHANGE_VENT_HEAT_RECOVERY => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_heat_recovery_eta = read_json_bin(&mut reader).map_err(|e| malformed("new_heat_recovery_eta", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentHeatRecovery(ChangeVentHeatRecovery { vent_id, new_heat_recovery_eta }))
            }
            TAG_CHANGE_VENT_ODA_CLASS => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_oda_class = read_json_bin(&mut reader).map_err(|e| malformed("new_oda_class", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentOdaClass(ChangeVentOdaClass { vent_id, new_oda_class }))
            }
            TAG_CHANGE_VENT_FILTER_SUP => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_filter_sup_class = read_json_bin(&mut reader).map_err(|e| malformed("new_filter_sup_class", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentFilterSup(ChangeVentFilterSup { vent_id, new_filter_sup_class }))
            }
            TAG_CHANGE_VENT_INSPECTION => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_years_since_inspection = read_json_bin(&mut reader).map_err(|e| malformed("new_years_since_inspection", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentInspection(ChangeVentInspection { vent_id, new_years_since_inspection }))
            }
            TAG_CHANGE_VENT_DUCT_CLASS => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_duct_class = read_json_bin(&mut reader).map_err(|e| malformed("new_duct_class", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentDuctClass(ChangeVentDuctClass { vent_id, new_duct_class }))
            }
            TAG_CHANGE_VENT_DUCT_LEAKAGE => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_duct_leakage_m3_s_m2 = read_json_bin(&mut reader).map_err(|e| malformed("new_duct_leakage_m3_s_m2", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentDuctLeakage(ChangeVentDuctLeakage { vent_id, new_duct_leakage_m3_s_m2 }))
            }
            TAG_CHANGE_VENT_DESIGN_AIRFLOW => {
                let vent_id = read_json_bin(&mut reader).map_err(|e| malformed("vent_id", reader.position(), e))?;
                let new_design_airflow_m3_h = read_json_bin(&mut reader).map_err(|e| malformed("new_design_airflow_m3_h", reader.position(), e))?;
                Ok(Din16798Mutation::ChangeVentDesignAirflow(ChangeVentDesignAirflow { vent_id, new_design_airflow_m3_h }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<Din16798Mutation> {
    vec![
        Din16798Mutation::ChangeAnnex(ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        Din16798Mutation::ChangeThetaRm(ChangeThetaRm { new_theta_rm_c: 1.0 }),
        Din16798Mutation::ChangeOutdoorCo2(ChangeOutdoorCo2 { new_outdoor_co2_ppm: 1.0 }),
        Din16798Mutation::ChangeEnvelopeN50(ChangeEnvelopeN50 { new_envelope_n50_h_inv: 1.0 }),
        Din16798Mutation::ChangeEnvelopeVolume(ChangeEnvelopeVolume { new_envelope_volume_m3: 1.0 }),
        Din16798Mutation::ChangeCellarArea(ChangeCellarArea { new_cellar_area_m2: 1.0 }),
        Din16798Mutation::ChangeCellarVentilation(ChangeCellarVentilation { new_cellar_ventilation_m3_h: 1.0 }),
        Din16798Mutation::ChangeNightSetback(ChangeNightSetback { new_night_setback_k: 1.0 }),
        Din16798Mutation::InsertZone(InsertZone { index: 0, zone: crate::ZoneDocument::default() }),
        Din16798Mutation::RemoveZone(RemoveZone { zone_id: "x".into() }),
        Din16798Mutation::ChangeZoneUsageType(ChangeZoneUsageType { zone_id: "x".into(), new_usage_type: "x".into() }),
        Din16798Mutation::ChangeZoneFloorArea(ChangeZoneFloorArea { zone_id: "x".into(), new_floor_area_m2: 1.0 }),
        Din16798Mutation::ChangeZoneOccupants(ChangeZoneOccupants { zone_id: "x".into(), new_occupants: 1 }),
        Din16798Mutation::ChangeZoneComfortCategory(ChangeZoneComfortCategory { zone_id: "x".into(), new_comfort_category: "x".into() }),
        Din16798Mutation::ChangeZonePollutionClass(ChangeZonePollutionClass { zone_id: "x".into(), new_pollution_class: "x".into() }),
        Din16798Mutation::ChangeZoneComfortModel(ChangeZoneComfortModel { zone_id: "x".into(), new_comfort_model: "x".into() }),
        Din16798Mutation::ChangeZoneTOpWinter(ChangeZoneTOpWinter { zone_id: "x".into(), new_t_op_winter_c: 1.0 }),
        Din16798Mutation::ChangeZoneTOpSummer(ChangeZoneTOpSummer { zone_id: "x".into(), new_t_op_summer_c: 1.0 }),
        Din16798Mutation::ChangeZoneAirSpeed(ChangeZoneAirSpeed { zone_id: "x".into(), new_air_speed_m_s: 1.0 }),
        Din16798Mutation::ChangeZoneClothing(ChangeZoneClothing { zone_id: "x".into(), new_clothing_clo: 1.0 }),
        Din16798Mutation::ChangeZoneMetabolicRate(ChangeZoneMetabolicRate { zone_id: "x".into(), new_metabolic_rate_met: 1.0 }),
        Din16798Mutation::ChangeZoneRh(ChangeZoneRh { zone_id: "x".into(), new_rh_percent: 1.0 }),
        Din16798Mutation::ChangeZoneOutdoorAir(ChangeZoneOutdoorAir { zone_id: "x".into(), new_outdoor_air_supplied_m3_h: 1.0 }),
        Din16798Mutation::ChangeZoneCo2(ChangeZoneCo2 { zone_id: "x".into(), new_co2_ppm: 1.0 }),
        Din16798Mutation::ChangeZoneIlluminance(ChangeZoneIlluminance { zone_id: "x".into(), new_illuminance_lx: 1.0 }),
        Din16798Mutation::ChangeZoneNoise(ChangeZoneNoise { zone_id: "x".into(), new_noise_db: 1.0 }),
        Din16798Mutation::ChangeZoneVentSystemId(ChangeZoneVentSystemId { zone_id: "x".into(), new_vent_system_id: "x".into() }),
        Din16798Mutation::InsertVentSystem(InsertVentSystem { index: 0, vent: crate::VentSystemDocument::default() }),
        Din16798Mutation::RemoveVentSystem(RemoveVentSystem { vent_id: "x".into() }),
        Din16798Mutation::ChangeVentSystemType(ChangeVentSystemType { vent_id: "x".into(), new_system_type: "x".into() }),
        Din16798Mutation::ChangeVentSfp(ChangeVentSfp { vent_id: "x".into(), new_sfp_w_m3_s: 1.0 }),
        Din16798Mutation::ChangeVentSfpClass(ChangeVentSfpClass { vent_id: "x".into(), new_sfp_required_class: 1 }),
        Din16798Mutation::ChangeVentHeatRecovery(ChangeVentHeatRecovery { vent_id: "x".into(), new_heat_recovery_eta: 1.0 }),
        Din16798Mutation::ChangeVentOdaClass(ChangeVentOdaClass { vent_id: "x".into(), new_oda_class: "x".into() }),
        Din16798Mutation::ChangeVentFilterSup(ChangeVentFilterSup { vent_id: "x".into(), new_filter_sup_class: "x".into() }),
        Din16798Mutation::ChangeVentInspection(ChangeVentInspection { vent_id: "x".into(), new_years_since_inspection: 1 }),
        Din16798Mutation::ChangeVentDuctClass(ChangeVentDuctClass { vent_id: "x".into(), new_duct_class: "x".into() }),
        Din16798Mutation::ChangeVentDuctLeakage(ChangeVentDuctLeakage { vent_id: "x".into(), new_duct_leakage_m3_s_m2: 1.0 }),
        Din16798Mutation::ChangeVentDesignAirflow(ChangeVentDesignAirflow { vent_id: "x".into(), new_design_airflow_m3_h: 1.0 }),
        Din16798Mutation::ChangeZoneTurbulence(ChangeZoneTurbulence { zone_id: "x".into(), new_turbulence_intensity_percent: 1.0 }),
        Din16798Mutation::ChangeZoneVentMethod(ChangeZoneVentMethod { zone_id: "x".into(), new_vent_method: "method_3_predefined_rates".into() }),
    ]
}
//#endregion 🔖️DemoCases

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
