//! 🧬️ Din16798 mutations — hierarchical zone/vent subject vocabulary.

use crate::{Din16798Diff, Din16798Snapshot};

use super::change_annex;
use super::change_theta_rm;
use super::change_outdoor_co2;
use super::change_envelope_n50;
use super::change_envelope_volume;
use super::change_cellar_area;
use super::change_cellar_ventilation;
use super::change_night_setback;
use super::insert_zone;
use super::remove_zone;
use super::change_zone_usage_type;
use super::change_zone_floor_area;
use super::change_zone_occupants;
use super::change_zone_comfort_category;
use super::change_zone_pollution_class;
use super::change_zone_comfort_model;
use super::change_zone_t_op_winter;
use super::change_zone_t_op_summer;
use super::change_zone_air_speed;
use super::change_zone_clothing;
use super::change_zone_metabolic_rate;
use super::change_zone_rh;
use super::change_zone_outdoor_air;
use super::change_zone_co2;
use super::change_zone_illuminance;
use super::change_zone_noise;
use super::change_zone_vent_system_id;
use super::change_zone_turbulence;
use super::change_zone_vent_method;
use super::insert_vent_system;
use super::remove_vent_system;
use super::change_vent_system_type;
use super::change_vent_sfp;
use super::change_vent_sfp_class;
use super::change_vent_heat_recovery;
use super::change_vent_oda_class;
use super::change_vent_filter_sup;
use super::change_vent_inspection;
use super::change_vent_duct_class;
use super::change_vent_duct_leakage;
use super::change_vent_design_airflow;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = Din16798Snapshot, diff = Din16798Diff, schema = "s.norm.din16798")]
pub enum Din16798Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeThetaRm(change_theta_rm::ChangeThetaRm),
    ChangeOutdoorCo2(change_outdoor_co2::ChangeOutdoorCo2),
    ChangeEnvelopeN50(change_envelope_n50::ChangeEnvelopeN50),
    ChangeEnvelopeVolume(change_envelope_volume::ChangeEnvelopeVolume),
    ChangeCellarArea(change_cellar_area::ChangeCellarArea),
    ChangeCellarVentilation(change_cellar_ventilation::ChangeCellarVentilation),
    ChangeNightSetback(change_night_setback::ChangeNightSetback),
    InsertZone(insert_zone::InsertZone),
    RemoveZone(remove_zone::RemoveZone),
    ChangeZoneUsageType(change_zone_usage_type::ChangeZoneUsageType),
    ChangeZoneFloorArea(change_zone_floor_area::ChangeZoneFloorArea),
    ChangeZoneOccupants(change_zone_occupants::ChangeZoneOccupants),
    ChangeZoneComfortCategory(change_zone_comfort_category::ChangeZoneComfortCategory),
    ChangeZonePollutionClass(change_zone_pollution_class::ChangeZonePollutionClass),
    ChangeZoneComfortModel(change_zone_comfort_model::ChangeZoneComfortModel),
    ChangeZoneTOpWinter(change_zone_t_op_winter::ChangeZoneTOpWinter),
    ChangeZoneTOpSummer(change_zone_t_op_summer::ChangeZoneTOpSummer),
    ChangeZoneAirSpeed(change_zone_air_speed::ChangeZoneAirSpeed),
    ChangeZoneClothing(change_zone_clothing::ChangeZoneClothing),
    ChangeZoneMetabolicRate(change_zone_metabolic_rate::ChangeZoneMetabolicRate),
    ChangeZoneRh(change_zone_rh::ChangeZoneRh),
    ChangeZoneOutdoorAir(change_zone_outdoor_air::ChangeZoneOutdoorAir),
    ChangeZoneCo2(change_zone_co2::ChangeZoneCo2),
    ChangeZoneIlluminance(change_zone_illuminance::ChangeZoneIlluminance),
    ChangeZoneNoise(change_zone_noise::ChangeZoneNoise),
    ChangeZoneVentSystemId(change_zone_vent_system_id::ChangeZoneVentSystemId),
    ChangeZoneTurbulence(change_zone_turbulence::ChangeZoneTurbulence),
    ChangeZoneVentMethod(change_zone_vent_method::ChangeZoneVentMethod),
    InsertVentSystem(insert_vent_system::InsertVentSystem),
    RemoveVentSystem(remove_vent_system::RemoveVentSystem),
    ChangeVentSystemType(change_vent_system_type::ChangeVentSystemType),
    ChangeVentSfp(change_vent_sfp::ChangeVentSfp),
    ChangeVentSfpClass(change_vent_sfp_class::ChangeVentSfpClass),
    ChangeVentHeatRecovery(change_vent_heat_recovery::ChangeVentHeatRecovery),
    ChangeVentOdaClass(change_vent_oda_class::ChangeVentOdaClass),
    ChangeVentFilterSup(change_vent_filter_sup::ChangeVentFilterSup),
    ChangeVentInspection(change_vent_inspection::ChangeVentInspection),
    ChangeVentDuctClass(change_vent_duct_class::ChangeVentDuctClass),
    ChangeVentDuctLeakage(change_vent_duct_leakage::ChangeVentDuctLeakage),
    ChangeVentDesignAirflow(change_vent_design_airflow::ChangeVentDesignAirflow),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-theta-rm",
    "change-outdoor-co2",
    "change-envelope-n50",
    "change-envelope-volume",
    "change-cellar-area",
    "change-cellar-ventilation",
    "change-night-setback",
    "insert-zone",
    "remove-zone",
    "change-zone-usage-type",
    "change-zone-floor-area",
    "change-zone-occupants",
    "change-zone-comfort-category",
    "change-zone-pollution-class",
    "change-zone-comfort-model",
    "change-zone-t-op-winter",
    "change-zone-t-op-summer",
    "change-zone-air-speed",
    "change-zone-clothing",
    "change-zone-metabolic-rate",
    "change-zone-rh",
    "change-zone-outdoor-air",
    "change-zone-co2",
    "change-zone-illuminance",
    "change-zone-noise",
    "change-zone-vent-system-id",
    "change-zone-turbulence",
    "change-zone-vent-method",
    "insert-vent-system",
    "remove-vent-system",
    "change-vent-system-type",
    "change-vent-sfp",
    "change-vent-sfp-class",
    "change-vent-heat-recovery",
    "change-vent-oda-class",
    "change-vent-filter-sup",
    "change-vent-inspection",
    "change-vent-duct-class",
    "change-vent-duct-leakage",
    "change-vent-design-airflow",
];

impl Din16798Mutation {
    pub fn from_snapshot(base: &Din16798Snapshot, target: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        let mut out = Vec::new();
        if base.annex != target.annex { out.push(Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex })); }
        if (base.theta_rm_c - target.theta_rm_c).abs() > f64::EPSILON { out.push(Din16798Mutation::ChangeThetaRm(change_theta_rm::ChangeThetaRm { new_theta_rm_c: target.theta_rm_c })); }
        if (base.outdoor_co2_ppm - target.outdoor_co2_ppm).abs() > f64::EPSILON { out.push(Din16798Mutation::ChangeOutdoorCo2(change_outdoor_co2::ChangeOutdoorCo2 { new_outdoor_co2_ppm: target.outdoor_co2_ppm })); }
        if base.zones != target.zones {
            // whole-list rebuild via remove+insert is sufficient for from_snapshot consumers
            for z in &base.zones { out.push(Din16798Mutation::RemoveZone(remove_zone::RemoveZone { zone_id: z.id.clone() })); }
            for (i, z) in target.zones.iter().enumerate() { out.push(Din16798Mutation::InsertZone(insert_zone::InsertZone { index: i, zone: z.clone() })); }
        }
        if base.vent_systems != target.vent_systems {
            for v in &base.vent_systems { out.push(Din16798Mutation::RemoveVentSystem(remove_vent_system::RemoveVentSystem { vent_id: v.id.clone() })); }
            for (i, v) in target.vent_systems.iter().enumerate() { out.push(Din16798Mutation::InsertVentSystem(insert_vent_system::InsertVentSystem { index: i, vent: v.clone() })); }
        }
        out
    }
}

pub fn decode_din16798_mutation_json(text: &str) -> Result<Din16798Mutation, String> {
    pack::json::from_json_str(text).map_err(|e| e.to_string())
}
pub fn apply_din16798_mutation(base: &Din16798Snapshot, mutation: &Din16798Mutation) -> Result<(Din16798Snapshot, Vec<String>), String> {
    let raised = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|m| format!("{:?}:{}", m.level, m.code.0)).collect();
    let applied = <Din16798Diff as protocol::MutationDiff<Din16798Snapshot>>::apply(raised.diff(), base).map_err(|e| format!("{e:?}"))?;
    Ok((applied, messages))
}
pub fn inverse_din16798_mutation(mutation: &Din16798Mutation, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(mutation, base)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
