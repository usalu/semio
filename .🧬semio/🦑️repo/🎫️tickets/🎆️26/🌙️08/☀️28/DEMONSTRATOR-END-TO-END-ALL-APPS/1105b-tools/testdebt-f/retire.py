P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛫️encode/♻️retirement/🧪️tests/🦀️.rs"
lines=open(P,encoding='utf-8').read().split('\n')
head=lines[:7]
head[2]="use semio_framework_value_retirement::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};"
body=r'''#[path="../../🧪️tests/🎟️eff/🦀️.rs"]
mod eff;
fn close<T:RetireOwned>(value:T,bank:&mut RetainedCloneGrant,fuel:&mut u64){
 let mut owner=ControlledRetirement::retain(value);let(mut born,mut released)=(0,0);
 for _ in 0..4096{
  if owner.terminal_is_empty(){break}
  let quote=owner.next_computational_work(65536).unwrap();assert!(quote>0&&quote<=*fuel);let g=RetainedCloneGrant{maximum_items:bank.maximum_items.min(1),..*bank};let before=owner.next_demand(65536).unwrap();let mut marker=None;
  let(denied,heap)=eff::observed(RetainedCloneGrant{maximum_items:0,..g},quote,&mut marker,|effect|eff::settled(owner.step(effect)));assert_eq!(denied.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(marker,None);assert_eq!(owner.next_demand(65536).unwrap(),before);
  let(turn,heap)=eff::observed(g,quote,&mut marker,|effect|eff::settled(owner.step(effect)));let p=turn.result.unwrap().progress();let work=u64::try_from(owner.last_turn_work()).unwrap();assert!(p.fits(g));assert!(work<=quote);assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(marker,if p.retained_capacity_bytes==0{None}else{Some(p.retained_capacity_bytes)});born+=heap.0;released+=heap.1;bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;bank.maximum_capacity_bytes-=p.retained_capacity_bytes;bank.maximum_release_bytes-=p.released_bytes;*fuel-=work;
 }
 assert!(owner.terminal_is_empty());assert_eq!(born,released);
}
fn error(name:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,name)}
#[test]
fn failure_and_allocation_cursors_have_only_the_admitted_native_backing(){
 let mut bank=stock();let mut fuel=32768;assert_eq!(row("allocation-settled")["closeAllowed"],true);assert_eq!(row("operation-failure")["closeAllowed"],true);assert_eq!(row("effect-failure")["birthHeap"],0);
 close(NativeEncodeOperationFailure{primary:Some(error("primary")),work_refusal:Some(error("work"))},&mut bank,&mut fuel);
 close(NativeEncodeAllocationOutcome{request:Some(NativeAllocationRequest{bytes:16,owned_bytes:0,next_owned_bytes:16,maximum_bytes:64}),admitted:true,primary:Some(error("primary")),forwarded_quote:Some(error("forwarded quote").into()),scoped_quote:Some(error("scoped quote").into()),extent_refusal:Some(error("extent")),forwarded_refusal:Some(error("forwarded").into()),scoped_refusal:Some(error("scoped").into()),actual:Some(0)},&mut bank,&mut fuel);
 close(NativeEncodeEffectFailure{primary:Some(error("primary")),invariant:Some(error("invariant")),physical_invariant:Some(error("physical invariant")),capacity_invariant:Some(error("capacity invariant")),work:Some(error("work")),local:Some(error("local")),physical:Some(error("physical")),allocation:None},&mut bank,&mut fuel);
}
#[test]
fn consuming_continuation_constructs_its_typed_cursor_without_a_hidden_allocation(){
 let mut callback=|_|true;let mut original=NativeEncodeControl::new(64,&mut callback);original.charge(16).unwrap();let continuation=original.pause().unwrap();assert!(continuation.retirement_ready());assert_eq!(continuation.retirement_birth_bytes(),Some(size_of::<<NativeEncodeContinuation as RetireOwned>::RetirementCursor>()));
 let(mut cursor,heap)=observe(||continuation.retirement_seed());assert_eq!(heap,(row("continuation-settled")["birthHeap"].as_u64().unwrap()as usize,0));let mut actual=Some(73);let step=eff::with_effect(Default::default(),1<<20,&mut actual,|effect|cursor.close_step(effect));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!(actual,Some(73));assert!(!cursor.terminal_is_empty());let step=eff::with_effect(stock(),1<<20,&mut actual,|effect|cursor.close_step(effect));assert!(matches!(step,RetirementStep::Complete(1)));assert_eq!(actual,Some(73));assert!(cursor.terminal_is_empty());
}
#[test]
fn failed_typed_cursor_birth_preserves_both_original_failures_and_its_real_marker(){
 let mut bank=stock();let mut fuel=32768;let mut source=Some(NativeEncodeOperationFailure{primary:Some(error("primary")),work_refusal:Some(error("work"))});let mut actual=None;
 let(((turn,attempts),heap),())=(eff::with_effect(stock(),1<<20,&mut actual,|effect|observe(||refuse_one_allocation_on_this_thread(||semio_framework_value_retirement::retirement::admit_retirement_cursor(&mut source,effect)))),());
 assert_eq!(attempts,1);assert_eq!(heap,(0,0));assert_eq!(actual,Some(0));assert_eq!(turn.work,1);assert_eq!(turn.result.err().unwrap().kind,ValueRefusalKind::AllocationFailed);let source=source.take().unwrap();assert_eq!(source.primary.as_ref().unwrap().message.as_ref(),"primary");assert_eq!(source.work_refusal.as_ref().unwrap().message.as_ref(),"work");close(source,&mut bank,&mut fuel);
}
struct QuoteInterrupted;
impl NativeAllocationAuthorityPort for QuoteInterrupted{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{if matches!(event,NativeAllocationEvent::Quote(_)){let reply=reply.complete(Err(error("original quote refused").into()));std::hint::black_box(&reply);panic!("actual original quote callback interrupted")}Ok(reply.complete(Ok(())))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{Ok(reply.pending())}
}
#[test]
fn actual_pending_allocation_returns_to_its_original_callback_before_retirement(){
 let mut bank=stock();let mut fuel=32768;assert_eq!(row("allocation-pending")["closeAllowed"],false);
 let mut authority=NativeAllocationAuthority::new(QuoteInterrupted);let mut callback=|_|true;
 authority.with_send_loan(&mut (),|loan|{
  let mut native=NativeEncodeControl::new_forwarded(64,&mut callback,loan);let mut actual=None;let mut progress=None;
  let interrupted=catch_unwind(AssertUnwindSafe(||native.allocate_effect(16,&mut actual,&mut progress,&mut |_,_|panic!("refused original quote must not run an allocator"))));assert!(interrupted.is_err());assert_eq!(actual,None);assert!(progress.as_ref().unwrap().is_pending());assert!(!progress.as_ref().unwrap().retirement_ready());assert_eq!(progress.as_ref().unwrap().retirement_birth_bytes(),None);
  let request=progress.as_ref().unwrap().request();let(mut retired,heap)=observe(||progress.take().unwrap().retirement_seed());assert_eq!(heap,(0,0));assert!(!retired.terminal_is_empty());
  let mut marker=Some(73);let(step,heap)=eff::observed(stock(),1<<20,&mut marker,|effect|retired.close_step(effect));let refusal=match step{RetirementStep::Failure(refusal,0)=>refusal,_=>panic!("pending original was not retained")};assert_eq!(marker,Some(73));assert_eq!(heap,(0,0));close(refusal,&mut bank,&mut fuel);
  let mut output=retired.into_pending().ok().expect("pending original returns to its caller");assert_eq!(output.request(),request);
  assert!(native.recover_allocation_quotes(&mut output).unwrap());assert!(native.settle_allocation_progress(&mut output,None).unwrap());assert!(!output.is_pending());assert_eq!(output.outcome().forwarded_quote.as_ref().unwrap().owned().unwrap().message.as_ref(),"original quote refused");assert!(output.retirement_ready());drop(native);close(output,&mut bank,&mut fuel);
 }).unwrap();assert!(authority.into_port().is_ok());
}
struct SettlementWork{interrupt:bool,posted:u64,quote:u64,actual:u64,remaining:u64}
impl OriginalWorkRecipient for SettlementWork{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,_:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{assert_eq!(additional,self.quote);quote.complete(Ok(additional<=self.remaining))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{assert_eq!(check.total(),u128::from(self.actual));self.remaining=self.remaining.checked_sub(self.actual).unwrap();self.posted+=self.actual;let reply=check.complete(Err(error("original work post")));if self.interrupt{std::hint::black_box(&reply);panic!("original work post interrupted")}reply}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.pending()}
}
struct SettlementPhysical{interrupt:bool,submitted:RetainedCloneProgress,remaining:RetainedCloneGrant}
impl OriginalConstructionRecipient for SettlementPhysical{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{assert_eq!(quote.demand().copy_bytes,3);quote.complete(Ok(self.remaining))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let progress=receipt.progress();assert!(progress.fits(receipt.grant())&&progress.fits(self.remaining));self.remaining.maximum_items-=progress.copied_items;self.remaining.maximum_copy_bytes-=progress.copied_bytes;self.remaining.maximum_capacity_bytes-=progress.retained_capacity_bytes;self.remaining.maximum_release_bytes-=progress.released_bytes;self.submitted=self.submitted.checked_add(progress).unwrap();let reply=receipt.complete(Err(error("original physical post")));if self.interrupt{std::hint::black_box(&reply);panic!("original physical post interrupted")}reply}
}
#[test]
fn settled_and_interrupted_effects_keep_their_original_post_owners_until_reconciled(){
 for (id,interrupt_work,interrupt_physical) in [("settlement-settled",false,false),("settlement-work-pending",true,false),("settlement-physical-pending",false,true)]{
  let expected=row(id);let quote=expected["selectedWork"].as_u64().unwrap();let actual=expected["actualWork"].as_u64().unwrap();assert!(actual<quote);let mut bank=stock();let mut fuel=32768;
  let effect_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_depth:1,..Default::default()};assert!(effect_grant.maximum_items<=bank.maximum_items&&effect_grant.maximum_copy_bytes<=bank.maximum_copy_bytes&&quote<=fuel);bank.maximum_items-=effect_grant.maximum_items;bank.maximum_copy_bytes-=effect_grant.maximum_copy_bytes;fuel-=quote;
  let mut work=OriginalWorkAuthority::new(SettlementWork{interrupt:interrupt_work,posted:0,quote,actual,remaining:quote});let mut physical=OriginalConstructionAuthority::new(SettlementPhysical{interrupt:interrupt_physical,submitted:Default::default(),remaining:effect_grant});
  let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut state=NativeEncodeEffectState::default();let mut failure=None;let mut local=RetainedCloneProgress::default();let mut output=[0u8;3];
  macro_rules! turn{()=>{physical.with_loan(&mut (),|physical_loan|work.with_loan(&mut (),|work_loan|native.with_construction_recipient(physical_loan,|native|native.with_work_recipient(work_loan,|native|native.effect_allocated_with_work(effect_grant,&mut local,effect_grant,None,quote,&mut state.view(),&mut failure,&mut |progress,effect|{assert!(effect.performed(actual.try_into().unwrap())?);for byte in output.iter_mut(){*byte=1}*progress=RetainedCloneProgress{copied_items:1,copied_bytes:3,..Default::default()};Ok(())}))))).unwrap().unwrap().unwrap().unwrap()}}}
  let interrupted=interrupt_work||interrupt_physical;
  let turn=if interrupted{let attempt=catch_unwind(AssertUnwindSafe(||{turn!()}));assert!(attempt.is_err());assert_eq!(RetireOwned::retirement_ready(&state),expected["closeAllowed"].as_bool().unwrap());assert_eq!(RetireOwned::retirement_birth_bytes(&state).is_none(),expected["pending"].as_bool().unwrap());
   let(mut pending,heap)=observe(||RetireOwned::retirement_seed(std::mem::take(&mut state)));assert_eq!(heap,(expected["birthHeap"].as_u64().unwrap()as usize,0));let mut marker=Some(73);let(step,heap)=eff::observed(stock(),1<<20,&mut marker,|effect|pending.close_step(effect));let refusal=match step{RetirementStep::Failure(refusal,0)=>refusal,_=>panic!("pending original callback was consumed")};assert_eq!(marker,Some(73));assert_eq!(heap,(0,0));close(refusal,&mut bank,&mut fuel);state=pending.into_pending().ok().expect("pending effect returns to its caller");
   turn!()}else{turn!()};
  let failure_value=turn.result.err().expect("both independent post refusals are retained");
  assert_eq!(failure_value.work.as_ref().unwrap().message.as_ref(),"original work post");assert_eq!(failure_value.physical.as_ref().unwrap().message.as_ref(),"original physical post");assert!(failure_value.invariant.is_none());
  drop(native);assert_eq!(output,[1;3]);assert_eq!(work.inspect(|payer|payer.posted),actual);assert_eq!(work.inspect(|payer|payer.remaining),expected["unspentWork"].as_u64().unwrap());assert_eq!(physical.inspect(|recipient|recipient.submitted.copied_items),1);assert_eq!(physical.inspect(|recipient|recipient.submitted.copied_bytes),3);
  assert!(RetireOwned::retirement_ready(&state));close(failure_value,&mut bank,&mut fuel);close(state,&mut bank,&mut fuel);assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());
 }
}
'''
open(P,'w',encoding='utf-8').write('\n'.join(head)+'\n'+body)
print("ok")
