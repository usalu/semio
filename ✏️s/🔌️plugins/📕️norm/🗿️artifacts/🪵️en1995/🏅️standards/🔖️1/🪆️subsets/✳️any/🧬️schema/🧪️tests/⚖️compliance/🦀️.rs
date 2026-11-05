use semio_s_artifact_norm_en1995_schema::checks::timber::*;
use semio_framework_test_fixtures::fixture_path;
use semio_s_plugin_norm_app_surface::{apply_remedy_edit, get_value_at_path, parse_path};
use semio_s_plugin_norm_compliance::{CheckReport, CheckStatus};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;
use semio_s_artifact_norm_en1995_model::{En1995Snapshot, LayerOrientation as LO};
use semio_s_plugin_norm_compliance::{drain_ndp_reads, AnnexChoice, ReliabilityClass};

fn report_of(snap: &En1995Snapshot) -> CheckReport {
    evaluate_structure(&snap)
}

fn bundled_examples() -> Vec<(String, En1995Snapshot)> {
    semio_s_artifact_norm_en1995_examples::examples()
        .into_iter()
        .map(|example| {
            let snap = semio_s_artifact_norm_en1995_io::text::snapshot::decode_en1995_dsl(&example.document()).unwrap_or_else(|e| panic!("{}: {e}", example.id()));
            (example.id().to_string(), snap)
        })
        .collect()
}

fn check<'a>(report: &'a CheckReport, id: &str) -> &'a semio_s_plugin_norm_compliance::CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id} in {:?}", report.checks.iter().map(|c| c.id.clone()).collect::<Vec<_>>()))
}

fn apply_remedy(snap: &En1995Snapshot, report: &CheckReport, check_id: &str, remedy_index: usize) -> En1995Snapshot {
    let mut tree = semio_framework_value::ToValue::to_value(snap);
    apply_remedy_edit(report, check_id, remedy_index, 0, &mut tree).unwrap_or_else(|f| panic!("{check_id}[{remedy_index}]: {f:?}"));
    semio_framework_value::FromValue::from_value(tree).unwrap_or_else(|e| panic!("{check_id}[{remedy_index}] decode: {e}"))
}

#[semio_framework_async_macros::async_test]
async fn k_mod_sc1_permanent() {
    assert!((k_mod(ServiceClass::Sc1, LoadDuration::Permanent) - 0.6).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn k_def_depends_on_service_class_only() {
    use semio_s_artifact_norm_en1995_model::TimberProduct::Solid;
    assert!((k_def(ServiceClass::Sc1, Solid) - 0.60).abs() < 1e-9);
    assert!((k_def(ServiceClass::Sc2, Solid) - 0.80).abs() < 1e-9);
    assert!((k_def(ServiceClass::Sc3, Solid) - 2.00).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn strength_class_c24_and_gl28h_tables() {
    let c24 = properties_for_class("C24").unwrap();
    assert!((c24.f_m_k / 1e6 - 24.0).abs() < 1e-9);
    assert!((c24.f_v_k / 1e6 - 4.0).abs() < 1e-9);
    let gl = properties_for_class("GL28h").unwrap();
    assert!((gl.f_m_k / 1e6 - 28.0).abs() < 1e-9);
    assert_eq!(gl.product, semio_s_artifact_norm_en1995_model::TimberProduct::Glulam);
}

#[semio_framework_async_macros::async_test]
async fn k_cr_diverges_between_en_and_de_for_c24() {
    let f_v = 4.0e6;
    use semio_s_artifact_norm_en1995_model::TimberProduct::Solid;
    let k_cr_en = AnnexParams::en().k_cr(Solid, f_v);
    let k_cr_de = AnnexParams::de().k_cr(Solid, f_v);
    assert!((k_cr_en - 0.67).abs() < 1e-9);
    assert!((k_cr_de - 0.625).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn shear_utilization_diverges_between_annexes_for_c24() {
    let mut m = En1995Snapshot::compliant_building_beam().members.remove(0);
    m.strength_class = semio_s_artifact_norm_en1995_model::StrengthClass::C24;
    m.b_m = 0.20;
    m.h_m = 0.30;
    m.actions = vec![semio_s_artifact_norm_en1995_model::CharacteristicAction {
        id: "g".into(),
        kind: semio_s_artifact_norm_en1995_model::ActionKind::Permanent,
        category: semio_s_artifact_norm_en1995_model::ImposedCategory::NotImposed,
        load_duration: semio_s_artifact_norm_en1995_model::LoadDurationClass::Medium,
        q_line_n_per_m: 0.0,
        f_point_n: 0.0,
        m_k_nm: 1.0,
        v_k_n: 15_000.0,
        n_k_n: 0.0,
        n_t_k_n: 0.0,
        f_c90_k_n: 0.0,
    }];
    let en = evaluate_structure(&En1995Snapshot { members: vec![m.clone()], ..En1995Snapshot::empty(AnnexChoice::En) });
    let de = evaluate_structure(&En1995Snapshot { members: vec![m], ..En1995Snapshot::empty(AnnexChoice::De) });
    let shear_en = en.checks.iter().find(|c| c.id.contains("shear")).unwrap();
    let shear_de = de.checks.iter().find(|c| c.id.contains("shear")).unwrap();
    assert!((shear_en.utilization - shear_de.utilization).abs() > 1e-3);
    assert!(shear_de.utilization > shear_en.utilization);
    let k_cr_ratio = AnnexParams::en().k_cr(semio_s_artifact_norm_en1995_model::TimberProduct::Solid, 4.0e6) / AnnexParams::de().k_cr(semio_s_artifact_norm_en1995_model::TimberProduct::Solid, 4.0e6);
    assert!((shear_de.utilization / shear_en.utilization - k_cr_ratio).abs() < 1e-6, "shear utilization must scale with 1/k_cr: de={} en={} ratio={}", shear_de.utilization, shear_en.utilization, k_cr_ratio);
}

/// 🪜️ EN 1990 A1.3.1 on a two-span timber member 4.5 m + 4.5 m (g 1.5, q 2.5 kN/m): the governing ULS moment of `ContinuousTwoSpan` is the support moment, where the permanent load of both spans takes γG,sup, and equals the hogging of the shared load arrangement envelope (14 617.97 N·m, PyNite and anastruct); the span moment with γG,inf on the neighbouring span (10 376.73 N·m) stays below it, so the closed forms of `action_internals` need no span split.
#[test]
fn the_two_span_support_moment_is_the_hogging_of_the_shared_arrangement_envelope() {
    use semio_s_artifact_norm_en1995_model::{ActionKind, CharacteristicAction, ImposedCategory, LoadDurationClass, SupportType};
    let action = |id: &str, kind: ActionKind, category: ImposedCategory, duration: LoadDurationClass, q: f64| CharacteristicAction { id: id.into(), kind, category, load_duration: duration, q_line_n_per_m: q, f_point_n: 0.0, m_k_nm: 0.0, v_k_n: 0.0, n_k_n: 0.0, n_t_k_n: 0.0, f_c90_k_n: 0.0 };
    let mut member = En1995Snapshot::compliant_building_beam().members.remove(0);
    member.support = SupportType::ContinuousTwoSpan;
    member.span_m = 4.5;
    member.actions = vec![action("g", ActionKind::Permanent, ImposedCategory::NotImposed, LoadDurationClass::Permanent, 1500.0), action("q", ActionKind::Imposed, ImposedCategory::B, LoadDurationClass::Medium, 2500.0)];
    let params = AnnexParams::de();
    let combos = enumerate_combos(&member, &params);
    let m_ed = governing_uls(&combos).expect("uls").m_ed_nm.abs();
    let factors = semio_s_plugin_norm_mechanics::SpanFactors { gamma_g: params.factors.gamma_g_sup(), gamma_g_inf: params.factors.gamma_g_inf(), gamma_q: params.factors.gamma_q_building() };
    let beam = semio_s_plugin_norm_mechanics::SpanBeam { spans: &[4.5, 4.5], e: 11.0e9, area: 0.03, inertia: 2.25e-4 };
    let envelope = semio_s_plugin_norm_mechanics::continuous_beam_envelope(&beam, 1500.0, 2500.0, &factors).expect("envelope");
    assert!((envelope.hogging - 14_617.97).abs() < 1e-3 * 14_617.97, "{}", envelope.hogging);
    assert!((m_ed - envelope.hogging).abs() < 1e-3 * envelope.hogging, "{m_ed} {}", envelope.hogging);
    assert!(envelope.sagging.iter().all(|sagging| (sagging - 10_376.73).abs() < 1e-3 * 10_376.73 && *sagging < m_ed), "{:?}", envelope.sagging);
}

#[semio_framework_async_macros::async_test]
async fn column_buckling_kc_from_lambda_rel() {
    let kc = kc_factor(1.2, 0.2);
    assert!(kc < 1.0 && kc > 0.3);
}

#[semio_framework_async_macros::async_test]
async fn johansen_mode_capacity_positive() {
    let fh = embedment_fh("bolt", "timber", 350.0, 12.0, 0.0, "softwood", 0.0);
    let my = yield_moment("bolt", 400.0, 12.0, 0.0);
    let f = modes_tt_single(200.0, 200.0, 12.0, fh, fh, my, 0.0, 0.25).iter().map(|m| m.1).fold(f64::INFINITY, f64::min);
    assert!(f > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn yield_moment_is_si_for_an_m12_bolt() {
    let expected_nmm = 0.3 * 400.0 * 12.0_f64.powf(2.6);
    assert!((yield_moment("bolt", 400.0, 12.0, 0.0) - expected_nmm).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn fire_def_r60() {
    let beta_n = AnnexParams::en().beta_n(semio_s_artifact_norm_en1995_model::TimberProduct::Glulam) * 1e3 * 60.0;
    let d = d_ef_mm(beta_n * 60.0, 60.0);
    assert!((d - 49.0).abs() < 0.5);
}

#[semio_framework_async_macros::async_test]
async fn compliant_examples_pass() {
    for snap in [En1995Snapshot::compliant_building_beam(), En1995Snapshot::compliant_bridge()] {
        let report = report_of(&snap);
        assert!(report.failing().next().is_none(), "compliant example must not fail: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
        assert_eq!(report.complies(), report.unverified().next().is_none(), "complies only without an unverified basis");
    }
}

#[semio_framework_async_macros::async_test]
async fn noncompliant_example_has_failures_with_remedies() {
    let report = report_of(&En1995Snapshot::noncompliant_building());
    assert!(!report.complies());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 3);
    for f in fails {
        assert!(!f.remedies.is_empty(), "fail {} missing remedies", f.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn remedy_increasing_height_flips_bending() {
    let snap = En1995Snapshot::noncompliant_building();
    let report = report_of(&snap);
    let id = "en1995.6.1.6.bending.beam-B2";
    let bending = check(&report, id);
    assert_eq!(bending.status, CheckStatus::Fail);
    let index = bending.remedies.iter().position(|r| r.target.path == "members[id=beam-B2].hM" && r.applicable).expect("height remedy");
    let after = report_of(&apply_remedy(&snap, &report, id, index));
    assert!(check(&after, id).utilization <= 1.0, "u={}", check(&after, id).utilization);
}

#[semio_framework_async_macros::async_test]
async fn remedy_spacing_or_compression_flips_fail() {
    let snap = En1995Snapshot::noncompliant_building();
    let report = report_of(&snap);
    for id in ["en1995.8.spacing.a1.conn-C2", "en1995.6.3.2.compression.col-C1"] {
        let failing = check(&report, id);
        assert_eq!(failing.status, CheckStatus::Fail, "{id}");
        let index = failing.remedies.iter().position(|r| r.applicable).unwrap_or_else(|| panic!("{id} has no applicable remedy"));
        let after = report_of(&apply_remedy(&snap, &report, id, index));
        let flipped = check(&after, id);
        assert_ne!(flipped.status, CheckStatus::Fail, "{id} still fails with u={}", flipped.utilization);
    }
    let spacing = check(&report, "en1995.8.spacing.a1.conn-C2");
    assert_eq!(spacing.remedies[0].target.path, "connections[id=conn-C2].spacingM");
    assert!((spacing.remedies[0].required.value - spacing.computed.value).abs() < 1e-3 + 1e-9, "a1 remedy must land on the tabulated minimum rounded up to a millimetre");
}

#[semio_framework_async_macros::async_test]
async fn every_emitted_path_resolves_on_examples() {
    for (name, snap) in bundled_examples() {
        let root = semio_framework_value::ToValue::to_value(&snap);
        for annex in [AnnexChoice::En, AnnexChoice::De] {
            let report = evaluate_structure(&En1995Snapshot { annex, ..snap.clone() });
            assert!(!report.checks.is_empty(), "{name}");
            for c in &report.checks {
                let paths = std::iter::once(&c.subject.path).chain(c.remedies.iter().map(|r| &r.target.path));
                for path in paths.filter(|p| !p.is_empty()) {
                    assert!(path.starts_with("members[id=") || path.starts_with("connections[id="), "{name} {}: '{path}' must address an entity by id", c.id);
                    parse_path(path).unwrap_or_else(|e| panic!("{name} {}: parse '{path}': {e}", c.id));
                    get_value_at_path(&root, path).unwrap_or_else(|e| panic!("{name} {}: resolve '{path}': {e}", c.id));
                }
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn every_applicable_remedy_alone_flips_its_fail_on_examples() {
    let mut applied = 0usize;
    for (name, snap) in bundled_examples() {
        let report = report_of(&snap);
        for c in report.failing() {
            assert!(!c.remedies.is_empty(), "{name} {} has no remedy", c.id);
            for (index, remedy) in c.remedies.iter().enumerate().filter(|(_, r)| r.applicable) {
                let after = report_of(&apply_remedy(&snap, &report, &c.id, index));
                let updated = check(&after, &c.id);
                assert_ne!(updated.status, CheckStatus::Fail, "{name} {} remedy[{index}] '{}' left u={}", c.id, remedy.target.path, updated.utilization);
                applied += 1;
            }
        }
    }
    assert!(applied >= 8, "expected at least eight applicable remedies across the examples, got {applied}");
}

#[semio_framework_async_macros::async_test]
async fn annex_changes_results_on_every_example_with_timber_resistance() {
    for (name, snap) in bundled_examples() {
        let en = evaluate_structure(&En1995Snapshot { annex: AnnexChoice::En, ..snap.clone() });
        let de = evaluate_structure(&En1995Snapshot { annex: AnnexChoice::De, ..snap.clone() });
        let differs = en.checks.iter().zip(&de.checks).any(|(a, b)| a.id == b.id && (a.utilization - b.utilization).abs() > 1e-9);
        assert!(differs, "{name}: EN and DE must diverge on at least one check");
    }
}

fn leaf_paths(prefix: &str, value: &semio_framework_value::DslValue, out: &mut Vec<String>) {
    match value {
        semio_framework_value::DslValue::Object(fields) => {
            for (key, child) in fields {
                let path = if prefix.is_empty() { key.clone() } else { format!("{prefix}.{key}") };
                leaf_paths(&path, child, out);
            }
        }
        semio_framework_value::DslValue::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                leaf_paths(&format!("{prefix}[{index}]"), child, out);
            }
        }
        _ => out.push(prefix.to_string()),
    }
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_every_editable_leaf_has_en_de_meta() {
    let mut leaves = Vec::new();
    for snap in [En1995Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), En1995Snapshot::noncompliant_building(), En1995Snapshot::compliant_bridge()] {
        leaf_paths("", &semio_framework_value::ToValue::to_value(&snap), &mut leaves);
    }
    assert!(leaves.len() > 60, "expected every member, action and connection leaf, got {}", leaves.len());
    for path in &leaves {
        let meta = semio_s_artifact_norm_en1995_editor::field_meta::en1995_field_meta(path).unwrap_or_else(|| panic!("{path} has no field meta"));
        assert!(!meta.label.en.trim().is_empty() && !meta.label.de.trim().is_empty(), "{path}");
        assert_ne!(meta.label.en, meta.label.de, "{path} must carry a German label distinct from English");
        let parent = &path[..path.rfind(['.', '[']).unwrap_or(0)];
        if !parent.is_empty() {
            let parent_meta = semio_s_artifact_norm_en1995_editor::field_meta::en1995_field_meta(parent);
            assert!(parent_meta.map_or(true, |p| p.label.en != meta.label.en), "{path} resolves only to its parent's meta '{}'", meta.label.en);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn steel_plate_changes_capacity_and_can_make_the_check_fail() {
    let mut c = En1995Snapshot::compliant_building_beam().connections.remove(0);
    c.t1_m = 0.04;
    c.t2_m = 0.04;
    let annex = AnnexChoice::De;
    let run = |c: &semio_s_artifact_norm_en1995_model::TimberConnection| evaluate_structure(&En1995Snapshot { connections: vec![c.clone()], ..En1995Snapshot::empty(annex) });
    let timber = run(&c);
    let timber_check = check(&timber, "en1995.8.2.2.johansen.conn-C1");
    let scale = 0.99 / timber_check.utilization;
    for action in &mut c.actions {
        action.f_k_n *= scale;
    }
    let timber = run(&c);
    let timber_check = check(&timber, "en1995.8.2.2.johansen.conn-C1");
    assert!((timber_check.utilization - 0.99).abs() < 1e-9 && timber_check.status == CheckStatus::UnverifiedBasis && timber_check.ndps.iter().any(|id| id.starts_with("ndp.en1995.kmod.")));
    let mut thick = c.clone();
    thick.steel_plate = true;
    thick.steel_plate_thickness_m = c.diameter_m;
    let thick_report = run(&thick);
    assert!(thick_report.checks.iter().all(|x| !x.id.starts_with("en1995.8.2.2.")), "steel plate must switch to clause 8.2.3");
    let thick_check = check(&thick_report, "en1995.8.2.3.johansen.conn-C1");
    assert!(thick_check.limit.value > timber_check.limit.value * 1.05, "thick plate F_v,Rd {} must exceed timber-timber {}", thick_check.limit.value, timber_check.limit.value);
    assert_eq!(thick_check.status, CheckStatus::UnverifiedBasis);
    let mut thin = c.clone();
    thin.steel_plate = true;
    thin.steel_plate_thickness_m = 0.0;
    let thin_report = run(&thin);
    let thin_check = check(&thin_report, "en1995.8.2.3.johansen.conn-C1");
    assert!((thin_check.limit.value - timber_check.limit.value).abs() > 1e-6, "thin plate must change F_v,Rd");
    assert_eq!(thin_check.status, CheckStatus::Fail, "thin plate F_v,Rd {} must fall below F_Ed {}", thin_check.limit.value, thin_check.computed.value);
    assert!(!thin_check.remedies.is_empty());
}

fn family_any_dir() -> PathBuf {
    fixture_path!("../../../../..")
}

fn oracle_script() -> PathBuf {
    family_any_dir().join("🔮️oracles").join("⚖️compliance").join("🐍️.py")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema").join("📸️snapshot").join("🔣️.json")
}

fn python_oracle(input: &str, label: &str) -> serde_json::Value {
    let mut child = semio_s_plugin_norm_compliance::python_oracle::command()
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
    serde_json::from_slice(&output.stdout).expect("oracle json")
}

fn connection_variants() -> Vec<(String, En1995Snapshot)> {
    [("thin-plate", true, 0.0, 1), ("interpolated-plate", true, 0.75, 1), ("thick-plate", true, 1.0, 1), ("central-plate", true, 0.5, 2), ("double-shear", false, 0.0, 2)]
        .into_iter()
        .map(|(name, steel_plate, plate_ratio, shear_planes)| {
            let mut snap = En1995Snapshot::compliant_building_beam();
            for c in &mut snap.connections {
                c.steel_plate = steel_plate;
                c.steel_plate_thickness_m = plate_ratio * c.diameter_m;
                c.shear_planes = shear_planes;
            }
            (name.to_string(), snap)
        })
        .collect()
}

#[test]
fn python_oracle_matches_rust_utilizations_within_half_percent() {
    for (label, base) in bundled_examples().into_iter().chain(connection_variants()) {
        for annex in [AnnexChoice::En, AnnexChoice::De] {
            let snap = En1995Snapshot { annex, ..base.clone() };
            let rust = report_of(&snap);
            let py = python_oracle(&serde_json::to_string(&snap).expect("json"), &label);
            let py_checks = py.get("checks").and_then(|c| c.as_array()).expect("checks array");
            let py_ids: BTreeSet<&str> = py_checks.iter().filter_map(|c| c.get("id").and_then(|v| v.as_str())).collect();
            let rust_ids: BTreeSet<&str> = rust.checks.iter().map(|c| c.id.as_str()).collect();
            assert_eq!(py_ids, rust_ids, "{label}/{annex:?}: python and rust must emit the same check ids");
            for pc in py_checks {
                let id = pc.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let pu = pc.get("utilization").and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("{label} {id}: python utilization missing"));
                let rc = check(&rust, id);
                let rel = (pu - rc.utilization).abs() / pu.abs().max(rc.utilization.abs()).max(1e-9);
                assert!(rel <= 0.005, "{label}/{annex:?} check {id}: python={pu} rust={} rel={rel}", rc.utilization);
            }
        }
    }
}

#[test]
fn example_snapshots_validate_against_json_schema() {
    let schema_path = snapshot_schema_path();
    for (name, snap) in bundled_examples() {
        let instance = serde_json::to_string(&snap).expect("instance");
        let tmp = std::env::temp_dir().join(format!("en1995-instance-{name}.json"));
        std::fs::write(&tmp, &instance).unwrap();
        let status = semio_s_plugin_norm_compliance::python_oracle::command()
            .arg("-c")
            .arg("import json,sys,jsonschema; s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2])); jsonschema.validate(instance=i, schema=s); print('ok')")
            .arg(&schema_path)
            .arg(&tmp)
            .output()
            .expect("python jsonschema");
        let _ = std::fs::remove_file(&tmp);
        assert!(status.status.success(), "{name} jsonschema: {}\n{}", String::from_utf8_lossy(&status.stdout), String::from_utf8_lossy(&status.stderr));
    }
}

fn check_signature(report: &CheckReport) -> Vec<(String, String, i64, i64, i64)> {
    let mut sig: Vec<_> = report
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
    sig.sort_by(|a, b| a.0.cmp(&b.0));
    sig
}

fn walk_json_leaves(prefix: &str, value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let path = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                match v {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => walk_json_leaves(&path, v, out),
                    _ => out.push(path),
                }
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                let path = format!("{prefix}[{i}]");
                match v {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => walk_json_leaves(&path, v, out),
                    _ => out.push(path),
                }
            }
        }
        _ => {}
    }
}

fn json_at<'a>(tree: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut cur = tree;
    let mut buf = String::new();
    let mut chars = path.chars().peekable();
    let mut segs = Vec::new();
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
    for seg in segs {
        if let Ok(i) = seg.parse::<usize>() {
            cur = cur.as_array()?.get(i)?;
        } else {
            cur = cur.as_object()?.get(&seg)?;
        }
    }
    Some(cur)
}

fn json_at_mut<'a>(tree: &'a mut serde_json::Value, path: &str) -> Option<&'a mut serde_json::Value> {
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
        return None;
    }
    let mut cur = tree;
    let last = segs.len() - 1;
    for (i, seg) in segs.iter().enumerate() {
        if i == last {
            if let Ok(idx) = seg.parse::<usize>() {
                return cur.as_array_mut()?.get_mut(idx);
            }
            return cur.as_object_mut()?.get_mut(seg);
        }
        if let Ok(idx) = seg.parse::<usize>() {
            cur = cur.as_array_mut()?.get_mut(idx)?;
        } else {
            cur = cur.as_object_mut()?.get_mut(seg)?;
        }
    }
    None
}

fn leaf_name(path: &str) -> &str {
    path.rsplit(['.', ']']).next().unwrap_or(path).trim_start_matches('[')
}

fn entity_prefix<'a>(path: &'a str, key: &str) -> Option<&'a str> {
    let start = path.find(key)?;
    let after = &path[start..];
    let end = after.find(']')? + start + 1;
    Some(&path[start..end])
}

fn scope_skips(tree: &serde_json::Value, path: &str) -> bool {
    let leaf = leaf_name(path);
    if matches!(leaf, "id" | "name" | "title" | "label" | "labelEn" | "labelDe") {
        return true;
    }
    if let Some(mem) = entity_prefix(path, "members[") {
        if let Some(role) = json_at(tree, &format!("{mem}.role")).and_then(|v| v.as_str()) {
            if role != "bridge" && leaf.starts_with("bridge") {
                return true;
            }
        }
    }
    if leaf == "steelPlateThicknessM" {
        if let Some(conn) = entity_prefix(path, "connections[") {
            if json_at(tree, &format!("{conn}.steelPlate")).and_then(|v| v.as_bool()) != Some(true) {
                return true;
            }
        }
    }
    false
}

fn perturb_json(tree: &mut serde_json::Value, path: &str) -> bool {
    let Some(v) = json_at_mut(tree, path) else {
        return false;
    };
    match v {
        serde_json::Value::Number(n) => {
            let f = n.as_f64().unwrap_or(0.0);
            *v = serde_json::json!(if f.abs() < 1e-15 { 1.0 } else { f * 1.15 });
            true
        }
        serde_json::Value::Bool(b) => {
            *v = serde_json::json!(!*b);
            true
        }
        serde_json::Value::String(s) => {
            let leaf = leaf_name(path);
            let ns = if s == "en" || s == "En" {
                "de".into()
            } else if s == "de" || s == "De" {
                "en".into()
            } else if leaf == "role" {
                match s.as_str() {
                    "floor" => "beam".into(),
                    "beam" => "column".into(),
                    "column" => "floor".into(),
                    "bridge" => "beam".into(),
                    _ => format!("{s}-x"),
                }
            } else if leaf == "support" {
                match s.as_str() {
                    "simplySupported" => "cantilever".into(),
                    "cantilever" => "continuousTwoSpan".into(),
                    _ => "simplySupported".into(),
                }
            } else if leaf == "strengthClass" {
                if s.starts_with("gl") { "c24".into() } else { "gl28h".into() }
            } else if leaf == "loadDuration" {
                match s.as_str() {
                    "permanent" => "medium".into(),
                    "medium" => "short".into(),
                    "short" => "long".into(),
                    "long" => "instantaneous".into(),
                    _ => "permanent".into(),
                }
            } else if leaf == "kind" {
                match s.as_str() {
                    "permanent" => "imposed".into(),
                    "imposed" => "snow".into(),
                    "snow" => "wind".into(),
                    _ => "permanent".into(),
                }
            } else if leaf == "fastenerType" {
                match s.as_str() {
                    "bolt" => "dowel".into(),
                    "dowel" => "screw".into(),
                    "screw" => "nail".into(),
                    _ => "bolt".into(),
                }
            } else if leaf == "category" {
                match s.as_str() {
                    "a" => "c".into(),
                    "c" => "e".into(),
                    "snowHigh" => "notImposed".into(),
                    _ => "a".into(),
                }
            } else if leaf == "ulsEquation" {
                if s == "eq610" { "eq610ab".into() } else { "eq610".into() }
            } else {
                format!("{s}-x")
            };
            *v = serde_json::json!(ns);
            true
        }
        _ => false,
    }
}

#[test]
fn perturb_every_editable_leaf_in_committed_examples_changes_a_check() {
    for (name, snap) in bundled_examples() {
        let base_sig = check_signature(&report_of(&snap));
        let value = serde_json::to_value(&snap).expect("json");
        let mut paths = Vec::new();
        walk_json_leaves("", &value, &mut paths);
        let mut unchanged = Vec::new();
        for path in &paths {
            if scope_skips(&value, path) {
                continue;
            }
            let mut tree = value.clone();
            if !perturb_json(&mut tree, path) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1995Snapshot>(tree) else {
                continue;
            };
            let sig = check_signature(&report_of(&perturbed));
            if sig == base_sig {
                unchanged.push(path.clone());
            }
        }
        assert!(unchanged.is_empty(), "{name}: editable leaves without check influence: {unchanged:?}");
    }
}

#[test]
fn duplicate_member_and_connection_ids_fail_with_one_of() {
    let mut snap = En1995Snapshot::compliant_building_beam();
    let dup_m = snap.members[0].clone();
    snap.members.push(dup_m);
    let report = report_of(&snap);
    let fail = report
        .checks
        .iter()
        .find(|c| c.id.contains("integrity.duplicate.members") && c.status == CheckStatus::Fail)
        .expect("duplicate member Fail");
    assert!(!fail.remedies.is_empty());
    assert!(fail.remedies.iter().any(|r| !r.options.is_empty() || r.applicable));

    let mut snap = En1995Snapshot::compliant_building_beam();
    let dup_c = snap.connections[0].clone();
    snap.connections.push(dup_c);
    let report = report_of(&snap);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.duplicate.connections") && c.status == CheckStatus::Fail));
}

#[test]
fn reference_table_k_mod_cell_equals_evaluated_modification_factor() {
    use semio_s_plugin_norm_app_surface::CatalogueCell;
    use semio_s_artifact_norm_en1995_editor::panels::catalogue::reference_tables;
    use semio_ui_locale::Locale;
    let snap = En1995Snapshot::compliant_building_beam();
    let km_table = reference_tables(Locale::En).into_iter().find(|t| t.id == "k-mod").expect("k-mod table");
    let sc1 = km_table.rows.iter().find(|r| r.id == "sc1").expect("sc1");
    let CatalogueCell::Number { value: expected, .. } = sc1.cells[3] else {
        panic!("k-mod sc1 medium cell must be numeric");
    };
    let member = &snap.members[0];
    let combos = enumerate_combos(member, &AnnexParams::de());
    let uls = governing_uls(&combos).expect("uls");
    let km = k_mod(parse_service_class(member.service_class), uls.duration);
    assert!((km - expected).abs() < 1e-9, "evaluate k_mod={km} vs table cell={expected}");
}

#[test]
fn lateral_restraint_spacing_changes_bending_utilization() {
    let mut snap = En1995Snapshot::compliant_building_beam();
    snap.members[0].lateral_restraint_spacing_m = snap.members[0].span_m;
    let loose = check(&report_of(&snap), "en1995.6.1.6.bending.beam-B1").utilization;
    snap.members[0].lateral_restraint_spacing_m = snap.members[0].span_m * 0.25;
    let tight = check(&report_of(&snap), "en1995.6.1.6.bending.beam-B1").utilization;
    assert!(tight < loose - 1e-6, "closer lateral restraints must raise M_crit and lower bending utilization ({tight} vs {loose})");
}

#[test]
fn connection_rows_change_johansen_capacity() {
    let mut snap = En1995Snapshot::compliant_building_beam();
    snap.connections[0].number = 4;
    snap.connections[0].rows = 1;
    let one_row = check(&report_of(&snap), "en1995.8.2.2.johansen.conn-C1").limit.value;
    snap.connections[0].rows = 2;
    let two_rows = check(&report_of(&snap), "en1995.8.2.2.johansen.conn-C1").limit.value;
    assert!((two_rows - one_row).abs() > 1e-6, "rows must change F_v,Rd ({one_row} vs {two_rows})");
}

#[test]
fn snow_high_category_reads_its_own_table_a1_1_row() {
    use semio_s_artifact_norm_en1995_model::{ActionKind as K, ImposedCategory as C};
    for annex in [AnnexParams::de(), AnnexParams::en()] {
        let factors = annex.factors;
        assert_eq!(psi_factors(&factors, K::Snow, C::SnowHigh), (0.7, 0.5, 0.2));
        assert_eq!(psi_factors(&factors, K::Snow, C::NotImposed), (0.5, 0.2, 0.0));
        assert_eq!(psi_factors(&factors, K::Imposed, C::SnowHigh), psi_factors(&factors, K::Imposed, C::NotImposed));
        assert_eq!(psi_factors(&factors, K::Permanent, C::SnowHigh), (1.0, 1.0, 1.0));
    }
    let mut snap = En1995Snapshot::compliant_building_beam();
    let snow = semio_s_artifact_norm_en1995_model::CharacteristicAction { id: "s".into(), kind: K::Snow, category: C::NotImposed, load_duration: semio_s_artifact_norm_en1995_model::LoadDurationClass::Short, q_line_n_per_m: 1500.0, ..snap.members[0].actions[0].clone() };
    snap.members[0].actions.push(snow);
    let wfreq = |snap: &En1995Snapshot| evaluate_structure(snap).checks.iter().find(|c| c.id.contains("7.2.wfreq")).expect("SLS frequent deflection check").utilization;
    let low = wfreq(&snap);
    snap.members[0].actions.last_mut().expect("snow").category = C::SnowHigh;
    let high = wfreq(&snap);
    assert!(high > low, "snow above 1000 m a.s.l. carries psi1 0.5 instead of 0.2 ({low} vs {high})");
}

#[test]
fn sls_frequent_uses_psi1_and_de_snow_high_diverges() {
    let factors = AnnexParams::de().factors;
    use semio_s_artifact_norm_en1995_model::{ActionKind as K, ImposedCategory as C};
    let (_, psi1_a, _) = psi_factors(&factors, K::Imposed, C::A);
    let (_, psi1_e, _) = psi_factors(&factors, K::Imposed, C::E);
    let (_, psi1_snow, _) = psi_factors(&factors, K::Snow, C::NotImposed);
    let (_, psi1_snow_high, _) = psi_factors(&factors, K::Snow, C::SnowHigh);
    assert!((psi1_a - 0.5).abs() < 1e-12);
    assert!((psi1_e - 0.9).abs() < 1e-12);
    assert!((psi1_snow - 0.2).abs() < 1e-12);
    assert!((psi1_snow_high - 0.5).abs() < 1e-12, "DE NA snow >1000 m ψ₁ must diverge from EN snow ψ₁");

    let mut snap = En1995Snapshot::compliant_building_beam();
    let combos = enumerate_combos(&snap.members[0], &AnnexParams::de());
    assert!(combos.iter().any(|c| c.kind == ComboKind::SlsFrequent));
    assert!(combos.iter().any(|c| c.kind == ComboKind::SlsCharacteristic));
    assert!(combos.iter().any(|c| c.kind == ComboKind::SlsQuasiPermanent));
    assert!(combos.iter().any(|c| c.kind == ComboKind::Uls));
    let freq = combos
        .iter()
        .filter(|c| c.kind == ComboKind::SlsFrequent)
        .max_by(|a, b| a.q_ed_line.abs().partial_cmp(&b.q_ed_line.abs()).unwrap_or(std::cmp::Ordering::Equal))
        .expect("SLS frequent combo");
    let char = combos
        .iter()
        .filter(|c| c.kind == ComboKind::SlsCharacteristic)
        .max_by(|a, b| a.q_ed_line.abs().partial_cmp(&b.q_ed_line.abs()).unwrap_or(std::cmp::Ordering::Equal))
        .expect("SLS characteristic combo");
    let qp = combos.iter().find(|c| c.kind == ComboKind::SlsQuasiPermanent).expect("SLS QP combo");
    assert!(
        freq.q_ed_line.abs() + 1e-9 < char.q_ed_line.abs(),
        "frequent (ψ₁) line load must be below characteristic (q_freq={}, q_char={})",
        freq.q_ed_line,
        char.q_ed_line
    );
    assert!((freq.q_ed_line - qp.q_ed_line).abs() > 1e-6, "frequent must differ from quasi-permanent");

    let report = evaluate_structure(&En1995Snapshot { annex: AnnexChoice::De, ..snap.clone() });
    let wfreq = report.checks.iter().find(|c| c.id.contains("7.2.wfreq")).expect("SLS frequent deflection check");
    assert!(wfreq.computed.value > 0.0);
    let id = wfreq.id.clone();
    let util_a = wfreq.utilization;

    snap.members[0].actions.iter_mut().find(|a| a.kind == semio_s_artifact_norm_en1995_model::ActionKind::Imposed).expect("imposed").category = semio_s_artifact_norm_en1995_model::ImposedCategory::E;
    let util_e = check(&evaluate_structure(&En1995Snapshot { annex: AnnexChoice::De, ..snap.clone() }), &id).utilization;
    assert!((util_e - util_a).abs() > 1e-9, "ψ₁ category A→E must move frequent utilization");

}


fn catalogue_fixture() -> serde_json::Value {
    let path = family_any_dir().join("🧪️tests").join("🧫️fixtures").join("📇️catalogue").join("🔣️.json");
    serde_json::from_str(&std::fs::read_to_string(path).expect("catalogue fixture")).expect("catalogue json")
}

fn case_of(entry: &serde_json::Value) -> Case {
    let c = &entry["case"];
    let annex = if c["annex"].as_str() == Some("De") { AnnexChoice::De } else { AnnexChoice::En };
    let mut case = Case::new("fixture", annex);
    for (k, v) in c["num"].as_object().unwrap() {
        case = case.with_n(k, v.as_f64().unwrap());
    }
    for (k, v) in c["text"].as_object().unwrap() {
        case = case.with_s(k, v.as_str().unwrap());
    }
    case
}

fn oracle_catalogue(input: &serde_json::Value) -> serde_json::Value {
    oracle_mode("--catalogue", input)
}

fn oracle_mode(flag: &str, input: &serde_json::Value) -> serde_json::Value {
    use std::io::Write;
    let mut child = semio_s_plugin_norm_compliance::python_oracle::command()
        .arg(oracle_script())
        .arg(flag)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("oracle spawn failed: {e}"));
    child.stdin.as_mut().unwrap().write_all(input.to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "catalogue oracle stderr={}", String::from_utf8_lossy(&output.stderr));
    serde_json::from_slice(&output.stdout).expect("oracle json")
}

#[test]
fn catalogue_matches_independent_python_oracle_within_one_percent() {
    let fixture = catalogue_fixture();
    let oracle = oracle_catalogue(&fixture);
    let rows = fixture["cases"].as_array().unwrap();
    let expected = oracle["cases"].as_array().unwrap();
    assert_eq!(rows.len(), expected.len());
    let mut compared = 0;
    for (row, exp) in rows.iter().zip(expected) {
        let id = row["entry"].as_str().unwrap();
        let outcomes = evaluate_outcomes(id, &case_of(row)).unwrap_or_else(|e| panic!("{id}: {e}"));
        for (key, value) in exp["values"].as_object().unwrap() {
            let o = outcomes.iter().find(|o| &o.key == key).unwrap_or_else(|| panic!("{id}: outcome {key} missing"));
            let rust = if o.mode == Mode::Info { o.capacity } else { o.utilization() };
            let py = value.as_f64().unwrap();
            let rel = (rust - py).abs() / py.abs().max(rust.abs()).max(1e-12);
            assert!(rel <= 0.01, "{id}/{key}: rust={rust} python={py} rel={rel}");
            compared += 1;
        }
    }
    assert!(compared >= 30, "only {compared} values compared");
}

#[test]
fn the_stabilising_load_and_force_of_t_95_follow_en_1995_1_1_expressions_9_35_and_9_37() {
    let value = |case: Case, key: &str| evaluate_outcomes("T-95", &case).expect("T-95").iter().find(|o| o.key == key).map(|o| o.capacity).expect("outcome");
    for annex in [AnnexChoice::En, AnnexChoice::De] {
        let glulam = Case::new("t95", annex).with_n("l_m", 6.0).with_n("n_braced", 4.0).with_n("n_d", 5_000.0).with_s("product", "glulam");
        assert!((value(glulam.clone(), "q_d") - 4.0 * 5_000.0 / (30.0 * 6.0)).abs() < 1e-9);
        assert!((value(glulam, "f_d") - 5_000.0 / 80.0).abs() < 1e-9, "F_d belongs to one member, not to the sum");
        let solid = Case::new("t95", annex).with_n("l_m", 20.0).with_n("n_braced", 4.0).with_n("n_d", 5_000.0).with_s("product", "solid");
        assert!((value(solid.clone(), "q_d") - (15.0_f64 / 20.0).sqrt() * 4.0 * 5_000.0 / (30.0 * 20.0)).abs() < 1e-9);
        assert!((value(solid.clone(), "f_d") - 5_000.0 / 50.0).abs() < 1e-9);
        let single = Case::new("t95", annex).with_n("l_m", 20.0).with_n("n_braced", 1.0).with_n("n_d", 20_000.0).with_s("product", "solid");
        assert!((value(single, "q_d") - value(solid.clone(), "q_d")).abs() < 1e-9, "n N_d decides q_d");
        assert!(evaluate_outcomes("T-95", &Case::new("t95", annex).with_n("l_m", 20.0).with_n("n_d", 5_000.0)).is_err(), "the number of braced members is a required input");
    }
}

#[test]
fn the_size_factor_of_solid_timber_follows_the_density_limit_of_en_1995_1_1_3_2_3_and_not_the_wood_type() {
    let kh = |class: &str| evaluate_outcomes("T-05", &Case::new("kh", AnnexChoice::En).with_s("class", class).with_n("h", 100.0)).expect("T-05").iter().find(|o| o.key == "k_h").map(|o| o.capacity).expect("k_h");
    assert!((kh("C24") - 1.5_f64.powf(0.2)).abs() < 1e-12);
    assert!((kh("D35") - 1.5_f64.powf(0.2)).abs() < 1e-12, "a hardwood class with a characteristic density up to 700 kg/m3 has the size factor");
    assert!((kh("D60") - 1.5_f64.powf(0.2)).abs() < 1e-12, "700 kg/m3 itself is inside the limit");
    assert_eq!(kh("D70"), 1.0, "D70 (800 kg/m3) is above the limit");
}

#[test]
fn the_size_factor_flips_at_the_registry_density_limit_of_700_kg_per_m3_without_any_flag() {
    use semio_s_artifact_norm_en1995_model::TimberProduct;
    let limit = semio_s_plugin_norm_compliance::ndp_common("ndp.en1995.kh.density-limit");
    assert_eq!(limit, 700.0);
    let inside = (150.0_f64 / 100.0).powf(0.2);
    assert!((size_factor_kh(TimberProduct::Solid, 350.0, 100.0, 0.12) - inside).abs() < 1e-12);
    assert!((size_factor_kh(TimberProduct::Solid, limit, 100.0, 0.12) - inside).abs() < 1e-12, "exactly 700 kg/m3 is inside the limit");
    assert_eq!(size_factor_kh(TimberProduct::Solid, limit + 0.1, 100.0, 0.12), 1.0, "just above 700 kg/m3 has no size factor");
    assert_eq!(size_factor_kh(TimberProduct::Solid, 800.0, 100.0, 0.12), 1.0);
    assert!((size_factor_kh(TimberProduct::Glulam, 800.0, 300.0, 0.12) - 2.0_f64.powf(0.1)).abs() < 1e-12, "the limit only concerns solid timber");
}

#[test]
fn registry_covers_every_catalogue_id() {
    let ids: BTreeSet<&str> = registry().iter().map(|e| e.id).collect();
    let expected = [
        "T-01", "T-02", "T-03", "T-04", "T-05", "T-06", "T-10", "T-11", "T-12", "T-13", "T-14", "T-15", "T-16", "T-17", "T-18", "T-20", "T-21", "T-22", "T-23", "T-24", "T-30", "T-31", "T-32", "T-33", "T-45", "T-46", "T-50", "T-60", "T-61", "T-62", "T-63", "T-64", "T-65", "T-66", "T-70", "T-71", "T-72", "T-73", "T-74", "T-75", "T-76", "T-77", "T-78", "T-79", "T-80", "T-81", "T-82", "T-83", "T-90", "T-91", "T-92", "T-93", "T-94", "T-95", "T-96", "T-97", "T-98", "T-100", "T-101", "T-102", "T-103", "T-104", "T-105", "T-120", "T-121", "T-122", "T-123", "T-124",
    ];
    for id in expected {
        assert!(ids.contains(id), "catalogue entry {id} missing");
    }
}

#[test]
fn every_entry_reports_missing_inputs_instead_of_panicking() {
    for e in registry() {
        let results = evaluate_case(e.id, &Case::new("empty", AnnexChoice::De));
        assert_eq!(results.len(), 1, "{}", e.id);
        assert_eq!(results[0].status, CheckStatus::NotApplicable, "{}", e.id);
    }
}

#[test]
fn spec_example_e2_column_buckling_kc_is_0_28457() {
    let case = Case::new("e2", AnnexChoice::De).with_s("class", "C24").with_s("duration", "medium").with_n("sc", 2.0).with_n("b", 100.0).with_n("h", 100.0).with_n("lef_y", 3000.0).with_n("lef_z", 3000.0).with_n("n_c", 36_770.0);
    let out = evaluate_outcomes("T-20", &case).unwrap();
    assert!((out[0].utilization() - 1.0).abs() < 2e-3, "N_Rd must be 36.77 kN, got {}", out[0].utilization());
    assert!((kc_factor(1.7622, 0.2) - 0.28457).abs() < 1e-4);
}

#[test]
fn spec_example_e6_johansen_mode_c_governs_for_m12_in_c24() {
    let case = Case::new("e6", AnnexChoice::En).with_s("class", "C24").with_s("duration", "medium").with_s("kind", "bolt").with_n("d", 12.0).with_n("t1", 60.0).with_n("t2", 60.0).with_n("f_u", 400.0).with_n("sc", 1.0);
    let out = evaluate_outcomes("T-71", &case).unwrap();
    let rk = out.iter().find(|o| o.key == "f_v_rk").unwrap().capacity;
    let rd = out.iter().find(|o| o.key == "f_v_rd").unwrap().capacity;
    assert!((rk - 7532.0).abs() < 2.0 && (rd - 4635.0).abs() < 2.0, "rk={rk} rd={rd}");
}

#[test]
fn johansen_mode_c_uses_t2_over_t1_for_unequal_thicknesses() {
    let fh = 25.0;
    let thin_main = modes_tt_single(40.0, 100.0, 12.0, fh, fh, 76_745.0, 0.0, 0.25);
    let c = thin_main.iter().find(|m| m.0 == "c").unwrap().1;
    let r = 100.0_f64 / 40.0;
    let expected = fh * 40.0 * 12.0 / 2.0 * ((1.0 + 2.0 * (1.0 + r + r * r) + r * r).sqrt() - (1.0 + r));
    assert!((c - expected).abs() < 1e-6, "mode c {c} vs {expected}");
}

#[test]
fn nail_end_distance_at_zero_degrees_is_fifteen_diameters() {
    let table = spacing_table("nail", 4.0, 0.0, 350.0);
    let a3t = table.iter().find(|r| r.0 == "a3t").unwrap().1;
    assert!((a3t - 60.0).abs() < 1e-9);
}

#[test]
fn gamma_m_is_1_3_in_the_german_annex_and_per_product_in_en_and_k_cr_is_one_for_lvl() {
    use semio_s_artifact_norm_en1995_model::TimberProduct::{Glulam, Lvl, Solid};
    let de = AnnexParams::de();
    assert!((de.gamma_m(Solid) - 1.3).abs() < 1e-12 && (de.gamma_m(Glulam) - 1.3).abs() < 1e-12 && (de.gamma_m(Lvl) - 1.3).abs() < 1e-12);
    let en = AnnexParams::en();
    assert!((en.gamma_m(Glulam) - 1.25).abs() < 1e-12 && (en.gamma_m(Lvl) - 1.2).abs() < 1e-12);
    assert!((de.k_cr(Lvl, 4.5e6) - 1.0).abs() < 1e-12);
}

#[test]
fn final_deflection_applies_psi2_to_the_variable_part_only() {
    let case = Case::new("w", AnnexChoice::De).with_s("class", "C24").with_n("sc", 1.0).with_n("l", 4000.0).with_n("b", 100.0).with_n("h", 200.0).with_n("load_g", 1.0).with_n("load_q", 1.0).with_n("psi_2", 0.3);
    let inst = evaluate_outcomes("T-60", &case).unwrap();
    let wg = inst[0].capacity * 1e3;
    let wq = inst[1].demand * 1e3;
    let fin = evaluate_outcomes("T-61", &case).unwrap();
    let expected = wg * 1.6 + wq * (1.0 + 0.3 * 0.6);
    assert!((fin[0].demand * 1e3 - expected).abs() < 1e-9);
}

#[test]
fn unverified_strength_rows_are_flagged_and_pinned_fixtures_are_exact() {
    assert!(strength_row("C24").unwrap().verified);
    for name in ["C16", "C30", "GL24H", "GL28H", "GL32H"] {
        assert!(!strength_row(name).unwrap().verified, "{name} disagrees between the spec and the library and awaits the print");
    }
    let c24 = strength_row("C24").unwrap();
    assert!((c24.f_t_0_k - 14.5).abs() < 1e-12 && (c24.f_t_90_k - 0.4).abs() < 1e-12 && (c24.e_90_mean - 370.0).abs() < 1e-12);
    let gl24 = strength_row("GL24h").unwrap();
    assert!((gl24.f_t_0_k - 19.2).abs() < 1e-12 && (gl24.f_c_90_k - 2.5).abs() < 1e-12 && (gl24.g_mean - 650.0).abs() < 1e-12);
    let c30 = strength_row("C30").unwrap();
    assert!((c30.f_t_0_k - 19.0).abs() < 1e-12 && (c30.f_c_0_k - 24.0).abs() < 1e-12);
    assert!((strength_row("C16").unwrap().f_t_0_k - 8.5).abs() < 1e-12);
}

#[test]
fn strength_kmod_and_kdef_tables_come_from_the_shared_registry() {
    drain_ndp_reads();
    let c24 = strength_row("C24").unwrap();
    let reads = drain_ndp_reads();
    assert_eq!(reads.len(), 12);
    assert!(reads.iter().all(|id| id.starts_with("ndp.en1995.strength.c24.")));
    assert!(c24.verified && !strength_row("C20").unwrap().verified && !strength_row("D30").unwrap().verified && !strength_row("GL20h").unwrap().verified);
    assert_eq!(k_mod_for("solid", 1, LoadDuration::Permanent), Some(0.6));
    assert_eq!(k_mod_for("osb3", 2, LoadDuration::Long), Some(0.4));
    assert_eq!(k_mod_for("osb3", 3, LoadDuration::Long), None);
    assert_eq!(k_mod_for("unknown", 1, LoadDuration::Long), None);
    assert_eq!(k_def_for("plywood-636-3", 3), Some(2.5));
    assert_eq!(k_def_for("plywood-636-1", 2), None);
    assert!(drain_ndp_reads().iter().any(|id| id.starts_with("ndp.en1995.kdef.")));
    assert!(!semio_s_plugin_norm_compliance::ndp_is_verified("ndp.en1995.kmod.solid.sc1.permanent") && !semio_s_plugin_norm_compliance::ndp_is_verified("ndp.en1995.kdef.solid.sc1"), "k_mod and k_def have no independent source yet");
}

#[test]
fn clt_gamma_method_equals_shear_analogy_for_three_layers() {
    let (ei, gamma, _) = clt_gamma(&[(30.0, true), (40.0, false), (30.0, true)], 11_000.0, 50.0, 4000.0);
    assert!(gamma[0] > 0.0 && gamma[0] < 1.0 && gamma[1] == 0.0);
    let (z, e) = (70.0_f64, 11_000.0_f64);
    let b = e * 30.0 * z * z / 2.0;
    let s = 50.0 * z * z / 40.0;
    let analogy = 2.0 * e * 30.0_f64.powi(3) / 12.0 + b / (1.0 + b * std::f64::consts::PI.powi(2) / (s * 4000.0 * 4000.0));
    assert!((ei - analogy).abs() / analogy < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn member_checks_emit_stations_along_the_length_that_follow_the_closed_forms() {
    let report = report_of(&En1995Snapshot::compliant_building_beam());
    for id in ["en1995.6.1.6.bending.beam-B1", "en1995.6.1.7.shear.beam-B1", "en1995.7.2.wfin.beam-B1"] {
        let c = check(&report, id);
        assert!(c.stations_consistent(), "{id}");
        assert_eq!(c.stations.len(), 11, "{id}");
        assert!(c.station_field.is_empty());
    }
    let bending = check(&report, "en1995.6.1.6.bending.beam-B1");
    let mid = bending.stations.iter().find(|s| (s.at[0] - 0.5).abs() < 1e-12).expect("mid");
    assert!((mid.utilization - bending.utilization).abs() < 1e-9);
    let quarter = bending.stations.iter().find(|s| (s.at[0] - 0.2).abs() < 1e-12).expect("fifth");
    assert!(quarter.utilization < mid.utilization && quarter.utilization > 0.5 * mid.utilization);
    let shear = check(&report, "en1995.6.1.7.shear.beam-B1");
    assert!((shear.stations[0].utilization - shear.utilization).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn row_shares_sum_to_one_and_peak_at_the_inverse_effective_number() {
    for (n, n_ef) in [(2, 1.7), (5, 3.2), (10, 6.0)] {
        let shares = row_shares(n, n_ef);
        assert!((shares.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        let peak = shares.iter().copied().fold(0.0, f64::max);
        assert!((peak - 1.0 / n_ef).abs() < 1e-9, "n={n}");
        assert!((shares[0] - shares[n - 1]).abs() < 1e-12);
    }
    assert!(row_shares(4, 4.0).iter().all(|w| (w - 0.25).abs() < 1e-9));
}

#[semio_framework_async_macros::async_test]
async fn fire_and_fastener_stations_have_their_own_fields() {
    for (name, snap) in bundled_examples() {
        let report = report_of(&snap);
        for c in &report.checks {
            assert!(c.stations_consistent(), "{name}: {}", c.id);
            if c.id.starts_with("en1995.1-2.4.fire.") {
                assert_eq!(c.station_field, "fire-time");
                assert!(c.stations.windows(2).all(|w| w[1].utilization >= w[0].utilization - 1e-12));
            }
            if c.id.contains(".johansen.") {
                assert_eq!(c.station_field, "fastener-force");
                assert!(c.stations.iter().all(|s| s.at.len() == 2));
            }
        }
    }
}

/// 🔮️ EN 1995 third-party oracle cases: the shared harness recomputes every blessed case in `🧪️tests/🧫️fixtures/🔮️oracle-cases` with the registered libraries and fails on any mismatch beyond the tolerance profile or between libraries.
#[test]
fn third_party_libraries_reproduce_the_blessed_oracle_cases() {
    let verified = semio_s_plugin_norm_compliance::python_oracle::verify_cases_above(env!("CARGO_MANIFEST_DIR"));
    assert!(verified >= 24, "expected at least 24 verified oracle cases, got {verified}");
}

/// 📇️ A document case is evaluated through its catalogue entry, listed under `cases[id=…]` and follows an edit of its numeric input.
#[test]
fn document_cases_are_checked_and_follow_their_inputs() {
    let mut snap = En1995Snapshot::compliant_building_beam();
    let k_ser = |snap: &En1995Snapshot| {
        let report = report_of(snap);
        let check = report.checks.iter().find(|c| c.id == "en1995.t-83.k_ser.case-kser").expect("k_ser of case-kser");
        assert_eq!(check.subject.path, "cases[id=case-kser]");
        check.limit.value
    };
    let before = k_ser(&snap);
    assert!((before - 420.0_f64.powf(1.5) * 12.0 / 23.0).abs() / before < 1e-9, "K_ser = rho_m^1.5 d / 23 (N/mm) = {before}");
    snap.cases[0].numbers[0].value = 16.0;
    assert!((k_ser(&snap) / before - 16.0 / 12.0).abs() < 1e-9, "K_ser is proportional to the dowel diameter");
}

/// 🧮️ Incomplete case inputs yield one not-applicable result naming the missing key, never a panic.
#[test]
fn a_case_without_its_inputs_reports_the_missing_key() {
    let mut snap = En1995Snapshot::compliant_building_beam();
    snap.cases[0].numbers.clear();
    let report = report_of(&snap);
    let check = report.checks.iter().find(|c| c.id.ends_with(".input.case-kser")).expect("input result");
    assert_eq!(check.status, CheckStatus::NotApplicable);
}

/// 🧩️ Equations 6.10a and 6.10b (ξ = 0.85) are selectable and never govern more than 6.10 for the building example.
#[test]
fn uls_equations_6_10_a_and_b_are_selectable() {
    let plain = En1995Snapshot::compliant_building_beam();
    let mut ab = plain.clone();
    ab.uls_equation = semio_s_artifact_norm_en1995_model::UlsEquation::Eq610ab;
    let combos = enumerate_combos(&plain.members[0], &AnnexParams::de().with_uls_equation(semio_s_artifact_norm_en1995_model::UlsEquation::Eq610ab)).into_iter().map(|c| c.id).collect::<Vec<_>>();
    assert!(combos.contains(&"uls.6.10a".to_string()) && combos.contains(&"uls.6.10b.lead.q".to_string()), "{combos:?}");
    assert!(!enumerate_combos(&plain.members[0], &AnnexParams::de()).iter().any(|c| c.id == "uls.6.10a"));
    let util = |s: &En1995Snapshot| check(&report_of(s), "en1995.6.1.6.bending.beam-B1").utilization;
    assert!(util(&ab) <= util(&plain) + 1e-12, "6.10a/b {} vs 6.10 {}", util(&ab), util(&plain));
    assert!(util(&ab) < util(&plain), "xi < 1 must lower the permanent part of the leading combination");
}

/// 🪢 M_crit is computed from the section, the span and the restraint spacing: halving the spacing doubles it and the exact 6.31 stress matches the independent closed form.
#[test]
fn critical_moment_is_computed_from_geometry() {
    let mut member = En1995Snapshot::compliant_building_beam().members.remove(0);
    let e = 10.2e9;
    member.lateral_restraint_spacing_m = 2.0;
    let full = effective_m_crit_nm(&member, e);
    member.lateral_restraint_spacing_m = 1.0;
    assert!((effective_m_crit_nm(&member, e) / full - 2.0).abs() < 1e-9);
    let (b, h, lef) = (member.b_m, member.h_m, 1.0);
    let itor = h * b.powi(3) * (1.0 / 3.0 - 0.21 * b / h * (1.0 - b.powi(4) / (12.0 * h.powi(4))));
    let sigma = std::f64::consts::PI * (e * (h * b.powi(3) / 12.0) * (e / 16.0) * itor).sqrt() / (lef * b * h * h / 6.0);
    assert!((effective_m_crit_nm(&member, e) / (sigma * b * h * h / 6.0) - 1.0).abs() < 1e-9);
}

mod design_case_bridge {
    use super::*;
    use semio_s_artifact_norm_en1995_model::{CaseNumber, CaseRow, CaseText};
    use semio_s_plugin_norm_compliance::design_case::{declared, issues, search, seed, CaseCatalogue, DesignCase};
    use semio_s_plugin_norm_compliance::RemedyBound;

    fn bending(id: &str, m_y: f64) -> CaseRow {
        CaseRow {
            id: id.into(),
            label_en: "Girder bending".into(),
            label_de: "Trägerbiegung".into(),
            entry_id: "T-14".into(),
            numbers: [("m_y", m_y), ("b", 200.0), ("h", 400.0)].into_iter().map(|(id, value)| CaseNumber { id: id.into(), value }).collect(),
            texts: vec![CaseText { id: "class".into(), value: "GL24h".into() }],
        }
    }

    #[test]
    fn the_truss_and_bracing_detail_inputs_are_declared_and_the_demo_cases_carry_them_without_issues() {
        for (id, keys) in [("T-95", ["n_braced", "kind", "span", "height", "bays", "chord_b", "chord_h", "web_b", "web_h"].as_slice()), ("T-96", ["n_braced", "kind", "spacing", "member_b", "member_h", "brace_b", "brace_h"].as_slice())] {
            let entry = En1995Cases::entries().into_iter().find(|e| e.id == id).expect("entry");
            for key in keys {
                assert!(entry.inputs.contains(key), "{id} declares {key}");
            }
            let row = catalogue_demo_cases().into_iter().find(|c| c.entry_id == id).expect("demo case");
            let case = DesignCase::from(&row);
            assert!(issues::<En1995Cases>(&entry, &case.values).is_empty(), "{id}");
            assert!(keys.iter().filter(|k| **k != "kind").all(|k| case.values.numbers.contains_key(*k)), "{id}: every detail number has a demo value");
        }
    }

    #[test]
    fn a_duplicate_case_number_is_filed_under_its_case_and_two_cases_may_repeat_a_key() {
        let mut snap = En1995Snapshot::compliant_building_beam();
        let mut first = bending("c1", 40e6);
        first.numbers.push(CaseNumber { id: "b".into(), value: 200.0 });
        snap.cases = vec![first, bending("c2", 40e6)];
        snap.cases[1].numbers.push(CaseNumber { id: "b".into(), value: 200.0 });
        let report = report_of(&snap);
        let fails: Vec<_> = report.checks.iter().filter(|c| c.id.contains("integrity.duplicate.caseNumbers") && c.status == CheckStatus::Fail).collect();
        assert_eq!(fails.iter().map(|c| (c.id.as_str(), c.subject.entity_id.as_str())).collect::<Vec<_>>(), vec![("en1995.integrity.duplicate.caseNumbers.c1.b", "c1"), ("en1995.integrity.duplicate.caseNumbers.c2.b", "c2")]);
    }

    #[test]
    fn every_declared_input_of_every_entry_has_a_typed_vocabulary_row() {
        for entry in En1995Cases::entries() {
            assert_eq!(declared::<En1995Cases>(&entry).len(), entry.inputs.len(), "{}", entry.id);
        }
        let mut seen = BTreeSet::new();
        for row in case_inputs::VOCABULARY {
            assert!(seen.insert((row.key, row.scope)), "duplicate vocabulary row {} {}", row.key, row.scope);
            assert!(!row.en.is_empty() && !row.de.is_empty(), "{}", row.key);
            assert!(row.kind != semio_s_plugin_norm_compliance::design_case::InputKind::Choice || !row.choices.is_empty(), "{}", row.key);
        }
    }

    #[test]
    fn strength_class_choices_are_exactly_the_tabulated_classes() {
        let class = semio_s_plugin_norm_compliance::design_case::spec::<En1995Cases>("T-14", "class").expect("class row");
        let values: Vec<&str> = class.choices.iter().map(|c| c.value).collect();
        assert_eq!(values, strength_class_options());
    }

    #[test]
    fn seeded_cases_of_all_entries_have_no_input_issue() {
        for entry in En1995Cases::entries() {
            let case = seed::<En1995Cases>(&entry, "seed");
            assert!(issues::<En1995Cases>(&entry, &case.values).is_empty(), "{}", entry.id);
            assert!(evaluate_design_case(&case, AnnexChoice::De).iter().all(|r| !r.id.starts_with("case.")), "{}", entry.id);
        }
    }

    #[test]
    fn search_finds_entries_by_clause_part_and_both_languages() {
        let all = En1995Cases::entries();
        let ids = |query: &str| search(&all, query).iter().map(|e| e.id).collect::<Vec<_>>();
        assert_eq!(ids("t-14"), ["T-14"]);
        assert!(ids("6.1.6").contains(&"T-14"));
        assert!(ids("biegung").contains(&"T-14"));
        assert!(ids("bending").contains(&"T-14"));
        assert!(ids("fire").iter().any(|id| id.starts_with("T-10")));
        assert_eq!(ids("").len(), all.len());
    }

    #[test]
    fn a_failing_document_case_reports_clause_utilisation_and_a_sizing_remedy_on_its_row() {
        let mut snap = En1995Snapshot::empty(AnnexChoice::De);
        snap.cases = vec![bending("ok", 20.0e6), bending("over", 900.0e6)];
        let report = evaluate_structure(&snap);
        let over: Vec<_> = report.checks.iter().filter(|c| c.subject.entity_id == "over").collect();
        assert!(!over.is_empty());
        assert!(over.iter().all(|c| c.subject.path == "cases[id=over]" && c.clause.to_string().contains("6.1.6")));
        let worst = over.iter().max_by(|a, b| a.utilization.total_cmp(&b.utilization)).unwrap();
        assert_eq!(worst.status, CheckStatus::Fail);
        let remedy = worst.remedies.first().expect("sizing remedy");
        assert_eq!((remedy.bound, remedy.target.path.as_str()), (RemedyBound::AtLeast, "cases[id=over].numbers[id=b].value"));
        assert!((remedy.required.value / remedy.current.value - worst.utilization).abs() < 1e-9);
        assert!(report.checks.iter().filter(|c| c.subject.entity_id == "ok").all(|c| c.status != CheckStatus::Fail));
    }

    #[test]
    fn invalid_inputs_of_a_document_case_fail_with_a_remedy() {
        let mut bad = bending("bad", 20.0e6);
        bad.texts[0].value = "GL99z".into();
        bad.numbers.push(CaseNumber { id: "not_in_the_vocabulary".into(), value: 1.0 });
        let results = evaluate_case_row(AnnexChoice::De, &bad);
        let class = results.iter().find(|r| r.id.ends_with(".class")).expect("choice issue");
        assert_eq!((class.status, class.remedies[0].bound), (CheckStatus::Fail, RemedyBound::OneOf));
        assert!(class.remedies[0].options.iter().any(|o| o == "GL24h"));
        assert!(results.iter().any(|r| r.id.ends_with(".not_in_the_vocabulary") && r.status == CheckStatus::Fail));
    }

    #[test]
    fn the_neutral_case_roundtrips_the_document_rows() {
        let case: DesignCase = DesignCase::from(&bending("c", 1.0));
        assert_eq!(case.values.numbers.get("b"), Some(&200.0));
        assert_eq!(case.values.texts.get("class").map(String::as_str), Some("GL24h"));
        assert_eq!((case.label_en.as_str(), case.entry_id.as_str()), ("Girder bending", "T-14"));
    }
}

#[test]
fn independent_oracle_accepts_the_design_case_vocabulary_and_registry() {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../🔮️oracles/🗂️design-case/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&script, &[], None);
    let answer: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
    assert!(answer["entries"].as_u64().unwrap_or(0) >= 67);
    assert_eq!(answer["rows"].as_u64(), Some(semio_s_artifact_norm_en1995_schema::checks::timber::case_inputs::VOCABULARY.len() as u64));
}

#[test]
fn reduced_properties_follow_the_perimeter_over_area_of_the_residual_section() {
    let r = reduced_properties(200.0, 400.0, [1.0, 1.0, 0.0, 1.0], 0.7, 60.0);
    assert!((r.b - 116.0).abs() < 1e-9 && (r.h - 358.0).abs() < 1e-9);
    assert!((r.ratio - 832.0 / 41528.0 * 1e3).abs() < 1e-9);
    assert!((r.k_bend - (1.0 - r.ratio / 200.0)).abs() < 1e-12 && (r.k_comp - (1.0 - r.ratio / 125.0)).abs() < 1e-12 && (r.k_ten - (1.0 - r.ratio / 330.0)).abs() < 1e-12);
    let early = reduced_properties(200.0, 400.0, [1.0, 1.0, 0.0, 1.0], 0.7, 10.0);
    assert!(early.k_bend > r.k_bend && early.k_bend < 1.0, "linear in time below 20 min");
    assert_eq!(reduced_properties(100.0, 100.0, [1.0; 4], 0.7, 120.0).area, 0.0);
}

#[test]
fn clt_char_accelerates_after_fall_off_and_matches_the_hand_integration() {
    let layers = [30.0; 5];
    let (plain, _) = clt_char_depth(&layers, 0.65, 90.0, false, 2.0, 25.0);
    assert!((plain - 58.5).abs() < 1e-9);
    let (depth, times) = clt_char_depth(&layers, 0.65, 90.0, true, 2.0, 25.0);
    assert!((times[0] - 30.0 / 0.65).abs() < 1e-9, "first glue line");
    assert!((times[1] - (30.0 / 0.65 + 25.0 / 1.3 + 5.0 / 0.65)).abs() < 1e-9, "second glue line after 25 mm at twice the rate");
    assert!((depth - (60.0 + 1.3 * (90.0 - times[1]))).abs() < 1e-9, "{depth}");
    assert!(depth > plain);
    let (same, _) = clt_char_depth(&layers, 0.65, 40.0, true, 2.0, 25.0);
    assert!((same - 26.0).abs() < 1e-9, "before the first glue line fall-off changes nothing");
}

#[test]
fn bridge_helpers_reproduce_miner_and_crowd_hand_values() {
    assert!((miner_damage(12.0, 4.0, 10.0, 5e7) - 0.5).abs() < 1e-12);
    assert_eq!((crowd_psi(0.4), crowd_psi(1.2), crowd_psi(2.2)), (0.0, 1.0, 0.0));
    assert!((crowd_psi(1.9) - 0.5).abs() < 1e-12 && (crowd_psi(0.75) - 0.5).abs() < 1e-12);
    let a = crowd_acceleration(30.0, 3.0, 0.8, 1.8, 800.0, 0.03);
    let p = 0.8 * 280.0 * 10.8 * (0.03_f64 / 72.0).sqrt() * 0.75;
    assert!((a - 2.0 * p * 3.0 / (std::f64::consts::PI * 0.03 * 800.0)).abs() < 1e-12);
}

#[test]
fn fire_connection_and_deck_entries_report_the_reduced_load_time_and_staged_losses() {
    let fire = Case::new("c", AnnexChoice::En).with_n("f_v_rk", 8000.0).with_n("t_req", 30.0).with_n("f_fi_d", 3000.0).with_n("f_rd_20", 9000.0).with_n("k_fire", 0.05);
    let out = evaluate_outcomes("T-104", &fire).unwrap();
    let time = out.iter().find(|o| o.key == "connection_fire_time").unwrap();
    assert!((time.capacity - (3.0_f64).ln() / 0.05).abs() < 1e-9 && time.utilization() > 1.0);
    let deck = Case::new("d", AnnexChoice::En).with_n("sigma_p_0", 1.0).with_n("loss_relax", 0.15).with_n("loss_creep", 0.25).with_n("loss_mc", 0.1);
    let kept = 0.85 * 0.75 * 0.9;
    let out = evaluate_outcomes("T-122", &deck).unwrap();
    assert!((out[0].capacity / 1e6 - kept).abs() < 1e-9);
}

#[test]
fn annex_switch_reads_the_registry_and_changes_only_the_expected_values() {
    use semio_s_artifact_norm_en1995_model::TimberProduct;
    let (en, de) = (AnnexParams::en(), AnnexParams::de());
    assert_eq!((en.w_fin_limit_divisor(), de.w_fin_limit_divisor()), (250.0, 200.0));
    assert_eq!(en.w_inst_limit_divisor(), de.w_inst_limit_divisor());
    assert_eq!((en.gamma_m(TimberProduct::Glulam), de.gamma_m(TimberProduct::Glulam)), (1.25, 1.3));
    assert_eq!(en.gamma_m(TimberProduct::Solid), de.gamma_m(TimberProduct::Solid));
    assert_eq!(en.k_fi(TimberProduct::Solid), de.k_fi(TimberProduct::Solid));
}

#[test]
fn sectionproperties_and_pynite_agree_with_the_gamma_method_the_residual_section_and_the_dynamics() {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../🔮️oracles/📐️third-party/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&script, &[std::ffi::OsStr::new("--check")], None);
    let answer: serde_json::Value = serde_json::from_slice(&output.stdout).expect("third-party json");
    let item = |label: &str| answer["cases"].as_array().unwrap().iter().find(|c| c["label"] == label).unwrap_or_else(|| panic!("case {label}")).clone();
    let num = |c: &serde_json::Value, key: &str| c["case"]["num"][key].as_f64().unwrap_or_else(|| panic!("number {key}"));
    let val = |c: &serde_json::Value, key: &str| c["values"][key].as_f64().unwrap_or_else(|| panic!("value {key}"));
    let close = |ours: f64, theirs: f64, tolerance: f64, what: &str| assert!((ours - theirs).abs() <= tolerance * theirs.abs(), "{what}: ours {ours} library {theirs}");
    for label in ["t-beam", "box-three-parts"] {
        let c = item(label);
        let parts: Vec<Part> = (1..=3)
            .filter(|k| c["case"]["num"].get(format!("b{k}")).is_some())
            .map(|k| {
                let (b, h) = (num(&c, &format!("b{k}")), num(&c, &format!("h{k}")));
                Part { a: num(&c, &format!("a{k}")), i: b * h.powi(3) / 12.0, e: num(&c, &format!("e{k}")), h, s: if k == 2 { 1.0 } else { num(&c, &format!("s{k}")) } }
            })
            .collect();
        let g = gamma_beam(&parts, num(&c, "k"), num(&c, "l"));
        close(g.ei_ef, val(&c, "ei_ef"), 1e-9, label);
        close(g.ei_rigid, val(&c, "ei_rigid"), 1e-9, label);
        close(g.gamma[0], val(&c, "gamma_1"), 1e-12, label);
        assert!((g.a[1] - val(&c, "a_2")).abs() < 1e-9, "{label}: a_2");
    }
    let c = item("t-beam");
    close(5.0 * num(&c, "q_d") * num(&c, "l").powi(4) / (384.0 * val(&c, "ei_ef")), val(&c, "deflection"), 5e-3, "t-beam deflection");
    for label in ["clt-three-layer", "clt-five-layer"] {
        let c = item(label);
        let layers: Vec<(f64, bool)> = (1..=num(&c, "n_layers") as usize).map(|k| (num(&c, &format!("t{k}")), num(&c, &format!("o{k}")) > 0.5)).collect();
        let (rigid, _, _) = clt_gamma(&layers, num(&c, "e"), 1.0e12, num(&c, "l"));
        close(rigid, val(&c, "ei_rigid"), 1e-6, &format!("{label}: a rigid shear layer is the sectionproperties composite"));
        let (flexible, gamma, _) = clt_gamma(&layers, num(&c, "e"), num(&c, "g_r"), num(&c, "l"));
        assert!(val(&c, "ei_separate") < flexible && flexible < val(&c, "ei_rigid"), "{label}: the γ-method lies between the own second moments and the rigid composite");
        assert!(gamma.iter().all(|g| (0.0..=1.0).contains(g)), "{label}: γ_i within [0, 1]");
        let (own, _, _) = clt_gamma(&layers, num(&c, "e"), 1.0e-12, num(&c, "l"));
        close(own, val(&c, "ei_separate"), 1e-5, &format!("{label}: no shear transfer leaves the own second moments"));
    }
    for label in ["three-sided", "early"] {
        let c = item(label);
        let r = reduced_properties(num(&c, "b"), num(&c, "h"), [1.0, 1.0, 0.0, 1.0], num(&c, "beta_n"), num(&c, "t_req"));
        close(r.b, val(&c, "b_r"), 1e-12, label);
        close(r.h, val(&c, "h_r"), 1e-12, label);
        close(r.area, val(&c, "area"), 1e-9, label);
        close(r.b * r.h.powi(3) / 12.0, val(&c, "i_y"), 1e-9, label);
        close(r.ratio, val(&c, "ratio"), 1e-9, label);
        close(r.k_bend, val(&c, "k_mod_fi_bending"), 1e-9, label);
        close(r.k_comp, val(&c, "k_mod_fi_compression"), 1e-9, label);
        close(r.k_ten, val(&c, "k_mod_fi_tension"), 1e-9, label);
    }
    let c = item("strip");
    close(floor_f1(num(&c, "span_m"), num(&c, "ei_l"), num(&c, "mass_kg_m2")), val(&c, "f1"), 1e-3, "strip f1");
    let c = item("girder");
    let f1 = bridge_f1(num(&c, "span_m"), num(&c, "ei_nm2"), num(&c, "mass_kg_m"));
    close(f1, val(&c, "f1"), 1e-3, "girder f1");
    close(single_pedestrian_acceleration(f1, num(&c, "mass_kg_m") * num(&c, "span_m") / 2.0, num(&c, "zeta"), true), val(&c, "a_single"), 1e-9, "girder a");
    let c = item("floor-mode");
    let (lx, ly, ei_l, ei_b, m) = (num(&c, "span_m"), num(&c, "width_m"), num(&c, "ei_l"), num(&c, "ei_b"), num(&c, "mass_kg_m2"));
    close(floor_plate_f1(lx, ly, ei_l, ei_b, m), val(&c, "f1"), 1e-3, "orthotropic floor f1");
    close(m * lx * ly / 4.0, val(&c, "modal_mass"), 1e-3, "modal mass of the first plate mode");
    let zeta = num(&c, "zeta");
    let (v, _) = velocity_response(floor_f1(lx, ei_l, m), lx, ly, m, ei_l, ei_b);
    close(floor_plate_peak_acceleration(lx, ly, ei_l, ei_b, m), 2.0 * std::f64::consts::PI * floor_plate_f1(lx, ly, ei_l, ei_b, m) * v, 1e-12, "heat map peak");
    close(num(&c, "force_n") / (2.0 * zeta * val(&c, "modal_mass")), val(&c, "a_peak"), 1e-9, "resonant footfall acceleration from the library modal mass");
    let shape = c["values"]["phi"].as_array().expect("mode shape grid");
    for (i, row) in shape.iter().enumerate() {
        for (j, library) in row.as_array().expect("row").iter().enumerate() {
            let ours = floor_plate_mode(lx * i as f64 / (shape.len() - 1) as f64, ly * j as f64 / (row.as_array().unwrap().len() - 1) as f64, lx, ly);
            assert!((library.as_f64().unwrap() - ours).abs() < 0.01, "mode shape at node ({i}, {j})");
        }
    }
}

#[test]
fn constants_flagged_from_memory_are_registry_values_cited_by_the_checks_that_use_them() {
    let cited = |entry: &str, case: &Case| evaluate_case(entry, case).into_iter().flat_map(|r| r.ndps).collect::<BTreeSet<String>>();
    let fire = Case::new("c", AnnexChoice::En).with_s("class", "GL28h").with_n("b", 200.0).with_n("h", 400.0).with_n("t_req", 30.0).with_n("m_fi", 60e6);
    let reduced = cited("T-106", &fire);
    for id in ["bending", "compression", "tension"] {
        assert!(reduced.contains(&format!("ndp.en1995.fire.reduced-{id}-divisor")), "{id}");
    }
    assert!(reduced.contains("ndp.en1995.fire.reduced-time-limit") && reduced.contains("ndp.en1995.k-fi.glulam"));
    let clt = Case::new("c", AnnexChoice::En).with_s("class", "CLT100").with_n("n_layers", 3.0).with_n("t1", 30.0).with_n("o1", 1.0).with_n("t2", 30.0).with_n("o2", 0.0).with_n("t3", 30.0).with_n("o3", 1.0).with_n("t_req", 60.0).with_n("m_fi", 1e6);
    let clt_cited = cited("T-107", &clt);
    assert!(clt_cited.contains("ndp.en1995.fire.clt-fall-off-factor") && clt_cited.contains("ndp.en1995.fire.clt-fall-off-protect"));
    let crowd = Case::new("c", AnnexChoice::En).with_s("crowd_model", "setra").with_n("span_m", 24.0).with_n("ei_nm2", 3.0e8).with_n("mass_kg_m", 450.0).with_n("crowd_density", 0.8).with_n("deck_width_m", 3.0);
    let crowd_cited = cited("T-120", &crowd);
    for id in ["crowd-force", "crowd-factor", "crowd-psi-start", "crowd-psi-full-from", "crowd-psi-full-to", "crowd-psi-end"] {
        assert!(crowd_cited.contains(&format!("ndp.en1995.bridge.{id}")), "{id}");
    }
    let column = Case::new("c", AnnexChoice::En).with_s("class", "C24").with_s("joint", "bolted").with_n("lef", 3000.0).with_n("i_tot", 40.0).with_n("l1", 500.0).with_n("i1", 12.0).with_n("n_parts", 3.0).with_n("n_c", 1e4).with_n("area", 30000.0);
    assert!(cited("T-93", &column).contains("ndp.en1995.annex-c.eta.bolted"));
    let connection = Case::new("c", AnnexChoice::En).with_n("f_v_rk", 8000.0).with_n("t_req", 30.0).with_n("f_fi_d", 3000.0).with_n("f_rd_20", 9000.0);
    assert!(cited("T-104", &connection).contains("ndp.en1995.fire.connection-reduced-load-k"));
    assert!(evaluate_outcomes("T-104", &connection).unwrap().iter().all(|o| o.key != "connection_fire_time"), "the open constant is never defaulted");
    for id in ["ndp.en1995.fire.reduced-bending-divisor", "ndp.en1995.bridge.crowd-force", "ndp.en1995.annex-c.eta.nailed", "ndp.en1995.fire.clt-fall-off-factor"] {
        assert!(!semio_s_plugin_norm_compliance::ndp_is_verified(id), "{id} stays unverified until a human checks the print");
    }
}

fn entity_document() -> En1995Snapshot {
    entity_demo_document()
}

#[test]
fn typed_entities_are_checked_through_the_case_bridge_and_re_homed_onto_their_fields() {
    let d = entity_document();
    let report = evaluate_structure(&d);
    for (entity, collection) in [("taper-1", "tapers"), ("taper-2", "tapers"), ("cutout-1", "cutouts"), ("cutout-2", "cutouts"), ("fastener-1", "fasteners"), ("panel-1", "panels"), ("floor-1", "floors")] {
        let own: Vec<_> = report.checks.iter().filter(|c| c.subject.entity_id == entity).collect();
        assert!(!own.is_empty(), "{entity} has checks");
        assert!(own.iter().all(|c| c.subject.path.starts_with(&format!("{collection}[id={entity}]")) || c.subject.path.starts_with("members[") || c.subject.path.starts_with("connections[")), "{entity}: {:?}", own.iter().map(|c| c.subject.path.clone()).collect::<Vec<_>>());
    }
    assert!(report.checks.iter().all(|c| !c.subject.path.contains(":T-") && c.remedies.iter().all(|r| !r.target.path.contains(":T-"))), "no check addresses a synthetic case row");
    assert_eq!(entity_ids(&d, "floors"), ["floor-1"]);
    assert!(report.checks.iter().any(|c| c.id.starts_with("en1995.7.2.floor-creep.")), "the creep check exists");
    let scoped: usize = ENTITY_TABLES.iter().map(|table| (0..entity_ids(&d, table).len()).map(|i| evaluate_scoped(&d, semio_s_plugin_norm_compliance::incremental_evaluation::Scope::Row(table, i)).checks.len()).sum::<usize>()).sum();
    assert!(scoped > 0 && scoped <= report.checks.len(), "the entity rows are separately evaluable");
}

#[test]
fn a_remedy_on_a_re_homed_taper_input_names_the_entity_field() {
    let mut d = entity_document();
    d.tapers[0].lamella_thickness_m = 0.0;
    let report = evaluate_structure(&d);
    let paths: Vec<String> = report.checks.iter().filter(|c| c.subject.entity_id == "taper-1").flat_map(|c| c.remedies.iter().map(|r| r.target.path.clone())).collect();
    assert!(paths.iter().all(|p| p.starts_with("tapers[id=taper-1]") || p.starts_with("members[id=beam-B1]")), "{paths:?}");
}

#[test]
fn entities_with_missing_references_fail_with_a_one_of_remedy() {
    let mut d = entity_document();
    d.tapers[0].member_id = "ghost".into();
    d.fasteners[0].connection_id = "ghost".into();
    d.floors[0].member_id = "ghost".into();
    let report = evaluate_structure(&d);
    for id in ["en1995.integrity.dangling.tapers.memberId.taper-1", "en1995.integrity.dangling.fasteners.connectionId.fastener-1", "en1995.integrity.dangling.floors.memberId.floor-1"] {
        let c = check(&report, id);
        assert_eq!(c.status, CheckStatus::Fail);
        assert!(!c.remedies[0].options.is_empty() && c.remedies[0].options.iter().all(|o| o != "ghost"), "{id}");
    }
}

#[test]
fn the_fastener_pattern_must_hold_the_declared_number_of_fasteners() {
    let mut d = entity_document();
    d.fasteners[0].columns = 3;
    let report = evaluate_structure(&d);
    let c = check(&report, "en1995.8.pattern-count.fastener-1");
    assert_eq!(c.status, CheckStatus::Fail);
    assert_eq!(c.remedies[0].target.path, "connections[id=conn-C1].number");
}

#[test]
fn the_creep_curve_starts_at_the_instantaneous_deflection_and_ends_at_the_final_deflection_of_7_2() {
    let d = entity_document();
    let floor = &d.floors[0];
    let k_def = floor_k_def(&d, floor);
    assert!(k_def > 0.0, "k_def of the joist is registered");
    let (w_g, w_q) = (floor_strip_deflection(floor.permanent_load_pa, floor.span_m, floor.ei_l_n_m2_per_m), floor_strip_deflection(floor.imposed_load_pa, floor.span_m, floor.ei_l_n_m2_per_m));
    assert!((floor_creep_at(&d, floor, 0.0) - (w_g + w_q)).abs() < 1e-15);
    let end = w_g * (1.0 + k_def) + w_q * (1.0 + floor.psi_2 * k_def);
    assert!((creep_deflection(w_g, w_q, floor.psi_2, k_def, f64::INFINITY) - end).abs() < 1e-15);
    let samples: Vec<f64> = (0..=50).map(|years| floor_creep_at(&d, floor, f64::from(years))).collect();
    assert!(samples.windows(2).all(|pair| pair[1] >= pair[0]), "the deflection grows monotonically with the age");
    assert!((floor_strip_deflection(1000.0, 4.0, 1.0e6) - 5.0 * 1000.0 * 256.0 / (384.0 * 1.0e6)).abs() < 1e-15);
    let check = report_of(&d).checks.into_iter().find(|c| c.id == "en1995.7.2.floor-creep.floor-1").expect("creep check");
    assert_eq!(check.station_field, "creep-time");
    assert!(check.stations_consistent());
    assert!(check.ndps.iter().any(|id| id == "ndp.en1995.creep.time-constant-years"), "the unverified time constant is cited");
}

#[test]
fn the_german_floor_vibration_variant_reads_the_unverified_registry_limits() {
    let mut d = entity_document();
    d.annex = AnnexChoice::De;
    let report = evaluate_structure(&d);
    let f1 = report.checks.iter().find(|c| c.subject.entity_id == "floor-1" && c.id.contains("t-63") && c.id.contains("f1")).expect("T-63 frequency check");
    assert!(f1.ndps.iter().any(|id| id == "ndp.en1995.floor.f1-limit") && f1.unverified_basis(), "the limit is cited and unverified");
    assert!((semio_s_plugin_norm_compliance::ndp_number("ndp.en1995.floor.f1-limit", AnnexChoice::De) - 6.0).abs() < 1e-12);
    assert!((semio_s_plugin_norm_compliance::ndp_number("ndp.en1995.floor.f1-limit", AnnexChoice::En) - 8.0).abs() < 1e-12);
    assert!((semio_s_plugin_norm_compliance::ndp_number("ndp.en1995.floor.unit-load-a", AnnexChoice::De) - 0.5).abs() < 1e-12);
}

fn layer(id: &str, thickness_m: f64, orientation: semio_s_artifact_norm_en1995_model::LayerOrientation) -> semio_s_artifact_norm_en1995_model::PanelLayer {
    semio_s_artifact_norm_en1995_model::PanelLayer { id: id.into(), thickness_m, orientation, grade: "C24".into() }
}

#[test]
fn clt_layers_are_read_bottom_to_top_and_malformed_layers_are_rejected() {
    let mut panel = entity_document().panels[0].clone();
    panel.layers = vec![layer("a", 0.04, LO::Longitudinal), layer("b", 0.03, LO::Transverse), layer("c", 0.04, LO::Longitudinal)];
    assert_eq!(layers_mm(&panel), Some(vec![(40.0, true), (30.0, false), (40.0, true)]));
    panel.layers[1].thickness_m = -0.01;
    assert_eq!(layers_mm(&panel), None, "a negative thickness");
    panel.layers[1].thickness_m = 0.03;
    panel.layers.clear();
    assert_eq!(layers_mm(&panel), None, "a panel without layers");
}

#[semio_framework_async_macros::async_test]
async fn a_panel_without_layers_fails_with_a_check_on_the_layers_and_a_foreign_grade_warns() {
    let mut d = entity_document();
    d.panels[0].layers.clear();
    let report = report_of(&d);
    let failed = report.checks.iter().find(|c| c.id == "en1995.clt.layers.panel-1").expect("layers check");
    assert_eq!(failed.status, CheckStatus::Fail);
    assert_eq!(failed.subject.path, "panels[id=panel-1].layers");
    let mut d = entity_document();
    d.panels[0].layers[2].grade = "C16".into();
    let report = report_of(&d);
    assert_eq!(report.checks.iter().find(|c| c.id == "en1995.clt.mixed-grade.panel-1").map(|c| c.status), Some(CheckStatus::Warning));
}

#[semio_framework_async_macros::async_test]
async fn the_section_follows_the_layer_thicknesses_and_orientations() {
    let thin = report_of(&entity_document());
    let mut d = entity_document();
    for layer in &mut d.panels[0].layers {
        layer.thickness_m *= 2.0;
    }
    let thick = report_of(&d);
    let utilisation = |report: &CheckReport| report.checks.iter().filter(|c| c.id.contains("t-97") && c.subject.entity_id == "panel-1").map(|c| c.utilization).fold(0.0_f64, f64::max);
    assert!(utilisation(&thick) < utilisation(&thin), "thicker layers lower the utilisation");
}

#[test]
fn irregular_fastener_positions_replace_the_regular_pattern_and_give_the_actual_minima() {
    let mut d = entity_document();
    let f = &mut d.fasteners[0];
    f.positions = vec![
        semio_s_artifact_norm_en1995_model::FastenerPosition { id: "p1".into(), x_m: 0.10, y_m: 0.04 },
        semio_s_artifact_norm_en1995_model::FastenerPosition { id: "p2".into(), x_m: 0.17, y_m: 0.04 },
        semio_s_artifact_norm_en1995_model::FastenerPosition { id: "p3".into(), x_m: 0.30, y_m: 0.0805 },
    ];
    let points = pattern_mm(f);
    assert_eq!(points, vec![(100.0, 40.0), (170.0, 40.0), (300.0, 80.5)]);
    let facts = pattern_facts(f, 4000.0, 200.0);
    assert_eq!(facts.count, 3);
    assert_eq!(facts.per_row, 2, "two fasteners share the row at y = 40 mm");
    assert!((facts.a1 - 70.0).abs() < 1e-9, "a1 is the nearest neighbour inside one row");
    assert!((facts.a2 - 40.5).abs() < 1e-9, "a2 is the distance between the two rows");
    assert!((facts.a3t - 100.0).abs() < 1e-9 && (facts.a4c - 40.0).abs() < 1e-9);
    let regular = pattern_facts(&entity_document().fasteners[0], 4000.0, 200.0);
    assert_eq!(regular.count, 8);
    assert!((regular.a1 - 80.0).abs() < 1e-9 && (regular.a2 - 40.0).abs() < 1e-9);
    let (row, homes) = drawing_row(&d, "fasteners", 0).expect("fastener drawing row");
    assert!(row.numbers.iter().any(|n| n.id == "explicit" && n.value == 1.0));
    assert!(homes.iter().any(|h| h.key == "px3" && h.path == "fasteners[id=fastener-1].positions[id=p3].xM"));
    assert!(!homes.iter().any(|h| h.key == "a1"), "an irregular pattern has no spacing field to write");
}

#[test]
fn libraries_agree_with_the_taper_net_section_creep_and_strip_functions_of_the_typed_entities() {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../🔮️oracles/📐️third-party/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&script, &[std::ffi::OsStr::new("--check")], None);
    let answer: serde_json::Value = serde_json::from_slice(&output.stdout).expect("third-party json");
    let item = |label: &str| answer["cases"].as_array().unwrap().iter().find(|c| c["label"] == label).unwrap_or_else(|| panic!("case {label}")).clone();
    let num = |c: &serde_json::Value, key: &str| c["case"]["num"][key].as_f64().unwrap_or_else(|| panic!("number {key}"));
    let val = |c: &serde_json::Value, key: &str| c["values"][key].as_f64().unwrap_or_else(|| panic!("value {key}"));
    let close = |ours: f64, theirs: f64, tolerance: f64, what: &str| assert!((ours - theirs).abs() <= tolerance * theirs.abs(), "{what}: ours {ours} library {theirs}");

    let c = item("taper");
    close(deflection_tapered_udl(num(&c, "q"), num(&c, "l"), num(&c, "b"), num(&c, "h_a"), num(&c, "h_b"), num(&c, "e"), 1.0e15, 1.2), val(&c, "deflection"), 5e-3, "tapered beam deflection (PyNite segments with sectionproperties I)");

    let c = item("hole");
    let utilisation = |hole_d: f64| {
        let mut case = Case::new("hole", AnnexChoice::En);
        for (key, value) in [("n_t", 1.0e4), ("b", num(&c, "b")), ("h", num(&c, "h")), ("hole_d", hole_d), ("hole_count", 1.0), ("sc", 1.0)] {
            case.num.insert(key.to_string(), value);
        }
        case.text.insert("class".into(), "C24".into());
        case.text.insert("duration".into(), "medium".into());
        evaluate_case("T-18", &case).into_iter().find(|r| r.id.contains(".net_section.")).expect("net section check").utilization
    };
    close(utilisation(num(&c, "hole_d")) / utilisation(0.0), val(&c, "a_gross") / val(&c, "a_net"), 1e-9, "net section ratio A/A_net");

    let c = item("creep");
    assert!((semio_s_plugin_norm_compliance::ndp_common("ndp.en1995.creep.time-constant-years") - num(&c, "tau")).abs() < 1e-12, "the oracle runs with the registry time constant");
    for ((age, phi), w) in c["values"]["ages"].as_array().unwrap().iter().zip(c["values"]["phi"].as_array().unwrap()).zip(c["values"]["w"].as_array().unwrap()) {
        let age = age.as_f64().unwrap();
        assert!((creep_factor(num(&c, "k_def"), age) - phi.as_f64().unwrap()).abs() < 1e-7, "creep factor at {age} a");
        close(creep_deflection(val(&c, "w_g"), val(&c, "w_q"), num(&c, "psi_2"), num(&c, "k_def"), age), w.as_f64().unwrap(), 1e-6, "creep deflection");
    }
    close(floor_strip_deflection(num(&c, "g"), num(&c, "span_m"), num(&c, "ei_l")), val(&c, "w_g"), 1e-9, "permanent strip deflection");

    let c = item("strip-deflection");
    close(floor_strip_deflection(num(&c, "q"), num(&c, "span_m"), num(&c, "ei_l")), val(&c, "deflection"), 1e-9, "floor strip deflection");
}

#[test]
fn the_homes_convert_case_units_into_the_stored_units_of_the_entity_fields() {
    let d = entity_document();
    let (row, homes) = drawing_row(&d, "cutouts", 0).expect("notch drawing row");
    let home = |key: &str| homes.iter().find(|h| h.key == key).unwrap_or_else(|| panic!("home {key}"));
    let depth = home("h_ef");
    assert_eq!(depth.path, "cutouts[id=cutout-1].depthM");
    let h_mm = row.numbers.iter().find(|n| n.id == "h").expect("h").value;
    assert!((depth.offset - h_mm).abs() < 1e-9 && depth.scale < 0.0, "h_ef = h − depth is a mirrored field counted from h");
    assert!((depth.stored(h_mm - 60.0, 0.0).0 - 0.06).abs() < 1e-12, "an effective depth of h − 60 mm is a notch depth of 0.06 m");
    assert_eq!(depth.unit(), Some("m"));
    assert_eq!(home("x").unit(), Some("m"));
    let (_, taper) = drawing_row(&d, "tapers", 0).expect("taper drawing row");
    let angle = taper.iter().find(|h| h.key == "alpha_ap_deg").expect("apex angle");
    assert_eq!(angle.unit(), Some("rad"));
    assert!((angle.stored(0.0, 180.0 / std::f64::consts::PI).1 - 1.0).abs() < 1e-9, "one degree of the case is one degree per degree, one radian of the field");
    let (_, fastener) = drawing_row(&d, "fasteners", 0).expect("fastener drawing row");
    assert_eq!(fastener.iter().find(|h| h.key == "n").and_then(Home::unit), None, "a count keeps its unit");
    let (row, _) = drawing_row(&d, "fasteners", 0).expect("fastener drawing row");
    let points = pattern_mm(&d.fasteners[0]);
    assert_eq!(points.len() as u32, d.fasteners[0].columns * d.fasteners[0].lines);
    assert!(row.numbers.iter().filter(|n| n.id.starts_with("px")).count() == points.len());
    assert!((points[0].0 - d.fasteners[0].end_distance_m * 1000.0).abs() < 1e-9 && (points[0].1 - d.fasteners[0].edge_distance_m * 1000.0).abs() < 1e-9, "the first fastener stands at a₃ and a₄");
}

#[semio_framework_async_macros::async_test]
async fn a_remedy_on_a_unit_converted_entity_input_is_one_click_and_lands_in_the_stored_unit() {
    let mut d = entity_document();
    d.cutouts[0].depth_m = 0.25;
    let report = report_of(&d);
    let mut converted = 0;
    for c in report.checks.iter().filter(|c| c.subject.entity_id == "cutout-1") {
        for (index, r) in c.remedies.iter().enumerate().filter(|(_, r)| r.options.is_empty()) {
            assert!(r.applicable, "{}: remedy on {} is one click", c.id, r.target.path);
            if r.target.path == "members[id=beam-B1].bM" {
                assert!((r.current.value - d.members[0].b_m).abs() < 1e-12, "current is the stored width in metres, not millimetres: {}", r.current.value);
                assert!(r.required.value > r.current.value && r.required.value < 10.0, "required {} m", r.required.value);
                let after = apply_remedy(&d, &report, &c.id, index);
                assert!((after.members[0].b_m - r.required.value).abs() < 1e-12, "the click writes the converted value");
                converted += 1;
            }
        }
    }
    assert!(converted > 0, "a failing notch offers a width remedy converted from millimetres");
}

#[test]
fn a_diaphragm_a_truss_and_a_bracing_are_checked_through_the_case_bridge_and_re_homed_onto_their_fields() {
    let report = report_of(&entity_document());
    for (entity, prefix, collection) in [("diaphragm-1", "en1995.t-94.", "diaphragms"), ("truss-1", "en1995.t-95.", "trusses"), ("bracing-1", "en1995.t-96.", "bracings")] {
        let checks: Vec<_> = report.checks.iter().filter(|c| c.subject.entity_id == entity && c.id.starts_with(prefix)).collect();
        assert!(!checks.is_empty(), "{entity} is checked");
        assert!(checks.iter().all(|c| c.subject.path.starts_with(&format!("{collection}[id={entity}]"))), "{entity}: the verdicts address the entity fields");
    }
    let bracing = report.checks.iter().find(|c| c.id.starts_with("en1995.t-96.bracing_force")).expect("bracing force");
    assert!(bracing.ndps.iter().any(|read| read == "ndp.en1995.bracing.stabilising-divisor"), "the unverified stabilising divisor is recorded as read");
}

#[test]
fn a_bracing_on_a_missing_member_and_a_degenerate_truss_fail_with_a_check_on_the_entity() {
    let mut d = entity_document();
    d.bracings[0].member_id = "nowhere".into();
    d.trusses[0].bays = 1;
    let report = report_of(&d);
    let dangling = report.checks.iter().find(|c| c.id == "en1995.integrity.dangling.bracings.memberId.bracing-1").expect("dangling member");
    assert_eq!(dangling.status, CheckStatus::Fail);
    let layout = report.checks.iter().find(|c| c.id == "en1995.trusses.layout.truss-1").expect("truss layout");
    assert_eq!(layout.status, CheckStatus::Fail);
    assert_eq!(layout.subject.path, "trusses[id=truss-1].spanM");
}

#[test]
fn pynite_agrees_with_the_jacking_force_and_the_bar_stress_of_the_typed_deck() {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../🔮️oracles/📐️third-party/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&script, &[std::ffi::OsStr::new("--check")], None);
    let answer: serde_json::Value = serde_json::from_slice(&output.stdout).expect("third-party json");
    let c = answer["cases"].as_array().unwrap().iter().find(|c| c["label"] == "deck-bar").expect("deck-bar").clone();
    let num = |key: &str| c["case"]["num"][key].as_f64().unwrap_or_else(|| panic!("number {key}"));
    let val = |key: &str| c["values"][key].as_f64().unwrap_or_else(|| panic!("value {key}"));
    let d = entity_document();
    let deck = &d.decks[0];
    assert!((num("sigma_p_0") - deck.initial_prestress_pa / 1e6).abs() < 1e-12 && (num("bar_spacing") - deck.bar_spacing_m * 1e3).abs() < 1e-9 && (num("deck_depth") - deck.deck_depth_m * 1e3).abs() < 1e-9, "the library case is the demo deck");
    assert!((num("bar_area") - std::f64::consts::FRAC_PI_4 * (deck.bar_diameter_m * 1e3).powi(2)).abs() < 1e-9, "the bar area is that of the bar diameter");
    let report = report_of(&d);
    let bar = report.checks.iter().find(|r| r.id.starts_with("en1995.t-122.bar_force")).expect("bar force");
    let resistance = deck.bar_resistance_pa / 1e6;
    assert!((bar.utilization * num("bar_area") * resistance - val("axial")).abs() < 1e-6 * val("axial"), "the jacking force P = σ_p,0 s h is the axial force of the library");
    assert!((bar.utilization * resistance - val("stress")).abs() < 1e-6 * val("stress"), "the bar stress of the library over the bar resistance is the utilisation");
}

#[test]
fn a_stress_laminated_deck_reproduces_the_hand_values_of_t_122_and_is_re_homed_onto_its_fields() {
    let report = report_of(&entity_document());
    let of = |key: &str| report.checks.iter().find(|c| c.subject.entity_id == "deck-1" && c.id.starts_with(&format!("en1995.t-122.{key}"))).unwrap_or_else(|| panic!("{key} is checked"));
    let kept = 0.85 * 0.75 * 0.9;
    let (force, area) = (1.0 * 800.0 * 250.0, std::f64::consts::FRAC_PI_4 * 32.0 * 32.0);
    assert!((of("prestress_min").utilization - 0.35 / kept).abs() < 1e-9);
    assert!((of("friction").utilization - 0.1 / (0.3 * kept)).abs() < 1e-9);
    assert!((of("bar_force").utilization - force / (area * 700.0)).abs() < 1e-9);
    assert!((of("plate_bearing").utilization - force / (300.0 * 300.0) / (1.5 * 1.54)).abs() < 1e-9);
    assert!((of("wheel_spread").utilization - 100_000.0 / ((400.0 + 2.0 * 250.0) * 400.0) / 1.54).abs() < 1e-9);
    assert!(report.checks.iter().filter(|c| c.subject.entity_id == "deck-1").all(|c| c.subject.path.starts_with("decks[id=deck-1]")), "the verdicts address the deck fields");
    for id in ["ndp.en1995.deck.min-prestress", "ndp.en1995.deck.friction-coefficient", "ndp.en1995.deck.k-c90"] {
        assert!(report.checks.iter().any(|c| c.id.starts_with("en1995.t-122.") && c.ndps.iter().any(|read| read == id)), "{id} is recorded as read");
        assert!(!semio_s_plugin_norm_compliance::ndp_is_verified(id), "{id} stays unverified until a human checks the print");
    }
}

#[test]
fn a_weak_deck_fails_with_remedies_on_the_deck_fields_and_the_remedies_fix_it() {
    let mut d = entity_document();
    d.decks[0].loss_creep = 0.6;
    d.decks[0].plate_side_m = 0.1;
    d.decks[0].bar_resistance_pa = 200.0e6;
    let report = report_of(&d);
    for (key, leaf) in [("prestress_min", "initialPrestressPa"), ("bar_force", "barResistancePa"), ("plate_bearing", "plateSideM")] {
        let check = report.checks.iter().find(|c| c.subject.entity_id == "deck-1" && c.id.starts_with(&format!("en1995.t-122.{key}"))).unwrap_or_else(|| panic!("{key}"));
        assert_eq!(check.status, CheckStatus::Fail, "{key}");
        let remedy = check.remedies.iter().find(|r| r.target.path == format!("decks[id=deck-1].{leaf}")).unwrap_or_else(|| panic!("{key}: {:?}", check.remedies.iter().map(|r| r.target.path.clone()).collect::<Vec<_>>()));
        assert!(remedy.applicable && remedy.required.value > remedy.current.value, "{key}");
    }
    let wheel = report.checks.iter().find(|c| c.id.starts_with("en1995.t-122.wheel_spread")).expect("wheel spread");
    assert!(wheel.remedies.is_empty(), "no sizing on the prestress for the wheel spread");
    let check = report.checks.iter().find(|c| c.id.starts_with("en1995.t-122.plate_bearing")).expect("plate");
    let fixed = apply_remedy(&d, &report, &check.id, 0);
    assert!(fixed.decks[0].plate_side_m > 0.1);
    let again = report_of(&fixed);
    assert!(again.checks.iter().find(|c| c.id.starts_with("en1995.t-122.plate_bearing")).expect("plate").utilization <= 1.0 + 1e-9, "the click satisfies the bearing check");
}

#[test]
fn a_degenerate_deck_fails_with_a_layout_check_on_the_entity() {
    let mut d = entity_document();
    d.decks[0].loss_moisture = 1.0;
    let report = report_of(&d);
    let layout = report.checks.iter().find(|c| c.id == "en1995.decks.layout.deck-1").expect("deck layout");
    assert_eq!(layout.status, CheckStatus::Fail);
    assert_eq!(layout.subject.path, "decks[id=deck-1].spanM");
    assert!(!report.checks.iter().any(|c| c.id.starts_with("en1995.t-122.") && c.subject.entity_id == "deck-1"), "no verdict is drawn from a degenerate deck");
}

#[test]
fn a_wall_and_a_floor_diaphragm_read_their_own_inputs() {
    let mut d = entity_document();
    let wall = report_of(&d);
    assert!(wall.checks.iter().any(|c| c.id.starts_with("en1995.t-94.racking")) && !wall.checks.iter().any(|c| c.id.starts_with("en1995.t-94.diaphragm_shear")));
    d.diaphragms[0].kind = semio_s_artifact_norm_en1995_model::DiaphragmKind::Floor;
    let floor = report_of(&d);
    assert!(floor.checks.iter().any(|c| c.id.starts_with("en1995.t-94.diaphragm_shear")) && !floor.checks.iter().any(|c| c.id.starts_with("en1995.t-94.racking")));
}

#[test]
fn every_truss_layout_is_a_statically_determinate_pin_jointed_frame() {
    use semio_s_artifact_norm_en1995_model::TrussKind;
    for kind in [TrussKind::Warren, TrussKind::Pratt, TrussKind::Howe] {
        for bays in 2..=12 {
            let (joints, members) = truss_layout(kind, 12.0, 1.5, bays);
            assert_eq!(members.len() + 3, 2 * joints.len(), "{kind:?} with {bays} bays: 2j - 3 = m");
            assert!(members.iter().all(|(a, b, _)| a != b && *a < joints.len() && *b < joints.len()));
        }
    }
    let (joints, members) = truss_layout(TrussKind::KingPost, 8.0, 2.0, 6);
    assert_eq!((joints.len(), members.len()), (4, 5));
}

#[test]
fn an_unknown_strength_class_of_a_panel_or_a_layer_fails_against_the_catalogue_and_offers_the_catalogue() {
    let mut d = entity_document();
    d.panels[0].layers[0].grade = "C99".into();
    let report = report_of(&d);
    let check = report.checks.iter().find(|c| c.id == "en1995.catalogue.strength-class.panels.panel-1").expect("catalogue reference check");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_eq!(check.subject.path, "panels[id=panel-1].layers[id=layer-1].grade");
    assert!(check.remedies.iter().any(|r| r.options.iter().any(|o| o == "C24")), "the remedy lists the catalogue ids");
}

#[test]
fn pynite_agrees_with_the_truss_layouts_the_chord_forces_and_the_bracing_force_of_the_typed_entities() {
    use semio_s_artifact_norm_en1995_model::TrussKind;
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../🔮️oracles/📐️third-party/🐍️.py");
    let output = semio_s_plugin_norm_compliance::python_oracle::run_ok(&script, &[std::ffi::OsStr::new("--check")], None);
    let answer: serde_json::Value = serde_json::from_slice(&output.stdout).expect("third-party json");
    let item = |label: &str| answer["cases"].as_array().unwrap().iter().find(|c| c["label"] == label).unwrap_or_else(|| panic!("case {label}")).clone();
    let num = |c: &serde_json::Value, key: &str| c["case"]["num"][key].as_f64().unwrap_or_else(|| panic!("number {key}"));
    let val = |c: &serde_json::Value, key: &str| c["values"][key].as_f64().unwrap_or_else(|| panic!("value {key}"));
    for (label, kind) in [("warren-truss", TrussKind::Warren), ("pratt-truss", TrussKind::Pratt)] {
        let c = item(label);
        let (joints, members) = truss_layout(kind, num(&c, "span_m"), num(&c, "height_m"), num(&c, "bays") as u32);
        assert_eq!((joints.len() as f64, members.len() as f64), (val(&c, "joints"), val(&c, "members")), "{label}: the layout has the joints and members the library solved");
        let closed = num(&c, "q") * num(&c, "span_m").powi(2) / 8.0 / num(&c, "height_m");
        assert!((val(&c, "n_chord") - closed).abs() < 1e-6 * closed, "{label}: the chord force of the library is M/H = q l²/(8 h)");
    }
    let c = item("bracing-spring");
    let mut d = entity_document();
    let b = &mut d.bracings[0];
    (b.stabilised_count, b.compression_force_n, b.length_m, b.resistance_n, b.stiffness_n_per_m) = (num(&c, "n_braced") as u32, num(&c, "n_d"), num(&c, "l_m"), 2000.0, num(&c, "k_br") * 1000.0);
    let report = report_of(&d);
    let force = report.checks.iter().find(|r| r.id.starts_with("en1995.t-96.bracing_force")).expect("bracing force");
    assert!((force.utilization * 2000.0 - val(&c, "f_d")).abs() < 1e-6 * val(&c, "f_d"), "ΣF_d = q_d l equals the sum of the spring reactions of the library");
    let deflection = report.checks.iter().find(|r| r.id.starts_with("en1995.t-96.bracing_deflection")).expect("bracing deflection");
    assert!((deflection.utilization * val(&c, "limit_mm") - val(&c, "delta_mm")).abs() < 1e-6 * val(&c, "delta_mm"), "the kernel deflection F/k_br of the bracing system equals the end deflection of the library with the system stiffness split over the two end springs");
}

#[test]
fn reliability_class_rc3_raises_the_governing_design_effect_by_k_fi() {
    for annex in [AnnexChoice::De, AnnexChoice::En] {
        let member = En1995Snapshot::compliant_building_beam().members[0].clone();
        let governing = |class| {
            let combo = governing_uls(&enumerate_combos(&member, &AnnexParams::new(annex, class))).expect("uls combination").clone();
            combo.q_ed_line.abs() + combo.f_ed_point.abs()
        };
        let k_fi = ReliabilityClass::RC3.k_fi(annex);
        assert!((k_fi - 1.1).abs() < 1e-9, "registry K_FI of RC3 (unverified) is {k_fi}");
        let ratio = governing(ReliabilityClass::RC3) / governing(ReliabilityClass::RC2);
        assert!((ratio - k_fi / ReliabilityClass::RC2.k_fi(annex)).abs() < 1e-9, "governing design effect ratio {ratio}");
    }
}

#[test]
fn reliability_class_of_the_document_reaches_the_evaluation() {
    let rc2 = En1995Snapshot::compliant_building_beam();
    let rc3 = En1995Snapshot { reliability_class: ReliabilityClass::RC3, ..rc2.clone() };
    let load_effect = |document: &En1995Snapshot| report_of(document).checks.iter().map(|c| c.utilization).sum::<f64>();
    assert!(load_effect(&rc3) > load_effect(&rc2));
}

#[test]
fn the_buckling_probe_reproduces_the_spec_example_for_both_axes() {
    let case = Case::new("col", AnnexChoice::De).with_s("class", "C24").with_s("duration", "medium").with_n("sc", 2.0).with_n("b", 100.0).with_n("h", 100.0).with_n("lef_y", 3000.0).with_n("lef_z", 4500.0).with_n("n_c", 10_000.0);
    let p = column_probe(&case).unwrap();
    assert!((p.beta_c - 0.2).abs() < 1e-12);
    assert!((p.y.lambda_rel - 1.7622).abs() < 1e-3 && (p.y.kc - 0.28457).abs() < 1e-4, "{p:?}");
    assert!((p.z.lambda_rel - 1.5 * p.y.lambda_rel).abs() < 1e-9 && (p.z.kc - 0.1328).abs() < 5e-4, "{p:?}");
    assert!((p.y.kc - kc_factor(p.y.lambda_rel, p.beta_c)).abs() < 1e-15 && (p.z.lef - 4500.0).abs() < 1e-12);
    let util = |key: &str| evaluate_outcomes("T-20", &case).unwrap().into_iter().find(|o| o.key == key).unwrap().utilization();
    assert!(util("buckling_z") > util("buckling_y"), "the longer axis governs");
}

#[test]
fn the_lateral_buckling_probe_follows_the_simplified_critical_stress_and_the_restraint() {
    let case = Case::new("ltb", AnnexChoice::En).with_s("class", "C24").with_n("b", 120.0).with_n("h", 400.0).with_n("lef", 9000.0);
    let p = ltb_probe(&case).unwrap();
    let (rel, kcrit) = p.at(9000.0);
    assert!((rel - 1.01956).abs() < 1e-4 && (kcrit - 0.79533).abs() < 1e-4, "{rel} {kcrit}");
    let (short, free) = p.at(2250.0);
    assert!((short - 0.50058).abs() < 1e-4 && (free - 1.0).abs() < 1e-12, "a short span does not buckle laterally");
    assert!(p.at(18_000.0).1 < kcrit, "a longer span buckles earlier");
    let restrained = ltb_probe(&case.clone().with_n("restrained", 1.0)).unwrap();
    assert!((restrained.at(9000.0).1 - 1.0).abs() < 1e-12 && (restrained.at(9000.0).0 - rel).abs() < 1e-12);
    let span = ltb_probe(&Case::new("ltb", AnnexChoice::En).with_s("class", "C24").with_n("b", 120.0).with_n("h", 400.0).with_n("l", 10_000.0)).unwrap();
    assert!((span.lef - lef_ltb(10_000.0, 400.0, "udl", "centroid")).abs() < 1e-9, "Table 6.1 without lef");
}

#[test]
fn the_deflection_probe_is_the_final_deflection_of_t61_at_every_k_def() {
    let case = Case::new("defl", AnnexChoice::En).with_s("class", "C24").with_n("b", 100.0).with_n("h", 200.0).with_n("l", 4000.0).with_n("load_g", 1.5).with_n("load_q", 2.0).with_n("k_def", 0.6).with_n("psi_2", 0.3).with_n("w_c", 5.0);
    let row = strength_row("C24").unwrap();
    let one = |q: f64| deflection_case("udl_ss", q, 4000.0, row.e_0_mean, 100.0 * 200.0_f64.powi(3) / 12.0, row.g_mean, 100.0 * 200.0, 1.2);
    let p = deflection_probe(&case).unwrap();
    assert!((p.w_g - one(1.5)).abs() < 1e-12 && (p.w_q - one(2.0)).abs() < 1e-12);
    let (fin, net) = p.at(0.6);
    assert!((fin - (one(1.5) * 1.6 + one(2.0) * 1.18)).abs() < 1e-9 && (net - (fin - 5.0)).abs() < 1e-12);
    let outcomes = evaluate_outcomes("T-61", &case).unwrap();
    let w_fin = outcomes.iter().find(|o| o.key == "w_fin").unwrap();
    assert!((w_fin.demand * 1e3 - fin).abs() < 1e-9 && (w_fin.capacity * 1e3 - p.limit_fin).abs() < 1e-9, "the probe and the check agree");
    assert!((p.at(0.0).0 - (p.w_g + p.w_q)).abs() < 1e-12, "no creep, the instantaneous sum");
    assert!((p.at(2.0).0 - p.at(1.0).0 - (p.w_g + 0.3 * p.w_q)).abs() < 1e-9, "linear in k_def with the slope w_G + ψ₂ w_Q");
}

#[test]
fn the_embedment_probe_reproduces_the_angle_law_of_bolts_and_the_constancy_of_nails() {
    let bolt = Case::new("bolt", AnnexChoice::En).with_s("kind", "bolt").with_n("d", 12.0).with_n("rho_k1", 350.0).with_n("f_u", 400.0).with_n("alpha_deg", 30.0);
    let p = embedment_probe(&bolt).unwrap();
    let f0 = 0.082 * (1.0 - 0.12) * 350.0;
    assert!((p.at(0.0) - f0).abs() < 1e-9 && (p.at(90.0) - f0 / 1.53).abs() < 1e-9 && p.angle_dependent());
    let a = 30.0_f64.to_radians();
    assert!((p.at(30.0) - f0 / (1.53 * a.sin().powi(2) + a.cos().powi(2))).abs() < 1e-9);
    let out = evaluate_outcomes("T-70", &bolt).unwrap();
    assert!((out.iter().find(|o| o.key == "f_h_k").unwrap().capacity / 1e6 - p.at(30.0)).abs() < 1e-9, "the check reads the probe");
    let nail = Case::new("nail", AnnexChoice::En).with_s("kind", "nail").with_n("d", 3.0).with_n("rho_k1", 350.0).with_n("f_u", 600.0);
    let q = embedment_probe(&nail).unwrap();
    assert!(!q.angle_dependent() && (q.at(0.0) - 0.082 * 350.0 * 3.0_f64.powf(-0.3)).abs() < 1e-9);
}

#[test]
fn the_protected_probe_starts_charring_behind_the_cladding_and_doubles_the_rate_after_its_failure() {
    let case = Case::new("clad", AnnexChoice::En).with_s("class", "C24").with_n("h_p", 20.0).with_n("t_min", 60.0);
    let p = protected_probe(&case).unwrap();
    assert!((p.t_ch - 42.0).abs() < 1e-12 && (p.t_f - 42.0).abs() < 1e-12 && (p.b0 - 0.65).abs() < 1e-12);
    assert!(p.depth_at(30.0).abs() < 1e-12 && p.depth_at(42.0).abs() < 1e-12);
    assert!((p.depth_at(60.0) - 2.0 * 0.65 * 18.0).abs() < 1e-9, "a type F board fails when charring starts");
    let out = evaluate_outcomes("T-103", &case).unwrap();
    assert!((out.iter().find(|o| o.key == "d_char_protected").unwrap().capacity * 1e3 - p.depth_at(60.0)).abs() < 1e-9, "the check reads the probe");
    let late = protected_probe(&case.clone().with_s("board", "A").with_n("h_p", 30.0)).unwrap();
    assert!((late.t_ch - 70.0).abs() < 1e-12 && (late.t_f - 56.0).abs() < 1e-12 && late.depth_at(60.0).abs() < 1e-12, "charring has not started at 60 min");
}

#[test]
fn the_response_curves_of_the_diagrams_match_the_numpy_oracle_at_every_abscissa() {
    let row = |entry: &str, num: serde_json::Value, text: serde_json::Value, at: Vec<f64>| serde_json::json!({"entry": entry, "case": {"annex": "En", "num": num, "text": text}, "at": at});
    let rows = vec![
        row("T-20", serde_json::json!({"b": 100.0, "h": 100.0, "lef_y": 3000.0, "lef_z": 3000.0}), serde_json::json!({"class": "C24"}), vec![0.0, 0.2, 0.3, 0.31, 0.8, 1.7622, 2.6433, 3.3]),
        row("T-22", serde_json::json!({"b": 120.0, "h": 400.0, "lef": 9000.0}), serde_json::json!({"class": "C24"}), vec![1500.0, 2250.0, 4500.0, 6000.0, 9000.0, 18_000.0, 27_000.0]),
        row("T-61", serde_json::json!({"b": 100.0, "h": 200.0, "l": 4000.0, "load_g": 1.5, "load_q": 2.0, "k_def": 0.6}), serde_json::json!({"class": "C24"}), vec![0.0, 0.25, 0.6, 1.0, 2.0, 2.5]),
        row("T-70", serde_json::json!({"d": 12.0, "rho_k1": 350.0, "f_u": 400.0}), serde_json::json!({"kind": "bolt"}), vec![0.0, 15.0, 30.0, 45.0, 60.0, 90.0]),
        row("T-103", serde_json::json!({"h_p": 20.0, "t_min": 60.0}), serde_json::json!({"class": "C24"}), vec![0.0, 30.0, 42.0, 50.0, 60.0, 90.0]),
        row("T-103", serde_json::json!({"h_p": 30.0, "t_min": 90.0}), serde_json::json!({"class": "GL28h", "board": "A"}), vec![0.0, 56.0, 70.0, 80.0, 120.0]),
    ];
    let oracle = oracle_mode("--curves", &serde_json::json!({"cases": rows}));
    let mut compared = 0;
    for (item, expected) in rows.iter().zip(oracle["cases"].as_array().unwrap()) {
        let entry = item["entry"].as_str().unwrap();
        let case = case_of(item);
        let at: Vec<f64> = item["at"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
        let rust: Vec<f64> = match entry {
            "T-20" => { let beta = column_probe(&case.clone().with_n("n_c", 1.0).with_n("lef_z", 3000.0)).unwrap().beta_c; at.iter().map(|x| kc_factor(*x, beta)).collect() }
            "T-22" => { let p = ltb_probe(&case).unwrap(); at.iter().map(|x| p.at(*x).1).collect() }
            "T-61" => { let p = deflection_probe(&case).unwrap(); at.iter().map(|x| p.at(*x).0).collect() }
            "T-70" => { let p = embedment_probe(&case).unwrap(); at.iter().map(|x| p.at(*x)).collect() }
            _ => { let p = protected_probe(&case).unwrap(); at.iter().map(|x| p.depth_at(*x)).collect() }
        };
        let values = expected["values"].as_array().unwrap();
        assert_eq!(values.len(), rust.len(), "{entry}");
        for ((x, r), v) in at.iter().zip(&rust).zip(values) {
            let py = v.as_f64().unwrap();
            assert!((r - py).abs() <= 1e-9 * py.abs().max(1.0), "{entry} at {x}: rust={r} numpy={py}");
            compared += 1;
        }
    }
    assert!(compared >= 38, "only {compared} values compared");
}

#[test]
fn the_entity_curves_of_the_tapers_and_cutouts_match_hand_values_and_the_numpy_oracle() {
    let notch = serde_json::json!({"b": 140.0, "h": 400.0, "h_ef": 300.0, "x": 100.0, "incl": 0.5});
    let row = |entry: &str, num: serde_json::Value, text: serde_json::Value, at: Vec<f64>| serde_json::json!({"entry": entry, "case": {"annex": "En", "num": num, "text": text}, "at": at});
    let apex = serde_json::json!({"h_ap": 900.0, "alpha_ap_deg": 10.0});
    let radii = vec![3000.0, 12_000.0, 30_000.0];
    let rows = vec![
        row("T-45", notch, serde_json::json!({"class": "GL28h", "side": "support"}), vec![0.0, 40.0, 100.0, 200.0, 280.0]),
        row("T-46", serde_json::json!({"h": 400.0}), serde_json::json!({"class": "GL28h"}), vec![0.0, 40.0, 100.0, 200.0]),
        row("T-31", apex.clone(), serde_json::json!({"class": "GL28h"}), radii.clone()),
        row("T-32", apex, serde_json::json!({"class": "GL28h"}), radii),
        row("T-33", serde_json::json!({"t_lam": 33.0}), serde_json::json!({"class": "GL28h"}), vec![3000.0, 7920.0, 8000.0, 30_000.0]),
    ];
    let oracle = oracle_mode("--curves", &serde_json::json!({"cases": rows}));
    let mut compared = 0;
    for (item, expected) in rows.iter().zip(oracle["cases"].as_array().unwrap()) {
        let entry = item["entry"].as_str().unwrap();
        let at: Vec<f64> = item["at"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
        let num = |key: &str| item["case"]["num"][key].as_f64().unwrap_or(0.0);
        let alpha = num("alpha_ap_deg").to_radians();
        let rust: Vec<f64> = match entry {
            "T-45" => { let p = notch_probe(&case_of(item)).unwrap(); at.iter().map(|d| p.kv(1.0 - d / p.h)).collect() }
            "T-46" => at.iter().map(|d| notch_ft90(1.0, 1.0 - d / num("h"))).collect(),
            "T-31" => at.iter().map(|r| apex_kl(alpha, num("h_ap") / (r + 0.5 * num("h_ap")))).collect(),
            "T-32" => at.iter().map(|r| apex_kp(alpha, num("h_ap") / (r + 0.5 * num("h_ap")))).collect(),
            _ => at.iter().map(|r| apex_kr(*r, num("t_lam"))).collect(),
        };
        let values = expected["values"].as_array().unwrap();
        assert_eq!(values.len(), rust.len(), "{entry}");
        for ((x, r), v) in at.iter().zip(&rust).zip(values) {
            let py = v.as_f64().unwrap();
            assert!((r - py).abs() <= 1e-9 * py.abs().max(1.0), "{entry} at {x}: rust={r} numpy={py}");
            compared += 1;
        }
    }
    assert!(compared >= 19, "only {compared} values compared");
    assert!((notch_ft90(1.0, 0.9) - 0.0364).abs() < 1e-12 && (notch_ft90(1.0, 0.75) - 0.203_125).abs() < 1e-12, "F_t,90,d / V_d = 1.3 (3·0.1² − 2·0.1³) and 1.3 (3·0.25² − 2·0.25³)");
    assert!((apex_kr(3000.0, 33.0) - (0.76 + 0.001 * 3000.0 / 33.0)).abs() < 1e-12 && apex_kr(7920.0, 33.0) == 1.0, "k_r is 1 from r_in / t = 240");
    assert!(HOLE_LIMIT_RATIO == 0.15 && NOTCH_NA_MIN_RATIO == 0.5);
}

#[test]
fn a_timber_concrete_composite_beam_matches_the_hand_values_and_the_newmark_oracle() {
    let case = Case::new("tcc", AnnexChoice::En).with_n("a1", 48_000.0).with_n("i1", 25.6e6).with_n("e1", 33_000.0).with_n("h1", 80.0).with_n("s1", 150.0).with_n("a2", 28_800.0).with_n("i2", 138.24e6).with_n("e2", 11_500.0).with_n("h2", 240.0).with_n("k", 15_000.0).with_n("l", 6000.0).with_n("m_d", 20e6).with_n("v_d", 10_000.0);
    let parts = vec![Part { a: 48_000.0, i: 25.6e6, e: 33_000.0, h: 80.0, s: 150.0 }, Part { a: 28_800.0, i: 138.24e6, e: 11_500.0, h: 240.0, s: 1.0 }];
    let service = joint_profile(&parts, 15_000.0, 6000.0, 20e6, 10_000.0, false);
    let rigid = joint_profile(&parts, 15_000.0, 6000.0, 20e6, 10_000.0, true);
    let ultimate = gamma_beam(&parts, ultimate_slip_modulus(15_000.0), 6000.0);
    assert!((service.gamma[0] - 0.187_174).abs() < 1e-6 && (service.ei_ef - 6.439_445e12).abs() < 1e7 && (rigid.ei_ef - 9.447_035e12).abs() < 1e7 && (ultimate.ei_ef - 5.732_267e12).abs() < 1e7);
    assert!((service.fibres[0].top + 5.7193).abs() < 1e-4 && (service.fibres[0].bottom - 2.4801).abs() < 1e-4 && (service.fibres[1].top + 1.5867).abs() < 1e-4 && (service.fibres[1].bottom - 6.9854).abs() < 1e-4, "slab on top is compressed, the timber carries tension at its bottom");
    assert!((service.fastener[0] - 5830.6).abs() < 0.1 && (service.slip[0] - 0.388_71).abs() < 1e-4 && service.slip_at(0, 3000.0, 6000.0) < 1e-9 && (service.slip_at(0, 0.0, 6000.0) - service.slip[0]).abs() < 1e-12, "the slip is largest at the support and zero at midspan");
    assert!(rigid.slip[0] == 0.0 && rigid.fibres[1].top > service.fibres[1].top, "a rigid composite takes less compression into the timber web");
    assert!(is_mineral_part(33_000.0) && !is_mineral_part(11_500.0) && !is_mineral_part(13_000.0));
    let out = evaluate_outcomes("T-90", &case).unwrap();
    let get = |key: &str| out.iter().find(|o| o.key == key).unwrap_or_else(|| panic!("outcome {key}")).capacity;
    assert!((get("ei_ef_u") - ultimate.ei_ef).abs() < 1.0 && (get("slip_support") * 1e3 - service.slip[0]).abs() < 1e-9);
    let oracle = oracle_mode("--joint", &serde_json::json!({"cases": [{"num": {"a1": 48000.0, "i1": 25.6e6, "e1": 33000.0, "h1": 80.0, "s1": 150.0, "a2": 28800.0, "i2": 138.24e6, "e2": 11500.0, "h2": 240.0, "k": 15000.0, "l": 6000.0, "m_d": 20e6, "v_d": 10000.0}}]}));
    let py = &oracle["cases"][0];
    let near = |a: f64, b: serde_json::Value| (a - b.as_f64().unwrap()).abs() <= 1e-9 * b.as_f64().unwrap().abs().max(1.0);
    for (rust, name) in [(&service, "service"), (&rigid, "rigid")] {
        assert!(near(rust.ei_ef, py[name]["ei"].clone()), "{name} EI");
        assert!(near(rust.fibres[0].top, py[name]["part1"][0].clone()) && near(rust.fibres[0].bottom, py[name]["part1"][1].clone()) && near(rust.fibres[1].top, py[name]["part2"][0].clone()) && near(rust.fibres[1].bottom, py[name]["part2"][1].clone()), "{name} fibres");
    }
    assert!(near(service.fastener[0], py["service"]["fastener"].clone()) && near(service.slip[0], py["service"]["slip"].clone()) && near(ultimate.ei_ef, py["ultimate"]["ei"].clone()));
}

#[test]
fn imposed_categories_f_and_g_read_their_table_a1_1_rows_through_the_shared_action_factors() {
    use semio_s_artifact_norm_en1995_model::{ActionKind as K, ImposedCategory as C};
    for annex in [AnnexParams::de(), AnnexParams::en()] {
        let factors = annex.factors;
        assert_eq!(psi_factors(&factors, K::Imposed, C::F), (0.7, 0.7, 0.6), "traffic area up to 30 kN");
        assert_eq!(psi_factors(&factors, K::Imposed, C::G), (0.7, 0.5, 0.3), "traffic area 30 to 160 kN");
        assert_eq!(psi_factors(&factors, K::Imposed, C::H), (0.0, 0.0, 0.0), "roofs");
        assert_eq!(psi_factors(&factors, K::Imposed, C::F), (factors.psi("F").psi_0, factors.psi("F").psi_1, factors.psi("F").psi_2), "the timber crate adds no table of its own");
    }
}

#[test]
fn bolts_in_plywood_state_that_the_nail_formula_is_unverified() {
    let case = |kind: &str| Case::new("ply", AnnexChoice::En).with_s("kind", kind).with_s("panel", "plywood").with_n("d", 12.0).with_n("rho_k1", 350.0).with_n("f_u", 400.0);
    let flagged = |kind: &str| evaluate_outcomes("T-70", &case(kind)).unwrap().iter().any(|o| o.key == "plywood_bolt_unverified");
    assert!(flagged("bolt") && flagged("dowel") && !flagged("nail"));
    let fh = embedment_probe(&case("bolt")).unwrap().at(0.0);
    assert!((fh - 0.11 * 350.0 * 12.0_f64.powf(-0.3)).abs() < 1e-12, "the nail value stays as the stand-in");
}

#[test]
fn the_de_floor_band_between_6_and_8_hz_requires_the_unit_load_criterion_and_nothing_else_is_invented() {
    let floor = |ei: f64, annex| Case::new("floor", annex).with_n("span_m", 4.0).with_n("ei_l", ei).with_n("mass_kg_m2", 150.0);
    let f1 = |ei: f64| floor_f1(4.0, ei, 150.0);
    let (low, high) = (0.8e6, 3.0e6);
    assert!(f1(low) > 6.0 && f1(low) < 8.0 && f1(high) > 8.0, "{} {}", f1(low), f1(high));
    let band = |ei: f64, annex| evaluate_outcomes("T-63", &floor(ei, annex)).unwrap().iter().any(|o| o.key == "additional_criterion");
    assert!(band(low, AnnexChoice::De) && !band(high, AnnexChoice::De) && !band(low, AnnexChoice::En));
}

#[test]
fn a_tapered_beam_is_governed_at_the_section_of_its_moment_line_and_not_at_midspan() {
    let alpha = 0.05_f64.atan().to_degrees();
    let case = |load: &str| Case::new("tp", AnnexChoice::En).with_s("class", "GL28h").with_s("side", "tension").with_s("duration", "medium").with_n("sc", 1.0).with_n("b", 160.0).with_n("h", 800.0).with_n("alpha_deg", alpha).with_n("m_d", 100e6).with_n("l", 8000.0).with_s("load_case", load);
    let p = taper_probe(&case("udl_ss")).unwrap();
    let g = p.governing();
    assert!((g.x - 3200.0).abs() < 1e-3, "x* = l / (2 + c) with c = l tan α / h = 0.5, not midspan: {}", g.x);
    assert!((g.h - 960.0).abs() < 1e-3 && (g.m - 0.96e8).abs() < 1.0, "h(x*) = 960 mm, M(x*) = 4 ξ (1 − ξ) M = 0.96 M: {g:?}");
    assert!((g.sigma - 3.90625).abs() < 1e-9, "σ* = 6 M / (b h² (1 + c)) = 6e8 / (160 · 640000 · 1.5) = 3.90625 N/mm²: {}", g.sigma);
    assert!((g.fmd - 17.92).abs() < 1e-9 && (g.k - 0.875_71).abs() < 1e-4 && (g.tapered - 0.248_925).abs() < 1e-5, "{g:?}");
    let mid = p.section(4000.0);
    assert!(mid.sigma < g.sigma && (mid.sigma - 3.75).abs() < 1e-9 && mid.tapered < g.tapered, "midspan (σ = 3.75 N/mm²) underestimates the governing section by 4 %");
    let bare = Case::new("tp", AnnexChoice::En).with_s("class", "GL28h").with_n("b", 160.0).with_n("h", 800.0).with_n("alpha_deg", alpha).with_n("m_d", 100e6);
    assert!(taper_probe(&bare).is_err(), "a constant moment is no moment line: the fallback is gone");
    assert!(evaluate_outcomes("T-30", &bare).is_err() && evaluate_outcomes("T-30", &bare.clone().with_s("load_case", "udl_ss")).is_err(), "span and load case are both required");
    let refused = semio_s_plugin_norm_compliance::explain_reason(&evaluate_outcomes("T-30", &bare).err().expect("refusal"));
    assert_eq!(refused.code, "input.moment-line");
    assert_eq!(refused.de, "Die Momentenlinie des Bauteils ist unbekannt: Spannweite l und Lastfall angeben");
    let out = evaluate_outcomes("T-30", &case("udl_ss")).unwrap();
    let get = |key: &str| out.iter().find(|o| o.key == key).unwrap_or_else(|| panic!("outcome {key}"));
    assert!((get("tapered_edge").demand - 3.90625e6).abs() < 1e-3 && (get("governing_section").capacity - 3.2).abs() < 1e-9);
    let mixed = case("udl_ss").with_n("point_share", 0.5);
    let cases = ["udl_ss", "point_mid", "cantilever_udl", "cantilever_point"].map(|load| (load, if load == "udl_ss" { mixed.clone() } else { case(load) }));
    let rows: Vec<serde_json::Value> = cases.iter().map(|(_, c)| serde_json::json!({"entry": "T-30-governing", "case": {"annex": "En", "num": c.num, "text": c.text}, "at": []})).collect();
    let oracle = oracle_mode("--curves", &serde_json::json!({"cases": rows}));
    for ((load, c), expected) in cases.iter().zip(oracle["cases"].as_array().unwrap()) {
        let (x, ratio) = (expected["values"][0].as_f64().unwrap(), expected["values"][1].as_f64().unwrap());
        let g = taper_probe(c).unwrap().governing();
        assert!((g.x - x).abs() <= 0.05 && (g.tapered - ratio).abs() <= 1e-9 * ratio.max(1.0), "{load}: rust x={} u={} numpy x={x} u={ratio}", g.x, g.tapered);
    }
    let grid = serde_json::json!({"cases": [{"entry": "T-30", "case": {"annex": "En", "num": mixed.num, "text": mixed.text}, "at": [0.0, 1000.0, 3200.0, 4000.0, 7000.0, 8000.0]}]});
    let curve = oracle_mode("--curves", &grid);
    let along = taper_probe(&mixed).unwrap();
    for (x, v) in [0.0, 1000.0, 3200.0, 4000.0, 7000.0, 8000.0].iter().zip(curve["cases"][0]["values"].as_array().unwrap()) {
        let py = v.as_f64().unwrap();
        assert!((along.section(*x).tapered - py).abs() <= 1e-9 * py.abs().max(1.0), "section at {x}");
    }
}

#[test]
fn the_moment_shapes_of_the_taper_check_peak_at_one_and_follow_the_load_case() {
    for load in ["udl_ss", "udl", "point_mid", "cantilever_udl", "cantilever_point", "two_span_udl", "two_span_point"] {
        let peak = (0..=100).map(|i| moment_shape(load, 0.3, f64::from(i) / 100.0).unwrap()).fold(0.0_f64, f64::max);
        assert!((peak - 1.0).abs() < 1e-12, "{load} peaks at one");
    }
    assert!((moment_shape("udl_ss", 0.0, 0.25).unwrap() - 0.75).abs() < 1e-12 && (moment_shape("point_mid", 0.0, 0.25).unwrap() - 0.5).abs() < 1e-12);
    assert!((moment_shape("cantilever_udl", 0.0, 0.5).unwrap() - 0.25).abs() < 1e-12 && (moment_shape("cantilever_point", 0.0, 0.5).unwrap() - 0.5).abs() < 1e-12);
    assert!((moment_shape("udl_ss", 0.5, 0.25).unwrap() - 0.625).abs() < 1e-12 && moment_shape("two_span", 0.0, 0.5).is_none());
    let signed = |load: &str, share: f64, xi: f64| moment_signed(load, share, xi).unwrap();
    assert!((signed("two_span_udl", 0.0, 0.5) - 1.0).abs() < 1e-12, "the interior support carries the peak (q L²/8)");
    assert!((signed("two_span_udl", 0.0, 0.1875) + 0.5625).abs() < 1e-12, "the field moment 9/128 q L² is 9/16 of the support moment and turns against it");
    assert!(signed("two_span_udl", 0.0, 0.375).abs() < 1e-12 && signed("two_span_udl", 0.0, 0.0).abs() < 1e-12, "no moment at the end support and at x = 3/4 L");
    assert!((signed("two_span_point", 0.0, 0.5) - 1.0).abs() < 1e-12 && (signed("two_span_point", 0.0, 0.25) + 5.0 / 6.0).abs() < 1e-12, "3 P L / 16 at the support, 5 P L / 32 under the load: 5/6 of it");
    assert!((signed("two_span_udl", 0.4, 0.25) - (0.6 * (4.0 * 0.25 - 3.0 * 0.5) + 0.4 * (16.0 / 3.0 * 0.0 - 5.0 / 3.0 * 0.5))).abs() < 1e-12, "a mixture weights both shapes by the point share");
    assert!((0..=100).all(|i| signed("udl_ss", 0.3, f64::from(i) / 100.0) >= 0.0 && signed("cantilever_udl", 0.0, f64::from(i) / 100.0) >= 0.0), "single members keep the sign of their peak");
}

#[test]
fn the_taper_moment_line_is_the_shared_mechanics_line_for_any_member_configuration() {
    use semio_s_plugin_norm_mechanics::{BeamSupports, MomentLine as Statics, PointLoad, SpanLoad};
    let three = MomentLine::build(BeamSupports::Pinned, 3, 6000.0, 0.0).unwrap();
    assert!((three.span - 18_000.0).abs() < 1e-9, "three spans of 6 m");
    let near = |a: f64, b: f64, what: &str| assert!((a - b).abs() < 1e-9, "{what}: {a} vs {b}");
    near(three.signed(1.0 / 3.0), 1.0, "support B carries the peak q L²/10");
    near(three.signed(0.4 / 3.0), -0.8, "end field 0.08 q L² against 0.1 q L²");
    near(three.signed(0.5), -0.25, "middle field 0.025 q L² against 0.1 q L²");
    near(three.signed(0.0), 0.0, "end support");
    let tip = MomentLine::build(BeamSupports::Cantilever, 1, 3000.0, 1.0).unwrap();
    near(tip.signed(0.0), 1.0, "fixed end");
    near(tip.signed(0.5), 0.5, "tip load: linear");
    near(tip.signed(1.0), 0.0, "free end");
    let mixed = MomentLine::of("two_span_udl", 0.4, 6000.0).unwrap();
    let (l, q, p) = (6000.0, 0.6 * 8.0 / (6000.0 * 6000.0), 0.4 * 16.0 / (3.0 * 6000.0));
    let load = SpanLoad { udl: q, points: vec![PointLoad { at: l / 2.0, force: p }] };
    let direct = Statics::continuous(&[l, l], &[load.clone(), load], false).unwrap();
    for i in 0..=24 {
        let xi = f64::from(i) / 24.0;
        near(mixed.signed(xi), direct.moment_at(xi * 2.0 * l) / direct.support_moment(1), "mixture equals the mechanics line");
    }
    assert!(MomentLine::of("two_span", 0.0, 6000.0).is_none() && MomentLine::of("udl_ss", 0.0, 0.0).is_none() && MomentLine::build(BeamSupports::Pinned, 0, 1.0, 0.0).is_none());
}

fn continuous_case(load: &str, share: f64) -> Case {
    let alpha = 0.05_f64.atan().to_degrees();
    Case::new("tp", AnnexChoice::En).with_s("class", "GL28h").with_s("side", "tension").with_s("duration", "medium").with_n("sc", 1.0).with_n("b", 160.0).with_n("h", 800.0).with_n("alpha_deg", alpha).with_n("m_d", 100e6).with_n("l", 6000.0).with_s("load_case", load).with_n("point_share", share)
}

#[test]
fn a_continuous_tapered_beam_is_checked_on_both_edge_states_along_both_spans() {
    let udl = taper_probe(&continuous_case("two_span_udl", 0.0)).unwrap();
    assert!((udl.line.span - 12_000.0).abs() < 1e-9, "the sweep runs over both spans of 6 m");
    let g = udl.governing();
    let sigma_support = 6.0 * 1.0e8 / (160.0 * 1100.0 * 1100.0);
    assert!((g.x - 6000.0).abs() < 1e-3 && g.tension && (g.h - 1100.0).abs() < 1e-9 && (g.m - 1.0e8).abs() < 1.0, "hogging support: M = q L²/8 = M_d at h = 800 + 6000 · 0.05 = 1100 mm, tapered edge in tension: {g:?}");
    assert!((g.sigma - sigma_support).abs() < 1e-9 && (sigma_support - 3.099_173_553_7).abs() < 1e-9, "σ = 6 M / (b h²) = 6e8 / (160 · 1.21e6)");
    assert!((g.k - 0.875_71).abs() < 1e-4 && (g.tapered - 0.197_494).abs() < 1e-6, "tension edge k_m,α (6.38) 0.87571: {g:?}");
    let field = udl.section(2250.0);
    assert!(!field.tension && (field.m - 0.5625e8).abs() < 1.0 && (field.h - 912.5).abs() < 1e-9, "x = 3/8 L: M = 9/16 M_d against the support, the tapered edge is in compression: {field:?}");
    assert!((field.sigma - 6.0 * 0.5625e8 / (160.0 * 912.5 * 912.5)).abs() < 1e-9 && (field.k - 0.965_88).abs() < 1e-4 && (field.tapered - 0.146_361).abs() < 1e-6, "compression edge k_m,α (6.39) 0.96588: {field:?}");
    assert!(field.tapered < g.tapered, "under a uniform load the support governs");
    let point = taper_probe(&continuous_case("two_span_point", 0.0)).unwrap().governing();
    assert!((point.x - 3000.0).abs() < 1e-3 && !point.tension && (point.m - 5.0 / 6.0 * 1.0e8).abs() < 1.0 && (point.h - 950.0).abs() < 1e-9, "under midspan loads the field section below the load governs, in compression: {point:?}");
    assert!((point.sigma - 3.462_603_878_1).abs() < 1e-8 && (point.tapered - 0.200_051).abs() < 1e-6 && point.tapered > g.tapered, "σ = 6 · 5/6 M / (b · 950²) = 3.4626 N/mm² on the deeper compression edge exceeds the support: {point:?}");
    let outcomes = evaluate_outcomes("T-30", &continuous_case("two_span_point", 0.0)).unwrap();
    let get = |key: &str| outcomes.iter().find(|o| o.key == key).unwrap_or_else(|| panic!("outcome {key}"));
    assert!((get("governing_section").capacity - 3.0).abs() < 1e-9 && get("tapered_edge").steps.contains("compression edge"), "{:?}", get("tapered_edge").steps);
    let cases = [continuous_case("two_span_udl", 0.0), continuous_case("two_span_point", 0.0), continuous_case("two_span_udl", 0.4)];
    let rows: Vec<serde_json::Value> = cases.iter().map(|c| serde_json::json!({"entry": "T-30-governing", "case": {"annex": "En", "num": c.num, "text": c.text}, "at": []})).collect();
    let oracle = oracle_mode("--curves", &serde_json::json!({"cases": rows}));
    for (c, expected) in cases.iter().zip(oracle["cases"].as_array().unwrap()) {
        let (x, ratio) = (expected["values"][0].as_f64().unwrap(), expected["values"][1].as_f64().unwrap());
        let g = taper_probe(c).unwrap().governing();
        assert!((g.x - x).abs() <= 0.05 && (g.tapered - ratio).abs() <= 1e-9 * ratio.max(1.0), "rust x={} u={} numpy x={x} u={ratio}", g.x, g.tapered);
    }
    let at = [0.0, 1500.0, 2250.0, 3000.0, 4500.0, 6000.0, 9000.0, 11_000.0, 12_000.0];
    for c in &cases {
        let curve = oracle_mode("--curves", &serde_json::json!({"cases": [{"entry": "T-30", "case": {"annex": "En", "num": c.num, "text": c.text}, "at": at}]}));
        let probe = taper_probe(c).unwrap();
        for (x, v) in at.iter().zip(curve["cases"][0]["values"].as_array().unwrap()) {
            let py = v.as_f64().unwrap();
            assert!((probe.section(*x).tapered - py).abs() <= 1e-9 * py.abs().max(1.0), "section at {x}: rust {} numpy {py}", probe.section(*x).tapered);
        }
    }
}

#[test]
fn forces_without_loads_are_resolved_to_a_line_and_point_load_by_statics_or_refused() {
    use semio_s_artifact_norm_en1995_model::SupportType;
    let mut member = En1995Snapshot::compliant_building_beam().members.remove(0);
    for (support, name, span, q, p) in [(SupportType::SimplySupported, "simply_supported", 4.0, 10_000.0, 8_000.0), (SupportType::Cantilever, "cantilever", 3.0, 5_000.0, 6_000.0), (SupportType::ContinuousTwoSpan, "continuous_two_span", 4.5, 4_000.0, 10_000.0)] {
        member.support = support;
        member.span_m = span;
        let (wq, wp) = Profile::of(&member, q, p).unwrap().moment_weights();
        let (m, v) = match support {
            SupportType::SimplySupported => (wq + wp, q * span / 2.0 + p / 2.0),
            SupportType::Cantilever => (wq + wp, q * span + p),
            SupportType::ContinuousTwoSpan => (wq + wp, 5.0 * q * span / 8.0 + 11.0 * p / 16.0),
        };
        let back = Profile::from_forces(&member, m, v).unwrap_or_else(|| panic!("{name}"));
        let (bq, bp) = back.moment_weights();
        assert!((bq - wq).abs() < 1e-6 * m && (bp - wp).abs() < 1e-6 * m, "{name}: the line load {q} N/m and the point load {p} N are recovered");
        let oracle = oracle_mode("--curves", &serde_json::json!({"cases": [{"entry": "T-30-statics", "case": {"annex": "En", "num": {"l": span, "m_d": m, "v_d": v}, "text": {"support": name}}, "at": []}]}));
        let values: Vec<f64> = oracle["cases"][0]["values"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
        assert!((values[0] - q).abs() < 1e-6 * q && (values[1] - p).abs() < 1e-6 * p && (values[2] - bp / (bq + bp)).abs() < 1e-9, "{name}: numpy.linalg.solve gives {values:?}");
    }
    member.support = SupportType::SimplySupported;
    member.span_m = 4.0;
    assert!((Profile::from_forces(&member, 28_000.0, 24_000.0).unwrap().moment_weights().0 - 20_000.0).abs() < 1e-6, "q L² / 8 = 20 kN m of 10 kN/m over 4 m");
    assert!(Profile::from_forces(&member, 20_000.0, 0.0).is_none(), "a moment without shear force needs a negative line load");
    assert!(Profile::from_forces(&member, 20_000.0, 50_000.0).is_none(), "V L above 4 M needs a negative point load");
    assert!(Profile::from_forces(&member, 0.0, 10_000.0).is_none(), "no moment, no line");
}

#[test]
fn the_entities_of_a_continuous_or_force_only_member_are_swept_or_refused_with_a_localized_reason() {
    use semio_s_artifact_norm_en1995_model::{ActionKind, CharacteristicAction, ImposedCategory, LoadDurationClass, SupportType};
    let forces = |m: f64, v: f64| CharacteristicAction { id: "g".into(), kind: ActionKind::Permanent, category: ImposedCategory::NotImposed, load_duration: LoadDurationClass::Permanent, q_line_n_per_m: 0.0, f_point_n: 0.0, m_k_nm: m, v_k_n: v, n_k_n: 0.0, n_t_k_n: 0.0, f_c90_k_n: 0.0 };
    let loads = |q: f64, f: f64| CharacteristicAction { q_line_n_per_m: q, f_point_n: f, ..forces(0.0, 0.0) };
    let check_of = |d: &En1995Snapshot, entity: &str, prefix: &str| {
        let report = evaluate_structure(d);
        report.checks.iter().find(|c| c.subject.entity_id == entity && c.id.starts_with(prefix)).cloned().unwrap_or_else(|| panic!("{entity} {prefix}"))
    };
    let row_of = |d: &En1995Snapshot, index: usize| drawing_row(d, "tapers", index).expect("taper row").0;
    let mut d = entity_demo_document();
    d.members[0].support = SupportType::ContinuousTwoSpan;
    d.members[0].span_m = 4.5;
    d.members[0].actions = vec![loads(4_000.0, 10_000.0)];
    let row = row_of(&d, 1);
    let text = |key: &str| row.texts.iter().find(|t| t.id == key).map(|t| t.value.clone());
    let number = |key: &str| row.numbers.iter().find(|n| n.id == key).map(|n| n.value).expect(key);
    assert_eq!(text("load_case").as_deref(), Some("two_span_point"), "two spans under a line and midspan loads: the interior support carries the line");
    let (wq, wp) = Profile::of(&d.members[0], 4_000.0, 10_000.0).unwrap().moment_weights();
    assert!((number("point_share") - wp / (wq + wp)).abs() < 1e-12 && (number("l") - 4500.0).abs() < 1e-9);
    let swept = check_of(&d, "taper-2", "en1995.t-30.governing_section");
    assert_ne!(swept.status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
    d.members[0].actions = vec![forces(18_562.5, 18_125.0)];
    let derived = row_of(&d, 1);
    assert_eq!(derived.texts.iter().find(|t| t.id == "load_case").map(|t| t.value.as_str()), Some("two_span_udl"), "forces without loads: q = 4 kN/m and P = 10 kN by statics");
    d.members[0].actions = vec![forces(20_000.0, 0.0)];
    let refused = check_of(&d, "taper-2", "en1995.t-30");
    assert_eq!(refused.status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable, "forces that are no line plus midspan load are refused, never swept at a constant moment");
    assert!(refused.explanation.en.contains("moment line") && refused.explanation.de.contains("Momentenlinie"), "{:?}", refused.explanation);
    let apex = check_of(&d, "taper-1", "en1995.t-31");
    assert_eq!(apex.status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable, "the apex moment needs the moment line as well");
    d.members[0].support = SupportType::Cantilever;
    d.members[0].span_m = 3.0;
    d.members[0].actions = vec![loads(5_000.0, 0.0)];
    let cantilever = check_of(&d, "taper-1", "en1995.t-31.apex_bending");
    d.members[0].support = SupportType::SimplySupported;
    d.members[0].actions = vec![loads(5_000.0, 0.0)];
    let simple = check_of(&d, "taper-1", "en1995.t-31.apex_bending");
    assert!(simple.utilization > 0.0 && (cantilever.utilization - simple.utilization).abs() < 1e-9 * simple.utilization, "the apex of a cantilever with q = 5 kN/m over 3 m carries M(l/2) = q (l/2)² / 2 = q l² / 8 = 5.625 kN m, exactly the midspan moment of the simply supported beam with the same load");
}
