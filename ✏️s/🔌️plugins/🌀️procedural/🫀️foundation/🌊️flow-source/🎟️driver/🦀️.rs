//! 🎟️ Procedural scratch producers share the original caller's work and retirement driver.
use semio_framework_job::{OriginalTurnFailure,OriginalTurnPrimary,StepContext,receive_original_retained_work};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,retained_clone_grant_admits};
/// 🪙️ Runs one scratch step inside the turn's own original effect and settles its retained receipt and performed work once; `None` means the quote was not admitted and nothing ran.
pub fn funded_turn<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,E>)->Result<Option<RetainedCloneStep>,OriginalTurnFailure>{settled_turn(cx,false,run)}
/// 🪙️ Like [`funded_turn`] over an effect whose Work carries this resident Source's own turn.
pub fn funded_source_turn<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,E>)->Result<Option<RetainedCloneStep>,OriginalTurnFailure>{settled_turn(cx,true,run)}
fn settled_turn<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,source:bool,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,E>)->Result<Option<RetainedCloneStep>,OriginalTurnFailure>{
 let grant=cx.retained_grant();let Some((turn,performed))=(if source{cx.with_original_source_effect(grant,|_,effect|run(effect))?}else{cx.with_original_effect(grant,|_,effect|run(effect))?})else{return Ok(None)};
 let overrun=turn.work!=performed;let step=receive_original_retained_work(cx,turn)?;
 if overrun{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"scratch work differs from its original effect").into())}Ok(Some(step))
}
/// ⛽️ Reads the work this turn can still fund.
pub fn turn_work(cx:&StepContext<'_>)->u64{cx.fuel_remaining().min(cx.original_total_fuel_remaining())}
/// 🧾️ Charges one local scratch event to the effect; `false` means the quote was not funded and nothing was charged.
pub fn pay(effect:&mut RetainedAllocationEffect<'_>,work:u64)->Result<bool,ValueError>{
 if effect.remaining_work_units()?<work{return Ok(false)}if effect.performed(work)?{Ok(true)}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scratch work exceeded its quote"))}
}
/// 🧾️ The physical receipt of one local scratch event.
pub fn local_progress(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
/// ⏳️ A suspended child keeps its state and retries next turn as zero progress; only a settled failure is an error.
pub fn settled_step(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError>{match poll{RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(OriginalRetirementFailure::into_value_error)}}
/// 🪢️ Runs one reply-shaped callee and returns its recorded result; `None` means the callee suspended.
pub fn replied<E>(effect:&mut RetainedAllocationEffect<'_>,action:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->Result<Option<Result<RetainedCloneStep,E>>,ValueError>{
 let mut slot=None;let reply=OriginalEffectReply::borrow(&mut slot)?;action(&mut *effect,reply)?;Ok(slot.take())
}
pub trait OriginalScratchWork<S>{
 fn origin(&self)->(u64,u64);
 fn source_binding(&self)->semio_framework_job::OriginalSourceBinding;
 fn ready(&self)->bool;
 fn advancing_retirement(&self)->bool;
 fn next_demands(&self,source:&S,copy:usize,work:u64)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,source:&mut S,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>;
 fn step_progress(&self)->RetainedCloneProgress;
}
pub fn drive<S,W:OriginalScratchWork<S>>(work:&mut W,source:&mut S,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if work.origin()!=(cx.operation().0,cx.generation().0)||!work.source_binding().matches(cx){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"scratch work requires its initiating caller").into())}
 cx.original_source_parent()?.validate_borrowed_context(cx)?;
 for _ in 0..512{
  if work.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let retiring=work.advancing_retirement();
  if retiring&&!retained_clone_grant_admits(cx.original_total_retained_grant(),work.next_demands(source,cx.retained_grant().maximum_copy_bytes,turn_work(cx))?){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original scratch close demand exceeds the whole grant of its turn and can never fit").into())}
  let run=|effect:&mut RetainedAllocationEffect<'_>|{let turn=work.advance(source,effect);let progress=work.step_progress();RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(progress)),work:turn.work}};
  let step=if retiring{cx.with_original_retirement(|cx|funded_turn(cx,run))?}else{funded_turn(cx,run)?};
  let Some(step)=step else{return Ok(work.ready())};
  if work.ready()||step.progress()==Default::default(){return Ok(work.ready())}
 }
 Ok(work.ready())
}
/// 🧾️ Records the work a grant-shaped leaf reported into the caller's effect; an unfunded unit is a typed work-limit failure.
pub fn reported(effect:&mut RetainedAllocationEffect<'_>,work:u128)->Result<(),ValueError>{
 if work==0{return Ok(())}match u64::try_from(work){Ok(work) if effect.performed(work)?=>Ok(()),_=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scratch leaf work exceeds its effect quote"))}
}
/// 🚪️ Starts the retirement of one resident owner through the shared close field; a vacant begin is zero progress.
pub fn begun<T:semio_framework_value::retirement::RetireOwned>(retirement:&mut semio_framework_value::retained_clone::RetainedCloneClose,slot:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,semio_framework_value::RetirementPollExit>{
 retirement.begin_granted(slot,effect).result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(semio_framework_value::RetirementPollExit::from)
}
/// 🧾️ Folds a typed native receiving failure into its cause with the receipt it retained.
pub fn receiving_fault<E:semio_framework_pack_json::native_decoding::NativeReceivingEffectError>(error:E)->ValueError{let progress=error.receiving_progress();error.cause().cloned().unwrap_or_else(||ValueError::invariant("native receiving failed without a typed cause")).with_retained_progress(progress)}
/// 🫴️ Runs one quoted effect-shaped step inside one original effect; the turn is charged exactly the work it performed.
pub fn effected(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<bool,OriginalTurnFailure>{
 if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}
 let grant=cx.retained_grant();let Some((result,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(false)};
 let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 cx.consume_retained(progress)?;cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
 result?;Ok(true)
}
/// 🫴️ One native receiving turn of a polled cursor under the context's receiving scope.
pub fn receive<E:semio_framework_pack_json::native_decoding::NativeReceivingEffectError>(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut semio_framework_value::NativeDecodeControl<'_>,u64,semio_framework_value::RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,E>)->Result<(),OriginalTurnFailure>{
 if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(())}
 let maximum=cx.fuel_remaining();let mut out=None;let mut polled=None;
 cx.with_native_receiving(&mut out,|native,grant|{polled=Some(run(native,maximum,grant));Ok(())})?;
 match polled{Some(RetainedWorkPoll::Complete(turn))=>{semio_framework_job::receive_original_receiving_work(cx,RetainedWorkOutcome{result:turn.result.map_err(receiving_fault),work:turn.work})?;Ok(())},Some(RetainedWorkPoll::Pending{work})=>{cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));Ok(())},None=>Ok(())}
}
/// ♻️ Pays the receiver close start once, then closes it through its reply-shaped retirement cursor.
pub fn receiver_close<P:semio_framework_pack_json::native_decoding::owned::NativeReceive<semio_framework_value::DslValue>>(receiver:&mut semio_framework_value_native_receiving_original::NativeReceivingOriginal<semio_framework_value::DslValue,P>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
 let mut slot=None;
 let ran=if !receiver.is_closing(){OriginalEffectReply::borrow(&mut slot).and_then(|reply|receiver.begin_close_into(effect,reply).map(drop)).map(|()|None)}else{let mut stepped=None;OriginalEffectReply::borrow(&mut stepped).and_then(|reply|RetirementCursor::close_into(receiver,body,effect,reply).map(drop)).map(|()|stepped.take())};
 let work=effect.performed_work();
 match(ran,slot.take()){
  (Err(error),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),
  (Ok(_),Some(settled))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled,work}),
  (Ok(Some(settled)),None)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled.and_then(|step|match step{RetirementStep::Progress(progress,_)=>Ok(RetainedCloneStep::Progress(progress)),RetirementStep::Failure(failure,_)=>Err(failure),RetirementStep::Complete(_)|RetirementStep::Pending(_)|RetirementStep::BudgetExhausted|RetirementStep::Advanced=>Ok(RetainedCloneStep::Progress(Default::default())),_=>Err(ValueError::invariant("original provisional receiver close produced a foreign child step").into())}),work}),
  (Ok(None),None)=>RetainedWorkPoll::Pending{work},
 }
}
