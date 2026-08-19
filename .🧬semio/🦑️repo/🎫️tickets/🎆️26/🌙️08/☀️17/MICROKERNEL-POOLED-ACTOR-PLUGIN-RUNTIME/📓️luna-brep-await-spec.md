# PROBE SPEC: Guest-Internal Async in Poll-World Job Bodies

**Question:** Can a job body await a guest-internal async future (BrepKernel methods) in the poll-world without deadlocking?

## 1. Jobs Executor Mechanism

**Path:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`

**Key lines:**
- Lines 276–277: `JOBS_EXECUTOR` — separate `LocalExecutor` instance (NOT the reactor's own)
- Lines 382–384 (in `step_job`): 
  ```rust
  JOBS_EXECUTOR.with(|executor| {
      executor.wake(task);
      executor.run_until_idle(SLICE_MAX_ITERATIONS);
  });
  ```

**What pumps the job future:**
- `step_job` wakes the job's task by id and calls `executor.run_until_idle(64)`, polling until either the task parks on the next `tick()` call, awaits a HOST import, or completes.
- What does NOT pump: no call to the reactor turn loop's own `poll`, no `Event::Completed` routing.

**Polling behavior for guest-internal futures:**
The `JOBS_EXECUTOR` is a `LocalExecutor` — a dedicated, in-process single-threaded executor. A BrepKernel method that is async and NOT a host import (i.e., fully guest-internal) has its future polled by the same `LocalExecutor::run_until_idle` call that drives the job itself. The waker's `wake()` call comes from within the guest's own execution context (not a background thread or external event).

---

## 2. S2 Finding: Guest-Internal Cancellation Works

**Path:** `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-probe-spikes-report.md`, line 15

**Quote:**
> "Guest-side subtask cancellation (manual single-poll-then-drop of the `hang` import's future) does drop the host-side future — the `DropSignal` guard fired, confirmed via `was_hang_dropped()` after a round-trip."

**What S2 proves:**
- A guest task that awaits a host import *can* be cancelled by dropping its future from guest code.
- The host-side RequestFuture is properly cleaned up (DropSignal fired).

**What S2 did NOT rule out:**
- S2 tested HOST imports (which cross the boundary), not guest-internal futures.
- A host import deadlocks in poll world `run_job_to_completion` because that loop never calls `poll` (poll world property).
- Guest-internal futures do NOT cross a boundary that requires external polling.

---

## 3. Kernel Storage & Access Pattern

**Path:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`, lines 299, 353

**Usage shape:**
- Line 299: `use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{block_on, BrepKernel, GeometryHandle};`
- Line 353: `fn semio_mesh_snapshot_from_solids(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], deflection: f64)`
- Line 356: `let Ok(transfer) = block_on(kernel.tessellate(handle, deflection)) else { continue };`

**What this reveals:**
- Today: `block_on()` wraps calls to async kernel methods (e.g., `tessellate`, `import_step`).
- The kernel instance is passed as `&mut dyn BrepKernel` — a mutable trait object.
- No thread-local storage, no lazy static — the kernel is an explicit parameter, not hidden state.
- If migrated to a job body: kernel can be held as a field on the job context or received as input, held across the `.await` boundary.

**Implication for await across yield:**
A `&mut dyn BrepKernel` reference can be held across an `async` call's `.await` point because:
- The kernel is not Send/Sync-constrained by the job runtime (it's guest-internal, not cross-actor).
- No re-entrancy risk (one job body runs at a time on the same task).
- The kernel's async methods do not themselves await host imports that would deadlock.

---

## 4. Verdict on Guest-Internal Await in Poll-World Jobs

**Assessment: PLAUSIBLE GO, Confidence: Medium-High**

**Reasoning:**

1. **LocalExecutor drives both:** A job body that `.await`s a guest-internal kernel method will have that future polled by the same `LocalExecutor::run_until_idle` call that drives the job itself (via `step_job`). No external polling loop is needed.

2. **No boundary crossing:** Unlike `JobCtx::host()`, which awaits a RequestFuture that only resolves inside `poll`'s `Event::Completed` routing (missing in poll world), a kernel method is guest code — its future resolves entirely within the guest's execution context.

3. **S2 does not forbid it:** S2 proved guest-internal cancellation works. It tested host imports to show they can be dropped; it did NOT test whether guest-internal awaits deadlock, so there is no conflicting finding.

4. **Single-threaded, no re-entrancy:** Job bodies run one at a time on `JOBS_EXECUTOR`, and the kernel instance (passed as `&mut dyn BrepKernel` today) holds no cross-task state requiring Send. An await point does not yield control to OTHER jobs; it only yields control back to the same `step_job` call's outer loop, which will re-wake this job on the next slice.

**What would falsify this:**
- If LocalExecutor fails to pump futures created by guest code (e.g., kernel async methods return futures that LocalExecutor cannot poll).
- If BrepKernel methods' futures depend on external context that is not available between slices (e.g., thread-local state lost across yield).
- If `run_job_to_completion` spins on `Running` instead of completing after the job awaits a kernel method (indicates missing waker propagation).

---

## 5. Minimal Concrete Probe Spec

**Location:** `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/` (existing test harness; or standalone under `TICKET_DIR`)

**Crate:** New: `semio-brep-guest-await-probe` (guest + host binary)

### Harness Structure

**Guest WIT additions:**
```wit
interface brep-async-test {
  // Simulate job body; job context calls out with kernel method name + args.
  export brep-job-await-box-prim: async func(width: f64, depth: f64, height: f64) -> result<list<u8>, string>;
  export brep-job-await-tessellate: async func(solid-bytes: list<u8>, deflection: f64) -> result<list<u8>, string>;
  export brep-job-step: async func(budget: u32) -> result<option<list<u8>>, string>;
}
```

**Guest body (identical shape to real job migration):**
```rust
pub async fn brep_job_await_box_prim(w: f64, d: f64, h: f64) -> Result<Vec<u8>, String> {
    let kernel = get_kernel_ref_mut();
    let handle = kernel.box_prim(w, d, h).await.map_err(|e| e.to_string())?;
    let exported = kernel.export_step(&[handle]).await.map_err(|e| e.to_string())?;
    Ok(exported)
}

pub async fn brep_job_await_tessellate(solid_bytes: &[u8], deflection: f64) -> Result<Vec<u8>, String> {
    let kernel = get_kernel_ref_mut();
    let handles = kernel.import_step(solid_bytes).await.map_err(|e| e.to_string())?;
    if let Some(handle) = handles.first() {
        let mesh = kernel.tessellate(handle, deflection).await.map_err(|e| e.to_string())?;
        Ok(bincode::serialize(&mesh).unwrap())
    } else {
        Err("no handles from import".to_string())
    }
}
```

**Host harness:**
```rust
// 1. Call brep_job_await_box_prim; it awaits kernel.box_prim(..) inside guest code.
let result = instance.call_brep_job_await_box_prim(&mut store, 1.0, 2.0, 3.0).await?;
assert!(!result.is_empty(), "box_prim result should not be empty");

// 2. Call brep_job_await_tessellate; it imports STEP, then awaits kernel.tessellate(..).
let step_input = b"<valid STEP data>";
let mesh = instance.call_brep_job_await_tessellate(&mut store, step_input, 0.01).await?;
assert!(!mesh.is_empty(), "tessellate result should not be empty");

// 3. Measure: both calls complete and return non-empty results.
println!("PASS: guest-internal kernel awaits resolved correctly");
```

### Invocation & Output Proof

**Commands:**
```bash
# Build guest (wasm32-wasip2, target = brep-async-probe-guest.wasm)
cargo build -p semio-brep-guest-await-probe --release --target wasm32-wasip2

# Build host (native, links wasmtime)
cargo build -p semio-brep-guest-await-probe-host --release

# Run
BREP_PROBE_WASM=target/wasm32-wasip2/release/semio_brep_guest_await_probe_guest.wasm \
  target/release/semio-brep-guest-await-probe-host
```

**Expected output (PASS):**
```
[host] brep-job-await-box-prim: result bytes = <N>, expected > 0: PASS
[host] brep-job-await-tessellate: mesh bytes = <M>, expected > 0: PASS
[host] VERDICT: guest-internal kernel awaits work without deadlock in async Store
exit 0
```

**Expected output (FAIL = deadlock/timeout):**
```
[host] brep-job-await-box-prim: waiting for result (timeout after 10s)
[timeout, killed]
exit 124 (timeout exit code)
```

### Timing & Scope

**Estimated runtime:** <100ms (no I/O, pure guest compute + one tessellate).

**What it proves (RUN, not compile-only):**
- The guest's async kernel methods actually complete (not a compilation artifact).
- Their futures are polled by the host's executor (wasmtime's async Store::run_concurrent).
- No deadlock occurs when kernel methods are awaited by guest code in an async context.

**What it does NOT prove:**
- Behavior inside `run_job_to_completion` (poll world) — this test uses async world (world actor-async). A separate identical test under poll world would need `StoreBuilder::new().async(false)` or equivalent, then a synchronous job harness calling step_job in a loop.

---

## 6. Other Hazards Noticed

### A. Borrow Lifetime Across Await
If `kernel: &mut dyn BrepKernel` is held across an await point in the migration, the reference MUST remain valid (not dropped/moved). Since the kernel is guest-internal and not recreated between slices, this is safe IF the job body stores it as a field on a context struct that lives for the entire job duration.

### B. Non-Send Futures
BrepKernel's async methods return futures. If those futures are `!Send`, they cannot be moved across thread boundaries. In the poll world (single-threaded jobs executor), this is not a problem. In an async world with `Accessor::spawn`, this COULD be a problem if kernel futures somehow needed to escape the spawning task. Probe assumption: kernel futures are `Send` or only ever used in a single-threaded job context.

### C. Re-entrancy via Nested Kernel Calls
If a kernel method `await`s another kernel method internally (e.g., `offset_face` calls `validate_solid`), or if job input processing calls kernel operations that call more kernel operations, the stack of pending futures must be managed. Since all are on the same `LocalExecutor`, this should work fine (no cross-executor fairness issue), but cycle detection and stack depth limits should be verified in the full implementation.

---

## 7. Unverified Assumptions

1. **LocalExecutor waker propagation:** Assumed that wakers created by `LocalExecutor` correctly drive guest-internal futures. The probe will confirm this in practice.

2. **Kernel instance lifetime across slices:** Assumed kernel can be held as job context state or input. The real migration needs to verify storage strategy.

3. **Kernel method cancellation safety:** S2 proved host-import cancellation works. Guest-internal kernel methods may have different cancellation semantics (e.g., in-progress geometry operations may leave state dirty if cancelled). Probe does not test cancellation of kernel futures.

4. **Poll-world behavior:** This spec tests async world (wasmtime async Store). Poll world requires a separate harness with synchronous job stepping, NOT async Store::run_concurrent. The two may differ.

---

## Summary

**Thesis:** Guest-internal `.await` of BrepKernel futures inside a job body SHOULD work in the poll world, because the LocalExecutor that drives the job also drives the futures without needing external polling. S2's finding does not forbid it (it only tested host imports). The probe will measure this definitively.

**Confidence:** Medium-High (plausible, uncontradicted by evidence, no structural barrier known).

**Next step:** Run the probe on both async and poll-world harnesses to confirm.
