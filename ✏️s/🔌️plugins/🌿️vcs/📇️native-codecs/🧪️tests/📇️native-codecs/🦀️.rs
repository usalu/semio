//! 🧪 Literal VCS factory identity, typed-codec parity and closed hostile projections.

use semio_framework_os_kernel as store;
use semio_s_plugin_vcs::native_codecs::{native_codec_factory_receipts, NativeVcsCodecIdentityV1};

fn hexadecimal(bytes: [u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn projection(identity: &NativeVcsCodecIdentityV1) -> serde_json::Value {
    serde_json::json!({ "factoryId": identity.factory_id, "kind": identity.artifact_kind, "schema": identity.schema, "extension": identity.extension, "capability": identity.capability, "protocolSha256": hexadecimal(identity.protocol_sha256) })
}

#[semio_framework_async_macros::async_test]
async fn vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    let document_ids: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🌱️artifact-document-id-v1/🔣️.json")).unwrap();
    let document_id = document_ids["cases"].as_array().unwrap().iter().find(|row| row["accepted"] == true).unwrap()["documentId"].as_str().unwrap();
    let receipts = native_codec_factory_receipts().expect("complete inert VCS closure");
    assert_eq!(receipts.len(), fixture["receipts"].as_array().unwrap().len());
    for (receipt, expected) in receipts.into_iter().zip(fixture["receipts"].as_array().unwrap()) {
        let identity = receipt.identity();
        assert_eq!(identity.plugin_id, fixture["pluginId"]);
        assert_eq!(identity.package_id, fixture["packageId"]);
        assert_eq!(identity.package_version, fixture["packageVersion"]);
        assert_eq!(identity.factory_id, expected["factoryId"]);
        assert_eq!(identity.artifact_kind, expected["kind"]);
        assert_eq!(identity.schema, expected["schema"]);
        assert_eq!(identity.extension, expected["extension"]);
        assert_eq!(identity.capability, expected["capability"]);
        assert_ne!(identity.pack_schema_hash, [0; 32]);
        assert_eq!(hexadecimal(identity.protocol_sha256), expected["protocolSha256"]);
        let (codec, genesis) = receipt.into_codec_and_genesis().expect("actual typed factory and genesis match immutable receipt");
        assert_eq!(codec.schema, identity.schema);
        assert_eq!(codec.extension, identity.extension);
        assert_eq!(codec.pack_schema_hash, identity.pack_schema_hash);
        let declared = store::ArtifactCodec::of::<semio_s_artifact_vcs_vcs::VcsSnapshot, semio_s_artifact_vcs_vcs::VcsDemoMutation>(identity.schema);
        assert_eq!(codec.schema, declared.schema);
        assert_eq!(codec.extension, declared.extension);
        assert_eq!(codec.pack_schema_hash, declared.pack_schema_hash);
        assert!(std::ptr::fn_addr_eq(codec.compile_dsl, declared.compile_dsl));
        assert!(std::ptr::fn_addr_eq(codec.print_mirror, declared.print_mirror));
        assert!(std::ptr::fn_addr_eq(codec.edit_text_from_envelope, declared.edit_text_from_envelope));
        assert!(std::ptr::fn_addr_eq(codec.apply_ops_binary, declared.apply_ops_binary));
        let dialect: semio_framework::ArtifactDialect = semio_s_artifact_vcs_vcs::VCS_DIALECT.into();
        let files = genesis(document_id, &dialect).await.expect("package-owned VCS genesis");
        let parsed = store::parse_document_pack::<semio_s_artifact_vcs_vcs::VcsSnapshot, semio_s_artifact_vcs_vcs::VcsDemoMutation>(&files.pack, &files.spr).await.unwrap();
        let envelope = parsed.envelope.into_owners();
        assert_eq!(envelope.id, document_id);
        assert_eq!(envelope.dialect.as_ref(), Some(&dialect));
        assert!(envelope.vcs.edits.is_empty());
        assert!(envelope.vcs.changes.is_empty());
        assert!(envelope.vcs.checkpoints.is_empty());
        assert!(envelope.vcs.alternatives.is_empty());
        let cursor = envelope.cursor.expect("genesis cursor");
        assert!(cursor.applied_edit_ids.is_empty());
        assert!(cursor.redo_edit_ids.is_empty());
        assert!(cursor.checkpoint_id.is_none());
        let hostile = semio_framework::ArtifactDialect { artifact_kind: dialect.artifact_kind.clone(), standard: dialect.standard.clone(), subset: "strict".into() };
        assert!(genesis(document_id, &hostile).await.is_err(), "a package genesis factory must reject a substituted dialect");
        for row in document_ids["cases"].as_array().unwrap().iter().filter(|row| row["accepted"] == false) {
            let hostile_id = row["documentId"].as_str().unwrap();
            assert!(genesis(hostile_id, &dialect).await.is_err(), "VCS genesis admitted hostile document id {}", row["id"]);
        }
    }
}

#[test]
fn vcs_native_receipt_closure_denies_every_hostile_row_including_the_retired_document_kind() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    let identity = native_codec_factory_receipts().expect("complete inert VCS closure")[0].identity();
    let exact = projection(&identity);
    let admits = |package: &str, version: &str, rows: &[serde_json::Value]| {
        package == identity.package_id
            && version == identity.package_version
            && rows.len() == 1
            && rows.iter().all(|row| *row == exact && row["protocolSha256"] != "00".repeat(32))
    };
    assert!(admits(identity.package_id, identity.package_version, std::slice::from_ref(&exact)), "the literal VCS closure must be admitted");
    let mut denied = 0;
    for hostile in fixture["hostile"].as_array().unwrap() {
        let mut package = identity.package_id.to_owned();
        let mut version = identity.package_version.to_owned();
        let mut rows = vec![exact.clone()];
        match hostile.as_str().unwrap() {
            "missing" => rows.clear(),
            "duplicate" => rows.push(exact.clone()),
            "foreign-package" => package = "semio:stdio".to_owned(),
            "wrong-version" => version = "0.2.0".to_owned(),
            "bare-kind" => rows[0]["kind"] = "vcs.vcs".into(),
            "legacy-kind" => rows[0]["kind"] = "vcs.document".into(),
            "legacy-schema" => rows[0]["schema"] = "vcs.document".into(),
            "wrong-extension" => rows[0]["extension"] = "vcsdocument".into(),
            "zero-hash" => rows[0]["protocolSha256"] = "00".repeat(32).into(),
            other => panic!("unhandled VCS hostile row {other}"),
        }
        assert!(!admits(&package, &version, &rows), "hostile VCS row admitted: {hostile}");
        denied += 1;
    }
    assert_eq!(denied, fixture["hostile"].as_array().unwrap().len());
    assert_ne!(identity.artifact_kind, "vcs.document");
    assert_ne!(identity.schema, "vcs.document");
    println!("vcs-native-codec-laws: exact=1 hostile-denied={denied}; no hub catalog activation or client mount");
}
