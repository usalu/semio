use semio_framework_test_fixtures::fixture_path;
use semio_s_artifact_norm_en1998_schema::*;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckStatus};
use semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{active_coefficient, thrust, Acceleration, Interface, Side, Water};
use semio_s_artifact_norm_en1998_editor::field_meta::en1998_field_meta;
use semio_s_artifact_norm_en1998_inferences as inferences;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use std::path::PathBuf;
use std::process::Command;

#[semio_framework_async_macros::async_test]
async fn de_na_br_spectrum_at_t1() {
    let combo = na_de::GroundCombo::BR;
    let (s, tb, tc, td) = combo.spectrum_params();
    let a_g = na_de::a_gr_from_s_ap_r(1.5);
    let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, 0.25);
    assert!((s_e - 1.875).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn en_type1_vs_de_divergence() {
    let de = AnnexParams::De { s_ap_r: 1.5, combo: na_de::GroundCombo::BR };
    let en = AnnexParams::En { a_gr: 0.6, ground: 'B', spectrum_type1: true };
    let s_e_de = de.elastic_response_spectrum(0.25);
    let s_e_en = en.elastic_response_spectrum(0.25);
    assert!((s_e_de - 1.875).abs() < 1e-9);
    assert!((s_e_en - 0.6 * 1.2 * 2.5).abs() < 1e-9);
    assert!((s_e_en - s_e_de).abs() > 0.05);
}

#[semio_framework_async_macros::async_test]
async fn t1_ct_and_base_shear_lambda() {
    let h = 12.0;
    let t1 = part_1::t1_from_ct(0.075, h);
    assert!((t1 - 0.075 * h.powf(0.75)).abs() < 1e-12);
    let lambda = part_1::lambda_factor(t1, 0.25, 4);
    assert!((lambda - 0.85).abs() < 1e-12);
    let f_b = part_1::base_shear_n(0.5, 1_000_000.0, lambda);
    assert!((f_b - 0.5 * 1_000_000.0 * 0.85).abs() < 1e-6);
    let tall = part_1::t1_from_ct(0.075, 14.0);
    assert!((part_1::lambda_factor(tall, 0.25, 4) - 1.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn de_plateau_acceleration_maps_to_reference_acceleration() {
    assert!((na_de::a_gr_from_s_ap_r(2.0) - 0.8).abs() < 1e-12);
    assert_eq!(na_de::a_gr_from_s_ap_r(0.0), 0.0);
    let mut doc = semio_s_artifact_norm_en1998_model::En1998Snapshot::compliant_de_office();
    doc.site.s_ap_r = 0.0;
    doc.site.a_gr = 0.0;
    let report = inferences::check_full_seismic(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("zone0") && c.status == CheckStatus::NotApplicable));
}

#[semio_framework_async_macros::async_test]
async fn compliant_default_has_no_fail() {
    let report = inferences::evaluate(&semio_s_artifact_norm_en1998_model::En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(report.complies(), "fails: {:?}", report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn noncompliant_has_failures_and_remedies() {
    let report = inferences::evaluate(&semio_s_artifact_norm_en1998_model::En1998Snapshot::noncompliant_de_office());
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).collect();
    assert!(fails.len() >= 2);
    assert!(fails.iter().all(|c| !c.remedies.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn remedy_raising_vrd_improves_base_shear() {
    let mut doc = semio_s_artifact_norm_en1998_model::En1998Snapshot::noncompliant_de_office();
    let before = inferences::evaluate(&doc);
    let fail = before.checks.iter().find(|c| c.id.contains("baseShear") && c.status == CheckStatus::Fail).expect("base shear fail");
    let remedy = fail.remedies.iter().find(|r| r.target.path.contains("baseShearResistanceN")).expect("vrd remedy");
    doc.buildings[0].systems[0].base_shear_resistance_n = remedy.required.value;
    doc.buildings[0].systems[1].base_shear_resistance_n = remedy.required.value;
    let after = inferences::evaluate(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert!(after_check.utilization <= 1.0 + 1e-9, "u={}", after_check.utilization);
}

#[semio_framework_async_macros::async_test]
async fn remedy_setting_detailing_flips_q_detail() {
    let mut doc = semio_s_artifact_norm_en1998_model::En1998Snapshot::noncompliant_de_office();
    let before = inferences::evaluate(&doc);
    let fail = before.checks.iter().find(|c| c.id.contains("qDetail") && c.status == CheckStatus::Fail).expect("qDetail fail");
    assert!(fail.remedies.iter().any(|r| r.target.path.contains("detailingCompatibleWithQ") && r.target.path.contains("[id=")));
    for m in &mut doc.buildings[0].members {
        m.detailing_compatible_with_q = true;
        m.min_dimension_m = 0.35;
        m.rho = 0.012;
        m.rho_prime = 0.006;
        m.omega_wd = 0.12;
    }
    let after = inferences::evaluate(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same check");
    assert_eq!(after_check.status, CheckStatus::Pass, "qDetail must Pass after detailing; u={}", after_check.utilization);
}

#[semio_framework_async_macros::async_test]
async fn remedy_regularity_flips_when_both_flags_set() {
    let mut doc = semio_s_artifact_norm_en1998_model::En1998Snapshot::noncompliant_de_office();
    let before = inferences::evaluate(&doc);
    let fail = before.checks.iter().find(|c| c.id.contains("regularity") && c.status == CheckStatus::Fail).expect("regularity fail");
    assert!(fail.remedies.iter().any(|r| r.target.path.contains("planRegular")));
    assert!(fail.remedies.iter().any(|r| r.target.path.contains("elevationRegular")));
    doc.buildings[0].plan_regular = true;
    doc.buildings[0].elevation_regular = true;
    let after = inferences::evaluate(&doc);
    let after_check = after.checks.iter().find(|c| c.id == fail.id).expect("same");
    assert_eq!(after_check.status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn compliant_example_evaluates_clean() {
    let snap = En1998Snapshot::compliant_de_office();
    let report = inferences::evaluate(&snap);
    assert!(report.complies(), "fails={:?}", report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn fail_example_has_expected_fails() {
    let snap = En1998Snapshot::noncompliant_de_office();
    let report = inferences::evaluate(&snap);
    let fails = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).count();
    assert!(fails >= 2, "fail_count={fails}");
}

#[semio_framework_async_macros::async_test]
async fn emitted_remedy_paths_use_id_selectors_and_resolve() {
    let report = inferences::evaluate(&En1998Snapshot::noncompliant_de_office());
    let mut saw_id = false;
    for check in &report.checks {
        for remedy in &check.remedies {
            let path = &remedy.target.path;
            if path.contains("buildings[") || path.contains("systems[") || path.contains("storeys[") || path.contains("members[") {
                assert!(path.contains("[id="), "expected id selector in {path}");
                saw_id = true;
                let tree = semio_framework_value::ToValue::to_value(&En1998Snapshot::noncompliant_de_office());
                semio_s_plugin_norm_app_surface::get_value_at_path(&tree, path).unwrap_or_else(|e| panic!("path {path} must resolve: {e}"));
            }
        }
    }
    assert!(saw_id, "expected at least one id-path remedy");
}

#[semio_framework_async_macros::async_test]
async fn field_meta_covers_every_editable_leaf_of_default_snapshot() {
    let tree = semio_framework_value::ToValue::to_value(&En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    let mut missing = Vec::new();
    fn walk(prefix: &str, value: &semio_framework_value::DslValue, missing: &mut Vec<String>) {
        match value {
            semio_framework_value::DslValue::Object(entries) => {
                for (k, v) in entries {
                    let p = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    walk(&p, v, missing);
                }
            }
            semio_framework_value::DslValue::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    let p = format!("{prefix}[{i}]");
                    walk(&p, item, missing);
                }
            }
            _ => {
                if prefix.is_empty() { return; }
                match en1998_field_meta(prefix) {
                    Some(meta) if !meta.label.en.is_empty() && !meta.label.de.is_empty() => {}
                    Some(_) => missing.push(format!("{prefix} (empty label)")),
                    None => missing.push(prefix.to_string()),
                }
            }
        }
    }
    walk("", &tree, &mut missing);
    assert!(missing.is_empty(), "missing field-meta for: {missing:?}");
    let plateau = en1998_field_meta("site.sApR").expect("plateau meta");
    assert!(plateau.label.en.contains("S_aP,R") && plateau.label.de.contains("S_aP,R"));
    let combo = en1998_field_meta("site.deGroundCombo").expect("combo");
    assert!(combo.choices.unwrap().iter().any(|c| c.value == "B-R" && (c.label_en.contains("rock") || c.label_de.contains("Fels") || c.label_en.contains("B-R"))));
}

#[semio_framework_async_macros::async_test]
async fn example_snapshot_validates_against_json_schema() {
    let schema_path = snapshot_schema_path();
    for snap in [En1998Snapshot::compliant_de_office(), En1998Snapshot::noncompliant_de_office()] {
        let instance = serde_json::to_string(&snap).expect("instance");
        let tmp = std::env::temp_dir().join(format!("en1998-instance-{}.json", snap.buildings[0].id));
        std::fs::write(&tmp, &instance).unwrap();
        let status = semio_s_plugin_norm_compliance::python_oracle::command()
            .arg("-c")
            .arg("import json,sys,jsonschema; s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2])); jsonschema.validate(instance=i, schema=s); print('ok')")
            .arg(&schema_path)
            .arg(&tmp)
            .output()
            .expect("python jsonschema");
        assert!(status.status.success(), "jsonschema: {}\n{}", String::from_utf8_lossy(&status.stdout), String::from_utf8_lossy(&status.stderr));
    }
}

#[semio_framework_async_macros::async_test]
async fn python_oracle_matches_within_half_percent() {
    let script = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    assert!(script.exists(), "oracle script missing: {}", script.display());
    for (label, snap) in [("compliant", En1998Snapshot::compliant_de_office()), ("noncompliant", En1998Snapshot::noncompliant_de_office())] {
        let rust = inferences::evaluate(&snap);
        let input = serde_json::to_string(&snap).expect("json");
        let mut child = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).arg("--json").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().expect("oracle spawn");
        {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{label} oracle failed: {}", String::from_utf8_lossy(&output.stderr));
        let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
        let py_checks = py.get("checks").and_then(|c| c.as_array()).expect("oracle checks array");
        assert!(!py_checks.is_empty(), "{label}: oracle returned no checks");
        for pc in py_checks {
            let id = pc.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let pu = pc.get("utilization").and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("{label} oracle check {id} has no utilization"));
            let rc = rust.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{label} oracle check {id} missing in the Rust report"));
            let denom = pu.abs().max(rc.utilization.abs()).max(1e-9);
            let rel = (pu - rc.utilization).abs() / denom;
            assert!(rel <= 0.005 || (pu - rc.utilization).abs() <= 0.005, "{label} {id}: py={pu} rust={}", rc.utilization);
        }
    }
}

fn is_numbers_only(s: &str) -> bool {
    let t = s.trim();
    !t.is_empty()
        && t.chars().all(|c| c.is_ascii_digit() || " .,;:%°+-*/=<>≤≥≈±()[]{}_".contains(c))
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
                if matches!(v, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
                    walk_leaves(&child, v, visit);
                } else {
                    visit(&child);
                }
            }
        }
        _ => visit(path),
    }
}

fn resolve_path<'a>(root: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut cursor = root;
    let mut rest = path;
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let close = rest.find(']')?;
            let inner = &rest[1..close];
            let arr = cursor.as_array()?;
            cursor = if let Some(id) = inner.strip_prefix("id=") {
                arr.iter().find(|el| el.get("id").and_then(|v| v.as_str()) == Some(id))?
            } else {
                arr.get(inner.parse::<usize>().ok()?)?
            };
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        let field = &rest[..end];
        cursor = cursor.get(field)?;
        rest = &rest[end..];
    }
    Some(cursor)
}

enum PathSeg<'a> {
    Field(&'a str),
    Index(usize),
}

fn parse_segments(path: &str) -> Result<Vec<PathSeg<'_>>, String> {
    let mut rest = path;
    let mut out = Vec::new();
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let close = rest.find(']').ok_or_else(|| "unclosed [".to_string())?;
            let inner = &rest[1..close];
            if inner.starts_with("id=") {
                return Err("id selectors not supported for mutation paths in perturbation".into());
            }
            out.push(PathSeg::Index(inner.parse::<usize>().map_err(|e| e.to_string())?));
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        if end == 0 {
            return Err("empty field".into());
        }
        out.push(PathSeg::Field(&rest[..end]));
        rest = &rest[end..];
    }
    Ok(out)
}

fn walk_mut<'a>(root: &'a mut serde_json::Value, parents: &[PathSeg<'_>]) -> Result<&'a mut serde_json::Value, String> {
    let mut cursor = root;
    for seg in parents {
        cursor = match seg {
            PathSeg::Field(name) => cursor.get_mut(name).ok_or_else(|| format!("missing {name}"))?,
            PathSeg::Index(i) => cursor.as_array_mut().and_then(|a| a.get_mut(*i)).ok_or_else(|| format!("missing [{i}]"))?,
        };
    }
    Ok(cursor)
}

fn set_number_at_path(root: &mut serde_json::Value, path: &str, value: f64) -> Result<(), String> {
    let segments = parse_segments(path)?;
    let (last, parents) = segments.split_last().ok_or_else(|| "empty path".to_string())?;
    let cursor = walk_mut(root, parents)?;
    match last {
        PathSeg::Field(name) => {
            let slot = cursor.get_mut(name).ok_or_else(|| format!("missing leaf {name}"))?;
            *slot = serde_json::json!(value);
            Ok(())
        }
        _ => Err("leaf must be a field".into()),
    }
}

fn set_bool_at_path(root: &mut serde_json::Value, path: &str, value: bool) -> Result<(), String> {
    let segments = parse_segments(path)?;
    let (last, parents) = segments.split_last().ok_or_else(|| "empty path".to_string())?;
    let cursor = walk_mut(root, parents)?;
    match last {
        PathSeg::Field(name) => {
            let slot = cursor.get_mut(name).ok_or_else(|| format!("missing leaf {name}"))?;
            *slot = serde_json::Value::Bool(value);
            Ok(())
        }
        _ => Err("leaf must be a field".into()),
    }
}

fn set_i64_at_path(root: &mut serde_json::Value, path: &str, value: i64) -> Result<(), String> {
    let segments = parse_segments(path)?;
    let (last, parents) = segments.split_last().ok_or_else(|| "empty path".to_string())?;
    let cursor = walk_mut(root, parents)?;
    match last {
        PathSeg::Field(name) => {
            let slot = cursor.get_mut(name).ok_or_else(|| format!("missing leaf {name}"))?;
            *slot = serde_json::json!(value);
            Ok(())
        }
        _ => Err("leaf must be a field".into()),
    }
}

fn set_string_at_path(root: &mut serde_json::Value, path: &str, value: &str) -> Result<(), String> {
    let segments = parse_segments(path)?;
    let (last, parents) = segments.split_last().ok_or_else(|| "empty path".to_string())?;
    let cursor = walk_mut(root, parents)?;
    match last {
        PathSeg::Field(name) => {
            let slot = cursor.get_mut(name).ok_or_else(|| format!("missing leaf {name}"))?;
            *slot = serde_json::json!(value);
            Ok(())
        }
        _ => Err("leaf must be a field".into()),
    }
}

fn swap_enum(s: &str) -> String {
    match s {
        "de" => "en".into(),
        "en" => "de".into(),
        "zone0" => "zone1".into(),
        "zone1" => "zone2".into(),
        "zone2" => "zone3".into(),
        "zone3" => "zone2".into(),
        "A-R" => "B-R".into(),
        "B-R" => "C-R".into(),
        "C-R" => "B-T".into(),
        "B-T" => "C-T".into(),
        "C-T" => "C-S".into(),
        "C-S" => "A-R".into(),
        "frame" => "wall".into(),
        "wall" => "dual".into(),
        "dual" => "frame".into(),
        "rc" => "steel".into(),
        "steel" => "rc".into(),
        "dch" => "dcm".into(),
        "dcm" => "dcl".into(),
        "dcl" => "dch".into(),
        "ct" => "given".into(),
        "given" => "rayleigh".into(),
        "rayleigh" => "ct".into(),
        "ductile" => "brittle".into(),
        "brittle" => "ductile".into(),
        "x" => "y".into(),
        "y" => "x".into(),
        "nc" => "sd".into(),
        "sd" => "dl".into(),
        "dl" => "nc".into(),
        "kl1" => "kl2".into(),
        "kl2" => "kl3".into(),
        "kl3" => "kl1".into(),
        "A" | "a" => "B".into(),
        "B" | "b" => "E".into(),
        "C" | "c" => "D".into(),
        "D" | "d" => "E".into(),
        "E" | "e" => "F".into(),
        "F" | "f" => "A".into(),
        "H" | "h" => "B".into(),
        "column" => "beam".into(),
        "beam" => "column".into(),
        "type1" => "type2".into(),
        "type2" => "type1".into(),
        "II" => "III".into(),
        "III" => "II".into(),
        other => format!("{other}-x"),
    }
}

fn is_reference_id_leaf(path: &str) -> bool {
    let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
    matches!(
        leaf,
        "supportedBuildingId" | "supported_building_id"
    ) || leaf.ends_with("BuildingId") || leaf.ends_with("buildingId")
}

fn assert_perturbation_for_snapshot(base: &En1998Snapshot, label: &str) {
    let base_report = inferences::evaluate(base);
    let base_sig: Vec<_> = base_report
        .checks
        .iter()
        .map(|c| {
            (
                c.id.clone(),
                c.status,
                (c.computed.value * 1e9).round() as i64,
                (c.limit.value * 1e9).round() as i64,
                (c.utilization * 1e12).round() as i64,
            )
        })
        .collect();
    let value = serde_json::to_value(base).expect("json");
    let mut leaves = Vec::new();
    walk_leaves("", &value, &mut |path| {
        if path.is_empty() {
            return;
        }
        let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
        if leaf == "id" || leaf == "name" || leaf == "title" || leaf.starts_with("id=") {
            return;
        }
        leaves.push(path.to_string());
    });
    assert!(!leaves.is_empty(), "{label}: expected editable leaves");
    let mut unchanged = Vec::new();
    for path in &leaves {
        let mut tree = serde_json::to_value(base).unwrap();
        let Some(cur) = resolve_path(&tree, path).cloned() else {
            unchanged.push(format!("{path} (missing)"));
            continue;
        };
        let ok = if is_reference_id_leaf(path) {
            set_string_at_path(&mut tree, path, "dangling-ref-does-not-exist").is_ok()
        } else {
            match cur {
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64().or_else(|| n.as_u64().map(|u| u as i64)) {
                        set_i64_at_path(&mut tree, path, i.saturating_add(1)).is_ok()
                    } else {
                        let v = n.as_f64().unwrap_or(0.0);
                        let nv = if v.abs() < 1e-12 { 0.1 } else { v * 1.10 };
                        set_number_at_path(&mut tree, path, nv).is_ok()
                    }
                }
                serde_json::Value::Bool(b) => set_bool_at_path(&mut tree, path, !b).is_ok(),
                serde_json::Value::String(s) => set_string_at_path(&mut tree, path, &swap_enum(&s)).is_ok(),
                _ => false,
            }
        };
        if !ok {
            unchanged.push(format!("{path} (set failed)"));
            continue;
        }
        let Ok(perturbed) = serde_json::from_value::<En1998Snapshot>(tree) else {
            unchanged.push(format!("{path} (decode failed)"));
            continue;
        };
        let rep = inferences::evaluate(&perturbed);
        let sig: Vec<_> = rep
            .checks
            .iter()
            .map(|c| {
                (
                    c.id.clone(),
                    c.status,
                    (c.computed.value * 1e9).round() as i64,
                    (c.limit.value * 1e9).round() as i64,
                    (c.utilization * 1e12).round() as i64,
                )
            })
            .collect();
        if sig == base_sig {
            unchanged.push(path.clone());
        }
    }
    assert!(unchanged.is_empty(), "{label}: leaves without report change: {unchanged:?}");
}

#[semio_framework_async_macros::async_test]
async fn every_editable_leaf_perturbation_changes_report() {
    assert_perturbation_for_snapshot(&En1998Snapshot::compliant_de_office(), "compliant_de_office");
    assert_perturbation_for_snapshot(&En1998Snapshot::compliant_de_multipart(), "compliant_de_multipart");
    assert_perturbation_for_snapshot(&En1998Snapshot::compliant_en_office(), "compliant_en_office");
    assert_perturbation_for_snapshot(&En1998Snapshot::compliant_de_torsion_regular(), "compliant_de_torsion_regular");
}

fn assert_localized_report(snap: &En1998Snapshot, label: &str) {
    let report = inferences::evaluate(snap);
    let mut bad = Vec::new();
    for c in &report.checks {
        let en = c.explanation.en.trim();
        let de = c.explanation.de.trim();
        if en == de && !is_numbers_only(en) {
            bad.push(format!("{} explanation: {en}", c.id));
        }
        for (i, r) in c.remedies.iter().enumerate() {
            let en = r.action.en.trim();
            let de = r.action.de.trim();
            if en == de && !is_numbers_only(en) {
                bad.push(format!("{} remedy[{i}]: {en}", c.id));
            }
        }
    }
    assert!(bad.is_empty(), "{label}: identical en/de text: {bad:?}");
}

#[semio_framework_async_macros::async_test]
async fn committed_examples_have_localized_explanations_and_remedies() {
    assert_localized_report(&En1998Snapshot::compliant_de_office(), "compliant_de_office");
    assert_localized_report(&En1998Snapshot::noncompliant_de_office(), "noncompliant_de_office");
    assert_localized_report(&En1998Snapshot::compliant_de_multipart(), "compliant_de_multipart");
    assert_localized_report(&En1998Snapshot::noncompliant_de_multipart(), "noncompliant_de_multipart");
    assert_localized_report(&En1998Snapshot::compliant_en_office(), "compliant_en_office");
    assert_localized_report(&En1998Snapshot::noncompliant_de_torsion_irregular(), "noncompliant_de_torsion_irregular");
}

#[semio_framework_async_macros::async_test]
async fn multipart_examples_evaluate_expected_verdicts() {
    let ok = inferences::evaluate(&En1998Snapshot::compliant_de_multipart());
    assert!(
        ok.complies(),
        "multipart compliant fails: {:?}",
        ok.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>()
    );
    assert!(ok.checks.iter().any(|c| c.id.contains("en1998.2.") && c.status != CheckStatus::NotApplicable));
    assert!(ok.checks.iter().any(|c| c.id.contains("en1998.4.") && c.status != CheckStatus::NotApplicable));
    assert!(ok.checks.iter().any(|c| c.id.contains("en1998.5.") && c.status != CheckStatus::NotApplicable));
    assert!(ok.checks.iter().any(|c| c.id.contains("en1998.6.") && c.status != CheckStatus::NotApplicable));
    let bad = inferences::evaluate(&En1998Snapshot::noncompliant_de_multipart());
    let fails = bad.checks.iter().filter(|c| c.status == CheckStatus::Fail).count();
    assert!(fails >= 2, "multipart fail_count={fails}");
}

#[semio_framework_async_macros::async_test]
async fn de_ground_combo_table_matches_oracle_for_all_combos() {
    let script = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).arg("--combo-table").output().expect("oracle combo table");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    for (combo, rust_combo) in [
        ("A-R", na_de::GroundCombo::AR),
        ("B-R", na_de::GroundCombo::BR),
        ("C-R", na_de::GroundCombo::CR),
        ("B-T", na_de::GroundCombo::BT),
        ("C-T", na_de::GroundCombo::CT),
        ("C-S", na_de::GroundCombo::CS),
    ] {
        let (s, tb, tc, td) = rust_combo.spectrum_params();
        let row = py.get(combo).expect(combo);
        let ps = row.get("S").and_then(|v| v.as_f64()).unwrap();
        let ptb = row.get("TB").and_then(|v| v.as_f64()).unwrap();
        let ptc = row.get("TC").and_then(|v| v.as_f64()).unwrap();
        let ptd = row.get("TD").and_then(|v| v.as_f64()).unwrap();
        assert!((ps - s).abs() <= 0.005 * s.max(1.0), "{combo} S rust={s} py={ps}");
        assert!((ptb - tb).abs() <= 1e-9, "{combo} TB");
        assert!((ptc - tc).abs() <= 1e-9, "{combo} TC");
        assert!((ptd - td).abs() <= 1e-9, "{combo} TD");
        let mut snap = En1998Snapshot::compliant_de_office();
        snap.site.de_ground_combo = match combo {
            "A-R" => semio_s_artifact_norm_en1998_model::DeGroundCombo::AR,
            "B-R" => semio_s_artifact_norm_en1998_model::DeGroundCombo::BR,
            "C-R" => semio_s_artifact_norm_en1998_model::DeGroundCombo::CR,
            "B-T" => semio_s_artifact_norm_en1998_model::DeGroundCombo::BT,
            "C-T" => semio_s_artifact_norm_en1998_model::DeGroundCombo::CT,
            "C-S" => semio_s_artifact_norm_en1998_model::DeGroundCombo::CS,
            _ => unreachable!(),
        };
        let input = serde_json::to_string(&snap).unwrap();
        let mut child = semio_s_plugin_norm_compliance::python_oracle::command()
            .arg(&script)
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
        }
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success(), "{combo} oracle: {}", String::from_utf8_lossy(&out.stderr));
        let py_rep: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        let rust = inferences::evaluate(&snap);
        for pc in py_rep.get("checks").and_then(|c| c.as_array()).unwrap() {
            let id = pc.get("id").and_then(|v| v.as_str()).unwrap();
            let pu = pc.get("utilization").and_then(|v| v.as_f64()).unwrap();
            let rc = rust.checks.iter().find(|c| c.id == id).expect(id);
            let denom = pu.abs().max(rc.utilization.abs()).max(1e-9);
            let rel = (pu - rc.utilization).abs() / denom;
            assert!(rel <= 0.005 || (pu - rc.utilization).abs() <= 0.005, "{combo} {id}: py={pu} rust={}", rc.utilization);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn torsion_irregular_examples_evaluate_expected_verdicts() {
    let ok = inferences::evaluate(&En1998Snapshot::compliant_de_torsion_regular());
    assert!(
        ok.complies(),
        "torsion-regular compliant fails: {:?}",
        ok.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>()
    );
    assert!(ok.checks.iter().any(|c| c.id.contains("planRegularity")));
    assert!(ok.checks.iter().any(|c| c.id.contains(".torsion")));
    let bad = inferences::evaluate(&En1998Snapshot::noncompliant_de_torsion_irregular());
    let fails = bad.checks.iter().filter(|c| c.status == CheckStatus::Fail).count();
    assert!(fails >= 2, "torsion-irregular fail_count={fails}");
    assert!(bad.checks.iter().any(|c| c.id.contains("planRegularity") && c.status == CheckStatus::Fail)
        || bad.checks.iter().any(|c| c.id.contains(".torsion") && c.status == CheckStatus::Fail)
        || bad.checks.iter().any(|c| c.id.contains("elevationRegularity") && c.status == CheckStatus::Fail));
}

#[semio_framework_async_macros::async_test]
async fn en_annex_examples_evaluate_expected_verdicts() {
    let ok = inferences::evaluate(&En1998Snapshot::compliant_en_office());
    assert!(ok.complies(), "EN compliant fails: {:?}", ok.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>());
    let bad = inferences::evaluate(&En1998Snapshot::noncompliant_en_office());
    let fails = bad.checks.iter().filter(|c| c.status == CheckStatus::Fail).count();
    assert!(fails >= 2, "EN fail_count={fails}");
}

#[semio_framework_async_macros::async_test]
async fn catalogue_na4_cell_matches_evaluated_spectrum_params() {
    use semio_s_plugin_norm_app_surface::CatalogueCell;
    use semio_s_artifact_norm_en1998_editor::panels::catalogue::reference_tables;
    use semio_s_artifact_norm_en1998_schema::{na_de, AnnexParams};

    let snap = En1998Snapshot::compliant_de_office();
    let combo = na_de::GroundCombo::from(snap.site.de_ground_combo);
    let annex = AnnexParams::De {
        s_ap_r: snap.site.s_ap_r,
        combo,
    };
    let (_a_g, s, tb, tc, td) = annex.ground_params();
    let label = match combo {
        na_de::GroundCombo::AR => "A-R",
        na_de::GroundCombo::BR => "B-R",
        na_de::GroundCombo::CR => "C-R",
        na_de::GroundCombo::BT => "B-T",
        na_de::GroundCombo::CT => "C-T",
        na_de::GroundCombo::CS => "C-S",
    };
    let tables = reference_tables();
    let na4 = tables.iter().find(|t| t.id == "table-na-4-ground-combos").expect("catalogue NA.4");
    assert_ne!(na4.title_en, na4.title_de);
    let row = na4.rows.iter().find(|r| r.id == label).expect(label);
    let CatalogueCell::Number { value: cell_s, .. } = &row.cells[1] else { panic!("S cell") };
    let CatalogueCell::Number { value: cell_tb, .. } = &row.cells[2] else { panic!("TB cell") };
    let CatalogueCell::Number { value: cell_tc, .. } = &row.cells[3] else { panic!("TC cell") };
    let CatalogueCell::Number { value: cell_td, .. } = &row.cells[4] else { panic!("TD cell") };
    assert!((*cell_s - s).abs() <= 0.005 * s.max(1.0), "S catalogue={cell_s} evaluated={s}");
    assert!((*cell_tb - tb).abs() <= 1e-9);
    assert!((*cell_tc - tc).abs() <= 1e-9);
    assert!((*cell_td - td).abs() <= 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn duplicate_building_id_fails_integrity() {
    use semio_s_plugin_norm_compliance::RemedyBound;

    let mut snap = En1998Snapshot::compliant_de_office();
    let twin = snap.buildings[0].clone();
    snap.buildings.push(twin);
    let report = inferences::evaluate(&snap);
    let dup = report
        .checks
        .iter()
        .find(|c| c.id.contains("integrity.duplicate.buildings") && c.status == CheckStatus::Fail)
        .expect("duplicate building Fail");
    assert!(!dup.remedies.is_empty());
    assert!(dup.remedies.iter().any(|r| r.bound == RemedyBound::OneOf));
    assert_ne!(dup.explanation.en, dup.explanation.de);
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn dangling_supported_building_fails_with_one_of_remedy() {
    use semio_s_plugin_norm_compliance::RemedyBound;

    let mut snap = En1998Snapshot::compliant_de_multipart();
    assert!(!snap.foundations.is_empty() || !snap.assessments.is_empty());
    if let Some(f) = snap.foundations.first_mut() {
        f.supported_building_id = "missing-building".into();
    }
    if let Some(a) = snap.assessments.first_mut() {
        a.supported_building_id = "missing-building".into();
    }
    let report = inferences::evaluate(&snap);
    let href = report
        .checks
        .iter()
        .find(|c| c.id.contains("supportedBuilding") && c.status == CheckStatus::Fail)
        .expect("dangling supportedBuilding Fail");
    let remedy = href.remedies.iter().find(|r| r.bound == RemedyBound::OneOf).expect("one_of remedy");
    assert!(!remedy.options.is_empty());
    assert!(remedy.options.iter().any(|id| snap.buildings.iter().any(|b| b.id == *id)));
    assert_ne!(href.explanation.en, href.explanation.de);
}

pub(crate) fn family_any_dir() -> PathBuf {
    fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema/📸️snapshot/🔣️.json")
}

#[semio_framework_async_macros::async_test]
async fn python_oracle_formula_table_matches_rust_part_1() {
    let script = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    assert!(script.exists(), "oracle script missing: {}", script.display());
    let output = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).arg("--formulas").output().expect("oracle formulas");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    let close = |label: &str, key: &str, rust: f64| {
        let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("{label}: oracle key {key} missing"));
        assert!((expected - rust).abs() <= 1e-9 + 1e-6 * expected.abs(), "{label}: oracle={expected} rust={rust}");
    };
    let (a_g, s, tb, tc, td) = (0.6, 1.25, 0.05, 0.25, 2.0);
    for (key, t) in [("sd_q3_t0_02", 0.02), ("sd_q3_t0_15", 0.15), ("sd_q3_t0_6", 0.6), ("sd_q3_t3", 3.0)] {
        close("design spectrum", key, part_1::design_spectrum_horizontal(a_g, s, tb, tc, td, 3.0, t));
    }
    for (key, t) in [("sve_t0_03", 0.03), ("sve_t0_1", 0.1), ("sve_t0_5", 0.5), ("sve_t2", 2.0)] {
        close("vertical spectrum", key, part_1::vertical_elastic_spectrum(a_g, 0.9, t));
    }
    close("torsion", "delta_x5_le10", part_1::torsion_amplification(5.0, 10.0));
    close("drift", "drift_util", part_1::drift_utilization(0.012, 0.5, 0.0075, 3.5));
    close("cqc", "cqc_rho_0_9", part_1::cqc_correlation(0.05, 1.0, 0.9));
    close("srss", "srss_3_4", part_1::srss_combine(&[3.0, 4.0]));
    close("cqc", "cqc_two_modes", part_1::cqc_combine(&[3.0, 4.0], &[0.5, 0.45], 0.05));
    close("envelope", "envelope", part_1::horizontal_component_envelope(100.0, 50.0));
    let (periods, shapes) = part_1::shear_building_modes(&[1.0e5, 1.0e5], &[4.0e7, 4.0e7]);
    close("mode 1", "two_storey_t1", periods[0]);
    close("mode 2", "two_storey_t2", periods[1]);
    let meff = part_1::effective_modal_masses(&[1.0e5, 1.0e5], &shapes);
    close("meff sum", "two_storey_meff_sum", meff.iter().sum::<f64>());
    close("non-structural", "fa", part_1::non_structural_force_n(0.06, 1.25, 8.0, 12.0, 0.2, 0.5, 1000.0, 1.0, 2.0));
    close("cantilever period", "cantilever_period", part_6::uniform_cantilever_period_s(5.0e5, 45.0, 2.0e10));
    close("liq rd 6", "liq_rd_6", part_5::liquefaction_rd(6.0));
    close("liq rd 15", "liq_rd_15", part_5::liquefaction_rd(15.0));
    close("liq msf", "liq_msf_6_5", part_5::liquefaction_msf(6.5));
    close("liq n cs", "liq_n1_60cs_fc20", part_5::liquefaction_n1_60cs(12.0, 20.0));
    close("liq crr", "liq_crr_12", part_5::liquefaction_crr75(12.0));
    close("liq factor", "liq_factor", part_5::liquefaction_safety_factor(12.0, 20.0, 0.75, 18000.0 * 6.0, 18000.0 * 6.0 - 9810.0 * 4.0, 6.0, 6.5));
    close("pier shear", "pier_shear", part_2::pier_capacity_design_shear_n(1.35, 4.0e6, 8.0));
    close("timber", "timber_util", part_1::timber_connection_utilization(14.0, 100.0));
    close("masonry", "masonry_util", part_1::masonry_wall_utilization(3.6, 0.3));
    close("v_s30", "vs30_two_layers", part_1::vs30_from_layers(&[(10.0, 200.0), (20.0, 400.0)]));
    close("v_s30 layers", "gt_vs_layers", part_1::harmonic_average_over_layers(&[(10.0, 200.0), (20.0, 400.0)]));
    close("n_spt layers", "gt_n_layers", part_1::harmonic_average_over_layers(&[(5.0, 10.0), (25.0, 40.0)]));
    close("c_u layers", "gt_cu_layers", part_1::harmonic_average_over_layers(&[(10.0, 50.0), (20.0, 100.0)]));
    let index = |t: Option<part_1::GroundType>| match t.expect("ground type") { part_1::GroundType::A => 0.0, part_1::GroundType::B => 1.0, part_1::GroundType::C => 2.0, part_1::GroundType::D => 3.0, part_1::GroundType::E => 4.0, part_1::GroundType::S1 => 5.0, part_1::GroundType::S2 => 6.0 };
    let by = |v_s30_m_s: f64, n_spt: f64, c_u_kpa: f64| index(part_1::ground_type_table_3_1(&part_1::GroundInputs { v_s30_m_s, n_spt, c_u_kpa, ..Default::default() }));
    close("depth continued", "gt_depth_continued", part_1::time_averaged_over_depth(&[(10.0, 200.0), (15.0, 300.0)], 30.0));
    close("depth cut", "gt_depth_cut", part_1::time_averaged_over_depth(&[(20.0, 200.0), (20.0, 400.0)], 30.0));
    assert_eq!(part_1::time_averaged_over_depth(&[(10.0, 0.0)], 30.0), 0.0);
    close("class v_s layers", "gt_class_vs_layers", by(part_1::harmonic_average_over_layers(&[(10.0, 200.0), (20.0, 400.0)]), 0.0, 0.0));
    close("class v_s 900", "gt_class_vs_900", by(900.0, 0.0, 0.0));
    close("class v_s 800", "gt_class_vs_800", by(800.0, 0.0, 0.0));
    close("class v_s 360", "gt_class_vs_360", by(360.0, 0.0, 0.0));
    close("class v_s 120", "gt_class_vs_120", by(120.0, 0.0, 0.0));
    close("class n layers", "gt_class_n_layers", by(0.0, part_1::harmonic_average_over_layers(&[(5.0, 10.0), (25.0, 40.0)]), 0.0));
    close("class n 15", "gt_class_n_15", by(0.0, 15.0, 0.0));
    close("class n 60", "gt_class_n_60", by(0.0, 60.0, 0.0));
    close("class c_u layers", "gt_class_cu_layers", by(0.0, 0.0, part_1::harmonic_average_over_layers(&[(10.0, 50.0), (20.0, 100.0)])));
    close("class c_u 250", "gt_class_cu_250", by(0.0, 0.0, 250.0));
    close("class c_u 60", "gt_class_cu_60", by(0.0, 0.0, 60.0));
    let alluvium = part_1::GroundInputs { v_s30_m_s: 300.0, alluvium_thickness_m: 12.0, alluvium_v_s_m_s: 250.0, below_alluvium_v_s_m_s: 900.0, ..Default::default() };
    close("class alluvium", "gt_class_alluvium", index(part_1::ground_type_table_3_1(&alluvium)));
    close("class soft clay", "gt_class_soft_clay", index(part_1::ground_type_table_3_1(&part_1::GroundInputs { v_s30_m_s: 90.0, soft_clay_thickness_m: 12.0, soft_clay_pi: 50.0, ..Default::default() })));
    close("class liquefiable", "gt_class_liquefiable", index(part_1::ground_type_table_3_1(&part_1::GroundInputs { v_s30_m_s: 250.0, liquefiable_or_sensitive: true, ..Default::default() })));
    assert_eq!(part_1::ground_type_table_3_1(&part_1::GroundInputs { soft_clay_thickness_m: 8.0, soft_clay_pi: 50.0, v_s30_m_s: 150.0, ..Default::default() }), Some(part_1::GroundType::D));
    assert_eq!(part_1::ground_type_table_3_1(&part_1::GroundInputs::default()), None);
    assert_eq!(part_1::GroundType::S1.spectrum_letter(), None);
    assert_eq!(part_1::GroundType::E.spectrum_letter(), Some('E'));
    close("slenderness", "slenderness_util_30_10", part_1::plan_slenderness_utilization(30.0, 10.0));
    close("recess", "envelope_util_0_06", part_1::plan_envelope_utilization(0.06));
    close("set-back", "setback_util_0_3", part_1::setback_utilization(0.3));
    close("strong column", "strong_column_ratio", part_1::strong_column_ratio(1.0e6, 6.0e5));
    let walls = [(6.0, 0.3, 0.0, "s1"), (4.0, 0.3, 0.0, "s1"), (5.0, 0.3, std::f64::consts::FRAC_PI_2, "s1"), (3.0, 0.3, std::f64::consts::FRAC_PI_2, ""), (2.0, 0.25, 0.3, "s2")].map(|(length_m, thickness_m, direction_rad, storey_id)| part_1::MasonryWall { length_m, thickness_m, direction_rad, storey_id: storey_id.into() });
    for (key, along_y, storey) in [("mw_x_1", false, "s1"), ("mw_y_1", true, "s1"), ("mw_x_2", false, "s2"), ("mw_y_2", true, "s2")] {
        close(key, key, part_1::masonry_wall_density(&walls, 80.0, along_y, storey));
    }
    close("mw governing 1", "mw_governing_1", part_1::masonry_governing_density(&walls, 80.0, &["s1"]).expect("walls").0);
    close("mw governing 2", "mw_governing_2", part_1::masonry_governing_density(&walls, 80.0, &["s1", "s2"]).expect("walls").0);
    assert_eq!(part_1::masonry_governing_density(&walls, 80.0, &["s1", "s2"]).map(|g| (g.1, g.2)), Some((false, "s2".to_owned())));
    assert_eq!(part_1::masonry_governing_density(&[], 80.0, &["s1", "s2"]), None);
    let dangling = [part_1::MasonryWall { length_m: 9.0, thickness_m: 0.3, direction_rad: 0.0, storey_id: "missing".into() }];
    assert_eq!(part_1::masonry_governing_density(&dangling, 80.0, &["s1", "s2"]).map(|g| g.0), Some(0.0), "a wall naming no storey row counts in no storey");
}

#[semio_framework_async_macros::async_test]
async fn ground_class_follows_table_3_1_with_the_lower_class_on_a_boundary() {
    for (v, class) in [(900.0, 'A'), (800.0, 'B'), (500.0, 'B'), (360.0, 'C'), (200.0, 'C'), (180.0, 'D'), (100.0, 'D')] {
        assert_eq!(part_1::ground_class_from_vs30(v), Some(class), "{v}");
    }
    assert_eq!(part_1::ground_class_from_vs30(0.0), None);
}

fn details_report(edit: impl FnOnce(&mut En1998Snapshot)) -> CheckReport {
    let mut snap = En1998Snapshot::compliant_de_office();
    edit(&mut snap);
    inferences::evaluate(&snap)
}

fn status_of(report: &CheckReport, suffix: &str) -> CheckStatus {
    report.checks.iter().find(|c| c.id.ends_with(suffix)).unwrap_or_else(|| panic!("no check {suffix}")).status
}

#[semio_framework_async_macros::async_test]
async fn geometry_derived_regularity_fails_a_false_claim_with_a_remedy() {
    let fine = details_report(|_| {});
    assert_eq!(status_of(&fine, ".plan.slenderness"), CheckStatus::Pass);
    assert_eq!(status_of(&fine, ".regularity.derived"), CheckStatus::Pass);
    let bad = details_report(|d| { d.buildings[0].setback_ratio = 0.3; d.buildings[0].plan_envelope_gap_ratio = 0.06; });
    assert_eq!(status_of(&bad, ".elevation.setback"), CheckStatus::Fail);
    assert_eq!(status_of(&bad, ".plan.envelope"), CheckStatus::Fail);
    let claim = bad.checks.iter().find(|c| c.id.ends_with(".regularity.derived")).expect("claim check");
    assert_eq!(claim.status, CheckStatus::Fail);
    assert!(!claim.remedies.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn non_structural_element_and_capacity_design_checks_have_remedies() {
    let ok = details_report(|d| { let b = &mut d.buildings[0]; b.nonstructural_weight_n = 1000.0; b.nonstructural_height_m = 8.0; b.nonstructural_period_s = 0.2; b.nonstructural_resistance_n = 5000.0; b.column_moment_sum_nm = 1.0e6; b.beam_moment_sum_nm = 6.0e5; });
    assert_eq!(status_of(&ok, ".nonstructural.force"), CheckStatus::Pass);
    assert_eq!(status_of(&ok, ".capacityDesign.nodes"), CheckStatus::Pass);
    let bad = details_report(|d| { let b = &mut d.buildings[0]; b.nonstructural_weight_n = 1.0e6; b.nonstructural_height_m = 12.0; b.nonstructural_resistance_n = 1.0; b.column_moment_sum_nm = 5.0e5; b.beam_moment_sum_nm = 6.0e5; });
    for suffix in [".nonstructural.force", ".capacityDesign.nodes"] {
        let check = bad.checks.iter().find(|c| c.id.ends_with(suffix)).expect(suffix);
        assert_eq!(check.status, CheckStatus::Fail, "{suffix}");
        assert!(!check.remedies.is_empty(), "{suffix}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_site_inputs_drive_the_ground_type_and_the_spectrum_letter_with_the_unverified_basis_reported() {
    let by_spt = details_report(|d| { d.site.v_s30_m_s = 0.0; d.site.n_spt = 30.0; });
    assert_eq!(status_of(&by_spt, "site.groundClass"), CheckStatus::UnverifiedBasis);
    assert!(find(&by_spt, "site.groundClass").explanation.en.contains("ground type C"));
    let liquefiable = details_report(|d| { d.site.v_s30_m_s = 500.0; d.site.liquefiable_layer = true; });
    assert!(find(&liquefiable, "site.groundClass").explanation.en.contains("ground type S2"));
    let mut site = En1998Snapshot::compliant_de_office().site;
    site.en_ground_type = semio_s_artifact_norm_en1998_model::EnGroundType::B;
    assert_eq!(part_1::spectrum_ground_letter(&site), 'B');
    site.v_s30_m_s = 150.0;
    assert_eq!(part_1::spectrum_ground_letter(&site), 'D', "the derived type replaces the declared letter");
    site.liquefiable_layer = true;
    assert_eq!(part_1::spectrum_ground_letter(&site), 'B', "S2 has no spectrum letter and falls back to the declared one");
    site.liquefiable_layer = false;
    site.soft_clay_thickness_m = 12.0;
    site.soft_clay_pi = 50.0;
    assert_eq!(part_1::ground_type_table_3_1(&part_1::ground_inputs_of_site(&site, 150.0)), Some(part_1::GroundType::S1));
    assert!(part_1::GROUND_NDP_IDS.iter().all(|id| !semio_s_plugin_norm_compliance::ndp_is_verified(id)));
}

#[semio_framework_async_macros::async_test]
async fn the_spectrum_ground_follows_the_foundation_layers_before_the_site_input() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    snap.annex = semio_s_plugin_norm_compliance::AnnexChoice::En;
    snap.site.en_ground_type = semio_s_artifact_norm_en1998_model::EnGroundType::B;
    snap.site.v_s30_m_s = 500.0;
    snap.soil_layers.clear();
    assert_eq!(part_1::effective_v_s30(&snap), (500.0, part_1::GroundSource::Site));
    assert_eq!(part_1::spectrum_ground_letter_of(&snap), 'B');
    let foundation = snap.foundations[0].id.clone();
    let layer = |id: &str, thickness_m: f64, v_s: f64| semio_s_artifact_norm_en1998_model::En1998SoilLayer { id: id.into(), foundation_id: foundation.clone(), thickness_m, unit_weight_n_m3: 18000.0, shear_wave_velocity_m_s: v_s, n1_60: 0.0, fines_percent: 0.0 };
    snap.soil_layers = vec![layer("sl-1", 30.0, 150.0)];
    let (v, source) = part_1::effective_v_s30(&snap);
    assert!((v - 150.0).abs() < 1e-9 && source == part_1::GroundSource::FoundationLayers { foundation_id: foundation.clone() });
    assert_eq!(part_1::spectrum_ground_letter_of(&snap), 'D', "the layers replace the site value");
    snap.site.liquefiable_layer = true;
    assert_eq!(part_1::spectrum_ground_letter_of(&snap), 'B', "S2 keeps the special study and the declared letter");
}

#[semio_framework_async_macros::async_test]
async fn simple_masonry_measures_the_wall_rows_per_direction_and_reports_the_unverified_limit() {
    let wall = |d: &En1998Snapshot, id: &str, length: f64, direction: f64| {
        let mut m = d.buildings[0].members[0].clone();
        (m.id, m.material, m.role) = (id.into(), "masonry".into(), semio_s_artifact_norm_en1998_model::MemberKind::Wall);
        (m.wall_length_m, m.wall_thickness_m, m.wall_direction_rad, m.wall_storey_id) = (length, 0.3, direction, String::new());
        m
    };
    let rows = |d: &mut En1998Snapshot, y_length: f64| {
        d.buildings[0].claims_simple_masonry = true;
        let long = 0.2 * d.buildings[0].plan_width_m * d.buildings[0].plan_length_m / 0.3;
        let (x, y) = (wall(d, "w-x", long, 0.0), wall(d, "w-y", y_length, std::f64::consts::FRAC_PI_2));
        d.buildings[0].members.extend([x, y]);
    };
    let strong = details_report(|d| rows(d, 1.0e6));
    let ok = find(&strong, ".masonry.simple");
    assert_eq!(ok.status, CheckStatus::UnverifiedBasis, "the wall-ratio limits are unverified registry rows");
    assert!(ok.explanation.en.contains("governing direction"), "{}", ok.explanation.en);
    let weak = details_report(|d| rows(d, 1.0));
    let bad = find(&weak, ".masonry.simple");
    assert_eq!(bad.status, CheckStatus::Fail);
    assert!(bad.explanation.en.contains("governing direction y"), "{}", bad.explanation.en);
    assert!(bad.remedies.iter().any(|r| r.target.path.ends_with("members[id=w-y].wallLengthM")));
}

#[semio_framework_async_macros::async_test]
async fn a_wall_storey_id_must_name_a_storey_row_of_the_building_and_empty_means_every_storey() {
    let fine = details_report(|d| d.buildings[0].members[0].wall_storey_id = d.buildings[0].storeys[1].id.clone());
    assert!(!fine.checks.iter().any(|c| c.id.contains("integrity.reference") && c.id.ends_with("wallStoreyId")));
    let every = details_report(|d| d.buildings[0].members[0].wall_storey_id = String::new());
    assert!(!every.checks.iter().any(|c| c.id.contains("integrity.reference") && c.id.ends_with("wallStoreyId")));
    let dangling = details_report(|d| d.buildings[0].members[0].wall_storey_id = "s9".into());
    let check = dangling.checks.iter().find(|c| c.id.contains("integrity.reference") && c.id.ends_with("wallStoreyId")).expect("dangling reference");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.subject.path.contains("members[id=col-c1].wallStoreyId"), "{}", check.subject.path);
    assert!(!check.remedies.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn v_s30_ground_class_and_joint_gap_checks_report_mismatches() {
    let ok = details_report(|d| d.site.v_s30_m_s = 500.0);
    assert_eq!(status_of(&ok, "site.groundClass"), CheckStatus::UnverifiedBasis, "the Table 3.1 thresholds are unverified registry rows");
    let bad = details_report(|d| d.site.v_s30_m_s = 150.0);
    assert_eq!(status_of(&bad, "site.groundClass"), CheckStatus::Fail);
    let mut two = En1998Snapshot::compliant_de_office();
    let mut second = two.buildings[0].clone();
    second.id = "bldg-annex".into();
    two.buildings.push(second);
    two.buildings[0].joint_gap_m = 0.001;
    let report = inferences::evaluate(&two);
    assert_eq!(status_of(&report, ".joint.gap"), CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn zero_resistance_with_demand_fails_instead_of_passing() {
    let mut snap = En1998Snapshot::compliant_de_office();
    for b in &mut snap.buildings {
        for sys in &mut b.systems {
            sys.base_shear_resistance_n = 0.0;
        }
    }
    let report = inferences::evaluate(&snap);
    assert!(report.checks.iter().any(|c| c.id.ends_with(".baseShear") && c.status == CheckStatus::Fail));
}

#[semio_framework_async_macros::async_test]
async fn member_and_masonry_check_ids_are_unique() {
    let snap = En1998Snapshot::compliant_de_multipart();
    let report = inferences::evaluate(&snap);
    let mut seen = std::collections::BTreeSet::new();
    for c in &report.checks {
        assert!(seen.insert(c.id.clone()), "duplicate check id {}", c.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn drift_limit_multiplies_nu_on_the_demand_side() {
    assert!((part_1::drift_utilization(0.012, 0.5, 0.0075, 3.5) - 0.012 * 0.5 / (0.0075 * 3.5)).abs() < 1e-12);
    assert!(part_1::drift_utilization(0.001, 0.5, 0.0, 3.5).is_infinite());
    assert!((part_1::torsion_amplification(5.0, 10.0) - 1.3).abs() < 1e-12);
    assert!(part_1::soft_storey_utilization(1.0e6, 2.0e6) > 1.0);
    assert!(part_1::soft_storey_utilization(2.0e6, 1.0e6) < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn torsion_le_is_the_plan_side_perpendicular_to_the_action() {
    let mut snap = En1998Snapshot::compliant_de_office();
    for b in &mut snap.buildings {
        b.plan_width_m = 30.0;
        b.plan_length_m = 10.0;
        for st in &mut b.storeys {
            st.centre_of_mass_x_m = 0.0;
            st.centre_of_stiffness_x_m = 0.0;
            st.centre_of_mass_y_m = 0.0;
            st.centre_of_stiffness_y_m = 0.0;
        }
        if !b.systems.iter().any(|sys| sys.id == "sys-y") {
            let mut y = b.systems[0].clone();
            y.id = "sys-y".into();
            b.systems.push(y);
        }
        for sys in &mut b.systems {
            sys.direction = if sys.id == "sys-y" { "y".into() } else { "x".into() };
        }
    }
    let report = inferences::evaluate(&snap);
    let explanation = |suffix: &str| report.checks.iter().find(|c| c.id.ends_with(suffix)).unwrap_or_else(|| panic!("missing {suffix}")).explanation.en.clone();
    assert!(explanation(".sys-x.torsion").contains("L_e=10.0"), "{}", explanation(".sys-x.torsion"));
    assert!(explanation(".sys-y.torsion").contains("L_e=30.0"), "{}", explanation(".sys-y.torsion"));
}

#[semio_framework_async_macros::async_test]
async fn review_regressions_match_the_python_oracle() {
    let script = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).arg("--formulas").output().expect("oracle formulas");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    let close = |key: &str, rust: f64| {
        let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("oracle key {key} missing"));
        assert!((expected - rust).abs() <= 1e-9 + 1e-6 * expected.abs(), "{key}: oracle={expected} rust={rust}");
    };
    let t = part_1::rayleigh_period_shear_building(&[4.0e5; 4], &[1.2e9; 4]).expect("rayleigh");
    close("rayleigh_4_storeys", t);
    let exact = py.get("exact_t1_4_storeys").and_then(|v| v.as_f64()).expect("exact t1");
    assert!(t <= exact * (1.0 + 1e-12) && t >= exact * 0.95, "the Rayleigh frequency bounds the exact one from above, so its period lies just below T1: rayleigh={t} exact={exact}");
    assert!((t - 1.74).abs() > 0.3, "the Rayleigh period must follow the stiffness, got {t}");
    assert!(part_1::rayleigh_period_shear_building(&[4.0e5; 4], &[0.0; 4]).is_none());
    close("psi_e_retail_storey", part_1::psi_e("retail", false, true));
    close("psi_e_office_storey", part_1::psi_e("office", false, true));
    close("kae_kh_0_2", active_coefficient(&Interface::vertical(30.0, 0.0), 0.2_f64.atan()).expect("limit equilibrium"));
    let wall = |permeable: bool| thrust(Side::Active, &Interface::vertical(32.0, 0.0), 18000.0, 4.0, Water { height_m: 3.0, saturated_unit_weight: 20000.0, permeable }, Acceleration { kh: 0.2, kv: 0.0 }).expect("wall thrust");
    close("wall_thrust_permeable", wall(true).total());
    close("wall_thrust_low_permeability", wall(false).total());
    close("westergaard_thrust", wall(true).water_dynamic);
    close("sliding_friction_n", part_5::sliding_friction_resistance_n(2.0e7, 40000.0 * 200.0, 25.0, 1.0));
    close("sd_below_tb_q39", part_1::design_spectrum_at(0.6, 1.0, 1.25, 0.05, 0.25, 2.0, 3.9, 0.02));
    close("sd_floor_gamma_1_4", part_1::design_spectrum_at(0.6, 1.4, 1.25, 0.05, 0.25, 2.0, 3.0, 3.0));
    assert!(active_coefficient(&Interface::vertical(30.0, 0.0), 0.7_f64.atan()).is_none());
    assert_eq!(part_3::design_capacity_n(1.0e6, 1.2, 0.0), 0.0);
    assert!(part_1::drift_sensitivity(1.0e6, 0.01, 0.0, 3.0).is_infinite());
    assert!(part_1::p_delta_amplification(0.35).is_none());
}

#[semio_framework_async_macros::async_test]
async fn cancelled_evaluation_never_complies() {
    let snap = En1998Snapshot::compliant_de_office();
    let report = inferences::check_full_seismic_scoped(&snap, inferences::Scope::All, &mut |_| false);
    assert!(!report.complies());
    assert!(report.checks.iter().any(|c| c.id == "en1998.cancelled" && c.status == CheckStatus::Fail));
}

/// 🔮️ EN 1998 third-party oracle cases: the shared harness recomputes every blessed case in `🧪️tests/🧫️fixtures/🔮️oracle-cases` with the registered libraries and independent formulations and fails on any mismatch beyond the tolerance profile or between references.
#[test]
fn third_party_libraries_reproduce_the_blessed_oracle_cases() {
    let verified = semio_s_plugin_norm_compliance::python_oracle::verify_cases_above(env!("CARGO_MANIFEST_DIR"));
    assert!(verified >= 10, "expected at least 10 verified oracle cases, got {verified}");
}

#[semio_framework_async_macros::async_test]
async fn liquefaction_stress_ratio_matches_groundhog_cyclicstressratio_youd() {
    let sigma_v = 18000.0 * 6.0;
    let csr = part_5::liquefaction_csr(0.75, sigma_v, sigma_v - 9810.0 * 4.0, 6.0);
    assert!((csr - 0.0744710360728181).abs() < 1e-9, "{csr}");
    assert!((part_5::liquefaction_msf(6.5) - 1.4419221291650135).abs() < 1e-9);
    assert!((part_5::liquefaction_rd(6.0) - 0.9541).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn part_detail_checks_report_failures_with_remedies_and_unverified_warnings() {
    let mut snap = En1998Snapshot::compliant_de_office();
    snap.towers.push(semio_s_artifact_norm_en1998_model::En1998Tower { id: "tw-x".into(), height_m: 45.0, m_rd_nm: 1.0e12, is_chimney: false, q_nominal: 2.5, permanent_gk_n: 5.0e6, correlated_occupancy: true, variables: Vec::new(), ei_nm2: 2.0e10, base_rotational_stiffness_nm: 0.0 });
    snap.assessments.push(semio_s_artifact_norm_en1998_model::En1998Assessment { id: "as-x".into(), knowledge_level: semio_s_artifact_norm_en1998_model::KnowledgeLevel::Kl2, limit_state: semio_s_artifact_norm_en1998_model::LimitState::Nc, supported_building_id: "bldg-office".into(), r_k_n: 1.0e9, gamma_el: 1.0, chord_rotation_demand_rad: 0.05, chord_rotation_yield_rad: 0.005, chord_rotation_ultimate_rad: 0.02, section_depth_m: 0.0, section_width_m: 0.0, effective_depth_m: 0.0, shear_span_m: 0.0, axial_force_n: 0.0, tension_steel_area_m2: 0.0, compression_steel_area_m2: 0.0, bar_diameter_mm: 0.0, steel_yield_mpa: 0.0, concrete_strength_mpa: 0.0, stirrup_leg_area_m2: 0.0, stirrup_spacing_m: 0.0, stirrup_yield_mpa: 0.0, core_width_m: 0.0, core_depth_m: 0.0, restrained_bar_count: 0.0, restrained_bar_spacing_m: 0.0, diagonal_reinforcement_ratio: 0.0, shear_cracking_before_yield: 0.0, shear_force_demand_n: 0.0 });
    snap.foundations.push(semio_s_artifact_norm_en1998_model::En1998Foundation { id: "fd-x".into(), supported_building_id: "bldg-office".into(), area_m2: 100.0, p_rd_pa: 1.0e9, passive_resistance_n: 1.0e9, k_foundation: 1.0e9, k_soil: 1.0e8, liq_n1_60: 3.0, liq_fines_percent: 10.0, liq_depth_m: 6.0, liq_water_depth_m: 2.0, liq_unit_weight_n_m3: 18000.0, liq_magnitude: 7.0, liq_layer_thickness_m: 0.0, footing_width_m: 0.0, bearing_axial_n: 0.0, bearing_shear_n: 0.0, bearing_moment_nm: 0.0, bearing_n_max_n: 0.0, soil_friction_angle_deg: 0.0, pile_diameter_m: 0.0, pile_length_m: 0.0, pile_modulus_pa: 0.0, pile_subgrade_modulus_n_m3: 0.0, pile_head_shear_n: 0.0, pile_head_fixed: 0.0, pile_interface_depth_m: 0.0, pile_upper_vs_m_s: 0.0, pile_lower_vs_m_s: 0.0, pile_moment_rd_nm: 0.0, base_friction_angle_deg: 0.0, base_pore_pressure_pa: 0.0 });
    snap.bridges.push(semio_s_artifact_norm_en1998_model::En1998Bridge { id: "br-x".into(), period_ratio: 3.0, fundamental_period_s: 1.0, v_rd_n: 1.0e9, bearing_d_rd_m: 1.0, permanent_gk_n: 1.0e6, correlated_occupancy: true, variables: Vec::new(), pier_height_m: 8.0, pier_moment_rd_nm: 4.0e6, pier_shear_rd_n: 1.0e5, deck_length_m: 0.0, deck_ei_nm2: 0.0, importance_class: 2.0, vertical_period_s: 0.0, pier_cracked_inertia_ratio: 1.0 });
    let members = &mut snap.buildings[0].members;
    members[0].fastener_diameter_mm = 16.0;
    members[0].timber_thickness_mm = 100.0;
    members[1].wall_thickness_m = 0.2;
    members[1].wall_effective_height_m = 3.6;
    let report = inferences::evaluate(&snap);
    for suffix in [".period", ".chordRotation", ".liquefaction", ".pier.shear", ".timberConnection", ".masonryWall"] {
        let check = report.checks.iter().find(|c| c.id.ends_with(suffix)).unwrap_or_else(|| panic!("no check {suffix}"));
        if suffix != ".period" {
            assert_eq!(check.status, CheckStatus::Fail, "{suffix}");
            assert!(!check.remedies.is_empty(), "{suffix}");
        }
    }
    let liquefaction = report.checks.iter().find(|c| c.id.ends_with(".liquefaction")).expect("liquefaction check");
    assert!(liquefaction.ndps.iter().any(|id| id == "ndp.en1998.ref.liquefaction.youd-2001"));
    let pier = report.checks.iter().find(|c| c.id.ends_with(".pier.shear")).expect("pier check");
    assert!(pier.ndps.iter().any(|id| id == "ndp.en1998.en2.pier.gamma-0"));
}

pub(crate) fn oracle_values() -> serde_json::Value {
    let script = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).arg("--formulas").output().expect("oracle formulas");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    serde_json::from_slice(&output.stdout).expect("json")
}

#[semio_framework_async_macros::async_test]
async fn the_section_response_follows_its_strain_limits_and_leaves_the_registry_range_to_the_elastic_yield_curvature() {
    let k = part_3::AnnexA::from_registry(semio_s_plugin_norm_compliance::AnnexChoice::En);
    let member = part_3::RcMember { h: 0.5, b: 0.3, d: 0.45, l_v: 2.0, n: 300.0e3, a_s1: 1.8e-3, a_s2: 0.9e-3, a_si: 0.0, d_b: 0.02, f_y: 500.0, f_c: 30.0, a_sx: 1.57e-4, s_h: 0.1, f_yw: 500.0, b_0: 0.24, h_0: 0.44, restrained: 4.0, b_i: 0.12, rho_d: 0.0, shear_cracking: false };
    let response = member.section_response(&k).expect("response");
    assert!(response.kappa_u > response.kappa_y && response.m_u > response.m_y && response.m_y > 0.0, "{response:?}");
    let curve = member.moment_curvature(&k, 32);
    assert_eq!(curve.first(), Some(&(0.0, 0.0)));
    assert!((curve.last().expect("last").1 - response.m_u).abs() <= 1e-9 * response.m_u);
    let theta_y = part_3::chord_rotation_yield_of_curvature_rad(&member, &k, response.kappa_y);
    assert!((part_3::chord_rotation_yield_rad(&member, &k) - theta_y).abs() <= 1e-15);
    let strong = part_3::RcMember { f_c: k.max_concrete_mpa + 1.0, ..member };
    assert!(strong.section_response(&k).is_none() && strong.moment_curvature(&k, 8).is_empty());
    let elastic = part_3::chord_rotation_yield_of_curvature_rad(&strong, &k, part_3::yield_curvature_per_m(&strong, k.steel_modulus_mpa));
    assert!((part_3::chord_rotation_yield_rad(&strong, &k) - elastic).abs() <= 1e-15);
    let crushed = part_3::RcMember { n: 1.0e9, ..member };
    assert!(crushed.section_response(&k).is_none());
}

#[semio_framework_async_macros::async_test]
async fn annex_a_b_f_pile_n2_and_cantilever_formulas_match_the_independent_oracle() {
    let py = oracle_values();
    let close = |key: &str, rust: f64, relative: f64| {
        let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("oracle key {key} missing"));
        assert!((expected - rust).abs() <= 1e-12 + relative * expected.abs(), "{key}: oracle={expected} rust={rust}");
    };
    let annex = semio_s_plugin_norm_compliance::AnnexChoice::En;
    let k = part_3::AnnexA::from_registry(annex);
    let member = part_3::RcMember { h: 0.5, b: 0.3, d: 0.45, l_v: 2.0, n: 300.0e3, a_s1: 1.8e-3, a_s2: 0.9e-3, a_si: 0.0, d_b: 0.02, f_y: 500.0, f_c: 30.0, a_sx: 1.57e-4, s_h: 0.1, f_yw: 500.0, b_0: 0.24, h_0: 0.44, restrained: 4.0, b_i: 0.12, rho_d: 0.0, shear_cracking: false };
    close("rc_xi_y", part_3::yield_neutral_axis_ratio(&member, k.steel_modulus_mpa), 1e-9);
    close("rc_phi_y", part_3::yield_curvature_per_m(&member, k.steel_modulus_mpa), 1e-9);
    close("rc_theta_y_elastic", part_3::chord_rotation_yield_of_curvature_rad(&member, &k, part_3::yield_curvature_per_m(&member, k.steel_modulus_mpa)), 1e-9);
    let section = member.section_response(&k).expect("section response");
    close("rc_section_phi_y", section.kappa_y, 3e-3);
    close("rc_section_m_y", section.m_y, 3e-3);
    close("rc_section_phi_u", section.kappa_u, 3e-3);
    close("rc_section_m_u", section.m_u, 3e-3);
    close("rc_theta_y", part_3::chord_rotation_yield_rad(&member, &k), 3e-3);
    close("rc_alpha", part_3::confinement_effectiveness(&member), 1e-12);
    close("rc_theta_um", part_3::chord_rotation_ultimate_rad(&member, 1.5, &k), 1e-9);
    close("rc_stress_ratio", part_3::concrete_stress_ratio_at_yield(&member, k.steel_modulus_mpa), 1e-9);

    let layer = part_5::LiquefactionLayer { n1_60: 12.0, fines_percent: 20.0, unit_weight_n_m3: 18000.0, water_depth_m: 2.0, magnitude: 6.5, a_g_s_ms2: 0.75, csr_factor: semio_s_plugin_norm_compliance::ndp_number("ndp.en1998.en5.liquefaction.annex-b.csr-factor", annex) };
    let (total, effective) = layer.stresses(6.0);
    close("liq_csr_annex_b", part_5::liquefaction_csr_annex_b(layer.csr_factor, 0.75, total, effective), 1e-9);
    close("liq_csr_youd", part_5::liquefaction_csr(0.75, total, effective, 6.0), 1e-9);
    close("liq_factor_annex_b", layer.factor_annex_b(6.0), 1e-9);
    close("liq_factor_youd_check", layer.factor_youd(6.0), 1e-9);

    let surface = part_5::AnnexF::from_registry(annex);
    let f_bar = part_5::AnnexF::soil_inertia(0.25 * 9.81, 35.0);
    let (n, v, m) = surface.normalise(0.3e6, 0.05e6, 0.08e6, 2.0, 1.0e6).expect("normalised loads");
    close("annexf_f_bar", f_bar, 1e-12);
    close("annexf_bound", surface.vertical_bound(f_bar), 1e-12);
    close("annexf_surface", surface.surface(n, v, m, f_bar), 1e-9);
    close("annexf_util", surface.utilisation(n, v, m, f_bar), 1e-9);
    close("annexf_v_limit", surface.horizontal_limit(n, f_bar), 1e-9);
    close("annexf_m_limit", surface.moment_limit(n, f_bar), 1e-9);

    let free = part_5::pile_inertial(30.0e9, 0.6, 40.0, 20.0e6, 100.0e3, false);
    let fixed = part_5::pile_inertial(30.0e9, 0.6, 40.0, 20.0e6, 100.0e3, true);
    close("pile_beta", free.beta, 1e-9);
    close("pile_free_mmax", free.max_moment_nm, 1e-4);
    close("pile_free_y0", free.head_deflection_m, 1e-4);
    close("pile_fixed_mmax", fixed.max_moment_nm, 1e-4);
    close("pile_fixed_y0", fixed.head_deflection_m, 1e-4);
    let tau = part_5::interface_shear_stress_pa(0.25 * 9.81, 18000.0, 6.0);
    let e_1 = part_5::soil_young_modulus_pa(18000.0, 100.0);
    close("pile_kinematic", part_5::pile_kinematic_moment_nm(&part_5::Nikolaou::from_registry(annex), tau, 0.6, 20.0, 30.0e9, e_1, 100.0, 300.0), 1e-9);

    let (masses, stiffness, resistance) = ([200.0e3, 200.0e3, 150.0e3], [80.0e6, 70.0e6, 60.0e6], [900.0e3, 650.0e3, 350.0e3]);
    let (periods, shapes) = part_1::shear_building_modes(&masses, &stiffness);
    let roof = *shapes[0].last().expect("roof");
    let shape: Vec<f64> = shapes[0].iter().map(|x| x / roof).collect();
    let (gamma, m_star) = part_1::n2_participation(&masses, &shape).expect("participation");
    let (d_y, v_y, _) = part_1::n2_pushover_yield(&masses, &stiffness, &resistance, &shape).expect("yield");
    let sdof = [(0.0, 0.0), (d_y / gamma, v_y / gamma), (1.0e3 * d_y / gamma, v_y / gamma)];
    let spectrum = |t: f64| part_1::elastic_response_spectrum(1.0, 1.2, 0.15, 0.5, 2.0, t);
    let (target, _) = part_1::n2_solve(&sdof, &spectrum, 0.5, m_star).expect("n2");
    close("n2_period_1", periods[0], 1e-7);
    close("n2_gamma", gamma, 1e-7);
    close("n2_m_star", m_star, 1e-7);
    close("n2_d_y", d_y, 1e-7);
    close("n2_v_y", v_y, 1e-7);
    close("n2_t_star", target.t_star, 1e-6);
    close("n2_q_u", target.q_u, 1e-6);
    close("n2_d_t_star", target.d_t_star, 1e-6);
    let (adrs_sd, adrs_sa) = part_1::n2_adrs_point(spectrum(target.t_star), target.t_star);
    close("adrs_sa_y", part_1::n2_adrs_capacity_sa(v_y / gamma, m_star), 1e-9);
    close("adrs_sa_e", adrs_sa, 1e-9);
    close("adrs_sd_e", adrs_sd, 1e-6);
    close("adrs_radial_at_dy", part_1::n2_adrs_radial_sa(target.t_star, d_y / gamma), 1e-6);

    for n in 1..=3_u32 {
        close(&format!("cantilever_root_{n}"), part_6::cantilever_beta_l(n), 1e-12);
        close(&format!("cantilever_shape_{n}_half"), part_6::cantilever_mode_shape(n, 0.5), 1e-9);
        close(&format!("cantilever_participation_{n}"), part_6::cantilever_participation(n), 1e-4);
        close(&format!("cantilever_period_ratio_{n}"), part_6::cantilever_period_ratio(n), 1e-12);
    }
}

#[semio_framework_async_macros::async_test]
async fn every_registry_row_the_annex_a_b_f_and_pile_checks_cite_exists_and_is_unverified() {
    let ids = [
        "ndp.en1998.en3.annex-a.theta-y.base", "ndp.en1998.en3.annex-a.theta-y.slenderness", "ndp.en1998.en3.annex-a.theta-y.bar-slip", "ndp.en1998.en3.annex-a.theta-um.base", "ndp.en1998.en3.annex-a.theta-um.axial",
        "ndp.en1998.en3.annex-a.theta-um.reinforcement", "ndp.en1998.en3.annex-a.theta-um.shear-span", "ndp.en1998.en3.annex-a.theta-um.confinement", "ndp.en1998.en3.annex-a.theta-um.diagonal", "ndp.en1998.en3.annex-a.steel-modulus-mpa", "ndp.en1998.en3.annex-a.section.eps-c2-permille", "ndp.en1998.en3.annex-a.section.eps-cu-permille", "ndp.en1998.en3.annex-a.section.parabola-exponent", "ndp.en1998.en3.annex-a.section.eps-su-permille", "ndp.en1998.en3.annex-a.section.max-concrete-mpa",
        "ndp.en1998.en5.liquefaction.annex-b.csr-factor", "ndp.en1998.en5.liquefaction.screen-ag-s-g", "ndp.en1998.en5.liquefaction.screen-depth-m", "ndp.en1998.en5.annex-f.gamma-rd", "ndp.en1998.en5.annex-f.sand.a", "ndp.en1998.en5.annex-f.sand.beta",
        "ndp.en1998.en5.annex-f.sand.gamma", "ndp.en1998.en5.annex-f.sand.k-prime", "ndp.en1998.en5.pile.stiffness-contrast", "ndp.en1998.ref.pile.nikolaou-2001.coefficient", "ndp.en1998.ref.pile.nikolaou-2001.velocity-exponent",
    ];
    for id in ids {
        let entry = semio_s_plugin_norm_compliance::ndp(id).unwrap_or_else(|| panic!("registry row {id} missing"));
        assert_eq!(entry.status, semio_s_plugin_norm_compliance::NdpStatus::Unverified, "{id}");
    }
}

pub(crate) fn find<'a>(report: &'a CheckReport, suffix: &str) -> &'a semio_s_plugin_norm_compliance::CheckResult {
    report.checks.iter().find(|c| c.id.ends_with(suffix)).unwrap_or_else(|| panic!("no check {suffix}"))
}

#[semio_framework_async_macros::async_test]
async fn chord_rotation_capacity_is_derived_from_the_section_and_fails_with_a_stirrup_remedy() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    let a = &mut snap.assessments[0];
    (a.section_depth_m, a.section_width_m, a.effective_depth_m, a.shear_span_m, a.axial_force_n) = (0.5, 0.3, 0.45, 2.0, 100.0e3);
    (a.tension_steel_area_m2, a.compression_steel_area_m2, a.bar_diameter_mm, a.steel_yield_mpa, a.concrete_strength_mpa) = (1.8e-3, 0.9e-3, 20.0, 500.0, 30.0);
    (a.stirrup_leg_area_m2, a.stirrup_spacing_m, a.stirrup_yield_mpa, a.core_width_m, a.core_depth_m, a.restrained_bar_count, a.restrained_bar_spacing_m) = (1.57e-4, 0.2, 500.0, 0.24, 0.44, 4.0, 0.12);
    a.chord_rotation_yield_rad = 0.0;
    a.chord_rotation_ultimate_rad = 0.0;
    let k = part_3::AnnexA::from_registry(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let cf = part_3::confidence_factor(a.knowledge_level);
    let ultimate = part_3::chord_rotation_ultimate_rad(&part_3::RcMember::of(a).expect("section").over_cf(cf), a.gamma_el, &k);
    let design = 0.75 * ultimate;
    a.chord_rotation_demand_rad = 1.05 * design;
    let failing = inferences::evaluate(&snap);
    let check = find(&failing, ".chordRotation");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.utilization > 1.0 && !check.remedies.is_empty());
    assert!(check.remedies.iter().any(|r| r.target.path.ends_with(".stirrupSpacingM")), "{:?}", check.remedies.iter().map(|r| r.target.path.clone()).collect::<Vec<_>>());
    assert!(failing.checks.iter().any(|c| c.id.ends_with(".chordRotation.derivation")));
    assert!(failing.checks.iter().any(|c| c.id.ends_with(".chordRotation.linearity")));
    snap.assessments[0].chord_rotation_demand_rad = 0.5 * design;
    let passing = inferences::evaluate(&snap);
    let ok = find(&passing, ".chordRotation");
    assert!(ok.status != CheckStatus::Fail && ok.utilization < 1.0, "{:?} {}", ok.status, ok.utilization);
}

#[semio_framework_async_macros::async_test]
async fn annex_b_liquefaction_is_stricter_than_youd_by_the_depth_reduction_and_screens_are_reported() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    let f = &mut snap.foundations[0];
    (f.liq_n1_60, f.liq_fines_percent, f.liq_depth_m, f.liq_water_depth_m, f.liq_unit_weight_n_m3, f.liq_magnitude, f.liq_layer_thickness_m) = (12.0, 20.0, 6.0, 2.0, 18000.0, 6.5, 0.0);
    let report = inferences::evaluate(&snap);
    let annex_b = find(&report, ".liquefaction");
    let youd = find(&report, ".liquefaction.youd");
    let factor_b = annex_b.limit.value;
    let ratio = factor_b / youd.computed.value;
    assert!((ratio - part_5::liquefaction_rd(6.0)).abs() < 1e-9, "{ratio}");
    assert_eq!(youd.status, CheckStatus::Info);
    assert!(report.checks.iter().any(|c| c.id.ends_with(".liquefaction.screen")));
    snap.foundations[0].liq_layer_thickness_m = 4.0;
    let thick = inferences::evaluate(&snap);
    assert!(find(&thick, ".liquefaction").limit.value <= factor_b + 1e-9, "the governing depth of a thick layer cannot be safer than its middle");
}

#[semio_framework_async_macros::async_test]
async fn annex_f_bearing_check_fails_with_an_n_max_remedy_and_pile_moments_are_checked() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    let f = &mut snap.foundations[0];
    (f.footing_width_m, f.bearing_n_max_n, f.soil_friction_angle_deg, f.bearing_axial_n, f.bearing_shear_n, f.bearing_moment_nm) = (2.0, 1.0e6, 35.0, 0.3e6, 0.05e6, 0.08e6);
    (f.pile_diameter_m, f.pile_length_m, f.pile_modulus_pa, f.pile_subgrade_modulus_n_m3, f.pile_head_shear_n, f.pile_head_fixed) = (0.6, 20.0, 30.0e9, 20.0e6, 100.0e3, 0.0);
    (f.pile_interface_depth_m, f.pile_upper_vs_m_s, f.pile_lower_vs_m_s, f.pile_moment_rd_nm, f.liq_unit_weight_n_m3) = (6.0, 100.0, 300.0, 1.0e9, 18000.0);
    let safe = inferences::evaluate(&snap);
    assert!(find(&safe, ".annexF").status != CheckStatus::Fail);
    for suffix in [".pile.length", ".pile.inertial", ".pile.kinematic", ".pile.moment"] {
        assert!(safe.checks.iter().any(|c| c.id.ends_with(suffix)), "{suffix}");
    }
    assert!(find(&safe, ".pile.moment").utilization < 1.0);
    let f = &mut snap.foundations[0];
    f.bearing_shear_n *= 8.0;
    f.bearing_moment_nm *= 8.0;
    f.pile_moment_rd_nm = 1.0e4;
    let unsafe_report = inferences::evaluate(&snap);
    let bearing = find(&unsafe_report, ".annexF");
    assert_eq!(bearing.status, CheckStatus::Fail);
    assert!(bearing.remedies.iter().any(|r| r.target.path.ends_with(".bearingNMaxN")));
    let moment = find(&unsafe_report, ".pile.moment");
    assert_eq!(moment.status, CheckStatus::Fail);
    assert!(moment.remedies.iter().any(|r| r.target.path.ends_with(".pileMomentRdNm")));
}

#[test]
fn annex_switch_reads_the_registry_and_changes_only_the_expected_values() {
    let en = AnnexParams::En { a_gr: 1.0, ground: 'B', spectrum_type1: true };
    assert_eq!(en.ground_params(), (1.0, 1.2, 0.15, 0.5, 2.0));
    assert_eq!(en.ndp_ids()[0], "ndp.en1998.t32.b.s");
    let de = AnnexParams::De { s_ap_r: 2.5, combo: na_de::GroundCombo::BT };
    assert_eq!(de.ground_params(), (1.0, 1.25, 0.10, 0.30, 2.0));
    assert_eq!(de.ndp_ids()[3], "ndp.en1998.na4.bt.td");
    assert_eq!(part_1::psi2_de("office"), 0.3);
}

#[test]
fn table_f1_constants_follow_the_target_spec_and_stay_unverified() {
    let annex = semio_s_plugin_norm_compliance::AnnexChoice::En;
    let k = part_5::AnnexF::from_registry(annex);
    let spec = [("a", k.a, 0.92), ("b", k.b, 1.25), ("c", k.c, 0.92), ("d", k.d, 1.25), ("e", k.e, 0.92), ("f", k.f, 0.41), ("m", k.m, 0.96), ("k", k.k, 0.39), ("k'", k.k_prime, 0.39), ("c_T", k.c_t, 2.14), ("c'_T", k.c_t_prime, 1.81), ("beta", k.beta, 2.57), ("gamma", k.gamma, 4.0)];
    for (name, value, expected) in spec {
        assert_eq!(value, expected, "Table F.1 {name} must equal the target spec GE-06");
    }
    for key in ["a", "b", "c", "d", "e", "f", "m", "k", "k-prime", "c-t", "c-t-prime", "c-m", "c-m-prime", "beta", "gamma"] {
        let id = format!("ndp.en1998.en5.annex-f.sand.{key}");
        let entry = semio_s_plugin_norm_compliance::ndp(&id).unwrap_or_else(|| panic!("registry row {id} missing"));
        assert_eq!(entry.status, semio_s_plugin_norm_compliance::NdpStatus::Unverified, "{id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn soil_profile_n2_capacity_and_accelerogram_acceptance_match_the_independent_oracle() {
    let py = oracle_values();
    let close = |key: &str, rust: f64, relative: f64| {
        let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("oracle key {key} missing"));
        assert!((expected - rust).abs() <= 1e-12 + relative * expected.abs(), "{key}: oracle={expected} rust={rust}");
    };
    let annex = semio_s_plugin_norm_compliance::AnnexChoice::En;
    let layer = |thickness_m: f64, unit_weight_n_m3: f64, shear_wave_velocity_m_s: f64, n1_60: f64, fines_percent: f64| part_5::ProfileLayer { thickness_m, unit_weight_n_m3, shear_wave_velocity_m_s, n1_60, fines_percent };
    let profile = part_5::SoilProfile {
        layers: vec![layer(3.0, 17000.0, 150.0, 0.0, 40.0), layer(4.0, 18000.0, 120.0, 12.0, 20.0), layer(5.0, 19000.0, 200.0, 20.0, 10.0)],
        water_depth_m: 2.0,
        magnitude: 6.5,
        a_g_s_ms2: 0.75,
        csr_factor: semio_s_plugin_norm_compliance::ndp_number("ndp.en1998.en5.liquefaction.annex-b.csr-factor", annex),
    };
    let (total, effective) = profile.stresses(6.0);
    close("soil_total_6", total, 1e-9);
    close("soil_effective_6", effective, 1e-9);
    close("soil_factor_6", profile.factor(6.0), 1e-9);
    close("soil_governing_2", profile.layer_governing(1).1, 1e-9);
    close("soil_governing_3", profile.layer_governing(2).1, 1e-9);
    close("soil_vs30", profile.equivalent_velocity_m_s(30.0), 1e-9);
    close("soil_period", profile.site_period_s(), 1e-9);
    assert_eq!(profile.layer_governing(0).1, f64::INFINITY, "a layer with (N1)60 = 0 is not assessed");

    let (masses, stiffness, resistance) = ([200.0e3, 200.0e3, 150.0e3], [80.0e6, 70.0e6, 60.0e6], [900.0e3, 650.0e3, 350.0e3]);
    let (_, shapes) = part_1::shear_building_modes(&masses, &stiffness);
    let roof = *shapes[0].last().expect("roof");
    let shape: Vec<f64> = shapes[0].iter().map(|x| x / roof).collect();
    let spectrum = |t: f64| part_1::elastic_response_spectrum(1.0, 1.2, 0.15, 0.5, 2.0, t);
    let assessment = part_1::n2_assess(&masses, &stiffness, &resistance, &shape, &spectrum, 0.5).expect("n2");
    close("n2c_weakest", assessment.weakest as f64, 0.0);
    close("n2c_weakest_drift", assessment.weakest_drift_m, 1e-7);
    close("n2c_roof_target", assessment.target_roof_m, 1e-6);
    close("n2c_roof_capacity", part_1::n2_roof_capacity_m(&assessment, 3.2, 0.03), 1e-6);
    close("n2c_required_theta", part_1::n2_required_drift_capacity_rad(&assessment, 3.2), 1e-6);

    let records = vec![vec![1.0, 1.1, 0.95, 0.9], vec![0.9, 1.2, 1.0, 0.8], vec![1.1, 1.0, 1.05, 1.0]];
    let acceptance = part_1::accelerogram_set_acceptance(&records, &[1.0, 1.0, 1.0, 1.0], 3, 0.9);
    close("th_lowest_mean_ratio", acceptance.lowest_mean_ratio, 1e-12);
    assert!(acceptance.enough_records && acceptance.spectrum_matched);
    assert!(!part_1::accelerogram_set_acceptance(&records[..2], &[1.0; 4], 3, 0.9).enough_records);
}

#[semio_framework_async_macros::async_test]
async fn soil_profile_checks_fail_with_a_blow_count_remedy_and_report_the_site_response() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    snap.annex = semio_s_plugin_norm_compliance::AnnexChoice::En;
    snap.site.en_ground_type = semio_s_artifact_norm_en1998_model::EnGroundType::B;
    let f = &mut snap.foundations[0];
    (f.liq_magnitude, f.liq_water_depth_m) = (6.5, 2.0);
    let id = f.id.clone();
    let row = |name: &str, thickness_m: f64, n1_60: f64, v_s: f64| semio_s_artifact_norm_en1998_model::En1998SoilLayer { id: name.into(), foundation_id: id.clone(), thickness_m, unit_weight_n_m3: 18000.0, shear_wave_velocity_m_s: v_s, n1_60, fines_percent: 10.0 };
    snap.soil_layers = vec![row("l1", 3.0, 0.0, 300.0), row("l2", 4.0, 3.0, 120.0), row("l3", 5.0, 30.0, 200.0)];
    let report = inferences::evaluate(&snap);
    let weak = find(&report, ".layer.l2.liquefaction");
    assert_eq!(weak.status, CheckStatus::Fail);
    assert!(weak.remedies.iter().any(|r| r.target.path.ends_with("soilLayers[id=l2].n160")));
    assert!(!report.checks.iter().any(|c| c.id.ends_with(".layer.l1.liquefaction")), "(N1)60 = 0 is not assessed");
    assert!(report.checks.iter().any(|c| c.id.ends_with(".profile.liquefaction")));
    let site = find(&report, ".profile.siteResponse");
    assert_eq!(site.status, CheckStatus::Fail, "v_s,30 of the profile is 189 m/s (class C) while the site declares B");
    assert!(site.remedies.iter().any(|r| r.target.path == "site.enGroundType"));
}

#[semio_framework_async_macros::async_test]
async fn n2_performance_points_are_checked_against_the_displacement_capacity_with_remedies() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    let building_id = snap.buildings[0].id.clone();
    for storey in &mut snap.buildings[0].storeys {
        (storey.stiffness_x, storey.stiffness_y, storey.shear_resistance_n) = (8.0e7, 8.0e7, 2.0e4);
    }
    let a = &mut snap.assessments[0];
    (a.supported_building_id, a.chord_rotation_yield_rad, a.chord_rotation_ultimate_rad, a.gamma_el) = (building_id, 0.005, 0.05, 1.0);
    let roomy = inferences::evaluate(&snap);
    for state in [".n2.sd", ".n2.nc"] {
        assert!(roomy.checks.iter().any(|c| c.id.ends_with(state)), "{state}");
    }
    snap.assessments[0].chord_rotation_ultimate_rad = 1.0e-4;
    let tight = inferences::evaluate(&snap);
    let nc = find(&tight, ".n2.nc");
    assert_eq!(nc.status, CheckStatus::Fail);
    assert!(nc.remedies.iter().any(|r| r.target.path.ends_with(".chordRotationUltimateRad")));
    assert!(nc.utilization > find(&roomy, ".n2.nc").utilization);
}

#[semio_framework_async_macros::async_test]
async fn every_sei_id_is_traced_and_the_overview_standing_aggregates_the_building_checks() {
    use semio_s_plugin_norm_compliance::spec_trace::{assert_traced, standing, Emitted};
    let report = inferences::evaluate(&En1998Snapshot::compliant_de_multipart());
    let mut with_records = En1998Snapshot::compliant_de_multipart();
    with_records.accelerograms = record_rows(&with_records, 1.0, 3);
    with_records.diaphragms = vec![diaphragm_row(&with_records, 0.25)];
    let extra = inferences::evaluate(&with_records);
    let structures = inferences::evaluate(&En1998Snapshot::compliant_de_structures());
    let showcase = inferences::evaluate(&En1998Snapshot::full_showcase());
    let low = inferences::evaluate(&En1998Snapshot::low_seismicity());
    let emitted = Emitted::default().with_checks(report.checks.iter().chain(extra.checks.iter()).chain(structures.checks.iter()).chain(showcase.checks.iter()).chain(low.checks.iter()).map(|c| c.id.clone()));
    assert_traced(&inferences::spec_trace::TABLE, &emitted);
    let overview = standing(&inferences::spec_trace::TABLE, &report);
    assert!(overview.iter().any(|row| row.id == "SEI-54" && row.checks > 0));
    for suffix in [".importance", ".ductility", ".diaphragm"] {
        assert!(report.checks.iter().any(|c| c.id.ends_with(suffix)), "{suffix}");
    }
}

fn oracle_record(seed: i64) -> Vec<f64> {
    (0..200_i64).map(|k| (((k * (37 + 2 * seed) + 11 * (seed + 1)) % 101 - 50) as f64) / 25.0).collect()
}

fn diaphragm_row(snap: &En1998Snapshot, thickness_m: f64) -> semio_s_artifact_norm_en1998_model::En1998Diaphragm {
    let building = &snap.buildings[0];
    semio_s_artifact_norm_en1998_model::En1998Diaphragm { id: "dia-1".into(), building_id: building.id.clone(), storey_id: building.storeys[1].id.clone(), material: semio_s_artifact_norm_en1998_model::DiaphragmMaterial::Concrete, thickness_m, span_m: 0.0, depth_m: 0.0, opening_width_m: 2.4, shear_strength_mpa: 1.0, chord_area_m2: 0.01, chord_strength_mpa: 435.0, gamma_d: 0.0 }
}

fn record_rows(snap: &En1998Snapshot, scale: f64, count: usize) -> Vec<semio_s_artifact_norm_en1998_model::En1998Accelerogram> {
    (0..count as i64)
        .map(|seed| semio_s_artifact_norm_en1998_model::En1998Accelerogram { id: format!("rec-{seed}"), building_id: snap.buildings[0].id.clone(), label: format!("synthetic {seed}"), samples: oracle_record(seed).iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" "), dt_s: 0.02, scale_factor: scale })
        .collect()
}

#[semio_framework_async_macros::async_test]
async fn horizontal_kernels_diaphragm_overturning_accelerogram_and_surface_match_the_independent_oracle() {
    use horizontal::{diaphragm, overturning, spectrum_surface, time_history};
    let py = oracle_values();
    let close = |key: &str, rust: f64, relative: f64| {
        let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("oracle key {key} missing"));
        assert!((expected - rust).abs() <= 1e-12 + relative * expected.abs(), "{key}: oracle={expected} rust={rust}");
    };
    let (f_d, span, depth) = (diaphragm::design_force_n(500.0e3, 1.3), 24.0, 16.0);
    close("dia_f_d", f_d, 1e-12);
    close("dia_v_ed", diaphragm::support_shear_n(f_d), 1e-12);
    close("dia_v_rd", diaphragm::shear_resistance_n(1.0, 0.2, depth, 2.4), 1e-12);
    close("dia_t_req", diaphragm::required_thickness_m(diaphragm::support_shear_n(f_d), 1.0, depth, 2.4), 1e-12);
    close("dia_shear_flow", diaphragm::shear_flow_n_per_m(diaphragm::support_shear_n(f_d), depth, 2.4), 1e-12);
    close("dia_n_mid", diaphragm::chord_force_n(f_d, span, depth), 1e-9);
    close("dia_n_mid", diaphragm::chord_force_at_n(f_d, span, depth, span / 2.0), 1e-9);
    close("dia_n_quarter", diaphragm::chord_force_at_n(f_d, span, depth, span / 4.0), 1e-9);
    close("dia_n_rd", diaphragm::chord_resistance_n(0.01, 435.0), 1e-12);
    close("dia_a_req", diaphragm::required_chord_area_m2(diaphragm::chord_force_n(f_d, span, depth), 435.0), 1e-9);
    assert!((diaphragm::shear_at_n(f_d, span, 0.0) - f_d / 2.0).abs() < 1e-9 && diaphragm::shear_at_n(f_d, span, span / 2.0).abs() < 1e-9);

    let (forces, elevations) = ([100.0e3, 200.0e3, 300.0e3], [3.5, 7.0, 10.5]);
    close("ov_moment", overturning::moment_nm(&forces, &elevations), 1e-12);
    close("ov_stabilising", overturning::stabilising_moment_nm(1.0e6, 4.0, 1.0e5, 1.0), 1e-12);
    close("ov_base_req", overturning::required_base_width_m(4.9e6, 1.0e6, 1.0e5, 1.0), 1e-9);
    close("ov_tie_req", overturning::required_tie_down_moment_nm(4.9e6, 1.0e6, 4.0, 1.0), 1e-9);

    let records: Vec<Vec<f64>> = (0..3).map(oracle_record).collect();
    close("th_pga_0", time_history::peak_acceleration(&records[0]), 1e-12);
    close("th_sa_0_t0p1", time_history::pseudo_acceleration(&records[0], 0.02, 0.1, 0.05), 1e-9);
    close("th_sa_0_t0p5", time_history::pseudo_acceleration(&records[0], 0.02, 0.5, 0.05), 1e-9);
    close("th_sa_1_t1p0", time_history::pseudo_acceleration(&records[1], 0.02, 1.0, 0.05), 1e-9);
    close("th_sa_0_t0p5_xi2", time_history::pseudo_acceleration(&records[0], 0.02, 0.5, 0.02), 1e-9);
    let (a_g, s, tb, tc, td, t1) = (1.5, 1.2, 0.15, 0.5, 2.0, 0.6);
    let periods = time_history::periods_s(t1, time_history::PERIOD_SAMPLES);
    let elastic: Vec<f64> = periods.iter().map(|t| part_1::elastic_response_spectrum(a_g, s, tb, tc, td, *t)).collect();
    let set: Vec<(f64, Vec<f64>)> = records.iter().zip([1.0, 1.2, 0.8]).map(|(r, k)| (time_history::peak_acceleration(r) * k, time_history::record_spectrum(r, k, 0.02, &periods))).collect();
    let judged = time_history::judge_set(&set, &periods, &elastic, a_g * s, 1.0, 0.9);
    close("th_mean_zero", judged.mean_zero_period, 1e-12);
    close("th_lowest_ratio", judged.lowest_mean_ratio, 1e-9);
    close("th_lowest_period", judged.lowest_period_s, 1e-12);
    close("th_scale_to_accept", judged.scale_to_accept, 1e-9);

    let surface = spectrum_surface::grid(0.6, 1.2, 0.15, 0.5, 2.0, &[0.3, 1.0, 0.1, 3.0], &[2.0, 5.0, 30.0]);
    close("surface_xi2_t0p3", surface[0][0], 1e-12);
    close("surface_xi5_t1", surface[1][1], 1e-12);
    close("surface_xi30_t0p1", surface[2][2], 1e-12);
    close("surface_xi5_t3", surface[1][3], 1e-12);
    close("surface_eta_xi30", part_1::damping_correction(30.0), 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn diaphragm_shear_and_chord_fail_with_thickness_strength_opening_and_chord_remedies_that_fix_them() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    snap.diaphragms = vec![diaphragm_row(&snap, 0.25)];
    let roomy = inferences::evaluate(&snap);
    for suffix in [".diaphragm.dia-1.shear", ".diaphragm.dia-1.chord"] {
        assert_eq!(find(&roomy, suffix).status, CheckStatus::Pass, "{suffix}");
    }
    assert!(!roomy.checks.iter().any(|c| c.id.ends_with(".diaphragm")), "the informative amplified force gives way to the modelled diaphragm");
    snap.diaphragms[0] = semio_s_artifact_norm_en1998_model::En1998Diaphragm { thickness_m: 0.001, chord_area_m2: 1.0e-6, ..snap.diaphragms[0].clone() };
    let weak = inferences::evaluate(&snap);
    let shear = find(&weak, ".diaphragm.dia-1.shear");
    assert_eq!(shear.status, CheckStatus::Fail);
    let paths: Vec<&str> = shear.remedies.iter().map(|r| r.target.path.as_str()).collect();
    for field in ["thicknessM", "shearStrengthMpa", "openingWidthM"] {
        assert!(paths.iter().any(|p| p.ends_with(field)), "{field} in {paths:?}");
    }
    let chord = find(&weak, ".diaphragm.dia-1.chord");
    assert_eq!(chord.status, CheckStatus::Fail);
    assert!(chord.remedies.iter().any(|r| r.target.path.ends_with("chordAreaM2")) && chord.remedies.iter().any(|r| r.target.path.ends_with("chordStrengthMpa")));
    let thickness = shear.remedies.iter().find(|r| r.target.path.ends_with("thicknessM")).map(|r| r.required.value).expect("thickness remedy");
    let area = chord.remedies.iter().find(|r| r.target.path.ends_with("chordAreaM2")).map(|r| r.required.value).expect("area remedy");
    snap.diaphragms[0].thickness_m = thickness * 1.001;
    snap.diaphragms[0].chord_area_m2 = area * 1.001;
    let fixed = inferences::evaluate(&snap);
    assert_eq!(find(&fixed, ".diaphragm.dia-1.shear").status, CheckStatus::Pass);
    assert_eq!(find(&fixed, ".diaphragm.dia-1.chord").status, CheckStatus::Pass);
    snap.diaphragms[0].storey_id = "no-such-storey".into();
    let dangling = inferences::evaluate(&snap);
    assert_eq!(find(&dangling, "diaphragms.dia-1.storeyId").status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn overturning_fails_on_a_narrow_base_and_the_base_width_and_tie_down_remedies_fix_it() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    let roomy = inferences::evaluate(&snap);
    assert!(roomy.checks.iter().any(|c| c.id.ends_with(".overturning") && c.status == CheckStatus::Pass));
    (snap.buildings[0].overturning_base_x_m, snap.buildings[0].overturning_base_y_m) = (0.1, 0.1);
    let narrow = inferences::evaluate(&snap);
    let failing: Vec<_> = narrow.checks.iter().filter(|c| c.id.ends_with(".overturning") && c.status == CheckStatus::Fail).collect();
    assert!(!failing.is_empty());
    for check in &failing {
        assert!(check.remedies.iter().any(|r| r.target.path.ends_with("tieDownMomentNm")));
        let base = check.remedies.iter().find(|r| r.target.path.contains("overturningBase")).expect("base remedy");
        if base.target.path.ends_with("overturningBaseYM") { snap.buildings[0].overturning_base_y_m = base.required.value * 1.001 } else { snap.buildings[0].overturning_base_x_m = base.required.value * 1.001 }
    }
    let widened = inferences::evaluate(&snap);
    assert!(widened.checks.iter().filter(|c| c.id.ends_with(".overturning")).all(|c| c.status == CheckStatus::Pass));
    (snap.buildings[0].overturning_base_x_m, snap.buildings[0].overturning_base_y_m) = (0.1, 0.1);
    let tie = failing.iter().flat_map(|c| c.remedies.iter()).filter(|r| r.target.path.ends_with("tieDownMomentNm")).map(|r| r.required.value).fold(0.0_f64, f64::max);
    snap.buildings[0].tie_down_moment_nm = tie * 1.001;
    let anchored = inferences::evaluate(&snap);
    assert!(anchored.checks.iter().filter(|c| c.id.ends_with(".overturning")).all(|c| c.status == CheckStatus::Pass));
}

#[semio_framework_async_macros::async_test]
async fn accelerogram_set_is_judged_with_scale_factor_remedies_and_unusable_records_fail() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    snap.accelerograms = record_rows(&snap, 0.01, 3);
    let weak = inferences::evaluate(&snap);
    assert_eq!(find(&weak, ".timeHistory.records").status, CheckStatus::Pass);
    let zero = find(&weak, ".timeHistory.zeroPeriod");
    let spectrum = find(&weak, ".timeHistory.spectrum");
    assert_eq!((zero.status, spectrum.status), (CheckStatus::Fail, CheckStatus::Fail));
    assert_eq!(spectrum.remedies.len(), 3);
    for (row, remedy) in snap.accelerograms.iter_mut().zip(&spectrum.remedies) {
        assert!(remedy.target.path.ends_with("scaleFactor") && remedy.required.value > row.scale_factor);
        row.scale_factor = remedy.required.value * 1.001;
    }
    let scaled = inferences::evaluate(&snap);
    assert_eq!(find(&scaled, ".timeHistory.spectrum").status, CheckStatus::Pass);
    assert_eq!(find(&scaled, ".timeHistory.zeroPeriod").status, CheckStatus::Pass);
    let two = record_rows(&snap, 5.0, 2);
    snap.accelerograms = two;
    let short = inferences::evaluate(&snap);
    let records = find(&short, ".timeHistory.records");
    assert_eq!(records.status, CheckStatus::Fail);
    assert_eq!(records.remedies.len(), 1);
    snap.accelerograms = record_rows(&snap, 5.0, 3);
    snap.accelerograms[1].samples = "0.1 x 0.3".into();
    let broken = inferences::evaluate(&snap);
    assert_eq!(find(&broken, ".timeHistory.record.rec-1").status, CheckStatus::Fail);
    assert_eq!(find(&broken, ".timeHistory.records").status, CheckStatus::Fail, "only two records are usable");
    let trace = semio_s_plugin_norm_compliance::spec_trace::standing(&inferences::spec_trace::TABLE, &broken);
    assert!(trace.iter().any(|r| r.id == "SEI-06" && r.checks >= 3 && r.failing >= 1));
    snap.accelerograms[0].building_id = "no-such-building".into();
    assert_eq!(find(&inferences::evaluate(&snap), "accelerograms.rec-0.buildingId").status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn horizontal_constants_are_registered_unverified_and_cited_by_their_checks() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    snap.diaphragms = vec![diaphragm_row(&snap, 0.25)];
    snap.accelerograms = record_rows(&snap, 1.0, 3);
    let report = inferences::evaluate(&snap);
    for (suffix, id) in [(".overturning", "ndp.en1998.en1.overturning.stabilising-factor"), (".diaphragm.dia-1.shear", "ndp.en1998.en1.diaphragm.transfer-factor"), (".timeHistory.zeroPeriod", "ndp.en1998.en1.time-history.zero-period-ratio"), (".timeHistory.spectrum", "ndp.en1998.en1.time-history.mean-spectrum-ratio")] {
        assert!(find(&report, suffix).ndps.iter().any(|n| n == id), "{suffix} cites {id}");
        let entry = semio_s_plugin_norm_compliance::ndp(id).expect("registered");
        assert_eq!(entry.status, semio_s_plugin_norm_compliance::NdpStatus::Unverified, "{id} stays unverified until read from the printed standard");
    }
}

const SHOWCASE_UNREACHED: &[(&str, &str)] = &[(".zone0", "the exemption of very low seismicity needs a_gR = 0, which switches off every other building check of the same document")];

#[semio_framework_async_macros::async_test]
async fn the_low_seismicity_example_reaches_the_zone_exemption() {
    let report = inferences::evaluate(&En1998Snapshot::low_seismicity());
    assert!(report.checks.iter().any(|c| c.id.ends_with(".zone0") && c.status == CheckStatus::NotApplicable));
    assert!(report.complies());
}

#[semio_framework_async_macros::async_test]
async fn the_full_showcase_alone_reaches_every_check_fragment_of_the_trace_but_the_zone_exemption() {
    use semio_s_plugin_norm_compliance::spec_trace::{assert_reaches_all_but, Emitted};
    let showcase = En1998Snapshot::full_showcase();
    for (collection, rows) in [("buildings", showcase.buildings.len()), ("bridges", showcase.bridges.len()), ("assessments", showcase.assessments.len()), ("silos", showcase.silos.len()), ("tanks", showcase.tanks.len()), ("foundations", showcase.foundations.len()), ("retainingWalls", showcase.retaining_walls.len()), ("towers", showcase.towers.len()), ("soilLayers", showcase.soil_layers.len()), ("diaphragms", showcase.diaphragms.len()), ("accelerograms", showcase.accelerograms.len()), ("piers", showcase.piers.len()), ("bearings", showcase.bearings.len()), ("joints", showcase.joints.len()), ("placements", showcase.placements.len())] {
        assert!(rows > 0, "the showcase leaves {collection} empty");
    }
    let emitted = Emitted::default().with_checks(inferences::evaluate(&showcase).checks.into_iter().map(|c| c.id));
    assert_reaches_all_but(&inferences::spec_trace::TABLE, &emitted, SHOWCASE_UNREACHED);
}

fn wall_rows(snap: &En1998Snapshot, material: &str) -> Vec<semio_s_artifact_norm_en1998_model::En1998Member> {
    [("w-a", 1.0, 2.0, 6.0, 0.0), ("w-b", 9.0, 1.0, 5.0, std::f64::consts::FRAC_PI_2), ("w-c", 2.0, 10.0, 4.0, 0.0), ("w-d", 3.0, 3.0, 4.0, 0.5)]
        .into_iter()
        .map(|(id, x, y, length, direction)| {
            let mut m = snap.buildings[0].members[0].clone();
            (m.id, m.material, m.role) = (id.into(), material.into(), semio_s_artifact_norm_en1998_model::MemberKind::Wall);
            (m.wall_start_x_m, m.wall_start_y_m, m.wall_length_m, m.wall_thickness_m, m.wall_direction_rad, m.wall_storey_id) = (x, y, length, 0.3, direction, String::new());
            m
        })
        .collect()
}

#[semio_framework_async_macros::async_test]
async fn positioned_walls_give_the_stiffness_centre_and_radii_of_the_python_oracle_and_feed_the_plan_regularity() {
    let script = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).arg("--formulas").output().expect("oracle formulas");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    let close = |key: &str, rust: f64| {
        let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("oracle key {key} missing"));
        assert!((expected - rust).abs() <= 1e-9 + 1e-6 * expected.abs(), "{key}: oracle={expected} rust={rust}");
    };
    let mut snap = En1998Snapshot::compliant_de_office();
    snap.buildings[0].storeys[0].height_m = 3.2;
    let rows = wall_rows(&snap, "rc");
    let walls = wall_plan::placed_walls_of(&rows);
    close("wall_plan_k_l6", wall_plan::stiffness_per_modulus_m(6.0, 0.3, 3.2, wall_plan::shear_to_young_ratio(0.25)));
    close("wall_plan_k_l6_nu02", wall_plan::stiffness_per_modulus_m(6.0, 0.3, 3.2, wall_plan::shear_to_young_ratio(0.2)));
    let plan = wall_plan::wall_plan_of_storey(&walls, &snap.buildings[0].storeys[0]).expect("wall plan");
    close("wall_plan_cs_x", plan.centre_of_stiffness_x_m);
    close("wall_plan_cs_y", plan.centre_of_stiffness_y_m);
    close("wall_plan_k_x", plan.stiffness_x);
    close("wall_plan_k_y", plan.stiffness_y);
    close("wall_plan_r_x", plan.radius_x_m);
    close("wall_plan_r_y", plan.radius_y_m);
    assert!(wall_plan::wall_plan_of_storey(&walls[..1], &snap.buildings[0].storeys[0]).is_none(), "parallel walls give no centre");
    let mut mixed = rows.clone();
    mixed[0].material = semio_s_artifact_norm_en1998_model::StructuralMaterial::Masonry;
    assert!(wall_plan::wall_plan_of_storey(&wall_plan::placed_walls_of(&mixed), &snap.buildings[0].storeys[0]).is_none(), "the Young moduli of mixed materials are unknown");
    let mut other_storey = rows.clone();
    other_storey.iter_mut().for_each(|m| m.wall_storey_id = "elsewhere".into());
    assert!(wall_plan::wall_plan_of_storey(&wall_plan::placed_walls_of(&other_storey), &snap.buildings[0].storeys[0]).is_none());

    let mut positioned = snap.clone();
    let building = &mut positioned.buildings[0];
    building.plan_width_m = 24.0;
    building.plan_length_m = 16.0;
    building.members.extend(rows);
    for storey in &mut building.storeys {
        (storey.height_m, storey.centre_of_mass_x_m, storey.centre_of_mass_y_m) = (3.2, plan.centre_of_stiffness_x_m + 1.5, plan.centre_of_stiffness_y_m + 1.5);
        (storey.centre_of_stiffness_x_m, storey.centre_of_stiffness_y_m) = (storey.centre_of_mass_x_m, storey.centre_of_mass_y_m);
    }
    let report = inferences::evaluate(&positioned);
    let regularity = report.checks.iter().find(|c| c.id.ends_with(&format!("storey.{}.planRegularity", positioned.buildings[0].storeys[0].id))).expect("plan regularity");
    assert!(regularity.explanation.en.contains("positioned wall rows") && regularity.explanation.en.contains("e0x=1.500") && regularity.explanation.en.contains("e0y=1.500"), "{}", regularity.explanation.en);
    assert!(regularity.explanation.de.contains("angeordneten Wandzeilen"), "{}", regularity.explanation.de);
    let torsion = report.checks.iter().find(|c| c.id.ends_with(".torsion")).expect("torsion");
    assert!(torsion.explanation.en.contains("e0(max over storeys)=1.500"), "the entered centre of stiffness is replaced by the walls: {}", torsion.explanation.en);
}

#[semio_framework_async_macros::async_test]
async fn joint_rows_follow_the_capacity_design_rule_of_the_numpy_oracle_with_remedies_for_the_failing_joint() {
    let script = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).arg("--formulas").output().expect("oracle formulas");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    let close = |key: &str, rust: f64| {
        let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("oracle key {key} missing"));
        assert!((expected - rust).abs() <= 1e-9 + 1e-9 * expected.abs(), "{key}: oracle={expected} rust={rust}");
    };
    let mut snap = En1998Snapshot::compliant_de_office();
    let building = snap.buildings[0].id.clone();
    let storey = snap.buildings[0].storeys[0].id.clone();
    snap.joints = [(0, 9.0e6, 4.0e6), (1, 6.0e6, 5.0e6), (2, 7.5e6, 2.5e6)]
        .into_iter()
        .map(|(i, rc, rb)| semio_s_artifact_norm_en1998_model::En1998Joint { id: format!("j{i}"), building_id: building.clone(), storey_id: storey.clone(), column_id: "col-c1".into(), beam_id: "beam-b1".into(), x_m: 4.0 + 4.0 * i as f64, y_m: 4.0, column_moment_rd_nm: rc, beam_moment_rd_nm: rb })
        .collect();
    let report = inferences::evaluate(&snap);
    for (i, joint) in snap.joints.iter().enumerate() {
        close(&format!("joint_sum_rc_{i}"), joint.column_moment_rd_nm);
        close(&format!("joint_sum_rb_{i}"), joint.beam_moment_rd_nm);
        close(&format!("joint_ratio_{i}"), joints::ratio(joint));
        close(&format!("joint_demand_{i}"), joints::demand_nm(joint));
        let check = report.checks.iter().find(|c| c.id == format!("en1998.1.{building}.capacityDesign.joint.{}", joint.id)).unwrap_or_else(|| panic!("joint check {i}"));
        close(&format!("joint_utilisation_{i}"), check.utilization);
        assert_eq!(check.status, if joints::is_satisfied(joint) { CheckStatus::Pass } else { CheckStatus::Fail });
    }
    let failing = report.checks.iter().find(|c| c.id.ends_with("capacityDesign.joint.j1")).expect("failing joint");
    assert_eq!(failing.status, CheckStatus::Fail);
    let column = failing.remedies.iter().find(|r| r.target.path == "joints[id=j1].columnMomentRdNm").expect("column remedy");
    close("joint_demand_1", column.required.value);
    let beam = failing.remedies.iter().find(|r| r.target.path == "joints[id=j1].beamMomentRdNm").expect("beam remedy");
    close("joint_max_beam_1", beam.required.value);
    assert!(joints::ratio(&semio_s_artifact_norm_en1998_model::En1998Joint { id: "j".into(), building_id: building.clone(), storey_id: storey.clone(), column_id: String::new(), beam_id: String::new(), x_m: 0.0, y_m: 0.0, column_moment_rd_nm: 1.0, beam_moment_rd_nm: 0.0 }).is_infinite());
    snap.joints[0].storey_id = "nowhere".into();
    let dangling = inferences::evaluate(&snap);
    assert!(dangling.checks.iter().any(|c| c.id == "en1998.integrity.reference.joints.j0.storeyId" && c.status == CheckStatus::Fail));
}

#[semio_framework_async_macros::async_test]
async fn cracked_stiffness_lengthens_the_period_by_the_inverse_root_of_the_factor_and_moves_the_stiffness_centre_like_the_scipy_oracle() {
    let script = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).arg("--formulas").output().expect("oracle formulas");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    let close = |key: &str, rust: f64| {
        let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("oracle key {key} missing"));
        assert!((expected - rust).abs() <= 1e-9 + 1e-6 * expected.abs(), "{key}: oracle={expected} rust={rust}");
    };
    close_value(cracked_stiffness::simplified_factor(), 0.5, "the registry factor of the simplified method");
    let mut snap = En1998Snapshot::compliant_de_office();
    let template = snap.buildings[0].storeys[0].clone();
    let mut building = snap.buildings[0].clone();
    building.storeys = [80.0e6, 70.0e6, 60.0e6].iter().enumerate().map(|(i, k)| semio_s_artifact_norm_en1998_model::En1998Storey { id: format!("s{}", i + 1), stiffness_x: *k, stiffness_y: *k, ..template.clone() }).collect();
    let masses = [200.0e3, 200.0e3, 150.0e3];
    let fundamental = |building: &semio_s_artifact_norm_en1998_model::En1998Building| {
        let (x, _) = cracked_stiffness::storeys_xy(building);
        part_1::shear_building_modes(&masses, &x).0.into_iter().fold(0.0_f64, f64::max)
    };
    (building.storey_stiffness_basis, building.storey_stiffness_ratio) = (semio_s_artifact_norm_en1998_model::StiffnessBasis::Declared, 1.0);
    let gross = fundamental(&building);
    building.storey_stiffness_basis = semio_s_artifact_norm_en1998_model::StiffnessBasis::Simplified;
    let simplified = fundamental(&building);
    (building.storey_stiffness_basis, building.storey_stiffness_ratio) = (semio_s_artifact_norm_en1998_model::StiffnessBasis::Declared, 0.8);
    let declared = fundamental(&building);
    close("cracked_t1_gross", gross);
    close("cracked_t1_half", simplified);
    close("cracked_t1_ratio", simplified / gross);
    close_value(simplified / gross, 1.0 / 0.5_f64.sqrt(), "pure frame: T scales with 1/sqrt(factor)");
    close_value(declared / gross, 1.0 / 0.8_f64.sqrt(), "declared effective stiffness");
    let rayleigh = |basis: semio_s_artifact_norm_en1998_model::StiffnessBasis, ratio: f64| {
        let mut b = building.clone();
        (b.storey_stiffness_basis, b.storey_stiffness_ratio) = (basis, ratio);
        let (x, _) = cracked_stiffness::storeys_xy(&b);
        part_1::rayleigh_period_shear_building(&masses, &x).expect("rayleigh")
    };
    close_value(rayleigh(semio_s_artifact_norm_en1998_model::StiffnessBasis::Simplified, 1.0) / rayleigh(semio_s_artifact_norm_en1998_model::StiffnessBasis::Declared, 1.0), 1.0 / 0.5_f64.sqrt(), "the Rayleigh period follows the same scaling");

    snap.buildings[0].storeys[0].height_m = 3.2;
    let mut rows = wall_rows(&snap, "rc");
    let storey = &snap.buildings[0].storeys[0];
    let uniform = wall_plan::wall_plan_of_storey(&wall_plan::placed_walls_of(&rows), storey).expect("gross plan");
    rows.iter_mut().for_each(|m| m.effective_stiffness_basis = semio_s_artifact_norm_en1998_model::StiffnessBasis::Simplified);
    let halved = wall_plan::wall_plan_of_storey(&wall_plan::placed_walls_of(&rows), storey).expect("uniform cracked plan");
    close_value(halved.stiffness_x, 0.5 * uniform.stiffness_x, "uniform factor scales the wall stiffness");
    close_value(halved.centre_of_stiffness_x_m, uniform.centre_of_stiffness_x_m, "uniform factor leaves the centre");
    close_value(halved.radius_x_m, uniform.radius_x_m, "uniform factor leaves the radii");
    for (row, basis) in rows.iter_mut().zip([semio_s_artifact_norm_en1998_model::StiffnessBasis::Simplified, semio_s_artifact_norm_en1998_model::StiffnessBasis::Declared, semio_s_artifact_norm_en1998_model::StiffnessBasis::Declared, semio_s_artifact_norm_en1998_model::StiffnessBasis::Simplified]) {
        (row.effective_stiffness_basis, row.effective_stiffness_ratio) = (basis, 1.0);
    }
    let mixed = wall_plan::wall_plan_of_storey(&wall_plan::placed_walls_of(&rows), storey).expect("mixed plan");
    close("cracked_wall_cs_x", mixed.centre_of_stiffness_x_m);
    close("cracked_wall_cs_y", mixed.centre_of_stiffness_y_m);
    close("cracked_wall_k_x", mixed.stiffness_x);
    close("cracked_wall_k_y", mixed.stiffness_y);
    close("cracked_wall_r_x", mixed.radius_x_m);
    close("cracked_wall_r_y", mixed.radius_y_m);
    assert!((mixed.centre_of_stiffness_x_m - uniform.centre_of_stiffness_x_m).abs() > 1e-3, "walls with different factors move the stiffness centre");
    assert_eq!(mixed.stiffness_factor_text(), "50–100 %");
}

#[test]
fn a_declared_stiffness_share_outside_zero_to_one_fails_while_the_simplified_basis_ignores_the_share() {
    let mut snap = En1998Snapshot::compliant_de_office();
    let id = snap.buildings[0].id.clone();
    let status = |snap: &En1998Snapshot| inferences::evaluate(snap).checks.iter().find(|c| c.id == format!("en1998.1.{id}.t1Given")).map(|c| c.status).expect("t1Given check");
    (snap.buildings[0].storey_stiffness_basis, snap.buildings[0].storey_stiffness_ratio) = (semio_s_artifact_norm_en1998_model::StiffnessBasis::Declared, 1.5);
    assert_eq!(status(&snap), CheckStatus::Fail, "a declared share above 1 is invalid");
    snap.buildings[0].storey_stiffness_basis = semio_s_artifact_norm_en1998_model::StiffnessBasis::Simplified;
    assert_ne!(status(&snap), CheckStatus::Fail, "the share is ignored under the simplified basis");
    close_value(cracked_stiffness::building_factor(&snap.buildings[0]), cracked_stiffness::simplified_factor(), "simplified basis uses the registry factor");
}

fn close_value(actual: f64, expected: f64, what: &str) {
    assert!((actual - expected).abs() <= 1e-9 + 1e-9 * expected.abs(), "{what}: expected {expected}, got {actual}");
}

#[semio_framework_async_macros::async_test]
async fn joints_of_the_top_storey_of_a_multi_storey_building_are_exempt_from_the_capacity_design_rule() {
    let mut snap = En1998Snapshot::compliant_de_office();
    let building = snap.buildings[0].id.clone();
    let storeys: Vec<String> = snap.buildings[0].storeys.iter().map(|s| s.id.clone()).collect();
    assert!(storeys.len() > 1, "the office has several storeys");
    let joint = |id: &str, storey: &str| semio_s_artifact_norm_en1998_model::En1998Joint { id: id.into(), building_id: building.clone(), storey_id: storey.into(), column_id: "col-c1".into(), beam_id: "beam-b1".into(), x_m: 4.0, y_m: 4.0, column_moment_rd_nm: 5.0e6, beam_moment_rd_nm: 6.0e6 };
    snap.joints = vec![joint("lower", &storeys[0]), joint("top", storeys.last().expect("top storey"))];
    assert!(!joints::is_top_storey_exempt(&snap.buildings[0], &snap.joints[0]));
    assert!(joints::is_top_storey_exempt(&snap.buildings[0], &snap.joints[1]));
    let report = inferences::evaluate(&snap);
    let find = |id: &str| report.checks.iter().find(|c| c.id == format!("en1998.1.{building}.capacityDesign.joint.{id}")).unwrap_or_else(|| panic!("joint check {id}"));
    assert_eq!(find("lower").status, CheckStatus::Fail, "ΣM_Rc below 1.3 ΣM_Rb fails below the top storey");
    assert_eq!(find("top").status, CheckStatus::NotApplicable, "the same resistances at the top storey are exempt");
    let mut single = snap.clone();
    single.buildings[0].storeys.truncate(1);
    single.joints = vec![joint("only", &storeys[0])];
    assert!(!joints::is_top_storey_exempt(&single.buildings[0], &single.joints[0]), "a single-storey building has no exemption");
}

fn sectioned_assessment() -> En1998Snapshot {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    let a = &mut snap.assessments[0];
    (a.section_depth_m, a.section_width_m, a.effective_depth_m, a.shear_span_m, a.axial_force_n) = (0.5, 0.3, 0.45, 2.0, 100.0e3);
    (a.tension_steel_area_m2, a.compression_steel_area_m2, a.bar_diameter_mm, a.steel_yield_mpa, a.concrete_strength_mpa) = (1.8e-3, 0.9e-3, 20.0, 500.0, 30.0);
    (a.stirrup_leg_area_m2, a.stirrup_spacing_m, a.stirrup_yield_mpa, a.core_width_m, a.core_depth_m, a.restrained_bar_count, a.restrained_bar_spacing_m) = (1.57e-4, 0.2, 500.0, 0.24, 0.44, 4.0, 0.12);
    a.chord_rotation_yield_rad = 0.0;
    a.chord_rotation_ultimate_rad = 0.0;
    a.chord_rotation_demand_rad = 0.01;
    snap
}

#[semio_framework_async_macros::async_test]
async fn the_cyclic_shear_check_verifies_the_brittle_mechanism_with_a_stirrup_remedy_that_fixes_it() {
    let mut snap = sectioned_assessment();
    assert!(!inferences::evaluate(&snap).checks.iter().any(|c| c.id.ends_with(".cyclicShear")), "no check without a shear demand");
    let capacity = part_3_state::assessment_shear_capacity(&snap.assessments[0], AnnexChoice::De).expect("capacity").resistance_n;
    assert!(capacity > 0.0);
    snap.assessments[0].shear_force_demand_n = 0.5 * capacity;
    let ok = inferences::evaluate(&snap);
    let check = find(&ok, ".cyclicShear");
    assert_eq!(check.status, CheckStatus::Pass);
    assert!((check.utilization - 0.5).abs() < 1e-9, "{}", check.utilization);
    snap.assessments[0].shear_force_demand_n = 1.5 * capacity;
    let bad = inferences::evaluate(&snap);
    let check = find(&bad, ".cyclicShear");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!((check.utilization - 1.5).abs() < 1e-9, "{}", check.utilization);
    let remedy = check.remedies.first().expect("a remedy");
    assert!(remedy.target.path.ends_with(".stirrupSpacingM") || remedy.target.path.ends_with(".shearForceDemandN"), "{}", remedy.target.path);
    if remedy.target.path.ends_with(".stirrupSpacingM") {
        snap.assessments[0].stirrup_spacing_m = remedy.required.value * 0.999;
        let fixed = part_3_state::assessment_shear_capacity(&snap.assessments[0], AnnexChoice::De).expect("capacity").resistance_n;
        assert!(fixed >= snap.assessments[0].shear_force_demand_n, "{fixed}");
    }
}

#[semio_framework_async_macros::async_test]
async fn brittle_strengths_divide_by_cf_and_the_partial_factor_so_a_lower_knowledge_level_lowers_the_shear_capacity() {
    let mut snap = sectioned_assessment();
    snap.assessments[0].limit_state = semio_s_artifact_norm_en1998_model::LimitState::Nc;
    snap.assessments[0].knowledge_level = semio_s_artifact_norm_en1998_model::KnowledgeLevel::Kl3;
    let kl3 = part_3_state::assessment_shear_capacity(&snap.assessments[0], AnnexChoice::De).expect("kl3").resistance_n;
    snap.assessments[0].knowledge_level = semio_s_artifact_norm_en1998_model::KnowledgeLevel::Kl1;
    let kl1 = part_3_state::assessment_shear_capacity(&snap.assessments[0], AnnexChoice::De).expect("kl1").resistance_n;
    assert!(kl1 < kl3, "{kl1} {kl3}");
}

#[semio_framework_async_macros::async_test]
async fn the_pushover_at_knowledge_level_one_warns_with_a_remedy_that_raises_the_level() {
    let mut snap = sectioned_assessment();
    let building = snap.assessments[0].supported_building_id.clone();
    snap.buildings.iter_mut().filter(|b| b.id == building).for_each(|b| b.storeys.iter_mut().for_each(|s| s.shear_resistance_n = 1.0e7));
    snap.assessments[0].knowledge_level = semio_s_artifact_norm_en1998_model::KnowledgeLevel::Kl2;
    assert_eq!(find(&inferences::evaluate(&snap), ".analysisMethod").status, CheckStatus::Pass);
    snap.assessments[0].knowledge_level = semio_s_artifact_norm_en1998_model::KnowledgeLevel::Kl1;
    let report = inferences::evaluate(&snap);
    let check = find(&report, ".analysisMethod");
    assert_eq!(check.status, CheckStatus::Warning);
    assert!(check.remedies.iter().any(|r| r.target.path.ends_with(".knowledgeLevel")));
}

#[semio_framework_async_macros::async_test]
async fn the_assessment_rows_of_the_registry_exist_are_unverified_and_hold_the_spec_values() {
    for (id, value) in [("cf.kl1", 1.35), ("cf.kl2", 1.20), ("cf.kl3", 1.00), ("return-period.nc", 2475.0), ("return-period.sd", 475.0), ("return-period.dl", 225.0), ("return-period.exponent", 3.0), ("shear.axial-limit", 0.55), ("shear.ductility-cap", 5.0), ("shear.ductility-slope", 0.05), ("shear.base", 0.16), ("shear.rho-min", 0.5), ("shear.slenderness-slope", 0.16), ("shear.slenderness-cap", 5.0)] {
        let id = format!("ndp.en1998.en3.{id}");
        let entry = semio_s_plugin_norm_compliance::ndp(&id).unwrap_or_else(|| panic!("registry row {id} missing"));
        assert_eq!(entry.status, semio_s_plugin_norm_compliance::NdpStatus::Unverified, "{id}");
        assert_eq!(semio_s_plugin_norm_compliance::ndp_common(&id), value, "{id}");
    }
    assert_eq!(semio_s_plugin_norm_compliance::ndp_number("ndp.en1998.en3.na.return-period", AnnexChoice::De), 475.0);
    assert!(semio_s_plugin_norm_compliance::ndp_number_opt("ndp.en1998.en3.na.return-period", AnnexChoice::En).is_none());
}

#[semio_framework_async_macros::async_test]
async fn the_limit_state_check_reports_the_return_period_and_under_the_german_annex_the_na_period() {
    let report = inferences::evaluate(&sectioned_assessment());
    let explanation = &find(&report, ".limitState").explanation;
    assert!(explanation.en.contains("T_R = ") && explanation.en.contains("NA-DE fixes T_R = 475"), "{}", explanation.en);
    assert!(explanation.de.contains("NA-DE legt T_R = 475"), "{}", explanation.de);
}

#[semio_framework_async_macros::async_test]
async fn under_the_german_annex_the_capacity_demand_does_not_depend_on_the_limit_state() {
    let utilisation = |annex: semio_s_plugin_norm_compliance::AnnexChoice, state: semio_s_artifact_norm_en1998_model::LimitState| {
        let mut snap = sectioned_assessment();
        snap.annex = annex;
        snap.assessments[0].limit_state = state;
        snap.assessments[0].r_k_n = 1.0e6;
        find(&inferences::evaluate(&snap), ".capacity").utilization
    };
    use semio_s_artifact_norm_en1998_model::LimitState::{Nc, Sd};
    let (de_nc, de_sd) = (utilisation(AnnexChoice::De, Nc), utilisation(AnnexChoice::De, Sd));
    assert!((de_nc - de_sd).abs() <= 1e-9 * de_sd.max(1.0), "{de_nc} {de_sd}");
    assert!(de_sd > 0.0, "the demand is evaluated");
}

fn tie_slope_document(name: &str) -> En1998Snapshot {
    let path = family_any_dir().join(format!("🧪️tests/🧫️fixtures/🚦️check-outcome/{name}/🔣️.json"));
    serde_json::from_slice(&std::fs::read(path).expect("committed document")).expect("snapshot")
}

#[semio_framework_async_macros::async_test]
async fn the_tie_beam_force_is_the_ground_type_coefficient_times_alpha_s_and_the_larger_supported_weight() {
    let report = inferences::evaluate(&tie_slope_document("seismic-tie-slope-pass"));
    let tie = find(&report, "tb-1.tie");
    let doc = tie_slope_document("seismic-tie-slope-pass");
    let weight = |building: &str| {
        let b = doc.buildings.iter().find(|b| b.id == building).expect("building");
        b.storeys.iter().enumerate().map(|(i, s)| part_1::storey_seismic_action_n(s, i + 1 == b.storeys.len())).sum::<f64>()
    };
    let (a_g, s) = (1.2 * 1.5 / 2.5, 1.25);
    let expected = 0.3 * (a_g / 9.81) * s * weight("bldg-office").max(weight("bldg-annex"));
    assert!((tie.computed.value - expected).abs() <= 1e-6 * expected, "N_tie={} expected={expected}", tie.computed.value);
    assert_eq!(tie.status, CheckStatus::UnverifiedBasis);
    assert!((tie.utilization - 0.543353266).abs() < 1e-6, "{}", tie.utilization);
    assert!((find(&report, "tb-1.displacement").utilization - 1.0 / 3.0).abs() < 1e-12);
    let failing = inferences::evaluate(&tie_slope_document("seismic-tie-slope-fail"));
    assert_eq!(find(&failing, "tb-1.tie").status, CheckStatus::Fail);
    assert_eq!(find(&failing, "tb-1.displacement").status, CheckStatus::Fail);
    assert!(find(&failing, "tb-1.tie").remedies.iter().any(|r| r.target.path.ends_with(".steelArea")));
}

#[semio_framework_async_macros::async_test]
async fn ground_types_without_a_registered_tie_coefficient_are_not_applicable_and_dangling_ends_fail() {
    let mut doc = tie_slope_document("seismic-tie-slope-pass");
    doc.site.en_ground_type = semio_s_artifact_norm_en1998_model::EnGroundType::A;
    assert_eq!(find(&inferences::evaluate(&doc), "tb-1.tie").status, CheckStatus::NotApplicable);
    let mut dangling = tie_slope_document("seismic-tie-slope-pass");
    dangling.tie_beams[0].foundation_b_id = "fd-9".into();
    let report = inferences::evaluate(&dangling);
    assert_eq!(find(&report, "tb-1.foundationBId").status, CheckStatus::Fail);
    assert!(report.checks.iter().all(|c| c.id != "en1998.5.tb-1.tie"));
}

#[semio_framework_async_macros::async_test]
async fn the_seismic_slope_factor_is_the_least_bishop_factor_of_the_given_and_the_critical_circle() {
    let pass = inferences::evaluate(&tie_slope_document("seismic-tie-slope-pass"));
    let slope = find(&pass, "sl-1.slope");
    assert_eq!(slope.status, CheckStatus::UnverifiedBasis);
    assert!((slope.utilization - 0.847328895).abs() < 1e-5, "{}", slope.utilization);
    assert!((slope.computed.value - 1.0 / slope.utilization).abs() < 1e-9);
    let fail = inferences::evaluate(&tie_slope_document("seismic-tie-slope-fail"));
    let failing = find(&fail, "sl-1.slope");
    assert_eq!(failing.status, CheckStatus::Fail);
    assert!(failing.remedies.iter().any(|r| r.target.path.ends_with(".frictionAngle")) && failing.remedies.iter().any(|r| r.target.path.ends_with(".angle")));
    let mut doc = tie_slope_document("seismic-tie-slope-pass");
    doc.slopes[0].governing_layer_id = "l9".into();
    assert_eq!(find(&inferences::evaluate(&doc), "sl-1.slope").status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn a_steeper_site_amplification_lowers_the_slope_factor() {
    let mut doc = tie_slope_document("seismic-tie-slope-pass");
    let flat = find(&inferences::evaluate(&doc), "sl-1.slope").computed.value;
    doc.slopes[0].topographic_amplification = 1.4;
    assert!(find(&inferences::evaluate(&doc), "sl-1.slope").computed.value < flat);
}

/// 🪨 Table 5.1 cap of the sliding demand: the design base shear of a system whose q exceeds q_max is evaluated at q_max (plateau S_d = a_g·γ_I·S·2.5/q_max, hand value), reduced by 0.8 on an irregular elevation, with λ = 0.85 for T1 ≤ 2 T_C and more than two storeys; the oracle mirror `design_base_shear` of `🔮️oracles/🚦️check-outcome/🐍️.py` computes the same.
#[semio_framework_async_macros::async_test]
async fn the_design_base_shear_caps_q_at_the_table_5_1_limit_and_reduces_it_for_irregular_elevation() {
    let mut doc = En1998Snapshot::compliant_de_office();
    let system = &mut doc.buildings[0].systems[0];
    system.q0 = 50.0;
    let q_max = part_1::q_max_for_system(system.system_type, system.material, system.ductility_class);
    let (a_g, gamma_i, s, tb, tc, td, t1, mass) = (2.0, 1.0, 1.0, 0.15, 0.5, 2.0, 0.5, 1000.0);
    let regular = part_1::design_base_shear(Some(system), true, 3, mass, t1, a_g, gamma_i, s, tb, tc, td);
    assert!(regular.q > q_max);
    let expected_s_d = a_g * gamma_i * s * 2.5 / q_max;
    assert!((regular.s_d - expected_s_d).abs() < 1e-9 * expected_s_d, "{} != {expected_s_d}", regular.s_d);
    assert!((regular.f_b - expected_s_d * mass * 0.85).abs() < 1e-6 * regular.f_b);
    let irregular = part_1::design_base_shear(Some(system), false, 3, mass, t1, a_g, gamma_i, s, tb, tc, td);
    let expected_irregular = a_g * gamma_i * s * 2.5 / (q_max * 0.8);
    assert!((irregular.s_d - expected_irregular).abs() < 1e-9 * expected_irregular);
}

fn showcase_check(report: &CheckReport, id: &str) -> semio_s_plugin_norm_compliance::CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id} missing")).clone()
}

/// 🫧 The excess pore pressure ratio of a showcase layer is the arcsine law of its settled factor of safety (the layer check stores F_L as its limit), and a liquefied layer under a gently sloping ground flags lateral spreading with a densification and a grading remedy.
#[semio_framework_async_macros::async_test]
async fn the_pore_pressure_ratio_follows_the_settled_factor_and_a_liquefied_slope_flags_lateral_spreading_with_remedies() {
    let report = inferences::evaluate(&En1998Snapshot::full_showcase());
    let factor = showcase_check(&report, "en1998.5.fd-3.layer.l7.liquefaction").limit.value;
    assert!(factor < 1.25, "{factor}");
    let r_u = showcase_check(&report, "en1998.5.fd-3.layer.l7.poreRatio");
    assert_eq!(r_u.status, CheckStatus::Info);
    assert!((r_u.computed.value - liquefaction_hazard::excess_pore_pressure_ratio(factor, 0.7, 0.34)).abs() < 1e-12);
    let spreading = showcase_check(&report, "en1998.5.fd-3.spreading");
    assert_eq!(spreading.status, CheckStatus::UnverifiedBasis);
    assert!((spreading.computed.value - 4.0).abs() < 1e-12);
    let targets: Vec<String> = spreading.remedies.iter().map(|r| r.target.path.clone()).collect();
    assert!(targets.contains(&"soilLayers[id=l7].n160".to_owned()) && targets.contains(&"slopes[id=sl-2].angle".to_owned()), "{targets:?}");
    let grading = spreading.remedies.iter().find(|r| r.target.path == "slopes[id=sl-2].angle").expect("grading");
    assert!((grading.required.value - (0.1_f64 / 100.0).atan().to_degrees()).abs() < 1e-12);
}

/// 🏗️ The pile of the third showcase foundation buckles over the liquefied length of its layer l7 plus the depth to fixity: utilisation N_Ed/N_cr of the hand formula, and the head shear moment over the same length.
#[semio_framework_async_macros::async_test]
async fn the_pile_in_the_liquefied_showcase_layer_buckles_over_the_liquefied_length_plus_the_depth_to_fixity() {
    let showcase = En1998Snapshot::full_showcase();
    let report = inferences::evaluate(&showcase);
    let f = showcase.foundations.iter().find(|f| f.id == "fd-3").expect("fd-3");
    let ei = part_5::pile_bending_stiffness(f.pile_modulus_pa, f.pile_diameter_m);
    let beta = part_5::pile_beta_per_m(f.pile_modulus_pa, f.pile_diameter_m, f.pile_subgrade_modulus_n_m3);
    let l_eff = liquefaction_hazard::pile_effective_length_m(4.0, 1.0, beta, 1.0);
    let n_cr = liquefaction_hazard::pile_buckling_load_n(ei, l_eff);
    let buckling = showcase_check(&report, "en1998.5.fd-3.pile.liquefiedBuckling");
    assert!((buckling.utilization - f.pile_axial_load_n / n_cr).abs() < 1e-9 * (f.pile_axial_load_n / n_cr));
    let moment = showcase_check(&report, "en1998.5.fd-3.pile.liquefiedMoment");
    let expected = liquefaction_hazard::pile_unsupported_moment_nm(f.pile_head_shear_n, l_eff, true) / f.pile_moment_rd_nm;
    assert!((moment.utilization - expected).abs() < 1e-9 * expected);
}

/// 🛞 The showcase isolation system settles its equivalent linear state: the period of the maximum-period check is 2π √(M d/F) of the displacement and the isolation shear of the other checks, and the bridge shear resistance carries the isolation shear.
#[semio_framework_async_macros::async_test]
async fn the_showcase_isolation_system_settles_a_consistent_equivalent_linear_state() {
    let showcase = En1998Snapshot::full_showcase();
    let report = inferences::evaluate(&showcase);
    let bridge = showcase.bridges.iter().find(|b| b.id == "br-1").expect("br-1");
    let mass = part_1::entity_seismic_mass_kg(bridge.permanent_gk_n, &bridge.variables, false, bridge.correlated_occupancy);
    let d = showcase_check(&report, "en1998.isolation.iso-1.displacement").computed.value;
    let force = showcase_check(&report, "en1998.isolation.iso-1.shear").computed.value;
    let t_eff = showcase_check(&report, "en1998.isolation.iso-1.maxPeriod").computed.value;
    assert!((t_eff - isolation::effective_period_s(mass, force / d)).abs() < 1e-9 * t_eff);
    let system = &showcase.isolation_systems[0];
    let law = isolation::Bilinear { strength_n: system.strength_n, post_yield_n_per_m: system.stiffness_n_per_m, yield_displacement_m: system.yield_displacement_m };
    assert!((law.force_n(d) - force).abs() < 1e-9 * force);
    assert!((showcase_check(&report, "en1998.isolation.iso-1.damping").computed.value - law.state(d, mass).damping).abs() < 1e-9);
}
