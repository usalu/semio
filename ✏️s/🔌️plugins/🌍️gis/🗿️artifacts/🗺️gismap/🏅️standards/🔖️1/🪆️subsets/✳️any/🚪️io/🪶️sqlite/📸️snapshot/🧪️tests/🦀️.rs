use crate::standards::v1::subsets::any::io::sqlite::snapshot::*;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
use semio_framework_value::Number;
fn intrinsic(row:&serde_json::Value)->semio_framework_value::DslValue{match row["kind"].as_str().unwrap(){"null"=>semio_framework_value::DslValue::Null,"boolean"=>semio_framework_value::DslValue::Bool(row["value"].as_bool().unwrap()),"unsigned"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(row["value"].as_str().unwrap().parse().unwrap())),"signed"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(row["value"].as_str().unwrap().parse().unwrap())),"float"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(f64::from_bits(u64::from_str_radix(row["bits"].as_str().unwrap(),16).unwrap()))),"text"=>semio_framework_value::DslValue::String(row["value"].as_str().unwrap().into()),"bytes"=>semio_framework_value::DslValue::Bytes(row["value"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect()),"array"=>semio_framework_value::DslValue::Array(row["items"].as_array().unwrap().iter().map(intrinsic).collect()),"object"=>semio_framework_value::DslValue::Object(row["members"].as_array().unwrap().iter().map(|m|(m["name"].as_str().unwrap().into(),intrinsic(&m["value"]))).collect()),_=>panic!("fixture")}}
fn fixture()->GisMapSnapshot{let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();fn make<S>(c:&serde_json::Value)->store::ArtifactChild<S>{let t=&c["target"];let d=&t["dialect"];store::ArtifactChild::new(c["childId"].as_str().unwrap().into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:t["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:d["artifactKind"].as_str().unwrap().into(),standard:d["standard"].as_str().unwrap().into(),subset:d["subset"].as_str().unwrap().into()}})};GisMapSnapshot{positions:vec![MapFeature{id:"duplicate allowed".into(),data:intrinsic(&f["value"])},MapFeature{id:"duplicate allowed".into(),data:semio_framework_value::DslValue::Null}],routes:vec![],regions:vec![MapFeature{id:String::new(),data:semio_framework_value::DslValue::Bytes(vec![])}],drawing:make(&f["child"]),image:Some(make(&f["child"])),value:make(&f["child"])}}
#[test]
fn sqlite_snapshot_gis_map_all_owned_variants_and_independent_sql(){use std::{io::Write,process::{Command,Stdio}};let value=fixture();let limits=SqliteDatabaseLimits::default();let database=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let restored=GisMapSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),database);let bytes=export_sqlite_database(&database,limits,&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT COUNT(DISTINCT kind)AS n FROM gis_map_value').get().n!==9)throw Error('variants');d.query('UPDATE gis_map_text SET value=?').run('independent edited text');await Bun.write(Bun.stdout,d.serialize());d.close();";let mut process=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();process.stdin.take().unwrap().write_all(&bytes).unwrap();let output=process.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let edited=GisMapSnapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();match&edited.positions[0].data{semio_framework_value::DslValue::Object(members)=>assert!(matches!(&members[6].1,semio_framework_value::DslValue::String(v)if v=="independent edited text")),_=>panic!("object")};GisMapSnapshot::retire_sqlite_snapshot(value);GisMapSnapshot::retire_sqlite_snapshot(restored);GisMapSnapshot::retire_sqlite_snapshot(edited);}
#[test]
fn sqlite_snapshot_gis_map_actual_erased_complete_intrinsic_native_records(){let value=fixture();let limits=SqliteDatabaseLimits::default();let expected=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let codec=<GisMapSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.gis.gismap".into(),standard:"1".into(),subset:"*".into()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(&value)),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(&value))};assert_eq!((codec.export)(crate::GIS_MAP_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,expected);let payload=(codec.import)(crate::GIS_MAP_SCHEMA,&dialect,expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;assert_eq!((codec.export)(crate::GIS_MAP_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,expected);}GisMapSnapshot::retire_sqlite_snapshot(value);}
#[test]
fn sqlite_snapshot_gis_map_deep_retirement_budgets_cancellation_and_exact_owned_profile(){std::thread::Builder::new().stack_size(256*1024).spawn(||{let mut value=fixture();for _ in 0..600{let data=std::mem::replace(&mut value.positions[0].data,semio_framework_value::DslValue::Null);value.positions[0].data=semio_framework_value::DslValue::Array(vec![data]);}let limits=SqliteDatabaseLimits::default();let database=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let restored=GisMapSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),database);for limits in[SqliteDatabaseLimits{max_rows:10,..limits},SqliteDatabaseLimits{max_value_bytes:128,..limits}]{assert!(value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(GisMapSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}let mut interior=false;let mut progress=|p:store::sqlite_snapshot::SqliteSnapshotProgress|{if p.completed>=256{interior=true;false}else{true}};assert!(GisMapSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut progress,limits)).is_err());assert!(interior);let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.gis.gismap".into(),standard:"1".into(),subset:"*".into()};assert!(value.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok());let mut wrong=dialect;wrong.subset="invented".into();assert!(value.validate_sqlite_snapshot_subset(&wrong,&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());GisMapSnapshot::retire_sqlite_snapshot(value);GisMapSnapshot::retire_sqlite_snapshot(restored);}).unwrap().join().unwrap();}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_gismap_real_declaration_public_typed_io() {
    use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route,store::io::io_mechanism::io_run_with_snapshot_control};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("gis").label("GIS SQLite declaration").version("0.0.1").package_id("semio:gis").artifact(crate::declaration().unwrap()).try_build().unwrap();
    let dialect=ArtifactDialect{artifact_kind:"s.gis.gismap".into(),standard:"1".into(),subset:"*".into()};
    let snapshot=fixture();
    let limits=SqliteDatabaseLimits::default();
    let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
    let sqlite=ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);
    let export=io_route(&dialect,&sqlite,1).await.unwrap().value;
    let import=io_route(&sqlite,&dialect,1).await.unwrap().value;
    for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{
        let mut phases=Vec::new();
        let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,limits,&mut |p|{phases.push(p.phase);true}).await.unwrap().value;
        let restored=io_import_sqlite_snapshot::<GisMapSnapshot>(&dialect,&bytes,limits,&mut |_|true).await.unwrap().value;
        assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
        assert!(!phases.iter().any(|p|matches!(p,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative|store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative)));
        let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
        let file=io_run_with_snapshot_control(&export,payload,limits,&mut |_|true).await.unwrap().value;
        let payload=io_run_with_snapshot_control(&import,file,limits,&mut |_|true).await.unwrap().value;
        let decoded=GisMapSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
        assert_eq!(decoded.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
        <GisMapSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(decoded);
        <GisMapSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored);
    }
}

#[test]
fn sqlite_snapshot_gis_map_whole_native_control_preserves_exact_fields_and_interior_cancellation(){
 let mut value=fixture();let old=std::mem::replace(&mut value.positions[0].data,semio_framework_value::DslValue::Array((0..600).map(|i|semio_framework_value::DslValue::String(if i==0{"long 世界".repeat(20000)}else{"child".into()})).collect()));crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::retire_value(old);
 let limits=SqliteDatabaseLimits::default();let expected=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for payload in[store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(&value)),store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(&value))]{
  let restored=GisMapSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);GisMapSnapshot::retire_sqlite_snapshot(restored);
  for limits in[SqliteDatabaseLimits{max_rows:10,..limits},SqliteDatabaseLimits{max_value_bytes:4096,..limits}]{assert!(GisMapSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}
  let mut interior=false;assert!(GisMapSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative&&p.completed>=256&&p.completed<p.total{interior=true;false}else{true}},limits)).is_err());assert!(interior);
 }
 GisMapSnapshot::retire_sqlite_snapshot(value);
}
#[test]
fn sqlite_snapshot_map_explicit_native_output_boundary_preserves_complete_owned_state() {
 let snapshot=fixture();
 let limits=SqliteDatabaseLimits::default();
 let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let payload=<GisMapSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  let restored=<GisMapSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  <GisMapSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored);
 }
 <GisMapSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
#[test]
fn sqlite_snapshot_map_explicit_native_schema_and_row_admission_precedes_work() {
 let snapshot=fixture();
 let limits=SqliteDatabaseLimits::default();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let mut native_work=false;
  let limited=SqliteDatabaseLimits{max_rows:rows-1,..limits};
  let result=<GisMapSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>0{native_work=true;}true},limited));
  assert!(result.is_err());assert!(!native_work);
  let mut native_work=false;
  let limited=SqliteDatabaseLimits{max_schema_bytes:<GisMapSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA.len()-1,..limits};
  let result=<GisMapSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>0{native_work=true;}true},limited));
  assert!(result.is_err());assert!(!native_work);
 }
 <GisMapSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
#[test]
fn sqlite_snapshot_map_explicit_native_output_exact_file_frontier_and_unicode_cancel() {
 let mut snapshot=fixture();let old=std::mem::replace(&mut snapshot.positions[0].data,semio_framework_value::DslValue::String("interior 世界".repeat(20000)));crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::retire_value(old);
 let limits=SqliteDatabaseLimits::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let payload=<GisMapSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  let length=match &payload{store::os_io::IoPayload::Binary(bytes)=>bytes.len(),store::os_io::IoPayload::Text(text)=>text.len()};
  assert!(<GisMapSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:length,..limits})).is_ok());
  assert!(<GisMapSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:length-1,..limits})).is_err());
  let mut interior=false;
  assert!(<GisMapSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>=256&&p.completed<p.total{interior=true;false}else{true}},limits)).is_err());
  assert!(interior);
 }
 <GisMapSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
