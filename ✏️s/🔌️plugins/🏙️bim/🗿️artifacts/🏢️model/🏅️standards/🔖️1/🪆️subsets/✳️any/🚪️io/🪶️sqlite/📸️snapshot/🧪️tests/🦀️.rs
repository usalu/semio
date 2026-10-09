//! 🧮️ Original BIM admission and relational ownership laws.

#[test]
fn sqlite_snapshot_bim_original_reader_tracks_current_shared_ownership(){
 let snapshot=fixture();let original=database(&snapshot);assert_eq!(original.tables.len(),93);assert_eq!(original.tables.iter().map(|table|table.rows.len()).sum::<usize>(),3972);
 let mut reversed=original.clone();for table in &mut reversed.tables{table.rows.reverse();}assert_eq!(ModelSnapshot::from_sqlite_database(&reversed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
 for(table,owners)in[("bim_axis",3),("bim_top",5),("bim_profile",8),("bim_curtain_grid",4),("bim_annotation_anchor",2)]{let mut malformed=original.clone();let row=&mut malformed.tables.iter_mut().find(|value|value.name==table).unwrap().rows[0];for index in 1..=owners{row.values[index]=SqliteValue::Integer(1);}assert!(ModelSnapshot::from_sqlite_database(&malformed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table} multiply owned");}
 let mut malformed=original.clone();malformed.tables.iter_mut().find(|table|table.name=="bim_curtain_grid").unwrap().rows.retain(|row|!matches!(row.values[1],SqliteValue::Integer(_)));assert!(ModelSnapshot::from_sqlite_database(&malformed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"required type U grid absent");
 let mut malformed=original.clone();let row=malformed.tables.iter_mut().find(|table|table.name=="bim_annotation_anchor").unwrap().rows.iter_mut().find(|row|matches!(row.values[2],SqliteValue::Integer(_))).unwrap();row.values[3]=SqliteValue::Integer(1);assert!(ModelSnapshot::from_sqlite_database(&malformed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"leader anchor ordinal is not zero");
 println!("[DEBUG] BIM original reader tables=93 rows=3972 reversed_order=true shared_owner_sets=5 required_grid_and_leader_ordinal_refused");
}
use super::*;
use store::ArtifactSqliteSnapshot;
use store::sqlite_snapshot::{SqliteDatabaseLimits,SnapshotEncoding};
fn fixture()->ModelSnapshot{let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();semio_framework_pack_json::from_json_str(&corpus["cases"][0]["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn database(snapshot:&ModelSnapshot)->SqliteDatabase{snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn bytes(database:&SqliteDatabase)->usize{database.tables.iter().flat_map(|table|&table.rows).flat_map(|row|&row.values).map(|value|match value{SqliteValue::Null=>0,SqliteValue::Integer(_)|SqliteValue::Real(_)=>8,SqliteValue::Text(value)=>value.len(),SqliteValue::Blob(value)=>value.len()}).sum()}
#[test]
fn sqlite_snapshot_bim_original_census_precedes_typed_birth(){
 let snapshot=fixture();let database=database(&snapshot);let record=snapshot.__dsl_to_record();let exact=bytes(&database);
 for(rows,values,expected)in[(3972,exact,true),(3971,exact,false),(3972,exact-1,false)]{let limits=SqliteDatabaseLimits{max_rows:rows,max_value_bytes:values,..SqliteDatabaseLimits::default()};let mut callback=|_|true;let mut control=NativeDecodeControl::new(0,&mut callback);let result=semantic_cells::admit_record(&record,limits,&mut control);assert_eq!(result.is_ok(),expected,"rows={rows} values={values}");assert_eq!(control.owned_bytes(),0,"borrowed census created typed ownership");}
 let mut reached=false;let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|if event.completed>=256{reached=true;false}else{true};let mut control=NativeDecodeControl::new(0,&mut callback);assert!(semantic_cells::admit_record(&record,SqliteDatabaseLimits::default(),&mut control).is_err());assert_eq!(control.owned_bytes(),0);drop(control);assert!(reached);
 println!("[DEBUG] BIM original native census rows=3972 bytes={exact} typed_birth=0 exact_and_one_short_refusals cancellation=true");
}
#[test]
fn sqlite_snapshot_bim_original_borrowed_preflight_is_complete(){
 let snapshot=fixture();let exact=bytes(&database(&snapshot));
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{for(rows,values,expected)in[(3972,exact,true),(3971,exact,false),(3972,exact-1,false)]{let limits=SqliteDatabaseLimits{max_rows:rows,max_value_bytes:values,..SqliteDatabaseLimits::default()};let result=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(result.is_ok(),expected,"{encoding:?} rows={rows} values={values}");}}
 let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|if event.phase==SqliteSnapshotPhase::ProjectSnapshot&&event.completed==100{reached=true;false}else{true};assert!(snapshot.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());assert!(reached);
 println!("[DEBUG] BIM original borrowed preflight tables=93 rows=3972 bytes={exact} independent limits and cancellation");
}
#[test]
fn sqlite_snapshot_bim_original_reader_rejects_unowned_and_inactive_roles(){
 let original=database(&fixture());
 for role in["multiple-axis-parent","sparse-slab-hole","unowned-property","inactive-roof-arm","wrong-float-class"]{let mut malformed=original.clone();match role{
  "multiple-axis-parent"=>{let row=&mut malformed.tables.iter_mut().find(|table|table.name=="bim_axis").unwrap().rows[0];row.values[1]=SqliteValue::Integer(1);row.values[2]=SqliteValue::Integer(1);},
  "sparse-slab-hole"=>{let row=&mut malformed.tables.iter_mut().find(|table|table.name=="bim_slab_hole").unwrap().rows[0];row.values[2]=SqliteValue::Integer(7);},
  "unowned-property"=>{let row=&mut malformed.tables.iter_mut().find(|table|table.name=="bim_property_value").unwrap().rows[0];row.values[1]=SqliteValue::Integer(i64::MAX);},
  "inactive-roof-arm"=>{let row=malformed.tables.iter_mut().find(|table|table.name=="bim_roof").unwrap().rows.iter_mut().find(|row|row.text(6).unwrap()=="Flat").unwrap();row.values[7]=SqliteValue::Real(1.0);row.values[8]=SqliteValue::Integer(1.0f64.to_bits()as i64);row.values[9]=SqliteValue::Text("finite".into());},
  _=>{let row=&mut malformed.tables.iter_mut().find(|table|table.name=="bim_material").unwrap().rows[0];row.values[8]=SqliteValue::Text("nan".into());}
 }assert!(ModelSnapshot::from_sqlite_database(&malformed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{role}");}
 println!("[DEBUG] BIM original owned reader malformed_roles=5 refused");
}
