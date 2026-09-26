use super::*;
use crate::app_surface::{get_value_at_path, insert_value_at_path, remove_value_at_path, set_value_at_path};
use crate::document::{AnnexChoice, CheckStatus, RemedyBound};
use crate::field_meta::en1994_field_meta;
use crate::{encode_en1994_snapshot_json, En1994Snapshot, SteelSection};
use std::path::PathBuf;
use std::process::Command;

fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema").join("📸️snapshot").join("🔣️.json")
}

fn oracle_script() -> PathBuf {
    family_any_dir().join("🔮️oracles").join("⚖️compliance").join("🐍️.py")
}

fn example_dsl(name: &str) -> String {
    let path = match name {
        "composite-floor-beam" => family_any_dir().join("🖼️assets/🏢composite-floor-beam/🏢composite-floor-beam/🗣️.dsl.semio"),
        "composite-floor-beam-failing" => family_any_dir().join("🖼️assets/🏢composite-floor-beam-failing/🏢composite-floor-beam-failing/🗣️.dsl.semio"),
        "composite-bridge-girder" => family_any_dir().join("🖼️assets/🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio"),
        _ => panic!("unknown example {name}"),
    };
    std::fs::read_to_string(path).expect("dsl asset")
}

#[test]
fn full_composite_worked_example_passes_default() {
    let report = evaluate(&En1994Snapshot::default());
    assert!(report.checks.len() >= 10, "len={}", report.checks.len());
    assert!(
        report.complies(),
        "default subject should comply; fails={:?}",
        report.failing().map(|c| c.id.as_str()).collect::<Vec<_>>()
    );
}

#[test]
fn evaluate_runs_building_and_fire_parts() {
    let report = evaluate(&En1994Snapshot::default());
    assert!(report.checks.iter().any(|c| c.part.contains("1994-1-1")));
    assert!(report.checks.iter().any(|c| c.part.contains("1994-1-2")));
    assert!(report.checks.iter().any(|c| c.status == CheckStatus::NotApplicable && c.id.contains("fatigue")));
}

#[test]
fn failing_subject_has_remedies() {
    let mut doc = En1994Snapshot::default();
    for a in &mut doc.beams[0].actions {
        if a.kind == "imposed" { a.q_area_pa = 25.0e3; }
    }
    doc.insulation_thickness_m = 0.010;
    doc.beams[0].studs.total_count = 5;
    let report = evaluate(&doc);
    let fails: Vec<_> = report.failing().collect();
    assert!(!fails.is_empty());
    for f in fails {
        assert!(!f.remedies.is_empty(), "fail {} missing remedies", f.id);
        assert!(f.remedies.iter().all(|r| r.applicable), "fail {} has non-applicable remedy", f.id);
    }
}

#[test]
fn de_vs_en_bridge_fatigue_gamma_mf() {
    let mut en = En1994Snapshot::default();
    en.annex = AnnexChoice::En;
    en.structure_kind = "bridge".into();
    en.beams[0].n_cycles = 2.0e6;
    en.beams[0].actions.push(crate::CharacteristicAction {
        id: "FLM3".into(), kind: "fatigue".into(), category: "flm3".into(), stage: "composite".into(),
        q_area_pa: 0.0, f_k_n: 0.0,
        delta_sigma_k_pa: 65e6, delta_tau_k_pa: 40e6,
    });
    let mut de = en.clone();
    de.annex = AnnexChoice::De;
    let r_en = evaluate(&en);
    let r_de = evaluate(&de);
    let fat_en = r_en.checks.iter().find(|c| c.id.contains("delta-sigma")).unwrap();
    let fat_de = r_de.checks.iter().find(|c| c.id.contains("delta-sigma")).unwrap();
    assert_eq!(fat_en.status, CheckStatus::Pass, "EN γ_Mf=1.15 should pass");
    assert_eq!(fat_de.status, CheckStatus::Fail, "DE γ_Mf=1.35 should fail");
}

#[test]
fn remedy_law_stud_count() {
    let mut doc = En1994Snapshot::default();
    doc.beams[0].studs.total_count = 5;
    doc.beams[0].studs.spacing_m = 1.5;
    let report = evaluate(&doc);
    let fail = report.failing().find(|c| c.id.contains("prd") || c.id.contains("etamin")).expect("expected stud-related fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("totalCount")).expect("stud count remedy");
    assert!(remedy.target.path.contains("beams[id=beam-B1]"), "path={}", remedy.target.path);
    doc.beams[0].studs.total_count = remedy.required.value as u32;
    let after = evaluate(&doc);
    let again = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert!(again.utilization <= 1.0 || again.status != CheckStatus::Fail, "u={}", again.utilization);
}

#[test]
fn subject_paths_use_stable_ids_and_survive_reorder() {
    let mut doc = En1994Snapshot::default();
    doc.beams[0].studs.total_count = 5;
    doc.beams[0].studs.spacing_m = 1.5;
    let report = evaluate(&doc);
    for check in &report.checks {
        if check.subject.path.starts_with("beams") || check.subject.path.starts_with("columns") || check.subject.path.starts_with("slabs") {
            assert!(
                check.subject.path.contains("[id="),
                "expected id selector in {}",
                check.subject.path
            );
        }
        for remedy in &check.remedies {
            if remedy.target.path.starts_with("beams") || remedy.target.path.starts_with("columns") || remedy.target.path.starts_with("slabs") {
                assert!(remedy.target.path.contains("[id="), "remedy path {}", remedy.target.path);
            }
        }
    }
    let fail = report.failing().find(|c| c.id.contains("prd") || c.id.contains("etamin")).expect("stud fail");
    let remedy = fail.remedies.iter().find(|r| r.target.path.contains("totalCount")).expect("stud remedy");
    let path = remedy.target.path.clone();
    let required = remedy.required.value;

    let mut tree = dsl::ToValue::to_value(&doc);
    let extra = crate::CompositeBeam {
        id: "beam-FRONT".into(),
        ..crate::CompositeBeam::default_placeholder()
    };
    insert_value_at_path(&mut tree, "beams", 0, Some(dsl::ToValue::to_value(&extra))).expect("insert");
    let mut reordered: En1994Snapshot = dsl::FromValue::from_value(tree.clone()).expect("from tree");
    assert_eq!(reordered.beams[1].id, "beam-B1");
    set_value_at_path(&mut tree, &path, dsl::DslValue::uint(required as u64)).expect("set via id path after reorder");
    reordered = dsl::FromValue::from_value(tree.clone()).expect("decode");
    assert_eq!(reordered.beams.iter().find(|b| b.id == "beam-B1").unwrap().studs.total_count, required as u32);
    remove_value_at_path(&mut tree, "beams[id=beam-FRONT]", 0).expect("remove");
    let _ = get_value_at_path(&tree, "beams[id=beam-B1].studs.totalCount").expect("resolve id path");
}

#[test]
fn heavier_section_oneof_remedy_writes_designation() {
    let mut doc = En1994Snapshot::default();
    for a in &mut doc.beams[0].actions {
        if a.kind == "imposed" { a.q_area_pa = 80.0e3; }
    }
    doc.beams[0].span_m = 14.0;
    let report = evaluate(&doc);
    let fail = report.failing().find(|c| c.id.contains("mrd")).expect("bending fail");
    let one = fail.remedies.iter().find(|r| matches!(r.bound, RemedyBound::OneOf) && r.applicable).expect("oneof");
    assert!(one.target.path.ends_with("steel.designation"), "{}", one.target.path);
    assert!(!one.options.is_empty());
    let mut tree = dsl::ToValue::to_value(&doc);
    set_value_at_path(&mut tree, &one.target.path, dsl::DslValue::String(one.options[0].clone())).expect("write designation");
    let next: En1994Snapshot = dsl::FromValue::from_value(tree).expect("decode");
    assert_eq!(SteelSection::normalize_designation(&next.beams[0].steel.designation), SteelSection::normalize_designation(&one.options[0]));
    let after = evaluate(&next);
    let again = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert!(again.utilization < fail.utilization || again.status != CheckStatus::Fail);
}

#[test]
fn insulation_remedy_flips_fire_check() {
    let mut doc = En1994Snapshot::default();
    doc.insulation_thickness_m = 0.008;
    let report = evaluate(&doc);
    let fail = report.failing().find(|c| c.id.contains("insulation")).expect("fire fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).expect("remedy");
    doc.insulation_thickness_m = remedy.required.value;
    let after = evaluate(&doc);
    let again = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert_ne!(again.status, CheckStatus::Fail);
}

#[test]
fn passing_example_dsl_complies() {
    let doc = crate::decode_en1994_dsl(&example_dsl("composite-floor-beam")).expect("decode");
    let report = evaluate(&doc);
    assert!(report.complies(), "fails={:?}", report.failing().map(|c| c.id.as_str()).collect::<Vec<_>>());
}

#[test]
fn failing_example_dsl_does_not_comply_with_named_ids() {
    let doc = crate::decode_en1994_dsl(&example_dsl("composite-floor-beam-failing")).expect("decode");
    assert!(doc.beams[0].steel.a_m2 > 0.01, "nested steel must be present");
    assert!(doc.beams[0].studs.total_count > 0);
    let report = evaluate(&doc);
    assert!(!report.complies());
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
    let ids: Vec<_> = report.failing().map(|c| c.id.as_str()).collect();
    assert!(ids.iter().any(|id| id.contains("mrd") || id.contains("beff") || id.contains("prd") || id.contains("insulation") || id.contains("crack") || id.contains("ltb")), "{ids:?}");
}

#[test]
fn bridge_example_runs_fatigue_checks() {
    let doc = crate::decode_en1994_dsl(&example_dsl("composite-bridge-girder")).expect("decode");
    assert_eq!(doc.structure_kind, "bridge");
    assert!(doc.beams[0].steel.a_m2 > 0.01);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("delta-sigma")));
    assert!(report.checks.iter().any(|c| c.id.contains("stud") && c.part.contains("1994-2")));
}

#[test]
fn every_default_leaf_has_en_de_field_meta() {
    fn walk(value: &dsl::DslValue, path: &str, leaves: &mut Vec<String>) {
        match value {
            dsl::DslValue::Object(map) => {
                for (k, v) in map.iter() {
                    let next = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                    walk(v, &next, leaves);
                }
            }
            dsl::DslValue::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    let id = match item {
                        dsl::DslValue::Object(map) => map
                            .iter()
                            .find(|(k, _)| k == "id")
                            .and_then(|(_, v)| match v {
                                dsl::DslValue::String(s) => Some(s.as_str()),
                                _ => None,
                            }),
                        _ => None,
                    };
                    let seg = id.map(|id| format!("[id={id}]")).unwrap_or_else(|| format!("[{i}]"));
                    let next = format!("{path}{seg}");
                    walk(item, &next, leaves);
                }
            }
            _ => {
                if !path.is_empty() {
                    leaves.push(path.to_string());
                }
            }
        }
    }
    let doc = En1994Snapshot::default();
    let mut leaves = Vec::new();
    walk(&dsl::ToValue::to_value(&doc), "", &mut leaves);
    assert!(!leaves.is_empty());
    for path in &leaves {
        let meta = en1994_field_meta(path).unwrap_or_else(|| panic!("missing meta for {path}"));
        assert!(!meta.label_en.is_empty(), "{path}");
        assert!(!meta.label_de.is_empty(), "{path}");
        if let Some(choices) = meta.choices {
            for c in choices {
                assert_ne!(c.label_en, c.value, "raw wire label_en for {} on {path}", c.value);
                assert!(!c.label_de.is_empty());
                // human labels must differ from raw codes for non-identical catalogue codes
                if matches!(c.value, "building" | "bridge" | "propped" | "unpropped" | "true" | "false" | "en" | "de" | "r30" | "r60" | "r90" | "r120") {
                    assert_ne!(c.label_en, c.value, "{path} {}", c.value);
                }
            }
        }
    }
}

#[test]
fn example_snapshot_validates_against_json_schema() {
    let schema_path = snapshot_schema_path();
    for (label, snap) in [
        ("default", En1994Snapshot::default()),
        ("passing", crate::decode_en1994_dsl(&example_dsl("composite-floor-beam")).unwrap()),
        ("failing", crate::decode_en1994_dsl(&example_dsl("composite-floor-beam-failing")).unwrap()),
    ] {
        let instance = encode_en1994_snapshot_json(&snap);
        let tmp = std::env::temp_dir().join(format!("en1994-instance-{label}.json"));
        std::fs::write(&tmp, &instance).unwrap();
        let status = Command::new("python3")
            .arg("-c")
            .arg("import json,sys,jsonschema; s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2])); jsonschema.validate(instance=i, schema=s); print('ok')")
            .arg(&schema_path)
            .arg(&tmp)
            .output()
            .expect("python jsonschema");
        assert!(
            status.status.success(),
            "{label} jsonschema: {}\n{}",
            String::from_utf8_lossy(&status.stdout),
            String::from_utf8_lossy(&status.stderr)
        );
    }
}

#[test]
fn python_oracle_matches_evaluate_json_within_half_percent() {
    for (label, snap) in [
        ("compliant", En1994Snapshot::default()),
        ("failing", crate::decode_en1994_dsl(&example_dsl("composite-floor-beam-failing")).unwrap()),
    ] {
        let rust = evaluate(&snap);
        let report_json = serde_json::to_string(&rust).expect("report json");
        let snap_json = encode_en1994_snapshot_json(&snap);
        let mut child = Command::new("python3")
            .arg(oracle_script())
            .arg("--evaluate-json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| panic!("oracle spawn: {e}"));
        {
            use std::io::Write;
            let payload = serde_json::json!({ "snapshot": serde_json::from_str::<serde_json::Value>(&snap_json).unwrap(), "report": serde_json::from_str::<serde_json::Value>(&report_json).unwrap() });
            child.stdin.as_mut().unwrap().write_all(payload.to_string().as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{label} oracle stderr={}", String::from_utf8_lossy(&output.stderr));
        let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
        let compared = py.get("compared").and_then(|v| v.as_u64()).unwrap_or(0);
        assert!(compared >= 3, "{label}: compared={compared} body={}", String::from_utf8_lossy(&output.stdout));
        let max_rel = py.get("maxRel").and_then(|v| v.as_f64()).unwrap_or(1.0);
        assert!(max_rel <= 0.005, "{label} maxRel={max_rel}");
    }
}


#[test]
fn every_editable_leaf_affects_at_least_one_check() {
    /// 🧪 Normative signature: status + computed + limit + utilization (not explanation text).
    fn norm_sig(report: &crate::document::CheckReport) -> Vec<(String, String, i64, i64, i64)> {
        let mut out: Vec<_> = report
            .checks
            .iter()
            .map(|c| {
                (
                    c.id.clone(),
                    format!("{:?}", c.status),
                    (c.computed.value * 1e6).round() as i64,
                    (c.limit.value * 1e6).round() as i64,
                    (c.utilization * 1e9).round() as i64,
                )
            })
            .collect();
        out.sort();
        out
    }

    fn leaf_name(path: &str) -> &str {
        path.rsplit('.').next().unwrap_or(path)
    }

    fn is_label(path: &str) -> bool {
        matches!(leaf_name(path), "id" | "name" | "title" | "label" | "labelEn" | "labelDe")
    }

    fn walk(value: &dsl::DslValue, path: &str, leaves: &mut Vec<(String, dsl::DslValue)>) {
        match value {
            dsl::DslValue::Object(map) => {
                for (k, v) in map.iter() {
                    let next = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                    walk(v, &next, leaves);
                }
            }
            dsl::DslValue::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    let id = match item {
                        dsl::DslValue::Object(map) => map
                            .iter()
                            .find(|(k, _)| k == "id")
                            .and_then(|(_, v)| match v {
                                dsl::DslValue::String(s) => Some(s.clone()),
                                _ => None,
                            }),
                        _ => None,
                    };
                    let seg = id.map(|id| format!("[id={id}]")).unwrap_or_else(|| format!("[{i}]"));
                    walk(item, &format!("{path}{seg}"), leaves);
                }
            }
            leaf => {
                if !path.is_empty() && !is_label(path) {
                    leaves.push((path.to_string(), leaf.clone()));
                }
            }
        }
    }

    fn candidates(original: &dsl::DslValue, path: &str) -> Vec<dsl::DslValue> {
        match original {
            dsl::DslValue::Number(n) => {
                let v = n.as_f64();
                let leaf = leaf_name(path);
                if leaf.starts_with("delta") || leaf.ends_with("KPa") {
                    let base = if v.abs() < 1.0 { 50e6 } else { v };
                    return vec![dsl::DslValue::float(base * 3.0), dsl::DslValue::float(base * 0.1)];
                }
                let base = if v.abs() < 1e-12 { 1.0 } else { v };
                vec![
                    if n.is_integer() {
                        dsl::DslValue::uint((base * 3.0).max(1.0).round() as u64)
                    } else {
                        dsl::DslValue::float(base * 3.0)
                    },
                    if n.is_integer() {
                        dsl::DslValue::uint((base * 0.25).max(1.0).round() as u64)
                    } else {
                        dsl::DslValue::float(base * 0.25)
                    },
                ]
            }
            dsl::DslValue::Bool(v) => vec![dsl::DslValue::Bool(!*v)],
            dsl::DslValue::String(s) => {
                if leaf_name(path) == "annex" {
                    return ["En", "De"]
                        .into_iter()
                        .filter(|c| *c != s.as_str())
                        .map(|c| dsl::DslValue::String((*c).into()))
                        .collect();
                }
                let meta = en1994_field_meta(path);
                if let Some(choices) = meta.and_then(|m| m.choices) {
                    choices
                        .iter()
                        .map(|c| c.value)
                        .filter(|c| *c != s.as_str())
                        .map(|c| dsl::DslValue::String(c.to_string()))
                        .collect()
                } else if leaf_name(path) == "category" {
                    ["A", "B", "C", "E", "self_steel", "finishes", "flm3"]
                        .into_iter()
                        .filter(|c| *c != s.as_str())
                        .map(|c| dsl::DslValue::String(c.into()))
                        .collect()
                } else if s == "HEB300" || s.starts_with("HEB") {
                    vec![dsl::DslValue::String("HEB400".into()), dsl::DslValue::String("CUSTOM-PLATE".into())]
                } else if s == "CUSTOM-PLATE" {
                    vec![dsl::DslValue::String("HEB300".into())]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }

    fn catalogue_steel_geom(path: &str) -> bool {
        path.contains(".steel.") && !path.ends_with("designation")
    }

    type Pred = fn(&str) -> bool;
    let failing_beam = crate::decode_en1994_dsl(&example_dsl("composite-floor-beam-failing")).expect("failing beam");
    let scopes: [(&str, En1994Snapshot, Pred); 6] = [
        (
            "default-building",
            En1994Snapshot::default(),
            |p| {
                let leaf = leaf_name(p);
                if catalogue_steel_geom(p) {
                    return false;
                }
                if matches!(leaf, "ltbLengthM" | "nCycles" | "fatigueDetail" | "annex") {
                    return false;
                }
                if p.contains("deltaSigma") || p.contains("deltaTau") || p.contains("FLM3") {
                    return false;
                }
                if leaf == "category" {
                    return p.contains("beams[") && p.contains("Q-office");
                }
                true
            },
        ),
        (
            "failing-beam",
            failing_beam,
            |p| {
                let leaf = leaf_name(p);
                // Failing DSL base: walk governing load / stud / span leaves that prove Fail paths.
                if p.contains("beams[") && matches!(leaf, "qAreaPa" | "spacingM" | "totalCount" | "spanM" | "slabThicknessM") {
                    return true;
                }
                false
            },
        ),
        (
            "unpropped-ltb",
            En1994Snapshot::unpropped_building(),
            |p| matches!(leaf_name(p), "ltbLengthM" | "construction"),
        ),
        (
            "custom-plate",
            En1994Snapshot::custom_plate_building(),
            |p| catalogue_steel_geom(p) || p.ends_with("steel.designation"),
        ),
        (
            "bridge-fatigue",
            En1994Snapshot::bridge_girder(),
            |p| {
                let leaf = leaf_name(p);
                if matches!(leaf, "annex" | "structureKind" | "nCycles" | "fatigueDetail") {
                    return true;
                }
                if p.contains("[id=FLM3") && matches!(leaf, "deltaSigmaKPa" | "deltaTauKPa" | "kind" | "fKN" | "qAreaPa") {
                    return true;
                }
                false
            },
        ),
        (
            "fire-demanding",
            En1994Snapshot::fire_demanding(),
            |p| matches!(leaf_name(p), "fireRating" | "insulationThicknessM"),
        ),
    ];

    let mut unaffected = Vec::new();
    let mut checked = 0usize;
    for (label, base, pred) in scopes {
        let base_sig = norm_sig(&evaluate(&base));
        let mut leaves = Vec::new();
        walk(&dsl::ToValue::to_value(&base), "", &mut leaves);
        let scoped: Vec<_> = leaves.into_iter().filter(|(p, _)| pred(p)).collect();
        assert!(!scoped.is_empty(), "{label}: expected scoped leaves");
        for (path, original) in scoped {
            let mut changed = false;
            for mutated in candidates(&original, &path) {
                if mutated == original {
                    continue;
                }
                let mut trial_tree = dsl::ToValue::to_value(&base);
                if set_value_at_path(&mut trial_tree, &path, mutated).is_err() {
                    continue;
                }
                let Ok(trial): Result<En1994Snapshot, _> = dsl::FromValue::from_value(trial_tree) else {
                    continue;
                };
                if norm_sig(&evaluate(&trial)) != base_sig {
                    changed = true;
                    break;
                }
            }
            checked += 1;
            if !changed {
                unaffected.push(format!("{label}:{path}"));
            }
        }
    }
    // Annex γ_Mf on bridge fatigue limits (DE 1.35 vs EN 1.15) — same proof as de_vs_en_bridge_fatigue_gamma_mf.
    {
        let mut de = En1994Snapshot::bridge_girder();
        de.annex = AnnexChoice::De;
        let mut en = de.clone();
        en.annex = AnnexChoice::En;
        let r_de = evaluate(&de);
        let r_en = evaluate(&en);
        let fat_de = r_de.checks.iter().find(|c| c.id.contains("delta-sigma")).expect("de fatigue");
        let fat_en = r_en.checks.iter().find(|c| c.id.contains("delta-sigma")).expect("en fatigue");
        assert!(fat_de.limit.value < fat_en.limit.value, "DE γ_Mf must lower Δσ_R (de={} en={})", fat_de.limit.value, fat_en.limit.value);
        checked += 1;
    }

    // Column force leaves on default (characteristic N_k / M_k enter ULS 6.10).



    {
        let base = En1994Snapshot::default();
        let base_sig = norm_sig(&evaluate(&base));
        let mut leaves = Vec::new();
        walk(&dsl::ToValue::to_value(&base), "", &mut leaves);
        for (path, original) in leaves {
            if !(path.contains("columns[") && matches!(leaf_name(&path), "mKNm" | "nKN" | "kind" | "wallThicknessM" | "outerSizeM" | "steelFYPa" | "concreteFCkPa" | "bucklingCurve")) {
                continue;
            }
            let mut changed = false;
            for mutated in candidates(&original, &path) {
                if mutated == original { continue; }
                let mut trial_tree = dsl::ToValue::to_value(&base);
                if set_value_at_path(&mut trial_tree, &path, mutated).is_err() { continue; }
                let Ok(trial): Result<En1994Snapshot, _> = dsl::FromValue::from_value(trial_tree) else { continue; };
                if norm_sig(&evaluate(&trial)) != base_sig {
                    changed = true;
                    break;
                }
            }
            checked += 1;
            if !changed {
                unaffected.push(format!("column-forces:{path}"));
            }
        }
    }
    assert!(checked > 20, "expected broad leaf coverage, checked={checked}");
    assert!(unaffected.is_empty(), "editable leaves with no normative check impact: {unaffected:?}");
}

#[test]
fn ts_snapshot_facets_have_no_unknown_and_match_rust_leaves() {
    let snap_ts = include_str!("../../../📸️snapshot/🟦️.ts");
    let schema_ts = include_str!("../../../🟦️.ts");
    for (label, body) in [("snapshot", snap_ts), ("schema", schema_ts)] {
        assert!(!body.contains("unknown"), "{label} TS must not use unknown");
        assert!(!body.contains("Record<string, unknown>"), "{label} TS must not use Record unknown");
        assert!(!body.contains("_placeholder"), "{label} TS must not use _placeholder");
        assert!(body.contains("interface CompositeBeam"), "{label} missing CompositeBeam");
        assert!(body.contains("interface CharacteristicAction"), "{label} missing CharacteristicAction");
        assert!(body.contains("interface ColumnAction"), "{label} missing ColumnAction");
        assert!(body.contains("qAreaPa"), "{label} missing qAreaPa");
        assert!(!body.contains("qLineNPerM"), "{label} must not keep qLineNPerM");
    }
    let encoded = serde_json::to_value(En1994Snapshot::default()).expect("json");
    let beams = encoded.get("beams").and_then(|v| v.as_array()).expect("beams");
    assert!(!beams.is_empty());
    let beam = &beams[0];
    for key in ["spanM", "spacingM", "steel", "studs", "actions", "sheeting"] {
        assert!(beam.get(key).is_some(), "rust beam missing {key}");
    }
    assert!(beam.get("actions").unwrap().as_array().unwrap()[0].get("qAreaPa").is_some());
    assert!(beam.get("actions").unwrap().as_array().unwrap()[0].get("qLineNPerM").is_none());
}


