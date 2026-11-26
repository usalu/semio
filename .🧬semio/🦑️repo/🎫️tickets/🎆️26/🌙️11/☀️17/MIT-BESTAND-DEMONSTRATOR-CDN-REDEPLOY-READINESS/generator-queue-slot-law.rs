//! 📏️ Checks the actual retirement queue's exact reservation and original frame quote.
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value_retirement_queue::{
    RetainedAllocationEffect, RetainedCloneGrant, RetainedWorkPoll, RetirementDemand, RetirementQueue,
};

fn funded<T>(demand: RetirementDemand, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> T) -> T {
    let grant = RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    };
    OriginalRetirementPump::new().turn(1 << 16, u64::MAX, || Some(0), grant).unwrap()
        .with_effect(run).unwrap().unwrap().0
}

fn main() {
    let mut queue = RetirementQueue::default();
    let mut original = Some("原🌊".repeat(4097));
    for _ in 0..64 {
        if queue.has_reserved_slot() {
            break;
        }
        funded(queue.reserve_demand().unwrap(), |effect| queue.reserve_step(effect)).result.unwrap();
    }
    assert!(queue.has_reserved_slot());
    let quoted = queue.admit_owned_slot_demand::<String>().unwrap();
    let concrete = queue.admit_owned_demand(original.as_ref().unwrap()).unwrap();
    assert_eq!(quoted, concrete);
    assert!(quoted.copy_bytes > 0);
    let short = RetirementDemand { copy_bytes: 0, ..quoted };
    assert!(funded(short, |effect| queue.admit_owned(&mut original, effect)).result.unwrap().is_none());
    assert_eq!(original.as_ref().unwrap().len(), 28679);
    assert!(funded(quoted, |effect| queue.admit_owned(&mut original, effect)).result.unwrap().is_some());
    assert!(original.is_none());
    for _ in 0..100_000 {
        if queue.terminal_is_empty() {
            return;
        }
        let copy = 64;
        if let RetainedWorkPoll::Complete(turn) = funded(queue.next_demand(copy).unwrap(), |effect| queue.step(copy, effect)) {
            turn.result.unwrap();
        }
    }
    panic!("exact queue retirement did not settle");
}
