//! 🧮️ Native semantic limits use every authored SQL storage-class cell.
use super::*;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SnapshotEncoding,SqliteValue}};
#[test]
fn sqlite_snapshot_process3d_native_exact_semantic_cells_both_directions(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🧮️census/🔣️.json")).unwrap();assert_eq!(contract["scalarBytes"],8);
 for word in words(){
  let expected=full(word);
  let database=expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let bytes=database.tables.iter().flat_map(|table|&table.rows).flat_map(|row|&row.values).try_fold(0usize,|sum,value|sum.checked_add(match value{SqliteValue::Null=>0,SqliteValue::Integer(_)|SqliteValue::Real(_)=>8,SqliteValue::Text(text)=>text.len(),SqliteValue::Blob(blob)=>blob.len()})).unwrap();assert!(bytes>1);
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&expected)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&expected))};
   let exact=SqliteDatabaseLimits{max_value_bytes:bytes,..SqliteDatabaseLimits::default()};
   let restored=Process3dSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap();assert_full(&restored,&expected,word);
   let encoded=expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap();
   let restored=Process3dSnapshot::decode_sqlite_snapshot_native(&encoded,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap();assert_full(&restored,&expected,word);
   for grant in [0,bytes-1]{
    let limits=SqliteDatabaseLimits{max_value_bytes:grant,..SqliteDatabaseLimits::default()};
    let decoded=Process3dSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits));
    assert!(matches!(decoded,Err(ref cause) if cause.kind==semio_framework_value::ValueRefusalKind::OwnershipLimit),"native decode must enforce full semantic cell grant {grant}/{bytes}: {decoded:?}");
    let encoded=expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));
    assert!(matches!(encoded,Err(ref cause) if cause.kind==semio_framework_value::ValueRefusalKind::OwnershipLimit),"native encode must enforce full semantic cell grant {grant}/{bytes}: {encoded:?}");
   }
  }
 }
}
