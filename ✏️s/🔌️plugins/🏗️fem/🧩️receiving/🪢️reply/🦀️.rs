//! 🪢️ Reply-shaped receiving steps shared by the FEM snapshot owners.
use semio_framework_dsl_record::receiving::NativeSnapshotFailure;
use semio_framework_value_native_owned::native_decoding::{NativeReceivingFailure,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionRecipient,OriginalSourceTurnIdentity};
use semio_framework_value::{RetainedAllocationEffect,retirement::RetireOwned,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{StagedSourceAdmission,RetainedCloneSource,OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits}};

/// 📬️ Runs one reply-shaped step inside its own action; a reply left pending is refused typed.
pub fn settled<T,E:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,call:impl for<'r,'e,'a> FnOnce(&'e mut RetainedAllocationEffect<'a>,OriginalEffectReply<'r,T,E>)->Result<OriginalEffectReplyComplete<'r,T,E>,ValueError>)->RetainedWorkPoll<T,E>{
 let before=effect.performed_work();let mut slot=None;
 let result=(||->Result<Result<T,E>,ValueError>{drop(call(&mut *effect,OriginalEffectReply::borrow(&mut slot)?)?);slot.take().ok_or_else(||ValueError::invariant("FEM receiving reply stayed pending"))})();
 RetainedWorkPoll::Complete(RetainedWorkOutcome{result:match result{Ok(result)=>result,Err(error)=>Err(error.into())},work:effect.performed_work().saturating_sub(before)})
}
/// 🪢️ Maps a native receiving failure onto the snapshot failure that keeps its exact cause and progress.
pub fn receiving_failure(failure:NativeReceivingFailure)->NativeSnapshotFailure{
 match failure{
  NativeReceivingFailure::Primary(primary)=>primary.into(),
  NativeReceivingFailure::ValueEffect(effect)=>effect.child.or(effect.producer_refusal).or(effect.work_refusal).map_or_else(||effect.receiving.into(),Into::into),
  NativeReceivingFailure::RetirementEffect(effect)=>effect.child.map(NativeSnapshotFailure::from).or_else(||effect.producer_refusal.or(effect.work_refusal).map(Into::into)).unwrap_or_else(||effect.receiving.into()),
  NativeReceivingFailure::ReceivingEffect(effect)=>effect.child.map(NativeSnapshotFailure::from).or_else(||effect.producer_refusal.or(effect.work_refusal).map(Into::into)).unwrap_or_else(||effect.receiving.into()),
 }
}
/// 📬️ Like [`settled`], but yields the settled result without the poll envelope.
pub fn step<T,E:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,call:impl for<'r,'e,'a> FnOnce(&'e mut RetainedAllocationEffect<'a>,OriginalEffectReply<'r,T,E>)->Result<OriginalEffectReplyComplete<'r,T,E>,ValueError>)->Result<T,E>{
 match settled(effect,call){RetainedWorkPoll::Complete(turn)=>turn.result,RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("FEM receiving reply stayed pending").into())}
}
/// 🎟️ Funds each sealed-row construction of one original receiving step from what the step grant still holds.
pub struct OriginalStepRecipient{grant:RetainedCloneGrant,progress:RetainedCloneProgress,submitted:RetainedCloneProgress}
impl OriginalStepRecipient{
 /// 🎫️ Starts a step with nothing spent.
 pub fn new(grant:RetainedCloneGrant)->Self{Self{grant,progress:Default::default(),submitted:Default::default()}}
}
impl OriginalConstructionRecipient for OriginalStepRecipient{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn grant<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'reply>)->OriginalConstructionGrantComplete<'reply>where 'source:'turn{
  let(g,p)=(self.grant,self.progress);
  let remaining=RetainedCloneGrant{maximum_items:g.maximum_items.saturating_sub(p.copied_items),maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(p.copied_bytes),maximum_capacity_bytes:g.maximum_capacity_bytes.saturating_sub(p.retained_capacity_bytes),maximum_release_bytes:g.maximum_release_bytes.saturating_sub(p.released_bytes),..g};
  let result=if retained_clone_grant_admits(remaining,quote.demand()){Ok(remaining)}else{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"FEM sealed row exceeds the original receiving step grant"))};
  quote.complete(result)
 }
 fn accept<'reply,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'reply>)->OriginalConstructionAcceptComplete<'reply>where 'source:'turn{
  let(g,p)=(receipt.grant(),receipt.progress());
  let result=self.submitted.checked_add(p).and_then(|submitted|{self.submitted=submitted;if p.fits(g){self.progress.checked_add(p).map(|progress|self.progress=progress)}else{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"FEM sealed row receipt exceeds its original receiving grant").with_retained_progress(p))}});
  receipt.complete(result)
 }
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
}

/// 📏️ Quotes the staged admission's identity projection while it is selected, else its next admission action.
pub fn admission_demand<V:RetireOwned+Sync+'static>(admission:&StagedSourceAdmission<V,V>)->Result<RetirementDemand,ValueError>{
 if admission.is_projecting(){admission.owned_projection_demand()}else{admission.next_demand()}
}
/// 🪡️ Publishes the sealed owner's own address while the identity projection is selected, else advances the admission.
pub fn admission_advance<V:RetireOwned+Sync+'static>(admission:&mut StagedSourceAdmission<V,V>,source:&mut Option<RetainedCloneSource<V>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
 if admission.is_projecting(){admission.project_owned(effect)}else{admission.advance(source,effect)}
}
