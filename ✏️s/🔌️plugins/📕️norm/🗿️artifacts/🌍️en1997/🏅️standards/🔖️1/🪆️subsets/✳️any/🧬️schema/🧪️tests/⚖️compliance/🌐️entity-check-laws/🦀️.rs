/// 🌐️ Entity laws: anchors, embedded walls, pile groups, laboratory series and layer water are catalogue cases of the document, so the snapshot reaches ANC, RW-07, PILE-06, GI-11/12 and HYD-01 and every result is bound to its entity row.

fn entity_checks<'a>(report: &'a semio_s_plugin_norm_compliance::CheckReport, collection: &str) -> Vec<&'a semio_s_plugin_norm_compliance::CheckResult> {
    report.checks.iter().filter(|c| c.subject.path.starts_with(&format!("{collection}[id="))).collect()
}

#[test]
fn compliant_demo_entities_are_checked_and_pass() {
    let report = check_project(&compliant_demo());
    for collection in ["anchors", "embeddedWalls", "pileGroups", "labTests", "layerWaters"] {
        let checks = entity_checks(&report, collection);
        assert!(!checks.is_empty(), "{collection} must raise checks");
        for check in checks {
            assert!(check.status != CheckStatus::Fail, "{collection}: {} failed ({})", check.id, check.explanation.en);
            assert!(!check.subject.entity_id.is_empty(), "{collection}: {} must be bound to its entity", check.id);
        }
    }
}

#[test]
fn reference_methods_of_the_entities_are_checks_of_their_rows() {
    let report = check_project(&compliant_demo());
    for (prefix, path) in [("en1997.geo-f03.", "footings[id=footing-F1]"), ("en1997.rw-03.", "embeddedWalls[id=ewall-E1]"), ("en1997.pile-09.", "piles[id=pile-P1]")] {
        let found: Vec<_> = report.checks.iter().filter(|c| c.id.starts_with(prefix)).collect();
        assert!(!found.is_empty(), "{prefix} raises no check");
        assert!(found.iter().all(|c| c.subject.path == path && c.status != CheckStatus::Fail && c.status != CheckStatus::NotApplicable), "{prefix}: {:?}", found.iter().map(|c| (&c.id, &c.status)).collect::<Vec<_>>());
    }
    let settlement = report.checks.iter().find(|c| c.id == "en1997.pile-09.s_total.pile-P1").expect("pile settlement");
    assert!(settlement.unverified_basis(), "E_c is an unverified registry row");
}

#[test]
fn noncompliant_demo_entities_fail_with_remedies() {
    let report = check_project(&noncompliant_demo());
    for (collection, id_fragment) in [("anchors", "anc-"), ("embeddedWalls", "rw-07"), ("pileGroups", "pile-06"), ("labTests", "gi-12"), ("layerWaters", "hyd-01")] {
        let failing: Vec<_> = entity_checks(&report, collection).into_iter().filter(|c| c.status == CheckStatus::Fail && c.id.contains(id_fragment)).collect();
        assert!(!failing.is_empty(), "{collection} must fail in the non-compliant demo");
        assert!(failing.iter().all(|c| !c.remedies.is_empty()), "{collection} failures need remedies");
    }
}

#[test]
fn embedded_wall_embedment_is_compared_with_the_required_depth() {
    let mut doc = compliant_demo();
    let long = check_project(&doc);
    let util = |report: &semio_s_plugin_norm_compliance::CheckReport| report.checks.iter().find(|c| c.id.contains("rw-07.embedment.")).map(|c| c.utilization).expect("embedment check");
    let before = util(&long);
    doc.embedded_walls[0].embedment = 2.0;
    assert!(util(&check_project(&doc)) > before, "a shorter wall raises the embedment utilisation");
    doc.embedded_walls[0].phi_deg = 24.0;
    assert!(util(&check_project(&doc)) > 0.0);
}

#[test]
fn dangling_entity_references_fail_with_a_one_of_remedy() {
    let mut doc = compliant_demo();
    doc.anchors[0].wall_id = "missing".into();
    doc.pile_groups[0].pile_id = "missing".into();
    doc.lab_tests[0].layer_id = "missing".into();
    doc.layer_waters[0].layer_id = "missing".into();
    let report = check_project(&doc);
    for field in ["wallId", "pileId", "layerId"] {
        let link = report.checks.iter().find(|c| c.id.contains(&format!("link.")) && c.id.contains(&format!(".{field}.")) && c.status == CheckStatus::Fail).unwrap_or_else(|| panic!("link check for {field}"));
        assert!(link.remedies.iter().any(|r| !r.options.is_empty()));
    }
}

#[test]
fn lab_series_phi_is_the_regression_slope() {
    let report = check_project(&compliant_demo());
    let phi = report.checks.iter().find(|c| c.id.contains("gi-12.phi_deg.")).expect("phi from the series");
    assert!(phi.explanation.en.contains("33"), "tan 33° slope reproduces φ′ = 33°: {}", phi.explanation.en);
}

#[test]
fn duplicate_ids_fail_for_the_new_collections() {
    let base = compliant_demo();
    for table in ["anchors", "tests", "embeddedWalls", "pileGroups", "labTests", "points", "layerWaters"] {
        let mut doc = base.clone();
        match table {
            "anchors" => doc.anchors.push(doc.anchors[0].clone()),
            "tests" => doc.anchors[0].tests.push(doc.anchors[0].tests[0].clone()),
            "embeddedWalls" => doc.embedded_walls.push(doc.embedded_walls[0].clone()),
            "pileGroups" => doc.pile_groups.push(doc.pile_groups[0].clone()),
            "labTests" => doc.lab_tests.push(doc.lab_tests[0].clone()),
            "points" => doc.lab_tests[0].points.push(doc.lab_tests[0].points[0].clone()),
            _ => doc.layer_waters.push(doc.layer_waters[0].clone()),
        }
        let report = check_project(&doc);
        let found = report.checks.iter().find(|c| c.id.contains(&format!("integrity.duplicate.{table}."))).unwrap_or_else(|| panic!("duplicate check for {table}"));
        assert_eq!(found.status, CheckStatus::Fail, "{table}");
    }
}

#[test]
fn slip_circle_override_replaces_the_search_and_a_tiny_circle_is_not_evaluated() {
    let mut doc = compliant_demo();
    let searched = check_project(&doc).checks.iter().find(|c| c.id == "en1997.11.bishop.slope-S1").map(|c| c.computed.value).expect("searched factor");
    doc.slopes[0].circle_centre_x = 3.0;
    doc.slopes[0].circle_centre_z = 11.0;
    doc.slopes[0].circle_radius = 13.0;
    let chosen = check_project(&doc).checks.iter().find(|c| c.id == "en1997.11.bishop.slope-S1").cloned().expect("chosen factor");
    assert_ne!(chosen.status, CheckStatus::NotApplicable);
    assert!(chosen.computed.value.is_finite() && chosen.computed.value > 0.0, "chosen circle factor {} (grid search gave {searched})", chosen.computed.value);
    doc.slopes[0].circle_radius = 0.2;
    doc.slopes[0].circle_centre_z = 50.0;
    let none = check_project(&doc).checks.into_iter().find(|c| c.id == "en1997.11.bishop.slope-S1").expect("check");
    assert_eq!(none.status, CheckStatus::NotApplicable);
}

#[test]
fn slope_width_adds_the_extruded_slip_body_check_and_the_gain_falls_with_the_width() {
    let mut doc = compliant_demo();
    doc.layers[0].cohesion_effective = 10_000.0;
    let factor = |doc: &En1997Snapshot| check_project(doc).checks.iter().find(|c| c.id == "en1997.11.bishop3d.slope-S1").map(|c| c.computed.value);
    assert!(factor(&doc).is_none(), "width 0 stays plane strain");
    doc.slopes[0].width = 20.0;
    let narrow = factor(&doc).expect("extruded check");
    doc.slopes[0].width = 40.0;
    let wide = factor(&doc).expect("extruded check");
    assert!(narrow.is_finite() && wide.is_finite() && narrow > wide, "F3D {narrow} (20 m) must exceed {wide} (40 m)");
}

#[test]
fn showcase_demo_has_a_water_head_at_the_embedded_wall_so_piping_is_reported() {
    assert!(!check_project(&compliant_demo()).checks.iter().any(|c| c.id.starts_with("en1997.hyd-02.")), "the compliant baseline has no head");
    let report = check_project(&semio_s_artifact_norm_en1997_model::showcase_demo());
    let piping = report.checks.iter().find(|c| c.id == "en1997.hyd-02.piping.ewall-E1").expect("HYD-02 on the embedded wall");
    assert!(piping.utilization > 0.0 && piping.utilization < 1.0, "piping {}", piping.utilization);
}

/// 🏗️ The three showcase footings with neighbours superposed: the expected utilisations come from the independent Python oracle (`🔮️oracles/⚖️compliance/🐍️.py`, `scipy.integrate.dblquad` of the Boussinesq stress and half-space displacement kernels, `numpy.linalg.lstsq` plane), agreement within 0.5 %.
#[test]
fn placed_footings_raise_angular_distortion_rotation_and_tilt_checks() {
    let report = check_project(&semio_s_artifact_norm_en1997_model::showcase_demo());
    for (id, expected) in [
        ("en1997.6.6.angular.oed.footing-F1.footing-F2", 0.117_356_376_540_260_9),
        ("en1997.6.6.angular.oed.footing-F1.footing-F3", 0.060_326_978_870_745_74),
        ("en1997.6.6.angular.oed.footing-F2.footing-F3", 0.040_325_873_816_024_65),
        ("en1997.6.6.angular.el.footing-F1.footing-F2", 0.120_748_825_753_536_53),
        ("en1997.6.6.angular.el.footing-F1.footing-F3", 0.061_824_634_463_552_75),
        ("en1997.6.6.angular.el.footing-F2.footing-F3", 0.041_665_695_235_934_6),
        ("en1997.6.6.tilt.oed", 0.065_977_010_196_770_1),
        ("en1997.6.6.tilt.el", 0.067_828_025_828_975_6),
    ] {
        let found = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id}"));
        assert!((found.utilization - expected).abs() <= 5e-3 * expected, "{id}: {} against the oracle {expected}", found.utilization);
    }
    for id in ["en1997.6.6.rotation.oed.footing-F1.footing-F2", "en1997.6.6.rotation.el.footing-F1.footing-F3"] {
        let found = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id}"));
        assert!(found.utilization < 1e-6, "three points define the plane exactly, so no relative rotation remains: {id} {}", found.utilization);
    }
    let bare = check_project(&compliant_demo());
    assert!(bare.checks.iter().any(|c| c.id == "en1997.6.6.pairs.na" && c.status == CheckStatus::NotApplicable), "no placements, no pairs");
}

#[test]
fn settlement_plane_is_exact_for_a_plane_and_a_pair_beyond_the_limit_fails_with_a_remedy() {
    let points = [(0.0, 0.0, 0.010), (10.0, 0.0, 0.012), (0.0, 10.0, 0.014), (10.0, 10.0, 0.016)];
    let (a, b, c) = semio_s_artifact_norm_en1997_checks::sls_pairs::settlement_plane(&points).expect("plane");
    assert!((a - 0.010).abs() < 1e-12 && (b - 0.0002).abs() < 1e-12 && (c - 0.0004).abs() < 1e-12);
    assert!(semio_s_artifact_norm_en1997_checks::sls_pairs::settlement_plane(&points[..2]).is_none());
    let mut doc = semio_s_artifact_norm_en1997_model::showcase_demo();
    doc.placements.iter_mut().find(|p| p.id == "footing-F2").expect("placement").x = 0.5;
    let near = check_project(&doc);
    let pair = near.checks.iter().find(|c| c.id == "en1997.6.6.angular.oed.footing-F1.footing-F2").expect("pair");
    assert!(pair.utilization > 1.0 && !pair.remedies.is_empty(), "footings 0.5 m apart with different load: {}", pair.utilization);
}

fn midpoint_rectangle(b: f64, l: f64, px: f64, py: f64, kernel: impl Fn(f64, f64) -> f64) -> f64 {
    let n = 400;
    let (dx, dy) = (b / f64::from(n), l / f64::from(n));
    let mut sum = 0.0;
    for i in 0..n {
        for j in 0..n {
            let (x, y) = (-0.5 * b + (f64::from(i) + 0.5) * dx - px, -0.5 * l + (f64::from(j) + 0.5) * dy - py);
            sum += kernel(x, y) * dx * dy;
        }
    }
    sum
}

#[test]
fn corner_solutions_match_the_integrated_point_load_kernels_inside_and_outside_the_loaded_area() {
    use semio_s_artifact_norm_en1997_checks::catalogue::settlement as sl;
    let (q, b, l, z, nu, e) = (100_000.0, 2.5, 4.0, 2.0, 0.3, 30e6);
    for (px, py) in [(0.0, 0.0), (0.7, -0.4), (3.0, 1.5), (-5.0, 2.0)] {
        let stress = midpoint_rectangle(b, l, px, py, |x, y| q * 3.0 * z.powi(3) / (2.0 * std::f64::consts::PI * (x * x + y * y + z * z).powf(2.5)));
        let exact = sl::sigma_z_rect_at(q, b, l, px, py, z);
        assert!((exact - stress).abs() <= 2e-3 * stress.abs().max(1.0), "sigma_z at ({px}, {py}): {exact} vs {stress}");
    }
    assert!((sl::sigma_z_rect_at(q, b, l, 0.0, 0.0, z) - sl::sigma_z_rect_centre(q, b, l, z)).abs() < 1e-9 * q);
    for (px, py) in [(3.0, 1.5), (-5.0, 2.0), (0.0, 4.0)] {
        let w = midpoint_rectangle(b, l, px, py, |x, y| q * (1.0 - nu * nu) / (std::f64::consts::PI * e * (x * x + y * y).sqrt()));
        let exact = sl::displacement_rect_at(q, b, l, px, py, nu, e);
        assert!((exact - w).abs() <= 2e-3 * w.abs(), "w at ({px}, {py}): {exact} vs {w}");
    }
}

#[test]
fn elastic_centre_settlement_follows_the_published_influence_factors_for_rectangles() {
    let (q, b, e) = (100_000.0, 2.0, 25e6);
    for (ratio, alpha) in [(1.0, 1.12), (2.0, 1.53), (5.0, 2.10)] {
        let s = part_1::elastic_settlement_m(q, b, ratio * b, 0.0, e);
        assert!((s / (q * b / e) - alpha).abs() < 0.01, "L/B = {ratio}: {}", s / (q * b / e));
    }
    assert!((part_1::elastic_settlement_m(q, 4.0, 2.0, 0.3, e) - part_1::elastic_settlement_m(q, 2.0, 4.0, 0.3, e)).abs() < 1e-12, "symmetric in the sides");
}

#[test]
fn neighbouring_footings_add_settlement_and_distant_ones_vanish() {
    use semio_s_artifact_norm_en1997_checks::sls_pairs::{footing_settlements, settlements_with_interaction};
    let doc = semio_s_artifact_norm_en1997_model::showcase_demo();
    let gwl = doc.groundwater_level;
    let placed = |doc: &En1997Snapshot| doc.footings.iter().filter_map(|f| doc.placements.iter().find(|p| p.id == f.id).map(|p| (f, p))).collect::<Vec<_>>();
    let near = settlements_with_interaction(&doc, &placed(&doc), gwl);
    assert!(near.len() >= 3);
    for ((footing, _), (oed, el)) in placed(&doc).into_iter().zip(&near) {
        let (alone_oed, alone_el) = footing_settlements(&doc, footing, gwl);
        assert!(*oed > alone_oed && *el > alone_el, "{}: neighbours must add settlement ({oed} vs {alone_oed}, {el} vs {alone_el})", footing.id);
    }
    let mut far = doc.clone();
    for (index, placement) in far.placements.iter_mut().enumerate() {
        placement.x += 1.0e6 * index as f64;
    }
    for ((footing, _), (oed, el)) in placed(&far).into_iter().zip(settlements_with_interaction(&far, &placed(&far), gwl)) {
        let (alone_oed, alone_el) = footing_settlements(&far, footing, gwl);
        assert!((oed - alone_oed).abs() < 1e-6 * alone_oed.abs().max(1e-9) && (el - alone_el).abs() < 1e-6 * alone_el.abs().max(1e-9), "{} far apart", footing.id);
    }
}

/// 🪵️ PILE-04 and PILE-07 on the compliant pile: the head deflection of the finite pile (`scipy.integrate.solve_bvp` of the Winkler beam) and the critical load (`scipy.linalg.eigh` of the finite-difference beam) from the Python oracle, agreement within 0.5 %.
#[test]
fn lateral_pile_deflection_and_buckling_agree_with_the_scipy_oracle() {
    let report = check_project(&compliant_demo());
    for (id, expected) in [("en1997.pile-04.deflection.pile-P1", 0.292_770_124_055_693_47), ("en1997.pile-07.buckling.pile-P1", 0.008_308_401_762_252_256)] {
        let found = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id}"));
        assert!((found.utilization - expected).abs() <= 5e-3 * expected, "{id}: {} against the oracle {expected}", found.utilization);
        assert_eq!(found.subject.path, "piles[id=pile-P1]", "{id} is bound to the pile row");
    }
}

#[test]
fn a_soft_pile_fails_the_deflection_and_buckling_checks_with_stiffness_remedies() {
    let mut doc = compliant_demo();
    doc.piles[0].bending_stiffness_ei = 1.0e5;
    doc.piles[0].compression_permanent = 4_000_000.0;
    let report = check_project(&doc);
    for id in ["en1997.pile-04.deflection.pile-P1", "en1997.pile-07.buckling.pile-P1"] {
        let found = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id}"));
        assert!(found.utilization > 1.0 && found.status == CheckStatus::Fail, "{id}: {}", found.utilization);
        assert!(found.remedies.iter().any(|r| r.target.path.is_empty() || r.target.path.starts_with("piles[id=pile-P1]")), "{id} needs a remedy on the pile");
    }
    let mut stiff = doc.clone();
    let required = report.checks.iter().find(|c| c.id == "en1997.pile-04.deflection.pile-P1").and_then(|c| c.remedies.last()).map(|r| r.required.value).expect("EI remedy");
    stiff.piles[0].bending_stiffness_ei = required * 1.001;
    let after = check_project(&stiff);
    assert!(after.checks.iter().find(|c| c.id == "en1997.pile-04.deflection.pile-P1").is_some_and(|c| c.utilization <= 1.0), "the remedy EI must clear the deflection check");
    doc.piles[0].bending_stiffness_ei = 0.0;
    assert!(!check_project(&doc).checks.iter().any(|c| c.id.starts_with("en1997.pile-04.") || c.id.starts_with("en1997.pile-07.")), "EI = 0 means not given");
}

/// 🌋 RW-08 on the showcase wall (k_h 0.15, k_v 0.05): the base sliding utilisation comes from the independent Python oracle (`scipy.optimize.minimize_scalar` over the wedge force polygon), agreement within 0.5 %.
#[test]
fn seismic_wall_sliding_agrees_with_the_wedge_maximisation_oracle_and_carries_remedies() {
    let report = check_project(&semio_s_artifact_norm_en1997_model::showcase_demo());
    let sliding = report.checks.iter().find(|c| c.id == "en1998.5.7.3.seismic-sliding.wall-W1").expect("seismic sliding");
    let expected = 0.471_978_540_776_282_4;
    assert!((sliding.utilization - expected).abs() <= 5e-3 * expected, "{} against the oracle {expected}", sliding.utilization);
    assert!(report.checks.iter().any(|c| c.id.starts_with("en1997.rw-08.e_ae.wall-W1")), "the RW-08 reference accompanies the check");
    assert!(!check_project(&compliant_demo()).checks.iter().any(|c| c.id.starts_with("en1998.5.7.3.")), "k_h = 0 has no seismic situation");
    let mut doc = semio_s_artifact_norm_en1997_model::showcase_demo();
    doc.retaining_walls[0].seismic_kh = 0.5;
    doc.retaining_walls[0].base_width = 1.0;
    let failing = check_project(&doc).checks.into_iter().find(|c| c.id == "en1998.5.7.3.seismic-sliding.wall-W1").expect("seismic sliding");
    assert!(failing.status == CheckStatus::Fail && failing.utilization > 1.0 && failing.remedies.iter().any(|r| r.target.path.ends_with(".baseWidth")), "{}", failing.utilization);
    doc.retaining_walls[0].seismic_kh = 0.9;
    let impossible = check_project(&doc).checks.into_iter().find(|c| c.id == "en1998.5.7.3.seismic-sliding.wall-W1").expect("no equilibrium");
    assert!(impossible.status == CheckStatus::Fail && impossible.remedies.iter().any(|r| r.target.path.ends_with(".seismicKh")));
}
