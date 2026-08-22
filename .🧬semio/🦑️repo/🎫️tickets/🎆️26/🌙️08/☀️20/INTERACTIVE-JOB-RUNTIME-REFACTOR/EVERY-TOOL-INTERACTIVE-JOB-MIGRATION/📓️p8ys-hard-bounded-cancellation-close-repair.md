# P8ys Hard-Bounded Cancellation and Close Repair

> 2026-08-22 continuation: this section is the authoritative final state of P8ys and supersedes the earlier intermediate census retained below for audit history.

## Final continuation verdict

**REJECT for complete cancellation/close acceptance. PASS only for the fail-closed shared foundation. Phase 8 remains red and production activation remains zero.**

The continuation closed the resizable ready-ticket path, separated production reactor tasks from arbitrary Rust futures, added exact operation/generation/cancellation authority to the task budget, cursorized task/resume/timer cleanup, and added a stable app-owned snapshot retirement protocol with an explicit terminal-empty witness. It also made the old actor job `Future` executor test-only: production returns `job.explicit-state-machine-required` before constructing a future.

It does not claim complete close acceptance. `VcsArtifactApp::close_step` still returns structural `Blocked` after media/segmented/snapshot cleanup because its stores, child owners, histories, caches, composition state, logs, transactions, and plugin app payload do not expose bounded disassembly. Layout's concrete snapshot disposer remains a separate owner packet and remains fail closed. No app or command row is activated.

## Final continuation changes

### Persistent reactor task/disposal authority

- Added required `ReactorTask::{step, close_step, terminal_is_empty}`. There is no blanket `Future` implementation.
- `ReactorTaskBudget` carries exact `operation`, admitted `generation`, current `cancellation_generation`, item fuel, byte fuel, and an absolute deadline.
- Added `ReactorExecutor`, with fixed-capacity numeric slots, generation-stamped IDs, direct cancellation, and bounded one-slot close. `Blocked` consumes one scheduler scan unit and cannot starve unrelated slots.
- Admission failure returns `RejectedReactorTask`, which itself must be cursor-disposed and refuses terminal drop until `terminal_is_empty` is true.
- Task slots and rejected ownership use `ManuallyDrop` fail-safe shells. An incomplete implicit destructor asserts and does not deep-drop nested state.
- Added `begin_shutdown`/`shutdown_step`; each shutdown call releases at most one task owner, one free descriptor, or one empty slot. The 1,024-slot saturation fixture proves terminal slot/free lengths reach zero before the final executor shell drops.
- Production `REACTOR_EXECUTOR` begins allocation-free. The exact 1,024-slot allocation is available only through explicit `pre_admit`; no production interactive caller currently invokes it because task submission remains fail closed.
- Kept the generic `ColdFutureExecutor` only for `#[cfg(test)]` legacy task fixtures and the now-test-only actor jobs path. Its intrusive ready list has one node per live slot, O(1) exact removal, generation validation, and no stale-ticket accumulation after ten-times-capacity detach/reuse.
- Added source fixtures for self-detach during poll, zero-fuel close, one nested owner per close step, rejected-task disposal, blocked-peer fairness, stale cancellation generation, and complete executor shutdown.

### Production opaque job futures fail closed

- Renamed the actor jobs executor to `TEST_JOBS_FUTURE_EXECUTOR` and gated it with `#[cfg(test)]`.
- Non-test `spawn_job` records `ExplicitStateMachineRequired` and returns before calling the registered `JobFn`; arbitrary future construction, poll, completion drop, cancellation drop, and executor drop are therefore unreachable in production.
- `step_job` reports the exact `job.explicit-state-machine-required` fault. Existing cold job implementations remain metadata only until replaced by explicit bounded state-machine factories.

### Reactor close-owned registries

- Replaced the dynamic armed-timer vector with a fixed 1,024-slot direct registry and an intrusive fixed insertion-order list. Insert/remove/first are O(1), collisions fail closed, and no push can grow or rehash.
- Timer admission now occurs while the producing app instance is still known. The WIT boundary rejects an unowned timer instead of manufacturing authority.
- Instance close cursors through requests, admitted task resumes, typed reactor tasks, owned timers, and metadata. Resume entries have exact actor/payload byte admission and a fixed pre-reserved queue; one close step rotates or removes one entry.
- Checkpoint restore now rejects timer rows that lack the new exact numeric instance owner rather than restoring ownerless close state.
- `RequestRegistry` remains fixed at 1,024 direct slots with bounded one-slot close; instance metadata and close cursors remain fixed direct registries.

### Snapshot A-to-B retirement

- Added required `ArtifactSnapshotDisposer<T>` and erased typed owner support. Media submission must construct the disposer and admit the exact snapshot retention into the app-owned fixed retirement registry before the Layout job starts.
- A lease is only pointer identity plus a live retirement-generation flag; it never uses `Arc::strong_count` or a racy weak-count observation.
- Cache advance A→B does not make the job or `ActiveMediaExport` the final A owner. The app retirement registry remains the exact strong owner and drains A through its owner-specific cursor.
- The erased disposer now requires `terminal_is_empty`; `Complete` before an empty exact snapshot shell faults. Incomplete retention uses a `ManuallyDrop` fail-safe so an accidental field destructor cannot deep-drop the snapshot.
- The source fixture proves the job/lease cannot drop A, cache B is independent, and only the bounded retirement owner releases A one nested item per step.

### Segmented output retained guarantees

- `ArtifactOutputChunks` uses a pre-admitted `VecDeque` with exact `ceil(maximum / 4096)` slots. Push cannot grow after admission; pop drains length to zero, so terminal deallocation does not walk capacity descriptors.
- Push/seal/take share one nonblocking state authority, closing the append-after-seal race.
- Preserved nonempty chunks, `<=4096` bytes, checked total cap, FIFO, exact `Arc` identity, seal, last `Some` → `None` → unknown, and the lossless WIT result/error bridge.

## Final verifier and ledger evidence

The two byte-canonical ledgers are identical:

`9d70e96b92337fbcfcf1446003e19b29c543f08cdb93deccd2d9a9336881bce1`

| Inventory | Final count |
| --- | ---: |
| Production macro hosts / invocations | 50 / 50 |
| Production command rows | 775 |
| Unique command rows | 773 |
| Literal registrations | 656 |
| Fixture hosts / invocations / rows | 1 / 2 / 4 |
| Production factories / registrations | 11 / 0 |
| Typed dispatches / aliases | 3 / 4 |
| Admitted complete operations | 0 |
| Remaining command rows | 884 |
| Framework-reserved residual routes | 8 |
| Pending importer owners | 35 |
| Process-global payload-store candidates | 34 |
| Verifier self-tests | 64 |
| Fail-closed failure classes | 10 |

The exact failure classes remain:

1. framework reserved routes lack route-specific state machines and commit-held authority;
2. import submission still prepares/clones the media envelope outside the job;
3. typed command prepare and commit application remain outside a full-operation state machine;
4. owned media export still lacks complete bounded poll/output/disposal proof;
5. app close still permits unresolved nested payload destruction and lacks a complete disposal owner;
6. the full reactor close gate remains red pending the production all-layer saturation/final-drop proof;
7. 34 process-global payload candidates remain;
8. eight framework-reserved routes remain fail closed;
9. 35 app-owned import routes remain fail closed;
10. 884 live command registrations remain fail closed.

## Final commands executed

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=64 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean; one recorded allowlisted test-only bridge |
| `bun ./📜️script.ts verify interactivity tool-jobs` | Expected RED: 0 admitted, 884 residual, ten exact failure classes |
| two `--format json --output .../📊️p8ys-ledger-{a,b}.json` runs | Expected RED after writing both ledgers |
| `cmp -s` and `shasum -a 256` | PASS: byte-identical; checksum above |
| `rustfmt --check` for the edited executor and actor-jobs files | PASS after formatting |
| `git diff --check` | PASS |

No Cargo command, Rust compilation, native test binary, Wasm/component build, browser runtime, or real watchdog timing was run. Rust fixtures are source evidence only. Layout TypeScript and Diagram/plugin-owner files were not edited.

## Final files in this continuation

- `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📮️requests/🦀️component.rs`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8ys-ledger-a.json`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8ys-ledger-b.json`
- this report.

## Final readiness

Ready for an independent audit of the fail-closed shared foundation and exact residual inventory. Not ready for full-route acceptance, runtime activation, or a Phase 8 PASS. The next source seam is concrete bounded disposal for `ArtifactStore`/Vcs/plugin payload owners and Layout's snapshot disposer, followed by the real reserved/import/full-command state machines.

## Verdict

**REJECT for complete cancellation/close acceptance. PASS only as a deeper fail-closed foundation. Phase 8 remains red.**

This packet replaces the live app-instance and operation-cancellation authorities with finite, pre-admitted numeric registries; removes blocking/scanning work from those registries; closes the segmented-output allocation and seal races; and production-links a synchronous, watchdog-governed app close-step protocol. It does not claim that a detached app can yet be destroyed safely. `VcsArtifactApp::close_step` deliberately returns `PluginCloseStep::Blocked` after draining the cancellation and segmented-download layers because active media jobs and the remaining app-owned stores do not expose bounded destructors. The runtime therefore preserves ownership in a finite close quarantine instead of deep-dropping on `InstanceClose`.

The permanent verifier reports ten fail-closed failure classes, zero admitted complete operations, and 884 remaining command rows. This packet is ready for an independent audit of the intermediate foundation, not for a PASS finding on the full route.

## Implemented source changes

### Fixed-width cancellation authority

- Added `ArtifactDocumentAuthority(u32)` and removed string-keyed document identity from `ToolOperationKey`.
- Replaced the cancellation `HashMap` with 1,024 pre-admitted direct slots. Slot identity is `document_id % 1_024`; a different document occupying that slot is an exact collision and fails closed.
- `begin`, supersession, exact cancellation, document cancellation, and cleanup use `Mutex::try_lock`. Contention and poison return `interactive-job.cancellation-busy`; no poison recovery or waiting occurs.
- Every removal detaches the exact scope under the guard, explicitly releases the guard, and calls `cancel_now` afterward.
- App close revokes the parent `CancelToken` and generation atomically without walking descendants. Descendant cancellation is inherited through the token parent chain.
- Added one-slot cleanup for the later app close cursor. The close protocol inspects at most one cancellation slot per step.
- Production typed, reserved, and media-operation keys now use the live numeric instance authority. Typed activation remains zero and still fails before preparation.

Source tests added for live-instance isolation, stale supersession leases, 1,024 saturated scopes, arbitrary ID `100_000`, direct-slot collision/reuse, contention, poison, and parent-scope close cancellation. These are source evidence only; native tests were not executed under the no-Cargo policy.

### Fixed media/segmented live registries

- Replaced `media_exports: HashMap` and `segmented_downloads: HashMap` with a pre-admitted 64-slot `ArtifactFixedRegistry`.
- Replaced string-keyed `media_export_documents` with one numeric current-export authority for the app instance.
- Submission fails before media preparation when the fixed live-export registry is saturated.
- Supersession no longer synchronously removes and deep-drops the old export. Its parent token is revoked and its fixed registry entry remains owned until a later explicit path handles it.
- Poll, cancel, and segmented-drain lookup use fixed numeric operation identities.
- The exact segmented terminal protocol remains last `Some(chunk)` → `None` → `unknown-segmented-download`; the WIT result/error bridge remains lossless.

This layer is not accepted as complete: explicit media cancellation still removes and drops `ActiveMediaExport`, and an active export owns an opaque `WorkerJobSession<ArtifactReservedToolJob>` whose nested job destructor has no bounded disposal protocol.

### Hard-cap segmented output and linearized seal

- `ArtifactOutputChunks::new(maximum)` calculates the exact admitted chunk slots as `ceil(maximum / 4_096)`.
- `ArtifactFixedQueue<T>` creates `VecDeque::new()`, then performs one `try_reserve_exact(exact_slots)` admission. Producer pushes reject at the exact slot cap, so `push_back` cannot grow or copy prior descriptors.
- Draining uses `pop_front`; queue length reaches zero before terminal ownership removal. Dropping the empty queue deallocates its raw buffer without walking capacity slots.
- Rejected the earlier `Box<[Option<Vec<u8>>]>` shape, which initialized and later walked as many as 8,192 empty options.
- `push`, `seal`, and `take_chunk` now use the same nonblocking state authority. `seal` acquires `try_lock` before its compare/exchange and byte snapshot, closing the append-after-seal race.
- Preserved checked byte accounting, nonempty chunks, the 4,096-byte per-push cap, exact total maximum, one seal, FIFO order, `Arc::ptr_eq` authority, and terminal `None` semantics.

Focused source tests cover 65 slots across a former growth boundary, the exact maximum, plus one rejection, FIFO drain, a fully drained destructor sentinel, and a deterministic producer/seal contention interleaving.

One residual remains: the current media-export submission constructs `ArtifactOutputChunks::new(max_output_bytes)` before `WorkerJobSession`. The admitted allocation therefore is not yet made inside a governed worker step; the verifier intentionally keeps media export red.

### Fixed runtime instance and actor registries

- Replaced `PluginRuntime.instances: LocalAsyncMutex<Vec<Arc<RuntimeAppCell>>>` with a pre-admitted 1,024-slot `RuntimeInstanceRegistry<Arc<RuntimeAppCell>>`.
- Create performs live/quarantine slot admission before constructing the app. It cannot grow, rehash, scan, or shift a fleet after construction.
- Lookup is direct by fixed-width `u32`; collisions, saturation, contention, and poison fail closed.
- `RuntimeAppCell` now uses `std::sync::Mutex<AppInstance>` only through `try_lock`. The former custom async mutex and its resizable waiter vector/draining `Drop` were removed from this path.
- Replaced `instance_actors: HashMap<u32, String>` with the same fixed registry and a 4,096-byte actor-ID cap.
- `plugin_destroy_app` directly takes one exact cell and inserts it into an equally pre-admitted close quarantine. Failed quarantine admission restores the exact cell before returning an error; ownership is neither leaked nor dropped.
- A quarantined ID cannot be recreated, including modulo collisions.

Focused source tests cover 1,024 fixed slots, `100_000`, a forced collision, slot reuse, and a nested destructor sentinel proving the live→quarantine handoff itself does not drop the app.

### Required synchronous app close protocol

- `PluginApp::close_step(maximum_items, maximum_bytes)` is a required synchronous contract, not a default forever-pending implementation.
- `RuntimeCloseCleanupJob<PA>` is an `InteractiveJob`. Each invocation uses `drive_step`, one fuel unit, one item, 4,096 bytes, an 8 ms deadline, and the runtime close-cleanup watchdog site.
- `reactor::poll_kernel` calls `plugin_step_close_cleanup(runtime)` on every production poll after `InstanceClose` detaches the app. This is the existing production lifecycle route; no second destroy route was introduced.
- `VcsArtifactApp::close_step` revokes the app scope once, advances exactly one of 1,024 cancellation slots per call, then advances exactly one of 64 segmented slots or drains one chunk up to 4,096 bytes.
- Empty-slot inspection counts as one work item, so cursor progress cannot masquerade as zero progress.
- The runtime detects repeated `Pending { 0, 0 }` and marks the quarantine entry blocked after eight steps. `PluginCloseStep::Blocked` records a structural disposal blocker immediately and prevents silent infinite cleanup retries.
- `VcsArtifactApp` explicitly returns `Blocked` after the bounded cancellation/segment layers. It never returns `Complete`, so the runtime never runs its still-deep final destructor from this route.

This is a production-linked bounded cleanup seam, but not the required final scheduler architecture. It is driven inline by the reactor through `drive_step`, not admitted to `WorkerPool`: the current `WorkerPool::submit` itself has an unbounded, blocking `VecDeque` enqueue and no saturation result. Moving this non-`Sync` app state to that pool without first replacing the pool enqueue would merely move the bypass. The verifier therefore rejects complete close acceptance.

## Verifier changes

`verify interactivity tool-jobs` now requires or rejects:

- exact pre-reserved segmented slots, no dynamic producer growth, zero-length terminal queue, and no boxed empty-slot terminal walk;
- one shared nonblocking authority across push/seal/take and the deterministic seal-race fixture;
- fixed numeric cancellation authority, exact capacity/collision behavior, try-lock/poison fail-closure, explicit detach-before-cancel, and saturation tests;
- no string-keyed/resizable cancellation or live media/segmented registries;
- fixed runtime instance and actor registries, preconstruction admission, direct detach, close quarantine, and the production `InstanceClose` caller;
- a required app close-step protocol, fixed per-step item/byte limits, production cleanup driving, final-destructor proof, and saturation ownership;
- rejection of implicit field destruction, fake cleanup queues, zero-saturation behavior, blocking locks, lock-held cancellation, whole-map drop cancellation, resizable/scanning runtime instance close, and premature segmented removal;
- rejection of reactor task/request/open-instance cleanup while it still scans resizable maps/vectors or retains/wakes whole queues.

The verifier self-test total increased from 46 at P8yr to 55. New adversarial fixtures include string-keyed cancellation, blocking lock-held cancellation, implicit close destruction, fake cleanup enqueue, scanning runtime instance close, scanning reactor task/request close, dynamic segmented growth, boxed terminal capacity walking, and append-after-seal racing.

## Exact deterministic ledger

Both ledgers are byte-identical with SHA-256:

`3fe807abcc5e4d6e59cedd2ce0d0c620fcd8b760c47bed2540325e8661503b2f`

| Inventory | Count |
| --- | ---: |
| Production macro hosts / invocations | 50 / 50 |
| Production command rows | 775 |
| Unique command rows | 773 |
| Literal registrations | 656 |
| Production factories / registrations | 11 / 0 |
| Typed dispatches / aliases | 3 / 4 |
| Admitted complete operations | 0 |
| Remaining command rows | 884 |
| Framework-reserved residual routes | 8 |
| Pending importer owners | 35 |
| Process-global payload-store candidates | 34 |
| Verifier self-tests | 55 |
| Fail-closed failure classes | 10 |

The ten exact failure classes are:

1. framework reserved routes still lack real route-specific state machines and commit-held authority;
2. shared import submission still prepares/clones the whole media envelope before its job;
3. typed preparation/commit remains outside the full-operation protocol;
4. owned media export is still rejected, including pre-job segmented allocation and incomplete disposal;
5. app close still lacks a final bounded destructor and saturation-safe complete cleanup owner;
6. reactor task/request/open-instance cleanup still scans resizable collections or whole ready queues;
7. 34 global payload-store candidates remain;
8. eight framework-reserved routes remain fail closed;
9. 35 app-owned importer routes remain fail closed;
10. 884 live command registrations remain fail closed.

## Gates executed

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=55 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean; one recorded allowlisted test-only blocking bridge |
| `bun ./📜️script.ts verify interactivity tool-jobs` | Expected fail-closed exit 1: 0 admitted, 884 remaining, ten failure classes |
| Two `--format json --output .../📊️p8ys-ledger-{a,b}.json` generations | Expected fail-closed exit 1 after writing each ledger |
| `cmp -s` on both ledgers | PASS: byte-identical |
| `shasum -a 256` on both ledgers | PASS: identical hash shown above |
| `git diff --check --` on this packet's script/Rust/evidence paths | PASS |

No Cargo, Rust compiler, native test, Wasm/component bindgen, browser integration, or real watchdog timing command was run. Source tests are not claimed executed. Layout-owned TypeScript and Diagram/plugin-owner files were not edited.

## Remaining mandatory repair

1. Give `ActiveMediaExport` and every concrete `ArtifactReservedToolJob` a bounded disposal state machine. A cancelled session must release at most one admitted item/byte slice per cleanup step, and its final destructor must be constant after all job-owned state is detached.
2. Extend the same explicit disposal protocol across the remaining `VcsArtifactApp` stores, children, histories, logs, pending transactions, composition state, and other nested collections. Only then may `VcsArtifactApp::close_step` return `Complete`.
3. Replace `WorkerPool::submit` with a finite, nonblocking, saturation-reporting admission path, then schedule the persistent close job there or prove an equivalent reactor scheduler contract. Queue-full handling must retain exact ownership without leaking or deep-dropping.
4. Replace reactor `OPEN_INSTANCES`, `TASK_RECORDS`, `TASK_KEYS`, `TASK_RESUMES`, and `INSTANCE_QUOTAS` close-reachable `Vec`/`VecDeque`/`HashMap` paths with finite numeric authorities and cursorized cleanup. `RequestRegistry::cancel_instance` and `LocalExecutor::cancel` must stop whole-map/whole-ready-queue scans and synchronous future drops.
5. Move `ArtifactOutputChunks` capacity admission inside the operation-owned media job stage.
6. Implement the separate real reserved/import/full typed-operation state machines. Production activation remains zero until every prepare/reducer/output/commit boundary is exact and bounded.

## Files changed

- `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8ys-ledger-a.json`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8ys-ledger-b.json`
- this report
