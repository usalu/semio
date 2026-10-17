//! 🎬️ The exact registered Sequence parent snapshot ownership laws.
use semio_framework_test_fixtures::fixture_text;
use semio_framework_test_original_snapshot_owners as original_owners;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1::subsets::any::schema::snapshot::SequenceSnapshot;

#[test]
fn sqlite_snapshot_sequence_demo_matches_the_actual_parent_native_printer(){
 let expected=crate::standards::v1::subsets::any::schema::snapshot::default_persisted_snapshot();
 assert_eq!(pack::snapshot::ArtifactDsl::print_dsl(&expected),fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio"));
}

use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
use semio_framework_plugin::{PluginApp,VcsArtifactApp,EditorApp,ViewerApp,__semio_dispatch_PluginApp,plugin_app_close_prelude::*};
semio_framework_dispatch_macros::dyn_enum_close!{
 /// 🎬️ The real Sequence editor and viewer declaration closure.
 enum SqliteApps:PluginApp{
  Editor(VcsArtifactApp<EditorApp<crate::editor::sequence::app::SequencePlayApp>,semio_s_artifact_stdio_semio::SemioMembers>),
  Viewer(VcsArtifactApp<ViewerApp<crate::viewer::sequence::SequenceViewer>,semio_s_artifact_stdio_semio::SemioMembers>),
 }
}
fn laws()->serde_json::Value{serde_json::from_str(fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn fixture()->SequenceSnapshot{semio_framework_pack_json::from_json_str(&laws()["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn database(snapshot:&SequenceSnapshot)->SqliteDatabase{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).unwrap()}
fn restore(database:&SqliteDatabase)->SequenceSnapshot{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|SequenceSnapshot::from_sqlite_database(database, sqlite_control)).unwrap()}
fn file(snapshot:&SequenceSnapshot)->Vec<u8>{export_sqlite_database(&database(snapshot),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()}
fn native(snapshot:&SequenceSnapshot,encoding:SnapshotEncoding)->store::io_schema::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(snapshot)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(snapshot))}}
fn dialect()->semio_framework_artifact_reference::ArtifactDialect{semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.sequence.sequence".into(),standard:"1".into(),subset:"*".into()}}

#[test]
fn sqlite_snapshot_sequence_complete_parent_and_empty_reference_fields_are_exact(){
 let expected=fixture();let d=database(&expected);assert_eq!(d.tables.len(),2);assert_eq!(restore(&import_sqlite_database(&file(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
 let empty:SequenceSnapshot=semio_framework_pack_json::from_json_str(r#"{"schema":"","content":{"childId":"","target":{"artifactId":"","dialect":{"artifactKind":"","standard":"","subset":""}}}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 assert_eq!(restore(&database(&empty)),empty);
 let materialized=crate::default_snapshot();let projected=database(&materialized);let restored=restore(&projected);assert_eq!(restored,materialized);assert!(restored.content.require_local_owner::<crate::SequenceWorkingScene>().is_err());neural_engine::ColdRetire::retire_cold(materialized);
}

#[test]
fn sqlite_snapshot_sequence_all_owned_reference_domains_survive_both_erased_directions(){
 let codec=crate::standards::v1::subsets::any::io::io().native.codec;let provider=codec.snapshot_sqlite.unwrap();
 for text in["","id!kind@standard/subset%","世界\0","😀"]{
  let value=serde_json::json!({"schema":text,"content":{"childId":text,"target":{"artifactId":text,"dialect":{"artifactKind":text,"standard":text,"subset":text}}}});
  let expected:SequenceSnapshot=semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let d=original_owners::with_decoding(|native_owner|(provider.export)(&codec.schema,&dialect(),&native(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;assert_eq!(d,database(&expected));
   let d=import_sqlite_database(&export_sqlite_database(&d,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
   let payload=original_owners::with_encoding(|native_owner|(provider.import)(&codec.schema,&dialect(), &mut Some(d),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;
   assert_eq!(original_owners::with_decoding(|native_owner|SequenceSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap(),expected);
  }
 }
}

#[test]
fn sqlite_snapshot_sequence_independent_query_and_reference_edits_are_semantic(){
 use std::{io::Write,process::{Command,Stdio}};let mut expected=fixture();
 let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('SELECT artifact_id,artifact_kind,standard,subset FROM sequence_content').get();if(row.artifact_id!=='artifact 世界\0'||row.artifact_kind!=='s.stdio.semio'||row.standard!=='v1'||row.subset!=='flow')throw Error('literal identity');db.run('UPDATE sequence_document SET schema=?',['edited 世界']);db.run('UPDATE sequence_content SET id=99,child_id=?,artifact_id=?,artifact_kind=?,standard=?,subset=?',['new child','new artifact','s.stdio.json','rfc8259','*']);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file(&expected)).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 expected.schema="edited 世界".into();expected.content=store::os_store::artifact_child::ArtifactChild::new("new child".into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:"new artifact".into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"*".into()}});
 assert_eq!(restore(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}

#[test]
fn sqlite_snapshot_sequence_independent_malformed_sql_rejects_shape_parents_and_types(){
 use std::{io::Write,process::{Command,Stdio}};
 let bytes=file(&fixture());let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(new Uint8Array(input.bytes));db.run('PRAGMA ignore_check_constraints=ON');db.run(input.sql);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 for sql in laws()["malformedSql"].as_array().unwrap(){
  let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"bytes":bytes,"sql":sql}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{sql}: {}",String::from_utf8_lossy(&output.stderr));
  assert!(import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).and_then(|d|original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|SequenceSnapshot::from_sqlite_database(&d, sqlite_control))).is_err(),"{sql}");
 }
}

#[test]
fn sqlite_snapshot_sequence_caller_limits_precede_complete_native_and_relational_ownership(){
 let expected=fixture();let d=database(&expected);let limits=SqliteDatabaseLimits::default();
 assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits{max_rows:1,..limits},&mut |_|true,|sqlite_control|expected.to_sqlite_database(sqlite_control)).is_err());
 assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits{max_value_bytes:1,..limits},&mut |_|true,|sqlite_control|SequenceSnapshot::from_sqlite_database(&d, sqlite_control)).is_err());
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{for restricted in[SqliteDatabaseLimits{max_rows:1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits},SqliteDatabaseLimits{max_file_bytes:1,..limits}]{assert!(original_owners::with_decoding(|native_owner|SequenceSnapshot::decode_sqlite_snapshot_native(&native(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,restricted), native_owner)).is_err());}assert!(expected.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1024,..limits})).is_err());}
}

#[test]
fn sqlite_snapshot_sequence_initial_and_large_text_cancellation_reaches_all_native_phases(){
 let mut snapshot=fixture();let limits=SqliteDatabaseLimits::default();assert!(original_owners::with_sqlite_recipient(limits,&mut |_|false,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).is_err());assert!(original_owners::with_sqlite_recipient(limits,&mut |_|false,|sqlite_control|SequenceSnapshot::from_sqlite_database(&database(&snapshot), sqlite_control)).is_err());
 snapshot.content.child_id="😀".repeat(laws()["control"]["largeTextBytes"].as_u64().unwrap()as usize);let d=database(&snapshot);
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{
  let mut reached=false;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>0&&(event.total==0||event.total>=65536){reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut progress,limits);
  let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>SequenceSnapshot::from_sqlite_database(&d,&mut control).is_err(),SqliteSnapshotPhase::EncodeNative=>original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control, native_owner)).is_err(),_=>original_owners::with_decoding(|native_owner|SequenceSnapshot::decode_sqlite_snapshot_native(&native(&snapshot,SnapshotEncoding::Text),&mut control, native_owner)).is_err()};assert!(rejected,"{phase:?}");assert!(reached,"{phase:?}");
 }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_sequence_actual_declaration_mount_routes_queryable_files(){
 use store::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};
 semio_framework_plugin::PluginBuilder::<_, SqliteApps>::new("sequence").label("Sequence owned SQLite").version("0.0.1").package_id("semio:sequence").declare_artifact(crate::artifact::<SqliteApps>()).try_build().unwrap();
 assert!(store::os_store::codec_registry::document_codec(crate::SEQUENCE_DOCUMENT_SCHEMA).await.unwrap().unwrap().snapshot_sqlite.is_some());
 let expected=public_owner::Owned::new(fixture());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=async{let original_file_dialect=&dialect();let original_file_input=&*expected;let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_decoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;assert!(bytes.starts_with(b"SQLite format 3\0"));let restored=public_owner::Owned::new(async{let original_file_dialect=&dialect();let original_file_input=&bytes;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_encoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<SequenceSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value);assert_eq!(*restored,*expected);let edited=public_owner::verify_and_edit(&bytes,encoding);let restored=public_owner::Owned::new(async{let original_file_dialect=&dialect();let original_file_input=&edited;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_encoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<SequenceSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value);let mut literal=public_owner::Owned::new(fixture());let public=public_owner::corpus();let value=public["independentEdit"]["value"].as_str().unwrap();literal.schema=value.into();;assert_eq!(*restored,*literal);}
}


#[test]
fn sqlite_snapshot_sequence_actual_native_declaration_exposes_relational_capability(){
 let codec=crate::standards::v1::subsets::any::io::io().native.codec;
 assert!(codec.snapshot_sqlite.is_some(),"Sequence native declaration has no relational snapshot capability");
}

#[test]
fn sqlite_snapshot_sequence_neutral_json_retains_every_child_reference_field(){
 let laws:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let expected=&laws["snapshot"];
 let snapshot:SequenceSnapshot=semio_framework_pack_json::from_json_str(&expected.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&snapshot)).unwrap(),*expected);
 assert_eq!(snapshot.content.target.artifact_id,"artifact 世界\0");assert_eq!(snapshot.content.child_id,"child 世界\0; content");
}

#[test]
fn sqlite_snapshot_sequence_native_child_carrier_preserves_all_owned_string_domains(){
 for text in["","id!kind@standard/subset%","世界\0","😀"]{
  let value=serde_json::json!({"schema":text,"content":{"childId":text,"target":{"artifactId":text,"dialect":{"artifactKind":text,"standard":text,"subset":text}}}});
  let expected:SequenceSnapshot=semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let pack=pack::snapshot::ArtifactPack::encode_pack(&expected);
  let packed=<SequenceSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack).unwrap();
  assert_eq!(packed,expected);
  let printed=pack::snapshot::ArtifactDsl::print_dsl(&expected);
  let parsed=<SequenceSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap();
  assert_eq!(parsed,expected);
 }
}

#[test]
fn sqlite_snapshot_sequence_genuine_output_admits_exact_row_and_file_frontiers(){
 use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding,export_sqlite_database}};
 use std::{io::Write,process::{Command,Stdio}};
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();let expected=original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).unwrap();let rows=expected.tables.iter().map(|table|table.rows.len()).sum::<usize>();assert!(rows>0);
 let bytes=export_sqlite_database(&expected,limits,&mut |_|true).unwrap();
 let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all();const rows=tables.reduce((n,t)=>n+Number(db.query('SELECT COUNT(*) AS count FROM "'+t.name.replaceAll('"','""')+'"').get().count),0);await Bun.write(Bun.stdout,String(rows));db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(String::from_utf8(output.stdout).unwrap().parse::<usize>().unwrap(),rows);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,..limits}), native_owner)).unwrap();
  let physical=match &payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};assert!(physical>0);
  let repeated=original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,max_file_bytes:physical,..limits}), native_owner)).unwrap();assert_eq!(repeated,payload);
  let restored=original_owners::with_decoding(|native_owner|SequenceSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).unwrap();assert_eq!(original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|restored.to_sqlite_database(sqlite_control)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted), native_owner)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(original_owners::with_encoding(|native_owner|snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits), native_owner)).is_err());
 }
}


#[test]
fn sqlite_snapshot_sequence_complete_independent_sql_semantic_cell_extent(){
 use std::{io::Write,process::{Command,Stdio}};
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/📸️snapshot/📏️semantic-cells/🔣️.json")).unwrap();
 for c in cases["cases"].as_array().unwrap(){let mut s=fixture();if let Some(field)=c["field"].as_str(){let value=c["text"].as_str().unwrap();for name in["schema","childId","artifactId","artifactKind","standard","subset"]{if field!="all"&&field!=name{continue}match name{"schema"=>s.schema=value.into(),"childId"=>s.content.child_id=value.into(),"artifactId"=>s.content.target.artifact_id=value.into(),"artifactKind"=>s.content.target.dialect.artifact_kind=value.into(),"standard"=>s.content.target.dialect.standard=value.into(),"subset"=>s.content.target.dialect.subset=value.into(),_=>unreachable!()}}}
  let d=database(&s);let max=c["bytes"].as_u64().unwrap()as usize;let rows=c["rows"].as_u64().unwrap()as usize;
  let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const names=JSON.parse(process.env.SEMIO_SEQUENCE_EXPECTED_TABLES);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const actual=d.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(r=>r.name);if(JSON.stringify(actual)!==JSON.stringify([...names].sort()))throw Error('complete table names');let bytes=0,rows=0;for(const name of names){const actual=d.query('SELECT * FROM "'+name+'"').all();rows+=actual.length;for(const row of actual)for(const v of Object.values(row))bytes+=v===null?0:typeof v==='string'?Buffer.byteLength(v,'utf8'):v instanceof Uint8Array?v.length:8;}if(bytes!==Number(process.env.SEMIO_SEQUENCE_EXPECTED_BYTES)||rows!==Number(process.env.SEMIO_SEQUENCE_EXPECTED_ROWS))throw Error('complete physical SQL cells '+bytes+'/'+rows);d.close();"#;
  let mut child=Command::new("bun").args(["-e",script]).env("SEMIO_SEQUENCE_EXPECTED_TABLES",cases["tables"].to_string()).env("SEMIO_SEQUENCE_EXPECTED_BYTES",max.to_string()).env("SEMIO_SEQUENCE_EXPECTED_ROWS",rows.to_string()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file(&s)).unwrap();let observed=child.wait_with_output().unwrap();assert!(observed.status.success(),"{}",String::from_utf8_lossy(&observed.stderr));
  let limits=SqliteDatabaseLimits{max_value_bytes:max,..SqliteDatabaseLimits::default()};assert_eq!(original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|s.to_sqlite_database(sqlite_control)).unwrap(),d);let restored=original_owners::with_sqlite_recipient(limits,&mut |_|true,|sqlite_control|SequenceSnapshot::from_sqlite_database(&d, sqlite_control)).unwrap();assert_eq!(restored,s);let short=SqliteDatabaseLimits{max_value_bytes:max-1,..limits};assert!(original_owners::with_sqlite_recipient(short,&mut |_|true,|sqlite_control|s.to_sqlite_database(sqlite_control)).is_err());assert!(original_owners::with_sqlite_recipient(short,&mut |_|true,|sqlite_control|SequenceSnapshot::from_sqlite_database(&d, sqlite_control)).is_err());
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let expected=native(&s,encoding);assert_eq!(original_owners::with_encoding(|native_owner|s.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).unwrap(),expected);let restored=original_owners::with_decoding(|native_owner|SequenceSnapshot::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).unwrap();assert_eq!(restored,s);assert!(original_owners::with_encoding(|native_owner|s.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short), native_owner)).is_err(),"{} native encode",c["name"]);assert!(original_owners::with_decoding(|native_owner|SequenceSnapshot::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,short), native_owner)).is_err(),"{} native decode",c["name"]);}
 }
}

#[path="🚦️public/🦀️.rs"]
mod public_owner;
