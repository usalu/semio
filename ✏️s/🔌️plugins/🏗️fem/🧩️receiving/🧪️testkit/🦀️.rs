//! 🧪️ Shared driven harness of the FEM model test crates: every funded turn runs as a genuine original effect of one finite bank.
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_testing_admission::{EffectBank,Wallet,bank_run,driven};

const BODY:usize=4096;
const STEPS:u32=1_000_000;
thread_local!{static BANK:std::cell::RefCell<EffectBank>=std::cell::RefCell::new(EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<20},1<<40)));}
/// 🎟️ Funds exactly one quoted demand.
pub fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🏦️ Runs one body inside the original effect of the shared bank, funded with exactly `grant`.
pub fn granted<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 BANK.with(|bank|{let mut output=None;
  bank_run(&mut bank.borrow_mut(),grant,1<<20,|effect|{output=Some(run(&mut *effect));RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work:effect.performed_work()})});
  output.expect("the shared bank ran its original action")
 })
}
/// ♻️ Retires one owned value through paid original close turns until terminal-empty.
pub fn retire<T:RetireOwned>(value:T){
 BANK.with(|bank|{let bank=&mut *bank.borrow_mut();
  let Ok(mut owner)=ControlledRetirement::new(value) else{panic!("owned value refused its controlled retirement")};
  for _ in 0..STEPS{
   if owner.terminal_is_empty(){return}
   let d=owner.next_demand(BODY).expect("retirement quote");let work=owner.next_computational_work(BODY).expect("retirement work quote");
   let grant=RetainedCloneGrant{maximum_copy_bytes:BODY.max(d.copy_bytes),..exact(d)};
   let(poll,_)=driven::<OriginalRetirementFailure>(bank,grant,work.max(1),|effect,reply,recover|if recover{owner.recover_step_into(BODY,effect,reply)}else{owner.step_into(BODY,effect,reply)});
   if let RetainedWorkPoll::Complete(turn)=poll{if let Err(error)=turn.result{panic!("retirement turn failed: {error:?}")}}
  }
  panic!("owned value failed its funded retirement")
 })
}
