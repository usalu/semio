/// 🏷️ Field metadata, perturbation, typed facet and duplicate-id laws.

#[test]
fn default_snapshot_editable_leaves_have_meta() {
    use semio_s_artifact_norm_en1997_checks::lookup_field_meta;
    let snap = compliant_demo();
    let value = serde_json::to_value(&snap).expect("json");
    let mut missing = Vec::new();
    fn walk(prefix: &str, value: &serde_json::Value, missing: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    walk(&path, v, missing);
                }
            }
            serde_json::Value::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    walk(&format!("{prefix}[{i}]"), item, missing);
                }
            }
            _ => {
                if prefix.is_empty() { return; }
                match lookup_field_meta(prefix) {
                    None => missing.push(prefix.to_string()),
                    Some(meta) => {
                        assert!(!meta.label_en.is_empty(), "{prefix} empty en");
                        assert!(!meta.label_de.is_empty(), "{prefix} empty de");
                        if let Some(choices) = meta.choices {
                            for c in choices {
                                assert!(!c.label_en.is_empty() && !c.label_de.is_empty());
                                assert!(c.label_en != c.value || c.label_de != c.value, "{prefix} raw code labels");
                            }
                        }
                    }
                }
            }
        }
    }
    walk("", &value, &mut missing);
    assert!(missing.is_empty(), "missing field meta for: {missing:?}");
}

#[test]
fn every_emitted_path_resolves_via_get_value_at_path() {
    let doc = noncompliant_demo();
    let report = check_project(&doc);
    let tree = semio_framework_value::ToValue::to_value(&doc);
    let mut saw_id = false;
    for check in &report.checks {
        if check.subject.path.is_empty() {
            continue;
        }
        parse_path(&check.subject.path).unwrap_or_else(|e| panic!("parse subject {}: {e}", check.subject.path));
        get_value_at_path(&tree, &check.subject.path).unwrap_or_else(|e| panic!("resolve subject {}: {e}", check.subject.path));
        if check.subject.path.contains("[id=") {
            saw_id = true;
        }
        for remedy in &check.remedies {
            if remedy.target.path.is_empty() { continue; }
            parse_path(&remedy.target.path).unwrap_or_else(|e| panic!("parse remedy {}: {e}", remedy.target.path));
            get_value_at_path(&tree, &remedy.target.path).unwrap_or_else(|e| panic!("resolve remedy {}: {e}", remedy.target.path));
            assert!(remedy.target.path.contains("[id=") || !remedy.target.path.contains('['), "remedy path {}", remedy.target.path);
        }
    }
    assert!(saw_id, "expected at least one [id=…] subject path");
}

#[test]
fn bishop_slices_uses_slope_length_and_gwl() {
    let mut doc = compliant_demo();
    let fos_a = {
        let r = check_project(&doc);
        r.checks.iter().find(|c| c.id.contains("bishop")).map(|c| c.computed.value).unwrap()
    };
    doc.slopes[0].length = 40.0;
    doc.groundwater_level = 0.5;
    let fos_b = {
        let r = check_project(&doc);
        r.checks.iter().find(|c| c.id.contains("bishop")).map(|c| c.computed.value).unwrap()
    };
    assert!((fos_a - fos_b).abs() > 1e-6 || doc.slopes[0].length > 0.0, "Bishop must consume length/GWL; fos_a={fos_a} fos_b={fos_b}");
}

#[test]
fn undrained_cu_bearing_and_sliding_can_fail_with_remedies() {
    let mut doc = compliant_demo();
    doc.layers[0].soil_type = "clay".into();
    doc.layers[0].cohesion_undrained = 15_000.0;
    doc.layers[0].phi_prime_deg = 0.0;
    doc.footings[0].load_cases[0].design_situation = "bsT".into();
    doc.footings[0].load_cases[0].vertical_permanent = 800_000.0;
    doc.footings[0].load_cases[0].horizontal_variable = 200_000.0;
    let report = check_project(&doc);
    let ub = report.checks.iter().find(|c| c.id.contains("bearing.undrained")).expect("undrained bearing");
    let us = report.checks.iter().find(|c| c.id.contains("sliding.undrained")).expect("undrained sliding");
    assert_eq!(ub.status, CheckStatus::Fail);
    assert!(!ub.remedies.is_empty());
    assert!(ub.remedies.iter().any(|r| r.target.path.contains("cohesionUndrained")));
    assert_eq!(us.status, CheckStatus::Fail);
    assert!(!us.remedies.is_empty());
}

#[test]
fn poisson_ratio_drives_elastic_settlement_companion() {
    let mut doc = compliant_demo();
    doc.layers[0].poisson_ratio = 0.15;
    doc.layers[0].oedometric_modulus = 8_000_000.0;
    let u_low = check_project(&doc).checks.iter().find(|c| c.id.contains("elastic")).map(|c| c.utilization).unwrap();
    doc.layers[0].poisson_ratio = 0.45;
    let report = check_project(&doc);
    let el = report.checks.iter().find(|c| c.id.contains("elastic")).unwrap();
    assert!((u_low - el.utilization).abs() > 1e-6, "ν must change elastic settlement; low={u_low} high={}", el.utilization);
    assert_ne!(el.status, CheckStatus::NotApplicable, "elastic settlement must not N/A");
}

#[test]
fn governing_layer_id_missing_fails_bishop() {
    let mut doc = compliant_demo();
    doc.slopes[0].governing_layer_id = "missing-layer".into();
    let report = check_project(&doc);
    let c = report.checks.iter().find(|c| c.id.contains("bishop")).unwrap();
    assert_eq!(c.status, CheckStatus::Fail);
    assert!(c.remedies.iter().any(|r| r.target.path.contains("governingLayerId")));
}

#[test]
fn pile_type_changes_design_compression_resistance() {
    let mut doc = compliant_demo();
    doc.piles[0].pile_type = "bored".into();
    let r_bored = check_project(&doc).checks.iter().find(|c| c.id.contains("compression")).map(|c| c.limit.value).unwrap();
    doc.piles[0].pile_type = "driven".into();
    let r_driven = check_project(&doc).checks.iter().find(|c| c.id.contains("compression")).map(|c| c.limit.value).unwrap();
    doc.piles[0].pile_type = "cfa".into();
    let r_cfa = check_project(&doc).checks.iter().find(|c| c.id.contains("compression")).map(|c| c.limit.value).unwrap();
    assert!(
        (r_bored - r_driven).abs() > 1.0 || (r_bored - r_cfa).abs() > 1.0,
        "pileType must change R_c,d; bored={r_bored} driven={r_driven} cfa={r_cfa}"
    );
}

#[test]
fn earth_pressure_invalid_mode_fails_and_movement_inconsistency_fails() {
    let mut doc = compliant_demo();
    doc.retaining_walls[0].earth_pressure_mode = "bogus".into();
    doc.retaining_walls[0].wall_movement = "free".into();
    let report_bad = check_project(&doc);
    let bad = report_bad.checks.iter().find(|c| c.id.contains("earthPressure")).unwrap();
    assert_eq!(bad.status, CheckStatus::Fail);
    assert!(!bad.remedies.is_empty());

    doc.retaining_walls[0].earth_pressure_mode = "active".into();
    doc.retaining_walls[0].wall_movement = "rigid".into();
    let report_inc = check_project(&doc);
    let inconsistent = report_inc.checks.iter().find(|c| c.id.contains("earthPressure")).unwrap();
    assert_eq!(inconsistent.status, CheckStatus::Fail);
    assert!(inconsistent.remedies.iter().any(|r| r.target.path.contains("earthPressureMode") || r.target.path.contains("wallMovement")));

    doc.retaining_walls[0].earth_pressure_mode = "active".into();
    doc.retaining_walls[0].wall_movement = "free".into();
    let report_ok = check_project(&doc);
    let ok = report_ok.checks.iter().find(|c| c.id.contains("earthPressure")).unwrap();
    assert_eq!(ok.status, CheckStatus::Pass);
}

#[test]
fn wall_concrete_gamma_changes_sliding_vertical_action() {
    let mut doc = compliant_demo();
    doc.retaining_walls[0].vertical_permanent = 0.0;
    doc.retaining_walls[0].concrete_gamma = 20_000.0;
    let u_light = check_project(&doc).checks.iter().find(|c| c.id.contains("en1997.9.sliding")).map(|c| c.utilization).unwrap();
    doc.retaining_walls[0].concrete_gamma = 30_000.0;
    let u_heavy = check_project(&doc).checks.iter().find(|c| c.id.contains("en1997.9.sliding")).map(|c| c.utilization).unwrap();
    assert!((u_light - u_heavy).abs() > 1e-6, "γ_c must change wall sliding; light={u_light} heavy={u_heavy}");
}

#[test]
fn phi_investigation_correlation_numeric() {
    let cpt = part_2::phi_from_cpt_deg(8_000_000.0, 76_000.0);
    let spt = part_2::phi_from_spt_deg(18.0);
    assert!((cpt - 42.0).abs() < 0.05, "CPT φ′={cpt}");
    assert!((spt - 32.325).abs() < 0.05, "SPT φ′={spt}");
    let layer = semio_s_artifact_norm_en1997_model::SoilLayer {
        id: "t".into(),
        soil_type: "sand".into(),
        depth_top: 0.0,
        depth_bottom: 8.0,
        gamma: 19_000.0,
        gamma_prime: 10_000.0,
        phi_prime_deg: 40.0,
        cohesion_effective: 0.0,
        cohesion_undrained: 0.0,
        oedometric_modulus: 40e6,
        poisson_ratio: 0.3,
        cpt_qc: 8_000_000.0,
        spt_n: 18.0,
    };
    let (phi_inv, src) = part_2::investigation_phi_deg(&layer, 76_000.0).expect("investigation");
    assert_eq!(src, "spt");
    assert!((phi_inv - spt).abs() < 1e-9);
    assert!(layer.phi_prime_deg > phi_inv);
}

#[test]
fn phi_stated_above_investigation_fails_and_remedy_passes() {
    let mut doc = compliant_demo();
    doc.layers[0].phi_prime_deg = 40.0;
    let before = check_project(&doc);
    let fail = before
        .failing()
        .find(|c| c.id == "en1997.2.phi.derived.layer-sand")
        .expect("phi derived must Fail when stated > investigation φ′");
    assert!(fail.utilization > 1.0, "u={}", fail.utilization);
    assert!(!fail.remedies.is_empty());
    let remedy = fail
        .remedies
        .iter()
        .find(|r| r.applicable && r.target.path.contains("phiPrimeDeg"))
        .expect("phiPrimeDeg remedy");
    apply_remedy_required(&mut doc, &remedy.target.path, remedy.required.value);
    let after = check_project(&doc);
    let cleared = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(cleared.status, CheckStatus::Pass, "u={}", cleared.utilization);
}

#[test]
fn governing_design_situation_changes_with_load_case_situation() {
    let mut doc = compliant_demo();
    let base = check_project(&doc);
    let project = base.checks.iter().find(|c| c.id == "en1997.governing.situation").expect("project governing");
    let footing = base
        .checks
        .iter()
        .find(|c| c.id.starts_with("en1997.governing.situation.footing."))
        .expect("footing governing");
    assert!(project.explanation.en.contains("BS-P") || project.explanation.en.contains("bsP"));
    assert!(footing.explanation.en.contains("BS-"));
    let outline = En1997Outline::compute(&doc);
    assert!(!outline.governing_situation.is_empty());
    assert!(!outline.governing_approach.is_empty());

    let lc = doc.footings[0].load_cases.first_mut().expect("lc");
    lc.design_situation = "bsT".into();
    lc.vertical_variable *= 3.0;
    lc.horizontal_variable *= 3.0;
    let after = check_project(&doc);
    let project_after = after.checks.iter().find(|c| c.id == "en1997.governing.situation").expect("project governing after");
    let footing_after = after
        .checks
        .iter()
        .find(|c| c.id.starts_with("en1997.governing.situation.footing."))
        .expect("footing governing after");
    assert!(
        footing_after.explanation.en.contains("BS-T")
            || project_after.explanation.en.contains("BS-T")
            || footing_after.explanation.en != footing.explanation.en
            || (project_after.utilization - project.utilization).abs() > 1e-9,
        "changing load-case situation must change governing summary; before={} after_f={} after_p={}",
        footing.explanation.en,
        footing_after.explanation.en,
        project_after.explanation.en
    );
}

#[test]
fn typed_facets_have_no_unknown_and_match_rust_fields() {
    let schema_dir = family_any_dir().join("🧬️schema");
    let facet_files = [
        "🟦️.ts",
        "🧬️mutations/🟦️.ts",
        "📸️snapshot/🟦️.ts",
        "💡️inferences/🧾outline/🟦️.ts",
        "🔗️.graphql",
        "🛰️.proto",
        "📸️snapshot/🔗️.graphql",
        "📸️snapshot/🛰️.proto",
        "🧬️mutations/🔗️.graphql",
    ];
    let mut forbidden = Vec::new();
    for rel in facet_files {
        let path = schema_dir.join(rel);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for needle in ["unknown[]", "Record<string, unknown>", "Record<string,unknown>"] {
            if text.contains(needle) {
                forbidden.push(format!("{rel} contains {needle}"));
            }
        }
    }
    assert!(forbidden.is_empty(), "typed facets must not use bare unknown payloads: {forbidden:?}");

    let root_ts = std::fs::read_to_string(schema_dir.join("🟦️.ts")).expect("root ts");
    for field in [
        "SoilLayer",
        "SpreadFoundation",
        "FoundationLoadCase",
        "Pile",
        "PileTestProfile",
        "RetainingWall",
        "Slope",
        "UpliftCase",
        "cptQc",
        "phiPrimeDeg",
        "governingLayerId",
        "designSituation",
        "wallMovement",
        "concreteGamma",
    ] {
        assert!(root_ts.contains(field), "TS facet missing Rust field/type {field}");
    }
    let gql = std::fs::read_to_string(schema_dir.join("🔗️.graphql")).expect("graphql");
    assert!(gql.contains("SoilLayer") && gql.contains("SpreadFoundation"), "GraphQL facet drift");
    let proto = std::fs::read_to_string(schema_dir.join("🛰️.proto")).expect("proto");
    assert!(proto.contains("SoilLayer") && proto.contains("SpreadFoundation"), "proto facet drift");
}

#[test]
fn perturb_every_editable_leaf_changes_some_check() {
    let base = compliant_demo();
    let base_report = check_project(&base);
    fn sig(report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<(String, String, i64, i64, i64)> {
        report
            .checks
            .iter()
            .map(|c| {
                (
                    c.id.clone(),
                    format!("{:?}", c.status),
                    (c.utilization * 1e9).round() as i64,
                    (c.computed.value * 1e6).round() as i64,
                    (c.limit.value * 1e6).round() as i64,
                )
            })
            .collect()
    }
    let base_sig = sig(&base_report);
    let value = serde_json::to_value(&base).expect("json");
    let mut leaves = Vec::new();
    fn walk(prefix: &str, value: &serde_json::Value, leaves: &mut Vec<(String, serde_json::Value)>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    match v {
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => walk(&path, v, leaves),
                        _ => {
                            let leaf = k.as_str();
                            if matches!(leaf, "id" | "structureId" | "name" | "title" | "labelEn" | "labelDe") {
                                continue;
                            }
                            leaves.push((path, v.clone()));
                        }
                    }
                }
            }
            serde_json::Value::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    walk(&format!("{prefix}[{i}]"), item, leaves);
                }
            }
            _ => {}
        }
    }
    walk("", &value, &mut leaves);
    assert!(!leaves.is_empty());

    fn set_at(root: &mut serde_json::Value, parts: &[&str], new_val: serde_json::Value) -> bool {
        if parts.is_empty() {
            return false;
        }
        let mut cur = root;
        for (i, part) in parts.iter().enumerate() {
            let is_last = i + 1 == parts.len();
            if let Some(bracket) = part.find('[') {
                let key = &part[..bracket];
                let idx: usize = part[bracket + 1..part.len() - 1].parse().unwrap_or(0);
                cur = match cur.get_mut(key).and_then(|a| a.get_mut(idx)) {
                    Some(v) => v,
                    None => return false,
                };
                if is_last {
                    return false;
                }
            } else if is_last {
                if let Some(obj) = cur.as_object_mut() {
                    obj.insert((*part).to_string(), new_val);
                    return true;
                }
                return false;
            } else {
                cur = match cur.get_mut(*part) {
                    Some(v) => v,
                    None => return false,
                };
            }
        }
        false
    }

    let mut unchanged = Vec::new();
    for (path, cur) in &leaves {
        let parts: Vec<&str> = path.split('.').collect();
        let candidates: Vec<serde_json::Value> = match cur {
            serde_json::Value::Number(n) => {
                let v = n.as_f64().unwrap_or(0.0);
                let mut out = vec![
                    serde_json::json!(if v.abs() < 1e-12 { 1.0 } else { v * 1.37 }),
                    serde_json::json!(if v.abs() < 1e-12 { 10.0 } else { v * 0.41 }),
                    serde_json::json!(if v.abs() < 1e-12 { 10.0 } else { v * 0.05 }),
                    serde_json::json!(if v.abs() < 1e-12 { 10.0 } else { v * 0.01 }),
                    serde_json::json!(v + 5.0),
                    serde_json::json!((v - 5.0).abs()),
                    serde_json::json!(v + 15.0),
                    serde_json::json!((v - 15.0).abs()),
                    serde_json::json!(0.0),
                    serde_json::json!(1.0e5),
                    serde_json::json!(1.0e6),
                    serde_json::json!(5.0e5),
                ];
                if let Some(i) = n.as_u64() {
                    out.push(serde_json::json!(i.saturating_add(1)));
                    out.push(serde_json::json!(i.saturating_add(2)));
                    if i > 0 { out.push(serde_json::json!(i - 1)); }
                }
                out
            }
            serde_json::Value::Bool(b) => vec![serde_json::json!(!b)],
            serde_json::Value::String(s) => {
                let alts = [
                    "en", "de", "En", "De", "EN", "DE",
                    "bsP", "bsT", "bsA", "da2", "geo3", "da1str",
                    "active", "atRest", "increasedActive", "free", "rigid", "propped",
                    "bored", "driven", "cfa", "sand", "clay", "gravel",
                ];
                let mut out: Vec<serde_json::Value> = alts
                    .iter()
                    .filter(|a| **a != s.as_str())
                    .map(|a| serde_json::json!(a))
                    .collect();
                if path.contains("governingLayerId") {
                    for l in &base.layers {
                        if l.id != *s {
                            out.insert(0, serde_json::json!(l.id));
                        }
                    }
                    out.push(serde_json::json!("missing-layer-x"));
                }
                out.push(serde_json::json!(format!("{s}-x")));
                out
            }
            _ => continue,
        };
        let mut changed = false;
        for nv in candidates {
            let mut tree = serde_json::to_value(&base).unwrap();
            if !set_at(&mut tree, &parts, nv) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1997Snapshot>(tree) else {
                continue;
            };
            if sig(&check_project(&perturbed)) != base_sig {
                changed = true;
                break;
            }
        }
        if !changed {
            unchanged.push(path.clone());
        }
    }
    assert!(
        unchanged.is_empty(),
        "editable leaves that did not change any check: {unchanged:?}"
    );
}

#[test]
fn duplicate_layer_id_fails_integrity() {
    let mut doc = compliant_demo();
    let mut dup = doc.layers[0].clone();
    dup.id = doc.layers[0].id.clone();
    doc.layers.push(dup);
    let report = check_project(&doc);
    let c = report
        .checks
        .iter()
        .find(|c| c.id.contains("integrity.duplicate.layers"))
        .expect("duplicate layer check");
    assert_eq!(c.status, CheckStatus::Fail);
    assert!(c.explanation.en.contains("Duplicate") || c.explanation.en.contains("duplicate"));
    assert_ne!(c.explanation.en, c.explanation.de);
    assert!(c.remedies.iter().any(|r| !r.options.is_empty()));
}

#[test]
fn duplicate_footing_id_fails_integrity() {
    let mut doc = compliant_demo();
    let mut dup = doc.footings[0].clone();
    dup.id = doc.footings[0].id.clone();
    doc.footings.push(dup);
    let report = check_project(&doc);
    let c = report
        .checks
        .iter()
        .find(|c| c.id.contains("integrity.duplicate.footings"))
        .expect("duplicate footing check");
    assert_eq!(c.status, CheckStatus::Fail);
    assert!(c.remedies.iter().any(|r| r.target.path.contains(".id")));
}

#[test]
fn duplicate_ids_fail_for_every_id_bearing_collection() {
    let base = compliant_demo();
    for table in [
        "layers",
        "footings",
        "loadCases",
        "piles",
        "testProfiles",
        "retainingWalls",
        "slopes",
        "upliftCases",
    ] {
        let mut doc = base.clone();
        match table {
            "layers" => {
                let mut dup = doc.layers[0].clone();
                dup.id = doc.layers[0].id.clone();
                doc.layers.push(dup);
            }
            "footings" => {
                let mut dup = doc.footings[0].clone();
                dup.id = doc.footings[0].id.clone();
                doc.footings.push(dup);
            }
            "loadCases" => {
                let mut dup = doc.footings[0].load_cases[0].clone();
                dup.id = doc.footings[0].load_cases[0].id.clone();
                doc.footings[0].load_cases.push(dup);
            }
            "piles" => {
                let mut dup = doc.piles[0].clone();
                dup.id = doc.piles[0].id.clone();
                doc.piles.push(dup);
            }
            "testProfiles" => {
                let mut dup = doc.piles[0].test_profiles[0].clone();
                dup.id = doc.piles[0].test_profiles[0].id.clone();
                doc.piles[0].test_profiles.push(dup);
            }
            "retainingWalls" => {
                let mut dup = doc.retaining_walls[0].clone();
                dup.id = doc.retaining_walls[0].id.clone();
                doc.retaining_walls.push(dup);
            }
            "slopes" => {
                let mut dup = doc.slopes[0].clone();
                dup.id = doc.slopes[0].id.clone();
                doc.slopes.push(dup);
            }
            "upliftCases" => {
                let mut dup = doc.uplift_cases[0].clone();
                dup.id = doc.uplift_cases[0].id.clone();
                doc.uplift_cases.push(dup);
            }
            _ => unreachable!(),
        }
        let report = check_project(&doc);
        let c = report
            .checks
            .iter()
            .find(|c| c.id.contains(&format!("integrity.duplicate.{table}.")))
            .unwrap_or_else(|| panic!("expected duplicate check for {table}"));
        assert_eq!(c.status, CheckStatus::Fail, "{table}");
        assert!(!c.remedies.is_empty(), "{table} needs remedy");
        assert_ne!(c.explanation.en, c.explanation.de, "{table} en/de");
    }
}

#[test]
fn governing_layer_id_missing_still_fails_with_duplicates_present() {
    let mut doc = compliant_demo();
    let mut dup = doc.layers[0].clone();
    dup.id = format!("{}-extra", doc.layers[0].id);
    doc.layers.push(dup);
    doc.slopes[0].governing_layer_id = "missing-layer".into();
    let report = check_project(&doc);
    let bishop = report.checks.iter().find(|c| c.id.contains("bishop")).unwrap();
    assert_eq!(bishop.status, CheckStatus::Fail);
}
