# Phase 2 Current Status and Gap Audit — 2026-08-23

## Scope and evidence rules

This is a read-only status audit of Phase 2's three planned packets. It reads the governing plan,
the two retained Phase 2 implementation reports, current job/actor/plugin-shard/renderer source, the
existing static verifier, and the earlier readiness audits. It does not accept report prose as proof
of the current shared tree. No Cargo, Nx, Wasm, browser, runtime, or network command was run. No
production, test, verifier, manifest, or ticket metadata source was edited; this report is the sole
output.

The plan names three packets:

- **P2a:** universal resumable job protocol, identity, cancellation, checkpoint/candidate, structured
  children, watchdog, and batch adapter;
- **P2b:** progress types and delivery policy plus a generation-tagged preview overlay distinct from
  committed scenes and revision/generation commit validation;
- **P2c:** one job step per actor turn, deterministic replay log and executable harness, and the
  synthetic torture job gate.

Only `📓️p2a-job-protocol.md` and `📓️p2b-actor-job-bridge.md` exist in the Phase 2 folder. There is no
separate P2c report or current independent acceptance report. The historical status page says the
phase gate was met, while the later `📓️terra-p1-p2-readiness-audit-2026-08-23.md` correctly reopens
the phase for missing preview/replay work. Its old shard-executor `block_on` finding is now stale, but
its preview/replay verdict is still supported by current source.

## Verdict

| Packet | Current verdict | Source mechanisms present | Acceptance blocker |
|---|---|---|---|
| P2a | **RED** | `InteractiveJob`, `StepContext`, fuel/deadline, cancellation, watchdog, operation/revision/generation validation, checkpoints, candidates, worker session, batch adapters, and `TortureJob` exist. | The protocol owns cloneable dynamic byte vectors without item/byte/output credits; child completion is debug-only; live batch/self-requeue consumers can run to terminal or lose terminal ownership; current runtime/timing gates were not run. |
| P2b | **RED** | `ProgressEvent`, the six-way policy selector, preview sequence vocabulary, and `validate_commit` exist. | There are zero external `ProgressEvent` publishers, no preview overlay, no live fixed progress owner, and the actor `SceneStore` is committed/pending state only. The main WGPU host does not mount that store. |
| P2c | **RED** | Actor job wire records, pack codecs, `JobTurnBridge`, one-step bridge tests, a serializable `JobReplayLog`, shard `JobTurn` validation, and torture tests exist. The shard executor now polls one retained drive future once per WorkerPool opportunity. | `JobTurnBridge` and `JobReplayLog` have zero production callers; the live shard reimplements the bridge with dynamic registries; the WGPU host explicitly ignores `ShardOutcome::Job`; no executable recorded-log replay driver exists; the torture suite is not connected through the live actor/shard/publication path. |

Phase 2 is therefore **RED overall**. Useful foundations are present, but none of P2a/P2b/P2c has a
current complete acceptance chain.

## Exact gate matrix

| Planned/current gate | Status | Implemented or historical evidence | Exact current gap |
|---|---|---|---|
| Synchronous resumable step protocol | **GREEN, source only** | `InteractiveJob::step` and `drive_step` exist in `🧰️framework/🔨️modules/🧵️job/🦀️component.rs:322-389`. | Source presence is not a runtime acceptance claim. |
| Operation, revision, generation and preview sequence | **AMBER** | `Operation` and `validate_commit` are source-live; `JobTurnBridge` checks operation/revision/generation/sequence. | The live shard uses a separate validator and advances counters with `saturating_add`; no single production publication owner applies the actor bridge's exact checks through final preview/commit consumption. |
| Per-step cancellation and watchdog | **AMBER** | `drive_step` prechecks cancellation and records watchdog outcomes. Historical P2a reports 17/17 debug and release tests. | No current runtime gate was permitted. `StepBudget` contains only fuel and deadline (`:145-153`), not item, byte, or output ownership. A job can return arbitrarily large `Vec` payloads in one step. |
| Structured child ownership | **RED** | `JobScope` counts children and derives cancellation from parent tokens. | `assert_completable` is only `debug_assert_eq!` (`:440-445`), so release builds do not prevent a parent from completing with live children. The guard has no fixed registry or terminal handback contract. |
| Checkpoint/candidate/fault ownership | **RED** | The three payload records and every outcome variant exist. | `Checkpoint.state`, both `CommitCandidate` fields, preview and fault are cloneable `Vec<u8>` (`:276-307`). No pre-admission, fixed pages, exact rejected owner, or bounded terminal disposal is defined at this universal seam. |
| Headless batch adapter | **RED** | `run_to_completion` calls individually watchdog-wrapped steps. | It is a public terminal loop (`:689-699`) and has one production caller inside a renderer WorkerPool closure. That closure remains a full-operation drain, not one admitted turn. |
| Worker adapter/session | **RED** | `WorkerJobSession` can submit one step and persistent callers exist. | `run_on_worker` owns an unbounded standard channel and blocking mutex, recursively requeues, and has a live caller that discards its terminal receiver (`:701-738`). `WorkerJobSession` uses a blocking mutex, `submit_step` panics on admission rejection, and `try_submit_step` drops the rejected closure (`:752-804`). |
| Progress taxonomy and channel policy matrix | **AMBER, definitions only** | Ten `ProgressEvent` variants and six policies are present (`:486-655`). The P2a report itself says product projections still need to route values into mailboxes. | Exact external use census is zero: no production caller constructs a `ProgressEvent` or calls its policy selector. Dynamic `Vec` payloads and a dynamically allocated coalescing key remain. |
| Generation-tagged preview overlay separate from committed scenes | **RED / absent** | `PreviewReady` and preview sequence fields exist. | No preview/overlay type or store exists in actor or shard source. `SceneStore` has only `current`, dynamic `pending`, and `pending_node_delta` (`actor/component.rs:2313-2373`), with no operation/base revision/generation/sequence key. |
| Actor scene path mounted in the live desktop host | **RED / unreachable** | `Kernel::apply_scene_patch`, `commit_frame`, and `scene_of` exist. | Exact framework call scan finds no production caller outside their definitions; the main WGPU host explicitly says every actor is activated with `window: None`, so the store stays empty (`wgpu/📦️glue.rs:4130-4137`). |
| One job step per actor turn | **AMBER** | `JobTurnBridge::step` invokes one `drive_step`; the updated shard executor polls its retained `ShardDriveFuture` once and takes at most one buffered outcome (`executor.rs:580-648`). | Every `JobTurnBridge::new` call is in actor tests. The live shard calls `runtime.step_job(...).await` directly and maintains its own cursors. Guest turn comments still admit runtime calls can consume lane budgets above 8 ms (`shard/component.rs:796-809`). |
| Live actor/shard publication consumption | **RED** | The shard emits `ShardOutcome::Job { publication }`. | The WGPU `run_turn` match explicitly ignores `Job`, `Checkpoint`, `Resumed`, and `Cancelled` outcomes (`wgpu/📦️glue.rs:4212-4221`). Thus the live host neither publishes the preview nor consumes the terminal job candidate through actor freshness validation. |
| Deterministic replay log format | **AMBER, record only** | `JobReplayLog` packs ordered `JobPublication` entries. A test builds the same scripted log twice and compares bytes. | It is `Clone + Default` over `Vec<JobPublication>`, grows through `push`, and has zero production callers. This is a serializable record, not a bounded replay authority. |
| Executable replay harness across 1..N workers | **RED / absent** | The standalone torture test historically compares direct final output on 1/2/4 worker pools. | There is no driver that consumes `JobReplayLog`, replays recorded turns/publications, validates each prefix, and reproduces the terminal result across schedules. The actor byte test executes the job twice; it never replays the log. |
| Torture responsiveness, continuous preview, cancellation p99, checkpoint resume, worker-count identity | **AMBER, historical** | Current source contains six named torture functions and 20 job test attributes. P2a reports prior debug/release, watchdog, p99, checkpoint, and 1/2/4 successes. | Those tests were not run in this read-only lane, raw current captures are absent, and they do not exercise the live shard → host → preview overlay/replay route. |
| Dedicated Phase 2 static verifier/mutations | **RED / absent** | The generic tool-job verifier currently has 299 clean self-tests. | `📜️script.ts` contains zero references to `JobTurnBridge`, `JobReplayLog`, `TortureJob`, `ProgressEvent`, or a preview overlay. It does not reject the live ignored-job outcome, missing overlay, test-only bridge, dynamic replay vector, or run-to-terminal consumers. |
| Mounted native/release/Wasm gate | **RED / unverified** | P2b reports actor/job successes from 2026-08-21. | That report says plugin-host native/test/clippy was blocked upstream and claims no plugin-host test/release/clippy/Wasm success. No build was permitted now. |

## Live caller and reachability census

The following counts exclude the job module's own tests and distinguish test-only references.

| Surface | Production reachability | Finding |
|---|---:|---|
| `run_to_completion` | **1** external production call | Renderer clipboard I/O invokes it inside a submitted WorkerPool closure (`Interpreter/🧊️component.rs:558-573`). The frame-job and native-I/O references are tests. |
| `run_on_worker` | **1** external production call | Renderer native I/O starts the self-requeue adapter and discards the returned terminal receiver (`wgpu/📦️glue.rs:2749-2768`). |
| `run_on_worker_async` | **0** external calls | Definition only. |
| `WorkerJobSession::new` | **8** production construction sites | Six plugin component sites, one plugin-host cold relay, and one reactor inference site. Two additional host sites are test helpers. |
| Explicit session-to-terminal loops | **at least 3** production loops | Plugin-host cold relay (`host/component.rs:3480-3513`), reactor inference (`infer/component.rs:214-223` onward), and framework reserved dispatch (`plugin/component.rs:15447-15469`) repeatedly await steps until terminal. Persistent media/tool-command routes also use the session, but are not counted as loops. |
| `JobTurnBridge::new` | **0 production; 7 test references** | All references are in the actor test region (`actor/component.rs:3354-3529`). |
| `JobReplayLog` | **0 production consumers/producers** | Definition, actor test construction, and schema export only. |
| `ProgressEvent` / policy selector | **0 external production references** | All construction and routing calls are inside the job component and its tests. |
| Actor `SceneStore` apply/commit/lookup | **0 external production calls found** | The only `commit_frame` call is the internal Kernel wrapper; the remaining calls are actor tests. The live WGPU host documents the missing mount. |
| Shard job publication | **live producer, ignored live desktop consumer** | `ShardLoop::drive_one` emits a `JobPublication`; the WGPU kernel loop's catch-all discards it. |

This is not a claim that the eight `WorkerJobSession` sites are all wrong. It establishes that the
universal protocol is live, while the planned actor progress/preview/replay authority is not the
common publication path.

## Placeholder, blocking, and dynamic-owner findings

### Universal job layer

- `StepOutcome`, `Checkpoint`, `CommitCandidate`, `JobFault`, and `ProgressEvent` expose cloneable
  dynamic owners without universal hard caps.
- `run_to_completion` is an unbounded outer loop. `run_on_worker` uses
  `std::sync::mpsc::channel`, `Arc<Mutex<WorkerDriveState<_>>>`, and recursive requeue.
- `WorkerJobSession` locks a standard mutex inside each worker opportunity. Its public convenience
  submit panics on finite admission failure; its non-panicking submit drops the rejected closure.
- structured child completion is a release-mode no-op.

### Actor records and scene state

- `JobStepOutcome`, checkpoint/candidate/fault publications, and `JobPublication` are cloneable. In
  particular, `JobPublication::turn_status` clones preview/checkpoint/candidate/fault owners
  (`actor/component.rs:788-797`).
- pack decode uses whole dynamic byte/vector readers. `JobReplayLog` is an ordinary dynamic vector.
- `SceneStore::apply_patch` grows `pending`; `commit_frame` loops all pending patches, builds a new
  whole `Vec`, clears the collection, and replaces/clones the snapshot Arc in one call
  (`actor/component.rs:2341-2372`).
- `Kernel` owns scene, actor, ordinal, and link `HashMap`s. Its `commit_frame` constructs a result
  `HashMap` and scans every scene.

### Live shard and host bridge

- The positive repair is real: production `block_on` has been removed from the shard executor. Its
  retained drive is polled once per WorkerPool opportunity. The two remaining `block_on` calls are in
  its test region.
- `ShardLoop` nevertheless owns live jobs in a `BTreeSet` plus multiple `HashMap`s; unregister uses
  three whole `retain` scans (`shard/component.rs:569-571`), selection scans the running-job set, and
  insertion has no visible hard fixed admission at those registries.
- completion takes a second whole runtime checkpoint await and turns checkpoint failure into empty
  state with `unwrap_or_default` (`:741-747`). That is fault swallowing, not an exact replay record.
- shard outcome encoding builds a whole dynamic `Vec`; actor UI/effect/command results still use the
  file's documented JSON placeholder.
- `OutcomeSink` is a blocking `Mutex<VecDeque<_>>`; producer growth is dynamic, both receive APIs
  drain/collect the entire queue, and `wait_for` blocks a caller thread (`executor.rs:95-140`).
- the live desktop kernel ignores all job/checkpoint/resume/cancel shard outcomes, so those owners
  are not delivered to the actor preview/commit path.

## Test and verifier coverage

### Present source coverage

- Job component: **20** test/async-test attributes in current source, including six named torture
  tests for watchdog, previews, cancellation p99, 1/2/4 workers, checkpoint resume, and seed identity.
- Actor component: **51** total test/async-test attributes. The job bridge region contains seven
  focused tests: one-step, checkpoint propagation, cancellation terminality, stale commit, replayed
  preview rejection, exact preview increment, and same-execution byte equality.
- Shard component: **35** total test/async-test attributes, including live-shaped spawn/multi-pump,
  cancellation, checkpoint/resume, budget, and turn-status cases.
- Shard executor: **9** tests for pool drive/fairness and retained ingress behavior.

### Missing meaningful coverage

- no preview overlay lifecycle, stale-generation cleanup, supersession, close, cap/+1, exact rejected
  owner, or committed-vs-preview non-persistence tests;
- no live `ProgressEvent` publisher/backpressure/coalescing ordering tests;
- no production consumer test that fails when `ShardOutcome::Job` is ignored;
- no executable log replay, corrupted prefix, missing/duplicate turn, worker-count replay-through-log,
  or replay close test;
- no release-mode structured-child terminal rejection;
- no universal payload max/+1 or exact owner handback test;
- no verifier mutation that erases the production bridge, adds a run-to-terminal loop, substitutes
  dynamic registries, removes freshness validation, or makes preview state committed/persisted.

### Read-only commands run now

| Command | Result |
|---|---|
| `rustfmt --edition 2021 --check` on actor, shard component, shard executor | **PASS** |
| `rustfmt --edition 2021 --check` on job component | **RED** — import ordering, multiline `validate_commit`, and a test conditional currently drift from canonical formatting. No write-format was performed. |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | **PASS — 299 self-tests**; none is Phase 2-specific. |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | **RED — 3 unallowlisted failures**, all concurrent P3 prepared-raster findings in `ui/.../wgpu/prepared.rs`; no Phase 2 finding was emitted. |
| exact `rg` scans for bridge/replay/progress/overlay/callers | Results recorded in the census above. |

No test execution, compilation, timing, worker-count runtime, or browser behavior is claimed.

## Smallest next source packet

The earlier readiness audit recommended repairing the shard executor first. Current source has done
that: the executor now retains and polls one drive future once. The next smallest coherent packet is
therefore **P2b live fixed preview/progress publication**, not replay and not another protocol-only
record.

Bound this packet to the actor job-publication owner plus the minimum WGPU kernel consumption seam:

1. Add an actor-owned fixed-capacity `PreviewOverlayStore` keyed by exact
   `(window/actor, job, operation, base_revision, generation, preview_sequence)`. Admit fixed item and
   byte/page credits before taking a `PreviewReady` owner. Do not derive it from or store it in the
   committed `SceneStore` snapshot.
2. Add a non-Clone exact publication cursor that consumes one `ShardOutcome::Job` from the live WGPU
   outcome match. Revalidate active operation, base revision, generation, next step sequence, next
   preview sequence, cancellation, and overlay capacity immediately before one O(1) overlay swap.
   Return the exact rejected publication on contention, stale authority, or saturation.
3. Emit the corresponding typed progress owner through one real bounded policy queue. Preserve FIFO
   for checkpoint/commit, exact coalescing identity for preview, and retain the owner on backpressure.
4. On supersede/cancel/fault/complete/window close, detach one overlay page/item per admitted cleanup
   grant; committed scene remains last-valid. Only an accepted `Complete` candidate may proceed to a
   separately revision/generation-validated commit boundary.
5. Replace the WGPU catch-all that currently ignores job outcomes with this mounted path. Add a
   production-reachability verifier predicate plus mutations for ignored job outcome, missing base or
   generation check, wrong preview sequence, committed-state aliasing, dynamic/unbounded storage,
   owner-erasing error adapters, saturation, and wholesale close.

Required source tests for that packet: first preview, ordered replacement, duplicate/replayed/stale
preview, generation supersession, base-revision advance, max/+1 bytes and items, progress
backpressure, exact rejected pointer handback, cancel/fault/complete cleanup, window close, and proof
that committed snapshots and undo state never contain previews.

Do **not** fold executable replay into this packet. Once this production publication seam is accepted,
the next P2c packet can replace `JobReplayLog` with a fixed retained log and drive the same live
publication cursor at 1/2/4 worker schedules. That ordering prevents a second test-only replay path.

## Final audit conclusion

- **P2a: RED** — protocol foundation exists, current hard ownership/release/runtime acceptance does not.
- **P2b: RED** — taxonomy exists; live progress delivery and preview overlay are absent.
- **P2c: RED** — record/test bridge exists; production consumption and executable replay are absent.
- **Phase 2: RED** — research verdict only, not ticket acceptance or closure.
