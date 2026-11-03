use semio_s_artifact_norm_en1990_checks::*;
use semio_s_artifact_norm_en1990_inferences::*;
use semio_s_artifact_norm_en1990_model::*;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport};
use semio_framework_test_fixtures::fixture_path;
use semio_s_plugin_norm_compliance::CheckStatus;

#[semio_framework_async_macros::async_test]
async fn evaluate_default_produces_checks() {
    let doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = evaluate(&doc);
    assert!(!report.checks.is_empty(), "default document must produce checks");
    assert!(report.checks.iter().any(|c| c.id.contains("6.10") || c.clause.section.contains("6.10")));
}

#[semio_framework_async_macros::async_test]
async fn default_subject_complies() {
    let report = evaluate(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| c.id.clone()).collect();
    assert!(fails.is_empty(), "default must comply, fails={fails:?}");
    assert!(report.summary.fail == 0);
}

#[semio_framework_async_macros::async_test]
async fn high_consequence_office_fails_with_multiple_checks() {
    let doc = semio_s_artifact_norm_en1990_examples::high_consequence_office::reference_snapshot();
    let report = evaluate(&doc);
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).collect();
    assert!(fails.len() >= 2, "expected ≥2 fails, got {} {:?}", fails.len(), fails.iter().map(|c| &c.id).collect::<Vec<_>>());
    assert!(!report.complies());
    assert!(fails.iter().all(|c| !c.remedies.is_empty()), "every Fail needs a remedy");
}

#[semio_framework_async_macros::async_test]
async fn high_consequence_office_dsl_asset_decodes_and_fails() {
    let text = semio_s_artifact_norm_en1990_examples::high_consequence_office::PRIMARY_TEXT;
    let doc = semio_s_artifact_norm_en1990_io::text::snapshot::decode_en1990_dsl(text).expect("dsl");
    assert_eq!(doc.consequence_class, 3);
    let report = evaluate(&doc);
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
}

#[semio_framework_async_macros::async_test]
async fn default_office_dsl_asset_decodes_and_complies() {
    let text = include_str!("../../../../🖼️assets/🏢️default-office/🏢️default-office/🗣️.dsl.semio");
    let doc = semio_s_artifact_norm_en1990_io::text::snapshot::decode_en1990_dsl(text).expect("dsl");
    assert!(evaluate(&doc).summary.fail == 0);
}

#[semio_framework_async_macros::async_test]
async fn evaluate_marks_seismic_not_applicable_when_no_a_ek() {
    let doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    assert!(doc.seismics.is_empty());
    let report = evaluate(&doc);
    let seismic = report.checks.iter().find(|c| c.clause.section == "6.12b" || c.id.contains("6.12b")).expect("6.12b check present on default");
    assert_eq!(seismic.status, CheckStatus::NotApplicable, "empty seismics must yield NotApplicable, got {:?}", seismic.status);
}

#[semio_framework_async_macros::async_test]
async fn dangling_effect_action_id_fails_referential_integrity() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.effects[0].action_id = format!("{}-x", doc.effects[0].action_id);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.effects") && c.id.contains("actionId") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn dangling_bridge_sls_member_id_fails_referential_integrity() {
    let mut doc = semio_s_artifact_norm_en1990_examples::road_bridge_compliant::reference_snapshot();
    doc.bridge_sls[0].member_id = format!("{}-x", doc.bridge_sls[0].member_id);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.bridgeSls") && c.id.contains("memberId") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn duplicate_member_id_fails_integrity() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let twin = doc.members[0].clone();
    doc.members.push(twin);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.duplicate.members") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn every_emitted_subject_path_parses_and_resolves() {
    let doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = evaluate(&doc);
    let root = semio_framework_value::ToValue::to_value(&doc);
    let mut seen = 0usize;
    for check in &report.checks {
        for path in std::iter::once(check.subject.path.as_str()).chain(check.remedies.iter().map(|r| r.target.path.as_str())) {
            if path.is_empty() {
                continue;
            }
            semio_s_plugin_norm_app_surface::parse_path(path).unwrap_or_else(|e| panic!("parse '{path}': {e}"));
            semio_s_plugin_norm_app_surface::get_value_at_path(&root, path).unwrap_or_else(|e| panic!("resolve '{path}': {e}"));
            seen += 1;
            assert!(!path.contains("members[") || path.contains("members[id="), "entity list path must use [id=…]: {path}");
        }
    }
    assert!(seen > 0, "expected at least one subject/remedy path");
}

#[semio_framework_async_macros::async_test]
async fn editing_gk_changes_str_utilization() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let before = evaluate(&doc);
    let u0 = before.checks.iter().find(|c| c.id.starts_with("en1990.str.") && c.id.ends_with(".6.10")).map(|c| c.utilization).unwrap();
    doc.permanents.iter_mut().find(|p| p.kind == semio_s_artifact_norm_en1990_model::PermanentKind::GSup).unwrap().gk *= 2.0;
    let after = evaluate(&doc);
    let u1 = after.checks.iter().find(|c| c.id.starts_with("en1990.str.") && c.id.ends_with(".6.10")).map(|c| c.utilization).unwrap();
    assert!(u1 > u0 + 0.05, "doubling gk must raise STR utilization ({u0} → {u1})");
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_beta_writing_required_makes_check_pass() {
    let mut doc = semio_s_artifact_norm_en1990_examples::high_consequence_office::reference_snapshot();
    let report = evaluate(&doc);
    let fail = report.checks.iter().find(|c| c.id.contains("beta") && c.status == CheckStatus::Fail && c.remedies.iter().any(|r| r.applicable)).expect("β fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).unwrap();
    apply_path(&mut doc, &remedy.target.path, remedy.required.value);
    let after = evaluate(&doc);
    let same = after.checks.iter().find(|c| c.id == fail.id).expect("check");
    assert!(same.status != CheckStatus::Fail || semio_s_plugin_norm_compliance::within_limit(same.utilization), "β remedy failed: {:?} util={}", same.status, same.utilization);
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_deflection_writing_required_makes_check_pass() {
    let mut doc = semio_s_artifact_norm_en1990_examples::high_consequence_office::reference_snapshot();
    let report = evaluate(&doc);
    let fail = report
        .checks
        .iter()
        .find(|c| c.id.contains("deflection") && c.status == CheckStatus::Fail && c.remedies.iter().any(|r| r.applicable))
        .expect("deflection fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).unwrap();
    apply_path(&mut doc, &remedy.target.path, remedy.required.value);
    let after = evaluate(&doc);
    let same = after.checks.iter().find(|c| c.id == fail.id).expect("check");
    assert!(same.status != CheckStatus::Fail || semio_s_plugin_norm_compliance::within_limit(same.utilization), "deflection remedy failed: {:?} util={}", same.status, same.utilization);
}

#[semio_framework_async_macros::async_test]
async fn sls_clause_ids_are_614b_615b_616b() {
    let report = evaluate(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.14b"));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.15b"));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.16b"));
    assert!(!report.checks.iter().any(|c| c.clause.section == "6.17"));
}

#[semio_framework_async_macros::async_test]
async fn geo_and_equ_stab_checks_emitted() {
    let report = evaluate(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(report.checks.iter().any(|c| c.id.contains(".geo.")));
    assert!(report.checks.iter().any(|c| c.id.contains("equ.stab")));
}

#[test]
fn catalogue_matches_the_independent_python_oracle() {
    let oracle = fixture_path!("../../../../🔮️oracles/⚖️compliance/🐍️.py");
    let fixture = fixture_path!("../../../../🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json");
    let cases: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(&fixture).expect("fixture")).expect("fixture json");
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&oracle, &[std::ffi::OsStr::new("catalogue"), fixture.as_os_str()], None);
    let expected: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
    let mut compared = 0usize;
    for case in &cases {
        let id = case["id"].as_str().expect("id");
        let entry = case["entry"].as_str().expect("entry");
        let annex = if case["annex"] == "De" { semio_s_plugin_norm_compliance::AnnexChoice::De } else { semio_s_plugin_norm_compliance::AnnexChoice::En };
        let mut input = catalogue::Case::new(id, annex);
        for (key, value) in case["num"].as_object().expect("num") {
            input = input.with_n(key, value.as_f64().expect("number"));
        }
        for (key, value) in case["text"].as_object().expect("text") {
            input = input.with_s(key, value.as_str().expect("text"));
        }
        let outcomes = catalogue::evaluate_outcomes(entry, &input).unwrap_or_else(|reason| panic!("{entry} {id}: {reason}"));
        let wanted = expected[id].as_object().unwrap_or_else(|| panic!("oracle has no case {id}"));
        let tolerance = if id.starts_with("form-") { 2e-4 } else { 1e-7 };
        for outcome in &outcomes {
            let pair = wanted.get(&outcome.key).unwrap_or_else(|| panic!("{entry} {id}: oracle has no outcome {}", outcome.key));
            let (demand, capacity) = (pair[0].as_f64().expect("demand"), pair[1].as_f64().expect("capacity"));
            for (rust, python, side) in [(outcome.demand, demand, "demand"), (outcome.capacity, capacity, "capacity")] {
                assert!((rust - python).abs() <= tolerance * python.abs().max(1.0), "{entry} {id} {} {side}: rust {rust} vs python {python}", outcome.key);
            }
            compared += 1;
        }
        for key in wanted.keys().filter(|key| !key.starts_with('_')) {
            assert!(outcomes.iter().any(|outcome| &outcome.key == key), "{entry} {id}: rust lacks outcome {key}");
        }
        if let Some(exact) = wanted.get("_exact-beta") {
            let beta = outcomes.iter().find(|outcome| outcome.key == "beta").expect("beta").capacity;
            assert!((beta / exact[1].as_f64().expect("exact") - 1.0).abs() < 0.01, "{id}: FORM beta {beta} vs exact integration {exact}");
        }
    }
    assert!(compared >= 200, "compared only {compared} outcomes");
}

#[test]
fn snapshot_checks_match_the_brute_force_python_oracle() {
    let oracle = fixture_path!("../../../../🔮️oracles/⚖️compliance/🐍️.py");
    let mut documents = serde_json::Map::new();
    let examples = [
        ("default", En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)),
        ("high", semio_s_artifact_norm_en1990_examples::high_consequence_office::reference_snapshot()),
        ("road", semio_s_artifact_norm_en1990_examples::road_bridge_compliant::reference_snapshot()),
        ("road-failing", semio_s_artifact_norm_en1990_examples::road_bridge_failing::reference_snapshot()),
        ("accidental", semio_s_artifact_norm_en1990_examples::accidental_seismic_compliant::reference_snapshot()),
        ("accidental-failing", semio_s_artifact_norm_en1990_examples::accidental_seismic_failing::reference_snapshot()),
        ("fatigue", semio_s_artifact_norm_en1990_examples::fatigue_compliant::reference_snapshot()),
        ("fatigue-failing", semio_s_artifact_norm_en1990_examples::fatigue_failing::reference_snapshot()),
        ("analysed", analysed()),
    ];
    for (name, snapshot) in examples {
        let mut english = snapshot.clone();
        english.annex = semio_s_plugin_norm_compliance::AnnexChoice::En;
        documents.insert(format!("{name}-de"), serde_json::to_value(&snapshot).expect("json"));
        documents.insert(format!("{name}-en"), serde_json::to_value(&english).expect("json"));
    }
    let payload = serde_json::Value::Object(documents.clone()).to_string();
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&oracle, &[std::ffi::OsStr::new("snapshot")], Some(payload.as_bytes()));
    let expected: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
    let mut compared = 0usize;
    for (name, value) in &documents {
        let snapshot: En1990Snapshot = serde_json::from_value(value.clone()).expect("snapshot");
        let report = evaluate(&snapshot);
        let wanted = expected[name].as_object().expect("oracle snapshot");
        for check in &report.checks {
            if let Some(number) = wanted.get(&format!("_form.{}", check.id)).and_then(|n| n.as_f64()) {
                assert!((check.computed.value - number).abs() <= 1e-3 * number.abs().max(1.0), "{name} {}: FORM rust {} vs scipy {number}", check.id, check.computed.value);
                compared += 1;
            }
            if let Some(number) = wanted.get(&check.id).and_then(|n| n.as_f64()) {
                assert!((check.computed.value - number).abs() <= 1e-9 * number.abs().max(1.0), "{name} {}: rust {} vs python {number}", check.id, check.computed.value);
                compared += 1;
            }
        }
        let beta = report.checks.iter().find(|c| c.id == "en1990.annex-c.beta").expect("beta check");
        let target = wanted["_limit.en1990.annex-c.beta"].as_f64().expect("target");
        assert!((beta.limit.value - target).abs() < 1e-6, "{name}: beta target {} vs {target}", beta.limit.value);
    }
    assert!(compared >= 300, "compared only {compared} checks");
}

fn apply_path(doc: &mut En1990Snapshot, path: &str, value: f64) {
    let mut root = semio_framework_value::ToValue::to_value(&*doc);
    semio_s_plugin_norm_app_surface::set_value_at_path(&mut root, path, semio_framework_value::DslValue::float(value)).unwrap_or_else(|e| panic!("set {path}: {e}"));
    *doc = semio_framework_value::FromValue::from_value(root).expect("from_value");
}

#[semio_framework_async_macros::async_test]
async fn table_2_1_bridge_category_remedy_flips_to_pass() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.structure_kind = semio_s_artifact_norm_en1990_model::StructureKind::Footbridge;
    doc.design_working_life_category = 4;
    doc.design_working_life_years = 50.0;
    let report = evaluate(&doc);
    let fail = report
        .checks
        .iter()
        .find(|c| c.id == "en1990.table-2.1.category" && c.status == CheckStatus::Fail)
        .expect("category fail for bridge");
    let remedy = fail.remedies.iter().find(|r| r.applicable).expect("remedy");
    assert_eq!(remedy.target.path, "designWorkingLifeCategory");
    let choice = remedy.options.first().expect("one_of option");
    assert_eq!(choice, "5");
    doc.design_working_life_category = choice.parse().unwrap();
    doc.design_working_life_years = 100.0;
    let after = evaluate(&doc);
    let same = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert_ne!(same.status, CheckStatus::Fail, "category remedy should pass");
}

#[semio_framework_async_macros::async_test]
async fn bridge_a2_partial_factors_come_from_the_ndp_table() {
    use semio_s_plugin_norm_compliance::AnnexChoice;
    for annex in [AnnexChoice::De, AnnexChoice::En] {
        assert!((gamma_q(annex, Domain::Bridge, FactorSet::StrB, "rail_traffic") - 1.45).abs() < 1e-12);
        assert!((gamma_q(annex, Domain::Bridge, FactorSet::StrB, "road_traffic") - 1.35).abs() < 1e-12);
        assert!((gamma_q(annex, Domain::Bridge, FactorSet::EquA, "wind_bridge") - 1.5).abs() < 1e-12);
    }
}

#[semio_framework_async_macros::async_test]
async fn evaluate_reads_project_id_in_subject_labels() {
    let doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = evaluate(&doc);
    assert!(
        report.checks.iter().any(|c| c.subject.label.en.contains(&doc.project_id) || c.subject.entity_id == doc.project_id),
        "projectId must appear as report/entity label"
    );
}

#[semio_framework_async_macros::async_test]
async fn road_bridge_compliant_subject_complies() {
    let doc = semio_s_artifact_norm_en1990_examples::road_bridge_compliant::reference_snapshot();
    let report = evaluate(&doc);
    assert!(report.summary.fail == 0, "bridge compliant must pass, fails={:?}", report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn road_bridge_failing_fails_with_multiple_checks() {
    let doc = semio_s_artifact_norm_en1990_examples::road_bridge_failing::reference_snapshot();
    let report = evaluate(&doc);
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn accidental_seismic_compliant_activates_6_11b_and_6_12b() {
    let doc = semio_s_artifact_norm_en1990_examples::accidental_seismic_compliant::reference_snapshot();
    let report = evaluate(&doc);
    let c611 = report.checks.iter().find(|c| c.clause.section == "6.11b").expect("6.11");
    let c612 = report.checks.iter().find(|c| c.clause.section == "6.12b").expect("6.12b");
    assert_ne!(c611.status, CheckStatus::NotApplicable);
    assert_ne!(c612.status, CheckStatus::NotApplicable);
    assert!(report.summary.fail == 0, "accidental/seismic compliant should pass");
}

#[semio_framework_async_macros::async_test]
async fn accidental_seismic_failing_fails_6_11_or_6_12b() {
    let doc = semio_s_artifact_norm_en1990_examples::accidental_seismic_failing::reference_snapshot();
    let report = evaluate(&doc);
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
    assert!(report.checks.iter().any(|c| c.clause.section == "6.11b" && c.status == CheckStatus::Fail));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.12b" && c.status == CheckStatus::Fail));
}

#[semio_framework_async_macros::async_test]
async fn fatigue_compliant_subject_complies() {
    let doc = semio_s_artifact_norm_en1990_examples::fatigue_compliant::reference_snapshot();
    let report = evaluate(&doc);
    let fat = report.checks.iter().find(|c| c.id.contains("fat") || c.clause.section.to_lowercase().contains("fat")).expect("FAT");
    assert_ne!(fat.status, CheckStatus::NotApplicable);
    assert!(report.summary.fail == 0);
}

#[semio_framework_async_macros::async_test]
async fn fatigue_failing_fails_fat_check() {
    let doc = semio_s_artifact_norm_en1990_examples::fatigue_failing::reference_snapshot();
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| (c.id.contains("fat") || c.clause.section.to_lowercase().contains("fat")) && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn every_editable_leaf_changes_a_check_when_perturbed_in_applicable_scope() {











    let building = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let mut building_snow = semio_s_artifact_norm_en1990_examples::high_consequence_office::reference_snapshot();
    building_snow.annex = semio_s_plugin_norm_compliance::AnnexChoice::De;
    building_snow.altitude_m = 0.0;
    let bridge = semio_s_artifact_norm_en1990_examples::road_bridge_compliant::reference_snapshot();

    let mut unchanged = Vec::new();
    let mut checked = 0usize;

    {
        let base = &building;
        let base_sig = sig(&evaluate(base));
        let tree0 = serde_json::to_value(base).expect("json");
        let mut leaves = Vec::new();
        walk("", &tree0, &mut leaves);
        for path in leaves {
            if is_exempt(&path) {
                continue;
            }
            if path.starts_with("bridgeSls") || path == "altitudeM" {
                continue;
            }
            let cur = match get_json(&tree0, &path) {
                Some(v) => v.clone(),
                None => continue,
            };
            let next = if path == "referencePeriodYears" {
                let v = cur.as_f64().unwrap_or(50.0);
                serde_json::json!(if v >= 25.0 { 10.0 } else { 50.0 })
            } else {
                match perturb_leaf(&path, &cur, &tree0) {
                    Some(v) => v,
                    None => continue,
                }
            };
            let mut tree = tree0.clone();
            if !set_json(&mut tree, &path, next) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1990Snapshot>(tree) else { continue };
            checked += 1;
            if path == "annex" {
                let mut annex_base = semio_s_artifact_norm_en1990_examples::accidental_seismic_compliant::reference_snapshot();
                annex_base.annex = semio_s_plugin_norm_compliance::AnnexChoice::De;
                let annex_sig = sig(&evaluate(&annex_base));
                annex_base.annex = semio_s_plugin_norm_compliance::AnnexChoice::En;
                if sig(&evaluate(&annex_base)) == annex_sig {
                    unchanged.push(format!("building:{path}"));
                }
            } else if sig(&evaluate(&perturbed)) == base_sig {
                unchanged.push(format!("building:{path}"));
            }
        }
    }

    {
        let base = &building_snow;
        let base_sig = sig(&evaluate(base));
        let mut tree = serde_json::to_value(base).expect("json");
        assert!(set_json(&mut tree, "altitudeM", serde_json::json!(1200.0)));
        let perturbed: En1990Snapshot = serde_json::from_value(tree).expect("from");
        checked += 1;
        if sig(&evaluate(&perturbed)) == base_sig {
            unchanged.push("building_snow:altitudeM".into());
        }
    }

    {
        let base = &bridge;
        let base_sig = sig(&evaluate(base));
        let tree0 = serde_json::to_value(base).expect("json");
        let mut leaves = Vec::new();
        walk("", &tree0, &mut leaves);
        for path in leaves {
            if is_exempt(&path) {
                continue;
            }
            if !path.starts_with("bridgeSls") {
                continue;
            }
            let cur = match get_json(&tree0, &path) {
                Some(v) => v.clone(),
                None => continue,
            };
            let Some(next) = perturb(&cur) else { continue };
            let mut tree = tree0.clone();
            if !set_json(&mut tree, &path, next) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1990Snapshot>(tree) else { continue };
            checked += 1;
            if sig(&evaluate(&perturbed)) == base_sig {
                unchanged.push(format!("bridge:{path}"));
            }
        }
    }

    
    {
        let base = semio_s_artifact_norm_en1990_examples::accidental_seismic_compliant::reference_snapshot();
        let base_report = evaluate(&base);
        assert!(base_report.checks.iter().any(|c| c.clause.section == "6.11b" && c.status != CheckStatus::NotApplicable));
        assert!(base_report.checks.iter().any(|c| c.clause.section == "6.12b" && c.status != CheckStatus::NotApplicable));
        let base_sig = sig(&base_report);
        let tree0 = serde_json::to_value(&base).expect("json");
        let mut leaves = Vec::new();
        walk("", &tree0, &mut leaves);
        for path in leaves {
            if is_exempt(&path) {
                continue;
            }
            if !(path.starts_with("accidentals") || path.starts_with("seismics")) {
                continue;
            }
            let cur = match get_json(&tree0, &path) {
                Some(v) => v.clone(),
                None => continue,
            };
            let next = if path.ends_with("importanceClass") {
                let s = cur.as_str().unwrap_or("II");
                serde_json::json!(match s { "I" => "II", "II" => "III", "III" => "IV", _ => "I" })
            } else {
                match perturb_leaf(&path, &cur, &tree0) {
                    Some(v) => v,
                    None => continue,
                }
            };
            let mut tree = tree0.clone();
            if !set_json(&mut tree, &path, next) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1990Snapshot>(tree) else { continue };
            checked += 1;
            let after = evaluate(&perturbed);
            if sig(&after) == base_sig {
                unchanged.push(format!("accidental_seismic:{path}"));
            }
        }
        let mut cleared = base.clone();
        cleared.accidentals.clear();
        cleared.seismics.clear();
        cleared.effects.retain(|e| e.action_id != "A-impact" && e.action_id != "E-1");
        let cleared_report = evaluate(&cleared);
        assert!(cleared_report.checks.iter().any(|c| c.clause.section == "6.11b" && c.status == CheckStatus::NotApplicable));
        assert!(cleared_report.checks.iter().any(|c| c.clause.section == "6.12b" && c.status == CheckStatus::NotApplicable));
    }

    assert!(checked >= 20, "expected broad leaf coverage, checked={checked}");
    assert!(unchanged.is_empty(), "leaves with no check influence: {unchanged:?} (checked={checked})");
}

const LABEL_EXCEPTIONS: &[&str] = &["projectId", "labelEn", "labelDe"];

fn leaf_name(path: &str) -> &str {
    path.rsplit(['.', ']']).next().unwrap_or(path).trim_start_matches('.')
}

fn is_exempt(path: &str) -> bool {
    let leaf = leaf_name(path);
    LABEL_EXCEPTIONS.contains(&leaf) || leaf == "name" || leaf == "title"
}

fn table_name(path: &str) -> Option<&str> {
    path.split('[').next()
}

fn sibling_id(path: &str, tree: &serde_json::Value, current: &str) -> Option<String> {
    let table = table_name(path)?;
    let arr = get_json(tree, table)?.as_array()?;
    for item in arr {
        if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
            if id != current {
                return Some(id.to_string());
            }
        }
    }
    None
}

fn perturb_leaf(path: &str, cur: &serde_json::Value, tree: &serde_json::Value) -> Option<serde_json::Value> {
    let leaf = leaf_name(path);
    if leaf == "memberId" || leaf == "actionId" {
        let s = cur.as_str()?;
        return Some(serde_json::json!(format!("{s}-x")));
    }
    if leaf == "id" {
        let s = cur.as_str()?;
        if let Some(sibling) = sibling_id(path, tree, s) {
            return Some(serde_json::json!(sibling));
        }
        return Some(serde_json::json!(format!("{s}-x")));
    }
    perturb(cur)
}

fn walk(prefix: &str, value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let p = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                walk(&p, v, out);
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                let id = v.get("id").and_then(|x| x.as_str());
                let p = match id {
                    Some(id) => format!("{prefix}[id={id}]"),
                    None => format!("{prefix}[{i}]"),
                };
                walk(&p, v, out);
            }
        }
        _ => {
            if !prefix.is_empty() {
                out.push(prefix.to_string());
            }
        }
    }
}

fn get_json<'a>(root: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut cur = root;
    let mut rest = path;
    while !rest.is_empty() {
        if let Some(r) = rest.strip_prefix('.') {
            rest = r;
        }
        if let Some(r) = rest.strip_prefix('[') {
            let close = r.find(']')?;
            let inner = &r[..close];
            let arr = cur.as_array()?;
            cur = if let Some(id) = inner.strip_prefix("id=") {
                arr.iter().find(|v| v.get("id").and_then(|x| x.as_str()) == Some(id))?
            } else {
                arr.get(inner.parse::<usize>().ok()?)?
            };
            rest = &r[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        let key = &rest[..end];
        cur = cur.get(key)?;
        rest = &rest[end..];
    }
    Some(cur)
}

fn set_json(root: &mut serde_json::Value, path: &str, value: serde_json::Value) -> bool {
    let mut cur = root;
    let mut rest = path;
    while !rest.is_empty() {
        if let Some(r) = rest.strip_prefix('.') {
            rest = r;
        }
        if let Some(r) = rest.strip_prefix('[') {
            let close = match r.find(']') {
                Some(c) => c,
                None => return false,
            };
            let inner = &r[..close];
            let arr = match cur.as_array_mut() {
                Some(a) => a,
                None => return false,
            };
            let idx = if let Some(id) = inner.strip_prefix("id=") {
                match arr.iter().position(|v| v.get("id").and_then(|x| x.as_str()) == Some(id)) {
                    Some(i) => i,
                    None => return false,
                }
            } else {
                match inner.parse::<usize>() {
                    Ok(i) => i,
                    Err(_) => return false,
                }
            };
            rest = &r[close + 1..];
            if rest.is_empty() {
                arr[idx] = value;
                return true;
            }
            cur = &mut arr[idx];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        let key = &rest[..end];
        rest = &rest[end..];
        if rest.is_empty() {
            if let Some(obj) = cur.as_object_mut() {
                obj.insert(key.to_string(), value);
                return true;
            }
            return false;
        }
        cur = match cur.get_mut(key) {
            Some(v) => v,
            None => return false,
        };
    }
    false
}

fn perturb(cur: &serde_json::Value) -> Option<serde_json::Value> {
    match cur {
        serde_json::Value::Number(n) => {
            let v = n.as_f64().unwrap_or(0.0);
            let nv = if v.abs() < 1e-12 {
                1.0
            } else if (0.0..1000.0).contains(&v) && (v - 0.0).abs() < 1e-9 {
                1200.0
            } else {
                v * 1.25 + 0.5
            };
            Some(serde_json::json!(nv))
        }
        serde_json::Value::String(s) => {
            let ns = match s.as_str() {
                "De" => "En",
                "En" => "De",
                "building" => "roadBridge",
                "roadBridge" | "footbridge" | "railBridge" => "building",
                "dsl1" => "dsl2",
                "dsl2" => "dsl3",
                "dsl3" => "dsl1",
                "il1" => "il2",
                "il2" => "il3",
                "il3" => "il1",
                "office" => "snow",
                "wind" => "snow",
                "snow" => "wind",
                "road_traffic" => "rail_traffic",
                "g_sup" => "g_inf",
                "g_inf" => "prestress",
                other => return Some(serde_json::json!(format!("{other}-x"))),
            };
            Some(serde_json::json!(ns))
        }
        serde_json::Value::Bool(b) => Some(serde_json::json!(!b)),
        _ => None,
    }
}

fn sig(report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<(String, CheckStatus, i64, i64, i64)> {
    report
        .checks
        .iter()
        .map(|c| {
            (
                c.id.clone(),
                c.status,
                (c.computed.value * 1e3).round() as i64,
                (c.limit.value * 1e3).round() as i64,
                (c.utilization * 1e6).round() as i64,
            )
        })
        .collect()
}

/// 🔮️ EN 1990 third-party oracle cases: the shared harness recomputes every blessed case in `🧪️tests/🧫️fixtures/🔮️oracle-cases` with the registered libraries and fails on any mismatch beyond the tolerance profile or between libraries.
#[test]
fn third_party_libraries_reproduce_the_blessed_oracle_cases() {
    let verified = semio_s_plugin_norm_compliance::python_oracle::verify_cases_above(env!("CARGO_MANIFEST_DIR"));
    assert!(verified >= 12, "expected at least 12 verified oracle cases, got {verified}");
}

/// 🧪️ A subject with a spectrum, two FORM analyses and two test series: the document-driven Annex B/C/D and Miner checks.
fn analysed() -> En1990Snapshot {
    use semio_s_artifact_norm_en1990_model::{FatigueSpectrum, ReliabilityAnalysis, TestSeries};
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.fatigue_spectra = vec![
        FatigueSpectrum { id: "FS-1".into(), member_id: "beam-B1".into(), detail_category: 71.0e6, stress_range: 80.0e6, cycles: 2.0e6 },
        FatigueSpectrum { id: "FS-2".into(), member_id: "beam-B1".into(), detail_category: 71.0e6, stress_range: 40.0e6, cycles: 2.0e6 },
        FatigueSpectrum { id: "FS-3".into(), member_id: "beam-B1".into(), detail_category: 71.0e6, stress_range: 20.0e6, cycles: 1.0e7 },
    ];
    doc.reliability_analyses = vec![
        ReliabilityAnalysis { id: "RA-1".into(), resistance_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Lognormal, resistance_mean: 600.0e3, resistance_sd: 60.0e3, action_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Gumbel, action_mean: 250.0e3, action_sd: 50.0e3, second_action_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Normal, second_action_mean: 0.0, second_action_sd: 0.0, member_id: String::new() },
        ReliabilityAnalysis { id: "RA-2".into(), resistance_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Normal, resistance_mean: 300.0e3, resistance_sd: 45.0e3, action_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Normal, action_mean: 200.0e3, action_sd: 40.0e3, second_action_distribution: semio_s_artifact_norm_en1990_model::DistributionKind::Normal, second_action_mean: 0.0, second_action_sd: 0.0, member_id: String::new() },
    ];
    doc.test_series = vec![
        TestSeries { id: "TS-1".into(), quantity: "yield strength".into(), distribution: semio_s_artifact_norm_en1990_model::TestDistribution::Normal, sample_size: 10, mean: 30.0, std_dev: 3.0, variation_known: 0.0, conversion_factor: 1.0, required: 20.0, pairs: String::new(), theory_variation: 0.0, theory_mean: 0.0 },
        TestSeries { id: "TS-2".into(), quantity: "bond strength".into(), distribution: semio_s_artifact_norm_en1990_model::TestDistribution::Lognormal, sample_size: 6, mean: 12.0, std_dev: 1.5, variation_known: 0.1, conversion_factor: 0.9, required: 10.0, pairs: String::new(), theory_variation: 0.0, theory_mean: 0.0 },
    ];
    doc
}

#[semio_framework_async_macros::async_test]
async fn analysis_rows_produce_miner_form_and_annex_d_checks() {
    let report = evaluate(&analysed());
    let get = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing {id}"));
    let damage = get("en1990.fat.damage.beam-B1");
    assert!(damage.computed.value > 1.0 && damage.status == CheckStatus::Fail && !damage.remedies.is_empty(), "D = {}", damage.computed.value);
    let form = get("en1990.annex-c.form.RA-1");
    assert!(form.computed.value > 3.0 && form.computed.value < 7.0, "beta = {}", form.computed.value);
    assert!(get("en1990.annex-c.form.RA-2").status == CheckStatus::Fail);
    assert_eq!(get("en1990.annex-d.TS-1").status, CheckStatus::Pass);
    assert!(get("en1990.annex-d.TS-1").computed.value > 20.0 && get("en1990.annex-d.TS-1").computed.value < 30.0);
    assert!(report.checks.iter().filter(|c| c.status == CheckStatus::Fail).all(|c| !c.remedies.is_empty()), "every Fail needs a remedy");
}

#[semio_framework_async_macros::async_test]
async fn indeterminate_analyses_are_not_applicable_and_never_pass() {
    let mut doc = analysed();
    doc.test_series[0].sample_size = 2;
    doc.fatigue_spectra[0].detail_category = 0.0;
    let report = evaluate(&doc);
    for id in ["en1990.annex-d.TS-1", "en1990.fat.damage.beam-B1"] {
        assert_eq!(report.checks.iter().find(|c| c.id == id).expect(id).status, CheckStatus::NotApplicable, "{id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn dangling_spectrum_member_and_duplicate_analysis_ids_fail_integrity() {
    let mut doc = analysed();
    doc.fatigue_spectra[0].member_id = "missing".into();
    doc.test_series[1].id = "TS-1".into();
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id == "en1990.integrity.fatigueSpectra.FS-1.memberId" && c.status == CheckStatus::Fail));
    assert!(report.checks.iter().any(|c| c.id == "en1990.integrity.duplicate.testSeries.TS-1" && c.status == CheckStatus::Fail));
}

#[semio_framework_async_macros::async_test]
async fn analyses_evaluate_without_members_and_per_row_units_equal_the_whole() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let mut doc = analysed();
    doc.members.clear();
    doc.effects.clear();
    doc.fatigue_spectra.clear();
    let whole = evaluate(&doc);
    assert!(whole.checks.iter().any(|c| c.id == "en1990.annex-c.form.RA-1"));
    let parts = En1990Units::units(&analysed());
    assert!(parts.len() >= 5, "global, member, two analyses and two series units, got {}", parts.len());
}

#[semio_framework_async_macros::async_test]
async fn exclusive_variable_group_never_acts_together() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.variables.push(semio_s_artifact_norm_en1990_model::VariableAction { id: "Q-span2".into(), category: "office".into(), qk: 40_000.0, group: "pattern".into() });
    doc.variables[0].group = "pattern".into();
    doc.effects.push(semio_s_artifact_norm_en1990_model::MemberEffect { member_id: "beam-B1".into(), action_id: "Q-span2".into(), influence: 1.0 });
    let grouped = member_actions(&doc, "beam-B1");
    assert!(grouped.iter().filter(|a| a.group == "pattern").count() == 2);
    let ctx = context_for(&doc, FactorSet::StrB).expect("context");
    let together: f64 = combinations(&ctx, &grouped, Equation::E610).expect("combinations").iter().map(|c| c.total()).fold(f64::MIN, f64::max);
    let mut apart = doc.clone();
    apart.variables[0].group.clear();
    apart.variables[3].group.clear();
    let free: f64 = combinations(&context_for(&apart, FactorSet::StrB).expect("context"), &member_actions(&apart, "beam-B1"), Equation::E610).expect("combinations").iter().map(|c| c.total()).fold(f64::MIN, f64::max);
    assert!(together < free, "exclusive pattern loads {together} must stay below independent loads {free}");
}

#[semio_framework_async_macros::async_test]
async fn table_b2_is_continuous_and_monotonic_between_and_beyond_its_anchors() {
    for rc in 1u8..=3 {
        let betas: Vec<f64> = [0.5, 1.0, 2.0, 10.0, 40.0, 50.0, 51.0, 100.0].iter().map(|t| target_reliability_index_for(rc, *t).expect("beta")).collect();
        assert!(betas.windows(2).all(|w| w[1] < w[0]), "RC{rc} must fall with T: {betas:?}");
        let (below, above) = (target_reliability_index_for(rc, 49.999).unwrap(), target_reliability_index_for(rc, 50.001).unwrap());
        assert!((below - above).abs() < 1e-3, "continuous at 50 a");
    }
}

#[semio_framework_async_macros::async_test]
async fn seismic_combination_scales_psi_by_phi() {
    let ctx = context_for(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), FactorSet::StrB).expect("context");
    let build = |scale: f64| vec![
        Action::new("G", ActionClass::Permanent, 100.0),
        Action::new("E", ActionClass::Seismic, 50.0),
        Action::new("Q", ActionClass::Variable, 40.0).category("office").psi_scale(scale),
    ];
    let total = |scale: f64| combinations(&ctx, &build(scale), Equation::E612b).expect("6.12b").iter().map(|c| c.total()).fold(f64::MIN, f64::max);
    assert!((total(0.5) - total(1.0)).abs() > 1e-9 && total(0.5) < total(1.0));
}

#[semio_framework_async_macros::async_test]
async fn geo2_uses_the_factors_of_set_b_and_geo3_those_of_set_c() {
    use semio_s_artifact_norm_en1990_checks::{gamma_g, gamma_q, Domain};
    for annex in [semio_s_plugin_norm_compliance::AnnexChoice::En, semio_s_plugin_norm_compliance::AnnexChoice::De] {
        assert_eq!(gamma_g(annex, Domain::Building, FactorSet::Geo2), gamma_g(annex, Domain::Building, FactorSet::StrB));
        assert_eq!(gamma_q(annex, Domain::Building, FactorSet::Geo2, "office"), 1.5);
        assert_eq!(gamma_q(annex, Domain::Building, FactorSet::GeoC, "office"), 1.3);
    }
}

fn bridge_document(kind: semio_s_artifact_norm_en1990_model::StructureKind, annex: semio_s_plugin_norm_compliance::AnnexChoice) -> En1990Snapshot {
    use semio_s_artifact_norm_en1990_model::{MemberEffect, VariableAction};
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.annex = annex;
    doc.structure_kind = kind;
    doc.design_working_life_category = 5;
    doc.design_working_life_years = 100.0;
    doc.reference_period_years = 100.0;
    doc.variables = vec![
        VariableAction { id: "TS".into(), category: "road_traffic".into(), qk: 60_000.0, group: String::new() },
        VariableAction { id: "SA".into(), category: "road_single_axle".into(), qk: 50_000.0, group: String::new() },
        VariableAction { id: "W".into(), category: "wind_bridge".into(), qk: 20_000.0, group: String::new() },
    ];
    doc.effects = ["G-sup", "TS", "SA", "W"].iter().map(|action| MemberEffect { member_id: "beam-B1".into(), action_id: (*action).into(), influence: 1.0 }).collect();
    doc.effects.push(MemberEffect { member_id: "beam-B1".into(), action_id: "G-inf".into(), influence: -1.0 });
    doc
}

fn status_of(report: &semio_s_plugin_norm_compliance::CheckReport, id: &str) -> CheckStatus {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}")).status
}

#[semio_framework_async_macros::async_test]
async fn bridges_use_eq_610_only_and_buildings_keep_610ab_in_en() {
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let bridge = evaluate(&bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge, En));
    assert!(bridge.checks.iter().any(|c| c.id == "en1990.str.beam-B1.6.10"));
    assert!(bridge.checks.iter().all(|c| !c.id.ends_with("6.10a") && !c.id.ends_with("6.10b")), "Annex A2.3.1 uses Eq. 6.10 only");
    let building = evaluate(&En1990Snapshot { annex: En, ..En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De) });
    assert!(building.checks.iter().any(|c| c.id == "en1990.str.beam-B1.6.10a") && building.checks.iter().any(|c| c.id == "en1990.str.beam-B1.6.10b"));
}

#[semio_framework_async_macros::async_test]
async fn traffic_load_groups_must_be_exclusive_and_the_remedy_names_the_group() {
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let mut doc = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge, En);
    let report = evaluate(&doc);
    let check = report.checks.iter().find(|c| c.id == "en1990.a2.traffic-groups").expect("traffic groups check");
    assert_eq!(check.status, CheckStatus::Warning);
    assert!(!check.remedies.is_empty(), "a warning carries the remedy");
    doc.variables[0].group = "gr1a".into();
    doc.variables[1].group = "gr1b".into();
    assert_eq!(status_of(&evaluate(&doc), "en1990.a2.traffic-groups"), CheckStatus::Pass);
    doc.variables[1].group = "gr1a".into();
    assert_eq!(status_of(&evaluate(&doc), "en1990.a2.traffic-groups"), CheckStatus::Warning, "two groups may not share one group id");
    assert_eq!(status_of(&evaluate(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)), "en1990.annex-b.kfi-applied"), CheckStatus::Info);
}

#[semio_framework_async_macros::async_test]
async fn execution_actions_act_in_the_transient_situation_only() {
    use semio_s_artifact_norm_en1990_model::{MemberEffect, VariableAction};
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let mut doc = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge, En);
    doc.variables.push(VariableAction { id: "QC".into(), category: "construction_load".into(), qk: 90_000.0, group: String::new() });
    doc.effects.push(MemberEffect { member_id: "beam-B1".into(), action_id: "QC".into(), influence: 1.0 });
    let persistent = evaluate(&bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge, En));
    let with_execution = evaluate(&doc);
    let effect = |report: &semio_s_plugin_norm_compliance::CheckReport| report.checks.iter().find(|c| c.id == "en1990.str.beam-B1.6.10").expect("str").computed.value;
    assert!((effect(&persistent) - effect(&with_execution)).abs() < 1e-9, "construction loads never enter the persistent combination");
    let transient = with_execution.checks.iter().find(|c| c.id == "en1990.a2.transient.beam-B1").expect("transient check");
    let ctx = context_for(&doc, FactorSet::StrB).expect("context");
    let expected = 1.35 * 80_000.0 - 1.0 * 20_000.0 + 1.5 * 90_000.0;
    let own = combinations(&ctx.transient(), &member_actions(&doc, "beam-B1"), Equation::E610).expect("combinations");
    assert!((governing(&own, false).expect("governing").total() - expected).abs() < 1e-6 && (transient.computed.value - expected).abs() < 1e-6, "transient E_d {} vs {expected}", transient.computed.value);
    assert!(persistent.checks.iter().all(|c| c.id != "en1990.a2.transient.beam-B1"));
}

#[semio_framework_async_macros::async_test]
async fn declared_limits_more_lenient_than_the_recommendation_warn_with_a_remedy() {
    use semio_s_artifact_norm_en1990_model::BridgeSls;
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.members[0].deflection_limit_ratio = 150.0;
    doc.members[0].vibration_frequency_min = 2.0;
    let report = evaluate(&doc);
    for id in ["en1990.a14.basis.net-ratio.beam-B1", "en1990.a14.basis.vibration.beam-B1"] {
        let check = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing {id}"));
        assert!(matches!(check.status, CheckStatus::Warning | CheckStatus::UnverifiedBasis) && !check.remedies.is_empty(), "{id}: {:?}", check.status);
    }
    assert!(report.checks.iter().all(|c| c.id != "en1990.a14.basis.w3-ratio.beam-B1" || c.status != CheckStatus::Fail));
    let mut rail = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RailBridge, En);
    rail.members[0].span = 30.0;
    rail.bridge_sls = vec![BridgeSls { id: "BS".into(), member_id: "beam-B1".into(), deck_acceleration: 1.0, deck_acceleration_limit: 9.0, deck_twist: 0.001, deck_twist_limit: 0.004, bridge_deflection: 0.01, bridge_deflection_limit: 0.1, track_gauge: 0.0, line_speed: 0.0, ballast_depth: 0.0, deck_mass: 0.0, natural_frequency: 0.0, end_rotation: 0.0, basic_wind_speed: 0.0, damping_ratio: 0.0 }];
    let report = evaluate(&rail);
    for key in ["acceleration", "twist", "deflection"] {
        let check = report.checks.iter().find(|c| c.id == format!("en1990.a2.basis.{key}.beam-B1.BS")).unwrap_or_else(|| panic!("missing {key}"));
        assert!(!check.remedies.is_empty() && check.utilization > 1.0, "{key}: lenient declared limit exceeds the recommendation");
    }
}

#[semio_framework_async_macros::async_test]
async fn annex_c_derived_rows_and_annex_d_sample_warning() {
    let mut doc = analysed();
    doc.test_series[1].sample_size = 2;
    doc.test_series[1].variation_known = 0.1;
    let report = evaluate(&doc);
    let alpha = report.checks.iter().find(|c| c.id == "en1990.annex-c.alpha.RA-1").expect("alpha row");
    assert!((alpha.computed.value - 50.0 / 60.0).abs() < 1e-9, "sigma_E / sigma_R");
    assert_eq!(alpha.status, CheckStatus::Info);
    let factors = report.checks.iter().find(|c| c.id == "en1990.annex-c.partial-factors.RA-1").expect("partial factors");
    let form = analysis_form(("lognormal", 600.0e3, 60.0e3), ("gumbel", 250.0e3, 50.0e3)).expect("form");
    let r = Dist::parse("lognormal", 600.0e3, 60.0e3).expect("r");
    let e = Dist::parse("gumbel", 250.0e3, 50.0e3).expect("e");
    let wanted = (r.fractile(0.05) / form.design_r) * (form.design_e / e.fractile(0.98));
    assert!((factors.computed.value - wanted).abs() < 1e-9 * wanted.abs(), "{} vs {wanted}", factors.computed.value);
    assert_eq!(status_of(&report, "en1990.annex-d.TS-2.sample"), CheckStatus::Warning);
    assert!(report.checks.iter().all(|c| c.id != "en1990.annex-d.TS-1.sample"));
}

#[semio_framework_async_macros::async_test]
async fn member_without_actions_is_not_assessed_and_table_2_1_deviation_is_a_warning() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.effects.clear();
    doc.design_working_life_years = 80.0;
    let report = evaluate(&doc);
    assert_eq!(status_of(&report, "en1990.actions.beam-B1"), CheckStatus::NotApplicable);
    assert!(report.checks.iter().all(|c| !c.id.starts_with("en1990.str.beam-B1")), "no trivial pass against an empty combination");
    assert_eq!(status_of(&report, "en1990.table-2.1.design-working-life"), CheckStatus::Warning);
}

#[semio_framework_async_macros::async_test]
async fn na_de_psi_rows_cover_settlement_other_actions_rail_wind_and_the_stair_categories() {
    use semio_s_plugin_norm_compliance::AnnexChoice::De;
    for (category, expected) in [("settlement", (1.0, 1.0, 1.0)), ("variable_other", (0.8, 0.7, 0.5)), ("wind_rail", (0.75, 0.5, 0.0)), ("rail_hslm", (1.0, 1.0, 0.0)), ("T1", (0.7, 0.5, 0.3)), ("Z", (0.7, 0.5, 0.3))] {
        let row = psi_lookup(De, category).unwrap_or_else(|| panic!("no psi row for {category}"));
        assert_eq!((row.psi_0, row.psi_1, row.psi_2), expected, "{category}");
        assert!(psi_is_ordered(row));
    }
}

#[semio_framework_async_macros::async_test]
async fn table_d1_matches_the_student_t_derivation_of_scipy() {
    let oracle = fixture_path!("../../../../🔮️oracles/⚖️compliance/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&oracle, &[std::ffi::OsStr::new("table-d1")], None);
    let derived: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
    let nodes = derived["nodes"].as_array().expect("nodes");
    let mut compared = 0;
    for (index, node) in nodes.iter().enumerate() {
        let n = node.as_f64().expect("n");
        let known = derived["known"][index].as_f64().expect("known");
        assert!((k_n(n, true).expect("k_n") - known).abs() < 0.02, "V known n = {n}: table {:?} vs t derivation {known}", k_n(n, true));
        compared += 1;
        if let Some(unknown) = derived["unknown"][index].as_f64() {
            assert!((k_n(n, false).expect("k_n") - unknown).abs() < 0.02, "V unknown n = {n}: table {:?} vs t derivation {unknown}", k_n(n, false));
            compared += 1;
        } else {
            assert!(k_n(n, false).is_none());
        }
    }
    assert!((k_n(f64::INFINITY, true).expect("k_n") - derived["infinity"].as_f64().expect("u")).abs() < 0.01 && compared >= 18);
}

#[semio_framework_async_macros::async_test]
async fn table_d2_stays_within_the_scipy_closed_form_band() {
    let oracle = fixture_path!("../../../../🔮️oracles/⚖️compliance/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&oracle, &[std::ffi::OsStr::new("table-d2")], None);
    let derived: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
    for (index, node) in derived["nodes"].as_array().expect("nodes").iter().enumerate() {
        let n = node.as_f64().expect("n");
        let closed = derived["known"][index].as_f64().expect("closed");
        let printed = k_d_n(n, true).expect("k_d_n");
        assert_eq!(printed, derived["printed"][index].as_f64().expect("printed"));
        assert!(printed >= closed && printed - closed < 0.07, "n = {n}: {printed} vs {closed}");
    }
    assert!((k_d_n(f64::INFINITY, true).expect("k_d_n") - derived["infinity"].as_f64().expect("u")).abs() < 0.01);
}

/// 🧪️ The V-unknown row of Table D2 is not transcribed (print checklist W3-S01c): no value is invented, so a series with unknown V_X gets no direct design value.
#[test]
fn table_d2_has_no_value_for_an_unknown_variation_coefficient() {
    for n in [3.0, 10.0, 100.0, f64::INFINITY] {
        assert!(k_d_n(n, false).is_none(), "n = {n}");
    }
}

fn utilization_of(report: &semio_s_plugin_norm_compliance::CheckReport, id: &str) -> f64 {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}")).utilization
}

#[semio_framework_async_macros::async_test]
async fn rail_twist_acceleration_and_end_rotation_follow_the_line_speed() {
    use semio_s_artifact_norm_en1990_model::BridgeSls;
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let row = |speed: f64, ballast: f64| BridgeSls { id: "BS".into(), member_id: "beam-B1".into(), deck_acceleration: 4.0, deck_acceleration_limit: 9.0, deck_twist: 0.002, deck_twist_limit: 0.004, bridge_deflection: 0.0, bridge_deflection_limit: 0.0, track_gauge: 1.435, line_speed: speed, ballast_depth: ballast, deck_mass: 20_000.0, natural_frequency: 4.0, end_rotation: 0.006, basic_wind_speed: 0.0, damping_ratio: 0.0 };
    let run = |speed: f64, ballast: f64| {
        let mut rail = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RailBridge, En);
        rail.members[0].span = 30.0;
        rail.bridge_sls = vec![row(speed, ballast)];
        evaluate(&rail)
    };
    let slow = run(100.0, 0.0);
    assert!((utilization_of(&slow, "en1990.a2.rail-twist.beam-B1.BS") - 2.87 / 4.5).abs() < 1e-9, "t = 0.002 rad · 1.435 m · 1000 against the lowest band");
    assert!(utilization_of(&slow, "en1990.a2.rail-acceleration.beam-B1.BS") < 1.0, "slab track allows 5 m/s²");
    assert!(utilization_of(&slow, "en1990.a2.rail-end-rotation.beam-B1.BS") < 1.0);
    let ballasted = run(100.0, 0.35);
    assert!(utilization_of(&ballasted, "en1990.a2.rail-acceleration.beam-B1.BS") > 1.0, "ballasted track allows 3.5 m/s²");
    let fast = run(250.0, 0.0);
    assert!(utilization_of(&fast, "en1990.a2.rail-twist.beam-B1.BS") > 1.0, "2.87 mm exceeds the 1.5 mm band above 200 km/h");
    assert!(utilization_of(&fast, "en1990.a2.rail-end-rotation.beam-B1.BS") > 1.0, "the high-speed end rotation limit is tighter");
    let building = evaluate(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(building.checks.iter().all(|c| !c.id.starts_with("en1990.a2.rail-")));
}

#[semio_framework_async_macros::async_test]
async fn building_frame_and_storey_drift_use_h_over_500_and_h_over_300() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.members[0].frame_height = 12.0;
    doc.members[0].frame_drift = 0.03;
    doc.members[0].storey_height = 3.0;
    doc.members[0].storey_drift = 0.008;
    let report = evaluate(&doc);
    assert!((utilization_of(&report, "en1990.a14.frame-drift.beam-B1") - 0.03 / (12.0 / 500.0)).abs() < 1e-9);
    assert!((utilization_of(&report, "en1990.a14.storey-drift.beam-B1") - 0.008 / (3.0 / 300.0)).abs() < 1e-9);
    assert!(evaluate(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)).checks.iter().all(|c| !c.id.starts_with("en1990.a14.frame-drift") && !c.id.starts_with("en1990.a14.storey-drift")));
}

#[test]
fn wind_and_thermal_never_act_together_and_the_wind_cap_applies_with_traffic() {
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let ctx = Context::new(En, Domain::Bridge, FactorSet::StrB, 1.0);
    assert!(ctx.wind_thermal_exclusive, "the registry default excludes simultaneous wind and thermal actions");
    let actions = vec![
        Action::new("G", ActionClass::Permanent, 100.0),
        Action::new("TS", ActionClass::Variable, 60.0).category("road_traffic").group("gr1a"),
        Action::new("W", ActionClass::Variable, 20.0).category("wind_bridge").cap(5.0),
        Action::new("T", ActionClass::Variable, 10.0).category("temperature_bridge"),
    ];
    let has = |combination: &Combination, id: &str| combination.terms.iter().any(|term| term.action == id);
    let all = combinations(&ctx, &actions, Equation::E610).expect("combinations");
    assert!(all.iter().all(|combination| !(has(combination, "W") && has(combination, "T"))), "no combination holds wind and thermal together");
    assert!(all.iter().any(|combination| combination.excluded == "thermal" && has(combination, "W")) && all.iter().any(|combination| combination.excluded == "wind" && has(combination, "T")));
    for combination in all.iter().filter(|combination| has(combination, "W")) {
        let term = combination.terms.iter().find(|term| term.action == "W").expect("wind term");
        assert!((term.contribution - term.factor * 5.0).abs() < 1e-9, "F*_W is capped at 5 while traffic acts");
    }
    let free = Context { wind_thermal_exclusive: false, ..ctx };
    assert!(combinations(&free, &actions, Equation::E610).expect("combinations").iter().any(|combination| has(combination, "W") && has(combination, "T")));
    let alone = vec![Action::new("G", ActionClass::Permanent, 100.0), Action::new("W", ActionClass::Variable, 20.0).category("wind_bridge").cap(5.0)];
    let wind = combinations(&ctx, &alone, Equation::E610).expect("combinations");
    assert!(wind.iter().all(|combination| combination.terms.iter().filter(|term| term.action == "W").all(|term| (term.contribution - term.factor * 20.0).abs() < 1e-9)), "without traffic the wind action is uncapped");
}

#[semio_framework_async_macros::async_test]
async fn the_declared_basic_wind_speed_caps_the_member_wind_action_at_the_registry_speed() {
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let mut doc = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge, En);
    doc.bridge_sls = vec![semio_s_artifact_norm_en1990_model::BridgeSls { id: "BS".into(), member_id: "beam-B1".into(), deck_acceleration: 0.0, deck_acceleration_limit: 0.0, deck_twist: 0.0, deck_twist_limit: 0.0, bridge_deflection: 0.0, bridge_deflection_limit: 0.0, track_gauge: 0.0, line_speed: 0.0, ballast_depth: 0.0, deck_mass: 0.0, natural_frequency: 0.0, end_rotation: 0.0, basic_wind_speed: 50.0, damping_ratio: 0.0 }];
    let wind = |doc: &En1990Snapshot| member_actions(doc, "beam-B1").into_iter().find(|action| action.id == "W").expect("wind action").cap;
    let capped = wind(&doc).expect("capped");
    assert!((capped - 20_000.0 * (35.0_f64 / 50.0).powi(2)).abs() < 1e-6);
    doc.bridge_sls[0].basic_wind_speed = 30.0;
    assert_eq!(wind(&doc), None, "a basic wind speed below the cap speed needs no cap");
    doc.bridge_sls[0].basic_wind_speed = 0.0;
    assert_eq!(wind(&doc), None, "no declared speed, no cap");
}

#[semio_framework_async_macros::async_test]
async fn every_traced_verification_id_is_realised_by_a_document_check() {
    use semio_s_artifact_norm_en1990_model::{MemberEffect, VariableAction};
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let empty = En1990Snapshot { members: Vec::new(), ..En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De) };
    let mut execution = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge, En);
    execution.variables.push(VariableAction { id: "QC".into(), category: "construction_load".into(), qk: 90_000.0, group: String::new() });
    execution.effects.push(MemberEffect { member_id: "beam-B1".into(), action_id: "QC".into(), influence: 1.0 });
    let mut rail = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RailBridge, En);
    rail.members[0].span = 30.0;
    rail.bridge_sls = vec![semio_s_artifact_norm_en1990_model::BridgeSls { id: "BS".into(), member_id: "beam-B1".into(), deck_acceleration: 1.0, deck_acceleration_limit: 9.0, deck_twist: 0.001, deck_twist_limit: 0.004, bridge_deflection: 0.01, bridge_deflection_limit: 0.1, track_gauge: 1.435, line_speed: 160.0, ballast_depth: 0.0, deck_mass: 20_000.0, natural_frequency: 4.0, end_rotation: 0.004, basic_wind_speed: 0.0, damping_ratio: 0.0 }];
    rail.bridge_sls[0].natural_frequency = 3.0;
    rail.bridge_sls[0].deck_mass = 6000.0;
    rail.bridge_sls[0].line_speed = rail.bridge_sls[0].line_speed.max(220.0);
    let mut extended = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    extended.annex = En;
    extended.members[0].material_strength_k = 355.0e6;
    extended.members[0].material_partial_factor = 1.1;
    extended.members[0].nominal_dimension = 0.3;
    extended.members[0].dimension_tolerance = 0.005;
    extended.members[0].member_alpha_cr = 12.0;
    extended.factor_overrides = vec![override_row("O1", "psi0", "Q-wind", 0.4, "wind tunnel study")];
    let documents = [
        En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De),
        extended,
        empty,
        analysed(),
        execution,
        rail,
        semio_s_artifact_norm_en1990_examples::fatigue_compliant::reference_snapshot(),
        semio_s_artifact_norm_en1990_examples::accidental_seismic_compliant::reference_snapshot(),
    ];
    let ids: Vec<String> = documents.iter().flat_map(|document| evaluate(document).checks.into_iter().map(|check| check.id)).collect();
    let missing: Vec<&str> = semio_s_artifact_norm_en1990_checks::spec_trace::TABLE
        .items
        .iter()
        .filter_map(|item| match &item.realisation {
            semio_s_plugin_norm_compliance::spec_trace::Realisation::Check(fragments) => Some(fragments.iter().copied()),
            _ => None,
        })
        .flatten()
        .filter(|fragment| !ids.iter().any(|id| id.contains(fragment)))
        .collect();
    assert!(missing.is_empty(), "spec-trace fragments no document emits (catalogue keys must follow the clause spelling 6.11b, 6.12b): {missing:?}");
    let emitted = documents
        .iter()
        .fold(semio_s_plugin_norm_compliance::spec_trace::Emitted::default(), |emitted, document| emitted.with_checks(evaluate(document).checks.into_iter().map(|check| check.id)))
        .with_case_catalogue::<semio_s_artifact_norm_en1990_checks::catalogue::En1990Cases>(En);
    semio_s_plugin_norm_compliance::spec_trace::assert_traced(&semio_s_artifact_norm_en1990_checks::spec_trace::TABLE, &emitted);
}

fn override_row(id: &str, factor: &str, target: &str, value: f64, justification: &str) -> semio_s_artifact_norm_en1990_model::FactorOverride {
    semio_s_artifact_norm_en1990_model::FactorOverride { id: id.into(), factor: factor.into(), target: target.into(), value, justification: justification.into() }
}

fn term_factor(doc: &En1990Snapshot, action: &str) -> (f64, bool) {
    let ctx = context_for(doc, FactorSet::StrB).expect("context");
    let all = combinations(&ctx, &member_actions(doc, "beam-B1"), Equation::E610).expect("combinations");
    let combination = all.iter().find(|c| c.leading == "Q-office").expect("office leading");
    let term = combination.terms.iter().find(|t| t.action == action).expect("term");
    (term.factor, term.deviates)
}

#[test]
fn an_allowed_psi_override_changes_the_combination_and_warns_with_the_annex_value() {
    use semio_s_plugin_norm_compliance::AnnexChoice::De;
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.annex = De;
    let (annex_factor, annex_deviates) = term_factor(&doc, "Q-wind");
    assert!(!annex_deviates);
    doc.factor_overrides = vec![override_row("O1", "psi0", "Q-wind", 0.3, "wind tunnel study")];
    let (factor, deviates) = term_factor(&doc, "Q-wind");
    assert!(deviates && (factor - annex_factor / psi_lookup(De, "wind").unwrap().psi_0 * 0.3).abs() < 1e-9, "factor {factor}");
    let report = evaluate(&doc);
    let check = report.checks.iter().find(|c| c.id == "en1990.override.O1").expect("override check");
    assert_eq!(check.status, CheckStatus::Warning);
    assert!(check.explanation.en.contains("Deviates from the National Annex") && check.explanation.de.contains("Weicht vom Nationalen Anhang ab"));
    assert!(!check.remedies.is_empty());
    assert!(report.checks.iter().any(|c| c.id.starts_with("en1990.str.beam-B1") && c.explanation.en.contains("deviates from NA")));
}

#[test]
fn a_row_reset_to_the_annex_value_is_applied_without_deviating() {
    use semio_s_plugin_norm_compliance::AnnexChoice::De;
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.annex = De;
    let annex = factor_overrides::annex_for_action(&doc, "psi0", "Q-wind").expect("annex psi0");
    doc.factor_overrides = vec![override_row("O1", "psi0", "Q-wind", annex, "reset")];
    assert_eq!(factor_overrides::verdicts(&doc), vec![factor_overrides::Verdict::Applied]);
    assert_eq!(factor_overrides::deviations(&doc), vec![false]);
    assert!(factor_overrides::action_override(&doc, "Q-wind").is_empty());
    assert!(!term_factor(&doc, "Q-wind").1);
    let report = evaluate(&doc);
    let check = report.checks.iter().find(|c| c.id == "en1990.override.O1").expect("override check");
    assert_eq!(check.status, CheckStatus::Info);
    assert!(check.remedies.is_empty() && check.explanation.en.contains("no effect") && check.explanation.de.contains("wirkungslos"));
    doc.factor_overrides = vec![override_row("O0", "psi0", "", 0.3, "brief"), override_row("O1", "psi0", "Q-wind", annex, "reset")];
    assert_eq!(factor_overrides::deviations(&doc), vec![true, false]);
    assert_eq!(factor_overrides::action_override(&doc, "Q-wind").psi[0], None, "the later, more specific reset wins over the general row");
    assert_eq!(factor_overrides::action_override(&doc, "Q-office").psi[0], Some(0.3));
    assert_eq!(factor_overrides::rows_for(&doc, "Q-wind").len(), 2);
}

#[test]
fn a_gamma_override_applies_only_where_the_annex_allows_it() {
    use semio_s_plugin_norm_compliance::AnnexChoice::{De, En};
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.annex = En;
    let (base, _) = term_factor(&doc, "Q-office");
    doc.factor_overrides = vec![override_row("G1", "gamma", "Q-office", 1.2, "controlled use")];
    let (factor, deviates) = term_factor(&doc, "Q-office");
    assert!(deviates && (factor - 1.2).abs() < 1e-9 && factor < base);
    doc.annex = De;
    let (factor_de, deviates_de) = term_factor(&doc, "Q-office");
    assert!(!deviates_de && factor_de > 1.2, "the National Annex fixes gamma: the row is ignored");
    let report = evaluate(&doc);
    let check = report.checks.iter().find(|c| c.id == "en1990.override.G1").expect("override check");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(!check.remedies.is_empty());
}

#[test]
fn invalid_unjustified_unknown_and_superseded_override_rows_never_reach_the_generator() {
    use semio_s_plugin_norm_compliance::AnnexChoice::De;
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.annex = De;
    doc.factor_overrides = vec![
        override_row("A", "psi0", "Q-wind", 1.7, "too large"),
        override_row("B", "psi0", "Q-wind", 0.4, ""),
        override_row("C", "psi1", "nothing", 0.4, "x"),
        override_row("D", "chi", "Q-wind", 0.4, "x"),
        override_row("E", "psi2", "Q-snow", 0.1, "first"),
        override_row("F", "psi2", "Q-snow", 0.2, "second"),
    ];
    let verdicts = factor_overrides::verdicts(&doc);
    use factor_overrides::Verdict::*;
    assert_eq!(verdicts, vec![InvalidValue, MissingJustification, UnknownTarget, UnknownKind, Superseded, Applied]);
    assert_eq!(factor_overrides::action_override(&doc, "Q-snow").psi[2], Some(0.2));
    assert!(factor_overrides::action_override(&doc, "Q-wind").is_empty());
    let report = evaluate(&doc);
    assert_eq!(report.checks.iter().filter(|c| c.id.starts_with("en1990.override.") && c.status == CheckStatus::Fail).count(), 4);
    assert!(report.checks.iter().filter(|c| c.id.starts_with("en1990.override.") && c.status == CheckStatus::Fail).all(|c| !c.remedies.is_empty()));
}

#[test]
fn the_xi_and_k_fi_overrides_reach_the_context() {
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    doc.annex = En;
    doc.factor_overrides = vec![override_row("X", "xi", "", 0.8, "client brief"), override_row("K", "k_fi", "", 1.0, "client brief")];
    let ctx = context_for(&doc, FactorSet::StrB).expect("context");
    assert_eq!(ctx.xi, Some(0.8));
    assert!(!ctx.k_fi_overridden, "K_FI is not overridable in the registry default");
    assert_eq!(factor_overrides::verdicts(&doc), vec![factor_overrides::Verdict::Applied, factor_overrides::Verdict::NotAllowed]);
}

#[test]
fn material_geometry_and_analysis_rows_follow_e90_10_to_e90_12() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let member = &mut doc.members[0];
    member.material_strength_k = 355.0e6;
    member.material_partial_factor = 1.1;
    member.material_conversion_factor = 0.9;
    member.material_design_strength = 300.0e6;
    member.nominal_dimension = 0.3;
    member.dimension_tolerance = 0.005;
    member.design_dimension = 0.302;
    member.member_alpha_cr = 7.0;
    let report = evaluate(&doc);
    let xd = 0.9 * 355.0e6 / 1.1;
    assert!((utilization_of(&report, "en1990.e10.material.beam-B1") - 300.0e6 / xd).abs() < 1e-9);
    assert_eq!(status_of(&report, "en1990.e10.material.beam-B1"), CheckStatus::Fail);
    assert!(!report.checks.iter().find(|c| c.id == "en1990.e10.material.beam-B1").unwrap().remedies.is_empty());
    assert_eq!(status_of(&report, "en1990.e11.geometry.beam-B1"), CheckStatus::Fail, "0.002 m is less than the tolerance 0.005 m");
    assert_eq!(status_of(&report, "en1990.e12.analysis.beam-B1"), CheckStatus::Fail, "first order needs alpha_cr >= 10");
    let member = &mut doc.members[0];
    member.material_design_strength = 280.0e6;
    member.design_dimension = 0.306;
    member.second_order_analysis = 1.0;
    let ok = evaluate(&doc);
    assert_eq!(status_of(&ok, "en1990.e10.material.beam-B1"), CheckStatus::Pass);
    assert_eq!(status_of(&ok, "en1990.e11.geometry.beam-B1"), CheckStatus::Pass);
    assert_eq!(status_of(&ok, "en1990.e12.analysis.beam-B1"), CheckStatus::Info);
}

#[test]
fn the_figure_6_10_limits_and_the_dynamic_flag_follow_span_and_speed() {
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let (lower, upper) = rail_dynamics::frequency_limits(En, 10.0).expect("limits");
    assert!((lower - 8.0).abs() < 1e-12 && (upper - 94.76 * 10f64.powf(-0.748)).abs() < 1e-12);
    let (lower_long, _) = rail_dynamics::frequency_limits(En, 40.0).expect("limits");
    assert!((lower_long - 23.58 * 40f64.powf(-0.592)).abs() < 1e-12);
    assert!(rail_dynamics::frequency_limits(En, 120.0).is_none());
    use rail_dynamics::DynamicReason::*;
    assert_eq!(rail_dynamics::dynamic_need(En, 10.0, 10.0, 160.0), (false, NotRequired));
    assert_eq!(rail_dynamics::dynamic_need(En, 10.0, 6.0, 160.0), (true, BelowLowerLimit));
    assert_eq!(rail_dynamics::dynamic_need(En, 10.0, 30.0, 160.0), (true, AboveUpperLimit));
    assert_eq!(rail_dynamics::dynamic_need(En, 10.0, 10.0, 250.0), (true, FastLine));
    assert_eq!(rail_dynamics::dynamic_need(En, 150.0, 10.0, 100.0), (true, SpanOutOfRange));
}

#[test]
fn the_kernel_deck_modes_and_the_modal_superposition_show_resonance_and_damping() {
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let modes = rail_dynamics::DeckModes::compute(20.0, 5000.0, 4.0).expect("modes");
    assert!((modes.frequencies[0] - 4.0).abs() / 4.0 < 5.0e-3, "f1 = {}", modes.frequencies[0]);
    assert!((modes.frequencies[1] - 16.0).abs() / 16.0 < 0.02 && (modes.frequencies[2] - 36.0).abs() / 36.0 < 0.03);
    let train = rail_dynamics::Train::hslm(En, 1);
    assert_eq!(train.axles().len(), 2 * (train.coaches + 1));
    let resonant = train.resonance_speeds(modes.frequencies[0], 1)[0].1;
    let on = modes.peak_acceleration(&train, resonant, 0.01);
    let off = modes.peak_acceleration(&train, resonant * 0.75, 0.01);
    assert!(on > 2.0 * off, "on {on} off {off}");
    let damped = modes.peak_acceleration(&train, resonant, 0.02);
    assert!(damped / on > 0.4 && damped / on < 0.8, "doubling the damping roughly halves the resonant response: {}", damped / on);
}

#[test]
fn a_fast_rail_bridge_reports_the_dynamic_flag_and_a_resonance_estimate_with_remedy() {
    use semio_s_artifact_norm_en1990_model::BridgeSls;
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let mut rail = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RailBridge, En);
    rail.members[0].span = 20.0;
    rail.bridge_sls = vec![BridgeSls { id: "BS".into(), member_id: "beam-B1".into(), deck_acceleration: 0.0, deck_acceleration_limit: 0.0, deck_twist: 0.0, deck_twist_limit: 0.0, bridge_deflection: 0.0, bridge_deflection_limit: 0.0, track_gauge: 0.0, line_speed: 250.0, ballast_depth: 0.35, deck_mass: 5000.0, natural_frequency: 4.0, end_rotation: 0.0, basic_wind_speed: 0.0, damping_ratio: 0.005 }];
    let report = evaluate(&rail);
    let flag = report.checks.iter().find(|c| c.id == "en1990.a2.rail-dynamic.beam-B1.BS").expect("flag");
    assert!(flag.explanation.en.contains("dynamic analysis required") && flag.explanation.de.contains("dynamische Berechnung erforderlich"));
    let resonance = report.checks.iter().find(|c| c.id == "en1990.a2.rail-resonance.beam-B1.BS").expect("resonance");
    assert!(resonance.utilization > 0.0 && resonance.explanation.en.contains("HSLM"));
    assert!(resonance.status != CheckStatus::Fail || !resonance.remedies.is_empty());
    assert!(!resonance.ndps.is_empty());
}

#[test]
fn the_modal_superposition_matches_the_scipy_oracle() {
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let modes = rail_dynamics::DeckModes::compute(20.0, 5000.0, 4.0).expect("modes");
    let train = rail_dynamics::Train::hslm(En, 1);
    let resonant = 4.0 * train.coach_length;
    assert!((resonant - 72.0).abs() < 1e-12);
    for (speed, zeta, reference) in [(resonant, 0.01, 74.9358808371768), (resonant, 0.02, 49.1885636247798), (0.75 * resonant, 0.01, 3.8894305414716124)] {
        let computed = modes.peak_acceleration(&train, speed, zeta);
        assert!((computed - reference).abs() / reference < 0.06, "v {speed} zeta {zeta}: computed {computed} oracle {reference}");
    }
    let (lower, upper) = rail_dynamics::frequency_limits(En, 40.0).expect("limits");
    assert!((lower - 2.6553653626415885).abs() < 1e-9 && (upper - 6.001836751392443).abs() < 1e-9);
}

#[test]
fn every_emitted_check_family_is_traced_or_an_infrastructure_family() {
    use semio_s_plugin_norm_compliance::spec_trace::Realisation;
    let invalid = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let mut dangling = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    dangling.effects.push(semio_s_artifact_norm_en1990_model::MemberEffect { member_id: "ghost".into(), action_id: "ghost".into(), influence: 1.0 });
    let fragments: Vec<&str> = semio_s_artifact_norm_en1990_checks::spec_trace::TABLE.items.iter().filter_map(|item| match &item.realisation { Realisation::Check(f) => Some(f.iter().copied()), _ => None }).flatten().collect();
    let infrastructure = ["en1990.input.", "en1990.integrity.", "en1990.actions."];
    let untraced: Vec<String> = [En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), invalid, dangling, En1990Snapshot { members: Vec::new(), ..En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De) }]
        .iter()
        .flat_map(|document| evaluate(document).checks.into_iter().map(|check| check.id))
        .filter(|id| !fragments.iter().any(|f| id.contains(f)) && !infrastructure.iter().any(|p| id.starts_with(p)))
        .collect();
    assert!(untraced.is_empty(), "check families without a spec-trace row: {untraced:?}");
}

#[semio_framework_async_macros::async_test]
async fn the_reliability_class_is_an_enum_and_k_fi_follows_it_from_the_registry() {
    use semio_s_artifact_norm_en1990_model::ReliabilityClass;
    let applied = |class: ReliabilityClass| {
        let report = evaluate(&En1990Snapshot { reliability_class: class, ..En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De) });
        report.checks.iter().find(|c| c.id == "en1990.annex-b.kfi-applied").expect("kfi applied").computed.value
    };
    for (class, k_fi) in [(ReliabilityClass::RC1, 0.9), (ReliabilityClass::RC2, 1.0), (ReliabilityClass::RC3, 1.1)] {
        assert_eq!(ReliabilityClass::from_number(class.number()), Some(class));
        assert!((k_fi_for_rc(class.number()).expect("registry row") - k_fi).abs() < 1e-12, "Table B3 K_FI of {class:?}");
        assert!((applied(class) / applied(ReliabilityClass::RC2) - k_fi).abs() < 1e-9, "the applied gamma scales with K_FI of {class:?}");
    }
    assert_eq!(ReliabilityClass::from_number(0), None);
    assert!(evaluate(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)).checks.iter().all(|c| c.id != "en1990.annex-b.kfi"), "no declared K_FI is stored or checked any more");
}

#[semio_framework_async_macros::async_test]
async fn the_traffic_group_table_drives_exclusive_groups_rows_and_group_combinations() {
    use semio_s_artifact_norm_en1990_model::{LoadGroup, TrafficGroup};
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    let mut doc = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge, En);
    doc.traffic_groups = vec![
        TrafficGroup { id: "TG-ts".into(), group: LoadGroup::GR1A, variable_id: "TS".into() },
        TrafficGroup { id: "TG-sa".into(), group: LoadGroup::GR1B, variable_id: "SA".into() },
    ];
    let report = evaluate(&doc);
    assert_eq!(status_of(&report, "en1990.a2.traffic-groups"), CheckStatus::Pass, "the table replaces the free group text");
    let row = report.checks.iter().find(|c| c.id == "en1990.a2.traffic-group.TG-ts").expect("row check");
    assert_eq!(row.status, CheckStatus::UnverifiedBasis, "every registry coefficient is unverified");
    assert!((row.utilization - 1.0).abs() < 1e-12);
    let total = |id: &str| report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing {id}")).computed.value;
    let (a, b) = (total("en1990.a2.traffic-group-combination.gr1a.beam-B1"), total("en1990.a2.traffic-group-combination.gr1b.beam-B1"));
    assert!((a - b - 1.35 * (60_000.0 - 50_000.0)).abs() < 1e-6, "hand value: the groups differ by gamma_Q (Q_TS - Q_SA), got {a} and {b}");
    assert!(report.checks.iter().find(|c| c.id == "en1990.a2.traffic-group-governing.beam-B1").expect("governing").explanation.en.starts_with("gr1a"));
    doc.traffic_groups[1].group = LoadGroup::GR1A;
    let mixed = evaluate(&doc);
    assert_eq!(status_of(&mixed, "en1990.a2.traffic-group.TG-sa"), CheckStatus::Warning, "single axle belongs to gr1b by its category");
    assert_eq!(status_of(&mixed, "en1990.a2.traffic-groups"), CheckStatus::Warning);
    doc.traffic_groups[1].variable_id = "missing".into();
    assert_eq!(status_of(&evaluate(&doc), "en1990.a2.traffic-group.TG-sa"), CheckStatus::Fail, "a dangling component fails with the variable ids as remedy");
}

#[semio_framework_async_macros::async_test]
async fn the_unset_accompanying_lm1_cells_of_gr2_and_gr5_report_an_awaiting_value_visibly() {
    use semio_s_artifact_norm_en1990_model::{LoadGroup, TrafficGroup};
    use semio_s_plugin_norm_compliance::AnnexChoice::En;
    assert!(semio_s_artifact_norm_en1990_checks::ndp_awaits_value(En, "ndp.en1990.a2.group.gr2.road_traffic") && semio_s_artifact_norm_en1990_checks::ndp_awaits_value(En, "ndp.en1990.a2.group.gr5.road_traffic"));
    assert!(!semio_s_artifact_norm_en1990_checks::ndp_awaits_value(En, "ndp.en1990.a2.group.gr1a.road_traffic") && !semio_s_artifact_norm_en1990_checks::ndp_awaits_value(En, "ndp.en1990.a2.group.gr9.road_traffic"), "a filled cell and an unknown id are not awaiting");
    assert_eq!(semio_s_plugin_norm_compliance::ndp_number_opt("ndp.en1990.a2.group.gr2.road_traffic", En), None, "an Unset row is no number");
    for group in [LoadGroup::GR2, LoadGroup::GR5] {
        let mut doc = bridge_document(semio_s_artifact_norm_en1990_model::StructureKind::RoadBridge, En);
        doc.traffic_groups = vec![TrafficGroup { id: "TG-lm1".into(), group, variable_id: "TS".into() }];
        let row = evaluate(&doc).checks.into_iter().find(|c| c.id == "en1990.a2.traffic-group.TG-lm1").expect("row check");
        assert_eq!(row.status, CheckStatus::UnverifiedBasis, "{group:?}: the missing cell is a visible unverified basis, never a silent skip");
        assert!(row.explanation.en.starts_with("Awaiting value") && row.explanation.de.starts_with("Wert ausstehend"), "{group:?}: {:?}", row.explanation);
        assert!(row.ndps.iter().any(|id| id.ends_with(&format!("{}.road_traffic", group.as_str()))));
    }
}

fn unassessable_ids(doc: &En1990Snapshot) -> Vec<String> {
    evaluate(doc).checks.iter().filter(|c| c.id.ends_with(".unassessable")).map(|c| c.id.clone()).collect()
}

#[semio_framework_async_macros::async_test]
async fn only_a_member_without_a_wired_effect_is_unassessable() {
    let doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    assert!(unassessable_ids(&doc).is_empty(), "every member of the example is wired");
    let mut lonely = doc.clone();
    let mut extra = lonely.members[0].clone();
    extra.id = "lonely".into();
    lonely.members.push(extra);
    assert_eq!(unassessable_ids(&lonely), ["en1990.integrity.member.lonely.unassessable"], "a member no effect names is unassessable, the others are not");
    let mut unwired = doc.clone();
    let id = unwired.members[0].id.clone();
    for effect in unwired.effects.iter_mut().filter(|effect| effect.member_id == id) {
        effect.action_id = format!("{}-x", effect.action_id);
    }
    assert_eq!(unassessable_ids(&unwired), [format!("en1990.integrity.member.{id}.unassessable")], "effects that name an unknown action wire nothing");
}

#[semio_framework_async_macros::async_test]
async fn a_member_row_unit_of_a_thousand_wired_members_stays_within_the_commit_budget() {
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let template = doc.members[0].clone();
    let wiring: Vec<MemberEffect> = doc.effects.iter().filter(|effect| effect.member_id == template.id).cloned().collect();
    for index in 0..1000 {
        let mut member = template.clone();
        member.id = format!("scaled-{index}");
        doc.effects.extend(wiring.iter().map(|effect| MemberEffect { member_id: member.id.clone(), ..effect.clone() }));
        doc.members.push(member);
    }
    let first = doc.members.len() - 1000;
    let mut times: Vec<f64> = (0..9)
        .map(|round| {
            let started = std::time::Instant::now();
            let report = evaluate_scoped(&doc, semio_s_plugin_norm_compliance::incremental_evaluation::Scope::Row("members", first + round * 100));
            assert!(!report.checks.is_empty());
            started.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    times.sort_by(f64::total_cmp);
    let (p50, p95, budget) = (times[4], times[8], if cfg!(debug_assertions) { 8000.0 } else { 400.0 });
    eprintln!("[latency] en1990-member-row-unit-1000-members/commit p50_ms={p50:.1} p95_ms={p95:.1} budget_ms={budget:.1}");
    assert!(p95 <= budget, "a member row unit took {p95:.1} ms against {budget:.1} ms");
}

#[semio_framework_async_macros::async_test]
async fn a_shared_row_gate_answers_every_row_like_a_gate_computed_per_row() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::{Gate, Scope};
    let example = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let mut invalid = example.clone();
    invalid.consequence_class = 9;
    let mut unwired = example.clone();
    let id = unwired.members[0].id.clone();
    for effect in unwired.effects.iter_mut().filter(|effect| effect.member_id == id) {
        effect.action_id = format!("{}-x", effect.action_id);
    }
    let mut empty = example.clone();
    empty.members.clear();
    for doc in [example, invalid, unwired, empty] {
        let gate = Gate::new();
        let rows = [("members", doc.members.len()), ("reliabilityAnalyses", doc.reliability_analyses.len()), ("testSeries", doc.test_series.len()), ("cases", doc.cases.len())];
        for (tag, count) in rows {
            for index in 0..count {
                assert_eq!(evaluate_gated(&doc, Scope::Row(tag, index), &gate), evaluate_scoped(&doc, Scope::Row(tag, index)), "{tag}[{index}]");
            }
        }
        for scope in [Scope::All, Scope::Globals] {
            assert_eq!(evaluate_gated(&doc, scope, &gate), evaluate_scoped(&doc, scope), "{scope:?} never reads the gate of the rows");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn the_row_gate_reads_nothing_the_member_units_do_not_declare() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::{audit_reads, with_every_collection, CheckUnit, CostTier, UnitId, UnitSource};
    use semio_s_plugin_norm_compliance::{CheckResult, ClauseId, LocalizedCopy, SubjectRef};
    let example = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let reads = En1990Units::units(&example).into_iter().find(|unit| unit.id.0.starts_with("members:")).expect("a member unit").reads;
    let marker = |id: String| CheckResult::assess(id, "EN 1990", ClauseId::new("EN 1990", "§2", "gate"), SubjectRef::whole(LocalizedCopy::new("Gate", "Tor")), LocalizedCopy::new("Gate", "Tor")).build();
    let probe = CheckUnit::new(UnitId::new("gate", "rows"), Vec::new(), reads, CostTier::T0, move |document: &En1990Snapshot| {
        let gate = RowGate::of(document);
        std::iter::once(marker(format!("gate.admitted.{}", gate.admitted()))).chain(gate.broken().iter().map(|member| marker(format!("gate.broken.{member}")))).collect()
    });
    for document in with_every_collection(vec![example]) {
        assert_eq!(audit_reads(&probe, &document), Vec::<String>::new(), "the gate of the member units depends on something their read set leaves out");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_thousand_member_rows_share_one_gate_evaluation_within_the_cold_budget() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::{Gate, Scope};
    let mut doc = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let template = doc.members[0].clone();
    let wiring: Vec<MemberEffect> = doc.effects.iter().filter(|effect| effect.member_id == template.id).cloned().collect();
    for index in 0..1000 {
        let mut member = template.clone();
        member.id = format!("scaled-{index}");
        doc.effects.extend(wiring.iter().map(|effect| MemberEffect { member_id: member.id.clone(), ..effect.clone() }));
        doc.members.push(member);
    }
    let gate = Gate::new();
    let started = std::time::Instant::now();
    for _ in 0..1000 {
        assert!(evaluate_gated(&doc, Scope::Row("reliabilityAnalyses", usize::MAX), &gate).checks.is_empty());
    }
    let total = started.elapsed().as_secs_f64() * 1000.0;
    let budget = if cfg!(debug_assertions) { 5000.0 } else { 100.0 };
    eprintln!("[latency] en1990-gate-1000-rows-1000-members/cold p50_ms={total:.1} p95_ms={total:.1} budget_ms={budget:.1}");
    assert!(total <= budget, "1000 row units took {total:.1} ms against {budget:.1} ms, the gate must be evaluated once");
}

/// 🩹 The fatigue remedies write the numeric fields of the governing bin (FS-1 carries the largest damage share), never the cycle count with a stress; applying either clears the damage sum.
#[semio_framework_async_macros::async_test]
async fn fatigue_remedies_target_the_governing_bin_and_clear_the_damage_sum() {
    use semio_s_plugin_norm_app_surface::apply_remedy_edit;
    let doc = analysed();
    let report = evaluate(&doc);
    let damage = report.checks.iter().find(|c| c.id == "en1990.fat.damage.beam-B1").expect("damage check");
    let paths: Vec<&str> = damage.remedies.iter().map(|r| r.target.path.as_str()).collect();
    assert_eq!(paths, ["fatigueSpectra[id=FS-1].detailCategory", "fatigueSpectra[id=FS-1].cycles"]);
    for index in 0..damage.remedies.len() {
        let mut tree = semio_framework_value::ToValue::to_value(&doc);
        apply_remedy_edit(&report, &damage.id, index, 0, &mut tree).expect("apply the fatigue remedy");
        let fixed: En1990Snapshot = semio_framework_value::FromValue::from_value(tree).expect("the document decodes after the remedy");
        let again = evaluate(&fixed);
        let after = again.checks.iter().find(|c| c.id == damage.id).expect("damage check after the remedy");
        assert!(after.status != CheckStatus::Fail && after.computed.value <= 1.0 + 1e-6, "remedy {index}: D = {}", after.computed.value);
    }
}

/// 🩹 The Annex D standard evaluation never writes into the text field `pairs`: its typed remedy lowers the numeric `required`, the second remedy is an advisory.
#[semio_framework_async_macros::async_test]
async fn test_standard_remedy_targets_the_numeric_required_value() {
    use semio_s_plugin_norm_app_surface::apply_remedy_edit;
    let mut doc = analysed();
    doc.test_series[0].pairs = "10:9;11:10.5;9.5:9.8;12:11".into();
    doc.test_series[0].required = 1.0e6;
    doc.test_series[0].theory_mean = 30.0;
    doc.test_series[0].theory_variation = 0.1;
    doc.test_series[0].conversion_factor = 1.0;
    let report = evaluate(&doc);
    let check = report.checks.iter().find(|c| c.id == "en1990.annex-d.TS-1.standard").expect("standard evaluation check");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.remedies.iter().all(|r| !r.target.path.ends_with(".pairs")), "{:?}", check.remedies.iter().map(|r| &r.target.path).collect::<Vec<_>>());
    let typed = check.remedies.iter().position(|r| r.applicable).expect("a typed remedy");
    assert!(check.remedies[typed].target.path.ends_with(".required"));
    let mut tree = semio_framework_value::ToValue::to_value(&doc);
    apply_remedy_edit(&report, &check.id, typed, 0, &mut tree).expect("apply the remedy");
    let fixed: En1990Snapshot = semio_framework_value::FromValue::from_value(tree).expect("decode");
    assert_eq!(fixed.test_series[0].pairs, doc.test_series[0].pairs);
    let again = evaluate(&fixed);
    assert_ne!(again.checks.iter().find(|c| c.id == check.id).expect("check after").status, CheckStatus::Fail);
}

/// 🩹 The spectrum solvers return the smallest category and the largest cycle count of the governing bin that give D ≤ 1, and nothing when the other bins alone exceed 1.
#[test]
fn spectrum_solvers_hold_their_bounds() {
    let bins = [(80.0e6, 2.0e6, 71.0e6), (40.0e6, 2.0e6, 71.0e6), (20.0e6, 1.0e7, 71.0e6)];
    assert_eq!(governing_spectrum_bin(&bins), Some(0));
    let category = required_detail_category(&bins, 0).expect("the category is reachable");
    let with_category = |c: f64| spectrum_damage(&[(bins[0].0, bins[0].1, c), bins[1], bins[2]]).expect("damage");
    assert!(with_category(category) <= 1.0 && with_category(category * 0.999) > 1.0, "{category}");
    let cycles = allowed_spectrum_cycles(&bins, 0).expect("the cycle count is reachable");
    let with_cycles = |n: f64| spectrum_damage(&[(bins[0].0, n, bins[0].2), bins[1], bins[2]]).expect("damage");
    assert!(with_cycles(cycles) <= 1.0 && with_cycles(cycles * 1.001) > 1.0, "{cycles}");
    let crowded = [(80.0e6, 0.86e6, 71.0e6); 3];
    assert!(spectrum_damage(&crowded).expect("damage") > 1.0);
    assert!(required_detail_category(&crowded, 0).is_none() && allowed_spectrum_cycles(&crowded, 0).is_none());
}
