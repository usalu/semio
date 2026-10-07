//! 📤️ Borrowed Forms fields project into explicit flat native records with one admission control.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::{records::*,FormsSnapshot};
use crate::FormExpr;
use semio_framework_value::DslValue;
use semio_framework_value::NativeEncodeControl;
use crate::standards::v1::subsets::any::io::sqlite::snapshot::admission::Counts;
struct Builder<'s,'n,'p>{values:Vec<&'s DslValue>,conditions:Vec<&'s FormExpr>,control:&'n mut NativeEncodeControl<'p>}
impl<'s>Builder<'s,'_,'_>{
 fn text(&mut self,value:&str)->Result<String,ValueError>{self.control.copy_text(value)}
 fn optional(&mut self,value:&Option<String>)->Result<Option<String>,ValueError>{value.as_deref().map(|v|self.text(v)).transpose()}
 fn value(&mut self,value:&'s DslValue)->Result<u64,ValueError>{let id=u64::try_from(self.values.len()).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Forms native value count overflow"))?;self.values.push(value);Ok(id)}
 fn condition(&mut self,value:&'s FormExpr)->Result<u64,ValueError>{let id=u64::try_from(self.conditions.len()).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Forms native condition count overflow"))?;self.conditions.push(value);Ok(id)}
 fn question(&mut self,q:&'s crate::FormQuestion)->Result<Question,ValueError>{
  let default=q.default.as_ref().map(|v|self.value(v)).transpose()?;let params=q.params.as_ref().map(|v|self.value(v)).transpose()?;let condition=q.condition.as_ref().map(|v|self.condition(v)).transpose()?;
  let options=if let Some(values)=&q.options{let mut out=self.control.allocate_vec(values.len())?;for value in values{out.push(OptionRecord{value:self.text(&value.value)?,label:self.text(&value.label)?});self.control.step()?;}Some(out)}else{None};
  let fields=if let Some(values)=&q.fields{let mut out=self.control.allocate_vec(values.len())?;for value in values{out.push(VectorRecord{key:self.text(&value.key)?,label:self.optional(&value.label)?,value:value.value});self.control.step()?;}Some(out)}else{None};
  self.control.step()?;Ok(Question{id:self.text(&q.id)?,label:self.text(&q.label)?,kind:self.text(&q.kind)?,description:self.optional(&q.description)?,required:q.required,placeholder:self.optional(&q.placeholder)?,default,min:q.min,max:q.max,step:q.step,unit:self.optional(&q.unit)?,text:self.optional(&q.text)?,options,fields,schema:self.optional(&q.schema)?,src:self.optional(&q.src)?,accept:self.optional(&q.accept)?,example_id:self.optional(&q.example_id)?,params,condition})
 }
 fn child<S>(&mut self,child:&store::ArtifactChild<S>)->Result<store::ArtifactChild<S>,ValueError>{Ok(store::ArtifactChild::new(self.text(&child.child_id)?,semio_framework_artifact_reference::ArtifactRef{artifact_id:self.text(&child.target.artifact_id)?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:self.text(&child.target.dialect.artifact_kind)?,standard:self.text(&child.target.dialect.standard)?,subset:self.text(&child.target.dialect.subset)?}}))}
}
pub(super) fn project<'s>(s:&'s FormsSnapshot,counts:&Counts,c:&mut NativeEncodeControl<'_>)->Result<Document,ValueError>{
 let work=[counts.steps,counts.questions,counts.options,counts.fields,counts.responses,counts.answers,counts.values,counts.conditions,counts.array_elements,counts.object_members,counts.condition_items].into_iter().try_fold(0usize,|n,m|n.checked_add(m).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms native projection work overflow")))?;c.begin_stage(work)?;let values=c.allocate_vec(counts.values)?;let conditions=c.allocate_vec(counts.conditions)?;let mut b=Builder{values,conditions,control:c};
 let mut document=Document{schema:b.text(&s.schema)?,id:b.text(&s.id)?,version:b.text(&s.version)?,title:b.optional(&s.title)?,structure:b.child(&s.structure)?,results:b.child(&s.results)?,steps:b.control.allocate_vec(s.definition.steps.len())?,responses:b.control.allocate_vec(s.responses.len())?,values:b.control.allocate_vec(counts.values)?,conditions:b.control.allocate_vec(counts.conditions)?};
 for step in &s.definition.steps{let mut out=Step{id:b.text(&step.id)?,title:b.text(&step.title)?,description:b.optional(&step.description)?,questions:b.control.allocate_vec(step.blocks.len())?};for q in &step.blocks{out.questions.push(b.question(q)?)}document.steps.push(out);b.control.step()?;}
 for response in &s.responses{let mut out=Response{id:b.text(&response.id)?,submitted_at:response.submitted_at,definition_version:b.text(&response.definition_version)?,answers:b.control.allocate_vec(response.answers.len())?};for answer in &response.answers{out.answers.push(Answer{question_id:b.text(&answer.question_id)?,label:b.text(&answer.label)?,kind:b.text(&answer.kind)?,value:b.value(&answer.value)?});b.control.step()?;}document.responses.push(out);b.control.step()?;}
 let mut index=0;while index<b.conditions.len(){let value=b.conditions[index];let mut node=Condition{kind:ConditionKind::Const,value:None,name:None,left:None,right:None,expression:None,items:Vec::new()};match value{
  FormExpr::Const{value}=>node.value=Some(b.value(value)?),FormExpr::Var{name}=>{node.kind=ConditionKind::Var;node.name=Some(b.text(name)?)},
  FormExpr::Eq{left,right}=>{node.kind=ConditionKind::Eq;node.left=Some(b.condition(left)?);node.right=Some(b.condition(right)?)},
  FormExpr::Truthy{expr}=>{node.kind=ConditionKind::Truthy;node.expression=Some(b.condition(expr)?)},
  FormExpr::And{items}|FormExpr::Or{items}=>{node.kind=if matches!(value,FormExpr::And{..}){ConditionKind::And}else{ConditionKind::Or};node.items=b.control.allocate_vec(items.len())?;for item in items{node.items.push(b.condition(item)?);b.control.step()?;}}
 }document.conditions.push(node);index+=1;b.control.step()?;}
 index=0;while index<b.values.len(){let value=b.values[index];let mut node=Value{kind:Kind::Null,boolean:None,unsigned:None,signed:None,float_bits:None,text:None,bytes:None,items:Vec::new(),members:Vec::new()};match value{
  semio_framework_value::DslValue::Null=>{},semio_framework_value::DslValue::Bool(value)=>{node.kind=Kind::Boolean;node.boolean=Some(*value)},
  semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value))=>{node.kind=Kind::Unsigned;node.unsigned=Some(*value)},semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(value))=>{node.kind=Kind::Signed;node.signed=Some(*value)},semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(value))=>{node.kind=Kind::Float;node.float_bits=Some(value.to_bits())},
  semio_framework_value::DslValue::String(value)=>{node.kind=Kind::Text;node.text=Some(b.text(value)?)},semio_framework_value::DslValue::Bytes(value)=>{node.kind=Kind::Bytes;node.bytes=Some(Octets(b.control.copy_bytes(value)?))},
  semio_framework_value::DslValue::Array(values)=>{node.kind=Kind::Array;node.items=b.control.allocate_vec(values.len())?;for value in values{node.items.push(b.value(value)?);b.control.step()?;}},
  semio_framework_value::DslValue::Object(values)=>{node.kind=Kind::Object;node.members=b.control.allocate_vec(values.len())?;for(name,value)in values{node.members.push(Member{name:b.text(name)?,value:b.value(value)?});b.control.step()?;}}
 }document.values.push(node);index+=1;b.control.step()?;}
 if document.values.len()!=counts.values||document.conditions.len()!=counts.conditions{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Forms native forecast disagrees with literal projection"))}b.control.checkpoint()?;Ok(document)
}
