//! ⚖️ DIN V 18599 / GEG compliance numeric + remedy tests.

use semio_s_plugin_norm_app_surface::{get_value_at_path, parse_path, set_value_at_path};
use semio_framework_value::ToValue;

use semio_s_plugin_norm_compliance::{CheckReport, CheckStatus};
use crate::{cooling_utilization, derive_balance, heating_utilization, evaluate_document, geg_ht_prime_limit, h_t_prime, primary_energy_factor, required_u_for_ht_prime, transmission_loss_coefficient, usage_profile_row, ventilation_loss_coefficient, zone_balances, DIN_V_18599_1_RHO_CA, DIN_V_18599_2_INFILTRATION_N_INF};
use semio_s_artifact_norm_din18599_examples as examples;
use crate::{internal_gains_w_m2, residential_daily_gain_wh_m2, zone_heating_months};
use semio_s_artifact_norm_din18599_model::{Attachment, AutomationClass, BuildingCategory, CalculationMethod, Din18599Snapshot, ElementKind, UsageProfile, UseClass};

fn apply_path_f64(doc: &mut semio_s_artifact_norm_din18599_model::Din18599Snapshot, path: &str, value: f64) {
    if path == "deltaUWbWM2k" {
        doc.delta_u_wb_w_m2k = value;
        return;
    }
    if path == "renewables.pvAreaM2" {
        doc.renewables.pv_area_m2 = value;
        return;
    }
    if path == "dhw.storageLossKwhA" {
        doc.dhw.storage_loss_kwh_a = value;
        return;
    }
    if path == "dhw.distributionLossKwhA" {
        doc.dhw.distribution_loss_kwh_a = value;
        return;
    }
    if path == "dhw.specificDemandKwhPersonA" {
        doc.dhw.specific_demand_kwh_person_a = value;
        return;
    }
    if path == "lighting.controlFactor" {
        doc.lighting.control_factor = value;
        return;
    }
    if path == "heatedVolumeM3" {
        doc.heated_volume_m3 = value;
        return;
    }
    if path == "ventilation.heatRecoveryEta" {
        doc.ventilation.heat_recovery_eta = value;
        return;
    }
    if path == "heating.generationEfficiency" {
        doc.heating.generation_efficiency = value;
        return;
    }
    if let Some(rest) = path.strip_prefix("elements[id=") {
        let (id, field) = rest.split_once("].").expect("elements[id=…].field");
        if let Some(el) = doc.elements.iter_mut().find(|e| e.id == id) {
            match field {
                "uValueWM2k" => el.u_value_w_m2k = value,
                "gValue" => el.g_value = value,
                _ => panic!("unsupported element field {field}"),
            }
            return;
        }
    }
    if let Some(rest) = path.strip_prefix("zones[id=") {
        let (id, field) = rest.split_once("].").expect("zones[id=…].field");
        if let Some(z) = doc.zones.iter_mut().find(|z| z.id == id) {
            match field {
                "lightingPowerWM2" => z.lighting_power_w_m2 = value,
                "volumeM3" => z.volume_m3 = value,
                "areaM2" => z.area_m2 = value,
                "internalGainsWM2" => z.internal_gains_w_m2 = value,
                "thetaIHeatC" => z.theta_i_heat_c = value,
                "thetaICoolC" => z.theta_i_cool_c = value,
                _ => panic!("unsupported zone field {field}"),
            }
            return;
        }
    }
}

fn apply_path_text(doc: &mut semio_s_artifact_norm_din18599_model::Din18599Snapshot, path: &str, value: &str) {
    if path == "automationClass" {
        doc.automation_class = match value {
            "A" => AutomationClass::A,
            "B" => AutomationClass::B,
            "C" => AutomationClass::C,
            "D" => AutomationClass::D,
            _ => panic!("automation {value}"),
        };
        return;
    }
    if path == "heating.energyCarrier" {
        doc.heating.energy_carrier = value.to_string();
    }
}

#[test]
fn ventilation_hv_matches_hand_derivation() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    doc.ventilation.airflow_m3_h = 120.0;
    doc.ventilation.heat_recovery_eta = 0.0;
    let row = usage_profile_row(doc.zones[0].usage_profile);
    let expected = DIN_V_18599_1_RHO_CA
        * (120.0 + row.outdoor_air_change_1_h * doc.zones[0].volume_m3 + DIN_V_18599_2_INFILTRATION_N_INF * doc.heated_volume_m3);
    let h_v = ventilation_loss_coefficient(&doc);
    assert!((h_v - expected).abs() < 1e-9, "H_V={h_v} expected={expected}");
}

#[test]
fn potsdam_compliant_house_passes_geg_gates() {
    let doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    let report = evaluate_document(&doc);
    assert!(report.failing().next().is_none(), "unexpected failure: {:?}", report.failing().next().map(|c| &c.id));
    let ht = h_t_prime(&doc);
    let limit = geg_ht_prime_limit(&doc).expect("positive envelope area");
    assert!(ht <= limit, "H'T={ht} limit={limit}");
    let d = derive_balance(&doc);
    assert!(d.q_p_kwh <= d.q_p_limit_kwh, "Q_P={} limit={}", d.q_p_kwh, d.q_p_limit_kwh);
}

#[test]
fn noncompliant_house_fails_ht_and_qp_with_remedies() {
    let doc = semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house();
    let report = evaluate_document(&doc);
    assert!(!report.complies());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.iter().any(|c| c.id.contains("ht-prime") || c.id.contains("qp")));
    for fail in fails.iter().filter(|c| c.remedies.iter().any(|r| r.applicable)) {
        assert!(!fail.remedies.is_empty());
        for remedy in fail.remedies.iter().filter(|r| r.applicable) {
            assert!(!remedy.target.path.contains("elements[0]") && !remedy.target.path.contains("elements[1]"), "positional path {}", remedy.target.path);
            if remedy.target.path.starts_with("elements[") {
                assert!(remedy.target.path.contains("[id="), "{}", remedy.target.path);
                assert!(!remedy.target.entity_id.is_empty(), "entity_id for {}", remedy.target.path);
            }
        }
    }
}

#[test]
fn remedy_lowering_worst_u_improves_ht_prime() {
    let doc = semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house();
    let limit = geg_ht_prime_limit(&doc).expect("positive envelope area");
    assert!(h_t_prime(&doc) > limit);
    let mut fixed = doc.clone();
    for el in &mut fixed.elements {
        el.u_value_w_m2k = crate::reference_u(el);
    }
    fixed.delta_u_wb_w_m2k = 0.05;
    assert!(h_t_prime(&fixed) <= limit + 1e-6, "after reference envelope H'T={} limit={}", h_t_prime(&fixed), limit);
    let (_, u_req) = required_u_for_ht_prime(&doc, limit).expect("worst element");
    assert!(u_req < doc.elements.iter().map(|e| e.u_value_w_m2k).fold(0.0, f64::max));
}

#[test]
fn remedy_law_apply_required_u_makes_ht_check_pass() {
    let doc = semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house();
    let report = evaluate_document(&doc);
    let ht = report.checks.iter().find(|c| c.id == "din18599.geg.ht-prime").expect("ht check");
    assert_eq!(ht.status, CheckStatus::Fail);
    let mut fixed = doc.clone();
    for remedy in ht.remedies.iter().filter(|r| r.applicable) {
        apply_path_f64(&mut fixed, &remedy.target.path, remedy.required.value);
    }
    let again = evaluate_document(&fixed);
    let ht2 = again.checks.iter().find(|c| c.id == "din18599.geg.ht-prime").unwrap();
    assert!(ht2.utilization <= 1.0 + 1e-6, "u={}", ht2.utilization);
}

#[test]
fn renewable_electricity_credit_is_bounded_month_by_month() {
    let vectors = include_str!("../../../🧪️tests/🧫️fixtures/☀️monthly-renewable-credit/🔣️.tsv");
    for row in vectors.lines().skip(1) {
        let columns: Vec<_> = row.split('\t').collect();
        let expected = columns[3].parse::<f64>().ok();
        assert_eq!(crate::monthly_renewable_electricity_credit_kwh(
            columns[1].parse().unwrap(), columns[2].parse().unwrap(),
        ), expected, "{}", columns[0]);
    }
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    doc.renewables.pv_area_m2 = 10000.0;
    let balance = derive_balance(&doc);
    assert!(balance.renewable_electricity_months.iter().all(|row| row.credit_kwh <= row.eligible_electricity_kwh));
    assert!(balance.renewable_electricity_months.iter().any(|row| row.credit_kwh < row.generation_kwh));
    assert_eq!(balance.q_f_light_kwh, 0.0);
    let before = balance.q_p_kwh;
    doc.renewables.solar_thermal_kwh_a += 5000.0;
    assert_eq!(derive_balance(&doc).q_p_kwh, before);
    let report = evaluate_document(&doc);
    let renewable = report.checks.iter().find(|c| c.id == "din18599.geg.renewable-electricity").expect("renewable credit");
    assert_eq!(renewable.status, CheckStatus::UnverifiedBasis);
    assert_eq!(renewable.computed.value, balance.q_pv_kwh);
}

#[test]
fn remedy_law_automation_oneof_makes_automation_pass() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    doc.automation_class = AutomationClass::D;
    let report = evaluate_document(&doc);
    let auto = report.checks.iter().find(|c| c.id == "din18599.11.automation").unwrap();
    assert_eq!(auto.status, CheckStatus::Fail);
    let remedy = auto.remedies.iter().find(|r| r.applicable).expect("automation remedy");
    let choice = remedy.options.first().expect("oneof option");
    apply_path_text(&mut doc, &remedy.target.path, choice);
    let again = evaluate_document(&doc);
    let auto2 = again.checks.iter().find(|c| c.id == "din18599.11.automation").unwrap();
    assert_eq!(auto2.status, CheckStatus::Pass);
}

#[test]
fn primary_energy_factor_table_geg_annex4() {
    let fixture = include_str!("../../../🧪️tests/🧫️fixtures/💡primary-energy-factors/🔣️.tsv");
    let rows = crate::geg_anlage4_primary_energy_factors::ROWS;
    let mut cases = fixture.lines().skip(1);
    for (index, factor) in rows.iter().enumerate() {
        let mut cells = cases.next().unwrap().split('\t');
        assert_eq!(cells.next().unwrap().parse::<usize>().unwrap(), index + 1);
        assert_eq!(cells.next().unwrap(), factor.carrier);
        let expected = cells.next().unwrap().parse::<f64>().ok();
        assert_eq!(expected, factor.factor);
        if let Some(value) = expected {
            assert_eq!(primary_energy_factor(factor.carrier), value);
        }
        assert_eq!(crate::has_direct_primary_energy_factor(factor.carrier), index < 9);
    }
    assert!(cases.next().is_none());
    assert!(!crate::has_direct_primary_energy_factor("district_heating"));
    assert!(!crate::has_direct_primary_energy_factor("unnamed"));
    assert!(crate::uses_grid_electricity("heat_pump_electric"));
}

#[test]
fn energy_reports_unverified_carrier_when_a_supply_factor_is_needed() {
    let mut doc = Din18599Snapshot::default();
    doc.dhw.energy_carrier = "district_heating".into();
    let report = evaluate_document(&doc);
    let basis = report.checks.iter().find(|check| check.id == "din18599.geg.primary-energy-factor-basis").unwrap();
    assert_eq!(basis.subject.path, "dhw.energyCarrier");
    assert_eq!(basis.status, CheckStatus::UnverifiedBasis);
    let primary = report.checks.iter().find(|check| check.id == "din18599.geg.qp").unwrap();
    assert_eq!(primary.status, CheckStatus::UnverifiedBasis);
}

#[test]
fn transmission_ht_increases_with_u() {
    let good = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    let bad = semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house();
    assert!(transmission_loss_coefficient(&bad) > transmission_loss_coefficient(&good));
}

#[test]
fn automation_class_d_fails() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    doc.automation_class = AutomationClass::D;
    let report = evaluate_document(&doc);
    let auto = report.checks.iter().find(|c| c.id == "din18599.11.automation").unwrap();
    assert_eq!(auto.status, CheckStatus::Fail);
    assert!(!auto.remedies.is_empty());
}

#[test]
fn q_p_ref_changes_with_geometry() {
    let small = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    let mut large = small.clone();
    large.net_floor_area_m2 *= 2.0;
    large.heated_volume_m3 *= 2.0;
    for zone in &mut large.zones {
        zone.area_m2 *= 2.0;
        zone.volume_m3 *= 2.0;
    }
    for el in &mut large.elements {
        el.area_m2 *= 2.0;
    }
    let q_small = derive_balance(&small).q_p_ref_kwh;
    let q_large = derive_balance(&large).q_p_ref_kwh;
    assert!(q_small > 0.0);
    assert!((q_large - q_small).abs() > 1.0, "small={q_small} large={q_large}");
    assert!(q_large > q_small);
}

#[test]
fn q_p_ref_changes_when_only_wall_area_grows() {
    let base = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    let mut wider = base.clone();
    for el in &mut wider.elements {
        if el.kind == ElementKind::Wall {
            el.area_m2 *= 1.5;
        }
    }
    let q_base = derive_balance(&base).q_p_ref_kwh;
    let q_wider = derive_balance(&wider).q_p_ref_kwh;
    assert!(q_base > 0.0);
    assert!((q_wider - q_base).abs() > 0.5, "wall-only geometry must move Q_P,Ref (base={q_base}, wider={q_wider})");
    assert!(q_wider > q_base);
}

#[test]
fn example_assets_decode_and_match_claimed_verdict() {
    for (id, text, expect_comply) in [
        ("demo", examples::demo::PRIMARY_TEXT, true),
        ("compliant-detached", examples::compliant_detached::PRIMARY_TEXT, true),
        ("noncompliant-detached", examples::noncompliant_detached::PRIMARY_TEXT, false),
        ("compliant-two-zone", examples::compliant_two_zone::PRIMARY_TEXT, true),
        ("cooled-office", examples::cooled_office::PRIMARY_TEXT, false),
    ] {
        let doc = <semio_s_artifact_norm_din18599_model::Din18599Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).unwrap_or_else(|e| panic!("{id} parse: {e}"));
        let report = evaluate_document(&doc);
        if expect_comply {
            assert!(report.failing().next().is_none(), "{id}: unexpected failure {:?}", report.failing().next().map(|c| &c.id));
        } else if id == "cooled-office" {
            assert!(!report.complies(), "{id} requires a verified ground-floor area");
            assert!(report.checks.iter().any(|c| c.id.starts_with("din18599.geg.mean-u.1.") && c.status == CheckStatus::UnverifiedBasis));
        } else {
            assert!(!report.complies(), "{id} should fail");
            assert!(report.failing().count() >= 2, "{id} fail_count={}", report.failing().count());
        }
    }
}

#[test]
fn residential_hot_water_matches_table_4_fixtures() {
    let cases = include_str!("../../../🧪️tests/🧫️fixtures/🚿residential-hot-water/🔣️.tsv");
    for row in cases.lines().skip(1) {
        let mut cells = row.split('\t');
        let area = cells.next().unwrap().parse().unwrap();
        let dwellings = cells.next().unwrap().parse().unwrap();
        let expected = cells.next().unwrap();
        let estimate = crate::residential_dhw_useful_kwh_a(area, dwellings);
        if expected == "scope" {
            assert_eq!(estimate, None, "{row}");
        } else {
            let expected = expected.parse::<f64>().unwrap();
            assert!((estimate.unwrap() - expected).abs() < 1e-9, "{row}");
        }
    }
}

#[test]
fn residential_hot_water_uses_geometry_in_live_balance() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    doc.net_floor_area_m2 = 140.0;
    doc.dhw.dwelling_count = 1;
    doc.dhw.specific_demand_kwh_person_a = 900.0;
    doc.zones[0].occupants = 12;
    assert_eq!(crate::derive_balance(&doc).q_w_useful_kwh, 1330.0);
    doc.dhw.dwelling_count = 2;
    assert_eq!(crate::derive_balance(&doc).q_w_useful_kwh, 1820.0);
    doc.dhw.dwelling_count = 0;
    let report = evaluate_document(&doc);
    for id in ["din18599.10.residential-dhw", "din18599.8.dhw", "din18599.geg.qp"] {
        let result = report.checks.iter().find(|result| result.id == id).unwrap();
        assert_eq!(result.status, CheckStatus::UnverifiedBasis, "{id}");
    }
}

#[test]
fn dhw_remedy_inverts_profile_limit() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    doc.dhw.specific_demand_kwh_person_a = 900.0;
    doc.dhw.storage_loss_kwh_a = 800.0;
    doc.dhw.distribution_loss_kwh_a = 400.0;
    let report = evaluate_document(&doc);
    let dhw = report.checks.iter().find(|c| c.id == "din18599.8.dhw").unwrap();
    assert_eq!(dhw.status, CheckStatus::Fail);
    let mut fixed = doc.clone();
    for remedy in dhw.remedies.iter().filter(|r| r.applicable) {
        apply_path_f64(&mut fixed, &remedy.target.path, remedy.required.value);
    }
    let again = evaluate_document(&fixed);
    let dhw2 = again.checks.iter().find(|c| c.id == "din18599.8.dhw").unwrap();
    assert!(dhw2.utilization <= 1.0 + 1e-6, "u={}", dhw2.utilization);
}

#[test]
fn norm_table_rows_match_cited_sources() {
    use crate::{
        din_v_18599_10_dhw_specific, din_v_18599_10_fan_hours, din_v_18599_10_zone_defaults, din_v_18599_11_automation_factor, din_v_18599_2_adjacency_fx, din_v_18599_2_solar_geometry_factors, din_v_18599_4_lighting_power_density, DIN_V_18599_1_DAYS_PER_MONTH, DIN_V_18599_1_RHO_CA, din_v_18599_2_c_wirk_wh_m2k, zone_heat_capacity_wh_k, DIN_V_18599_2_UTILIZATION_A0, DIN_V_18599_2_UTILIZATION_TAU0_H, DIN_V_18599_5_ETA_SYS_FLOOR, DIN_V_18599_5_ETA_SYS_MIN, DIN_V_18599_7_COOLING_TO_HEATING_RATIO, din_v_18599_7_g_remedy, din_v_18599_8_dhw_loss_fractions, geg_anlage1_tabelle1_heating_eta, GEG_SECTION_16_HT_PRIME_RATIO, geg_anlage1_reference_u, geg_anlage4_primary_energy_factors, GEG_ANLAGE1_DELTA_U_WB_REF, GEG_ANLAGE1_TABELLE1_ETA_WRG_REF, GEG_ANLAGE1_WINDOW_G_REF, GEG_SECTION10_QP_FACTOR_DEFAULT,
    };
    assert!((geg_anlage4_primary_energy_factors::NATURAL_GAS - 1.1).abs() < 1e-12);
    assert!((geg_anlage4_primary_energy_factors::ELECTRICITY_GRID - 1.8).abs() < 1e-12);
    assert!((geg_anlage4_primary_energy_factors::BIOMASS - 0.2).abs() < 1e-12);
    assert!((geg_anlage1_reference_u::WALL_AND_FLOOR_TO_OUTDOORS - 0.28).abs() < 1e-12);
    assert!((GEG_ANLAGE1_TABELLE1_ETA_WRG_REF - 0.80).abs() < 1e-12);
    assert!((GEG_ANLAGE1_DELTA_U_WB_REF - 0.05).abs() < 1e-12);
    assert!((geg_anlage1_tabelle1_heating_eta::GENERATION - 0.95).abs() < 1e-12);
    assert!((GEG_ANLAGE1_WINDOW_G_REF - 0.60).abs() < 1e-12);
    assert!((din_v_18599_4_lighting_power_density::OFFICE - 12.0).abs() < 1e-12);
    assert!((din_v_18599_10_fan_hours::RESIDENTIAL_WFH - 5000.0).abs() < 1e-12);
    assert!((din_v_18599_10_dhw_specific::RESIDENTIAL - 500.0).abs() < 1e-12);
    assert!((din_v_18599_8_dhw_loss_fractions::STORAGE_OF_USEFUL - 0.25).abs() < 1e-12);
    assert!((DIN_V_18599_5_ETA_SYS_MIN - 0.85).abs() < 1e-12);
    assert!((DIN_V_18599_7_COOLING_TO_HEATING_RATIO - 0.5).abs() < 1e-12);
    assert!((din_v_18599_7_g_remedy::REDUCE_FACTOR - 0.7).abs() < 1e-12);
    assert!((GEG_SECTION_16_HT_PRIME_RATIO - 1.0).abs() < 1e-12);
    assert!((din_v_18599_11_automation_factor::CLASS_A - 0.88).abs() < 1e-12);
    assert!((din_v_18599_2_adjacency_fx::GROUND - 0.6).abs() < 1e-12);
    assert!((din_v_18599_10_zone_defaults::THETA_I_HEAT_C - 20.0).abs() < 1e-12);
    assert!((din_v_18599_2_solar_geometry_factors::SOUTH_HORIZ - 1.0).abs() < 1e-12);
    assert!((DIN_V_18599_2_UTILIZATION_A0 - 1.0).abs() < 1e-12);
    assert!((DIN_V_18599_2_UTILIZATION_TAU0_H - 16.0).abs() < 1e-12);
    assert!((din_v_18599_2_c_wirk_wh_m2k::LIGHT - 50.0).abs() < 1e-12);
    assert!((din_v_18599_2_c_wirk_wh_m2k::MEDIUM - 90.0).abs() < 1e-12);
    assert!((din_v_18599_2_c_wirk_wh_m2k::HEAVY - 130.0).abs() < 1e-12);
    assert!((DIN_V_18599_1_RHO_CA - 0.34).abs() < 1e-12);
    assert!((DIN_V_18599_1_DAYS_PER_MONTH.iter().sum::<f64>() - 365.0).abs() < 1e-12);
    assert!((DIN_V_18599_5_ETA_SYS_FLOOR - 0.05).abs() < 1e-12);
    assert!((GEG_SECTION10_QP_FACTOR_DEFAULT - 0.55).abs() < 1e-12);
}

#[test]
fn remedy_law_apply_delta_u_wb_makes_ht_check_pass() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    doc.delta_u_wb_w_m2k = 0.55;
    let report = evaluate_document(&doc);
    let ht = report.checks.iter().find(|c| c.id == "din18599.geg.ht-prime").expect("ht");
    assert_eq!(ht.status, CheckStatus::Fail);
    assert_eq!(ht.subject.path, "deltaUWbWM2k");
    let remedy = ht.remedies.iter().find(|r| r.applicable && r.target.path == "deltaUWbWM2k").expect("ΔU_WB remedy on failing H′T");
    let mut tree = semio_framework_value::ToValue::to_value(&doc);
    set_value_at_path(&mut tree, &remedy.target.path, semio_framework_value::DslValue::float(remedy.required.value)).expect("set deltaUWbWM2k");
    apply_path_f64(&mut doc, &remedy.target.path, remedy.required.value);
    assert!((doc.delta_u_wb_w_m2k - remedy.required.value).abs() < 1e-12);
    let again = evaluate_document(&doc);
    let ht2 = again.checks.iter().find(|c| c.id == "din18599.geg.ht-prime").unwrap();
    assert!(ht2.utilization <= 1.0 + 1e-6, "u={} after ΔU_WB remedy", ht2.utilization);
    assert_eq!(ht2.status, CheckStatus::Pass);
}

#[test]
fn every_emitted_path_resolves_via_get_value_at_path() {
    let doc = semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house();
    let report = evaluate_document(&doc);
    let tree = semio_framework_value::ToValue::to_value(&doc);
    let mut saw_id = false;
    let mut resolved = 0usize;
    for check in &report.checks {
        if !check.subject.path.is_empty() {
            parse_path(&check.subject.path).unwrap_or_else(|e| panic!("parse subject {}: {e}", check.subject.path));
            get_value_at_path(&tree, &check.subject.path).unwrap_or_else(|e| panic!("resolve subject {}: {e}", check.subject.path));
            resolved += 1;
            if check.subject.path.contains("[id=") {
                saw_id = true;
            }
        }
        for remedy in &check.remedies {
            if remedy.target.path.is_empty() {
                continue;
            }
            parse_path(&remedy.target.path).unwrap_or_else(|e| panic!("parse remedy {}: {e}", remedy.target.path));
            get_value_at_path(&tree, &remedy.target.path).unwrap_or_else(|e| panic!("resolve remedy {}: {e}", remedy.target.path));
            resolved += 1;
            if remedy.target.path.contains("[id=") {
                saw_id = true;
            }
            if remedy.options.is_empty() {
                let mut probe = tree.clone();
                set_value_at_path(&mut probe, &remedy.target.path, semio_framework_value::DslValue::float(remedy.required.value))
                    .unwrap_or_else(|e| panic!("set_value_at_path {}: {e}", remedy.target.path));
            }
        }
    }
    assert!(resolved >= 4, "resolved={resolved}");
    assert!(saw_id, "expected at least one [id=…] entity path");
}

#[test]
fn two_zone_moving_element_changes_zone_balances() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house();
    let before = zone_balances(&doc, &doc.elements, doc.delta_u_wb_w_m2k);
    let living_before = before.iter().find(|z| z.zone_id == "zone-living").unwrap();
    let office_before = before.iter().find(|z| z.zone_id == "zone-office").unwrap();
    if let Some(el) = doc.elements.iter_mut().find(|e| e.id == "win-s") {
        el.zone_id = "zone-office".into();
    }
    let after = zone_balances(&doc, &doc.elements, doc.delta_u_wb_w_m2k);
    let living_after = after.iter().find(|z| z.zone_id == "zone-living").unwrap();
    let office_after = after.iter().find(|z| z.zone_id == "zone-office").unwrap();
    assert!((living_before.h_t - living_after.h_t).abs() > 0.5, "living H_T {} -> {}", living_before.h_t, living_after.h_t);
    assert!((office_before.h_t - office_after.h_t).abs() > 0.5, "office H_T {} -> {}", office_before.h_t, office_after.h_t);
    assert!(
        (living_before.q_h_nd_kwh - living_after.q_h_nd_kwh).abs() > 1.0
            || (living_before.q_c_nd_kwh - living_after.q_c_nd_kwh).abs() > 1.0,
        "living demand q_h {}→{} q_c {}→{}",
        living_before.q_h_nd_kwh,
        living_after.q_h_nd_kwh,
        living_before.q_c_nd_kwh,
        living_after.q_c_nd_kwh
    );
    assert!(
        (office_before.q_h_nd_kwh - office_after.q_h_nd_kwh).abs() > 1.0
            || (office_before.q_c_nd_kwh - office_after.q_c_nd_kwh).abs() > 1.0,
        "office demand q_h {}→{} q_c {}→{}",
        office_before.q_h_nd_kwh,
        office_after.q_h_nd_kwh,
        office_before.q_c_nd_kwh,
        office_after.q_c_nd_kwh
    );
}

#[test]
fn usage_profile_change_changes_zone_hours_and_gains() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    let q_l0 = derive_balance(&doc).q_l_kwh;
    let aux0 = derive_balance(&doc).q_f_aux_kwh;
    doc.zones[0].usage_profile = UsageProfile::Office;
    let q_l1 = derive_balance(&doc).q_l_kwh;
    let aux1 = derive_balance(&doc).q_f_aux_kwh;
    assert!((q_l1 - q_l0).abs() > 1.0, "lighting {q_l0} -> {q_l1}");
    assert!((aux1 - aux0).abs() > 0.1 || (q_l1 - q_l0).abs() > 1.0);
}

#[test]

fn every_editable_leaf_influences_a_check() {
    let fixtures: [(&str, semio_s_artifact_norm_din18599_model::Din18599Snapshot); 3] = [
        ("cooled_office_building", semio_s_artifact_norm_din18599_model::subjects::cooled_office_building()),
        ("compliant_two_zone_house", semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house()),
        ("compliant_detached_house", semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house()),
    ];
    for (fixture_name, base) in fixtures {
        assert_fixture_leaves_influence_checks(fixture_name, &base);
        assert_climate_payload_influences_checks(fixture_name, &base);
    }
}

fn check_sig(report: &CheckReport) -> Vec<(String, CheckStatus, i64, i64, i64)> {
    report
        .checks
        .iter()
        .map(|c| {
            (
                c.id.clone(),
                c.status,
                (c.computed.value * 1e6).round() as i64,
                (c.limit.value * 1e6).round() as i64,
                (c.utilization * 1e9).round() as i64,
            )
        })
        .collect()
}

fn assert_fixture_leaves_influence_checks(fixture_name: &str, base: &semio_s_artifact_norm_din18599_model::Din18599Snapshot) {
    let base_sig = check_sig(&evaluate_document(base));
    let value = serde_json::to_value(base).expect("json");
    let mut leaves = Vec::new();
    walk_leaves("", &value, &mut |path| {
        if path.is_empty() || path == "attachment" {
            return;
        }
        let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
        if residential_daily_gain_wh_m2(base).is_some() {
            let residential_zone_gain = base.zones.iter().enumerate().any(|(index, zone)| {
                zone.usage_profile == UsageProfile::WFH && path == format!("zones[{index}].internalGainsWM2")
            });
            if residential_zone_gain {
                return;
            }
        }
        if leaf == "labelEn" || leaf == "labelDe" || leaf == "id" || leaf.starts_with("id=") {
            return;
        }
        leaves.push(path.to_string());
    });
    assert!(!leaves.is_empty(), "{fixture_name}: expected editable leaves");
    let mut unchanged = Vec::new();
    for path in &leaves {
        let mut tree = serde_json::to_value(base).unwrap();
        let cur = resolve_json_path(&tree, path).cloned();
        let Some(cur) = cur else { continue };
        match cur {
            serde_json::Value::Number(n) => {
                let v = n.as_f64().unwrap_or(0.0);
                let nv = if path.contains("orientation") {
                    v + 90.0
                } else if path.contains("tilt") {
                    if v > 60.0 {
                        20.0
                    } else if v < 30.0 {
                        70.0
                    } else {
                        5.0
                    }
                } else if path.ends_with(".fc") || path.contains(".fc") {
                    if v >= 0.99 {
                        0.4
                    } else {
                        (v * 0.5).max(0.1)
                    }
                } else if path.contains("thetaI") {
                    v + 8.0
                } else if path.contains("pvArea") || path.contains("pvEfficiency") {
                    (v * 0.05).max(0.01)
                } else if v.abs() < 1e-12 {
                    1.0
                } else {
                    v * 2.0
                };
                set_json_number(&mut tree, path, nv);
            }
            serde_json::Value::Bool(b) => set_json_bool(&mut tree, path, !b),
            serde_json::Value::String(s) => {
                let ns = perturb_string(path, &s);
                set_json_string(&mut tree, path, &ns);
            }
            _ => continue,
        }
        let Ok(mut perturbed) = serde_json::from_value::<semio_s_artifact_norm_din18599_model::Din18599Snapshot>(tree) else {
            continue;
        };
        let sig = check_sig(&evaluate_document(&perturbed));
        if sig == base_sig {
            unchanged.push(path.clone());
        }
    }
    assert!(
        unchanged.is_empty(),
        "{fixture_name}: editable leaves with no check influence: {unchanged:?}"
    );
}

fn assert_climate_payload_influences_checks(fixture_name: &str, base: &semio_s_artifact_norm_din18599_model::Din18599Snapshot) {
    let base_sig = check_sig(&evaluate_document(base));
    let mut cold = base.clone();
    for t in &mut cold.climate.theta_e_c {
        *t -= 5.0;
    }
    cold.climate_table = semio_s_artifact_norm_din18599_model::din18599_climate_table_child(&cold.climate);
    let sig_t = check_sig(&evaluate_document(&cold));
    assert!(
        sig_t != base_sig,
        "{fixture_name}: climate θ_e payload must change Q_h/Q_c/q_p checks"
    );
    let mut bright = base.clone();
    for g in &mut bright.climate.g_h_w_m2 {
        *g = (*g * 2.0).max(50.0);
    }
    bright.climate_table = semio_s_artifact_norm_din18599_model::din18599_climate_table_child(&bright.climate);
    let sig_g = check_sig(&evaluate_document(&bright));
    assert!(
        sig_g != base_sig,
        "{fixture_name}: climate G_h payload must change Q_h/Q_c/q_p checks"
    );
}

fn perturb_string(path: &str, s: &str) -> String {
    if path == "useClass" {
        return if s == "ResidentialUpToTwoDwellings" { "ResidentialMultifamily".into() } else { "ResidentialUpToTwoDwellings".into() };
    }
    if path.ends_with("usageProfile") || path.contains("usageProfile") {
        return if s == "WFH" { "Office".into() } else { "WFH".into() };
    }
    if path.ends_with("zoneId") || path.contains(".zoneId") {
        return if s == "zone-living" { "zone-office".into() } else { "zone-living".into() };
    }
    if path.contains("childId") || path.ends_with("artifactId") {
        return format!("dangling-climate-{s}");
    }
    if path.contains("artifactKind") {
        return "s.stdio.bogus".into();
    }
    if path.ends_with("standard") && path.contains("climate") {
        return "v9".into();
    }
    if path.ends_with("subset") && path.contains("climate") {
        return "matrix".into();
    }
    if path.contains("energyCarrier") {
        return if s == "electricity" { "biomass".into() } else { "electricity".into() };
    }
    if s == "Residential" {
        "Office".into()
    } else if s == "Office" {
        "School".into()
    } else if s == "natural_gas" {
        "heating_oil".into()
    } else if s == "B" {
        "A".into()
    } else if s == "DetailedMonthly" {
        "Tabular".into()
    } else if s == "Detached" {
        "SemiDetached".into()
    } else if s == "Wall" {
        "Roof".into()
    } else if s == "Outdoor" {
        "Ground".into()
    } else {
        format!("{s}-x")
    }
}

fn walk_leaves(path: &str, value: &serde_json::Value, visit: &mut dyn FnMut(&str)) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let child = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                if matches!(v, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
                    walk_leaves(&child, v, visit);
                } else {
                    visit(&child);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                let child = format!("{path}[{i}]");
                walk_leaves(&child, v, visit);
            }
        }
        _ => visit(path),
    }
}

#[test]
fn duplicate_zone_id_fails_integrity() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house();
    assert!(doc.zones.len() >= 2);
    let first = doc.zones[0].id.clone();
    doc.zones[1].id = first;
    let report = evaluate_document(&doc);
    let fail = report
        .checks
        .iter()
        .find(|c| c.id.starts_with("din18599.integrity.duplicate.zones.") && c.status == CheckStatus::Fail)
        .expect("duplicate zone id Fail");
    assert!(!fail.remedies.is_empty());
    assert!(fail.remedies.iter().any(|r| r.applicable && !r.options.is_empty()));
    assert_ne!(fail.explanation.en, fail.explanation.de);
}

#[test]
fn duplicate_element_id_fails_integrity() {
    let mut doc = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house();
    assert!(doc.elements.len() >= 1);
    let dup = doc.elements[0].clone();
    doc.elements.push(dup);
    let report = evaluate_document(&doc);
    let fail = report
        .checks
        .iter()
        .find(|c| c.id.starts_with("din18599.integrity.duplicate.elements.") && c.status == CheckStatus::Fail)
        .expect("duplicate element id Fail");
    assert!(!fail.remedies.is_empty());
    assert!(fail.remedies.iter().any(|r| r.applicable && !r.options.is_empty()));
    assert_ne!(fail.explanation.en, fail.explanation.de);
}

#[test]
fn no_identical_en_de_explanations_in_committed_examples() {
    let examples = [
        semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house(),
        semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house(),
        semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house(),
        semio_s_artifact_norm_din18599_model::subjects::cooled_office_building(),
    ];
    let mut identical = Vec::new();
    for snap in examples {
        let report = evaluate_document(&snap);
        for c in &report.checks {
            if c.explanation.en == c.explanation.de && !c.explanation.en.is_empty() {
                let stripped: String = c.explanation.en.chars().filter(|ch| ch.is_alphabetic()).collect();
                if !stripped.is_empty() {
                    identical.push((c.id.clone(), c.explanation.en.clone()));
                }
            }
            for rem in &c.remedies {
                if rem.action.en == rem.action.de {
                    let stripped: String = rem.action.en.chars().filter(|ch| ch.is_alphabetic()).collect();
                    if !stripped.is_empty() {
                        identical.push((format!("{}#remedy", c.id), rem.action.en.clone()));
                    }
                }
            }
        }
    }
    assert!(identical.is_empty(), "identical en/de text: {identical:?}");
}

fn resolve_json_path<'a>(root: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut cur = root;
    for seg in path.split('.') {
        if let Some((name, rest)) = seg.split_once('[') {
            if !name.is_empty() {
                cur = cur.get(name)?;
            }
            let idx: usize = rest.trim_end_matches(']').parse().ok()?;
            cur = cur.get(idx)?;
        } else {
            cur = cur.get(seg)?;
        }
    }
    Some(cur)
}

fn set_json_number(root: &mut serde_json::Value, path: &str, value: f64) {
    if let Some(slot) = resolve_json_path_mut(root, path) {
        *slot = serde_json::json!(value);
    }
}
fn set_json_bool(root: &mut serde_json::Value, path: &str, value: bool) {
    if let Some(slot) = resolve_json_path_mut(root, path) {
        *slot = serde_json::json!(value);
    }
}
fn set_json_string(root: &mut serde_json::Value, path: &str, value: &str) {
    if let Some(slot) = resolve_json_path_mut(root, path) {
        *slot = serde_json::json!(value);
    }
}
fn resolve_json_path_mut<'a>(root: &'a mut serde_json::Value, path: &str) -> Option<&'a mut serde_json::Value> {
    let mut cur = root;
    for seg in path.split('.') {
        if let Some((name, rest)) = seg.split_once('[') {
            if !name.is_empty() {
                cur = cur.get_mut(name)?;
            }
            let idx: usize = rest.trim_end_matches(']').parse().ok()?;
            cur = cur.get_mut(idx)?;
        } else {
            cur = cur.get_mut(seg)?;
        }
    }
    Some(cur)
}

#[test]
fn the_utilisation_factors_reproduce_the_independent_mpmath_vectors() {
    let doc: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔮️cross-checks/🔣️.json")).expect("cross-check vectors");
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-9 * a.abs().max(b.abs());
    for row in doc["utilisationFactor"].as_array().expect("rows") {
        let (a, gamma) = (row["a"].as_f64().expect("a"), row["gamma"].as_f64().expect("gamma"));
        assert!(near(heating_utilization(gamma, a), row["etaHeating"].as_f64().expect("heating")), "heating a {a} gamma {gamma}");
        assert!(near(cooling_utilization(gamma, a), row["etaCooling"].as_f64().expect("cooling")), "cooling a {a} gamma {gamma}");
    }
    assert!((heating_utilization(1e300, 60.0) - 1e-300).abs() < 1e-290);
    assert!((cooling_utilization(1e-3, 60.0) - 1e-3).abs() < 1e-12);
}

#[test]
fn the_zone_heat_capacity_follows_the_thermal_mass_class_or_the_explicit_value_of_the_oracle_vectors() {
    use semio_s_artifact_norm_din18599_model::ThermalMassClass;
    let doc: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔮️cross-checks/🔣️.json")).expect("cross-check vectors");
    let mut zone = semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house().zones[0].clone();
    for row in doc["heatCapacity"].as_array().expect("rows") {
        zone.thermal_mass_class = match row["thermalMassClass"].as_str().expect("class") {
            "Light" => ThermalMassClass::Light,
            "Medium" => ThermalMassClass::Medium,
            "Heavy" => ThermalMassClass::Heavy,
            _ => ThermalMassClass::Custom,
        };
        zone.area_m2 = row["areaM2"].as_f64().expect("area");
        zone.c_wirk_wh_m2k = row["cWirkWhM2k"].as_f64().expect("c_wirk");
        let expected = row["capacityWhK"].as_f64().expect("capacity");
        assert!((zone_heat_capacity_wh_k(&zone) - expected).abs() <= 1e-12 * expected, "{row}");
    }
}

#[test]
fn residential_table_4_gains_flow_into_the_monthly_balance_and_report() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔮️cross-checks/🏘️residential-gains.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut doc = Din18599Snapshot::default();
        doc.dhw.dwelling_count = serde_json::from_value(case["dwellingCount"].clone()).unwrap();
        doc.zones[0].area_m2 = fixture["netFloorAreaM2"].as_f64().unwrap();
        doc.zones[0].internal_gains_w_m2 = 100.0;
        let (_, months) = &zone_heating_months(&doc)[0];
        assert_eq!(residential_daily_gain_wh_m2(&doc), case["dailyWhM2"].as_f64());
        assert_eq!(internal_gains_w_m2(&doc, &doc.zones[0]), case["averageWM2"].as_f64().unwrap());
        assert_eq!(months[0].q_i_kwh, case["januaryKWh"].as_f64().unwrap());
        assert_eq!(months[1].q_i_kwh, case["februaryKWh"].as_f64().unwrap());
        assert_eq!(months.iter().map(|m| m.q_i_kwh).sum::<f64>(), case["annualKWh"].as_f64().unwrap());
        let report = evaluate_document(&doc);
        let check = report.checks.iter().find(|check| check.id == "din18599.10.residential-internal-gains").unwrap();
        assert_eq!(check.status, CheckStatus::Pass);
        assert!(check.explanation.en.contains("Table 4"));
        assert!(check.explanation.de.contains("Tabelle 4"));
    }
}

#[test]
fn residential_table_4_rejects_unspecified_basis_and_gates_on_profile_and_method() {
    let mut doc = Din18599Snapshot::default();
    doc.dhw.dwelling_count = 0;
    let report = evaluate_document(&doc);
    let check = report.checks.iter().find(|check| check.id == "din18599.10.residential-internal-gains").unwrap();
    assert_eq!(check.status, CheckStatus::UnverifiedBasis);
    assert_eq!(check.subject.path, "dhw.dwellingCount");
    assert!(residential_daily_gain_wh_m2(&doc).is_none());

    doc.dhw.dwelling_count = 3;
    doc.zones[0].usage_profile = UsageProfile::Office;
    assert_eq!(internal_gains_w_m2(&doc, &doc.zones[0]), doc.zones[0].internal_gains_w_m2);
    doc.zones[0].usage_profile = UsageProfile::WFH;
    doc.method = CalculationMethod::Tabular;
    assert!(residential_daily_gain_wh_m2(&doc).is_none());
    doc.method = CalculationMethod::DetailedMonthly;
    doc.building_category = BuildingCategory::NonResidential;
    assert!(residential_daily_gain_wh_m2(&doc).is_none());
}

#[test]
fn residential_transmission_limit_uses_the_same_geometry_reference_envelope() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔮️cross-checks/🧱️residential-envelope.json")).unwrap();
    let close = |actual: f64, expected: f64| assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
    for case in fixture["cases"].as_array().unwrap() {
        let mut doc = Din18599Snapshot::default();
        let wall = doc.elements[0].clone();
        doc.delta_u_wb_w_m2k = fixture["deltaUWbWM2k"].as_f64().unwrap();
        doc.elements = fixture["components"].as_array().unwrap().iter().zip(case["actualUWM2k"].as_array().unwrap()).enumerate().map(|(index, (component, actual))| {
            let mut element = wall.clone();
            element.id = format!("wall-{index}");
            element.adjacency = serde_json::from_value(component["adjacency"].clone()).unwrap();
            element.area_m2 = component["areaM2"].as_f64().unwrap();
            element.u_value_w_m2k = actual.as_f64().unwrap();
            if let Some(expected) = component["referenceUWM2k"].as_f64() {
                close(crate::reference_u(&element), expected);
            }
            element
        }).collect();
        let expected_limit = fixture["limitWM2k"].as_f64().unwrap();
        let expected_ht = case["hTPrimeWM2k"].as_f64().unwrap();
        close(geg_ht_prime_limit(&doc).unwrap(), expected_limit);
        close(h_t_prime(&doc), expected_ht);

        doc.attachment = Attachment::MidTerrace;
        doc.net_floor_area_m2 = 400.0;
        let report = evaluate_document(&doc);
        let check = report.checks.iter().find(|check| check.id == "din18599.geg.ht-prime").unwrap();
        close(check.limit.value, expected_limit);
        close(check.computed.value, expected_ht);
        assert_eq!(format!("{:?}", check.status), case["status"].as_str().unwrap());
        assert!(check.explanation.en.contains("Annex 1"));
    }
}

#[test]
fn potsdam_geometry_and_monthly_irradiance_match_the_published_examples() {
    let fixture = include_str!("../../../🧪️tests/🧫️fixtures/☀️potsdam-irradiance/🔣️.tsv");
    for row in fixture.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        let month = fields[0].parse::<usize>().unwrap() - 1;
        let irradiance = crate::potsdam_irradiance_w_m2(month, fields[1].parse().unwrap(), fields[2].parse().unwrap());
        if fields[3] == "scope" {
            assert_eq!(irradiance, None, "{row}");
            continue;
        }
        let expected = fields[3].parse::<f64>().unwrap();
        let value = irradiance.unwrap();
        assert!((value - expected).abs() < 1e-9, "{row}: {value}");
        if let Ok(expected_yield) = fields[4].parse::<f64>() {
            let energy = value * crate::DIN_V_18599_1_DAYS_PER_MONTH[month] * 24.0 / 1000.0
                * 602.0 * 0.166 * 0.9 * 0.75;
            assert!((energy - expected_yield).abs() <= 0.005, "{row}: {energy}");
        }
    }
}

#[test]
fn tabular_method_scopes_building_category_and_withholds_primary_energy_verdict() {
    let mut doc = Din18599Snapshot::default();
    doc.method = CalculationMethod::Tabular;
    for (category, expected) in [
        (BuildingCategory::Residential, CheckStatus::UnverifiedBasis),
        (BuildingCategory::NonResidential, CheckStatus::NotApplicable),
    ] {
        doc.building_category = category;
        let report = evaluate_document(&doc);
        let tabular = report.checks.iter().find(|check| check.id == "din18599.12.tabular").unwrap();
        let primary = report.checks.iter().find(|check| check.id == "din18599.geg.qp").unwrap();
        assert_eq!(tabular.status, expected);
        assert_eq!(primary.status, CheckStatus::UnverifiedBasis);
    }
}

#[test]
fn annex3_groups_by_class_and_exact_heating_setpoint() {
    let mut doc = Din18599Snapshot::default();
    doc.building_category = BuildingCategory::NonResidential;
    let mut different_zone = doc.zones[0].clone();
    different_zone.id = "zone-second".into();
    different_zone.theta_i_heat_c = 21.0;
    doc.zones.push(different_zone);
    let mut wall = doc.elements[0].clone();
    wall.kind = semio_s_artifact_norm_din18599_model::ElementKind::Wall;
    wall.adjacency = semio_s_artifact_norm_din18599_model::Adjacency::Outdoor;
    wall.u_value_w_m2k = 0.5;
    let mut other_wall = wall.clone();
    other_wall.id = "other-wall".into();
    other_wall.zone_id = "zone-second".into();
    other_wall.u_value_w_m2k = 0.16;
    doc.elements = vec![wall, other_wall];
    let (groups, missing_basis) = crate::annex3_envelope_groups(&doc);
    assert!(!missing_basis);
    assert_eq!(groups.len(), 2);
    assert_ne!(groups[0].check_id(), groups[1].check_id());
    let report = evaluate_document(&doc);
    let results: Vec<_> = groups.iter().map(|group| {
        report.checks.iter().find(|check| check.id == group.check_id()).unwrap()
    }).collect();
    assert_eq!(results[0].status, CheckStatus::Fail);
    assert_eq!(results[1].status, CheckStatus::Pass);
    let remedy = &results[0].remedies[0];
    assert!((remedy.required.value - 0.28).abs() < 1e-12);
}

#[test]
fn annex3_normative_tables_and_weighting_match_fixtures() {
    use crate::{annex3_limit, annex3_mean_u, Annex3EnvelopeClass};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🏢annex-3-envelope/🔣️.json")).unwrap();
    for row in fixture.as_array().unwrap() {
        let class = Annex3EnvelopeClass::ALL.into_iter().find(|class| format!("{class:?}") == row["envelopeClass"].as_str().unwrap()).unwrap();
        let theta = row["setpointC"].as_f64().unwrap();
        let components: Vec<(f64, f64, f64)> = serde_json::from_value(row["components"].clone()).unwrap();
        assert_eq!(annex3_limit(class, theta), row["limitU"].as_f64());
        match (annex3_mean_u(components), row["meanU"].as_f64()) {
            (Some(mean), Some(expected)) => assert!((mean - expected).abs() < 1e-12, "{row}"),
            (None, None) => {}
            mismatch => panic!("Unexpected mean {mismatch:?} for {row}"),
        }
    }
}

#[test]
fn annex2_reference_preserves_geometry_and_shading() {
    use semio_s_artifact_norm_din18599_model::{Adjacency, ElementKind};
    let mut doc = Din18599Snapshot::default();
    doc.building_category = BuildingCategory::NonResidential;
    doc.zones[0].theta_i_heat_c = 18.0;
    let mut element = doc.elements[0].clone();
    element.kind = ElementKind::CurtainWall;
    element.adjacency = Adjacency::Outdoor;
    element.fc = 0.45;
    let reference = crate::energy_reference_element(&doc, &element);
    assert_eq!(reference.u_value_w_m2k, 1.9);
    assert_eq!(reference.g_value, 0.60);
    assert_eq!(reference.area_m2, element.area_m2);
    assert_eq!(reference.fc, element.fc);
    let roof = crate::annex2_reference_values(ElementKind::RooflightStrip, Adjacency::Outdoor, 19.0).unwrap();
    assert_eq!(roof.u_value_w_m2k, 2.4);
    assert_eq!(roof.solar_transmittance, Some(0.55));
    assert_eq!(roof.visible_transmittance, Some(0.48));
}

#[test]
fn annex2_bridge_covers_only_listed_exterior_components() {
    use semio_s_artifact_norm_din18599_model::{Adjacency, ElementKind};
    let doc = Din18599Snapshot::default();
    let mut wall = doc.elements[0].clone();
    wall.kind = ElementKind::Wall;
    wall.adjacency = Adjacency::Outdoor;
    wall.area_m2 = 20.0;
    let mut curtain_wall = wall.clone();
    curtain_wall.kind = ElementKind::CurtainWall;
    let mut internal_wall = wall.clone();
    internal_wall.adjacency = Adjacency::Heated;
    let components = [&wall, &curtain_wall, &internal_wall];
    assert_eq!(crate::annex2_reference_bridge_w_k(&components, 19.0), 1.0);
    assert_eq!(crate::annex2_reference_bridge_w_k(&components, 18.0), 2.0);
}
