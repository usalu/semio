# Packet P2a: The `InteractiveJob` Protocol (`semio-framework-job`)

New crate: `🧰️framework/🔨️modules/🧵️job/` (package `semio-framework-job`, lib `semio_framework_job`),
built the same way `⏱️trace` was in Phase 0 (single `🦀️component.rs` domain file + `📦️packages/🦀️rust`
glue). Zero new third-party dependencies — `bun ./📜️script.ts verify dependencies` stayed at 238
before and after. Only workspace-internal deps: `semio-framework-trace`, `semio-framework-async` (plus
`serde` transitively via `semio-framework-async`, already counted).

Integration points touched (per the brief, kept to exactly these two):
- root `Cargo.toml`: added the workspace member and a `semio-framework-job` alias in
  `[workspace.dependencies]`.
- `📋️project.json`/launch.json: `⏱️trace` has a `project.json` with `test`/`test-quick`/`test-long`/
  `test-exhaustive` targets and NO launch.json entry anywhere in `.vscode/launch.json` or
  `🧩️launch.seed.jsonc` (confirmed by grep). `🧵️job`'s `📋️project.json` mirrors that exactly; no
  launch.json entry was added, following the same precedent.

No existing Rust source outside this new crate and the two integration points above was edited.

## 1. Public API

```rust
// —— Identity ——
pub use semio_framework_trace::{allocate_operation_id, Generation, OperationId};
pub struct RevisionId(pub u64);
pub struct Operation { pub operation: OperationId, pub base_revision: RevisionId, pub generation: Generation, pub preview_sequence: u64, pub seed: u64 }
impl Operation { fn new(...) -> Operation; fn next_preview_sequence(&mut self) -> u64; }
pub enum CommitValidation { Accepted, Stale { live_revision: RevisionId, live_generation: Generation } }
pub fn validate_commit(op: &Operation, live_revision: RevisionId, live_generation: Generation) -> CommitValidation;

// —— Budget ——
pub struct StepBudget { pub fuel: u64, pub deadline_ms: u64 }
pub const INTERACTIVE_LANE_WALL_MS: u64 = 4;      pub const INTERACTIVE_LANE_FUEL: u64 = 2_000_000;
pub const USER_VISIBLE_LANE_WALL_MS: u64 = 16;    pub const USER_VISIBLE_LANE_FUEL: u64 = 6_000_000;
pub const BACKGROUND_LANE_WALL_MS: u64 = 50;      pub const BACKGROUND_LANE_FUEL: u64 = 20_000_000;
pub const MAINTENANCE_LANE_WALL_MS: u64 = 200;    pub const MAINTENANCE_LANE_FUEL: u64 = 80_000_000;

// —— The protocol itself ——
pub struct StepContext<'a> { /* private fields, see §2 */ }
impl<'a> StepContext<'a> {
    fn operation(&self) -> OperationId;
    fn generation(&self) -> Generation;
    fn stage(&self) -> &'static str;
    fn now_ms(&self) -> u64;
    fn deadline_ms(&self) -> u64;
    fn deadline_exceeded(&self) -> bool;
    fn fuel_remaining(&self) -> u64;
    fn consume_fuel(&mut self, units: u64);
    fn fuel_exhausted(&self) -> bool;
    fn should_yield(&self) -> bool;              // fuel_exhausted() || deadline_exceeded()
    fn is_cancelled(&self) -> bool;               // single non-blocking poll, see §5
    fn cancel_token(&self) -> CancelToken;
    fn set_stage(&mut self, label: &'static str) -> TraceEvent;
    fn next_preview_sequence(&mut self) -> u64;
}

pub struct Checkpoint { pub state: Vec<u8>, pub applied_progress: u64 }
pub struct CommitCandidate { pub state: Vec<u8>, pub output: Vec<u8> }
pub struct JobFault { pub detail: Vec<u8> }
pub enum StepOutcome { Yield, PreviewReady(Vec<u8>), CheckpointReady(Checkpoint), Complete(CommitCandidate), Cancelled, Fault(JobFault) }
impl StepOutcome { fn is_terminal(&self) -> bool; }

pub trait InteractiveJob: Send {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome;   // SYNCHRONOUS, not async fn
}

pub fn drive_step<J: InteractiveJob + ?Sized>(job: &mut J, site: &'static str, operation: OperationId,
    generation: Generation, stage: InteractiveStage, budget: StepBudget, cancel: CancelToken,
    now_ms: fn() -> u64, preview_sequence: &mut u64) -> StepOutcome;

// —— Structured child jobs ——
pub fn root_cancel_token() -> CancelToken;
pub struct JobScope { /* CancelToken + AtomicU32 live-child count */ }
impl JobScope {
    fn root() -> JobScope;
    fn child_of(parent: &CancelToken) -> JobScope;
    fn cancel_token(&self) -> CancelToken;
    fn is_cancelled(&self) -> bool;
    fn spawn_child(&self) -> ChildJobGuard<'_>;
    fn live_child_count(&self) -> u32;
    fn has_live_children(&self) -> bool;
    fn assert_completable(&self);                 // debug_assert!, called before StepOutcome::Complete
}
pub struct ChildJobGuard<'a> { /* releases live-child slot on Drop */ }

// —— Progress stream ——
pub struct EntityId(pub u64);
pub enum DiagnosticKind { Info, Warning, Stalled, Error }
pub enum ProgressEvent {
    Started { operation, generation, base_revision, at_ms },
    StageChanged { operation, generation, sequence, stage, at_ms },
    CandidateTested { operation, generation, sequence, entity, accepted, quality, at_ms },
    PreviewPatch { operation, generation, sequence, base_revision, stage, completed_units,
                   total_units: Option<u64>, quality, tolerance, affected: Vec<EntityId>, patch: Vec<u8>, at_ms },
    Diagnostic { operation, generation, sequence, kind, detail: Vec<u8>, at_ms },
    Checkpoint { operation, generation, sequence, base_revision, applied_progress, at_ms },
    CommitCandidate { operation, generation, sequence, base_revision, at_ms },
    Completed { operation, generation, sequence, at_ms },
    Cancelled { operation, generation, sequence, at_ms },
    Failed { operation, generation, sequence, kind, detail: Vec<u8>, at_ms },
}
impl ProgressEvent { fn operation(&self) -> OperationId; fn generation(&self) -> Generation; }

pub enum ProgressChannelKind { PointerHover, PreviewGeometry, CommitAndCheckpoint, DiagnosticRing, Telemetry, LargeGeometry }
pub fn channel_policy_for(kind: ProgressChannelKind) -> semio_framework_async::ChannelPolicy;
pub const LARGE_PREVIEW_PATCH_BYTES: usize = 256 * 1024;
pub fn default_channel_kind_for(event: &ProgressEvent) -> ProgressChannelKind;

// —— Batch adapter ——
pub struct BatchDriveConfig { pub site: &'static str, pub stage: InteractiveStage, pub fuel_per_step: u64, pub step_budget_ms: u64 }
pub struct BatchJobParams { pub operation: OperationId, pub generation: Generation, pub cancel: CancelToken, pub config: BatchDriveConfig, pub now_ms: fn() -> u64 }
pub fn run_to_completion<J: InteractiveJob>(job: &mut J, params: &BatchJobParams) -> StepOutcome;
pub fn run_on_worker<J: InteractiveJob + 'static>(pool: &WorkerPool, lane: Lane, job: J, params: BatchJobParams) -> std::sync::mpsc::Receiver<StepOutcome>;

// —— Conformance job ——
pub struct TortureJob { /* seed-driven xorshift64 accumulator */ }
impl TortureJob {
    fn new(seed: u64, total_units: u64, checkpoint_every_units: u64, preview_every_units: u64, parent_cancel: &CancelToken) -> TortureJob;
    fn from_checkpoint(bytes: &[u8], parent_cancel: &CancelToken) -> TortureJob;
    fn completed_units(&self) -> u64;
    fn total_units(&self) -> u64;
}
impl InteractiveJob for TortureJob { fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome; }
```

## 2. Budgeting model

Two-bound per design doc Decision 3: `StepBudget { fuel: u64, deadline_ms: u64 }`, `deadline_ms` is
**absolute** wall-clock (`now_ms() + slice`), never a remaining duration, so no job re-derives
wall-clock subtraction itself. `StepContext` keeps both as private state; `should_yield()` is the one
call a job needs (`fuel_exhausted() || deadline_exceeded()`). Four lane-default constant pairs are
provided verbatim from `🎭️actor::Budget`'s existing `Interactive`/`UserVisible`/`Background`/
`Maintenance` numbers (4ms/2M, 16ms/6M, 50ms/20M, 200ms/80M) so a caller building a `StepBudget`/
`BatchDriveConfig` doesn't have to depend on the actor crate to get them.

`drive_step` wraps every single `step()` call in a `semio_framework_trace::Watchdog::start(site,
operation, generation, stage)` RAII guard — this is how the 8ms hard ceiling
(`INTERACTIVE_STEP_CEILING_US`) is enforced/observed: NOT by the job trusting its own `should_yield()`
check, but by an independent, always-on external timer whose violations land in
`Watchdog::violations()`, queryable by operation id. The exit-gate test
(`torture_job_never_trips_the_watchdog_ceiling`) asserts against that ring, never by eyeballing
elapsed time in the test itself.

`TortureJob::step` checks `should_yield()` every `TORTURE_YIELD_CHECK_INTERVAL` (64) units of cheap
integer work — small enough that worst-case overshoot within one check window is nanoseconds, not
milliseconds.

## 3. Generations and commit validation

`Operation { operation: OperationId, base_revision: RevisionId, generation: Generation,
preview_sequence: u64, seed: u64 }` bundles everything the ticket's governing rule requires an
operation to carry. `OperationId`/`Generation` are **reused directly from `semio_framework_trace`**
(re-exported, not redefined) — see §5's "no duplicate id types" note. `RevisionId` is new (the trace
crate has no notion of document revision).

`validate_commit(op, live_revision, live_generation) -> CommitValidation` is the ONLY gate: `Accepted`
only when BOTH the base revision and the generation still match the live document; otherwise
`Stale { live_revision, live_generation }`, which the caller must explicitly rebase or discard — there
is no path in this crate that applies a `CommitCandidate` without going through this check, and no
silent-apply fallback exists to bypass it.

`StepOutcome::Complete(CommitCandidate)`/`StepOutcome::CheckpointReady(Checkpoint)` both carry opaque
`Vec<u8>` state; `Checkpoint::applied_progress: u64` generalizes Puzzle 3D's `FillBuilder.applied_count`
— "these N units are committed, the rest is tentative and may be replanned/discarded."

## 4. Channel-policy matrix

| `ProgressChannelKind` | `ChannelPolicy` chosen | Reasoning |
|---|---|---|
| `PointerHover` | `LatestWins { max_bytes: 4KiB }` | one slot, always the newest pointer/hover sample |
| `PreviewGeometry` | `Coalesced { key: "operation:entity:stage", max_items: 64, max_bytes: 4MiB }` | dedups repeated previews for the same entity/stage within an operation |
| `CommitAndCheckpoint` | `LosslessBounded { max_items: 256, max_bytes: 16MiB }` | commits/checkpoints are never dropped — backpressure (reject/stall) instead |
| `DiagnosticRing` | `Coalesced { key: "operation:diagnostic_kind", max_items: 128, max_bytes: 512KiB }` | closest existing fit to "bounded ring" — see deviation note below |
| `Telemetry` | `LatestWins { max_bytes: 1KiB }` | lossy, single most-recent sample |
| `LargeGeometry` | `ByteCredit { max_items: 32, max_bytes: 32MiB }` | byte-credit controlled, for oversized preview patches |

`default_channel_kind_for(&ProgressEvent)` routes all ten vocabulary events onto one of the six
categories above; `ProgressEvent::PreviewPatch` specifically splits by payload size
(`LARGE_PREVIEW_PATCH_BYTES = 256KiB`) between `PreviewGeometry` and `LargeGeometry`. Every
`ChannelPolicy` variant bounds both items AND bytes (Phase 1a's requirement, verified — see
`channel_policy_matrix_bounds_every_kind_in_items_and_bytes` test).

**Deviation from the design doc's literal wording**: the ticket brief calls for diagnostics to be "a
bounded ring." `semio_framework_async::ChannelPolicy` has exactly four variants
(`LatestWins`/`Coalesced`/`LosslessBounded`/`ByteCredit`), none of which is a literal fixed-capacity
overwrite-oldest ring. `Coalesced` keyed by `(operation, diagnostic_kind)` is the closest available
shape (bounded, drops rather than stalls) and is what this packet ships. A true ring variant would be a
change to `semio_framework_async::ChannelPolicy` itself, out of scope for this packet's "stay strictly
inside the new module" constraint — flagged here for whichever later phase actually wires a live
diagnostics channel to decide whether the `Coalesced` approximation is good enough or a fifth
`ChannelPolicy` variant is warranted.

## 5. How the trace crate is used

- `drive_step` is the **single place** a returned `StepOutcome` becomes a
  `semio_framework_trace::record_*` call: `PreviewReady → record_preview_published`,
  `CheckpointReady → record_checkpoint`, `Complete → record_committed`,
  `Cancelled → record_cancelled`, `Fault → record_failed`. `Yield` records nothing (not a
  lifecycle event).
- `run_to_completion` calls `record_operation_started` exactly once, before the first `drive_step` —
  never per-step, since `Started` is an operation-lifecycle event, not a step event.
- `StepContext::set_stage` is the job's own escape hatch for `record_stage_changed` — Puzzle 3D's
  brush→fill lane switch is the template use.
- `OperationId`/`Generation` are the trace crate's own types, re-exported rather than redefined —
  there is no translation/mapping layer between "this crate's operation id" and "the trace ring's
  operation id," they are the same type. This directly satisfies design doc Decision 7 ("no new API").
- No second preview/checkpoint/cancellation channel exists in this crate. Preview/cancellation
  latency queries (`preview_latency_us`, `cancellation_latency_us`) already live in the trace crate
  and work unmodified against operations driven through `drive_step`.

## 6. Deviations from the design doc, and why

1. **`StepContext` fields are private with accessor methods**, not the design doc's sketch of public
   `pub fuel: &mut u64` / `pub cancel: CancelToken` fields. Reason: `is_cancelled()` needs to cross the
   sync/async seam (see #2 below) in exactly one place; public fields would push every job author to
   reimplement that seam-crossing themselves.
2. **`poll_ready_now` instead of `semio_framework_async::block_on`** for reading `CancelToken` state
   synchronously. `semio_framework_async::CancelToken`'s ops (`is_cancelled`, `cancel`, `child`, `root`,
   …) are `async fn` even though every one is a pure atomic load/store with no real suspension point —
   the same "88% of `async fn` never suspend" pattern Phase 0's census flagged, just inside a crate this
   packet must not edit. Since `InteractiveJob::step` must be synchronous, `poll_ready_now` polls such a
   future exactly once with a no-op waker and returns its (always-`Ready`) value; on `Pending` it
   panics loudly rather than silently spinning. This is deliberately NOT `block_on`: that function is
   explicitly gated to process entry points and forbidden on interactive-reachable code by its own
   module doc (parking/looping is exactly the run-to-completion shape the whole refactor forbids).
3. **Structured child jobs use `CancelToken`'s parent-chain directly, not a live
   `semio_framework_async::ScopeHandle`/`HostAsyncRuntime`.** The design doc says "build this on the
   async crate's existing scopes and CancelToken parent-chain" — `JobScope` takes the CancelToken half
   literally (cancelling an ancestor transitively cancels every descendant scope's token, verified by
   `job_scope_cascades_cancellation_from_parent`) but does NOT open a live `ScopeHandle` via
   `HostAsyncRuntime::open_scope`, because `InteractiveJob::step` is synchronous and does not spawn
   async tasks — there is no concrete `HostAsyncRuntime` instance to open a scope against in this
   packet's scope, and inventing one just to satisfy the type would mean linking a test-only
   `ManualRuntime` into non-test code. `JobScope::spawn_child`/`assert_completable` implement "cannot
   complete while any child is live" via a plain `AtomicU32` live-count, not a registry. Once a later
   phase wires `InteractiveJob` into the actor bridge (which DOES own a `HostAsyncRuntime`), `JobScope`
   can grow an optional `ScopeHandle` field without breaking this packet's API — flagged for Phase 3+.
4. **Checkpoint/preview/output state is hand-rolled little-endian bytes in `TortureJob`, not
   `pack::encode_record_body`.** The design doc suggested reusing "the pack codec's lightweight record
   body encoding." Investigation found `encode_record_body`/`decode_record_body` live in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack` and are typed against `os_dsl::schema::RecordSpec`/
   `RecordValue` — a schema-typed encode belonging to the OS product, not a generic bytes-in/bytes-out
   codec this framework-tier, product-neutral crate could depend on without a large, wrong layering
   violation (this crate must stay usable by any product, the same discipline `⏱️trace`/`⏳️async`
   already follow). `Checkpoint`/`CommitCandidate`/preview payloads stay opaque `Vec<u8>` at the
   protocol level regardless — this only affects how `TortureJob` itself, as one job implementation,
   happens to encode its own state. A later phase's job that already has a `RecordSpec` for its state
   is free to use `pack::encode_record_body` for its own `Checkpoint::state`; the protocol does not
   care.
5. **`run_to_completion`/`run_on_worker` take a `BatchJobParams` struct rather than five-plus loose
   parameters** — a straightforward clippy `too_many_arguments` fix, not a design-doc deviation, noted
   here only because the design doc's sketch didn't specify these functions' exact shape.

## 7. What Phases 3–8 must do to adopt this

- **Actor bridge** (ticket text's own next deliverable, likely Phase 3): generalize
  `Payload::JobStep`/`Suspend`/`Resume` and `TurnStatus::CheckpointReady` onto `drive_step`/
  `InteractiveJob` — one job step per actor turn. `TurnStatus::CheckpointReady` should carry
  `Checkpoint::applied_progress`, not just a bool (design doc's explicit note under Decision 2).
- **Puzzle 3D `FillBuilder`** (design doc §3's proven template): migrate `precompute_step`/
  `precompute_step_lane` to implement `InteractiveJob` directly — `FillBuilder.applied_count` maps onto
  `Checkpoint::applied_progress`, `fill.rng_state` is the same seeded-RNG-in-state discipline
  `TortureJob` demonstrates, `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS` (12ms) becomes a `StepBudget` value
  (likely close to `USER_VISIBLE_LANE_WALL_MS`).
- **`semio_framework_machine`**: its `PersistedSnapshot`/`step` round-trip is the stable-id persistence
  template but is count-bounded with no yield/preview/fault channel — a later phase should have machine
  steps drive through `InteractiveJob` instead of (or as well as) the current `run_to_completion`-style
  entry point, retiring the separate `Inspector` trait once trace integration covers the same ground.
  **Not touched by this packet** — only read for its design lessons; the machine crate itself was not
  modified.
- **Any new interactive operation** written from Phase 3 onward should implement `InteractiveJob`
  directly rather than a bespoke run-to-completion function, and should be driven through
  `drive_step`/`run_to_completion`/`run_on_worker` rather than a hand-rolled loop, so CLI/headless and
  interactive paths can never diverge (packet item 6's whole point).
- **Structured child jobs**: once a phase owns a live `HostAsyncRuntime`, extend `JobScope` with an
  optional `ScopeHandle` (see deviation #3) rather than inventing a second scope type.
- **Progress stream wiring**: `ProgressEvent`/`channel_policy_for`/`default_channel_kind_for` are ready
  to use, but no live channel/actor-mailbox implementation exists yet in this crate (deliberately —
  packet P2a's scope is the protocol, not a concrete transport). A later phase wires an actual bounded
  channel per `ChannelPolicy` and a UI-facing consumer.
- **Determinism discipline**: any job with checkpointed/persisted state must, like `TortureJob`, seed
  its RNG once at construction (never re-seed per step) and use only stable-order collections
  (`BTreeMap`/`BTreeSet`/`Vec`) for anything that ends up in `Checkpoint::state`/`CommitCandidate`.

## 8. Exit gate — what was actually run

```
cargo check  -p semio-framework-job                                  # clean
cargo clippy -p semio-framework-job --all-targets -- -D warnings     # clean
cargo test   -p semio-framework-job                                  # 16/16 passed (debug)
cargo test   -p semio-framework-job --release                        # 16/16 passed (release)
cargo build  -p semio-framework-job --target wasm32-unknown-unknown  # clean
cargo build  -p semio-framework-job --target wasm32-wasip2           # clean
bun ./📜️script.ts verify dependencies                                 # 238 -> 238, no new deps
bun ./📜️script.ts test   (from the crate's own 📦️packages/🦀️rust dir)  # nextest: 16/16 passed
```

The five `torture_job_*` tests are the conformance suite for the ticket's exit gate:
- `torture_job_never_trips_the_watchdog_ceiling` — asserts `Watchdog::violations()` (filtered to this
  operation) is empty across a full run, not by eyeballing timings.
- `torture_job_previews_continuously` — at least 5 `PreviewReady` outcomes across a 20,000-unit run.
- `torture_job_observes_cancellation_within_8ms_at_p99` — 40 trials, p99 cancellation latency measured
  from `cancel()` to the observed `StepOutcome::Cancelled`, asserted `< 8ms`.
- `torture_job_replays_deterministically_across_worker_counts` — the SAME job (seed/total_units fixed)
  actually run via `WorkerPool`s configured with 1, 2, and 4 workers; all three `CommitCandidate.output`
  byte vectors compared equal.
- `torture_job_checkpoint_restore_resume_matches_uninterrupted_run` — an uninterrupted
  `run_to_completion` output compared byte-for-byte against stepping to a checkpoint,
  `TortureJob::from_checkpoint`, then stepping the resumed job to completion.

One real bug was caught and fixed during this work: `TortureJob`'s original seed handling
(`rng_state: seed | 1`) collapsed adjacent seeds (`42`/`43`) onto the identical RNG state, since `| 1`
only ever touches bit 0 — two DIFFERENT seeds silently produced IDENTICAL output, exactly the kind of
determinism bug this conformance job exists to catch. Fixed by expanding the seed through a splitmix64
mix step before feeding it to xorshift64 (`torture_job_is_deterministic_given_identical_seed_and_inputs`
now asserts both "same seed replays identical" and "different seeds diverge").
