/// 🧮️ Design approach, resistance, settlement, pile and remedy laws.

#[test]
fn annex_d_n_factors_phi30() {
    let n_q = part_1::bearing_factor_n_q(30.0);
    let n_c = part_1::bearing_factor_n_c(30.0);
    let n_g = part_1::bearing_factor_n_gamma(30.0);
    assert!((n_q - 18.401).abs() < 0.05, "N_q={n_q}");
    assert!((n_c - 30.140).abs() < 0.1, "N_c={n_c}");
    assert!((n_g - 20.093).abs() < 0.15, "N_γ={n_g}");
}

#[test]
fn da2_star_de_gamma_r_v_diverges_from_en() {
    let de = DesignApproach::Da2.annex_params(AnnexChoice::De).gamma_r_v;
    let en = DesignApproach::Da2.annex_params(AnnexChoice::En).gamma_r_v;
    assert!((de / en - 1.4).abs() < 1e-9);
}

#[test]
fn da2_star_uses_characteristic_soil_and_resistance_factors() {
    let p = resolve_params(DesignApproach::Da2, AnnexChoice::De, "bsP");
    assert!((p.gamma_c - 1.0).abs() < 1e-12, "DA2* keeps characteristic c′");
    assert!((p.gamma_phi - 1.0).abs() < 1e-12, "DA2* keeps characteristic φ′");
    assert!((p.gamma_g - 1.35).abs() < 1e-12);
    assert!((p.gamma_q - 1.50).abs() < 1e-12);
    assert!((p.gamma_r_v - 1.40).abs() < 1e-12);
    let da1 = resolve_params(DesignApproach::Da1Str, AnnexChoice::De, "bsP");
    assert!(da1.gamma_c > 1.0 + 1e-9, "DA1-C1 factors soil — not used for DA2*");
}

#[test]
fn da1_str_en_uses_en_material_factors() {
    let en = DesignApproach::Da1Str.annex_params(AnnexChoice::En);
    let de = DesignApproach::Da1Str.annex_params(AnnexChoice::De);
    assert!((en.gamma_c - 1.25).abs() < 1e-9);
    assert!((de.gamma_c - 1.4).abs() < 1e-9);
}

#[test]
fn bs_p_vs_bs_t_changes_bearing_utilization() {
    let mut doc = compliant_demo();
    doc.annex = AnnexChoice::De;
    doc.design_approach = "da2".into();
    doc.design_situation = "bsP".into();
    if let Some(lc) = doc.footings[0].load_cases.first_mut() {
        lc.design_situation = "bsP".into();
        lc.vertical_permanent = 900_000.0;
        lc.vertical_variable = 400_000.0;
    }
    let u_p = check_project(&doc)
        .checks
        .iter()
        .find(|c| c.id.contains("bearing"))
        .map(|c| c.utilization)
        .expect("bearing");
    doc.design_situation = "bsT".into();
    if let Some(lc) = doc.footings[0].load_cases.first_mut() {
        lc.design_situation = "bsT".into();
    }
    let u_t = check_project(&doc)
        .checks
        .iter()
        .find(|c| c.id.contains("bearing"))
        .map(|c| c.utilization)
        .expect("bearing");
    assert!(
        (u_p - u_t).abs() > 0.02,
        "BS-P vs BS-T must change utilization; u_p={u_p} u_t={u_t}"
    );
    let p_p = resolve_params(DesignApproach::Da2, AnnexChoice::De, "bsP");
    let p_t = resolve_params(DesignApproach::Da2, AnnexChoice::De, "bsT");
    assert!(p_p.gamma_g > p_t.gamma_g);
    assert!(p_p.gamma_q > p_t.gamma_q);
    assert!(u_p > u_t, "higher γ under BS-P ⇒ higher utilization");
}

#[test]
fn passive_earth_pressure_can_change_sliding_pass_fail() {
    let mut doc = compliant_demo();
    doc.annex = AnnexChoice::De;
    doc.design_approach = "da2".into();
    {
        let footing = doc.footings.first_mut().expect("footing");
        footing.width = 2.0;
        footing.length = 2.0;
        footing.embedment = 2.5;
        if let Some(lc) = footing.load_cases.first_mut() {
            lc.vertical_permanent = 300_000.0;
            lc.vertical_variable = 50_000.0;
            lc.horizontal_permanent = 160_000.0;
            lc.horizontal_variable = 20_000.0;
            lc.moment_permanent = 10_000.0;
            lc.moment_variable = 0.0;
        }
    }
    let layer = doc.layers.first().cloned().expect("layer");
    let approach = parse_design_approach(&doc.design_approach);
    let p = resolve_params(approach, doc.annex, &doc.design_situation);
    let footing = &doc.footings[0];
    let lc = &footing.load_cases[0];
    let v_d = p.gamma_g * lc.vertical_permanent + p.gamma_q * lc.vertical_variable;
    let h_d = p.gamma_g * lc.horizontal_permanent + p.gamma_q * lc.horizontal_variable;
    let a = footing.width * footing.length;
    let r_base = part_1::sliding_resistance_with_params(
        layer.phi_prime_deg, layer.cohesion_effective, v_d, a, &p, 0.0, 0.0, layer.gamma, false,
    );
    let r_pass = part_1::sliding_resistance_with_params(
        layer.phi_prime_deg,
        layer.cohesion_effective,
        v_d,
        a,
        &p,
        footing.embedment,
        footing.length,
        layer.gamma,
        true,
    );
    assert!(r_pass > r_base + 1.0, "passive must increase R_h; base={r_base} pass={r_pass}");
    assert!(h_d > r_base, "without passive should be over-utilized; H={h_d} R={r_base}");
    assert!(h_d <= r_pass, "with DIN 1054 ≤0.5·E_p,k should pass; H={h_d} R={r_pass}");
    let report = check_project(&doc);
    let sliding = report.checks.iter().find(|c| c.id.contains("sliding") && c.id.contains("footing")).expect("sliding");
    assert_eq!(sliding.status, CheckStatus::Pass, "wired check must count passive; u={}", sliding.utilization);
}

#[test]
fn settlement_oedometric_has_no_hardcoded_influence() {
    let layers = compliant_demo().layers;
    let (s, gov) = part_1::settlement_oedometric_m(&layers, 2.5, 2.5, 1.5, 88_000.0);
    assert!(s > 0.0 && s < 0.05, "s={s}");
    assert!(!gov.is_empty());
    let s2 = part_1::settlement_m(&layers, 2.5, 88_000.0, 0.3);
    assert!(s2 > 0.0);
}

#[test]
fn pile_shaft_worked_example() {
    let r_s = part_1::shaft_resistance_n(0.7, 0.6, 80_000.0, 12.0);
    assert!((r_s / 1000.0 - 1266.69).abs() < 0.1, "R_s={}", r_s / 1000.0);
}

#[test]
fn pile_correlation_n2() {
    assert_eq!(part_1::pile_correlation_factors(2), (1.35, 1.27));
}

#[test]
fn investigation_depth_rule() {
    assert!((semio_s_artifact_norm_en1997_checks::part_2::min_investigation_depth_m(2.5) - 7.5).abs() < 1e-9);
}

#[test]
fn compliant_demo_passes() {
    let report = check_project(&compliant_demo());
    assert!(
        report.complies(),
        "compliant demo must have no Fail; fails={:?}",
        report.failing().map(|c| c.id.clone()).collect::<Vec<_>>()
    );
}

#[test]
fn noncompliant_demo_has_failures_with_remedies() {
    let report = check_project(&noncompliant_demo());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 4, "expected multiple fails, got {}", fails.len());
    for f in &fails {
        assert!(!f.remedies.is_empty(), "fail {} missing remedies", f.id);
    }
}

#[test]
fn remedy_law_footing_width_clears_bearing_or_improves() {
    let mut doc = noncompliant_demo();
    let before = check_project(&doc);
    let fail = before.failing().find(|c| c.id.contains("bearing")).expect("bearing fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("width")).expect("width remedy");
    apply_remedy_required(&mut doc, &remedy.target.path, remedy.required.value);
    let after = check_project(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(
        after_check.status,
        CheckStatus::Pass,
        "bearing must Pass after width remedy; u={} (required width={})",
        after_check.utilization,
        remedy.required.value
    );
}

#[test]
fn remedy_law_investigation_depth_clears() {
    let mut doc = noncompliant_demo();
    let before = check_project(&doc);
    let fail = before.failing().find(|c| c.id.contains("investigation")).expect("investigation fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("investigationDepth")).expect("depth remedy");
    apply_remedy_required(&mut doc, &remedy.target.path, remedy.required.value);
    let after = check_project(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(after_check.status, CheckStatus::Pass, "investigation must Pass; u={}", after_check.utilization);
}

#[test]
fn remedy_law_pile_length_clears_compression() {
    let mut doc = noncompliant_demo();
    if let Some(p) = doc.piles.first_mut() {
        p.test_profiles.clear();
        p.alpha_s = 0.7;
        p.diameter = 0.6;
        p.unit_shaft_resistance = 80_000.0;
        p.unit_base_resistance = 2_000_000.0;
        p.length = 6.0;
        p.count = 1;
        p.compression_permanent = 1_200_000.0;
        p.compression_variable = 400_000.0;
    }
    let before = check_project(&doc);
    let fail = before.failing().find(|c| c.id.contains("compression")).expect("pile compression fail");
    // Prefer length; if length alone under-corrects (base term), also apply count.
    let length_remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("length"));
    let count_remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("count"));
    if let Some(r) = length_remedy {
        apply_remedy_required(&mut doc, &r.target.path, r.required.value);
    }
    let mid = check_project(&doc);
    if mid.checks.iter().find(|c| c.id == fail.id).map(|c| c.status) != Some(CheckStatus::Pass) {
        let fail2 = mid.failing().find(|c| c.id == fail.id).or_else(|| mid.failing().find(|c| c.id.contains("compression"))).expect("still failing");
        let r = fail2.remedies.iter().find(|r| r.applicable && r.target.path.contains("count")).or(count_remedy).expect("count remedy");
        apply_remedy_required(&mut doc, &r.target.path, r.required.value);
    }
    let after = check_project(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(after_check.status, CheckStatus::Pass, "pile compression must Pass; u={}", after_check.utilization);
}

#[test]
fn settlement_remedy_targets_governing_layer_id_path() {
    let report = check_project(&noncompliant_demo());
    let fail = report.failing().find(|c| c.id.contains("settlement")).expect("settlement fail");
    assert!(
        fail.remedies.iter().any(|r| r.target.path.contains("layers[id=") && r.target.path.contains("oedometricModulus")),
        "expected layers[id=…].oedometricModulus remedy, got {:?}",
        fail.remedies.iter().map(|r| r.target.path.clone()).collect::<Vec<_>>()
    );
}

#[test]
fn field_meta_enum_choices_are_localized() {
    use semio_s_artifact_norm_en1997_checks::lookup_field_meta;
    let meta = lookup_field_meta("designSituation").expect("meta");
    let choices = meta.choices.expect("choices");
    assert!(choices.iter().any(|c| c.value == "bsP" && c.label_de.contains("BS-P")));
    assert!(lookup_field_meta("piles[].testProfiles[].shaftResistance").is_some());
    assert!(lookup_field_meta("retainingWalls[].stemThickness").is_some());
    assert!(lookup_field_meta("slopes[].governingLayerId").is_some());
    assert!(lookup_field_meta("upliftCases[].variableDestabilizing").is_some());
    assert!(lookup_field_meta("footings[].loadCases[].designSituation").is_some());
}

#[test]
fn empty_collections_are_not_applicable() {
    let mut doc = En1997Snapshot::default();
    doc.footings.clear();
    doc.piles.clear();
    doc.retaining_walls.clear();
    doc.slopes.clear();
    doc.uplift_cases.clear();
    let report = check_project(&doc);
    assert!(report.checks.iter().any(|c| c.status == CheckStatus::NotApplicable));
}
