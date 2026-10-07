//! ⚡️ DIN EN 16798 — hand-rolled OpText/OpBinary for `Din16798Mutation`.

use crate::artifact_schema::mutations::Din16798Mutation;

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
fn enc_json<T: semio_framework_value::ToValue>(value: &T) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(value))
}
fn dec_json<T: semio_framework_value::FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
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
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_din16798_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec















































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

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{Din16798Diff, Din16798Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::change_annex;
use crate::standards::v1::subsets::any::schema::mutations::change_theta_rm;
use crate::standards::v1::subsets::any::schema::mutations::change_outdoor_co2;
use crate::standards::v1::subsets::any::schema::mutations::change_envelope_n50;
use crate::standards::v1::subsets::any::schema::mutations::change_envelope_volume;
use crate::standards::v1::subsets::any::schema::mutations::change_cellar_area;
use crate::standards::v1::subsets::any::schema::mutations::change_cellar_ventilation;
use crate::standards::v1::subsets::any::schema::mutations::change_night_setback;
use crate::standards::v1::subsets::any::schema::mutations::insert_zone;
use crate::standards::v1::subsets::any::schema::mutations::remove_zone;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_usage_type;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_floor_area;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_occupants;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_comfort_category;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_pollution_class;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_comfort_model;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_t_op_winter;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_t_op_summer;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_air_speed;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_clothing;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_metabolic_rate;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_rh;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_outdoor_air;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_co2;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_illuminance;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_noise;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_vent_system_id;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_turbulence;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_vent_method;
use crate::standards::v1::subsets::any::schema::mutations::insert_vent_system;
use crate::standards::v1::subsets::any::schema::mutations::remove_vent_system;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_system_type;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_sfp;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_sfp_class;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_heat_recovery;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_oda_class;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_filter_sup;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_inspection;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_duct_class;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_duct_leakage;
use crate::standards::v1::subsets::any::schema::mutations::change_vent_design_airflow;

pub fn decode_din16798_mutation_json(text: &str) -> Result<Din16798Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
