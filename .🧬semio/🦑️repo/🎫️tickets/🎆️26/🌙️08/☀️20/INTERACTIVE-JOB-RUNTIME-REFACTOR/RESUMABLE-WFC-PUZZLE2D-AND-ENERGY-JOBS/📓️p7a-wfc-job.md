# P7a Persistent WFC Job

## 2026-08-22 checkpoint boundary single-source repair

Status: **source/static audit-ready; all executable and Wasm gates remain explicitly unrun**.

The WFC checkpoint format now has one production fixed-header contract. Its byte count is computed
from the eight-byte magic and grouped identity, RNG, progress, and count field totals; the encoder's
header is a compiler-checked array with that exact field count. `CheckpointCounts::checked_bytes`
is the sole aggregate sizing path for `CheckpointBuild` admission/reservation and `WfcRestore`
exact-length validation. Domain words, trail entries, decisions, and observations use the same
derived entry sizes for checked capacity arithmetic and pre-append guards. The divergent 168- and
176-byte literals are absent.

The exact-maximum fixture no longer patches serialized offsets. It completes a real one-node job,
sets a typed observed history whose count is derived from the admitted bytes remaining after the
shared base calculation, then drives production `begin_checkpoint`/`checkpoint_one` serialization.
The source regression requires the result to equal `MAX_CHECKPOINT_BYTES`, admits it to
`WfcRestore`, incrementally crosses every restore phase, and obtains the restored job. The maximum
allocation test uses the same typed-state helper; one additional observed entry is rejected before
checkpoint reservation. Raw `MAX_CHECKPOINT_BYTES + 1` restore admission is checked before restore
state construction. Separate regressions cover a valid zero-domain header-only checkpoint and
checked size overflow through the real restore decoder.

Allowed current-tree checks:

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on the WFC job leaf | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Exit 0; 775/775 bounded, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch, zero failures |
| Scoped `git diff --check` on the WFC leaf and P7 records | Exit 0 |
| Checkpoint boundary scan | Shared fixed-header/checked-count paths present; zero standalone 168/176 checkpoint literals |
| WFC forbidden/debug scan | Zero `block_on`, private thread/spawn, `mem::forget`, `ManuallyDrop`, `[DEBUG]`, `dbg!`, `println!`, or `eprintln!` hits |
| Cleanup relay and hot-shard cancellation | Not modified by this isolated repair |
| Cargo/test/build/runtime/Wasm | **Not run by instruction; no executable pass claimed** |

Still explicitly unrun: the exact-maximum serialization/restore, header-only, overflow, maximum
allocation, and maximum-plus-one regressions; all prior WFC/Assembly/bridge and relay/shard tests in
debug or release; allocation/timing watchdogs; public-factory worker-count replay; mounted
freshness/document-close integration; procedural native, strict-warning, and release gates; and
both Wasm targets.

## 2026-08-22 cleanup-pending ownership race repair

Status: **source/static audit-ready; all executable and Wasm gates remain explicitly unrun**.

The cold guest slot now has explicit `Available`, `Leased`, `CleanupPending`, and
`Quarantined` ownership. A mounted start/step lease defaults to an owned cleanup-pending
disposition, so ordinary fault handling and retained-future unwind restore `CleanupPending`
atomically before the semaphore permit can be released. The slot retains the guest and owned typed
detail; it is never transiently `Available` between producer release and cancellation scheduling
or resolution. Mounted routes preflight cleanup/quarantine and also recheck after acquiring the
permit. A route that raced the preflight receives a non-cleaning rejection and cannot schedule a
cancel for an unadmitted job.

Cleanup has a distinct lease path. It may take the guest only from `CleanupPending`, while the
publicly visible slot remains cleanup-pending with no mountable instance for the entire fallible
`cancel-job` future. Successful cancellation alone resolves the lease to `Available`; an error,
panic, or already-consumed admission resolves it to `Quarantined` with owned detail. External
Drop cleanup can mark either an available or currently leased slot cleanup-pending. A normal
start/step completion cannot erase that obligation, while a producer-owned successful foreground
cancellation can resolve it. Scheduling and guest admission remain separate exactly-once atomic
gates, and no cancellation result is retried or discarded.

Three deterministic mounted-route regressions pause after the failed producer has restored the
slot and released its permit but before completion reaches the receiver. They cover ordinary
start failure followed by cleanup success, ordinary step failure followed by cancel error, and
retained start panic followed by cleanup success. Each launches a second mounted inference while
the barrier is held and asserts `CleanupPending`, prompt rejection, unchanged start/step counts,
zero guest reuse, and zero premature cancel admission. Releasing the barrier proves deterministic
success-to-`Available` or failure-to-`Quarantined`, exactly one cancel admission, stable later
reuse/rejection, and original producer fault delivery.

Allowed current-tree checks:

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on host relay, shard, inference bridge, and WFC leaves | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Exit 0; 775/775 bounded, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch, zero failures |
| Scoped `git diff --check` on the host and shard leaves | Exit 0 |
| Production relay forbidden/debug scans | Zero `block_on`, private pool/thread, batch driver, `mem::forget`, `ManuallyDrop`, `[DEBUG]`, `dbg!`, `println!`, or `eprintln!` hits |
| Fallible cancel-result scan | Zero discarded `GuestRuntime::cancel_job` or `cancel_guest_job_once` results in relay/shard production source |
| Hot-shard and WFC/bridge preservation scan | Stable `BTreeSet` cancellation order/retirement remains; checked exact reservations, per-append bounds, fixed two-item lossless storage, and checked aggregate bytes remain present |
| Cargo/test/build/runtime/Wasm | **Not run by instruction; no executable pass claimed** |

Still explicitly unrun: the three new deterministic cleanup-race regressions and every prior
relay/shard/Assembly/WFC/bridge regression in debug or release; allocation/timing watchdogs; public
factory worker-count replay; mounted freshness/document-close integration; procedural native,
strict-warning, and release gates; and both Wasm targets.

## 2026-08-22 fallible cancellation quarantine repair

Status: **source/static audit-ready; all executable and Wasm gates remain explicitly unrun**.

The cold relay now preserves the sole admitted `cancel_job` result. Only `Ok(())` publishes
`Cancelled` and clears the cleanup obligation. An ordinary `TurnFault` becomes one terminal fault
for foreground context cancellation; the active lease records the same detail and atomically
restores the instance as `Quarantined` before its semaphore permit is released. Background cleanup
from Drop, start failure, and step failure applies the same lease disposition. It never retries the
consumed admission or exposes uncertain guest state. The quarantine owns its fault bytes, so every
later mounted route returns the stored failure without entering `start-job` or `step-job`.
The generic worker session uses an independent live driver token, while `GuestColdRelayJob` owns
and observes the caller token, preventing generic pre-cancellation from publishing `Cancelled`
before the fallible guest cleanup resolves.

The adjacent hot-shard result audit found two other discarded `GuestRuntime::cancel_job` results.
`Effect::CancelJob` now keeps its running-job, replay-turn, and placement records until cancellation
succeeds. Failure retires the actor instance and publishes a typed `ShardOutcome::Fault`, so the
actor cannot be stepped or reused. Actor-level `Payload::Cancel` cancels jobs in stable order,
always retires the instance, and reports the first cancellation fault instead of publishing the
success-only `Cancelled` outcome.

New source regressions cover ordinary cancel failure during context cancellation on a one-worker
pool, nonterminal Drop cleanup, start-failure cleanup, step-failure cleanup, hot-shard effect
cancellation, and actor teardown. They assert exactly one cancellation admission, one foreground
fault followed by `Yield`, stored quarantine rejection on the next mounted route, no guest re-entry,
permit/worker survival, actor retirement, and no false cancellation success. The tests are authored
only and were not executed.

Allowed current-tree checks:

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on the relay and shard host leaves | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Exit 0; 775/775 production rows bounded, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch, zero failures |
| scoped `git diff --check` on both source leaves and both audit records | Exit 0 |
| scoped ignored-cancel scan | Zero discarded `GuestRuntime::cancel_job` or `cancel_guest_job_once` results |
| production relay forbidden/debug and shard debug scans | Zero `block_on`, private pool/thread, batch driver, `mem::forget`, `ManuallyDrop`, `[DEBUG]`, `dbg!`, `println!`, or `eprintln!` hits |
| WFC/bridge bound-preservation scan | Checked exact checkpoint/commit reservations, per-append guards, fixed two-item lossless storage, and checked byte addition remain present |
| Cargo/test/build/runtime/Wasm | **Not run by instruction; no executable pass claimed** |

Still explicitly unrun: all new ordinary-cancel-failure regressions; prior retained-waker,
panic/poison, semaphore-release, and terminal-race regressions; focused relay, shard, Assembly/WFC,
and inference-bridge tests in debug and release; allocation-pressure/watchdog timing; public-factory
replay at 1/2/4/default worker counts; mounted freshness integration; procedural native dev,
strict `-D warnings`, and release; `wasm32-unknown-unknown`; and `wasm32-wasip2`.

## 2026-08-22 final relay unwind and lossless-capacity repair

Status: **source/static audit-ready; all executable and Wasm gates remain explicitly unrun**.

The final independent relay findings are repaired at their ownership boundaries. Every cold relay
request now acquires the sole `GuestInstance` through `GuestInstanceLease`; the lease holds the
instance across every async poll and poison-tolerantly restores it from `Drop` before the semaphore
permit can be released. `GuestRelayPoolFuture` still catches unwind so the process worker survives,
but now drops the retained future first, then atomically schedules cleanup and publishes a typed
terminal fault. A panic in `start-job`, `step-job`, foreground `cancel-job`, or retained polling can
therefore neither strand the slot at `None` nor leave the receiver silently open. A `cancel-job`
panic explicitly transitions the restored instance to `Quarantined`; later mounted routes receive
that typed quarantine fault promptly and never reuse possibly live guest job state.

Cancellation scheduling and guest admission use distinct atomic gates. Fault, closed-channel,
caller cancellation, nonterminal abandonment, and panic recovery may all request cleanup, but they
schedule at most one cleanup task and admit `cancel-job` at most once. Caller cancellation without
an in-flight request submits a recoverable foreground cancel request and remains nonterminal until
that request returns; a cancel panic is consequently delivered as one fault instead of being hidden
by `terminal_delivered` or `Drop`. Normal `Done`, guest `Failed`, and successful cancellation clear
the cleanup obligation; other terminal faults retain it.

The WFC checkpoint/commit materializers now use checked capacity arithmetic and typed admission,
overflow, and allocation faults. Checkpoint writes check their pre-reserved byte limit before every
append. Commit construction reserves both the serialized envelope and the full assignment side
vector before materialization; every item/byte append is checked against its admitted limits. The
inference bridge's two-item lossless checkpoint/commit FIFO now uses fixed two-slot storage, so it
cannot allocate or grow on publication. Exact maximum and maximum-plus-one source tests cover byte
and item policies.

New public/source regressions inject start, step, foreground-cancel, and background-cleanup-cancel
panics after instance acquisition. They assert available or explicitly quarantined nonmissing
ownership, one terminal fault followed by `Yield`, exactly one guest cancel admission,
survival/progress of a competing one-worker-pool task, semaphore release, successful mounted reuse
after recoverable start/step panic, and prompt mounted rejection after cancel quarantine. An
additional mounted regression poisons the instance mutex and proves the poison-tolerant lease
retains the route. These tests are authored only and were not executed in this lane.

Allowed current-tree checks:

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on the relay, bridge, and WFC leaves | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Exit 0; 775/775 production rows bounded, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch, zero failures |
| scoped `git diff --check` on the repaired leaves and records | Exit 0 |
| production relay forbidden scan | Zero `block_on`, private worker pool/thread, batch driver, `mem::forget`, or `ManuallyDrop` hits |
| repaired relay/bridge/WFC debug scan | Zero `[DEBUG]`, `dbg!`, `println!`, or `eprintln!` hits; the host leaf's two pre-existing actor logging calls are outside the relay region |
| Cargo/test/build/runtime/Wasm | **Not run by instruction; no executable pass claimed** |

Still explicitly unrun: all new panic/poison/capacity regressions; focused relay, Assembly, WFC,
and bridge tests in debug and release; allocation-pressure/watchdog timing; actual public-factory
replay at 1/2/4/default worker counts; mounted freshness integration; procedural native dev,
strict `-D warnings`, and release; `wasm32-unknown-unknown`; and `wasm32-wasip2`.

## 2026-08-22 retained-waker guest relay repair

Status: **source/static re-audit-ready; executable acceptance remains unrun**.

The production host no longer synchronously drives a guest future from
`GuestColdRelayJob::step`. Cold `start-job`, `step-job`, and cancellation now move the
semaphore-arbitrated guest instance into one owned future on the existing process-wide
`WorkerPool`. A retained waker submits one finite poll closure only when the future is notified;
a pending poll releases the worker. The synchronous relay turn either submits at most one future,
tries one completion receiver, or returns `Yield`. It never holds the instance mutex across
suspension and never self-requeues another guest step.

The same relay is the sole dispatch behind `io_run`, `io_sniff`, `infer`,
`mutation_plan`, `migrate`, and `compose`. The 6,000,000-fuel/2-ms guest grant is unchanged.
The guest-side checkpoint/publication bridge and the host router's final live
revision/generation validation are unchanged.

Cancellation and terminal ownership are explicit. Dropping any submitted, nonterminal relay
cancels its owned token and schedules guest cleanup on the same retained-waker pool substrate.
Pending-call cancellation and Drop cleanup share one atomic admission gate, so
`cancel-job` is admitted at most once. `WorkerJobSession` records terminal delivery and refuses
to enter the job or repeat a terminal outcome on a later caller turn.

The adjacent periodic plugin-host timer no longer waits inside a worker closure. It registers its
deadline with `WorkerPool::submit_at`; the due callback performs one tick and registers the next
deadline.

New source tests deliberately suspend a mock guest step on a retained waker while a competing
user-visible job completes on the same one-worker pool. They also assert one start/step admission
across repeated pending polls, one completion outcome, no post-terminal re-entry, the
cancel/completion race, exactly one guest cancellation, and live-token nonterminal Drop cleanup.
These tests are authored but were not executed in this lane.

Allowed current-tree checks:

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on the host and job leaves | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs` | Exit 0; 775/775 production rows bounded, zero batch-only/forbidden/deleted, one factory/registration/dispatch |
| scoped production relay/timer scan | Zero `block_on`, private worker pool, thread spawn/builder, run-to-completion, or async self-requeue hits |
| host-component qualified `block_on` reconciliation | Six remaining hits are inside the pre-existing `#[cfg(test)] mock_guest_runtime_tests` module; zero in the production component path |
| whitespace/debug scan on repaired regions | Zero trailing-whitespace, `[DEBUG]`, `dbg!`, `println!`, or `eprintln!` hits |
| Cargo/test/build/runtime/Wasm | Not run by instruction; no runtime pass claimed |

Still unrun: the three newly authored relay regressions; focused Assembly/WFC/bridge debug and
release tests; procedural native development, strict `-D warnings`, and release gates; actual
public-factory pool replay at 1/2/4/default; mounted freshness integration; and
`wasm32-unknown-unknown` plus `wasm32-wasip2`.

## 2026-08-22 production bridge repair

Status: **source/static re-audit-ready; runtime gates deferred by the serialized Cargo owner**.

The real production path is now discoverable and exact. Procedural assembly declares a
metadata-only routed inference for `s.assembly` / `s.assembly.solve`; it does not manufacture a
synchronous `ArtifactInferenceService`. `PluginBuilder` freezes that route into the installed
plugin roster, `describe_plugin` reads the frozen roster, and the package descriptor therefore
advertises the same route the host registers. The `semio.infer` cold handler selects the exact
ActionBus key and delegates wire decoding to the factory-owned
`s.assembly.inference.request.v1` decoder, including its separately carried restart checkpoint.

`WorkerJobSession` is the shared scheduling boundary. One caller turn submits exactly one
`InteractiveJob::step` closure to the shared `WorkerPool`; it never self-requeues. The guest
`semio.infer` bridge and the host cold relay both use this session. The former host grant of
50,000,000 fuel / 200 ms and its run-to-completion relay are absent; every relay step uses the
user-visible 2 ms grant. The source test makes three explicit session calls and proves the step
counter advances exactly once per call. Assembly replay is authored against real pools configured
with 1, 2, 4, and host-default worker counts.

The owned bridge has explicit policies and envelopes:

- preview: one latest-wins/coalesced item, maximum 1 MiB;
- checkpoint/commit: lossless FIFO, maximum two items and 2 MiB total, with explicit backpressure;
- diagnostics: 32-item / 64-KiB overwrite-oldest ring.

Identity, scheduled turns, previews, and terminal diagnostics use the bridge framing. Checkpoint
bytes pass losslessly to the cold-job checkpoint boundary, while terminal candidates pass the
lossless channel before result encoding. The host owns a mutable live `(revision, generation)`
authority for each active cancellation identity. A model actor may advance that authority while the
worker runs; after the guest returns, the router removes the live value and calls
`validate_commit` immediately before exposing the result. A source regression changes both values
and proves the candidate is stale.

New focused source coverage includes factory-owned wire decode/checkpoint preservation, frozen
roster and package-descriptor advertisement without a synchronous facade, preview coalescing,
lossless item/byte saturation, diagnostic ring bounds, production registration idempotence and
collision non-replacement, mounted `semio.infer` checkpoint/cancel/restore, deterministic pool
replay at 1/2/4/default, and live revision+generation rejection. These tests are authored but are
not claimed executed while Cargo is prohibited.

Current allowed validation:

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on the nine touched Rust leaves | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Exit 0; 775/775 bounded, zero batch-only/forbidden/deleted, one production factory/registration/dispatch, zero failures |
| scoped `git diff --check` on the repaired source and reports | Exit 0 |
| debug-output scan on the changed bridge/builder/Assembly leaves | Zero `[DEBUG]`, `dbg!`, `println!`, or `eprintln!` hits |
| legacy relay scan | No `50_000_000`, `deadline_ms: 200`, `run_to_completion`, or `run_on_worker_async` hit in the production host/inference bridge |
| Cargo/test/build | Not run by instruction; no runtime pass claimed |

Deferred exact gate families: focused Assembly/WFC and bridge tests in debug and release; procedural
native dev, strict `-D warnings`, and release; real WorkerPool timing/replay at 1/2/4/default; and
`wasm32-unknown-unknown` plus `wasm32-wasip2`. Independent review should treat the source/static
repair as ready for re-audit and the runtime matrix as still open.

## 2026-08-22 post-audit source repair

Status: **source audit-ready; runtime acceptance intentionally pending**. The coordinator prohibited
Cargo in this repair lane. Rustfmt, diff hygiene, and static scans below describe the current tree;
all Cargo results later in this report predate this source repair and must not be read as current-tree
passes.

The production operation is now one worker-owned `AssemblyInferenceJob`. Its persistent stages are
`Weights`, `Modules`, `Rules`, `Model`, `Slots`, `Edges`, `Topology`, `Fixed`, `Restore`, `Solve`,
`MapCommit`, `EncodeCommit`, and `Complete`. Every factory call performs only O(1) admission checks
and moves the typed snapshot/checkpoint into the job; no clone or serde occurs before the first
step. The model stage constructs validated weights, compatibility rows, transposed supporters, and
fingerprint one bounded unit at a time. The topology stage constructs canonical outgoing/incoming
CSR storage one admitted arc or node boundary at a time. Neither stage calls the legacy opaque
`ModelBuilder::compile` or `GraphTopologyBuilder::build`. Fixed pins, authoritative slot-to-module
mapping, and canonical commit encoding are cursor-driven.

`AssemblyInferenceJobFactory` owns the exact `semio.infer` / `s.assembly.solve` key with payload
schema `s.assembly.inference.request.v1` and `Migrated` classification. Procedural `plugin()` now
idempotently registers it on `ActionBus::production()`. Source coverage registers twice, rejects a
competing factory without replacement, dispatches through the exact bus lookup, preserves the owned
operation, and checks base-revision/generation commit validation. `AssemblySolve` and
`AssemblyContradiction` retain only an explicit headless `run_to_completion` adapter over this same
complete parent job.

## Interim-audit reconciliation

| Blocking finding | Current source resolution |
| --- | --- |
| Synchronous Assembly compilation and no public route | One parent `InteractiveJob` owns all compile/restore/solve/map/encode stages. The factory is registered on the production action bus. The typed payload is moved, not cloned/serialized, before step one. |
| Preview gaps in long stages | WFC publishes the first preview after the first substantive unit, then every 16 units or 16 ms through initialize, choose, propagation, contradiction, and backtrack. Parent compile/map/encode stages use the same cadence. Preview inspection remains capped at 256 items. |
| Uniform rejection loop | Multiply-high mapping consumes exactly one RNG word; the rejection loop is gone. |
| Monolithic restore constructor | `WfcRestore<T>` owns header/domain/trail/decision/observation decode, verify, and per-pattern cache/heap rebuild stages. It checks cancel/fuel/freshness and publishes restore progress. Interactive restart uses it; `WfcJob::from_checkpoint` is explicitly batch-only. |
| One-shot unbounded allocations | Checkpoint and commit envelopes are admitted at fixed 1 MiB maxima with checked exact reservation; source tests exercise maximum admission, allocation pressure, cancellation in both materializers, and one-byte-over rejection. Incremental domain/allowed/supporter/CSR construction starts empty and appends bounded units. Runtime timing evidence is pending because Cargo was forbidden. |
| Broad warning masking | The WFC module-wide `dead_code`, procedural crate-wide `unused_*`, and crate-wide `async_fn_in_trait` allowances are removed. Production mounts only bitset/error/ids/job/model/topology/weights; advanced leaves are `cfg(test)`. The only procedural `unused_*` allowance is attached to the exact dyn-enum macro invocation. Existing leaf/item allowances remain local. |

## Current source tests added

- maximum admitted factory payload is moved with the same backing allocation, emits preview on unit
  one, and never exceeds the 16-unit cadence;
- exact action-bus registration, idempotence, collision rejection, dispatch, stale generation, and
  commit revision/generation validation;
- restart from a checkpoint after dropping the original parent and clearing all compiled maps;
- cancellation before compile mutation, during restore, during authoritative mapping, and during
  canonical encoding;
- specialized model and topology cursor compilers match legacy canonical fingerprints, tables, CSR
  ordering, and multiplicity;
- uniform sampling advances the RNG by exactly one word;
- every interactive WFC stage is cadence-eligible and continuous preview gaps are at most 16 units;
- maximum 1 MiB restore after empty-cache/process-state construction, with cancellation in Header,
  Domains, Trail, Decisions, Observed, Verify, Rebuild, and Complete;
- cancellation during checkpoint/commit materialization, maximum allocation-pressure watchdog
  assertions, and one-byte-over checkpoint rejection.

## Current source-only verification

| Gate | Current result |
| --- | --- |
| Rust parsing/format | `rustfmt --edition 2021 --check` on the procedural root/glue and all changed Assembly/WFC leaves: exit 0. |
| Diff hygiene | Scoped `git diff --check`: exit 0. |
| Private scheduler scan | Zero `thread`, private pool, Rayon/Crossbeam, Tokio spawn, `block_on`, `TASK_POOL`, mutex, or private channel hits in the parent/job files. |
| Batch-driver scan | Two classified hits: the explicit headless Assembly inference adapter and explicit batch `WfcJob::from_checkpoint`; neither is called by the production factory path. |
| Opaque compiler scan | Zero parent/job `ModelBuilder::compile` or `GraphTopologyBuilder::build` hits. Legacy builders remain test-only comparison references. |
| Debug scan | Zero `[DEBUG]`, `dbg!`, `println!`, or `eprintln!` hits in Assembly, procedural root, and glue. Stale AC-4 differential-test debug printing was removed. |
| Broad allowances | Zero crate-level allowance and zero WFC module-level `dead_code` allowance. One macro-local procedural `unused_*` allowance remains. |
| Cargo/test/runtime | **Not run by instruction; no pass claimed.** |

## Runtime matrix still required

The next serialized Cargo owner must run the newly added focused tests in debug and release,
maximum-admission allocation-pressure timing, the full WFC harness, real WorkerPool replay for
1/2/4/default counts, procedural native dev/strict-warning/release, and both
`wasm32-unknown-unknown` and `wasm32-wasip2`. Independent Terra must review this source before P7
acceptance. No ticket state or JSON was changed.

## Historical pre-audit outcome (superseded source shape)

Assembly inference now compiles the shared `WfcJob<GraphTopology>` and drives it through
`semio_framework_job::run_to_completion` with one fuel unit, a 2 ms step budget, cancellation,
operation/generation identity, and the framework watchdog. It no longer calls `GraphSolver::solve`,
owns a worker pool, spawns a thread, blocks an executor, or contains a private scheduler. The
completed inference consumes the job's incrementally materialized `CommitCandidate.output`; it no
longer performs a second post-terminal domain scan.

Assembly is not registered as a plugin artifact/editor/viewer or activation surface in the current
procedural product. Therefore its synchronous `InferredField::compute` batch boundary is not
UI-reachable today. Before Assembly is mounted, the existing root registration note still requires
the framework's upstream `semio.infer` effect/job route. This packet does not claim that a future UI
may invoke the synchronous batch adapter directly.

The ordinary synchronous solver APIs remain explicit batch APIs. Advanced restart/nogood modes can
still enter `solve_inner`; cancellation/budget terminal `PartialState` cloning also exists only in
that synchronous batch driver. Neither path is used by Assembly inference or any mounted UI. The
multi-start batch helper now runs attempts in stable index order and owns no CPU threads.

## Bounded interactive state machine

`WfcJob<T>` persists the compiled model, topology, optional initial domains, cached domain counts
and entropy sums, revision generations, stale-safe entropy heap, FIFO propagation queue,
compatibility/selection cursors, removal trail, decision/backtrack frames, seeded xoshiro state,
preview deltas, checkpoint cursor, commit cursor, and deterministic counters.

Its stages are `InitializeDomains`, `FindMinimumEntropySlot`, `ChooseCandidate`,
`PropagateCompatibilityEdge`, `DetectContradiction`, `BacktrackTrailEntry`, `CommitSlot`,
`MaterializeCheckpoint`, `MaterializeCommit`, and `Complete`. Each consumed unit performs one
bounded action:

- initialization advances one domain phase/pattern word/fixed restriction;
- entropy selection uses cached counts/sums and one stale heap pop;
- weighted or uniform selection removes one candidate per unit;
- topology access uses `out_arc_bound`/`out_arc_at`, so high-degree nodes do not materialize an arc vector;
- compatibility union advances one source-pattern/bitset-word cursor and restriction removes one pattern;
- contradiction detection is O(1) from cached empty/singleton counts;
- backtracking restores one trail entry;
- preview construction shares one 256-item inspection budget across candidates, changed domains,
  waves, paths, and partial assignment, and reports `domain_count` plus `truncated`;
- checkpoint serialization advances one header/domain word/trail entry/decision/observation cursor
  per unit, then transfers the finished byte vector in O(1);
- terminal commit serialization writes one assignment per unit before returning `Complete`.

Cancellation, deadline/fuel exhaustion, operation id, and generation freshness are checked around
mutation. Checkpoints are lossless custom binary state, emitted after the first observation, every
64 observations, and terminally before commit. Restore validates operation/model/topology identity,
restores RNG/cursors/progress, and preserves preview continuity. The byte decoder is intentionally a
batch restore constructor; it is not executed inside an interactive `step()` or on the current UI
surface.

## Audit reconciliation

| Original audit concern | Resolution and evidence |
| --- | --- |
| UI-reachable synchronous `run_to_completion` | No current UI reachability: Assembly is unmounted. The only hit is the documented headless inference batch boundary. Future mounting remains gated on `semio.infer`. |
| Private thread/pool/executor | Static WFC/inference scan has no `thread::`, `std::thread`, `WorkerPool`, `Mutex`, `block_on`, `spawn_blocking`, `TASK_POOL`, Rayon, Crossbeam, or Tokio spawn hit. Actual scheduling is supplied by the process worker pool. |
| Unbounded domain initialization | Persistent acquire/full-domain/fixed/measure cursors; one bounded unit at a time. |
| Unbounded entropy/domain scan | Cached counts, weight sums, weighted-log sums, singleton and empty counts; stale heap entries are popped singly. |
| Unbounded compatibility/propagation | Persistent topology arc, allowed-union word, and restriction cursors; no per-step neighbor or allowed-set clone. |
| Unbounded preview clone/encode | One shared 256-item budget, bounded deltas, explicit truncation metadata. |
| Unbounded commit | Assignment JSON is materialized one slot per `MaterializeCommit` unit. Production consumers use the returned bytes; the old post-terminal production rescan was removed. |
| Whole-state checkpoint serialization | `CheckpointBuild` persists phase/outer/inner byte cursors and writes one state word/entry per unit. The adversarial fuel=1 test includes an actual `CheckpointReady` and requires p99 <2 ms and max <8 ms. |
| Cancel/budget partial-state clone | Exists only in the explicit synchronous batch solver API, outside `InteractiveJob::step` and outside current Assembly/UI reachability. |
| Progress/checkpoint continuity | Focused tests require contiguous preview sequence, monotonic observations/compatibility/backtracks, checkpoint/resume preview continuity, byte-identical replay, and resumed-result parity. |
| Determinism across worker counts | The real shared process `WorkerPool` test runs 1, 2, 4, and host-default 10 workers and requires byte-identical output. |
| Wasm cooperative risk | WFC-focused browser and WASI checks passed on 2026-08-21. Current production browser validation reached an attributable direct-dependency boundary below; current production WASI was not started. |

## Executed verification

| Gate | Exact observed result |
| --- | --- |
| Focused WFC job tests | 8 passed, 0 failed, 0.17 s. Includes cancel/freshness, monotonic progress, replay, checkpoint continuity, and the 8,192-node fuel=1 checkpoint watchdog. |
| Full focused debug before worker-count addition | 278 passed, 0 failed, 0.28 s. |
| Actual worker-count replay | 1 passed, 0 failed, 1.04 s; counts 1/2/4/default 10 were byte-identical. |
| Full focused release after worker-count addition | 279 passed, 0 failed, 1.03 s. |
| Production native dev | `cargo check --manifest-path <procedural>/Cargo.toml --message-format short`: exit 0, 26.31 s. No WFC/Assembly diagnostic; dependency crates emitted warnings. |
| Production strict warning denial | `cargo rustc --manifest-path <procedural>/Cargo.toml --lib --message-format short -- -D warnings`: exit 0, 57.57 s. The procedural crate emitted no warning under denial; dependency warnings are outside the rustc denial boundary. |
| Production native release | `cargo check --release --manifest-path <procedural>/Cargo.toml --message-format short`: exit 0, 69 s. |
| Production browser Wasm attempt 1 | Exit 101: infrastructure ENOSPC while writing a dependency `.rmeta`; no WFC diagnostic. |
| Production browser Wasm attempt 2 | Exit 101: 14 procedural2d/procedural3d WASM VCS futures lacked `.await`; all 14 were repaired. No WFC diagnostic. |
| Production browser Wasm attempt 3 | Exit 101 in 25.74 s: the 14 errors were gone; exactly two E0433 errors required the direct target-scoped `wasm-bindgen-futures` dependency. The existing identity was added as `0.4.71` and resolves to the already-locked `0.4.76`. Parent direction prohibited another Cargo run, so the final row is source-validated but not compiled. |
| Production WASI | Not started after free space fell to 959 MiB and parent directed no more Cargo. This is an explicit open gate, not a pass. |

The final consumer simplification and direct Wasm dependency row landed after the last native/release
commands. They are rustfmt/static validated but are not represented as Cargo-green in this report.
No Cargo command was run after the explicit stop instruction.

## Static and formatting gates

- `rustfmt --edition 2021` completed for every changed WFC/inference/harness leaf and both repaired
  procedural WASM bridges.
- Scoped `git diff --check` passed.
- `[DEBUG]` scan across WFC, Assembly inference, procedural roots, and both repaired WASM bridges: zero hits.
- Forbidden executor scan has only the documented Assembly headless `run_to_completion` and the
  explicit batch-only `solve_inner` hits; it has zero private executor/thread/pool hits.
- Dependency ratchet was checked without Cargo: the manifest adds only
  `wasm-bindgen-futures = "0.4.71"`, the workspace lock already contains 0.4.76, and the locked
  procedural package dependency list contains `wasm-bindgen-futures`.
- Disk changed externally from 959 MiB to 3.3 GiB free during source-only auditing. No cache or
  target artifact was removed after the preservation instruction.

## Warning allowances

Three new allowances are intentional and bounded by mounting constraints:

- procedural root `unused_doc_comments, unused_qualifications`: the closed dyn-enum registration
  macro cannot attach the contract rustdoc, and explicit fully-qualified registrations prevent
  same-name app/type collisions in the generated dispatch surface;
- private production `wfc_engine` module `dead_code`: production Assembly mounts only the inference
  subset while the focused harness mounts all 41 WFC leaves and executes all 279 tests. Item-level
  allowances for every latent solver feature would duplicate the private mount boundary;
- `WfcJob::operation` `dead_code`: production inference consumes it, while the all-module focused
  mount builds a configuration in which the production inference leaf is absent.

Pre-existing item-level Clippy/dead-code allowances inside legacy advanced WFC modules were not
introduced by this repair.

## Remaining gates

1. Re-run production `wasm32-unknown-unknown` after the direct dependency row.
2. Run production `wasm32-wasip2` only with adequate disk headroom.
3. Re-run native dev/strict/release and the focused suite after the final source-only consumer
   simplification if acceptance requires one immutable-tree matrix.
4. Before Assembly becomes UI-reachable, mount inference through the framework `semio.infer`
   worker effect rather than calling the synchronous batch adapter from UI code.

The ticket remains open as required; no ticket status or git state was modified.
