use crate::standards::v1::subsets::any::io::sqlite::snapshot::GisTerrainSnapshot;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
#[path="💰️backing/🦀️.rs"]mod backing;
fn neutral_value(value:&serde_json::Value)->semio_framework_value::DslValue{
 use semio_framework_value::{DslValue,Number};match value["kind"].as_str().unwrap(){
  "null"=>DslValue::Null,"boolean"=>DslValue::Bool(value["value"].as_bool().unwrap()),
  "unsigned"=>{let decimal=value["value"].as_str().unwrap();let number=decimal.parse::<u64>().unwrap();assert_eq!(number.to_string(),decimal);DslValue::Number(Number::UInt(number))},
  "signed"=>{let decimal=value["value"].as_str().unwrap();let number=decimal.parse::<i64>().unwrap();assert_eq!(number.to_string(),decimal);DslValue::Number(Number::Int(number))},
  "float"=>DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(value["word"].as_str().unwrap(),16).unwrap()))),
  "text"=>DslValue::String(value["value"].as_str().unwrap().into()),
  "bytes"=>{let hex=value["hex"].as_str().unwrap();DslValue::Bytes((0..hex.len()).step_by(2).map(|i|u8::from_str_radix(&hex[i..i+2],16).unwrap()).collect())},
  "array"=>DslValue::Array(value["items"].as_array().unwrap().iter().map(neutral_value).collect()),
  "object"=>DslValue::Object(value["members"].as_array().unwrap().iter().map(|m|(m["name"].as_str().unwrap().into(),neutral_value(&m["value"]))).collect()),
  _=>panic!("closed neutral variant required"),
 }
}
fn complete_fixture(word:u64)->GisTerrainSnapshot{
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🗺️imported-map/🔣️.json")).unwrap();let mut map=crate::schema::ImportedMap{positions:vec![neutral_value(&plan["position"])],routes:vec![neutral_value(&plan["route"])],regions:vec![neutral_value(&plan["region"])],properties:plan["properties"].as_array().unwrap().iter().map(|m|crate::schema::ImportedProperty{name:m["name"].as_str().unwrap().into(),value:neutral_value(&m["value"])}).collect()};
 map.positions.push(map.positions[0].clone());let all=semio_framework_value::DslValue::Array(map.properties.iter().map(|m|m.value.clone()).collect());for records in [&mut map.positions,&mut map.routes,&mut map.regions]{let semio_framework_value::DslValue::Object(members)=&mut records[0] else{panic!("fixture object required")};members.push(("every intrinsic domain".into(),all.clone()));}
 let(_,mut snapshot)=fixture();snapshot.exaggeration=f64::from_bits(word);snapshot.imported_map=Some(map);let child=&plan["child"];let target=&child["target"];let dialect=&target["dialect"];snapshot.mesh=Some(store::ArtifactChild::new(child["childId"].as_str().unwrap().into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:target["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:dialect["artifactKind"].as_str().unwrap().into(),standard:dialect["standard"].as_str().unwrap().into(),subset:dialect["subset"].as_str().unwrap().into()}}));snapshot
}
fn literal_equal(actual:&GisTerrainSnapshot,expected:&GisTerrainSnapshot){let a=crate::schema::ImportedMap{positions:vec![semio_framework_value::ToValue::to_value(actual)],..Default::default()};let b=crate::schema::ImportedMap{positions:vec![semio_framework_value::ToValue::to_value(expected)],..Default::default()};assert!(a.same(&b),"complete actual terrain fields, raw words, numeric variants, octets and occurrence order");}
fn map_with_scalar(value:semio_framework_value::DslValue)->crate::schema::ImportedMap{crate::schema::ImportedMap{positions:vec![semio_framework_value::DslValue::Object(vec![("unknown literal scalar".into(),value)])],..Default::default()}}
fn assert_map_mutation(base_map:Option<crate::schema::ImportedMap>,requested:Option<crate::schema::ImportedMap>,changed:bool){
 use protocol::{MutationKind,MutationDiff};
 let(_,mut base)=fixture();base.imported_map=base_map;
 let payload=crate::mutations::change_imported_features::ChangeImportedFeatures{new_imported_map:requested.clone()};
 let outcome=payload.diff(&base);assert_eq!(outcome.diff().imported_map.is_some(),changed,"actual mutation must compare complete tags and raw words");
 let actual=protocol::apply_diff(outcome.diff(), &base).unwrap();let mut expected=base.clone();expected.imported_map=requested;literal_equal(&actual,&expected);assert_eq!(actual.mesh,base.mesh);assert_eq!(actual.exaggeration.to_bits(),base.exaggeration.to_bits());
}
#[test]
fn sqlite_snapshot_terrain_map_mutation_preserves_zero_word_change(){
 assert_map_mutation(Some(map_with_scalar(semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(-0.0)))),Some(map_with_scalar(semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(0.0)))),true);
}
#[test]
fn sqlite_snapshot_terrain_map_mutation_preserves_integer_variant_change(){
 assert_map_mutation(Some(map_with_scalar(semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(1)))),Some(map_with_scalar(semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(1)))),true);
}
#[test]
fn sqlite_snapshot_terrain_map_mutation_identical_nan_word_is_noop(){
 let map=map_with_scalar(semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(f64::from_bits(0x7ff8000000000042))));assert_map_mutation(Some(map.clone()),Some(map),false);
}
#[test]
fn sqlite_snapshot_terrain_map_mutation_distinguishes_nan_and_optional_owner(){
 assert_map_mutation(Some(map_with_scalar(semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(f64::from_bits(0x7ff8000000000042))))),Some(map_with_scalar(semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(f64::from_bits(0x7ff8000000000043))))),true);
 assert_map_mutation(None,None,false);assert_map_mutation(None,Some(crate::schema::ImportedMap::default()),true);assert_map_mutation(Some(crate::schema::ImportedMap::default()),None,true);assert_map_mutation(Some(crate::schema::ImportedMap::default()),Some(crate::schema::ImportedMap::default()),false);
}
fn row_limit_owners()->[GisTerrainSnapshot;3]{let absent=GisTerrainSnapshot{mesh:None,imported_map:None,..Default::default()};let empty=GisTerrainSnapshot{imported_map:Some(crate::schema::ImportedMap::default()),..absent.clone()};[absent,empty,complete_fixture(0x8000000000000000)]}
#[test]
fn sqlite_snapshot_terrain_native_encode_counts_complete_map_rows_before_work(){
 let limits=SqliteDatabaseLimits::default();for(owner_index,snapshot)in row_limit_owners().into_iter().enumerate(){let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();if owner_index<2{assert_eq!(rows,2+owner_index);}else{assert!(rows>3);}
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let exact=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,..limits})).unwrap();let restored=GisTerrainSnapshot::decode_sqlite_snapshot_native(&exact,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();literal_equal(&restored,&snapshot);restored.retire_sqlite_snapshot();let mut positive_work=false;let result=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>0{positive_work=true;}true},SqliteDatabaseLimits{max_rows:rows-1,..limits}));assert!(result.is_err(),"owner {owner_index}: complete map row refusal");assert!(!positive_work,"owner {owner_index}: row admission precedes native encoding work");}
 snapshot.retire_sqlite_snapshot();}
}
#[test]
fn sqlite_snapshot_terrain_native_decode_counts_complete_map_rows(){
 let limits=SqliteDatabaseLimits::default();for(owner_index,snapshot)in row_limit_owners().into_iter().enumerate(){let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();if owner_index<2{assert_eq!(rows,2+owner_index);}else{assert!(rows>3);}
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let restored=GisTerrainSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,..limits})).unwrap();literal_equal(&restored,&snapshot);restored.retire_sqlite_snapshot();let result=GisTerrainSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows-1,..limits}));assert!(result.is_err(),"owner {owner_index}: complete map reconstruction row refusal");}
 snapshot.retire_sqlite_snapshot();}
}
#[test]
fn sqlite_snapshot_terrain_complete_imported_intrinsic_owner_roundtrips(){let limits=SqliteDatabaseLimits::default();let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🗺️imported-map/🔣️.json")).unwrap();for word in plan["words"].as_array().unwrap(){let snapshot=complete_fixture(u64::from_str_radix(word.as_str().unwrap(),16).unwrap());let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let restored=GisTerrainSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();literal_equal(&restored,&snapshot);for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let decoded=GisTerrainSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();literal_equal(&decoded,&snapshot);decoded.retire_sqlite_snapshot();}restored.retire_sqlite_snapshot();snapshot.retire_sqlite_snapshot();}}
fn fixture()->(serde_json::Value,GisTerrainSnapshot){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let child=&f["child"];let target=&child["target"];let dialect=&target["dialect"];let s=|v:&serde_json::Value|v.as_str().unwrap().to_owned();let value=GisTerrainSnapshot{exaggeration:1.5,imported_map:None,mesh:Some(store::ArtifactChild::new(s(&child["childId"]),semio_framework_artifact_reference::ArtifactRef{artifact_id:s(&target["artifactId"]),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:s(&dialect["artifactKind"]),standard:s(&dialect["standard"]),subset:s(&dialect["subset"])}}))};(f,value)}
#[test]
fn sqlite_snapshot_gis_terrain_all_ieee_words_and_independent_sql(){use std::{io::Write,process::{Command,Stdio}};let(f,mut value)=fixture();let limits=SqliteDatabaseLimits::default();for word in f["words"].as_array().unwrap(){value.exaggeration=f64::from_bits(u64::from_str_radix(word.as_str().unwrap(),16).unwrap());let database=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let restored=GisTerrainSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.exaggeration.to_bits(),value.exaggeration.to_bits());assert_eq!(restored.imported_map,value.imported_map);assert_eq!(restored.mesh,value.mesh);let bytes=export_sqlite_database(&database,limits,&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');d.query('UPDATE gis_terrain_mesh_child SET child_id=?').run('independent semantic edit');await Bun.write(Bun.stdout,d.serialize());d.close();";let mut p=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();p.stdin.take().unwrap().write_all(&bytes).unwrap();let output=p.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let edited=GisTerrainSnapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(edited.exaggeration.to_bits(),value.exaggeration.to_bits());assert_eq!(edited.mesh.unwrap().child_id,"independent semantic edit");}}
#[test]
fn sqlite_snapshot_gis_terrain_complete_native_record_and_actual_erased_provider(){let(f,mut value)=fixture();let limits=SqliteDatabaseLimits::default();let provider=<GisTerrainSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.gis.gisterrain".into(),standard:"1".into(),subset:"*".into()};for word in f["words"].as_array().unwrap(){value.exaggeration=f64::from_bits(u64::from_str_radix(word.as_str().unwrap(),16).unwrap());let expected=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(store::ArtifactPack::encode_pack(&value)),SnapshotEncoding::Text=>store::io::IoPayload::Text(store::ArtifactDsl::print_dsl(&value))};assert_eq!((provider.export)(crate::GIS_3D_TERRAIN_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,expected);let restored=(provider.import)(crate::GIS_3D_TERRAIN_SCHEMA,&dialect,expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;assert_eq!((provider.export)(crate::GIS_3D_TERRAIN_SCHEMA,&dialect,&restored,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,expected);}}}
#[test]
fn sqlite_snapshot_gis_terrain_exact_profile_and_resource_admission(){let(_,value)=fixture();let limits=SqliteDatabaseLimits::default();let database=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.gis.gisterrain".into(),standard:"1".into(),subset:"*".into()};assert!(value.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok());let mut wrong=dialect;wrong.subset="unknown".into();assert!(value.validate_sqlite_snapshot_subset(&wrong,&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for limits in[SqliteDatabaseLimits{max_rows:2,..limits},SqliteDatabaseLimits{max_value_bytes:16,..limits}]{assert!(value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(GisTerrainSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}assert!(value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());assert!(GisTerrainSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_gisterrain_real_declaration_public_typed_io() {
    use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route,store::io::io_mechanism::io_run_with_snapshot_control};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("gis").label("GIS SQLite declaration").version("0.0.1").package_id("semio:gis").artifact(crate::declaration().unwrap()).try_build().unwrap();
    let dialect=ArtifactDialect{artifact_kind:"s.gis.gisterrain".into(),standard:"1".into(),subset:"*".into()};
    let (_,snapshot)=fixture();
    let limits=SqliteDatabaseLimits::default();
    let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
    let sqlite=ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);
    let export=io_route(&dialect,&sqlite,1).await.unwrap().value;
    let import=io_route(&sqlite,&dialect,1).await.unwrap().value;
    for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{
        let mut phases=Vec::new();
        let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,limits,&mut |p|{phases.push(p.phase);true}).await.unwrap().value;
        let restored=io_import_sqlite_snapshot::<GisTerrainSnapshot>(&dialect,&bytes,limits,&mut |_|true).await.unwrap().value;
        assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
        assert!(!phases.iter().any(|p|matches!(p,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative|store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative)));
        let payload=match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>store::io::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
        let file=io_run_with_snapshot_control(&export,payload,limits,&mut |_|true).await.unwrap().value;
        let payload=io_run_with_snapshot_control(&import,file,limits,&mut |_|true).await.unwrap().value;
        let decoded=GisTerrainSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
        assert_eq!(decoded.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
        <GisTerrainSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(decoded);
    }
}

#[test]
fn sqlite_snapshot_gis_terrain_whole_native_controls_admission_and_interior_cancellation(){let(_,mut value)=fixture();value.exaggeration=f64::from_bits(0xfff0000000001234);value.imported_map=Some(crate::schema::ImportedMap{properties:vec![crate::schema::ImportedProperty{name:"literal".into(),value:semio_framework_value::DslValue::String("literal intrinsic text 世界".repeat(10000))}],..Default::default()});let limits=SqliteDatabaseLimits::default();for payload in[store::io::IoPayload::Binary(store::ArtifactPack::encode_pack(&value)),store::io::IoPayload::Text(store::ArtifactDsl::print_dsl(&value))]{let restored=GisTerrainSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.exaggeration.to_bits(),value.exaggeration.to_bits());assert_eq!(restored.imported_map,value.imported_map);assert_eq!(restored.mesh,value.mesh);for limits in[SqliteDatabaseLimits{max_rows:2,..limits},SqliteDatabaseLimits{max_value_bytes:4096,..limits}]{assert!(GisTerrainSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err())}let mut interior=false;assert!(GisTerrainSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative&&p.completed>=256&&p.completed<p.total{interior=true;false}else{true}},limits)).is_err());assert!(interior);}}
#[test]
fn sqlite_snapshot_terrain_explicit_native_output_boundary_preserves_complete_owned_state() {
 let (_,snapshot)=fixture();
 let limits=SqliteDatabaseLimits::default();
 let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let payload=<GisTerrainSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  let restored=<GisTerrainSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  <GisTerrainSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored);
 }
 <GisTerrainSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
#[test]
fn sqlite_snapshot_terrain_explicit_native_schema_and_row_admission_precedes_work() {
 let (_,snapshot)=fixture();
 let limits=SqliteDatabaseLimits::default();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let mut native_work=false;
  let limited=SqliteDatabaseLimits{max_rows:rows-1,..limits};
  let result=<GisTerrainSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>0{native_work=true;}true},limited));
  assert!(result.is_err());assert!(!native_work);
  let mut native_work=false;
  let limited=SqliteDatabaseLimits{max_schema_bytes:<GisTerrainSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA.len()-1,..limits};
  let result=<GisTerrainSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>0{native_work=true;}true},limited));
  assert!(result.is_err());assert!(!native_work);
 }
 <GisTerrainSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
#[test]
fn sqlite_snapshot_terrain_explicit_native_output_exact_file_frontier_and_unicode_cancel() {
 let (_,mut snapshot)=fixture();snapshot.imported_map=Some(crate::schema::ImportedMap{properties:vec![crate::schema::ImportedProperty{name:"literal".into(),value:semio_framework_value::DslValue::String("interior 世界".repeat(20000))}],..Default::default()});
 let limits=SqliteDatabaseLimits::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let payload=<GisTerrainSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  let length=match &payload{store::io::IoPayload::Binary(bytes)=>bytes.len(),store::io::IoPayload::Text(text)=>text.len()};
  assert!(<GisTerrainSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:length,..limits})).is_ok());
  assert!(<GisTerrainSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:length-1,..limits})).is_err());
  let mut interior=false;
  assert!(<GisTerrainSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>=256&&p.completed<p.total{interior=true;false}else{true}},limits)).is_err());
  assert!(interior);
 }
 <GisTerrainSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
