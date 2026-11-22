//! 🧮️ Native semantic limits use every authored SQL storage-class cell.
use semio_framework_test_original_snapshot_owners as original_owners;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::*;
use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SnapshotEncoding,SqliteValue}};
use semio_framework_value::ValueRefusalKind;
type Stage<T> = store::sqlite_snapshot::SqliteStageFailure<T, store::NativeSnapshotFailure>;

/// 🧭️ Reads the refusal kind a native snapshot stage failure carries.
fn stage_kind<T>(stage: &Stage<T>) -> Option<ValueRefusalKind> {
    match stage {
        Stage::NotStarted(error) => Some(error.kind),
        Stage::Operation(failure) => Some(failure.kind()),
        Stage::Settlement { refusal, .. } => Some(refusal.kind),
        Stage::WorkSettlement { operation: Err(failure), .. } => Some(failure.kind()),
        Stage::WorkSettlement { .. } => None,
    }
}

/// 🛬️ Runs one controlled native decode and answers the restored snapshot or the refusal kind.
fn decoded(payload: &store::io_schema::IoPayload, limits: SqliteDatabaseLimits) -> Result<Process3dSnapshot, Option<ValueRefusalKind>> {
    let mut slot = None;
    let turn = original_owners::with_decoding(|native_owner| Process3dSnapshot::decode_sqlite_snapshot_native(payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits), native_owner, &mut slot));
    if let Err(stage) = turn.result {
        return Err(stage_kind(&stage));
    }
    match slot.expect("a completed native decode fills its output slot") {
        Ok(value) => Ok(value),
        Err(stage) => Err(stage_kind(&stage)),
    }
}

/// 🛫️ Runs one controlled native encode and answers the payload or the refusal kind.
fn encoded(snapshot: &Process3dSnapshot, encoding: SnapshotEncoding, limits: SqliteDatabaseLimits) -> Result<store::io_schema::IoPayload, Option<ValueRefusalKind>> {
    let mut slot = None;
    let turn = original_owners::with_encoding(|native_owner| snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits), native_owner, &mut slot));
    if let Err(stage) = turn.result {
        return Err(stage_kind(&stage));
    }
    match slot.expect("a completed native encode fills its output slot") {
        Ok(value) => Ok(value),
        Err(stage) => Err(stage_kind(&stage)),
    }
}

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
   let restored=decoded(&payload,exact).expect("exact grant decodes");assert_full(&restored,&expected,word);
   let payload2=encoded(&expected,encoding,exact).expect("exact grant encodes");
   let restored=decoded(&payload2,exact).expect("re-decode under the exact grant");assert_full(&restored,&expected,word);
   for grant in [0,bytes-1]{
    let limits=SqliteDatabaseLimits{max_value_bytes:grant,..SqliteDatabaseLimits::default()};
    let refused=decoded(&payload,limits).err();
    assert_eq!(refused,Some(Some(ValueRefusalKind::OwnershipLimit)),"native decode must enforce full semantic cell grant {grant}/{bytes}");
    let refused=encoded(&expected,encoding,limits).err();
    assert_eq!(refused,Some(Some(ValueRefusalKind::OwnershipLimit)),"native encode must enforce full semantic cell grant {grant}/{bytes}");
   }
  }
 }
}
