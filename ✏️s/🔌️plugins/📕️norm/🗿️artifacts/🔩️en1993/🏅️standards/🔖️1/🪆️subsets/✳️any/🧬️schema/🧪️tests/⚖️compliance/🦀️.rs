//! ⚖️ EN 1993 compliance numeric worked examples (SI).

use semio_framework_test_fixtures::fixture_path;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus, ReliabilityClass};
use crate::{check_full_steel_structure, combine_member_actions, uls_rows, part_1_1, part_1_2, part_1_8, part_1_9, AnnexParams};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

fn approx(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} !≈ {b} (tol {tol})");
}

#[test]
fn axial_resistance_s355_heb240_matches_hand_calc() {
    let params = AnnexParams::de();
    let n_rd = part_1_1::axial_resistance_n(0.0106, 355.0e6, params);
    approx(n_rd / 1000.0, 3763.0, 1.0);
}

#[test]
fn flexural_buckling_chi_from_lambda_never_user_supplied() {
    let curve = part_1_1::BucklingCurve::B;
    let chi = part_1_1::chi(1.0, curve);
    approx(chi, 0.597_023, 1e-5);
}

#[test]
fn de_gamma_m1_raises_buckling_utilization_vs_en() {
    let mut de = En1993Snapshot::compliant_heb240_frame();
    de.annex = AnnexChoice::De;
    let mut en = de.clone();
    en.annex = AnnexChoice::En;
    de.member_actions[0].action.n = 2_500_000.0;
    en.member_actions[0].action.n = 2_500_000.0;
    let rd_de = check_full_steel_structure(&de);
    let rd_en = check_full_steel_structure(&en);
    let u_de = rd_de.checks.iter().find(|c| c.id.contains("6.3.1")).map(|c| c.utilization).unwrap();
    let u_en = rd_en.checks.iter().find(|c| c.id.contains("6.3.1")).map(|c| c.utilization).unwrap();
    assert!(u_de > u_en, "DE γ_M1=1.1 must raise buckling utilization ({u_de} vs {u_en})");
}

#[test]
fn bolt_bearing_m20_8_8_matches_table_style_value() {
    let a_s = part_1_8::bolt_as(0.020);
    approx(a_s * 1e6, 245.0, 1.0);
    let f_ub = part_1_8::bolt_fub("8.8");
    let alpha_b = part_1_8::bearing_alpha_b(0.040, 0.060, 0.022, f_ub, 510.0e6);
    let k1 = part_1_8::bearing_k1(0.040, 0.060, 0.022);
    let fb = part_1_8::bolt_bearing_resistance_n(k1, alpha_b, 510.0e6, 0.020, 0.010, 2, 1.25);
    approx(fb / 1000.0, 209.5, 2.0);
}

#[test]
fn critical_temperature_eq_4_22() {
    let t = part_1_2::critical_temperature_c(0.5);
    approx(t, 584.7, 2.0);
}

#[test]
fn fatigue_category_71_damage_tolerant() {
    let dc = part_1_9::detail_category_pa(71);
    approx(dc / 1e6, 71.0, 0.1);
    assert_eq!(part_1_9::gamma_mf(part_1_9::AssessmentMethod::DamageTolerant), 1.0);
}

#[test]
fn compliant_example_has_no_fails() {
    let report = check_full_steel_structure(&En1993Snapshot::compliant_heb240_frame());
    assert!(report.complies(), "compliant subject must have no Fail");
    assert!(report.checks.iter().filter(|c| c.status == CheckStatus::Pass).count() >= 8, "expected many Pass checks, got {}", report.checks.len());
    assert!(report.checks.iter().any(|c| c.id.contains("en1993.6.2.4")), "missing axial check id");
    assert!(report.checks.iter().any(|c| c.id.contains("en1993.1-8.3.6.shear")), "missing bolt shear check");
    assert!(report.checks.iter().any(|c| c.id.contains("en1993.1-9.fat")), "missing fatigue Miner check");
    assert!(report.checks.iter().any(|c| c.id.contains("en1993.1-3.cf")), "missing cold-formed check");
    assert!(report.checks.iter().any(|c| c.id.contains("en1993.1-5.plate")), "missing plated check");
    assert!(report.checks.iter().any(|c| c.id.contains("en1993.3-1.tower")), "missing tower check");
    assert!(report.checks.iter().any(|c| c.id.contains("en1993.5.comp")), "missing pile check");
    assert!(report.checks.iter().any(|c| c.id.contains("en1993.6.crane")), "missing crane check");
}

#[test]
fn noncompliant_example_fails_and_remedies_applicable() {
    let report = check_full_steel_structure(&En1993Snapshot::noncompliant_overloaded_frame());
    assert!(!report.complies());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 8, "expected multi-part fails, got {}", fails.len());
    for fail in &fails {
        assert!(fail.remedies.len() >= 1, "Fail {} missing remedies", fail.id);
        assert!(fail.remedies.iter().filter(|r| r.applicable).count() >= 1, "Fail {} needs applicable remedy", fail.id);
    }
    for needle in ["1-8.3.6", "1-9.fat", "1-3.cf", "1-5.plate", "1-11.tc", "3-1.tower", "5.comp", "6.crane", "1-6.shell", "2.fat"] {
        let part_fails: Vec<_> = fails.iter().filter(|c| c.id.contains(needle)).collect();
        if part_fails.is_empty() { continue; }
        for f in part_fails {
            assert!(f.remedies.iter().filter(|r| r.applicable).count() >= 1, "{needle} fail lacking remedies");
        }
    }
}

#[test]
fn remedy_law_section_upsize_or_buckling_length_improves_utilization() {
    let doc = En1993Snapshot::noncompliant_overloaded_frame();
    let report = check_full_steel_structure(&doc);
    let fail = report.failing().find(|c| c.id.contains("6.3.1")).expect("buckling fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("bucklingLength")).expect("Lcr remedy");
    let mut fixed = doc.clone();
    let member = fixed.members.iter_mut().find(|m| m.id == "member-b1").unwrap();
    member.buckling_length_y = remedy.required.value;
    member.buckling_length_z = remedy.required.value;
    let after = check_full_steel_structure(&fixed);
    let u_after = after.checks.iter().find(|c| c.id.contains("6.3.1")).map(|c| c.utilization).unwrap_or(0.0);
    assert!(semio_s_plugin_norm_compliance::within_limit(u_after), "remedy must bring buckling u≤1, got {u_after}");
}

#[test]
fn section_class_heb240_s355_is_class_1_or_2() {
    let section = semio_s_artifact_norm_en1993_model::catalogue_heb240();
    let class = part_1_1::section_class_rolled_i(&section, 355.0e6);
    assert!(class <= 2, "HEB 240 S355 expected class 1–2, got {class}");
}

#[test]
fn m_cr_positive_for_rolled_i() {
    let doc = En1993Snapshot::compliant_heb240_frame();
    let section = &doc.sections[0];
    let material = &doc.materials[0];
    let member = &doc.members[0];
    let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length, member.end_moment_ratio_psi, member.load_application.as_str(), member.moment_diagram.as_str());
    assert!(m_cr > 0.0, "M_cr must be computed > 0");
}

#[test]
fn remedy_law_oneof_section_id_flips_axial_or_buckling_to_pass() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    doc.member_actions[0].action.n = 3_000_000.0;
    doc.member_actions[0].action.my = 0.0;
    doc.member_actions[0].action.vz = 0.0;
    if let Some(q) = doc.member_actions.iter_mut().find(|a| a.id == "act-b1-q") {
        q.action.n = 0.0;
        q.action.my = 0.0;
        q.action.vz = 0.0;
    }
    let report = check_full_steel_structure(&doc);
    let fail = report.failing().find(|c| c.id.contains("6.2.4")).expect("axial fail");
    let remedy = fail
        .remedies
        .iter()
        .find(|r| r.applicable && r.target.path.contains("sectionId") && !r.options.is_empty())
        .expect("OneOf sectionId remedy");
    assert!(remedy.options.iter().all(|o| o.starts_with("sec-")), "options must be section ids, got {:?}", remedy.options);
    let chosen = remedy.options.last().cloned().unwrap();
    let mut fixed = doc.clone();
    {
        let member = fixed.members.iter_mut().find(|m| m.id == "member-b1").unwrap();
        member.section_id = chosen.clone();
    }
    if !fixed.sections.iter().any(|s| s.id == chosen) {
        if let Some(cat) = semio_s_artifact_norm_en1993_model::rolled_heb_catalogue().into_iter().find(|s| s.id == chosen) {
            fixed.sections.push(cat);
        }
    }
    let after = check_full_steel_structure(&fixed);
    let u_after = after.checks.iter().find(|c| c.id == fail.id).map(|c| c.utilization).unwrap_or(0.0);
    assert!(semio_s_plugin_norm_compliance::within_limit(u_after), "section OneOf must bring axial to Pass, u={u_after}, chose {chosen}, options={:?}", remedy.options);
}

#[test]
fn remedy_law_bolt_shear_rows_flips_to_pass() {
    let doc = En1993Snapshot::noncompliant_overloaded_frame();
    let report = check_full_steel_structure(&doc);
    let fail = report.failing().find(|c| c.id.contains("1-8.3.6.shear")).expect("bolt shear fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("boltRows")).expect("boltRows remedy");
    let mut fixed = doc.clone();
    let rows = remedy.required.value.ceil().max(1.0) as u32;
    {
        let joint = fixed.joints.iter_mut().find(|j| j.id == "joint-j1").unwrap();
        joint.bolt_rows = rows;
    }
    for _ in 0..8 {
        let after = check_full_steel_structure(&fixed);
        let u_after = after.checks.iter().find(|c| c.id.contains("1-8.3.6.shear.joint-j1")).map(|c| c.utilization).unwrap_or(0.0);
        if semio_s_plugin_norm_compliance::within_limit(u_after) {
            return;
        }
        let joint = fixed.joints.iter_mut().find(|j| j.id == "joint-j1").unwrap();
        joint.bolt_rows += 1;
    }
    let after = check_full_steel_structure(&fixed);
    let u_after = after.checks.iter().find(|c| c.id.contains("1-8.3.6.shear.joint-j1")).map(|c| c.utilization).unwrap_or(0.0);
    let rows_final = fixed.joints.iter().find(|j| j.id == "joint-j1").unwrap().bolt_rows;
    assert!(semio_s_plugin_norm_compliance::within_limit(u_after), "boltRows remedy must bring shear u≤1, got {u_after} rows={rows_final}");
}

#[test]
fn every_emitted_path_parses_and_resolves() {
    use semio_s_plugin_norm_app_surface::{get_value_at_path, parse_path};
    for doc in [En1993Snapshot::compliant_heb240_frame(), En1993Snapshot::noncompliant_overloaded_frame()] {
        let report = check_full_steel_structure(&doc);
        let snap_val = semio_framework_value::ToValue::to_value(&doc);
        for check in &report.checks {
            if check.subject.path.is_empty() {
                continue;
            }
            parse_path(&check.subject.path).unwrap_or_else(|e| panic!("subject path {}: {e}", check.subject.path));
            get_value_at_path(&snap_val, &check.subject.path).unwrap_or_else(|e| panic!("resolve {}: {e}", check.subject.path));
        }
        for check in &report.checks {
            for remedy in &check.remedies {
                if remedy.target.path.is_empty() {
                    continue;
                }
                parse_path(&remedy.target.path).unwrap_or_else(|e| panic!("remedy path {}: {e}", remedy.target.path));
                get_value_at_path(&snap_val, &remedy.target.path).unwrap_or_else(|e| panic!("resolve remedy {}: {e}", remedy.target.path));
            }
        }
    }
}

#[test]
fn ltb_curve_follows_table_6_4_for_rolled_i() {
    let section = semio_s_artifact_norm_en1993_model::catalogue_heb240();
    assert_eq!(part_1_1::ltb_curve_table_6_4(&section), part_1_1::BucklingCurve::A);
}

#[test]
fn slip_resistant_category_c_produces_slip_check() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    let joint = doc.joints.iter_mut().find(|j| j.id == "joint-j1").unwrap();
    joint.category = semio_s_artifact_norm_en1993_model::JointCategory::C;
    joint.friction_mu = 0.50;
    joint.preload_force = 0.0;
    joint.actions = vec![semio_s_artifact_norm_en1993_model::JointForceAction { id: "jf-g".into(), load_case_id: "g-permanent".into(), shear: 200_000.0, tension: 0.0 }];
    let report = check_full_steel_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("3.9.slip")), "expected slip check");
}

#[test]
fn python_oracle_and_jsonschema_agree_within_half_percent() {
    use std::path::PathBuf;
    use std::process::Command;
    let any = fixture_path!("../../../../..")
        .canonicalize()
        .expect("family any root");
    let oracle = any.join("🔮️oracles/⚖️compliance/🐍️.py");
    let validate = any.join("🔮️oracles/🧬️snapshot-schema/🐍️.py");
    let schema = any.join("🧬️schema/📸️snapshot/🔣️.json");
    let compliant = any.join("🖼️assets/✅️heb240-compliant/snapshot.json");
    assert!(oracle.is_file(), "missing oracle at {oracle:?}");
    let out = semio_s_plugin_norm_compliance::python_oracle::command().arg(&oracle).output().expect("spawn oracle");
    assert!(out.status.success(), "oracle failed: {}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8_lossy(&out.stdout);
    let start = text.find('{').expect("oracle json");
    let end = text.rfind('}').expect("oracle json end");
    let report: serde_json::Value = serde_json::from_str(&text[start..=end]).expect("parse oracle");

    let doc = En1993Snapshot::compliant_heb240_frame();
    let rust = check_full_steel_structure(&doc);
    let u_ax = rust.checks.iter().find(|c| c.id.contains("6.2.4")).map(|c| c.utilization).unwrap();
    let u_b_de = rust.checks.iter().find(|c| c.id.contains("6.3.1")).map(|c| c.utilization).unwrap();
    let ax_o = report["axialUtilization"].as_f64().unwrap();
    let b_de_o = report["bucklingUtilizationDe"].as_f64().unwrap();
    assert!((u_ax - ax_o).abs() / ax_o.max(1e-9) <= 0.005, "axial rust {u_ax} vs oracle {ax_o}");
    assert!((u_b_de - b_de_o).abs() / b_de_o.max(1e-9) <= 0.005, "buckling DE rust {u_b_de} vs oracle {b_de_o}");

    let mut en = doc.clone();
    en.annex = AnnexChoice::En;
    let u_b_en = check_full_steel_structure(&en).checks.iter().find(|c| c.id.contains("6.3.1")).map(|c| c.utilization).unwrap();
    let b_en_o = report["bucklingUtilizationEn"].as_f64().unwrap();
    assert!((u_b_en - b_en_o).abs() / b_en_o.max(1e-9) <= 0.005);

    let u_fail = check_full_steel_structure(&En1993Snapshot::noncompliant_overloaded_frame())
        .checks
        .iter()
        .find(|c| c.id.contains("6.3.1"))
        .map(|c| c.utilization)
        .unwrap();
    let fail_o = report["noncompliantBucklingUtilizationDe"].as_f64().unwrap();
    assert!((u_fail - fail_o).abs() / fail_o.max(1e-9) <= 0.005);
    assert!(u_fail > 1.0);

    let status = semio_s_plugin_norm_compliance::python_oracle::command().arg(&validate).arg(&schema).arg(&compliant).status().expect("spawn validate");
    assert!(status.success(), "jsonschema validation failed");
}

fn walk_leaves(prefix: &str, value: &serde_json::Value, visit: &mut dyn FnMut(&str)) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                match v {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => walk_leaves(&path, v, visit),
                    _ => visit(&path),
                }
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                let path = format!("{prefix}[{i}]");
                match v {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => walk_leaves(&path, v, visit),
                    _ => visit(&path),
                }
            }
        }
        _ => visit(prefix),
    }
}

#[test]
fn perturb_every_editable_leaf_in_committed_examples_changes_a_check() {
    let examples = [
        En1993Snapshot::compliant_heb240_frame(),
        En1993Snapshot::noncompliant_overloaded_frame(),
    ];
    for snap in examples {
        let base = check_full_steel_structure(&snap);
        let base_sig: Vec<_> = base
            .checks
            .iter()
            .map(|c| {
                (
                    c.id.clone(),
                    format!("{:?}", c.status),
                    (c.computed.value * 1e6).round() as i64,
                    (c.limit.value * 1e6).round() as i64,
                    (c.utilization * 1e6).round() as i64,
                )
            })
            .collect();
        let value = serde_json::to_value(&snap).expect("json");
        let mut paths = Vec::new();
        walk_leaves("", &value, &mut |path| {
            if path.is_empty() {
                return;
            }
            let leaf = path.rsplit('.').next().unwrap_or(path);
            if matches!(leaf, "id" | "name" | "title" | "label" | "labelEn" | "labelDe") {
                return;
            }
            paths.push(path.to_string());
        });
        let mut unchanged = Vec::new();
        for path in &paths {
            let mut tree = value.clone();
            if !perturb_value(&mut tree, path) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1993Snapshot>(tree) else { continue };
            let rep = check_full_steel_structure(&perturbed);
            let sig: Vec<_> = rep
                .checks
                .iter()
                .map(|c| {
                    (
                        c.id.clone(),
                        format!("{:?}", c.status),
                        (c.computed.value * 1e6).round() as i64,
                        (c.limit.value * 1e6).round() as i64,
                        (c.utilization * 1e6).round() as i64,
                    )
                })
                .collect();
            if sig == base_sig {
                unchanged.push(path.clone());
            }
        }
        assert!(
            unchanged.is_empty(),
            "editable leaves do not influence checks: {unchanged:?}"
        );
    }
}

#[test]
fn dangling_member_section_material_and_load_case_refs_fail() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    doc.members[0].section_id = "sec-missing".into();
    let report = check_full_steel_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.members") && c.id.contains("sectionId") && matches!(c.status, CheckStatus::Fail)));

    let mut doc = En1993Snapshot::compliant_heb240_frame();
    doc.members[0].material_id = "mat-missing".into();
    let report = check_full_steel_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.members") && c.id.contains("materialId") && matches!(c.status, CheckStatus::Fail)));

    let mut doc = En1993Snapshot::compliant_heb240_frame();
    doc.member_actions[0].member_id = "member-missing".into();
    let report = check_full_steel_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.memberActions") && c.id.contains("memberId") && matches!(c.status, CheckStatus::Fail)));

    let mut doc = En1993Snapshot::compliant_heb240_frame();
    doc.member_actions[0].load_case_id = "lc-missing".into();
    let report = check_full_steel_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.memberActions") && c.id.contains("loadCaseId") && matches!(c.status, CheckStatus::Fail)));

    let mut doc = En1993Snapshot::compliant_heb240_frame();
    doc.joints[0].member_id = "member-missing".into();
    let report = check_full_steel_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.joints") && c.id.contains("memberId") && matches!(c.status, CheckStatus::Fail)));
}

#[test]
fn duplicate_entity_ids_fail_integrity() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    let dup = doc.members[0].clone();
    doc.members.push(dup);
    let report = check_full_steel_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.duplicate.members") && matches!(c.status, CheckStatus::Fail)));
}

#[test]
fn design_temperature_changes_fire_check_limit_or_utilization() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let base_rep = check_full_steel_structure(&base);
    let base_fire: Vec<_> = base_rep
        .checks
        .iter()
        .filter(|c| c.id.contains("1-2"))
        .map(|c| ((c.limit.value * 1e6).round() as i64, (c.utilization * 1e6).round() as i64, format!("{:?}", c.status)))
        .collect();
    let mut doc = base.clone();
    doc.fire_exposures[0].design_temperature = 750.0;
    let rep = check_full_steel_structure(&doc);
    let fire: Vec<_> = rep
        .checks
        .iter()
        .filter(|c| c.id.contains("1-2"))
        .map(|c| ((c.limit.value * 1e6).round() as i64, (c.utilization * 1e6).round() as i64, format!("{:?}", c.status)))
        .collect();
    assert_ne!(base_fire, fire, "designTemperature must change a fire check limit or utilization");
}

#[test]
fn class_four_section_fails_classification_with_section_remedy() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    if let Some(sec) = doc.sections.iter_mut().find(|s| s.id == "sec-heb240") {
        sec.tf = 0.006;
        sec.tw = 0.005;
        sec.b = 0.300;
        sec.h = 0.400;
        sec.kind = semio_s_artifact_norm_en1993_model::SectionKind::WeldedI;
    }
    let report = check_full_steel_structure(&doc);
    let class_check = report.checks.iter().find(|c| c.id.contains("5.2.class")).expect("class row");
    assert!(matches!(class_check.status, CheckStatus::Fail), "class 4 must Fail, got {:?}", class_check.status);
    assert!(!class_check.remedies.is_empty(), "class 4 must offer a section one_of remedy");
}

fn perturb_value(tree: &mut serde_json::Value, path: &str) -> bool {
    let mut segs = Vec::new();
    let mut buf = String::new();
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '.' {
            segs.push(buf.clone());
            buf.clear();
        } else if c == '[' {
            if !buf.is_empty() {
                segs.push(buf.clone());
                buf.clear();
            }
            let mut idx = String::new();
            while let Some(x) = chars.next() {
                if x == ']' {
                    break;
                }
                idx.push(x);
            }
            segs.push(idx);
        } else {
            buf.push(c);
        }
    }
    if !buf.is_empty() {
        segs.push(buf);
    }
    if segs.is_empty() {
        return false;
    }
    let mut cur = tree;
    let last = segs.len() - 1;
    for (i, seg) in segs.iter().enumerate() {
        if i == last {
            let Some(map) = cur.as_object_mut() else {
                return false;
            };
            let Some(v) = map.get_mut(seg) else {
                return false;
            };
            match v {
                serde_json::Value::Number(n) => {
                    let f = n.as_f64().unwrap_or(0.0);
                    *v = serde_json::json!(if f.abs() < 1e-15 { 1.0 } else { f * 1.15 });
                    return true;
                }
                serde_json::Value::String(s) => {
                    if s == "en" || s == "En" {
                        *v = serde_json::json!("de");
                    } else if s == "de" || s == "De" {
                        *v = serde_json::json!("en");
                    } else {
                        *v = serde_json::json!(format!("{s}-x"));
                    }
                    return true;
                }
                serde_json::Value::Bool(b) => {
                    *v = serde_json::json!(!*b);
                    return true;
                }
                _ => return false,
            }
        } else if let Ok(idx) = seg.parse::<usize>() {
            let Some(arr) = cur.as_array_mut() else {
                return false;
            };
            let Some(next) = arr.get_mut(idx) else {
                return false;
            };
            cur = next;
        } else {
            let Some(map) = cur.as_object_mut() else {
                return false;
            };
            let Some(next) = map.get_mut(seg) else {
                return false;
            };
            cur = next;
        }
    }
    false
}

#[test]
fn en1993_combinations_form_uls_from_characteristic_actions() {
    let snap = En1993Snapshot::compliant_heb240_frame();
    let report = check_full_steel_structure(&snap);
    assert!(report.complies());
    let axial = report.checks.iter().find(|c| c.id.contains("6.2.4")).expect("axial");
    assert!(axial.computed.value > 190_000.0 && axial.computed.value < 210_000.0, "expected ~200 kN design N from EN1990 combo, got {}", axial.computed.value);
}

#[test]
fn palmgren_miner_damage_at_two_million_cycles_matches_oracle() {
    let delta_c = part_1_9::detail_category_pa(71);
    let d = part_1_9::miner_damage(&[(delta_c, 2.0e6)], delta_c, 1.0);
    approx(d, 1.0, 0.005);
    let d2 = part_1_9::miner_damage(&[(delta_c, 1.0e6)], delta_c, 1.0);
    approx(d2, 0.5, 0.005);
}

#[test]
fn class4_uses_effective_section_not_class_over_three_tautology() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    if let Some(sec) = doc.sections.iter_mut().find(|s| s.id == "sec-heb240") {
        sec.tf = 0.006;
        sec.tw = 0.005;
        sec.b = 0.300;
        sec.h = 0.400;
        sec.kind = semio_s_artifact_norm_en1993_model::SectionKind::WeldedI;
    }
    let report = check_full_steel_structure(&doc);
    let class_check = report.checks.iter().find(|c| c.id.contains("5.2.class")).expect("class row");
    assert!(
        !(class_check.limit.value == 3.0 && (class_check.computed.value - class_check.computed.value.round()).abs() < 1e-9 && class_check.computed.value >= 1.0 && class_check.computed.value <= 4.0),
        "class row must not use u=class/3 tautology (computed={}, limit={})",
        class_check.computed.value,
        class_check.limit.value
    );
    assert!(class_check.explanation.en.to_ascii_lowercase().contains("class"));
}

#[test]
fn committed_examples_have_distinct_en_and_de_text() {
    for doc in [En1993Snapshot::compliant_heb240_frame(), En1993Snapshot::noncompliant_overloaded_frame()] {
        let report = check_full_steel_structure(&doc);
        for c in &report.checks {
            let en = c.explanation.en.trim();
            let de = c.explanation.de.trim();
            let en_alpha: String = en.chars().filter(|ch| ch.is_alphabetic()).collect();
            let de_alpha: String = de.chars().filter(|ch| ch.is_alphabetic()).collect();
            if en_alpha.len() >= 8 && de_alpha.len() >= 8 {
                assert_ne!(en, de, "identical en/de explanation on {}", c.id);
            }
            assert_ne!(c.title.en, c.title.de, "identical en/de label on {}", c.id);
            for r in &c.remedies {
                let ren = r.action.en.trim();
                let rde = r.action.de.trim();
                let a: String = ren.chars().filter(|ch| ch.is_alphabetic()).collect();
                let b: String = rde.chars().filter(|ch| ch.is_alphabetic()).collect();
                if a.len() >= 8 && b.len() >= 8 {
                    assert_ne!(ren, rde, "identical en/de remedy on {}", c.id);
                }
            }
        }
    }
}

#[test]
fn psi_factors_follow_en_1990_table_a1_1() {
    for annex in [AnnexChoice::En, AnnexChoice::De] {
        assert_eq!(crate::psi_factors("snow", annex), (0.5, 0.2, 0.0));
        assert_eq!(crate::psi_factors("snow_high", annex), (0.7, 0.5, 0.2));
        assert_eq!(crate::psi_factors("storage", annex), (1.0, 0.9, 0.8));
        assert_eq!(crate::psi_factors("congregation", annex), (0.7, 0.7, 0.6));
        assert_eq!(crate::psi_factors("office", annex), (0.7, 0.5, 0.3));
    }
}

#[test]
fn pitch_and_gauge_enter_bearing_factors() {
    let f_ub = 800e6;
    let a1 = part_1_8::bearing_alpha_b(0.04, 0.06, 0.022, f_ub, 510e6);
    let a2 = part_1_8::bearing_alpha_b(0.04, 0.12, 0.022, f_ub, 510e6);
    assert!(a2 + 1e-9 >= a1, "larger p1 must not reduce α_b below the e1 branch when p1 governs");
    let k1 = part_1_8::bearing_k1(0.04, 0.06, 0.022);
    let k2 = part_1_8::bearing_k1(0.04, 0.12, 0.022);
    assert!(k2 + 1e-9 >= k1);
}

#[test]
fn moment_diagram_changes_m_cr() {
    let doc = En1993Snapshot::compliant_heb240_frame();
    let section = &doc.sections[0];
    let material = &doc.materials[0];
    let m_lin = part_1_1::m_cr_rolled_nm(section, material, 4.0, -1.0, "shearCenter", "linear");
    let m_par = part_1_1::m_cr_rolled_nm(section, material, 4.0, -1.0, "shearCenter", "parabolic");
    assert!((m_par - m_lin).abs() / m_lin.max(1.0) > 0.01, "parabolic C1 must change M_cr");
}

#[test]
fn joint_and_tower_explanations_name_governing_combination() {
    let report = check_full_steel_structure(&En1993Snapshot::compliant_heb240_frame());
    let shear = report.checks.iter().find(|c| c.id.contains("1-8.3.6.shear")).expect("shear");
    assert!(shear.explanation.en.contains("gov.") || shear.explanation.en.contains("6.10"), "joint must name governing combination");
    let tower = report.checks.iter().find(|c| c.id.contains("3-1.tower")).expect("tower");
    assert!(tower.explanation.en.contains("gov.") || tower.explanation.en.contains("6.10"));
}

#[test]
fn multi_part_examples_populate_all_eight_entity_lists() {
    let doc = En1993Snapshot::compliant_heb240_frame();
    assert!(!doc.bridge_fatigue.is_empty());
    assert!(!doc.tower_legs.is_empty());
    assert!(!doc.piles.is_empty());
    assert!(!doc.crane_runways.is_empty());
    assert!(!doc.cold_formed_members.is_empty());
    assert!(!doc.plated_panels.is_empty());
    assert!(!doc.silo_shells.is_empty());
    assert!(!doc.tension_components.is_empty());
    let bad = En1993Snapshot::noncompliant_overloaded_frame();
    assert!(bad.bridge_fatigue.iter().any(|_| true));
    assert!(!bad.crane_runways.is_empty());
}

#[test]
fn member_type_gates_buckling_and_ltb_checks() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    doc.members[0].member_type = semio_s_artifact_norm_en1993_model::SteelMemberType::BeamColumn;
    let full = check_full_steel_structure(&doc);
    assert!(full.checks.iter().any(|c| c.id.contains("6.3.1.nb")), "beamColumn must run flexural buckling");
    assert!(full.checks.iter().any(|c| c.id.contains("6.3.2.mb")), "beamColumn must run LTB when My present");

    doc.members[0].member_type = semio_s_artifact_norm_en1993_model::SteelMemberType::Tie;
    let tie = check_full_steel_structure(&doc);
    assert!(
        tie.checks.iter().all(|c| !c.id.contains("6.3.1.nb") && !c.id.contains("6.3.2.mb") && !c.id.contains("6.2.4.n")),
        "tie must skip buckling, LTB and compression"
    );
    assert!(tie.checks.len() < full.checks.len(), "tie must emit fewer member ULS checks than beamColumn");

    doc.members[0].member_type = semio_s_artifact_norm_en1993_model::SteelMemberType::Beam;
    let beam = check_full_steel_structure(&doc);
    assert!(beam.checks.iter().all(|c| !c.id.contains("6.3.1.nb")), "beam skips flexural buckling");
    assert!(beam.checks.iter().any(|c| c.id.contains("6.3.2.mb")), "beam keeps LTB");

    doc.members[0].member_type = semio_s_artifact_norm_en1993_model::SteelMemberType::Column;
    let column = check_full_steel_structure(&doc);
    assert!(column.checks.iter().any(|c| c.id.contains("6.3.1.nb")), "column keeps flexural buckling");
    assert!(column.checks.iter().all(|c| !c.id.contains("6.3.2.mb")), "column skips LTB");
}

#[test]
fn clause_oracle_matches_rust_clause_functions() {
    use crate::clauses::{fatigue_fire, joints, mcr_fe, resistance, stability};
    let any = fixture_path!("../../../../..").canonicalize().expect("family any root");
    let oracle = any.join("🔮️oracles/🧮️clauses/🐍️.py");
    let out = semio_s_plugin_norm_compliance::python_oracle::command().arg(&oracle).output().expect("spawn oracle");
    assert!(out.status.success(), "oracle failed: {}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&text[text.find('{').expect("json")..=text.rfind('}').expect("json end")]).expect("parse oracle");
    let f = |k: &str| v[k].as_f64().unwrap_or_else(|| panic!("missing {k}"));
    let close = |a: f64, b: f64, rel: f64, what: &str| assert!((a - b).abs() <= rel * b.abs().max(1e-12), "{what}: rust {a} vs oracle {b}");

    let ncr = stability::n_cr(stability::e_steel() * 3923.0e4, 4000.0);
    let lam = stability::lambda_bar_from_ncr(10600.0 * 355.0, ncr);
    let chi = stability::chi_flexural(lam, stability::Curve::C, 1.8e6, ncr);
    close(chi, f("e01_chi_z"), 1e-9, "E01 chi_z");
    close(chi * 10600.0 * 355.0 / 1.1, f("e01_nbrd_z_de"), 1e-9, "E01 Nb,Rd DE");

    let (iz, it, iw, l) = (603.8e4, 20.12e4, 125900.0e6, 5000.0);
    let mcr = stability::mcr_closed_form(stability::e_steel(), stability::g_steel(), iz, it, iw, l, 1.0, 1.0, 1.127, 0.0, 0.0);
    close(mcr, f("e02_mcr_closed"), 1e-9, "E02 Mcr");
    let lam_lt = stability::lambda_lt_from_mcr(628.4e3, 235.0, mcr);
    close(stability::chi_lt_general(lam_lt, 0.34, 70e6, mcr), f("e02_chi_lt_234"), 1e-9, "E02 chi_LT");
    let chi_mod = stability::chi_lt_mod(stability::chi_lt_rolled(lam_lt, 0.34, 0.4, 0.75, 70e6, mcr), stability::ltb_f(0.94, lam_lt));
    close(chi_mod, f("e02_chi_lt_mod"), 1e-9, "E02 chi_LT,mod");

    let p = mcr_fe::BeamProps { l, e: stability::e_steel(), g: stability::g_steel(), iz, it, iw };
    let uniform = |_x: f64| 1.0;
    let none = |_x: f64| 0.0;
    let r = mcr_fe::Restraints { supports: vec![mcr_fe::Support::fork(0.0), mcr_fe::Support::fork(l)], torsional_spring_per_length: 0.0, lateral_springs: vec![] };
    let fe = mcr_fe::mcr_fe(&p, &mcr_fe::Load { moment: &uniform, line_load: &none, zg: 0.0 }, &r, 40).expect("fe uniform");
    close(fe.lambda, f("e02_mcr_uniform_galerkin"), 5e-3, "Mcr FE uniform vs Galerkin");
    let parabola = move |x: f64| x * (l - x) / 2.0;
    let q = |_x: f64| 1.0;
    let fe = mcr_fe::mcr_fe(&p, &mcr_fe::Load { moment: &parabola, line_load: &q, zg: 0.0 }, &r, 40).expect("fe udl");
    close(fe.lambda * l * l / 8.0, f("e02_mcr_udl_galerkin"), 5e-3, "Mcr FE UDL vs Galerkin");
    close(fe.lambda * l * l / 8.0, f("e02_mcr_udl_scipy"), 5e-3, "Mcr FE UDL vs scipy eigh");
    let fe = mcr_fe::mcr_fe(&p, &mcr_fe::Load { moment: &parabola, line_load: &q, zg: 150.0 }, &r, 40).expect("fe top flange");
    close(fe.lambda * l * l / 8.0, f("e02_mcr_top_flange_scipy"), 5e-3, "Mcr FE top-flange load vs scipy eigh");
    close(chi, f("e01_chi_z_brentq"), 1e-9, "E01 chi vs Ayrton-Perry brentq");
    close(stability::chi_lt_general(lam_lt, 0.34, 70e6, mcr), f("e02_chi_lt_brentq"), 1e-9, "E02 chi_LT vs brentq");

    let eff = crate::clauses::effective_width::effective_internal(1000.0, 8.0, 355.0, -1.0);
    close(eff.b_eff, f("e04_beff"), 1e-9, "E04 beff");
    let eps = crate::clauses::classification::epsilon(355.0);
    let lw = resistance::lambda_w(1000.0, 8.0, eps, resistance::k_tau(2.0));
    close(resistance::v_bw_rd(resistance::chi_w(lw, 1.2, true), 355.0, 1000.0, 8.0, 1.0) / 1000.0, f("e07_vbw"), 1e-9, "E07 Vbw");
    let ab = joints::alpha_b(true, 40.0, 100.0, 22.0, 800.0, 490.0);
    close(joints::fb_rd(joints::k1_bearing(true, 35.0, 100.0, 22.0), ab, 490.0, 20.0, 12.0, 1.25, 1.0), f("e08_fb"), 1e-9, "E08 Fb,Rd");
    close(fatigue_fire::miner_damage(&[(60.0, 1.0e6)], 71.0, 1.15, 1.0), f("e11_damage"), 1e-9, "E11 damage");

    let rf = fatigue_fire::rainflow(&[0.0, 10.0, 2.0, 8.0, 0.0, 9.0, 1.0, 6.0, 0.0]);
    close(rf.iter().map(|c| c.1).sum::<f64>(), f("rainflow_count"), 1e-12, "rainflow count");
    let mut ranges: Vec<f64> = rf.iter().map(|c| c.0).collect();
    ranges.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let expected: Vec<f64> = v["rainflow_ranges"].as_array().expect("ranges").iter().map(|x| x.as_f64().unwrap()).collect();
    assert_eq!(ranges, expected);

    let iso = |t: f64| fatigue_fire::gas_iso834(t);
    let h = fatigue_fire::unprotected_steel_temperature(&iso, fatigue_fire::k_sh_i_section(167.26 / 215.58), 215.58, 900.0, 5.0, 0.7, 1.0);
    let at = |s: f64| h.iter().find(|(t, _)| (*t - s).abs() < 1e-9).map(|p| p.1).expect("sample");
    for (i, s) in [300.0, 600.0, 900.0].into_iter().enumerate() {
        let o = v["e12_theta"][i].as_f64().expect("theta");
        assert!((at(s) - o).abs() <= 3.0, "fire θ at {s}: rust {} vs oracle {o}", at(s));
    }
    close(crate::clauses::global::sway_phi(6.0, 2.0), f("e13_phi"), 1e-12, "E13 phi");
    for (gm1, key) in [(1.0, "en"), (1.1, "de")] {
        let i = stability::InteractionInput {
            n_ed: 800e3,
            my_ed: 100e6,
            mz_ed: 0.0,
            n_rk: 3763.0e3,
            my_rk: 373.8e6,
            mz_rk: 1.0,
            chi_y: stability::chi_flexural(0.5079, stability::Curve::B, 800e3, 1.0),
            chi_z: stability::chi_flexural(0.8605, stability::Curve::C, 800e3, 1.0),
            chi_lt: 1.0,
            lambda_y: 0.5079,
            lambda_z: 0.8605,
            cmy: 0.6,
            cmz: 0.6,
            cm_lt: 0.6,
            class: 1,
            susceptible_torsion: true,
            rhs: false,
            gm1,
            delta_my: 0.0,
            delta_mz: 0.0,
        };
        let r = stability::interaction_annex_b(&i);
        close(r.eta_61, f(&format!("e06_eta61_{key}")), 2e-3, "E06 eta61");
        close(r.eta_62, f(&format!("e06_eta62_{key}")), 2e-3, "E06 eta62");
    }
    let t = crate::clauses::joint_components::t_stub_modes(210.0, 210.0, 15.0, 355.0, 40.0, 40.0, 0.0, 282240.0, 245.0, 2.0, 60.0, crate::clauses::joint_components::Mode1Method::Two, 1.0);
    close(t.ft_2, f("e10_ft2"), 1e-9, "E10 FT2");
}

#[test]
fn every_ultimate_row_is_checked_and_minor_axis_bending_is_reported() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    doc.member_actions.iter_mut().for_each(|a| a.action.mz = 20_000.0);
    let report = check_full_steel_structure(&doc);
    let rows = |needle: &str| report.checks.iter().filter(|c| c.id.contains(needle)).count();
    let n_rows = crate::uls_rows(&crate::combine_member_actions(&doc, "member-b1")).len();
    assert!(n_rows >= 2, "the example combines at least 6.10a and 6.10b");
    assert_eq!(rows("6.2.5.mz."), n_rows, "Mz must be checked on every row");
    assert_eq!(rows("6.2.5.my."), n_rows, "My must be checked on every row");
    assert_eq!(rows("6.3.3.intb."), n_rows, "Annex B interaction on every row");
}

#[test]
fn member_checks_emit_stations_that_follow_the_declared_moment_diagram() {
    for (diagram, peak_at) in [("parabolic", 0.5), ("linear", 0.0)] {
        let mut document = En1993Snapshot::compliant_heb240_frame();
        document.members[0].moment_diagram = diagram.into();
        document.members[0].end_moment_ratio_psi = 0.5;
        document.member_actions[0].action.my = 40_000.0;
        let report = check_full_steel_structure(&document);
        let bending = report.checks.iter().find(|c| c.id.starts_with("en1993.6.2.5.my.")).expect("bending check");
        assert!(bending.stations_consistent(), "{diagram}");
        assert_eq!(bending.stations.len(), 11);
        let peak = bending.stations.iter().max_by(|a, b| a.utilization.total_cmp(&b.utilization)).unwrap();
        approx(peak.at[0], peak_at, 1e-12);
        approx(peak.utilization, bending.utilization, 1e-12);
        let ends = (bending.stations[0].utilization, bending.stations[10].utilization);
        if diagram == "parabolic" {
            approx(ends.0, 0.0, 1e-12);
            approx(ends.1, 0.0, 1e-12);
        } else {
            approx(ends.1, ends.0 * 0.5, 1e-9);
        }
    }
}

#[test]
fn every_member_check_with_stations_satisfies_the_station_law() {
    let report = check_full_steel_structure(&En1993Snapshot::compliant_heb240_frame());
    let sampled: Vec<_> = report.checks.iter().filter(|c| !c.stations.is_empty()).collect();
    assert!(sampled.len() >= 4, "bending, shear, interaction and buckling emit stations");
    assert!(sampled.iter().all(|c| c.stations_consistent()));
}

/// 🔮️ EN 1993 third-party oracle cases: the shared harness recomputes every blessed case in `🧪️tests/🧫️fixtures/🔮️oracle-cases` with the registered libraries and fails on any mismatch beyond the tolerance profile or between libraries.
#[test]
fn third_party_libraries_reproduce_the_blessed_oracle_cases() {
    let verified = semio_s_plugin_norm_compliance::python_oracle::verify_cases_above(env!("CARGO_MANIFEST_DIR"));
    assert!(verified >= 37, "expected at least 37 verified oracle cases, got {verified}");
}

#[test]
fn blessed_third_party_cases_feed_the_evaluator() {
    use crate::clauses::{resistance, stability};
    let any = fixture_path!("../../../../..").canonicalize().expect("family any root");
    let text = std::fs::read_to_string(any.join("🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party.json")).expect("cases");
    let doc: serde_json::Value = serde_json::from_str(&text).expect("parse cases");
    let tol = |profile: &str| match profile {
        "closed-form-v1" => 1e-9,
        "eurocode-quantities-v1" => 1e-4,
        "numerical-v1" => 5e-3,
        "method-spread-v1" => 3e-2,
        other => panic!("unknown profile {other}"),
    };
    let catalogue = semio_s_artifact_norm_en1993_model::rolled_heb_catalogue();
    let mut compared = 0;
    for case in doc["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let inp = |k: &str| case["inputs"][k].as_f64().unwrap_or_else(|| panic!("{id}: input {k}"));
        let mut got: Vec<(&str, f64)> = Vec::new();
        if let Some(name) = id.strip_prefix("en1993.section.heb") {
            let s = catalogue.iter().find(|s| s.designation == format!("HEB {name}")).expect("catalogue row");
            got = vec![("A", s.area), ("Iy", s.iy), ("Iz", s.iz), ("It", s.it), ("Iw", s.iw), ("Wel_y", s.w_el_y), ("Wel_z", s.w_el_z), ("Wpl_y", s.w_pl_y), ("Wpl_z", s.w_pl_z)];
        } else if id.starts_with("en1993.6.3.2.mcr-ipe300") {
            let e = &case["expected"];
            let q = |k: &str| e[k]["value"].as_f64().expect("expected input quantity");
            got = vec![("M_cr", stability::mcr_closed_form(inp("E"), inp("G"), q("Iz"), q("It"), q("Iw"), inp("L"), 1.0, 1.0, inp("C1"), 0.0, 0.0))];
        } else if id.contains(".6.2.npl-") {
            got = vec![("N_pl_Rd", resistance::n_pl_rd(inp("a"), inp("f_y"), inp("gamma_m0")))];
        } else if id.contains(".6.2.mc-rd-") {
            got = vec![("M_c_Rd", inp("w_pl") * inp("f_y") / inp("gamma_m0"))];
        } else if id.contains(".6.2.vpl-rd-") {
            got = vec![("V_pl_Rd", resistance::v_pl_rd(inp("a_v"), inp("f_y"), inp("gamma_m0")))];
        } else if id.contains(".6.2.av-rolled-i-") {
            got = vec![("A_v", resistance::av_rolled_i(inp("a"), inp("b1"), inp("tf1"), inp("tw"), inp("r1"), inp("hw"), inp("eta")))];
        } else if id.contains(".6.2.my-v-rd-") {
            got = vec![("M_y_V_Rd", resistance::m_y_v_rd_i(inp("w_pl_y"), inp("rho"), inp("h_w") * inp("t_w"), inp("t_w"), inp("f_y"), inp("gamma_m0")).min(inp("m_y_c_rd")))];
        } else if id.contains(".6.2.my-n-rd-") {
            got = vec![("M_N_y_Rd", resistance::m_n_y_rd_i(inp("mpl_y_rd"), inp("n"), inp("a")))];
        } else if id.contains(".6.3.mb-rd-") {
            got = vec![("M_b_Rd", inp("chi_lt") * inp("w_y") * inp("f_y") / inp("gamma_m1"))];
        } else if id.ends_with(".5.2.alpha-cr") {
            got = vec![("alpha_cr", crate::clauses::global::alpha_cr_storey(inp("h_ed"), inp("v_ed"), inp("h"), inp("delta_h_ed")).expect("alpha_cr"))];
        }
        for (key, value) in got {
            let exp = &case["expected"][key];
            let expected = exp["value"].as_f64().unwrap_or_else(|| panic!("{id}: expected {key}"));
            let t = tol(exp["profile"].as_str().expect("profile"));
            assert!((value - expected).abs() <= t * expected.abs().max(1e-300), "{id} {key}: rust {value} vs third-party {expected} (tol {t})");
            compared += 1;
        }
    }
    assert!(compared >= 60, "only {compared} quantities compared");
}

#[test]
fn heb240_catalogue_torsion_and_warping_give_the_fe_critical_moment() {
    let s = &semio_s_artifact_norm_en1993_model::rolled_heb_catalogue()[0];
    let m_cr = crate::clauses::stability::mcr_closed_form(210.0e9, 81.0e9, s.iz, s.it, s.iw, 6.0, 1.0, 1.0, 1.0, 0.0, 0.0);
    assert!((m_cr / 1000.0 - 501.7).abs() < 3.0, "M_cr {} kNm", m_cr / 1000.0);
}

#[test]
fn annex_switch_reads_the_registry_and_changes_only_gamma_m1() {
    let (en, de) = (AnnexParams::en(), AnnexParams::de());
    assert_eq!((en.gamma_m0, en.gamma_m1, en.gamma_m2, en.gamma_m3, en.gamma_mf), (1.0, 1.0, 1.25, 1.25, 1.15));
    assert_eq!((de.gamma_m0, de.gamma_m1, de.gamma_m2, de.gamma_m3, de.gamma_mf), (1.0, 1.1, 1.25, 1.25, 1.15));
}

#[test]
fn governing_effects_choose_by_unit_free_demand_not_by_a_sum_of_kn_and_knm() {
    use crate::{governing_effects, CombinedEffects};
    use semio_s_artifact_norm_en1993_model::DesignAction;
    let row = |id: &str, n: f64, my: f64, vz: f64| CombinedEffects { combination_id: id.into(), situation: "uls".into(), action: DesignAction { n, vy: 0.0, vz, my, mz: 0.0, t: 0.0 } };
    let rows = [row("axial", 3.0e6, 0.2e6, 0.01e6), row("bending", 0.5e6, 0.6e6, 0.2e6)];
    let by_sum = rows.iter().max_by(|a, b| (a.action.n.abs() + a.action.my.abs() + a.action.vz.abs()).total_cmp(&(b.action.n.abs() + b.action.my.abs() + b.action.vz.abs()))).expect("rows");
    assert_eq!(by_sum.combination_id, "axial");
    let by_demand = governing_effects(&rows, "uls").expect("uls rows");
    assert_eq!(by_demand.combination_id, "bending", "shares of the peaks: axial 1.00 + 0.33 + 0.05, bending 0.17 + 1.00 + 1.00");
}

#[test]
fn reliability_class_rc3_raises_the_governing_design_effect_by_k_fi() {
    for annex in [AnnexChoice::De, AnnexChoice::En] {
        let base = En1993Snapshot { annex, ..En1993Snapshot::compliant_heb240_frame() };
        let member_id = base.members[0].id.clone();
        let governing = |reliability_class| {
            let document = En1993Snapshot { reliability_class, ..base.clone() };
            uls_rows(&combine_member_actions(&document, &member_id)).iter().map(|e| e.action.n.abs() + e.action.my.abs() + e.action.vz.abs()).fold(0.0_f64, f64::max)
        };
        let k_fi = ReliabilityClass::RC3.k_fi(annex);
        assert!((k_fi - 1.1).abs() < 1e-9, "registry K_FI of RC3 (unverified) is {k_fi}");
        let ratio = governing(ReliabilityClass::RC3) / governing(ReliabilityClass::RC2);
        assert!((ratio - k_fi / ReliabilityClass::RC2.k_fi(annex)).abs() < 1e-9, "governing effect ratio {ratio}");
    }
}

#[test]
fn reliability_class_of_the_document_reaches_the_evaluation() {
    let rc2 = En1993Snapshot::compliant_heb240_frame();
    let rc3 = En1993Snapshot { reliability_class: ReliabilityClass::RC3, ..rc2.clone() };
    let load_effect = |document: &En1993Snapshot| check_full_steel_structure(document).checks.iter().map(|c| c.utilization).sum::<f64>();
    assert!(load_effect(&rc3) > load_effect(&rc2));
}

#[test]
fn the_hollow_section_interaction_follows_6_2_9_1_with_the_oracle_value() {
    let mut section = En1993Snapshot::compliant_heb240_frame().sections[0].clone();
    section.kind = semio_s_artifact_norm_en1993_model::SectionKind::Hss;
    section.h = 0.2;
    section.b = 0.1;
    section.tf = 0.01;
    section.tw = 0.01;
    section.area = 0.0056;
    let eta = part_1_1::section_mn_eta(&section, 50.0, 100.0 * 1.0 / 3.0, 100.0 * 0.4375 / 1.5, 100.0, 100.0, 100.0, 1);
    approx(eta, 0.402_318_824_907_548, 1e-9);
    let linear = part_1_1::section_mn_eta(&section, 50.0, 25.0, 25.0, 100.0, 100.0, 100.0, 3);
    approx(linear, 1.0, 1e-12);
}
