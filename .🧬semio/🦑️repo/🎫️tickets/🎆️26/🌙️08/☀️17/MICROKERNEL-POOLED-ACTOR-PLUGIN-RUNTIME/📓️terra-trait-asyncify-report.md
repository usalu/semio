# terra-trait-asyncify (W6) — report

Executor `terra-trait-asyncify`. Owned paths only: `🖥️host/🦀️component.rs`'s `GuestRuntime` trait
region + `WasmtimeRuntime` impl + `MockGuestRuntime` impl, and `🖥️host/🧵️shard/🦀️component.rs`'s
`ShardLoop::pump`/`pump_primed` call sites.

## trait before/after

BEFORE (`🖥️host/🦀️component.rs`, was ~L490-508):
```rust
pub trait GuestRuntime: Send + Sync {
    fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError>;
    fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError>;
    fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault>;
    fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault>;
    fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault>;
    fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault>;
    fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError>;
    fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> Result<(), PluginHostError>;
    fn drop_instance(&self, inst: GuestInstance);
}
```

AFTER (now L506-524):
```rust
pub trait GuestRuntime: Send + Sync {
    fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError>;
    fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError>;
    fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> HostFuture<Result<TurnResult, TurnFault>>;
    fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> HostFuture<Result<(), TurnFault>>;
    fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> HostFuture<Result<JobStep, TurnFault>>;
    fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> HostFuture<Result<(), TurnFault>>;
    fn checkpoint(&self, inst: &mut GuestInstance) -> HostFuture<Result<Vec<u8>, PluginHostError>>;
    fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> HostFuture<Result<(), PluginHostError>>;
    fn drop_instance(&self, inst: GuestInstance);
}
```
`compile`/`instantiate`/`drop_instance` are unchanged (sync — no await point per the mission brief:
`compile` is CPU-bound, `instantiate` only builds a task spec for an async backend, `drop_instance`
is a destructor). The other six methods now return `semio_framework_async::HostFuture<T> =
Pin<Box<dyn Future<Output = T> + Send + 'static>>` (imported via `use semio_framework_async::
HostFuture;`, top of file — the crate already depended on `semio-framework-async`, no `Cargo.toml`
edit needed).

A `pub fn poll_ready<T>(future: HostFuture<T>) -> T` sits right after the trait (L544-553) — the
single poll-once-with-a-noop-waker helper every synchronous caller in the crate (and, per lease-
request, outside it) uses to consume an eagerly-ready `HostFuture` without an executor.

## how each impl returns ready

Both `WasmtimeRuntime` and `MockGuestRuntime` keep their exact prior method body, now wrapped in an
immediately-invoked closure `(|| -> Result<T, E> { ...original body, `?`/`return` unchanged... })()`
whose result is captured into a `let result = ...;`, then returned as `Box::pin(std::future::
ready(result))`. This is a mechanical wrap, not a rewrite — every `?`, early `return Err(...)`
(the `let GuestInstanceState::Wasmtime(state) = &mut inst.state else { return Err(...) }` guard
pattern used throughout), lock-poisoning error mapping, and the final `Ok(...)` construction are
byte-for-byte what they were before this packet. `drop_instance` on both impls is untouched (still
plain sync, matches the trait).

`std::future::ready(result)` never suspends — it resolves to `Poll::Ready` the very first time
anything polls it — and the closure runs to completion (doing all of today's synchronous work,
including every `&mut inst.state` borrow) BEFORE `Box::pin(std::future::ready(...))` is even
constructed, so the returned future captures no borrow of `inst` at all — it owns a plain `Result`
value. This satisfies the mission's "the `&mut GuestInstance` borrow must NOT be captured by the
returned future — compute first, return a ready future second" requirement directly: there is
nothing to capture, the borrow is already released by the time the future value exists.

## call-site conversion + why the expect is sound

`🖥️host/🧵️shard/🦀️component.rs`'s `ShardLoop::pump`/`pump_primed` (the only owned call sites) had
8 raw `self.runtime.<method>(...)` calls across `execute_turn` (1), `start_job` (1), `step_job` (1),
`checkpoint` (1), `restore` (1), `cancel_job` (3, in the `SpawnJob`/`CancelJob` effect-dispatch loop
and the `Payload::Cancel` teardown arm). Every one is now `poll_ready(self.runtime.<method>(...))`,
same `match`/`let _ =` shape as before — no branch, ordering, or error-mapping logic touched.
`ShardLoop` runs on a plain per-shard OS thread (`🏃️executor.rs`'s `ShardExecutor`), not inside a
spawned async task, so it has no executor to `.await` on; `poll_ready` (defined once in
`🖥️host/🦀️component.rs`, imported via `use super::{poll_ready, ...}`) polls the future exactly
once with `Waker::noop()` and panics loudly if that single poll is not `Ready` — it never spins,
retries, or blocks a thread.

The expect/panic is sound here specifically because `ShardLoop`'s `runtime` field is always either
`WasmtimeRuntime` or `MockGuestRuntime` (see `🏃️executor.rs`'s `ShardLoop::new`/`spawn` signature,
`Arc<dyn GuestRuntime>`, both concrete types this packet converted) — both do their work eagerly
before ever constructing the future, so the future IS ready on the first poll by construction, not
by assumption about timing. The panic only fires if some future `GuestRuntime` impl is wired into a
`ShardLoop` without also giving `ShardLoop` a real executor to drive it on — at which point a loud
panic (not a silent hang on a no-op waker nobody re-polls) is exactly the right failure mode, and is
called out explicitly in `poll_ready`'s own doc comment.

Mock-impl tests inside the owned `MockGuestRuntime` region (`mod mock_guest_runtime_tests`) were
converted the same way: `runtime.execute_turn(...).expect(...)` → `poll_ready(runtime.execute_turn(
...)).expect(...)`, same for `checkpoint`/`restore` (5 call sites, all within the owned region,
lines listed below).

## behaviour-preservation argument

- **Fuel/epoch setup**: `WasmtimeRuntime::execute_turn`/`step_job` still call `state.store.
  set_fuel(budget.fuel)` and `state.store.set_epoch_deadline(budget.deadline_ms as u64)` as the
  FIRST two statements inside the closure, in the same order, before touching `wit_budget` or
  calling into the guest — unchanged.
- **Error mapping**: the fuel/epoch trap-message sniffing in `execute_turn` (`lowered.contains(
  "fuel")` → `FuelExhausted`, `"epoch"`/`"interrupt"` → `DeadlineExceeded`, else `Trapped(message)`)
  is copied verbatim inside the closure; the double-`.map_err` shape on `start_job`/`step_job`
  (outer trap-level `wasmtime::Result`, inner `result<_, plugin-error>`) and the single-`.map_err`
  shape on `cancel_job` (no inner `result<_,_>` per `jobs.wit`) are both unchanged, matching the
  pre-existing doc comment that explains why they differ.
- **Effect conversion**: `wit_effect_to_kernel(effect).map_err(TurnFault::Host)?` in the same
  `for effect in wit_turn_result.effects` loop, same `Vec::with_capacity` sizing.
  `wit_turn_status_to_kernel`/`kernel_event_to_wit` calls are untouched (outside anything this
  packet edited).
  Every element of `TurnResult` (`ui_patches: Vec::new()`, `effects`, `next_wake`, `status`,
  `fuel_used`) is constructed identically, including the pre-existing `ui_patches` TODO comment
  about the unimplemented patch marshaling — this packet did not touch that gap, only the
  wrapping shape around the whole method.
- **MockGuestRuntime**: `observed_events`/`scripts` lock-poisoning error mapping
  (`TurnFault::Host(PluginHostError::LockPoisoned("mock runtime"))`), FIFO `pop_front()` semantics,
  and the `ScriptedOutcome::Turn`/`Job`/`Fault` match arms are all copied verbatim into their
  respective closures.
- **`TurnResult` shape**: no field was added, removed, or reordered on `TurnResult`/`JobStep`/
  `TurnFault` themselves — only the trait's RETURN TYPE wrapper (`Result<T,E>` →
  `HostFuture<Result<T,E>>`) changed; the `T`/`E` payload types are identical.
- **No new suspension is introduced anywhere in this packet.** Every future returned by every impl
  this packet touches resolves on its first poll, always — `poll_ready`'s panic branch (`Poll::
  Pending`) is unreachable for `WasmtimeRuntime`/`MockGuestRuntime` today, by construction, not by
  timing luck.

## line ranges edited per file

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (current line numbers,
after this packet's edits, INCLUDING the post-check import cleanup below):
- Import block: added `use semio_framework_async::HostFuture;` right after `use
  semio_framework_actor::ActorId as RuntimeActorId;` (~L19-21).
- `GuestRuntime` trait + doc comment: L485-523 (was 485-508; the trait's own doc comment grew by
  ~12 lines explaining the `HostFuture`/dyn-compatibility rationale).
- `poll_ready` helper (new): L525-555.
- `MockGuestRuntime` impl (`execute_turn`/`start_job`/`step_job`/`cancel_job`/`checkpoint`/
  `restore`): ~L640-737, `drop_instance` untouched at L739 (region `//#region 🔖️MockGuestRuntime`,
  L557-827).
- `mock_guest_runtime_tests` (5 call sites wrapped in `poll_ready`): ~L759-796, inside the same
  owned region (module starts L745, region ends L827).
- `WasmtimeRuntime` impl (`execute_turn`/`start_job`/`step_job`/`cancel_job`/`checkpoint`/
  `restore`): ~L961-1085 (region `//#region 🐎️WasmtimeRuntime`, L829-1400; `impl GuestRuntime for
  WasmtimeRuntime` starts L939). `drop_instance` (L1098) untouched.
- Post-check fix: removed a stray `use std::future::Future;` inside `poll_ready` (the one cheap
  `cargo check` flagged it `unused_imports` — this edition's prelude already has `Future` in scope,
  confirmed by the compiler, not by me guessing). Replaced with a doc comment explaining why. This
  is a no-op import removal, not re-verified by a second `cargo check` (rule 4 permits only one).

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`:
- Import: `use super::{poll_ready, GuestInstance, GuestRuntime, JobBudget, JobStep,
  PluginHostError, TurnFault};` (L26, added `poll_ready` to the existing `use super::{...}`).
- 8 call sites wrapped in `poll_ready(...)`: L315 (`execute_turn`), L329 (`start_job`), L341
  (`cancel_job`, `SpawnJob` effect loop), L386 (`step_job`), L468 (`checkpoint`), L484 (`restore`),
  L509 (`cancel_job`, `Payload::Cancel` teardown). No other line in this file changed — `pump`/
  `pump_primed`'s control flow, `consume_frame`/`dispatch_envelope`, and everything below
  `send_outcome` are untouched.

## commands

**One cheap check permitted by rule 4, run to completion**:
```
CARGO_TARGET_DIR=".../🎯️target-trait-asyncify" cargo check -p semio-framework-plugin-host --lib
```
Exit: `error: could not compile `semio-framework-plugin-host` (lib) due to 2 previous errors; 1
warning emitted` (harness-reported exit code `0` is the wrapper/backgrounding shell's, not
`cargo`'s — cargo itself reported the 2 errors below, i.e. this check FAILED, per rule 10's own
warning about not trusting a piped exit code). Both errors are `E0599: no method named map_err
found for ... Pin<Box<dyn Future<Output = Result<..., TurnFault>> + Send>>`, at
`🖥️host/🦀️component.rs:1436:65` and `:1438:79` — the EXACT two `self.runtime.start_job(...)`/
`self.runtime.step_job(...)` lines inside `PluginInstanceHandle::run_job_to_completion` named in
the `terra-trait-asyncify-lease-post-turn-relay.md` lease-request, i.e. the check fails ONLY on the
known out-of-scope region, confirming (not merely arguing) that every file/region this packet
itself owns and edited — the trait, both impls, `poll_ready`, and `ShardLoop::pump`/`pump_primed`'s
8 call sites — compiles clean with zero errors and zero warnings attributable to my own code (the
one warning in the raw output, `unused_imports` on my own stray `use std::future::Future;` inside
`poll_ready`, was fixed afterward — see `## line ranges edited per file`'s last bullet — but that
fix itself was not re-verified by a second `cargo check`, since rule 4 permits only one). The two
error blocks above are pasted verbatim from the command's own output, not paraphrased.

**Every other acceptance command is UNRUN, per rule 4 (coordinator owns every build)**:
```
cargo check -p semio-framework-plugin-host --all-targets      # UNRUN
cargo test  -p semio-framework-plugin-host --lib -- --skip schema_parity   # UNRUN
cargo check -p semio-framework-os-renderer-wgpu --lib          # UNRUN
bench plugins --renderer native --count 50 --extensions 50 --shards 4   # UNRUN
```

## lease-requests

Two files, both mechanical (a two-line `poll_ready(...)` wrap each), because the trait signature
change breaks every call site in the tree, and two real call sites sit outside every path this
packet owns:

1. `📓️terra-trait-asyncify-lease-post-turn-relay.md` — `🖥️host/🦀️component.rs`'s own
   `//#region 🔀️PostTurnRelay` (`PluginInstanceHandle::run_job_to_completion`, ~L1428-1439), SAME
   FILE but a region I do not own. Blocks even the narrow `-p semio-framework-plugin-host --lib`
   check from succeeding until applied.
2. `📓️terra-trait-asyncify-lease-mcp-workspace.md` — `💻️os/🔨️modules/🌉️mcp/🏠️workspace/
   🦀️component.rs`, a different product module entirely, two real `execute_turn` call sites
   (`activate_plugin_instance` ~L327, `PluginArtifactChannel::exchange_one_real` ~L399).

To make both trivial to apply, `poll_ready` was made `pub` (not `pub(crate)`) and is re-exported at
the plugin-host crate root via the existing `glue.rs`'s `pub use component::*;` — the mcp lease-
request just imports `semio_framework_plugin_host::poll_ready` rather than needing a second copy of
the helper.

## scratch

`🎯️target-trait-asyncify/` (this ticket dir, ~657MB after the one check) holds the dedicated build
cache for the cheap check above, per `📌️important.md`'s "give each concurrently-building packet its
own `🎯️target-<packet>` dir" rule. Left in place (not deleted) in case the coordinator wants to
re-check after applying the two lease-requests without a cold rebuild; disk was at 200GB free when
this was written, well outside the pressure `📌️important.md` describes for 2026-08-17.

## honest gaps

- **The crate as a whole does not yet compile** — confirmed, not assumed: `cargo check` on the full
  `--lib` target fails with exactly the 2 errors on the 2 lines named in the `PostTurnRelay`
  lease-request, nothing else. My own owned regions (trait, both impls, `poll_ready`, `ShardLoop`'s
  8 call sites) are confirmed error-free by that same run — this is not a claim I could not verify;
  it is what the pasted compiler output shows. `semio-framework-os-mcp` (the other lease-request)
  was not itself checked — it depends on this crate, and this crate does not build yet, so a check
  of `os-mcp` would only have re-reported "semio-framework-plugin-host failed to build", telling me
  nothing new about the mcp file's own two call sites; those were verified by reading the file
  directly (see the `terra-trait-asyncify-lease-mcp-workspace.md` lease-request itself), not by a
  compiler run.
- **`WasmtimeAsyncRuntime`** (the actually-suspending backend this trait shape exists to enable) is
  explicitly a sibling packet's job, not built here — this packet only proves the trait CAN carry
  such a backend without breaking `dyn GuestRuntime`.
- **Two lease-requests are unmerged** (see above) — until sol (or whoever owns those paths) applies
  them, `semio-framework-plugin-host` and `semio-framework-os-mcp` do not compile. Both patches are
  two lines each and fully specified in the lease-request files.
- I did not search beyond `--include="*.rs"` grep across the whole repo tree (excluding
  `🎯️target*`) for `GuestRuntime`/`.execute_turn(`/`.start_job(`/`.step_job(`/`.cancel_job(`/
  `.checkpoint(`/`.restore(` call sites — every match that was live code (not a doc comment) is
  accounted for above (`🖥️host` crate itself, `🌉️mcp/🏠️workspace`); everything else that matched
  (`🏃️run/🦀️component.rs`, `🎭️actor`, `services`, `ProgramBridge`, `process-transport`) was doc-
  comment-only or an `unreachable!()`-gated stub with no live call, confirmed by reading each hit in
  context, not just by grep count.

## re-verification pass (fresh terra-trait-asyncify session, later same day)

Resumed under the same executor id, same binding rules (this session's rule 4 is even stricter:
"DO NOT RUN CARGO AT ALL" — no cheap-check exception). Re-read every owned file from disk (not
trusting this report's own prior claims) and confirmed, by direct inspection, that everything
above is still true on disk with no drift:

- `GuestRuntime` trait: `🖥️host/🦀️component.rs` L506-523, signatures byte-identical to the "AFTER"
  block above. `use semio_framework_async::HostFuture;` at L23; `semio-framework-async` already a
  workspace dependency in this crate's `Cargo.toml` (`📦️packages/🦀️rust/Cargo.toml:37,59`) — no
  edit needed, confirmed by reading the file, not assumed.
- `poll_ready` at L544-553, doc comment and no-op-waker/panic body unchanged.
- `MockGuestRuntime`/`WasmtimeRuntime` `impl GuestRuntime`: every method body re-read in full —
  each is still the exact eager-closure-then-`Box::pin(std::future::ready(result))` shape described
  above, `drop_instance` on both still plain sync.
- `ShardLoop::pump`/`pump_primed` (`🧵️shard/🦀️component.rs`): all 8 call sites still wrapped in
  `poll_ready(...)`, `use super::{poll_ready, ...}` present (L26), and the pump-invariant docstring
  ("this loop has no executor of its own ... Sound because every impl `ShardLoop` is ever handed is
  eagerly-ready by construction — see `poll_ready`'s own doc comment") still sits directly above the
  `execute_turn` call site (L309-313).

**Both lease-requests are now APPLIED** (this was an open gap in the original report; it is closed):
- `terra-trait-asyncify-lease-post-turn-relay.md`: `PluginInstanceHandle::run_job_to_completion`
  (`🖥️host/🦀️component.rs` L1436, L1438) now reads `poll_ready(self.runtime.start_job(...))...`
  and `poll_ready(self.runtime.step_job(...))...` — matches the lease's proposed fix exactly.
- `terra-trait-asyncify-lease-mcp-workspace.md`: `🌉️mcp/🏠️workspace/🦀️component.rs` now imports
  `semio_framework_plugin_host::poll_ready` (referenced inline at call sites rather than via a
  top-level `use`) and both `activate_plugin_instance` (L327) and
  `PluginArtifactChannel::exchange_one_real` (L399) wrap their `execute_turn` calls in
  `semio_framework_plugin_host::poll_ready(...)`.

**Fresh whole-repo grep** (`grep -rn --include="*.rs" -E
"\.execute_turn\(|\.start_job\(|\.step_job\(|\.cancel_job\(|\.checkpoint\(|\.restore\(" .`,
excluding `🎯️target*`) for every trait method name, re-run this session (not reused from before):
every hit is either already `poll_ready`-wrapped (the two lease sites above, `ShardLoop`'s 8 sites,
the 5 `mock_guest_runtime_tests` sites) or is unrelated — a same-named method on a completely
different type (`🧮️math/🎯️sampling` RNG `restore`/`checkpoint`, `🖥️server/🗄️storage`
`checkpoint`, `🛢️db` version-graph/engine `checkpoint`, `💻️os/🖥️host` `rollback.restore`,
`🎞️animate` scene/geometry `restore`) or a doc-comment-only mention with no live call
(`🏃️run/🦀️component.rs` L1713, L1767 — confirmed by reading the surrounding `///`/`//` lines,
both inside doc comments describing a not-yet-implemented "real body"). No unwrapped live
`GuestRuntime` call site exists anywhere in the tree.

**Conclusion**: the type-level move this packet's mission describes is complete and, as far as can
be confirmed without running `cargo` (forbidden this session by rule 4, no exception this time),
internally consistent across every owned and leased call site. Whether it actually compiles and
passes the gate (`cargo test -p semio-framework-plugin-host --lib -- --skip schema_parity`, native
scale bench 7/8 budget-3 `perShardCounts {25,25,25,25}`) is for the coordinator to run — still
UNRUN by me, no exception.

## commands (this session)

All UNRUN, per this session's rule 4 (stricter than the prior pass — no cheap-check exception):
```
cargo check -p semio-framework-plugin-host --lib                          # UNRUN
cargo check -p semio-framework-plugin-host --all-targets                  # UNRUN
cargo test  -p semio-framework-plugin-host --lib -- --skip schema_parity  # UNRUN
cargo check -p semio-framework-os-renderer-wgpu --lib                     # UNRUN (peer-owned build in flight per briefing — not touched)
bench plugins --renderer native --count 50 --extensions 50 --shards 4     # UNRUN
```
Only tool used this session: `grep`/`sed`/`find` (read-only, `timeout: 600000` available but not
needed — every command returned well under a second).

## lease-requests (this session)

None new. The two existing lease-requests are confirmed applied (see above) — no action needed.

## honest gaps (this session)

- Did not run any cargo command (forbidden this session, stricter than the prior pass). All
  confirmation above is by direct source reading, not compiler output — the prior pass's `cargo
  check` run (before both leases landed) is the only compiler evidence on record for this packet's
  own owned regions, and it predates this session's re-read.
- Did not re-verify the peer-owned `semio-framework-os-renderer-wgpu`/`semio-framework-os-services`
  builds mentioned as in-flight in this session's briefing — not in scope, not touched, no owned
  file overlaps them.
