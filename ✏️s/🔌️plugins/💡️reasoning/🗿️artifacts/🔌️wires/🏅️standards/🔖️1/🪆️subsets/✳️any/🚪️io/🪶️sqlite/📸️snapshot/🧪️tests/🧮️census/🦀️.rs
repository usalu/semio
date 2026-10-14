//! 🧮️ Native semantic limits use every authored SQL storage-class cell.
#[path = "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs"]
mod original_owners;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::*;
use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SnapshotEncoding,SqliteValue}};
#[test]
fn sqlite_snapshot_wires_native_exact_semantic_cells_both_directions(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧮️census/🔣️.json")).unwrap();assert_eq!(contract["scalarBytes"],8);
 for word in serde_json::from_str::<serde_json::Value>(include_str!("../🧫️fixtures/🔣️.json")).unwrap()["floatWords"].as_array().unwrap().iter().map(|value|u64::from_str_radix(value.as_str().unwrap(),16).unwrap()).collect::<Vec<_>>(){
  let mut expected=native_fixture();if let semio_framework_value::DslValue::Array(values)=&mut expected.meta{values.push(semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(f64::from_bits(word))));}else{panic!("complete fixture meta array")}
  let database=original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|expected.to_sqlite_database(sqlite_control)).unwrap();
  let bytes=database.tables.iter().flat_map(|table|&table.rows).flat_map(|row|&row.values).try_fold(0usize,|sum,value|sum.checked_add(match value{SqliteValue::Null=>0,SqliteValue::Integer(_)|SqliteValue::Real(_)=>8,SqliteValue::Text(text)=>text.len(),SqliteValue::Blob(blob)=>blob.len()})).unwrap();assert!(bytes>1);
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(&expected)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(&expected))};
   let exact=SqliteDatabaseLimits{max_value_bytes:bytes,..SqliteDatabaseLimits::default()};
   let restored=original_owners::with_decoding(|native_owner|WiresSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();assert_eq!(restored.content,expected.content);assert_intrinsic(&expected.wires_snapshot,&restored.wires_snapshot);assert_intrinsic(&expected.meta,&restored.meta);
   let encoded=original_owners::with_encoding(|native_owner|expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();
   let restored=original_owners::with_decoding(|native_owner|WiresSnapshot::decode_sqlite_snapshot_native(&encoded,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();assert_eq!(restored.content,expected.content);assert_intrinsic(&expected.wires_snapshot,&restored.wires_snapshot);assert_intrinsic(&expected.meta,&restored.meta);
   for grant in [0,bytes-1]{
    let limits=SqliteDatabaseLimits{max_value_bytes:grant,..SqliteDatabaseLimits::default()};
    let decoded=original_owners::with_decoding(|native_owner|WiresSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner));
    assert!(matches!(decoded,Err(ref cause) if cause.kind==semio_framework_value::ValueRefusalKind::OwnershipLimit),"native decode must enforce full semantic cell grant {grant}/{bytes}: {decoded:?}");
    let encoded=original_owners::with_encoding(|native_owner|expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner));
    assert!(matches!(encoded,Err(ref cause) if cause.kind==semio_framework_value::ValueRefusalKind::OwnershipLimit),"native encode must enforce full semantic cell grant {grant}/{bytes}: {encoded:?}");
   }
  }
 }
}
