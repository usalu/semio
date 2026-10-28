#[global_allocator]
static HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_s_plugin_energy_engine_bestest as bestest;
use semio_s_plugin_energy_engine_sim as sim;
use semio_s_plugin_energy_engine_site::WeatherData;

struct AnnualCaller {
    caller: semio_framework_job::OriginalJobFixtureCaller,
    decoder: semio_framework_job::OriginalJobDecodeOwner,
}

impl AnnualCaller {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        let stock = semio_framework_job::RetainedCloneGrant { maximum_items: 1 << 30, maximum_copy_bytes: 1_usize.checked_shl(34).unwrap_or(usize::MAX), maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4096 };
        Self { caller: semio_framework_job::OriginalJobFixtureCaller::admit(operation, generation, stock, semio_framework_job::default_now_us, u64::MAX, 1 << 30), decoder: semio_framework_job::OriginalJobDecodeOwner::empty() }
    }

    fn turn<R>(&mut self, grant: semio_framework_job::RetainedCloneGrant, fuel: u64, deadline: u64, run: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> Result<R, semio_framework_job::OriginalTurnFailure>) -> Result<R, semio_framework_job::OriginalTurnFailure> {
        self.caller.turn_with_decoder(grant, &mut self.decoder, |cx| {
            let mut output = None;
            cx.with_original_work_loan(fuel, deadline, |cx| { output = Some(run(cx)?); Ok(()) })?;
            Ok(output.unwrap())
        })
    }

    fn finish(self) {
        assert!(self.decoder.terminal_is_empty());
        self.caller.retire(sim::testing::STOCK);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let case = &args[1];
    let weather: WeatherData = semio_framework_pack_json::from_json_str(&std::fs::read_to_string(&args[2]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let operation = sim::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut original = AnnualCaller::new(operation.operation, operation.generation);
    let mut input = Some(bestest::simulation_job(operation, case, weather, 30).unwrap_or_else(|_| panic!("BESTEST admission refused")));
    let mut output = None;
    for _ in 0..4096 {
        if original.turn(sim::ENERGY_BATCH_GRANT, 64, u64::MAX, |cx| sim::EnergyBatchRun::admit(&mut input, &mut output, cx)).unwrap() { break; }
    }
    let mut batch = output.unwrap();
    let mut terminal = None;
    let mut turn = 0_u64;
    let mut dormant = 0_u32;
    let mut previous_cursor = format!("{:?}", batch.cursor());
    loop {
        let (seen, items) = original.turn(sim::ENERGY_BATCH_GRANT, 32, u64::MAX, |cx| {
            let mut seen = sim::EnergyRun::Yield;
            for _ in 0..32 {
                if cx.should_yield() { break; }
                seen = batch.step(cx)?;
                if matches!(seen, sim::EnergyRun::Complete | sim::EnergyRun::Fault | sim::EnergyRun::Cancelled) { break; }
            }
            Ok((seen, cx.retained_progress().copied_items))
        }).unwrap();
        let cursor = format!("{:?}", batch.cursor());
        dormant = if cursor == previous_cursor { dormant + 1 } else { 0 };
        previous_cursor = cursor;
        if dormant > 100_000 { eprintln!("[DEBUG] No numerical cursor progress in 100000 funded turns, receipt items {items}, remaining {:?}, fuel {}", original.caller.remaining_grant(), original.caller.fuel_remaining()); break; }
        if turn % 100_000 == 0 { eprintln!("[DEBUG] turn {turn}: {:?}", batch.cursor()); }
        if matches!(seen, sim::EnergyRun::Complete | sim::EnergyRun::Fault | sim::EnergyRun::Cancelled) { terminal = Some(seen); break; }
        turn += 1;
    }
    eprintln!("[DEBUG] terminal {terminal:?}: {:?}, remaining {:?}, fuel {}", batch.cursor(), original.caller.remaining_grant(), original.caller.fuel_remaining());
    let mut result = None;
    if terminal == Some(sim::EnergyRun::Complete) {
        for _ in 0..4096 {
            if original.turn(sim::ENERGY_BATCH_GRANT, 64, u64::MAX, |cx| batch.take_results(&mut result, cx)).unwrap() { break; }
        }
    }
    batch.begin_close();
    for _ in 0..10_000_000 {
        if batch.terminal_is_empty() { break; }
        original.turn(sim::ENERGY_BATCH_GRANT, 64, u64::MAX, |cx| cx.with_original_retirement(|cx| batch.close_step(cx))).unwrap();
    }
    assert!(batch.terminal_is_empty());
    drop(batch);
    original.finish();
    let results = result.expect("native numerical run did not complete");
    print!("{}", bestest::results_report_json(case, &results, &args[2], "1d0402144460a26265555a18a9cdfe4f0f7d9b4f57d6194847af7959b518571f"));
}
