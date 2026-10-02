//! 📋️ Thirty literal Forms tables retain the complete owned intrinsic and condition domains.
use super::FormsSnapshot;
use crate::{FormExpr,FormQuestion};
use dsl::DslValue;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Projection,Cell}};
#[path="📏️admission/🦀️.rs"]pub mod admission;
pub const SQL:&str=include_str!("🗄️.sql");
fn text(value:&Option<String>)->Cell<'_>{value.as_deref().map_or(Cell::Null,Cell::Text)}
fn reference(value:Option<i64>)->Cell<'static>{value.map_or(Cell::Null,Cell::Integer)}
fn float(value:Option<f64>)->[Cell<'static>;3]{match value{None=>[Cell::Null;3],Some(value)=>[if value.is_nan(){Cell::Null}else{Cell::Real(value)},Cell::Integer(value.to_bits()as i64),Cell::Text(if value.is_nan(){"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"})]}}
fn words(value:u64)->[Cell<'static>;2]{[Cell::Integer((value>>32)as i64),Cell::Integer((value&0xffffffff)as i64)]}
fn ordinal(value:usize)->Result<Cell<'static>,String>{Ok(Cell::Integer(i64::try_from(value).map_err(|e|e.to_string())?))}
fn next(value:&mut i64)->Result<i64,String>{*value=value.checked_add(1).ok_or("Forms surrogate identity overflow")?;Ok(*value)}
struct Projector<'s,'c,'p>{p:Projection<'c,'p>,values:Vec<(&'s DslValue,i64)>,conditions:Vec<(&'s FormExpr,i64)>,value_id:i64,condition_id:i64,total:usize}
impl<'s>Projector<'s,'_,'_>{
 fn value(&mut self,root:&'s DslValue)->Result<i64,String>{
  let root_id=next(&mut self.value_id)?;self.values.push((root,root_id));while let Some((value,id))=self.values.pop(){
   let kind=match value{DslValue::Null=>"null",DslValue::Bool(_)=>"boolean",DslValue::Number(dsl::Number::UInt(_))=>"unsigned",DslValue::Number(dsl::Number::Int(_))=>"signed",DslValue::Number(dsl::Number::Float(_))=>"float",DslValue::String(_)=>"text",DslValue::Bytes(_)=>"bytes",DslValue::Array(_)=>"array",DslValue::Object(_)=>"object"};self.p.insert_key("forms_value",id,&[Cell::Text(kind)])?;
   match value{
    DslValue::Null=>self.p.insert_key("forms_null",id,&[])?,DslValue::Bool(value)=>self.p.insert_key("forms_boolean",id,&[Cell::Integer(i64::from(*value))])?,DslValue::Number(dsl::Number::UInt(value))=>self.p.insert_key("forms_unsigned",id,&words(*value))?,DslValue::Number(dsl::Number::Int(value))=>self.p.insert_key("forms_signed",id,&[Cell::Integer(*value)])?,DslValue::Number(dsl::Number::Float(value))=>self.p.insert_key("forms_float",id,&float(Some(*value)))?,DslValue::String(value)=>self.p.insert_key("forms_text",id,&[Cell::Text(value)])?,
    DslValue::Bytes(values)=>{self.p.insert_key("forms_bytes",id,&[])?;for(i,value)in values.iter().enumerate(){self.p.insert("forms_octet",&[Cell::Integer(id),ordinal(i)?,Cell::Integer(i64::from(*value))])?;}}
    DslValue::Array(values)=>{self.p.insert_key("forms_array",id,&[])?;for(i,value)in values.iter().enumerate(){let child=next(&mut self.value_id)?;self.p.insert("forms_array_element",&[Cell::Integer(id),ordinal(i)?,Cell::Integer(child)])?;self.values.push((value,child));}}
    DslValue::Object(values)=>{self.p.insert_key("forms_object",id,&[])?;for(i,(name,value))in values.iter().enumerate(){let child=next(&mut self.value_id)?;self.p.insert("forms_object_member",&[Cell::Integer(id),ordinal(i)?,Cell::Text(name),Cell::Integer(child)])?;self.values.push((value,child));}}
   }self.p.checkpoint_total(self.total)?;
  }Ok(root_id)
 }
 fn condition(&mut self,root:&'s FormExpr)->Result<i64,String>{
  let root_id=next(&mut self.condition_id)?;self.conditions.push((root,root_id));while let Some((value,id))=self.conditions.pop(){let kind=match value{FormExpr::Const{..}=>"const",FormExpr::Var{..}=>"var",FormExpr::Eq{..}=>"eq",FormExpr::And{..}=>"and",FormExpr::Or{..}=>"or",FormExpr::Truthy{..}=>"truthy"};self.p.insert_key("forms_condition",id,&[Cell::Text(kind)])?;
   match value{FormExpr::Const{value}=>{let value=self.value(value)?;self.p.insert_key("forms_condition_const",id,&[Cell::Integer(value)])?;}FormExpr::Var{name}=>self.p.insert_key("forms_condition_var",id,&[Cell::Text(name)])?,FormExpr::Eq{left,right}=>{let left_id=next(&mut self.condition_id)?;let right_id=next(&mut self.condition_id)?;self.p.insert_key("forms_condition_eq",id,&[Cell::Integer(left_id),Cell::Integer(right_id)])?;self.conditions.push((right,right_id));self.conditions.push((left,left_id));}FormExpr::Truthy{expr}=>{let child=next(&mut self.condition_id)?;self.p.insert_key("forms_condition_truthy",id,&[Cell::Integer(child)])?;self.conditions.push((expr,child));}FormExpr::And{items}|FormExpr::Or{items}=>{self.p.insert_key(if matches!(value,FormExpr::And{..}){"forms_condition_and"}else{"forms_condition_or"},id,&[])?;for(i,item)in items.iter().enumerate(){let child=next(&mut self.condition_id)?;self.p.insert("forms_condition_item",&[Cell::Integer(id),ordinal(i)?,Cell::Integer(child)])?;self.conditions.push((item,child));}}}self.p.checkpoint_total(self.total)?;
  }Ok(root_id)
 }
 fn question(&mut self,q:&'s FormQuestion,step:i64,index:usize)->Result<(),String>{
  let default=q.default.as_ref().map(|v|self.value(v)).transpose()?;let params=q.params.as_ref().map(|v|self.value(v)).transpose()?;let condition=q.condition.as_ref().map(|v|self.condition(v)).transpose()?;
  let min=float(q.min);let max=float(q.max);let increment=float(q.step);
  let id=self.p.insert("forms_question",&[Cell::Integer(step),ordinal(index)?,Cell::Text(&q.id),Cell::Text(&q.label),Cell::Text(&q.kind),text(&q.description),q.required.map_or(Cell::Null,|v|Cell::Integer(i64::from(v))),text(&q.placeholder),reference(default),min[0],min[1],min[2],max[0],max[1],max[2],increment[0],increment[1],increment[2],text(&q.unit),text(&q.text),Cell::Integer(i64::from(q.options.is_some())),Cell::Integer(i64::from(q.fields.is_some())),text(&q.schema),text(&q.src),text(&q.accept),text(&q.fixture_slug),reference(params),reference(condition)])?;
  for(i,value)in q.options.iter().flatten().enumerate(){self.p.insert("forms_option",&[Cell::Integer(id),ordinal(i)?,Cell::Text(&value.value),Cell::Text(&value.label)])?;}
  for(i,value)in q.fields.iter().flatten().enumerate(){let number=float(value.value);self.p.insert("forms_vector_field",&[Cell::Integer(id),ordinal(i)?,Cell::Text(&value.key),text(&value.label),number[0],number[1],number[2]])?;}Ok(())
 }
}
/** 📤️ Project literal borrowed fields after complete row and cell admission. */
pub fn project<'s>(snapshot:&'s FormsSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
 let counts=admission::forecast(snapshot,control,SqliteSnapshotPhase::ProjectSnapshot)?;let frontier=counts.values.checked_mul(std::mem::size_of::<(&DslValue,i64)>()).and_then(|n|counts.conditions.checked_mul(std::mem::size_of::<(&FormExpr,i64)>()).and_then(|m|n.checked_add(m))).ok_or("Forms projection frontier byte overflow")?;control.check_value_bytes(counts.bytes.checked_add(frontier).ok_or("Forms projection frontier byte overflow")?)?;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,counts.rows)?;
 let mut values=Vec::new();values.try_reserve_exact(counts.values).map_err(|_|"Forms value frontier allocation failed")?;let mut conditions=Vec::new();conditions.try_reserve_exact(counts.conditions).map_err(|_|"Forms condition frontier allocation failed")?;
 let mut owner=Projector{p:Projection::new(SQL,control)?,values,conditions,value_id:0,condition_id:0,total:counts.rows};
 let document=owner.p.insert("forms_document",&[Cell::Text(&snapshot.schema),Cell::Text(&snapshot.id),Cell::Text(&snapshot.version),text(&snapshot.title)])?;
 owner.p.insert_key("forms_structure_child",document,&[Cell::Text(&snapshot.structure.child_id),Cell::Text(&snapshot.structure.target.artifact_id),Cell::Text(&snapshot.structure.target.dialect.artifact_kind),Cell::Text(&snapshot.structure.target.dialect.standard),Cell::Text(&snapshot.structure.target.dialect.subset)])?;
 owner.p.insert_key("forms_results_child",document,&[Cell::Text(&snapshot.results.child_id),Cell::Text(&snapshot.results.target.artifact_id),Cell::Text(&snapshot.results.target.dialect.artifact_kind),Cell::Text(&snapshot.results.target.dialect.standard),Cell::Text(&snapshot.results.target.dialect.subset)])?;
 for(i,step)in snapshot.definition.steps.iter().enumerate(){let id=owner.p.insert("forms_step",&[Cell::Integer(document),ordinal(i)?,Cell::Text(&step.id),Cell::Text(&step.title),text(&step.description)])?;for(i,question)in step.blocks.iter().enumerate(){owner.question(question,id,i)?;}}
 for(i,response)in snapshot.responses.iter().enumerate(){let timestamp=words(response.submitted_at);let id=owner.p.insert("forms_response",&[Cell::Integer(document),ordinal(i)?,Cell::Text(&response.id),timestamp[0],timestamp[1],Cell::Text(&response.definition_version)])?;for(i,answer)in response.answers.iter().enumerate(){let value=owner.value(&answer.value)?;owner.p.insert("forms_answer",&[Cell::Integer(id),ordinal(i)?,Cell::Text(&answer.question_id),Cell::Text(&answer.label),Cell::Text(&answer.kind),Cell::Integer(value)])?;}}
 owner.p.checkpoint_total(counts.rows)?;owner.p.finish()
}
#[path="📥️reconstruction/🦀️.rs"]pub mod reconstruction;
pub use reconstruction::reconstruct;

impl store::ArtifactSqliteSnapshot for FormsSnapshot{
 const SQLITE_SCHEMA:&'static str=SQL;
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{reconstruct(database,control)}
 fn retire_sqlite_snapshot(self){reconstruction::retire_snapshot(self)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{let maximum=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),super::native_pack::record_spec_producer(),|record,native|super::native_pack::reconstruct_record_controlled(record,native,maximum),control)}
 fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{let counts=admission::forecast(self,control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),super::native_pack::record_spec_producer(),|native|super::native_pack::record_controlled(self,&counts,native),control)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  if dialect.artifact_kind!="s.forms.forms"||dialect.standard!="1"||dialect.subset!="*"{return Err("Forms exact coordinate differs".to_string().into())}
  let candidate=reconstruct(database,control)?;let actual=project(&candidate,control);reconstruction::retire_snapshot(candidate);
  if project(self,control)?!=actual?{return Err("Forms relational fields disagree with typed snapshot".to_string().into())}
  Ok(store::io_schema::IoOutcome::clean(()))
 }
}
