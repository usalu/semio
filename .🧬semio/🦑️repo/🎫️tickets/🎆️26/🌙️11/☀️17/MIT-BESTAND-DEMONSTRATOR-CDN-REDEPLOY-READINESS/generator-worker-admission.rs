//! 🎟️ Exercises the real mounted worker admission on the recorded Generator phase grant.

use semio_framework_actor::{OriginalActorTurnWork, RetainedTurnInput, retained_context::OriginalActorContextSlot};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, InteractiveJobStep, InteractiveStage, JobOutcomeDescriptor, JobOutcomeView, OriginalJobReturnClaim, OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_job_worker::{MountedWorkerJobSession, WorkerJobSessionAdmission};
use semio_framework_job_worker_authority::BatchDriveConfig;
use semio_framework_value::{RetainedCloneGrant, ValueError};
use semio_plugin_ownership::reserved_job_session::RefusedWorkerSource;

const PHASE: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 54, maximum_copy_bytes: 30_639, maximum_capacity_bytes: 16_777_020,
    maximum_release_bytes: 1_045_988, maximum_depth: 64,
};

#[repr(C, align(4))]
struct Probe([u8;16]);
impl InteractiveJob for Probe {
    fn step<'a>(&'a mut self, _: &mut StepContext<'_>, _: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'a> {
        panic!("admitting a worker must not call the initialization job");
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        descriptor.yielded()
    }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, _: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        Ok(InteractiveJobCloseStep::Complete { progress: Default::default() })
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn terminal_is_empty(&self) -> bool { true }
}

fn clock() -> Option<u64> { Some(100) }
fn budget(input: RetainedTurnInput) -> StepBudget {
    StepBudget::new(input.work.fuel.min(65_536), input.work.absolute_deadline_us, input.grant)
}

fn update(input: &mut RetainedTurnInput, returned: semio_framework_actor::RetainedTurnReceipt) {
    input.epoch += 1;
    input.grant = returned.remaining;
    input.work.fuel = returned.remaining_fuel;
}

fn turn<T>(actor: &mut OriginalActorContextSlot, core: bool, input: &mut RetainedTurnInput, run: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    let mut spent = Default::default();
    let call = if core {
        actor.dispatch_core_wire_step(*input,budget(*input), &mut spent, clock, false,run)
    } else {
        actor.dispatch_step(*input,budget(*input), &mut spent,clock,|cx| {
            cx.acknowledge_return_rows_locally();
            run(cx)
        })
    };
    update(input, call.receipt().expect("original turn receipt"));
    call.result.unwrap_or_else(|failure| panic!("mounted turn failed: {failure:?}"))
}

fn run(core: bool) {
    let mut input=RetainedTurnInput {
        operation:256,generation:1,epoch:1,
        work:OriginalActorTurnWork{clock_binding:256,absolute_deadline_us:30_000_000,fuel:50_000_000},
        grant:RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<29,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:64},
    };
    let mut actor=OriginalActorContextSlot::empty();
    assert!((0..64).any(|_| {
        let call=actor.admit_step(input,budget(input),clock);
        update(&mut input,call.receipt().expect("actual actor receipt"));
        call.result.expect("admit actor").ready
    }));
    let capacity=WorkerJobSessionAdmission::<Probe,MountedWorkerJobSession<Probe>>::birth_capacity_bytes();
    eprintln!("[DEBUG] core={core} worker birth capacity {capacity}");
    let mut source=None;
    let mut output=None;
    let mut consecutive_empty=0;
    let admitted=(0..128).position(|index|turn(&mut actor,core,&mut input,|cx|{
        let source=source.get_or_insert_with(||RefusedWorkerSource::pending(Probe([0;16]),(cx.operation(),cx.generation())));
        let extent=source.admission_demands()?;
        assert_eq!(extent.capacity_bytes,capacity);
        assert!(extent.capacity_bytes<=PHASE.maximum_capacity_bytes);
        let before=cx.retained_progress();
        let work=cx.original_work_units_spent();
        let result=cx.with_retained_loan(PHASE,|cx|{
            let config=BatchDriveConfig{retained:PHASE,site:"artifact_store_replacement",stage:InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
            source.admit_pending(config,&mut output,cx)
        })?;
        let after=cx.retained_progress();
        if before==after { consecutive_empty+=1; } else { consecutive_empty=0; }
        eprintln!("[DEBUG] core={core} turn={index} progress={before:?}->{after:?} work={} input={} output={} empty={consecutive_empty}",cx.original_work_units_spent()-work,source.job().is_some(),output.is_some());
        Ok(result)
    })).expect("worker setup must progress on the exact observed grant");
    assert!(source.as_ref().unwrap().terminal_is_empty());
    assert!(output.is_some());
    eprintln!("[DEBUG] core={core} worker session acknowledged after {} turns",admitted+1);
    output.as_mut().unwrap().begin_close();
    assert!((0..128).any(|_|turn(&mut actor,core,&mut input,|cx|cx.with_original_retirement(|cx|{
        let session=output.as_mut().unwrap();
        session.close_step(cx)?;
        Ok(session.terminal_is_empty())
    }))));
    eprintln!("[DEBUG] core={core} worker session is terminal-empty");
    turn(&mut actor,core,&mut input,|cx|{
        cx.acknowledge_return_rows_locally();
        semio_framework_job::original_return_source::reserve_original_return_rows(cx,1)?;
        Ok(())
    });
    assert!((0..8192).any(|_|{
        let call=actor.close_step(input,budget(input));
        update(&mut input,call.receipt().expect("actor close receipt"));
        call.result.expect("paid actor close");
        actor.terminal_is_empty()
    }));
}

fn main() {
    run(false);
    run(true);
}
