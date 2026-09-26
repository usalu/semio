//! 🐍️ Python oracle + JSON-schema validation for EN 1996 examples.

use crate::artifact_schema::evaluate_building;
use crate::En1996Snapshot;
use std::process::Command;

use std::path::PathBuf;

fn write_snap_json(doc: &En1996Snapshot, path: &PathBuf) {
    let json = serde_json::to_string_pretty(doc).expect("serde snapshot");
    std::fs::write(path, json).expect("write snap");
}

fn write_report_json(doc: &En1996Snapshot, path: &PathBuf) {
    let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.storeys, &doc.walls);
    let json = serde_json::to_string_pretty(&report).expect("serde report");
    std::fs::write(path, json).expect("write report");
}

#[test]
fn python_oracle_matches_compliant_and_noncompliant() {
    let oracle = {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️evaluate-en1996-1/🐍️.py");
        p
    };
    assert!(oracle.exists(), "missing {}", oracle.display());
    let tmp = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS/🗑️generated/en1996");
    let _ = std::fs::create_dir_all(&tmp);
    for (name, doc) in [
        ("compliant", En1996Snapshot::compliant_clay_wall()),
        ("noncompliant", En1996Snapshot::noncompliant_multi_fail()),
    ] {
        let snap = tmp.join(format!("{name}.snap.json"));
        let rep = tmp.join(format!("{name}.report.json"));
        write_snap_json(&doc, &snap);
        write_report_json(&doc, &rep);
        let out = Command::new("python3")
            .arg(&oracle)
            .arg(&snap)
            .arg(&rep)
            .output()
            .expect("run python oracle");
        assert!(out.status.success(), "oracle failed for {name}: stderr={} stdout={}", String::from_utf8_lossy(&out.stderr), String::from_utf8_lossy(&out.stdout));
        let body = String::from_utf8_lossy(&out.stdout);
        let v: serde_json::Value = serde_json::from_str(&body).expect("oracle json");
        assert_eq!(v["ok"], true, "oracle not ok for {name}: {body}");
    }
}

#[test]
fn json_schema_validates_example_snapshots() {
    let schema = {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json");
        p
    };
    assert!(schema.exists());
    let tmp = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS/🗑️generated/en1996");
    let _ = std::fs::create_dir_all(&tmp);
    let snap = tmp.join("schema-check.snap.json");
    write_snap_json(&En1996Snapshot::compliant_clay_wall(), &snap);
    let py = format!(
        "import json,sys\nfrom pathlib import Path\nimport jsonschema\nschema=json.loads(Path(r'{schema}').read_text())\ndata=json.loads(Path(r'{snap}').read_text())\njsonschema.validate(data, schema)\nprint('ok', end='')\n",
        schema=schema.display(),
        snap=snap.display(),
    );
    let script = tmp.join("validate_schema.py");
    std::fs::write(&script, py).expect("write validator");
    let out = Command::new("python3").arg(&script).output().expect("run validator");
    assert!(out.status.success(), "jsonschema validation failed: stderr={} stdout={}", String::from_utf8_lossy(&out.stderr), String::from_utf8_lossy(&out.stdout));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout.trim(), "ok", "expected stdout exactly ok, got {stdout:?}");
}
