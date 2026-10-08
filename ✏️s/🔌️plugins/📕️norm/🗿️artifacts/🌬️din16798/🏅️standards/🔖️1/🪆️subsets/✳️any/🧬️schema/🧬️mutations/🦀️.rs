//! 🧬️ Din16798 mutations — hierarchical zone/vent subject vocabulary.

use crate::{Din16798Diff, Din16798Snapshot};

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::{resolve_edit, EDIT_RULES};
pub use crate::standards::v1::subsets::any::io::mutation_bridge::{apply_din16798_mutation, inverse_din16798_mutation};

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

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture;

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
