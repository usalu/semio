use super::*;
use crate::standards::v1::subsets::any::schema::inferences::En1990Inference;
use crate::standards::v1::subsets::any::schema::mutations::change_consequence_class::ChangeConsequenceClass;
use crate::standards::v1::subsets::any::schema::snapshot::text::{parse_dsl, EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT};
use crate::{En1990Mutation, En1990Snapshot};
use protocol::Inference;

use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};

#[semio_framework_async_macros::async_test]
async fn dsl_and_pack_wire_roundtrip_default() {
    let snapshot = En1990Snapshot::default();
    let dsl = en1990_to_dsl_bytes(&snapshot);
    let reparsed = en1990_from_dsl_bytes(&dsl).expect("dsl roundtrip");
    assert_eq!(reparsed, snapshot);
    let packed = en1990_to_pack(&snapshot);
    let unpacked = en1990_from_pack(&packed).expect("pack roundtrip");
    assert_eq!(unpacked, snapshot);
}

struct En1990AnyRoundtrip;

impl SubsetRoundtripSpec for En1990AnyRoundtrip {
    type Snapshot = En1990Snapshot;
    type Mutation = En1990Mutation;
    type Inference = En1990Inference;

    async fn dialect() -> store::os_io::ArtifactDialect {
        store::os_io::ArtifactDialect { artifact_kind: "s.norm.en1990".into(), standard: "1".into(), subset: "*".into() }
    }

    async fn fidelity() -> IoFidelityClass {
        IoFidelityClass::Canonical
    }

    async fn drops() -> &'static [&'static str] {
        &[]
    }

    async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
        if let Some(text) = asset.text {
            parse_dsl(text).map_err(|error| error.to_string())
        } else {
            en1990_from_pack(asset.bytes).map_err(|error| error.to_string())
        }
    }

    async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
        Ok(en1990_to_dsl_bytes(snapshot))
    }

    async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
        en1990_from_dsl_bytes(bytes).map_err(|error| error.to_string())
    }

    async fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
        En1990Inference::infer(snapshot)
    }

    async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
        vec![En1990Mutation::ChangeConsequenceClass(ChangeConsequenceClass { new_consequence_class: snapshot.consequence_class.saturating_add(1).min(3) })]
    }

    async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
        let text = std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()])?;
        let schema = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json");
        let tmp = std::env::temp_dir().join("en1990-validate-instance.json");
        std::fs::write(&tmp, text).map_err(|e| vec![e.to_string()])?;
        let out = std::process::Command::new("python3")
            .arg("-c")
            .arg("import json,sys,jsonschema; s=json.load(open(sys.argv[1])); i=json.load(open(sys.argv[2])); jsonschema.validate(instance=i, schema=s); print('ok')")
            .arg(&schema)
            .arg(&tmp)
            .output()
            .map_err(|e| vec![e.to_string()])?;
        if out.status.success() {
            Ok(())
        } else {
            Err(vec![format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))])
        }
    }

    async fn validate_negative(bytes: &[u8]) -> Result<Vec<String>, String> {
        match Self::validate_payload(bytes).await {
            Ok(()) => Err("expected schema failure".into()),
            Err(errors) => Ok(errors),
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn high_consequence_office_example_parses() {
    let _ = EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT;
    let parsed = parse_dsl(EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT).expect("example text");
    assert_eq!(parsed.consequence_class, 3);
}

#[semio_framework_async_macros::async_test]
async fn parse_en1990_artifact_ts_rejects_malformed_snapshot() {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪parse-en1990-artifact.bun.ts");
    assert!(script.exists(), "missing {}", script.display());
    let out = std::process::Command::new("bun")
        .arg(script.as_os_str())
        .output()
        .expect("bun");
    assert!(out.status.success(), "stdout={} stderr={}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(String::from_utf8_lossy(&out.stdout).contains("parse-en1990-artifact:ok"));
}
