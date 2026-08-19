# terra-brep-probe: Guest-Internal Await of the In-Process BrepKernel

## Verdict: **GO-with-constraints**

The mechanical rewrite `block_on(kernel.op(...))` → `kernel.op(...).await` is **safe for essentially
all 134 sites**, for a reason the spec (📓️luna-brep-await-spec.md) didn't anticipate: **the real
BrepKernel never actually suspends.** Grep confirms **zero `.await` calls anywhere in the entire
`✳️brep/🧬️schema` tree** (1600+ `async fn` matches, 184 kernel-trait methods) — every kernel future
resolves `Ready` on its very first `poll()`. `block_on`/`.await` are therefore behaviorally
identical for every real call site today.

The constraints:
1. **2 of the 134 sites are not BrepKernel calls at all** — `🎞️animate`'s two sites are `wgpu`
   `request_adapter`/`request_device` calls, a genuinely different (possibly real-async,
   possibly-host-bound) risk class. Do not sweep them with the same recipe; they need their own
   S2-style investigation (§Q4, bucket D).
2. **The bridge mechanism itself (`pollster::block_on`) is a confirmed landmine for any FUTURE
   guest-internal future that legitimately suspends** — see Q5. It works today only because
   nothing ever calls it on a truly-`Pending` future. The moment any kernel method starts a real
   internal `.await` (chunking a boolean op, say), `pollster::block_on` will **abort-trap the
   whole wasm32-wasip2 instance** (empirically confirmed below), not deadlock softly. The `.await`
   conversion removes this landmine going forward — one more reason to do the sweep, not just a
   safety check on it.
3. **The executor this all depends on (`⚛️reactor/🧵️executor/🦀️component.rs`) does not compile
   standalone right now** — the same mechanical async conversion broke it too (§"Executor itself
   is broken", below). This must be fixed before any of the 134 sites' surrounding call chains can
   compile, and the fix is NOT "add `.await` everywhere" — one class of function (the raw-waker
   vtable) must be reverted to synchronous.

---

## Q1 — LocalExecutor drives a guest task's multi-`Pending` await across pumps: **PASS**

Probe: `🧫️fixtures/🔌️brepprobe/🌐️native` (`cargo run`, native, exit 0). A guest task awaits a
hand-rolled `Countdown` future that returns genuine `Pending` 5 times (self-waking, modeling a
cooperative multi-step guest computation), pumped via `LocalExecutor::run_until_idle(1)` — capped
at ONE iteration per pump, so completion **requires** multiple pumps, not a single-poll fast path.

```
pump #1: run_until_idle(1) -> pending=true, result=None
pump #2: run_until_idle(1) -> pending=true, result=None
pump #3: run_until_idle(1) -> pending=true, result=None
pump #4: run_until_idle(1) -> pending=true, result=None
pump #5: run_until_idle(1) -> pending=true, result=None
pump #6: run_until_idle(1) -> pending=false, result=Some(6)
final result = Some(6), total polls_seen = Some(6), pumps required = 6
Q1 PASS
```
Full transcript: `terra-brepprobe-native-run.txt`. Process exit code: `0`.

## Q2 — job step (`JobCtx::tick()` slicing) drives a ≥3-step await: **PASS**

Same probe binary, `jobs_harness` module (a line-for-line port of ⚛️reactor/💼️jobs/🦀️component.rs's
`JobState`/`JobTick`/`step_job` algorithm — the literal file couldn't be vendored standalone, it
needs `semio_framework`/`dsl`, out of scope for a self-contained spike crate). A job body does
`ctx.tick().await` then awaits a 2-step guest-internal `Countdown`, three times:

```
step-job #1: Running(progress=Some([1, 3]))
step-job #2: Running(progress=Some([2, 3]))
step-job #3: Done([255])
total step-job calls = 3, progress slices observed = 2
Q2 PASS
```
(Slice 3's `ctx.progress()` call still runs — it's just swallowed by `Done` on the same slice,
matching real `step_job`'s own semantics exactly, lines 401-407: when `outcome` resolves the same
slice, `Done(bytes)` is returned, never `Running(progress)`.) Exit code: `0`.

## Q3 — a never-ready guest-internal future inside a job step does not hang the host: **PASS**

Job body awaits a `Countdown` that stores its waker and is **never** woken by anyone. `step_job`
called repeatedly with the SAME budget (stall-guard-eligible):

```
step-job #1: Running(progress=None) in 1.125µs
step-job #2: Running(progress=None) in 1.166µs
step-job #3: Running(progress=None) in 875ns
step-job #4: Failed(job.stalled after 3 consecutive no-progress static-budget calls) in 1.125µs
total step-job calls before stall-fail = 4, every call < 200ms = true
Q3 PASS
```
Every `step_job` call returns in low single-digit microseconds (`run_until_idle`'s ready-queue
naturally empties since nothing re-queues the parked task — no spin, no block) and the
`STALL_LIMIT`-based stall guard reclaims the job after 3 no-progress calls. **The host is never at
risk of hanging on a stuck guest-internal future** — provided the caller reaches it via
`JobCtx::tick()` slicing, not via `pollster::block_on` (see Q5).

## Q4 — minimal conversion recipe + per-plugin/per-bucket counts

134 `block_on(` call sites confirmed by an independent python3 sweep (not grep/shell globbing,
which breaks on the emoji paths) — counts match the packet exactly: flow 59, cad 45, stdio 15,
process 13, animate 2. All 134 live in only **9 files**, which made per-file (not per-heuristic)
classification tractable — every site was read in context, not guessed.

| Bucket | Sites | Files |
|---|---|---|
| **(A) ArtifactEditor-style handler** — mechanical swap | 65 | cad `✏️editor/⚙️engine/🕹️interaction/🦀️component.rs` (6, already `async fn`); flow `🧩️extensions/📐️brep/🦀️component.rs` `evaluate()` (39); flow `🧩️extensions/🖍️draw/🦀️component.rs` `evaluate()`/`drawing_dict()` (20) |
| **(B) io codec** — mechanical swap | 25 prod + 14 test-only + 1 definition | cad `🚪️io/🦀️component.rs` (15); cad `🚪️io/🗺️geometry-import/🦀️component.rs` (10); stdio `⚙️engine/🦀️component.rs` — 14 are `#[cfg(test)]` unit tests of the kernel itself, 1 is `block_on`'s OWN definition (lines 129-134) |
| **(C) job candidate** — mechanical swap works today, but flagged for future `JobCtx::tick()` chunking given the domain (iterative replay / heavy inference) | 27 | cad `🧬️schema/💡️inferences/🦀️component.rs` (14); process `🧬️schema/💡️inferences/🦀️component.rs` (13, includes `replay_process`'s per-step boolean-op loop — the strongest job candidate in the whole sweep) |
| **(D) NOT a BrepKernel call — different risk class, out of scope for this recipe** | 2 | animate `✏️editor/⚙️engine/🎥️video/🦀️component.rs` lines 696/698 — `wgpu::Instance::request_adapter` / `Adapter::request_device`, real host/GPU-driver calls, needs its own S2-shaped investigation before converting |

**65 + 25 + 14 + 1 + 27 + 2 = 134.**

### Recipe (a): ArtifactEditor-style handler

Before — `📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🕹️interaction/🦀️component.rs:514-520`:
```rust
async fn commit_primitive_box(kernel: &mut dyn BrepKernel, params: &HashMap<String, Value>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let corner_a = params.get("cornerA").and_then(parse_vec3)?;
    ...
    let solid = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on(kernel.box_prim(width, depth, height.max(0.05))).ok()?;
```
After:
```rust
async fn commit_primitive_box(kernel: &mut dyn BrepKernel, params: &HashMap<String, Value>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let corner_a = params.get("cornerA").and_then(parse_vec3)?;
    ...
    let solid = kernel.box_prim(width, depth, height.max(0.05)).await.ok()?;
```
The `use ...engine::block_on` import is deleted once no call in the file needs it. The fn signature
needs no change here — it's already `async fn`. Applies verbatim to all 65 (A) sites.

### Recipe (b): io codec

Before — `📐️cad/…/🚪️io/🦀️component.rs:353-356`:
```rust
async fn semio_mesh_snapshot_from_solids(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], deflection: f64) -> Option<SemioMeshSnapshot> {
    let mut meshes = Vec::new();
    for (index, handle) in solids.iter().enumerate() {
        let Ok(transfer) = block_on(kernel.tessellate(handle, deflection)) else { continue };
```
After:
```rust
async fn semio_mesh_snapshot_from_solids(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], deflection: f64) -> Option<SemioMeshSnapshot> {
    let mut meshes = Vec::new();
    for (index, handle) in solids.iter().enumerate() {
        let Ok(transfer) = kernel.tessellate(handle, deflection).await else { continue };
```
Identical transform. Applies to all 25 production (B) sites and the 14 `#[cfg(test)]` sites in
stdio's own `⚙️engine/🦀️component.rs` (the test fns there are already `#[test] async fn`, matching
the convention already used in ⚛️reactor/💼️jobs/🦀️component.rs's own test module). The 1 definition
site (lines 128-134):
```rust
/// ⏳️ Block the current thread until an async kernel call completes.
pub async fn block_on<F>(future: F) -> F::Output
where F: std::future::Future,
{ pollster::block_on(future) }
```
gets **deleted entirely** once its last caller converts — do not keep it "just in case"; per Q5 it
is a landmine, not a utility worth preserving. `pollster` can then be dropped from
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`'s `[dependencies]` (line 36) — nothing else in
the crate uses it (confirmed: `block_on` at line 129-134 is `pollster`'s only call site in the
entire stdio plugin).

### Recipe (c): should become a job

Before — `🏭️process/…/🧬️schema/💡️inferences/🦀️component.rs:197-233` (`replay_process`, already
`pub async fn`), the per-step loop:
```rust
let mut handle = current?;
for (index, step) in enabled_steps.iter().enumerate().skip(start) {
    let tool = tool_solid_for_measure(session.kernel_mut(), &step.measure)?;
    let next = match step.measure {
        ProcessMeasure::Attach { .. } => semio_s_plugin_stdio::...::block_on(session.kernel_mut().fuse(&handle, &tool)).ok()?,
        _ => semio_s_plugin_stdio::...::block_on(session.kernel_mut().cut(&handle, &tool)).ok()?,
    };
    handle = next;
    session.tables.memo.insert(prefix_signature(stock_signature, &enabled_steps[..=index]), handle.clone());
}
```
After — mechanical `.await` swap works immediately (same as (a)/(b), since the kernel never
suspends today), but the RECOMMENDED shape wraps the whole loop as a `register_job_kind`-registered
`JobFn` body with a per-step yield point, so a long process replay (many manufacturing steps, each
a real boolean op on potentially large geometry) becomes cancellable/progress-reporting instead of
one uninterruptible synchronous call:
```rust
async fn job_replay_process(ctx: JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Result<Vec<u8>, Fault> {
    let (mut session, scene, mut handle, resume_index) = decode_or_resume(input, restored)?;
    for (index, step) in enabled_steps.iter().enumerate().skip(resume_index) {
        ctx.tick().await; // slice boundary — one step per `step-job` call
        let tool = tool_solid_for_measure(session.kernel_mut(), &step.measure)?;
        handle = match step.measure {
            ProcessMeasure::Attach { .. } => session.kernel_mut().fuse(&handle, &tool).await.map_err(kernel_fault)?,
            _ => session.kernel_mut().cut(&handle, &tool).await.map_err(kernel_fault)?,
        };
        ctx.checkpoint(encode_resume(index, &handle));
        ctx.progress(encode_progress(index, enabled_steps.len()));
    }
    Ok(encode_handle(&handle))
}
```
This is a bigger, non-mechanical change (new job kind, checkpoint/resume encoding, a
`register_job_kind` registration call, a caller-side migration from a direct fn call to
`start-job`/`step-job`) — out of scope for a pure `block_on`→`.await` sweep packet. Recommend: sweep
all 27 (C) sites with the SAME mechanical (a)/(b) recipe now (safe, since the kernel never
suspends), and open a separate follow-up ticket for job-wrapping `replay_process` specifically —
it's the one site in the whole 134 with a genuine multi-step loop over kernel ops, not just a
single call.

## Q5 — is `block_on` even legal/definable in the guest post-conversion?

**It compiles today, and it is a confirmed landmine.**

`block_on` (✳️brep/🧬️schema/⚙️engine/🦀️component.rs:128-134) is `pollster::block_on(future)`.
`semio-s-plugin-stdio`'s `Cargo.toml:36` depends on `pollster = "0.4.0"` **unconditionally** — no
`target_arch` gate — so it's compiled into the wasm32-wasip2 guest binary, not just native
test/bench builds.

`pollster::block_on` (vendored source,
`~/.cargo/registry/.../pollster-0.4.0/src/lib.rs`) is a `Mutex`+`Condvar` loop — **not**
`thread::park`/`unpark`. Two micro-tests, `🧫️fixtures/🔌️brepprobe/🕸️wasmpollster`, compiled
`--release --target wasm32-wasip2` and run via `wasmtime run` (v46.0.1):

**Mode `self-driving`** (a future that calls `wake_by_ref()` synchronously before returning
`Pending` — the ONLY shape a real guest-internal-only future can safely have, matching what Q1-Q3
already proved the LocalExecutor path handles correctly):
```
[wasmpollster] RESULT = 42
[wasmpollster] PASS: block_on completed under wasm32-wasip2 for a self-driving future
```
exit code `0`. Confirms `pollster`'s `Mutex`/`Condvar` machinery at least initializes on
wasm32-wasip2 (the `Notified`-before-`wait()` fast path never actually reaches a real park).

**Mode `never-wakes`** (a future that returns `Pending` and drops the waker on the floor — nobody
will ever call it, modeling a genuinely-suspended future):
```
thread 'main' (1) panicked at .../library/std/src/sys/sync/condvar/no_threads.rs:23:9:
condvar wait not supported
Error: failed to run main module ...
Caused by:
    ...
    2: wasm trap: wasm `unreachable` instruction executed
```
returned in **0.01s** with exit code `134` (SIGABRT) — **not a hang, an immediate abort-trap of the
whole wasm instance.** wasm32-wasip2's Rust `std` ships a `no_threads` `Condvar` stub that panics
unconditionally the moment `wait()` is actually reached (this is baked into the target's `std`,
independent of any wasmtime flag). Full transcripts of both runs: `terra-brepprobe-wasm-pollster-run.txt`.

**What this means:** `block_on` "works" in production today for exactly one reason — every real
kernel future is `Ready` on the first `poll()` (Q1's grep finding), so `Condvar::wait()` is dead
code that has never executed. It is not a general-purpose bridge; it is a landmine that will
abort-trap the guest instance the instant any kernel method (or the process-replay loop in Recipe
(c), if a chunked variant is added without going through `JobCtx::tick()`) starts a real internal
suspension. The `.await` conversion, reaching the guest's own `LocalExecutor`/`JOBS_EXECUTOR` (Q1-Q3
PASS, cooperative wake/poll, no OS blocking primitives), is the only mechanism proven safe for a
future that might genuinely suspend. **What replaces `block_on`:** nothing — it is deleted (see
Recipe (b)); its callers become plain `.await` inside functions already reachable from the guest's
poll-driven executor.

## Executor itself is broken (found while building this probe, not part of the original spec)

Attempting to `#[path]`-include the LITERAL production file
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️component.rs` verbatim into
the probe crate failed with **9 compiler errors** — the same mechanical "convert everything to
`async fn`" pass that broke the plugins broke this file too:
- `run_until_idle` calls `self.waker_for(id)` (line 115) and `self.has_pending()` (line 128)
  without `.await`, even though both are now `async fn` — straightforward missing-`.await`, same
  class of bug as the 134 `block_on` sites.
- `raw_waker`, `waker_clone`, `waker_wake`, `waker_wake_by_ref`, `waker_drop` (lines 150-181) were
  ALSO turned into `async fn` — but `core::task::RawWakerVTable::new` requires actual synchronous
  `unsafe fn(*const ()) -> T` function pointers; the std library calls these itself through a raw
  vtable and can never `.await` them. **This is not a missing-`.await` bug — it cannot be fixed by
  adding `.await` anywhere. These 5 functions must be reverted to synchronous `fn`/`unsafe fn`.**

A patched copy with exactly this 3-site fix (and nothing else changed) lives at
`🧫️fixtures/🔌️brepprobe/🌐️native/🦀️src/executor_patched.rs`, each patch marked `// 🩹️ PATCH:` at
the exact line, for whoever fixes the real file. **This matters for the 134-site sweep because the
executor is a load-bearing dependency of every one of those call chains** — none of the 134 sites'
surrounding code can compile until this file compiles, and "add `.await` everywhere" (the mechanical
pass's own strategy) will not fix it; the raw-waker vtable functions need a manual revert.

## Files touched (all within owned paths)

- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️brepprobe/🌐️native/Cargo.toml` (new)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️brepprobe/🌐️native/🦀️src/main.rs` (new) — Q1/Q2/Q3 probe
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️brepprobe/🌐️native/🦀️src/executor_patched.rs` (new) — patched copy of the production `LocalExecutor`, see "Executor itself is broken"
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️brepprobe/🕸️wasmpollster/Cargo.toml` (new)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️brepprobe/🕸️wasmpollster/🦀️src/main.rs` (new) — Q5 wasm32-wasip2 pollster micro-test
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-brepprobe-native-run.txt` (new) — Q1-Q3 full run transcript, exit 0
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-brepprobe-wasm-pollster-run.txt` (new) — Q5 both-mode run transcript
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-brep-probe-report.md` (new, this file)

Neither the root `Cargo.toml` nor any file outside the owned paths above was modified. No
git-modifying commands were run.
