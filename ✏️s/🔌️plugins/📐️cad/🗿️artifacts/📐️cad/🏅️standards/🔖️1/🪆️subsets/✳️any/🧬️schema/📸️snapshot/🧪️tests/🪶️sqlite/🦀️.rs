//! 📐️ Genuine CAD parent native baselines before semantic provider opt-in.
#[path="../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod backing_observer;
fn complete_literal_cad(actual:&CadSnapshot,expected:&CadSnapshot){
 assert_eq!(actual.schema,expected.schema);assert_eq!(actual.id,expected.id);
 assert_eq!(actual.shape_model,expected.shape_model);assert_eq!(actual.building_model,expected.building_model);assert_eq!(actual.energy_model,expected.energy_model);assert_eq!(actual.structure_classic_model,expected.structure_classic_model);
 assert_eq!(actual.drawings,expected.drawings);assert_eq!(actual.nodes,expected.nodes);
 assert_eq!(actual.references_by_model_definition_id.len(),expected.references_by_model_definition_id.len());
 for(key,expected_rows)in &expected.references_by_model_definition_id{
  let actual_rows=actual.references_by_model_definition_id.get(key).expect("literal map group");assert_eq!(actual_rows.len(),expected_rows.len());
  for(actual,expected)in actual_rows.iter().zip(expected_rows){assert_eq!((&actual.id,&actual.source_url,&actual.media_kind,actual.hidden,actual.locked),(&expected.id,&expected.source_url,&expected.media_kind,expected.hidden,expected.locked));assert_eq!(actual.origin.map(f64::to_bits),expected.origin.map(f64::to_bits));assert_eq!(actual.orientation.map(|q|q.map(f64::to_bits)),expected.orientation.map(|q|q.map(f64::to_bits)));assert_eq!(actual.scale.map(f64::to_bits),expected.scale.map(f64::to_bits));assert_eq!(actual.width_world.to_bits(),expected.width_world.to_bits());assert_eq!(actual.opacity.map(f64::to_bits),expected.opacity.map(f64::to_bits));}
 }
}
#[test]
fn sqlite_snapshot_cad_parent_projection_settles_actual_system_backing_requests(){
 let expected=fixture();let wanted=database(&expected);let limits=SqliteDatabaseLimits::default();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,limits);
 let(result,observed)=backing_observer::measure(||expected.to_sqlite_database(&mut control));assert_eq!(result.unwrap(),wanted);assert!(observed.bytes>0&&observed.requests>0);assert_eq!(limits.max_allocation_bytes-control.allocation_remaining_bytes(),observed.bytes,"ProjectSnapshot full concrete request settlement");
}
#[test]
fn sqlite_snapshot_cad_parent_reconstruction_settles_actual_system_backing_requests(){
 let expected=fixture();let source=database(&expected);let limits=SqliteDatabaseLimits::default();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,limits);
 let(result,observed)=backing_observer::measure(||CadSnapshot::from_sqlite_database(&source,&mut control));complete_literal_cad(&result.unwrap(),&expected);assert!(observed.bytes>0&&observed.requests>0);assert_eq!(limits.max_allocation_bytes-control.allocation_remaining_bytes(),observed.bytes,"ReconstructSnapshot full concrete request settlement");
}
#[test]
fn sqlite_snapshot_cad_parent_all_fields_raw_words_and_cumulative_exact_backing(){
 let neutral=laws();assert_eq!(neutral["ownership"]["semanticBudget"],"completeSqlScalarBytes");assert_eq!(neutral["ownership"]["backingBudget"],"cumulativeSystemAllocatorRequests");
 for hex in neutral["binary64Words"].as_array().unwrap(){let mut expected=fixture();fill(&mut expected,f64::from_bits(u64::from_str_radix(hex.as_str().unwrap(),16).unwrap()));let source=database(&expected);complete_literal_cad(&restore(&source),&expected);backing_observer::verify_snapshot_backing_by(&expected,&source,complete_literal_cad);}
}
use super::CadSnapshot;
fn laws()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn fixture()->CadSnapshot{semio_framework_pack_json::from_json_str(&laws()["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn canonical_children(snapshot:&mut CadSnapshot){for child in[&mut snapshot.shape_model,&mut snapshot.building_model,&mut snapshot.energy_model,&mut snapshot.structure_classic_model].into_iter().flatten(){child.child_id=child.target.artifact_id.clone()}for child in &mut snapshot.drawings{child.child_id=child.target.artifact_id.clone()}}
#[test]
fn sqlite_snapshot_cad_parent_actual_native_factory_exposes_owned_relational_capability(){
 let codec=store::ArtifactCodec::bare::<CadSnapshot,crate::CadMutation>(crate::CAD_DOCUMENT_SCHEMA);assert!(codec.snapshot_sqlite.is_some(),"CAD parent native factory lacks semantic SQLite capability");
}
#[test]
fn sqlite_snapshot_cad_parent_independent_literal_child_aliases_survive_both_native_formats(){
 let expected=fixture();assert_ne!(expected.shape_model.as_ref().unwrap().child_id,expected.shape_model.as_ref().unwrap().target.artifact_id);
 for text in[false,true]{let actual=if text{<CadSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&expected)).unwrap()}else{<CadSnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&expected)).unwrap()};assert_eq!(actual,expected)}
}
#[test]
fn sqlite_snapshot_cad_parent_all_reference_numeric_slots_preserve_every_raw_word(){
 for hex in laws()["binary64Words"].as_array().unwrap(){let word=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let value=f64::from_bits(word);let mut expected=fixture();canonical_children(&mut expected);
  for rows in expected.references_by_model_definition_id.values_mut(){for row in rows{row.origin=[value;3];row.orientation=Some([value;4]);row.scale=Some(value);row.width_world=value;row.opacity=Some(value)}}
  for text in[false,true]{let actual=if text{<CadSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&expected)).unwrap()}else{<CadSnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&expected)).unwrap()};assert_eq!(actual.nodes,expected.nodes);assert_eq!(actual.drawings,expected.drawings);assert_eq!(actual.references_by_model_definition_id.len(),3);for(key,rows)in &actual.references_by_model_definition_id{assert_eq!(rows.len(),expected.references_by_model_definition_id[key].len());for row in rows{let q=row.orientation.unwrap();for number in[row.origin[0],row.origin[1],row.origin[2],q[0],q[1],q[2],q[3],row.scale.unwrap(),row.width_world,row.opacity.unwrap()]{assert_eq!(number.to_bits(),word,"{hex}: full native CAD reference numeric identity")}}}
  }
 }
}

use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SqliteSnapshotProgress,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
fn database(value:&CadSnapshot)->SqliteDatabase{value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(value:&SqliteDatabase)->CadSnapshot{CadSnapshot::from_sqlite_database(value,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn file(value:&CadSnapshot)->Vec<u8>{export_sqlite_database(&database(value),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()}
fn dialect()->store::io_schema::ArtifactDialect{store::io_schema::ArtifactDialect{artifact_kind:"s.cad.cad".into(),standard:"1".into(),subset:"*".into()}}
fn payload(value:&CadSnapshot,encoding:SnapshotEncoding)->store::io_schema::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(value)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(value))}}
fn oracle(script:&str,input:&[u8])->Vec<u8>{use std::{io::Write,process::{Command,Stdio}};let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(input).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));output.stdout}
fn words(value:&CadSnapshot)->Vec<Option<u64>>{let mut output=Vec::new();for rows in value.references_by_model_definition_id.values(){for row in rows{output.extend(row.origin.map(|v|Some(v.to_bits())));output.extend(row.orientation.map(|q|q.map(|v|Some(v.to_bits()))).unwrap_or([None;4]));output.extend([row.scale.map(f64::to_bits),Some(row.width_world.to_bits()),row.opacity.map(f64::to_bits)]);}}output}
fn fill(value:&mut CadSnapshot,number:f64){for rows in value.references_by_model_definition_id.values_mut(){for row in rows{row.origin=[number;3];row.orientation=Some([number;4]);row.scale=Some(number);row.width_world=number;row.opacity=Some(number)}}}
#[test]
fn sqlite_snapshot_cad_parent_complete_and_empty_owned_domains_retain_queryable_fields(){
 let expected=fixture();let d=database(&expected);assert_eq!(d.tables.len(),6);assert_eq!(d.tables.iter().map(|t|t.rows.len()).sum::<usize>(),18);assert_eq!(restore(&d),expected);assert_eq!(restore(&import_sqlite_database(&file(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
 let mut empty=expected.clone();empty.shape_model=None;empty.building_model=None;empty.energy_model=None;empty.structure_classic_model=None;empty.drawings.clear();empty.nodes.clear();for rows in empty.references_by_model_definition_id.values_mut(){rows.clear()}assert_eq!(database(&empty).tables.iter().map(|t|t.rows.len()).sum::<usize>(),4);assert_eq!(restore(&database(&empty)),empty);
}
#[test]
fn sqlite_snapshot_cad_parent_every_numeric_word_survives_both_erased_directions(){
 let codec=store::ArtifactCodec::bare::<CadSnapshot,crate::CadMutation>(crate::CAD_DOCUMENT_SCHEMA);let owner=codec.snapshot_sqlite.unwrap();
 for hex in laws()["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=fixture();fill(&mut expected,f64::from_bits(bits));assert_eq!(words(&expected),vec![Some(bits);40]);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let d=(owner.export)(&codec.schema,&dialect(),&payload(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(words(&restore(&d)),vec![Some(bits);40]);let output=(owner.import)(&codec.schema,&dialect(),d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let actual=CadSnapshot::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(words(&actual),vec![Some(bits);40]);assert_eq!(actual.nodes,expected.nodes);assert_eq!(actual.drawings,expected.drawings);}}
}
#[test]
fn sqlite_snapshot_cad_parent_independent_queries_and_surrogate_edits_preserve_literal_groups(){
 let mut expected=fixture();let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(db.query('SELECT count(*) AS count FROM cad_reference').get().count!==4n||db.query('SELECT count(*) AS count FROM cad_reference_group').get().count!==3n)throw Error('full domains');db.run('UPDATE cad_document SET schema=?',['new\0😀']);db.run('UPDATE cad_reference_group SET id=id+90');db.run('UPDATE cad_reference SET group_id=group_id+90,id=id+90');db.run('UPDATE cad_model_child SET id=id+90');db.run('UPDATE cad_drawing_child SET id=id+90');db.run('UPDATE cad_node SET id=id+90');await Bun.write(Bun.stdout,db.serialize());db.close();"#;let bytes=oracle(script,&file(&expected));expected.schema="new\0😀".into();assert_eq!(restore(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}
#[test]
fn sqlite_snapshot_cad_parent_independent_malformed_edits_refuse_owned_relations(){
 let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const output=[];for(const sql of input.sql){const db=Database.deserialize(new Uint8Array(input.bytes));db.run('PRAGMA ignore_check_constraints=ON');db.run(sql);output.push(Array.from(db.serialize()));db.close()}await Bun.write(Bun.stdout,JSON.stringify(output));"#;let laws=laws();let edited:Vec<Vec<u8>>=serde_json::from_slice(&oracle(script,serde_json::json!({"bytes":file(&fixture()),"sql":laws["malformedSql"]}).to_string().as_bytes())).unwrap();assert_eq!(edited.len(),21);for(sql,bytes)in laws["malformedSql"].as_array().unwrap().iter().zip(edited){assert!(import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).and_then(|d|CadSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))).is_err(),"{sql}")}
}
#[test]
fn sqlite_snapshot_cad_parent_exact_row_and_genuine_physical_output_admission(){
 let expected=fixture();let d=database(&expected);let limits=SqliteDatabaseLimits::default();assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:18,..limits})).is_ok());assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:17,..limits})).is_err());assert!(CadSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let output=expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let bytes=match &output{store::io_schema::IoPayload::Binary(v)=>v.len(),store::io_schema::IoPayload::Text(v)=>v.len()};assert!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:18,max_file_bytes:bytes,..limits})).is_ok());for restricted in[SqliteDatabaseLimits{max_rows:17,..limits},SqliteDatabaseLimits{max_file_bytes:bytes-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err());assert!(CadSnapshot::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err());}}
}
#[test]
fn sqlite_snapshot_cad_parent_initial_cancellation_reaches_all_four_owned_phases(){
 let expected=fixture();let d=database(&expected);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{let mut callback=|_|false;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let result=match phase{SqliteSnapshotPhase::ProjectSnapshot=>expected.to_sqlite_database(&mut control).map(|_|()),SqliteSnapshotPhase::ReconstructSnapshot=>CadSnapshot::from_sqlite_database(&d,&mut control).map(|_|()),SqliteSnapshotPhase::DecodeNative=>CadSnapshot::decode_sqlite_snapshot_native(&payload(&expected,SnapshotEncoding::Text),&mut control).map(|_|()),SqliteSnapshotPhase::EncodeNative=>expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).map(|_|()),_=>panic!("unexpected")};assert!(result.is_err(),"{phase:?}");}
}
#[test]
fn sqlite_snapshot_cad_parent_long_unicode_is_cancellable_inside_every_owned_phase(){
 let mut expected=fixture();expected.schema="😀".repeat(laws()["control"]["largeCharacters"].as_u64().unwrap()as usize);let d=database(&expected);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{let mut reached=false;let mut callback=|event:SqliteSnapshotProgress|if event.phase==phase&&event.total>=65536&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let result=match phase{SqliteSnapshotPhase::ProjectSnapshot=>expected.to_sqlite_database(&mut control).map(|_|()),SqliteSnapshotPhase::ReconstructSnapshot=>CadSnapshot::from_sqlite_database(&d,&mut control).map(|_|()),SqliteSnapshotPhase::DecodeNative=>CadSnapshot::decode_sqlite_snapshot_native(&payload(&expected,SnapshotEncoding::Text),&mut control).map(|_|()),SqliteSnapshotPhase::EncodeNative=>expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).map(|_|()),_=>panic!("unexpected")};assert!(result.is_err(),"{phase:?}");drop(control);assert!(reached,"{phase:?}");}
}
use semio_framework_plugin::{PluginApp,VcsArtifactApp,EditorApp,ViewerApp,__semio_dispatch_PluginApp,plugin_app_close_prelude::*};
semio_framework_dispatch_macros::dyn_enum_close!{
 /// 📐️ The actual CAD editor and viewer app roster.
 enum SqliteApps:PluginApp{
  Editor(VcsArtifactApp<EditorApp<crate::editor::cad::CadPlayApp>,semio_s_artifact_stdio_semio::SemioMembers>),
  Viewer(VcsArtifactApp<ViewerApp<crate::viewer::cad::CadViewer>,semio_s_artifact_stdio_semio::SemioMembers>),
 }
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_cad_parent_actual_native_declaration_routes_queryable_files(){
 semio_framework_plugin::Plugin::<SqliteApps>::builder("cad").label("CAD owned SQLite").version("0.0.1").package_id("semio:cad").artifact(crate::declaration().unwrap()).try_build().unwrap();let codec=store::document_codec(crate::CAD_DOCUMENT_SCHEMA).await.unwrap().unwrap();assert!(codec.snapshot_sqlite.is_some());let expected=fixture();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(),&expected,encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert!(bytes.starts_with(b"SQLite format 3\0"));let actual=store::io::io_mechanism::io_import_sqlite_snapshot::<CadSnapshot>(&dialect(),&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert_eq!(actual,expected);}
}
