//! 📄️ Exercises the worker page admission under the recorded Generator phase grant.

use semio_framework_actor::{OriginalActorTurnWork, OriginalJobBudget, RetainedTurnInput};
use semio_framework_job::{OriginalTurnFailure, OriginalWorkerPageReservation, StepBudget, StepContext};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_reactor_context::OriginalActorGuestEntry;

const PHASE: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 54, maximum_copy_bytes: 30_639, maximum_capacity_bytes: 16_777_020,
    maximum_release_bytes: 1_045_988, maximum_depth: 64,
};

fn clock() -> Option<u64> { Some(100) }

fn budget(input: RetainedTurnInput, cancelled: bool) -> OriginalJobBudget {
    OriginalJobBudget { input, retained_policy: input.grant, fuel: input.work.fuel.min(65_536), deadline_us: input.work.absolute_deadline_us, cancelled }
}

fn turn<T>(entry: &mut OriginalActorGuestEntry, input: &mut RetainedTurnInput, run: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    let mut spent = Default::default();
    let call = entry.dispatch_value_step(*input, StepBudget::new(budget(*input, false).fuel, input.work.absolute_deadline_us, input.grant), &mut spent, clock, |cx| {
        cx.acknowledge_return_rows_locally();
        run(cx)
    });
    let returned = call.receipt().expect("original turn receipt");
    input.epoch += 1;
    input.grant = returned.remaining;
    input.work.fuel = returned.remaining_fuel;
    call.result.unwrap_or_else(|failure| panic!("mounted turn failed: {failure:?}"))
}

fn main() {
    let mut input = RetainedTurnInput {
        operation: 256, generation: 1, epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 256, absolute_deadline_us: 30_000_000, fuel: 50_000_000 },
        grant: RetainedCloneGrant { maximum_items: 1<<20, maximum_copy_bytes: 1<<29, maximum_capacity_bytes: 1<<28, maximum_release_bytes: 1<<28, maximum_depth: 64 },
    };
    let mut entry=OriginalActorGuestEntry::new();
    assert!((0..256).any(|_| {
        let call=entry.admit_plugin_step(budget(input,false),clock,|_| Ok(()));
        let returned=call.receipt().expect("original admission receipt");
        input.epoch+=1;
        input.grant=returned.remaining;
        input.work.fuel=returned.remaining_fuel;
        call.result.expect("admit actor").ready
    }));
    let mut worker=OriginalWorkerPageReservation::new();
    assert!((0..64).any(|index| turn(&mut entry,&mut input,|cx|{
        let before=cx.retained_progress();
        let work=cx.original_work_units_spent();
        let (ready,entered,borrowed)=cx.with_retained_loan(PHASE,|same|{
            Ok((worker.admit_step(same)?, !same.should_yield(),same.original_return_budget_is_borrowed()))
        })?;
        if !ready { eprintln!("[DEBUG] worker source turn {index} progress {before:?}->{:?}; work {}; entered={entered} borrowed={borrowed}",cx.retained_progress(),cx.original_work_units_spent()-work); }
        Ok(ready)
    })));
    assert!((0..256).any(|_| turn(&mut entry,&mut input,|cx|cx.with_original_retirement(|cx|{
        worker.close_step(cx)?;
        Ok(worker.terminal_is_empty())
    }))));
    assert!((0..256).any(|_| {
        let call=entry.close_plugin_step(budget(input,true));
        let returned=call.receipt().expect("original close receipt");
        input.epoch+=1;
        input.grant=returned.remaining;
        input.work.fuel=returned.remaining_fuel;
        call.result.expect("close actor");
        entry.terminal_is_empty()
    }));
    println!("[DEBUG] exact browser phase loan admits and retires the real worker page source");
}
