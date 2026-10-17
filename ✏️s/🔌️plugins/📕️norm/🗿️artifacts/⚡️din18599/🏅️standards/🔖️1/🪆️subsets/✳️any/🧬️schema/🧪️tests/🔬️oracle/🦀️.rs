//! 🐍️ Python balance oracle + third-party JSON-schema validation.

use semio_framework_test_fixtures::fixture_path;
use semio_s_artifact_norm_din18599_checks::evaluate_document;
use semio_s_artifact_norm_din18599_io::text::snapshot::encode_din18599_snapshot_json;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use std::path::PathBuf;
use std::process::Command;

fn ticket_generated() -> PathBuf {
    std::env::temp_dir().join("semio-norm-din18599-oracle")
}

fn family_any_dir() -> PathBuf {
    fixture_path!("../../..")
}

#[test]
fn json_schema_validates_default_and_noncompliant_snapshots() {
    let schema = family_any_dir().join("🧬️schema/📸️snapshot/🔣️.json");
    assert!(schema.exists(), "{}", schema.display());
    let tmp = ticket_generated();
    let _ = std::fs::create_dir_all(&tmp);
    for (name, doc) in [
        ("default", Din18599Snapshot::default()),
        ("noncompliant", semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house()),
        ("cooled", semio_s_artifact_norm_din18599_model::subjects::cooled_office_building()),
        ("two_zone", semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house()),
    ] {
        let snap = tmp.join(format!("{name}.snap.json"));
        std::fs::write(&snap, encode_din18599_snapshot_json(&doc)).expect("write snap");
        let out = Command::new("python3")
            .arg("-c")
            .arg("import json,sys,jsonschema; s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2])); jsonschema.validate(instance=i, schema=s); print('ok')")
            .arg(&schema)
            .arg(&snap)
            .output()
            .expect("python jsonschema");
        assert!(out.status.success(), "jsonschema {name}: {}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    }
}

#[test]
fn python_oracle_matches_evaluate_within_half_percent() {
    let oracle = family_any_dir().join("🔮️oracles/⚖️compliance/🐍️.py");
    assert!(oracle.exists(), "{}", oracle.display());
    let tmp = ticket_generated();
    let _ = std::fs::create_dir_all(&tmp);
    for (name, doc) in [
        ("compliant", semio_s_artifact_norm_din18599_model::subjects::compliant_detached_house()),
        ("noncompliant", semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house()),
        ("two_zone", semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house()),
        ("cooled", semio_s_artifact_norm_din18599_model::subjects::cooled_office_building()),
    ] {
        let snap = tmp.join(format!("{name}.oracle.snap.json"));
        let report = evaluate_document(&doc);
        let rep = tmp.join(format!("{name}.oracle.report.json"));
        std::fs::write(&snap, encode_din18599_snapshot_json(&doc)).expect("snap");
        std::fs::write(&rep, serde_json::to_string_pretty(&report).expect("report")).expect("rep");
        let out = Command::new("python3").arg(&oracle).arg(&snap).arg(&rep).output().expect("run oracle");
        assert!(out.status.success(), "oracle {name}: {}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        let body = String::from_utf8_lossy(&out.stdout);
        let v: serde_json::Value = serde_json::from_str(body.trim()).expect("oracle json");
        assert_eq!(v["ok"], true, "oracle not ok for {name}: {body}");
    }
}
