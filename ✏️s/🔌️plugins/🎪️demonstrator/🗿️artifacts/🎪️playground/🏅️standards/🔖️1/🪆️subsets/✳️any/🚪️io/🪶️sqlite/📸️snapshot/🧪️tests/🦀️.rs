use store::{sqlite_snapshot::SnapshotEncoding, sqlite_snapshot::SqliteDatabaseLimits, sqlite_snapshot::SqliteSnapshotControl, sqlite_snapshot::SqliteSnapshotPhase, os_store::serialization::ArtifactSqliteSnapshotHost};
use semio_framework_test_original_snapshot_owners as original_owners;
use ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot as _;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};
fn fixture() -> serde_json::Value {
    serde_json::from_str(runtime_fixture_text("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()
}
#[test]
fn sqlite_snapshot_playground_actual_bare_owner_has_capability() {
    assert!(PlaygroundSnapshot::sqlite_snapshot_codec().is_some(), "actual Playground parent lacks SQLite");
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_playground_actual_declaration_registers_both_io_directions() {
    use store::io::io_mechanism::io_route;use original_owners::io_run;
    use {semio_framework_artifact_reference::ArtifactDialect,store::io_schema::IoPayload,store::io_schema::SQLITE_SNAPSHOT};
    semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("demonstrator")
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
            let exported = io_run(&into, payload.clone(), store::sqlite_snapshot::SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
            let IoPayload::Binary(bytes) = &exported else { panic!("SQLite is not a binary file") };
            assert_eq!(&bytes[..16], b"SQLite format 3\0");
            assert_eq!(io_run(&from, exported, store::sqlite_snapshot::SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value, payload);
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
    let sql = runtime_fixture_text("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql");
    let script="import{Database}from'bun:sqlite';const db=new Database(':memory:');try{db.exec(process.env.SEMIO_PLAYGROUND_SQL);if(db.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all().length!==1||db.query('PRAGMA table_info(playground_document)').all().length!==2)throw Error('hand schema');db.query('INSERT INTO playground_document VALUES(1001,?)').run('literal 日本');if(db.query('SELECT schema FROM playground_document').get().schema!=='literal 日本')throw Error('literal schema');console.log('[DEBUG] independent Playground SQL schema and literal row confirmed');}finally{db.close()}";
    let output = Command::new("bun").args(["-e", script]).env("SEMIO_PLAYGROUND_SQL", sql).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}
#[test]
fn sqlite_snapshot_playground_actual_erased_native_formats_use_queryable_parent() {
    use semio_framework_os_kernel::sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotControl};
    let codec = store::bare_artifact_codec::<PlaygroundSnapshot, crate::standards::v1::subsets::any::schema::mutations::PlaygroundMutation>(crate::PLAYGROUND_DOCUMENT_SCHEMA);
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
            let database = original_owners::with_decoding(|native_owner|(provider.export)(crate::PLAYGROUND_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()), native_owner)).unwrap().value;
            assert_eq!(database.tables.len(), 1);
            assert_eq!(database.table("playground_document").unwrap().single_row().unwrap().text(1).unwrap(), source.schema);
            let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
            let loaded = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
            assert_eq!(original_owners::with_encoding(|native_owner|(provider.import)(crate::PLAYGROUND_DOCUMENT_SCHEMA, &dialect, &mut Some(loaded), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()), native_owner)).unwrap().value, payload);
        }
    }
}

#[test]
fn sqlite_snapshot_playground_owned_rows_are_literal_and_refuse_hostile_shapes() {
    use store::{sqlite_snapshot::*, os_store::serialization::ArtifactSqliteSnapshotHost};
    let source = PlaygroundSnapshot { schema: fixture()["nativeCases"][2]["schema"].as_str().unwrap().into() };
    let limits = SqliteDatabaseLimits { max_rows: fixture()["controls"]["domainRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
    let database = original_owners::with_sqlite_recipient(limits,&mut |_| true,|sqlite_control|source.to_sqlite_database(sqlite_control)).unwrap();
    assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits { max_rows: 0, ..limits },&mut |_| true,|sqlite_control|source.to_sqlite_database(sqlite_control)).is_err());
    let mut renamed = database.clone();
    let row = &mut renamed.table_mut("playground_document").unwrap().rows[0];
    row.rowid += fixture()["identityOffset"].as_i64().unwrap();
    row.values[0] = SqliteValue::Integer(row.rowid);
    assert_eq!(original_owners::with_sqlite_recipient(limits,&mut |_| true,|sqlite_control|PlaygroundSnapshot::from_sqlite_database(&renamed, sqlite_control)).unwrap(), source);
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
        assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_| true,|sqlite_control|PlaygroundSnapshot::from_sqlite_database(&bad, sqlite_control)).is_err(), "case {case}");
    }
    for (kind, standard, subset) in [("s.demonstrator.other", "1", "*"), ("s.demonstrator.playground", "v1", "*"), ("s.demonstrator.playground", "1", "other")] {
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: kind.into(), standard: standard.into(), subset: subset.into() };
        assert!(source.validate_sqlite_snapshot_subset(&dialect, &database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    }
}

#[test]
fn sqlite_snapshot_playground_controlled_copies_refuse_inside_all_four_phases() {
    use store::{sqlite_snapshot::*, os_store::serialization::ArtifactSqliteSnapshotHost};
    let f = fixture();
    let source = PlaygroundSnapshot { schema: f["largeTextUnit"].as_str().unwrap().repeat(f["largeTextRepeats"].as_u64().unwrap() as usize) };
    let database = original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_| true,|sqlite_control|source.to_sqlite_database(sqlite_control)).unwrap();
    let bytes = source.schema.len() + 8;
    assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits { max_value_bytes: bytes, ..SqliteDatabaseLimits::default() },&mut |_| true,|sqlite_control|source.to_sqlite_database(sqlite_control)).is_ok());
    assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits { max_value_bytes: bytes - 1, ..SqliteDatabaseLimits::default() },&mut |_| true,|sqlite_control|source.to_sqlite_database(sqlite_control)).is_err());
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
            SqliteSnapshotPhase::ProjectSnapshot => original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut callback,|sqlite_control|source.to_sqlite_database(sqlite_control)).map(|_| ()),
            _ => original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut callback,|sqlite_control|PlaygroundSnapshot::from_sqlite_database(&database, sqlite_control)).map(|_| ()),
        };
        assert!(result.is_err());
        assert!(inside, "{phase:?}");
    }
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()), native_owner)).unwrap();
        assert_eq!(original_owners::with_decoding(|native_owner|PlaygroundSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()), native_owner)).unwrap(), source);
        let size = match &payload {
            store::io_schema::IoPayload::Binary(bytes) => bytes.len(),
            store::io_schema::IoPayload::Text(text) => text.len(),
        };
        assert_eq!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_file_bytes: size, ..SqliteDatabaseLimits::default() }), native_owner)).unwrap(), payload);
        assert!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_file_bytes: size - 1, ..SqliteDatabaseLimits::default() }), native_owner)).is_err());
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
                SqliteSnapshotPhase::DecodeNative => original_owners::with_decoding(|native_owner|PlaygroundSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()), native_owner)).map(|_| ()),
                _ => original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()), native_owner)).map(|_| ()),
            };
            assert!(result.is_err());
            assert!(inside, "{phase:?} {encoding:?}");
        }
        assert!(original_owners::with_decoding(|native_owner|PlaygroundSnapshot::decode_sqlite_snapshot_native(
            &payload,
            &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: f["controls"]["valueBytes"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() })
        , native_owner))
        .is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_playground_typed_io_and_independent_edit_use_final_metadata_rows() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    use store::{
        io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot},
        sqlite_snapshot::*,
        os_store::serialization::ArtifactSqliteSnapshotHost,
    };
    semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("demonstrator")
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
        let bytes = async{let original_file_dialect=&dialect;let original_file_input=&source;let original_file_encoding=encoding;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |p| {
            phases.push(p.phase);
            true
        };store::io::testing::with_snapshot_file_decoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await
        .unwrap()
        .value;
        assert!(!phases.iter().any(|p| matches!(p, SqliteSnapshotPhase::DecodeNative | SqliteSnapshotPhase::EncodeNative)));
        let database = import_sqlite_database(&bytes, limits, &mut |_| true).unwrap();
        assert_eq!(database.tables.iter().map(|table| table.rows.len()).sum::<usize>(), limits.max_rows);
        assert_eq!(async{let original_file_dialect=&dialect;let original_file_input=&bytes;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;store::io::testing::with_snapshot_file_encoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<PlaygroundSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value, source);
        assert!(async{let original_file_dialect=&dialect;let original_file_input=&source;let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits { max_rows: limits.max_rows - 1, ..limits };let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;store::io::testing::with_snapshot_file_decoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.is_err());
        let script = "import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));db.query('UPDATE playground_document SET id=1001,schema=?').run('independent 日本\\0');if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,db.serialize());db.close();";
        let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(&bytes).unwrap();
        let edited = child.wait_with_output().unwrap();
        assert!(edited.status.success(), "{}", String::from_utf8_lossy(&edited.stderr));
        assert_eq!(async{let original_file_dialect=&dialect;let original_file_input=&edited.stdout;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;store::io::testing::with_snapshot_file_encoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<PlaygroundSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value.schema, "independent 日本\0");
    }
}

/// 📏️ Uses SQLite's actual UTF8 storage to demand every field at its exact native semantic ceiling.
#[test]
fn sqlite_snapshot_playground_independent_exact_semantic_native_limits(){
 use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SnapshotEncoding,export_sqlite_database}};
 use std::{io::Write,process::{Command,Stdio}};
 let corpus:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let sources=corpus["nativeCases"].as_array().unwrap().iter().map(|case|PlaygroundSnapshot{schema:case["schema"].as_str().unwrap().into()}).chain(std::iter::once(PlaygroundSnapshot{schema:corpus["largeTextUnit"].as_str().unwrap().repeat(corpus["largeTextRepeats"].as_u64().unwrap()as usize)}));
 for source in sources{
  let database=original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).unwrap();
  let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  let mut child=Command::new("bun").arg("-e").arg("import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,String(Number(d.query('SELECT 8+length(CAST(schema AS BLOB)) AS bytes FROM playground_document').get().bytes)))}finally{d.close()}").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let cells:String=String::from_utf8(output.stdout).unwrap();let cells:usize=cells.trim().parse().unwrap();
  let exact=SqliteDatabaseLimits{max_value_bytes:cells,..SqliteDatabaseLimits::default()};
  assert_eq!(original_owners::with_sqlite_recipient(exact,&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).unwrap(),database);
  assert_eq!(original_owners::with_sqlite_recipient(exact,&mut |_|true,|sqlite_control|PlaygroundSnapshot::from_sqlite_database(&database, sqlite_control)).unwrap(),source);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let payload=original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();
   assert_eq!(original_owners::with_decoding(|native_owner|PlaygroundSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap(),source);
   let refused=SqliteDatabaseLimits{max_value_bytes:cells-1,..exact};
   assert!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,refused), native_owner)).is_err());
   assert!(original_owners::with_decoding(|native_owner|PlaygroundSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,refused), native_owner)).is_err());
  }
 }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
