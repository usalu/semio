import sys
p=sys.argv[1]
s=open(p,encoding='utf8').read()
def rep(a,b,count=1):
    global s
    assert s.count(a)>=1,("missing",a[:80])
    s=s.replace(a,b) if count==0 else s.replace(a,b,count)
rep("use semio_framework_value_retirement::{RetainedAllocationEffect,RetireOwned,RetirementDemand,RetainedWorkOutcome,RetainedCloneProgress,RetainedCloneStep};",
"use semio_framework_value_retirement::{RetainedAllocationEffect,RetireOwned,RetirementDemand,RetainedWorkOutcome,RetainedCloneProgress,RetainedCloneStep};\nuse semio_framework_value_clone_source::{OriginalRetirementPoll,OriginalRetirementOutcome,RetainedWorkPoll,OriginalSourceTurnIdentity,OriginalWorkAuthority,OriginalWork,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkSink};")
rep("const CPU:u64=1048576;","""const CPU:u64=1048576;
struct Producer{maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original effect over the caller's allocation marker and returns the performed work the effect recorded.
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,u128){
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().expect("original fixture Work partial outcome");assert_eq!(partial.total(),performed);assert!(partial.source_refusal().is_none());assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}(output,performed)}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ A fixture drives every retirement turn to completion, so a suspended poll is a fixture fault.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}""")
rep("let mut total=RetainedCloneProgress::default();let mut work=0u64;","let mut total=RetainedCloneProgress::default();let mut work=0u128;let mut actual=None;")
rep("let turn=admission.advance(grant);work=work.saturating_add(turn.work);","let(turn,_)=with_effect(grant,CPU,&mut actual,|effect|admission.advance(effect));actual=None;work=work.saturating_add(turn.work);")
rep("work:u64,error:ValueError)->RetainedWorkOutcome<R,(ValueError,A,B)>{","work:u128,error:ValueError)->RetainedWorkOutcome<R,(ValueError,A,B)>{")
rep("let mut inputs=None;\n for _ in 0..256{admission.return_inputs(&mut inputs,release).result.expect(","let mut inputs=None;let mut actual=None;\n for _ in 0..256{with_effect(release,CPU,&mut actual,|effect|admission.return_inputs(&mut inputs,effect)).0.result.expect(")
rep("fn debit<E:FixtureFailure>(remaining:&mut RetainedCloneGrant,cpu:&mut u64,g:RetainedCloneGrant,turn:RetainedWorkOutcome<RetainedCloneStep,E>,heap:(usize,usize))->RetainedCloneStep{*cpu=cpu.checked_sub(turn.work).expect(","fn debit<E:FixtureFailure>(remaining:&mut RetainedCloneGrant,cpu:&mut u64,g:RetainedCloneGrant,turn:RetainedWorkOutcome<RetainedCloneStep,E>,heap:(usize,usize))->RetainedCloneStep{*cpu=cpu.checked_sub(u64::try_from(turn.work).expect(\"fixture work fits its bank\")).expect(")
rep("->semio_framework_value_clone_source::OriginalRetirementOutcome{let g=effect.grant();","->OriginalRetirementPoll{let g=effect.grant();")
rep("observe_retirement_allocations(||self.source.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect(\"retirement child exceeds original effect\")));","observe_retirement_allocations(||settled(self.source.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect(\"retirement child exceeds original effect\"))));")
rep("self.cpu=self.cpu.checked_sub(turn.work).expect(\"same fixture CPU bank\");","self.cpu=self.cpu.checked_sub(u64::try_from(turn.work).expect(\"fixture work fits its bank\")).expect(\"same fixture CPU bank\");")
rep("released_bytes).unwrap();turn\n }","released_bytes).unwrap();RetainedWorkPoll::Complete(turn)\n }")
rep("let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||self.source.close_step(&mut crate::RetainedAllocationEffect::new(g,units,&mut self.actual)));","let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||{let(source,actual)=(&mut self.source,&mut self.actual);with_effect(g,units,actual,|effect|settled(source.close_step(effect))).0});")
rep("observe_retirement_allocations(||if admission.phase()==\"ready\"{admission.take_output(&mut output,g)}else{admission.advance(&mut crate::RetainedAllocationEffect::new(g,cpu,&mut allocation_actual))});","observe_retirement_allocations(||if admission.phase()==\"ready\"{admission.take_output(&mut output,g)}else{with_effect(g,cpu,&mut allocation_actual,|effect|admission.advance(effect)).0});")
rep("let turn=source.close_step(&mut crate::RetainedAllocationEffect::new(g,units,&mut actual));","let turn=with_effect(g,units,&mut actual,|effect|settled(source.close_step(effect))).0;")
open(p,'w',encoding='utf8').write(s)
