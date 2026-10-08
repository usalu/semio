use crate::standards::v1::subsets::any::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
fn fixture()->(serde_json::Value,WriterSnapshot){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let s=&f["snapshot"];let child=&s["document"];let dialect=&child["target"]["dialect"];let snapshot=WriterSnapshot{schema:s["schema"].as_str().unwrap().into(),id:s["id"].as_str().unwrap().into(),language_id:s["languageId"].as_str().unwrap().into(),uri:s["uri"].as_str().unwrap().into(),text:s["text"].as_str().unwrap().into(),document:store::ArtifactChild::new(child["childId"].as_str().unwrap().into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:child["target"]["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:dialect["artifactKind"].as_str().unwrap().into(),standard:dialect["standard"].as_str().unwrap().into(),subset:dialect["subset"].as_str().unwrap().into()}})};(f,snapshot)}
#[test]
fn sqlite_snapshot_writer_owned_strings_child_identity_and_independent_sql_edit(){use std::io::Write;use std::process::{Command,Stdio};let(f,snapshot)=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(WriterSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const child=db.query('SELECT child_id,target_artifact_id,target_subset FROM writer_document_child').get();if(child.child_id!=='document-child'||child.target_artifact_id!=='document-target'||child.target_subset!=='document')throw Error('child identity');db.query('UPDATE writer_document SET text=?').run(JSON.parse(process.argv[1]));await Bun.write(Bun.stdout,db.serialize());db.close();";let mut process=Command::new("bun").args(["-e",script]).arg(serde_json::to_string(&f["editedText"]).unwrap()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();process.stdin.take().unwrap().write_all(&bytes).unwrap();let output=process.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let restored=WriterSnapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored.text,f["editedText"].as_str().unwrap());assert_eq!(restored.document,snapshot.document);}
#[test]
fn sqlite_snapshot_writer_actual_erased_codec_and_request_bounds(){use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::IoPayload};let(_,snapshot)=fixture();let codec=store::ArtifactCodec::bare::<WriterSnapshot,crate::WriterMutation>(crate::WRITER_DOCUMENT_SCHEMA);let provider=codec.snapshot_sqlite.as_ref().unwrap();let dialect=ArtifactDialect{artifact_kind:"s.writer.writer".into(),standard:"1".into(),subset:"*".into()};let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};let database=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(database,expected);let payload=(provider.import)(&codec.schema,&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=match payload{IoPayload::Binary(bytes)=><WriterSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),IoPayload::Text(text)=><WriterSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()};assert_eq!(restored,snapshot);let limits=SqliteDatabaseLimits{max_value_bytes:64,..SqliteDatabaseLimits::default()};assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());}

#[test]
fn sqlite_snapshot_writer_neutral_exact_coordinates_fixed_identity_and_request_limits(){let(f,snapshot)=fixture();let limits=SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();for value in f["invalidDialects"].as_array().unwrap(){let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()};assert!(snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.writer.writer".into(),standard:"1".into(),subset:"*".into()};let mut changed=database.clone();changed.table_mut("writer_document").unwrap().rows[0].values[1]=store::sqlite_snapshot::SqliteValue::Text("foreign schema".into());assert!(snapshot.validate_sqlite_snapshot_subset(&dialect,&changed,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for limited in[SqliteDatabaseLimits{max_rows:f["limitedRows"].as_u64().unwrap()as usize,..limits},SqliteDatabaseLimits{max_value_bytes:f["limitedValueBytes"].as_u64().unwrap()as usize,..limits}]{assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());assert!(WriterSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());}assert!(snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());}

use semio_framework_plugin::{PluginApp,__semio_dispatch_PluginApp,plugin_app_close_prelude::*};
semio_framework_dispatch_macros::dyn_enum_close!{
 enum WriterSqliteApps:PluginApp{
  Editor(semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::writer::WriterPlayApp>,semio_s_artifact_stdio_semio::SemioMembers>),
  Viewer(semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::writer::WriterViewer>,semio_s_artifact_stdio_semio::SemioMembers>),
 }
}
fn independent_writer_database(snapshot:&WriterSnapshot,statements:&serde_json::Value)->store::sqlite_snapshot::SqliteDatabase{
 use std::{process::{Command,Stdio},io::Write};
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));for(const sql of {statements})d.run(sql);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,d.serialize());d.close();");
 let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()
}
#[test]
fn sqlite_snapshot_writer_signed_aliases_and_complete_owned_field_guard(){
 let(f,snapshot)=fixture();let limits=SqliteDatabaseLimits::default();
 let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.writer.writer".into(),standard:"1".into(),subset:"*".into()};
 let database=independent_writer_database(&snapshot,&f["renumberSql"]);
 assert_eq!(WriterSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);
 snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for statement in f["mismatchSql"].as_array().unwrap(){
  let database=independent_writer_database(&snapshot,&serde_json::json!([statement]));
  assert!(snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
 }
}
#[test]
fn sqlite_snapshot_writer_whole_controlled_native_stages_child_identity_and_limits(){
 use store::sqlite_snapshot::SqliteSnapshotPhase;
 let(f,mut snapshot)=fixture();snapshot.schema="nondefault Writer owned schema".into();snapshot.text="世界".repeat(65536);let limits=SqliteDatabaseLimits::default();
 let mut states=vec![snapshot];for reference in f["referenceCases"].as_array().unwrap(){let mut state=states[0].clone();state.document=semio_framework_pack_json::from_json_str(&reference.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();states.push(state);}
 for snapshot in states{
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let restored=WriterSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored,snapshot);
  for limited in[SqliteDatabaseLimits{max_rows:1,..limits},SqliteDatabaseLimits{max_value_bytes:64,..limits}]{assert!(WriterSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());}
  let mut cancelled=false;assert!(WriterSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>0&&event.total>0{cancelled=true;false}else{true}},limits)).is_err());assert!(cancelled);
 }
 }
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_writer_real_owned_declaration_public_and_erased_io(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧩️declaration/🔣️.json")).unwrap();
 let declaration=crate::artifact::<WriterSqliteApps>();
 assert_eq!(declaration.kind.as_str(),law["dialect"]["artifactKind"].as_str().unwrap());assert_eq!(declaration.standards.len(),1);assert_eq!(declaration.standards[0].id.0,law["dialect"]["standard"].as_str().unwrap());assert_eq!(declaration.standards[0].subsets.len(),1);
 let subset=&declaration.standards[0].subsets[0];assert_eq!(subset.dialect.artifact_kind,law["dialect"]["artifactKind"].as_str().unwrap());assert_eq!(subset.dialect.standard.0,law["dialect"]["standard"].as_str().unwrap());assert_eq!(subset.dialect.subset.0,law["dialect"]["subset"].as_str().unwrap());
 let codec=subset.io.native.codec.clone();assert_eq!(codec.schema,law["schema"].as_str().unwrap());let declared=codec.snapshot_sqlite.clone().expect("actual declared Snapshot semantic provider");assert_eq!(declared.snapshot_type,Some(std::any::TypeId::of::<WriterSnapshot>()));assert!(!WriterSnapshot::SQLITE_SCHEMA.trim().is_empty());assert_eq!(declared.schema.as_ref(),WriterSnapshot::SQLITE_SCHEMA);let typed=store::ArtifactCodec::bare::<WriterSnapshot,crate::WriterMutation>(law["schema"].as_str().unwrap());assert!(declared.identical_to(typed.snapshot_sqlite.as_ref().unwrap()));
 use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route,store::io::io_mechanism::io_run_with_snapshot_control};
 let plugin=semio_framework_plugin::Plugin::<WriterSqliteApps>::builder("writer").label("Writer SQLite owned declaration").version("0.0.1").package_id("semio:writer").declare_artifact(declaration).try_build().unwrap();
 let installed=store::document_codec(&codec.schema).await.unwrap().expect("actual owning builder installs document codec");assert_eq!(installed.schema,codec.schema);assert!(declared.identical_to(installed.snapshot_sqlite.as_ref().unwrap()));let native=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:law["dialect"]["artifactKind"].as_str().unwrap().into(),standard:law["dialect"]["standard"].as_str().unwrap().into(),subset:law["dialect"]["subset"].as_str().unwrap().into()};assert_eq!(store::io::io_mechanism::native_snapshot_sqlite_schema(&native).unwrap(),WriterSnapshot::SQLITE_SCHEMA);let sqlite=semio_framework_artifact_reference::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);for(from,into)in[(&native,&sqlite),(&sqlite,&native)]{let route=store::io::io_mechanism::io_route(from,into,1).await.unwrap().value;assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,store::io_schema::IoFidelity::Exact);}
 eprintln!("[DEBUG] Writer actual generic declaration Snapshot={} schema={} TypeId={:?} sql_bytes={} installed_same_hooks=true direct_exact_directions=2",law["snapshot"].as_str().unwrap(),codec.schema,declared.snapshot_type,declared.schema.len());

 let(_,mut snapshot)=fixture();snapshot.schema="registered Writer owned state".into();let limits=SqliteDatabaseLimits::default();let dialect=ArtifactDialect{artifact_kind:"s.writer.writer".into(),standard:"1".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);
 let export=io_route(&dialect,&sqlite,1).await.unwrap().value;let import=io_route(&sqlite,&dialect,1).await.unwrap().value;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let file=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,limits,&mut |_|true).await.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<WriterSnapshot>(&dialect,&file,limits,&mut |_|true).await.unwrap().value,snapshot);
  let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let file=io_run_with_snapshot_control(&export,payload,limits,&mut |_|true).await.unwrap().value;let payload=io_run_with_snapshot_control(&import,file,limits,&mut |_|true).await.unwrap().value;assert_eq!(WriterSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);
 }
 drop(plugin);
}
#[test]
fn sqlite_snapshot_writer_authored_native_text_boundary_is_complete(){
 let grammar=semio_framework_dsl::parse_grammar(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).unwrap();
 let recognizer=semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");let(_,snapshot)=fixture();
 for snapshot in[snapshot,WriterSnapshot::default()]{
  let text=store::ArtifactDsl::print_dsl(&snapshot);
  assert!(recognizer.recognize(&text).unwrap(),"Writer grammar must recognize its complete actual native state:\n{text}");
  assert!(!recognizer.recognize(&format!("{text}\nopaque=[]")).unwrap(),"Writer grammar must reject undeclared carrier fields");
 }
}

#[test]
fn sqlite_snapshot_writer_controlled_output_preserves_literal_handles_and_cancels_inside_text(){
 use store::sqlite_snapshot::SqliteSnapshotPhase;
 let(f,mut base)=fixture();base.schema="independent owned schema".into();base.text="世界 🪐\0".repeat(30000);let limits=SqliteDatabaseLimits::default();
 let mut cases=vec![base];for reference in f["referenceCases"].as_array().unwrap(){let mut state=cases[0].clone();state.document=semio_framework_pack_json::from_json_str(&reference.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();cases.push(state);}
 for snapshot in cases{for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).expect("Writer genuine controlled output owner");assert_eq!(WriterSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);let mut interior=false;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==snapshot.text.len()&&event.completed>=65536&&event.completed<event.total{interior=true;false}else{true}},limits)).is_err());assert!(interior);for limited in[SqliteDatabaseLimits{max_rows:1,..limits},SqliteDatabaseLimits{max_value_bytes:128,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());}}}
}
#[test]
fn sqlite_snapshot_writer_explicit_native_output_boundary_preserves_complete_owned_state() {
 let (_,snapshot)=fixture();
 let limits=SqliteDatabaseLimits::default();
 let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let payload=<WriterSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  let restored=<WriterSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  <WriterSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored);
 }
 <WriterSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
#[test]
fn sqlite_snapshot_writer_explicit_native_schema_and_row_admission_precedes_work() {
 let (_,snapshot)=fixture();
 let limits=SqliteDatabaseLimits::default();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let mut native_work=false;
  let limited=SqliteDatabaseLimits{max_rows:rows-1,..limits};
  let result=<WriterSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>0{native_work=true;}true},limited));
  assert!(result.is_err());assert!(!native_work);
  let mut native_work=false;
  let limited=SqliteDatabaseLimits{max_schema_bytes:<WriterSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA.len()-1,..limits};
  let result=<WriterSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>0{native_work=true;}true},limited));
  assert!(result.is_err());assert!(!native_work);
 }
 <WriterSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
#[test]
fn sqlite_snapshot_writer_explicit_native_output_exact_file_frontier_and_unicode_cancel() {
 let (_,mut snapshot)=fixture();snapshot.text="interior 世界".repeat(20000);
 let limits=SqliteDatabaseLimits::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let payload=<WriterSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  let length=match &payload{store::io::IoPayload::Binary(bytes)=>bytes.len(),store::io::IoPayload::Text(text)=>text.len()};
  assert!(<WriterSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:length,..limits})).is_ok());
  assert!(<WriterSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:length-1,..limits})).is_err());
  let mut interior=false;
  assert!(<WriterSnapshot as ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&p.completed>=256&&p.completed<p.total{interior=true;false}else{true}},limits)).is_err());
  assert!(interior);
 }
 <WriterSnapshot as ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);
}
#[test]
fn sqlite_snapshot_writer_explicit_native_input_preserves_actual_fields_and_request_admission() {
 let (_,snapshot)=fixture();let limits=SqliteDatabaseLimits::default();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for payload in [store::io::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),store::io::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))] {
  let restored=WriterSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);WriterSnapshot::retire_sqlite_snapshot(restored);
  for limited in [SqliteDatabaseLimits{max_rows:1,..limits},SqliteDatabaseLimits{max_schema_bytes:WriterSnapshot::SQLITE_SCHEMA.len()-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}] {assert!(WriterSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());}
 }
 WriterSnapshot::retire_sqlite_snapshot(snapshot);
}

#[test]
fn sqlite_snapshot_writer_authored_examples_are_exact_current_references(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔗️authored-examples/🔣️.json")).unwrap();let examples=law["examples"].as_array().unwrap();assert_eq!(examples.len(),2);
 for entry in examples{let s=&entry["snapshot"];let expected=crate::writer_snapshot_with_text(s["schema"].as_str().unwrap(),s["id"].as_str().unwrap(),s["languageId"].as_str().unwrap(),s["uri"].as_str().unwrap(),s["text"].as_str().unwrap());println!("[DEBUG] Writer actual authored producer id={} canonical={:?}",expected.id,crate::standards::v1::subsets::any::io::text::snapshot::print_writer_dsl(&expected));assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&expected)).unwrap(),*s);}
 for(entry,asset)in examples.iter().zip([crate::standards::v1::subsets::any::io::text::snapshot::JACK_EXAMPLE_TEXT,crate::standards::v1::subsets::any::io::text::snapshot::DAG_JACK_EXAMPLE_TEXT]){let actual=crate::standards::v1::subsets::any::io::text::snapshot::parse_writer_dsl(asset).expect("original handcrafted example must parse through its actual owner");assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&actual)).unwrap(),entry["snapshot"]);assert_eq!(crate::standards::v1::subsets::any::io::text::snapshot::print_writer_dsl(&actual),asset);assert_eq!(actual.document.child_id,actual.document.target.artifact_id);}
 println!("[DEBUG] Writer original two authored DSL assets preserve complete neutral owners and exact current four-field references");
}
