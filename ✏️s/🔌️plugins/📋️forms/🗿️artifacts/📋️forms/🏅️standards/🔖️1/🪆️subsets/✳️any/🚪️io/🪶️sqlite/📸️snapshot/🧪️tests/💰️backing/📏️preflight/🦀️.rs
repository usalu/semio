//! 📏️ Direct preflight law keeps encoded forecasts separate from real borrowed scratch requests.
use super::*;
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
use semio_framework_value::ValueRefusalKind;
#[test]
fn sqlite_snapshot_forms_borrowed_whitespace_kind_cancels_before_semantic_refusal(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/📏️preflight/🔤️whitespace/🔣️.json")).unwrap();
 let mut owner=Owned(Some(specimen()));let kind=contract["kindUnit"].as_str().unwrap().repeat(usize::try_from(contract["repeat"].as_u64().unwrap()).unwrap());
 let bytes=usize::try_from(contract["utf8Bytes"].as_u64().unwrap()).unwrap();assert_eq!(kind.len(),bytes);assert_eq!(kind.chars().count(),usize::try_from(contract["unicodeScalars"].as_u64().unwrap()).unwrap());assert!(kind.trim().is_empty());owner.0.as_mut().unwrap().definition.steps[0].blocks[0].kind=kind;
 let threshold=usize::try_from(contract["cancelAfterBytes"].as_u64().unwrap()).unwrap();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==bytes&&event.completed>=threshold&&event.completed<event.total{reached=true;false}else{true}};
  let mut control=SqliteSnapshotControl::new(&mut callback,limits);let error=owner.0.as_ref().unwrap().preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled,"borrowed whitespace validation must remain cancellable before semantic refusal");assert_eq!(control.allocation_remaining_bytes(),limits.max_allocation_bytes);assert!(reached,"actual borrowed kind must expose an interior UTF-8 frontier");
  let mut accepted=|_|true;let mut control=SqliteSnapshotControl::new(&mut accepted,limits);assert_eq!(owner.0.as_ref().unwrap().preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err().kind,ValueRefusalKind::InvalidValue);
 }
}
#[test]
fn sqlite_snapshot_forms_borrowed_preflight_pays_scratch_before_native_fields(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/📏️preflight/🔣️.json")).unwrap();
 assert_eq!(contract["scratchOwnership"],"paidFrontiersOnly");
 let f=fixture();let mut owner=Owned(Some(specimen()));
 let snapshot=owner.0.as_mut().unwrap();snapshot.title=Some(f["unicodeText"].as_str().unwrap().repeat(usize::try_from(f["unicodeRepeat"].as_u64().unwrap()).unwrap()));
 let count=usize::try_from(f["wideOptionsCount"].as_u64().unwrap()).unwrap();
 snapshot.definition.steps[0].blocks[0].options=Some((0..count).map(|index|crate::FormQuestionOption{value:format!("preflight{index}"),label:String::new()}).collect());
 let mut baseline=Owned(Some(specimen()));baseline.0.as_mut().unwrap().title=Some(String::new());baseline.0.as_mut().unwrap().definition.steps[0].blocks[0].options=Some((0..count).map(|index|crate::FormQuestionOption{value:format!("preflight{index}"),label:String::new()}).collect());
 let literal_bytes=owner.0.as_ref().unwrap().title.as_ref().unwrap().len();assert_eq!(literal_bytes,usize::try_from(contract["literalUtf8Bytes"].as_u64().unwrap()).unwrap());
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let snapshot=owner.0.as_ref().unwrap();let mut progress=|_|true;let limits=SqliteDatabaseLimits::default();let mut control=SqliteSnapshotControl::new(&mut progress,limits);
  let (result,requests)=observed::measure(||snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control));
  result.expect("actual Forms borrowed native preflight must exist");let admitted=limits.max_allocation_bytes-control.allocation_remaining_bytes();
  assert!(admitted>=requests.bytes,"borrowed scratch admission {admitted} must cover actual full requests {}",requests.bytes);
  let mut baseline_control=SqliteSnapshotControl::new(&mut progress,limits);let (baseline_result,baseline_requests)=observed::measure(||baseline.0.as_ref().unwrap().preflight_sqlite_snapshot_encoding(encoding,&mut baseline_control));baseline_result.unwrap();assert_eq!(requests.bytes,baseline_requests.bytes,"growing the borrowed literal must not create an owned text copy");
  let mut exact=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_allocation_bytes:admitted,..limits});
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut exact).unwrap();assert_eq!(exact.allocation_remaining_bytes(),0);
  if admitted>0{let mut short=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_allocation_bytes:admitted-1,..limits});assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut short).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);}
  for ceilings in[SqliteDatabaseLimits{max_value_bytes:1,..limits},SqliteDatabaseLimits{max_file_bytes:1,..limits}]{let mut control=SqliteSnapshotControl::new(&mut progress,ceilings);assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);}
  let mut rows=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_rows:0,..limits});assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut rows).unwrap_err().kind,ValueRefusalKind::WorkLimit);
  let mut stop=|_|false;let mut canceled=SqliteSnapshotControl::new(&mut stop,limits);assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut canceled).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(canceled.allocation_remaining_bytes(),limits.max_allocation_bytes);
  let mut reached=false;let mut callback=|p:store::sqlite_snapshot::SqliteSnapshotProgress|{if p.phase==SqliteSnapshotPhase::EncodeNative&&p.total>p.completed&&p.completed>=256{reached=true;false}else{true}};
  let mut canceled=SqliteSnapshotControl::new(&mut callback,limits);assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut canceled).unwrap_err().kind,ValueRefusalKind::Canceled);assert!(reached,"borrowed preflight must reach actual interior progress");
  let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut progress,limits)).unwrap();let encoded_bytes=match payload{store::io_schema::IoPayload::Binary(value)=>value.len(),store::io_schema::IoPayload::Text(value)=>value.len()};
  assert!(encoded_bytes>0);let mut ceiling=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_file_bytes:encoded_bytes-1,..limits});assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut ceiling).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
 }
}
