//! 📏️ Direct borrowed preflight proves scratch admission separately from owned fields.
use super::*;
use semio_framework_value::ValueRefusalKind;
use store::sqlite_snapshot::{SnapshotEncoding,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase};
#[test]
fn sqlite_snapshot_layout_borrowed_preflight_keeps_actual_dictionary_fields_borrowed(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧾️dictionary/🔣️.json")).unwrap();let plan=&plan["nativePreflight"];assert_eq!(plan["scratchOwnership"],"paidFrontiersOnly");
 let mut owner=complete_fixture(0.0,0.0);let mut baseline=complete_fixture(0.0,0.0);
 let literal=plan["literal"].as_str().unwrap().repeat(usize::try_from(plan["repeat"].as_u64().unwrap()).unwrap());let bytes=literal.len();assert_eq!(bytes,usize::try_from(plan["utf8Bytes"].as_u64().unwrap()).unwrap());owner.data_fields.as_mut().unwrap().entries[0].value=semio_framework_value::DslValue::String(literal);baseline.data_fields.as_mut().unwrap().entries[0].value=semio_framework_value::DslValue::String(String::new());
 let limits=SqliteDatabaseLimits::default();let threshold=usize::try_from(plan["cancelAfterBytes"].as_u64().unwrap()).unwrap();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut accepted=|_|true;let mut control=SqliteSnapshotControl::new(&mut accepted,limits);let(result,requests)=observed::measure(||owner.preflight_sqlite_snapshot_encoding(encoding,&mut control));result.expect("actual Layout bounded borrowed preflight must exist");let admitted=limits.max_allocation_bytes-control.allocation_remaining_bytes();assert!(admitted>=requests.bytes,"borrowed scratch admission {admitted} covers {} concrete requests",requests.bytes);
  let mut base_control=SqliteSnapshotControl::new(&mut accepted,limits);let(base_result,base_requests)=observed::measure(||baseline.preflight_sqlite_snapshot_encoding(encoding,&mut base_control));base_result.unwrap();assert_eq!(requests.bytes,base_requests.bytes,"borrowed literal growth must not allocate its owned text");
  let mut exact=SqliteSnapshotControl::new(&mut accepted,SqliteDatabaseLimits{max_allocation_bytes:admitted,..limits});owner.preflight_sqlite_snapshot_encoding(encoding,&mut exact).unwrap();assert_eq!(exact.allocation_remaining_bytes(),0);
  if admitted>0{let mut short=SqliteSnapshotControl::new(&mut accepted,SqliteDatabaseLimits{max_allocation_bytes:admitted-1,..limits});assert_eq!(owner.preflight_sqlite_snapshot_encoding(encoding,&mut short).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);}
  for ceilings in[SqliteDatabaseLimits{max_file_bytes:1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{let mut control=SqliteSnapshotControl::new(&mut accepted,ceilings);assert_eq!(owner.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);}
  let mut rows=SqliteSnapshotControl::new(&mut accepted,SqliteDatabaseLimits{max_rows:0,..limits});assert_eq!(owner.preflight_sqlite_snapshot_encoding(encoding,&mut rows).unwrap_err().kind,ValueRefusalKind::WorkLimit);
  let mut stop=|_|false;let mut control=SqliteSnapshotControl::new(&mut stop,limits);assert_eq!(owner.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(control.allocation_remaining_bytes(),limits.max_allocation_bytes);
  let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==bytes&&event.completed>=threshold&&event.completed<event.total{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,limits);assert_eq!(owner.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert!(reached,"actual borrowed dictionary text reaches an interior UTF-8 boundary");
  let payload=owner.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut accepted,limits)).unwrap();let actual=match payload{store::io_schema::IoPayload::Binary(value)=>value.len(),store::io_schema::IoPayload::Text(value)=>value.len()};let mut short=SqliteSnapshotControl::new(&mut accepted,SqliteDatabaseLimits{max_file_bytes:actual-1,..limits});assert_eq!(owner.preflight_sqlite_snapshot_encoding(encoding,&mut short).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
 }
 owner.retire_sqlite_snapshot();baseline.retire_sqlite_snapshot();
}
