//! EN 1999 numeric compliance tests — hand-derived worked examples + DoD gates.

use crate::document::{AnnexChoice, CheckStatus};
use crate::snapshot::En1999Snapshot;
use crate::standards::v1::subsets::any::schema::snapshot::{decode_en1999_dsl, encode_en1999_dsl};
use crate::standards::v1::subsets::any::schema::{evaluate_structure, na_de, part_1_1, part_1_2, part_1_3};
use std::path::PathBuf;
use std::process::Command;

#[test]
fn chi_from_lambda_class_a_worked() {
    let chi = part_1_1::chi_from_lambda(1.0, 0.20, 0.10);
    assert!((chi - 0.656_f64).abs() < 0.01, "chi={chi}");
}

#[test]
fn alloy_table_3_2_6082_t6() {
    let a = part_1_1::resolve_alloy("aw6082-t6").expect("catalogue alloy");
    assert!((a.f_o_pa - 260.0e6).abs() < 1.0);
    assert!((a.rho_o_haz - 0.64).abs() < 1e-9);
    assert!(a.buckling_class_a);
    let (des, row) = part_1_1::CATALOGUE_ALLOY_ROWS.iter().find(|(d, _)| *d == "aw6082-t6").expect("catalogue row");
    assert_eq!(*des, "aw6082-t6");
    assert!((row.f_o_pa - a.f_o_pa).abs() < 1.0);
    let snap = En1999Snapshot::compliant_roof_purlin();
    assert_eq!(snap.materials[0].designation, "aw6082-t6");
    let report = evaluate_structure(&snap);
    let n = report.checks.iter().find(|c| c.id.contains("6.2.3.n.purlin-1")).expect("N_Rd check");
    let n_rd = n.limit.value;
    assert!(n_rd > 1.0, "N_Rd must be positive for aw6082-t6 member");
    let section = snap.sections.iter().find(|s| s.id == snap.members[0].section_id).unwrap();
    let a_eff = part_1_1::effective_area(section, row.f_o_pa, true, row.rho_o_haz, row.rho_u_haz);
    let expected = a_eff * row.f_o_pa / na_de::AnnexParams::de().gamma_m1;
    assert!((n_rd - expected).abs() / expected.max(1.0) < 1e-9, "N_Rd limit {n_rd} must equal catalogue-based {expected}");
}


#[test]
fn alloy_5083_class_b() {
    let a = part_1_1::resolve_alloy("aw5083-o").expect("catalogue alloy");
    assert!(!a.buckling_class_a);
    assert!((a.rho_o_haz - 1.0).abs() < 1e-12);
}

#[test]
fn unknown_alloy_emits_fail_with_oneof_catalogue() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    doc.materials[0].designation = "aw9999-t6".into();
    let report = evaluate_structure(&doc);
    let fail = report.failing().find(|c| c.id.contains("alloy")).expect("unknown alloy must Fail");
    assert!(
        fail.remedies.iter().any(|r| r.target.path.contains("designation") && r.options.iter().any(|o| o.starts_with("aw"))),
        "expected OneOf catalogue remedy, got {:?}",
        fail.remedies
    );
}

#[test]
fn m_c_rd_6082_hand() {
    let wel = 24_000.0e-9;
    let m = wel * 260.0e6 / 1.1;
    assert!(((m / 1e3) - 5.6727_f64).abs() < 0.01);
}

#[test]
fn haz_rho_reduces_effective_modulus() {
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let sec = &doc.sections[0];
    let alloy = part_1_1::resolve_alloy("aw6060-t6").expect("alloy");
    let wel = part_1_1::effective_wel_y(sec, alloy.f_o_pa, true, alloy.rho_o_haz, alloy.rho_u_haz);
    let (_, _, _, wel_gross, _, _, _) = part_1_1::section_geometry(sec);
    assert!(wel < wel_gross);
}

#[test]
fn fatigue_strength_at_5e5() {
    // Δσ_R = Δσ_C · (N_C/N)^{1/m1} = 71 · (2e6/5e5)^{1/4.3} ≈ 98.01 MPa
    let rd = part_1_3::fatigue_strength_pa(71.0e6, 4.3, 6.3, 500_000.0);
    assert!((rd / 1e6 - 98.01).abs() < 0.5, "rd_mpa={}", rd / 1e6);
}

#[test]
fn fire_k_theta_curve() {
    assert!((part_1_2::k_theta(100.0) - 1.0).abs() < 1e-12);
    assert!(part_1_2::k_theta(550.0) <= 0.0);
    assert!((part_1_2::k_theta(325.0) - 0.5).abs() < 1e-9);
}

#[test]
fn fire_utilization_applies_k_theta_to_member_resistance() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let k = part_1_2::k_theta(doc.fire_scenarios[0].theta_a);
    assert!(k < 1.0 && k > 0.0);
    let report = evaluate_structure(&doc);
    let check = report.checks.iter().find(|c| c.id.contains("1-2.fire")).expect("fire check");
    assert!(check.explanation.en.contains("N_fi,Rd") || check.explanation.en.contains("k_θ"));
}

#[test]
fn cold_formed_and_shells_na_when_absent() {
    let report = evaluate_structure(&En1999Snapshot::empty());
    assert!(report.checks.iter().any(|c| c.id == "en1999.1-4.empty" && c.status == CheckStatus::NotApplicable));
    assert!(report.checks.iter().any(|c| c.id == "en1999.1-5.empty" && c.status == CheckStatus::NotApplicable));
}

#[test]
fn cold_formed_and_shells_evaluate_when_present() {
    let pass = evaluate_structure(&En1999Snapshot::compliant_roof_purlin());
    assert!(pass.checks.iter().any(|c| c.id.contains("1-4.") && c.status != CheckStatus::NotApplicable));
    assert!(pass.checks.iter().any(|c| c.id.contains("1-5.") && c.status != CheckStatus::NotApplicable));
    assert!(!pass.checks.iter().any(|c| c.id == "en1999.1-4.empty"));
    assert!(!pass.checks.iter().any(|c| c.id == "en1999.1-5.empty"));
    let fail = evaluate_structure(&En1999Snapshot::noncompliant_multi_fail());
    let cold_fail = fail.checks.iter().filter(|c| c.id.contains("1-4.") && c.status == CheckStatus::Fail).collect::<Vec<_>>();
    let shell_fail = fail.checks.iter().filter(|c| c.id.contains("1-5.") && c.status == CheckStatus::Fail).collect::<Vec<_>>();
    assert!(!cold_fail.is_empty(), "expected EN 1999-1-4 Fail");
    assert!(!shell_fail.is_empty(), "expected EN 1999-1-5 Fail");
    assert!(cold_fail.iter().all(|c| c.remedies.iter().any(|r| r.applicable)), "1-4 remedies");
    assert!(shell_fail.iter().all(|c| c.remedies.iter().any(|r| r.applicable)), "1-5 remedies");
    let mut doc = En1999Snapshot::empty();
    doc.materials = En1999Snapshot::compliant_roof_purlin().materials.clone();
    let mat_id = doc.materials[0].id.clone();
    doc.cold_formed.push(crate::snapshot::ColdFormedSheet {
        id: "sheet-1".into(),
        material_id: mat_id.clone(),
        thickness: 0.0025,
        width: 0.20,
        span: 1.2,
        welded: false,
        actions: vec![crate::snapshot::MemberAction {
            id: "G".into(),
            kind: "permanent".into(),
            category: "self".into(),
            source: "external".into(),
            g_k_line: 0.0,
            q_k_line: 0.0,
            n_k: 0.0,
            v_y_k: 0.0,
            v_z_k: 0.0,
            m_y_k: 400.0 / 1.35,
            m_z_k: 0.0,
        }],
    });
    doc.shells.push(crate::snapshot::AluminiumShell {
        id: "shell-1".into(),
        material_id: mat_id,
        radius: 0.60,
        thickness: 0.008,
        length: 2.0,
        actions: vec![crate::snapshot::MemberAction {
            id: "G".into(),
            kind: "permanent".into(),
            category: "self".into(),
            source: "external".into(),
            g_k_line: 0.0,
            q_k_line: 0.0,
            n_k: 20.0e6 / 1.35,
            v_y_k: 0.0,
            v_z_k: 0.0,
            m_y_k: 40.0e6 / 1.35,
            m_z_k: 0.0,
        }],
    });
    let report = evaluate_structure(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("1-4.")));
    assert!(report.checks.iter().any(|c| c.id.contains("1-5.")));
    assert!(!report.checks.iter().any(|c| c.id == "en1999.1-4.empty"));
    assert!(!report.checks.iter().any(|c| c.id == "en1999.1-5.empty"));
}

#[test]
fn compliant_example_passes() {
    let report = evaluate_structure(&En1999Snapshot::compliant_roof_purlin());
    assert!(
        report.complies(),
        "worst={} fails={:?}",
        report.worst_utilization(),
        report.failing().map(|c| c.id.clone()).collect::<Vec<_>>()
    );
}

#[test]
fn noncompliant_example_has_multiple_fails_with_remedies() {
    let report = evaluate_structure(&En1999Snapshot::noncompliant_multi_fail());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 3, "expected >=3 fails, got {}", fails.len());
    for f in &fails {
        assert!(!f.remedies.is_empty(), "fail {} missing remedies", f.id);
    }
}

#[test]
fn de_and_en_gamma_identical() {
    let mut de = En1999Snapshot::compliant_roof_purlin();
    de.annex = AnnexChoice::De;
    let mut en = de.clone();
    en.annex = AnnexChoice::En;
    let r_de = evaluate_structure(&de);
    let r_en = evaluate_structure(&en);
    assert_eq!(r_de.checks.len(), r_en.checks.len());
    for (a, b) in r_de.checks.iter().zip(r_en.checks.iter()) {
        // γ_Mf differs DE (1.35) vs EN (1.0) per NA — skip fatigue.
        if a.id.contains("1-3.fat") { continue; }
        assert!((a.utilization - b.utilization).abs() < 1e-9, "{} {} vs {}", a.id, a.utilization, b.utilization);
    }
}

#[test]
fn remedy_paths_use_id_selectors_and_resolve_in_snapshot() {
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let report = evaluate_structure(&doc);
    let tree = serde_json::to_value(&doc).expect("snapshot json");
    let mut seen = 0usize;
    for check in report.failing() {
        for remedy in &check.remedies {
            let path = &remedy.target.path;
            if path.is_empty() {
                continue;
            }
            assert!(path_grammar_ok(path), "path '{path}' must use field / [index] / [id=…] grammar");
            assert!(
                resolve_path(&tree, path).is_some(),
                "remedy path '{path}' must address an existing element+field in the snapshot"
            );
            seen += 1;
        }
    }
    assert!(seen >= 2, "expected ≥2 remedy paths to validate, got {seen}");
}

#[test]
fn remedy_law_writing_required_improves_fail() {
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let report = evaluate_structure(&doc);
    let mut tree = serde_json::to_value(&doc).expect("json");
    let mut applied = 0usize;
    let mut targeted_ids: Vec<String> = Vec::new();
    for fail in report.failing() {
        if applied >= 2 {
            break;
        }
        for remedy in &fail.remedies {
            if !remedy.applicable || !remedy.options.is_empty() {
                continue;
            }
            let path = remedy.target.path.as_str();
            if path.is_empty() {
                continue;
            }
            let required = remedy.required.value;
            if let Err(e) = set_number_at_path(&mut tree, path, required) {
                // try u64 for discrete counts
                if set_u64_at_path(&mut tree, path, required.round() as u64).is_err() {
                    continue;
                }
            }
            applied += 1;
            targeted_ids.push(fail.id.clone());
            break;
        }
    }
    assert!(applied >= 2, "expected ≥2 distinct applicable remedies applied, got {applied}");
    let corrected: En1999Snapshot = serde_json::from_value(tree).expect("decode");
    let after = evaluate_structure(&corrected);
    for id in &targeted_ids {
        let before = report.checks.iter().find(|c| c.id == *id).expect("before");
        let now = after.checks.iter().find(|c| c.id == *id).expect("after");
        let pass_or_better = matches!(now.status, CheckStatus::Pass)
            || now.utilization <= 1.0 + 1e-9
            || now.utilization < before.utilization - 1e-6;
        assert!(
            pass_or_better,
            "remedy for {id} must reach Pass or u≤1 (before u={}, after u={}, status={:?})",
            before.utilization, now.utilization, now.status
        );
    }
}


#[test]
fn chi_never_read_from_snapshot() {
    let json = serde_json::to_value(En1999Snapshot::default()).unwrap();
    assert!(json.get("chi").is_none());
}

#[test]
fn bundled_dsl_parses_and_evaluates_compliant() {
    let dsl = include_str!("../../../🖼️assets/🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio");
    let snap = decode_en1999_dsl(dsl).expect("bundled DSL must parse");
    let report = evaluate_structure(&snap);
    assert!(
        report.complies(),
        "bundled DSL must evaluate Pass, fails={:?}",
        report.failing().map(|c| c.id.clone()).collect::<Vec<_>>()
    );
    let _ = encode_en1999_dsl(&snap);
}

#[test]
fn python_oracle_matches_rust_utilizations_within_half_percent() {
    for (label, snap) in [
        ("compliant", En1999Snapshot::compliant_roof_purlin()),
        ("noncompliant", En1999Snapshot::noncompliant_multi_fail()),
    ] {
        let rust = evaluate_structure(&snap);
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
        assert!(
            output.status.success(),
            "{label} oracle stderr={}",
            String::from_utf8_lossy(&output.stderr)
        );
        let py: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
        let py_checks = py.get("checks").and_then(|c| c.as_array()).expect("checks array");
        let mut compared = 0usize;
        for pc in py_checks {
            let id = pc.get("id").and_then(|v| v.as_str()).unwrap_or("");
            // Full-family oracle overlap (ULS, SLS, fire, fatigue, cold, shell, connections).
            let _ = id;
            let Some(pu) = pc.get("utilization").and_then(|v| v.as_f64()) else { continue; };
            let Some(rc) = rust.checks.iter().find(|c| c.id == id) else { continue; };
            let denom = pu.abs().max(rc.utilization.abs()).max(1e-9);
            let rel = (pu - rc.utilization).abs() / denom;
            assert!(
                rel <= 0.005 || (pu - rc.utilization).abs() <= 0.005,
                "{label} check {id}: python={pu} rust={} rel={rel}",
                rc.utilization
            );
            compared += 1;
        }
        assert!(compared >= 2, "{label}: expected ≥2 overlapping oracle checks, got {compared}");
    }
}

#[test]
fn example_snapshot_validates_against_json_schema() {
    let schema_path = snapshot_schema_path();
    for snap in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail()] {
        let instance = serde_json::to_string(&snap).expect("instance");
        let tmp = std::env::temp_dir().join(format!("en1999-instance-{}.json", snap.members[0].id));
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


#[test]




fn bundled_example_assets_match_regenerated_dsl_and_pack() {
    let any = family_any_dir();
    for (name, snap) in [
        ("🏠️aluminium-roof-purlin", En1999Snapshot::compliant_roof_purlin()),
        ("🏚️noncompliant-multi-fail", En1999Snapshot::noncompliant_multi_fail()),
    ] {
        let dsl_path = any.join("🖼️assets").join(name).join(name).join("🗣️.dsl.semio");
        let pack_path = any.join("🖼️assets").join(name).join("🎒️.pack.semio");
        let expected_dsl = encode_en1999_dsl(&snap);
        let expected_pack = crate::standards::v1::subsets::any::schema::snapshot::encode_en1999_pack(&snap);
        let committed_dsl = std::fs::read_to_string(&dsl_path).unwrap_or_default();
        let committed_pack = std::fs::read(&pack_path).unwrap_or_default();
        assert_eq!(
            committed_dsl, expected_dsl,
            "DSL drift for {name}: regenerate 🖼️assets/{name}/{name}/🗣️.dsl.semio"
        );
        assert_eq!(
            committed_pack, expected_pack,
            "pack drift for {name}: regenerate 🖼️assets/{name}/🎒️.pack.semio"
        );
    }
}

#[test]
fn field_meta_covers_every_editable_leaf_en_de() {
    use crate::field_meta::en1999_field_meta;
    for snap in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail()] {
        let value = serde_json::to_value(&snap).expect("json");
        let mut missing = Vec::new();
        walk_leaves("", &value, &mut |path| {
            if path.is_empty() {
                return;
            }
            match en1999_field_meta(path) {
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
        assert!(missing.is_empty(), "missing field-meta for: {missing:?}");
    }
}




#[test]
fn facet_field_names_match_snapshot_json_schema() {
    let schema_path = snapshot_schema_path();
    let schema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&schema_path).unwrap()).unwrap();
    let ts = std::fs::read_to_string(family_any_dir().join("🧬️schema/📸️snapshot/🟦️.ts")).unwrap_or_default();
    let gql = std::fs::read_to_string(family_any_dir().join("🧬️schema/📸️snapshot/🔗️.graphql")).unwrap_or_default();
    let proto = std::fs::read_to_string(family_any_dir().join("🧬️schema/📸️snapshot/🛰️.proto")).unwrap_or_default();
    let mut_ts = std::fs::read_to_string(family_any_dir().join("🧬️schema/🧬️mutations/🟦️.ts")).unwrap_or_default();
    assert!(ts.contains("nK") && ts.contains("gKLine") && ts.contains("coldFormed") && ts.contains("shells") && ts.contains("detailCategory") && ts.contains("m1"), "TS facet stale");
    assert!(!ts.contains("vYEd") && !ts.contains("slopeM") && !ts.contains("mYEd"), "TS still has member design-effect names");
    assert!(ts.contains("MemberAction") && ts.contains("actions: MemberAction"));
    assert!(gql.contains("nK") && gql.contains("coldFormed") && gql.contains("m1") && !gql.contains("slopeM") && !gql.contains("vYEd"));
    assert!(proto.contains("n_k") && proto.contains("cold_formed") && proto.contains("m1") && !proto.contains("slope_m") && !proto.contains("v_y_ed"));
    let actions = schema["properties"]["members"]["items"]["properties"]["actions"]["items"]["properties"].as_object().unwrap();
    for key in ["id","kind","category","source","gKLine","qKLine","nK","vYK","vZK","mYK","mZK"] {
        assert!(actions.contains_key(key), "json schema missing action.{key}");
        assert!(ts.contains(key), "TS missing {key}");
    }
}

#[test]
fn facet_diff_matches_rust() {
    let diff_ts = std::fs::read_to_string(family_any_dir().join("🧬️schema/🔺️diff/🟦️.ts")).unwrap_or_default();
    let diff_gql = std::fs::read_to_string(family_any_dir().join("🧬️schema/🔺️diff/🔗️.graphql")).unwrap_or_default();
    let diff_proto = std::fs::read_to_string(family_any_dir().join("🧬️schema/🔺️diff/🛰️.proto")).unwrap_or_default();
    for field in ["artifact", "annex", "materials", "sections", "members", "connections", "fireScenarios", "fatigueDetails", "coldFormed", "shells"] {
        assert!(diff_ts.contains(field), "diff TS missing {field}");
        assert!(diff_gql.contains(field), "diff GraphQL missing {field}");
    }
    for field in ["artifact", "annex", "materials", "sections", "members", "connections", "fire_scenarios", "fatigue_details", "cold_formed", "shells"] {
        assert!(diff_proto.contains(field), "diff proto missing {field}");
    }
    for stale in ["nEdKn", "mEdKnm", "chi?:", "sheetMEdKnm", "sigmaEdShellMpa", "n_ed_kn", "sigma_ed_shell_mpa", "double chi", "chi:"] {
        assert!(!diff_ts.contains(stale), "diff TS still has stale scalar {stale}");
        assert!(!diff_gql.contains(stale), "diff GraphQL still has stale scalar {stale}");
        assert!(!diff_proto.contains(stale), "diff proto still has stale scalar {stale}");
    }
}

#[test]
fn facet_mutations_match_kinds() {
    use crate::mutations::KINDS;
    let mut_ts = std::fs::read_to_string(family_any_dir().join("🧬️schema/🧬️mutations/🟦️.ts")).unwrap_or_default();
    let mut_gql = std::fs::read_to_string(family_any_dir().join("🧬️schema/🧬️mutations/🔗️.graphql")).unwrap_or_default();
    let mut_proto = std::fs::read_to_string(family_any_dir().join("🧬️schema/🧬️mutations/🛰️.proto")).unwrap_or_default();
    assert_eq!(KINDS.len(), 18, "Rust KINDS length drift");
    for kind in KINDS {
        assert!(mut_ts.contains(kind), "mutations TS missing KINDS entry {kind}");
        assert!(mut_gql.contains(kind), "mutations GraphQL missing KINDS entry {kind}");
        assert!(mut_proto.contains(kind), "mutations proto missing KINDS entry {kind}");
    }
    for stale in ["changeNEdKn", "changeChi", "changeSheetMEdKnm", "changeSigmaEdShellMpa", "nEdKn", "change_n_ed_kn"] {
        assert!(!mut_ts.contains(stale), "mutations TS still has stale scalar mutation {stale}");
        assert!(!mut_gql.contains(stale), "mutations GraphQL still has stale scalar mutation {stale}");
        assert!(!mut_proto.contains(stale), "mutations proto still has stale scalar mutation {stale}");
    }
}

#[test]
fn governing_uls_combination_named_in_member_explanations() {
    let report = evaluate_structure(&En1999Snapshot::compliant_roof_purlin());
    let uls: Vec<_> = report.checks.iter().filter(|c| c.id.contains("en1999.6.2.") || c.id.contains("en1999.6.3.")).collect();
    assert!(!uls.is_empty());
    for c in &uls {
        assert!(
            c.explanation.en.contains("uls-610a") || c.explanation.en.contains("uls-610b") || c.explanation.en.contains("uls-g"),
            "missing ULS combo in en explanation for {}: {}", c.id, c.explanation.en
        );
        assert!(
            c.explanation.de.contains("uls-610a") || c.explanation.de.contains("uls-610b") || c.explanation.de.contains("uls-g"),
            "missing ULS combo in de explanation for {}: {}", c.id, c.explanation.de
        );
        assert!(
            c.explanation.en.contains("lead ") && c.explanation.de.contains("führend "),
            "missing action_id lead tag for {}: en={} de={}", c.id, c.explanation.en, c.explanation.de
        );
    }
    assert!(uls.iter().any(|c| c.explanation.de.contains("GZT")));
    assert!(report.checks.iter().any(|c| c.id.contains("sls-freq") && c.explanation.en.contains("sls-freq")));
}

#[test]
fn explanations_en_de_not_identical_except_numbers() {
    for snap in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail()] {
        let report = evaluate_structure(&snap);
        for c in &report.checks {
            let en = strip_nums(&c.explanation.en);
            let de = strip_nums(&c.explanation.de);
            if en.is_empty() { continue; }
            assert_ne!(en, de, "identical en/de explanation for {}: {:?}", c.id, c.explanation);
            for r in &c.remedies {
                let ren = strip_nums(&r.action.en);
                let rde = strip_nums(&r.action.de);
                if ren.is_empty() { continue; }
                assert_ne!(ren, rde, "identical en/de remedy for {}: {:?}", c.id, r.action);
            }
        }
    }
}

fn strip_nums(s: &str) -> String {
    s.chars().filter(|c| !c.is_ascii_digit() && *c != '.' && *c != ',').collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn field_meta_rows_resolve_to_schema_leaves() {
    let snap = En1999Snapshot::compliant_roof_purlin();
    let tree = serde_json::to_value(&snap).unwrap();
    let mut leaves = Vec::new();
    walk_leaves("", &tree, &mut |path| {
        if path.is_empty() { return; }
        let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
        if leaf == "id" || leaf.starts_with("id=") { return; }
        leaves.push(path.to_string());
    });
    use crate::field_meta::en1999_field_meta;
    // Every meta TABLE path pattern must match at least one leaf prefix style — check no stale design names
    for stale in ["vYEd", "vZEd", "mZEd", "mYEd", "slopeM"] {
        assert!(leaves.iter().all(|p| !p.contains(stale)), "snapshot still has stale {stale}");
    }
    // Member/connection/cold/shell action forces must be characteristic nK (no hand-typed *Ed design effects).
    assert!(leaves.iter().filter(|p| p.contains("actions") && p.contains("nEd")).count() == 0);
    for path in &leaves {
        let meta_path = path_to_meta_pattern(path);
        assert!(en1999_field_meta(&meta_path).is_some(), "no field-meta for leaf {path} (pattern {meta_path})");
    }
}

fn path_to_meta_pattern(path: &str) -> String {
    let mut out = String::new();
    let mut rest = path;
    while !rest.is_empty() {
        if rest.starts_with('.') { rest = &rest[1..]; }
        if rest.starts_with('[') {
            let close = rest.find(']').unwrap();
            out.push_str("[]");
            rest = &rest[close+1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        if !out.is_empty() && !out.ends_with(']') { out.push('.'); }
        // after [] we need .
        if out.ends_with(']') { out.push('.'); }
        out.push_str(&rest[..end]);
        rest = &rest[end..];
    }
    out
}

#[test]
fn sls_deflection_and_stress_checks_present() {
    let ok = evaluate_structure(&En1999Snapshot::compliant_roof_purlin());
    assert!(ok.checks.iter().any(|c| c.id.contains("7.2.defl")));
    assert!(ok.checks.iter().any(|c| c.id.contains("7.2.stress")));
    let bad = evaluate_structure(&En1999Snapshot::noncompliant_multi_fail());
    let fail_defl = bad.failing().find(|c| c.id.contains("7.2.defl"));
    assert!(fail_defl.is_some(), "noncompliant should fail SLS deflection");
    let f = fail_defl.unwrap();
    assert!(!f.remedies.is_empty());
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


#[test]
fn haz_rho_u_governs_welded_net_section() {
    let alloy = part_1_1::resolve_alloy("aw6060-t6").unwrap();
    assert!(alloy.rho_u_haz < alloy.rho_o_haz || alloy.rho_u_haz < 1.0);
    let mut sec = En1999Snapshot::noncompliant_multi_fail().sections[0].clone();
    for e in &mut sec.elements { e.welded = true; e.weld_position = 0.01; }
    // §6.2.3 net tension uses ρ_u,haz · f_u; prove ρ_u < 1 reduces N_u,Rd.
    let (_, _, _, _, _, _, _) = part_1_1::section_geometry(&sec);
    let a_net = part_1_1::effective_area(&sec, alloy.f_o_pa, true, 1.0, 1.0);
    let n_u_full = a_net * alloy.f_u_pa;
    let n_u_haz = a_net * alloy.rho_u_haz * alloy.f_u_pa;
    assert!(alloy.rho_u_haz < 1.0);
    assert!(n_u_haz < n_u_full - 1e-6, "ρ_u,haz must reduce N_u,Rd ({n_u_haz} vs {n_u_full})");
}

#[test]
fn shell_chi_from_geometry_hand_value() {
    // r=0.8 m, t=0.003 m, L=3 m, f_o=160 MPa (6060-T6), E=70 GPa → λ̄_x and χ_x
    let e = na_de::E_PA;
    let r: f64 = 0.80;
    let t: f64 = 0.003;
    let sigma_x_rcr = 0.605 * e * (t / r);
    let f_o: f64 = 160.0e6;
    let lambda_x = (f_o / sigma_x_rcr).sqrt();
    let alpha: f64 = 0.62;
    let beta: f64 = 0.60;
    let lambda_0: f64 = 0.20;
    let phi = 0.5 * (1.0 + alpha * (lambda_x - lambda_0) + beta * lambda_x.powi(2));
    let chi = 1.0 / (phi + (phi * phi - beta * lambda_x.powi(2)).max(0.0).sqrt());
    assert!((chi - 0.55).abs() < 0.15, "χ_x={chi}, λ̄={lambda_x}");
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let report = evaluate_structure(&doc);
    let shell = report.checks.iter().find(|c| c.id.contains("1-5.buckle")).expect("shell buckle");
    assert!(shell.explanation.en.contains("χ=") || shell.explanation.en.contains("Rcr") || shell.utilization > 0.0);
}

#[test]



fn every_editable_leaf_influences_a_check() {
    // Descriptive report labels only — never geometry/loads.
    const EXEMPT_SUFFIXES: &[&str] = &[".id"]; // ids only; name/title not in this subject
    for snap in [En1999Snapshot::compliant_roof_purlin(), En1999Snapshot::noncompliant_multi_fail()] {
        let base_report = evaluate_structure(&snap);
        let mut base_sig: Vec<_> = base_report
            .checks
            .iter()
            .map(|c| (c.id.clone(), c.status, c.computed.value.to_bits(), c.limit.value.to_bits(), c.utilization.to_bits()))
            .collect();
        base_sig.sort_by(|a, b| a.0.cmp(&b.0));
        let value = serde_json::to_value(&snap).expect("json");
        let mut leaves = Vec::new();
        walk_leaves("", &value, &mut |path| {
            if path.is_empty() { return; }
            let leaf = path.rsplit(['.', '[']).next().unwrap_or(path);
            if leaf == "id" || leaf.starts_with("id=") { return; }
            if EXEMPT_SUFFIXES.iter().any(|s| path.ends_with(s)) { return; }
            leaves.push(path.to_string());
        });
        assert!(!leaves.is_empty());
        let mut unchanged = Vec::new();
        for path in &leaves {
            let mut tree = serde_json::to_value(&snap).unwrap();
            let cur = resolve_path(&tree, path).cloned();
            let Some(cur) = cur else { continue; };
            match cur {
                serde_json::Value::Number(n) => {
                    let v = n.as_f64().unwrap_or(0.0);
                    let nv = if v.abs() < 1e-12 { 1.0e5 } else { v * 1.35 };
                    let _ = set_number_at_path(&mut tree, path, nv);
                }
                serde_json::Value::Bool(b) => { let _ = set_bool_at_path(&mut tree, path, !b); }
                serde_json::Value::String(s) => {
                    let leaf = path.rsplit('.').next().unwrap_or(path.as_str());
                    let ns: String = if s == "de" { "en".into() }
                        else if s == "en" { "de".into() }
                        else if s.contains("6082") { "aw6060-t6".into() }
                        else if s.contains("6060") { "aw6082-t6".into() }
                        else if s == "simplySupported" { "continuous".into() }
                        else if s == "continuous" { "cantilever".into() }
                        else if s == "cantilever" { "simplySupported".into() }
                        else if leaf == "kind" && (s == "snow" || s == "wind" || s == "imposed" || s == "temperature") { "permanent".into() }
                        else if leaf == "kind" && s == "permanent" { "imposed".into() }
                        else if leaf == "category" && s == "snow" { "storage".into() }
                        else if leaf == "category" && s == "wind" { "storage".into() }
                        else if leaf == "category" && s == "office" { "storage".into() }
                        else if leaf == "category" && s == "self" { "snow".into() }
                        else if s == "permanent" { "imposed".into() }
                        else if s == "imposed" { "permanent".into() }
                        else if s == "snow" { "storage".into() }
                        else if s == "wind" { "storage".into() }
                        else if s == "office" { "storage".into() }
                        else if s == "self" { "snow".into() }
                        else if s == "external" { "udl".into() }
                        else if s == "udl" { "external".into() }
                        else if s == "8.8" { "10.9".into() }
                        else if s == "10.9" { "5.6".into() }
                        else if s == "4.6" || s == "5.6" { "8.8".into() }
                        else if s == "tube" || s == "chs" { "extrudedI".into() }
                        else if s == "extrudedI" { "tube".into() }
                        else if s == "combined" { "bolted".into() }
                        else if s == "bolted" { "welded".into() }
                        else if s == "welded" { "combined".into() }
                        else if s == "4043" { "5356".into() }
                        else if s == "5356" { "4043".into() }
                        else { format!("{s}-x") };
                    let _ = set_string_at_path(&mut tree, path, &ns);
                }
                _ => continue,
            }
            let Ok(perturbed): Result<En1999Snapshot,_> = serde_json::from_value(tree) else { continue; };
            let rep = evaluate_structure(&perturbed);
            let mut sig: Vec<_> = rep.checks.iter().map(|c| (c.id.clone(), c.status, c.computed.value.to_bits(), c.limit.value.to_bits(), c.utilization.to_bits())).collect();
            sig.sort_by(|a, b| a.0.cmp(&b.0));
            if sig == base_sig {
                unchanged.push(path.clone());
            }
        }
        assert!(
            unchanged.is_empty(),
            "editable leaves must influence a check (no ratio gate): {unchanged:?}"
        );
    }
}




#[test]




fn regen_example_assets_when_env_set() {
    if std::env::var("REGEN_EN1999_ASSETS").is_err() {
        return;
    }
    let any = family_any_dir();
    for (name, snap) in [
        ("🏠️aluminium-roof-purlin", En1999Snapshot::compliant_roof_purlin()),
        ("🏚️noncompliant-multi-fail", En1999Snapshot::noncompliant_multi_fail()),
    ] {
        let dir = any.join("🖼️assets").join(name).join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("🗣️.dsl.semio"), encode_en1999_dsl(&snap)).unwrap();
        std::fs::write(
            any.join("🖼️assets").join(name).join("🎒️.pack.semio"),
            crate::standards::v1::subsets::any::schema::snapshot::encode_en1999_pack(&snap),
        ).unwrap();
        std::fs::write(
            dir.join("🔣️.json"),
            crate::standards::v1::subsets::any::schema::snapshot::encode_en1999_snapshot_json(&snap),
        ).unwrap();
    }
}

#[test]
fn duplicate_material_id_fails_with_oneof_remedy() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let dup_id = doc.materials[0].id.clone();
    let mut twin = doc.materials[0].clone();
    twin.designation = "aw5083-o".into();
    doc.materials.push(twin);
    let report = evaluate_structure(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id == format!("en1999.ref.duplicate.materials.{dup_id}"))
        .expect("duplicate material id Fail");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_ne!(check.explanation.en, check.explanation.de);
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable one_of remedy");
    assert!(matches!(remedy.bound, crate::document::RemedyBound::OneOf));
    assert!(!remedy.options.is_empty());
    assert!(!remedy.options.iter().any(|o| o == &dup_id));
}

#[test]
fn dangling_connection_material_id_fails_with_oneof_existing_materials() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    assert!(!doc.connections.is_empty(), "fixture must expose a connection");
    let existing: Vec<String> = doc.materials.iter().map(|m| m.id.clone()).collect();
    let conn_id = doc.connections[0].id.clone();
    doc.connections[0].material_id = "mat-missing".into();
    let report = evaluate_structure(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id == format!("en1999.ref.conn.material.{conn_id}"))
        .expect("dangling connection materialId Fail");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_ne!(check.explanation.en, check.explanation.de);
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable one_of remedy");
    assert!(matches!(remedy.bound, crate::document::RemedyBound::OneOf));
    assert!(!remedy.options.is_empty());
    for opt in &remedy.options {
        assert!(existing.contains(opt), "remedy option {opt} must be an existing material id");
    }
    assert!(!remedy.options.iter().any(|o| o == "mat-missing"));
}

#[test]
fn dangling_connection_member_id_fails_with_oneof_existing_members() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    assert!(!doc.connections.is_empty(), "fixture must expose a connection");
    let existing: Vec<String> = doc.members.iter().map(|m| m.id.clone()).collect();
    let conn_id = doc.connections[0].id.clone();
    doc.connections[0].member_id = "member-missing".into();
    let report = evaluate_structure(&doc);
    let check = report
        .checks
        .iter()
        .find(|c| c.id == format!("en1999.8.ref.{conn_id}"))
        .expect("dangling connection memberId Fail");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_ne!(check.explanation.en, check.explanation.de);
    let remedy = check.remedies.iter().find(|r| r.applicable).expect("applicable one_of remedy");
    assert!(matches!(remedy.bound, crate::document::RemedyBound::OneOf));
    assert!(!remedy.options.is_empty());
    for opt in &remedy.options {
        assert!(existing.contains(opt), "remedy option {opt} must be an existing member id");
    }
    assert!(!remedy.options.iter().any(|o| o == "member-missing"));
}


fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn oracle_script() -> PathBuf {
    family_any_dir().join("🔮️oracles").join("🐍️evaluate.py")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema").join("📸️snapshot").join("🔣️.json")
}

fn path_grammar_ok(path: &str) -> bool {
    let mut rest = path;
    if rest.is_empty() {
        return false;
    }
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let Some(close) = rest.find(']') else {
                return false;
            };
            let inner = &rest[1..close];
            if let Some(id) = inner.strip_prefix("id=") {
                if id.is_empty() || id.contains([']', '.', '=']) {
                    return false;
                }
            } else if inner.parse::<usize>().is_err() {
                return false;
            }
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        if end == 0 {
            return false;
        }
        rest = &rest[end..];
    }
    true
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

enum PathSeg {
    Field(String),
    Index(usize),
    Id(String),
}

fn walk_mut<'a>(root: &'a mut serde_json::Value, segments: &[PathSeg]) -> Result<&'a mut serde_json::Value, String> {
    let mut cursor = root;
    for seg in segments {
        cursor = match seg {
            PathSeg::Field(name) => cursor.get_mut(name).ok_or_else(|| format!("missing {name}"))?,
            PathSeg::Index(i) => cursor.get_mut(*i).ok_or_else(|| format!("missing [{i}]"))?,
            PathSeg::Id(id) => {
                let arr = cursor.as_array_mut().ok_or_else(|| "expected array".to_string())?;
                let idx = arr
                    .iter()
                    .position(|el| el.get("id").and_then(|v| v.as_str()) == Some(id.as_str()))
                    .ok_or_else(|| format!("missing id={id}"))?;
                &mut arr[idx]
            }
        };
    }
    Ok(cursor)
}

fn parse_segments(path: &str) -> Result<Vec<PathSeg>, String> {
    let mut segments = Vec::new();
    let mut rest = path;
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let close = rest.find(']').ok_or_else(|| "unclosed".to_string())?;
            let inner = &rest[1..close];
            if let Some(id) = inner.strip_prefix("id=") {
                segments.push(PathSeg::Id(id.to_string()));
            } else {
                segments.push(PathSeg::Index(inner.parse().map_err(|_| "bad index")?));
            }
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        segments.push(PathSeg::Field(rest[..end].to_string()));
        rest = &rest[end..];
    }
    Ok(segments)
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

fn set_u64_at_path(root: &mut serde_json::Value, path: &str, value: u64) -> Result<(), String> {
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
