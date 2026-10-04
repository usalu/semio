//! 🎬️ The exact registered Sequence parent snapshot ownership laws.
use super::SequenceSnapshot;

#[test]
fn sqlite_snapshot_sequence_demo_matches_the_actual_parent_native_printer(){
 let expected=super::default_persisted_snapshot();
 assert_eq!(store::ArtifactDsl::print_dsl(&expected),include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio"));
}

use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
use semio_framework_plugin::{PluginApp,VcsArtifactApp,EditorApp,ViewerApp,__semio_dispatch_PluginApp,plugin_app_close_prelude::*};
semio_framework_dispatch_macros::dyn_enum_close!{
 /// 🎬️ The real Sequence editor and viewer declaration closure.
 enum SqliteApps:PluginApp{
  Editor(VcsArtifactApp<EditorApp<crate::editor::sequence::SequencePlayApp>,semio_s_artifact_stdio_semio::SemioMembers>),
  Viewer(VcsArtifactApp<ViewerApp<crate::viewer::sequence::SequenceViewer>,semio_s_artifact_stdio_semio::SemioMembers>),
 }
}
fn laws()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn fixture()->SequenceSnapshot{semio_framework_pack_json::from_json_str(&laws()["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn database(snapshot:&SequenceSnapshot)->SqliteDatabase{snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(database:&SqliteDatabase)->SequenceSnapshot{SequenceSnapshot::from_sqlite_database(database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn file(snapshot:&SequenceSnapshot)->Vec<u8>{export_sqlite_database(&database(snapshot),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()}
fn native(snapshot:&SequenceSnapshot,encoding:SnapshotEncoding)->store::io_schema::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(snapshot)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(snapshot))}}
fn dialect()->store::io_schema::ArtifactDialect{store::io_schema::ArtifactDialect{artifact_kind:"s.sequence.sequence".into(),standard:"1".into(),subset:"*".into()}}

#[test]
fn sqlite_snapshot_sequence_complete_parent_and_empty_reference_fields_are_exact(){
 let expected=fixture();let d=database(&expected);assert_eq!(d.tables.len(),2);assert_eq!(restore(&import_sqlite_database(&file(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
 let empty:SequenceSnapshot=semio_framework_pack_json::from_json_str(r#"{"schema":"","content":{"childId":"","target":{"artifactId":"","dialect":{"artifactKind":"","standard":"","subset":""}}}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 assert_eq!(restore(&database(&empty)),empty);
 let materialized=super::default_snapshot();let projected=database(&materialized);let restored=restore(&projected);assert_eq!(restored,materialized);assert!(restored.content.require_local_owner::<crate::SequenceWorkingScene>().is_err());neural_engine::ColdRetire::retire_cold(materialized);
}

#[test]
fn sqlite_snapshot_sequence_all_owned_reference_domains_survive_both_erased_directions(){
 let codec=crate::standards::v1::subsets::any::io::io().native.codec;let provider=codec.snapshot_sqlite.unwrap();
 for text in["","id!kind@standard/subset%","世界\0","😀"]{
  let value=serde_json::json!({"schema":text,"content":{"childId":text,"target":{"artifactId":text,"dialect":{"artifactKind":text,"standard":text,"subset":text}}}});
  let expected:SequenceSnapshot=semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let d=(provider.export)(&codec.schema,&dialect(),&native(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(d,database(&expected));
   let d=import_sqlite_database(&export_sqlite_database(&d,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
   let payload=(provider.import)(&codec.schema,&dialect(),d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
   assert_eq!(SequenceSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
  }
 }
}

#[test]
fn sqlite_snapshot_sequence_independent_query_and_reference_edits_are_semantic(){
 use std::{io::Write,process::{Command,Stdio}};let mut expected=fixture();
 let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('SELECT artifact_id,artifact_kind,standard,subset FROM sequence_content').get();if(row.artifact_id!=='artifact 世界\0'||row.artifact_kind!=='s.stdio.semio'||row.standard!=='v1'||row.subset!=='flow')throw Error('literal identity');db.run('UPDATE sequence_document SET schema=?',['edited 世界']);db.run('UPDATE sequence_content SET id=99,child_id=?,artifact_id=?,artifact_kind=?,standard=?,subset=?',['new child','new artifact','s.stdio.json','rfc8259','*']);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file(&expected)).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 expected.schema="edited 世界".into();expected.content=store::ArtifactChild::new("new child".into(),store::io_schema::ArtifactRef{artifact_id:"new artifact".into(),dialect:store::io_schema::ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"*".into()}});
 assert_eq!(restore(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}

#[test]
fn sqlite_snapshot_sequence_independent_malformed_sql_rejects_shape_parents_and_types(){
 use std::{io::Write,process::{Command,Stdio}};
 let bytes=file(&fixture());let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(new Uint8Array(input.bytes));db.run('PRAGMA ignore_check_constraints=ON');db.run(input.sql);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 for sql in laws()["malformedSql"].as_array().unwrap(){
  let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"bytes":bytes,"sql":sql}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{sql}: {}",String::from_utf8_lossy(&output.stderr));
  assert!(import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).and_then(|d|SequenceSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))).is_err(),"{sql}");
 }
}

#[test]
fn sqlite_snapshot_sequence_caller_limits_precede_complete_native_and_relational_ownership(){
 let expected=fixture();let d=database(&expected);let limits=SqliteDatabaseLimits::default();
 assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:1,..limits})).is_err());
 assert!(SequenceSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{for restricted in[SqliteDatabaseLimits{max_rows:1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits},SqliteDatabaseLimits{max_file_bytes:1,..limits}]{assert!(SequenceSnapshot::decode_sqlite_snapshot_native(&native(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err());}assert!(expected.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1024,..limits})).is_err());}
}

#[test]
fn sqlite_snapshot_sequence_initial_and_large_text_cancellation_reaches_all_native_phases(){
 let mut snapshot=fixture();let limits=SqliteDatabaseLimits::default();assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());assert!(SequenceSnapshot::from_sqlite_database(&database(&snapshot),&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 snapshot.content.child_id="😀".repeat(laws()["control"]["largeTextBytes"].as_u64().unwrap()as usize);let d=database(&snapshot);
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{
  let mut reached=false;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>0&&(event.total==0||event.total>=65536){reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut progress,limits);
  let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>SequenceSnapshot::from_sqlite_database(&d,&mut control).is_err(),SqliteSnapshotPhase::EncodeNative=>snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).is_err(),_=>SequenceSnapshot::decode_sqlite_snapshot_native(&native(&snapshot,SnapshotEncoding::Text),&mut control).is_err()};assert!(rejected,"{phase:?}");assert!(reached,"{phase:?}");
 }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_sequence_actual_declaration_mount_routes_queryable_files(){
 use store::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};
 semio_framework_plugin::Plugin::<SqliteApps>::builder("sequence").label("Sequence owned SQLite").version("0.0.1").package_id("semio:sequence").declare_artifact(crate::artifact::<SqliteApps>()).try_build().unwrap();
 assert!(store::document_codec(crate::SEQUENCE_DOCUMENT_SCHEMA).await.unwrap().unwrap().snapshot_sqlite.is_some());
 let expected=fixture();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect(),&expected,encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert!(bytes.starts_with(b"SQLite format 3\0"));assert_eq!(io_import_sqlite_snapshot::<SequenceSnapshot>(&dialect(),&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,expected);}
}


#[test]
fn sqlite_snapshot_sequence_actual_native_declaration_exposes_relational_capability(){
 let codec=crate::standards::v1::subsets::any::io::io().native.codec;
 assert!(codec.snapshot_sqlite.is_some(),"Sequence native declaration has no relational snapshot capability");
}

#[test]
fn sqlite_snapshot_sequence_neutral_json_retains_every_child_reference_field(){
 let laws:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let expected=&laws["snapshot"];
 let snapshot:SequenceSnapshot=semio_framework_pack_json::from_json_str(&expected.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&snapshot)).unwrap(),*expected);
 assert_eq!(snapshot.content.target.artifact_id,"artifact 世界\0");assert_eq!(snapshot.content.child_id,"child 世界\0; content");
}

#[test]
fn sqlite_snapshot_sequence_native_child_carrier_preserves_all_owned_string_domains(){
 for text in["","id!kind@standard/subset%","世界\0","😀"]{
  let value=serde_json::json!({"schema":text,"content":{"childId":text,"target":{"artifactId":text,"dialect":{"artifactKind":text,"standard":text,"subset":text}}}});
  let expected:SequenceSnapshot=semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let pack=store::ArtifactPack::encode_pack(&expected);
  let packed=<SequenceSnapshot as store::ArtifactPack>::decode_pack(&pack).unwrap();
  assert_eq!(packed,expected);
  let printed=store::ArtifactDsl::print_dsl(&expected);
  let parsed=<SequenceSnapshot as store::ArtifactDsl>::parse_dsl(&printed).unwrap();
  assert_eq!(parsed,expected);
 }
}

#[test]
fn sqlite_snapshot_sequence_genuine_output_admits_exact_row_and_file_frontiers(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding,export_sqlite_database}};
 use std::{io::Write,process::{Command,Stdio}};
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let rows=expected.tables.iter().map(|table|table.rows.len()).sum::<usize>();assert!(rows>0);
 let bytes=export_sqlite_database(&expected,limits,&mut |_|true).unwrap();
 let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all();const rows=tables.reduce((n,t)=>n+Number(db.query('SELECT COUNT(*) AS count FROM "'+t.name.replaceAll('"','""')+'"').get().count),0);await Bun.write(Bun.stdout,String(rows));db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(String::from_utf8(output.stdout).unwrap().parse::<usize>().unwrap(),rows);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,..limits})).unwrap();
  let physical=match &payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};assert!(physical>0);
  let repeated=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,max_file_bytes:physical,..limits})).unwrap();assert_eq!(repeated,payload);
  let restored=SequenceSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 }
}
