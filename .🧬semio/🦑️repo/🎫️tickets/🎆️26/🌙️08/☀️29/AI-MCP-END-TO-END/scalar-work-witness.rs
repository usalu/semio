use semio_framework_actor::{OriginalGuestExecutionBudget, RetainedTurnInput};
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_interpreter::{CoreInstanceOpening, CoreModule, CoreOriginalInvocationStatus, Value};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

fn clock() -> Option<u64> {
    static BIRTH: OnceLock<Instant> = OnceLock::new();
    u64::try_from(BIRTH.get_or_init(Instant::now).elapsed().as_micros()).ok()?.checked_add(1)
}

fn main() {
    let bytes = std::fs::read(std::env::args_os().nth(1).unwrap()).unwrap();
    let module = Arc::new(CoreModule::parse(&bytes).unwrap());
    let grant = RetainedCloneGrant { maximum_items:10_000, maximum_copy_bytes:2_000_000, maximum_capacity_bytes:2_000_000, maximum_release_bytes:2_000_000, maximum_depth:64 };
    let deadline = clock().unwrap() + 120_000_000;
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(197), Generation(1), grant, clock, deadline, 1_000_000);
    let mut epoch = 1;
    let mut owner = caller.turn(grant, |cx| { cx.bind_original_source_epoch(epoch)?; CoreInstanceOpening::begin(&module,cx) }).unwrap().unwrap();
    while !owner.resources_ready() {
        epoch += 1;
        caller.turn(grant, |cx| { cx.bind_original_source_epoch(epoch)?; owner.advance(&module,cx) }).unwrap();
    }
    epoch += 1;
    assert!(caller.turn(grant, |cx| { cx.bind_original_source_epoch(epoch)?; owner.begin_function_step(&module,0,&[],cx) }).unwrap());
    caller.renew_each_turn(4096, 50_000_000);
    let mut instructions = 0;
    let mut work = 0;
    let mut receipt = RetainedCloneProgress::default();
    let started = Instant::now();
    loop {
        epoch += 1;
        let (call, work_spent) = caller.turn(grant, |cx| {
            cx.bind_original_source_epoch(epoch)?;
            let input = RetainedTurnInput { grant:cx.retained_grant(), ..RetainedTurnInput::borrow_original(cx)? };
            let before = cx.original_work_units_spent();
            let call = owner.advance_scalar(&module, OriginalGuestExecutionBudget {input,fuel:1_000_000,deadline_us:cx.deadline_us()}, cx)?;
            Ok((call, cx.original_work_units_spent() - before))
        }).unwrap();
        instructions += call.execution.spent_fuel.unwrap();
        work += work_spent;
        receipt = receipt.checked_add(caller.receipt()).unwrap();
        match call.result.unwrap() {
            CoreOriginalInvocationStatus::Complete => break,
            CoreOriginalInvocationStatus::Pending if epoch < 10_000 => {},
            _ => panic!("scalar fixture did not settle"),
        }
    }
    let nanos = started.elapsed().as_nanos();
    let Some([Value::I32(result)]) = owner.invocation_values() else { panic!("fixture requires an i32 result") };
    println!("{nanos} {result} {instructions} {work} {} {} {} {}", receipt.copied_items,receipt.copied_bytes,receipt.retained_capacity_bytes,receipt.released_bytes);
    caller.cancel().cancel_now();
    for _ in 0..20_000 {
        if owner.terminal_is_empty() { break; }
        epoch += 1;
        caller.turn(grant, |cx| { cx.bind_original_source_epoch(epoch)?; owner.close_step(&module,cx) }).unwrap();
    }
    assert!(owner.terminal_is_empty());
    caller.retire(grant);
}
