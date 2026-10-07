//! 📥️ Flat Forms native records move into typed trees with explicit ownership and retirement.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::{records::*,FormsSnapshot};
use crate::{FormExpr,FormQuestion,FormQuestionOption,FormVectorField,FormStep};
use crate::schema::{definition::FormsDefinition,response::{FormsResponse,FormsAnswer}};
use semio_framework_value::DslValue;
use semio_framework_value::NativeDecodeControl;
use crate::standards::v1::subsets::any::io::sqlite::snapshot::reconstruction::{retire_value,retire_condition,retire_snapshot};
struct Forest{values:Vec<Option<DslValue>>,conditions:Vec<Option<FormExpr>>}
impl Drop for Forest{fn drop(&mut self){for v in &mut self.values{if let Some(v)=v.take(){retire_value(v)}}for v in &mut self.conditions{if let Some(v)=v.take(){retire_condition(v)}}}}
struct Values(Vec<DslValue>);
impl Drop for Values{fn drop(&mut self){for v in self.0.drain(..){retire_value(v)}}}
struct Members(Vec<(String,DslValue)>);
impl Drop for Members{fn drop(&mut self){for(_,v)in self.0.drain(..){retire_value(v)}}}
struct Conditions(Vec<FormExpr>);
impl Drop for Conditions{fn drop(&mut self){for v in self.0.drain(..){retire_condition(v)}}}
struct Expression(Option<FormExpr>);
impl Drop for Expression{fn drop(&mut self){if let Some(v)=self.0.take(){retire_condition(v)}}}
struct Primitive(Option<DslValue>);
impl Drop for Primitive{fn drop(&mut self){if let Some(v)=self.0.take(){retire_value(v)}}}
struct Owner(Option<FormsSnapshot>);
impl Drop for Owner{fn drop(&mut self){if let Some(v)=self.0.take(){retire_snapshot(v)}}}
fn take<T>(nodes:&mut[Option<T>],key:u64,parent:Option<usize>)->Result<T,ValueError>{let index=usize::try_from(key).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Forms native reference exceeds platform domain"))?;if index>=nodes.len()||parent.is_some_and(|p|index<=p){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native ordered topology differs"))}nodes[index].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Forms native repeated or missing ownership"))}
pub(super) fn reconstruct(document:Document,c:&mut NativeDecodeControl<'_>)->Result<FormsSnapshot,ValueError>{
 let Document{schema,id,version,title,structure,results,steps,responses,values,conditions}=document;
 c.begin_stage(0)?;let mut value_slots=c.allocate_vec(values.len())?;value_slots.resize_with(values.len(),||None);let mut condition_slots=c.allocate_vec(conditions.len())?;condition_slots.resize_with(conditions.len(),||None);let mut forest=Forest{values:value_slots,conditions:condition_slots};
 for(index,node)in values.into_iter().enumerate().rev(){c.step()?;let Value{kind,boolean,unsigned,signed,float_bits,text,bytes,items,members}=node;let value=match(kind,boolean,unsigned,signed,float_bits,text,bytes,items.is_empty(),members.is_empty()){
  (Kind::Null,None,None,None,None,None,None,true,true)=>semio_framework_value::DslValue::Null,
  (Kind::Boolean,Some(v),None,None,None,None,None,true,true)=>semio_framework_value::DslValue::Bool(v),
  (Kind::Unsigned,None,Some(v),None,None,None,None,true,true)=>semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(v)),
  (Kind::Signed,None,None,Some(v),None,None,None,true,true)=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(v)),
  (Kind::Float,None,None,None,Some(v),None,None,true,true)=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(f64::from_bits(v))),
  (Kind::Text,None,None,None,None,Some(v),None,true,true)=>semio_framework_value::DslValue::String(v),
  (Kind::Bytes,None,None,None,None,None,Some(v),true,true)=>semio_framework_value::DslValue::Bytes(v.0),
  (Kind::Array,None,None,None,None,None,None,_,true)=>{let mut out=Values(c.allocate_vec(items.len())?);for key in items{c.step()?;out.0.push(take(&mut forest.values,key,Some(index))?)}semio_framework_value::DslValue::Array(std::mem::take(&mut out.0))},
  (Kind::Object,None,None,None,None,None,None,true,_)=>{let mut out=Members(c.allocate_vec(members.len())?);for member in members{c.step()?;out.0.push((member.name,take(&mut forest.values,member.value,Some(index))?))}semio_framework_value::DslValue::Object(std::mem::take(&mut out.0))},
  _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native intrinsic record owns unrelated variant fields"))
 };forest.values[index]=Some(value);}
 for(index,node)in conditions.into_iter().enumerate().rev(){c.step()?;let Condition{kind,value,name,left,right,expression,items}=node;let value=match(kind,value,name,left,right,expression,items.is_empty()){
  (ConditionKind::Const,Some(key),None,None,None,None,true)=>FormExpr::Const{value:take(&mut forest.values,key,None)?},
  (ConditionKind::Var,None,Some(name),None,None,None,true)=>FormExpr::Var{name},
  (ConditionKind::Eq,None,None,Some(left),Some(right),None,true)=>{c.charge(2*std::mem::size_of::<FormExpr>())?;let mut left=Expression(Some(take(&mut forest.conditions,left,Some(index))?));let right=take(&mut forest.conditions,right,Some(index))?;FormExpr::Eq{left:Box::new(left.0.take().unwrap()),right:Box::new(right)}},
  (ConditionKind::Truthy,None,None,None,None,Some(key),true)=>{c.charge(std::mem::size_of::<FormExpr>())?;FormExpr::Truthy{expr:Box::new(take(&mut forest.conditions,key,Some(index))?)}},
  (kind @ ConditionKind::And,None,None,None,None,None,_)|(kind @ ConditionKind::Or,None,None,None,None,None,_)=>{let and=matches!(kind,ConditionKind::And);let mut out=Conditions(c.allocate_vec(items.len())?);for key in items{c.step()?;out.0.push(take(&mut forest.conditions,key,Some(index))?)}let items=std::mem::take(&mut out.0);if and{FormExpr::And{items}}else{FormExpr::Or{items}}},
  _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native condition record owns unrelated variant fields"))
 };forest.conditions[index]=Some(value);}
 let mut owner=Owner(Some(FormsSnapshot{schema,id,version,title,structure,results,definition:FormsDefinition{steps:c.allocate_vec(steps.len())?},responses:c.allocate_vec(responses.len())?}));
 for step in steps{let Step{id,title,description,questions}=step;owner.0.as_mut().unwrap().definition.steps.push(FormStep{id,title,description,blocks:c.allocate_vec(questions.len())?});for q in questions{let Question{id,label,kind,description,required,placeholder,default,min,max,step,unit,text,options,fields,schema,src,accept,example_id,params,condition}=q;
  let options=if let Some(values)=options{let mut out=c.allocate_vec(values.len())?;for v in values{out.push(FormQuestionOption{value:v.value,label:v.label});c.step()?;}Some(out)}else{None};
  let fields=if let Some(values)=fields{let mut out=c.allocate_vec(values.len())?;for v in values{out.push(FormVectorField{key:v.key,label:v.label,value:v.value});c.step()?;}Some(out)}else{None};
  let mut default=Primitive(default.map(|key|take(&mut forest.values,key,None)).transpose()?);let mut params=Primitive(params.map(|key|take(&mut forest.values,key,None)).transpose()?);let mut condition=Expression(condition.map(|key|take(&mut forest.conditions,key,None)).transpose()?);
  owner.0.as_mut().unwrap().definition.steps.last_mut().unwrap().blocks.push(FormQuestion{id,label,kind,description,required,placeholder,default:default.0.take(),min,max,step,unit,text,options,fields,schema,src,accept,example_id,params:params.0.take(),condition:condition.0.take()});c.step()?;
 }c.step()?;}
 for response in responses{let Response{id,submitted_at,definition_version,answers}=response;owner.0.as_mut().unwrap().responses.push(FormsResponse{id,submitted_at,definition_version,answers:c.allocate_vec(answers.len())?});for answer in answers{let value=take(&mut forest.values,answer.value,None)?;owner.0.as_mut().unwrap().responses.last_mut().unwrap().answers.push(FormsAnswer{question_id:answer.question_id,label:answer.label,kind:answer.kind,value});c.step()?;}c.step()?;}
 for slot in &forest.values{c.step()?;if slot.is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native unowned intrinsic node"))}}for slot in &forest.conditions{c.step()?;if slot.is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms native unowned condition node"))}}c.checkpoint()?;Ok(owner.0.take().unwrap())
}
