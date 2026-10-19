//! 🧮️ Original BIM admission and relational ownership laws.
#[path = "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs"]
mod original_owners;

#[test]
fn sqlite_snapshot_bim_original_reader_tracks_current_shared_ownership(){
 let snapshot=fixture();let original=database(&snapshot);assert_eq!(original.tables.len(),128);assert_eq!(original.tables.iter().map(|table|table.rows.len()).sum::<usize>(),4036);
 let mut reversed=original.clone();for table in &mut reversed.tables{table.rows.reverse();}assert_eq!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|ModelSnapshot::from_sqlite_database(&reversed, sqlite_control)).unwrap(),snapshot);
 for(table,owners)in[("bim_axis",3),("bim_top",5),("bim_profile",8),("bim_curtain_grid",4),("bim_annotation_anchor",2)]{let mut malformed=original.clone();let row=&mut malformed.tables.iter_mut().find(|value|value.name==table).unwrap().rows[0];for index in 1..=owners{row.values[index]=SqliteValue::Integer(1);}assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|ModelSnapshot::from_sqlite_database(&malformed, sqlite_control)).is_err(),"{table} multiply owned");}
 let mut malformed=original.clone();malformed.tables.iter_mut().find(|table|table.name=="bim_curtain_grid").unwrap().rows.retain(|row|!matches!(row.values[1],SqliteValue::Integer(_)));assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|ModelSnapshot::from_sqlite_database(&malformed, sqlite_control)).is_err(),"required type U grid absent");
 let mut malformed=original.clone();let row=malformed.tables.iter_mut().find(|table|table.name=="bim_annotation_anchor").unwrap().rows.iter_mut().find(|row|matches!(row.values[2],SqliteValue::Integer(_))).unwrap();row.values[3]=SqliteValue::Integer(1);assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|ModelSnapshot::from_sqlite_database(&malformed, sqlite_control)).is_err(),"leader anchor ordinal is not zero");
}
use ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot as _;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::*;
use store::os_store::serialization::ArtifactSqliteSnapshotHost;
use store::sqlite_snapshot::{SqliteDatabaseLimits,SnapshotEncoding};
fn fixture()->ModelSnapshot{let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();semio_framework_pack_json::from_json_str(&corpus["cases"][0]["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn database(snapshot:&ModelSnapshot)->SqliteDatabase{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).unwrap()}
fn bytes(database:&SqliteDatabase)->usize{database.tables.iter().flat_map(|table|&table.rows).flat_map(|row|&row.values).map(|value|match value{SqliteValue::Null=>0,SqliteValue::Integer(_)|SqliteValue::Real(_)=>8,SqliteValue::Text(value)=>value.len(),SqliteValue::Blob(value)=>value.len()}).sum()}
#[test]
fn sqlite_snapshot_bim_original_census_precedes_typed_birth(){
 let snapshot=fixture();let database=database(&snapshot);let record=snapshot.__dsl_to_record();let exact=bytes(&database);
 for(rows,values,expected)in[(4036,exact,true),(4035,exact,false),(4036,exact-1,false)]{let limits=SqliteDatabaseLimits{max_rows:rows,max_value_bytes:values,..SqliteDatabaseLimits::default()};let mut callback=|_|true;let mut control=NativeDecodeControl::new(0,&mut callback);let result=semantic_cells::admit_record(&record,limits,&mut control);assert_eq!(result.is_ok(),expected,"rows={rows} values={values}");assert_eq!(control.owned_bytes(),0,"borrowed census created typed ownership");}
 let mut reached=false;let mut callback=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|if event.completed>=256{reached=true;false}else{true};let mut control=NativeDecodeControl::new(0,&mut callback);assert!(semantic_cells::admit_record(&record,SqliteDatabaseLimits::default(),&mut control).is_err());assert_eq!(control.owned_bytes(),0);drop(control);assert!(reached);
}
#[test]
fn sqlite_snapshot_bim_original_borrowed_preflight_is_complete(){
 let snapshot=fixture();let exact=bytes(&database(&snapshot));
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{for(rows,values,expected)in[(4036,exact,true),(4035,exact,false),(4036,exact-1,false)]{let limits=SqliteDatabaseLimits{max_rows:rows,max_value_bytes:values,..SqliteDatabaseLimits::default()};let result=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(result.is_ok(),expected,"{encoding:?} rows={rows} values={values}");}}
 let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|if event.phase==SqliteSnapshotPhase::ProjectSnapshot&&event.completed==100{reached=true;false}else{true};assert!(snapshot.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());assert!(reached);
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
 }assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|ModelSnapshot::from_sqlite_database(&malformed, sqlite_control)).is_err(),"{role}");}
}

#[test]
fn sqlite_snapshot_bim_complete_semantics_preserve_original_named_maps(){
 let input:ModelSnapshot=semio_framework_pack_json::from_json_str(include_str!("./🧫️fixtures/🧩️complete/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let projected=database(&input);assert_eq!(projected.tables.len(),128);assert_eq!(projected.tables.iter().map(|t|t.rows.len()).sum::<usize>(),4036);
 let restored=original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|control|ModelSnapshot::from_sqlite_database(&projected,control)).unwrap();assert_eq!(restored,input);
 assert_eq!(restored.supports.len(),3);assert_eq!(restored.load_cases.len(),2);assert_eq!(restored.loads.len(),3);assert_eq!(restored.option_groups.len(),1);assert_eq!(restored.design_options.len(),2);assert_eq!(restored.worksets.len(),2);assert_eq!(restored.element_options.len(),1);assert_eq!(restored.element_worksets.len(),1);assert_eq!(restored.space_conditions.len(),2);assert_eq!(restored.clash_sets.len(),1);assert_eq!(restored.rules.len(),1);assert_eq!(restored.issues.len(),2);assert_eq!(restored.issue_comments.len(),1);assert_eq!(restored.components.len(),2);assert_eq!(restored.component_overrides.len(),1);assert_eq!(restored.mep_elements.len(),3);
}
#[test]
fn sqlite_snapshot_bim_complete_public_provider_is_actual(){
 let codec=<ModelSnapshot as pack::snapshot::ArtifactPack>::sqlite_snapshot_codec().expect("BIM original public owner lacks SQLite capability");
 let _=codec;assert_eq!(ModelSnapshot::SQLITE_SCHEMA,SCHEMA);
}
#[test]
fn sqlite_snapshot_bim_complete_children_and_variant_roles_reject_corruption(){
 let original=database(&fixture());
 for role in ["selector-parent","selector-ordinal","scope-parent","viewpoint-parent","path-ordinal","inactive-shape"]{
  let mut input=original.clone();match role{
   "selector-parent"=>input.tables.iter_mut().find(|t|t.name=="bim_clash_a").unwrap().rows[0].values[1]=SqliteValue::Integer(i64::MAX),
   "selector-ordinal"=>input.tables.iter_mut().find(|t|t.name=="bim_clash_a_class").unwrap().rows[0].values[2]=SqliteValue::Integer(99),
   "scope-parent"=>input.tables.iter_mut().find(|t|t.name=="bim_rule_scope").unwrap().rows[0].values[1]=SqliteValue::Integer(i64::MAX),
   "viewpoint-parent"=>input.tables.iter_mut().find(|t|t.name=="bim_issue_viewpoint").unwrap().rows[0].values[1]=SqliteValue::Integer(i64::MAX),
   "path-ordinal"=>input.tables.iter_mut().find(|t|t.name=="bim_mep_path").unwrap().rows[0].values[2]=SqliteValue::Integer(99),
   _=>{let row=input.tables.iter_mut().find(|t|t.name=="bim_mep_element").unwrap().rows.iter_mut().find(|r|r.text(6).unwrap()=="Duct").unwrap();row.values[13]=SqliteValue::Real(1.0);row.values[14]=SqliteValue::Integer(1.0f64.to_bits()as i64);row.values[15]=SqliteValue::Text("finite".into());}
  }
  assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|control|ModelSnapshot::from_sqlite_database(&input,control)).is_err(),"{role}");
 }
}
