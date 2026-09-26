use super::*;

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
async fn fire_def_r60() {
    let beta_n = AnnexParams::en().beta_n(crate::TimberProduct::Glulam);
    let d = d_ef(3600.0, beta_n);
    // β_n=0.70 mm/min * 60 + 7 mm = 42 + 7 = 49 mm
    assert!((d * 1000.0 - 49.0).abs() < 0.5);
}

#[semio_framework_async_macros::async_test]
async fn compliant_example_passes() {
    let snap = En1995Snapshot::compliant_building_beam();
    let report = evaluate_structure(snap.annex, &snap.members, &snap.connections);
    assert!(report.complies(), "compliant example must pass, fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn noncompliant_example_has_failures_with_remedies() {
    let snap = En1995Snapshot::noncompliant_building();
    let report = evaluate_structure(snap.annex, &snap.members, &snap.connections);
    assert!(!report.complies());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 3);
    for f in fails {
        assert!(!f.remedies.is_empty(), "fail {} missing remedies", f.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn remedy_increasing_height_improves_bending() {
    let snap = En1995Snapshot::noncompliant_building();
    let report = evaluate_structure(snap.annex, &snap.members, &snap.connections);
    let bending = report.failing().find(|c| c.id.contains("bending") && c.id.contains("beam-B2")).expect("bending fail");
    let remedy = bending.remedies.iter().find(|r| r.target.path.contains(".hM") && r.applicable).expect("height remedy");
    let mut fixed = snap.clone();
    fixed.members[0].h_m = remedy.required.value;
    let after = evaluate_structure(fixed.annex, &fixed.members, &fixed.connections);
    let bending_after = after.checks.iter().find(|c| c.id == bending.id).unwrap();
    assert!(bending_after.utilization <= 1.0 + 1e-6 || bending_after.utilization < bending.utilization);
}


use std::path::PathBuf;
use std::process::Command;

fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn oracle_script() -> PathBuf {
    family_any_dir().join("🔮️oracles").join("🐍️evaluate.py")
}

fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema").join("📸️snapshot").join("🔣️.json")
}

#[test]
fn python_oracle_matches_rust_utilizations_within_half_percent() {
    for (label, snap) in [
        ("compliant", En1995Snapshot::compliant_building_beam()),
        ("noncompliant", En1995Snapshot::noncompliant_building()),
    ] {
        let rust = evaluate_structure(snap.annex, &snap.members, &snap.connections);
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
        assert!(compared >= 4, "{label}: expected ≥4 overlapping oracle checks, got {compared}");
    }
}

#[test]
fn example_snapshot_validates_against_json_schema() {
    let schema_path = snapshot_schema_path();
    for snap in [
        En1995Snapshot::compliant_building_beam(),
        En1995Snapshot::noncompliant_building(),
    ] {
        let instance = serde_json::to_string(&snap).expect("instance");
        let tmp = std::env::temp_dir().join(format!("en1995-instance-{}.json", snap.members[0].id));
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
