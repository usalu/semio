//! 🧬️ Typed input schema admission scans genuine native fields in bounded original events.
use super::retained_typed_literal::InputIdentity;
use semio_s_artifact_procedural_3d_mutations::{ChangeWidgetInput,WidgetInputValue};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn count(input:&WidgetInputValue)->usize{match input{WidgetInputValue::NumberList(v)=>v.len(),WidgetInputValue::TextList(v)=>v.len(),WidgetInputValue::BooleanList(v)=>v.len(),WidgetInputValue::PointList(v)|WidgetInputValue::VectorList(v)=>v.len(),_=>1}}
fn text(input:&WidgetInputValue,item:usize)->Option<&str>{match input{WidgetInputValue::Text(value)=>Some(value),WidgetInputValue::TextList(values)=>values.get(item).map(String::as_str),_=>None}}
fn number(input:&WidgetInputValue,item:usize,axis:usize)->Option<f64>{match input{
 WidgetInputValue::Number(value)=>if axis==0{Some(*value)}else{None},
 WidgetInputValue::NumberList(values)=>if axis==0{values.get(item).copied()}else{None},
 WidgetInputValue::Point(values)|WidgetInputValue::Vector(values)=>values.get(axis).copied(),
 WidgetInputValue::PointList(values)|WidgetInputValue::VectorList(values)=>values.get(item).and_then(|values|values.get(axis)).copied(),
 WidgetInputValue::Plane(value)=>if axis<3{value.origin.get(axis).copied()}else{value.normal.get(axis-3).copied()},
 _=>None,
}}
fn lanes(input:&WidgetInputValue)->usize{match input{WidgetInputValue::Number(_)|WidgetInputValue::NumberList(_)=>1,WidgetInputValue::Point(_)|WidgetInputValue::Vector(_)|WidgetInputValue::PointList(_)|WidgetInputValue::VectorList(_)=>3,WidgetInputValue::Plane(_)=>6,_=>0}}
fn collection(input:&WidgetInputValue)->bool{matches!(input,WidgetInputValue::NumberList(_)|WidgetInputValue::TextList(_)|WidgetInputValue::BooleanList(_)|WidgetInputValue::PointList(_)|WidgetInputValue::VectorList(_))}
#[derive(Clone,Copy)]
struct Binding{record:usize,id:(usize,usize),channel:(usize,usize),input:InputIdentity}
impl Binding{
 fn capture(input:&ChangeWidgetInput)->Self{Self{record:input as*const ChangeWidgetInput as usize,id:(input.id.as_ptr()as usize,input.id.len()),channel:(input.channel.as_ptr()as usize,input.channel.len()),input:InputIdentity::capture(&input.input)}}
 fn matches(self,input:&ChangeWidgetInput)->bool{self.record==input as*const ChangeWidgetInput as usize&&self.id==(input.id.as_ptr()as usize,input.id.len())&&self.channel==(input.channel.as_ptr()as usize,input.channel.len())&&self.input.matches(&input.input)}
}
pub(crate) struct TypedInputAdmission{binding:Option<Binding>,phase:u8,item:usize,axis:usize,offset:usize,characters:usize,result:Option<bool>,progress:RetainedCloneProgress}
impl TypedInputAdmission{
 pub(crate) fn new()->Self{Self{binding:None,phase:0,item:0,axis:0,offset:0,characters:0,result:None,progress:Default::default()}}
 pub(crate) fn result(&self)->Option<bool>{self.result}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 fn validate(&self,input:&ChangeWidgetInput)->Result<(),ValueError>{if self.binding.is_some_and(|binding|!binding.matches(input)){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed input admission changed its original native payload"))}else{Ok(())}}
 pub(crate) fn next_demands(&self,input:&ChangeWidgetInput,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(input)?;if self.result.is_some(){return Ok(Default::default())}
  Ok(fixed(match self.phase{
   1=>input.channel.len().saturating_sub(self.offset).min(copy.max(1).min(64)),
   3=>if self.item==count(&input.input){0}else if let Some(value)=text(&input.input,self.item){value.len().saturating_sub(self.offset).min(copy.max(1).min(64))}else if self.axis<lanes(&input.input){std::mem::size_of::<f64>()}else{0},
   _=>0,
  }))
 }
 fn finish(&mut self,admitted:bool){self.result=Some(admitted)}
 pub(crate) fn advance(&mut self,input:&ChangeWidgetInput,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();let demand=match self.next_demands(input,grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if self.result.is_some(){return RetainedWorkOutcome{result:Ok(true),work:0}}if !retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(false),work:0}}
  self.binding=Some(Binding::capture(input));self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  match self.phase{
   0=>{if input.id.is_empty()||input.channel.is_empty(){self.finish(false)}else{self.phase=1}},
   1=>{
    let end=self.offset+demand.copy_bytes;self.characters+=input.channel.as_bytes()[self.offset..end].iter().filter(|byte|**byte&0xc0!=0x80).count();self.offset=end;
    if self.characters>256{self.finish(false)}else if self.offset==input.channel.len(){self.offset=0;self.characters=0;self.phase=2}
   },
   2=>{if collection(&input.input)&&count(&input.input)>1024{self.finish(false)}else{self.phase=3}},
   3=>{
    if self.item==count(&input.input){self.finish(true)}
    else if let Some(value)=text(&input.input,self.item){
     let end=self.offset+demand.copy_bytes;self.characters+=value.as_bytes()[self.offset..end].iter().filter(|byte|**byte&0xc0!=0x80).count();self.offset=end;
     if self.characters>16_777_216{self.finish(false)}else if self.offset==value.len(){self.item+=1;self.offset=0;self.characters=0}
    }else if let Some(value)=number(&input.input,self.item,self.axis){if value.is_finite(){self.axis+=1}else{self.finish(false)}}
    else{self.item+=1;self.axis=0}
   },
   _=>unreachable!(),
  }
  RetainedWorkOutcome{result:Ok(self.result.is_some()),work:1}
 }
}
