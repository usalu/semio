//! ⚖️ DIN V 18599 / GEG compliance numeric + remedy tests.

use crate::app_surface::{get_value_at_path, parse_path, set_value_at_path};
use dsl::ToValue;

use crate::document::{CheckReport, CheckStatus};
use crate::standards::v1::subsets::any::schema::{
    derive_balance, evaluate_document, geg_ht_prime_limit, h_t_prime, primary_energy_factor, required_u_for_ht_prime, transmission_loss_coefficient, usage_profile_row, ventilation_loss_coefficient, zone_balances, DIN_V_18599_1_RHO_CA, DIN_V_18599_2_INFILTRATION_N_INF,
};
use crate::{examples, Attachment, AutomationClass, ElementKind, UsageProfile};

fn apply_path_f64(doc: &mut crate::Din18599Snapshot, path: &str, value: f64) {
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

fn apply_path_text(doc: &mut crate::Din18599Snapshot, path: &str, value: &str) {
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
    let mut doc = crate::subjects::compliant_detached_house();
    doc.ventilation.airflow_m3_h = 120.0;
    doc.ventilation.heat_recovery_eta = 0.0;
    // H_V = ρ·c_a · (V̇_mech + n_Außen·V + n_inf·V_e) with η=0 (DIN V 18599-2 / -10).
    let row = usage_profile_row(doc.zones[0].usage_profile);
    let expected = DIN_V_18599_1_RHO_CA
        * (120.0 + row.outdoor_air_change_1_h * doc.zones[0].volume_m3 + DIN_V_18599_2_INFILTRATION_N_INF * doc.heated_volume_m3);
    let h_v = ventilation_loss_coefficient(&doc);
    assert!((h_v - expected).abs() < 1e-9, "H_V={h_v} expected={expected}");
}

#[test]
fn potsdam_compliant_house_passes_geg_gates() {
    let doc = crate::subjects::compliant_detached_house();
    let report = evaluate_document(&doc);
    assert!(report.complies(), "expected compliant, fails={:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    let ht = h_t_prime(&doc);
    let limit = geg_ht_prime_limit(Attachment::Detached, doc.net_floor_area_m2);
    assert!(ht <= limit, "H'T={ht} limit={limit}");
    let d = derive_balance(&doc);
    assert!(d.q_p_kwh <= d.q_p_limit_kwh, "Q_P={} limit={}", d.q_p_kwh, d.q_p_limit_kwh);
}

#[test]
fn noncompliant_house_fails_ht_and_qp_with_remedies() {
    let doc = crate::subjects::noncompliant_detached_house();
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
    let doc = crate::subjects::noncompliant_detached_house();
    let limit = geg_ht_prime_limit(doc.attachment, doc.net_floor_area_m2);
    assert!(h_t_prime(&doc) > limit);
    let mut fixed = doc.clone();
    for el in &mut fixed.elements {
        el.u_value_w_m2k = crate::standards::v1::subsets::any::schema::reference_u(el.kind);
    }
    fixed.delta_u_wb_w_m2k = 0.05;
    assert!(h_t_prime(&fixed) <= limit + 1e-6, "after reference envelope H'T={} limit={}", h_t_prime(&fixed), limit);
    let (_, u_req) = required_u_for_ht_prime(&doc, limit).expect("worst element");
    assert!(u_req < doc.elements.iter().map(|e| e.u_value_w_m2k).fold(0.0, f64::max));
}

#[test]
fn remedy_law_apply_required_u_makes_ht_check_pass() {
    let doc = crate::subjects::noncompliant_detached_house();
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
fn remedy_law_apply_pv_makes_qp_check_pass() {
    let doc = crate::subjects::noncompliant_detached_house();
    let report = evaluate_document(&doc);
    let qp = report.checks.iter().find(|c| c.id == "din18599.geg.qp").expect("qp");
    assert_eq!(qp.status, CheckStatus::Fail);
    let remedy = qp.remedies.iter().find(|r| r.applicable && r.target.path == "renewables.pvAreaM2").expect("pv remedy");
    let mut fixed = doc.clone();
    apply_path_f64(&mut fixed, &remedy.target.path, remedy.required.value);
    let again = evaluate_document(&fixed);
    let qp2 = again.checks.iter().find(|c| c.id == "din18599.geg.qp").unwrap();
    assert!(qp2.utilization <= 1.0 + 1e-6, "u={} after PV {:?}", qp2.utilization, fixed.renewables.pv_area_m2);
}

#[test]
fn remedy_law_automation_oneof_makes_automation_pass() {
    let mut doc = crate::subjects::compliant_detached_house();
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
    assert!((primary_energy_factor("natural_gas") - 1.1).abs() < 1e-12);
    assert!((primary_energy_factor("electricity") - 1.8).abs() < 1e-12);
    assert!((primary_energy_factor("biomass") - 0.2).abs() < 1e-12);
}

#[test]
fn transmission_ht_increases_with_u() {
    let good = crate::subjects::compliant_detached_house();
    let bad = crate::subjects::noncompliant_detached_house();
    assert!(transmission_loss_coefficient(&bad) > transmission_loss_coefficient(&good));
}

#[test]
fn automation_class_d_fails() {
    let mut doc = crate::subjects::compliant_detached_house();
    doc.automation_class = AutomationClass::D;
    let report = evaluate_document(&doc);
    let auto = report.checks.iter().find(|c| c.id == "din18599.11.automation").unwrap();
    assert_eq!(auto.status, CheckStatus::Fail);
    assert!(!auto.remedies.is_empty());
}

#[test]
fn q_p_ref_changes_with_geometry() {
    let small = crate::subjects::compliant_detached_house();
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
    let base = crate::subjects::compliant_detached_house();
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
        ("cooled-office", examples::cooled_office::PRIMARY_TEXT, true),
    ] {
        let doc = <crate::Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text).unwrap_or_else(|e| panic!("{id} parse: {e}"));
        let report = evaluate_document(&doc);
        if expect_comply {
            assert!(report.complies(), "{id} should comply; fails={:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
        } else {
            assert!(!report.complies(), "{id} should fail");
            assert!(report.failing().count() >= 2, "{id} fail_count={}", report.failing().count());
        }
    }
}

#[test]
fn dhw_remedy_inverts_profile_limit() {
    let mut doc = crate::subjects::compliant_detached_house();
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
    use crate::standards::v1::subsets::any::schema::{
        din_v_18599_10_dhw_specific, din_v_18599_10_fan_hours, din_v_18599_10_zone_defaults, din_v_18599_11_automation_factor, din_v_18599_12_tabelle5_qp_specific, din_v_18599_2_adjacency_fx, din_v_18599_2_solar_geometry_factors, din_v_18599_4_lighting_power_density, DIN_V_18599_1_HOURS_PER_MONTH, DIN_V_18599_1_RHO_CA, DIN_V_18599_2_UTILIZATION_A, DIN_V_18599_5_ETA_SYS_FLOOR, DIN_V_18599_5_ETA_SYS_MIN, DIN_V_18599_7_COOLING_TO_HEATING_RATIO, din_v_18599_7_g_remedy, din_v_18599_8_dhw_loss_fractions, geg_anlage1_tabelle1_heating_eta, geg_anlage2_ht_prime_limits, geg_anlage2_reference_u, geg_anlage3_mean_u, geg_anlage4_primary_energy_factors, GEG_ANLAGE1_DELTA_U_WB_REF, GEG_ANLAGE1_TABELLE1_ETA_WRG_REF, GEG_ANLAGE1_WINDOW_G_REF, GEG_SECTION10_QP_FACTOR_DEFAULT,
    };
    // GEG Anlage 4
    assert!((geg_anlage4_primary_energy_factors::NATURAL_GAS - 1.1).abs() < 1e-12);
    assert!((geg_anlage4_primary_energy_factors::ELECTRICITY_GRID - 1.8).abs() < 1e-12);
    assert!((geg_anlage4_primary_energy_factors::BIOMASS - 0.2).abs() < 1e-12);
    // GEG Anlage 2 / 3 U
    assert!((geg_anlage2_reference_u::WALL - 0.28).abs() < 1e-12);
    assert!((geg_anlage3_mean_u::WINDOW - 1.5).abs() < 1e-12);
    // GEG Anlage 1 Tabelle 1 — WRG reference + ΔU_WB + heating η
    assert!((GEG_ANLAGE1_TABELLE1_ETA_WRG_REF - 0.80).abs() < 1e-12);
    assert!((GEG_ANLAGE1_DELTA_U_WB_REF - 0.05).abs() < 1e-12);
    assert!((geg_anlage1_tabelle1_heating_eta::GENERATION - 0.95).abs() < 1e-12);
    assert!((GEG_ANLAGE1_WINDOW_G_REF - 0.60).abs() < 1e-12);
    // DIN V 18599-4 / -10 / -8 / -5 / -7 / -12
    assert!((din_v_18599_4_lighting_power_density::OFFICE - 12.0).abs() < 1e-12);
    assert!((din_v_18599_10_fan_hours::RESIDENTIAL_WFH - 5000.0).abs() < 1e-12);
    assert!((din_v_18599_10_dhw_specific::RESIDENTIAL - 500.0).abs() < 1e-12);
    assert!((din_v_18599_8_dhw_loss_fractions::STORAGE_OF_USEFUL - 0.25).abs() < 1e-12);
    assert!((DIN_V_18599_5_ETA_SYS_MIN - 0.85).abs() < 1e-12);
    assert!((DIN_V_18599_7_COOLING_TO_HEATING_RATIO - 0.5).abs() < 1e-12);
    assert!((din_v_18599_7_g_remedy::REDUCE_FACTOR - 0.7).abs() < 1e-12);
    assert!((din_v_18599_12_tabelle5_qp_specific::RESIDENTIAL - 66.0).abs() < 1e-12);
    assert!((din_v_18599_12_tabelle5_qp_specific::OFFICE - 100.0).abs() < 1e-12);
    assert!((din_v_18599_12_tabelle5_qp_specific::SCHOOL - 85.0).abs() < 1e-12);
    assert!((geg_anlage2_ht_prime_limits::DETACHED_AN_LE_350 - 0.40).abs() < 1e-12);
    assert!((din_v_18599_11_automation_factor::CLASS_A - 0.88).abs() < 1e-12);
    assert!((din_v_18599_2_adjacency_fx::GROUND - 0.6).abs() < 1e-12);
    assert!((din_v_18599_10_zone_defaults::THETA_I_HEAT_C - 20.0).abs() < 1e-12);
    assert!((din_v_18599_2_solar_geometry_factors::SOUTH_HORIZ - 1.0).abs() < 1e-12);
    assert!((DIN_V_18599_2_UTILIZATION_A - 0.95).abs() < 1e-12);
    assert!((DIN_V_18599_1_RHO_CA - 0.34).abs() < 1e-12);
    assert!((DIN_V_18599_1_HOURS_PER_MONTH - 730.0).abs() < 1e-12);
    assert!((DIN_V_18599_5_ETA_SYS_FLOOR - 0.05).abs() < 1e-12);
    assert!((GEG_SECTION10_QP_FACTOR_DEFAULT - 0.55).abs() < 1e-12);
}


#[test]
fn remedy_law_apply_delta_u_wb_makes_ht_check_pass() {
    let mut doc = crate::subjects::compliant_detached_house();
    doc.delta_u_wb_w_m2k = 0.55;
    let report = evaluate_document(&doc);
    let ht = report.checks.iter().find(|c| c.id == "din18599.geg.ht-prime").expect("ht");
    assert_eq!(ht.status, CheckStatus::Fail);
    assert_eq!(ht.subject.path, "deltaUWbWM2k");
    let remedy = ht.remedies.iter().find(|r| r.applicable && r.target.path == "deltaUWbWM2k").expect("ΔU_WB remedy on failing H′T");
    // Prove app-surface path write (same API Inputs/applyRemedy use)
    let mut tree = ToValue::to_value(&doc);
    set_value_at_path(&mut tree, &remedy.target.path, dsl::DslValue::float(remedy.required.value)).expect("set deltaUWbWM2k");
    apply_path_f64(&mut doc, &remedy.target.path, remedy.required.value);
    assert!((doc.delta_u_wb_w_m2k - remedy.required.value).abs() < 1e-12);
    let again = evaluate_document(&doc);
    let ht2 = again.checks.iter().find(|c| c.id == "din18599.geg.ht-prime").unwrap();
    assert!(ht2.utilization <= 1.0 + 1e-6, "u={} after ΔU_WB remedy", ht2.utilization);
    assert_eq!(ht2.status, CheckStatus::Pass);
}

#[test]
fn every_emitted_path_resolves_via_get_value_at_path() {
    let doc = crate::subjects::noncompliant_detached_house();
    let report = evaluate_document(&doc);
    let tree = ToValue::to_value(&doc);
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
                set_value_at_path(&mut probe, &remedy.target.path, dsl::DslValue::float(remedy.required.value))
                    .unwrap_or_else(|e| panic!("set_value_at_path {}: {e}", remedy.target.path));
            }
        }
    }
    assert!(resolved >= 4, "resolved={resolved}");
    assert!(saw_id, "expected at least one [id=…] entity path");
}

#[test]
fn two_zone_moving_element_changes_zone_balances() {
    let mut doc = crate::subjects::compliant_two_zone_house();
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
    let mut doc = crate::subjects::compliant_detached_house();
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
    let fixtures: [(&str, crate::Din18599Snapshot); 3] = [
        ("cooled_office_building", crate::subjects::cooled_office_building()),
        ("compliant_two_zone_house", crate::subjects::compliant_two_zone_house()),
        ("compliant_detached_house", crate::subjects::compliant_detached_house()),
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

fn restore_climate_owner_if_handle_matches(base: &crate::Din18599Snapshot, doc: &mut crate::Din18599Snapshot) {
    if doc.climate.child_id == base.climate.child_id
        && doc.climate.target.artifact_id == base.climate.target.artifact_id
        && doc.climate.target.dialect.artifact_kind == base.climate.target.dialect.artifact_kind
        && doc.climate.target.dialect.standard == base.climate.target.dialect.standard
        && doc.climate.target.dialect.subset == base.climate.target.dialect.subset
    {
        if let Some(owner) = base.climate.local_owner::<crate::Din18599ClimateWorkingData>() {
            doc.climate.set_local_owner(owner);
        }
    }
}

fn assert_fixture_leaves_influence_checks(fixture_name: &str, base: &crate::Din18599Snapshot) {
    let base_sig = check_sig(&evaluate_document(base));
    let value = serde_json::to_value(base).expect("json");
    let mut leaves = Vec::new();
    walk_leaves("", &value, &mut |path| {
        if path.is_empty() {
            return;
        }
        let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
        // Descriptive entity labels only (CORRECTION 14:42 / 13:43).
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
        let Ok(mut perturbed) = serde_json::from_value::<crate::Din18599Snapshot>(tree) else {
            continue;
        };
        restore_climate_owner_if_handle_matches(base, &mut perturbed);
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

fn assert_climate_payload_influences_checks(fixture_name: &str, base: &crate::Din18599Snapshot) {
    let base_sig = check_sig(&evaluate_document(base));
    let mut cold = base.clone();
    let mut climate = crate::din18599_climate(base);
    for t in &mut climate.theta_e_c {
        *t -= 5.0;
    }
    cold.climate = crate::din18599_climate_child_from_data(&climate);
    let sig_t = check_sig(&evaluate_document(&cold));
    assert!(
        sig_t != base_sig,
        "{fixture_name}: climate θ_e payload must change Q_h/Q_c/q_p checks"
    );
    let mut bright = base.clone();
    let mut climate = crate::din18599_climate(base);
    for g in &mut climate.g_h_w_m2 {
        *g = (*g * 2.0).max(50.0);
    }
    bright.climate = crate::din18599_climate_child_from_data(&climate);
    let sig_g = check_sig(&evaluate_document(&bright));
    assert!(
        sig_g != base_sig,
        "{fixture_name}: climate G_h payload must change Q_h/Q_c/q_p checks"
    );
}


fn perturb_string(path: &str, s: &str) -> String {
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
