//! 🌿️ Native VCS declaration and full semantic SQLite ownership laws.
use super::VcsSnapshot;

use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SqliteValue,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
use semio_framework_plugin::{PluginApp,VcsArtifactApp,EditorApp,ViewerApp,__semio_dispatch_PluginApp,plugin_app_close_prelude::*};
semio_framework_dispatch_macros::dyn_enum_close!{
 /// 🌿️ The real VCS declaration's editor and viewer application closure.
 enum SqliteApps:PluginApp{
  Editor(VcsArtifactApp<EditorApp<crate::editor::vcs::VcsPlayApp>>),
  Viewer(VcsArtifactApp<ViewerApp<crate::viewer::vcs::VcsViewer>>),
 }
}
fn laws()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn fixture()->VcsSnapshot{semio_framework_pack_json::from_json_str(&laws()["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn database(snapshot:&VcsSnapshot)->SqliteDatabase{snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(database:&SqliteDatabase)->VcsSnapshot{VcsSnapshot::from_sqlite_database(database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn file(snapshot:&VcsSnapshot)->Vec<u8>{export_sqlite_database(&database(snapshot),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()}
fn native(snapshot:&VcsSnapshot,encoding:SnapshotEncoding)->store::io_schema::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(snapshot)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(snapshot))}}
fn dialect()->store::io_schema::ArtifactDialect{store::io_schema::ArtifactDialect{artifact_kind:"s.vcs.vcs".into(),standard:"1".into(),subset:"*".into()}}

#[test]
fn sqlite_snapshot_vcs_complete_and_empty_domains_retain_all_native_fields(){
 let snapshot=fixture();let bytes=file(&snapshot);assert_eq!(restore(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),snapshot);
 let mut empty=snapshot;empty.tags.clear();empty.schema.clear();empty.status.clear();let d=database(&empty);assert_eq!(d.tables.len(),2);assert_eq!(restore(&d),empty);
}

#[test]
fn sqlite_snapshot_vcs_every_signed64_word_survives_both_declared_erased_directions(){
 let codec=crate::standards::v1::subsets::any::io::io().native.codec;let provider=codec.snapshot_sqlite.unwrap();
 for word in laws()["signed64"].as_array().unwrap(){let mut expected=fixture();expected.counter=word.as_str().unwrap().parse().unwrap();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let d=(provider.export)(&codec.schema,&dialect(),&native(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(d,database(&expected));
  let d=import_sqlite_database(&export_sqlite_database(&d,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  let payload=(provider.import)(&codec.schema,&dialect(),d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
  assert_eq!(VcsSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
 }}
}

#[test]
fn sqlite_snapshot_vcs_independent_sql_queries_and_edits_have_no_native_carrier(){
 use std::{io::Write,process::{Command,Stdio}};
 let mut expected=fixture();let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(db.query('SELECT counter FROM vcs_document').get().counter!==-42n)throw Error('native counter');const rows=db.query('SELECT value FROM vcs_tag JOIN vcs_document ON vcs_document.id=vcs_tag.document_id ORDER BY ordinal').all();if(rows.length!==6||rows[0].value!=='review'||rows[1].value!==''||rows[2].value!=='review'||rows[3].value!=='a;b'||rows[4].value!=='NUL\0世界')throw Error('ordered tags');db.run('UPDATE vcs_document SET counter=?,title=?,status=?',[9223372036854775807n,'Geändert 世界','any native status']);db.run('UPDATE vcs_tag SET id=id+100');db.run('UPDATE vcs_tag SET value=? WHERE ordinal=0',['edited; tag']);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file(&expected)).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 expected.counter=i64::MAX;expected.title="Geändert 世界".into();expected.status="any native status".into();expected.tags[0]="edited; tag".into();assert_eq!(restore(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}

#[test]
fn sqlite_snapshot_vcs_independent_malformed_sql_refuses_exact_shape_parents_ordinals_and_types(){
 use std::{io::Write,process::{Command,Stdio}};
 let bytes=file(&fixture());let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(new Uint8Array(input.bytes));db.run('PRAGMA ignore_check_constraints=ON');db.run(input.sql);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 for sql in laws()["malformedSql"].as_array().unwrap(){
  let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();let input=serde_json::json!({"bytes":bytes,"sql":sql});child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{sql}: {}",String::from_utf8_lossy(&output.stderr));
  assert!(import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).and_then(|d|VcsSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))).is_err(),"{sql}");
 }
 let mut wrong=database(&fixture());wrong.table_mut("vcs_document").unwrap().rows[0].values.push(SqliteValue::Integer(1));assert!(VcsSnapshot::from_sqlite_database(&wrong,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_vcs_caller_row_value_and_file_limits_precede_native_ownership(){
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:1,..limits})).is_err());
 assert!(VcsSnapshot::from_sqlite_database(&database(&snapshot),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=native(&snapshot,encoding);for restricted in[SqliteDatabaseLimits{max_value_bytes:1,..limits},SqliteDatabaseLimits{max_file_bytes:1,..limits},SqliteDatabaseLimits{max_rows:1,..limits}]{assert!(VcsSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err());}assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1024,..limits})).is_err());}
}

#[test]
fn sqlite_snapshot_vcs_initial_and_interior_control_cancels_every_expensive_phase(){
 let mut snapshot=fixture();let limits=SqliteDatabaseLimits::default();assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());assert!(VcsSnapshot::from_sqlite_database(&database(&snapshot),&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 snapshot.tags=(0..laws()["control"]["tagCount"].as_u64().unwrap()).map(|i|format!("tag {i}")).collect();let d=database(&snapshot);let cancel_at=laws()["control"]["cancelAt"].as_u64().unwrap()as usize;
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{
  let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=cancel_at{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,limits);
  let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>VcsSnapshot::from_sqlite_database(&d,&mut control).is_err(),SqliteSnapshotPhase::EncodeNative=>snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).is_err(),_=>VcsSnapshot::decode_sqlite_snapshot_native(&native(&snapshot,SnapshotEncoding::Binary),&mut control).is_err()};assert!(rejected,"{phase:?}");assert!(reached,"{phase:?}");
 }
}

#[test]
fn sqlite_snapshot_vcs_large_text_ownership_has_real_interior_copy_cancellation(){
 let mut snapshot=fixture();snapshot.notes="😀".repeat(laws()["control"]["largeTextBytes"].as_u64().unwrap()as usize);let d=database(&snapshot);
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{
  let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>0&&event.total>=65536{reached=true;false}else if event.phase==phase&&event.completed>0&&event.total==0{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());
  let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>VcsSnapshot::from_sqlite_database(&d,&mut control).is_err(),SqliteSnapshotPhase::EncodeNative=>snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).is_err(),_=>VcsSnapshot::decode_sqlite_snapshot_native(&native(&snapshot,SnapshotEncoding::Text),&mut control).is_err()};assert!(rejected,"{phase:?}");assert!(reached,"{phase:?}");
 }
}

#[test]
fn sqlite_snapshot_vcs_typed_tag_collection_declares_its_own_interior_workload(){
 let mut snapshot=fixture();let count=laws()["control"]["tagCount"].as_u64().unwrap()as usize;snapshot.tags=vec![String::new();count];let record=snapshot.__dsl_to_record();let mut reached=false;
 let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.total==count&&event.completed>=256{reached=true;false}else{true}};
 assert!(VcsSnapshot::__dsl_from_record_controlled(&record,&mut semio_framework_value::NativeDecodeControl::new(256*1024*1024,&mut progress)).is_err());assert!(reached);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_vcs_actual_declaration_mount_exports_and_imports_queryable_files(){
 use store::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};
 semio_framework_plugin::Plugin::<SqliteApps>::builder("vcs").label("VCS owned SQLite").version("0.0.1").package_id("semio:vcs").declare_artifact(crate::artifact::<SqliteApps>()).try_build().unwrap();
 let codec=store::document_codec(crate::VCS_DOCUMENT_SCHEMA).await.unwrap().unwrap();assert!(codec.snapshot_sqlite.is_some());
 let mut snapshot=fixture();for word in laws()["signed64"].as_array().unwrap(){snapshot.counter=word.as_str().unwrap().parse().unwrap();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect(),&snapshot,encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert!(bytes.starts_with(b"SQLite format 3\0"));assert_eq!(io_import_sqlite_snapshot::<VcsSnapshot>(&dialect(),&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);}}
}

#[test]
fn sqlite_snapshot_vcs_actual_native_declaration_exposes_relational_capability(){
 let codec=crate::standards::v1::subsets::any::io::io().native.codec;
 assert!(codec.snapshot_sqlite.is_some(),"VCS native declaration has no relational snapshot capability");
}

#[test]
fn sqlite_snapshot_vcs_neutral_native_fields_agree_with_independent_serde(){
 let laws:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 let text=laws["snapshot"].to_string();let snapshot:VcsSnapshot=semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 assert_eq!(serde_json::from_str::<VcsSnapshot>(&text).unwrap(),snapshot);
 for word in laws["signed64"].as_array().unwrap(){let word=word.as_str().unwrap();let text=format!("{{\"schema\":\"vcs.vcs\",\"title\":\"\",\"counter\":{word},\"notes\":\"\",\"status\":\"\",\"tags\":[]}}");assert_eq!(semio_framework_pack_json::from_json_str::<VcsSnapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(),serde_json::from_str::<VcsSnapshot>(&text).unwrap());}
 for word in laws["invalidSigned64"].as_array().unwrap(){let word=word.as_str().unwrap();let text=format!("{{\"schema\":\"vcs.vcs\",\"title\":\"\",\"counter\":{word},\"notes\":\"\",\"status\":\"\",\"tags\":[]}}");assert!(semio_framework_pack_json::from_json_str::<VcsSnapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());assert!(serde_json::from_str::<VcsSnapshot>(&text).is_err());}
}

#[test]
fn sqlite_snapshot_vcs_genuine_output_admits_exact_row_and_file_frontiers(){
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
  let restored=VcsSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 }
}
