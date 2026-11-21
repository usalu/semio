use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedWorkPoll, RetirementDemand, retirement::RetireOwned};
use semio_framework_value::retained_clone::{RetainedCloneSource, RetainedCloneSourceAdmission};
use std::mem::size_of;
use std::sync::Arc;

use semio_framework_value::retained_clone::{
    OriginalSourceTurnIdentity, OriginalWorkAuthority, OriginalWorkPartialCheck,
    OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete,
    OriginalWorkRecipient,
};
use semio_framework_value::{NativeDecodeControl, ValueError};

struct Work;
impl OriginalWorkRecipient for Work {
    type Context<'turn,'source> = () where 'source:'turn;
    fn validate_context<'turn,'source>(&self, _: &()) -> Result<(), ValueError> where 'source:'turn { Ok(()) }
    fn source_turn<'turn,'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source:'turn { None }
    fn admits<'r,'turn,'source>(&mut self, _: &mut (), q: OriginalWorkQuote<'r>, spent: u64, next: u64) -> OriginalWorkQuoteComplete<'r> where 'source:'turn { q.complete(Ok(spent.saturating_add(next) <= 1 << 22)) }
    fn settle_partial<'r,'turn,'source>(&mut self, _: &mut (), c: OriginalWorkPartialCheck<'r>) -> OriginalWorkPartialCheckComplete<'r> where 'source:'turn { c.complete(Ok(())) }
    fn recover_partial<'r,'turn,'source>(&mut self, _: &mut (), c: OriginalWorkPartialCheck<'r>) -> OriginalWorkPartialCheckComplete<'r> where 'source:'turn { c.complete(Ok(())) }
}

fn receive<R>(control: &mut NativeDecodeControl<'_>, run: impl FnOnce(&mut NativeDecodeControl<'_>) -> R) -> R {
    let mut authority = OriginalWorkAuthority::new(Work);
    authority.with_loan(&mut (), |work|
        control.with_work_recipient(work, |control| Ok::<_, ValueError>(run(control)))).unwrap().unwrap()
}



fn nested(mut d: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    d.depth += 1;
    Ok(d)
}

fn check<T: RetireOwned + Send + Sync + 'static>(original: T) {
    let mut admission = RetainedCloneSource::<T>::admission_owned(original, ());
    let mut output = None;
    let mut observer = |_| true;
    let mut caller = NativeDecodeControl::new(1 << 24, &mut observer);
    let mut blocked_work = Vec::new();
    let mut blocked_transfer = 0;

    for _ in 0..128 {
        if admission.terminal_is_empty() { break; }
        let stage = admission.phase();
        let demand = admission_demand(&admission).unwrap();
        let work = admission_work(&admission).unwrap();
        let mut advance = |quote: RetirementDemand, work: u64| {
            receive(&mut caller, |control| {
                control.with_allocated_effect_reported(quote, work, |_, effect| {
                    let mut child = effect.nested(RetainedCloneGrant {
                        maximum_depth: quote.depth - 1, ..effect.grant()
                    });
                    match stage {
                        "projectionStart" => admission.project_owned(&mut child),
                        "ready" => admission.take_output(&mut output, &mut child),
                        _ => admission.advance(&mut child),
                    }
                }).unwrap().unwrap().0.result.unwrap().progress()
            })
        };
        if work > 3 {
            assert_eq!(advance(demand, 3), RetainedCloneProgress::default(), "prior fixed Work grant");
            blocked_work.push(stage);
        }
        if stage == "ready" {
            let old = RetirementDemand { copy_bytes: size_of::<T>(), ..demand };
            assert!(old.copy_bytes < demand.copy_bytes);
            assert_eq!(advance(old, work), RetainedCloneProgress::default(), "prior pointer-size grant");
            blocked_transfer += 1;
        }
        assert_ne!(advance(demand, work), RetainedCloneProgress::default(), "{stage}");
    }

    assert!(admission.terminal_is_empty());
    assert!(blocked_work.contains(&"payloadInitialize"));
    assert!(blocked_work.contains(&"publication"));
    assert_eq!(blocked_transfer, 2);
    let mut output = output.unwrap();
    for _ in 0..512 {
        if output.terminal_is_empty() { break; }
        let demand = output.next_close_demand(4096).unwrap();
        let work = output.next_close_work_demand(demand.copy_bytes).unwrap();
        let old = caller.actual_work();
        let (step, _) = receive(&mut caller, |control| {
            control.with_allocated_effect_reported(demand, work, |_, effect| output.close_step(demand.copy_bytes, effect))
                .unwrap().unwrap()
        });
        if let RetainedWorkPoll::Complete(turn) = step {
            turn.result.unwrap();
        }
        assert!(caller.actual_work() > old || output.terminal_is_empty());
    }
    assert!(output.terminal_is_empty());
    println!("[DEBUG] Underfunded Work frontiers {blocked_work:?} and both underfunded handoff steps reproduced, then completed and retired through the real receiver.");
}

fn main() {
    check(Box::new(9u64));
    check(semio_framework_value::retirement::arc_custody::ArcLease::new(Arc::new(11u64)));
}
