use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
struct ScalarJob(bool);
impl InteractiveJob for ScalarJob{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{descriptor.yielded()}
 fn begin_close(&mut self){self.0=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if self.0{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{self.0=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))})}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn terminal_is_empty(&self)->bool{self.0}
}
fn source()->(WorkerJobSession<ScalarJob>,OriginalJobFixtureCaller){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(98201),Generation(1),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
 let params=BatchJobParams{operation:OperationId(98201),generation:Generation(1),cancel:caller.admit_alias(grant),config:BatchDriveConfig{retained:grant,site:"test.original-session-return",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:1000},now_us:logical_now_us};
 let session=admit_original_fixture_owner!(caller,WorkerJobSession,ScalarJob(false),params).unwrap_or_else(|_|panic!("original session admission"));(session,caller)
}
fn close_body(session:&mut WorkerJobSession<ScalarJob>,caller:&mut OriginalJobFixtureCaller){
 session.begin_close();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
 for _ in 0..64{if session.inner.phase()==SESSION_EMPTY{return;}if session.has_original_cancel_alias_witness(caller.cancel()).unwrap_or(false){caller.turn(grant,|cx|{let step=session.return_original_cancel_alias_step(cx.original_cancel_token(),cx.retained_grant()).map_err(|error|match error{WorkerJobDemandError::Refused(error)=>OriginalTurnFailure::from(error),WorkerJobDemandError::Contention(_)=>OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original frame alias remains checked out"))})?.unwrap();cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(step)}).unwrap();}else{caller.turn(grant,|cx|session.close_step(cx)).unwrap();}}
 panic!("original session body did not close");
}

fn original_return(session:WorkerJobSession<ScalarJob>,caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,WorkerJobSession<ScalarJob>)>{caller.turn(grant,|cx|{let result=session.return_terminal(cx.retained_grant());if let Ok(progress)=result.as_ref(){cx.consume_retained(*progress)?;cx.consume_fuel(progress.copied_items as u64);}Ok(result)}).unwrap()}

#[test]
fn original_session_frame_return_has_exact_custody(){
 let _admission=worker_session_slots_shared();
 let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../♻️session-return/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let(mut session,mut caller)=source();
  if row["phase"]=="terminal"{close_body(&mut session,&mut caller);}
  let held=(row["aliases"].as_u64().unwrap()>1).then(||session.inner.0.as_ref().unwrap().clone());
  let bytes=SessionHandle::<ScalarJob>::frame_bytes()+std::mem::size_of::<WorkerJobRetirementNode<ScalarJob>>();
  let granted_bytes=if row["releaseGrant"]==row["frameBytes"]{bytes}else{bytes-1};
  let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:granted_bytes,maximum_depth:1,..Default::default()};
  if row["waker"].as_bool().unwrap(){
   struct Wake;impl std::task::Wake for Wake{fn wake(self:Arc<Self>){}}
   let waker=std::task::Waker::from(Arc::new(Wake));
   unsafe{*session.inner.waker.get()=Some(waker)};
  }
  let (result,heap)=observe(||original_return(session,&mut caller,grant));
  assert_eq!(heap.requested_bytes,0);
  if row["expected"]=="returned"{
   let progress=result.unwrap_or_else(|_|panic!("funded original session return"));
   assert_eq!(progress.released_bytes,bytes);
   assert_eq!(heap.released_bytes,bytes);
  }else{
   let (_,original)=result.err().expect("original session retained");
   session=original;
   assert_eq!(heap.released_bytes,0);
   drop(held);
   if row["waker"].as_bool().unwrap(){unsafe{(&mut*session.inner.waker.get()).take()};}
   close_body(&mut session,&mut caller);
   let (result,heap)=observe(||original_return(session,&mut caller,RetainedCloneGrant{maximum_release_bytes:bytes,..grant}));
   assert_eq!(result.unwrap_or_else(|_|panic!("returned original after refusal")).released_bytes,bytes);
   assert_eq!((heap.requested_bytes,heap.released_bytes),(0,bytes));
  }
  caller.retire(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64});
 }
}

#[test]
fn original_terminal_witness_finishes_without_admitting_live_phase_contention(){let _slots=worker_session_slots_shared();let(mut session,mut caller)=source();for phase in [SESSION_TRANSITION,SESSION_IDLE,SESSION_SUBMITTED,SESSION_OUTCOME,SESSION_TERMINAL,SESSION_REJECTED,SESSION_CHECKED_OUT]{session.inner.phase.store(phase,Ordering::Release);assert!(matches!(session.has_original_cancel_alias_witness(caller.cancel()),Err(WorkerJobDemandError::Contention(_))));assert_eq!(session.inner.phase(),phase);}session.inner.phase.store(SESSION_IDLE,Ordering::Release);close_body(&mut session,&mut caller);assert_eq!(session.inner.phase(),SESSION_EMPTY);assert!(!session.has_original_cancel_alias_witness(caller.cancel()).unwrap());assert!(session.return_original_cancel_alias_step(caller.cancel(),WORKER_TEST_STOCK).unwrap().is_none());for _ in 0..8{if session.terminal_is_empty(){break}caller.close_owner(WORKER_TEST_STOCK,&mut session).unwrap();}assert!(session.terminal_is_empty());caller.retire(WORKER_TEST_STOCK);}

#[test]
fn parked_original_alias_uses_its_initiating_caller_without_slot_residue(){
 let _slots=worker_session_slots_exclusive();assert!(!worker_job_retirements_are_parked());let(mut session,mut caller)=source();session.begin_close();
 for _ in 0..32{if session.has_original_cancel_alias_witness(caller.cancel()).unwrap(){break}caller.turn(WORKER_TEST_STOCK,|cx|session.close_step(cx)).unwrap();}
 assert!(session.has_original_cancel_alias_witness(caller.cancel()).unwrap());assert_eq!(session.close_phase(),WorkerJobClosePhase::ParamsRelease);drop(session);assert!(worker_job_retirements_are_parked());
 let metadata=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
 for denied in [RetainedCloneGrant{maximum_items:0,..metadata},RetainedCloneGrant{maximum_depth:0,..metadata}]{let(step,heap)=observe(||caller.turn(denied,|cx|pump_worker_job_retirements(1,cx)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(worker_job_retirement_phase(1),Some(WorkerJobClosePhase::ParamsRelease));}
 let before=caller.cumulative_receipt();let(step,heap)=observe(||caller.turn(metadata,|cx|pump_worker_job_retirements(1,cx)).unwrap());let receipt=RetainedCloneProgress{copied_items:1,..Default::default()};assert_eq!(step,WorkerJobCloseStep::Pending{progress:receipt});assert_eq!(caller.cumulative_receipt(),before.checked_add(receipt).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 for _ in 0..32{if !worker_job_retirements_are_parked(){break}caller.turn(WORKER_TEST_STOCK,|cx|pump_worker_job_retirements(1,cx)).unwrap();}assert!(!worker_job_retirements_are_parked());assert!(caller.retire(WORKER_TEST_STOCK)>0);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
