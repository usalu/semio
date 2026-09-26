use super::*;
use crate::app_surface::{apply_remedy_edit, get_value_at_path, parse_path};
use crate::document::{CheckReport, CheckStatus};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn report_of(snap: &En1995Snapshot) -> CheckReport {
    evaluate_structure(snap.annex, &snap.members, &snap.connections)
}

fn bundled_examples() -> Vec<(String, En1995Snapshot)> {
    crate::examples()
        .into_iter()
        .map(|example| {
            let snap = crate::artifact_schema::snapshot::decode_en1995_dsl(&example.document()).unwrap_or_else(|e| panic!("{}: {e}", example.id()));
            (example.id().to_string(), snap)
        })
        .collect()
}

fn check<'a>(report: &'a CheckReport, id: &str) -> &'a crate::document::CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id} in {:?}", report.checks.iter().map(|c| c.id.clone()).collect::<Vec<_>>()))
}

fn apply_remedy(snap: &En1995Snapshot, report: &CheckReport, check_id: &str, remedy_index: usize) -> En1995Snapshot {
    let mut tree = dsl::ToValue::to_value(snap);
    apply_remedy_edit(report, check_id, remedy_index, 0, &mut tree).unwrap_or_else(|f| panic!("{check_id}[{remedy_index}]: {f:?}"));
    dsl::FromValue::from_value(tree).unwrap_or_else(|e| panic!("{check_id}[{remedy_index}] decode: {e}"))
}

#[semio_framework_async_macros::async_test]
async fn k_mod_sc1_permanent() {
    assert!((k_mod(ServiceClass::Sc1, LoadDuration::Permanent) - 0.6).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn k_def_depends_on_service_class_only() {
    assert!((k_def(ServiceClass::Sc1) - 0.60).abs() < 1e-9);
    assert!((k_def(ServiceClass::Sc2) - 0.80).abs() < 1e-9);
    assert!((k_def(ServiceClass::Sc3) - 2.00).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn strength_class_c24_and_gl28h_tables() {
    let c24 = properties_for_class("C24").unwrap();
    assert!((c24.f_m_k / 1e6 - 24.0).abs() < 1e-9);
    assert!((c24.f_v_k / 1e6 - 4.0).abs() < 1e-9);
    let gl = properties_for_class("GL28h").unwrap();
    assert!((gl.f_m_k / 1e6 - 28.0).abs() < 1e-9);
    assert_eq!(gl.product, crate::TimberProduct::Glulam);
}

#[semio_framework_async_macros::async_test]
async fn k_cr_diverges_between_en_and_de_for_c24() {
    let f_v = 4.0e6;
    let k_cr_en = AnnexParams::en().k_cr(f_v);
    let k_cr_de = AnnexParams::de().k_cr(f_v);
    assert!((k_cr_en - 0.67).abs() < 1e-9);
    assert!((k_cr_de - 0.625).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn shear_utilization_diverges_between_annexes_for_c24() {
    let mut m = En1995Snapshot::compliant_building_beam().members.remove(0);
    m.strength_class = "C24".into();
    m.b_m = 0.20;
    m.h_m = 0.30;
    m.actions = vec![crate::CharacteristicAction {
        id: "g".into(),
        kind: "permanent".into(),
        category: "".into(),
        load_duration: "medium".into(),
        q_line_n_per_m: 0.0,
        f_point_n: 0.0,
        m_k_nm: 1.0,
        v_k_n: 15_000.0,
        n_k_n: 0.0,
        n_t_k_n: 0.0,
        f_c90_k_n: 0.0,
    }];
    let en = evaluate_structure(AnnexChoice::En, &[m.clone()], &[]);
    let de = evaluate_structure(AnnexChoice::De, &[m], &[]);
    let shear_en = en.checks.iter().find(|c| c.id.contains("shear")).unwrap();
    let shear_de = de.checks.iter().find(|c| c.id.contains("shear")).unwrap();
    assert!((shear_en.utilization - shear_de.utilization).abs() > 1e-3);
    assert!(shear_de.utilization > shear_en.utilization);
    let k_cr_ratio = AnnexParams::en().k_cr(4.0e6) / AnnexParams::de().k_cr(4.0e6);
    assert!((shear_de.utilization / shear_en.utilization - k_cr_ratio).abs() < 1e-6, "shear utilization must scale with 1/k_cr: de={} en={} ratio={}", shear_de.utilization, shear_en.utilization, k_cr_ratio);
}

#[semio_framework_async_macros::async_test]
async fn column_buckling_kc_from_lambda_rel() {
    let kc = k_c_buckling(1.2);
    assert!(kc < 1.0 && kc > 0.3);
}

#[semio_framework_async_macros::async_test]
async fn johansen_mode_capacity_positive() {
    let fh = f_h_k(350.0, 0.012, "bolt");
    let my = m_y_k(400e6, 0.012);
    let f = johansen_single_shear_f_v_rk(0.2, 0.2, 0.012, fh, fh, my, 0.0);
    assert!(f > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn yield_moment_is_si_for_an_m12_bolt() {
    let expected_nmm = 0.3 * 400.0 * 12.0_f64.powf(2.6);
    assert!((m_y_k(400e6, 0.012) - expected_nmm / 1000.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn fire_def_r60() {
    let beta_n = AnnexParams::en().beta_n(crate::TimberProduct::Glulam);
    let d = d_ef(3600.0, beta_n);
    assert!((d * 1000.0 - 49.0).abs() < 0.5);
}

#[semio_framework_async_macros::async_test]
async fn compliant_examples_pass() {
    for snap in [En1995Snapshot::compliant_building_beam(), En1995Snapshot::compliant_bridge()] {
        let report = report_of(&snap);
        assert!(report.complies(), "compliant example must pass, fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
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
        let root = dsl::ToValue::to_value(&snap);
        for annex in [AnnexChoice::En, AnnexChoice::De] {
            let report = evaluate_structure(annex, &snap.members, &snap.connections);
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
        let en = evaluate_structure(AnnexChoice::En, &snap.members, &snap.connections);
        let de = evaluate_structure(AnnexChoice::De, &snap.members, &snap.connections);
        let differs = en.checks.iter().zip(&de.checks).any(|(a, b)| a.id == b.id && (a.utilization - b.utilization).abs() > 1e-9);
        assert!(differs, "{name}: EN and DE must diverge on at least one check");
    }
}

fn leaf_paths(prefix: &str, value: &dsl::DslValue, out: &mut Vec<String>) {
    match value {
        dsl::DslValue::Object(fields) => {
            for (key, child) in fields {
                let path = if prefix.is_empty() { key.clone() } else { format!("{prefix}.{key}") };
                leaf_paths(&path, child, out);
            }
        }
        dsl::DslValue::Array(items) => {
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
    for snap in [En1995Snapshot::default(), En1995Snapshot::noncompliant_building(), En1995Snapshot::compliant_bridge()] {
        leaf_paths("", &dsl::ToValue::to_value(&snap), &mut leaves);
    }
    assert!(leaves.len() > 60, "expected every member, action and connection leaf, got {}", leaves.len());
    for path in &leaves {
        let meta = crate::field_meta::en1995_field_meta(path).unwrap_or_else(|| panic!("{path} has no field meta"));
        assert!(!meta.label_en.trim().is_empty() && !meta.label_de.trim().is_empty(), "{path}");
        assert_ne!(meta.label_en, meta.label_de, "{path} must carry a German label distinct from English");
        let parent = &path[..path.rfind(['.', '[']).unwrap_or(0)];
        if !parent.is_empty() {
            let parent_meta = crate::field_meta::en1995_field_meta(parent);
            assert!(parent_meta.map_or(true, |p| p.label_en != meta.label_en), "{path} resolves only to its parent's meta '{}'", meta.label_en);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn steel_plate_changes_capacity_and_can_make_the_check_fail() {
    let mut c = En1995Snapshot::compliant_building_beam().connections.remove(0);
    c.t1_m = 0.04;
    c.t2_m = 0.04;
    let annex = AnnexChoice::De;
    let run = |c: &crate::TimberConnection| evaluate_structure(annex, &[], std::slice::from_ref(c));
    let timber = run(&c);
    let timber_check = check(&timber, "en1995.8.2.2.johansen.conn-C1");
    let scale = 0.99 / timber_check.utilization;
    for action in &mut c.actions {
        action.f_k_n *= scale;
    }
    let timber = run(&c);
    let timber_check = check(&timber, "en1995.8.2.2.johansen.conn-C1");
    assert!((timber_check.utilization - 0.99).abs() < 1e-9 && timber_check.status == CheckStatus::Pass);
    let mut thick = c.clone();
    thick.steel_plate = true;
    thick.steel_plate_thickness_m = c.diameter_m;
    let thick_report = run(&thick);
    assert!(thick_report.checks.iter().all(|x| !x.id.starts_with("en1995.8.2.2.")), "steel plate must switch to clause 8.2.3");
    let thick_check = check(&thick_report, "en1995.8.2.3.johansen.conn-C1");
    assert!(thick_check.limit.value > timber_check.limit.value * 1.05, "thick plate F_v,Rd {} must exceed timber-timber {}", thick_check.limit.value, timber_check.limit.value);
    assert_eq!(thick_check.status, CheckStatus::Pass);
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn oracle_script() -> PathBuf {
    family_any_dir().join("🔮️oracles").join("🐍️evaluate.py")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema").join("📸️snapshot").join("🔣️.json")
}

fn python_oracle(input: &str, label: &str) -> serde_json::Value {
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
        let status = Command::new("python3")
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
                if s.starts_with("GL") { "C24".into() } else { "GL28h".into() }
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
                    "A" => "C".into(),
                    "C" => "E".into(),
                    "" => "A".into(),
                    _ => "A".into(),
                }
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
fn unknown_strength_class_fails_with_one_of_options() {
    let mut snap = En1995Snapshot::compliant_building_beam();
    snap.members[0].strength_class = "MISSING".into();
    let report = report_of(&snap);
    let fail = report.checks.iter().find(|c| c.id.contains("material.unknown") && c.status == CheckStatus::Fail).expect("member material Fail");
    assert!(fail.remedies.iter().any(|r| !r.options.is_empty()));

    let mut snap = En1995Snapshot::compliant_building_beam();
    snap.connections[0].strength_class = "MISSING".into();
    let report = report_of(&snap);
    let fail = report.checks.iter().find(|c| c.id.contains("conn.material") && c.status == CheckStatus::Fail).expect("connection material Fail");
    assert!(fail.remedies.iter().any(|r| !r.options.is_empty()));
}

#[test]
fn reference_table_k_mod_cell_equals_evaluated_modification_factor() {
    use crate::app_surface::CatalogueCell;
    use crate::editor::en1995::panels::catalogue::reference_tables;
    use semio_framework_plugin::Locale;
    let snap = En1995Snapshot::compliant_building_beam();
    let km_table = reference_tables(Locale::En).into_iter().find(|t| t.id == "k-mod").expect("k-mod table");
    let sc1 = km_table.rows.iter().find(|r| r.id == "sc1").expect("sc1");
    let CatalogueCell::Number { value: expected, .. } = sc1.cells[3] else {
        panic!("k-mod sc1 medium cell must be numeric");
    };
    let member = &snap.members[0];
    let combos = enumerate_combos(member);
    let uls = governing_uls(&combos).expect("uls");
    let km = k_mod(parse_service_class(member.service_class), uls.duration);
    assert!((km - expected).abs() < 1e-9, "evaluate k_mod={km} vs table cell={expected}");
}

#[test]
fn lateral_restraint_spacing_changes_bending_utilization() {
    let mut snap = En1995Snapshot::compliant_building_beam();
    snap.members[0].m_crit_nm = 25_000.0;
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
fn sls_frequent_uses_psi1_and_de_snow_high_diverges() {
    let (_, psi1_a, _) = psi_factors("imposed", "A");
    let (_, psi1_e, _) = psi_factors("imposed", "E");
    let (_, psi1_snow, _) = psi_factors("snow", "");
    let (_, psi1_snow_high, _) = psi_factors("snow_high", "");
    assert!((psi1_a - 0.5).abs() < 1e-12);
    assert!((psi1_e - 0.9).abs() < 1e-12);
    assert!((psi1_snow - 0.2).abs() < 1e-12);
    assert!((psi1_snow_high - 0.5).abs() < 1e-12, "DE NA snow >1000 m ψ₁ must diverge from EN snow ψ₁");

    let mut snap = En1995Snapshot::compliant_building_beam();
    let combos = enumerate_combos(&snap.members[0]);
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

    let report = evaluate_structure(AnnexChoice::De, &snap.members, &snap.connections);
    let wfreq = report.checks.iter().find(|c| c.id.contains("7.2.wfreq")).expect("SLS frequent deflection check");
    assert!(wfreq.computed.value > 0.0);
    let id = wfreq.id.clone();
    let util_a = wfreq.utilization;

    snap.members[0].actions.iter_mut().find(|a| a.kind == "imposed").expect("imposed").category = "E".into();
    let util_e = check(&evaluate_structure(AnnexChoice::De, &snap.members, &snap.connections), &id).utilization;
    assert!((util_e - util_a).abs() > 1e-9, "ψ₁ category A→E must move frequent utilization");

    let mut snow = snap.members[0].clone();
    let var_idx = snow
        .actions
        .iter()
        .position(|a| a.kind != "permanent" && a.kind != "accidental")
        .expect("variable");
    snow.actions[var_idx].kind = "snow".into();
    snow.actions[var_idx].category = "".into();
    let q_snow = enumerate_combos(&snow)
        .into_iter()
        .filter(|c| c.kind == ComboKind::SlsFrequent)
        .map(|c| c.q_ed_line.abs())
        .fold(0.0_f64, f64::max);
    snow.actions[var_idx].kind = "snow_high".into();
    snow.actions[var_idx].category = "high".into();
    let q_high = enumerate_combos(&snow)
        .into_iter()
        .filter(|c| c.kind == ComboKind::SlsFrequent)
        .map(|c| c.q_ed_line.abs())
        .fold(0.0_f64, f64::max);
    assert!((q_high - q_snow).abs() > 1e-6, "DE snow_high ψ₁=0.5 must diverge from snow ψ₁=0.2 ({q_high} vs {q_snow})");
}

