# P2c Live Fixed Replay Driver Remediation Report

Date: 2026-08-25
Executor: `/root/p6h_audit_remediation`
Status: **SOURCE/STATIC AUDIT READY — runtime and build matrices remain deferred by instruction.**

## Result

The live replay path now retains the original `SpawnJob` kind, input, checkpoint, route, operation,
generation, deterministic seed, worker count, and shard slot through fixed admitted authorities.
Capture and replay advance one scalar, page, owner, or control transition per opportunity. Replay
uses the mounted shared process pool and the same shard `GuestRuntime::checkpoint`, `restore`,
`start_job`, and typed `JobStep` route as the recorded execution. Publication comparison remains in
the accepted P2d progress-overlay boundary and advances only after its exact ACK policy is recorded.

## Production Caller Census

The focused verifier follows these live bodies rather than names or comments:

1. `ActionBus::dispatch` resolves the registered factory and calls `factory.create_job(&mut spec)`.
2. `VcsArtifactApp::dispatch_typed_command_inner` calls `self.tool_jobs.dispatch(operation_spec)`,
   mounts the returned exact owner with `MountedWorkerJobSession::try_new`, and advances it with the
   shared process pool.
3. The plugin guest export forwards host job starts into `reactor::jobs::start_job`.
4. `WasmtimeRuntime::start_job` calls the generated `call_start_job` guest boundary.
5. `ShardLoop` uses that same `GuestRuntime::start_job` body for the original retained input and the
   restored replay input, with the same checkpoint/restore authority.
6. `KernelClient::replay_job_turn` submits typed `KernelRequest::ReplayJob`; `KernelPoolState`
   validates the mounted process worker count and shard slot, then submits `Payload::JobReplay` and
   one `Payload::JobStep` through the existing parallel runtime.

No replay-only `WorkerPool`, direct job `drive_step`, run-to-terminal loop, or P2d publication bypass
remains in the mounted replay bodies.

## Fixed Ownership and Admission

- `JobReplayLog` uses 256 fixed record slots and fixed retained payload pages. A capture reserves its
  record/page counts before transfer, consumes fuel before each scalar/page/seal/ACK action, and
  retains the exact `JobPublication` until P2d returns an accepted/displaced/rejected policy.
- `FixedReplaySeed` has separate fixed kind, input, and checkpoint page arrays. Process page credit
  is claimed before each page backing allocation. Each call copies at most one source page.
- The original `String`, input `Vec<u8>`, and checkpoint owner remain in `ManuallyDrop<Option<_>>`
  lanes until their fixed copy is complete. The temporary contiguous guest ABI owner is separately
  process-admitted and filled one fixed page per opportunity; it is never the persistent replay
  source.
- Replay seed activation is split across authority, turn, running-set, placement, and ready control
  opportunities. Seed and mounted shells retire in distinct close opportunities.
- Fixed refusal slots retain the exact rejected kind/input owners. Input, kind, completion
  publication, and shell retirement each consume their own close opportunity.
- `MountedJobReplay` pre-reserves a generation-qualified recovery token. Ordinary drop and panic
  transfer the exact log owner into the fixed primary/abandonment registry; maintenance drains one
  page/owner/control per grant and rejects double release.

## Counterexample Closure

| Counterexample | Production repair | Hostile evidence |
| --- | --- | --- |
| Dynamic replay log or whole payload owner | Fixed record/page arrays and scalar/page capture cursor | dynamic record/page mutations |
| Spawn input reduced to a digest | Exact original kind/input retained and copied into fixed pages | request-erasure and fixed-seed-family mutations |
| Admission failure loses kind/input | Fixed `ReplaySpawnRefusal` lane returns and incrementally closes exact owners | refusal-lane/input/kind/fuel mutations; MAX+1 identity law |
| Replay uses private executor or runs ahead | `KernelPoolState` uses `renderer_worker_pool`; replay sends one typed shard authority then one step | private-pool, submit-bypass, and run-ahead mutations |
| Wrong route/seed/generation/worker performs work | Validation precedes `begin_replay_seed`; mounted pool count and shard slot are checked | wrong route/seed/generation/worker law and worker-count mutation |
| Zero fuel or expired deadline changes state | Opportunity admission precedes phase dispatch and copy | unchanged-state deadline/fuel law and post-copy-fuel mutations |
| Actor disappears during active capture | One close opportunity transitions any active stale seed to `Closing` before further copy/runtime work | active-stale transition body mutation |
| Cancellation loses a retained seed or permits another step | Successful mounted `CancelJob` marks the exact seed `Closing`, removes live maps, and close drains incrementally | mounted cancel law and cancel transition/body mutation |
| Cancellation classification aliases ordinary yield | `cancellation_observed` derives from typed `JobStepOutcome::Cancelled` and participates in prefix digest/match | classification, prefix, and cancel-record mutations |
| Fault payload is dropped or aliased | Typed `JobStep::Failed` becomes a lossless replay `Fault` page sequence and exact digest/prefix match | fault-publication mutation and fault replay identity law |
| Panic/drop abandons the log | Pre-reserved fixed recovery publication plus one-owner maintenance drain | reserve/publication/drain/panic body mutations |
| Close frees whole backing | Record, page, ABI owner, fixed seed, refusal, and recovery shell close in separate credited transitions | actor/shard/recovery close mutations and terminal-zero laws |

## Mounted Laws

- Record/replay determinism is exercised for real `WorkerPool` configurations `1`, `2`, `4`, and
  host default using `ShardExecutor`, `Payload::JobReplay`, and `Payload::JobStep`.
- The mounted law checks identical request/output identity, the exact `ShardOutcome::Resumed`
  authority, no automatic run-ahead, and incremental unregister cleanup.
- MAX+1 seed input returns the original allocation identities unchanged.
- Zero fuel, expired deadline, stale actor, wrong route, wrong deterministic seed, wrong generation,
  zero worker count, and invalid worker slot refuse before work.
- Cancelled capture, expired capture, and stale capture return the exact publication payload owner.
- Cancellation and fault typed terminal records replay with exact classification, bytes, prefix, and
  P2d ACK policy.
- Populated drop and `catch_unwind` panic fixtures publish the exact generation/actor/job into the
  recovery registry and drain to terminal empty.
- Every measured mounted ingress/copy/close opportunity asserts `<8 ms`; final timing acceptance is
  still reserved for the instructed runtime matrix.

## Permanent Static Gate

`📜️script.ts verify interactivity tool-jobs --p2c-only --self-test` now uses ordered Rust-body
predicates for capture admission, scalar/page fuel order, replay construction, cancellation/stale
close, exact guest checkpoint/restore/start calls, P2d ACK, mounted worker dispatch, ActionBus/plugin/
guest caller bodies, and recovery publication/drain. It rejects **53** faithful source mutations.

## Gate Evidence

- `bun 📜️script.ts verify interactivity tool-jobs --p2c-only --self-test`
  - `live-source clean; hostile-mutations=53`
- `bun 📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test`
  - `live-source clean; hostile-mutations=13`
- `rustfmt --edition 2021 --check`:
  - shard `component.rs`: clean
  - shard `executor.rs`: clean
  - actor P2c regions: clean; the full file reports only inherited formatting at lines 320/331
  - WGPU P2c regions: clean; the module-root check traverses unrelated included renderer files and
    reports their pre-existing formatting backlog
- `git diff --check` on the five P2c source/verifier files: clean
- Caller census and forbidden-pattern census: no dynamic replay publication log, automatic
  `running_jobs` replay selection, `input.clone()`, or `mem::forget` remains in the P2c bodies.

No Cargo, Nx, Wasm, browser, or broad build/test command was run, as required. Runtime acceptance,
strict-warning builds, native/Wasm matrices, browser-mounted P2d observation, and measured p99
cancellation/timing remain for the coordinator's later acceptance phase.

## Files

- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `📜️script.ts`
- this report

## Terra RED Closure — Production Mount and Accepted-Submit Retry

The later independent audit found four source blockers in the production mount. This section
supersedes the earlier 53-mutation and caller-census statements for the current tree.

| Terra counterexample | Exact production repair | Faithful static/law witness |
| --- | --- | --- |
| Plugin command gate rejected every proved operation before ActionBus dispatch | `require_complete_tool_operation_pipeline` now validates the bound live instance plus the decoded-item, work, output, timing, cancellation, freshness, and resumable/bounded-first-step contract. The call remains ordered before ActionBus dispatch and mounted-session construction. | An unconditional `full-operation-pending` body mutation is rejected. |
| Native command completion gate still made the newly mounted caller unreachable | WGPU readiness now derives from nonzero fixed request/page capacity and page-byte coverage instead of an unconditional `false`; retained `CommandBatchDriver` exchange can reach the live plugin app. | An unconditional completion-port rejection mutation is rejected. |
| No product caller invoked `KernelClient::replay_job_turn` | Native `ProgramBridge::handle_action` and `handle_command` move a typed `mountedJobReplay` control owner out of the invocation, complete the normal command exchange and invocation frame first, then submit exactly one replay opportunity through `KernelClient`. The plugin command payload does not receive a duplicate replay control. | Ordered production-body predicates require `exchange` → `invocation_from_frames` → `drive_mounted_job_replay_turn` → `KernelClient::replay_job_turn`; erasing either caller or the client call fails. |
| Worker-count evidence stopped at a mock shard | The production control schema accepts exact 1/2/4 workers or omitted host default and rejects every other explicit count; the kernel still requires equality with the mounted shared process pool and the captured shard slot. | Production caller laws retain job/begin/worker identity for 1/2/4/default and reject wrong-worker continuation identity; matrix and hostile-law erasures fail. |
| Replay state advanced before actor-mailbox acceptance | `MountedJobReplay` retains `replay_submit_sequence`; a refused submit leaves `replay_started == false`, returns without a tick, and the next continuation or repeated identical begin re-emits the exact restore/start packet. A repeated begin with different worker/slot identity faults without clearing the packet. Only the `Backpressure::Accept` arm calls `accept_replay_submission`, validates the retained sequence, marks started, and clears the retry cursor. | The production state law proves rejected/retried packet identity and no `JobStep` transition before acceptance. Mutations for pre-submit start, lost sequence, repeated-begin identity, erased Accept commit, erased refusal return, and erased law all fail. |

### Final Focused Evidence

- `bun ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test`
  - `live-source clean; hostile-mutations=67`
- `bun ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test`
  - `live-source clean; hostile-mutations=13`
- Caller census contains one product caller in
  `ProgramBridge/🧊️component.rs`, followed by the client wrapper, mounted state handler, and kernel
  request dispatch. No additional product bypass exists.
- `rustfmt --edition 2021 --check` is clean for the directly changed ProgramBridge source. The WGPU
  module-root check parses the current source and continues to report inherited formatting in
  unrelated included renderer modules; the changed replay regions match rustfmt output.
- Scoped `git diff --check` is clean.

No Cargo, Nx, Wasm, browser, or broad build/test gate was run. Runtime acceptance and the full
1/2/4/default executable matrix remain deferred exactly as instructed.

Additional current files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs`

## Second Terra RED Closure — Typed Product Authority and Shared-Pool Profiles

The second independent audit found that the accepted-submit retry was sound but the native bridge
still accepted a detached `mountedJobReplay` JSON argument, and that its worker matrix stopped at a
private mock executor. This section supersedes every earlier caller-census, public replay-API, worker
matrix, mutation-count, and final-gate statement in this report.

| Terra counterexample | Current production repair | Faithful static/law witness |
| --- | --- | --- |
| Arbitrary invocation JSON could request replay | `handle_action` and `handle_command` now decode the unmodified typed invocation, complete their normal `exchange`, and obtain replay only by moving a typed `Effect::SpawnJob` out of that successful `ExchangeOutcome`. `mountedJobReplay`, its parser, and its driver are deleted. | Both production bodies require `exchange` → `take_product_replay_authority` → invocation-result decode → owned mount → one advance. The JSON-escape mutation and either caller erasure fail. A live census finds no `mountedJobReplay`, old parser/driver, or untyped `replay_job_turn` in renderer Rust. |
| The bridge cloned the normal result's effect owner | `invocation_from_frames` takes `&mut ExchangeOutcome` and moves the remaining effect list with `std::mem::take`; the selected spawn owner is moved into `MountedProductReplayRequest`, and a fixed-kind refusal restores the exact effect at its original index. | Product extraction requires typed `Effect::SpawnJob`, fixed construction, and exact indexed restoration. The producer, request-digest, and effect-clone mutations fail. |
| Replay control omitted operation/generation/seed/route/slot/checkpoint/terminal identity | The fixed request retains instance, job, fixed job-kind bytes, request digest/version, and placement. The mounted claim scans one sealed record per maintenance opportunity and retains the latest typed checkpoint witness. The qualified authority binds actor, `JobTurn` operation, generation, seed, route, checkpoint ordinal/digest/pages/progress, terminal ordinal/kind/policy/progress/payload digest/prefix digest, worker count/slot, begin state, accepted restore/start ordinal, and profile cursor. Validation returns `Err(self)` without losing the authority. | The full-identity law accepts every matching field and returns the same authority on a wrong seed. Field-specific route, operation, generation, seed, terminal, request-digest, exact-rejection, and checkpoint-loop mutations fail. Terminal qualification additionally requires its prefix digest to cover the retained log. |
| No product producer or mounted-session causal route existed | `ExchangeOutcome::take_product_replay_authority` is the only producer. `ProgramBridge` hands its exact owner to `KernelClient::mount_product_replay`; the singleton kernel inserts a fixed claim only after registered-instance, job-kind, request-schema/version/controller/tool, duplicate, and slot checks. Idle kernel maintenance advances that same claim through the existing job capture, fixed log, replay request, shared `ParallelRuntime`, pinned shard, and `GuestRuntime` restore/start/step route. | The verifier follows ActionBus `factory.create_job`, mounted plugin session construction, plugin guest start, product extraction, client request, kernel mount, one-record qualification, idle maintenance, shared-pool replay request, and one `run_turn`. Producer/session/client/mount/maintenance/guest-start mutations fail. |
| Production accepted only the singleton's one literal worker count | The mounted kernel now validates only the fixed logical profiles `1`, `2`, `4`, and the injected process default. Each profile is submitted through the same singleton `renderer_worker_pool`/`ParallelRuntime`; no pool is created. Its worker slot is deterministically qualified as the pinned process slot modulo the requested logical count, and profile transitions occur one control opportunity at a time. | The production law requires `[1, 2, 4, process_default]`, deterministic modulo slots, invalid-profile refusal, and no private pool. Forced-singleton, unchecked-profile, zero-slot, and private-pool mutations fail. The lower mock executor test remains only supplemental and is no longer acceptance evidence. |
| Product maintenance could starve normal requests or run to terminal | `KernelRequestQueue::try_next` is checked first. Product replay receives one maintenance opportunity only while the queue is empty, yields, then returns to the queue. A fault retires one fixed claim/authority. | Ordered predicates require queue polling before pending replay, one advance, explicit yield, and fixed abort. Queue-blocking, maintenance-erasure, checkpoint-scan-loop, and timing-law mutations fail. |
| Replay could advance before mailbox acceptance | The prior repair remains: the exact replay packet/sequence stays retained on backpressure, and begin/restore state commits only in the accepted-submit arm. Each logical profile resets through this same retained retry path. | Accepted-state, retry-sequence, nonaccept-return, repeated-begin identity, worker identity, and production retry-law mutations still fail. |

### Current Source/Static Evidence

- `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test`
  - `live-source clean; hostile-mutations=80`
- `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test`
  - `live-source clean; hostile-mutations=13`
- `rustfmt --edition 2021 --check ProgramBridge/🧊️component.rs`
  - clean
- `rustfmt --edition 2021 --check actor/🦀️component.rs`
  - parses the current actor source; reports only the inherited formatting at lines 320 and 331
- `rustfmt --edition 2021 --check --config skip_children=true wgpu/📦️glue.rs`
  - parses the current WGPU source; reports only the inherited formatting backlog outside the P2c
    regions at lines 91, 106, 115, 476, 569, 676, 727, 810, 2016, 2424, 2431, 2435, 2439,
    2443, 3004, 4053, 12812, and 13251
- Scoped `git diff --check HEAD --` across actor, WGPU, ProgramBridge, and `📜️script.ts`
  - clean
- Forbidden/public caller census
  - zero `mountedJobReplay`, `take_mounted_job_replay_turn`,
    `drive_mounted_job_replay_turn`, or untyped `replay_job_turn(` occurrences in renderer Rust
  - both ProgramBridge production handlers call the typed producer, exact mount, and one advance;
    the client, state, kernel request dispatcher, and idle maintenance consumers are present

No Cargo, Nx, Wasm, browser, executable, or broad build gate was run. The requested source/static
repair is ready for a fresh independent audit; runtime matrices remain deferred by instruction.

## Third Terra RED Closure — Complete Qualification and Exact Product Recovery

The third independent audit accepted the typed route, shared logical profiles, and accepted-submit
retry but found that the full authority did not qualify every field and that fault maintenance,
application close, realm close, and bridge refusal could discard a claim or authority. This section
supersedes every earlier field-qualification, product-retirement, mutation-count, and final-gate
statement in this report.

| Terra counterexample | Current production repair | Hostile/static witness |
| --- | --- | --- |
| Kind and placement were carried but not qualified | The native request retains fixed kind bytes and placement. Kind bytes are decoded and checked against the real shard `JobReplayRequest` kind identity on every authority validation. `ShardOutcome::Job` now carries the retained `JobPlacement` from the actual admitted shard job through pack encode/decode; WGPU stores it on `MountedJobReplay` and compares it to the product request. | Kind, request, placement, shard-placement handoff, capture-call, and pack-path mutations fail. |
| Checkpoint fields were test-only | `JobReplayLog` retains the latest sealed checkpoint index as a scalar. Claim qualification scans one record per opportunity, then compares ordinal, payload digest, page count, and applied progress against `last_checkpoint_header`. Every authority validation derives that live witness again before replay work. | Checkpoint equality, production log-read, page/digest/ordinal/progress law, and scan-loop mutations fail. |
| Physical slot and logical profile were overwritten | The terminal log must match the injected process worker count and pinned physical shard slot. The authority's logical `1`/`2`/`4`/default profile and modulo slot are derived once at profile transition, validated before work, and passed unchanged into `request_job_replay`; that method rechecks the mounted process slot. | Physical count/slot, logical count/slot, forced-singleton, modulo-slot, pre-validation overwrite, and downstream-request mutations fail. |
| Begin and restore/start ordinal were ignored or overwritten | Authority validation derives expected begin state and exact retained submit/accepted sequence from `MountedJobReplay`. Begin controls the start branch. The qualified restore ordinal is passed into `request_job_replay` and compared with the previous accepted profile before `begin_replay`; after submit, authority state is updated only from the retained accepted/backpressure sequence. | Begin, restore ordinal, pre-validation begin overwrite, downstream restore erasure, accepted-submit, retry-sequence, and repeated-begin mutations fail. |
| Terminal identity was partial | Validation compares the complete retained `JobReplayRecordHeader`, including ordinal, kind, policy, applied progress, payload shape/digest, prefix, worker identity, cancellation classification, and turn. Terminal prefix must also equal the live log prefix before authority construction. | Complete-terminal, prefix, cancellation classification, physical identity, and terminal-law mutations fail. |
| Validation error was reinserted and then discarded by abort | Every product request pre-reserves a fixed epoch-qualified recovery token before leaving `ExchangeOutcome`. Request → claim → authority transfers that same token without cloning. Validation error restores the exact authority, and `abort_product_replay_one` then `take()`s and explicitly retires it into the reserved slot; neither active array is assigned `None`. | Request/claim/authority Drop bodies, reserve/publication, authority-abort, claim-abort, exact rejection, generation rediscovery, and abort-close-law mutations fail. |
| App/realm close ordinary-dropped product owners | App destroy and realm close explicitly transfer one claim or authority per opportunity into recovery. App close drains only its matching instance one owner per call; realm close drains one owner per call before terminal completion. | App-authority discard, realm-claim discard, instance close, realm close, and recovery maintenance mutations fail. |
| ProgramBridge turned an exact mount refusal into a string-only drop | The bridge still captures the diagnostic first, then submits the exact rejected request through typed `KernelClient::retire_product_replay`. The kernel publishes the owner into its pre-reserved recovery slot and returns to the normal maintenance loop, which drains one fixed owner before more command work. | Both bridge bodies require the typed retirement client; bridge-drop, retirement-client, request publication, kernel request-dispatch, and close mutations fail. |
| Recovery could strand a contended owner | The fixed registry has generation epochs, one reserved primary slot per live shell, and a fixed discoverable abandonment lane. Publication checks epoch/reservation/emptiness. Maintenance removes exactly one request, claim, or authority owner per grant and only then releases its reservation. | Epoch reservation, publication predicate, abandonment, primary close, abandonment close, command-maintenance, claim identity, and authority identity laws/mutations fail. |

### Final Source/Static Evidence

- `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test`
  - `live-source clean; hostile-mutations=108`
- `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test`
  - `live-source clean; hostile-mutations=13`
- `rustfmt --edition 2021 --check` for shard and ProgramBridge
  - clean
- Actor rustfmt check
  - parses current source; only the inherited lines 320/331 remain
- WGPU `rustfmt --edition 2021 --check --config skip_children=true`
  - parses current source; all newly touched P2c regions match rustfmt; the remaining reported
    lines are inherited WGPU formatting drift documented by the prior sections
- Scoped `git diff --check HEAD --` across actor, shard, WGPU, ProgramBridge, verifier, and this report
  - clean
- Ownership/caller census
  - no active product claim/authority assignment to `None`
  - fault, success, app close, realm close, ordinary Drop, and bridge refusal all publish or retire
    the exact owner
  - no JSON replay control, untyped `replay_job_turn`, private replay pool, or test-only checkpoint
    consumer is accepted by the structural gate

No Cargo, Nx, Wasm, browser, executable, or broad build gate was run. Runtime matrices remain
deferred exactly as instructed; the current source/static boundary is ready for fresh Terra audit.

## Fourth Terra RED Closure — Atomic Raw Product Ingress Refusal

The fourth independent audit accepted the admitted request, claim, authority, qualification, and
recovery graph but found that pre-admission kind or recovery failure reinserted a raw `SpawnJob`
before both production handlers returned through `?`. This section supersedes every earlier product
ingress, refusal, mutation-count, and final-gate statement in this report.

| Terra counterexample | Current production repair | Hostile/static witness |
| --- | --- | --- |
| Oversized kind or input was moved before a fixed refusal destination existed | Each production handler reserves a generation-qualified `MountedProductReplayAdmissionPermit` before its normal exchange. Extraction borrows the selected effect and checks fixed kind bytes and the exact shard input-page byte ceiling before the first `effects.remove`. Refusal-slot, kind, input-page, and product-recovery availability are therefore known before the raw owner moves. | Ordered body predicates require refusal reservation before exchange and kind → input → recovery preflight before removal. MAX and MAX+1 laws plus permit/preflight/move-order mutations fail. |
| A full product recovery registry returned a raw effect and lost the outcome through `?` | Recovery MAX+1 produces non-`Clone` `RefusedProductReplay`, not a string or raw `Effect`. It owns the selected raw `SpawnJob`, its original index and cause, and the untouched remaining `ExchangeOutcome.effects` backing. Extraction never reinserts the raw effect and no longer returns `Result`. | Recovery-full, raw-reinsert, empty-remainder, and restored-question-return mutations fail. The recovery-full law fills every recovery reservation, verifies exact job/kind/input/placement and both remaining effects, releases one slot, and retries the exact digest-qualified request. |
| Action and command handlers could early-return before exact handback | Both handlers explicitly match `None`, `Admitted`, and `Refused`. The refusal branch captures its diagnostic, submits the exact refusal through typed `KernelClient::retire_product_replay_refusal`, then consumes frames; no fallible frame decode can precede refusal publication. Cancellation or panic while the typed kernel request is pending invokes the refusal's exact publishing `Drop`. | Separate action/command permit, match, retirement, discard, and `?` mutations fail. The caller census finds no `take_product_replay_authority(instance_id)?` path. |
| Refusal close could deep-drop kind, input, and the entire remainder | The fixed refusal registry checks token generation, instance, reservation, and vacancy. Recovery-capacity refusals retry one fixed request construction per maintenance opportunity. Terminal refusal advances through raw-spawn split, input backing, kind backing, one remaining effect, and remaining-Vec backing as separate opportunities; only terminal then releases its slot. App and realm close use the same forced-close cursor. | Publication, generation, retry, input/kind/remainder backing, maintenance, app-close, realm-close, ordinary-Drop, and law erasures fail. Laws bound each multi-effect refusal drain and verify terminal generation release. |
| Refusal-lane MAX+1 could create a new unowned product result | The handler reserves the refusal slot before starting ActionBus exchange. If all fixed refusal permits are occupied, admission returns before any new exchange can mint or transfer a `SpawnJob`; dropping an unused permit returns the exact generation. | The refusal-slot MAX+1 law fills all permits, proves the next reservation is denied, returns every permit, then proves ordinary refusal Drop publishes the exact generation and owner. |

### Current Source/Static Evidence

- `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test`
  - `live-source clean; hostile-mutations=133`
- `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test`
  - `live-source clean; hostile-mutations=13`
- ProgramBridge rustfmt check
  - clean
- WGPU rustfmt parse/check
  - all fourth-RED P2c regions match rustfmt; only the documented inherited module-wide drift remains
- Ownership/caller census
  - no raw effect reinsertion, fallible product extraction, or action/command refusal discard remains
  - typed refusal reserve, producer, kernel request, publication, retry, forced close, and terminal release consumers are present

No Cargo, Nx, Wasm, browser, executable, or broad build gate was run. Runtime matrices remain
deferred exactly as instructed; this closure is source/static only.

## Fifth Terra RED Closure — Exact Admitted Raw Spawn Ownership

The fifth independent audit accepted the pre-admission refusal path but found that successful
admission built only fixed projections before dropping the selected raw `SpawnJob`. This section
supersedes every earlier admitted-owner, retirement-boundary, close, mutation-count, and final-gate
statement in this report.

| Terra counterexample | Current production repair | Hostile/static witness |
| --- | --- | --- |
| Successful admission dropped the selected effect after deriving its digest | Preflight still borrows kind/input and reserves recovery before removal. The admitted branch now removes the exact effect once and moves its original job, kind `String`, input `Vec`, placement, and selected ordinal into non-`Clone` `RawSpawnJobOwner`. Fixed kind bytes and `JobReplayRequest` are witnesses built from that retained owner, followed by an explicit fixed-witness acknowledgement. | The mounted admission law compares the original `String::as_ptr` and `Vec::as_ptr` with the raw owner. Ordered predicates and mutations reject `drop(remove)`, projection-only construction, empty-input digesting, copied raw fields, or an absent raw slot. |
| Request, claim, and authority carried only fixed projections | `MountedProductReplayRequest` owns the raw shell; request → claim → authority moves the same request without cloning. Their retained recovery variants also contain the same raw owner. Mount and validation recompute job, kind, request digest, and placement from the raw allocation before accepting any projected identity. | Request, claim, and authority Drop laws compare allocation pointers in each recovery variant. Structural predicates require raw fields in both live and retained requests, exact `take()` transfer, raw-derived validation, and rejection-aware retirement; transfer/rejection mutations fail at every boundary. |
| Refusal retry could create a projected replacement | Recovery-capacity refusal retains the original `Effect`. Retry reserves a product recovery token, takes that exact effect, and constructs the admitted raw owner from it. No second owner or digest-only request is minted. | Retry predicates require `self.spawn.take()` followed by `from_admitted_effect`; borrow/projection and erased-transfer mutations fail. The prior MAX/MAX+1 refusal and remainder laws remain accepted. |
| Raw backing had no exact accepted retirement boundary | The raw owner records fixed, mount, qualification, and replay acknowledgements. Qualification occurs only after the retained log has no pending copy/publication policy and the complete checkpoint/terminal witness is accepted. Accepted retirement requires all four logical replay profiles terminal, no profile in flight, and the retained accepted replay ordinal; it records terminal ordinal/prefix before changing disposition to `Accepted`. | The early-retirement law first proves refusal, then completes the qualified profile state and proves the exact checkpoint, terminal, and accepted replay witnesses. Mutations deleting mount/qualification/replay ACKs, the profile guard, accepted ordinal, or ordered final ACK fail. |
| Rejection, abort, stale owner, application/realm close, or ordinary Drop could recursively release the raw buffers | Every pre-acceptance retirement changes only `Pending` to `Rejected` and publishes the exact request/claim/authority into its pre-reserved generation slot. Recovery maintenance observes full identity, closes input and kind as two separate opportunities, and removes/releases the shell only after terminal close. Accepted authority retirement uses the same incremental lane without changing `Accepted` to rejection. | Mount-refusal and request/claim/authority Drop laws prove pointer identity, disposition, two close opportunities, and terminal zero. Close-body predicates require `close_step` before slot removal; input, kind, early-take, publication, and law mutations fail. |

### Current Source/Static Evidence

- `bun ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test`
  - `live-source clean; hostile-mutations=159`
- `bun ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test`
  - `live-source clean; hostile-mutations=13`
- WGPU `rustfmt --edition 2021 --check`
  - parses the current source; the raw-owner, mounted transfer, acknowledgement, laws, and close
    regions match rustfmt output. Remaining reports are inherited module/include formatting drift
    outside the P2c repair.
- Ownership/caller census
  - zero `drop(self.effects.remove(index))` or admitted projection-only constructor remains
  - the exact removed effect is consumed once by `from_admitted_effect`
  - raw identity is validated at mount and authority qualification, acknowledged at the fixed,
    mounted, qualified, and completed replay boundaries, and closed incrementally in recovery
- Scoped `git diff --check` across WGPU, the root verifier, and this report is clean.

No Cargo, Nx, Wasm, browser, executable, or broad build gate was run. Runtime behavior remains
deferred exactly as instructed; this closure is source/static and ready for a fresh independent
Terra audit.
