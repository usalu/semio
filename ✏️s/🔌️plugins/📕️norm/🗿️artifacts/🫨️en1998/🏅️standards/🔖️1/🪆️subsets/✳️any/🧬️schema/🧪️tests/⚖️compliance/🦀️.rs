use super::*;
use crate::document::CheckStatus;
use crate::field_meta::en1998_field_meta;
use crate::standards::v1::subsets::any::schema::inferences;
use crate::En1998Snapshot;
use std::path::PathBuf;
use std::process::Command;

#[semio_framework_async_macros::async_test]
async fn de_na_br_spectrum_at_t1() {
    let combo = na_de::GroundCombo::BR;
    let (s, tb, tc, td) = combo.spectrum_params();
    let a_g = na_de::SeismicZone::Zone2.a_gr();
    let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, 0.25);
    assert!((s_e - 1.875).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn en_type1_vs_de_divergence() {
    let de = AnnexParams::De { zone: na_de::SeismicZone::Zone2, combo: na_de::GroundCombo::BR };
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
async fn de_seismic_zone_is_schema_enum_0_to_3() {
    use crate::DeSeismicZone;
    assert_eq!(DeSeismicZone::Zone0.as_u8(), 0);
    assert_eq!(DeSeismicZone::Zone3.a_gr(), 0.8);
    let mut doc = crate::En1998Snapshot::compliant_de_office();
    doc.site.seismic_zone = DeSeismicZone::Zone0;
    doc.site.a_gr = 0.0;
    let report = inferences::check_full_seismic(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("zone0") && c.status == CheckStatus::NotApplicable));
}

#[semio_framework_async_macros::async_test]
async fn compliant_default_has_no_fail() {
    let report = inferences::evaluate(&crate::En1998Snapshot::default());
    assert!(report.complies(), "fails: {:?}", report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn noncompliant_has_failures_and_remedies() {
    let report = inferences::evaluate(&crate::En1998Snapshot::noncompliant_de_office());
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).collect();
    assert!(fails.len() >= 2);
    assert!(fails.iter().all(|c| !c.remedies.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn remedy_raising_vrd_improves_base_shear() {
    let mut doc = crate::En1998Snapshot::noncompliant_de_office();
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
    let mut doc = crate::En1998Snapshot::noncompliant_de_office();
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
    let mut doc = crate::En1998Snapshot::noncompliant_de_office();
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
                let tree = dsl::ToValue::to_value(&En1998Snapshot::noncompliant_de_office());
                crate::app_surface::get_value_at_path(&tree, path).unwrap_or_else(|e| panic!("path {path} must resolve: {e}"));
            }
        }
    }
    assert!(saw_id, "expected at least one id-path remedy");
}

#[semio_framework_async_macros::async_test]
async fn field_meta_covers_every_editable_leaf_of_default_snapshot() {
    let tree = dsl::ToValue::to_value(&En1998Snapshot::default());
    let mut missing = Vec::new();
    fn walk(prefix: &str, value: &dsl::DslValue, missing: &mut Vec<String>) {
        match value {
            dsl::DslValue::Object(entries) => {
                for (k, v) in entries {
                    let p = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    walk(&p, v, missing);
                }
            }
            dsl::DslValue::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    let p = format!("{prefix}[{i}]");
                    walk(&p, item, missing);
                }
            }
            _ => {
                if prefix.is_empty() { return; }
                match en1998_field_meta(prefix) {
                    Some(meta) if !meta.label_en.is_empty() && !meta.label_de.is_empty() => {}
                    Some(_) => missing.push(format!("{prefix} (empty label)")),
                    None => missing.push(prefix.to_string()),
                }
            }
        }
    }
    walk("", &tree, &mut missing);
    assert!(missing.is_empty(), "missing field-meta for: {missing:?}");
    let zone = en1998_field_meta("site.seismicZone").expect("zone meta");
    let choices = zone.choices.expect("zone choices");
    assert!(choices.iter().any(|c| c.value == "zone2" && c.label_en.contains("Zone") && c.label_de.contains("Erdbebenzone")));
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
        let status = Command::new("python3")
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
    let script = family_any_dir().join("🔮️oracles/🐍️.py");
    if !script.exists() {
        return;
    }
    for (label, snap) in [("compliant", En1998Snapshot::compliant_de_office()), ("noncompliant", En1998Snapshot::noncompliant_de_office())] {
        let rust = inferences::evaluate(&snap);
        let input = serde_json::to_string(&snap).expect("json");
        let mut child = Command::new("python3").arg(&script).arg("--json").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().expect("oracle spawn");
        {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{label} oracle failed: {}", String::from_utf8_lossy(&output.stderr));
        let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
        let py_checks = py.get("checks").and_then(|c| c.as_array()).expect("oracle checks array");
        for pc in py_checks {
            let id = pc.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let Some(pu) = pc.get("utilization").and_then(|v| v.as_f64()) else { continue };
            let Some(rc) = rust.checks.iter().find(|c| c.id == id) else { continue };
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
    let script = family_any_dir().join("🔮️oracles/🐍️.py");
    let output = Command::new("python3").arg(&script).arg("--combo-table").output().expect("oracle combo table");
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
            "A-R" => crate::DeGroundCombo::AR,
            "B-R" => crate::DeGroundCombo::BR,
            "C-R" => crate::DeGroundCombo::CR,
            "B-T" => crate::DeGroundCombo::BT,
            "C-T" => crate::DeGroundCombo::CT,
            "C-S" => crate::DeGroundCombo::CS,
            _ => unreachable!(),
        };
        let input = serde_json::to_string(&snap).unwrap();
        let mut child = Command::new("python3")
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

fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema/📸️snapshot/🔣️.json")
}
