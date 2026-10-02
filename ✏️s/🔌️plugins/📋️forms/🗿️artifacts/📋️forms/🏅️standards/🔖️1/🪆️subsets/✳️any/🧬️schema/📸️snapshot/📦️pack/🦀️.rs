//! 📦️ Forms flat native transport is authored independently of relational storage.
use super::FormsSnapshot;
#[path="🧬️records/🦀️.rs"]mod records;
#[path="📤️projection/🦀️.rs"]mod projection;
#[path="📥️reconstruction/🦀️.rs"]mod reconstruction;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
use records::Document;
pub(crate) fn record_spec()->dsl::RecordSpec{Document::__dsl_spec()}
pub(crate) fn record_spec_producer()->dsl::RecordSpecProducer{Document::__dsl_spec_producer()}
pub(crate) fn record_controlled(snapshot:&FormsSnapshot,counts:&super::sqlite::admission::Counts,c:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::RecordValue,dsl::TextError>{
 projection::project(snapshot,counts,c).map_err(dsl::__rt::field_error)?.__dsl_to_record_controlled(c)
}
pub(crate) fn record(snapshot:&FormsSnapshot)->Result<dsl::RecordValue,String>{
 let mut callback=|_|true;let mut sqlite=SqliteSnapshotControl::new(&mut callback,store::sqlite_snapshot::SqliteDatabaseLimits{max_file_bytes:usize::MAX,max_value_bytes:usize::MAX,max_rows:usize::MAX,..Default::default()});let counts=super::sqlite::admission::forecast(snapshot,&mut sqlite,SqliteSnapshotPhase::EncodeNative)?;
 let mut callback=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;let mut c=dsl::NativeEncodeControl::new(usize::MAX,&mut callback);record_controlled(snapshot,&counts,&mut c).map_err(|e|e.message)
}
fn list(source:&dsl::RecordValue,key:u16)->Result<&[dsl::FieldValue],String>{match source.fields.get(&key){None|Some(dsl::FieldValue::Absent)=>Ok(&[]),Some(dsl::FieldValue::List(values))=>Ok(values),_=>Err("Forms native ordered record list differs".into())}}
fn optional_list(source:&dsl::RecordValue,key:u16)->Result<&[dsl::FieldValue],String>{match source.fields.get(&key){None|Some(dsl::FieldValue::Absent)=>Ok(&[]),Some(dsl::FieldValue::List(values))=>Ok(values),_=>Err("Forms native optional ordered list differs".into())}}
fn view(value:&dsl::FieldValue)->Result<&dsl::RecordValue,String>{match value{dsl::FieldValue::Record(record)=>Ok(record),_=>Err("Forms native entity record differs".into())}}
fn add(rows:&mut usize,count:usize,maximum:usize)->Result<(),String>{*rows=rows.checked_add(count).ok_or("Forms native row count overflow")?;if *rows>maximum{return Err("Forms native semantic row limit exceeded".into())}Ok(())}
pub(crate) fn borrowed_rows(source:&dsl::RecordValue,c:&mut dsl::NativeDecodeControl<'_>,maximum:usize)->Result<usize,String>{
 c.scoped_stage(|c|{c.begin_stage(0)?;let steps=list(source,6)?;let responses=list(source,7)?;let values=list(source,8)?;let conditions=list(source,9)?;let mut rows=3usize;
 for count in[steps.len(),responses.len(),values.len().checked_mul(2).ok_or("Forms value count overflow")?,conditions.len().checked_mul(2).ok_or("Forms condition count overflow")?]{add(&mut rows,count,maximum)?}
 for step in steps{c.step()?;let questions=list(view(step)?,3)?;add(&mut rows,questions.len(),maximum)?;for question in questions{c.step()?;let q=view(question)?;add(&mut rows,optional_list(q,12)?.len(),maximum)?;add(&mut rows,optional_list(q,13)?.len(),maximum)?;}}
 for response in responses{c.step()?;add(&mut rows,list(view(response)?,3)?.len(),maximum)?;}
 for value in values{c.step()?;let v=view(value)?;let bytes=match v.fields.get(&6){None|Some(dsl::FieldValue::Absent)=>0,Some(dsl::FieldValue::Bytes64(values))=>values.len(),_=>return Err("Forms native intrinsic octet field differs".into())};add(&mut rows,bytes,maximum)?;add(&mut rows,list(v,7)?.len(),maximum)?;add(&mut rows,list(v,8)?.len(),maximum)?;}
 for condition in conditions{c.step()?;add(&mut rows,list(view(condition)?,6)?.len(),maximum)?;}Ok(rows)})
}
pub(crate) fn reconstruct_record_controlled(source:&dsl::RecordValue,c:&mut dsl::NativeDecodeControl<'_>,maximum:usize)->Result<FormsSnapshot,dsl::TextError>{
 let result=(||{borrowed_rows(source,c,maximum)?;let document=Document::__dsl_from_record_controlled(source,c).map_err(|e|e.message)?;let snapshot=reconstruction::reconstruct(document,c)?;let validation=super::sqlite::admission::identities::snapshot(&snapshot,c).and_then(|()|validate(&snapshot,c));if let Err(error)=validation{super::sqlite::reconstruction::retire_snapshot(snapshot);return Err(error)}Ok(snapshot)})();result.map_err(dsl::__rt::field_error)
}
fn validate(s:&FormsSnapshot,c:&mut dsl::NativeDecodeControl<'_>)->Result<(),String>{
 c.begin_stage(0)?;if s.schema!="forms.form"{return Err("Forms marker differs".into())}for(target,subset)in[(&s.structure.target,"value"),(&s.results.target,"table")]{if target.dialect.artifact_kind!="s.stdio.semio"||target.dialect.standard!="v1"||target.dialect.subset!=subset{return Err("Forms child coordinate differs".into())}}
 for step in &s.definition.steps{for q in &step.blocks{c.step()?;if q.kind.trim().is_empty()||[q.min,q.max,q.step].into_iter().flatten().any(|v|!v.is_finite())||q.min.zip(q.max).is_some_and(|(a,b)|a>b)||q.step.is_some_and(|v|v<=0.0)||q.params.as_ref().is_some_and(|v|!matches!(v,dsl::DslValue::Object(_))){return Err("Forms question invariant differs".into())}for field in q.fields.iter().flatten(){c.step()?;if field.value.is_some_and(|v|!v.is_finite()){return Err("Forms vector invariant differs".into())}}}}
 for response in &s.responses{c.step()?;if response.definition_version.is_empty()||response.submitted_at>9_007_199_254_740_991{return Err("Forms response invariant differs".into())}}c.checkpoint()
}
pub(crate) fn reconstruct_record(source:&dsl::RecordValue)->Result<FormsSnapshot,dsl::TextError>{let mut callback=|_:protocol::native_decoding::NativeDecodeProgress|true;let mut c=dsl::NativeDecodeControl::new(usize::MAX,&mut callback);reconstruct_record_controlled(source,&mut c,usize::MAX)}
