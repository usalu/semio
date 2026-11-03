/// 📇️ Catalogue laws: independent Python parity for every entry, worked examples EX97-01 … EX97-15, factor tables, earth-pressure coefficients and the repaired snapshot checks.

fn catalogue_fixture() -> serde_json::Value {
    let path = family_any_dir().join("🧪️tests").join("🧫️fixtures").join("📇️catalogue").join("🔣️.json");
    serde_json::from_str(&std::fs::read_to_string(path).expect("catalogue fixture")).expect("catalogue json")
}

fn catalogue_case(row: &serde_json::Value) -> semio_s_artifact_norm_en1997_checks::catalogue::Case {
    let c = &row["case"];
    let annex = if c["annex"].as_str() == Some("De") { AnnexChoice::De } else { AnnexChoice::En };
    let mut case = semio_s_artifact_norm_en1997_checks::catalogue::Case::new("fixture", annex);
    for (k, v) in c["num"].as_object().expect("num") {
        case = case.with_n(k, v.as_f64().expect("number"));
    }
    for (k, v) in c["text"].as_object().expect("text") {
        case = case.with_s(k, v.as_str().expect("text"));
    }
    case
}

fn oracle_catalogue(input: &serde_json::Value) -> serde_json::Value {
    use std::io::Write;
    let mut child = semio_s_plugin_norm_compliance::python_oracle::command()
        .arg(oracle_script())
        .arg("--catalogue")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("oracle spawn failed: {e}"));
    child.stdin.as_mut().expect("stdin").write_all(input.to_string().as_bytes()).expect("write");
    let output = child.wait_with_output().expect("oracle");
    assert!(output.status.success(), "catalogue oracle stderr={}", String::from_utf8_lossy(&output.stderr));
    serde_json::from_slice(&output.stdout).expect("oracle json")
}

#[test]
fn catalogue_matches_independent_python_oracle_within_one_percent() {
    use semio_s_artifact_norm_en1997_checks::catalogue::evaluate_outcomes;
    let fixture = catalogue_fixture();
    let oracle = oracle_catalogue(&fixture);
    let rows = fixture["cases"].as_array().expect("cases");
    let expected = oracle["cases"].as_array().expect("oracle cases");
    assert_eq!(rows.len(), expected.len());
    let mut compared = 0;
    for (row, exp) in rows.iter().zip(expected) {
        let id = row["entry"].as_str().expect("entry");
        let label = row["label"].as_str().unwrap_or("");
        let outcomes = evaluate_outcomes(id, &catalogue_case(row)).unwrap_or_else(|e| panic!("{id} {label}: {e}"));
        for (key, value) in exp["values"].as_object().expect("values") {
            let o = outcomes.iter().find(|o| &o.key == key).unwrap_or_else(|| panic!("{id} {label}: outcome {key} missing"));
            let (rust, py) = (o.value(), value.as_f64().expect("number"));
            let rel = (rust - py).abs() / py.abs().max(rust.abs()).max(1e-12);
            assert!(rel <= 0.01 || (rust - py).abs() < 1e-9, "{id} {label}/{key}: rust={rust} python={py} rel={rel}");
            compared += 1;
        }
    }
    assert!(compared >= 500, "only {compared} values compared");
}

#[test]
fn fixture_covers_every_registry_entry_and_registry_has_all_catalogue_ids() {
    use semio_s_artifact_norm_en1997_checks::catalogue::registry;
    let fixture = catalogue_fixture();
    let used: std::collections::BTreeSet<&str> = fixture["cases"].as_array().expect("cases").iter().map(|r| r["entry"].as_str().expect("entry")).collect();
    let ids: std::collections::BTreeSet<&str> = registry().iter().map(|e| e.id).collect();
    for id in &ids {
        assert!(used.contains(id), "fixture has no case for {id}");
    }
    for expected in [
        "PF-01", "G-01", "G-02", "GEO-F01", "GEO-F02", "GEO-F03", "GEO-F04", "EQU-01", "UPL-01", "HYD-01", "SLS-01", "SLS-02", "SLS-03", "SLS-04",
        "PILE-01", "PILE-02", "PILE-03", "PILE-04", "PILE-05", "PILE-06", "PILE-07", "PILE-08", "PILE-09", "RW-01", "RW-02", "RW-03", "RW-04", "RW-05",
        "RW-06", "RW-07", "RW-08", "RW-09", "RW-10", "SLOPE-01", "SLOPE-02", "SLOPE-03", "SLOPE-04", "ANC-01", "ANC-02", "ANC-03", "ANC-04",
        "DEW-01", "DEW-02", "DEW-03", "GI-01", "GI-02", "GI-03", "GI-04", "GI-05", "GI-06", "GI-07", "GI-08", "GI-09", "GI-10", "GI-11", "GI-12",
        "GI-13", "GI-14", "GI-15", "GI-16",
    ] {
        assert!(ids.contains(expected), "catalogue entry {expected} missing");
    }
}

#[test]
fn every_entry_reports_missing_inputs_instead_of_panicking() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{evaluate_case, registry, Case};
    for e in registry() {
        let results = evaluate_case(e.id, &Case::new("empty", AnnexChoice::De));
        assert_eq!(results.len(), 1, "{}", e.id);
        assert_eq!(results[0].status, CheckStatus::NotApplicable, "{}", e.id);
    }
}

#[test]
fn failing_catalogue_outcomes_carry_a_remedy() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{evaluate_case, Case};
    let case = Case::new("ex07", AnnexChoice::De).with_n("g_dst", 30.0).with_n("g_stb", 33.0);
    let results = evaluate_case("UPL-01", &case);
    let uplift = results.iter().find(|c| c.id.contains(".uplift.")).expect("uplift result");
    assert_eq!(uplift.status, CheckStatus::Fail);
    assert!(!uplift.remedies.is_empty());
    assert!((uplift.utilization - 1.0101).abs() < 1e-3);
}

fn value_of(id: &str, case: &semio_s_artifact_norm_en1997_checks::catalogue::Case, key: &str) -> f64 {
    semio_s_artifact_norm_en1997_checks::catalogue::evaluate_outcomes(id, case).expect(id).iter().find(|o| o.key == key).unwrap_or_else(|| panic!("{id}/{key}")).value()
}

#[test]
fn worked_examples_of_the_specification_are_reproduced() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{foundations as fnd, Case};
    use AnnexChoice::{De, En};
    let ex01 = fnd::bearing_drained(&fnd::BearingIn { b: 2.0, l: 1.0e9, d: 1.0, alpha_deg: 0.0, v: 555.0, hb: 0.0, hl: 0.0, e_b: 0.0, e_l: 0.0, phi_deg: 30.0, c: 0.0, gamma_below: 18.0, q_over: 18.0, include_depth: false }).expect("bearing");
    assert!((ex01.r_over_a - 692.90).abs() < 0.05, "EX97-01 R/A′ = {}", ex01.r_over_a);
    assert!((555.0 / (ex01.r_over_a * 2.0 / 1.4) - 0.5607).abs() < 1e-3, "EX97-01 μ");
    let ex02 = Case::new("ex02", De).with_n("b", 3.0).with_n("l", 3.0).with_n("d", 1.0).with_n("v_d", 1.0e6).with_n("cu_k", 60e3).with_n("gamma2", 19e3);
    assert!((value_of("GEO-F02", &ex02, "r_over_a") - 389.19e3).abs() < 20.0 && (value_of("GEO-F02", &ex02, "r_k") - 3502.8e3).abs() < 100.0, "EX97-02");
    let ex03 = Case::new("ex03", En).with_s("approach", "da1str").with_n("b", 2.5).with_n("l", 4.0).with_n("d", 1.2).with_n("v_d", 1.5e6).with_n("mb_d", 450e3).with_n("ml_d", 600e3).with_n("phi_k_deg", 30.0).with_n("c_k", 5e3).with_n("gamma1", 19e3).with_n("gamma2", 19e3);
    assert!((value_of("GEO-F01", &ex03, "sc") - 1.31394).abs() < 1e-4 && (value_of("GEO-F01", &ex03, "r_over_a") - 1040.18e3).abs() < 100.0, "EX97-03");
    let ex04 = ex03.clone().with_n("hb_d", 200e3);
    assert!((value_of("GEO-F01", &ex04, "iq") - 0.79898).abs() < 1e-4 && (value_of("GEO-F01", &ex04, "ig") - 0.69606).abs() < 1e-4 && (value_of("GEO-F01", &ex04, "r_k") - 4852.6e3).abs() < 2e3, "EX97-04");
    let ex05 = Case::new("ex05", De).with_n("h_g", 100e3).with_n("h_q", 80e3).with_n("v_k", 800e3).with_n("delta_deg", 30.0);
    assert!((value_of("GEO-F04", &ex05, "sliding") - 0.6073).abs() < 1e-4, "EX97-05");
    let ex06 = Case::new("ex06", De).with_n("m_dst_g", 135e3).with_n("m_dst_q", 44e3).with_n("m_stb_g", 360e3);
    assert!((value_of("EQU-01", &ex06, "equ") - 0.6075).abs() < 1e-3 && (value_of("EQU-01", &Case::new("e", En).with_n("m_dst_g", 135e3).with_n("m_dst_q", 44e3).with_n("m_stb_g", 360e3), "equ") - 0.6620).abs() < 1e-3, "EX97-06");
    assert!((value_of("UPL-01", &Case::new("ex07", De).with_n("g_dst", 30.0).with_n("g_stb", 33.0), "ballast") - 0.3333).abs() < 1e-3, "EX97-07");
    let hyd = |annex| Case::new("ex08", annex).with_n("delta_h", 2.5).with_n("l_flow", 5.0).with_n("gamma_prime", 9.5).with_n("gamma_w", 10.0);
    assert!((value_of("HYD-01", &hyd(De), "heave") - 1.0526).abs() < 1e-3 && (value_of("HYD-01", &hyd(En), "heave") - 0.789).abs() < 1e-3 && (value_of("HYD-01", &hyd(De), "i_crit") - 0.95).abs() < 1e-9, "EX97-08");
    let ex09 = Case::new("ex09", De).with_s("shape", "circle").with_n("a", 1.5).with_n("sigma0", 200e3).with_n("h1", 2.0).with_n("e1", 8e6).with_n("h2", 2.0).with_n("e2", 12e6);
    assert!((value_of("SLS-01", &ex09, "sigma_z1") - 165.86e3).abs() < 20.0 && (value_of("SLS-01", &ex09, "s") - 0.05095).abs() < 5e-5, "EX97-09");
    let ex10 = Case::new("ex10", De).with_n("h", 3.0).with_n("cc", 0.3).with_n("e0", 0.9).with_n("sigma0", 60e3).with_n("dsigma", 80e3).with_n("cv", 1e-7).with_n("h_dr", 1.5);
    assert!((value_of("SLS-02", &ex10, "s_c") - 0.1743).abs() < 1e-4 && (value_of("SLS-02", &ex10, "t90") - 1.908e7).abs() < 1e3, "EX97-10");
    let ex11 = Case::new("ex11", De).with_n("gamma", 19e3).with_n("h", 5.0).with_n("phi_deg", 30.0).with_n("delta_deg", 20.0).with_n("q", 10e3);
    assert!((value_of("RW-01", &ex11, "ka") - 0.29731).abs() < 1e-5 && (value_of("RW-01", &ex11, "e_bot") - 31.218e3).abs() < 2.0 && (value_of("RW-01", &ex11, "e_a") - 85.48e3).abs() < 20.0 && (value_of("RW-01", &ex11, "e_ah") - 80.32e3).abs() < 20.0 && (value_of("RW-01", &ex11, "e_av") - 29.24e3).abs() < 20.0, "EX97-11");
    let coh = Case::new("ex11c", De).with_n("gamma", 19e3).with_n("h", 5.0).with_n("phi_deg", 30.0).with_n("c", 10e3);
    assert!((value_of("RW-02", &coh, "z0") - 1.823).abs() < 1e-3 && (value_of("RW-02", &coh, "e_z") - 20.12e3).abs() < 10.0 && (value_of("RW-02", &Case::new("r", De).with_n("gamma", 19e3).with_n("h", 5.0).with_n("phi_deg", 30.0), "ka") - 1.0 / 3.0).abs() < 1e-9, "EX97-11 Rankine");
    let ex12 = |annex| Case::new("ex12", annex).with_n("rm1", 1450e3).with_n("rm2", 1500e3).with_n("rm3", 1650e3);
    assert!((value_of("PILE-01", &ex12(De), "r_c_k") - 1333.33e3).abs() < 10.0 && (value_of("PILE-01", &ex12(De), "r_c_d") - 1212.12e3).abs() < 10.0 && (value_of("PILE-01", &ex12(En), "r_c_d") - 1161.62e3).abs() < 10.0, "EX97-12");
    let circle = Case::new("ex13", De).with_n("r", 6.0).with_n("arc_deg", 80.0).with_n("cu_k", 40e3).with_n("w", 720e3).with_n("x_w", 1.5);
    assert!((value_of("SLOPE-01", &circle, "m_r_k") - 2010.6e3).abs() < 100.0 && (value_of("SLOPE-01", &circle, "undrained_circle") - 0.6714).abs() < 1e-3, "EX97-13 circle");
    let inf = Case::new("ex13b", De).with_n("beta_deg", 25.0).with_n("phi_k_deg", 30.0);
    assert!((value_of("SLOPE-03", &inf, "f_k") - 1.2381).abs() < 1e-3 && (value_of("SLOPE-03", &inf, "infinite_slope") - 1.0096).abs() < 1e-3, "EX97-13 infinite slope");
    assert!((value_of("GI-04", &Case::new("g", De).with_n("qc", 12e6).with_n("sigma_v0_eff", 100e3), "phi_deg") - 40.47).abs() < 0.01, "EX97-14 CPT");
    assert!((value_of("GI-03", &Case::new("g", De).with_n("qt", 2400e3).with_n("sigma_v0", 60e3).with_n("nkt", 15.0), "cu") - 156e3).abs() < 1.0, "EX97-14 cu");
    assert!((value_of("GI-07", &Case::new("g", De).with_n("n1_60", 20.0), "phi_deg") - 40.0).abs() < 1e-9, "EX97-14 SPT");
    let stat = Case::new("g", De).with_n("x1", 30.0).with_n("x2", 32.0).with_n("x3", 34.0).with_n("x4", 31.0).with_n("x5", 33.0);
    assert!((value_of("G-02", &stat, "kn") - 0.9535).abs() < 1e-3 && (value_of("G-02", &stat, "xk") - 30.49).abs() < 0.01, "EX97-14 statistics");
    let well = Case::new("ex15", De).with_n("k", 1e-4).with_n("h_aquifer", 10.0).with_n("h_well", 6.0).with_n("r_well", 0.3);
    assert!((value_of("DEW-01", &well, "radius") - 120.0).abs() < 1e-9 && (value_of("DEW-01", &well, "q_m3h") - 12.08).abs() < 0.01, "EX97-15");
}

#[test]
fn coulomb_active_coefficient_is_the_textbook_value_and_rankine_at_zero_friction() {
    use semio_s_artifact_norm_en1997_checks::catalogue::earth_pressure::{ka_coulomb, ka_rankine, kp_coulomb, kp_curved, kp_rankine};
    assert!((ka_coulomb(30.0, 20.0, 0.0, 0.0) - 0.29731).abs() < 1e-5);
    assert!((ka_coulomb(30.0, 0.0, 0.0, 0.0) - 1.0 / 3.0).abs() < 1e-12);
    assert!((ka_coulomb(30.0, 0.0, 0.0, 0.0) - ka_rankine(30.0, 0.0)).abs() < 1e-12);
    assert!((part_1::ka_rankine(30.0) - 1.0 / 3.0).abs() < 1e-12);
    assert!((kp_rankine(30.0) - 3.0).abs() < 1e-12);
    let unsafe_negative_friction = kp_coulomb(30.0, -15.0, 0.0, 0.0);
    assert!(unsafe_negative_friction < kp_rankine(30.0), "Coulomb with a negative friction angle is below Rankine ({unsafe_negative_friction}) and must never be used for passive pressure");
    assert!((kp_curved(30.0, 0.0) - 3.0).abs() < 1e-12, "δ = 0 is Rankine");
    let half = kp_curved(30.0, 15.0);
    assert!((4.0..=4.9).contains(&half), "Caquot-Kerisel range for δ = φ/2: {half}");
    assert!(kp_curved(30.0, 30.0) > half && kp_curved(30.0, 30.0) < kp_coulomb(30.0, 30.0, 0.0, 0.0), "curved surface stays below the plane Coulomb value");
}

#[test]
fn shape_factor_sc_divides_by_nq_minus_one() {
    use semio_s_artifact_norm_en1997_checks::catalogue::foundations::{bearing_drained, BearingIn};
    let square = bearing_drained(&BearingIn { b: 2.0, l: 2.0, d: 0.0, alpha_deg: 0.0, v: 1.0e6, hb: 0.0, hl: 0.0, e_b: 0.0, e_l: 0.0, phi_deg: 30.0, c: 0.0, gamma_below: 18e3, q_over: 0.0, include_depth: false }).expect("bearing");
    assert!((square.sc - 1.5287).abs() < 1e-3 && (square.sq - 1.5).abs() < 1e-12 && (square.sg - 0.7).abs() < 1e-12, "sc={}", square.sc);
}

#[test]
fn pile_correlation_factors_follow_the_en_tables() {
    use semio_s_artifact_norm_en1997_checks::catalogue::piles::{xi_dynamic_tests, xi_ground_tests, xi_static_tests};
    assert_eq!(xi_static_tests(3, AnnexChoice::De, false), (1.15, 1.05));
    assert_eq!(xi_static_tests(3, AnnexChoice::En, false), (1.20, 1.05));
    assert_eq!(xi_static_tests(6, AnnexChoice::En, false), (1.0, 1.0));
    assert_eq!(xi_ground_tests(1), (1.40, 1.40));
    assert_eq!(xi_ground_tests(3), (1.33, 1.23));
    assert_eq!(xi_ground_tests(5), (1.29, 1.15));
    assert_eq!(xi_ground_tests(12), (1.25, 1.08));
    assert_eq!(xi_dynamic_tests(5), (1.50, 1.35));
    assert_eq!(part_1::pile_correlation_factors(7), (1.27, 1.12));
}

#[test]
fn din_4085_row_utilisation_is_the_required_over_the_used_coefficient() {
    let mut doc = compliant_demo();
    doc.retaining_walls[0].wall_movement = semio_s_artifact_norm_en1997_model::WallMovement::Free;
    doc.retaining_walls[0].earth_pressure_mode = semio_s_artifact_norm_en1997_model::EarthPressureMode::Active;
    let ep = check_project(&doc).checks.iter().find(|c| c.id.contains("earthPressure")).cloned().expect("DIN 4085 row");
    assert_eq!(ep.status, CheckStatus::Pass);
    assert!((ep.utilization - 1.0).abs() < 1e-12 && ep.computed.value > 0.0 && (ep.computed.value - ep.limit.value).abs() < 1e-12);
    doc.retaining_walls[0].earth_pressure_mode = semio_s_artifact_norm_en1997_model::EarthPressureMode::AtRest;
    let conservative = check_project(&doc).checks.iter().find(|c| c.id.contains("earthPressure")).cloned().expect("row");
    assert_eq!(conservative.status, CheckStatus::Pass);
    assert!(conservative.utilization < 1.0);
    doc.retaining_walls[0].wall_movement = semio_s_artifact_norm_en1997_model::WallMovement::Rigid;
    doc.retaining_walls[0].earth_pressure_mode = semio_s_artifact_norm_en1997_model::EarthPressureMode::Active;
    let unsafe_row = check_project(&doc).checks.iter().find(|c| c.id.contains("earthPressure")).cloned().expect("row");
    assert_eq!(unsafe_row.status, CheckStatus::Fail);
    assert!(unsafe_row.utilization > 1.0);
}

#[test]
fn governing_rows_warn_when_the_governing_utilisation_exceeds_one() {
    let report = check_project(&noncompliant_demo());
    let gov = report.checks.iter().find(|c| c.id == "en1997.governing.situation").expect("project governing row");
    assert!(gov.utilization > 1.0);
    assert!((gov.limit.value - 1.0).abs() < 1e-12, "limit is the real limit 1.0, not max(u, 1)");
    assert_eq!(gov.status, CheckStatus::Warning);
    assert_eq!(check_project(&compliant_demo()).checks.iter().find(|c| c.id == "en1997.governing.situation").expect("governing").status, CheckStatus::Pass);
}

#[test]
fn ground_model_gap_between_layers_fails_with_a_depth_remedy() {
    let mut doc = compliant_demo();
    doc.layers[1].depth_top = 9.0;
    let report = check_project(&doc);
    let model = report.checks.iter().find(|c| c.id == "en1997.layer.model.layer-sand").expect("model row");
    assert_eq!(model.status, CheckStatus::Fail);
    assert!(model.remedies.iter().any(|r| r.target.path.contains("layers[id=layer-gravel].depthTop")));
    doc.layers[1].depth_top = 8.0;
    doc.layers[0].gamma_prime = doc.layers[0].gamma;
    assert!(check_project(&doc).failing().any(|c| c.id == "en1997.layer.model.layer-sand"));
}

#[test]
fn sliding_uses_characteristic_permanent_vertical_force_not_the_factored_load() {
    let mut doc = compliant_demo();
    doc.layers[0].phi_prime_deg = 30.0;
    doc.layers[0].cpt_qc = 0.0;
    doc.layers[0].spt_n = 0.0;
    doc.footings[0].embedment = 0.0;
    doc.footings[0].load_cases[0].vertical_permanent = 800e3;
    doc.footings[0].load_cases[0].vertical_variable = 0.0;
    doc.footings[0].load_cases[0].horizontal_permanent = 100e3;
    doc.footings[0].load_cases[0].horizontal_variable = 80e3;
    doc.footings[0].load_cases[0].moment_permanent = 0.0;
    doc.footings[0].load_cases[0].moment_variable = 0.0;
    let report = check_project(&doc);
    let sliding = report.checks.iter().find(|c| c.id.contains("6.5.3.sliding.footing")).expect("sliding");
    assert!((sliding.utilization - 0.6073).abs() < 2e-3, "EX97-05 μ = {}", sliding.utilization);
}

#[test]
fn layered_bishop_agrees_with_the_catalogue_bishop_for_one_homogeneous_layer() {
    use semio_s_artifact_norm_en1997_checks::catalogue::slopes::{bishop, Circle};
    let mut layer = compliant_demo().layers[0].clone();
    layer.depth_bottom = 40.0;
    layer.cohesion_effective = 5_000.0;
    let (height, angle, gwl): (f64, f64, f64) = (6.0, 30.0, 100.0);
    let geo3 = DesignApproach::Geo3.annex_params(AnnexChoice::De);
    let run = height / angle.to_radians().tan();
    let layered = part_1::bishop_fos_slices(std::slice::from_ref(&layer), &layer.id, height, angle, run, gwl, geo3.gamma_phi, geo3.gamma_c).expect("fos");
    let mut best = f64::INFINITY;
    for xc_f in [0.0, 0.15, 0.3, 0.45, 0.6, 0.75, 0.9, 1.05] {
        for yc_f in [1.0, 1.4, 1.8, 2.4, 3.2] {
            for depth_f in [0.0, 0.15, 0.3, 0.6, 1.0] {
                let circle = Circle { h: height, beta_deg: angle, xc: xc_f * run, yc: yc_f * height, r: (yc_f + depth_f) * height, gamma: layer.gamma, gamma_sat: layer.gamma, gwl_level: -1.0e3, ru: 0.0, q: 0.0 };
                if let Some(f) = bishop(&circle, layer.phi_prime_deg.to_radians().tan() / geo3.gamma_phi, layer.cohesion_effective / geo3.gamma_c, 60) {
                    if f > 0.0 {
                        best = best.min(f);
                    }
                }
            }
        }
    }
    assert!(((layered - best) / best).abs() < 0.02, "layered={layered} catalogue={best}");
}

#[test]
fn zero_or_negative_capacity_never_passes() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{evaluate_case, Case, Outcome};
    assert!(Outcome::force("k", "a", "b", 1.0, 0.0).utilization() >= 1.0e6);
    assert!(Outcome::force("k", "a", "b", 1.0, -5.0).utilization() >= 1.0e6);
    assert_eq!(Outcome::force("k", "a", "b", 0.0, 0.0).utilization(), 0.0);
    let case = Case::new("zero", AnnexChoice::De).with_n("g_dst", 10.0).with_n("g_stb", 1.0e-30);
    assert!(evaluate_case("UPL-01", &case).iter().any(|c| c.status == CheckStatus::Fail));
}

#[test]
fn m2_soil_factors_reduce_passive_pressure_and_the_embedded_wall_gets_longer() {
    use semio_s_artifact_norm_en1997_checks::catalogue::Case;
    let passive = |approach: &str| Case::new("m2", AnnexChoice::En).with_s("approach", approach).with_n("gamma", 18e3).with_n("t", 2.0).with_n("phi_deg", 32.0).with_n("delta_deg", 0.0);
    assert!(value_of("RW-03", &passive("da3"), "kp") < value_of("RW-03", &passive("da2"), "kp"), "γφ = 1.25 lowers K_p");
    let wall = |approach: &str| Case::new("w", AnnexChoice::En).with_s("approach", approach).with_n("h", 4.0).with_n("gamma", 18e3).with_n("phi_deg", 32.0).with_n("prop_depth", 0.5);
    assert!(value_of("RW-07", &wall("da3"), "t0") > value_of("RW-07", &wall("da1str"), "t0"), "M2 needs more embedment");
}

#[test]
fn passive_wall_friction_is_capped_at_two_thirds_of_phi() {
    use semio_s_artifact_norm_en1997_checks::catalogue::earth_pressure::kp_curved;
    assert!((kp_curved(30.0, 30.0) - kp_curved(30.0, 20.0)).abs() < 1e-12);
    assert!((kp_curved(30.0, -25.0) - kp_curved(30.0, 20.0)).abs() < 1e-12);
}

#[test]
fn mononobe_okabe_without_equilibrium_is_an_error_and_takes_the_worse_vertical_direction() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{evaluate_outcomes, Case};
    use semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{thrust_for, Interface, Side, Water};
    let none = Case::new("mo", AnnexChoice::De).with_n("gamma", 18e3).with_n("h", 5.0).with_n("phi_deg", 20.0).with_n("kh", 0.5);
    assert!(evaluate_outcomes("RW-08", &none).is_err());
    let case = Case::new("mo", AnnexChoice::De).with_n("gamma", 18e3).with_n("h", 5.0).with_n("phi_deg", 33.0).with_n("delta_deg", 11.0).with_n("kh", 0.15).with_n("kv", 0.1);
    let e = value_of("RW-08", &case, "e_ae");
    let i = Interface::vertical(33.0, 11.0);
    let up = thrust_for(Side::Active, &i, 18e3, 5.0, Water::NONE, 0.15, 0.9).expect("kae").total();
    assert!(e >= up - 1e-6);
}

#[test]
fn pile_tables_are_step_functions_and_r1_distinguishes_the_pile_type() {
    use semio_s_artifact_norm_en1997_checks::catalogue::piles::{xi_dynamic_tests, xi_ground_tests};
    assert_eq!(xi_ground_tests(6), xi_ground_tests(5));
    assert_eq!(xi_ground_tests(9), xi_ground_tests(7));
    assert_eq!(xi_dynamic_tests(8), xi_dynamic_tests(5));
    let p = DesignApproach::Da1Str.annex_params(AnnexChoice::En);
    assert_eq!(part_1::pile_type_factors("bored", 3, &p, DesignApproach::Da1Str).0, 1.25);
    assert_eq!(part_1::pile_type_factors("driven", 3, &p, DesignApproach::Da1Str).0, 1.0);
}

#[test]
fn uplift_water_force_scales_with_the_area_and_bearing_is_symmetric_in_the_axes() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{foundations::{bearing_drained, BearingIn}, Case};
    let upl = |area: f64| Case::new("a", AnnexChoice::De).with_n("head", 3.0).with_n("area", area).with_n("g_stb", 4.0e5);
    assert!((value_of("UPL-01", &upl(10.0), "uplift") / value_of("UPL-01", &upl(5.0), "uplift") - 2.0).abs() < 1e-9);
    let at = |b: f64, l: f64, hb: f64, hl: f64| bearing_drained(&BearingIn { b, l, d: 1.0, alpha_deg: 0.0, v: 1.0e6, hb, hl, e_b: 0.0, e_l: 0.0, phi_deg: 30.0, c: 0.0, gamma_below: 18e3, q_over: 18e3, include_depth: false }).expect("bearing").r;
    assert!((at(4.0, 2.0, 1.0e5, 0.0) - at(2.0, 4.0, 0.0, 1.0e5)).abs() < 1.0e-6 * at(2.0, 4.0, 0.0, 1.0e5));
}

#[test]
fn cancelled_slip_search_is_indeterminate_and_missing_pore_pressure_skips_heave() {
    let layers = compliant_demo().layers;
    let cancelled = part_1::bishop_fos_slices_with(&layers, "layer-sand", 6.0, 25.0, 14.0, 4.0, 1.25, 1.25, &mut |_| false);
    assert!(cancelled.expect("known layer").is_nan());
    let mut doc = compliant_demo();
    doc.uplift_cases[0].pore_pressure = 0.0;
    let report = check_project(&doc);
    assert_eq!(report.checks.iter().find(|c| c.id.contains("10.hyd") || c.id.contains("hyd.")).expect("hyd row").status, CheckStatus::NotApplicable);
}

#[test]
fn laboratory_series_lower_bound_is_exact_for_a_noise_free_line_and_shrinks_with_scatter() {
    use semio_s_artifact_norm_en1997_checks::catalogue::Case;
    let line = |noise: f64| {
        (1..=5).fold(Case::new("lab", AnnexChoice::De), |c, i| {
            let sigma = 50e3 * i as f64;
            c.with_n(&format!("sigma{i}"), sigma).with_n(&format!("tau{i}"), 20e3 + 0.5 * sigma + if i % 2 == 0 { noise } else { -noise })
        })
    };
    let exact = line(0.0);
    assert!((value_of("GI-17", &exact, "phi_k_deg") - 0.5f64.atan().to_degrees()).abs() < 1e-9 && (value_of("GI-17", &exact, "c_k") - 20e3).abs() < 1e-6, "noise-free line");
    let noisy = line(2e3);
    assert!(value_of("GI-17", &noisy, "phi_k_deg") < value_of("GI-17", &noisy, "phi_mean_deg") && value_of("GI-17", &noisy, "c_k") < value_of("GI-17", &noisy, "c_mean"), "scatter lowers the characteristic values");
    assert!((value_of("GI-17", &noisy, "t") - 2.353).abs() < 1e-9, "t(0.95; 3)");
    let two = Case::new("lab", AnnexChoice::De).with_n("sigma1", 1.0).with_n("tau1", 1.0).with_n("sigma2", 2.0).with_n("tau2", 2.0);
    assert!(semio_s_artifact_norm_en1997_checks::catalogue::evaluate_outcomes("GI-17", &two).is_err(), "n < 3 is not evaluated");
}

#[test]
fn pile_load_test_interpolates_the_failure_settlement_and_flags_an_unreached_criterion() {
    use semio_s_artifact_norm_en1997_checks::catalogue::Case;
    let curve = |d: f64| {
        let mut c = Case::new("t", AnnexChoice::De).with_n("d", d);
        for (i, (s, q)) in [(0.002, 4.0e5), (0.005, 8.0e5), (0.01, 1.1e6), (0.02, 1.35e6), (0.04, 1.5e6)].iter().enumerate() {
            c = c.with_n(&format!("ts{}", i + 1), *s).with_n(&format!("tq{}", i + 1), *q);
        }
        c
    };
    let mid = curve(0.3);
    assert!((value_of("PILE-10", &mid, "r_c_m") - 1.425e6).abs() < 1e-6 && (value_of("PILE-10", &mid, "r_c_k") - 1.425e6 / 1.35).abs() < 1e-6, "s_g = 0.03 m lies midway between 0.02 and 0.04");
    assert!((value_of("PILE-10", &mid, "criterion_reached") - 0.03 / 0.04).abs() < 1e-12);
    assert!(value_of("PILE-10", &curve(0.6), "criterion_reached") > 1.0, "s_g = 0.06 m is beyond the last test settlement");
    assert!((value_of("PILE-10", &curve(0.3).with_n("rm1", 1.5e6), "xi1") - 1.25).abs() < 1e-12, "two tests give xi1 = 1.25 (DIN 1054)");
}

#[test]
fn ground_test_profiles_sum_unit_resistances_and_feed_the_pile_02_factors() {
    use semio_s_artifact_norm_en1997_checks::catalogue::Case;
    let d = 0.6_f64;
    let c = Case::new("p", AnnexChoice::De).with_n("d", d).with_n("p1_h1", 3.0).with_n("p1_qs1", 40e3).with_n("p1_h2", 5.0).with_n("p1_qs2", 80e3).with_n("p1_qb", 3.0e6);
    let shaft = std::f64::consts::PI * d * (3.0 * 40e3 + 5.0 * 80e3);
    assert!((value_of("PILE-11", &c, "r_s_cal1") - shaft).abs() < 1e-6 && (value_of("PILE-11", &c, "r_b_cal1") - 3.0e6 * std::f64::consts::FRAC_PI_4 * d * d).abs() < 1e-6);
    assert!((value_of("PILE-11", &c, "xi3") - 1.40).abs() < 1e-12, "one profile: xi3 = 1.40");
}

#[test]
fn piping_exit_gradient_follows_the_conformal_map_and_lane_uses_a_third_of_the_horizontal_creep() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{hydraulics::exit_gradient_sheet_pile, Case};
    assert!((exit_gradient_sheet_pile(std::f64::consts::PI, 1.0, 0.0) - 1.0).abs() < 1e-12 && (exit_gradient_sheet_pile(2.0, 1.0, 1.0) - 2.0 / (std::f64::consts::PI * 2.0f64.sqrt())).abs() < 1e-12);
    let c = Case::new("h", AnnexChoice::De).with_n("delta_h", 1.0).with_n("gamma_prime", 9500.0).with_n("lv1", 2.0).with_n("lv2", 2.0).with_n("lh1", 12.0).with_n("c_lane", 7.0);
    assert!((value_of("HYD-02", &c, "l_lane") - 8.0).abs() < 1e-12 && (value_of("HYD-02", &c, "l_bligh") - 16.0).abs() < 1e-12 && (value_of("HYD-02", &c, "lane") - 7.0 / 8.0).abs() < 1e-12);
    assert!((value_of("HYD-02", &c.clone().with_n("d_embed", 3.0), "i_exit") - 1.0 / (3.0 * std::f64::consts::PI)).abs() < 1e-12);
}

#[test]
fn extruded_slip_body_gains_only_from_cohesion_and_the_gain_falls_with_the_width() {
    use semio_s_artifact_norm_en1997_checks::catalogue::Case;
    let base = |c_k: f64, width: f64| Case::new("s", AnnexChoice::De).with_n("h", 6.0).with_n("beta_deg", 30.0).with_n("xc", 6.0).with_n("yc", 10.0).with_n("r", 11.0).with_n("gamma", 18e3).with_n("phi_k_deg", 30.0).with_n("c_k", c_k).with_n("width", width);
    assert!(value_of("SLOPE-05", &base(0.0, 20.0), "end_gain").abs() < 1e-12, "frictionless ends add nothing without cohesion");
    let (narrow, wide) = (value_of("SLOPE-05", &base(5e3, 20.0), "end_gain"), value_of("SLOPE-05", &base(5e3, 40.0), "end_gain"));
    assert!(narrow > 0.0 && (narrow / wide - 2.0).abs() < 1e-9, "gain {narrow} vs {wide}");
    let f2 = value_of("SLOPE-05", &base(5e3, 20.0), "f_2d");
    assert!((f2 - value_of("SLOPE-01", &base(5e3, 20.0), "f")).abs() < 1e-12, "plane-strain factor is the Bishop factor");
}

/// 🌋 Mononobe-Okabe thrust: the closed form E_ae = ½γ(1 ± k_v)H²K_ae and the maximised sliding-wedge force polygon agree with each other and with the values of the independent Python formulation (γ = 18 kN/m³, H = 4 m).
#[test]
fn mononobe_okabe_closed_form_wedge_maximum_and_python_formulation_agree() {
    use semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{critical_wedge, thrust, Acceleration, Interface, Side, Water};
    for (phi, delta, kh, kv, expected, rho) in [
        (30.0, 0.0, 0.0, 0.0, 48_000.0, 60.0),
        (30.0, 20.0, 0.15, 0.05, 60_604.029, 47.908),
        (35.0, 15.0, 0.2, 0.1, 57_783.189, 50.95),
        (32.0, 16.0, 0.1, 0.0, 49_204.99, 52.936),
    ] {
        let load = thrust(Side::Active, &Interface::vertical(phi, delta), 18_000.0, 4.0, Water::NONE, Acceleration { kh, kv }).expect("limit equilibrium");
        assert!((load.total() - expected).abs() <= 1e-4 * expected, "closed form {} vs {expected}", load.total());
        let (angle, force) = critical_wedge(18_000.0, 4.0, phi, delta, kh, load.vertical);
        assert!((force - expected).abs() <= 1e-4 * expected && (angle - rho).abs() < 0.01, "wedge {force} at {angle} deg vs {expected} at {rho} deg");
    }
    assert!(thrust(Side::Active, &Interface::vertical(30.0, 20.0), 18_000.0, 4.0, Water::NONE, Acceleration { kh: 0.9, kv: 0.0 }).is_none(), "θ beyond φ has no limit equilibrium");
}

/// 🪵️ The lateral pile lines: the head deflection at x = 0 equals 2Hλ/(k_h·D), the extremum of the moment line is 0.3224·H/λ and the elastic critical load drops to the Euler load without soil.
#[test]
fn winkler_pile_lines_reproduce_the_closed_forms() {
    use semio_s_artifact_norm_en1997_checks::catalogue::piles;
    let (ei, k, d, h, l) = (197.2e6, 20e6, 0.6, 50_000.0, 14.0);
    let lambda = piles::lateral_lambda(k, d, ei);
    assert!((piles::lateral_deflection(h, lambda, k, d, 0.0) - 2.0 * h * lambda / (k * d)).abs() < 1e-12);
    let peak = (0..2000).map(|i| piles::lateral_moment(h, lambda, f64::from(i) * 0.005).abs()).fold(0.0, f64::max);
    assert!((peak - 0.3224 * h / lambda).abs() < 2e-3 * peak, "{peak}");
    let euler = std::f64::consts::PI.powi(2) * ei / (l * l);
    assert!((piles::winkler_buckling_load(ei, l, 0.0, d) - euler).abs() < 1e-9 * euler);
    assert!((piles::winkler_buckling_load(ei, l, k, d) - 99_297_075.0).abs() < 1e-4 * 99_297_075.0, "scipy.linalg.eigh gives 99 297 076 N");
}

fn fixture_row(index: usize) -> semio_s_artifact_norm_en1997_checks::catalogue::Case {
    catalogue_case(&catalogue_fixture()["cases"][index])
}

fn close(value: f64, expected: f64, relative: f64) -> bool {
    (value - expected).abs() <= relative * expected.abs()
}

/// 🌊️ The finite-difference flow net of a sheet pile equals the scipy sparse solution of the same stencil, conserves the discharge, and the exit gradient of a deep layer approaches the conformal-map value ΔH/(π d).
#[test]
fn flow_net_matches_the_scipy_solution_and_the_conformal_map() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{hydraulics::exit_gradient_sheet_pile, seepage::FlowNet};
    let net = FlowNet::solve(2.0, 3.0, 9.0, 12).expect("net");
    assert_eq!((net.nx, net.nz), (168, 36));
    assert!(close(net.exit_gradient(), 0.200_757_656_062_556, 1e-6), "{}", net.exit_gradient());
    assert!(close(net.prism_head(), 0.674_785_669_409_186, 1e-6), "{}", net.prism_head());
    assert!(close(net.discharge(1.0), 1.258_357_688_963_923, 1e-6) && close(net.discharge_retained(1.0), net.discharge(1.0), 1e-8));
    let deep = FlowNet::solve(2.0, 3.0, 1000.0, 12).expect("deep net");
    assert!((deep.exit_gradient() / exit_gradient_sheet_pile(2.0, 3.0, 0.0) - 1.0).abs() < 0.05, "{}", deep.exit_gradient());
    let psi = net.stream_function();
    let total = psi.iter().copied().fold(0.0, f64::max);
    assert!(close(total, net.discharge(1.0), 1e-8) && psi[psi.len() - 1].abs() < 1e-8 * total && psi[net.nx].abs() < 1e-8 * total);
    for line in net.streamlines(5).iter().chain(net.equipotentials(5).iter()) {
        assert!(line.len() >= 2 && line.iter().all(|p| p[1] >= 0.0 && p[1] <= net.nz as f64 * net.spacing && p[0].abs() <= net.nx as f64 * net.spacing / 2.0));
    }
    assert!(net.equipotentials(5).len() >= 5);
    assert!(net.streamlines(5).len() >= 5);
}

/// 🔺️ HYD-03 reads the flow net: the oracle utilisations of the Terzaghi prism and the surface exit gradient, and the safety against the critical gradient.
#[test]
fn hyd_03_reports_prism_piping_and_discharge() {
    let c = fixture_row(117);
    assert!(close(value_of("HYD-03", &c, "heave_prism"), 0.473_534, 1e-4) && close(value_of("HYD-03", &c, "piping"), 0.422_648, 1e-4));
    assert!(close(value_of("HYD-03", &c, "q"), 1.258_357_688_963_923e-4, 1e-6) && close(value_of("HYD-03", &c, "safety"), 4.732_074, 1e-4));
    assert!(value_of("HYD-03", &c, "continuity") < 1e-8);
}

/// 🧭️ The largest gradient of the HYD-03 flow net equals the scipy/numpy oracle of the same stencil, is never below the surface exit gradient and exceeds the mean gradient ΔH/path of the wall: the toe concentrates the gradient.
#[test]
fn hyd_03_largest_gradient_matches_the_oracle_and_bounds_the_exit_gradient() {
    use semio_s_artifact_norm_en1997_checks::catalogue::seepage::FlowNet;
    let c = fixture_row(117);
    let i_max = value_of("HYD-03", &c, "i_max");
    assert!(i_max >= value_of("HYD-03", &c, "i_exit") * (1.0 - 1e-12), "{i_max}");
    let net = FlowNet::solve(2.0, 1.5, 6.0, 8).expect("net");
    assert!(close(net.gradient_at(net.centre(), 0), net.exit_gradient(), 1e-12));
    assert!(net.max_gradient() > net.delta_h / (2.0 * 1.5 + 2.0 * 1.5));
    for (i, j) in [(0, 0), (net.centre() - 1, net.wall_rows - 1), (net.centre(), net.wall_rows), (net.nx - 1, net.nz - 1)] {
        assert!(net.gradient_at(i, j).is_finite() && net.gradient_at(i, j) >= 0.0);
    }
}

/// 🧱️ RW-11 sums the stem, slab, heel soil and toe soil (243.8 kN/m about 1.7648 m from the toe) and passes the thrust of the virtual back to the gravity-wall checks.
#[test]
fn heel_wall_weights_and_thrust_follow_the_hand_calculation() {
    use semio_s_artifact_norm_en1997_checks::catalogue::wall_sections::heel_wall;
    let w = heel_wall(4.0, 0.5, 3.2, 0.6, 0.4, 25_000.0, 18_000.0, 32.0, 32.0 * 2.0 / 3.0, 10_000.0, 0.5).expect("wall");
    assert!((w.heel - 2.2).abs() < 1e-12 && (w.w - 243_800.0).abs() < 1e-6, "{w:?}");
    let by_hand = 0.4 * 4.0 * 25_000.0 * 0.8 + 3.2 * 0.5 * 25_000.0 * 1.6 + 2.2 * 4.0 * 18_000.0 * 2.1 + 0.6 * 0.5 * 18_000.0 * 0.3;
    assert!((w.w * w.x_w - by_hand).abs() < 1e-6 * by_hand);
    let c = fixture_row(119);
    assert!(close(w.x_w, 1.764_807, 1e-6) && close(value_of("RW-11", &c, "sigma_max"), 94_666.887_398, 1e-6) && close(value_of("RW-11", &c, "sliding"), 0.53049, 1e-4));
    assert!(close(value_of("RW-11", &c, "e_ah"), 46_688.389_175, 1e-6) && close(value_of("RW-11", &c, "bearing"), 0.236_667, 1e-4));
    let tight = semio_s_artifact_norm_en1997_checks::catalogue::Case::new("tight", AnnexChoice::De).with_n("h", 4.0).with_n("t_base", 0.5).with_n("b", 1.0).with_n("a_toe", 0.6).with_n("t_stem", 0.4).with_n("gamma", 18_000.0).with_n("phi_deg", 32.0);
    assert!(semio_s_artifact_norm_en1997_checks::catalogue::evaluate_outcomes("RW-11", &tight).is_err(), "no heel");
}

/// 🧗️ RW-12 in water: free earth support, Blum fixed earth support and cantilever reproduce the scipy quad and brentq solution, and the equilibrium moment at the toe vanishes.
#[test]
fn embedded_wall_in_water_matches_the_scipy_solution() {
    use semio_s_artifact_norm_en1997_checks::catalogue::wall_sections::ENTRIES;
    assert!(ENTRIES.iter().any(|e| e.id == "RW-12"));
    let free = fixture_row(121);
    assert!(close(value_of("RW-12", &free, "t0"), 5.873_514, 1e-4) && close(value_of("RW-12", &free, "prop_force"), 135_814.452_68, 1e-4) && close(value_of("RW-12", &free, "m_max_d"), 333_643.347, 2e-3));
    assert!(close(value_of("RW-12", &free, "u_net"), 30_000.0, 1e-9) && close(value_of("RW-12", &free, "delta_h"), 3.0, 1e-12) && close(value_of("RW-12", &free, "embedment"), 5.873_514 / 5.0, 1e-4));
    let fixed = fixture_row(122);
    assert!(close(value_of("RW-12", &fixed, "y"), 2.957_635, 1e-4) && close(value_of("RW-12", &fixed, "x"), 5.712_579, 1e-4) && close(value_of("RW-12", &fixed, "t_required"), 9.812_73, 1e-4));
    assert!(close(value_of("RW-12", &fixed, "c_force"), 109_822.02, 1e-4) && close(value_of("RW-12", &fixed, "prop_force"), 111_831.55, 1e-4));
    let cantilever = fixture_row(123);
    assert!(close(value_of("RW-12", &cantilever, "t0"), 4.831_864, 1e-4) && close(value_of("RW-12", &cantilever, "t_required"), 1.2 * 4.831_864, 1e-4));
    let dry_free = cantilever.clone().with_s("kind", "free_earth").with_n("prop_depth", 0.5);
    let wet = dry_free.clone().with_n("z_w", 1.0).with_n("z_wf", 3.0).with_n("gamma_sat", 20_000.0);
    assert!(value_of("RW-12", &wet, "t0") > value_of("RW-12", &dry_free, "t0"), "groundwater behind the wall lengthens the embedment");
}

/// 🛤️ EMB-01: Osterberg's fill stress is the full fill pressure at the surface and falls with depth, and the centre-line settlement equals the scipy Flamant-strip quadrature.
#[test]
fn embankment_stress_and_settlement_match_the_strip_quadrature() {
    use semio_s_artifact_norm_en1997_checks::catalogue::embankments::{centre_settlement, osterberg_centre};
    assert!((osterberg_centre(5.0, 8.0, 1e-9) - 1.0).abs() < 1e-6 && osterberg_centre(5.0, 8.0, 4.0) > osterberg_centre(5.0, 8.0, 8.0));
    let c = fixture_row(124);
    assert!(close(value_of("EMB-01", &c, "s_centre"), 0.216_52, 1e-4) && close(value_of("EMB-01", &c, "bearing"), 1.280_848, 1e-4) && close(value_of("EMB-01", &c, "spreading"), 0.137_898, 1e-4));
    assert!(close(value_of("EMB-01", &c, "sigma_z_mid"), 82_531.759, 1e-5) && close(centre_settlement(86_000.0, 5.0, 8.0, 8.0, 3.0e6), value_of("EMB-01", &c, "s_centre"), 1e-12));
}

/// 🧮️ Worked example of the M2 set in earth pressure (EN 1997-1 2.4.7.1, Table A.4): φ′ = 30°, c′ = 10 kPa, γ = 18 kN/m³, t = 2 m, DA3 → tan φ′d = tan 30°/1.25 → φ′d = 24.791°, K_p = tan²(45° + φ′d/2) = 2.44420 (3.00000 unfactored), c′d = 8 kPa and E_p,k = ½ γ t² K_p + 2 c′d t √K_p = 138.020 kN/m, both from the closed Rankine form that groundhog-free Python (`📌️reference-values.py`) and the oracle `rw_03` reproduce; DA1-C2 factors alike, DA2 does not.
#[test]
fn m2_factors_tan_phi_and_cohesion_before_the_passive_coefficient_and_thrust() {
    let (da3, c2) = (fixture_row(126), fixture_row(127));
    let phi_d = (30.0_f64.to_radians().tan() / 1.25).atan();
    let kp = (std::f64::consts::FRAC_PI_4 + phi_d / 2.0).tan().powi(2);
    assert!(close(phi_d.to_degrees(), 24.791_280_897, 1e-9) && close(kp, 2.444_201_814_844_742, 1e-12));
    assert!(close(value_of("RW-03", &da3, "kp_rankine"), kp, 1e-9) && close(value_of("RW-03", &da3, "kp"), kp, 1e-6));
    let thrust = 0.5 * 18_000.0 * 4.0 * kp + 2.0 * (10_000.0 / 1.25) * 2.0 * kp.sqrt();
    assert!(close(thrust, 138_019.883_728_296, 1e-12) && close(value_of("RW-03", &da3, "e_p_k"), thrust, 1e-6));
    let c2_thrust = 0.5 * 18_000.0 * 4.0 * kp + 2.0 * (5_000.0 / 1.25) * 2.0 * kp.sqrt() + 20_000.0 * kp * 2.0;
    assert!(close(value_of("RW-03", &c2, "e_p_k"), c2_thrust, 1e-6), "DA1-C2 uses M2 like DA3");
    let da2 = semio_s_artifact_norm_en1997_checks::catalogue::Case::new("da2", AnnexChoice::En).with_s("approach", "da2").with_n("gamma", 18e3).with_n("t", 2.0).with_n("phi_deg", 30.0).with_n("delta_deg", 0.0).with_n("c", 10_000.0);
    assert!(close(value_of("RW-03", &da2, "kp_rankine"), 3.0, 1e-9), "M1 keeps tan 30° (K_p = 3)");
}

/// 🌊️ HYD-03 reads the Terzaghi prism and the internal-erosion gradient from the same net: the required embedment d_req = γH·γw·h_m/(γH,stb·γ′) reproduces the prism ratio d_req/d, and the internal-erosion index is i_max/i_crit with i_max never below the exit gradient.
#[test]
fn hyd_03_internal_erosion_and_required_embedment_follow_the_prism_and_the_net() {
    let c = fixture_row(128);
    let (heave, piping, erosion, required) = (value_of("HYD-03", &c, "heave_prism"), value_of("HYD-03", &c, "piping"), value_of("HYD-03", &c, "internal_erosion"), value_of("HYD-03", &c, "d_required"));
    assert!(close(required / 2.0, heave, 1e-9), "d_req/d is the prism ratio");
    assert!(close(erosion, value_of("HYD-03", &c, "i_max") / value_of("HYD-03", &c, "i_crit"), 1e-9));
    assert!(value_of("HYD-03", &c, "i_max") >= value_of("HYD-03", &c, "i_exit") - 1e-12 && piping > 0.0 && required > 0.0);
}

/// ⏳️ SLS-05: the exact series reproduces the classical time factors (T50 = 0.197, T90 = 0.848), the laboratory c_v follows from t90, and the specimen is 90 % consolidated at its own t90.
#[test]
fn consolidation_series_gives_the_classical_time_factors_and_the_laboratory_cv_closes_the_loop() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{settlement::{degree_exact, time_factor_exact}, Case};
    assert!((time_factor_exact(0.5) - 0.1968).abs() < 2e-3 && (time_factor_exact(0.9) - 0.848).abs() < 2e-3);
    assert!((semio_s_artifact_norm_en1997_checks::catalogue::settlement::t90_factor() - 0.848_085_408).abs() < 1e-6 && (degree_exact(0.848_085_408) - 0.9).abs() < 1e-7);
    assert!((degree_exact(time_factor_exact(0.37)) - 0.37).abs() < 1e-9 && degree_exact(0.0) == 0.0 && degree_exact(10.0) > 0.999_999);
    let case = Case::new("c", AnnexChoice::De).with_n("s_final", 0.12).with_n("h_dr", 2.0).with_n("t90_lab", 1800.0).with_n("h_dr_lab", 0.01);
    let cv = value_of("SLS-05", &case, "cv");
    assert!((cv - time_factor_exact(0.9) * 1e-4 / 1800.0).abs() < 1e-18);
    let t90 = value_of("SLS-05", &case, "t90");
    assert!((t90 - time_factor_exact(0.9) * 4.0 / cv).abs() < 1e-3 * t90);
    let at = case.clone().with_n("t", t90);
    assert!((value_of("SLS-05", &at, "u_avg") - 0.9).abs() < 1e-9 && (value_of("SLS-05", &at, "s_t") - 0.108).abs() < 1e-9 && (value_of("SLS-05", &at, "s_remaining") - 0.012).abs() < 1e-9);
    assert!((value_of("SLS-05", &at.clone().with_n("s_res_lim", 0.024), "residual") - 0.5).abs() < 1e-9);
}

/// 🪵️ PILE-12: the equivalent raft grows with the spread over two thirds of the pile length and the settlement is the Boussinesq sum under its centre; doubling the stiffness halves it.
#[test]
fn pile_group_settlement_is_the_equivalent_raft_sum_and_halves_with_double_stiffness() {
    use semio_s_artifact_norm_en1997_checks::catalogue::{settlement::sigma_z_rect_centre, Case};
    let case = |e: f64| Case::new("g", AnnexChoice::De).with_n("m", 3.0).with_n("n", 4.0).with_n("s", 1.8).with_n("d", 0.5).with_n("l", 12.0).with_n("v", 6e6).with_n("spread", 0.25).with_n("raft_depth_factor", 2.0 / 3.0).with_n("h1", 4.0).with_n("h2", 6.0).with_n("e1", e).with_n("e2", 2.0 * e);
    let (b, l) = (3.0 * 1.8 + 0.5 + 4.0, 2.0 * 1.8 + 0.5 + 4.0);
    assert!((value_of("PILE-12", &case(15e6), "b_raft") - b).abs() < 1e-12 && (value_of("PILE-12", &case(15e6), "l_raft") - l).abs() < 1e-12);
    let p = 6e6 / (b * l);
    let expected = sigma_z_rect_centre(p, b, l, 2.0) * 4.0 / 15e6 + sigma_z_rect_centre(p, b, l, 7.0) * 6.0 / 30e6;
    assert!((value_of("PILE-12", &case(15e6), "s_group") - expected).abs() < 1e-12 && (value_of("PILE-12", &case(30e6), "s_group") * 2.0 - expected).abs() < 1e-12);
    assert!((value_of("PILE-12", &case(15e6).with_n("s_lim", 2.0 * expected), "serviceability") - 0.5).abs() < 1e-9);
}

/// ⛰️ SLOPE-06: at an interslice inclination of zero the moment factor of Spencer is the simplified Bishop factor, and at the root the force and the moment factor agree.
#[test]
fn spencer_moment_factor_at_zero_inclination_is_bishop_and_the_root_closes_force_equilibrium() {
    use semio_s_artifact_norm_en1997_checks::catalogue::slopes::{bishop, slices, spencer, spencer_pair, Circle};
    let circle = Circle { h: 6.0, beta_deg: 30.0, xc: 6.0, yc: 10.0, r: 11.0, gamma: 18e3, gamma_sat: 20e3, gwl_level: 1.5, ru: 0.0, q: 10e3 };
    let (tan_phi, c) = (30.0_f64.to_radians().tan(), 5e3);
    let sl = slices(&circle, 60).expect("slices");
    let (moment, _) = spencer_pair(&sl, tan_phi, c, 0.0).expect("pair");
    assert!((moment - bishop(&circle, tan_phi, c, 60).expect("bishop")).abs() < 1e-7);
    let (f, lambda) = spencer(&circle, tan_phi, c, 60).expect("spencer");
    let (fm, ff) = spencer_pair(&sl, tan_phi, c, lambda).expect("pair at root");
    assert!((fm - ff).abs() < 1e-6 && (fm - f).abs() < 1e-12 && lambda > 0.0 && lambda < 1.0, "{fm} {ff} {lambda}");
    assert!((f / moment - 1.0).abs() < 0.05, "Spencer {f} close to Bishop {moment}");
}

#[test]
fn the_depth_factor_input_is_labelled_as_outside_en_1997_1_annex_d() {
    let spec = semio_s_artifact_norm_en1997_checks::catalogue::case_inputs::VOCABULARY.iter().find(|s| s.key == "include_depth").expect("include_depth input");
    assert!(spec.en.contains("not in EN 1997-1 Annex D"), "{}", spec.en);
    assert!(spec.de.contains("nicht in EN 1997-1 Anhang D"), "{}", spec.de);
}
