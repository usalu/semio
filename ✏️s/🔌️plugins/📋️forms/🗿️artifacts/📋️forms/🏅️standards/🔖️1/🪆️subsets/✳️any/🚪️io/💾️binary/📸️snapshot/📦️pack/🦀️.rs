//! 📦️ Forms flat native transport is authored independently of relational storage.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::any::schema::snapshot::FormsSnapshot;
#[path="🧬️records/🦀️.rs"]mod records;
#[path="📤️projection/🦀️.rs"]mod projection;
#[path="📥️reconstruction/🦀️.rs"]mod reconstruction;
#[path="📏️bound/🦀️.rs"]mod bound;
pub(crate) use bound::preflight;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
use records::Document;
pub(crate) fn record_spec()->semio_framework_dsl_record::RecordSpec{Document::__dsl_spec()}
pub(crate) fn record_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{Document::__dsl_spec_producer()}
pub(crate) fn record_controlled(snapshot:&FormsSnapshot,counts:&crate::standards::v1::subsets::any::schema::snapshot::sqlite::admission::Counts,c:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
 projection::project(snapshot,counts,c)?.__dsl_to_record_controlled(c)
}
pub(crate) fn record(snapshot:&FormsSnapshot)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
 let mut callback=|_|true;let mut sqlite=SqliteSnapshotControl::new(&mut callback,store::sqlite_snapshot::SqliteDatabaseLimits{max_file_bytes:usize::MAX,max_value_bytes:usize::MAX,max_rows:usize::MAX,..Default::default()});let counts=crate::standards::v1::subsets::any::schema::snapshot::sqlite::admission::forecast(snapshot,&mut sqlite,SqliteSnapshotPhase::EncodeNative)?;
 let mut callback=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;let mut c=semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut callback);record_controlled(snapshot,&counts,&mut c)
}
fn list(source:&semio_framework_dsl_record::RecordValue,key:u16)->Result<&[semio_framework_dsl_record::FieldValue],ValueError>{match source.fields.get(&key){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native ordered record list differs"))}}
fn optional_list(source:&semio_framework_dsl_record::RecordValue,key:u16)->Result<&[semio_framework_dsl_record::FieldValue],ValueError>{match source.fields.get(&key){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native optional ordered list differs"))}}
fn view(value:&semio_framework_dsl_record::FieldValue)->Result<&semio_framework_dsl_record::RecordValue,ValueError>{match value{semio_framework_dsl_record::FieldValue::Record(record)=>Ok(record),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native entity record differs"))}}
fn add(rows:&mut usize,count:usize,maximum:usize)->Result<(),ValueError>{*rows=rows.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms native row count overflow"))?;if *rows>maximum{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Forms native semantic row limit exceeded"))}Ok(())}
pub(crate) fn borrowed_rows(source:&semio_framework_dsl_record::RecordValue,c:&mut semio_framework_value::NativeDecodeControl<'_>,maximum:usize)->Result<usize,ValueError>{
 c.scoped_stage(|c|{c.begin_stage(0)?;let steps=list(source,6)?;let responses=list(source,7)?;let values=list(source,8)?;let conditions=list(source,9)?;let mut rows=3usize;
 for count in[steps.len(),responses.len(),values.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms value count overflow"))?,conditions.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms condition count overflow"))?]{add(&mut rows,count,maximum)?}
 for step in steps{c.step()?;let questions=list(view(step)?,3)?;add(&mut rows,questions.len(),maximum)?;for question in questions{c.step()?;let q=view(question)?;add(&mut rows,optional_list(q,12)?.len(),maximum)?;add(&mut rows,optional_list(q,13)?.len(),maximum)?;}}
 for response in responses{c.step()?;add(&mut rows,list(view(response)?,3)?.len(),maximum)?;}
 for value in values{c.step()?;let v=view(value)?;let bytes=match v.fields.get(&6){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>0,Some(semio_framework_dsl_record::FieldValue::Bytes64(values))=>values.len(),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native intrinsic octet field differs"))};add(&mut rows,bytes,maximum)?;add(&mut rows,list(v,7)?.len(),maximum)?;add(&mut rows,list(v,8)?.len(),maximum)?;}
 for condition in conditions{c.step()?;add(&mut rows,list(view(condition)?,6)?.len(),maximum)?;}Ok(rows)})
}
pub(crate) fn reconstruct_record_controlled(source:&semio_framework_dsl_record::RecordValue,c:&mut semio_framework_value::NativeDecodeControl<'_>,maximum:usize)->Result<FormsSnapshot,ValueError>{
 let result=(||{borrowed_rows(source,c,maximum)?;let document=Document::__dsl_from_record_controlled(source,c)?;let snapshot=reconstruction::reconstruct(document,c)?;let validation=crate::standards::v1::subsets::any::schema::snapshot::sqlite::admission::identities::snapshot(&snapshot,c).and_then(|()|validate(&snapshot,c));if let Err(error)=validation{crate::standards::v1::subsets::any::schema::snapshot::sqlite::reconstruction::retire_snapshot(snapshot);return Err(error)}Ok(snapshot)})();result
}
fn validate(s:&FormsSnapshot,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
 c.begin_stage(0)?;if s.schema!="forms.form"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms marker differs"))}for(target,subset)in[(&s.structure.target,"value"),(&s.results.target,"table")]{if target.dialect.artifact_kind!="s.stdio.semio"||target.dialect.standard!="v1"||target.dialect.subset!=subset{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms child coordinate differs"))}}
 for step in &s.definition.steps{for q in &step.blocks{c.step()?;if q.kind.trim().is_empty()||[q.min,q.max,q.step].into_iter().flatten().any(|v|!v.is_finite())||q.min.zip(q.max).is_some_and(|(a,b)|a>b)||q.step.is_some_and(|v|v<=0.0)||q.params.as_ref().is_some_and(|v|!matches!(v,semio_framework_value::DslValue::Object(_))){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms question invariant differs"))}for field in q.fields.iter().flatten(){c.step()?;if field.value.is_some_and(|v|!v.is_finite()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms vector invariant differs"))}}}}
 for response in &s.responses{c.step()?;if response.definition_version.is_empty()||response.submitted_at>9_007_199_254_740_991{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms response invariant differs"))}}c.checkpoint()
}
pub(crate) fn reconstruct_record(source:&semio_framework_dsl_record::RecordValue)->Result<FormsSnapshot,ValueError>{let mut callback=|_:semio_framework_value::native_decoding::NativeDecodeProgress|true;let mut c=semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut callback);reconstruct_record_controlled(source,&mut c,usize::MAX)}
