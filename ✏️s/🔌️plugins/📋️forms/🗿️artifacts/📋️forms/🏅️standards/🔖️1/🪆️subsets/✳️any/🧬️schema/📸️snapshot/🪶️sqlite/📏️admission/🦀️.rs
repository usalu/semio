//! 📏️ Literal Forms borrowed entity counts precede projection or typed ownership.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::super::FormsSnapshot;
use crate::FormExpr;
use semio_framework_value::DslValue;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
#[path="🪪️identities/🦀️.rs"]pub mod identities;

#[derive(Default)]
pub struct Counts{pub rows:usize,pub bytes:usize,pub steps:usize,pub questions:usize,pub options:usize,pub fields:usize,pub responses:usize,pub answers:usize,pub conditions:usize,pub condition_items:usize,pub values:usize,pub octets:usize,pub array_elements:usize,pub object_members:usize}
struct Forecast<'c,'p>{counts:Counts,control:&'c mut SqliteSnapshotControl<'p>,phase:SqliteSnapshotPhase,units:usize}
impl Forecast<'_,'_>{
 fn add(&mut self,rows:usize,bytes:usize)->Result<(),ValueError>{self.counts.rows=self.counts.rows.checked_add(rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms row count overflow"))?;self.counts.bytes=self.counts.bytes.checked_add(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Forms value byte count overflow"))?;self.control.check_rows(self.counts.rows)?;self.control.check_value_bytes(self.counts.bytes)?;self.step()}
 fn text(&mut self,value:&str)->Result<(),ValueError>{self.add(0,value.len())}
 fn whitespace(&mut self,value:&str)->Result<bool,ValueError>{
  self.text(value)?;self.control.checkpoint(self.phase,0,value.len())?;let mut position=0;
  while position<value.len(){let mut end=position.saturating_add(65536).min(value.len());while !value.is_char_boundary(end){end-=1;}for scalar in value[position..end].chars(){if !scalar.is_whitespace(){return Ok(false);}}position=end;self.control.checkpoint(self.phase,position,value.len())?;}
  Ok(true)
 }
 fn optional(&mut self,value:&Option<String>)->Result<(),ValueError>{if let Some(value)=value{self.text(value)?;}Ok(())}
 fn step(&mut self)->Result<(),ValueError>{self.units=self.units.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms forecast work overflow"))?;if self.units%256==0{self.control.checkpoint(self.phase,self.units,0)?;}Ok(())}
 fn repeated(&mut self,count:usize,bytes:usize)->Result<(),ValueError>{self.add(count,count.checked_mul(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Forms repeated byte count overflow"))?)}
 fn count(&mut self,field:fn(&mut Counts)->&mut usize,count:usize)->Result<(),ValueError>{let value=field(&mut self.counts);*value=value.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms component count overflow"))?;Ok(())}
 fn value(&mut self,root:&DslValue)->Result<(),ValueError>{
  let mut pending=Vec::new();self.frontier(&mut pending,1)?;pending.push(root);
  while let Some(value)=pending.pop(){self.add(2,16)?;self.count(|c|&mut c.values,1)?;
   match value{
    semio_framework_value::DslValue::Null=>self.text("null")?,semio_framework_value::DslValue::Bool(_)=>{self.text("boolean")?;self.add(0,8)?;}
    semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(_))=>{self.text("unsigned")?;self.add(0,16)?;}
    semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(_))=>{self.text("signed")?;self.add(0,8)?;}
    semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(value))=>{self.text("float")?;self.add(0,if value.is_nan(){11}else if value.is_infinite(){32}else{22})?;}
    semio_framework_value::DslValue::String(value)=>{self.text("text")?;self.text(value)?;}
    semio_framework_value::DslValue::Bytes(value)=>{self.text("bytes")?;self.repeated(value.len(),32)?;self.count(|c|&mut c.octets,value.len())?;}
    semio_framework_value::DslValue::Array(values)=>{self.text("array")?;self.repeated(values.len(),32)?;self.count(|c|&mut c.array_elements,values.len())?;self.control.check_rows(self.counts.rows.checked_add(values.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms child row count overflow"))?).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms child row count overflow"))?)?;self.frontier(&mut pending,values.len())?;for value in values.iter().rev(){pending.push(value);self.step()?;}}
    semio_framework_value::DslValue::Object(values)=>{self.text("object")?;self.repeated(values.len(),32)?;self.count(|c|&mut c.object_members,values.len())?;self.control.check_rows(self.counts.rows.checked_add(values.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms child row count overflow"))?).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms child row count overflow"))?)?;self.frontier(&mut pending,values.len())?;for(name,value)in values.iter().rev(){self.text(name)?;pending.push(value);self.step()?;}}
   }
  }Ok(())
 }
 fn frontier<T>(&mut self,values:&mut Vec<T>,count:usize)->Result<(),ValueError>{let required=values.len().checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms frontier count overflow"))?;let bytes=required.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Forms frontier byte count overflow"))?;self.control.check_value_bytes(self.counts.bytes.checked_add(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Forms frontier byte count overflow"))?)?;if required>values.capacity(){let mut next=store::sqlite_snapshot::transfer::reserve(required,self.control)?;next.append(values);*values=next;}Ok(())}
 fn condition(&mut self,root:&FormExpr)->Result<(),ValueError>{
  let mut pending=Vec::new();self.frontier(&mut pending,1)?;pending.push(root);
  while let Some(value)=pending.pop(){self.add(2,16)?;self.count(|c|&mut c.conditions,1)?;
   match value{FormExpr::Const{value}=>{self.text("const")?;self.add(0,8)?;self.value(value)?;}FormExpr::Var{name}=>{self.text("var")?;self.text(name)?;}FormExpr::Eq{left,right}=>{self.text("eq")?;self.add(0,16)?;self.frontier(&mut pending,2)?;pending.push(right);pending.push(left);}FormExpr::Truthy{expr}=>{self.text("truthy")?;self.add(0,8)?;self.frontier(&mut pending,1)?;pending.push(expr);}FormExpr::And{items}|FormExpr::Or{items}=>{self.text(if matches!(value,FormExpr::And{..}){"and"}else{"or"})?;self.repeated(items.len(),32)?;self.count(|c|&mut c.condition_items,items.len())?;self.control.check_rows(self.counts.rows.checked_add(items.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms condition row count overflow"))?).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms condition row count overflow"))?)?;self.frontier(&mut pending,items.len())?;for value in items.iter().rev(){pending.push(value);self.step()?;}}}
  }Ok(())
 }
}
pub fn forecast(snapshot:&FormsSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<Counts,ValueError>{
 control.checkpoint(phase,0,0)?;let mut f=Forecast{counts:Counts::default(),control,phase,units:0};f.add(3,24)?;f.text(&snapshot.schema)?;f.text(&snapshot.id)?;f.text(&snapshot.version)?;f.optional(&snapshot.title)?;
 for(child,subset)in[(&snapshot.structure.child_id,&snapshot.structure.target,"value"),(&snapshot.results.child_id,&snapshot.results.target,"table")].map(|(id,target,subset)|((id,target),subset)){
  if child.1.dialect.artifact_kind!="s.stdio.semio"||child.1.dialect.standard!="v1"||child.1.dialect.subset!=subset{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms child coordinate differs"))}f.text(child.0)?;f.text(&child.1.artifact_id)?;f.text(&child.1.dialect.artifact_kind)?;f.text(&child.1.dialect.standard)?;f.text(&child.1.dialect.subset)?;
 }
 if snapshot.schema!="forms.form"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms marker differs"))}
 for step in&snapshot.definition.steps{if step.id.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms step identity differs"))}f.add(1,24)?;f.count(|c|&mut c.steps,1)?;f.text(&step.id)?;f.text(&step.title)?;f.optional(&step.description)?;
  for q in&step.blocks{
   let option_count=q.options.as_ref().map_or(0,Vec::len);let field_count=q.fields.as_ref().map_or(0,Vec::len);f.control.check_rows(f.counts.rows.checked_add(1).and_then(|n|n.checked_add(option_count)).and_then(|n|n.checked_add(field_count)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms question row count overflow"))?)?;
   if q.id.is_empty()||f.whitespace(&q.kind)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms question identity differs"))}if[q.min,q.max,q.step].into_iter().flatten().any(|n|!n.is_finite())||q.min.zip(q.max).is_some_and(|(a,b)|a>b)||q.step.is_some_and(|n|n<=0.0){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms finite range invariant differs"))}
   f.add(1,40+usize::from(q.required.is_some())*8+usize::from(q.default.is_some())*8+usize::from(q.params.is_some())*8+usize::from(q.condition.is_some())*8)?;f.count(|c|&mut c.questions,1)?;f.text(&q.id)?;f.text(&q.label)?;
   for text in[&q.description,&q.placeholder,&q.unit,&q.text,&q.schema,&q.src,&q.accept,&q.fixture_slug]{f.optional(text)?;}
   for _ in[q.min,q.max,q.step].into_iter().flatten(){f.add(0,22)?;}
   for option in q.options.iter().flatten(){if option.value.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms option identity differs"))}f.add(1,24)?;f.count(|c|&mut c.options,1)?;f.text(&option.value)?;f.text(&option.label)?;}
   for field in q.fields.iter().flatten(){if field.key.is_empty()||field.value.is_some_and(|n|!n.is_finite()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms vector invariant differs"))}f.add(1,24+usize::from(field.value.is_some())*22)?;f.count(|c|&mut c.fields,1)?;f.text(&field.key)?;f.optional(&field.label)?;}
   if let Some(value)=&q.default{f.value(value)?;}if let Some(value)=&q.params{if !matches!(value,DslValue::Object(_)){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms params must own an object"))}f.value(value)?;}if let Some(value)=&q.condition{f.condition(value)?;}
  }
 }
 for response in&snapshot.responses{if response.id.is_empty()||response.definition_version.is_empty()||response.submitted_at>9_007_199_254_740_991{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms response invariant differs"))}f.add(1,40)?;f.count(|c|&mut c.responses,1)?;f.text(&response.id)?;f.text(&response.definition_version)?;
  for answer in&response.answers{if answer.question_id.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms answer identity differs"))}f.add(1,32)?;f.count(|c|&mut c.answers,1)?;f.text(&answer.question_id)?;f.text(&answer.label)?;f.text(&answer.kind)?;f.value(&answer.value)?;}
 }
 {let maximum=f.control.limits().max_value_bytes.checked_sub(f.counts.bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Forms identity admission byte limit exceeded"))?;f.control.allocation_stage(phase,|remaining,progress|{let mut callback=|p:semio_framework_value::native_decoding::NativeDecodeProgress|progress(p.completed,p.total);let mut native=semio_framework_value::NativeDecodeControl::new(remaining.min(maximum),&mut callback);let result=identities::snapshot(snapshot,&mut native);(result,native.owned_bytes())})??;}
 f.control.checkpoint(phase,f.counts.rows,f.counts.rows)?;Ok(f.counts)
}
