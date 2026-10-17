//! 🏊️ Drawing retained mutation laws over the process arena pool: cap plus one, bootstrap deadline, failure at each allocation, exact cap and cancellation.

use super::*;

#[test]
fn retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba() {
    let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing process arena pool claims exact bytes and items before operation admission");
    let mut first = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let candidate = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_100 + index as u64), semio_framework_job::Generation(100 + index as u64), pool.clone())
            .expect("each fixed Drawing process arena slot admits exactly once");
        first.push((
            candidate.arena_slot,
            candidate.arena_generation,
            candidate.container_reverse.as_ref().expect("reverse arena owner").as_ptr(),
            candidate.container_output.as_ref().expect("output arena owner").as_ptr(),
            candidate.overlay_pages.as_ref().expect("page arena owner")[0].as_ptr(),
            candidate.duplicate_id_owner.as_ref().expect("duplicate id owner").as_ptr(),
            candidate,
        ));
    }
    match DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_999), semio_framework_job::Generation(999), pool.clone()) {
        Err(error) => assert_eq!(error, "drawing-store.mutation-arena-pool-saturated"),
        Ok(_) => panic!("fixed Drawing arena pool must reject capacity +1"),
    }
    for entry in &mut first {
        for phase in 0..4 {
            assert_eq!(entry.6.return_arena_owner().expect("one fixed Drawing root returns per opportunity"), Some(phase == 3));
            let state = pool.state.try_lock().expect("isolated Drawing pool is uncontended");
            let slot = &state.slots[entry.0];
            let returned = usize::from(slot.reverse.is_some()) + usize::from(slot.output.is_some()) + usize::from(slot.pages.is_some()) + usize::from(slot.duplicate_id.is_some());
            assert_eq!(returned, phase + 1, "exactly one fixed arena root returns per grant");
            assert_eq!(slot.leased, phase < 3, "slot becomes available only after the fourth exact owner returns");
        }
        close_candidate(&mut entry.6, None);
    }
    let first_witnesses: Vec<_> = first.iter().map(|entry| (entry.0, entry.1, entry.2, entry.3, entry.4, entry.5)).collect();
    for entry in first {
        drop(entry.6);
    }

    let mut second = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let candidate = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_200 + index as u64), semio_framework_job::Generation(200 + index as u64), pool.clone()).expect("returned Drawing arena slot re-admits");
        second.push(candidate);
    }
    for candidate in &second {
        let witness = first_witnesses.iter().find(|entry| entry.0 == candidate.arena_slot).expect("same fixed Drawing arena slot returns");
        assert!(candidate.arena_generation > witness.1, "slot generation advances to reject stale ABA returns");
        assert_eq!(candidate.container_reverse.as_ref().expect("reverse arena returned").as_ptr(), witness.2);
        assert_eq!(candidate.container_output.as_ref().expect("output arena returned").as_ptr(), witness.3);
        assert_eq!(candidate.overlay_pages.as_ref().expect("page arena returned")[0].as_ptr(), witness.4);
        assert_eq!(candidate.duplicate_id_owner.as_ref().expect("duplicate owner returned").as_ptr(), witness.5);
    }
    for candidate in &mut second {
        close_candidate(candidate, None);
    }
    for candidate in second {
        drop(candidate);
    }
}

fn step_arena_bootstrap(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> Result<bool, &'static str> {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_902), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel, semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
    bootstrap.step(&mut context)
}

thread_local! {
    static BOOTSTRAP_YIELD_CLOCK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

fn bootstrap_yield_clock() -> Option<u64> {
    Some(BOOTSTRAP_YIELD_CLOCK.with(|clock| {
        let now = clock.get();
        clock.set(now + 1);
        now
    }))
}

#[test]
fn retained_drawing_arena_bootstrap_deadline_between_guards_preserves_and_resumes_its_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧮️mutation-admission/🔣️.json")).unwrap();
    let law = &fixture["bootstrapYield"];
    let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().unwrap());
    assert_eq!(step_arena_bootstrap(&mut bootstrap), Ok(false));
    assert_eq!(step_arena_bootstrap(&mut bootstrap), Ok(false));
    let allocation = bootstrap.allocation;
    let operation = semio_framework_job::OperationId(7_906);
    let generation = semio_framework_job::Generation(79);
    let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).unwrap();
    let mut state = DrawingMutationArenaProcessState::Building(bootstrap);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    BOOTSTRAP_YIELD_CLOCK.with(|clock| clock.set(law["clockReadsUs"][0].as_u64().unwrap()));
    let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, law["deadlineUs"].as_u64().unwrap(), UNBOUNDED_STEP_GRANT), cancel.clone(), bootstrap_yield_clock, &mut preview_sequence, &mut context_retained);
    assert!(!context.should_yield(), "the outer admission guard still has time");
    let yielded = job.step_locked(&mut state, &mut context);
    let retained = match &state {
        DrawingMutationArenaProcessState::Building(owner) => Some((owner.allocation - allocation, owner.fault)),
        _ => None,
    };
    let mut terminal = None;
    for _ in 0..1_000 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
        match job.step_locked(&mut state, &mut context) {
            DrawingMutationArenaBootstrapStep::Pending { .. } | DrawingMutationArenaBootstrapStep::Blocked => {}
            outcome => { terminal = Some(outcome); break; }
        }
    }
    let actual = [if retained.is_some() { "yielded" } else { "retired" }, if terminal == Some(DrawingMutationArenaBootstrapStep::Ready) { "resumed" } else { "faulted" }];
    assert!(job.terminal, "the resumed or faulted owner completes its bounded lifecycle");
    drop(state);
    assert_eq!(yielded, DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 });
    assert_eq!(retained, Some((law["expectedAllocationDelta"].as_u64().unwrap() as usize, None)), "budget exhaustion preserves the exact building owner without allocating or faulting");
    assert_eq!(serde_json::to_value(actual).unwrap(), law["expectedStages"]);
}

fn close_arena_bootstrap_step(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> store::SnapshotRetirementStep {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_903), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel, semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
    bootstrap.close_step(&mut context)
}

fn close_arena_bootstrap(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> usize {
    let mut released_roots = 0;
    for _ in 0..10_000 {
        match close_arena_bootstrap_step(bootstrap) {
            store::SnapshotRetirementStep::Pending { released_items, .. } => {
                assert!(released_items <= 1, "Drawing arena bootstrap releases at most one exact root per grant");
                released_roots += released_items;
            }
            store::SnapshotRetirementStep::Complete => {
                assert!(bootstrap.terminal_is_empty());
                return released_roots;
            }
            store::SnapshotRetirementStep::Blocked => panic!("isolated Drawing arena bootstrap never blocks"),
        }
    }
    panic!("Drawing arena bootstrap retirement did not terminate")
}

fn build_arena_bootstrap_owners(bootstrap: &mut DrawingMutationArenaPoolBootstrap) {
    for _ in 0..10_000 {
        if bootstrap.owner == DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            return;
        }
        assert_eq!(step_arena_bootstrap(bootstrap), Ok(false));
    }
    panic!("Drawing arena bootstrap did not construct its fixed owner catalog")
}

#[test]
fn retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant() {
    let allocations = DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20;
    for failure_at in 0..allocations {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::new(Some(failure_at), None, usize::MAX, usize::MAX);
        let fault = loop {
            match step_arena_bootstrap(&mut bootstrap) {
                Ok(false) => {}
                Ok(true) => panic!("injected Drawing arena allocation failure was not observed"),
                Err(error) => break error,
            }
        };
        assert_eq!(fault, "drawing-store.mutation-arena-bootstrap-injected-allocation");
        assert_eq!(bootstrap.allocation, failure_at + 1);
        assert_eq!(close_arena_bootstrap(&mut bootstrap), failure_at, "every successfully constructed Vec/String/page root is handed to the retained fault cursor");
        drop(bootstrap);
    }
}

#[test]
fn retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close() {
    for owner in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::new(None, Some(owner), usize::MAX, usize::MAX);
        let fault = loop {
            match step_arena_bootstrap(&mut bootstrap) {
                Ok(false) => {}
                Ok(true) => panic!("injected Drawing arena bundle failure was not observed"),
                Err(error) => break error,
            }
        };
        assert_eq!(fault, "drawing-store.mutation-arena-bootstrap-injected-owner");
        assert_eq!(bootstrap.owner, owner + 1);
        assert_eq!(close_arena_bootstrap(&mut bootstrap), (owner + 1) * 20, "every completed bundle remains in the retained construction-fault owner");
        drop(bootstrap);
    }
}

#[test]
fn retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool() {
    let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().expect("fixed Drawing arena bootstrap claim"));
    let mut turns = 0;
    while !bootstrap.ready {
        let allocation = bootstrap.allocation;
        assert!(bootstrap.take_pool().is_none(), "an incomplete Drawing arena bootstrap cannot publish an operation-admission pool");
        assert!(matches!(step_arena_bootstrap(&mut bootstrap), Ok(false) | Ok(true)));
        assert!(bootstrap.allocation.saturating_sub(allocation) <= 1, "one governed bootstrap turn allocates at most one retained root");
        turns += 1;
        assert!(turns < 1_000);
    }
    let pool = bootstrap.take_pool().expect("terminal Drawing arena bootstrap publishes the fixed process pool");
    assert_eq!(pool.state.try_lock().expect("isolated Drawing arena pool is uncontended").slots.len(), DRAWING_MUTATION_ARENA_POOL_CAPACITY);
    drop(bootstrap);
    drop(pool);
}

#[test]
fn retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close() {
    let mut exact = DrawingMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
    build_arena_bootstrap_owners(&mut exact);
    let admitted_items = exact.admitted_items;
    let admitted_bytes = exact.admitted_bytes;
    let fixed = DrawingMutationArenaBootstrapAdmission::fixed().expect("four-slot bootstrap derives its claim from one configured arena owner");
    assert_eq!((fixed.maximum_items, fixed.maximum_bytes), (admitted_items, admitted_bytes), "boot4 is exactly pool4 multiplied by the actual one-owner A");
    exact.maximum_items = admitted_items;
    exact.maximum_bytes = admitted_bytes;
    assert_eq!(step_arena_bootstrap(&mut exact), Ok(true), "allocator-returned Drawing arena capacities admit at the exact boundary");
    assert_eq!(close_arena_bootstrap(&mut exact), DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20);
    drop(exact);

    for (maximum_items, maximum_bytes) in [(admitted_items - 1, admitted_bytes), (admitted_items, admitted_bytes - 1)] {
        let mut rejected = DrawingMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
        build_arena_bootstrap_owners(&mut rejected);
        rejected.maximum_items = maximum_items;
        rejected.maximum_bytes = maximum_bytes;
        assert_eq!(step_arena_bootstrap(&mut rejected), Err("drawing-store.mutation-arena-pool-capacity"));
        assert_eq!(close_arena_bootstrap(&mut rejected), DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20, "aggregate +1 rejection retains all eighty exact roots until cursorized close");
        drop(rejected);
    }
}

#[test]
fn retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation() {
    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let guard = state.try_lock().expect("isolated Drawing request fixture owns the inert process metadata");
    let witness = match &*guard {
        DrawingMutationArenaProcessState::Inert => (0, 0),
        DrawingMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
        DrawingMutationArenaProcessState::Ready(_) => (2, 0),
        DrawingMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
        DrawingMutationArenaProcessState::Fault(_) => (4, 0),
    };
    assert_eq!(request_drawing_mutation_arena_pool(), DrawingMutationArenaPoolAvailability::Contended);
    assert_eq!(request_drawing_mutation_arena_pool(), DrawingMutationArenaPoolAvailability::Contended, "a second app request coalesces fixed metadata without allocation");
    match borrow_drawing_mutation_arena() {
        Err(error) => assert_eq!(error, DrawingMutationArenaBorrowError::Contended),
        Ok(_) => panic!("borrow under process contention cannot expose an arena owner"),
    }
    let after = match &*guard {
        DrawingMutationArenaProcessState::Inert => (0, 0),
        DrawingMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
        DrawingMutationArenaProcessState::Ready(_) => (2, 0),
        DrawingMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
        DrawingMutationArenaProcessState::Fault(_) => (4, 0),
    };
    assert_eq!(after, witness, "default/request/borrow cannot advance a bootstrap allocation while no governed job owns the process turn");
    drop(guard);
}

#[test]
fn retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed() {
    let operation = semio_framework_job::OperationId(7_904);
    let generation = semio_framework_job::Generation(79);
    let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).expect("fixed Drawing bootstrap admission claim");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut exhausted_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut exhausted = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(0, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut exhausted_retained);
    assert_eq!(job.step(&mut exhausted), DrawingMutationArenaBootstrapStep::Blocked, "zero-budget bootstrap cannot allocate or retire");

    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let guard = state.try_lock().expect("isolated Drawing bootstrap fixture owns process contention");
    let mut contended_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut contended = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut contended_retained);
    assert_eq!(job.step(&mut contended), DrawingMutationArenaBootstrapStep::Blocked, "process contention leaves the exact bootstrap owner untouched");
    drop(guard);

    let local_operation = semio_framework_job::OperationId(7_905);
    let local_generation = semio_framework_job::Generation(79);
    let mut local_job = DrawingMutationArenaBootstrapJob::new(local_operation, local_generation).expect("local Drawing bootstrap job claims its fixed admission");
    let local_cancel = semio_framework_job::root_cancel_token();
    let mut local_preview_sequence = 0;
    let mut local_state = DrawingMutationArenaProcessState::Inert;
    for _ in 0..3 {
        let mut admitted_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut admitted = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), local_cancel.clone(), semio_framework_job::default_now_us, &mut local_preview_sequence, &mut admitted_retained);
        assert_eq!(local_job.step_locked(&mut local_state, &mut admitted), DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 });
    }
    let allocated = match &local_state {
        DrawingMutationArenaProcessState::Building(bootstrap) => bootstrap.allocation,
        _ => panic!("three governed Drawing bootstrap turns retain one partially allocated bundle"),
    };
    assert_eq!(allocated, 1, "only admitted worker turns may advance allocation boundaries");
    local_cancel.cancel_now();
    for _ in 0..100 {
        let mut cancelled_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut cancelled = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), local_cancel.clone(), semio_framework_job::default_now_us, &mut local_preview_sequence, &mut cancelled_retained);
        match local_job.step_locked(&mut local_state, &mut cancelled) {
            DrawingMutationArenaBootstrapStep::Pending { advanced_items } => assert!(advanced_items <= 1),
            DrawingMutationArenaBootstrapStep::Cancelled => break,
            DrawingMutationArenaBootstrapStep::Blocked => {}
            DrawingMutationArenaBootstrapStep::Ready | DrawingMutationArenaBootstrapStep::Fault(_) => panic!("cancelled partial Drawing bootstrap must retire to exact Cancelled"),
        }
    }
    assert!(local_job.terminal);

    let pool = DrawingMutationArenaPool::try_new().expect("isolated fixed Drawing pool admits exact saturation fixture");
    let mut candidates = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        candidates
            .push(DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_910 + index as u64), semio_framework_job::Generation(80 + index as u64), pool.clone()).expect("each fixed Drawing pool slot admits once"));
    }
    assert!(matches!(DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_999), semio_framework_job::Generation(99), pool), Err("drawing-store.mutation-arena-pool-saturated")));
    for candidate in &mut candidates {
        close_candidate(candidate, None);
    }
    for candidate in candidates {
        drop(candidate);
    }
}
