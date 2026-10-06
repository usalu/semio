/// 🪶️ Independently reads every domain row of one actual exported SQLite file.
fn primary_snapshot_physical_rows(bytes: &[u8]) -> serde_json::Value {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let script = r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));try{const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT IN ('semio_snapshot','sqlite_sequence') ORDER BY name").all();const domainRows={};for(const {name}of tables){if(!/^[a-z_]+$/.test(name))throw Error('unexpected domain table');domainRows[name]=db.query('SELECT * FROM "'+name+'" ORDER BY id').all();}console.log(JSON.stringify({integrity:db.query('PRAGMA integrity_check').all(),foreignKeys:db.query('PRAGMA foreign_key_check').all(),metadata:db.query('SELECT artifact_kind,standard,subset,schema_version,native_encoding FROM semio_snapshot ORDER BY id').all(),domainRows}));}finally{db.close();}"#;
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("real independent SQLite reader");
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    serde_json::from_slice(&result.stdout).unwrap()
}

/// 📸️ Runs ordinary public Binary/Text I/O and preserves the full concrete Snapshot owner.
async fn assert_primary_snapshot_payload<S>(snapshot: S, row: &serde_json::Value)
where S: semio_framework_os_kernel::ArtifactPack + semio_framework_os_kernel::ArtifactDsl + PartialEq + std::fmt::Debug {
    use semio_framework::io::io_mechanism::{io_identify, io_route, io_run};
    use semio_framework::io_schema::{ArtifactDialect, Confidence, IoFidelity, IoPayload, SQLITE_SNAPSHOT};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    packages();
    let native = ArtifactDialect { artifact_kind: row["kind"].as_str().unwrap().into(), standard: row["standard"].as_str().unwrap().into(), subset: row["subset"].as_str().unwrap().into() };
    let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
    let export = io_route(&native, &sqlite, 1).await.unwrap().value;
    let import = io_route(&sqlite, &native, 1).await.unwrap().value;
    assert_eq!(export.fidelity, IoFidelity::Exact);
    assert_eq!(import.fidelity, IoFidelity::Exact);
    for payload in [IoPayload::Binary(snapshot.encode_pack()), IoPayload::Text(snapshot.print_dsl())] {
        let file = io_run(&export, payload.clone()).await.expect("actual public SQLite export").value;
        let IoPayload::Binary(bytes) = &file else { panic!("SQLite file must be binary") };
        assert!(bytes.starts_with(b"SQLite format 3\0"));
        let evidence = primary_snapshot_physical_rows(bytes);
        assert_eq!(evidence["integrity"], law["integrity"]);
        assert_eq!(evidence["foreignKeys"], law["foreignKeys"]);
        assert_eq!(evidence["domainRows"], row["domainRows"], "complete authored domain rows and values");
        let encoding = match &payload { IoPayload::Binary(_) => "binary", IoPayload::Text(_) => "text" };
        assert_eq!(evidence["metadata"], serde_json::json!([{"artifact_kind":native.artifact_kind,"standard":native.standard,"subset":native.subset,"schema_version":law["schemaVersion"],"native_encoding":encoding}]));
        assert_eq!(io_identify(&file).await, vec![(sqlite.clone(), Confidence::High)]);
        let restored = io_run(&import, file).await.expect("actual public SQLite import").value;
        assert_eq!(restored, payload, "complete native wire");
        let decoded = match restored { IoPayload::Binary(bytes) => S::decode_pack(&bytes).unwrap(), IoPayload::Text(text) => S::parse_dsl(&text).unwrap() };
        assert_eq!(decoded, snapshot, "full concrete Snapshot equality");
        eprintln!("[DEBUG] primary-stdio-public-snapshot dialect={} native={} bytes={} complete_domain_rows=true full_owner=true", native.to_coordinate(), encoding, bytes.len());
    }
}

/// 🌳️ The actual public declaration trees agree with the Runtime owner installed by normal builders.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_new_declaration_trees() {
    async fn check<PA: PluginApp>(tree: semio_framework_plugin::app::declarations::ArtifactDeclaration<PA>) {
        use semio_framework::io::io_mechanism::native_snapshot_sqlite_schema;
        for standard in tree.standards {
            for subset in standard.subsets {
                let codec = subset.io.native.codec;
                let declared = codec.snapshot_sqlite.as_ref().expect("public tree semantic SQLite owner");
                assert!(declared.snapshot_type.is_some());
                let installed = semio_framework_os_kernel::document_codec(&codec.schema).await.unwrap().expect("installed public tree document codec");
                assert!(installed.snapshot_sqlite.as_ref().unwrap().identical_to(declared));
                assert_eq!(native_snapshot_sqlite_schema(&subset.dialect.into()).unwrap(), declared.schema.as_ref());
            }
        }
    }
    packages();
    check(semio_s_artifact_stdio_binary::artifact()).await;
    check(semio_s_artifact_stdio_txt::artifact()).await;
}

/// 💾️ Binary's actual authored demo crosses both public native forms and all byte relations.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_binary_public_payload() {
    use semio_framework_os_kernel::ArtifactDsl;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    let row = &law["witnesses"][0];
    let snapshot = semio_s_artifact_stdio_binary::BinarySnapshot::parse_dsl(row["naturalText"].as_str().unwrap()).unwrap();
    assert_eq!(semio_s_artifact_stdio_binary::examples::demo::source().document(), row["naturalText"].as_str().unwrap());
    assert_primary_snapshot_payload(snapshot, row).await;
}

/// 🔤️ Txt's authored demo crosses both public native forms and all line relations.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_txt_public_payload() {
    use semio_framework_os_kernel::ArtifactDsl;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    let row = &law["witnesses"][1];
    let snapshot = semio_s_artifact_stdio_txt::TxtSnapshot::parse_dsl(row["naturalText"].as_str().unwrap()).unwrap();
    assert_eq!(semio_s_artifact_stdio_txt::examples::demo::source().document(), row["naturalText"].as_str().unwrap());
    assert_primary_snapshot_payload(snapshot, row).await;
}

/// 🌍️ The non-editor GeoJSON declaration retains its complete typed Feature owner and conformance.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_geojson_public_payload() {
    use semio_s_artifact_stdio_json::JsonSnapshot;
    use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::snapshot::parse_json_text;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    let row = &law["witnesses"][2];
    let snapshot = JsonSnapshot::from_value(parse_json_text(row["naturalText"].as_str().unwrap()).unwrap());
    let independent: serde_json::Value = serde_json::from_str(row["naturalText"].as_str().unwrap()).unwrap();
    assert_eq!(independent, row["logicalValue"]);
    assert_eq!(snapshot.to_serde_value(), independent);
    assert!(semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::geojson::schema::check_geojson_conformance(&snapshot).iter().all(|diagnostic| !matches!(diagnostic.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)));
    assert_primary_snapshot_payload(snapshot, row).await;
    use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack};
    let native = semio_framework::io_schema::ArtifactDialect { artifact_kind: row["kind"].as_str().unwrap().into(), standard: row["standard"].as_str().unwrap().into(), subset: row["subset"].as_str().unwrap().into() };
    let export = semio_framework::io::io_mechanism::io_route(&native, &semio_framework::io_schema::SQLITE_SNAPSHOT.into(), 1).await.unwrap().value;
    for invalid in row["invalidNaturalTexts"].as_array().unwrap() {
        let invalid = JsonSnapshot::from_value(parse_json_text(invalid.as_str().unwrap()).unwrap());
        for payload in [semio_framework::io_schema::IoPayload::Binary(invalid.encode_pack()), semio_framework::io_schema::IoPayload::Text(invalid.print_dsl())] {
            assert!(semio_framework::io::io_mechanism::io_run(&export, payload).await.is_err(), "valid JSON that violates GeoJSON must be refused by the actual public route");
        }
    }
    eprintln!("[DEBUG] primary-stdio-geojson invalid-native-forms=2 refused-by-public-route=true");
}
