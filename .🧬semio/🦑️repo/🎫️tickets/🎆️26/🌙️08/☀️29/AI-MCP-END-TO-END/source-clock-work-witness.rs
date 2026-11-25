use semio_framework_job_context::{CancelToken, Generation, OperationId, StepBudget, StepContext};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

static READS: AtomicU64 = AtomicU64::new(0);

fn clock() -> Option<u64> {
    Some(READS.fetch_add(1, Ordering::Relaxed) * 8)
}

fn main() {
    let mut parameters = std::env::args().skip(1).map(|value| value.parse::<u64>().unwrap());
    let iterations = parameters.next().unwrap();
    let fuel = parameters.next().unwrap();
    let (mut sequence, mut retained) = (0, Default::default());
    let budget = StepBudget { fuel, deadline_us: u64::MAX, retained: Default::default() };
    let mut context = StepContext::new(OperationId(1), Generation(1), budget, CancelToken::root_now(), clock, &mut sequence, &mut retained);
    let started = Instant::now();
    for _ in 0..iterations {
        context.consume_fuel(1);
        black_box(context.deadline_exceeded());
    }
    let elapsed = started.elapsed().as_nanos();
    let mut checksum = 0;
    for amount in parameters {
        context.consume_fuel(amount);
        checksum += context.now_us().unwrap();
    }
    assert_eq!(context.retained_progress(), Default::default());
    println!("{elapsed} {} {} {} {checksum}", context.original_work_units_spent(), context.original_turn_fuel_remaining(), READS.load(Ordering::Relaxed));
}
