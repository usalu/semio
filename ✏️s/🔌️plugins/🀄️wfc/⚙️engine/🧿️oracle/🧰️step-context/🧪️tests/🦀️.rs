//! 🧪️ Portable lifecycle laws exercise real receiving custody and original source retirement.
use super::*;
use semio_framework_job::{InteractiveJobStep, JobOutcomeDescriptor, JobOutcomeView, OriginalJobReturnClaim, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};

struct FixtureJob {
    mode: String,
    steps: usize,
    closing: bool,
    empty: bool,
}

impl InteractiveJob for FixtureJob {
    fn step<'job>(&'job mut self, cx: &mut StepContext<'_>, _: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'job> {
        InteractiveJobStep::from_callback((|| {
            self.steps += 1;
            match self.mode.as_str() {
                "pending" if self.steps == 1 => { cx.consume_fuel(1); Ok(None) }
                "cancel" => JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from),
                "refusal" => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "portable read-only callback refusal").into()),
                _ => JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from),
            }
        })())
    }

    fn borrow_outcome<'job>(&'job self, descriptor: &'job JobOutcomeDescriptor) -> Result<JobOutcomeView<'job>, ValueError> {
        if self.mode == "cancel" { descriptor.cancelled() } else { descriptor.yielded() }
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        assert!(self.closing);
        if self.empty { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        self.empty = true;
        Ok(InteractiveJobCloseStep::Complete { progress })
    }

    fn terminal_is_empty(&self) -> bool { self.empty }
}

#[test]
fn portable_original_driver_returns_outcomes_continuations_and_every_source() {
    let law: serde_json::Value = serde_json::from_str(include_str!("contract.json")).expect("portable driver contract");
    let stock = &law["stock"];
    assert_eq!(TEST_SOURCE.maximum_items, stock["items"].as_u64().unwrap() as usize);
    assert_eq!(TEST_SOURCE.maximum_copy_bytes, stock["copyBytes"].as_u64().unwrap() as usize);
    assert_eq!(TEST_SOURCE.maximum_capacity_bytes, stock["capacityBytes"].as_u64().unwrap() as usize);
    assert_eq!(TEST_SOURCE.maximum_release_bytes, stock["releaseBytes"].as_u64().unwrap() as usize);
    assert_eq!(TEST_SOURCE.maximum_depth, stock["depth"].as_u64().unwrap() as usize);
    for case in law["cases"].as_array().unwrap() {
        let mut job = FixtureJob { mode: case["id"].as_str().unwrap().into(), steps: 0, closing: false, empty: false };
        let mut driver = TestJobDriver::new(semio_framework_job::allocate_operation_id(), Generation(1));
        driver.turn(|cx| {
            assert_eq!(cx.now_us(), law["clockUs"].as_u64());
            assert_eq!(cx.original_absolute_deadline_us(), law["deadlineUs"].as_str().unwrap().parse::<u64>().unwrap());
            Ok(())
        }).unwrap();
        let mut previous_fuel = driver.caller.as_ref().unwrap().fuel_remaining();
        let fuel = case["stepFuel"].as_u64().unwrap();
        let observed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for _ in 0..128 {
                let observed = driver.step(&mut job, fuel, |outcome| match outcome {
                    Some(JobOutcomeBorrow::Yield { .. }) => Some("yield"),
                    Some(JobOutcomeBorrow::Cancelled { .. }) => Some("cancelled"),
                    None => None,
                    _ => panic!("unexpected portable outcome"),
                });
                let remaining = driver.caller.as_ref().unwrap().fuel_remaining();
                assert!(remaining <= previous_fuel, "original lifetime fuel cannot renew across turns");
                previous_fuel = remaining;
                if observed.is_some() { return observed.unwrap(); }
            }
            panic!("portable driver never returned");
        }));
        if case["expectedReturned"] == "continuation" {
            assert!(observed.is_err(), "a real original refusal must remain observable");
            assert!(driver.origin.continuation().is_some());
        } else {
            assert_eq!(observed.unwrap(), case["observed"].as_str().unwrap());
            assert!(driver.origin.outcome().is_some());
        }
        assert_eq!(job.steps, case["businessSteps"].as_u64().unwrap() as usize);
        let caller = driver.caller.as_ref().unwrap();
        assert!(caller.fuel_remaining() < law["fuel"].as_u64().unwrap());
        assert!(caller.cumulative_receipt().fits(TEST_SOURCE));
        driver.close(&mut job);
        assert_eq!(job.terminal_is_empty(), law["close"]["jobEmpty"].as_bool().unwrap());
        assert_eq!(driver.origin.terminal_is_empty(), law["close"]["originEmpty"].as_bool().unwrap());
        assert_eq!(driver.caller.is_none(), law["close"]["callerEmpty"].as_bool().unwrap());
    }
}
