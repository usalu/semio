//! 🏭️ Ticket-only actual canonical Record producers emit all authored CSV and TSV owner assets.
use semio_framework_dsl_record::{DslField,FieldValue,JoinMode,Shape};
use semio_framework_dsl_record_derive::{DslRecord,DslScalar};
#[derive(Debug,PartialEq,DslRecord)]
struct CsvField{value:String,quoted:bool}
#[derive(Debug,PartialEq,DslRecord)]
struct CsvRecord{fields:Vec<CsvField>}
#[derive(Debug,PartialEq,DslRecord)]
struct CsvSnapshot{schema:String,has_header:bool,records:Vec<CsvRecord>}
#[derive(Debug,PartialEq,DslScalar)]
enum LineEnding{Lf,Crlf}
#[derive(Debug,PartialEq,DslRecord)]
struct TsvSnapshot{schema:String,records:Vec<Vec<String>>,trailing_newline:bool,line_ending:LineEnding}
fn csv(row:&serde_json::Value)->CsvSnapshot{CsvSnapshot{schema:row["schema"].as_str().unwrap().into(),has_header:row["hasHeader"].as_bool().unwrap(),records:row["records"].as_array().unwrap().iter().map(|row|CsvRecord{fields:row["fields"].as_array().unwrap().iter().map(|field|CsvField{value:field["value"].as_str().unwrap().into(),quoted:field["quoted"].as_bool().unwrap()}).collect()}).collect()}}
fn tsv(row:&serde_json::Value)->TsvSnapshot{TsvSnapshot{schema:row["schema"].as_str().unwrap().into(),records:row["records"].as_array().unwrap().iter().map(|row|row.as_array().unwrap().iter().map(|field|field.as_str().unwrap().into()).collect()).collect(),trailing_newline:row["trailingNewline"].as_bool().unwrap(),line_ending:match row["lineEnding"].as_str().unwrap(){"lf"=>LineEnding::Lf,"crlf"=>LineEnding::Crlf,_=>panic!("closed line ending")}}}
fn emit<T:DslField>(owner:&T,kind:&str,prefix:&str,directory:&std::path::Path){
 let Shape::Record(make)=T::shape()else{panic!("actual declared owner record")};let spec=(make.ordinary)();let FieldValue::Record(record)=owner.to_value()else{panic!("actual declared owner value")};
 let text=semio_framework_dsl_record::print(&record,&spec,JoinMode::Document);assert_eq!(semio_framework_dsl_record::parse_exact(&text,&spec,&Default::default()).unwrap(),record);
 let(grammar,protocol)=match kind{"csv"=>(include_str!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio"),include_str!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio")),"tsv"=>(include_str!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio"),include_str!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio")),_=>panic!("closed descriptor")};
 let grammar=semio_framework_dsl::parse_grammar(grammar).unwrap();let recognizer=semio_framework_dsl::Recognizer::compile(&grammar,&Default::default(),Vec::new()).unwrap();assert!(recognizer.recognize(&text).unwrap(),"{kind} complete canonical text must match authored grammar");semio_framework_dsl::parse_protocol(protocol).unwrap();
 let body=pack::record::encode_document(&spec,&record,&Default::default()).unwrap();assert_eq!(pack::record::encode_document(&spec,&record,&Default::default()).unwrap(),body);assert_eq!(pack::record::decode_document(&body,&spec,&Default::default()).unwrap().0,record);
 let token=format!("stdio.{kind}.pack v1");let mut binary=vec![137,83,69,77,13,10,26,10];binary.extend_from_slice(&u32::try_from(token.len()).unwrap().to_le_bytes());binary.extend_from_slice(token.as_bytes());binary.extend_from_slice(&body);
 std::fs::create_dir_all(directory).unwrap();std::fs::write(directory.join(format!("🗣️{prefix}.dsl.semio")),format!("semio stdio.{kind}.dsl v1\n{}",text.trim_start())).unwrap();std::fs::write(directory.join(format!("🎒️{prefix}.pack.semio")),&binary).unwrap();
 std::fs::write(directory.join("🧬️schema-hash.txt"),pack::record::schema_hash(&spec).iter().map(|byte|format!("{byte:02x}")).collect::<String>()).unwrap();
 println!("[DEBUG] actual core {kind} prefix={prefix:?} Text={} Pack={} complete ordinary Record roundtrip",text.len(),binary.len());
}
#[test]
fn canonical_csv_tsv_owner_asset_producer(){
 let directory=std::path::PathBuf::from(std::env::var_os("SEMIO_OWNER_ASSET_OUTPUT").expect("explicit generated owner output"));
 let input:serde_json::Value=serde_json::from_str(include_str!("🔣️owners.json")).unwrap();
 for(kind,rows)in[("csv",&input["csv"]),("tsv",&input["tsv"])]{for(index,row)in rows["snapshots"].as_array().unwrap().iter().enumerate(){let prefix=index.to_string();if kind=="csv"{emit(&csv(row),kind,&prefix,&directory.join(kind));}else{emit(&tsv(row),kind,&prefix,&directory.join(kind));}}}
 emit(&csv(&input["csvDemo"]),"csv","",&directory.join("csv-demo"));emit(&tsv(&input["tsvDemo"]),"tsv","",&directory.join("tsv-demo"));
}
