//! ⚖️ Numeric worked examples + path/remedy/perturb laws for EN 1996 masonry checks.

use crate::app_surface::{get_value_at_path, parse_path, set_value_at_path};
use crate::artifact_schema::{evaluate_building, f_k_pa, fire_min_thickness_m, fk_factors, phi_m, phi_s, rho_n, slenderness};
use crate::artifact_schema::snapshot::text::parse_dsl;
use crate::document::{AnnexChoice, CheckReport, CheckStatus};
use crate::{En1996Snapshot, MasonryClass, UnitMaterial};
use crate::field_meta::en1996_field_meta;

#[test]
fn de_na_fk_clay_group1_m10() {
    let doc = En1996Snapshot::compliant_clay_wall();
    let w = &doc.walls[0];
    let f = fk_factors(AnnexChoice::De, w.unit_material, w.unit_group, w.mortar_type);
    assert!((f.k - 0.60).abs() < 1e-9);
    assert!((f.alpha - 0.65).abs() < 1e-9);
    assert!((f.beta - 0.25).abs() < 1e-9);
    let got = f_k_pa(AnnexChoice::De, w) / 1e6;
    assert!(got > 5.0 && got < 12.0, "f_k got {got}");
}

#[test]
fn en_vs_de_fk_divergence() {
    let doc = En1996Snapshot::compliant_clay_wall();
    let w = &doc.walls[0];
    assert!((f_k_pa(AnnexChoice::De, w) - f_k_pa(AnnexChoice::En, w)).abs() > 1e5);
}

#[test]
fn de_na_gamma_m_by_category_and_execution_class() {
    assert!((MasonryClass::Class1.gamma_m_de(false) - 1.5).abs() < 1e-12);
    assert!((MasonryClass::Class2.gamma_m_de(false) - 1.7).abs() < 1e-12);
    assert!((MasonryClass::Class3.gamma_m_de(false) - 1.7).abs() < 1e-12);
    assert!((MasonryClass::Class4.gamma_m_de(false) - 2.0).abs() < 1e-12);
    assert!((MasonryClass::Class1.gamma_m_de(true) - 1.3).abs() < 1e-12);
}

#[test]
fn phi_s_de_vs_en_na_simplified() {
    let lambda = 10.0;
    assert!((phi_s(AnnexChoice::En, lambda) - (0.85 - 0.0011 * 100.0)).abs() < 1e-9);
    assert!((phi_s(AnnexChoice::De, lambda) - (0.70 - 0.0011 * 100.0)).abs() < 1e-9);
}

#[test]
fn annex_g_phi_m_midheight_na_method() {
    let phi = phi_m(12.0, 0.02, 0.365);
    assert!(phi > 0.5 && phi < 1.0, "phi_m={phi}");
}

#[test]
fn fire_min_thickness_rei90_clay_and_aac_tabulated() {
    assert!((fire_min_thickness_m(90, UnitMaterial::Clay) - 0.140).abs() < 1e-9);
    assert!((fire_min_thickness_m(90, UnitMaterial::Aerated) - 0.175).abs() < 1e-9);
}

#[test]
fn simplified_method_storey_limit_de_na() {
    let mut doc = En1996Snapshot::compliant_clay_wall();
    doc.storeys = 4;
    let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
    let simp = report.checks.iter().find(|c| c.id.contains("simplified")).expect("simplified check");
    assert!(matches!(simp.status, CheckStatus::NotApplicable), "storeys>3 must be NotApplicable, got {:?}", simp.status);
}

#[test]
fn compliant_evaluate_passes() {
    let doc = En1996Snapshot::compliant_clay_wall();
    let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
    assert!(report.complies(), "{:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}

#[test]
fn every_emitted_path_parses_and_resolves_with_id_selectors() {
    let doc = En1996Snapshot::noncompliant_multi_fail();
    let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
    let snap_val = dsl::ToValue::to_value(&doc);
    for check in &report.checks {
        if !check.subject.path.is_empty() {
            parse_path(&check.subject.path).unwrap_or_else(|e| panic!("subject path {}: {e}", check.subject.path));
            get_value_at_path(&snap_val, &check.subject.path).unwrap_or_else(|e| panic!("resolve {}: {e}", check.subject.path));
            assert!(check.subject.path.contains("[id="), "path {}", check.subject.path);
        }
        for remedy in &check.remedies {
            if remedy.target.path.is_empty() { continue; }
            parse_path(&remedy.target.path).unwrap_or_else(|e| panic!("remedy path {}: {e}", remedy.target.path));
            get_value_at_path(&snap_val, &remedy.target.path).unwrap_or_else(|e| panic!("resolve remedy {}: {e}", remedy.target.path));
            assert!(remedy.target.path.contains("[id="), "expected id selector in {}", remedy.target.path);
        }
    }
}

#[test]
fn remedy_law_flips_at_least_two_distinct_fails_to_pass() {
    let doc = En1996Snapshot::noncompliant_multi_fail();
    let before = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
    assert!(!before.complies());
    let fails: Vec<_> = before.failing().cloned().collect();
    assert!(fails.len() >= 2);
    let mut flipped = Vec::new();
    for fail in &fails {
        if flipped.len() >= 2 { break; }
        let Some(remedy) = fail.remedies.iter().find(|r| r.applicable && !r.target.path.is_empty() && r.options.is_empty()) else { continue };
        let mut tree = dsl::ToValue::to_value(&doc);
        if set_value_at_path(&mut tree, &remedy.target.path, dsl::DslValue::float(remedy.required.value)).is_err() { continue; }
        let Ok(trial) = dsl::FromValue::from_value(tree) else { continue };
        let trial: En1996Snapshot = trial;
        let after = evaluate_building(trial.annex, trial.masonry_class, trial.design_situation, trial.storeys, &trial.walls);
        if let Some(c) = after.checks.iter().find(|c| c.id == fail.id) {
            if matches!(c.status, CheckStatus::Pass) { flipped.push(fail.id.clone()); }
        }
    }
    assert!(flipped.len() >= 2, "flipped={flipped:?}");
}

#[test]
fn every_editable_leaf_has_en_de_field_meta() {
    let wildcards = [
        "annex", "masonryClass", "designSituation", "storeys",
        "walls[].thicknessM", "walls[].mortarStrengthPa", "walls[].densityKgM3", "walls[].phiInfinity", "walls[].isBasement", "walls[].mu",
        "walls[].slabBearingDepthM", "walls[].unitHeightM", "walls[].asHorizontalM2",
        "walls[].openings[].heightM", "walls[].openings[].sillHeightM",
        "walls[].loadCases[].designSituation", "walls[].loadCases[].imposedCategory",
        "walls[].loadCases[].gKSlabN", "walls[].loadCases[].qKImposedPa", "walls[].loadCases[].tributaryAreaM2",
        "walls[].loadCases[].slabSpanM", "walls[].loadCases[].qKSnowPa", "walls[].loadCases[].qPWindPa",
        "walls[].loadCases[].cPe", "walls[].loadCases[].hKEarthN",
    ];
    for path in wildcards {
        let meta = en1996_field_meta(path).unwrap_or_else(|| panic!("missing field meta for {path}"));
        assert!(!meta.label_en.is_empty());
        assert!(!meta.label_de.is_empty());
        if path.ends_with("designSituation") || path.ends_with("imposedCategory") {
            assert!(meta.choices.is_some(), "choices required for {path}");
        }
    }
}

#[test]
fn perturb_every_editable_leaf_changes_some_check() {
    fn sig(report: &CheckReport) -> Vec<(String, String, i64, i64, i64)> {
        report
            .checks
            .iter()
            .map(|c| {
                (
                    c.id.clone(),
                    format!("{:?}", c.status),
                    (c.utilization * 1e9).round() as i64,
                    (c.computed.value * 1e3).round() as i64,
                    (c.limit.value * 1e3).round() as i64,
                )
            })
            .collect()
    }
    fn walk(prefix: &str, value: &serde_json::Value, leaves: &mut Vec<(String, serde_json::Value)>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    match v {
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => walk(&path, v, leaves),
                        _ => {
                            if matches!(k.as_str(), "id" | "labelEn" | "labelDe") {
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
    fn perturb_doc(base: &En1996Snapshot) -> Vec<String> {
        let base_report = evaluate_building(base.annex, base.masonry_class, base.design_situation, base.storeys, &base.walls);
        let base_sig = sig(&base_report);
        let value = serde_json::to_value(base).expect("json");
        let mut leaves: Vec<(String, serde_json::Value)> = Vec::new();
        walk("", &value, &mut leaves);
        assert!(!leaves.is_empty(), "expected editable leaves");
        let mut unchanged = Vec::new();
        for (path, cur) in &leaves {
            let mut tree = value.clone();
            let next = match cur {
                serde_json::Value::Bool(b) => serde_json::Value::Bool(!b),
                serde_json::Value::Number(n) => {
                    let v = n.as_f64().unwrap_or(0.0);
                    let bumped = if v.abs() < 1e-12 {
                        1.0
                    } else if path.contains("bedJoint") {
                        v * 2.0 + 0.01
                    } else if path.contains("mu") {
                        (v * 0.1).max(0.01)
                    } else if path.contains("slabSpan") {
                        (v * 1.35).min(6.0).max(v + 0.5)
                    } else {
                        v * 1.5 + (if v.abs() < 1.0 { 0.05 } else { 0.0 })
                    };
                    serde_json::json!(bumped)
                }
                serde_json::Value::String(s) => {
                    let alt = if path.ends_with("annex") {
                        if s == "de" { "en" } else { "de" }.to_string()
                    } else if path.ends_with("masonryClass") {
                        if s == "class1" { "class3" } else { "class1" }.to_string()
                    } else if path.ends_with("designSituation") {
                        if s == "persistent" { "transient" } else { "persistent" }.to_string()
                    } else if path.ends_with("wallType") {
                        if s == "loadBearing" { "shear" } else { "loadBearing" }.to_string()
                    } else if path.ends_with("unitMaterial") {
                        if s == "clay" { "concrete" } else { "clay" }.to_string()
                    } else if path.ends_with("unitGroup") {
                        if s == "group1" { "group2" } else { "group1" }.to_string()
                    } else if path.ends_with("mortarType") {
                        if s == "generalPurpose" { "thinLayer" } else { "generalPurpose" }.to_string()
                    } else if path.ends_with("mortarClass") {
                        if s == "M10" { "M5" } else { "M10" }.to_string()
                    } else if path.ends_with("exposure") {
                        if s == "Mx1" { "Mx3" } else { "Mx1" }.to_string()
                    } else if path.ends_with("imposedCategory") {
                        if s == "A" { "C" } else { "A" }.to_string()
                    } else {
                        format!("{s}-x")
                    };
                    serde_json::Value::String(alt)
                }
                _ => continue,
            };
            if !set_json_at_path(&mut tree, path, next) {
                continue;
            }
            let Ok(trial) = serde_json::from_value::<En1996Snapshot>(tree) else { continue };
            let report = evaluate_building(trial.annex, trial.masonry_class, trial.design_situation, trial.storeys, &trial.walls);
            if sig(&report) == base_sig {
                unchanged.push(path.clone());
            }
        }
        unchanged
    }
    let scopes = [
        ("compliant", En1996Snapshot::compliant_clay_wall()),
        ("openings", En1996Snapshot::opening_wall_example()),
        ("basement", En1996Snapshot::basement_wall_example()),
        ("concentrated", En1996Snapshot::concentrated_load_example()),
        ("reinforced", En1996Snapshot::reinforced_wall_example()),
    ];
    for (name, doc) in &scopes {
        let unchanged = perturb_doc(doc);
        assert!(
            unchanged.is_empty(),
            "scope {name}: perturbing these editable leaves did not change status/computed/limit/utilization: {unchanged:?}"
        );
    }
}

fn set_json_at_path(root: &mut serde_json::Value, path: &str, value: serde_json::Value) -> bool {
    let mut cur = root;
    let parts: Vec<&str> = path.split('.').collect();
    for (i, part) in parts.iter().enumerate() {
        let last = i + 1 == parts.len();
        if let Some((name, idx_s)) = part.split_once('[') {
            let idx: usize = idx_s.trim_end_matches(']').parse().unwrap_or(usize::MAX);
            if !name.is_empty() {
                cur = match cur.get_mut(name) {
                    Some(v) => v,
                    None => return false,
                };
            }
            cur = match cur.as_array_mut().and_then(|a| a.get_mut(idx)) {
                Some(v) => v,
                None => return false,
            };
            if last {
                *cur = value;
                return true;
            }
        } else if last {
            match cur.as_object_mut() {
                Some(map) => {
                    map.insert((*part).to_string(), value);
                    return true;
                }
                None => return false,
            }
        } else {
            cur = match cur.get_mut(*part) {
                Some(v) => v,
                None => return false,
            };
        }
    }
    false
}

#[test]
fn committed_dsl_pack_assets_parse_and_assert_verdicts() {
    use std::path::PathBuf;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
    for (folder, expect_pass) in [("🧱️loadbearing-wall", true), ("❌️multi-fail-masonry", false)] {
        let dir = root.join(folder).join(folder);
        let dsl = std::fs::read_to_string(dir.join("🗣️.dsl.semio"))
            .unwrap_or_else(|e| panic!("missing committed DSL at {}: {e}", dir.display()));
        let pack = std::fs::read(dir.join("🎒️.pack.semio"))
            .unwrap_or_else(|e| panic!("missing committed pack at {}: {e}", dir.display()));
        let parsed = parse_dsl(&dsl).expect("parse DSL");
        let from_pack = <En1996Snapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode pack");
        assert_eq!(parsed.walls.len(), from_pack.walls.len(), "{folder}");
        let report = evaluate_building(parsed.annex, parsed.masonry_class, parsed.design_situation, parsed.storeys, &parsed.walls);
        assert_eq!(report.complies(), expect_pass, "{folder}");
        if !expect_pass {
            assert!(report.summary.fail >= 2, "{folder}");
        }
    }
}

#[test]
fn mutation_and_check_labels_differ_en_de() {
    use std::path::PathBuf;
    fn numbers_only(s: &str) -> bool {
        !s.is_empty()
            && s.chars().all(|c| {
                c.is_ascii_digit()
                    || c.is_whitespace()
                    || matches!(c, '.' | ',' | '-' | '+' | '=' | '≤' | '≥' | '(' | ')' | '/' | '°' | '×' | '·' | ':' | ';')
            })
    }
    fn assert_pair(en: &str, de: &str, ctx: &str) {
        if en == de && !numbers_only(en) {
            panic!("identical en/de for {ctx}: {en:?}");
        }
    }
    let mut_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    for entry in std::fs::read_dir(&mut_root).expect("mutations dir") {
        let entry = entry.expect("entry");
        let rs = entry.path().join("🦀️.rs");
        if !rs.is_file() {
            continue;
        }
        let text = std::fs::read_to_string(&rs).expect("read mutation");
        let key = "LocalizedLabel::native("";
        let mut from = 0;
        while let Some(rel) = text[from..].find(key) {
            let start = from + rel + key.len();
            let rest = &text[start..];
            let Some(en_end) = rest.find('"') else { break };
            let en = &rest[..en_end];
            let after_en = &rest[en_end + 1..];
            let Some(de_rel) = after_en.find('"') else { break };
            let after_de0 = &after_en[de_rel + 1..];
            let Some(de_end) = after_de0.find('"') else { break };
            let de = &after_de0[..de_end];
            assert_pair(en, de, entry.file_name().to_string_lossy().as_ref());
            from = start + en_end + de_rel + de_end + 2;
        }
    }
    let doc = En1996Snapshot::compliant_clay_wall();
    let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
    for c in &report.checks {
        assert_pair(&c.title.en, &c.title.de, &format!("title {}", c.id));
        assert_pair(&c.explanation.en, &c.explanation.de, &format!("explanation {}", c.id));
    }
}

#[test]
fn slenderness_of_compliant_within_27() {
    assert!(slenderness(&En1996Snapshot::compliant_clay_wall().walls[0]) <= 27.0);
}

#[test]
fn noncompliant_has_multiple_fails() {
    let doc = En1996Snapshot::noncompliant_multi_fail();
    let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
    assert!(report.summary.fail >= 3);
}

#[test]
#[test]
fn typed_diff_and_snapshot_facets_have_no_unknown() {
    use std::path::PathBuf;
    let schema = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    let files = [
        "🔺️diff/🟦️.ts",
        "📸️snapshot/🟦️.ts",
        "🧬️mutations/🟦️.ts",
        "🟦️.ts",
    ];
    let mut bad = Vec::new();
    for rel in files {
        let path = schema.join(rel);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for needle in ["unknown[]", "Record<string, unknown>", "Record<string,unknown>", "_placeholder"] {
            if text.contains(needle) {
                bad.push(format!("{rel} contains {needle}"));
            }
        }
        // Bare `: unknown` / `?: unknown` on exported schema fields (not parse guards).
        if rel.ends_with("🔺️diff/🟦️.ts") || rel.ends_with("📸️snapshot/🟦️.ts") {
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("//") || trimmed.starts_with("*") || trimmed.starts_with("import") {
                    continue;
                }
                if ": unknown" in trimmed || "?: unknown" in trimmed {
                    bad.push(format!("{rel} line uses bare unknown: {trimmed}"));
                }
            }
        }
    }
    let snap = std::fs::read_to_string(schema.join("📸️snapshot/🟦️.ts")).expect("snapshot ts");
    for field in [
        "MasonryWall",
        "WallOpening",
        "WallLoadCase",
        "ConcentratedLoad",
        "slabSpanM",
        "mortarStrengthPa",
        "isBasement",
        "openings",
        "concentrated",
    ] {
        assert!(snap.contains(field), "snapshot TS missing {field}");
    }
    let diff = std::fs::read_to_string(schema.join("🔺️diff/🟦️.ts")).expect("diff ts");
    assert!(diff.contains("MasonryWall"), "diff TS must type walls via MasonryWall");
    assert!(!diff.contains("unknown[]"), "diff TS still has unknown[]");
    assert!(bad.is_empty(), "facet parity failures: {bad:?}");
}

fn rho_n_four_sided_less_than_two() {
    assert!(rho_n(4, 2.75, 5.0) < rho_n(2, 2.75, 5.0));
}
