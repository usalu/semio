//! 📥️ Original widget fields retain expected domain refusals before substituting declared defaults.
use super::*;
use semio_framework_artifact_flow_flow::{neural::{Dictionary,Tree},FlowGui,OrderedSet};
use semio_framework_value::{RetainedWorkOutcome,retirement::RetireOwned};
#[derive(Clone,Copy)]
pub(super) enum FieldKind{Text,Dictionary,Ports,Boolean,Number,Expanded,Tree,Gui}
#[derive(semio_framework_value::RetireOwned)]
pub(super) enum FieldValue{Text(String),Dictionary(Dictionary),Ports(Vec<String>),Boolean(bool),Number(f64),Expanded(OrderedSet),Tree(Tree),Gui(FlowGui)}
enum FieldCursor{Text(JsonField<String>),Dictionary(JsonField<Dictionary>),Ports(JsonField<Vec<String>>),Boolean(JsonField<bool>),Number(JsonField<f64>),Expanded(JsonField<OrderedSet>),Tree(JsonField<Tree>),Gui(JsonField<FlowGui>)}
macro_rules! cursor{($value:expr,$field:ident,$body:expr)=>{match $value{FieldCursor::Text($field)=>$body,FieldCursor::Dictionary($field)=>$body,FieldCursor::Ports($field)=>$body,FieldCursor::Boolean($field)=>$body,FieldCursor::Number($field)=>$body,FieldCursor::Expanded($field)=>$body,FieldCursor::Tree($field)=>$body,FieldCursor::Gui($field)=>$body}}}
impl FieldCursor{
 fn new(kind:FieldKind)->Self{match kind{FieldKind::Text=>Self::Text(JsonField::new()),FieldKind::Dictionary=>Self::Dictionary(JsonField::new()),FieldKind::Ports=>Self::Ports(JsonField::new()),FieldKind::Boolean=>Self::Boolean(JsonField::new()),FieldKind::Number=>Self::Number(JsonField::new()),FieldKind::Expanded=>Self::Expanded(JsonField::new()),FieldKind::Tree=>Self::Tree(JsonField::new()),FieldKind::Gui=>Self::Gui(JsonField::new())}}
 fn ready(&self)->bool{cursor!(self,f,f.ready())}
 fn demands(&self,source:&dyn JsonReadSource,copy:usize,work:u64)->Result<RetirementDemand,ValueError>{cursor!(self,f,f.demands(source,copy,work))}
 fn step(&mut self,source:&dyn JsonReadSource,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{cursor!(self,f,f.step(source,cx))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(FieldValue,RetainedCloneProgress)>,ValueError>{
  macro_rules! take{($f:expr,$v:ident)=>{$f.take(g).map(|result|result.map(|(value,p)|(FieldValue::$v(value),p)))}}
  match self{Self::Text(f)=>take!(f,Text),Self::Dictionary(f)=>take!(f,Dictionary),Self::Ports(f)=>take!(f,Ports),Self::Boolean(f)=>take!(f,Boolean),Self::Number(f)=>take!(f,Number),Self::Expanded(f)=>take!(f,Expanded),Self::Tree(f)=>take!(f,Tree),Self::Gui(f)=>take!(f,Gui)}
 }
 fn take_demand(&self)->RetirementDemand{cursor!(self,f,f.take_demand())}
 fn begin_close(&mut self){cursor!(self,f,f.begin_close())}
 fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{cursor!(self,f,f.close_demands(copy))}
 fn close_step(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{cursor!(self,f,f.close_step(g,effect))}
 fn terminal_is_empty(&self)->bool{cursor!(self,f,f.terminal_is_empty())}
}
struct FieldOwners{output:Option<FieldValue>,failure:Option<OriginalTurnFailure>}
pub(super) struct FieldWork{owners:ManuallyDrop<FieldOwners>,cursor:Option<FieldCursor>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl FieldWork{
 pub(super) fn new(kind:FieldKind)->Self{Self{owners:ManuallyDrop::new(FieldOwners{output:None,failure:None}),cursor:Some(FieldCursor::new(kind)),retirement:Default::default(),phase:0,closing:false}}
 fn expected_failure(error:&OriginalTurnFailure)->bool{if error.receipt_refusal.is_some()||error.context_refusal.is_some(){return false}if let Some(value)=error.primary_value(){return value.kind==ValueRefusalKind::InvalidValue}error.primary_receiving().is_some_and(|value|value.quote_refusal.is_none()&&value.allocation_refusal.is_none()&&value.primary.as_ref().is_some_and(|error|error.kind==ValueRefusalKind::InvalidValue))}
 pub(super) fn step(&mut self,source:&dyn JsonReadSource,cx:&mut StepContext<'_>)->Result<Option<Result<FieldValue,()>>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}match self.phase{
   0=>{let field=self.cursor.as_mut().ok_or_else(invalid)?;if field.ready(){let d=field.take_demand();if !retained_clone_grant_admits(cx.retained_grant(),d){return Ok(None)}if let Some((output,p))=field.take(cx.retained_grant())?{self.owners.output=Some(output);field.begin_close();self.phase=1;cx.consume_retained(p)?;cx.consume_fuel(1)}}else{match field.step(source,cx){Ok(_)=>{},Err(error)if Self::expected_failure(&error)=>{self.owners.failure=Some(error);field.begin_close();self.phase=1},Err(error)=>return Err(error)}}},
   1=>{let field=self.cursor.as_mut().ok_or_else(invalid)?;if !field.terminal_is_empty(){let d=field.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|g,effect|field.close_step(g,effect))?;return Ok(None)}let d=RetirementDemand{copy_bytes:size_of::<Option<FieldCursor>>(),..fixed()};if retained_clone_grant_admits(cx.retained_grant(),d)&&!cx.should_yield(){self.cursor=None;self.phase=2;cx.consume_retained(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})?;cx.consume_fuel(1)}},
   2=>{if !self.retirement.is_empty()||self.owners.failure.is_some(){let d=if self.retirement.is_empty(){self.retirement.next_owner_demands::<OriginalTurnFailure>(true,&None,cx.retained_grant().maximum_copy_bytes)?}else{self.retirement.demands(cx.retained_grant().maximum_copy_bytes)?};close_with_effect(cx,d,|g,effect|if self.retirement.is_empty(){begin_close_owner(&mut self.retirement,&mut self.owners.failure,effect)}else{poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))})?;return Ok(None)}let d=RetirementDemand{copy_bytes:size_of::<Option<FieldValue>>(),..fixed()};if retained_clone_grant_admits(cx.retained_grant(),d)&&!cx.should_yield(){self.phase=3;cx.consume_retained(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})?;cx.consume_fuel(1);return Ok(Some(self.owners.output.take().ok_or(())))}},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(field)=self.cursor.as_mut(){field.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(field)=self.cursor.as_ref(){return if field.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FieldCursor>>(),..fixed()})}else{field.close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.owners.failure.is_some(){return self.retirement.next_owner_demands::<OriginalTurnFailure>(true,&None,copy)}self.retirement.next_owner_demands::<FieldValue>(self.owners.output.is_some(),&None,copy)}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if let Some(field)=self.cursor.as_mut(){if !field.terminal_is_empty(){return field.close_step(g,effect)}let d=RetirementDemand{copy_bytes:size_of::<Option<FieldCursor>>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.cursor=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}}
  if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}
  macro_rules! owner{($field:ident)=>{if self.owners.$field.is_some(){return begin_close_owner(&mut self.retirement,&mut self.owners.$field,effect)}}}
  owner!(failure);owner!(output);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.is_none()&&self.owners.output.is_none()&&self.owners.failure.is_none()&&self.retirement.is_empty()}
}
impl Drop for FieldWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original composed field requires complete recipient and refusal closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners)}}}}
