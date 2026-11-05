//! 🧰️ Owned original callers drive WFC test jobs and return every payload and source before teardown.

use semio_framework_value_testing_allocator as _;
use semio_framework_job::{Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobOutcomeBorrow, OperationId, OriginalJobReturn, OriginalJobReturnOrigin, OriginalTurnFailure, RetainedCloneGrant, StepContext};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;

/// 🪙️ One finite lifetime stock covers each test's source admission, business turns and physical close.
pub const TEST_SOURCE: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 16_000_000,
    maximum_copy_bytes: 1 << 30,
    maximum_capacity_bytes: 1 << 30,
    maximum_release_bytes: 1 << 30,
    maximum_depth: 256,
};

/// 🦶️ A turn narrows the same source without renewing its cumulative stock.
pub const TEST_TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64,
    maximum_copy_bytes: 1 << 20,
    maximum_capacity_bytes: 1 << 20,
    maximum_release_bytes: 2 << 20,
    maximum_depth: 128,
};

/// 🎟️ Owns one caller and its prepaid receiving place across every step, acknowledgement and close.
pub struct TestJobDriver {
    caller: Option<OriginalJobFixtureCaller>,
    origin: OriginalJobReturnOrigin,
}

impl TestJobDriver {
    /// 🏡️ Admits the declared operation once from the test's finite original source.
    pub fn new(operation: OperationId, generation: Generation) -> Self {
        Self {
            caller: Some(OriginalJobFixtureCaller::admit(operation, generation, TEST_SOURCE, || Some(0), u64::MAX, 100_000_000)),
            origin: OriginalJobReturnOrigin::empty(),
        }
    }

    /// 🪢️ Borrows the same original source for private producer stages without issuing another wallet.
    pub fn turn<R>(&mut self, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> {
        self.caller.as_mut().expect("live original test caller").turn(TEST_TURN, run)
    }

    /// ▶️ Visits the original borrowed result before receiving its descriptor on the same caller.
    pub fn step<J: InteractiveJob, R>(&mut self, job: &mut J, fuel: u64, visit: impl FnOnce(Option<&JobOutcomeBorrow<'_>>) -> R) -> R {
        let caller = self.caller.as_mut().expect("live original test caller");
        for turn in 0..4096 {
            if self.origin.outcome().is_none() { break; }
            let acknowledged = caller.turn(TEST_TURN, |cx| self.origin.acknowledge_outcome(cx)).expect("original outcome acknowledgement");
            if acknowledged { break; }
            assert!(turn < 4095, "original outcome acknowledgement stalled");
        }
        let mut visitor = Some(visit);
        let mut observed = None;
        caller.turn(TEST_TURN, |cx| cx.with_original_work_loan(fuel, cx.original_absolute_deadline_us(), |cx| {
            let mut verdict = None;
            let returned = if self.origin.admit(cx)? {
                semio_framework_job::drive_step(job, cx, self.origin.receiver()?, "wfc.original-test-driver", InteractiveStage::InteractiveStep, &mut verdict)?
            } else { OriginalJobReturn::Preparing };
            match returned {
                OriginalJobReturn::Preparing | OriginalJobReturn::Pending => {
                    observed = Some(visitor.take().expect("one result visitor")(None));
                }
                OriginalJobReturn::Outcome(reply) => {
                    observed = Some(visitor.take().expect("one result visitor")(reply.original()));
                    let mut input = Some(reply.into_descriptor());
                    assert!(self.origin.receive_outcome(cx, &mut input)?);
                    assert!(input.is_none(), "original descriptor returned exactly once");
                }
                OriginalJobReturn::Continuation(value) => {
                    let mut input = Some(value);
                    assert!(self.origin.receive_continuation(cx, &mut input)?);
                    let failure = self.origin.continuation().expect("received continuation").view();
                    panic!("original test callback refused: callback={:?}, qualification={:?}, receipt={:?}", failure.callback, failure.qualification, failure.receipt_qualification);
                }
            }
            Ok(())
        })).expect("original test step");
        observed.expect("one original result observation")
    }

    /// 📭️ Observes a bounded parent answer, then closes all original custody before returning test bytes.
    pub fn completed_output<J: InteractiveJob>(&mut self, job: &mut J, fuel: u64, maximum_turns: usize) -> Result<Vec<u8>, String> {
        let mut terminal = None;
        for _ in 0..maximum_turns {
            terminal = self.step(job, fuel, |outcome| match outcome {
                Some(JobOutcomeBorrow::Complete { output, .. }) => Some(Ok(output.map(observed_bytes).unwrap_or_default())),
                Some(JobOutcomeBorrow::Fault { detail, .. }) => Some(Err(String::from_utf8_lossy(&observed_bytes(detail)).into_owned())),
                Some(JobOutcomeBorrow::Cancelled { .. }) => Some(Err("inference-test-cancelled".into())),
                _ => None,
            });
            if terminal.is_some() { break; }
        }
        self.close(job);
        terminal.unwrap_or_else(|| Err("inference-test-step-bound".into()))
    }

    /// 🛬️ Retires an original source only after every receiving place has returned.
    pub fn retire_source(&mut self) {
        assert!(self.origin.terminal_is_empty(), "original receiving place must close first");
        self.caller.take().expect("original caller returned once").retire(TEST_TURN);
    }

    /// 🧹️ Returns descriptor custody, closes the real job, then retires its once-issued source.
    pub fn close<J: InteractiveJob>(&mut self, job: &mut J) {
        let Some(caller) = self.caller.as_mut() else {
            assert!(job.terminal_is_empty(), "closed test driver cannot own another job");
            return;
        };
        for turn in 0..2_000_000 {
            if self.origin.terminal_is_empty() { break; }
            caller.turn(TEST_TURN, |cx| self.origin.close_step(cx)).expect("original return origin close");
            assert!(turn < 1_999_999, "original return origin close stalled");
        }
        job.begin_close();
        for turn in 0..2_000_000 {
            if job.terminal_is_empty() { break; }
            let step = caller.turn(TEST_TURN, |cx| job.close_step(cx)).expect("same-source test job close");
            assert!(!matches!(step, InteractiveJobCloseStep::Blocked), "same-source test job close blocked");
            assert!(turn < 1_999_999, "same-source test job close stalled");
        }
        assert!(self.origin.terminal_is_empty());
        assert!(job.terminal_is_empty());
        self.retire_source();
    }
}

fn observed_bytes(payload: &semio_framework_job_payload::RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|index| payload.page(index).expect("observed payload page").iter().copied()).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
