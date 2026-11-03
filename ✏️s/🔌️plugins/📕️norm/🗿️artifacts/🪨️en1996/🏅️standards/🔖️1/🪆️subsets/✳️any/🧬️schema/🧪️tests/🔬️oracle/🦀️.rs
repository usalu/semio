//! 🐍️ JSON-schema validation of the example snapshots with the third-party `jsonschema` package; the snapshot goes through stdin and no file is written.

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use std::ffi::OsStr;

#[test]
fn json_schema_validates_example_snapshots() {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let schema = loop {
        let candidate = dir.join("🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json");
        if candidate.exists() {
            break candidate;
        }
        assert!(dir.pop(), "snapshot schema not found above {}", env!("CARGO_MANIFEST_DIR"));
    };
    let code = "import json,sys\nfrom pathlib import Path\nimport jsonschema\nschema=json.loads(Path(sys.argv[1]).read_text())\njsonschema.validate(json.load(sys.stdin), schema)\nprint('ok', end='')\n";
    for doc in [En1996Snapshot::compliant_clay_wall(), En1996Snapshot::noncompliant_multi_fail(), En1996Snapshot::opening_wall_example(), En1996Snapshot::reinforced_wall_example()] {
        let json = serde_json::to_string(&doc).expect("serde snapshot");
        let out = semio_s_plugin_norm_compliance::python_oracle::run_ok("-c", &[OsStr::new(code), schema.as_os_str()], Some(json.as_bytes()));
        assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "ok");
    }
}
