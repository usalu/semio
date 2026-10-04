use super::*;
use semio_framework_os_kernel::{
    sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue},
    ArtifactSqliteSnapshot,
};

fn fixture() -> PngSnapshot {
    crate::io::decode_png(include_bytes!("../../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-multi-idat-private.png")).unwrap()
}

#[test]
fn sqlite_snapshot_owns_one_exact_canonical_byte_carrier() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let table = database.table("png_document").unwrap();
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].text(1).unwrap(), snapshot.schema);
    assert_eq!(table.rows[0].text(2).unwrap(), "native");
    assert_eq!(database.tables.len(),24);
    assert!(database.table("png_literal_octet").unwrap().rows.is_empty());
    assert_eq!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).filter(|value| matches!(value, SqliteValue::Blob(_))).count(), 0);
    let file = export_sqlite_database(&database, Default::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&file, Default::default(), &mut |_| true).unwrap();
    let restored = PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    assert_eq!(restored, snapshot);
    assert_eq!(crate::io::encode_png(&restored).unwrap(), snapshot.bytes);
}

#[semio_framework_async_macros::async_test]
async fn typed_sqlite_file_round_trip_preserves_chunk_order_and_idat_boundaries() {
    use semio_framework_os_kernel::io::{io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}, ArtifactDialect};
    use store::sqlite_snapshot::SnapshotEncoding;
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")
        .label("PNG canonical SQLite").version("0.0.1").package_id("semio:stdio")
        .artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.png".into(), standard: "1.2".into(), subset: "*".into() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let snapshot = fixture();
        let file = io_export_sqlite_snapshot(&dialect, &snapshot, encoding, Default::default(), &mut |_| true).await.unwrap().value;
        assert_eq!(&file[..16], b"SQLite format 3\0");
        let restored = io_import_sqlite_snapshot::<PngSnapshot>(&dialect, &file, Default::default(), &mut |_| true).await.unwrap().value;
        assert_eq!(restored, snapshot);
    }
}

#[test]
fn reconstruction_refuses_invalid_canonical_bytes_atomically() {
    let snapshot = fixture();
    let mut database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    database.table_mut("png_document").unwrap().rows[0].values[2] = SqliteValue::Text("literal".into());
    assert!(PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).is_err());
}

#[test]
fn sqlite_projection_and_reconstruction_obey_cancellation_and_ownership_limits() {
    let snapshot = fixture();
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, Default::default())).is_err());
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_allocation_bytes: snapshot.bytes.len() - 1, ..Default::default() };
    assert!(PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}

#[test]
fn sqlite_snapshot_png_semantic_chunks_scanlines_and_exact_native_owner() {
    use std::{io::Write,process::{Command,Stdio}};
    let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧩️semantic/🔣️.json")).unwrap();
    let snapshot=crate::io::decode_png(include_bytes!("../../../../../../../../🧫️fixtures/🧬️canonical-source/indexed-2bit-duplicate-palette.png")).unwrap();
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
    let header=database.table("png_ihdr").expect("real native PNG must expose all seven interpreted IHDR fields");
    assert_eq!(header.rows.len(),1);
    assert_eq!(header.rows[0].integer(1).unwrap(),neutral["ihdr"]["width"].as_i64().unwrap());
    assert!(database.table("png_scanline").expect("native PNG must expose actual filtered scanlines").rows.len()>0);
    assert!(database.table("png_sample").expect("native PNG must expose packed source samples").rows.len()>0);
    assert!(database.table("png_deflate_block").expect("native PNG must expose complete compression blocks").rows.len()>0);
    let file=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();
    let script=r#"import {Database} from 'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));console.log(JSON.stringify({header:d.query('SELECT width,height,bit_depth AS bitDepth,color_type AS colorType,compression,filter,interlace FROM png_ihdr').get(),chunks:d.query('SELECT kind FROM png_chunk ORDER BY ordinal').all().map(r=>r.kind),fk:d.query('PRAGMA foreign_key_check').all(),blobs:d.query("SELECT COUNT(*) AS count FROM sqlite_schema WHERE type='table' AND sql LIKE '%BLOB%'").get()}));d.close();"#;
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let write=child.stdin.take().unwrap().write_all(&file);let output=child.wait_with_output().unwrap();
    assert!(write.is_ok()&&output.status.success(),"independent SQL oracle: write={write:?}; stderr={}",String::from_utf8_lossy(&output.stderr));
    let observed:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(observed["header"],neutral["ihdr"]);assert_eq!(observed["chunks"],neutral["chunkKinds"]);assert_eq!(observed["fk"],serde_json::json!([]));assert_eq!(observed["blobs"]["count"],0);
    assert_eq!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap(),snapshot);
    for encoding in [store::sqlite_snapshot::SnapshotEncoding::Binary,store::sqlite_snapshot::SnapshotEncoding::Text]{let native=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(PngSnapshot::decode_sqlite_snapshot_native(&native,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap(),snapshot);}
    let mut edited=database;edited.table_mut("png_ihdr").unwrap().rows[0].values[1]=SqliteValue::Integer(neutral["edit"]["value"].as_i64().unwrap());
    let restored=PngSnapshot::from_sqlite_database(&edited,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
    assert_ne!(restored.bytes,snapshot.bytes);
    let script=r#"import {PNG} from 'pngjs';const image=PNG.sync.read(Buffer.from(await Bun.stdin.arrayBuffer()));console.log(JSON.stringify({width:image.width,height:image.height,rgba:[...image.data]}));"#;
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();let write=child.stdin.take().unwrap().write_all(&restored.bytes);let output=child.wait_with_output().unwrap();assert!(write.is_ok()&&output.status.success(),"independent pixel oracle: write={write:?}; stderr={}",String::from_utf8_lossy(&output.stderr));let pixels:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(pixels["width"],neutral["edit"]["value"]);assert_eq!(pixels["rgba"],neutral["editedRgba"]);
}
