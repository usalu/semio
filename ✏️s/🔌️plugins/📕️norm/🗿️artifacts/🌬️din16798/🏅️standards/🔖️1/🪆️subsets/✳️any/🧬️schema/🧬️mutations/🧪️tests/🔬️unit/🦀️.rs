//! 🧪 Every DIN EN 16798 mutation changes its intended leaf; inverse restores the fixture.

use crate::mutations::*;
use crate::{Din16798Mutation, Din16798Snapshot};
use protocol::MutationDiff;

fn apply(mutation: &Din16798Mutation, base: &Din16798Snapshot) -> Din16798Snapshot {
    let outcome = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(mutation, base);
    assert_eq!(outcome.worst_level(), None, "mutation should apply cleanly: {mutation:?}");
    MutationDiff::apply(outcome.diff(), base).expect("applies")
}

fn assert_mutates_and_restores(label: &str, base: &Din16798Snapshot, mutation: Din16798Mutation) {
    let after = apply(&mutation, base);
    assert_ne!(&after, base, "{label} must change the snapshot");
    let inverse = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(&mutation, base);
    let mut restored = after;
    for step in &inverse {
        restored = apply(step, &restored);
    }
    assert_eq!(&restored, base, "{label} inverse must restore the base snapshot");
}

fn all_sample_mutations(base: &Din16798Snapshot) -> Vec<(&'static str, Din16798Mutation)> {
    vec![
        ("change-annex", Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: if base.annex == crate::document::AnnexChoice::De { crate::document::AnnexChoice::En } else { crate::document::AnnexChoice::De } })),
        ("change-theta-rm", Din16798Mutation::ChangeThetaRm(change_theta_rm::ChangeThetaRm { new_theta_rm_c: base.theta_rm_c + 1.0 })),
        ("change-outdoor-co2", Din16798Mutation::ChangeOutdoorCo2(change_outdoor_co2::ChangeOutdoorCo2 { new_outdoor_co2_ppm: base.outdoor_co2_ppm + 10.0 })),
        ("change-envelope-n50", Din16798Mutation::ChangeEnvelopeN50(change_envelope_n50::ChangeEnvelopeN50 { new_envelope_n50_h_inv: base.envelope_n50_h_inv + 0.1 })),
        ("change-envelope-volume", Din16798Mutation::ChangeEnvelopeVolume(change_envelope_volume::ChangeEnvelopeVolume { new_envelope_volume_m3: base.envelope_volume_m3 + 1.0 })),
        ("change-cellar-area", Din16798Mutation::ChangeCellarArea(change_cellar_area::ChangeCellarArea { new_cellar_area_m2: base.cellar_area_m2 + 1.0 })),
        ("change-cellar-ventilation", Din16798Mutation::ChangeCellarVentilation(change_cellar_ventilation::ChangeCellarVentilation { new_cellar_ventilation_m3_h: base.cellar_ventilation_m3_h + 1.0 })),
        ("change-night-setback", Din16798Mutation::ChangeNightSetback(change_night_setback::ChangeNightSetback { new_night_setback_k: base.night_setback_k + 0.5 })),
        ("insert-zone", Din16798Mutation::InsertZone(insert_zone::InsertZone { index: base.zones.len(), zone: { let mut z = crate::ZoneDocument::default(); z.id = "zone-inserted".into(); z } })),
        ("remove-zone", Din16798Mutation::RemoveZone(remove_zone::RemoveZone { zone_id: base.zones[0].id.clone() })),
        ("change-zone-usage-type", Din16798Mutation::ChangeZoneUsageType(change_zone_usage_type::ChangeZoneUsageType { zone_id: base.zones[0].id.clone(), new_usage_type: "classroom".into() })),
        ("change-zone-floor-area", Din16798Mutation::ChangeZoneFloorArea(change_zone_floor_area::ChangeZoneFloorArea { zone_id: base.zones[0].id.clone(), new_floor_area_m2: base.zones[0].floor_area_m2 + 1.0 })),
        ("change-zone-occupants", Din16798Mutation::ChangeZoneOccupants(change_zone_occupants::ChangeZoneOccupants { zone_id: base.zones[0].id.clone(), new_occupants: base.zones[0].occupants + 1 })),
        ("change-zone-comfort-category", Din16798Mutation::ChangeZoneComfortCategory(change_zone_comfort_category::ChangeZoneComfortCategory { zone_id: base.zones[0].id.clone(), new_comfort_category: "III".into() })),
        ("change-zone-pollution-class", Din16798Mutation::ChangeZonePollutionClass(change_zone_pollution_class::ChangeZonePollutionClass { zone_id: base.zones[0].id.clone(), new_pollution_class: "non_low".into() })),
        ("change-zone-comfort-model", Din16798Mutation::ChangeZoneComfortModel(change_zone_comfort_model::ChangeZoneComfortModel { zone_id: base.zones[0].id.clone(), new_comfort_model: "adaptive".into() })),
        ("change-zone-t-op-winter", Din16798Mutation::ChangeZoneTOpWinter(change_zone_t_op_winter::ChangeZoneTOpWinter { zone_id: base.zones[0].id.clone(), new_t_op_winter_c: base.zones[0].t_op_winter_c + 0.5 })),
        ("change-zone-t-op-summer", Din16798Mutation::ChangeZoneTOpSummer(change_zone_t_op_summer::ChangeZoneTOpSummer { zone_id: base.zones[0].id.clone(), new_t_op_summer_c: base.zones[0].t_op_summer_c + 0.5 })),
        ("change-zone-air-speed", Din16798Mutation::ChangeZoneAirSpeed(change_zone_air_speed::ChangeZoneAirSpeed { zone_id: base.zones[0].id.clone(), new_air_speed_m_s: base.zones[0].air_speed_m_s + 0.05 })),
        ("change-zone-clothing", Din16798Mutation::ChangeZoneClothing(change_zone_clothing::ChangeZoneClothing { zone_id: base.zones[0].id.clone(), new_clothing_clo: base.zones[0].clothing_clo + 0.1 })),
        ("change-zone-metabolic-rate", Din16798Mutation::ChangeZoneMetabolicRate(change_zone_metabolic_rate::ChangeZoneMetabolicRate { zone_id: base.zones[0].id.clone(), new_metabolic_rate_met: base.zones[0].metabolic_rate_met + 0.1 })),
        ("change-zone-rh", Din16798Mutation::ChangeZoneRh(change_zone_rh::ChangeZoneRh { zone_id: base.zones[0].id.clone(), new_rh_percent: base.zones[0].rh_percent + 1.0 })),
        ("change-zone-outdoor-air", Din16798Mutation::ChangeZoneOutdoorAir(change_zone_outdoor_air::ChangeZoneOutdoorAir { zone_id: base.zones[0].id.clone(), new_outdoor_air_supplied_m3_h: base.zones[0].outdoor_air_supplied_m3_h + 10.0 })),
        ("change-zone-co2", Din16798Mutation::ChangeZoneCo2(change_zone_co2::ChangeZoneCo2 { zone_id: base.zones[0].id.clone(), new_co2_ppm: base.zones[0].co2_ppm + 10.0 })),
        ("change-zone-illuminance", Din16798Mutation::ChangeZoneIlluminance(change_zone_illuminance::ChangeZoneIlluminance { zone_id: base.zones[0].id.clone(), new_illuminance_lx: base.zones[0].illuminance_lx + 10.0 })),
        ("change-zone-noise", Din16798Mutation::ChangeZoneNoise(change_zone_noise::ChangeZoneNoise { zone_id: base.zones[0].id.clone(), new_noise_db: base.zones[0].noise_db + 1.0 })),
        ("change-zone-vent-system-id", Din16798Mutation::ChangeZoneVentSystemId(change_zone_vent_system_id::ChangeZoneVentSystemId { zone_id: base.zones[0].id.clone(), new_vent_system_id: "vent-other".into() })),
        ("insert-vent-system", Din16798Mutation::InsertVentSystem(insert_vent_system::InsertVentSystem { index: base.vent_systems.len(), vent: { let mut v = crate::VentSystemDocument::default(); v.id = "vent-inserted".into(); v } })),
        ("remove-vent-system", Din16798Mutation::RemoveVentSystem(remove_vent_system::RemoveVentSystem { vent_id: base.vent_systems[0].id.clone() })),
        ("change-vent-system-type", Din16798Mutation::ChangeVentSystemType(change_vent_system_type::ChangeVentSystemType { vent_id: base.vent_systems[0].id.clone(), new_system_type: "decentral_mech".into() })),
        ("change-vent-sfp", Din16798Mutation::ChangeVentSfp(change_vent_sfp::ChangeVentSfp { vent_id: base.vent_systems[0].id.clone(), new_sfp_w_m3_s: base.vent_systems[0].sfp_w_m3_s + 50.0 })),
        ("change-vent-sfp-class", Din16798Mutation::ChangeVentSfpClass(change_vent_sfp_class::ChangeVentSfpClass { vent_id: base.vent_systems[0].id.clone(), new_sfp_required_class: 4 })),
        ("change-vent-heat-recovery", Din16798Mutation::ChangeVentHeatRecovery(change_vent_heat_recovery::ChangeVentHeatRecovery { vent_id: base.vent_systems[0].id.clone(), new_heat_recovery_eta: (base.vent_systems[0].heat_recovery_eta - 0.05_f64).max(0.1) })),
        ("change-vent-oda-class", Din16798Mutation::ChangeVentOdaClass(change_vent_oda_class::ChangeVentOdaClass { vent_id: base.vent_systems[0].id.clone(), new_oda_class: "ODA1".into() })),
        ("change-vent-filter-sup", Din16798Mutation::ChangeVentFilterSup(change_vent_filter_sup::ChangeVentFilterSup { vent_id: base.vent_systems[0].id.clone(), new_filter_sup_class: "ePM1_80".into() })),
        ("change-vent-inspection", Din16798Mutation::ChangeVentInspection(change_vent_inspection::ChangeVentInspection { vent_id: base.vent_systems[0].id.clone(), new_years_since_inspection: base.vent_systems[0].years_since_inspection + 1 })),
        ("change-vent-duct-class", Din16798Mutation::ChangeVentDuctClass(change_vent_duct_class::ChangeVentDuctClass { vent_id: base.vent_systems[0].id.clone(), new_duct_class: "B".into() })),
        ("change-vent-duct-leakage", Din16798Mutation::ChangeVentDuctLeakage(change_vent_duct_leakage::ChangeVentDuctLeakage { vent_id: base.vent_systems[0].id.clone(), new_duct_leakage_m3_s_m2: base.vent_systems[0].duct_leakage_m3_s_m2 + 0.01 })),
        ("change-vent-design-airflow", Din16798Mutation::ChangeVentDesignAirflow(change_vent_design_airflow::ChangeVentDesignAirflow { vent_id: base.vent_systems[0].id.clone(), new_design_airflow_m3_h: base.vent_systems[0].design_airflow_m3_h + 50.0 })),
        ("change-zone-turbulence", Din16798Mutation::ChangeZoneTurbulence(change_zone_turbulence::ChangeZoneTurbulence { zone_id: base.zones[0].id.clone(), new_turbulence_intensity_percent: base.zones[0].turbulence_intensity_percent + 5.0 })),
        ("change-zone-vent-method", Din16798Mutation::ChangeZoneVentMethod(change_zone_vent_method::ChangeZoneVentMethod { zone_id: base.zones[0].id.clone(), new_vent_method: "method_3_predefined_rates".into() })),
    ]
}

#[semio_framework_async_macros::async_test]
async fn kinds_nonempty() {
    assert!(crate::artifact_schema::mutations::KINDS.len() >= 41);
}

#[semio_framework_async_macros::async_test]
async fn every_mutation_kind_changes_intended_leaf_and_inverse_restores() {
    let base = Din16798Snapshot::default();
    let cases = all_sample_mutations(&base);
    assert_eq!(cases.len(), 41, "expected one sample per mutation kind");
    for (label, mutation) in cases {
        assert_mutates_and_restores(label, &base, mutation);
    }
}
