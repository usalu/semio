//! 🧫️ Forms exact owner capability precedes semantic and independently edited SQLite I/O.
use super::*;
#[path="💰️backing/🦀️.rs"]
mod backing;

#[test]
fn sqlite_snapshot_forms_declared_native_owner_has_semantic_sqlite_capability(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 assert_eq!(fixture["schema"],crate::FORMS_DOCUMENT_SCHEMA);
 assert!(<FormsSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().is_some(),"the actual owning Forms native declaration must expose its semantic SQLite snapshot codec");
}


fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn string(value:&serde_json::Value,key:&str)->String{value[key].as_str().unwrap().into()}
fn primitive(value:&serde_json::Value)->semio_framework_value::DslValue{
 match value["kind"].as_str().unwrap(){
  "null"=>semio_framework_value::DslValue::Null,"boolean"=>semio_framework_value::DslValue::Bool(value["value"].as_bool().unwrap()),"unsigned"=>semio_framework_value::DslValue::uint(value["value"].as_str().unwrap().parse().unwrap()),"signed"=>semio_framework_value::DslValue::int(value["value"].as_str().unwrap().parse().unwrap()),"float"=>semio_framework_value::DslValue::float(f64::from_bits(u64::from_str_radix(value["bits"].as_str().unwrap(),16).unwrap())),"text"=>semio_framework_value::DslValue::String(string(value,"value")),"bytes"=>semio_framework_value::DslValue::Bytes(value["octets"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect()),"array"=>semio_framework_value::DslValue::Array(value["items"].as_array().unwrap().iter().map(primitive).collect()),"object"=>semio_framework_value::DslValue::Object(value["members"].as_array().unwrap().iter().map(|v|(string(v,"name"),primitive(&v["value"]))).collect()),_=>panic!("neutral primitive")
 }
}
fn specimen()->FormsSnapshot{
 use crate::{FormQuestion,FormQuestionOption,FormVectorField,FormStep,FormExpr};
 use crate::schema::{definition::FormsDefinition,response::{FormsResponse,FormsAnswer}};
 let f=fixture();let q=&f["question"];let values:Vec<_>=f["values"].as_array().unwrap().iter().map(primitive).collect();
 let number=|v:&serde_json::Value,key:&str|f64::from_bits(u64::from_str_radix(v[key].as_str().unwrap(),16).unwrap());
 let question=FormQuestion{id:string(q,"id"),label:string(q,"label"),kind:string(q,"kind"),description:Some(string(q,"description")),required:Some(q["required"].as_bool().unwrap()),placeholder:Some(string(q,"placeholder")),default:Some(semio_framework_value::DslValue::Array(values.clone())),min:Some(number(q,"minimumBits")),max:Some(number(q,"maximumBits")),step:Some(number(q,"incrementBits")),unit:Some(string(q,"unit")),text:Some(string(q,"text")),options:Some(q["options"].as_array().unwrap().iter().map(|v|FormQuestionOption{value:string(v,"value"),label:string(v,"label")}).collect()),fields:Some(q["fields"].as_array().unwrap().iter().map(|v|FormVectorField{key:string(v,"key"),label:v["label"].as_str().map(Into::into),value:Some(number(v,"valueBits"))}).collect()),schema:Some(string(q,"questionSchema")),src:Some(string(q,"src")),accept:Some(string(q,"accept")),fixture_slug:Some(string(q,"fixtureSlug")),params:Some(primitive(&f["values"][8])),condition:Some(FormExpr::Eq{left:Box::new(FormExpr::Const{value:primitive(&f["values"][2])}),right:Box::new(FormExpr::Or{items:vec![FormExpr::Var{name:"answer".into()},FormExpr::Truthy{expr:Box::new(FormExpr::And{items:Vec::new()})}]})})};
 let mut snapshot=FormsSnapshot::default();snapshot.schema=string(&f,"schema");snapshot.id=string(&f,"id");snapshot.version=string(&f,"version");snapshot.title=Some(string(&f,"title"));snapshot.definition=FormsDefinition{steps:vec![FormStep{id:string(&f["step"],"id"),title:string(&f["step"],"title"),description:Some(string(&f["step"],"description")),blocks:vec![question]}]};snapshot.responses=vec![FormsResponse{id:string(&f["response"],"id"),submitted_at:f["response"]["submittedAt"].as_str().unwrap().parse().unwrap(),definition_version:string(&f["response"],"definitionVersion"),answers:values.into_iter().enumerate().map(|(i,value)|FormsAnswer{question_id:format!("answer{i}"),label:format!("Label{i}"),kind:"free".into(),value}).collect()}];
 snapshot.structure.child_id=string(&f["structure"],"childId");snapshot.structure.target.artifact_id=string(&f["structure"],"artifactId");snapshot.results.child_id=string(&f["results"],"childId");snapshot.results.target.artifact_id=string(&f["results"],"artifactId");snapshot.validate().unwrap();snapshot
}
fn database(snapshot:&FormsSnapshot)->store::sqlite_snapshot::SqliteDatabase{
 let mut callback=|_|true;let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut callback,Default::default());sqlite::project(snapshot,&mut control).expect("owned Forms native relational projection")
}
#[test]
fn sqlite_snapshot_forms_neutral_typed_state_and_independent_editable_sqlite(){
 use store::sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SqliteSnapshotControl,SqliteDatabaseLimits};
 use std::io::Write;
 let snapshot=specimen();let projected=database(&snapshot);assert_eq!(projected.tables.len(),30);let limits=SqliteDatabaseLimits::default();let bytes=export_sqlite_database(&projected,limits,&mut |_|true).unwrap();
 let script=r#"import{Database}from'bun:sqlite';import assert from'node:assert/strict';const bytes=new Uint8Array(await new Response(Bun.stdin.stream()).arrayBuffer());const db=Database.deserialize(bytes);assert.deepEqual(db.query('PRAGMA integrity_check').get(),{integrity_check:'ok'});assert.deepEqual(db.query('PRAGMA foreign_key_check').all(),[]);assert.deepEqual(db.query('SELECT high,low FROM forms_unsigned ORDER BY id LIMIT 1').get(),{high:4294967295,low:4294967295});assert.deepEqual(db.query("SELECT printf('%016llx',value_ieee754_bits) AS bits,value_numeric_class,value FROM forms_float ORDER BY id LIMIT 1").get(),{bits:'fff0000000000123',value_numeric_class:'nan',value:null});db.run('UPDATE forms_document SET title=?',['Independent SQLite 世界']);process.stdout.write(db.serialize());db.close();"#;
 let mut child=std::process::Command::new("bun").args(["-e",script]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::inherit()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success());let edited=import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,limits);let restored=sqlite::reconstruct(&edited,&mut control).expect("native relational reconstruction");assert_eq!(restored.title.as_deref(),fixture()["editedTitle"].as_str());assert_eq!(database(&restored),{let mut expected=projected;expected.table_mut("forms_document").unwrap().rows[0].values[4]=store::sqlite_snapshot::SqliteValue::Text("Independent SQLite 世界".into());expected});
}
#[test]
fn sqlite_snapshot_forms_native_borrowed_row_frontier_refuses_before_projection(){
 let snapshot=specimen();let mut callback=|_|true;let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut callback,store::sqlite_snapshot::SqliteDatabaseLimits{max_rows:5,..Default::default()});assert!(sqlite::project(&snapshot,&mut control).unwrap_err().kind==semio_framework_value::ValueRefusalKind::WorkLimit,"actual borrowed full descendant forecast must reject the complete state");
}

fn oracle_sql(database:&store::sqlite_snapshot::SqliteDatabase,sql:&str)->store::sqlite_snapshot::SqliteDatabase{
 use store::sqlite_snapshot::{export_sqlite_database,import_sqlite_database};use std::io::Write;
 let bytes=export_sqlite_database(database,Default::default(),&mut |_|true).unwrap();let sql=serde_json::to_string(sql).unwrap();let script=format!("import{{Database}}from'bun:sqlite';import assert from'node:assert/strict';const db=Database.deserialize(new Uint8Array(await new Response(Bun.stdin.stream()).arrayBuffer()));db.run({sql});assert.deepEqual(db.query('PRAGMA integrity_check').get(),{{integrity_check:'ok'}});assert.deepEqual(db.query('PRAGMA foreign_key_check').all(),[]);process.stdout.write(db.serialize());db.close();");
 let mut child=std::process::Command::new("bun").args(["-e",&script]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::inherit()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success());import_sqlite_database(&output.stdout,Default::default(),&mut |_|true).unwrap()
}
fn restore(database:&store::sqlite_snapshot::SqliteDatabase)->Result<FormsSnapshot,semio_framework_value::ValueError>{let mut callback=|_|true;let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut callback,Default::default());sqlite::reconstruct(database,&mut control)}
struct Owned(Option<FormsSnapshot>);
impl Drop for Owned{fn drop(&mut self){if let Some(value)=self.0.take(){sqlite::reconstruction::retire_snapshot(value)}}}
#[test]
fn sqlite_snapshot_forms_independent_surrogates_and_distinct_owned_handles(){
 let snapshot=specimen();let original=database(&snapshot);
 let sql="UPDATE forms_document SET id=1001;UPDATE forms_structure_child SET id=1001,child_id='independent-structure';UPDATE forms_results_child SET id=1001,child_id='independent-results';UPDATE forms_step SET document_id=1001;UPDATE forms_response SET document_id=1001;";
 let input=oracle_sql(&original,sql);let restored=Owned(Some(restore(&input).unwrap()));let mut expected=original;expected.table_mut("forms_structure_child").unwrap().rows[0].values[1]=store::sqlite_snapshot::SqliteValue::Text("independent-structure".into());expected.table_mut("forms_results_child").unwrap().rows[0].values[1]=store::sqlite_snapshot::SqliteValue::Text("independent-results".into());assert_eq!(database(restored.0.as_ref().unwrap()),expected);
}
#[test]
fn sqlite_snapshot_forms_native_deep_owned_frontiers_and_retirement(){
 let levels=fixture()["deepLevels"].as_u64().unwrap()as usize;let mut snapshot=Owned(Some(specimen()));let mut value=semio_framework_value::DslValue::uint(u64::MAX);let mut condition=crate::FormExpr::Const{value:semio_framework_value::DslValue::int(i64::MIN)};for _ in 0..levels{value=semio_framework_value::DslValue::Array(vec![value]);condition=crate::FormExpr::Truthy{expr:Box::new(condition)};}
 let q=&mut snapshot.0.as_mut().unwrap().definition.steps[0].blocks[0];if let Some(v)=q.default.replace(value){sqlite::reconstruction::retire_value(v)}if let Some(v)=q.condition.replace(condition){sqlite::reconstruction::retire_condition(v)}
 let projected=database(snapshot.0.as_ref().unwrap());let restored=Owned(Some(restore(&oracle_sql(&projected,"UPDATE forms_document SET title=title;")).unwrap()));assert_eq!(database(restored.0.as_ref().unwrap()),projected);
 let mut observed=false;let mut callback=|p:store::sqlite_snapshot::SqliteSnapshotProgress|{if p.phase==store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot&&p.total>levels&&p.completed>=256{observed=true;false}else{true}};let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut callback,Default::default());assert!(sqlite::reconstruct(&projected,&mut control).unwrap_err().kind==semio_framework_value::ValueRefusalKind::Canceled);assert!(observed);
 let mut callback=|_|true;let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut callback,store::sqlite_snapshot::SqliteDatabaseLimits{max_value_bytes:1024,..Default::default()});assert!(sqlite::reconstruct(&projected,&mut control).is_err());
}
#[test]
fn sqlite_snapshot_forms_independent_relational_cycles_and_duplicate_metadata_refuse(){
 let original=database(&specimen());let input=oracle_sql(&original,"UPDATE forms_condition_truthy SET expression_id=id;");assert!(restore(&input).is_err(),"SQLite-valid retained condition cycles must refuse");
 let mut snapshot=specimen();snapshot.definition.steps[0].blocks[0].options.as_mut().unwrap().push(crate::FormQuestionOption{value:"duplicate-original".into(),label:"label".into()});let original=database(&snapshot);let input=oracle_sql(&original,"UPDATE forms_option SET value=(SELECT value FROM forms_option ORDER BY id LIMIT 1);");assert!(restore(&input).is_err(),"SQLite-valid duplicate owned option identity must refuse");
}
#[test]
fn sqlite_snapshot_forms_actual_erased_native_import_requires_owned_output(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl};let value=specimen();let expected=database(&value);let codec=<FormsSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.forms.forms".into(),standard:"1".into(),subset:"*".into()};
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let payload=(codec.import)(crate::FORMS_DOCUMENT_SCHEMA,&dialect,expected.clone(),encoding,&mut control).expect("actual owned native output");let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let restored=(codec.export)(crate::FORMS_DOCUMENT_SCHEMA,&dialect,&payload.value,&mut control).expect("actual owned native input");assert_eq!(restored.value,expected);}
}

#[test]
fn sqlite_snapshot_forms_flat_native_deep_both_real_erased_directions(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl};
 let levels=fixture()["deepLevels"].as_u64().unwrap()as usize;let mut owner=Owned(Some(specimen()));let mut value=semio_framework_value::DslValue::float(f64::from_bits(0xfff0000000000123));let mut condition=crate::FormExpr::Const{value:semio_framework_value::DslValue::uint(u64::MAX)};for _ in 0..levels{value=semio_framework_value::DslValue::Object(vec![("ordered".into(),value)]);condition=crate::FormExpr::Truthy{expr:Box::new(condition)}}
 let q=&mut owner.0.as_mut().unwrap().definition.steps[0].blocks[0];if let Some(v)=q.default.replace(value){sqlite::reconstruction::retire_value(v)}if let Some(v)=q.condition.replace(condition){sqlite::reconstruction::retire_condition(v)}
 let expected=database(owner.0.as_ref().unwrap());let codec=crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.expect("actual declaration-owned provider");let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.forms.forms".into(),standard:"1".into(),subset:"*".into()};
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let payload=(codec.import)(crate::FORMS_DOCUMENT_SCHEMA,&dialect,expected.clone(),encoding,&mut control).unwrap().value;let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let actual=(codec.export)(crate::FORMS_DOCUMENT_SCHEMA,&dialect,&payload,&mut control).unwrap().value;assert_eq!(actual,expected);}
}
#[test]
fn sqlite_snapshot_forms_controlled_flat_row_admission_and_unicode_output(){
 use store::ArtifactSqliteSnapshot;use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
 let value=specimen();let record=native_pack::record(&value).unwrap();let mut callback=|_:semio_framework_value::native_decoding::NativeDecodeProgress|true;let mut c=semio_framework_value::NativeDecodeControl::new(0,&mut callback);assert!(native_pack::reconstruct_record_controlled(&record,&mut c,5).unwrap_err().kind==semio_framework_value::ValueRefusalKind::WorkLimit);assert_eq!(c.owned_bytes(),0,"the complete borrowed semantic forecast precedes typed field copies");
 let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,store::sqlite_snapshot::SqliteDatabaseLimits{max_rows:5,..Default::default()});assert!(value.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).unwrap_err().kind==semio_framework_value::ValueRefusalKind::WorkLimit);
 let f=fixture();let mut value=specimen();value.title=Some(f["unicodeText"].as_str().unwrap().repeat(f["unicodeRepeat"].as_u64().unwrap()as usize));let total=value.title.as_ref().unwrap().len();let mut observed=false;let mut callback=|p:store::sqlite_snapshot::SqliteSnapshotProgress|{if p.phase==SqliteSnapshotPhase::EncodeNative&&p.total==total&&p.completed>=65536{observed=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());assert!(value.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).unwrap_err().kind==semio_framework_value::ValueRefusalKind::Canceled);assert!(observed);
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_forms_actual_declared_registry_and_public_typed_file(){
 use semio_framework::io::io_mechanism::{NativeSnapshotRegistration,register_native_snapshots_in_assembly,native_snapshot_sqlite_schema,io_export_sqlite_snapshot,io_import_sqlite_snapshot};
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteDatabaseLimits,export_sqlite_database};
 let declaration=crate::standards::v1::subsets::any::io::io();let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.forms.forms".into(),standard:"1".into(),subset:"*".into()};let registration=NativeSnapshotRegistration::from_capability(dialect.clone(),declaration.native.codec).expect("actual declared native codec capability");let assembly=store::begin_artifact_assembly().unwrap();register_native_snapshots_in_assembly(&assembly,&[registration]).unwrap();drop(assembly);assert_eq!(native_snapshot_sqlite_schema(&dialect).unwrap(),sqlite::SQL);
 let value=specimen();let limits=SqliteDatabaseLimits::default();let bytes=io_export_sqlite_snapshot(&dialect,&value,SnapshotEncoding::Text,limits,&mut |_|true).await.unwrap().value;let parsed=store::sqlite_snapshot::import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();let edited=oracle_sql(&parsed,"UPDATE forms_document SET title='Independent SQLite 世界';");let bytes=export_sqlite_database(&edited,limits,&mut |_|true).unwrap();let restored=Owned(Some(io_import_sqlite_snapshot::<FormsSnapshot>(&dialect,&bytes,limits,&mut |_|true).await.unwrap().value));assert_eq!(restored.0.as_ref().unwrap().title.as_deref(),Some("Independent SQLite 世界"));let mut expected=database(&value);expected.table_mut("forms_document").unwrap().rows[0].values[4]=store::sqlite_snapshot::SqliteValue::Text("Independent SQLite 世界".into());assert_eq!(database(restored.0.as_ref().unwrap()),expected);
}
