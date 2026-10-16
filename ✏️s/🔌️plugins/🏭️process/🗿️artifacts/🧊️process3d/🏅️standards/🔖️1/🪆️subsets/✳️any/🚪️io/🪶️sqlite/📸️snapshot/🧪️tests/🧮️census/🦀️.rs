//! 🧮️ Native semantic limits use every authored SQL storage-class cell.
use semio_framework_test_original_snapshot_owners as original_owners;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::*;
use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SnapshotEncoding,SqliteValue}};
#[test]
fn sqlite_snapshot_process3d_native_exact_semantic_cells_both_directions(){
 let contract:serde_json::Value=serde_json::from_str(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🧮️census/🔣️.json")).expect("../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🧮️census/🔣️.json").leak()).unwrap();assert_eq!(contract["scalarBytes"],8);
 for word in words(){
  let expected=full(word);
  let database=original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|expected.to_sqlite_database(sqlite_control)).unwrap();
  let bytes=database.tables.iter().flat_map(|table|&table.rows).flat_map(|row|&row.values).try_fold(0usize,|sum,value|sum.checked_add(match value{SqliteValue::Null=>0,SqliteValue::Integer(_)|SqliteValue::Real(_)=>8,SqliteValue::Text(text)=>text.len(),SqliteValue::Blob(blob)=>blob.len()})).unwrap();assert!(bytes>1);
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(&expected)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(&expected))};
   let exact=SqliteDatabaseLimits{max_value_bytes:bytes,..SqliteDatabaseLimits::default()};
   let restored=original_owners::with_decoding(|native_owner|Process3dSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();assert_full(&restored,&expected,word);
   let encoded=original_owners::with_encoding(|native_owner|expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();
   let restored=original_owners::with_decoding(|native_owner|Process3dSnapshot::decode_sqlite_snapshot_native(&encoded,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();assert_full(&restored,&expected,word);
   for grant in [0,bytes-1]{
    let limits=SqliteDatabaseLimits{max_value_bytes:grant,..SqliteDatabaseLimits::default()};
    let decoded=original_owners::with_decoding(|native_owner|Process3dSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner));
    assert!(matches!(decoded,Err(ref cause) if cause.kind==semio_framework_value::ValueRefusalKind::OwnershipLimit),"native decode must enforce full semantic cell grant {grant}/{bytes}: {decoded:?}");
    let encoded=original_owners::with_encoding(|native_owner|expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner));
    assert!(matches!(encoded,Err(ref cause) if cause.kind==semio_framework_value::ValueRefusalKind::OwnershipLimit),"native encode must enforce full semantic cell grant {grant}/{bytes}: {encoded:?}");
   }
  }
 }
}
