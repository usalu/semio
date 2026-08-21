//! 🧪️ P3b verification crate — NOT a workspace member, never built by CI. `frame_job.rs` (the real
//! file, landed at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/
//! 🎯️targets/🧊️wgpu/🦀️frame_job.rs`) lives inside `semio-framework-os-renderer-wgpu`, which cannot be
//! `cargo check`-ed this session (blocked by a sibling packet's in-progress de-async codemod on
//! `semio-framework-os-infinite`/`semio-s-plugin-stdio` — see `📓️p3b-frame-building.md` §7). This crate
//! is the SAME technique P3a's own `🧪️render-snapshot-verify` used: the risky new logic, copied
//! byte-for-byte (module docstrings trimmed; `crate::app_now_ms`/`crate::renderer_worker_pool`/
//! `crate::sweep_expired_camera_dispatch_deadlines` — all pre-existing, unedited functions the real
//! `frame_job.rs` calls via `crate::` — are reproduced here as local equivalents so this crate has no
//! dependency on the renderer crate itself), depending on the REAL `semio-framework-job`/
//! `semio-framework-async`/`semio-framework-trace` crates via path, actually compiled and actually run.
//! Confirms: (1) the `InteractiveJob` impl, encode/decode round trip, and deadline-scan logic are
//! correct: `cargo test`; (2) the non-blocking poll/resubmit contract holds under a REAL `WorkerPool`
//! and a deliberately stalling job — the ticket's own item 5 ("test proving the UI thread still
//! presents at cadence under a builder stall").

use semio_framework_async::{Lane, WorkerPool};
use semio_framework_job::{root_cancel_token, run_on_worker, BatchDriveConfig, BatchJobParams, CommitCandidate, InteractiveJob, StepContext, StepOutcome, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_MS};
use semio_framework_trace::{Generation, InteractiveStage, OperationId};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};

//#region 🪞️LocalStandIns
/// 🪞️ Reproduces `📦️glue.rs`'s own `sweep_expired_camera_dispatch_deadlines` (unedited by this
/// packet, already independently tested in that file's `camera_dispatch_deadline_tests`) so this
/// crate needs no dependency on the renderer crate itself.
fn sweep_expired_camera_dispatch_deadlines(pending: &mut HashMap<String, f64>, now_ms: f64) -> Vec<String> {
    let expired: Vec<String> = pending.iter().filter(|(_, deadline)| now_ms >= **deadline).map(|(surface_id, _)| surface_id.clone()).collect();
    for surface_id in &expired {
        pending.remove(surface_id);
    }
    expired
}
//#endregion 🪞️LocalStandIns

//#region 📥️FrameBuildInputs
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameBuildInputs {
    pub world3d_camera_dispatch_deadlines_ms: HashMap<String, f64>,
    pub wheel_zoom_deadline_ms: f64,
    pub now_ms: f64,
}
//#endregion 📥️FrameBuildInputs

//#region 📤️FrameDirectives
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameDirectives {
    pub expired_world3d_surfaces: Vec<String>,
    pub wheel_zoom_deadline_cleared: bool,
}
//#endregion 📤️FrameDirectives

//#region 🧩️FrameBuildJob
pub struct FrameBuildJob {
    inputs: FrameBuildInputs,
    /// 🐌️ Test-only: an artificial stall this real `step()` sleeps through before computing, so the
    /// worker-side stress test below can prove the UI-thread poll never waits on it.
    artificial_stall: std::time::Duration,
}

impl FrameBuildJob {
    pub fn new(inputs: FrameBuildInputs) -> Self {
        Self { inputs, artificial_stall: std::time::Duration::ZERO }
    }

    pub fn with_artificial_stall(inputs: FrameBuildInputs, stall: std::time::Duration) -> Self {
        Self { inputs, artificial_stall: stall }
    }

    fn compute(&self) -> FrameDirectives {
        let mut candidates = self.inputs.world3d_camera_dispatch_deadlines_ms.clone();
        let expired = sweep_expired_camera_dispatch_deadlines(&mut candidates, self.inputs.now_ms);
        let wheel_zoom_deadline_cleared = self.inputs.wheel_zoom_deadline_ms > 0.0 && self.inputs.now_ms >= self.inputs.wheel_zoom_deadline_ms;
        FrameDirectives { expired_world3d_surfaces: expired, wheel_zoom_deadline_cleared }
    }
}

impl InteractiveJob for FrameBuildJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if !self.artificial_stall.is_zero() {
            std::thread::sleep(self.artificial_stall);
        }
        let directives = self.compute();
        cx.consume_fuel((self.inputs.world3d_camera_dispatch_deadlines_ms.len() as u64).max(1));
        let output = match serde_json::to_vec(&directives) {
            Ok(bytes) => bytes,
            Err(error) => return StepOutcome::Fault(semio_framework_job::JobFault { detail: error.to_string().into_bytes() }),
        };
        StepOutcome::Complete(CommitCandidate { state: Vec::new(), output })
    }
}

fn decode_directives(bytes: &[u8]) -> FrameDirectives {
    serde_json::from_slice(bytes).unwrap_or_default()
}
//#endregion 🧩️FrameBuildJob

//#region 📮️FrameBuildHandle
pub struct FrameBuildHandle {
    in_flight: Option<Receiver<StepOutcome>>,
    last: FrameDirectives,
    /// 📈️ Test-only: incremented every time a fresh `Complete` result is adopted.
    pub adoptions: u64,
}

fn now_ms_u64() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

fn batch_params(operation: OperationId, generation: Generation) -> BatchJobParams {
    BatchJobParams {
        operation,
        generation,
        cancel: root_cancel_token(),
        config: BatchDriveConfig { site: "p3b_verify_frame_build", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_ms: INTERACTIVE_LANE_WALL_MS },
        now_ms: now_ms_u64,
    }
}

impl FrameBuildHandle {
    pub fn new() -> Self {
        Self { in_flight: None, last: FrameDirectives::default(), adoptions: 0 }
    }

    /// 🔁️ The exact contract `🦀️frame_job.rs::FrameBuildHandle::poll_and_resubmit`'s native path
    /// implements: `try_recv` (never `recv`), adopt on `Complete`, resubmit only if nothing is in
    /// flight, always return immediately.
    pub fn poll_and_resubmit(&mut self, pool: &WorkerPool, job: FrameBuildJob, operation: OperationId, generation: Generation) -> FrameDirectives {
        if let Some(receiver) = &self.in_flight {
            match receiver.try_recv() {
                Ok(StepOutcome::Complete(candidate)) => {
                    self.last = decode_directives(&candidate.output);
                    self.adoptions += 1;
                    self.in_flight = None;
                }
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    self.in_flight = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        if self.in_flight.is_none() {
            let receiver = run_on_worker(pool, Lane::Interactive, job, batch_params(operation, generation));
            self.in_flight = Some(receiver);
        }
        self.last.clone()
    }
}

impl Default for FrameBuildHandle {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 📮️FrameBuildHandle

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs(now_ms: f64) -> FrameBuildInputs {
        FrameBuildInputs { world3d_camera_dispatch_deadlines_ms: HashMap::from([("s1".to_string(), 1_000.0)]), wheel_zoom_deadline_ms: 500.0, now_ms }
    }

    //#region ✅️ComputeLogic
    #[test]
    fn not_yet_expired_deadlines_are_kept() {
        // 🐛 `inputs()` fixes `wheel_zoom_deadline_ms` at 500.0 and the world3d deadline at 1_000.0 —
        // "not yet expired" for BOTH means `now_ms` must be before the earlier of the two (500.0), not
        // just before the world3d one. An earlier draft of this test used `now_ms = 999.0`, which is
        // past the wheel-zoom deadline and made this test wrongly assert `!wheel_zoom_deadline_cleared`
        // — caught by actually running `cargo test`, not by inspection.
        let directives = FrameBuildJob::new(inputs(100.0)).compute();
        assert!(directives.expired_world3d_surfaces.is_empty());
        assert!(!directives.wheel_zoom_deadline_cleared);
    }

    #[test]
    fn expired_deadline_is_reported() {
        let directives = FrameBuildJob::new(inputs(1_000.0)).compute();
        assert_eq!(directives.expired_world3d_surfaces, vec!["s1".to_string()]);
        assert!(directives.wheel_zoom_deadline_cleared);
    }

    #[test]
    fn multiple_surfaces_expire_independently() {
        let mut inputs = inputs(1_000.0);
        inputs.world3d_camera_dispatch_deadlines_ms.insert("s2".to_string(), 5_000.0);
        let mut directives = FrameBuildJob::new(inputs).compute();
        directives.expired_world3d_surfaces.sort();
        assert_eq!(directives.expired_world3d_surfaces, vec!["s1".to_string()]);
    }

    #[test]
    fn directives_round_trip_through_json_encoding() {
        let directives = FrameBuildJob::new(inputs(1_000.0)).compute();
        let bytes = serde_json::to_vec(&directives).unwrap();
        assert_eq!(decode_directives(&bytes), directives);
    }

    #[test]
    fn malformed_bytes_decode_to_a_safe_default_rather_than_panicking() {
        assert_eq!(decode_directives(b"not json"), FrameDirectives::default());
    }
    //#endregion ✅️ComputeLogic

    //#region 🐕️NonBlockingUnderStall
    /// 🐕️ The ticket's own item 5: "add a test proving that under a builder stall the UI thread still
    /// presents at cadence." Submits a job that sleeps 300ms inside `step()` (simulating a stalled
    /// builder) to a REAL `WorkerPool`, then calls `poll_and_resubmit` at a simulated 240Hz (every
    /// ~4ms, faster than any real redraw cadence) for 100ms of wall time WHILE the job is still
    /// sleeping. Every single call must return in well under a frame budget and the result must stay
    /// the stale default throughout (no adoption yet) — proving the "UI thread" side of this test never
    /// blocks on the worker, matching `RenderSnapshotSink::acquire`'s own never-block precedent (P3a).
    #[test]
    fn poll_and_resubmit_never_blocks_while_the_builder_stalls() {
        let pool = WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 2));
        let mut handle = FrameBuildHandle::new();
        let operation = semio_framework_trace::allocate_operation_id();
        let stall = std::time::Duration::from_millis(300);
        let job = FrameBuildJob::with_artificial_stall(inputs(1_000.0), stall);
        let first = handle.poll_and_resubmit(&pool, job, operation, Generation(0));
        assert_eq!(first, FrameDirectives::default(), "nothing has completed yet — must return the stale default, not wait");

        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(100);
        let mut max_call_duration = std::time::Duration::ZERO;
        let mut calls = 0u64;
        while std::time::Instant::now() < deadline {
            let call_started = std::time::Instant::now();
            // 🪑️ A no-op stalled job is reused for every resubmit while one is already in flight — this
            // loop's `FrameBuildJob::new` value is only ever consumed if `poll_and_resubmit` decides to
            // resubmit, which it must not while the first job is still running.
            let placeholder = FrameBuildJob::new(inputs(1_000.0));
            let directives = handle.poll_and_resubmit(&pool, placeholder, operation, Generation(0));
            max_call_duration = max_call_duration.max(call_started.elapsed());
            assert_eq!(directives, FrameDirectives::default(), "the 300ms stall has not elapsed — must keep returning the stale default, never block for it");
            calls += 1;
            std::thread::sleep(std::time::Duration::from_millis(4));
        }
        assert!(calls > 10, "expected many poll ticks within 100ms, got {calls}");
        assert!(max_call_duration < std::time::Duration::from_millis(10), "a single poll_and_resubmit call took {max_call_duration:?} — it must never wait on the worker");
        assert_eq!(handle.adoptions, 0, "the stalled job had not finished during the polling window — nothing should have been adopted yet");

        // 🐌️➡️✅️ Now actually wait for the stalled job to land, and confirm the NEXT poll adopts it.
        std::thread::sleep(stall + std::time::Duration::from_millis(200));
        let placeholder = FrameBuildJob::new(inputs(1_000.0));
        let settled = handle.poll_and_resubmit(&pool, placeholder, operation, Generation(0));
        assert_eq!(settled.expired_world3d_surfaces, vec!["s1".to_string()], "once the stalled job actually completes, its real result must be adopted");
        assert_eq!(handle.adoptions, 1);
    }

    /// 🔂️ A second, independent proof at a smaller scale: 50 back-to-back `poll_and_resubmit` calls
    /// against a job that never even gets submitted-and-forgotten (each call either adopts or leaves
    /// `last` alone) never panics, never deadlocks, and every call individually completes fast — this
    /// is the "presents at cadence" half of item 5, phrased as a throughput bound instead of a stall.
    #[test]
    fn sixty_hertz_polling_cadence_holds_across_many_ticks() {
        let pool = WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 2));
        let mut handle = FrameBuildHandle::new();
        let operation = semio_framework_trace::allocate_operation_id();
        let budget = std::time::Duration::from_millis(2); // the ticket's own ≤2ms present-half budget
        for tick in 0..50u64 {
            let job = FrameBuildJob::new(inputs(1_000.0 + tick as f64));
            let started = std::time::Instant::now();
            let _ = handle.poll_and_resubmit(&pool, job, operation, Generation(tick));
            assert!(started.elapsed() < budget, "tick {tick} took {:?}, over the {:?} present-half budget", started.elapsed(), budget);
            std::thread::sleep(std::time::Duration::from_millis(16)); // ~60Hz
        }
    }
    //#endregion 🐕️NonBlockingUnderStall
}
