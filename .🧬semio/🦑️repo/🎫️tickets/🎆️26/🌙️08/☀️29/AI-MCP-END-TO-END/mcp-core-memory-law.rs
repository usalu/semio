use semio_framework_actor::retained_context::OriginalActorContextSlot;
use semio_framework_actor::retained_turn::OriginalActorTurnWork;
use semio_framework_actor::{OriginalGuestExecutionBudget, RetainedTurnInput};
use semio_framework_job::{OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_trace::observe_heap_allocations_on_this_thread;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value_testing_allocator as _;
use semio_plugin_interpreter::{CoreInstanceOpening, CoreModule, CoreOriginalInvocationStatus as Status, Value};
use std::sync::Arc;

const PAGE: usize = 65_536;
const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 8_192, maximum_copy_bytes: 2_000_000,
    maximum_capacity_bytes: 2_000_000, maximum_release_bytes: 2_000_000, maximum_depth: 64,
};

fn clock() -> Option<u64> { Some(1000) }

struct Caller {
    source: OriginalActorContextSlot,
    input: RetainedTurnInput,
}

impl Caller {
    fn new() -> Self {
        let mut caller = Self {
            source: OriginalActorContextSlot::empty(),
            input: RetainedTurnInput {
                operation: 344, generation: 3, epoch: 1,
                work: OriginalActorTurnWork { clock_binding: 195, absolute_deadline_us: 1_000_000, fuel: 1_000_000 },
                grant: RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 80_000_000,
                    maximum_capacity_bytes: 80_000_000, maximum_release_bytes: 80_000_000, maximum_depth: 64 },
            },
        };
        for _ in 0..30 {
            let budget = caller.budget(TURN);
            let call = caller.source.admit_step(caller.input, budget, clock);
            let ready = call.result.unwrap().ready;
            caller.settle(call.receipt_spent, call.work_units_spent);
            if ready { return caller; }
        }
        panic!("original Source did not reach funded readiness")
    }

    fn budget(&self, grant: RetainedCloneGrant) -> StepBudget {
        StepBudget::new(50_000, self.input.work.absolute_deadline_us, grant)
    }

    fn settle(&mut self, retained: RetainedCloneProgress, work: u64) {
        let receipt = self.input.return_original(retained, work).unwrap();
        self.input.epoch += 1;
        self.input.grant = receipt.remaining;
        self.input.work.fuel = receipt.remaining_fuel;
    }

    fn turn<T>(
        &mut self, grant: RetainedCloneGrant,
        body: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>,
    ) -> (T, (usize, usize, RetainedCloneProgress)) {
        let budget = self.budget(grant);
        let mut received = RetainedCloneProgress::default();
        let call = self.source.dispatch_step(self.input, budget, &mut received, clock, |cx| {
            let before = cx.retained_progress();
            let (result, heap) = observe_heap_allocations_on_this_thread(|| body(cx));
            let after = cx.retained_progress();
            let effects = RetainedCloneProgress {
                copied_items: after.copied_items - before.copied_items,
                copied_bytes: after.copied_bytes - before.copied_bytes,
                retained_capacity_bytes: after.retained_capacity_bytes - before.retained_capacity_bytes,
                released_bytes: after.released_bytes - before.released_bytes,
            };
            assert_eq!((heap.requested_bytes, heap.released_bytes), (effects.retained_capacity_bytes, effects.released_bytes));
            result.map(|value| (value, (heap.requested_bytes, heap.released_bytes, effects)))
        });
        self.settle(call.receipt_spent, call.work_units_spent);
        call.result.unwrap()
    }

    fn retire(mut self) {
        for _ in 0..100 {
            if self.source.terminal_is_empty() { return; }
            let budget = self.budget(TURN);
            let call = self.source.close_step(self.input, budget);
            call.result.unwrap();
            self.settle(call.receipt_spent, call.work_units_spent);
        }
        panic!("original Source did not retire")
    }
}

fn memory(owner: &CoreInstanceOpening) -> &[u8] { owner.resources().unwrap().memory(0).unwrap() }

fn start(caller: &mut Caller, module: &Arc<CoreModule>, argument: i32) -> CoreInstanceOpening {
    let mut owner = caller.turn(TURN, |cx| CoreInstanceOpening::begin(module, cx)).0.unwrap();
    for _ in 0..100 {
        if owner.resources_ready() { break; }
        caller.turn(TURN, |cx| owner.advance(module, cx));
    }
    assert_eq!(memory(&owner).len(), PAGE);
    assert!(caller.turn(TURN, |cx| owner.begin_function_step(module, 0, &[Value::I32(argument)], cx)).0);
    owner
}

fn step(owner: &mut CoreInstanceOpening, module: &Arc<CoreModule>, cx: &mut StepContext<'_>) ->
    Result<semio_plugin_interpreter::CoreOriginalInvocationCall, OriginalTurnFailure> {
    let input = RetainedTurnInput { grant: cx.retained_grant(), ..RetainedTurnInput::borrow_original(cx)? };
    owner.advance_scalar(module, OriginalGuestExecutionBudget { input, fuel: 1, deadline_us: cx.deadline_us() }, cx)
}

#[test]
fn zero_memory_growth_completes_with_no_capacity_or_release_budget() {
    let module = Arc::new(CoreModule::parse(include_bytes!(env!("SEMIO_LINKED_MEMORY_ZERO"))).unwrap());
    let mut caller = Caller::new();
    let mut owner = start(&mut caller, &module, 0);
    let no_allocation = RetainedCloneGrant { maximum_capacity_bytes: 0, maximum_release_bytes: 0, ..TURN };
    assert_eq!(caller.turn(TURN, |cx| step(&mut owner,&module,cx)).0.execution.spent_fuel, Some(1));
    let (call,(born,released,_)) = caller.turn(no_allocation, |cx| step(&mut owner,&module,cx));
    assert_eq!(call.result.unwrap(), Status::Pending);
    assert_eq!((call.execution.spent_fuel,born,released), (Some(1),0,0));
    assert_eq!(memory(&owner).len(),PAGE);
    for _ in 0..20 {
        if caller.turn(TURN, |cx| step(&mut owner,&module,cx)).0.result.unwrap() == Status::Complete { break; }
    }
    assert_eq!(owner.invocation_values(), Some(&[Value::I32(1)][..]));
    for _ in 0..100 {
        if owner.terminal_is_empty() { break; }
        caller.turn(TURN, |cx| owner.close_step(&module,cx));
    }
    assert!(owner.terminal_is_empty());
    caller.retire();
}

#[test]
fn original_memory_reuses_paid_pages_and_retires_exact_heap() {
    let binary = include_bytes!(env!("SEMIO_LINKED_MEMORY_FIXTURE"));
    let module = Arc::new(CoreModule::parse(binary).unwrap());
    for cancel in [false, true] {
        let mut caller = Caller::new();
        let mut owner = start(&mut caller, &module, 1);
        let scalar = |caller: &mut Caller, owner: &mut CoreInstanceOpening, grant| {
            let (call, receipt) = caller.turn(grant, |cx| step(owner,&module,cx));
            (call.result.unwrap(), call.execution.spent_fuel.unwrap(), receipt)
        };
        assert_eq!(scalar(&mut caller, &mut owner, TURN).1, 1);
        let (_, used, (_, _, effects)) = scalar(&mut caller, &mut owner, RetainedCloneGrant { maximum_capacity_bytes: PAGE, ..TURN });
        assert_eq!((used, effects), (0, RetainedCloneProgress::default()));
        let mut allocations = Vec::new();
        let mut releases = Vec::new();
        for _ in 0..50 {
            let (_, _, (born, released, _)) = scalar(&mut caller, &mut owner, TURN);
            if born >= PAGE { allocations.push(born); }
            if released >= PAGE { releases.push(released); }
            if memory(&owner).len() == 3 * PAGE { break; }
        }
        assert_eq!(memory(&owner).len(), 3*PAGE);
        for _ in 0..2 { assert_eq!(scalar(&mut caller,&mut owner,TURN).1,1); }
        let (status, used, (born, released, _)) = scalar(&mut caller, &mut owner, TURN);
        let (partial_status, partial_used, (part_born, part_released, partial)) = scalar(&mut caller, &mut owner,
            RetainedCloneGrant { maximum_copy_bytes: 1024, ..TURN });
        assert_eq!((status,used,partial_status,partial_used), (Status::Pending,0,Status::Pending,0));
        assert_eq!(memory(&owner).len(), 3*PAGE);
        assert_eq!(partial.copied_bytes, 1024);
        if cancel {
            caller.turn(TURN, |cx| { cx.original_cancel_token().cancel_now(); Ok(()) });
            assert_eq!(scalar(&mut caller,&mut owner,TURN).0, Status::Cancelled);
            assert_eq!(memory(&owner).len(), 3*PAGE);
        } else {
            for _ in 0..50 {
                if scalar(&mut caller,&mut owner,TURN).0 == Status::Complete { break; }
            }
            assert_eq!(owner.invocation_values(), Some(&[Value::I32(100)][..]));
            assert_eq!(memory(&owner).len(), 4*PAGE);
            assert_eq!(memory(&owner)[0], b'a');
            assert!(memory(&owner)[3*PAGE..].iter().all(|byte| *byte == 0));
        }
        let mut final_release = Vec::new();
        for _ in 0..100 {
            if owner.terminal_is_empty() { break; }
            let (_,(_, released, _)) = caller.turn(TURN, |cx| owner.close_step(&module,cx));
            if released >= PAGE { final_release.push(released); }
        }
        assert!(owner.terminal_is_empty());
        caller.retire();
        assert_eq!(allocations, [2*PAGE,4*PAGE]);
        assert_eq!(releases, [PAGE,2*PAGE]);
        assert_eq!((born,released,part_born,part_released), (0,0,0,0));
        assert_eq!(final_release, [4*PAGE]);
    }
}
