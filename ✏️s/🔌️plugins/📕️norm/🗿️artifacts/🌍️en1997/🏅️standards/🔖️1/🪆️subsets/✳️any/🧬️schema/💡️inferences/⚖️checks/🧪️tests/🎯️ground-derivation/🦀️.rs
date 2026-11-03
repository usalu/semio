use super::ground_derivation::*;
use super::*;
use semio_s_artifact_norm_en1997_model::{compliant_demo, CharacteristicBasis, CharacteristicParameter, Investigation, InvestigationKind, InvestigationReading, ParameterBasis, SoilLayer, SoilType};

const FIXTURE: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🎯️ground-derivation/🔣️.json");

fn layer(id: &str, soil_type: SoilType, top: f64, bottom: f64, gamma: f64, gamma_prime: f64, nkt: f64) -> SoilLayer {
    SoilLayer { id: id.into(), soil_type, depth_top: top, depth_bottom: bottom, gamma, gamma_prime, phi_prime_deg: 30.0, cohesion_effective: 0.0, cohesion_undrained: 0.0, oedometric_modulus: 20_000_000.0, poisson_ratio: 0.3, cpt_qc: 0.0, spt_n: 0.0, cpt_nkt: nkt }
}

fn reading(investigation: &str, k: usize, depth: f64, measurement: f64) -> InvestigationReading {
    InvestigationReading { id: format!("{investigation}-r{k}"), depth, measurement }
}

fn point(id: &str, kind: InvestigationKind, depth: f64, readings: Vec<InvestigationReading>) -> Investigation {
    Investigation { id: id.into(), position_x: 0.0, position_y: 0.0, ground_level: 0.0, depth, kind, readings }
}

fn basis(id: &str, layer: &str, parameter: CharacteristicParameter, basis: ParameterBasis) -> CharacteristicBasis {
    CharacteristicBasis { id: id.into(), layer_id: layer.into(), parameter, basis }
}

/// 🎯️ The worked example of the committed fixture (clay, silt, sand under a water table at 4 m): the q_t of each CPT reading is σ_v0 + N_kt·c_u for the target c_u of the three soundings in the clay (cpt-A 40, 42, 45, 41, 44 kPa; cpt-B 38, 43, 40; cpt-C 46, 41; N_kt = 15 on the layer) and in the silt (cpt-A 30, 28; cpt-B 33; N_kt from the registry row), and the SPT soundings spt-A N = 20, 24, 28, spt-B 22, 26 and spt-C 21, 30 lie in the sand.
fn worked_example() -> semio_s_artifact_norm_en1997_model::En1997Snapshot {
    let mut doc = compliant_demo();
    doc.annex = AnnexChoice::De;
    doc.groundwater_level = 4.0;
    doc.layers = vec![layer("clay-1", SoilType::Clay, 0.0, 6.0, 18_000.0, 8_000.0, 15.0), layer("silt-1", SoilType::Silt, 6.0, 9.0, 19_000.0, 9_000.0, 0.0), layer("sand-1", SoilType::Sand, 9.0, 12.0, 20_000.0, 10_000.0, 0.0)];
    let q = |z: f64, cu: f64| total_stress_pa(&doc.layers, 4.0, AnnexChoice::De, z) + 15.0 * cu;
    let cpt = |name: &str, rows: &[(f64, f64)]| point(name, InvestigationKind::Cpt, 12.0, rows.iter().enumerate().map(|(k, (z, cu))| reading(name, k + 1, *z, q(*z, *cu))).collect());
    let spt = |name: &str, rows: &[(f64, f64)]| point(name, InvestigationKind::Spt, 12.0, rows.iter().enumerate().map(|(k, (z, n))| reading(name, k + 1, *z, *n)).collect());
    doc.investigations = vec![
        cpt("cpt-A", &[(1.0, 40e3), (2.0, 42e3), (3.0, 45e3), (4.0, 41e3), (5.0, 44e3), (7.0, 30e3), (8.0, 28e3)]),
        spt("spt-A", &[(9.5, 20.0), (10.5, 24.0), (11.5, 28.0)]),
        cpt("cpt-B", &[(1.5, 38e3), (3.5, 43e3), (4.5, 40e3), (7.5, 33e3)]),
        cpt("cpt-C", &[(2.5, 46e3), (5.5, 41e3)]),
        spt("spt-B", &[(10.0, 22.0), (11.0, 26.0)]),
        spt("spt-C", &[(9.8, 21.0), (11.8, 30.0)]),
    ];
    doc.characteristic_bases = vec![
        basis("b1", "clay-1", CharacteristicParameter::UndrainedStrength, ParameterBasis::Derived),
        basis("b2", "silt-1", CharacteristicParameter::UndrainedStrength, ParameterBasis::Derived),
        basis("b3", "sand-1", CharacteristicParameter::FrictionAngle, ParameterBasis::Derived),
        basis("b4", "sand-1", CharacteristicParameter::UndrainedStrength, ParameterBasis::Derived),
    ];
    doc
}

fn blessed(key: &str, field: &str) -> f64 {
    let at = FIXTURE.rfind(&format!("\"{key}\": {{")).unwrap_or_else(|| panic!("no blessed value {key}"));
    let rest = &FIXTURE[at..];
    let from = rest.find(&format!("\"{field}\": ")).unwrap_or_else(|| panic!("no field {field} of {key}")) + field.len() + 4;
    rest[from..].split(|c| c == ',' || c == '\n').next().and_then(|t| t.trim().parse().ok()).unwrap_or_else(|| panic!("unreadable {field} of {key}"))
}

fn near(ours: f64, theirs: f64, relative: f64, what: &str) {
    assert!((ours - theirs).abs() <= relative * theirs.abs().max(1.0), "{what}: ours {ours}, scipy oracle {theirs}");
}

/// 🎯️ The Rust derivation reproduces the numpy/scipy oracle blessed in the fixture: the mean and V_x to 1e-9, k_n and X_k to the tabulated t quantile (1e-3).
#[test]
fn the_characteristic_values_of_the_worked_example_match_the_scipy_oracle() {
    let doc = worked_example();
    for (layer_id, parameter, key) in [("clay-1", CharacteristicParameter::UndrainedStrength, "clay-1.undrainedStrength"), ("silt-1", CharacteristicParameter::UndrainedStrength, "silt-1.undrainedStrength"), ("sand-1", CharacteristicParameter::FrictionAngle, "sand-1.frictionAngle")] {
        let layer = doc.layers.iter().find(|l| l.id == layer_id).expect("layer");
        let derived = derive_layer(&doc, layer, parameter).expect("derived");
        assert_eq!(derived.samples.len() as f64, blessed(key, "n"), "{key} sample count");
        assert_eq!(derived.readings as f64, blessed(key, "readings"), "{key} reading count");
        near(derived.mean, blessed(key, "mean"), 1e-9, &format!("{key} mean"));
        near(derived.variation, blessed(key, "variation"), 1e-6, &format!("{key} V_x"));
        near(derived.kn, blessed(key, "kn"), 1e-3, &format!("{key} k_n"));
        near(derived.characteristic, blessed(key, "characteristic"), 1e-3, &format!("{key} X_k"));
    }
    assert!(derive_layer(&doc, &doc.layers[2], CharacteristicParameter::UndrainedStrength).is_none(), "no CPT reading in the sand: nothing to derive");
}

/// 🎯️ Hand check of the clay: the three soundings average to 42.4, 40.3333 and 43.5 kPa (cpt-A 5, cpt-B 3, cpt-C 2 readings), so n = 3 independent samples with X_m = 42.0778 kPa, s = 1.6077 kPa, V_x = 0.03821 and, with k_n = 2.920·√(1/3) = 1.6859 (t(0.95; 2) tabulated), X_k = 42.0778·(1 − 1.6859·0.03821) = 39.367 kPa. Counting the 10 readings as samples would give 40.55 kPa (k_n ≈ 0.58), which is unconservative. The silt has two soundings (n = 2 < 3), so the characteristic value is the lowest reading, 28 kPa.
#[test]
fn the_clay_follows_the_statistical_rule_over_the_soundings_and_the_silt_the_cautious_estimate() {
    let doc = worked_example();
    let clay = derive_layer(&doc, &doc.layers[0], CharacteristicParameter::UndrainedStrength).expect("clay");
    assert_eq!(clay.method, Method::Statistical);
    assert_eq!((clay.samples.len(), clay.readings), (3, 10));
    near(clay.samples[0].value, 42_400.0, 1e-9, "cpt-A mean of 5 readings");
    near(clay.samples[1].value, 121_000.0 / 3.0, 1e-9, "cpt-B mean of 3 readings");
    near(clay.samples[2].value, 43_500.0, 1e-9, "cpt-C mean of 2 readings");
    near(clay.mean, 42_077.777_777_8, 1e-9, "mean");
    near(clay.kn, 2.920 / 3.0_f64.sqrt(), 1e-9, "k_n = t(0.95; 2) / sqrt(3)");
    near(clay.characteristic, 39_367.0, 1e-3, "X_k");
    let readings: Vec<f64> = [40.0, 42.0, 45.0, 41.0, 44.0, 38.0, 43.0, 40.0, 46.0, 41.0].iter().map(|c| c * 1e3).collect();
    let (_, _, _, _, naive) = catalogue::investigation::characteristic_value(&readings, false, None);
    near(naive, 40_554.0, 1e-3, "X_k with the 10 readings counted as samples");
    assert!(clay.characteristic < naive, "the correlated readings of one CPT must not shrink the confidence interval");
    let silt = derive_layer(&doc, &doc.layers[1], CharacteristicParameter::UndrainedStrength).expect("silt");
    assert_eq!(silt.method, Method::Cautious);
    assert_eq!((silt.samples.len(), silt.readings), (2, 3));
    near(silt.characteristic, 28_000.0, 1e-9, "cautious value: the lowest reading");
}

/// 📐️ One investigation point gives one sample per registry row value (hand values): the readings 1, 3, 5, 7 average to 4 for one sample, to 2 and 6 for two depth segments, and clamp to the four readings beyond; a long autocorrelated profile of one CPT stays one sample and falls back to the lowest reading.
#[test]
fn an_investigation_point_contributes_one_independent_sample_however_many_readings_it_has() {
    let readings = vec![(3.0, 5.0), (1.0, 1.0), (4.0, 7.0), (2.0, 3.0)];
    let values = |k: usize| independent_samples("cpt", readings.clone(), k).iter().map(|s| s.value).collect::<Vec<_>>();
    assert_eq!(values(1), vec![4.0]);
    assert_eq!(values(2), vec![2.0, 6.0]);
    assert_eq!(values(9), vec![1.0, 3.0, 5.0, 7.0]);
    assert_eq!(independent_samples("cpt", readings.clone(), 1)[0].depth, 2.5);
    assert!(independent_samples("cpt", Vec::new(), 1).is_empty());
    let mut doc = worked_example();
    let dense: Vec<InvestigationReading> = (0..200).map(|k| reading("cpt-D", k + 1, 0.02 + 0.02 * k as f64, total_stress_pa(&doc.layers, 4.0, AnnexChoice::De, 0.02 + 0.02 * k as f64) + 15.0 * (35e3 + 5e3 * ((k % 7) as f64 - 3.0) / 3.0))).collect();
    doc.investigations = vec![point("cpt-D", InvestigationKind::Cpt, 12.0, dense)];
    let clay = derive_layer(&doc, &doc.layers[0], CharacteristicParameter::UndrainedStrength).expect("clay");
    assert_eq!((clay.samples.len(), clay.readings), (1, 200));
    assert_eq!(clay.method, Method::Cautious);
    near(clay.characteristic, 30_000.0, 1e-6, "lowest of the 200 readings");
}

/// 🔩️ A layer without N_kt takes the registry row `ndp.en1997.2.cpt-nkt` of the annex, a layer value wins.
#[test]
fn the_cone_factor_comes_from_the_layer_or_else_the_registry_row() {
    let doc = worked_example();
    assert_eq!(cone_factor(&doc.layers[0], AnnexChoice::De), 15.0);
    for annex in [AnnexChoice::En, AnnexChoice::De] {
        assert_eq!(cone_factor(&doc.layers[1], annex), semio_s_plugin_norm_compliance::ndp_number("ndp.en1997.2.cpt-nkt", annex));
    }
    let mut richer = layer("clay-x", SoilType::Clay, 0.0, 1.0, 18_000.0, 8_000.0, 18.0);
    assert_eq!(cone_factor(&richer, AnnexChoice::De), 18.0);
    richer.cpt_nkt = 0.0;
    assert_eq!(cone_factor(&richer, AnnexChoice::De), 15.0);
}

/// 💧️ σ_v0 from the layer unit weights: at 6 m of the worked example σ′v0 = 4·18 + 2·8 = 88 kPa and the pore pressure 2·10 = 20 kPa below the water table at 4 m.
#[test]
fn the_total_stress_adds_the_pore_pressure_below_the_water_table() {
    let doc = worked_example();
    near(effective_stress_pa(&doc.layers, 4.0, 6.0), 88_000.0, 1e-12, "effective stress");
    near(total_stress_pa(&doc.layers, 4.0, AnnexChoice::De, 6.0), 108_000.0, 1e-12, "total stress DE");
    near(total_stress_pa(&doc.layers, 4.0, AnnexChoice::En, 6.0), 88_000.0 + 2.0 * 9_810.0, 1e-12, "total stress EN");
    near(total_stress_pa(&doc.layers, 4.0, AnnexChoice::De, 3.0), 54_000.0, 1e-12, "dry total stress");
}

/// 🧱️ SPT of the specification example EX97-14: (N1)60 = 20 at σ′v0 = 100 kPa gives φ′ = √(20·20) + 20 = 40°; C_N is capped at 1.7 below 34.6 kPa.
#[test]
fn the_spt_normalisation_follows_the_specification_example_and_caps_the_overburden_factor() {
    near(part_2::n1_60(20.0, 100_000.0), 20.0, 1e-12, "(N1)60");
    near(part_2::phi_from_spt_deg(20.0, 100_000.0), 40.0, 1e-12, "phi'");
    near(part_2::n1_60(20.0, 25_000.0), 34.0, 1e-12, "capped (N1)60");
}

/// 🧭️ The typed basis decides: a derived parameter carries the characteristic value (0 when nothing can be derived, flagged by a failing check, never the declared value), a declared one and a parameter without a row are untouched.
#[test]
fn the_resolved_document_applies_only_the_derived_bases_and_flags_what_cannot_be_derived() {
    let mut doc = worked_example();
    for l in &mut doc.layers {
        l.cohesion_undrained = 77_000.0;
    }
    doc.layers[2].phi_prime_deg = 31.0;
    let resolved = resolved_document(&doc);
    near(resolved.layers[0].cohesion_undrained, 39_367.0, 1e-3, "clay c_u");
    near(resolved.layers[1].cohesion_undrained, 28_000.0, 1e-9, "silt c_u");
    near(resolved.layers[2].cohesion_undrained, 0.0, 1e-9, "sand c_u: nothing derivable, not the declared value");
    near(resolved.layers[2].phi_prime_deg, blessed("sand-1.frictionAngle", "characteristic"), 1e-3, "sand phi'");
    let report = check_project(&doc);
    let find = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("no check {id}"));
    assert!(find("en1997.2.characteristic.cu.clay-1").utilization <= 1.0 + 1e-9, "3 soundings meet the statistical minimum");
    assert!(find("en1997.2.characteristic.cu.silt-1").utilization > 1.0, "2 soundings fall short of the statistical minimum");
    assert!(find("en1997.2.characteristic.cu.sand-1").utilization > 1.0 && !find("en1997.2.characteristic.cu.sand-1").remedies.is_empty(), "nothing derivable must fail with a remedy");
    let mut declared = doc.clone();
    declared.characteristic_bases.iter_mut().for_each(|b| b.basis = ParameterBasis::Declared);
    assert!(matches!(resolved_document(&declared), std::borrow::Cow::Borrowed(_)), "declared bases leave the document untouched");
    assert!(matches!(resolved_document(&semio_s_artifact_norm_en1997_model::compliant_demo()), std::borrow::Cow::Borrowed(_)), "no row, no change");
}

/// 🦶️ The undrained bearing check (GEO, Annex D.3) of a footing on a clay layer uses the declared c_u without a basis row and the c_u derived from the investigation points with the derived basis (30 kPa from readings of a constant c_u).
#[test]
fn a_derived_basis_feeds_the_undrained_footing_check_and_never_overrides_silently() {
    let mut doc = compliant_demo();
    doc.layers[0].soil_type = SoilType::Clay;
    doc.layers[0].cohesion_undrained = 90_000.0;
    doc.layers[0].cpt_nkt = 15.0;
    let readings: Vec<InvestigationReading> = (1..=5).map(|k| reading("cpt-1", k, k as f64, total_stress_pa(&doc.layers, doc.groundwater_level, doc.annex, k as f64) + 15.0 * 30_000.0)).collect();
    doc.investigations = vec![point("cpt-1", InvestigationKind::Cpt, 12.0, readings)];
    let explanation = |doc: &semio_s_artifact_norm_en1997_model::En1997Snapshot| check_project(doc).checks.iter().filter(|c| c.id.starts_with("en1997.6.5.bearing.undrained.")).map(|c| c.explanation.en.clone()).collect::<Vec<_>>();
    let layer_id = doc.layers[0].id.clone();
    let declared = explanation(&doc);
    assert!(!declared.is_empty() && declared.iter().all(|e| e.contains("c_u=90 kPa")), "no basis row: the declared c_u counts: {declared:?}");
    doc.characteristic_bases = vec![basis("b1", &layer_id, CharacteristicParameter::UndrainedStrength, ParameterBasis::Declared)];
    assert_eq!(explanation(&doc), declared, "a declared basis equals no row");
    doc.characteristic_bases[0].basis = ParameterBasis::Derived;
    let derived = explanation(&doc);
    assert!(derived.iter().all(|e| e.contains("c_u=30 kPa")), "derived basis: the c_u of the investigations counts: {derived:?}");
}

/// 🔎️ GI-01 follows the investigation points: the check passes when a point reaches z_a = max(6 m, 2 B) even though the declared depth of the document is shallow, fails with a remedy on the deepest point when none does, and the declared depth only counts without points.
#[test]
fn the_investigation_depth_check_passes_when_an_investigation_point_reaches_the_required_depth() {
    let mut doc = compliant_demo();
    doc.investigation_depth = 1.0;
    let id = format!("en1997.2.4.2.investigation.{}", doc.footings[0].id);
    let result = |doc: &semio_s_artifact_norm_en1997_model::En1997Snapshot| check_project(doc).checks.into_iter().find(|c| c.id == id).expect("investigation check");
    assert!(result(&doc).utilization > 1.0, "declared 1 m is too shallow");
    doc.investigations = vec![point("bh-1", InvestigationKind::Borehole, 12.0, Vec::new()), point("bh-2", InvestigationKind::Borehole, 3.0, Vec::new())];
    let passing = result(&doc);
    assert!(passing.utilization <= 1.0, "a 12 m point reaches z_a");
    assert!(passing.explanation.en.contains("deepest investigation point"));
    doc.investigations[0].depth = 4.0;
    let failing = result(&doc);
    assert!(failing.utilization > 1.0 && failing.remedies.iter().any(|r| r.target.path.contains("investigations[id=bh-1].depth")), "the remedy deepens the deepest point");
}

/// 🧱️ A basis row of an unknown layer or a second row for the same parameter fails the integrity check.
#[test]
fn an_ambiguous_or_dangling_basis_row_fails_the_integrity_check() {
    let mut doc = worked_example();
    doc.characteristic_bases.push(basis("b5", "missing", CharacteristicParameter::UndrainedStrength, ParameterBasis::Derived));
    doc.characteristic_bases.push(basis("b6", "clay-1", CharacteristicParameter::UndrainedStrength, ParameterBasis::Declared));
    let report = check_project(&doc);
    for id in ["en1997.integrity.basis.b5", "en1997.integrity.basis.b6"] {
        assert!(report.checks.iter().any(|c| c.id == id && c.status == CheckStatus::Fail), "{id} must fail");
    }
}
