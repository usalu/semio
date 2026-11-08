use semio_framework_test_fixtures::fixture_path;
use semio_s_artifact_norm_en1994_inferences::*;
use semio_s_plugin_norm_app_surface::{apply_remedy_edit, get_value_at_path, insert_value_at_path, remove_value_at_path, set_value_at_path};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus, RemedyBound};
use semio_s_artifact_norm_en1994_editor::field_meta::en1994_field_meta;
use semio_s_artifact_norm_en1994_model::{En1994Snapshot, SteelSection, Stud};
use semio_s_artifact_norm_en1994_io::text::snapshot::encode_en1994_snapshot_json;
use std::path::PathBuf;
use std::process::Command;

fn family_any_dir() -> PathBuf {
    fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
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
    let report = evaluate(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(report.checks.len() >= 10, "len={}", report.checks.len());
    assert!(
        report.complies(),
        "default subject should comply; fails={:?}",
        report.failing().map(|c| c.id.as_str()).collect::<Vec<_>>()
    );
}

#[test]
fn evaluate_runs_building_and_fire_parts() {
    let report = evaluate(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(report.checks.iter().any(|c| c.part.contains("1994-1-1")));
    assert!(report.checks.iter().any(|c| c.part.contains("1994-1-2")));
    assert!(report.checks.iter().any(|c| c.status == CheckStatus::NotApplicable && c.id.contains("fatigue")));
}

#[test]
fn failing_subject_has_remedies() {
    let mut doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    for a in &mut doc.beams[0].actions {
        if a.kind == semio_s_artifact_norm_en1994_model::ActionKind::Imposed { a.q_area_pa = 25.0e3; }
    }
    doc.insulation_thickness_m = 0.010;
    doc.studs = Stud::typical_row("beam-B1", 4.0, 1.5, 5);
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
    let mut en = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    en.annex = AnnexChoice::En;
    en.structure_kind = semio_s_artifact_norm_en1994_model::StructureKind::Bridge;
    en.beams[0].n_cycles = 2.0e6;
    en.beams[0].actions.push(semio_s_artifact_norm_en1994_model::CharacteristicAction {
        id: "FLM3".into(), kind: semio_s_artifact_norm_en1994_model::ActionKind::Fatigue, category: "flm3".into(), stage: semio_s_artifact_norm_en1994_model::ActionStage::Composite,
        q_area_pa: 0.0, f_k_n: 0.0,
        delta_sigma_k_pa: 65e6, delta_tau_k_pa: 40e6, x_m: 0.0,
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
fn remedy_law_stud_insert_repeats_until_the_stud_checks_clear() {
    let mut doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.studs = Stud::typical_row("beam-B1", 4.0, 1.5, 5);
    let report = evaluate(&doc);
    let fail = report.failing().find(|c| c.id.contains("prd") || c.id.contains("etamin")).expect("expected stud-related fail");
    assert!(fail.remedies.iter().any(|r| r.applicable && matches!(r.bound, RemedyBound::Insert) && r.target.path == "studs"), "stud insert remedy");
    let (id, before) = (fail.id.clone(), doc.studs.len());
    let mut tree = semio_framework_value::ToValue::to_value(&doc);
    for _ in 0..200 {
        let current: En1994Snapshot = semio_framework_value::FromValue::from_value(tree.clone()).expect("decode");
        let report = evaluate(&current);
        let check = report.checks.iter().find(|c| c.id == id).expect("check");
        if check.status != CheckStatus::Fail {
            break;
        }
        let index = check.remedies.iter().position(|r| r.applicable && matches!(r.bound, RemedyBound::Insert)).expect("insert remedy");
        apply_remedy_edit(&report, &id, index, 0, &mut tree).expect("apply the insert remedy");
    }
    let fixed: En1994Snapshot = semio_framework_value::FromValue::from_value(tree).expect("decode");
    let again = evaluate(&fixed);
    assert_ne!(again.checks.iter().find(|c| c.id == id).expect("check").status, CheckStatus::Fail);
    assert!(fixed.studs.len() > before);
}

#[test]
fn subject_paths_use_stable_ids_and_survive_reorder() {
    let mut doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.studs = Stud::typical_row("beam-B1", 4.0, 1.5, 5);
    let report = evaluate(&doc);
    for check in &report.checks {
        if check.subject.path.starts_with("beams") || check.subject.path.starts_with("columns") || check.subject.path.starts_with("slabs") || check.subject.path.starts_with("studs[") {
            assert!(check.subject.path.contains("[id="), "expected id selector in {}", check.subject.path);
        }
        for remedy in &check.remedies {
            if remedy.target.path.starts_with("beams") || remedy.target.path.starts_with("columns") || remedy.target.path.starts_with("slabs") || remedy.target.path.starts_with("studs[") {
                assert!(remedy.target.path.contains("[id="), "remedy path {}", remedy.target.path);
            }
        }
    }
    let spacing = report.checks.iter().find(|c| c.id.starts_with("en1994.6.6.5.5.spacing.")).expect("spacing check");
    assert!(spacing.subject.path.starts_with("studs[id=beam-B1-stud-"), "{}", spacing.subject.path);

    let mut tree = semio_framework_value::ToValue::to_value(&doc);
    let extra = Stud::typical("extra", "beam-B1", 0.2);
    insert_value_at_path(&mut tree, "studs", 0, Some(semio_framework_value::ToValue::to_value(&extra))).expect("insert");
    let mut reordered: En1994Snapshot = semio_framework_value::FromValue::from_value(tree.clone()).expect("from tree");
    assert_eq!(reordered.studs[1].id, "beam-B1-stud-0");
    set_value_at_path(&mut tree, "studs[id=beam-B1-stud-1].xM", semio_framework_value::DslValue::float(2.6)).expect("set via id path after reorder");
    reordered = semio_framework_value::FromValue::from_value(tree.clone()).expect("decode");
    assert_eq!(reordered.studs.iter().find(|s| s.id == "beam-B1-stud-1").unwrap().x_m, 2.6);
    remove_value_at_path(&mut tree, "studs[id=extra]", 0).expect("remove");
    let _ = get_value_at_path(&tree, "studs[id=beam-B1-stud-0].xM").expect("resolve id path");
}

#[test]
fn heavier_section_oneof_remedy_writes_designation() {
    let mut doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    for a in &mut doc.beams[0].actions {
        if a.kind == semio_s_artifact_norm_en1994_model::ActionKind::Imposed { a.q_area_pa = 80.0e3; }
    }
    doc.beams[0].span_m = 14.0;
    let report = evaluate(&doc);
    let fail = report.failing().find(|c| c.id.contains("mrd")).expect("bending fail");
    let one = fail.remedies.iter().find(|r| matches!(r.bound, RemedyBound::OneOf) && r.applicable).expect("oneof");
    assert!(one.target.path.ends_with("steel.designation"), "{}", one.target.path);
    assert!(!one.options.is_empty());
    let mut tree = semio_framework_value::ToValue::to_value(&doc);
    set_value_at_path(&mut tree, &one.target.path, semio_framework_value::DslValue::String(one.options[0].clone())).expect("write designation");
    let next: En1994Snapshot = semio_framework_value::FromValue::from_value(tree).expect("decode");
    assert_eq!(SteelSection::normalize_designation(&next.beams[0].steel.designation), SteelSection::normalize_designation(&one.options[0]));
    let after = evaluate(&next);
    let again = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert!(again.utilization < fail.utilization || again.status != CheckStatus::Fail);
}

#[test]
fn insulation_remedy_flips_fire_check() {
    let mut doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
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
    let doc = semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl(&example_dsl("composite-floor-beam")).expect("decode");
    let report = evaluate(&doc);
    assert!(report.complies(), "fails={:?}", report.failing().map(|c| c.id.as_str()).collect::<Vec<_>>());
}

#[test]
fn failing_example_dsl_does_not_comply_with_named_ids() {
    let doc = semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl(&example_dsl("composite-floor-beam-failing")).expect("decode");
    assert!(doc.beams[0].steel.a_m2 > 0.01, "nested steel must be present");
    assert!(!doc.studs.is_empty());
    let report = evaluate(&doc);
    assert!(!report.complies());
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
    let ids: Vec<_> = report.failing().map(|c| c.id.as_str()).collect();
    assert!(ids.iter().any(|id| id.contains("mrd") || id.contains("beff") || id.contains("prd") || id.contains("insulation") || id.contains("crack") || id.contains("ltb")), "{ids:?}");
}

#[test]
fn bridge_example_runs_fatigue_checks() {
    let doc = semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl(&example_dsl("composite-bridge-girder")).expect("decode");
    assert_eq!(doc.structure_kind, "bridge");
    assert!(doc.beams[0].steel.a_m2 > 0.01);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("delta-sigma")));
    assert!(report.checks.iter().any(|c| c.id.contains("stud") && c.part.contains("1994-2")));
}

#[test]
fn bridge_girder_ltb_and_crack_remedy0_clear() {
    let doc = semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl(&example_dsl("composite-bridge-girder")).expect("decode");
    let report = evaluate(&doc);
    for id in ["en1994.6.4.ltb.girder-G1", "en1994.7.4.crack.girder-G1"] {
        let fail = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing {id}"));
        assert_eq!(fail.status, CheckStatus::Fail, "{id} should start Fail");
        assert!(!fail.remedies.is_empty(), "{id} needs a remedy");
        let mut tree = semio_framework_value::ToValue::to_value(&doc);
        apply_remedy_edit(&report, id, 0, 0, &mut tree).unwrap_or_else(|e| panic!("{id} apply: {e:?}"));
        let fixed: En1994Snapshot = semio_framework_value::FromValue::from_value(tree).expect("decode after remedy");
        let after = evaluate(&fixed);
        let again = after.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing after {id}"));
        assert_ne!(again.status, CheckStatus::Fail, "{id} remedy[0] left Fail u={}", again.utilization);
    }
}

#[test]
fn every_default_leaf_has_en_de_field_meta() {
    fn walk(value: &semio_framework_value::DslValue, path: &str, leaves: &mut Vec<String>) {
        match value {
            semio_framework_value::DslValue::Object(map) => {
                for (k, v) in map.iter() {
                    let next = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                    walk(v, &next, leaves);
                }
            }
            semio_framework_value::DslValue::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    let id = match item {
                        semio_framework_value::DslValue::Object(map) => map
                            .iter()
                            .find(|(k, _)| k == "id")
                            .and_then(|(_, v)| match v {
                                semio_framework_value::DslValue::String(s) => Some(s.as_str()),
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
    let doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let mut leaves = Vec::new();
    walk(&semio_framework_value::ToValue::to_value(&doc), "", &mut leaves);
    assert!(!leaves.is_empty());
    for path in &leaves {
        let meta = en1994_field_meta(path).unwrap_or_else(|| panic!("missing meta for {path}"));
        assert!(!meta.label.en.is_empty(), "{path}");
        assert!(!meta.label.de.is_empty(), "{path}");
        if let Some(choices) = meta.choices() {
            for c in choices {
                assert_ne!(c.label_en, c.value, "raw wire label_en for {} on {path}", c.value);
                assert!(!c.label_de.is_empty());
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
        ("default", En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)),
        ("passing", semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl(&example_dsl("composite-floor-beam")).unwrap()),
        ("failing", semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl(&example_dsl("composite-floor-beam-failing")).unwrap()),
    ] {
        let instance = encode_en1994_snapshot_json(&snap);
        let tmp = std::env::temp_dir().join(format!("en1994-instance-{label}.json"));
        std::fs::write(&tmp, &instance).unwrap();
        let status = semio_s_plugin_norm_compliance::python_oracle::command()
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
        ("compliant", En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)),
        ("failing", semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl(&example_dsl("composite-floor-beam-failing")).unwrap()),
    ] {
        let rust = evaluate(&snap);
        let report_json = serde_json::to_string(&rust).expect("report json");
        let snap_json = encode_en1994_snapshot_json(&snap);
        let mut child = semio_s_plugin_norm_compliance::python_oracle::command()
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
    fn norm_sig(report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<(String, String, i64, i64, i64)> {
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

    fn walk(value: &semio_framework_value::DslValue, path: &str, leaves: &mut Vec<(String, semio_framework_value::DslValue)>) {
        match value {
            semio_framework_value::DslValue::Object(map) => {
                for (k, v) in map.iter() {
                    let next = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                    walk(v, &next, leaves);
                }
            }
            semio_framework_value::DslValue::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    let id = match item {
                        semio_framework_value::DslValue::Object(map) => map
                            .iter()
                            .find(|(k, _)| k == "id")
                            .and_then(|(_, v)| match v {
                                semio_framework_value::DslValue::String(s) => Some(s.clone()),
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

    fn candidates(original: &semio_framework_value::DslValue, path: &str) -> Vec<semio_framework_value::DslValue> {
        match original {
            semio_framework_value::DslValue::Number(n) => {
                let v = n.as_f64();
                let leaf = leaf_name(path);
                if leaf.starts_with("delta") || leaf.ends_with("KPa") {
                    let base = if v.abs() < 1.0 { 50e6 } else { v };
                    return vec![semio_framework_value::DslValue::float(base * 3.0), semio_framework_value::DslValue::float(base * 0.1)];
                }
                let base = if v.abs() < 1e-12 { 1.0 } else { v };
                vec![
                    if n.is_integer() {
                        semio_framework_value::DslValue::uint((base * 3.0).max(1.0).round() as u64)
                    } else {
                        semio_framework_value::DslValue::float(base * 3.0)
                    },
                    if n.is_integer() {
                        semio_framework_value::DslValue::uint((base * 0.25).max(1.0).round() as u64)
                    } else {
                        semio_framework_value::DslValue::float(base * 0.25)
                    },
                ]
            }
            semio_framework_value::DslValue::Bool(v) => vec![semio_framework_value::DslValue::Bool(!*v)],
            semio_framework_value::DslValue::String(s) => {
                if leaf_name(path) == "annex" {
                    return ["En", "De"]
                        .into_iter()
                        .filter(|c| *c != s.as_str())
                        .map(|c| semio_framework_value::DslValue::String((*c).into()))
                        .collect();
                }
                let meta = en1994_field_meta(path);
                if let Some(choices) = meta.and_then(|m| m.choices()) {
                    choices
                        .iter()
                        .map(|c| c.value)
                        .filter(|c| *c != s.as_str())
                        .map(|c| semio_framework_value::DslValue::String(c.to_string()))
                        .collect()
                } else if leaf_name(path) == "category" {
                    ["A", "B", "C", "E", "self_steel", "finishes", "flm3"]
                        .into_iter()
                        .filter(|c| *c != s.as_str())
                        .map(|c| semio_framework_value::DslValue::String(c.into()))
                        .collect()
                } else if s == "HEB300" || s.starts_with("HEB") {
                    vec![semio_framework_value::DslValue::String("HEB400".into()), semio_framework_value::DslValue::String("CUSTOM-PLATE".into())]
                } else if s == "CUSTOM-PLATE" {
                    vec![semio_framework_value::DslValue::String("HEB300".into())]
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
    let failing_beam = semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl(&example_dsl("composite-floor-beam-failing")).expect("failing beam");
    let mut continuous = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    continuous.beams[0].support = semio_s_artifact_norm_en1994_model::SupportCondition::EndSpan;
    continuous.beams[0].actions.push(semio_s_artifact_norm_en1994_model::CharacteristicAction::point_force("P1", semio_s_artifact_norm_en1994_model::ActionStage::Composite, semio_s_artifact_norm_en1994_model::ActionKind::Imposed, "B", 50e3));
    let mut encased = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    encased.fire_rating = semio_s_artifact_norm_en1994_model::FireRating::R60;
    {
        let column = &mut encased.columns[0];
        column.kind = semio_s_artifact_norm_en1994_model::ColumnKind::Encased;
        column.outer_size_m = 0.4;
        column.reinforcement_as_m2 = 4.0 * 314.16e-6;
        column.section_height_m = 0.2;
        column.section_width_m = 0.2;
        column.section_tw_m = 0.0085;
        column.section_tf_m = 0.013;
        column.rebar_cover_m = 0.05;
    }
    let mut detailed = En1994Snapshot::bridge_girder();
    {
        let beam = &mut detailed.beams[0];
        beam.stud_transverse_spacing_m = 0.1;
        beam.stud_edge_distance_m = 0.03;
        beam.transverse_as_top_m2_per_m = 5e-4;
        beam.end_plate_width_m = 0.3;
        beam.end_plate_height_m = 0.2;
        beam.end_zone_length_m = 0.6;
    }
    let scopes: [(&str, En1994Snapshot, Pred); 9] = [
        (
            "default-building",
            En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De),
            |p| {
                let leaf = leaf_name(p);
                if catalogue_steel_geom(p) {
                    return false;
                }
                if p.contains("beams[") && leaf == "reinforcementProduct" {
                    return false;
                }
                if matches!(leaf, "ltbLengthM" | "nCycles" | "fatigueDetail" | "annex" | "adjacentSpanM" | "patchBearingLengthM" | "bridgeLanes" | "crackedLengthFraction" | "sectionHeightM" | "sectionWidthM" | "sectionTwM" | "sectionTfM" | "rebarCoverM" | "studTransverseSpacingM" | "studEdgeDistanceM" | "transverseAsTopM2PerM" | "endPlateWidthM" | "endPlateHeightM" | "endZoneLengthM") {
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
                if matches!(leaf, "annex" | "structureKind" | "nCycles" | "fatigueDetail" | "bridgeLanes") {
                    return true;
                }
                if p.contains("[id=FLM3") && matches!(leaf, "deltaSigmaKPa" | "deltaTauKPa" | "kind" | "fKN" | "qAreaPa") {
                    return true;
                }
                false
            },
        ),
        ("bridge-girder-detail", detailed, |p| matches!(leaf_name(p), "studTransverseSpacingM" | "studEdgeDistanceM" | "transverseAsTopM2PerM" | "endPlateWidthM" | "endPlateHeightM" | "endZoneLengthM")),
        ("continuous-cracked", continuous, |p| matches!(leaf_name(p), "crackedLengthFraction" | "adjacentSpanM" | "patchBearingLengthM")),
        ("encased-section", encased, |p| matches!(leaf_name(p), "sectionHeightM" | "sectionWidthM" | "sectionTwM" | "sectionTfM" | "rebarCoverM")),
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
        walk(&semio_framework_value::ToValue::to_value(&base), "", &mut leaves);
        let scoped: Vec<_> = leaves.into_iter().filter(|(p, _)| pred(p)).collect();
        assert!(!scoped.is_empty(), "{label}: expected scoped leaves");
        for (path, original) in scoped {
            let mut changed = false;
            for mutated in candidates(&original, &path) {
                if mutated == original {
                    continue;
                }
                let mut trial_tree = semio_framework_value::ToValue::to_value(&base);
                if set_value_at_path(&mut trial_tree, &path, mutated).is_err() {
                    continue;
                }
                let Ok(trial): Result<En1994Snapshot, _> = semio_framework_value::FromValue::from_value(trial_tree) else {
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

    {
        let base = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
        let base_sig = norm_sig(&evaluate(&base));
        let mut leaves = Vec::new();
        walk(&semio_framework_value::ToValue::to_value(&base), "", &mut leaves);
        for (path, original) in leaves {
            if !(path.contains("columns[") && matches!(leaf_name(&path), "mKNm" | "nKN" | "kind" | "wallThicknessM" | "outerSizeM" | "steelFYPa" | "concreteFCkPa" | "bucklingCurve")) {
                continue;
            }
            let mut changed = false;
            for mutated in candidates(&original, &path) {
                if mutated == original { continue; }
                let mut trial_tree = semio_framework_value::ToValue::to_value(&base);
                if set_value_at_path(&mut trial_tree, &path, mutated).is_err() { continue; }
                let Ok(trial): Result<En1994Snapshot, _> = semio_framework_value::FromValue::from_value(trial_tree) else { continue; };
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
    let encoded = serde_json::to_value(En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)).expect("json");
    let beams = encoded.get("beams").and_then(|v| v.as_array()).expect("beams");
    assert!(!beams.is_empty());
    let beam = &beams[0];
    for key in ["spanM", "spacingM", "steel", "actions", "sheeting"] {
        assert!(beam.get(key).is_some(), "rust beam missing {key}");
    }
    assert!(beam.get("studs").is_none(), "the studs are rows of the top-level collection");
    assert!(encoded.get("studs").and_then(|v| v.as_array()).is_some_and(|rows| !rows.is_empty() && rows[0].get("beamId").is_some()));
    assert!(beam.get("actions").unwrap().as_array().unwrap()[0].get("qAreaPa").is_some());
    assert!(beam.get("actions").unwrap().as_array().unwrap()[0].get("qLineNPerM").is_none());
}


#[test]
fn beam_checks_emit_stations_along_the_whole_length_and_obey_the_station_law() {
    let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    document.beams[0].support = semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span;
    let report = evaluate(&document);
    for prefix in ["en1994.6.2.1.3.mrd.", "en1994.6.2.2.vpl.", "en1994.6.6.3.1.prd.", "en1994.6.2.1.2.hogging."] {
        let check = report.checks.iter().find(|c| c.id.starts_with(prefix)).unwrap_or_else(|| panic!("{prefix}"));
        assert_eq!(check.stations.len(), 17, "{prefix}");
        assert!(check.stations_consistent(), "{prefix}");
    }
    let sagging = report.checks.iter().find(|c| c.id.starts_with("en1994.6.2.1.3.mrd.")).expect("sagging");
    let peak = sagging.stations.iter().max_by(|a, b| a.utilization.total_cmp(&b.utilization)).expect("station");
    assert!((peak.at[0] - 3.0 / 16.0).abs() < 1e-12 || (peak.at[0] - 13.0 / 16.0).abs() < 1e-12);
    let hogging = report.checks.iter().find(|c| c.id.starts_with("en1994.6.2.1.2.hogging.")).expect("hogging");
    assert!((hogging.stations[8].utilization - hogging.utilization).abs() < 1e-12);
    assert_eq!(hogging.stations[0].utilization, 0.0);
}

/// 🔮️ EN 1994 third-party oracle cases: the shared harness recomputes every blessed case in `🧪️tests/🧫️fixtures/🔮️oracle-cases` with the registered libraries and fails on any mismatch beyond the tolerance profile or between libraries.
#[test]
fn third_party_libraries_reproduce_the_blessed_oracle_cases() {
    let verified = semio_s_plugin_norm_compliance::python_oracle::verify_cases_above(env!("CARGO_MANIFEST_DIR"));
    assert!(verified >= 11, "expected at least 11 verified oracle cases, got {verified}");
}

fn end_span_building() -> En1994Snapshot {
    let mut doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.beams[0].support = semio_s_artifact_norm_en1994_model::SupportCondition::EndSpan;
    doc
}

#[test]
fn continuous_spans_report_the_cracked_analysis_and_the_sagging_interaction() {
    let mut doc = end_span_building();
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.starts_with("en1994.5.4.2.3.cracked.") && c.status != CheckStatus::NotApplicable));
    assert!(report.checks.iter().any(|c| c.id.starts_with("en1994.6.2.2.4.vm-sagging.")));
    doc.beams[0].cracked_length_fraction = 0.0;
    let off = evaluate(&doc);
    assert!(off.checks.iter().any(|c| c.id.starts_with("en1994.5.4.2.3.cracked.") && c.status == CheckStatus::NotApplicable));
}

#[test]
fn cracked_stiffness_over_the_support_regions_raises_the_continuous_deflection() {
    let deflection = |d: &En1994Snapshot| evaluate(d).checks.iter().find(|c| c.id.starts_with("en1994.7.3.1.deflection.beam")).map(|c| c.computed.value).expect("deflection check");
    let mut doc = end_span_building();
    let cracked = deflection(&doc);
    doc.beams[0].cracked_length_fraction = 0.0;
    assert!(cracked > deflection(&doc));
}

#[test]
fn end_interior_and_cantilever_spans_have_distinct_statics_and_stations() {
    let moment = |support: semio_s_artifact_norm_en1994_model::SupportCondition| {
        let mut doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
        doc.beams[0].support = support;
        evaluate(&doc).checks.iter().find(|c| c.id.starts_with("en1994.6.2.1.3.mrd.")).map(|c| c.computed.value).expect("mrd")
    };
    let (end, interior, cantilever) = (moment(semio_s_artifact_norm_en1994_model::SupportCondition::EndSpan), moment(semio_s_artifact_norm_en1994_model::SupportCondition::InteriorSpan), moment(semio_s_artifact_norm_en1994_model::SupportCondition::Cantilever));
    assert!(end != interior && interior != cantilever && end != cantilever, "{end} {interior} {cantilever}");
}

#[test]
fn encased_columns_report_the_fire_model_and_the_untranscribed_tables() {
    let mut doc = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.fire_rating = semio_s_artifact_norm_en1994_model::FireRating::R60;
    let column = &mut doc.columns[0];
    column.kind = semio_s_artifact_norm_en1994_model::ColumnKind::Encased;
    column.outer_size_m = 0.4;
    column.reinforcement_as_m2 = 4.0 * 314.16e-6;
    column.section_height_m = 0.2;
    column.section_width_m = 0.2;
    column.section_tw_m = 0.0085;
    column.section_tf_m = 0.013;
    column.rebar_cover_m = 0.05;
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.starts_with("en1994.1-2.4.3.5.column.") && c.status != CheckStatus::NotApplicable));
    assert!(report.checks.iter().any(|c| c.id.starts_with("en1994.1-2.4.2.table-column.") && c.status == CheckStatus::NotApplicable));
}

#[test]
fn bridge_girders_report_shear_lag_and_the_cracked_section_reinforcement_range() {
    let mut doc = En1994Snapshot::bridge_girder();
    doc.fatigue_detail = semio_s_artifact_norm_en1994_model::FatigueDetail::Reinforcement;
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.starts_with("en1994.2.5.4.1.shear-lag.")));
    let rebar = report.checks.iter().find(|c| c.id.starts_with("en1994.2.6.8.2.rebar.")).expect("rebar fatigue check");
    assert!(rebar.ndps.iter().any(|n| n == "ndp.en1994.2.fatigue-tension-stiffening"));
    assert!(rebar.utilization.is_finite() && rebar.utilization > 0.0);
}

#[test]
fn bridge_girder_details_are_checked_with_remedies_and_skipped_while_undeclared() {
    let undeclared = evaluate(&En1994Snapshot::bridge_girder());
    for prefix in ["en1994.2.6.6.5.edge.", "en1994.2.6.6.5.cross-spacing.", "en1994.2.6.6.5.end-plate."] {
        assert!(undeclared.checks.iter().any(|c| c.id.starts_with(prefix) && c.status == CheckStatus::NotApplicable), "{prefix}");
    }
    let mut doc = En1994Snapshot::bridge_girder();
    {
        let beam = &mut doc.beams[0];
        beam.stud_edge_distance_m = 0.01;
        beam.stud_transverse_spacing_m = 0.03;
        beam.transverse_as_top_m2_per_m = 0.0;
        beam.transverse_as_m2_per_m = 1e-5;
        beam.end_plate_width_m = 0.05;
        beam.end_plate_height_m = 0.05;
        beam.end_zone_length_m = 1.5;
    }
    let report = evaluate(&doc);
    for prefix in ["en1994.2.6.6.5.edge.", "en1994.2.6.6.5.cross-spacing.", "en1994.2.6.6.6.transverse-min.", "en1994.2.6.6.5.end-plate."] {
        let check = report.checks.iter().find(|c| c.id.starts_with(prefix)).expect(prefix);
        assert_eq!(check.status, CheckStatus::Fail, "{prefix}");
        assert!(!check.remedies.is_empty(), "{prefix}");
        assert!(!check.ndps.is_empty(), "{prefix}");
    }
    let mut ok = doc.clone();
    {
        let beam = &mut ok.beams[0];
        beam.stud_edge_distance_m = 0.04;
        beam.stud_transverse_spacing_m = 0.2;
        beam.transverse_as_top_m2_per_m = 1e-3;
        beam.end_plate_width_m = 0.6;
        beam.end_plate_height_m = 0.25;
        beam.end_zone_length_m = 0.3;
    }
    let report = evaluate(&ok);
    for prefix in ["en1994.2.6.6.5.edge.", "en1994.2.6.6.5.cross-spacing.", "en1994.2.6.6.6.transverse-min.", "en1994.2.6.6.5.end-plate."] {
        let check = report.checks.iter().find(|c| c.id.starts_with(prefix)).expect(prefix);
        assert_ne!(check.status, CheckStatus::Fail, "{prefix}");
    }
}

#[test]
fn reliability_class_of_the_document_reaches_the_evaluation() {
    let rc2 = semio_s_artifact_norm_en1994_model::En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let rc3 = semio_s_artifact_norm_en1994_model::En1994Snapshot { reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass::RC3, ..rc2.clone() };
    let load_effect = |document: &semio_s_artifact_norm_en1994_model::En1994Snapshot| evaluate(document).checks.iter().map(|c| c.utilization).sum::<f64>();
    assert!(load_effect(&rc3) > load_effect(&rc2));
}

fn oracle_json(script: PathBuf, args: &[&str]) -> serde_json::Value {
    let output = semio_s_plugin_norm_compliance::python_oracle::command().arg(script).args(args).output().expect("oracle spawn");
    assert!(output.status.success(), "oracle stderr={}", String::from_utf8_lossy(&output.stderr));
    serde_json::from_slice(&output.stdout).expect("oracle json")
}

#[test]
fn third_party_cross_check_tables_agree() {
    let oracles = family_any_dir().join("🔮️oracles");
    let table = oracle_json(oracles.join("🎯️third-party-crosscheck").join("🐍️.py"), &[]);
    let rows = table["rows"].as_array().expect("rows");
    assert!(rows.len() >= 64, "rows={}", rows.len());
    for row in rows {
        let rel = row["rel"].as_f64().expect("rel");
        assert!(rel <= 5e-2, "{} rel={rel}", row["name"]);
    }
    for key in table["specRows"].as_object().expect("specRows").keys() {
        assert!(rows.iter().any(|row| row["name"].as_str().is_some_and(|name| name.starts_with(key.as_str()))), "no cross-check row for {key}");
    }
    let compliance = oracle_json(oracles.join("⚖️compliance").join("🐍️.py"), &["--third-party"]);
    assert!(compliance["worst"].as_f64().expect("worst") <= 5e-3, "worst={}", compliance["worst"]);
    let slabs = oracle_json(oracles.join("🚦️check-outcome").join("🧱️slabs").join("🐍️.py"), &[]);
    for (name, deviation) in slabs["deviations"].as_object().expect("deviations") {
        assert!(deviation.as_f64().expect("deviation") <= 1e-9, "{name} deviation={deviation}");
    }
}
