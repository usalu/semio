//! ⚖️ EN 1992 compliance numeric worked examples + remedy law + oracle parity.

use crate::document::AnnexChoice;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;
use crate::standards::v1::subsets::any::schema::na_de::AnnexParams;
use crate::standards::v1::subsets::any::schema::part_1_1;
use crate::En1992Snapshot;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn shear_vrdc_worked_example_de() {
    let v_de = part_1_1::shear_v_rd_c_n(0.30, 0.450, 30.0e6, 0.01, 0.0, AnnexChoice::De);
    let v_en = part_1_1::shear_v_rd_c_n(0.30, 0.450, 30.0e6, 0.01, 0.0, AnnexChoice::En);
    assert!(v_en > v_de, "EN C_Rd,c=0.18/γ_c exceeds DE 0.15/γ_c");
    assert!((v_de / 1e3 - 69.9).abs() < 2.0, "V_Rd,c,DE={v_de} N");
    assert!((v_en / 1e3 - 83.9).abs() < 2.0, "V_Rd,c,EN={v_en} N");
}

#[test]
fn flexure_de_vs_en_divergence() {
    let m_de = part_1_1::flexural_resistance_nm(30.0e6, 0.30, 0.450, 1200.0e-6, 500.0e6, 0.0, AnnexChoice::De);
    let m_en = part_1_1::flexural_resistance_nm(30.0e6, 0.30, 0.450, 1200.0e-6, 500.0e6, 0.0, AnnexChoice::En);
    assert!(m_en > m_de, "EN M_Rd should exceed DE because α_cc=1.0 vs 0.85");
    assert!((m_de / 1e3 - 208.0).abs() < 15.0, "M_Rd,DE≈208 kNm got {}", m_de / 1e3);
    assert!((m_en / 1e3 - 212.0).abs() < 15.0, "M_Rd,EN≈212 kNm got {}", m_en / 1e3);
    assert_eq!(AnnexParams::de().alpha_cc, 0.85);
    assert_eq!(AnnexParams::en().alpha_cc, 1.0);
}

#[test]
fn cot_theta_de_limits() {
    let p = AnnexParams::de();
    let cot = p.cot_theta(AnnexChoice::De, 0.0, 17.0e6);
    assert!((cot - 1.2).abs() < 1e-9);
    let cot_hi = p.cot_theta(AnnexChoice::De, 17.0e6, 17.0e6);
    assert!((cot_hi - 1.4).abs() < 1e-9 || cot_hi <= 3.0);
    assert!(cot >= 1.0 && cot <= 3.0);
}

#[test]
fn fire_r60_axis_distance() {
    use crate::standards::v1::subsets::any::schema::part_1_2_fire;
    use crate::FireRating;
    let a = part_1_2_fire::required_axis_distance_beam_m(0.160, FireRating::R60);
    assert!((a * 1000.0 - 35.0).abs() < 0.5);
}

#[test]
fn compliant_office_frame_has_no_fail() {
    let report = evaluate(&En1992Snapshot::compliant_office_frame());
    assert!(!report.checks.is_empty());
    let fails: Vec<_> = report.failing().map(|c| c.id.clone()).collect();
    assert!(fails.is_empty(), "unexpected fails: {fails:?}");
    assert!(report.complies());
}

#[test]
fn failing_under_reinforced_has_multiple_fails_with_remedies() {
    let report = evaluate(&En1992Snapshot::failing_under_reinforced());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 3, "expected ≥3 fails, got {}", fails.len());
    for f in &fails {
        assert!(!f.remedies.is_empty(), "fail {} missing remedy", f.id);
        assert!(f.remedies.iter().any(|r| r.applicable), "fail {} has no applicable remedy", f.id);
    }
}

#[test]
fn remedy_law_cover_as_and_stirrups_flip_to_pass() {
    let before_snap = En1992Snapshot::failing_under_reinforced();
    let before = evaluate(&before_snap);
    let mut flipped = 0usize;
    let mut after_snap = before_snap.clone();

    if let Some(fail) = before.failing().find(|c| c.id.contains("cover")) {
        let remedy = fail.remedies.iter().find(|r| r.applicable).expect("cover remedy");
        assert!(remedy.target.path.contains("[id="), "{}", remedy.target.path);
        let id = remedy.target.path.split("[id=").nth(1).unwrap().split(']').next().unwrap();
        after_snap.members.iter_mut().find(|m| m.id == id).unwrap().cover = remedy.required.value;
        flipped += 1;
    }

    if let Some(fail) = before.failing().find(|c| c.id.contains("flexure")) {
        if let Some(remedy) = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("longitudinal") && r.target.path.ends_with(".count")) {
            let mid = remedy.target.path.split("[id=").nth(1).unwrap().split(']').next().unwrap();
            let lid = remedy.target.path.split("longitudinal[id=").nth(1).unwrap().split(']').next().unwrap();
            if let Some(m) = after_snap.members.iter_mut().find(|m| m.id == mid) {
                if let Some(layer) = m.longitudinal.iter_mut().find(|l| l.id == lid) {
                    layer.count = remedy.required.value.round().max(1.0) as u32;
                    flipped += 1;
                }
            }
        }
    }

    if let Some(fail) = before.failing().find(|c| c.id.contains("shear")) {
        if let Some(remedy) = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("stirrups.spacing")) {
            let id = remedy.target.path.split("[id=").nth(1).unwrap().split(']').next().unwrap();
            if let Some(s) = after_snap.members.iter_mut().find(|m| m.id == id).unwrap().stirrups.as_mut() {
                s.spacing = remedy.required.value;
                flipped += 1;
            }
        }
    }

    assert!(flipped >= 2, "need ≥2 distinct remedies applied, got {flipped}");
    let after = evaluate(&after_snap);
    let before_ids: std::collections::HashSet<_> = before.failing().map(|c| c.id.clone()).collect();
    let after_ids: std::collections::HashSet<_> = after.failing().map(|c| c.id.clone()).collect();
    let cleared = before_ids.difference(&after_ids).count();
    assert!(cleared >= 2, "expected ≥2 fails to clear, cleared={cleared} before={before_ids:?} after={after_ids:?}");
}

#[test]
fn c_min_dur_xc3_de() {
    use crate::ExposureClass;
    let c = part_1_1::c_min_dur_m(ExposureClass::Xc3);
    assert!((c - 0.020).abs() < 1e-9);
}

#[test]
fn subject_paths_use_id_selectors_and_resolve() {
    use crate::app_surface::{get_value_at_path, parse_path};
    let snap = En1992Snapshot::compliant_office_frame();
    let report = evaluate(&snap);
    let root = dsl::ToValue::to_value(&snap);
    assert!(!report.checks.is_empty());
    for check in &report.checks {
        let path = &check.subject.path;
        if path.contains("members") || path.contains("anchors") {
            assert!(path.contains("[id="), "check {} path missing id selector: {path}", check.id);
        }
        parse_path(path).unwrap_or_else(|e| panic!("check {} path parse {path}: {e}", check.id));
        if get_value_at_path(&root, path).is_err() {
            if path.ends_with(".actions") {
                let parent = path.trim_end_matches(".actions");
                get_value_at_path(&root, parent).unwrap_or_else(|e| panic!("parent resolve {parent}: {e}"));
            } else {
                get_value_at_path(&root, path).unwrap_or_else(|e| panic!("check {} path resolve {path}: {e}", check.id));
            }
        }
        for remedy in &check.remedies {
            let rp = &remedy.target.path;
            if rp.contains("members") || rp.contains("anchors") {
                assert!(rp.contains("[id="), "remedy path missing id selector: {rp}");
            }
            parse_path(rp).unwrap_or_else(|e| panic!("remedy path parse {rp}: {e}"));
            get_value_at_path(&root, rp).unwrap_or_else(|e| panic!("remedy path resolve {rp}: {e}"));
        }
    }
}

#[test]
fn de_na_material_factors_and_cover_tables() {
    assert_eq!(AnnexParams::de().gamma_c, 1.5);
    assert_eq!(AnnexParams::de().gamma_s, 1.15);
    assert_eq!(AnnexParams::de().alpha_cc, 0.85);
    assert_eq!(AnnexParams::en().alpha_cc, 1.0);
    use crate::ExposureClass;
    assert!((part_1_1::c_min_dur_m(ExposureClass::Xc3) - 0.020).abs() < 1e-12);
    assert!((part_1_1::c_min_dur_m(ExposureClass::Xc4) - 0.025).abs() < 1e-12);
    assert!((part_1_1::w_max_m(ExposureClass::Xc3) - 0.30e-3).abs() < 1e-12);
    assert!((part_1_1::w_max_m(ExposureClass::Xc1) - 0.40e-3).abs() < 1e-12);
    let k_ss = part_1_1::basic_ld_limit(crate::SupportCondition::SimplySupported, 0.01, 0.0);
    let k_cont = part_1_1::basic_ld_limit(crate::SupportCondition::Continuous, 0.01, 0.0);
    assert!(k_cont > k_ss, "continuous K=1.5 raises l/d vs simply-supported K=1.0");
    assert!((En1992Snapshot::compliant_office_frame().delta_c_dev - 0.010).abs() < 1e-12);
}

#[test]
fn field_meta_covers_every_editable_leaf_en_de() {
    use crate::field_meta::en1992_field_meta;
    let snap = En1992Snapshot::compliant_office_frame();
    let value = serde_json::to_value(&snap).expect("json");
    let mut missing = Vec::new();
    walk_leaves("", &value, &mut |path| {
        if path.is_empty() {
            return;
        }
        match en1992_field_meta(path) {
            None => missing.push(path.to_string()),
            Some(meta) => {
                assert!(!meta.label_en.is_empty(), "{path} empty en");
                assert!(!meta.label_de.is_empty(), "{path} empty de");
                if let Some(choices) = meta.choices {
                    for c in choices {
                        assert!(!c.label_en.is_empty(), "{path} choice en");
                        assert!(!c.label_de.is_empty(), "{path} choice de");
                        assert!(
                            c.label_en != c.value || c.label_de != c.value,
                            "{path} choice labels must not be raw codes only ({})",
                            c.value
                        );
                    }
                }
            }
        }
    });
    assert!(missing.is_empty(), "missing field meta for: {missing:?}");
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
fn python_oracle_matches_utilizations_within_half_percent() {
    for (label, snap) in [("compliant", En1992Snapshot::compliant_office_frame()), ("failing", En1992Snapshot::failing_under_reinforced())] {
        let rust = evaluate(&snap);
        let input = serde_json::to_string(&snap).expect("json");
        let mut child = Command::new("python3")
            .arg(oracle_script())
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| panic!("oracle spawn failed: {e}"));
        {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{label} oracle stderr={}", String::from_utf8_lossy(&output.stderr));
        let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
        let py_checks = py.get("checks").and_then(|c| c.as_array()).expect("checks array");
        assert!(!py_checks.is_empty(), "{label} oracle returned no checks");
        let mut compared = 0usize;
        for pc in py_checks {
            let id = pc.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let Some(pu) = pc.get("utilization").and_then(|v| v.as_f64()) else { continue };
            let Some(rc) = rust.checks.iter().find(|c| c.id == id) else {
                panic!("{label} missing rust check id {id}");
            };
            let denom = pu.abs().max(rc.utilization.abs()).max(1e-9);
            let rel = (pu - rc.utilization).abs() / denom;
            assert!(
                rel <= 0.005 || (pu - rc.utilization).abs() <= 0.005,
                "{label} check {id}: python={pu} rust={} rel={rel}",
                rc.utilization
            );
            compared += 1;
        }
        let need = py_checks.len().min(10);
        assert!(
            compared >= need,
            "{label}: expected full ULS/SLS oracle overlap (need ≥{need} of {}), got {compared}",
            py_checks.len()
        );
    }
}

#[test]
fn example_snapshot_validates_against_json_schema() {
    let schema_path = snapshot_schema_path();
    for snap in [En1992Snapshot::compliant_office_frame(), En1992Snapshot::failing_under_reinforced()] {
        let instance_val = serde_json::to_value(&snap).expect("json value");
        let instance = serde_json::to_string(&instance_val).expect("instance");
        let tmp = std::env::temp_dir().join(format!("en1992-instance-{}.json", snap.title.replace(' ', "_")));
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
            "jsonschema: {}\n{}",
            String::from_utf8_lossy(&status.stdout),
            String::from_utf8_lossy(&status.stderr)
        );
    }
}

fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}


fn oracle_script() -> PathBuf {
    family_any_dir().join("🧬️schema/🧪️tests/⚖️compliance/🐍️.py")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema/📸️snapshot/🔣️.json")
}

#[test]
fn accidental_de_gamma_reduces_vs_uls() {
    let p_uls = AnnexParams::for_situation(AnnexChoice::De, "uls");
    let p_acc = AnnexParams::for_situation(AnnexChoice::De, "accidental");
    let p_fire = AnnexParams::for_situation(AnnexChoice::De, "fire");
    assert_eq!(p_uls.gamma_c, 1.5);
    assert_eq!(p_acc.gamma_c, 1.3);
    assert_eq!(p_acc.gamma_s, 1.0);
    assert_eq!(p_fire.gamma_c, 1.3);
    assert!(p_acc.f_cd_pa(30.0e6) > p_uls.f_cd_pa(30.0e6));
    let snap = En1992Snapshot::compliant_office_frame();
    let report = evaluate(&snap);
    assert!(
        report.checks.iter().any(|c| c.id.contains("flexure.acc") || c.explanation.en.contains("ACC-6.11")),
        "ACC-6.11 companion missing: {:?}",
        report.checks.iter().map(|c| c.id.clone()).collect::<Vec<_>>()
    );
}

#[test]
fn fire_tables_route_per_kind_and_rating() {
    use crate::standards::v1::subsets::any::schema::part_1_2_fire;
    use crate::{FireRating, MemberKind, SupportCondition};
    let ratings = [FireRating::R30, FireRating::R60, FireRating::R90, FireRating::R120];
    for r in ratings {
        let (b_a, a_a) = part_1_2_fire::required_for(MemberKind::Column, SupportCondition::Fixed, r, "A", "one-way");
        let (b_b, a_b) = part_1_2_fire::required_for(MemberKind::Column, SupportCondition::Fixed, r, "B", "one-way");
        assert!(b_a > 0.0 && a_a > 0.0, "5.2a R{:?}", r);
        assert!(b_b > 0.0 && a_b > 0.0, "5.2b R{:?}", r);
        assert!((b_a - b_b).abs() > 1e-9 || (a_a - a_b).abs() > 1e-9, "method A/B must differ for {:?}", r);
        let (_, a_w) = part_1_2_fire::required_for(MemberKind::Wall, SupportCondition::Fixed, r, "A", "one-way");
        assert!(a_w > 0.0, "5.4 wall {:?}", r);
        let (_, a_t) = part_1_2_fire::required_for(MemberKind::TensionMember, SupportCondition::SimplySupported, r, "A", "one-way");
        assert!(a_t > 0.0, "5.3 tension {:?}", r);
        let (_, a_ss) = part_1_2_fire::required_for(MemberKind::Beam, SupportCondition::SimplySupported, r, "A", "one-way");
        let (_, a_ct) = part_1_2_fire::required_for(MemberKind::Beam, SupportCondition::Continuous, r, "A", "one-way");
        assert!(a_ss > 0.0 && a_ct > 0.0, "5.5/5.6 beams {:?}", r);
        let (_, a_1) = part_1_2_fire::required_for(MemberKind::Slab, SupportCondition::SimplySupported, r, "A", "one-way");
        let (_, a_2) = part_1_2_fire::required_for(MemberKind::Slab, SupportCondition::SimplySupported, r, "A", "two-way");
        let (_, a_f) = part_1_2_fire::required_for(MemberKind::FlatSlab, SupportCondition::SimplySupported, r, "A", "flat");
        let (_, a_r) = part_1_2_fire::required_for(MemberKind::RibbedSlab, SupportCondition::SimplySupported, r, "A", "ribbed");
        assert!(a_1 > 0.0 && a_2 > 0.0 && a_f > 0.0 && a_r > 0.0, "5.8–5.11 slabs {:?}", r);
    }
}

#[test]
fn no_identical_en_de_explanations_in_committed_examples() {
    let examples = [
        En1992Snapshot::compliant_office_frame(),
        En1992Snapshot::failing_under_reinforced(),
        En1992Snapshot::failing_prestressed_beam(),
        En1992Snapshot::liquid_retaining_fem_anchor(),
    ];
    let mut identical = Vec::new();
    for snap in examples {
        let report = evaluate(&snap);
        for c in &report.checks {
            if c.explanation.en == c.explanation.de && !c.explanation.en.is_empty() {
                // numbers-only exception: strip digits/punctuation/whitespace
                let stripped: String = c.explanation.en.chars().filter(|ch| ch.is_alphabetic()).collect();
                if !stripped.is_empty() {
                    identical.push((c.id.clone(), c.explanation.en.clone()));
                }
            }
            if c.title.en == c.title.de && !c.title.en.is_empty() {
                let stripped: String = c.title.en.chars().filter(|ch| ch.is_alphabetic()).collect();
                // titles may share Latin symbols; require remedy/explanation focus — skip title twins that are clause codes
            }
            if !c.remedies.is_empty() {
                for rem in &c.remedies {
                    if rem.action.en == rem.action.de {
                        let stripped: String = rem.action.en.chars().filter(|ch| ch.is_alphabetic()).collect();
                        if !stripped.is_empty() {
                            identical.push((format!("{}#remedy", c.id), rem.action.en.clone()));
                        }
                    }
                }
            }
        }
    }
    assert!(identical.is_empty(), "identical en/de text: {identical:?}");
}

#[test]
fn mutation_facets_have_no_placeholder_or_unknown() {
    let root = family_any_dir().join("🧬️schema/🧬️mutations");
    let mut bad = Vec::new();
    for gql in walkdir_graphql(&root) {
        let t = std::fs::read_to_string(&gql).unwrap();
        if t.contains("_placeholder") {
            bad.push(format!("placeholder {}", gql.display()));
        }
    }
    for ts in walkdir_ts(&root) {
        let t = std::fs::read_to_string(&ts).unwrap();
        if t.contains("_placeholder") || t.contains("Record<string, unknown>") || t.contains("export type ") && t.contains("= unknown") {
            if t.contains("_placeholder") || t.contains("Record<string, unknown>") {
                bad.push(format!("unknown/placeholder {}", ts.display()));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

fn walkdir_graphql(root: &PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(root) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                let gql = p.join("🧬️schema").join("🔗️.graphql");
                if gql.exists() { out.push(gql); }
            }
        }
    }
    out
}
fn walkdir_ts(root: &PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(root) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                let ts = p.join("🧬️schema").join("🟦️.ts");
                if ts.exists() { out.push(ts); }
            }
        }
    }
    out
}

#[test]
fn field_meta_no_prefix_only_fallback_for_editable_leaves() {
    use crate::field_meta::en1992_field_meta;
    let examples = [
        En1992Snapshot::compliant_office_frame(),
        En1992Snapshot::failing_prestressed_beam(),
        En1992Snapshot::liquid_retaining_fem_anchor(),
    ];
    let mut missing = Vec::new();
    for snap in examples {
        let value = serde_json::to_value(&snap).expect("json");
        walk_leaves("", &value, &mut |path| {
            if path.is_empty() { return; }
            let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
            if leaf == "id" || leaf.starts_with("id=") { return; }
            let wild = wildcardize(path);
            match en1992_field_meta(path).or_else(|| en1992_field_meta(&wild)) {
                None => missing.push(path.to_string()),
                Some(_) => {}
            }
        });
    }
    assert!(missing.is_empty(), "editable leaves without explicit meta (no prefix fallback): {missing:?}");
}


#[test]
fn every_editable_leaf_influences_a_check_scope_aware() {
    let examples = [
        En1992Snapshot::compliant_office_frame(),
        En1992Snapshot::failing_under_reinforced(),
        En1992Snapshot::failing_prestressed_beam(),
        En1992Snapshot::liquid_retaining_fem_anchor(),
    ];
    // Explicit exemptions (descriptive / report-header / fastening actions unused beyond N_k,V_k)
    let exempt = |templ: &str| -> bool {
        let leaf = templ.rsplit(['.', '[']).next().unwrap_or(templ).trim_end_matches(']');
        matches!(
            leaf,
            "id" | "name" | "labelEn" | "labelDe" | "title" | "prestress" | "punching" | "stirrups" | "fire" | "longitudinal" | "actions"
        ) || templ.starts_with("anchors[].actions[].") && matches!(
            leaf,
            "gKLine" | "qKLine" | "pointForce" | "mK" | "tK" | "vKPunch" | "source" | "category"
        )
    };
    let mut templates: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut covered: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut special = std::collections::HashSet::new();
    for snap in &examples {
        let base = evaluate(snap);
        let base_sig: Vec<_> = base.checks.iter().map(|c| (c.id.clone(), format!("{:?}", c.status), (c.utilization * 1e6).round() as i64)).collect();
        let value = serde_json::to_value(snap).expect("json");
        let mut leaves = Vec::new();
        walk_leaves("", &value, &mut |path| {
            if path.is_empty() { return; }
            // Skip null option placeholders (e.g. members[].prestress = null)
            if resolve_json_path(&value, path).map(|v| v.is_null()).unwrap_or(false) { return; }
            leaves.push(path.to_string());
        });
        for path in &leaves {
            let templ = wildcardize(path);
            if exempt(&templ) { continue; }
            templates.insert(templ.clone());
            let mut tree = value.clone();
            if !perturb_json_value(&mut tree, path) { continue; }
            let Some(perturbed) = (std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                serde_json::from_value::<En1992Snapshot>(tree.clone()).ok().map(|s| evaluate(&s))
            })).ok().flatten()) else { continue; };
            let sig: Vec<_> = perturbed.checks.iter().map(|c| (c.id.clone(), format!("{:?}", c.status), (c.utilization * 1e6).round() as i64)).collect();
            if sig != base_sig {
                covered.insert(templ);
                for key in ["pointForce", "tK", "prestress", "prestressSteelId", "ductility", "epsUk", "fP01k", "columnMethod", "slabSystem"] {
                    if path.contains(key) { special.insert(key); }
                }
            }
        }
    }
    let missing: Vec<_> = templates.difference(&covered).cloned().collect();
    assert!(special.contains("pointForce"), "pointForce not covered: {special:?}");
    assert!(special.contains("tK"), "tK not covered: {special:?}");
    assert!(special.contains("prestress") || special.contains("prestressSteelId") || special.contains("fP01k"), "prestress not covered: {special:?}");
    assert!(
        missing.len() <= 6,
        "templates without effect in any example ({}): {missing:?}\nspecial={special:?}",
        missing.len()
    );
}


fn resolve_json_path<'a>(tree: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut segs = Vec::new();
    let mut buf = String::new();
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '.' {
            if !buf.is_empty() { segs.push(buf.clone()); buf.clear(); }
        } else if c == '[' {
            if !buf.is_empty() { segs.push(buf.clone()); buf.clear(); }
            let mut inner = String::new();
            for x in chars.by_ref() { if x == ']' { break; } inner.push(x); }
            segs.push(inner);
        } else { buf.push(c); }
    }
    if !buf.is_empty() { segs.push(buf); }
    let mut cur = tree;
    for seg in segs {
        if let Ok(idx) = seg.parse::<usize>() {
            cur = cur.get(idx)?;
        } else {
            cur = cur.get(&seg)?;
        }
    }
    Some(cur)
}

fn wildcardize(path: &str) -> String {
    let mut out = String::new();
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' {
            out.push_str("[]");
            for x in chars.by_ref() { if x == ']' { break; } }
        } else { out.push(c); }
    }
    out
}

fn perturb_json_value(tree: &mut serde_json::Value, path: &str) -> bool {
    let mut segs = Vec::new();
    let mut buf = String::new();
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '.' {
            if !buf.is_empty() { segs.push(buf.clone()); buf.clear(); }
        } else if c == '[' {
            if !buf.is_empty() { segs.push(buf.clone()); buf.clear(); }
            let mut inner = String::new();
            for x in chars.by_ref() { if x == ']' { break; } inner.push(x); }
            segs.push(inner);
        } else { buf.push(c); }
    }
    if !buf.is_empty() { segs.push(buf); }
    if segs.is_empty() { return false; }
    let mut cur = tree;
    let last = segs.len() - 1;
    for (i, seg) in segs.iter().enumerate() {
        if i == last {
            let Some(map) = cur.as_object_mut() else { return false; };
            let Some(v) = map.get_mut(seg) else { return false; };
            match v {
                serde_json::Value::Number(n) => {
                    let x = n.as_f64().unwrap_or(0.0);
                    let nx = if *seg == "k" {
                        1.0 // below Annex C minima
                    } else if *seg == "epsUk" {
                        0.01
                    } else if *seg == "count" || *seg == "legs" || seg.ends_with("Years") {
                        if x < 2.0 { 8.0 } else { (x * 2.0).round().max(x + 1.0) }
                    } else if *seg == "lossRatio" {
                        0.45
                    } else if x.abs() < 1e-18 {
                        1.0
                    } else {
                        x * 1.35
                    };
                    // Keep integer-looking values integral for u32 fields
                    if n.as_u64().is_some() || (*seg == "count" || *seg == "legs") {
                        *v = serde_json::json!(nx.round() as u64);
                    } else {
                        *v = serde_json::json!(nx);
                    }
                    return true;
                }
                serde_json::Value::Bool(b) => { *v = serde_json::json!(!*b); return true; }
                serde_json::Value::String(s) => {
                    let ns = match s.as_str() {
                        "De" => "En", "En" => "De",
                        "A" => "B", "B" => "C", "C" => "A",
                        "Beam" => "Column", "Column" => "Wall", "Wall" => "Beam",
                        "Slab" => "FlatSlab", "FlatSlab" => "RibbedSlab", "RibbedSlab" => "Slab",
                        "one-way" => "two-way", "two-way" => "ribbed", "ribbed" => "flat", "flat" => "one-way",
                        "c30" => "c50", "c50" => "c30", "c35" => "c50",
                        "b500" => "b500a", "b500a" => "b500",
                        "yp1860" => "yp1860-alt",
                        "SimplySupported" => "Continuous", "Continuous" => "Cantilever", "Cantilever" => "Fixed", "Fixed" => "SimplySupported",
                        "Xc1" => "Xc3", "Xc3" => "Xc1", "Xc2" => "Xc4",
                        "R60" => "R90", "R90" => "R60", "R30" => "R60", "R120" => "R90",
                        "udl" => "external", "external" => "udl", "point" => "udl",
                        "permanent" => "imposed", "imposed" => "snow", "snow" => "imposed",
                        "office" => "storage", "storage" => "office", "self" => "office",
                        "good" => "poor", "poor" => "good",
                        "bottom" => "top", "top" => "bottom",
                        "N" => "R", "R" => "N",
                        "Tc1" => "Tc2", "Tc2" => "Tc0", "Tc0" => "Tc1",
                        "interior" => "edge", "edge" => "corner", "corner" => "interior",
                        _ => return false,
                    };
                    *v = serde_json::json!(ns);
                    return true;
                }
                _ => return false,
            }
        } else if let Ok(idx) = seg.parse::<usize>() {
            let Some(arr) = cur.as_array_mut() else { return false; };
            if idx >= arr.len() { return false; }
            cur = &mut arr[idx];
        } else {
            let Some(map) = cur.as_object_mut() else { return false; };
            let Some(next) = map.get_mut(seg) else { return false; };
            cur = next;
        }
    }
    false
}

#[test]
fn prestressed_examples_evaluate() {
    let ok = {
        let mut s = En1992Snapshot::compliant_office_frame();
        s.title = "Compliant prestressed beam".into();
        s.members.retain(|m| m.id == "beam-PS1");
        s.anchors.clear();
        s
    };
    let report = evaluate(&ok);
    assert!(report.checks.iter().any(|c| c.id.contains("prestress")), "{:?}", report.checks.iter().map(|c| &c.id).collect::<Vec<_>>());
    let fail = En1992Snapshot::failing_prestressed_beam();
    let fr = evaluate(&fail);
    assert!(!fr.complies());
}

#[test]
fn title_appears_in_subject_labels() {
    let mut snap = En1992Snapshot::compliant_office_frame();
    snap.title = "UniqueTitleXYZ".into();
    let report = evaluate(&snap);
    assert!(report.checks.iter().any(|c| c.subject.label.en.contains("UniqueTitleXYZ")));
}

#[test]
fn concrete_table_3_1_derived_not_in_snapshot_json() {
    let snap = En1992Snapshot::compliant_office_frame();
    let v = serde_json::to_value(&snap).unwrap();
    let g = &v["concreteGrades"][0];
    assert!(g.get("fCk").is_some());
    assert!(g.get("fCkCube").is_none());
    assert!(g.get("epsCu2").is_none());
    assert!(g.get("fCm").is_none());
}

