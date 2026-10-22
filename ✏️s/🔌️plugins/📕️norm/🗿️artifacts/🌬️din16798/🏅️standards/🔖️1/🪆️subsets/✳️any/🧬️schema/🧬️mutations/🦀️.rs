//! 🧬️ Din16798 mutations — hierarchical zone/vent subject vocabulary.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES};

use semio_s_artifact_norm_din16798_mut_building::change_annex;
use semio_s_artifact_norm_din16798_mut_building::change_theta_rm;
use semio_s_artifact_norm_din16798_mut_building::change_outdoor_co2;
use semio_s_artifact_norm_din16798_mut_building::change_envelope_n50;
use semio_s_artifact_norm_din16798_mut_building::change_envelope_volume;
use semio_s_artifact_norm_din16798_mut_building::change_cellar_area;
use semio_s_artifact_norm_din16798_mut_building::change_cellar_ventilation;
use semio_s_artifact_norm_din16798_mut_building::change_night_setback;
use semio_s_artifact_norm_din16798_mut_zones::insert_zone;
use semio_s_artifact_norm_din16798_mut_zones::remove_zone;
use semio_s_artifact_norm_din16798_mut_zones::change_zone_usage_type;
use semio_s_artifact_norm_din16798_mut_zones::change_zone_floor_area;
use semio_s_artifact_norm_din16798_mut_zones::change_zone_occupants;
use semio_s_artifact_norm_din16798_mut_zones::change_zone_comfort_category;
use semio_s_artifact_norm_din16798_mut_zones::change_zone_pollution_class;
use semio_s_artifact_norm_din16798_mut_thermal::change_zone_comfort_model;
use semio_s_artifact_norm_din16798_mut_thermal::change_zone_t_op_winter;
use semio_s_artifact_norm_din16798_mut_thermal::change_zone_t_op_summer;
use semio_s_artifact_norm_din16798_mut_thermal::change_zone_air_speed;
use semio_s_artifact_norm_din16798_mut_thermal::change_zone_clothing;
use semio_s_artifact_norm_din16798_mut_thermal::change_zone_metabolic_rate;
use semio_s_artifact_norm_din16798_mut_thermal::change_zone_rh;
use semio_s_artifact_norm_din16798_mut_indoor::change_zone_outdoor_air;
use semio_s_artifact_norm_din16798_mut_indoor::change_zone_co2;
use semio_s_artifact_norm_din16798_mut_indoor::change_zone_illuminance;
use semio_s_artifact_norm_din16798_mut_indoor::change_zone_noise;
use semio_s_artifact_norm_din16798_mut_zones::change_zone_vent_system_id;
use semio_s_artifact_norm_din16798_mut_thermal::change_zone_turbulence;
use semio_s_artifact_norm_din16798_mut_zones::change_zone_vent_method;
use semio_s_artifact_norm_din16798_mut_ventilation::insert_vent_system;
use semio_s_artifact_norm_din16798_mut_ventilation::remove_vent_system;
use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_system_type;
use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_sfp;
use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_sfp_class;
use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_heat_recovery;
use semio_s_artifact_norm_din16798_mut_air_hygiene::change_vent_oda_class;
use semio_s_artifact_norm_din16798_mut_air_hygiene::change_vent_filter_sup;
use semio_s_artifact_norm_din16798_mut_air_hygiene::change_vent_inspection;
use semio_s_artifact_norm_din16798_mut_air_hygiene::change_vent_duct_class;
use semio_s_artifact_norm_din16798_mut_air_hygiene::change_vent_duct_leakage;
use semio_s_artifact_norm_din16798_mut_ventilation::change_vent_design_airflow;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = Din16798Snapshot, diff = Din16798Diff, schema = "s.norm.din16798", reference = Din16798Snapshot, reference_diff = Din16798Diff)]
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

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff and inverse through the central applier.
pub mod mutation_bridge {
/// 🌉️ Applies one mutation to `base` through the central applier, returning the next snapshot and the raised `<Level>:<code>` messages.
pub fn apply_din16798_mutation(base: &semio_s_artifact_norm_din16798_model::Din16798Snapshot, mutation: &crate::Din16798Mutation) -> Result<(semio_s_artifact_norm_din16798_model::Din16798Snapshot, Vec<String>), String> {
    let raised = <crate::Din16798Mutation as protocol::ColdMutation<semio_s_artifact_norm_din16798_model::Din16798Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|m| format!("{:?}:{}", m.level, m.code.0)).collect();
    let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|e| format!("{e:?}"))?;
    Ok((applied, messages))
}

/// ↩️ The mutation rows that undo `mutation` applied to `base`, in store order.
pub fn inverse_din16798_mutation(mutation: &crate::Din16798Mutation, base: &semio_s_artifact_norm_din16798_model::Din16798Snapshot) -> Result<Vec<crate::Din16798Mutation>, semio_framework_value::ValueError> {
    <crate::Din16798Mutation as protocol::ColdMutation<semio_s_artifact_norm_din16798_model::Din16798Snapshot>>::inverse(mutation, base)
}
}
pub use crate::mutation_bridge::*;

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod mutation_leaf_tests;
//#endregion 🪢️Mounts
