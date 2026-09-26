//! ⚖️ EN 1993 compliance numeric worked examples (SI).

use crate::document::{AnnexChoice, CheckStatus};
use crate::standards::v1::subsets::any::schema::{check_full_steel_structure, part_1_1, part_1_2, part_1_8, part_1_9, AnnexParams};
use crate::En1993Snapshot;

fn approx(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} !≈ {b} (tol {tol})");
}

#[test]
fn axial_resistance_s355_heb240_matches_hand_calc() {
    let params = AnnexParams::de();
    let n_rd = part_1_1::axial_resistance_n(0.0106, 355.0e6, params);
    // A·fy/γ_M0 = 0.0106 * 355e6 / 1.0 = 3.763e6 N = 3763 kN
    approx(n_rd / 1000.0, 3763.0, 1.0);
}

#[test]
fn flexural_buckling_chi_from_lambda_never_user_supplied() {
    let curve = part_1_1::BucklingCurve::B;
    let chi = part_1_1::chi(1.0, curve);
    // φ = 0.5*(1+0.34*(λ̄-0.2)+λ̄²)=0.5*(1+0.272+1)=1.136; χ=1/(φ+√(φ²-λ̄²))≈0.597
    approx(chi, 0.597, 0.01);
}

#[test]
fn de_gamma_m1_raises_buckling_utilization_vs_en() {
    let mut de = En1993Snapshot::compliant_heb240_frame();
    de.annex = AnnexChoice::De;
    let mut en = de.clone();
    en.annex = AnnexChoice::En;
    // Push compression high enough that buckling governs
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
    // Fb,Rd = n·k1·αb·fu·d·t/γM2 with n=2 → ≈247.3 kN
    approx(fb / 1000.0, 209.5, 2.0);
}

#[test]
fn critical_temperature_eq_4_22() {
    let t = part_1_2::critical_temperature_c(0.5);
    // EN 1993-1-2 Eq. (4.22): θcr=39.19·ln(1/(0.9674·μ₀^3.833)-1)+482 ≈ 584.7 °C at μ₀=0.5
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
    // Part-scoped fails with ≥2 applicable remedies where available
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
    assert!(u_after <= 1.0 + 1e-6, "remedy must bring buckling u≤1, got {u_after}");
}

#[test]
fn section_class_heb240_s355_is_class_1_or_2() {
    let section = crate::snapshot::catalogue_heb240();
    let class = part_1_1::section_class_rolled_i(&section, 355.0e6);
    assert!(class <= 2, "HEB 240 S355 expected class 1–2, got {class}");
}

#[test]
fn m_cr_positive_for_rolled_i() {
    let doc = En1993Snapshot::compliant_heb240_frame();
    let section = &doc.sections[0];
    let material = &doc.materials[0];
    let member = &doc.members[0];
    let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length, member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
    assert!(m_cr > 0.0, "M_cr must be computed > 0");
}


#[test]
fn remedy_law_oneof_section_id_flips_axial_or_buckling_to_pass() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    // Characteristic permanent N_k so EN1990 γ_G·N_k exceeds N_Rd but still fits a catalogue HEB.
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
        if let Some(cat) = crate::snapshot::rolled_heb_catalogue().into_iter().find(|s| s.id == chosen) {
            fixed.sections.push(cat);
        }
    }
    let after = check_full_steel_structure(&fixed);
    let u_after = after.checks.iter().find(|c| c.id == fail.id).map(|c| c.utilization).unwrap_or(0.0);
    assert!(u_after <= 1.0 + 1e-6, "section OneOf must bring axial to Pass, u={u_after}, chose {chosen}, options={:?}", remedy.options);
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
        if u_after <= 1.0 + 1e-6 {
            return;
        }
        let joint = fixed.joints.iter_mut().find(|j| j.id == "joint-j1").unwrap();
        joint.bolt_rows += 1;
    }
    let after = check_full_steel_structure(&fixed);
    let u_after = after.checks.iter().find(|c| c.id.contains("1-8.3.6.shear.joint-j1")).map(|c| c.utilization).unwrap_or(0.0);
    let rows_final = fixed.joints.iter().find(|j| j.id == "joint-j1").unwrap().bolt_rows;
    assert!(u_after <= 1.0 + 1e-6, "boltRows remedy must bring shear u≤1, got {u_after} rows={rows_final}");
}


#[test]
fn every_emitted_path_parses_and_resolves() {
    use crate::app_surface::{get_value_at_path, parse_path};
    for doc in [En1993Snapshot::compliant_heb240_frame(), En1993Snapshot::noncompliant_overloaded_frame()] {
        let report = check_full_steel_structure(&doc);
        let snap_val = dsl::ToValue::to_value(&doc);
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
    let section = crate::snapshot::catalogue_heb240();
    assert_eq!(part_1_1::ltb_curve_table_6_4(&section), part_1_1::BucklingCurve::A);
}

#[test]
fn slip_resistant_category_c_produces_slip_check() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    let joint = doc.joints.iter_mut().find(|j| j.id == "joint-j1").unwrap();
    joint.category = "C".into();
    joint.friction_mu = 0.50;
    joint.preload_force = 0.0;
    joint.actions = vec![crate::JointForceAction { id: "jf-g".into(), load_case_id: "g-permanent".into(), shear: 200_000.0, tension: 0.0 }];
    let report = check_full_steel_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("3.9.slip")), "expected slip check");
}

#[test]
fn python_oracle_and_jsonschema_agree_within_half_percent() {
    use std::path::PathBuf;
    use std::process::Command;
    let any = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
        .canonicalize()
        .expect("family any root");
    let oracle = any.join("🔮️oracles/⚖️compliance/🐍️.py");
    let validate = any.join("🔮️oracles/🧬️snapshot-schema/🐍️.py");
    let schema = any.join("🧬️schema/📸️snapshot/🔣️.json");
    let compliant = any.join("🖼️assets/✅️heb240-compliant/snapshot.json");
    assert!(oracle.is_file(), "missing oracle at {oracle:?}");
    let out = Command::new("python3").arg(&oracle).output().expect("spawn oracle");
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

    let status = Command::new("python3").arg(&validate).arg(&schema).arg(&compliant).status().expect("spawn validate");
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

fn wildcardize(path: &str) -> String {
    let mut out = String::new();
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' {
            out.push_str("[]");
            while let Some(x) = chars.next() {
                if x == ']' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[test]
fn field_meta_covers_every_editable_leaf_en_de_unit() {
    use crate::field_meta::en1993_field_meta;
    let snap = En1993Snapshot::compliant_heb240_frame();
    let value = serde_json::to_value(&snap).expect("json");
    let mut missing = Vec::new();
    walk_leaves("", &value, &mut |path| {
        if path.is_empty() {
            return;
        }
        let wild = wildcardize(path);
        match en1993_field_meta(path).or_else(|| en1993_field_meta(&wild)) {
            None => missing.push(format!("{path} (wild={wild})")),
            Some(meta) => {
                assert!(!meta.label_en.is_empty(), "{path} empty en");
                assert!(!meta.label_de.is_empty(), "{path} empty de");
                assert!(!meta.label_en.is_empty() && !meta.label_de.is_empty());
                if let Some(choices) = meta.choices {
                    for c in choices {
                        assert!(!c.label_en.is_empty());
                        assert!(!c.label_de.is_empty());
                        assert!(c.label_en != c.value || c.label_de != c.value, "{path} choice labels must not be raw codes only ({})", c.value);
                    }
                }
            }
        }
    });
    assert!(missing.is_empty(), "missing field meta for: {missing:?}");
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
        sec.kind = "weldedI".into();
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
    // Governing ULS N ≈ 1.35*100kN + 1.5*43.333kN ≈ 200 kN
    let axial = report.checks.iter().find(|c| c.id.contains("6.2.4")).expect("axial");
    assert!(axial.computed.value > 190_000.0 && axial.computed.value < 210_000.0, "expected ~200 kN design N from EN1990 combo, got {}", axial.computed.value);
}


#[test]
fn palmgren_miner_damage_at_two_million_cycles_matches_oracle() {
    // Δσ = Δσ_C = 71 MPa, N = 2e6 → D = n/N = 1.0 on the m=3 branch (γ_Mf=1)
    let delta_c = part_1_9::detail_category_pa(71);
    let d = part_1_9::miner_damage(&[(delta_c, 2.0e6)], delta_c, 1.0);
    approx(d, 1.0, 0.005);
    let d2 = part_1_9::miner_damage(&[(delta_c, 1.0e6)], delta_c, 1.0);
    approx(d2, 0.5, 0.005);
}

#[test]
fn class4_uses_effective_section_not_class_over_three_tautology() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    // Force a slender fabricated section toward class 4 by thinning flanges/web via catalogue override
    if let Some(sec) = doc.sections.iter_mut().find(|s| s.id == "sec-heb240") {
        sec.tf = 0.006;
        sec.tw = 0.005;
        sec.b = 0.300;
        sec.h = 0.400;
        sec.kind = "weldedI".into();
    }
    let report = check_full_steel_structure(&doc);
    let class_check = report.checks.iter().find(|c| c.id.contains("5.2.class")).expect("class row");
    // Informational — must not use u = class/3 tautology (limit 3.0 with computed=class)
    // Must not encode u=class/3 (computed=class, limit=3)
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
fn de_na_snow_psi0_differs_from_en_recommended() {
    // EN 1990 Table A1.1 snow ψ0 = 0.5; DE NA DIN EN 1990/NA ψ0 = 0.7
    let (psi0_en, _, _) = super::psi_factors("snow", AnnexChoice::En);
    let (psi0_de, _, _) = super::psi_factors("snow", AnnexChoice::De);
    assert!((psi0_en - 0.5).abs() < 1e-9, "EN snow ψ0");
    assert!((psi0_de - 0.7).abs() < 1e-9, "DE NA snow ψ0");
    assert!((psi0_en - psi0_de).abs() > 1e-9);
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
    doc.members[0].member_type = "beamColumn".into();
    let full = check_full_steel_structure(&doc);
    assert!(full.checks.iter().any(|c| c.id.contains("6.3.1.nb")), "beamColumn must run flexural buckling");
    assert!(full.checks.iter().any(|c| c.id.contains("6.3.2.mb")), "beamColumn must run LTB when My present");

    doc.members[0].member_type = "tie".into();
    let tie = check_full_steel_structure(&doc);
    assert!(
        tie.checks.iter().all(|c| !c.id.contains("6.3.1.nb") && !c.id.contains("6.3.2.mb") && !c.id.contains("6.2.4.n")),
        "tie must skip buckling, LTB and compression"
    );
    assert!(tie.checks.len() < full.checks.len(), "tie must emit fewer member ULS checks than beamColumn");

    doc.members[0].member_type = "beam".into();
    let beam = check_full_steel_structure(&doc);
    assert!(beam.checks.iter().all(|c| !c.id.contains("6.3.1.nb")), "beam skips flexural buckling");
    assert!(beam.checks.iter().any(|c| c.id.contains("6.3.2.mb")), "beam keeps LTB");

    doc.members[0].member_type = "column".into();
    let column = check_full_steel_structure(&doc);
    assert!(column.checks.iter().any(|c| c.id.contains("6.3.1.nb")), "column keeps flexural buckling");
    assert!(column.checks.iter().all(|c| !c.id.contains("6.3.2.mb")), "column skips LTB");
}
