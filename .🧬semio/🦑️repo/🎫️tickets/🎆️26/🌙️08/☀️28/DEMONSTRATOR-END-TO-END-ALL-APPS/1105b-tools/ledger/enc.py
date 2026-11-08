p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛫️encode/🎟️effect/🦀️.rs"
s=open(p,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:120])
    s=s.replace(a,b)
rep("effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress,&mut Option<usize>,&mut Option<u64>)->Result<(),ValueError>)->RetainedWorkOutcome<Option<()>,NativeEncodeEffectFailure>{",
    "effect:&mut impl FnMut(&mut RetainedCloneProgress,&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Result<(),ValueError>)->RetainedWorkOutcome<Option<()>,NativeEncodeEffectFailure>{")
old_start="Some(bytes)=>{let settlement=&mut *marker.settlement;let performed=&mut *marker.performed;let physical=&mut *marker.physical;let attempted_work=&mut *marker.attempted_work;let failures=&mut *failure;let mut operation=|actual:&mut Option<usize>,work:&mut crate::OriginalWorkTurn<'_,'_>|{"
old_end="Err(error)=>{failures.as_mut().expect(\"resident original failure\").context=Some(error);Ok(false)}}};"
i=s.index(old_start);j=s.index(old_end,i)+len(old_end)
new="""Some(bytes)=>{let settlement=&mut *marker.settlement;let performed=&mut *marker.performed;let physical=&mut *marker.physical;let receipt=&mut *marker.performed_receipt;let claim=&mut *marker.work_claim;let partial=&mut *marker.partial_outcome;let failures=&mut *failure;let mut operation=|actual:&mut Option<usize>,work:&mut crate::OriginalWorkTurn<'_,'_>|{*performed=true;match run_closure_effect(settlement.as_ref().expect("resident original effect"),work,&mut *receipt,&mut *claim,demand,actual,|effect_|effect(physical,effect_)){Ok(Ok(()))=>{},Ok(Err(error))=>failures.as_mut().expect("resident original failure").primary=Some(error),Err(error)=>{failures.as_mut().expect("resident original failure").context=Some(error);return Ok(false)}}match settle_closure_work(settlement.as_mut().expect("resident original effect"),work,&mut *partial,*physical,*actual){Ok(ready)=>Ok(ready),Err(error)=>{failures.as_mut().expect("resident original failure").context=Some(error);Ok(false)}}};"""
s=s[:i]+new+s[j:]
rep("None=>{let run=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{if let Err(error)=effect(&mut *marker.performed,&mut *marker.physical,&mut *marker.actual,&mut *marker.attempted_work){failure.as_mut().expect(\"resident original failure\").primary=Some(error)}let did=*marker.performed||*marker.physical!=RetainedCloneProgress::default()||marker.actual.is_some();let mut work=crate::OriginalWorkTurn::new(&mut self.action);marker.settlement.as_mut().expect(\"resident original effect\").record_before_post(&mut work,*marker.physical,did,*marker.actual,*marker.attempted_work)}));",
    "None=>{let run=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{*marker.performed=true;let mut work=crate::OriginalWorkTurn::new(&mut self.action);match run_closure_effect(marker.settlement.as_ref().expect(\"resident original effect\"),&mut work,&mut *marker.performed_receipt,&mut *marker.work_claim,demand,&mut *marker.actual,|effect_|effect(&mut *marker.physical,effect_)){Ok(Ok(()))=>{},Ok(Err(error))=>failure.as_mut().expect(\"resident original failure\").primary=Some(error),Err(error)=>return Err(error)}settle_closure_work(marker.settlement.as_mut().expect(\"resident original effect\"),&mut work,&mut *marker.partial_outcome,*marker.physical,*marker.actual)}));")
rep("{let mut work=crate::OriginalWorkTurn::new(&mut self.action);let did=*marker.performed||*marker.physical!=RetainedCloneProgress::default()||marker.actual.is_some();if !marker.settlement.as_mut().expect(\"resident original effect\").record_before_post(&mut work,*marker.physical,did,*marker.actual,*marker.attempted_work)?{return Ok(None)}}",
    "{let mut work=crate::OriginalWorkTurn::new(&mut self.action);if !settle_closure_work(marker.settlement.as_mut().expect(\"resident original effect\"),&mut work,&mut *marker.partial_outcome,*marker.physical,*marker.actual)?{return Ok(None)}}")
rep("   completed=true;Ok(Some(()))","   if let Some(outcome)=marker.partial_outcome.take(){if let Some(refusal)=outcome.into_source_refusal(){failure.as_mut().expect(\"resident original failure\").work=Some(refusal)}}completed=true;Ok(Some(()))")
rep("impl NativeEncodeEffectProgress<'_>{","""/// 🪢️ Runs one closure inside one actual original effect over the performed-work receipt of the settled action.
fn run_closure_effect(settlement:&NativeEncodeEffectSettlement,work:&mut crate::OriginalWorkTurn<'_,'_>,receipt:&mut Option<semio_framework_value_control_contract::OriginalWorkPerformedReceipt>,claim:&mut Option<Result<u128,ValueError>>,grant:RetainedCloneGrant,actual:&mut Option<usize>,run:impl FnOnce(&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Result<(),ValueError>)->Result<Result<(),ValueError>,ValueError>{
 settlement.issue_performed(work,receipt)?;let mut guard=PerformedClaim{work,settlement,receipt,claim};
 let output={let sink=guard.receipt.as_mut().expect("original performed receipt issued");let mut effect=semio_framework_value_control_contract::RetainedAllocationEffect::new(grant,u64::MAX,actual,sink);run(&mut effect)};drop(guard);Ok(output)
}
/// 🧾️ Settles the claimed receipt and records the physical marker before any allocation Post.
fn settle_closure_work(settlement:&mut NativeEncodeEffectSettlement,work:&mut crate::OriginalWorkTurn<'_,'_>,partial:&mut Option<semio_framework_value_control_contract::OriginalWorkPartialOutcome>,physical:RetainedCloneProgress,actual:Option<usize>)->Result<bool,ValueError>{
 let pending=settlement.performed_pending(work)?;if pending&&partial.is_none()&&!settlement.settle_performed_into(work,partial)?{return Ok(false)}
 settlement.record_performed_before_post(work,physical,pending||partial.is_some(),actual)
}
impl NativeEncodeEffectProgress<'_>{""")
open(p,'w',encoding='utf8').write(s);print("ok")
