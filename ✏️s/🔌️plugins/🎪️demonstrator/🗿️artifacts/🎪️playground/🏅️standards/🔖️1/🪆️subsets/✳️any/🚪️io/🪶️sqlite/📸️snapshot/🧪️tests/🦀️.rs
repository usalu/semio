use crate::standards::v1::subsets::any::io::sqlite::snapshot::PlaygroundSnapshot;
use store::{ArtifactDsl, ArtifactPack};
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}
#[test]
fn sqlite_snapshot_playground_actual_bare_owner_has_capability() {
    assert!(PlaygroundSnapshot::sqlite_snapshot_codec().is_some(), "actual Playground parent lacks SQLite");
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_playground_actual_declaration_registers_both_io_directions() {
    use store::io::io_mechanism::{io_route, io_run};
    use {semio_framework_artifact_reference::ArtifactDialect,store::io_schema::IoPayload,store::io_schema::SQLITE_SNAPSHOT};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("demonstrator")
        .label("Playground SQLite Declaration")
        .version("0.0.1")
        .package_id("semio:demonstrator")
        .artifact(crate::declaration().unwrap())
        .try_build()
        .unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.demonstrator.playground".into(), standard: "1".into(), subset: "*".into() };
    let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
    let into = io_route(&dialect, &sqlite, 1).await.expect("Playground declared SQLite export route missing").value;
    let from = io_route(&sqlite, &dialect, 1).await.expect("Playground declared SQLite import route missing").value;
    for case in fixture()["nativeCases"].as_array().unwrap() {
        let source = PlaygroundSnapshot { schema: case["schema"].as_str().unwrap().into() };
        for payload in [IoPayload::Binary(source.encode_pack()), IoPayload::Text(source.print_dsl())] {
            let exported = io_run(&into, payload.clone()).await.unwrap().value;
            let IoPayload::Binary(bytes) = &exported else { panic!("SQLite is not a binary file") };
            assert_eq!(&bytes[..16], b"SQLite format 3\0");
            assert_eq!(io_run(&from, exported).await.unwrap().value, payload);
        }
    }
}
#[test]
fn sqlite_snapshot_playground_ordinary_native_formats_keep_every_literal_field() {
    for case in fixture()["nativeCases"].as_array().unwrap() {
        let source = PlaygroundSnapshot { schema: case["schema"].as_str().unwrap().into() };
        assert_eq!(PlaygroundSnapshot::parse_dsl(&source.print_dsl()).unwrap(), source);
        assert_eq!(PlaygroundSnapshot::decode_pack(&source.encode_pack()).unwrap(), source);
    }
}
#[test]
fn sqlite_snapshot_playground_authored_single_table_is_independently_queryable() {
    use std::process::Command;
    let sql = include_str!("../🗄️.sql");
    let script="import{Database}from'bun:sqlite';const db=new Database(':memory:');try{db.exec(process.env.SEMIO_PLAYGROUND_SQL);if(db.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all().length!==1||db.query('PRAGMA table_info(playground_document)').all().length!==2)throw Error('hand schema');db.query('INSERT INTO playground_document VALUES(1001,?)').run('literal 日本');if(db.query('SELECT schema FROM playground_document').get().schema!=='literal 日本')throw Error('literal schema');console.log('[DEBUG] independent Playground SQL schema and literal row confirmed');}finally{db.close()}";
    let output = Command::new("bun").args(["-e", script]).env("SEMIO_PLAYGROUND_SQL", sql).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}
#[test]
fn sqlite_snapshot_playground_actual_erased_native_formats_use_queryable_parent() {
    use semio_framework_os_kernel::sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotControl};
    let codec = store::ArtifactCodec::bare::<PlaygroundSnapshot, crate::standards::v1::subsets::any::schema::mutations::PlaygroundMutation>(crate::PLAYGROUND_DOCUMENT_SCHEMA);
    let provider = codec.snapshot_sqlite.expect("Playground declared snapshot capability missing");
    assert_eq!(provider.snapshot_type, Some(std::any::TypeId::of::<PlaygroundSnapshot>()));
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.demonstrator.playground".into(), standard: "1".into(), subset: "*".into() };
    for case in fixture()["nativeCases"].as_array().unwrap() {
        let source = PlaygroundSnapshot { schema: case["schema"].as_str().unwrap().into() };
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let payload = match encoding {
                SnapshotEncoding::Binary => store::io::IoPayload::Binary(source.encode_pack()),
                SnapshotEncoding::Text => store::io::IoPayload::Text(source.print_dsl()),
            };
            let database = (provider.export)(crate::PLAYGROUND_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database.tables.len(), 1);
            assert_eq!(database.table("playground_document").unwrap().single_row().unwrap().text(1).unwrap(), source.schema);
            let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
            let loaded = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
            assert_eq!((provider.import)(crate::PLAYGROUND_DOCUMENT_SCHEMA, &dialect, loaded, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value, payload);
        }
    }
}

#[test]
fn sqlite_snapshot_playground_owned_rows_are_literal_and_refuse_hostile_shapes() {
    use store::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
    let source = PlaygroundSnapshot { schema: fixture()["nativeCases"][2]["schema"].as_str().unwrap().into() };
    let limits = SqliteDatabaseLimits { max_rows: fixture()["controls"]["domainRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
    let database = source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: 0, ..limits })).is_err());
    let mut renamed = database.clone();
    let row = &mut renamed.table_mut("playground_document").unwrap().rows[0];
    row.rowid += fixture()["identityOffset"].as_i64().unwrap();
    row.values[0] = SqliteValue::Integer(row.rowid);
    assert_eq!(PlaygroundSnapshot::from_sqlite_database(&renamed, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), source);
    for case in 0..6 {
        let mut bad = database.clone();
        let table = bad.table_mut("playground_document").unwrap();
        match case {
            0 => table.rows.clear(),
            1 => {
                let mut extra = table.rows[0].clone();
                extra.rowid = 2;
                extra.values[0] = SqliteValue::Integer(2);
                table.rows.push(extra);
            }
            2 => table.rows[0].values[0] = SqliteValue::Integer(2),
            3 => table.rows[0].values[1] = SqliteValue::Integer(1),
            4 => {
                table.rows[0].rowid = -1;
                table.rows[0].values[0] = SqliteValue::Integer(-1);
            }
            _ => table.sql = table.sql.replace("schema TEXT NOT NULL", "schema TEXT"),
        }
        assert!(PlaygroundSnapshot::from_sqlite_database(&bad, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err(), "case {case}");
    }
    for (kind, standard, subset) in [("s.demonstrator.other", "1", "*"), ("s.demonstrator.playground", "v1", "*"), ("s.demonstrator.playground", "1", "other")] {
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: kind.into(), standard: standard.into(), subset: subset.into() };
        assert!(source.validate_sqlite_snapshot_subset(&dialect, &database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    }
    println!("[DEBUG] Playground literal identity, closed rows and exact dialect refusals confirmed");
}

#[test]
fn sqlite_snapshot_playground_controlled_copies_refuse_inside_all_four_phases() {
    use store::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
    let f = fixture();
    let source = PlaygroundSnapshot { schema: f["largeTextUnit"].as_str().unwrap().repeat(f["largeTextRepeats"].as_u64().unwrap() as usize) };
    let database = source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = source.schema.len() + 8;
    assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: bytes, ..SqliteDatabaseLimits::default() })).is_ok());
    assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: bytes - 1, ..SqliteDatabaseLimits::default() })).is_err());
    let cutoff = f["controls"]["cancelAt"].as_u64().unwrap() as usize;
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] {
        let mut inside = false;
        let mut callback = |p: SqliteSnapshotProgress| {
            if p.phase == phase && p.completed >= cutoff && p.completed < p.total {
                inside = true;
                false
            } else {
                true
            }
        };
        let result = match phase {
            SqliteSnapshotPhase::ProjectSnapshot => source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ()),
            _ => PlaygroundSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ()),
        };
        assert!(result.is_err());
        assert!(inside, "{phase:?}");
    }
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = source.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_eq!(PlaygroundSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), source);
        let size = match &payload {
            store::io_schema::IoPayload::Binary(bytes) => bytes.len(),
            store::io_schema::IoPayload::Text(text) => text.len(),
        };
        assert_eq!(source.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_file_bytes: size, ..SqliteDatabaseLimits::default() })).unwrap(), payload);
        assert!(source.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_file_bytes: size - 1, ..SqliteDatabaseLimits::default() })).is_err());
        for phase in [SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
            let mut inside = false;
            let mut callback = |p: SqliteSnapshotProgress| {
                if p.phase == phase && p.completed >= cutoff && p.completed < p.total {
                    inside = true;
                    false
                } else {
                    true
                }
            };
            let result = match phase {
                SqliteSnapshotPhase::DecodeNative => PlaygroundSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ()),
                _ => source.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ()),
            };
            assert!(result.is_err());
            assert!(inside, "{phase:?} {encoding:?}");
        }
        assert!(PlaygroundSnapshot::decode_sqlite_snapshot_native(
            &payload,
            &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: f["controls"]["valueBytes"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() })
        )
        .is_err());
    }
    println!("[DEBUG] Playground all four copy phases cancel inside the literal text; exact native file ceilings confirmed");
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_playground_typed_io_and_independent_edit_use_final_metadata_rows() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    use store::{
        io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot},
        sqlite_snapshot::*,
        ArtifactSqliteSnapshot,
    };
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("demonstrator")
        .label("Playground Typed SQLite Declaration")
        .version("0.0.1")
        .package_id("semio:demonstrator")
        .artifact(crate::declaration().unwrap())
        .try_build()
        .unwrap();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.demonstrator.playground".into(), standard: "1".into(), subset: "*".into() };
    let source = PlaygroundSnapshot { schema: fixture()["nativeCases"][2]["schema"].as_str().unwrap().into() };
    let limits = SqliteDatabaseLimits { max_rows: fixture()["controls"]["finalRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let mut phases = Vec::new();
        let bytes = io_export_sqlite_snapshot(&dialect, &source, encoding, limits, &mut |p| {
            phases.push(p.phase);
            true
        })
        .await
        .unwrap()
        .value;
        assert!(!phases.iter().any(|p| matches!(p, SqliteSnapshotPhase::DecodeNative | SqliteSnapshotPhase::EncodeNative)));
        let database = import_sqlite_database(&bytes, limits, &mut |_| true).unwrap();
        assert_eq!(database.tables.iter().map(|table| table.rows.len()).sum::<usize>(), limits.max_rows);
        assert_eq!(io_import_sqlite_snapshot::<PlaygroundSnapshot>(&dialect, &bytes, limits, &mut |_| true).await.unwrap().value, source);
        assert!(io_export_sqlite_snapshot(&dialect, &source, encoding, SqliteDatabaseLimits { max_rows: limits.max_rows - 1, ..limits }, &mut |_| true).await.is_err());
        let script = "import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));db.query('UPDATE playground_document SET id=1001,schema=?').run('independent 日本\\0');if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,db.serialize());db.close();";
        let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(&bytes).unwrap();
        let edited = child.wait_with_output().unwrap();
        assert!(edited.status.success(), "{}", String::from_utf8_lossy(&edited.stderr));
        assert_eq!(io_import_sqlite_snapshot::<PlaygroundSnapshot>(&dialect, &edited.stdout, limits, &mut |_| true).await.unwrap().value.schema, "independent 日本\0");
    }
    println!("[DEBUG] Playground typed I/O and independent Bun mutation preserve literal schema at identity1001");
}

/// 📏️ Uses SQLite's actual UTF8 storage to demand every field at its exact native semantic ceiling.
#[test]
fn sqlite_snapshot_playground_independent_exact_semantic_native_limits(){
 use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SnapshotEncoding,export_sqlite_database}};
 use std::{io::Write,process::{Command,Stdio}};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let sources=corpus["nativeCases"].as_array().unwrap().iter().map(|case|PlaygroundSnapshot{schema:case["schema"].as_str().unwrap().into()}).chain(std::iter::once(PlaygroundSnapshot{schema:corpus["largeTextUnit"].as_str().unwrap().repeat(corpus["largeTextRepeats"].as_u64().unwrap()as usize)}));
 for source in sources{
  let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  let mut child=Command::new("bun").arg("-e").arg("import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,String(Number(d.query('SELECT 8+length(CAST(schema AS BLOB)) AS bytes FROM playground_document').get().bytes)))}finally{d.close()}").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let cells:String=String::from_utf8(output.stdout).unwrap();let cells:usize=cells.trim().parse().unwrap();
  let exact=SqliteDatabaseLimits{max_value_bytes:cells,..SqliteDatabaseLimits::default()};
  assert_eq!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap(),database);
  assert_eq!(PlaygroundSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap(),source);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap();
   assert_eq!(PlaygroundSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap(),source);
   let refused=SqliteDatabaseLimits{max_value_bytes:cells-1,..exact};
   assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,refused)).is_err());
   assert!(PlaygroundSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,refused)).is_err());
  }
 }
}
