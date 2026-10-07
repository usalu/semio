//! ⚖️ DIN EN 16798 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Din16798Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Din16798Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Din16798Mutation, protocol::ProtocolError> {
    Din16798Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
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
fn write_json_bin<T: semio_framework_value::ToValue>(out: &mut Vec<u8>, value: &T) {
    let bytes = semio_framework_pack_json::to_json_string(value);
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes.as_bytes());
}

fn read_json_bin<T: semio_framework_value::FromValue>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

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

const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
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
}
