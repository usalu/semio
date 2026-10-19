//! ✅️ Compliance gates of the EN 1999 compliance tests: the compliant and non-compliant example reports, the remedy laws, the bundled DSL and the Python oracle. See [`super`].

use super::*;

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
    let dsl = include_str!("../../../../🖼️assets/🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio");
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
