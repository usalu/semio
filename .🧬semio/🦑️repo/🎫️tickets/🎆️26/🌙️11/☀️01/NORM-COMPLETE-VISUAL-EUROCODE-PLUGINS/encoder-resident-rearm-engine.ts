
const p="🧰️framework/🔨️modules/🌱️value/🛬️decode/📬️outcome/🛫️encoding/🎟️effect/🏭️producer/🦀️.rs";let s=await Bun.file(p).text();const at=s.indexOf("macro_rules! native_encode_owned_producer");if(at<0||s.includes("struct NativeEncodeOwnedProducerBody"))throw Error("resident kernel guard");let head=s.slice(0,at),body=s.slice(at).replaceAll("NativeEncodeOwnedProducerFrame","NativeEncodeOwnedProducerBody").replace("pub struct NativeEncodeOwnedProducerBody","struct NativeEncodeOwnedProducerBody");
const trait="crate::native_receiving::span::effect::OriginalEffectProducer<Output=NativeEncodeProducerOutcome,Failure=NativeEncodeProducerFailure>";
const wrapper=`
 /// 🏠️ Owns native body, reply and rearm payment state without self-borrowing.
 pub struct NativeEncodeOwnedProducerFrame<P:${trait}>{body:NativeEncodeOwnedProducerBody<P>,rearm:Option<NativeEncodeReplyHeader>}
 impl<P> NativeEncodeOwnedProducerFrame<P> where P:${trait}{
  /// 🌱️ Freezes the actual original producer before its first callback.
  pub fn new(control:&mut NativeEncodeControl<'_>,source:crate::OriginalSourceTurnWitness<'_>,producer:P)->Result<Self,ValueError>{Ok(Self{body:NativeEncodeOwnedProducerBody::new(control,source,producer)?,rearm:None})}
  /// 👓️ Borrows the resident producer without mutable custody.
  pub fn producer(&self)->Option<&P>{self.body.producer()}
  /// 📭️ Borrows only a physically published reply outside an active rearm.
  pub fn reply(&self)->Option<&Result<NativeEncodeProducerOutcome,NativeEncodeProducerFailure>>{if self.rearm.as_ref().is_some_and(|header|!header.closed()){None}else{self.body.reply()}}
  /// 🧾️ Borrows the original body refusal.
  pub fn failure(&self)->Option<&NativeEncodeEffectFailure>{self.body.failure()}
  /// 🧮️ Reports actual unreported body Work.
  pub fn claim_body_work(&mut self)->Result<Option<u128>,ValueError>{self.body.claim_body_work()}
  /// ✅️ Acknowledges a successful original body partial.
  pub fn acknowledge_body_partial_success(&mut self)->Result<Option<u128>,ValueError>{self.body.acknowledge_body_partial_success()}
  /// 🧵️ Advances only the resident producer after its prior payment settles.
  pub fn advance_body<'turn,'context>(&mut self,control:&mut NativeEncodeControl<'_>,source:crate::OriginalSourceTurnWitness<'_>,context:&mut P::Context<'turn,'context>,grant:RetainedCloneGrant,selected:RetainedCloneGrant,maximum_work:u64,bytes:Option<usize>)->Result<NativeEncodeSourcePreparation,ValueError>where 'context:'turn{if let Some(header)=self.rearm.as_ref(){if !header.closed(){return Ok(NativeEncodeSourcePreparation::Pending)}self.rearm=None}self.body.advance_body(control,source,context,grant,selected,maximum_work,bytes)}
  /// 📐️ Quotes the real original reply destination.
  pub fn reply_demand(&self)->Result<RetirementDemand,ValueError>{self.body.reply_demand()}
  /// 📤️ Keeps the reply hidden through its actual Post.
  pub fn advance_reply<'turn,'context>(&mut self,control:&mut NativeEncodeControl<'_>,source:crate::OriginalSourceTurnWitness<'_>,context:&mut P::Context<'turn,'context>,grant:RetainedCloneGrant)->Result<NativeEncodeProducerReplyStep,ValueError>where 'context:'turn{if self.rearm.as_ref().is_some_and(|header|!header.closed()){return Ok(NativeEncodeProducerReplyStep::Pending)}self.body.advance_reply(control,source,context,grant)}
  /// 🧾️ Borrows the reply's original partial.
  pub fn reply_partial_outcome(&self)->Option<&semio_framework_value_control_contract::OriginalWorkPartialOutcome>{self.body.reply_partial_outcome()}
  /// ✅️ Acknowledges the original successful reply partial.
  pub fn acknowledge_reply_partial_success(&mut self)->Result<Option<u128>,ValueError>{self.body.acknowledge_reply_partial_success()}
  /// 🧮️ Reports original reply Work once.
  pub fn claim_reply_work(&mut self)->Result<Option<u128>,ValueError>{self.body.claim_reply_work()}
  /// 📐️ Quotes the complete rearm metadata without moving the actual producer.
  pub fn rearm_success_demand(&self)->Result<RetirementDemand,ValueError>{let mut demand=self.body.rearm_success_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<NativeEncodeReplyHeader>>()).ok_or_else(||refusal("owned rearm metadata extent overflow"))?;Ok(demand)}
  /// 🔄️ Performs the native paid rearm; the caller hides this owner through Post.
  pub fn rearm_success(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{let before=effect.performed_work();let result=(||{if self.rearm.as_ref().is_some_and(|header|!header.closed()){return Err(refusal("owned rearm already has its actual payment"))}let demand=self.rearm_success_demand()?;if !crate::retained_clone_grant_admits(effect.grant(),demand){return Ok(None)}let turn=self.body.rearm_success(effect);let result=turn.result?;if let Some(mut progress)=result{self.rearm=None;progress.copied_bytes=demand.copy_bytes;Ok(Some(progress))}else{Ok(None)}})();RetainedWorkOutcome{work:effect.performed_work()-before,result}}
  /// 🧵️ Retains the real rearm quote, Work and physical Post inside this same owner.
  pub fn advance_rearm(&mut self,control:&mut NativeEncodeControl<'_>,source:crate::OriginalSourceTurnWitness<'_>,grant:RetainedCloneGrant)->Result<NativeEncodeProducerReplyStep,ValueError>{self.body.validates_control(control,source)?;if self.rearm.is_none(){if !matches!(self.body.reply(),Some(Ok(_))){return Err(refusal("owned rearm requires its published success"))}self.rearm=Some(NativeEncodeReplyHeader{state:Default::default(),failure:None,local:Default::default(),demand:self.rearm_success_demand()?,complete:false,canceled:false})}self.rearm.as_mut().expect("owned rearm retained").advance_rearm(control,&mut self.body,source,grant)}
  /// 🧾️ Borrows the actual rearm partial and any resident Source refusal.
  pub fn rearm_partial_outcome(&self)->Option<&semio_framework_value_control_contract::OriginalWorkPartialOutcome>{self.rearm.as_ref().and_then(NativeEncodeReplyHeader::partial_outcome)}
  /// ✅️ Acknowledges only the genuine successful rearm partial.
  pub fn acknowledge_rearm_partial_success(&mut self)->Result<Option<u128>,ValueError>{match self.rearm.as_mut(){Some(header)=>header.acknowledge_partial_success(),None=>Ok(None)}}
  /// 🧮️ Reports this same rearm action's Work once.
  pub fn claim_rearm_work(&mut self)->Result<Option<u128>,ValueError>{match self.rearm.as_mut(){Some(header)=>header.claim_work(),None=>Ok(Some(0))}}
  /// 🧾️ Borrows independent real rearm receipt refusals.
  pub fn rearm_failure(&self)->Option<&NativeEncodeEffectFailure>{self.rearm.as_ref().and_then(|header|header.failure.as_ref())}
  /// 📦️ Quotes the complete final owner and actual producer destination.
  pub fn owner_transfer_demand(&self)->Result<RetirementDemand,ValueError>{let copy_bytes=size_of::<Self>().checked_add(size_of::<Option<P>>()).ok_or_else(||refusal("owned producer handoff extent overflow"))?;Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
  /// 🫴️ Moves the actual producer only under the original paid handoff.
  pub fn transfer_owner_into<'turn,'context>(&mut self,context:&P::Context<'turn,'context>,output:&mut Option<P>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>where 'context:'turn{let before=effect.performed_work();let result=(||{if self.rearm.as_ref().is_some_and(|header|!header.closed()){return Err(refusal("owned producer handoff retains its rearm payment"))}let demand=self.owner_transfer_demand()?;if !crate::retained_clone_grant_admits(effect.grant(),demand){return Ok(None)}let result=self.body.transfer_owner_into(context,output,effect).result?;if let Some(mut progress)=result{self.rearm=None;progress.copied_bytes=demand.copy_bytes;Ok(Some(progress))}else{Ok(None)}})();RetainedWorkOutcome{work:effect.performed_work()-before,result}}
 }
 impl<P> Drop for NativeEncodeOwnedProducerFrame<P>where P:${trait}{fn drop(&mut self){assert!(std::thread::panicking()||self.rearm.as_ref().is_none_or(NativeEncodeReplyHeader::closed),"owned encoder retains its actual rearm payment")}}
 impl NativeEncodeReplyHeader{
  fn advance_rearm<P:${trait}>(&mut self,control:&mut NativeEncodeControl<'_>,body:&mut NativeEncodeOwnedProducerBody<P>,source:crate::OriginalSourceTurnWitness<'_>,grant:RetainedCloneGrant)->Result<NativeEncodeProducerReplyStep,ValueError>{
   if self.complete{return Ok(if self.canceled{NativeEncodeProducerReplyStep::Canceled}else{NativeEncodeProducerReplyStep::Transferred(self.local)})}body.validates_control(control,source)?;
   let selected=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:self.demand.copy_bytes,maximum_capacity_bytes:self.demand.capacity_bytes,maximum_release_bytes:self.demand.release_bytes,maximum_depth:self.demand.depth};
   if self.state.native_outcome.is_none(){match control.prepare_source_effect(source,&mut self.state.view(),grant,self.local,selected,1)?{NativeEncodeSourcePreparation::Denied=>return Ok(NativeEncodeProducerReplyStep::Denied),NativeEncodeSourcePreparation::Pending=>return Ok(NativeEncodeProducerReplyStep::Pending),NativeEncodeSourcePreparation::Canceled=>{if control.cancel_source_effect(source,&mut self.state.view())?.is_none(){return Ok(NativeEncodeProducerReplyStep::Pending)}self.complete=true;self.canceled=true;return Ok(NativeEncodeProducerReplyStep::Canceled)},NativeEncodeSourcePreparation::Refused=>return Ok(NativeEncodeProducerReplyStep::Refused),NativeEncodeSourcePreparation::Ready=>{}}control.move_original_rearm(self,body,selected)?;}
   if self.state.producer_refusal.is_some()||self.state.claim_refusal.is_some(){return Ok(NativeEncodeProducerReplyStep::Refused)}if !control.settle_original_effect_work(&mut self.state.view())?||self.state.partial_outcome.is_some(){return Ok(NativeEncodeProducerReplyStep::Pending)}if !control.settle_source_effect(source,&mut self.state.view(),&mut self.failure,grant,&mut self.local)?{return Ok(NativeEncodeProducerReplyStep::Pending)}if self.failure.as_ref().is_some_and(|failure|!failure.is_empty()){return Ok(NativeEncodeProducerReplyStep::Refused)}self.complete=true;Ok(NativeEncodeProducerReplyStep::Transferred(self.local))
  }
 }
 impl NativeEncodeControl<'_>{
  fn move_original_rearm<P:${trait}>(&mut self,header:&mut NativeEncodeReplyHeader,body:&mut NativeEncodeOwnedProducerBody<P>,selected:RetainedCloneGrant)->Result<(),ValueError>{
   header.state.view().reconcile_work_claim()?;let settlement=header.state.settlement.as_ref().ok_or_else(||refusal("owned rearm settlement absent"))?;let mut work=crate::OriginalWorkTurn::new(&mut self.action);if header.state.producer_refusal.is_some()||header.state.partial_outcome.is_some()||header.state.claim_refusal.is_some()||header.state.performed_receipt.is_some()||settlement.performed_pending(&mut work)?{return Ok(())}settlement.issue_performed(&mut work,&mut header.state.performed_receipt)?;let guard=PerformedClaim{work:&mut work,settlement,receipt:&mut header.state.performed_receipt,claim:&mut header.state.work_claim};let turn={let receipt=guard.receipt.as_mut().expect("owned rearm receipt retained");let mut effect=semio_framework_value_control_contract::RetainedAllocationEffect::new(selected,1,&mut header.state.actual,receipt);body.rearm_success(&mut effect)};match turn.result{Ok(Some(mut progress))=>{progress.copied_bytes=header.demand.copy_bytes;header.state.native_outcome=Some(NativeEncodeProducerOutcome{performed:true,progress});header.state.producer_ready=true},Ok(None)=>{},Err(error)=>header.state.producer_refusal=Some(error)}drop(guard);header.state.view().reconcile_work_claim()
  }
 }
`;const end=body.lastIndexOf("}}");if(end<0)throw Error("macro close");body=body.slice(0,end)+wrapper+body.slice(end);await Bun.write(p,head+body);console.log("private owning body and resident rearm payment kernel authored");
