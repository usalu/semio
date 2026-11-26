//! 🏢️ Original BIM public IO owns every authored SQLite role.
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_value::{DslValue,Number};
use semio_os_kernel as pack;
use semio_os_kernel as store;
use semio_framework_test_original_snapshot_owners as original_owners;

fn on_sqlite_law_stack(run: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
        .name("bim-public-sqlite".into())
        .stack_size(16 * 1024 * 1024)
        .spawn(run)
        .expect("the native codec law thread starts")
        .join();
    if let Err(failure) = result {
        std::panic::resume_unwind(failure);
    }
}

#[test]
fn sqlite_snapshot_bim_complete_public_sql_edits_are_authoritative(){
 on_sqlite_law_stack(|| {
 let provider=semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.expect("original BIM public capability");
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let original=fixture();let input=payload(&original,encoding);let database=public_export(&provider,&input,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let edited=oracle(&bytes,"edit",None);assert!(edited.starts_with(b"SQLite format 3\0"));let database=import_sqlite_database(&edited,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let output=public_import(&provider,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).expect("actual public import reads independently edited SQL");let mut expected=original;expected.project.name="SQLite edited BIM".into();*expected.properties.get_mut("external/element").unwrap().get_mut("set/λ").unwrap().get_mut("Length").unwrap()=semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue::Length{value:9.75};assert_eq!(decode(&output),expected);}
 });
}
#[test]
fn sqlite_snapshot_bim_complete_public_limits_and_cancellation(){
 on_sqlite_law_stack(|| {
 let provider=semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.expect("original BIM public capability");let snapshot=fixture();let schema_bytes=read_fixture("🗄️.sql").len();
 let counts=counts();let rows=number(&counts,"totalRows");let tables=number(&counts,"tableCount");let columns=number(&counts,"maxColumns");
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=payload(&snapshot,encoding);for(limits,expected)in[
 (SqliteDatabaseLimits{max_rows:rows,max_tables:tables,max_columns:columns,..SqliteDatabaseLimits::default()},true),
 (SqliteDatabaseLimits{max_rows:rows-1,..SqliteDatabaseLimits::default()},false),
 (SqliteDatabaseLimits{max_tables:tables-1,..SqliteDatabaseLimits::default()},false),
 (SqliteDatabaseLimits{max_columns:columns-1,..SqliteDatabaseLimits::default()},false),
 (SqliteDatabaseLimits{max_schema_bytes:schema_bytes-1,..SqliteDatabaseLimits::default()},false)]{let result=public_export(&provider,&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(result.is_ok(),expected,"public {encoding:?} {limits:?}: {result:?}");}
 let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.completed>=256{reached=true;false}else{true}};assert!(public_export(&provider,&input,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err(),"original public export cancels");assert!(reached);
 let database=public_export(&provider,&input,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();for limits in[SqliteDatabaseLimits{max_rows:rows-1,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_tables:tables-1,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_columns:columns-1,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_schema_bytes:schema_bytes-1,..SqliteDatabaseLimits::default()}]{assert!(public_import(&provider,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"original public import refuses one-short");}
 let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.completed>=256{reached=true;false}else{true}};assert!(public_import(&provider,database,encoding,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err(),"original public import cancels");assert!(reached);}
 });
}
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::{BIM_MODEL_DOCUMENT_SCHEMA, BIM_MODEL_DIALECT};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteDatabaseLimits,SqliteSnapshotControl,export_sqlite_database,import_sqlite_database};
use store::io_schema::{IoPayload,IoResult};
use store::os_store::serialization::ArtifactSqliteSnapshotCodec;

fn public_export(codec:&ArtifactSqliteSnapshotCodec,input:&IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<store::sqlite_snapshot::SqliteDatabase,String>{
 let mut output=None;
 let turn=original_owners::with_decoding(|owner|(codec.export)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT.into(),input,&mut output,control,owner));
 published(turn.result,output)
}
fn public_import(codec:&ArtifactSqliteSnapshotCodec,input:store::sqlite_snapshot::SqliteDatabase,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,String>{
 let mut input=Some(input);
 let mut output=None;
 let turn=original_owners::with_encoding(|owner|(codec.import)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT.into(),&mut input,encoding,&mut output,control,owner));
 published(turn.result,output)
}
fn published<T>(result:Result<(),impl std::fmt::Debug>,output:Option<IoResult<T>>)->Result<T,String>{
 result.map_err(|error|format!("{error:?}"))?;
 output.expect("completed SQLite turn published its output").map(|output|output.value).map_err(|error|format!("{error:?}"))
}

const SQL_ROOT:&str="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot";
fn read_fixture(path:&str)->&'static str{semio_framework_test_fixtures::repo_text!(std::path::Path::new(SQL_ROOT).join(path))}
fn counts()->serde_json::Value{serde_json::from_str(read_fixture("🧪️tests/🧫️fixtures/📏️counts/🔣️.json")).unwrap()}
fn number(counts:&serde_json::Value,name:&str)->usize{counts[name].as_u64().unwrap() as usize}
fn fixture()->ModelSnapshot{
 let corpus:serde_json::Value=serde_json::from_str(read_fixture("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 semio_framework_pack_json::from_json_str(&corpus["cases"][0]["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("original all-role schema fixture binds")
}
fn payload(snapshot:&ModelSnapshot,encoding:SnapshotEncoding)->IoPayload{match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(snapshot)),SnapshotEncoding::Text=>IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(snapshot))}}
fn decode(payload:&IoPayload)->ModelSnapshot{match payload{IoPayload::Binary(bytes)=><ModelSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).unwrap(),IoPayload::Text(text)=><ModelSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).unwrap()}}
fn oracle(bytes:&[u8],mode:&str,word:Option<&str>)->Vec<u8>{
 use std::{io::Write,process::{Command,Stdio}};
 let mut command=Command::new("repo");
 command.args(["tools","exec","--light","bun","--"]).arg(semio_framework_test_fixtures::repo_root!().join(SQL_ROOT).join("🧪️tests/🔮️oracle/🟦️.ts")).arg(mode);
 if let Some(word)=word{command.arg(word);}
 let mut child=command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("repository SQLite oracle");
 child.stdin.take().unwrap().write_all(bytes).expect("original SQLite bytes reach independent oracle");
 let result=child.wait_with_output().expect("independent SQLite oracle finishes");
 assert!(result.status.success(),"SQLite oracle: {}",String::from_utf8_lossy(&result.stderr));
 result.stdout
}
#[test]
fn sqlite_snapshot_bim_complete_public_io_owns_current_model(){
 on_sqlite_law_stack(|| {
 let bare=store::bare_artifact_codec::<ModelSnapshot,ModelMutation>(BIM_MODEL_DOCUMENT_SCHEMA);
 assert!(bare.snapshot_sqlite.is_some(),"original BIM bare factory has no authored SQLite capability");
 let declaration=semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::io();
 let provider=declaration.native.codec.snapshot_sqlite.expect("original BIM public IO has no authored SQLite capability");
 assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<ModelSnapshot>()));
 let counts=counts();
 let expected=fixture();
 let roles=expected.__dsl_to_record();assert_eq!(roles.fields.len(),65,"original current model fields");
 for id in (1..65).filter(|id|*id!=4){let Some(semio_framework_dsl_record::FieldValue::Map(values))=roles.get(id)else{panic!("original collection role {id} is absent")};assert!(!values.is_empty(),"current collection role {id} has no independent witness");}
 let table_count=counts["tableCount"].as_u64().unwrap()as usize;let row_count=counts["totalRows"].as_u64().unwrap()as usize;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let input=payload(&expected,encoding);let mut progress=0usize;
  let database=public_export(&provider,&input,&mut SqliteSnapshotControl::new(&mut |_|{progress+=1;true},SqliteDatabaseLimits::default())).expect("original native public payload projects");
  assert_eq!(database.tables.len(),table_count);assert_eq!(database.tables.iter().map(|table|table.rows.len()).sum::<usize>(),row_count);
  for(name,count)in counts["tables"].as_object().unwrap(){assert_eq!(database.table(name).unwrap().rows.len(),count.as_u64().unwrap()as usize,"{name}");}
  assert!(progress>0,"original public route emits progress");
  let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert!(bytes.starts_with(b"SQLite format 3\0"));
  let database=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  let output=public_import(&provider,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).expect("original public SQLite restores");
  assert_eq!(decode(&output),expected);
 }
 });
}
fn replace_literal_floats(value:&mut DslValue,word:f64){
 match value{
  DslValue::Number(Number::Float(value))=>*value=word,
  DslValue::Object(fields)=>fields.iter_mut().for_each(|(_,value)|replace_literal_floats(value,word)),
  DslValue::Array(values)=>values.iter_mut().for_each(|value|replace_literal_floats(value,word)),
  _=>{},
 }
}
fn replace_record_floats(record:&mut RecordValue,word:f64){
 for value in record.fields.values_mut(){replace_field_floats(value,word);}
}
fn replace_field_floats(value:&mut FieldValue,word:f64){
 match value{
  FieldValue::Float(value)=>*value=word,
  FieldValue::Value(value)=>replace_literal_floats(value,word),
  FieldValue::Block(value)=>replace_field_floats(value,word),
  FieldValue::Record(value)=>replace_record_floats(value,word),
  FieldValue::List(values)|FieldValue::Tuple(values)=>values.iter_mut().for_each(|value|replace_field_floats(value,word)),
  FieldValue::Map(values)=>values.iter_mut().for_each(|(_,value)|replace_field_floats(value,word)),
  FieldValue::Statements(values)=>values.iter_mut().for_each(|(_,value)|replace_record_floats(value,word)),
  _=>{},
 }
}
#[test]
fn sqlite_snapshot_bim_complete_public_exact_words(){
 on_sqlite_law_stack(|| {
 let provider=semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.expect("original BIM public capability");
 let counts=counts();let laws:serde_json::Value=serde_json::from_str(read_fixture("🧪️tests/🧫️fixtures/🎛️control/🔣️.json")).unwrap();let words=laws["binary64Words"].as_array().expect("neutral ordered binary64 corpus");assert_eq!(words.len(),11);
 for word in words{let word=word.as_str().expect("neutral word is hexadecimal text");assert_eq!(word.len(),16);let bits=u64::from_str_radix(word,16).expect("neutral binary64 word");
  let mut record=fixture().__dsl_to_record();replace_record_floats(&mut record,f64::from_bits(bits));let expected=ModelSnapshot::__dsl_from_record(&record).unwrap();let original=pack::snapshot::ArtifactPack::encode_pack(&expected);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=payload(&expected,encoding);let database=public_export(&provider,&input,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
   let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
   let receipt:serde_json::Value=serde_json::from_slice(&oracle(&bytes,"words",Some(word))).expect("independent word query receipt");
   assert_eq!(receipt["schema"],"bim.sqlite.oracle/v1");assert_eq!(receipt["mode"],"words");assert_eq!(receipt["word"],word);assert_eq!(receipt["tableCount"],counts["tableCount"]);assert_eq!(receipt["totalRows"],counts["totalRows"]);assert!(number(&receipt,"populatedWords")>0);assert_eq!(receipt["checked"],true);assert_eq!(receipt["declaredRoles"],counts["floatRoles"]);assert_eq!(receipt["populatedRoles"],counts["floatRoles"]);
   let database=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
   let output=public_import(&provider,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
   assert_eq!(pack::snapshot::ArtifactPack::encode_pack(&decode(&output)),original,"{bits:016x} {encoding:?}");
  }
 }
 });
}
