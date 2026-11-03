use crate::*;
use semio_s_artifact_norm_en1991_model::FireLoadBasis;

fn near(actual: f64, expected: f64, relative: f64) {
    assert!((actual - expected).abs() <= relative * expected.abs().max(1e-12), "{actual} differs from {expected} by more than {relative}");
}

#[semio_framework_async_macros::async_test]
async fn snow_zone2_ground_load_pa() {
    near(part_1_3::ground_snow_pa("2", 150.0).expect("zone 2"), 850.0, 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn snow_zone1a_diverges_from_zone1() {
    let z1 = part_1_3::ground_snow_pa("1", 500.0).expect("zone 1");
    let z1a = part_1_3::ground_snow_pa("1a", 500.0).expect("zone 1a");
    near(z1a / z1, 1.25, 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn unknown_snow_zone_is_rejected() {
    assert!(part_1_3::ground_snow_pa("7", 100.0).is_none());
    assert!(part_1_3::ground_snow_pa("", 100.0).is_none());
    assert!(part_1_3::design_ground_snow_pa(AnnexChoice::De, "x", 100.0, 850.0).is_none());
    near(part_1_3::design_ground_snow_pa(AnnexChoice::En, "x", 100.0, 850.0).expect("EN value"), 850.0, 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn unknown_wind_zone_and_terrain_are_rejected() {
    assert!(na_de::WindZone::from_u8(0).is_none());
    assert!(na_de::WindZone::from_u8(5).is_none());
    assert!(part_1_4::TerrainCategory::from_u8(5).is_none());
    assert!(part_1_4::design_vb(AnnexChoice::De, 0, 25.0).is_none());
    assert!(part_1_4::peak_velocity_pressure_pa(AnnexChoice::De, 0, 2, 10.0, 25.0, 1.25, 1.0, 2, 0.0, false).is_none());
    assert!(part_1_4::peak_velocity_pressure_pa(AnnexChoice::De, 2, 9, 10.0, 25.0, 1.25, 1.0, 2, 0.0, false).is_none());
}

#[semio_framework_async_macros::async_test]
async fn unknown_categories_materials_and_activities_are_rejected() {
    assert!(part_1_1::imposed_qk_pa("Q9", AnnexChoice::De).is_none());
    assert!(part_1_1::imposed_qk_pa("B", AnnexChoice::De).is_none());
    assert!(part_1_1::imposed_qk_pa("B1", AnnexChoice::En).is_none());
    assert!(part_1_1::self_weight_n_m3("unobtainium", AnnexChoice::De).is_none());
    assert!(part_1_6::construction_qk_pa("dancing").is_none());
    assert!(part_1_2::design_fire_load_j_m2(FireLoadBasis::OccupancyTable, 420e6, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Office, 100.0, 1.0).is_none());
    assert!(part_1_2::design_fire_load_j_m2(FireLoadBasis::UserSpecified, 0.0, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Office, 100.0, 1.0).is_none());
    assert!(part_3::crane_vertical_wheel_n("HC9").is_none());
}

#[semio_framework_async_macros::async_test]
async fn imposed_category_b1_de() {
    near(part_1_1::imposed_qk_pa("B1", AnnexChoice::De).expect("B1"), 2000.0, 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn imposed_reduction_factors_follow_the_annex() {
    near(part_1_1::alpha_a("C1", 40.0, AnnexChoice::En), 0.75, 1e-9);
    near(part_1_1::alpha_a("C1", 10.0, AnnexChoice::En), 1.0, 1e-9);
    near(part_1_1::alpha_a("C1", 200.0, AnnexChoice::En), 0.6, 1e-9);
    near(part_1_1::alpha_a("C1", 40.0, AnnexChoice::De), 0.5 + 10.0 / 40.0_f64.sqrt(), 1e-9);
    assert!((part_1_1::alpha_a("C1", 40.0, AnnexChoice::De) - part_1_1::alpha_a("C1", 40.0, AnnexChoice::En)).abs() > 1e-3);
    near(part_1_1::alpha_n("B", 5, AnnexChoice::En), 0.82, 1e-9);
    near(part_1_1::alpha_n("B1", 2, AnnexChoice::De), 1.0, 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn standard_fire_curve_matches_iso_834() {
    near(part_1_2::standard_gas_temp_c(60.0), 945.3, 1e-4);
    near(part_1_2::standard_gas_temp_c(0.0), 20.0, 1e-12);
    near(part_1_2::standard_gas_temp_c(30.0), 841.8, 1e-4);
    near(part_1_2::nominal_gas_temp_k(semio_s_artifact_norm_en1991_model::part_1_2::FireCurve::Standard, 3600.0).expect("standard") - 273.15, 945.3, 1e-4);
    assert!(part_1_2::nominal_gas_temp_k(semio_s_artifact_norm_en1991_model::part_1_2::FireCurve::Parametric, 3600.0).is_none());
}

#[semio_framework_async_macros::async_test]
async fn external_and_hydrocarbon_curves_are_monotone_and_bounded() {
    let mut last = (0.0, 0.0);
    for t in 1..=120 {
        let (e, h) = (part_1_2::external_gas_temp_c(f64::from(t)), part_1_2::hydrocarbon_gas_temp_c(f64::from(t)));
        assert!(e >= last.0 && h >= last.1);
        assert!(e <= 680.0 + 1e-9 && h <= 1100.0 + 1e-9);
        last = (e, h);
    }
    near(part_1_2::external_gas_temp_c(5.0), 588.5, 2e-4);
    near(part_1_2::hydrocarbon_gas_temp_c(5.0), 947.7, 2e-4);
}

#[semio_framework_async_macros::async_test]
async fn parametric_fire_is_continuous_and_cools() {
    let p = part_1_2::parametric_fire(50.0, 184.84, 5.0, 2.0, 1160.0, 336.0, 20.0 / 60.0);
    assert!(!p.fuel_controlled);
    let before = part_1_2::parametric_theta_c(&p, p.t_max_h - 1e-9);
    let after = part_1_2::parametric_theta_c(&p, p.t_max_h + 1e-9);
    assert!((before - after).abs() < 1e-3, "{before} {after}");
    assert!(part_1_2::parametric_theta_c(&p, p.t_max_h + 2.0) < p.theta_max_c);
    assert!(part_1_2::parametric_theta_c(&p, 100.0) >= 20.0);
    let fuel = part_1_2::parametric_fire(50.0, 184.84, 20.0, 2.0, 1160.0, 80.0, 20.0 / 60.0);
    assert!(fuel.fuel_controlled);
    let b = part_1_2::parametric_theta_c(&fuel, fuel.t_max_h - 1e-9);
    let a = part_1_2::parametric_theta_c(&fuel, fuel.t_max_h + 1e-9);
    assert!((a - b).abs() < 1e-2, "{a} {b}");
}

#[semio_framework_async_macros::async_test]
async fn wind_suction_keeps_its_sign() {
    let suction = part_1_4::governing_wind_pressure_pa(1000.0, -1.2, 0.2, 1.0, 1.0);
    near(suction, 1400.0, 1e-12);
    let pressure = part_1_4::governing_wind_pressure_pa(1000.0, 0.8, 0.0, 1.0, 1.0);
    near(pressure, 1100.0, 1e-12);
    assert!(part_1_4::wind_pressure_pa(1000.0, -1.2, 0.2, 1.0, 1.0) < 0.0);
}

#[semio_framework_async_macros::async_test]
async fn wall_coefficients_interpolate_in_h_over_d() {
    let (d10, d1) = part_1_4::wall_cpe("D", 0.6).expect("zone D");
    near(d10, 0.746667, 1e-5);
    near(d1, 1.0, 1e-12);
    near(part_1_4::wall_cpe("E", 0.6).expect("zone E").0, -0.393333, 1e-5);
    assert!(part_1_4::wall_cpe("Z", 1.0).is_none());
}

#[semio_framework_async_macros::async_test]
async fn de_wind_zone_basic_velocity() {
    near(na_de::WindZone::Zone2.v_b0_m_s(), 25.0, 1e-12);
    near(part_1_4::basic_velocity_pressure_pa(1.25, 25.0), 390.625, 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn bridge_lm1_values_follow_table_4_2() {
    near(part_2::lm1_tandem_n(AnnexChoice::De, 1), 300_000.0, 1e-12);
    near(part_2::lm1_tandem_n(AnnexChoice::En, 2), 200_000.0, 1e-12);
    near(part_2::lm1_udl_pa(AnnexChoice::De, 1), 9000.0, 1e-12);
    near(part_2::lm1_udl_pa(AnnexChoice::En, 3), 2500.0, 1e-12);
    assert_eq!(part_2::notional_lanes(5.0), (1, 3.0, 2.0));
}

#[semio_framework_async_macros::async_test]
async fn crane_forces_follow_the_dynamic_factors() {
    near(part_3::design_vertical_wheel_n("HC2", "HC2", 0.5).expect("known classes"), 100_000.0 * (1.10 + 0.34 * 0.5), 1e-12);
    near(part_3::design_horizontal_force_n("HC2", AnnexChoice::De).expect("HC2"), 30_000.0, 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn silo_pressures_follow_janssen() {
    let p_ho = part_4::janssen_p_ho_pa(9000.0, 1.5, 0.4);
    near(p_ho, 33_750.0, 1e-12);
    near(part_4::janssen_horizontal_pa(9000.0, 1.5, 0.4, 0.55, 10.0), 25_964.1, 1e-4);
    near(part_4::wall_friction_pa(9000.0, 1.5, 0.4, 0.55, 10.0), 0.4 * 25_964.1, 1e-4);
    near(part_4::patch_pressure_pa(9000.0, 1.5, 0.4, 0.55, 10.0, AnnexChoice::De), 0.2 * 25_964.1, 1e-4);
}

#[semio_framework_async_macros::async_test]
async fn thermal_ranges_use_the_bridge_shifts_and_accept_zero_degrees() {
    let (con, exp) = part_1_5::bridge_delta_t_n(1, -24.0, 37.0, 10.0).expect("type 1");
    near(con, 37.0, 1e-12);
    near(exp, 43.0, 1e-12);
    assert!(part_1_5::bridge_delta_t_n(4, -24.0, 37.0, 10.0).is_none());
    let building = part_1_5::required_delta_t("building", 0.0, 0.0, 10.0, 1, 0.0).expect("building");
    near(building, 10.0, 1e-12);
    assert!(part_1_5::required_delta_t("bridge2", 37.0, -24.0, 10.0, 1, 0.0).is_none());
}

#[semio_framework_async_macros::async_test]
async fn accidental_formulas() {
    near(part_1_7::vehicle_impact_force_n(1500.0, 15.0), 318_198.0, 1e-5);
    near(part_1_7::gas_explosion_pd_kn_m2(5.0, 0.04), 30.5, 1e-12);
    assert!(part_1_7::explosion_pressure_pa(100.0, 10.0) > part_1_7::explosion_pressure_pa(100.0, 20.0));
}

#[semio_framework_async_macros::async_test]
async fn covers_check_reports_real_utilisation_and_warns_near_the_limit() {
    use semio_s_plugin_norm_compliance::CheckStatus;
    let make = |assumed: f64, required: f64| {
        assess_covers("x", "p", ClauseId::new("EN 1991", "§1", "1"), SubjectRef::whole(copy("a", "a")), copy("t", "t"), pressure_pa(assumed), pressure_pa(required), copy("", ""), AnnexChoice::De, if assumed < required { Some(raise_pressure_remedy("p", "e", "x", "x", assumed, required)) } else { None })
    };
    let fail = make(500.0, 1000.0);
    assert_eq!(fail.status, CheckStatus::Fail);
    near(fail.utilization, 2.0, 1e-9);
    let tight = make(1000.0, 980.0);
    assert_eq!(tight.status, CheckStatus::Warning);
    let loose = make(2000.0, 1000.0);
    assert_eq!(loose.status, CheckStatus::Pass);
    near(loose.utilization, 0.5, 1e-9);
    let none = make(0.0, 1000.0);
    assert_eq!(none.status, CheckStatus::Fail);
    assert!(none.utilization > 1.0);
}

#[semio_framework_async_macros::async_test]
async fn pitched_roof_pressure_series_does_not_interpolate_across_the_sign_change() {
    let at = part_1_4::pitched_roof_cpe("F", 52.5).expect("zone F");
    assert!(at.iter().any(|p| (p.0 - 0.7).abs() < 1e-9), "{at:?}");
    assert!(at.iter().all(|p| p.0 > 0.0), "{at:?}");
    let mid = part_1_4::pitched_roof_cpe("F", 15.0).expect("zone F");
    assert_eq!(mid.len(), 2);
    assert!(mid[0].0 < 0.0 && mid[1].0 > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn dominant_opening_interpolates_between_ratio_two_and_three() {
    assert!(part_1_4::dominant_opening_cpi(-1.2, 1.0).is_none());
    near(part_1_4::dominant_opening_cpi(-1.2, 2.0).expect("ratio 2"), -0.9, 1e-12);
    near(part_1_4::dominant_opening_cpi(-1.2, 2.9).expect("ratio 2.9"), -1.062, 1e-9);
    near(part_1_4::dominant_opening_cpi(-1.2, 3.0).expect("ratio 3"), -1.08, 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn orography_enters_the_na_profile_squared() {
    let flat = part_1_4::peak_velocity_pressure_pa(AnnexChoice::De, 2, 2, 10.0, 25.0, 1.25, 1.0, 2, 0.0, false).expect("site");
    let hill = part_1_4::peak_velocity_pressure_pa(AnnexChoice::De, 2, 2, 10.0, 25.0, 1.25, 1.3, 2, 0.0, false).expect("site");
    near(hill / flat, 1.69, 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn snow_drift_cap_and_cylindrical_roofs() {
    near(part_1_3::obstruction_drift(2.0, 850.0, AnnexChoice::En).1, 4.0, 1e-12);
    near(part_1_3::obstruction_drift(2.0, 850.0, AnnexChoice::De).1, 2.0, 1e-12);
    near(part_1_3::shape_mu("cylindrical", 0.0, false, 0.0, 0.0, false, 850.0, AnnexChoice::De), 2.0, 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn fire_and_execution_constants_follow_the_review() {
    near(part_1_2::alpha_c_for_curve(semio_s_artifact_norm_en1991_model::part_1_2::FireCurve::Parametric), 35.0, 1e-12);
    near(part_1_2::characteristic_fire_load_j_m2(AnnexChoice::De, "office").expect("office"), 511e6, 1e-12);
    near(part_1_6::construction_qk_pa("storage").expect("storage"), 200.0, 1e-12);
    assert!(part_1_2::characteristic_fire_load_j_m2(AnnexChoice::De, "warehouse").is_none());
}

#[semio_framework_async_macros::async_test]
async fn design_fire_load_uses_the_occupancy_fractile_of_table_e4_from_the_registry() {
    let dq1 = 1.10 + 0.4 * (100.0_f64 / 25.0).ln() / (250.0_f64 / 25.0).ln();
    near(part_1_2::delta_q1(100.0), dq1, 1e-12);
    near(part_1_2::delta_q1(25.0), 1.10, 1e-12);
    near(part_1_2::delta_q1(250.0), 1.50, 1e-12);
    near(part_1_2::delta_q1(10000.0), 2.13, 1e-12);
    use FireLoadBasis::{OccupancyTable, UserSpecified};
    near(part_1_2::design_fire_load_j_m2(OccupancyTable, 420e6, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Office, 100.0, 1.0).expect("office"), 511e6 * 0.8 * dq1, 1e-9);
    near(part_1_2::design_fire_load_j_m2(OccupancyTable, 420e6, AnnexChoice::En, semio_s_artifact_norm_en1991_model::FireOccupancy::Office, 100.0, 1.0).expect("office"), 548.14e6, 1e-3);
    near(part_1_2::design_fire_load_j_m2(OccupancyTable, 900e6, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Office, 100.0, 1.0).expect("the table basis ignores the entered value"), 511e6 * 0.8 * dq1, 1e-9);
    near(part_1_2::design_fire_load_j_m2(UserSpecified, 420e6, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Office, 100.0, 1.0).expect("user value"), 420e6 * 0.8 * dq1, 1e-9);
    near(part_1_2::design_fire_load_j_m2(UserSpecified, 900e6, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Office, 100.0, 1.0).expect("the user basis needs no occupancy row"), 900e6 * 0.8 * dq1, 1e-9);
    near(part_1_2::design_fire_load_j_m2(OccupancyTable, 100e6, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Dwelling, 25.0, 1.0).expect("dwelling alias"), 948e6 * 0.8 * 1.10, 1e-9);
    assert_eq!(part_1_2::fire_load_ndp_ids(OccupancyTable, semio_s_artifact_norm_en1991_model::FireOccupancy::Shopping), vec!["ndp.en1991.1-2.combustion-factor".to_owned(), "ndp.en1991.1-2.fire-load-fractile.shopping".to_owned()]);
    assert_eq!(part_1_2::fire_load_ndp_ids(UserSpecified, semio_s_artifact_norm_en1991_model::FireOccupancy::Shopping), vec!["ndp.en1991.1-2.combustion-factor".to_owned()]);
    let (en, de) = part_1_2::fire_load_source(OccupancyTable, 420e6, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Office).expect("source");
    assert!(en.contains("Table E.4 row \"office\"") && en.contains("511 MJ/m²") && de.contains("Tabelle E.4 Zeile"));
    assert!(part_1_2::fire_load_source(UserSpecified, 420e6, AnnexChoice::De, semio_s_artifact_norm_en1991_model::FireOccupancy::Office).expect("source").0.contains("user-specified q_f,k = 420 MJ/m²"));
}

#[semio_framework_async_macros::async_test]
async fn silo_extremes_and_rail_domain() {
    near(part_4::horizontal_max_pa(9000.0, 1.5, 0.4, 0.55, 20.0), 36_994.25, 1e-5);
    assert!(part_4::wall_friction_max_pa(9000.0, 1.5, 0.4, 0.55, 20.0) > 0.0);
    near(part_2::rail_centrifugal_f(160.0, 2.0), 1.0, 1e-12);
    assert!(part_2::rail_dynamic_factors(0.0).0 >= 1.0);
}

#[semio_framework_async_macros::async_test]
async fn lm1_tandem_moment_follows_both_axles_like_the_blessed_pynite_and_anastruct_cases() {
    for (span, expected) in [(6.0, 729_000.0), (9.6, 1_265_625.0), (15.6, 2_163_461.538_461_539)] {
        near(part_2::lm1_tandem_moment_nm(AnnexChoice::De, 1, span), expected, 1e-9);
        near(part_2::tandem_moment_nm(300_000.0, span), expected, 1e-9);
    }
    near(part_2::tandem_moment_nm(300_000.0, 1.0), 75_000.0, 1e-12);
    assert!(part_2::lm1_tandem_moment_nm(AnnexChoice::De, 1, 9.6) > 1.7 * 300_000.0 * 9.6 / 4.0);
}
