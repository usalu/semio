//! 🪡️ Writer's original typing algebra moves whole edits and composes borrowed splice inputs boundedly.
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalCallerSourceLoan,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind,retained_clone::RetainedCloneClose};
use semio_plugin_decl_app::tool_contract::{OriginalTypingFoldPolicy,ArtifactOriginalTypingFoldFactory,ArtifactOriginalTypingFoldProducer};
use semio_framework_tool_typing_edit::OriginalTypingFold;
use semio_framework_text_splice_native::{TextSpliceView,OriginalTextSpliceComposer,OriginalTextSpliceComposition};
use semio_s_artifact_writer_writer_operations::{op::WriterMutation,schema::mutations::splice_text::SpliceText};
use std::mem::{ManuallyDrop,size_of};

/// 🏛️ The statically declared Writer algebra never falls back to a borrowed Clone callback.
pub static WRITER_ORIGINAL_TYPING_FOLD:WriterOriginalTypingFoldFactory=WriterOriginalTypingFoldFactory;
/// 🧭️ The concrete Writer producer is selected for its canonical mutation type.
pub fn original_typing_fold()->OriginalTypingFoldPolicy<WriterMutation>{OriginalTypingFoldPolicy::Factory(&WRITER_ORIGINAL_TYPING_FOLD)}
/// 🏭️ Writer declares one original producer for both whole-text and range-text edits.
pub struct WriterOriginalTypingFoldFactory;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Writer typing fold changed its original source or mutation pair")}
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>(),depth:1,..Default::default()}}
fn demand(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn pair(previous:&[WriterMutation],incoming:&[WriterMutation])->(usize,usize,usize,usize){(previous.as_ptr()as usize,previous.len(),incoming.as_ptr()as usize,incoming.len())}
fn views<'a>(previous:&'a[WriterMutation],incoming:&'a[WriterMutation])->Option<(TextSpliceView<'a>,TextSpliceView<'a>)>{
 let([WriterMutation::SpliceText(a)],[WriterMutation::SpliceText(b)])=(previous,incoming)else{return None};
 let view=|p:&'a SpliceText|TextSpliceView{start:p.start,deleted:p.deleted.as_str(),insert:p.insert.as_str(),before:p.before.as_str(),after:p.after.as_str()};
 Some((view(a),view(b)))
}
impl ArtifactOriginalTypingFoldFactory<WriterMutation> for WriterOriginalTypingFoldFactory{
 fn birth_demands(&self,_:&[WriterMutation],_:&[WriterMutation],_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{capacity_bytes:size_of::<WriterOriginalTypingFold>(),..fixed::<WriterOriginalTypingFold>()})}
 fn admit(&self,previous:&[WriterMutation],incoming:&[WriterMutation],cx:&mut StepContext<'_>,output:&mut Option<Box<dyn ArtifactOriginalTypingFoldProducer<WriterMutation>>>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?;
  let d=self.birth_demands(previous,incoming,cx.retained_grant().maximum_copy_bytes)?;let binding=OriginalSourceBinding::borrow(cx);let identity=pair(previous,incoming);let body=cx.retained_grant().maximum_copy_bytes;
  admit_original_construction(cx,output,demand(d),|control|{let decoder=(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(invalid)?);Ok((Box::new(WriterOriginalTypingFold{binding,decoder,identity,body,phase:0,closing:false,caller:ManuallyDrop::new(None),composer:ManuallyDrop::new(None),composition:ManuallyDrop::new(None),decision:ManuallyDrop::new(None),retirement:Default::default()})as Box<dyn ArtifactOriginalTypingFoldProducer<WriterMutation>>,receipt(d)))})
 }
}
struct WriterOriginalTypingFold{
 binding:OriginalSourceBinding,decoder:(u64,u64),identity:(usize,usize,usize,usize),body:usize,phase:u8,closing:bool,
 caller:ManuallyDrop<Option<OriginalCallerSourceLoan>>,composer:ManuallyDrop<Option<Box<OriginalTextSpliceComposer>>>,composition:ManuallyDrop<Option<OriginalTextSpliceComposition>>,decision:ManuallyDrop<Option<OriginalTypingFold<WriterMutation>>>,retirement:RetainedCloneClose,
}
impl WriterOriginalTypingFold{
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}
  if let Some(caller)=self.caller.as_ref(){caller.validate_context(cx)?;}Ok(())
 }
 fn validate_close(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}
  if let Some(caller)=self.caller.as_ref(){if !caller.terminal_is_empty(){caller.validate_retirement_context(cx)?;}}Ok(())
 }
 fn validate_pair(&self,previous:&[WriterMutation],incoming:&[WriterMutation])->Result<(),ValueError>{if self.identity!=pair(previous,incoming){return Err(invalid())}Ok(())}
 fn transition(&mut self,phase:u8,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=fixed::<u8>();let mut done=None;admit_original_construction(cx,&mut done,demand(d),|_|{self.phase=phase;Ok(((),receipt(d)))})}
 fn close_composer(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(composer)=self.composer.as_mut()else{return Ok(true)};
  if !composer.terminal_is_empty(){composer.close_step(cx)?;return Ok(false)}
  let d=OriginalTextSpliceComposer::release_demands();close_original_retained_step(cx,d,|_|{drop(self.composer.take());Ok(RetainedCloneStep::Complete(receipt(d)))})?;Ok(self.composer.is_none())
 }
 fn make_decision(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let composed=matches!(self.composition.as_ref(),Some(OriginalTextSpliceComposition::Composed(_)));
  let d=RetirementDemand{capacity_bytes:if composed{size_of::<WriterMutation>()}else{0},..fixed::<(OriginalTextSpliceComposition,OriginalTypingFold<WriterMutation>,WriterMutation,u8)>()};
  admit_original_construction(cx,&mut self.decision,demand(d),|_|{
   let result=match self.composition.take().ok_or_else(invalid)?{
    OriginalTextSpliceComposition::Composed(splice)=>{let mut net=Vec::with_capacity(1);net.push(WriterMutation::SpliceText(SpliceText{start:splice.start,deleted:splice.deleted,insert:splice.insert,before:splice.before,after:splice.after}));OriginalTypingFold::Net(net)},
    OriginalTextSpliceComposition::Cancelled=>OriginalTypingFold::Net(Vec::new()),
    OriginalTextSpliceComposition::Disjoint=>OriginalTypingFold::Split,
   };self.phase=3;Ok((result,receipt(d)))
  })
 }
}
impl ArtifactOriginalTypingFoldProducer<WriterMutation> for WriterOriginalTypingFold{
 fn demands(&self,previous:&[WriterMutation],incoming:&[WriterMutation],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate_pair(previous,incoming)?;
  if self.closing{return self.close_demands(body)}
  if self.caller.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  match self.phase{
   0=>if views(previous,incoming).is_some(){if self.composer.is_none(){Ok(OriginalTextSpliceComposer::birth_demands())}else{Ok(fixed::<u8>())}}else{Ok(fixed::<(OriginalTypingFold<WriterMutation>,u8)>())},
   1=>{let(a,b)=views(previous,incoming).ok_or_else(invalid)?;let composer=self.composer.as_ref().ok_or_else(invalid)?;if self.composition.is_some(){Ok(fixed::<u8>())}else if composer.ready(){composer.take_demands()}else{composer.demands(a,b,body)}},
   2=>if let Some(composer)=self.composer.as_ref(){if composer.terminal_is_empty(){Ok(OriginalTextSpliceComposer::release_demands())}else{composer.close_demands(body)}}else{let composed=matches!(self.composition.as_ref(),Some(OriginalTextSpliceComposition::Composed(_)));Ok(RetirementDemand{capacity_bytes:if composed{size_of::<WriterMutation>()}else{0},..fixed::<(OriginalTextSpliceComposition,OriginalTypingFold<WriterMutation>,WriterMutation,u8)>()})},
   3|4=>Ok(Default::default()),
   _=>Err(invalid()),
  }
 }
 fn advance(&mut self,previous:&[WriterMutation],incoming:&[WriterMutation],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;self.validate_pair(previous,incoming)?;if self.closing{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"Writer original typing fold was cancelled").into())}if self.ready(){return Ok(true)}
  if self.caller.is_none(){*self.caller=cx.original_source_parent()?.admit_loan(cx)?;return Ok(false)}
  match self.phase{
   0=>{if let Some((a,b))=views(previous,incoming){if self.composer.is_none(){OriginalTextSpliceComposer::admit(a,b,semio_ui_scene_views::TEXT_SPLICE_CONTEXT_SCALARS,cx,&mut self.composer)?;}else{self.transition(1,cx)?;}}else{let replace=matches!((previous,incoming),([WriterMutation::EditText(_)],[WriterMutation::EditText(_)]));let d=fixed::<(OriginalTypingFold<WriterMutation>,u8)>();admit_original_construction(cx,&mut self.decision,demand(d),|_|{self.phase=3;Ok((if replace{OriginalTypingFold::ReplaceIncoming}else{OriginalTypingFold::Split},receipt(d)))})?;}},
   1=>{if self.composition.is_some(){self.transition(2,cx)?;}else{let(a,b)=views(previous,incoming).ok_or_else(invalid)?;let composer=self.composer.as_mut().ok_or_else(invalid)?;if composer.ready(){composer.take_into(&mut self.composition,cx)?;}else{composer.advance(a,b,cx)?;}}},
   2=>{if self.composer.is_some(){self.close_composer(cx)?;}else{self.make_decision(cx)?;}},
   _=>return Err(invalid().into()),
  }Ok(self.ready())
 }
 fn ready(&self)->bool{self.phase==3&&!self.closing&&self.decision.is_some()}
 fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed::<(Option<OriginalTypingFold<WriterMutation>>,u8)>())}
 fn take_into(&mut self,output:&mut Option<OriginalTypingFold<WriterMutation>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if output.is_some(){return Err(invalid().into())}let d=self.take_demands()?;admit_original_construction(cx,output,demand(d),|_|{self.phase=4;Ok((self.decision.take().ok_or_else(invalid)?,receipt(d)))})
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Ok(fixed::<bool>())}
  if let Some(composer)=self.composer.as_ref(){return if composer.terminal_is_empty(){Ok(OriginalTextSpliceComposer::release_demands())}else{composer.close_demands(body)}}
  if !self.retirement.is_empty(){return self.retirement.demands(self.body)}
  if self.composition.is_some(){return self.retirement.next_owner_demands::<OriginalTextSpliceComposition>(true,&None,self.body)}
  if self.decision.is_some(){return self.retirement.next_owner_demands::<OriginalTypingFold<WriterMutation>>(true,&None,self.body)}
  if let Some(caller)=self.caller.as_ref(){return Ok(caller.retirement_demands())}Ok(fixed::<u8>())
 }
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_close(cx)?;if self.terminal_is_empty(){return Ok(true)}
  if !self.closing{let d=fixed::<bool>();close_original_retained_step(cx,d,|_|{self.closing=true;Ok(RetainedCloneStep::Progress(receipt(d)))})?;return Ok(false)}
  if self.composer.is_some(){self.close_composer(cx)?;return Ok(false)}
  if !self.retirement.is_empty(){let d=self.retirement.demands(self.body)?;close_original_retained_step(cx,d,|grant|self.retirement.select(self.body)?.step(grant))?;return Ok(false)}
  if self.composition.is_some(){let d=self.retirement.next_owner_demands::<OriginalTextSpliceComposition>(true,&None,self.body)?;close_original_retained_step(cx,d,|grant|Ok(self.retirement.begin_granted(&mut self.composition,grant)?.unwrap_or(RetainedCloneStep::Progress(Default::default()))))?;return Ok(false)}
  if self.decision.is_some(){let d=self.retirement.next_owner_demands::<OriginalTypingFold<WriterMutation>>(true,&None,self.body)?;close_original_retained_step(cx,d,|grant|Ok(self.retirement.begin_granted(&mut self.decision,grant)?.unwrap_or(RetainedCloneStep::Progress(Default::default()))))?;return Ok(false)}
  if let Some(caller)=self.caller.as_mut(){caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){drop(self.caller.take());}return Ok(false)}
  let d=fixed::<u8>();close_original_retained_step(cx,d,|_|{self.phase=5;Ok(RetainedCloneStep::Complete(receipt(d)))})?;Ok(self.terminal_is_empty())
 }
 fn terminal_is_empty(&self)->bool{self.phase==5&&self.caller.is_none()&&self.composer.is_none()&&self.composition.is_none()&&self.decision.is_none()&&self.retirement.is_empty()}
}
impl Drop for WriterOriginalTypingFold{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Writer original typing producer requires paid closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
