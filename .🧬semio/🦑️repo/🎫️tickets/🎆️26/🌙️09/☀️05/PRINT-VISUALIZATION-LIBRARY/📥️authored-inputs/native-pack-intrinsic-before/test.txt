//! 🫳️ Actual core producers define exact forecasts, scratch requests and full refusal ownership.
use super::{encode_document,measure_document_borrowed,EncodeOptions};
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as S,BorrowedShape as H,DslField,FieldValue,JoinMode,RecordLayout,RecordSpec,native_encoding::FieldProjectionSource};
use semio_framework_dsl_record_derive::{DslRecord,DslScalar};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind as K};

#[derive(Debug,PartialEq,DslRecord)]
struct CsvField{value:String,quoted:bool}
#[derive(Debug,PartialEq,DslRecord)]
struct CsvRecord{fields:Vec<CsvField>}
#[derive(Debug,PartialEq,DslRecord)]
struct CsvOwner{schema:String,has_header:bool,records:Vec<CsvRecord>}
#[derive(Debug,PartialEq,DslScalar)]
enum Ending{Lf,Crlf}
#[derive(Debug,PartialEq,DslRecord)]
struct TsvOwner{schema:String,records:Vec<Vec<String>>,trailing_newline:bool,line_ending:Ending}
#[derive(Debug,PartialEq,DslRecord)]
struct Symbols{values:Vec<String>}

fn text()->H{H::Text}
fn text_list()->H{H::List(text)}
static CSV_FIELD:[F;2]=[F::new(0,"value",H::Text),F::new(1,"quoted",H::Bool)];
fn csv_field()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&CSV_FIELD}}
fn csv_field_shape()->H{H::Record(csv_field)}
static CSV_RECORD:[F;1]=[F::new(0,"fields",H::List(csv_field_shape))];
fn csv_record()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&CSV_RECORD}}
fn csv_record_shape()->H{H::Record(csv_record)}
static CSV:[F;3]=[F::new(0,"schema",H::Text),F::new(1,"has-header",H::Bool),F::new(2,"records",H::List(csv_record_shape))];
fn csv_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&CSV}}
static TSV:[F;4]=[F::new(0,"schema",H::Text),F::new(1,"records",H::List(text_list)),F::new(2,"trailing-newline",H::Bool),F::new(3,"line-ending",H::Enum(&[("lf",0),("crlf",1)]))];
fn tsv_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&TSV}}
static SYMBOLS:[F;1]=[F::new(0,"values",H::List(text))];
fn symbols_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&SYMBOLS}}

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn csv(row:&serde_json::Value)->CsvOwner{CsvOwner{schema:row["schema"].as_str().unwrap().into(),has_header:row["hasHeader"].as_bool().unwrap(),records:row["records"].as_array().unwrap().iter().map(|row|CsvRecord{fields:row.as_array().unwrap().iter().map(|field|CsvField{value:field["value"].as_str().unwrap().into(),quoted:field["quoted"].as_bool().unwrap()}).collect()}).collect()}}
fn tsv(row:&serde_json::Value)->TsvOwner{TsvOwner{schema:row["schema"].as_str().unwrap().into(),records:row["records"].as_array().unwrap().iter().map(|row|row.as_array().unwrap().iter().map(|value|value.as_str().unwrap().into()).collect()).collect(),trailing_newline:row["trailingNewline"].as_bool().unwrap(),line_ending:match row["lineEnding"].as_str().unwrap(){"lf"=>Ending::Lf,"crlf"=>Ending::Crlf,_=>panic!("closed ending")}}}
fn owned<T:DslField>(source:&T)->(RecordSpec,semio_framework_dsl_record::RecordValue){let semio_framework_dsl_record::Shape::Record(spec)=T::shape()else{panic!("declared record")};let FieldValue::Record(record)=source.to_value()else{panic!("declared record")};((spec.ordinary)(),record)}
fn measure<T:FieldProjectionSource>(source:&T,spec:&S,pack:bool,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{if pack{let mut options=EncodeOptions::default();options.limits.max_file_len=maximum as u64;measure_document_borrowed(source,spec,&options,control)}else{semio_framework_dsl_record::measure_print_borrowed(source,spec,maximum,control)}}
fn failure<T:FieldProjectionSource>(source:&T,spec:&S,pack:bool,maximum:usize,control:&mut NativeEncodeControl<'_>,kind:K){
 let ((actual,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=measure(source,spec,pack,maximum,control).expect_err("actual borrowed refusal");let result=(error.kind,error.message.capacity());drop(error);result});
 assert_eq!(actual,kind);assert_eq!(requested,released,"complete failed scratch and diagnostic release");assert!(requested<=control.owned_bytes().checked_add(diagnostic).unwrap(),"full requests include actual owned diagnostic");assert!(diagnostic>0);
}
fn parity<T:DslField>(source:&T,spec:&S,pack:bool,spill:bool){
 let (ordinary_spec,record)=owned(source);
 if pack{let mut allow=|_|true;let mut verify=NativeEncodeControl::new(0,&mut allow);assert_eq!(super::borrowed_preflight::borrowed_schema_hash(*spec,&mut verify).unwrap(),super::schema_hash(&ordinary_spec),"canonical structural graph digest matches actual producer");assert_eq!(verify.owned_bytes(),0);}
 let bytes=if pack{encode_document(&ordinary_spec,&record,&EncodeOptions::default()).unwrap()}else{semio_framework_dsl_record::print(&record,&ordinary_spec,JoinMode::Document).into_bytes()};
 let mut allow=|_|true;let mut control=NativeEncodeControl::new(if spill{usize::MAX}else{0},&mut allow);
 let (predicted,requested,released)=crate::test_allocation::observe_backing(||measure(source,spec,pack,usize::MAX,&mut control));
 assert_eq!(predicted.unwrap(),bytes.len(),"exact real canonical producer length");assert_eq!(requested,released,"successful traversal scratch fully released");assert_eq!(requested,control.owned_bytes(),"complete actual scratch requests");if spill&&pack{assert!(requested>0)}else{assert_eq!(requested,0)}
 let scratch=requested;let mut exact=NativeEncodeControl::new(scratch,&mut allow);let (predicted,requests,releases)=crate::test_allocation::observe_backing(||measure(source,spec,pack,bytes.len(),&mut exact));assert_eq!(predicted.unwrap(),bytes.len());assert_eq!(requests,scratch);assert_eq!(releases,scratch);assert_eq!(exact.owned_bytes(),scratch);
 let mut file_short=NativeEncodeControl::new(usize::MAX,&mut allow);failure(source,spec,pack,bytes.len()-1,&mut file_short,K::WorkLimit);
 if scratch>0{
  let mut zero=NativeEncodeControl::new(0,&mut allow);failure(source,spec,pack,usize::MAX,&mut zero,K::OwnershipLimit);assert_eq!(zero.owned_bytes(),0);
  let mut short=NativeEncodeControl::new(scratch-1,&mut allow);failure(source,spec,pack,usize::MAX,&mut short,K::OwnershipLimit);
  let mut cumulative=NativeEncodeControl::new(scratch*2-1,&mut allow);assert_eq!(measure(source,spec,pack,usize::MAX,&mut cumulative).unwrap(),bytes.len());let previous=cumulative.owned_bytes();failure(source,spec,pack,usize::MAX,&mut cumulative,K::OwnershipLimit);assert!(cumulative.owned_bytes()>=previous);
  let mut fired=false;let mut cancel=|_|{if crate::test_allocation::observed_requested_bytes().is_some_and(|bytes|bytes>0){fired=true;false}else{true}};let mut canceled=NativeEncodeControl::new(usize::MAX,&mut cancel);failure(source,spec,pack,usize::MAX,&mut canceled,K::Canceled);drop(canceled);assert!(fired,"real post-materialization scratch cancellation");
 }
 let mut reject=|_|false;let mut start=NativeEncodeControl::new(usize::MAX,&mut reject);failure(source,spec,pack,usize::MAX,&mut start,K::Canceled);assert_eq!(start.owned_bytes(),0);
 assert_eq!(source.to_value(),FieldValue::Record(record),"retained full owner unchanged");
}
fn law(pack:bool){
 let fixture=fixture();assert_eq!(fixture["contract"]["packCodec"],1);
 for row in fixture["csv"].as_array().unwrap(){parity(&csv(row),&csv_spec(),pack,false)}
 for row in fixture["tsv"].as_array().unwrap(){parity(&tsv(row),&tsv_spec(),pack,false)}
 let long=fixture["longText"]["text"].as_str().unwrap().repeat(fixture["longText"]["repeat"].as_u64().unwrap()as usize);
 let source=CsvOwner{schema:"stdio.csv".into(),has_header:false,records:vec![CsvRecord{fields:vec![CsvField{value:long,quoted:true}]}]};parity(&source,&csv_spec(),pack,false);
 let mut interior=false;let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{if progress.completed>=256{interior=true;false}else{true}};let mut control=NativeEncodeControl::new(usize::MAX,&mut cancel);failure(&source,&csv_spec(),pack,usize::MAX,&mut control,K::Canceled);drop(control);assert!(interior,"real long payload interior boundary");
 let repeated=fixture["repeatedInlineSymbols"]["text"].as_str().unwrap();let count=fixture["repeatedInlineSymbols"]["repeat"].as_u64().unwrap()as usize;parity(&Symbols{values:vec![repeated.into();count]},&symbols_spec(),pack,false);
 let prefix=fixture["distinctSymbols"]["prefix"].as_str().unwrap();let count=fixture["distinctSymbols"]["count"].as_u64().unwrap();let repeat=fixture["distinctSymbols"]["repeat"].as_u64().unwrap();let mut values=Vec::new();for _ in 0..repeat{for index in 0..count{values.push(format!("{prefix}{index}"));}}parity(&Symbols{values},&symbols_spec(),pack,true);
}
#[test]
fn record_borrowed_preflight_text_exact_full_owners_and_cancellation(){law(false)}
#[test]
fn record_borrowed_preflight_pack_exact_compression_spill_and_cumulative_ownership(){law(true)}
