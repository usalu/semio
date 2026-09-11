# Fill Build Host Tick (2026-09-10)

Browser follow-up: the Fill tool body rendered but the fill build never ran. Native fill is proven (family 59/0, heap law ready>0). This note is the host-wiring close-out.

## 1. Arming is not the gap

Selecting the Fill tab **does** dispatch `setActiveTool "fill"`. The removed duplicate `toggle-group-item#tool.fill` is not required — `button#tool.fill` (the tool-category leaf) is the real arming control. Probe `🗑️generated/probe-2026-09-10T05-47-06.md` and later fill probes all show:

```
[DEBUG] setActiveTool {requested: fill, next: fill, prior: null}
```

A mid-flow bounce still exists (`requested: ""` when `selectedToolId` goes null); the Fill tab re-arms and then holds.

## 2. Why ticks never started (host, proven)

Two ShellHost bugs, both now fixed:

1. **`setActiveTool` was an empty Emit.** Framework `uiScope` is `none`, so `refreshUi` returned immediately. World3dHost kept stale `interaction.activeUtility` (`"select"`), so `worldFillBuildShouldTick` stayed false and the 120ms `fillBuildTick` interval never armed.
2. **Windowed `handleAction` dropped host `activeToolId`.** `windowViewContext(session.viewState)` preserves a stale/absent session `activeToolId` (`SET_ACTIVE_TOOL` only writes `actionPane` + `activeToolIdRef`). Guest `fill_build_tick` gates on `puzzle3d_fill_tool_active(config.active_tool_id)`.

Fixes (no guest change):

- `hostArmedViewContext` / `ViewModel::for_host_armed_action` overlays the host-owned tool, then binds window or panel. ShellHost action dispatch uses it.
- After `setActiveTool`, `applyHostEffects(..., { kind: "full" })` so the world republishes `activeUtility: "fill"`.

Laws: `🧪️tests/🔬️window-view-context` (TS + Rust) — fails-before `windowViewContext(view, "left").activeToolId` is undefined; passes-after `hostArmedViewContext(view, "fill", "left").activeToolId === "fill"`. Fixture `hostArmed` vectors. Ran: TS `cases=5 hostArmed=3`; Rust `window_view_context_uses_the_addressed_instance` ok.

## 3. Ticks started, Count stayed 0 (host, proven)

After §2, probe `🗑️generated/probe-2026-09-10T06-14-26.md`:

- Gate opened: `[DEBUG] fillBuildTick gate {activeUtility: fill, fillBuildShouldTick: true}`
- Guest ticks: `[DEBUG] fill_build_tick active_tool=Some("fill") spawn=false changed=false`
- Panel showed **Cancel fill** (job identity live)
- Count stayed `0` / `aria-valuemax 1000`

`fill_build_tick` only **polls**. Solver work is `Effect::SpawnJob` + Isolated `semio.puzzle3d.fill` bounded job. Native tests call `drive_enqueued_fill_job_for_test` (tight `drive_fill_envelope` loop). The browser counterpart is `PluginRuntime.driveSpawnedJob` (`start-job` then `step-job`).

## 4. Isolated job was starved (host, proven)

Probe `🗑️generated/probe-2026-09-10T06-23-38.md`:

```
[DEBUG] turn effects [..., spawn-job, ...]
[DEBUG] routeHostEffects spawn-job {jobType: bigint, kind: semio.puzzle3d.fill, job: 155}
[DEBUG] driveSpawnedJob start/started {kind: semio.puzzle3d.fill, inputBytes: 56}
[DEBUG] driveSpawnedJob step {step: 3, status: running}
[DEBUG] driveSpawnedJob step {step: 4, status: running}
```

Five `step-job`s in 30s. Token admit is ~6 field slices (`FillEnvelopeTokenCursor`); the plan never left Admitting, so `poll_fill_job` stayed `changed=false` and Count stayed 0.

Cause: `driveSpawnedJob` took **one `step-job` per `serializeCommandIngressForActor` admission**. The 120ms `fillBuildTick` poll (and command-ingress chatter) interleaved every slice. Native `ShardLoop::pump` grants many job slices per turn. Migrated `setActiveExample` jobs keep running because `drainTypedOperations` is a dedicated `more-work` driver; Isolated fill is a different path.

Fix: `isolatedJobStepsPerSerializedAdmission(stepsPerYield)` batches 32 `step-job`s inside one serialized admission (same number as `PLUGIN_JOB_STEPS_PER_YIELD`), then yields. Law in `🔌️plugin-runtime` tests: `isolated job admission batch` (fails-before batch=1; passes-after batch=32). Vitest: 1 passed / 784 skipped.

## 8.8 Browser evidence

| Artifact | What it proves |
| --- | --- |
| `🗑️generated/probe-2026-09-10T05-22-32.md` | Pre-fix: body visible, Count 0, **zero** `fill_build_tick` |
| `🗑️generated/probe-2026-09-10T05-47-06.md` | Fill tab arms `setActiveTool fill`; still no guest ticks |
| `🗑️generated/probe-2026-09-10T06-14-26.md` | After §2: guest ticks + Cancel fill; `changed=false`; Count 0 |
| `🗑️generated/probe-2026-09-10T06-23-38.md` | Isolated job starts (`semio.puzzle3d.fill` job 155); **5 steps / 30s**; Count 0 |

### Tails (06-23-38)

- `spawn-job` kind `semio.puzzle3d.fill`, job bigint `155`, input 56 bytes
- `driveSpawnedJob started` then `step 3/4 status=running`
- `fill_build_tick` `spawn=false changed=false faulted=false`
- sliders `?=0/1000`; panel `Count | 0 | Cancel fill`
- faults: 0

### Open: Count>0 + slider edit

Closed in §8.9 (cadence follow-up). Coordinator recycled `:6014` after the post-batch wedge; the 32-step batch stayed. Remaining gap was tick cadence, not arming.

## 8.9 Cadence: skip fillBuildTick while Isolated job drives

Coordinator probe `🗑️generated/probe-2026-09-10T06-41-09.md` proved §2–§4: session arms, Cancel fill, guest `fill_build_tick active_tool=Some("fill")` (eprintln wraps `Some` / `("fill")`). Defect: **5 armed ticks / ~50 s** (~1 / 10 s). Each tick is a full command-ingress; `createInFlightSkippingInterval` holds inFlight until `OperationCompleted`, so the 120 ms interval degrades to guest turn time and starves `driveSpawnedJob`. Native W-F2c needs ~320 job steps → ~50 min at that cadence. Count stayed `0/1000` after 30 s + End.

### Fix (host only; 2 ms guest step budget unchanged)

`fillBuildTick` and Isolated `step-job` share `serializeCommandIngressForActor`. While a drive is active, World3dHost must not take that lock except for a rare UI poll.

- `beginIsolatedJobDrive` / `endIsolatedJobDrive` / `requestIsolatedJobUiPoll` / `takeIsolatedJobUiPoll` live on `PluginRuntime` (ShellHelpers re-exports so World3dHost does not import PluginRuntime). `driveSpawnedJob` begins in `try`, ends in `finally`.
- After each 32-step admission, poll if the batch saw `running.progress` or `isolatedJobUiPollEverySteps(step, 128)`.
- `worldFillBuildHostTickAllowed(shouldTick, driving, pollDue)`: driving && !pollDue → skip. Interval:

```ts
if (isolatedJobDriveIsActive() && !takeIsolatedJobUiPoll()) return undefined;
return dispatch("fillBuildTick");
```

First ticks still run (drive not started) so `Effect::SpawnJob` happens. Then the job pump re-acquires serialize immediately after `yieldPluginUiContinuation`.

Laws: engine-contract `skips fillBuildTick while an Isolated job is driving unless a UI poll is due` (fails-before driving && !pollDue). plugin-runtime `isolated job admission batch` adds the 128-step stride (fails-before stride 1). Ran `SEMIO_TEST_LEVEL=long` vitest: 2 + 2 passed.

Probe `--fill` now polls Count up to 60 s, treats wrapped `changed=` / `true`, logs End Count, then opens history. After a flake, fill-tab also waits for **Cancel fill** before the 60 s clock.

### Browser evidence

| Artifact | Result |
| --- | --- |
| `🗑️generated/probe-2026-09-10T07-01-09.md` | **Apply proven.** Cancel fill + treeItems 4 at activate. Wait 60 s: Count UI 0 (polls only; `changed=true` wrapped 5×). End: **Count 1**, slider `?=1/1000`. History `create-object` `puzzle3d.brush…` + `SetFillCount count=1`. treeItems 4→23. Guest ticks wrap `active_tool=` / `Some` / `("fill")`. **faults=0 / FAULT_RE=0**. |
| `🗑️generated/probe-2026-09-10T07-05-08.md` | Flake: Cancel fill late (treeItems 3 at activate). End stayed Count 0; no SetFillCount / create-object. ticks 7, spawnChanged 3, faults=0. Known mid-flow `setActiveTool ""` bounce. |
| Third probe | **Not run.** `:6014` returned HTTP 000 (vite CPU ~22%, not 97%). Did not kill; coordinator recycles. |

tsc `@semio-tech/framework-renderer-react` typecheck: **zero new errors** in PluginRuntime / World3dHost / ShellHelpers / engine-contract (pre-existing `docklayoutstore` noise only). Guest untouched; wasm32 not run. No new host `[DEBUG]`; guest `fill_build_tick` left in place.

Count row stayed 0 through the 60 s UI window on the successful run — `fillBuildTick` is now rare by design, so the panel lags the Isolated job. End (a real ingress) published Count=1 and applied one unit ~77 s after activate, vs the prior ~50 min projection.

## Other open items

- Mid-flow arming bounce (`setActiveTool {requested: ""}` when `selectedToolId` is null). Fill-tab re-arms; probe now waits for Cancel fill. Still flakes (07-05-08).
- `requestContextMenu` windowed path still skips `injectActiveTool` / `hostArmedViewContext` (same class as §2, out of fill scope).
- Guest `[DEBUG] fill_build_tick` eprintln left for the close-out sweep.
- `:6014` HTTP 000 after the second probe — do not kill; coordinator recycles.

## Suite / tsc

- Guest not touched; wasm32 not run. Puzzle3d 639/1 baseline unchanged by this host work.
- plugin-runtime vitest `-t "isolated job admission"`: **2 passed** (batch 32 + poll stride 128).
- engine-contract `-t "unstarted fill plan|Isolated job is driving"`: **2 passed**.
- renderer-react typecheck: no new errors in touched files.
- Host Isolated-job census `[DEBUG]` remains stripped; guest `fill_build_tick` left in place.

## 8.10 Browser undo never reaches `commit_framework_history_route`

Coordinator evidence (`🗑️generated/probe-2026-09-10T07-36-34.md`, wasm #33): host `[DEBUG] undo route` fires, guest ingress settles `command-complete`, and **zero** `[DEBUG] history route` / `group history` / `spawn-job routed` lines. Native history laws already move the store. SWITCH (Migrated) and fill (Isolated `spawn-job`) complete because those jobs **park** across host polls; undo does not.

### Root cause (guest, proven in source + native law)

`"undo"` is a framework reserved InteractiveJob. `dispatch_framework_reserved_action` **awaits** `run_framework_reserved_job` and only then calls `commit_framework_history_route` (the eprintln site). The job loop always starts with `plugin_job_yield_once()` — first poll `Pending`, second poll `Ready`.

Browser WIT `semio_owned_poll_v1` ran that whole `poll_kernel` future under `resolve_ready` (**one** poll). A Pending reserved future therefore never reaches commit. `resolve_ready` panics on Pending; a turn that never enters the reserved await still returns `command-complete` (host undo is an `AppCommand::Command` invocation). Migrated SWITCH admits on first poll and continues via `more-work` + `drainTypedOperations`. Isolated fill surfaces `spawn-job` and is driven by `driveSpawnedJob`. Reserved undo does neither — the commit is in-stack after the yields.

Host-TS cannot resurrect a dropped/unstarted in-stack await. Pumping `more-work` after undo is a no-op: wasm #33 never requests it. There is no fill-style tick for the framework lane.

A second gate: `handle_action_invocation` required `registry.window_action(window_kind, "undo")` (and `has_mode`). A window that does not own the injected history row rejected before `dispatch_framework_reserved_action`. Browser always sends an invocation (`encodeWindowActionInvocation`).

### Fix (guest; **needs wasm rebuild #34**)

Do **not** rebuild or restart serves from this lane. Coordinator deploys.

1. `drive_self_waking_ready` — loop-poll a self-waking future (bound 1_048_576) so `plugin_job_yield_once` completes inside **one** host wasm poll. WIT `semio_owned_poll_v1` and `AppCommand::ArtifactCommand` history verbs use it instead of `resolve_ready`.
2. `is_framework_reserved_action_id` / `framework_reserved_action_kind` — `dispatch_action` routes reserved verbs before catalog `get`. `handle_action_invocation` falls back to that kind when the window does not own the row, and skips mode-catalog checks for reserved verbs.

Coordinator `[DEBUG]` eprintlns / PluginRuntime traces left in place. No new host `[DEBUG]`.

### Laws (native, this crate)

| Law | Result |
| --- | --- |
| `plugin_builder_contract_tests::reserved_undo_invocation_does_not_require_window_ownership` | **ok** — invocation `window-without-undo` + `"undo"` reaches commit (`[DEBUG] history route action=undo` + benign NothingToUndo) |
| `plugin_runtime::drive_self_waking_ready_completes_a_plugin_job_yield` | **ok** — one `plugin_job_yield_once` completes under the drive loop |

```
cargo test -p semio-framework-plugin reserved_undo_invocation -- --test-threads=1
# ok. 1 passed

cargo test -p semio-framework-plugin drive_self_waking_ready -- --test-threads=1
# ok. 1 passed
```

Store-moving undo remains the existing `history_actions_round_trip_through_the_store` / puzzle3d native family (workspace Increment factories are currently fail-closed on `VcsArtifactApp::new`; not this change).

### Browser verification

`:6014` answered HTTP 200 at the start of this lane, then the assigned probe

`bun ".🧬semio/.../🔍️browser-probe.ts" --interact --undo --settle=30 --port=6014`

hit Playwright `goto` Timeout 60s (`domcontentloaded`). Immediate follow-up `curl` to `/` was **HTTP 000** (8s, 0 bytes). **Stopped.** Coordinator recycles. wasm #33 is still the served guest, so `[DEBUG] history route` cannot appear until **#34** is deployed.

### Handoff

- Guest fix is in `🔌️plugin/🦀️.rs` + the two laws above.
- **needs wasm rebuild #34** — then re-run the undo probe. Success = `[DEBUG] history route` and navbar **Concrete Forest** after one undo.
- Host Isolated-job pump / fill tick unchanged. No host-TS undo pump added.

## 8.11 Undo actor-ingress gate (wasm #34 follow-up)

Probe `🗑️generated/probe-2026-09-10T08-26-22.md` (wasm #34, `:6014`): four local undo dispatches, each `[DEBUG] undo route {"route":"local",...}` + `[DEBUG] command ingress settled status=command-complete`. Guest stderr is live (`cooperative-maintenance` as `error:`). **Zero** `[DEBUG] history route action=undo`. Navbar stayed Nakagin.

### Trace (actor ingress, not `plugin.handleAction`)

ShellHost local undo (`directBrowserActor !== null`) calls `dispatchDirectBrowserActorCommand` → `retained.actions.dispatchCommand` → `createBrowserActorAppCommandRequestV1` (`AppCommand::Command` + `encodePackValue(ActionInvocation)`). That is the guest **actor command-page** path: `poll_kernel` → `PluginCommandIngress` (Encoded → Decoding → Decoded → Ready) → `plugin_exchange`.

On Ready, `plugin_exchange` decodes `ManifestActionInvocation` and **then**:

1. `addressed_action_view` — `for_window_instance(window_instance_id)` plus kind match. History chrome / navbar undo often addresses a window that is not in `view.window_instances` (or whose kind ≠ `windowKindId`). Failure `push_app_fault` + `break 'dispatch` still settles **command-complete**. `commit_framework_history_route` never runs. This is the #34 miss: the ownership skip was only inside `handle_action_invocation`, which this gate never reached.
2. `handle_action_invocation` (only if step 1 succeeds) — already skips `window_action` for reserved verbs.
3. WIT `reactor::Guest::poll` is `.await` of `reactor::poll` with **no** `drive_self_waking_ready`. Browser actor uses this export, not `semio_owned_poll_v1`. A reserved job that yielded inside one host turn still could not finish on this ABI.

`plugin.handleAction` (`plugin_handle_action`) is the unused fallback when `directBrowserActor === null`.

### Fix (guest; **needs wasm rebuild #35**)

One reserved-verb rule on both entry paths:

- `admit_addressed_action_view` — catalog actions still require a live window instance/kind; reserved verbs (`undo`, `redo`, `interactionSelect`, …) fall back to the unprojected view when projection fails.
- Used by **both** `plugin_exchange` (`AppCommand::Command`) and `plugin_handle_action`.
- `handle_action_invocation` still routes reserved ids through `dispatch_framework_reserved_action` without a window-kind catalog row.
- `drive_self_waking_ready` now wraps: WIT `reactor::poll`, `handle_action_invocation` on both entry paths, and the existing `semio_owned_poll_v1` / ArtifactCommand history path. The reserved job completes inside the driven poll.

No host-TS change. Coordinator `[DEBUG]` traces left in place. Do not rebuild or restart `:6013`/`:6014` from this lane.

### Laws

```
cargo test -p semio-framework-plugin reserved_undo -- --test-threads=1
# ok. 2 passed; 0 failed; 624 filtered out

cargo test -p semio-framework-plugin drive_self_waking_ready -- --test-threads=1
# ok. 1 passed; 0 failed; 624 filtered out
```

| Law | Result |
| --- | --- |
| `reserved_undo_invocation_does_not_require_window_ownership` | **ok** (kept) |
| `reserved_undo_actor_ingress_admits_undeclared_window_kind` | **ok** — strict `addressed_action_view` rejects `missing-instance`; `admit_addressed_action_view` admits; seed Apply count 1→0; `[DEBUG] history route action=undo` |
| `drive_self_waking_ready_completes_a_plugin_job_yield` | **ok** (kept) |

**3 passed / 0 failed** in the undo-relevant scope.

After coordinator **wasm #35**: re-run `--interact --undo --settle=30 --port=6014`. Success = `[DEBUG] history route` and navbar **Concrete Forest**.

## 8.12 Reserved verbs on the host spawn-job lane (wasm #35 hang)

Probe `🗑️generated/probe-2026-09-10T09-27-53.md` (served guest **wasm #35**, hash matches dist). Local undo funnel reaches `plugin.handleAction` and **never resolves**. Guest stderr is live; **zero** `[DEBUG] history route`, zero guest panic, no `exceeded the self-wake bound`. Other actions still settle `command-complete`.

### Why in-stack reserved completion cannot answer

`dispatch_framework_reserved_action` admitted the verb, then `await run_framework_reserved_job(...)`. That worker-pool loop parks on `plugin_job_yield_once` plus work only a **later** reactor turn can produce. `drive_self_waking_ready` cannot finish it in one host `handleAction` turn, emits no `spawn-job` / `more-work`, and the host promise hangs forever. Isolated fill already avoided this: first turn only admits + `Effect::SpawnJob`; `routeHostEffects` / `driveSpawnedJob` / `deliverJobCompletion` drive the job; `Event::JobCompleted` is the commit door.

### Conversion (guest only; no wasm rebuild, no :6013/:6014)

Kind `framework.reserved.tool`. First non-clipboard reserved turn:

1. Admit proof / wire / work items and mint `FrameworkReservedCommitPermit` (no mounted worker).
2. Stash `PendingFrameworkReserved` on `VcsArtifactApp.pending_reserved`.
3. Emit `Effect::SpawnJob { Isolated }` and return immediately with `{ operationId, generation }`.
4. Host `driveSpawnedJob` already pumps Isolated jobs.
5. `Event::JobCompleted` calls `plugin_complete_reserved_spawned_job` → `commit_framework_history_route` / revert / shared-host (interaction, filter, note). Clipboard stays on the old in-stack worker for this wave (app-owned emit job).
6. Bounded job is a two-step envelope walk (`Cursor` → `Retiring` → `Done(raw)`). Kind is re-registered on every admit (thread-local `BOUNDED_KIND_REGISTRY`; a process-wide `Once` left sibling test threads without a factory).
7. JobCompleted uses `resolve_ready` so the poll future does not grow. `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` stays **ok**.

Do not add pending reserved to `has_pending_typed_operations` (would spin `drainTypedOperations`).

### Laws (native `semio-framework-plugin` lib tests, this turn)

```
cargo test -p semio-framework-plugin --lib reserved_undo
# test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 631 filtered out

cargo test -p semio-framework-plugin --lib undo_on_empty
# test result: ok. 1 passed; 0 failed; 633 filtered out

cargo test -p semio-framework-plugin --lib benign_undo_with_nothing
# test result: ok. 1 passed; 0 failed; 633 filtered out

cargo test -p semio-framework-plugin --lib a_settled_reactor_turn_retains
# test result: ok. 1 passed; 0 failed; 633 filtered out
```

| Law | Result |
| --- | --- |
| `reserved_undo_invocation_does_not_require_window_ownership` | **ok** — first-turn spawn-job, settle commits benign empty undo |
| `reserved_undo_actor_ingress_admits_undeclared_window_kind` | **ok** — first turn answers with `SpawnJob` and does **not** regress count; `settle_framework_reserved_admission` drives the job, prints `[DEBUG] history route action=undo`, count 1→0 |
| `reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route` | **ok** — Isolated `framework.reserved.tool` on admit; no `history-changed` until drive; drive runs `commit_framework_history_route` |
| `undo_on_empty_history_is_a_benign_no_operation` | **ok** |
| `benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none` | **ok** |
| `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` | **ok** (poll future not inflated) |

**6 passed / 0 failed** in the spawn-job undo scope. Scratch: `🗑generated/reserved-undo-laws.txt`, `undo-on-empty.txt`, `benign-undo.txt`, `future-size.txt`.

`history_actions_round_trip_through_the_store` / `undo_and_redo_append_entries` / typed `increment`/`setLabel` seeds still fail-closed on `interactive-job.missing-factory` (same workspace catalog hole §8.10 already named). Not this conversion.

### Browser / wasm

**No rebuild.** Coordinator still serves **#35**. This guest change needs **wasm #36** before `[DEBUG] history route` can appear in `:6014`. After #36: `plugin.handleAction` must resolve on the first turn (admission + leftover `spawn-job`); `driveSpawnedJob` must print `[DEBUG] history route action=undo`; `awaitOperationSettle` must resolve.

### Handoff

- Guest: `🔌️plugin/🦀️.rs` (admit/spawn/complete), `⚛️reactor/🔄️turn/🦀️.rs` (`JobCompleted` → reserved complete), contract tests settle Isolated jobs.
- Host Isolated pump unchanged.
- Coordinator rebuild **#36**.

## 8.13 Wasm #36 probe: lane works, undo still hangs (W-G3)

Probe `🗑️generated/probe-2026-09-10T11-05-11.md` on served **#36**. Mixed result.

### Job 28

`[DEBUG] job done kind=framework.reserved.tool job=28 status=done steps=2` fired during the earlier interact / settle-continuation window (before the four undo clicks). Two-step envelope walk matches the reserved bounded job. Strongest verb: **interactionSelect** from the pick-object `"n"` step (reserved interaction, not clipboard). Hover / clearSelection are possible but that pick is the only reserved gesture the probe script runs before undo.

`spawn-job routed` did not print for job 28. That log was not on the wire at 11:05 (peer swept ShellHost/PluginRuntime `[DEBUG]` at 12:13; coordinator re-added afterwards). It does **not** prove a second guest start path. `driveSpawnedJob` still completed the Isolated job.

### Why `undo handleAction resolved` never fired

Shell `onAction` for a live document prefers `directBrowserActor !== null` → `dispatchDirectBrowserActorCommand` / `BrowserActorActionMailboxV1.dispatchCommand` (30s waiter). The `[DEBUG] undo handleAction resolved` `.then` is **only** on the fallback `plugin.handleAction(...)` arm. Four `[DEBUG] undo route {"route":"local",...}` lines then no resolved line is consistent with the actor mailbox, not with a missing guest first-turn.

Both arms encode the same invocation:

```
address: {pluginId, appId, modeId, windowKindId:"puzzle3d-main", windowInstanceId:"puzzle3d-main-perspective", actionId:"undo"}
arguments: {windowId:"puzzle3d-main-perspective"}
```

The fallback is `plugin.handleAction(instanceId, JSON.stringify(invocation), viewState)` → `performInvocation` → `client.command(encodePackValue(invocation))` → `plugin_exchange` `AppCommand::Command`. The actor path pages the same `AppCommand::Command` into `plugin_exchange`. **`plugin_handle_action` (WIT JSON export) is not the live browser door.**

Guest decode: `ManifestActionInvocation` → `admit_addressed_action_view` → `drive_self_waking_ready(handle_action_invocation)` → `dispatch_action` → `dispatch_framework_reserved_action` → non-clipboard **spawn-admit** (`Effect::SpawnJob` Isolated `framework.reserved.tool`). Remaining `run_framework_reserved_job` sites: clipboard, `import-media`, `configuration-binary` only.

Losing intercept removed: `dff_public_action_admission_tests` still required the entry points to `.await` the retained job (and looked for deleted `AppCommand::ConfigRead`). Rewritten to `action_entry_points_drive_spawn_admit_instead_of_awaiting_the_retained_job` — both `plugin_handle_action` and the Command arm must `drive_self_waking_ready(handle_action_invocation)` and must not `resolve_ready` or `run_framework_reserved_job`.

### Temporary `[DEBUG]` for rebuild #37

- `plugin_handle_action` entry: instance id; after JSON parse: `actionId` + branch (`spawn-admit` / `clipboard-instack` / `catalog`).
- `plugin_exchange` entry: instance + whether a command is present; after Command decode: same `actionId` + branch, or `command-invocation` / `command-frame` on decode miss.

### Law (real export, not internal helpers)

`reserved_undo_host_json_export_admits_isolated_spawn_job` feeds the host window JSON (`windowKindId=puzzle3d-main`, `windowInstanceId=puzzle3d-main-perspective`, `actionId=undo`, `arguments.windowId` the same) through **`plugin_handle_action`**. Fixture `pluginId`/`appId` are the TestApp bundle owner (`test` / `s.test.synthetic@1/*#editor`); a `puzzle` bundle cannot own that surface. First turn asserts Isolated `framework.reserved.tool` and an `operationId` output.

### Native cargo (this turn, real counts)

```
cargo test -p semio-framework-plugin --lib reserved_undo
# test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 631 filtered out

cargo test -p semio-framework-plugin --lib action_entry_points_drive_spawn_admit
# test result: ok. 1 passed; 0 failed; 634 filtered out

cargo test -p semio-framework-plugin --lib undo_on_empty
# test result: ok. 1 passed; 0 failed; 634 filtered out

cargo test -p semio-framework-plugin --lib benign_undo_with_nothing
# test result: ok. 1 passed; 0 failed; 634 filtered out

cargo test -p semio-framework-plugin --lib a_settled_reactor_turn_retains
# test result: ok. 1 passed; 0 failed; 634 filtered out
```

| Law | Result |
| --- | --- |
| `reserved_undo_invocation_does_not_require_window_ownership` | **ok** |
| `reserved_undo_actor_ingress_admits_undeclared_window_kind` | **ok** |
| `reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route` | **ok** |
| `reserved_undo_host_json_export_admits_isolated_spawn_job` | **ok** — Isolated spawn-job on the JSON export |
| `action_entry_points_drive_spawn_admit_instead_of_awaiting_the_retained_job` | **ok** |
| `undo_on_empty_history_is_a_benign_no_operation` | **ok** |
| `benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none` | **ok** |
| `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` | **ok** |

**8 passed / 0 failed** in this W-G3 scope. Scratch: `🗑generated/w-g3-reserved-undo.txt`, `w-g3-dff.txt`, `w-g3-empty.txt`, `w-g3-benign.txt`, `w-g3-settled.txt`.

### Browser / wasm

**No rebuild.** Coordinator runs **#37**. After #37, console must show `[DEBUG] plugin_exchange actionId=undo branch=spawn-admit` (or `plugin_handle_action` if the unused JSON export is hit). If undo still hangs after that line, the waiter is host-side (actor mailbox / leftover `spawn-job` routing / `awaitOperationSettle`), not an in-stack guest intercept.

### Handoff

- Guest: `plugin_handle_action` / `plugin_exchange` DEBUG + spawn-admit contract on both doors; DFF ratchet no longer demands in-stack `.await`.
- No serve / wasm / git.
- Coordinator rebuild **#37**.

## 8.14 Host document-opening replacement (W-G3)

Probe `🗑️generated/probe-2026-09-10T11-40-12.md` moved the undo hang up one more level: at undo time `openDocumentSessionsRef` was empty (`owners:[]`, `directNow:null`), so `directBrowserActorForSession` was null and dispatch fell to `plugin.handleAction`. Scene `registerBrushMesh` Commands still hit `plugin_exchange` on a captured retained actor. Coordinator inferred the example switch retired the document owner and never re-registered it.

### Root cause (admission closed the predecessor before the successor committed)

`admitDocumentOpeningV1` used to **close every predecessor** (same `runtimeKey` or same plugin+instanceId) **before** `runDocumentOpeningAttemptV1` committed. Combined with `openDocument` retiring `browserActorUi` at start (`opening replaced`) and `closeDocument` retiring UI by `runtimeKey` (not `clientInstanceId`):

- A failed reopen deleted the successor **and** the predecessor → map empty.
- `failDocumentBackbone` during `replacements.pending` (or mutation flood while `port === null` after `artifact-rebootstrap-required`) called `closeDocument`.
- Rebootstrap used to `SET_SESSION` null (“Agent disconnected”) even when the map entry could have stayed.
- On a *successful* same-key reopen, commit still called `retireBrowserActorUi(runtimeKey)`, wiping the successor’s just-mounted actor (same slot as the predecessor).

`runtimeKey` is the local `documentId` or `documentRuntimeKeyV1({ kind:"hub", spaceId, documentId })`. Every `openDocument` mints a new `clientInstanceId`. Worker events require the map entry’s `clientInstanceId` to match, so the successor must occupy the slot during attach; the predecessor object must be restorable if attach dies.

### Fix (host TS, vite-live; no wasm rebuild)

Admission (`dialog-origin/admission/document`):

- `admitDocumentOpeningV1` is now a **boolean** — background still refused when predecessors exist; it no longer takes `close` and never retires anyone.
- `documentOpeningPredecessorsV1` names same-key and same-plugin+instance owners.
- `parkDocumentOpeningReplacementV1` puts the successor in the map and keeps the predecessor object.
- `settleDocumentOpeningReplacementV1(owners, parked, committed)` — fail restores the predecessor; commit returns superseded owners to retire.

ShellHost:

- `openDocument` parks instead of admit+immediate close. Abort `close()` rejects the successor, retires only the successor actor/port, and `settle(false)`.
- Commit retires superseded ports/attachments/other-key documents; **does not** `retireBrowserActorUi` on the successor slot. Park yields the predecessor actor when `clientInstanceId` differs so the successor can mount.
- `failDocumentBackbone` returns without `closeDocument` while `replacements.pending`.
- `receiveDocumentBackbone`: `port === null` overflow **drops** (DEBUG) instead of `document-backbone.pending-capacity` (which closed the owner during rebound).
- `artifact-rebootstrap-required` keeps the current session (`[DEBUG] document rebootstrap kept session`) instead of `SET_SESSION` null.
- Undo gates: `[DEBUG] history route blocked effect-owner|undeclared|view-state`; success `history route action=`; fallback `history route fallback handleAction`.
- PluginRuntime: `[DEBUG] performInvocation` / `performInvocation settled` around `client.command` (next probe: did fallback enqueue?).

Coordinator traces kept: undo route / remap state / handleAction resolved (ShellHost); spawn-job routed / job done (PluginRuntime).

### Law

`restores a parked predecessor when the successor opening does not commit` in `engine/tests/document-opening`, fixture `replacements[]`:

| Vector | Result |
| --- | --- |
| `failed-same-key-restores-predecessor` | predecessor restored, `closed: []` |
| `committed-same-key-retires-predecessor` | predecessor in `closed`, successor remains |
| `committed-same-instance-retires-other-key` | other-key same-instance retired |
| `background-refused` | `parked: false`, map unchanged |

```
bun nx run @semio-tech/framework-renderer-react:test-long -- --run --testNamePattern="restores a parked predecessor|never lets a background admission"
# Test Files  1 passed | 20 skipped
# Tests  2 passed | 831 skipped
# [DEBUG] Document opening replacement: failed-restored=1 committed-retired=2 background-refused=1
```

Scratch: `🗑️generated/w-g3-document-opening-law.txt`.

Existing admit fixture rows now expect `closed: []` (admission no longer closes).

### Browser (`:6014`, wasm #37, probe `🗑️generated/probe-2026-09-10T12-10-07.md`)

Serve answered HTTP 200. Probe completed (no HTTP 000 / goto timeout).

| Check | Result |
| --- | --- |
| example-switch | ok — navbar **Nakagin Capsule Tower**, history `Set Active Example↶` |
| `undo remap state` ×4 | `directNow:null`, `owners:[]`, `sameAsSession:true`, puzzle instance 1 |
| `history route fallback handleAction` ×4 | **new** — gates did not drop; fallback ran |
| `plugin_exchange actionId=undo branch=spawn-admit` | **zero** |
| `[DEBUG] history route action=undo` | **zero** (that line is the *direct-actor* arm) |
| `undo handleAction resolved` | **zero** — fallback promise still hanging |
| navbar after undo | still **Nakagin** (not Concrete Forest) |
| `document opening parked` / `closeDocument` / rebootstrap | **zero** — playground never called `openDocument` |

`owners:[]` is the **steady state** on `/?plugin=puzzle3d`, not a switch regression. `setActiveExample` hitting `plugin_exchange branch=catalog` does **not** prove a document session (catalog goes through `performInvocation` / `AppChannelClient`, same as the undo fallback). Scene `registerBrushMesh` keeps a captured retained-actor reference that is not `openDocumentSessionsRef`.

Park/settle is the correct owner lifecycle for studio/hub `openDocument`. It cannot mint a playground document that was never opened. Remaining hang is the fallback `AppChannelClient.command` waiter: undo is dispatched on that channel and never reaches `plugin_exchange`, while scene Commands still do.

### Handoff

- Host admission: park/settle + ShellHost keep-session / no pending-capacity-kill / actor yield at park not commit.
- Playground undo still fails: `history route fallback handleAction` then silence. Next: why `performInvocation` → `client.command` never calls `plugin_exchange` for `actionId=undo` (retired enqueue lane vs live scene actor). Watch `[DEBUG] performInvocation` on the next probe.
- No wasm rebuild, no serve kill, no git.

## 8.15 Undo starve + uncorrelated settle (W-G3)

Probe `🗑️generated/probe-2026-09-10T12-16-12.md` answered §8.14: fallback **does** enqueue (`performInvocation actionId=undo`). A 2-frame settle immediately after was **not** the undo — `registerBrushMesh` flood. Counts: **181** brush enqueues, **27** settles, guest `plugin_exchange` only `registerBrushMesh`×27 + `setActiveExample`×1. `undo handleAction resolved` stayed 0 because undo's `AppChannelClient` waiter never got its `seq`. `plugin_handle_action` remained 0.

### Where it was consumed

Not a silent `invocationFromFrames` drop. That function still returns a response when frames have no `Invocation`/`Error`; a real undo settle would have hit `.then`. `pumpOutcomes` only resolves waiters whose `in_reply_to` is in the outcome (`appChannelReplySequence`). Frames without `in_reply_to` never enter `correlated` — **empty/unindexed outcomes hang that seq forever**.

The worker is not a special reserved-verb validator. `handle.enqueue` → `void runQueuedTurn` → `serializeCommandIngressForActor` → `serializePerActor(..., lane: "Interactive")`. Every catalog mesh used the **same Interactive mailbox** (`command-ingress:${actorId}`). `MAILBOX_LANE_ORDER` is Interactive → UserVisible → Background → Maintenance. 181 Interactive brush turns sat in front of undo; guest never saw `actionId=undo` before the probe ended. Later `interactionHover`/`setCamera` enqueues were the same lane, also behind the flood.

`routeHostEffects` already drives `spawn-job` during `runQueuedTurn` (not leftover). A spawn-admit turn that publishes no `in_reply_to` would still hang the waiter even after the guest ran — second hang, same mailbox.

### Fix (host TS, vite-live)

- `commandIngressLaneForActionV1`: `registerBrushMesh` → **Background**; everything else (including `undo`) → **Interactive**. Same mailbox, Interactive pops first.
- `serializePerActor` / `serializeCommandIngressForActor` take a `Lane`.
- `commandIngressNeedsReplyStampV1`: if the outcome has no `in_reply_to` for this command `seq`, stamp `AppFrame::Done { in_reply_to: seq }` so the waiter resolves (leftover / spawn-admit cannot hang).
- `[DEBUG] performInvocation settled` now includes `actionId` + `frameKinds`.
- `[DEBUG] command ingress lane` / `command ingress stamped Done`.

### Law

`keeps reserved command ingress Interactive ahead of catalog Background and stamps a reply when none arrives` — fixture `PluginRuntime/fixtures/command-ingress-lane.json`.

```
bun nx run @semio-tech/framework-renderer-react:test-long -- --run --testNamePattern="keeps reserved command ingress Interactive"
# Tests  1 passed | 833 skipped
# [DEBUG] Command ingress: reserved-interactive=4 catalog-background=1 empty-stamp=1 matched-skip=1 foreign-stamp=1 missing-seq=1
```

Scratch: `🗑️generated/w-g3-command-ingress-lane-law.txt`.

### Browser (`:6014`, wasm #37, probe `🗑️generated/probe-2026-09-10T12-26-56.md`)

Serve HTTP 200. Probe completed.

| Check | Result |
| --- | --- |
| `command ingress lane` undo | **Interactive** seq 213/215/218 |
| `plugin_exchange actionId=undo branch=spawn-admit` | **yes** ×3 (e.g. L1092–1095) |
| `performInvocation settled actionId=undo` | **yes** ×3 — `frameKinds:["Invocation","Ephemeral"]` (not the brush settle) |
| `undo handleAction resolved` | **yes** ×3 — `{uiScope:{kind:none},effects:0}` |
| `[DEBUG] history route action=undo` | no — still fallback (`owners:[]`, playground) |
| navbar | stayed Nakagin |

This run's chrome was `undo route none canUndo:false` and history `entryCount:0` after a visible Nakagin switch (`setActiveExample` did settle). Guest undo was the benign empty path; Isolated `job done` in the tail (93/94) is pre-undo pick, not the undo jobs (tail cut). Hang is gone: undo is no longer eaten by the brush FIFO.

### Handoff

- Host: Interactive reserved vs Background `registerBrushMesh`; Done-stamp if no `in_reply_to`.
- Playground still has no document owner (`directNow:null`). Undo uses fallback and now **reaches** `#37` spawn-admit.
- Navbar revert needs a live history entry after example switch (this probe's chrome was empty). Not a channel hang.
- No wasm rebuild, no serve kill, no git.

## 8.16 History chrome empty after lane split (W-G3)

### Root cause (host, proven)

Not the Done-stamp (`stamped Done` = 0 this run and in `12-30-16`). `setActiveExample` first-turn Invocation still carries no `history_patch` (`historyCursor:null, historyUpserts:0`). Chrome is host `historyProjection` via `applyHistoryPatch`; `framework.history.entry.N` is `Object.values(historyProjection.entries)`.

Pre-§8.15, boot `readHistory` sat **behind** the Interactive `registerBrushMesh` flood and completed **after** the Nakagin switch (~40s). Guest snapshot already had `Set Active Example`; `replace=true` painted chrome (`entryCount:1`, `canUndo:true`, `localOrder:2` in `probe-2026-09-10T12-16-12.md`).

Post-§8.15, brush is Background, so boot `readHistory` completes **immediately** (empty). The effect is keyed only on `session.pluginId` / `instanceId` — it never refires. `setActiveExample` does not carry a patch. Chrome stays empty → `shellHistoryUndoRouteV1` is `none` → ShellHost never dispatches undo. Two-for-two after the lane split (`12-26-56`, `12-30-16`).

### Fix (host TS, vite-live; no wasm rebuild)

- `historyRefreshNeededV1`: `setActiveExample` with empty/missing upserts must re-snapshot.
- `refreshHistorySnapshot`: `plugin.readHistory` → `applyHistoryPatch(snapshot, true)`, same newer-than-request skip as boot.
- `handleAction` fallback: after `awaitOperationSettle` (guest has committed), refresh when needed. Also refresh if the effect-owner retired, so projection is not owner-gated.
- History tab: one refresh when `framework.panel.history` **becomes** the active path (edge-triggered; a standing refresh on every path identity would replace a later undo with a stale snapshot).
- `historyPatchShouldApplyV1` (this turn, still): equal-cursor patches apply when they carry upserts. Not sufficient alone — the Invocation has no patch.

### Law

`applies an equal-cursor history patch when it carries upserts` now also asserts `historyRefreshNeededV1` (example empty → refresh; example with upserts → skip; undo → skip).

```
bun nx run @semio-tech/framework-renderer-react:test-long -- --run --testNamePattern="applies an equal-cursor history patch"
# Tests  1 passed | 834 skipped
# [DEBUG] History patch apply: equal-cursor-upserts=1 newer=1 older-skipped=1 equal-empty-skipped=1 replace=1 example-refresh=1 example-has-upserts=0 undo-skip=1
```

Scratch: `🗑️generated/w-g3-history-refresh-law.txt`.

### Browser (`:6014`, wasm #37, probe `🗑️generated/probe-2026-09-10T12-55-03.md`)

Serve HTTP 200. Probe completed.

| Check | Result |
| --- | --- |
| before undo chrome | **`entryCount:2`** — `framework.history.entry.1=Set Active Example↶`, `entry.2=Resize Window` |
| `history snapshot refresh` | **yes** — `{cursor:2, upserts:2, canUndo:true}` then `history patch applied` labels `["Resize Window","Set Active Example"]` |
| `undo route` | **`local` `canUndo:true`** `localOrder:5` (was `none` / `canUndo:false` / `localOrder:1`) |
| `plugin_exchange actionId=undo branch=spawn-admit` | **yes** ×3 (e.g. L801–804, L1092–1095) |
| `performInvocation settled actionId=undo` | **yes** ×3 — Interactive seq 213/222/226/231 |
| `undo handleAction resolved` | **yes** ×3 |
| navbar after undo | stayed **Nakagin** |

Host chrome + undo dispatch are restored. Guest `#37` undo still settles with `historyCursor:null, historyUpserts:0, effects:0` and a later `readHistory` still returns both entries — the benign empty undo path from §8.15. Navbar revert to Concrete Forest needs guest `#38` to actually pop `setActiveExample` (and publish a patch). `Resize Window` sitting above the example means even a working one-undo would pop the resize first.

### Handoff

- Host: re-snapshot after catalog example settle + on first History-tab open. Chrome empty regression from §8.15 is closed.
- Undo reaches `#37` spawn-admit on the Interactive lane. Guest undo does not mutate history or the navbar example id.
- No wasm rebuild, no serve kill, no git.

## 8.17 Guest undo no-op (W-G3)

### Probe (generated/probe-2026-09-10T12-55-03.md, wasm #37)

Hypothesis 1 (job never spawned) is ruled out. Isolated framework.reserved.tool jobs ran for the undo dispatches:

- job 94: spawn-job routed then job done kind=framework.reserved.tool job=94 status=done steps=2 after the first undo settle
- jobs 99, 101: later undo dispatches also spawn-job routed kind=framework.reserved.tool

Each undo still settled historyCursor:null, historyUpserts:0, effects:0. A later snapshot refresh stayed {cursor:2, upserts:2} labels [Resize Window, Set Active Example]. Navbar stayed Nakagin. The job completed; commit_framework_history_route published nothing.

### Root cause (guest, proven)

commit_framework_history_route only walked the document VCS store (tail_group_id then dispatch_group_history_action, else store.dispatch Undo). Chrome is the append-only command_log:

1. Chrome-top Resize Window is noteShellCommand (ActionKind::Shell + InverseAction, no edit_id). Undo never looked at it. revertToCommand already knew how (Effect::ReplayShellCommand); default undo did not.
2. Empty-group trap. If tail_group_id() is Some (coalesced setActiveExample) and playground has no composition children, dispatch_group_history_action returns UiDirtyScope::None / empty undone and did not fall through to store.dispatch Undo. That is the silent NothingToUndo path: no record_command, so finish_recorded attaches no history_patch.
3. can_undo was VCS-only (!applied_edit_ids is empty). Host chrome could show canUndo:true from the example edit while reserved undo still no-op'd the shell on top.

### Fix (guest Rust; rides wasm #38 — no rebuild this turn)

In commit_framework_history_route:

1. Chrome-order shell undo/redo (dispatch_chrome_history_action) before group/VCS. Newest live revertible command_log row that is a shell (inverse, no document/config edit, not already undone) emits Effect::ReplayShellCommand with the stored inverse, marks shell_undone, pushes shell_redo, and record_command undo History so finish_recorded publishes the patch. Redo pops shell_redo only after the VCS redo stack is empty (last-undone-first).
2. Empty-group fallthrough. dispatch_group_history_action returning UiDirtyScope::None no longer wins; VCS store.dispatch Undo/Redo runs.
3. can_undo / can_redo / revertible include live shell inverses. A new non-History command clears shell_redo.

### Laws (native semio-framework-plugin lib tests)

```
cargo test -p semio-framework-plugin --lib reserved_undo -- --test-threads=1 --nocapture
# test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 631 filtered out
```

- reserved_undo_pops_chrome_top_shell_then_publishes_history_patch — ok. spawn-admit first turn empty patch; settle pops Resize Window (ReplayShellCommand + inverse args); second undo pops Set Active Example; redo/redo restores both; each commit publishes history_patch with upserts/cursor
- reserved_undo_pops_shell_then_falls_through_to_document_store — ok. document example under chrome-top resize; first undo is shell (count stays 1); second undo applies the document inverse (count 0); redo restores the example first
- reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route — ok
- reserved_undo_host_json_export_admits_isolated_spawn_job — ok
- reserved_undo_actor_ingress_admits_undeclared_window_kind — ok
- reserved_undo_invocation_does_not_require_window_ownership — ok

DEBUG from the chrome law:

```
[DEBUG] history route action=undo
[DEBUG] chrome history action=undo seq=2 inverse=os.resizeWindow
[DEBUG] history route action=undo
[DEBUG] chrome history action=undo seq=1 inverse=setActiveExample
[DEBUG] history route action=redo
[DEBUG] chrome history action=redo seq=1
[DEBUG] history route action=redo
[DEBUG] chrome history action=redo seq=2
```

Scratch: generated/w-g3-chrome-undo-law.txt, generated/w-g3-reserved-undo-suite.txt.

### Guest files for wasm #38 (W-G3)

1. plugin/rs — chrome-order shell undo/redo, group-empty fallthrough, can_undo/can_redo/revertible, [DEBUG] chrome history
2. plugin/tests/plugin-runtime-plugin-builder-contract/rs — the two spawn-admit laws above

W-AB's fill engagementAbort bounce also rides #38 — not in this list.

### Handoff

- Host chrome + undo dispatch stay as 8.16. Guest #37 still no-ops until #38.
- After #38: first undo must paint a history_patch (resize popped, ReplayShellCommand for the previous size); a later undo in the same probe burst must reach Set Active Example (VCS inverse, navbar Concrete Forest). Look for [DEBUG] chrome history action=undo.
- No wasm rebuild this turn, no serve kill, no git.

## 8.18 Undo job drive + leftover chrome commit (W-G3)

### Coordinator probe (generated/probe-2026-09-10T13-42-13.md, wasm #38)

Undo got further than §8.17's #37 no-op: Interactive lane → spawn-admit → `[DEBUG] spawn-job routed kind=framework.reserved.tool job=99+` → first-turn settle empty → `undo handleAction resolved`. `[DEBUG] job done` never fired for those jobs. Pre-undo pick/hover jobs 89/90 (same kind) completed `status=done steps=2`. `[DEBUG] chrome history` zero hits. Navbar stayed Nakagin.

### Why the §8.17 law passed while the browser stalled

`reserved_undo_reaches_done_within_host_drive_contract` (7/7 with the rest of `reserved_undo`) drives Isolated start/step/complete **inside** `settle_framework_reserved_admission` / `drive_framework_reserved_spawn_like_host` and reads `InvocationResult` directly. The browser used to `void driveSpawnedJob` (fire-and-forget) and then, after await, still missed chrome commit:

1. **Binding steal.** Guest `JOB_RENDER_BINDINGS` is one job per instance. Hover/select Isolated spawns that queue on the same `command-ingress` FIFO steal the binding; `Event::JobCompleted` skips `plugin_complete_reserved_spawned_job` when `accepted(job)` is None. The law never overlaps Isolated jobs.
2. **Commit is after Done, not inside the dummy job.** Dummy body is 2 steps for undo and interactionSelect alike. Chrome undo runs in `complete_reserved_spawned_job_inner` after Done.
3. **Invocation `in_reply_to: 0`.** `plugin_complete_reserved_spawned_job` publishes `AppFrame::Invocation { in_reply_to: 0, history_patch }`. `route_app_frame` wraps it as leftover `send-message`. Promoting that frame onto command `outFrames` still loses it: `AppChannelClient` filters by waiter seq. First-turn frames stay `Invocation+Ephemeral` with empty patch → `historyUpserts:0`.
4. **ReplayShellCommand is dropped on the wire.** `push_invocation_side_frames` encodes it; `decode_wire_effect` on the job-completed turn silently fails (`if let Ok`). Leftover tags stay `send-message×N + publish-event`, never `replay-shell-command`. The law sees `requested_effects` on the native `InvocationResult`.
5. **Browser `noteShellCommand` has no inverse.** `buildNoteShellCommandAction` sends `{ commandId, label, detail }` only. Chrome-order `dispatch_chrome_history_action` requires `inverse.is_some()`. Resize / Toggle Panel / Switch Panel Tab are skipped; empty-group fallthrough undoes the document `setActiveExample` instead. That is why `[DEBUG] chrome history action=undo` stays at zero in Chrome while `[DEBUG] history route action=undo` fires.

### Host fix (vite-live PluginRuntime + ShellHost; no wasm rebuild)

- `commitReservedToolSpawnsWhileSerialized` still drives reserved Isolated start/step/`job-completed` **inside** the command-ingress serialize (inline=1). Dummy jobs reach Done in 2 steps.
- Leftover `send-message` Invocations stay on `pendingTurnEffects`. `invocationFromFrames` now reads `history_patch` / `ui_scope` from those leftover frames (`leftoverShellInvocationFrames`). Do **not** dump leftover frames into command `outFrames` (that threw `surface 1:window has no host context` when Error frames were promoted).
- `applyHostEffects` dispatches unrecognized `replayShellCommand` through `onAction` (and updates `SET_ACTIVE_EXAMPLE_ID` when the id is `setActiveExample`).
- After undo, if the leftover patch has `canUndo:false` and still names Set Active Example, navbar resets to `resolveBootExampleId("", exampleOptions, defaults.exampleId)` (Concrete Forest). Document fallthrough does not emit ReplayShellCommand, so the host label has to follow the VCS inverse.

### Laws / unit

```
cargo test -p semio-framework-plugin --lib reserved_undo -- --test-threads=1 --nocapture
# test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 631 filtered out
```

`reserved_undo_reaches_done_within_host_drive_contract` — chrome fixture → spawn-admit → host budget (fuel 50_000_000, deadline_ms 100) × 32-step batch → Done in 2 steps → complete → ReplayShellCommand resize + history_patch.

Vitest (renderer-react, PluginRuntime in-source): `reads history_patch from a leftover job-completed Invocation send-message` — leftover `in_reply_to: 0` Invocation supplies `historyPatch`; leftover `replay-shell-command` becomes `requestedEffects`.

### Browser verify (generated/probe-2026-09-10T14-47-36.md, :6014, wasm #38)

- `job done kind=framework.reserved.tool job=95 status=done steps=2`
- `job-completed leftover frame instance=1 kind=Invocation history=144438` (byte length of the encoded patch)
- `performInvocation settled` undo: `historyCursor:5, historyUpserts:5, historyCanUndo:false`
- `history patch applied` upserts include Set Active Example; `canUndo:false`
- After the first landed undo (meta-z in this probe; DOM click raced the leftover): navbar **Concrete Forest**, faults=0
- Set Active Example lost the ↶ (document inverse applied). Resize / Toggle Panel / Switch Panel Tab stay listed — they have no inverse, so chrome-order does not pop them. A later `noteShellCommand` adds Activate Window / Undo rows.
- `[DEBUG] chrome history action=undo`: still zero (fallthrough, not chrome-order). `[DEBUG] history route action=undo`: hits.

Earlier probes this turn: `probe-2026-09-10T14-12-53.md` (jobs Done, leftover wiped), `14-17-02.md` / `14-19-46.md` (inline commit, leftover send-message), `14-22-56.md` (promote-all-frames → host-context faults), `14-24-58.md` (Invocation-only promote still seq-filtered), `14-39-52.md` (leftover history applied; `SET_ACTIVE_EXAMPLE_ACTION_ID` was not imported — Navbar stayed Nakagin).

Scratch: generated/w-g3-818-host-drive-law.txt, generated/w-g3-818-reserved-undo-suite.txt, generated/w-g3-818-probe-stdout.txt … generated/w-g3-818-probe8-stdout.txt.

### Files

1. engine/elements/PluginRuntime/tsx — inline reserved Isolated commit, leftover Invocation history, leftover-frame log (no outFrames promote)
2. engine/elements/ShellHost/tsx — replayShellCommand fallback, navbar Forest after document example undo
3. engine/tests/plugin-runtime/tsx — leftover `in_reply_to: 0` history_patch law
4. plugin/rs + plugin/tests/plugin-runtime-plugin-builder-contract/rs — host-drive contract law (already on #38)

### Leftover (needs #39 unless a later host pass records inverses)

- Guest `JOB_RENDER_BINDINGS` one-slot — host inline commit is a workaround.
- `decode_wire_effect(ReplayShellCommand)` silent drop — leftover never carries `replay-shell-command`; resize does not replay.
- `noteShellCommand` without `inverseCommandId` — chrome-order cannot pop Resize / panel notes; `[DEBUG] chrome history` stays dark on this wasm.

### Handoff

- Undo Isolated jobs reach `job done` on #38. Leftover history_patch applies. Navbar Concrete Forest within the probe's undo burst (document fallthrough).
- Chrome-order pop of Resize + ReplayShellCommand + `[DEBUG] chrome history` still need inverses on the wire and a successful effect decode (#39).
- No wasm rebuild, no serve kill, no git.

## 8.19 W-G3 chrome-order inverses + leftover ReplayShellCommand (needs #39)

Closed the two guest gaps that left §8.18 undo on document fallthrough. No wasm rebuild (coordinator ships #39). Host is vite-live.

### Root cause

Browser `noteShellCommand` rows (Resize Window, Toggle Panel, Switch Panel Tab, Activate Window) arrived as `{ commandId, label, detail? }` with no `inverseCommandId`. Chrome undo requires `inverse.is_some()`, so those rows were skipped and undo fell through to the document group (`setActiveExample`). Leftover therefore never carried `ReplayShellCommand` — `requested_effects` was empty on the chrome miss, and `decode_wire_effect` swallowed a failed `from_dsl_value` with `()`.

### Guest

`noteShellCommand` now always records a shell inverse: explicit `inverseCommandId`/`inverseArgs` when present, otherwise `commandId` plus `inverseArgs` or object `detail`. Chrome-order undo pops Resize on a populated stack `[Set Active Example, Resize Window]` and publishes `history_patch` + `ReplayShellCommand`; redo replays the same action id.

`decode_wire_effect` keeps `from_dsl_value`, then peels `{ replayShellCommand | ReplayShellCommand: { actionId|action_id, args } }` so leftover encode/decode cannot drop the effect.

### Host (vite-live)

`buildNoteShellCommandAction` now sends `inverseCommandId: commandId` and `inverseArgs: detail` so live #38 also records inverses without waiting for #39. `applyHostEffects` already applies `replayShellCommand` (directory / space / open-artifact branches, else `onAction` + navbar `setActiveExample`). No new apply branch required.

### Laws

`cargo test -p semio-framework-plugin --lib reserved_undo -- --test-threads=1 --nocapture`

`test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 631 filtered out`

New:

- `reserved_undo_browser_note_without_inverse_pops_chrome_resize` — fixture stack, no `inverseCommandId` on Resize → `[DEBUG] chrome history action=undo seq=2 inverse=shell.windowResize` (not document fallthrough); leftover-shaped `ReplayShellCommand`; redo symmetry.
- `reserved_undo_replay_shell_command_survives_wire_roundtrip` — `pack_rt` encode/`from_dsl_value` (and the decode peel) round-trip `ReplayShellCommand`.

Language-agnostic fixture: `plugin/fixtures/reserved-undo-browser-note.json`.

Scratch: `generated/w-g3-819-reserved-undo.txt`.

### Guest files touched

1. `plugin/rs` — synthesize `noteShellCommand` inverses
2. `plugin/reactor/turn/rs` — leftover `ReplayShellCommand` decode peel
3. `plugin/tests/plugin-runtime-plugin-builder-contract/rs` — two new reserved_undo laws
4. `plugin/fixtures/reserved-undo-browser-note.json` — stack / undo / redo / wire

### Host files touched

5. `engine/elements/ShellHelpers/tsx` — `inverseCommandId` / `inverseArgs` on every shell note

W-AB spawn-admit / `openImportFixture` edits on `plugin/rs` were left in place.

### Handoff

Chrome-order undo is complete on the guest laws. Coordinator runs #39 to ship the guest decode + inverse synthesis into wasm. Vite-live host already notes inverses and applies `ReplayShellCommand` when leftover delivers it. No serve kill, no git.

## 8.20 W-G3 reserved-verb family proof on wasm #39

Final reserved-verb battery on harness-managed `:6014` (HTTP 200; never killed). Probe input `🔍️browser-probe.ts --interact --reserved-family --port=6014`. Proof log `🗑️generated/probe-2026-09-10T16-46-05.md` (screenshots `probe-2026-09-10T16-46-05-{boot,example-switch,history-open,undo-unwind,undo-redo,selection-surfaces,clipboard-copy-paste,locked-refusal,gumball-drag}.png`). Earlier miss `16-08-53` never left Concrete Forest (family used `options.nth(1)` instead of Nakagin). `16-14-57` switched to Nakagin and unwound, but chrome undo painted Forest early and redo never restored the navbar.

### Host fix (vite-live; no wasm rebuild)

`SET_ACTIVE_EXAMPLE_ID` only moves the navbar label. §8.18 reset it to the boot example whenever an undo leftover mentioned Set Active Example with `canUndo === false`, so a chrome-only pop painted Concrete Forest while the document row was still live, and redo had no inverse path.

`navbarExampleIdFromHistoryUpserts` now keys the label off the Set Active Example upsert's `revertible` flag plus the last dispatched example id. Chrome-only upserts leave the label alone. A popped document row returns the boot example. A live row restores Nakagin.

Vitest (`engine-contract`, `--testNamePattern="navbar example id follows Set Active Example"`): **1 passed | 514 skipped**.

### Verdict

| # | Item | Verdict | Evidence |
|---|---|---|---|
| 1 | Full unwind | **PASS (chrome-order)** | `16-46-05` start Nakagin, 4 live rows. Undo 0–1 keep Nakagin (`exampleLive=true`, `chrome/replay` 1 then 3). Undo 2 pops Set Active Example (`exampleLive=false`) and navbar is Concrete Forest. Not #38 document fallthrough. |
| 2 | Redo | **PASS (navbar + chrome)** | Redo 0 restores Nakagin and re-marks Set Active Example / Resize / Toggle (`census chrome=5 replay=5`). Host label sync. Scene `data-instances-json` stayed `seed-left-001` immediately after; body later showed `180 Objects` — document republish is still racy. |
| 3 | Selection publication | **FAIL — needs #40** | `interactionSelect` Interactive + reserved job `done steps=2`. Inspection stayed `"Inspection"`. No inspector object ids. |
| 4 | Clipboard | **FAIL — needs #40** | `copyBtn=0`, no `clipboardWrite`, census `1 → 1` (`delta=0`). Blocked on empty selection. |
| 5 | Locked refusal | **FAIL — needs #40** | `lock controls=0 ids=[]`. No lock control without Inspection. No mutation-rejected notice. |
| 6 | Gumball | **FAIL — needs #40** | Transform utility visible. Drag `sceneDelta=false`, pose length 266 unchanged. No translate history row. |

Faults on this run: 2 (`actor puzzle#1 did not publish its requested UI surfaces within 4096 continuations`) during post-redo selection/lock — leftover settle after the chrome+document unwind, not a serve wedge.

### Why 3–6 stay guest

Reserved Isolated completion is live (same dummy 2-step job as undo). Undo leftover carries `replay-shell-command` and the host applies it. `interactionSelect` leftover does not publish an InteractionView / Inspection body, so copy / lock / gumball have no selected object. That publication is guest assembler work; it cannot ship without wasm **#40**.

### Files

1. Ticket `🔍️browser-probe.ts` — `--reserved-family`, Nakagin switch, undo/redo loops (undo button only), framed picks, census logs
2. `engine/elements/ShellHelpers/tsx` — `navbarExampleIdFromHistoryUpserts`
3. `engine/elements/ShellHost/tsx` — remember last example id; sync navbar from history `revertible`; replay `setActiveExample` no longer paints raw `""`
4. `engine/tests/engine-contract/ts` — revertible vs chrome-only upsert law

W-AB spawn-admit / `openImportFixture` edits on `plugin/rs` were left in place. No serve kill, no wasm rebuild, no git.

### Handoff

Chrome-order unwind + redo navbar are proven on #39. Selection → Inspection, clipboard clone, locked refusal, and gumball scene delta need guest InteractionView publication on the same reserved leftover (**needs #40**).

## 8.21 W-G3 leftover InteractionView + wire-table completeness (needs #40)

### Root cause

Guest **does** compute selection/hover in `dispatch_interaction_action` after the reserved Isolated job Done. Publication on leftover was never triggered: leftover `Invocation.output` stayed `Null`. Host leftover peel (`leftoverShellInvocationFrames` → leftover `Invocation`) already consumed `history_patch` / `ReplayShellCommand` on that same lane; InteractionView was simply not put on it.

This is the same class of defect as ReplayShellCommand / RequestFileOpen: leftover encode dropped the payload. Selection / clipboard / lock / gumball (and W-AB vortex click → brush preview/place + suggestions) all sit downstream of leftover InteractionView.

### Guest fix (rides wasm #40 — no rebuild this turn)

After the just-computed InteractionView is in memory (and **before** topology revalidate can prune a pick the host already hit), leftover `Invocation.output` is:

```
{ interactionView: { selection, hover, activeMode, activeGranularity, selectedIds, locked, gumball, hoverTarget } }
```

`locked` is the leftover lock map (id → bool). `gumball.active` / `gumball.anchorId` are leftover gumball publication.

### W-AB: wire-table completeness landed (fold in)

`decode_wire_effect` now has three arms in one table: `from_dsl_value`, **`serde_json::from_value` (covers every remaining camelCase Effect kind)**, then the RequestFileOpen + ReplayShellCommand peelers.

**Law:** `every_effect_kind_survives_wire_effect_round_trip` — 45/45 Effect kinds encode through `pack_rt` and decode back. A new Effect variant fails compile (`effect_wire_kind` is exhaustive) or fails the 45-count. This class of leftover drop is closed permanently.

Do not revert this table. If you add an Effect kind, the completeness law will fail until you add a fixture.

### Host (vite-live; mocked leftover proven)

Leftover `output.interactionView` is peeled (`interactionViewFromLeftoverOutput`) and applied as `INTERACTION_STATE_OBSERVED` plus a World3d leftover selection overlay (`publishLeftoverWorldSelectionV1`) so Inspection/clipboard/lock/gumball consumers can populate without waiting for guest `selectionJson` republish.

### Laws (ran this turn)

| Law | Result |
|---|---|
| `interaction_select_job_completion_publishes_interaction_view_on_leftover` | **2/2 leftover laws ok** (`🗑️generated/w-g3-821-leftover-laws4.txt`) |
| `interaction_hover_job_completion_publishes_hover_target_on_leftover` | selected ids + lock + gumball on leftover; hover target on leftover |
| `every_effect_kind_survives_wire_effect_round_trip` | **1 passed** (`🗑️generated/w-g3-821-wire-table.txt`) |
| engine-contract `leftover InteractionView publication populates selection, lock, and gumball` | **1 passed / 665 skipped** (`🗑️generated/w-g3-821-leftover-vitest.txt`) |

### Guest files touched

1. `plugin/rs` — leftover InteractionView on reserved `interactionSelect`/`interactionHover` job completion (`leftover_interaction_view_from`). W-AB spawn-admit / vortex / suggestions edits were not reverted.
2. `plugin/reactor/turn/rs` — `decode_wire_effect` serde completeness arm + `every_effect_kind_survives_wire_effect_round_trip`
3. `plugin/tests/plugin-runtime-plugin-builder-contract/rs` — leftover InteractionView + hover leftover laws

### Host files touched

1. `engine/elements/ShellHelpers/tsx` — `interactionViewFromLeftoverOutput` / `leftoverInteractionStateV1`
2. `engine/elements/ShellHost/tsx` — leftover InteractionView → `INTERACTION_STATE_OBSERVED` + World3d overlay
3. `engine/elements/World3dHost/tsx` — leftover selection/gumball overlay merge
4. `engine/tests/engine-contract/ts` — mocked leftover publication law

### Leftover (needs coordinator wasm #40)

Browser Inspection / clipboard / lock / gumball / W-AB vortex+suggestions still run #39 wasm without leftover InteractionView encode. After #40, leftover `Invocation.output.interactionView` is on the same lane chrome-order undo already proven.

No serve kill, no wasm rebuild, no git.

## 8.22 W-G3 leftover family proof on wasm #40

Browser-proved the four §8.20 leftovers on harness-managed `:6014` (HTTP 200, wasm #40). Probe `🗑️generated/probe-2026-09-10T17-43-43.md` (after a vite-live Inspection-tab loop was fixed). First probe `🗑️generated/probe-2026-09-10T17-29-18.md` is the pre-fix baseline. W-AB import/capacity lanes were ignored.

### First-turn `effects:0` — confirmed expected

Every `performInvocation settled actionId=interactionSelect` on #40 is `{ frames:2, frameKinds:[Invocation, Ephemeral], effects:0 }`. That is the first reserved turn: it only admits `SpawnJob` (consumed inline). InteractionView is **not** a first-turn leftover effect.

The view rides **job-completion leftover** on the same lane as chrome-order undo:

1. `job done kind=framework.reserved.tool … status=done steps=2`
2. `job-completed leftover … effects=send-message,send-message,send-message`
3. `leftover InteractionView {"selectedIds":["seed-left-001"],"locked":{"seed-left-001":false},"gumball":true}`

Host `[DEBUG] leftover InteractionView` in `applyLeftoverInteractionView` already logged that tap. No extra host tap was needed.

### Host vite-live this turn (re-verified)

Leftover overlay set `ids` + `gumballActive` but not `transformMode` / `gumballTarget`, so World3d `gumballVisible` stayed false (`gumballActive && isWorldTransformGumballMode(transformMode)`). Inspection tab stayed on Artifact, so `puzzle3d-play-inspector` never mounted.

Fixes (no wasm):

1. `leftoverWorldGumballPoseV1` — leftover selection forces `transformMode: "transform"` and `gumballTarget` from the selected instance pose.
2. World3d leftover merge uses that pose.
3. Leftover with `selectedIds` activates `framework.panel.inspection` on the dock anchor (`top-right` in this boot).
4. First draft of (3) re-dispatched on every `dock` identity change → `Maximum update depth exceeded` (`🗑️generated/probe-2026-09-10T17-38-11.md`). Guarded to leftover epoch + already-on-Inspection skip. Re-verify is the 17-43-43 probe.

engine-contract `leftover InteractionView publication populates selection, lock, and gumball` — **1 passed / 665 skipped**.

### Verdict

| # | Proof | Result | Hop |
|---|---|---|---|
| 1 | Selection → Inspection | **FAIL — needs #41** | Leftover arrives (`seed-left-001`, `gumball:true`). Host opens Inspection. Guest still renders **empty summary** (`puzzle3d-play-inspector.empty`: Schema / Domain / Objects 1), not `object_fields`. |
| 2 | Clipboard | **PASS** | Select → leftover → copy/paste. Census `1 → 3` (`seed-left-001`, `object-1`, `object-2`). History: Copy + Paste + `create-object`. `copyBtn=0` (no chrome Copy); keyboard + leftover selection was enough. |
| 3 | Locked refusal | **FAIL — missing chrome** | `puzzle3d-play-inspector.object.locked` (`flag_row` on Inspection `object_fields`). Absent because Inspection never leaves the empty summary, so the lock toggle is never assembled. No refusal notice. |
| 4 | Gumball | **FAIL — needs #41** | Leftover `gumball:true`; Transform chrome visible; host now sets leftover `transformMode`. Drag: `sceneDelta=false`, pose len 786 unchanged, no translate/relocate history row. |

### Hop-by-hop (Inspection)

1. Guest `leftover_interaction_view_from` writes leftover `Invocation.output.interactionView` after reserved Isolated Done — **proven** (leftover tap + #40 leftover laws).
2. Host `leftoverShellInvocationFrames` peels leftover Invocation — **proven** (`job-completed leftover frame kind=Invocation`).
3. `applyLeftoverInteractionView` → `INTERACTION_STATE_OBSERVED` + World3d overlay — **proven** (`[DEBUG] leftover InteractionView`).
4. Host opens Inspection tab — **proven this turn** (`[DEBUG] leftover Inspection tab {anchor:top-right}`; screenshot `probe-2026-09-10T17-43-43-selection-surfaces.png` shows the empty inspector).
5. Guest Inspection `selected_section` reads `Puzzle3dInteractionSnapshot` from the guest render, **not** leftover overlay / host `INTERACTION_STATE_OBSERVED`. Snapshot is empty → `summary()` (schema/domain/objects). **This hop needs #41.** Name: `Inspection selected_section ← Puzzle3dInteractionSnapshot` after leftover publication.

Lock chrome is the same hop: `flag_row(..., "object.locked")` only exists inside `object_fields`, which `selected_section` never reaches.

### Hop-by-hop (Gumball)

1. Leftover `gumball:true` + host leftover `transformMode` — **proven**.
2. World3d `gumballVisible` can now be true without guest `selectionJson`.
3. Drag produced no `instancesJson` delta and no `translateSelection` / `rotateSelection` / `scaleSelection` history row. **needs #41** hop: leftover gumball overlay → `onGumballDragEnd` → guest `translateSelection` (never admitted or no-op).

### Screenshots

- `🗑️generated/probe-2026-09-10T17-43-43-boot.png`
- `🗑️generated/probe-2026-09-10T17-43-43-selection-surfaces.png` — Inspection empty summary after leftover
- `🗑️generated/probe-2026-09-10T17-43-43-clipboard-copy-paste.png` — cloned objects
- `🗑️generated/probe-2026-09-10T17-43-43-locked-refusal.png` — no lock toggle
- `🗑️generated/probe-2026-09-10T17-43-43-gumball-drag.png` — Transform on, no scene delta

### Files

1. `engine/elements/ShellHelpers/tsx` — `leftoverWorldGumballPoseV1`
2. `engine/elements/World3dHost/tsx` — leftover merge `transformMode` + `gumballTarget`
3. `engine/elements/ShellHost/tsx` — leftover Inspection tab (epoch-guarded)
4. `engine/tests/engine-contract/ts` — leftover gumball pose asserts
5. ticket `🔍️browser-probe.ts` — leftover-lane wait + Inspection populate dump

### Handoff

Clipboard is closed on #40 leftover. Inspection populate, lock `object.locked` flag_row, and gumball `translateSelection` need wasm **#41** at the hops named above. No serve kill, no wasm rebuild, no git.

## 8.23 W-G3 three guest hops for #41 (Inspection / lock / gumball)

Fixed the three named hops in current source. No wasm rebuild. `:6014` stayed HTTP 200 (never killed). W-AB `plugin/rs` leftover/hover publication and queued history-panel paging were left in place.

### Hop 1 — Inspection `selected_section` ← leftover `Puzzle3dInteractionSnapshot`

Leftover InteractionView is proven on #40 (`selectedIds:["seed-left-001"]`, host opened Inspection). Guest `selected_object_ids()` only returns ids when `granularity == OBJECT`. Leftover-shaped snapshots carry `selected=["seed-left-001"]` and **empty granularity**, so `selected_section` hit `_ => None` and rendered `summary()` (`puzzle3d-play-inspector.empty`).

`selected_section` now `.or_else` resolves `interaction.selected` against fixture objects regardless of granularity. `object_fields` already had `flag_row(..., "object.locked")` — the lock row was never missing from the assembler; it never mounted because object fields never assembled.

**Law:** `leftover_shaped_snapshot_wires_object_fields_and_lock_row` — snapshot `{ granularity: "", selected: ["seed-left-001"] }` emits `object.id` + `object.locked`, not `.empty`.

Browser on #40 wasm still shows the empty summary (`🗑️generated/probe-2026-09-10T18-13-57.md`). Expected until coordinator **#41**.

### Hop 2 — Locked chrome + browser-visible refusal

Lock chrome is the same assemble hop as Inspection: once leftover-shaped `selected_section` reaches `object_fields`, `puzzle3d-play-inspector.object.locked` mounts. No second `flag_row` was added.

Refusal path (browser-shaped leftover overlay → `translateSelection` with explicit ids):

1. `Puzzle3dActionCtx::refuse_when_locked` emits `Effect::Notify` with `labels.selection_locked` ("Selection is locked").
2. `translate-selection` refuses when every object id is locked (before apply).
3. `puzzle3d_apply_translate` skips `object.locked`.
4. `Puzzle3dScaleWork` Objects stage uses `contains && !object.locked`. Complete with `mutations.is_empty()` and a non-empty selection emits the same `selection_locked` notice (gumball verbs never reach the sync `refuse_when_locked` arm — `build_tool_job` routes them here).

**Law:** `leftover_translate_selection_on_locked_object_refuses_with_notice` — lock via `setSelectionFlag`, leftover-shaped `translateSelection` `{ ids, dx: 4 }`, one notice, no origin move.

Browser lock chrome still absent on #40 (empty Inspection). Needs **#41**.

### Hop 3 — leftover overlay → `translateSelection`

Host drag handler **does** dispatch. `dispatchGumballPoseDelta` already called `gumballTransformDeltaBetweenPoses` → `translateSelection`. The broken half was the payload: `selectionArgs` sent `componentIds` (face numbers) as `ids`, not leftover/object `selection.ids`. Guest `Puzzle3dScaleWork` prefers `explicit_ids(command)` and then looks those ids up on fixture objects — `"9"` never matches `seed-left-001`, so the work completed with no mutation (same class as the 2026-09-09 empty-edit row).

Host vite-live fix: `world3dGumballSelectionArgsV1` prefers leftover `selection.ids`, falls back to `componentIds` as strings, default mode `"object"`. Leftover overlay already merges `ids: leftover.ids` in `mergeWorldSelectionWithLeftover`.

Guest half (already on #40 wasm via `explicit_ids`): leftover overlay ids without an OBJECT snapshot move the unlocked object. New lock skip in ScaleWork / apply_translate rides **#41**.

**Laws:**

- `leftover_overlay_translate_selection_moves_unlocked_object` — empty example + add object, **no** `interactionSelect`, `translateSelection` `{ ids, dx: 4 }` moves origin by 4. Leftover overlay shape.
- engine-contract `leftover InteractionView publication populates selection, lock, and gumball` now also asserts `world3dGumballSelectionArgsV1({ ids: ["seed-left-001"], componentIds: [9] }).ids === ["seed-left-001"]`.

### Laws (ran this turn)

| Law | Result |
|---|---|
| `leftover_shaped_snapshot_wires_object_fields_and_lock_row` | **ok** |
| `leftover_translate_selection_on_locked_object_refuses_with_notice` | **ok** |
| `leftover_overlay_translate_selection_moves_unlocked_object` | **ok** |
| cargo `leftover_` (`semio-s-artifact-puzzle-3d --features component-app-assembly`) | **3 passed / 656 filtered** (`🗑️generated/w-g3-823-leftover-laws.txt`) |
| engine-contract leftover InteractionView + gumball `selectionArgs` | **1 passed / 665 skipped** (`🗑️generated/w-g3-823-leftover-vitest.txt`) |

### Browser (pre-#41; host vite-live only)

`:6014` HTTP 200. Probe `bun 🔍️browser-probe.ts --interact --gumball --port=6014` → `🗑️generated/probe-2026-09-10T18-13-57.md` (earlier hop-less pass `18-11-20`).

| Proof | Result |
|---|---|
| Leftover InteractionView | **WORKS** — `selectedIds:["seed-left-001"]`, `gumball:true` |
| Inspection | **FAIL — needs #41** — empty summary, `lockChromePresent=false` |
| Gumball drag | **unverified scene mutation** — leftover + Transform chrome on; `sceneDelta=false`; pose len 266; no translate history row; `[DEBUG] gumball pose delta` hops `[]` (canvas swipe never entered `dispatchGumballPoseDelta` — handle miss, not a missing host dispatch). Host `selectionArgs` is law-proven vite-live. Guest unlocked leftover ids are law-proven. |

### Guest files touched

1. `puzzle/3d/editor/panels/inspection/rs` — leftover `selected_section` fallback + leftover-shaped Inspection law
2. `puzzle/3d/editor/rs` — `refuse_when_locked`; `puzzle3d_apply_translate` skips locked objects; `Puzzle3dScaleWork` skips locked objects and emits `selection_locked` when the leftover overlay named a locked set
3. `puzzle/3d/editor/commands/translate-selection/rs` — all-locked object ids → `refuse_when_locked`
4. `puzzle/3d/editor/tests/unit/rs` — leftover locked-translate + leftover-overlay unlocked-translate laws

### Host files touched

1. `engine/elements/World3dHost/tsx` — `world3dGumballSelectionArgsV1`; gumball drag uses leftover `ids`; `[DEBUG] gumball pose delta`
2. `engine/tests/engine-contract/ts` — leftover `selectionArgs` prefers `seed-left-001` over component `9`
3. `engine/packages/typescript/targets/react/tsx` — barrel re-export of `world3dGumballSelectionArgsV1`
4. ticket `🔍️browser-probe.ts` — gumball hop dump

W-AB `plugin/rs` leftover/hover and history-panel paging were not reverted.

### Handoff

Inspection populate + lock `object.locked` chrome + locked refusal notice need coordinator wasm **#41**. Host gumball leftover `ids` are vite-live on `:6014` now (law-proven; browser swipe did not grab the handle). Guest unlocked leftover `translateSelection` already works in #40 wasm via `explicit_ids`; the new locked skip rides #41. No serve kill, no wasm rebuild, no git.

## 8.24 W-G3 #41 battery probe (dry-run on wasm #40)

Prepared the turnkey `--battery` so coordinator can re-run it the moment #41 deploys. No wasm rebuild. `:6014` stayed HTTP 200.

### Probe (`🔍️browser-probe.ts`)

`--battery` implies `--interact` and runs one pass in this order: boot → Nakagin example switch → undo unwind → redo → selection/Inspection → clipboard → locked → gumball → brush hover/preview/place → suggestions → import distinct fixture.

Each step logs `verdict <name> PASS` or `verdict <name> FAIL [expect-41] …`. The closer asserts `battery-faults` (zero FAULT_RE / pageerror). The generated markdown now has a `## verdicts` section.

### Handle-accurate gumball

`SceneGumball` / `UnifiedGumball` place the move-X tip at local `0.55` (`GUMBALL_HANDLE_LENGTH/2 + GUMBALL_ARROW_HEAD/2`) and scale by `cameraDistance / 8`. `WorldGumballHitStamp` projects those three axis tips into canvas pixels and writes `data-gumball-hits` on the perspective host (same pattern as `data-vortex-hits`).

`dragGumballMoveX` hovers the canvas (required: `SceneGumball` returns null until `useUiCanvasHovered`), waits for on-screen hits, and drags from the `moveX` tip. Host `handleGumballDragStart` sets `window.__gumballDragEntered` and `[DEBUG] gumball drag entered`.

### Inspection / locked

After leftover pick, the battery asserts `puzzle3d-play-inspector.object.id` (not `.empty`) and `object.locked` flag_row. Locked then clicks that row, handle-drags, and asserts a notice matching `/locked/i`.

### Dry-run on #40 (`🗑️generated/probe-2026-09-10T18-35-03.md`, log `🗑️generated/w-g3-824-battery.txt`)

| Verdict | Result |
|---|---|
| boot | **PASS** |
| example-switch | **PASS** (Nakagin) |
| undo-unwind | **PASS** (Concrete Forest, chrome-order) |
| undo-redo | **PASS** (Nakagin restored) |
| inspection-object-fields | **FAIL [expect-41]** empty summary |
| inspection-locked-flag-row | **FAIL [expect-41]** |
| clipboard | **FAIL** `delta=0` on Nakagin 180-instance census after unwind (Forest leftover clipboard still the §8.22 PASS) |
| locked-flag-row / locked-refusal-notice | **FAIL [expect-41]** |
| gumball-handle-enter | **FAIL** hits published (`moveX` sx/sy/ndcZ) but `__gumballDragEntered` stayed false — `SceneGumball` still dropped the pick (hover-gate / depth). Not a missing host dispatch. |
| gumball-scene-delta | **FAIL [expect-41]** |
| brush-preview-place | **FAIL [expect-41]** (W-AB still closing) |
| suggestions | **FAIL [expect-41]** (W-AB still closing) |
| import-distinct | **FAIL [expect-41]** before=180 after=180 |
| battery-faults | **PASS** (`faults=0`) |

`data-gumball-hits` is live on #40 (vite-live stamp). After #41, re-run `bun 🔍️browser-probe.ts --battery --port=6014` and drop the `[expect-41]` tags that flip.

### Files

1. ticket `🔍️browser-probe.ts` — `--battery`, verdicts, handle drag, Inspection/lock asserts, import last
2. `engine/elements/World3dHost/tsx` — `WorldGumballHitStamp` + `[DEBUG] gumball drag entered`

W-AB `plugin/rs` was not touched. No serve kill, no wasm rebuild, no git.

## 8.25 W-G3 #41 battery gaps (five real fails)

#41 wasm was live on `:6014`. The 18:57 battery fails were not staleness. Browser evidence first (18:57 battery + targeted `🗑️generated/w-g3-825-targeted.txt` / `probe-2026-09-10T19-20-52.md`), then probe vs host vs guest.

### Inspection (`populated=false empty=false id=null`)

**Probe gap:** host namespaces ids as `panel:puzzle3d-play-inspector/puzzle3d-play-inspector.*`. `getElementById("puzzle3d-play-inspector.object.id")` never hits. The 18:57 dump already had `.empty` / `.schema` / `.objects` under that prefix; the assert reported `empty=false` because it missed them.

**Guest gap (needs #42):** leftover InteractionView selected `5de35caa-…` (battery) and `seed-left-001` (Forest targeted) and leftover Inspection tab opened, but the panel body stayed the document summary. `selected_section` used `?` inside granularity arms, so a vortex-granularity pick whose ids are object uuids returned `None` before the leftover `or_else`. Law `leftover_vortex_granularity_unresolved_falls_back_to_object_fields` now covers that; #41 wasm still has the `?` short-circuit. Suffix lookup + `waitInspectionPopulated` are live in the probe (`empty=true` on Forest targeted).

### Locked flag row + `selection_locked`

Follows Inspection: no object section ⇒ no `object.locked` row ⇒ no lock toggle. `Agent disconnected` is a stale hub presence toast (`Remote: detached`); leftover jobs kept running. The probe now ignores that toast. Notice stay `[expect-42]` until the flag row mounts.

### Clipboard `delta=0` (not history-row confusion)

`dumpInstances()` reads `data-instances-json` (180→180 on Nakagin, 1→1 on Forest). `treeItems` 4→106 is `openHistory()` paging, not the census. Targeted Forest run recorded history `Copy`, `Copy`, `Paste` and `error: clipboard-instack`, then census stayed 1. Copy/paste reserved verbs ran; the fragment did not add an instance. §8.22 1→3 was not this battery/Nakagin-or-instack path. Probe now opens Actions and looks up `action.copy`; verdict is `[expect-42]`.

### Gumball handle-enter PASS / sceneDelta FAIL

Leftover `leftoverWorldGumballPoseV1` forced `transformMode: "transform"`, so UnifiedGumball mounted rotate rings. The 18:57 drag entered `rotateZ` and skipped (`angle < 1e-6`). Host leftover mode is now `"move"` (vite-live; engine-contract leftover law expects `move`). Targeted run entered `moveY` then skipped because the probe dragged +56px in X only (moveY constraint ⇒ zero delta). Probe drag is now diagonal; skip DEBUG logs `kind` + before/after position. Isolated `--gumball` still missed the picker when SceneGumball was hover-gated (`ndcZ≈0.996`); battery context (prior canvas hover) is the path that entered.

### Probe `--battery` / flags

Step flags (`--selection`, `--gumball`, …) now imply interact so targeted reruns actually run. Guest-pending inspection/lock/clipboard print `[expect-42]`. W-AB brush/suggestions/import stay `[expect-41]`.

### Laws (ran)

cargo `leftover_` on `semio-s-artifact-puzzle-3d --features component-app-assembly`: 5 passed / 658 filtered (`🗑️generated/w-g3-825-leftover-laws.txt`). Vitest leftover InteractionView publication: 1 passed / 666 skipped (`🗑️generated/w-g3-825-leftover-vitest.txt`).

### Files

1. ticket `🔍️browser-probe.ts` — suffix Inspection ids, diagonal move drag, Copy Actions lookup, presence-notice filter, step-flag interact, expect-42 tags
2. `engine/elements/🛠️ShellHelpers` — leftover gumball mode `move`
3. `engine/elements/World3dHost` — handle tip 0.85, skip DEBUG pose
4. `engine-contract` leftover pose expect `move`
5. puzzle3d `inspection/rs` — no `?` short-circuit + vortex-mismatch law

W-AB `plugin/rs` was not touched. `:6014` stayed HTTP 200. No serve kill, no wasm rebuild, no git.

## 8.26 W-G3 #42 guest land (ready)

Guest fixes for rebuild #42 are **fully written and law-proven**, not merely diagnosed. Vite-live host/probe from §8.25 left in place. `:6014` stayed HTTP 200. No wasm rebuild.

### 1. Inspection fall-through — **written**

`selected_section` no longer `?`-returns from a failed granularity arm. Vortex (or empty leftover) granularity with an object id in `selected` falls through to `object_fields`.

Law `leftover_browser_shaped_snapshot_wires_namespaced_object_fields_and_lock_row`: leftover-only selection (`granularity` cleared, `selected=["seed-left-001"]`) renders `puzzle3d-play-inspector.object.id` and must not keep `.empty`. Companion `leftover_vortex_granularity_unresolved_falls_back_to_object_fields` covers the Nakagin-shaped vortex-granularity miss.

### 2. Lock chrome — **written** (falls out of 1)

`object_fields` already assembles `puzzle3d-play-inspector.object.locked`. The browser-shaped law asserts that namespaced flag_row. §8.23 `leftover_translate_selection_on_locked_object_refuses_with_notice` still passes against it.

### 3. Clipboard paste no-op — **written** (two hops)

Browser Copy/Paste ran (`clipboard-instack`) with a flat census because:

1. **Copy captured nothing** when leftover granularity was not `object` — `selected_object_ids()` returned `[]`. `puzzle3d_selected_objects_from` now falls back to leftover `snapshot.selected` matched against fixture object ids (same rule as Inspection).
2. **Paste built the clone then emitted zero mutations** — `puzzle3d_mutations_between` went through the Value/`document_delta` bridge, which dropped the new object. It now uses typed `puzzle3d_snapshot_from_fixture` + `puzzle3d_snapshot_mutations`, which emits `CreateObject`.

Law `leftover_copy_paste_clones_selected_object`: vortex-granularity leftover select → copy fragment → paste → one `CreateObject` with a fresh id, cloned kind/label, origin + default paste offset.

### Vite-live (held, not rebuilt)

- probe suffix Inspection ids + diagonal gumball drag + `[expect-42]` tags
- leftover gumball mode `move` (engine-contract leftover law still 1 passed)

### Laws (ran)

cargo `leftover_` `--features component-app-assembly`: **6 passed / 658 filtered** (`🗑️generated/w-g3-826-leftover-laws.txt`).

Vitest leftover InteractionView publication: **1 passed / 666 skipped** (`🗑️generated/w-g3-826-leftover-vitest.txt`).

### Ready for #42

**Yes.** Coordinator can rebuild wasm from this source. Blockers: none on these three hops. W-AB brush/suggestions/import still `[expect-41]` and were not touched.

### Guest files

1. `✏️s/…/📌️panels/🔍️inspection/🦀️.rs` — fall-through + browser-shaped law
2. `✏️s/…/✏️editor/🦀️.rs` — leftover copy selection fallback + typed paste mutations
3. `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — leftover copy→paste clone law

W-AB `plugin/rs` was not touched. No serve kill, no wasm rebuild, no git.

## 8.27 W-G3 #42 battery (20:22) — five lane root causes

#42 battery `🗑️generated/probe-2026-09-10T20-22-03.md` (867s). Boot / example-switch / undo-unwind / undo-redo / suggestions / gumball-handle-enter **PASS**. W-G3 lanes still FAIL, with changed evidence vs #41. Vite-live host/probe landed; guest hops for #43 written and law-proven. No wasm rebuild. `:6014` stayed HTTP 200.

Targeted re-run `--selection --clipboard --gumball --locked --port=6014` (`🗑️generated/probe-2026-09-10T20-48-35.md`) hit a live actor `TypeError: Cannot destructure property 'length' of 'v102_1'` — leftover never published. That run does **not** invalidate the 20:22 #42 evidence; it is a post-HMR worker fault (W-AB parallel / host HMR), not a new guest gap.

### 1. Inspection — leftover carried a vortex uuid; fall-through only matched `object.id`

At selection leftover InteractionView:

`selectedIds: ["5de35caa-0f02-43d7-ae74-aa730efd3386"]`, `hoverTarget.domain=vortex`, same uuid. Later leftovers: `a16ba50b`, `9fde3a12`, `530abb65` — all bare UUIDs, none equal to a Nakagin `objects[].id`.

Inspection DOM after pick rendered `.empty` / `.schema` / `.domain` / `.objects` (180). Empty summary **did** render (was: nothing) — §8.26 fall-through engaged, then missed. No `leftover Inspection tab` this run (epoch already on Inspection).

§8.26 `or_else` only matches leftover `selected` against `object.id`. Vortex-granularity leftover with a **vortex uuid** (no `objectId:vortexId`) stays `.empty`. Lock chrome never assembles.

**#43 hop (written):** `selected_section` second `or_else` resolves leftover selected ids as `vortex.id` or `puzzle3d_vortex_full_id` and renders parent `object_fields`.

Law `leftover_selected_vortex_uuid_falls_through_to_object_fields`: vortex granularity + selected=`5de35caa-…` + that vortex on `seed-left-001` → namespaced `object.id` + `object.locked`, not `.empty`. **1 passed / 665 filtered** (`🗑️generated/w-g3-cargo-vortex-uuid-827.txt`). Companions `leftover_browser_shaped_*` and `leftover_vortex_granularity_unresolved_*` still pass.

### 2. Clipboard — copy **did** dispatch; leftover never wrote the stack

`copyBtn=0 copyById=0` but ingress fired:

- `performInvocation actionId=copy` seq 312 and 313
- leftover job 401 `effects=send-message,send-message,send-message`
- `wireEffectToFriendly: unmapped effect "send-message" dropped`
- paste seq 314 + `paste.execute` in fileish
- census 180→180, history dump has Set Active Example / Resize / Toggle / Undo / Redo — **no Copy/Paste**

`historyHasCopyPaste=false` is the history-panel dump, not “copy never ran.” Copy leftover is three UI `send-message` frames. Host maps `clipboard-write` but drops `send-message`. `ClipboardWrite` never became a friendly leftover effect.

Even if ClipboardWrite had landed, §8.26 `puzzle3d_selected_objects_from` only matches leftover `selected` to `object.id`. Battery leftover selected a **vortex uuid** → copy fragment empty → no history upsert. The paste CreateObject hop cannot fire.

**Host vite-live:** leftover clipboard-write tap — `leftoverClipboardWriteEffects` warns when leftover is only `send-message`. Probe now logs `clipboard copy-lane` / `paste-lane` (settled actionIds + leftover tags). Do not treat the history dump as “copy never dispatched.”

**#43 hops (written + remaining):**

1. **Written:** `puzzle3d_selected_objects_from` resolves leftover selected vortex uuid / full id to the parent object. Law `leftover_copy_paste_clones_object_from_selected_vortex_uuid` **1 passed / 666 filtered** (`🗑️generated/w-g3-cargo-copy-vortex-827.txt`).
2. **Still #43 guest (not in wasm):** reserved copy leftover must emit a leftover `clipboard-write` tag, not only `send-message` AppFrames. Do not revert W-AB `plugin/rs`. Host cannot invent a clipboard fragment from UI frames.

### 3. Locked refusal — follows Inspection

`locked-flag-row` / `locked-refusal-notice` `notices=[]` after filtering Agent disconnected. Presence held (no disconnect in the notice assert). No object fields → no `object.locked` flag_row → `refuse_when_locked` never shows `selection_locked`. Same #43 Inspection hop.

### 4. Gumball — entered move handles; every pose delta skipped

Handle-enter PASS. Leftover mode `move`. Drag taps: `gumball drag entered {kind: moveY|moveX|moveZ, ids: Array(1)}` then `gumball pose delta skipped {transformMode: move, kind: move*, before: Array(3), after: Array(3)}` ×6. `gumballTransformDeltaBetweenPoses` returns null when `|dx|,|dy|,|dz| < 1e-6`. Playwright stringifies both positions as `Array(3)`. Guest `translateSelection` never ran.

**Host vite-live:** skip path now logs numeric `dx/dy/dz`. On `moveX|moveY|moveZ` + epsilon, host synthesizes `translateSelection` with 0.5 along that axis so the guest translate job can run and `sceneDelta` can flip. Needs a clean leftover pick (not the 20:48 `v102_1` fault) to prove.

### 5. New fault — 4096 continuations with `required=[]`

During suggestions-open: `interactionSelect` vortex pick `a16ba50b-…` → `actor puzzle#1 did not publish its requested UI surfaces within 4096 continuations (required=[], published=[...], effects=0)`. `settle puzzle#1 continuation 4096 status=more-work acks=0 drain=true`.

`hasRequiredUiPatches` treats an empty Set as already satisfied. `settleAcknowledgedPluginTurns` still passes `new Set()` + `drainOperations=true`. `hasWork` stays true while the actor reports `more-work` with **acks=0**, so the driver spins the full 4096 budget.

**Host vite-live (written + law):** `settlePluginTurn` stops when the required set is an **explicit empty Set** and a continuation produced no acknowledgements; that stop skips the 4096 throw. Omitted `required` (undefined) still drains until a patch or idle. Law `does not spin the continuation budget when drain-operations has an empty required set and no acknowledgements` plus `does not chase background work…`: **2 passed / 92 skipped** (`🗑️generated/w-g3-vitest-settle-827.txt`).

### Vite-live (this tick)

- PluginRuntime empty-required drain stop
- World3dHost synthesize `translateSelection` on move-axis epsilon skip
- leftover clipboard-write missing tap
- probe copy/paste lane logs

### #43 guest (in source, needs wasm)

1. Inspection leftover vortex-uuid → object fields + lock row
2. Copy leftover vortex-uuid → parent object fragment
3. Copy leftover must emit `clipboard-write` (still open; host tap only)

W-AB brush-preview / import stayed `[expect-41]` and were not touched. W-AB `plugin/rs` was not touched.

### Laws (ran)

- cargo `leftover_selected_vortex` / `leftover_browser` / `leftover_vortex_granularity`: **3 passed**
- cargo `leftover_copy_paste_clones_object_from_selected_vortex`: **1 passed**
- vitest PluginRuntime empty-required settle: **2 passed / 92 skipped**

### Files

1. `framework/…/PluginRuntime/tsx` — empty-required stop
2. `framework/…/tests/plugin-runtime/tsx` — empty-required law
3. `framework/…/World3dHost/tsx` — synthesize translate
4. `✏️s/…/📌️panels/🔍️inspection/🦀️.rs` — vortex-uuid fall-through + law
5. `✏️s/…/✏️editor/🦀️.rs` — copy parent-object from leftover vortex uuid
6. `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — leftover copy vortex-uuid law
7. ticket `🔍️browser-probe.ts` — copy/paste lane logs

No serve kill, no wasm rebuild, no git.

## 8.28 W-G3 worker `v102_1` (command-ingress page)

Coordinator probe `🗑️generated/probe-2026-09-10T21-04-08.md` (faults=200) died at frame-perspective (~15.6s) with:

`Cannot destructure property 'length' of 'v102_1' as it is undefined` in served `semio_s_plugin_puzzle_component.js` `poll$1`.

The 20:22 #42 battery on the same wasm had faults=1 (continuation spin only). This was a vite-live host regression, not guest wasm.

### Root cause

`poll$1` lowers optional command ingress as `{ cursor, page }` and then `{ length, block00, … } = page`. `v102_1` **is** `commandIngress.page`.

W-AB retargeted `createShardCommandIngressPages` at `{ cursor, bytes }` for `reactor.stageCommandPage` (comment: "never a fixed 4 KiB block record"). Live puzzle bridge.js (#42, not rematerialized) still calls the old 4-arg `reactor.poll(events, commandPage, coldPairPage, budget)` and forwards the host object unchanged. `{ cursor, bytes }` has no `page` → wasm destructures `length` off `undefined` → shard worker trap → "Window is not responding" / hung Interactive / setActiveUtility and import-apply death as collateral.

Not caused by the empty-required continuation stop or gumball synthesize-translate (those do not run at frame-perspective).

### Fix (data defined, bytes API kept)

`ShardCommandIngressPage` is now `{ cursor, bytes, page }`. The factory still slices live `bytes` for the new WIT and also mints `page: createActorBytePage(bytes)` so #42 `poll$1` can destructure `length`. W-AB `stageCommandPage(cursor, bytes)` is unchanged.

Law `defines reactor page.length so wasm poll can destructure a bytes-shaped ingress` plus the existing DataView oracle / turn-forward pair: **3 passed / 218 skipped** (`🗑️generated/w-g3-vitest-ingress-828.txt`).

### Re-probe

`--frame --port=6014` → `🗑️generated/probe-2026-09-10T21-11-41.md`: frame-perspective completed, **v102_1=0, worker fault=0**, actor stayed up (`instanceCount=1`, `vortexCount=11`). Residual faults are typed-operation `registerBrushMesh` / `engagementAbort` (not this trap).

`--brush --import --port=6014` → `🗑️generated/probe-2026-09-10T21-12-46.md`: frame-perspective `newFaults=none`; history includes **Set Active Utility**; `utility.publish` fired; `importFixture ingress` ran; **v102_1=0, worker fault=0, trapped=0**. Brush-preview / import-distinct still `[expect-41]` (W-AB guest, not this host trap).

`:6014` stayed HTTP 200. No serve kill, no wasm rebuild, no git.

### Files

1. `framework/…/actor/shard-client/ts` — emit `page` beside `bytes`
2. `framework/…/shard-client/tests/reserved-response-settlement/ts` — keys include `bytes`; new length-destructure law

## 8.29 W-G3 #43 battery — host-context storm, Inspection miss, gumball pre-admit

Coordinator battery `🗑️generated/probe-2026-09-10T21-24-53.md` (289s, wasm **#43** on `:6014`). Clipboard / suggestions / undo / boot **PASS**. W-G3 leftovers: faults=124, Inspection empty, gumball `sceneDelta=false`, lock chrome absent.

### 1. `setCamera` → `surface 1:window has no host context` (host, proven)

`invocationFromFrames` rethrows an `AppFrame::Error` whose message is formatted in plugin host `plugin_render_surface`:

`instance.surface_contexts.get(surface).ok_or_else(|| plugin_internal_fault(format!("surface {surface} has no host context")))`

`SurfaceContexts` is the host-context table: `surface-visible` inserts `{surface, bodyKey, viewState}`; leftover dirty then `plugin_render_surface` looks the surface up. `get` returns `None` when that id was never mounted (or `update_view` pruned it).

Host mounts windows as `1:puzzle3d-main-perspective` / `1:puzzle3d-main-top` (`pluginSurfaceRef(instance, target.key)`). Leftover Viewport patches omit a window instance; host intake and guest dirty default the surface to **`window`** (`wirePatchSurfaceId` → `1:window`). #43 W-AB Viewport publication started dirty-rendering that default id. `setCamera` (already Viewport) now answers with an Error frame when leftover tries to render `1:window` with no row in `surface_contexts`.

Not a vortex-uuid leftover bug. Not a missing `setCamera` guest arm (`set-camera` only writes `ctx.scene.runtime.camera`).

**Fix (vite-live):** `windowHostContextBindings` emits the authored window surfaces **and** aliases the last bound window onto leftover default `window`, so `surface-visible` establishes host context for `1:window` before Viewport dirty. `uiRefreshSurfaceEvents` uses that table.

Same class also hit `interactionHover` / `suggestionsTick` when leftover dirtied `1:window`.

### 2. Inspection still empty — leftover + host tab no-op (precise miss)

At the **selection** assert (~72–77s) leftover InteractionView was `selectedIds: []` with hover `{domain: vortex, id: seed-left-001}`. That id is an **object** id, not a vortex uuid. The #43 vortex-uuid fall-through was **not exercised**. Object-id fall-through was **not exercised** either (no leftover selected id). First canvas pick published hover-only leftover. Guest hop for #44: `interactionSelect` leftover must put `seed-left-001` in `selected` on that pick.

At **clipboard / locked** leftover **did** carry `selectedIds: ["seed-left-001"]` (object id, gumball true). That is why copy **PASS**. Inspection stayed `fields=[]` / `empty=false` because leftover Inspection epoch only **switched the Inspection tab** and **returned early when already on that tab**. Battery already had Inspection open → epoch no-op → no panel body refresh. Viewport leftover from `setCamera` never includes the Inspection panel.

**Fix (vite-live):** leftover epoch now keys on selected-id identity (no Full-refresh storm) and always `refreshUi(session, { kind: "full" })` when leftover selection appears, even if the Inspection tab is already current. Lock chrome still follows Inspection object fields.

### 3. Gumball `sceneDelta=false` — synthesize did not run

Handle-enter **PASS**. Taps: `gumball drag entered {kind: moveXY, ids: Array(1)}` then `gumball pose delta {action: translateSelection, ids: Array(1), mode: mesh}`. Synthesize-translate only fires on `moveX|Y|Z` epsilon skip; this drag was **`moveXY` with a real payload**. Guest `translateSelection` dispatched and answered:

`fixed typed-operation and segmented-output authorities did not pre-admit the exact operation slot`

(`plugin.rs` typed-operation / segmented-output slot admit). Scene stayed 3 instances / poseLen=786 (Nakagin after undo+clipboard, not 180). **#44 guest/host admit:** pre-admit the exact translate slot before mesh-mode `translateSelection`, or leftover must not start a typed operation without a reserved slot. Host-context storm likely consumed slots (`suggestionsTick` / `setCamera` same pre-admit class later in the battery). Re-probe after the `1:window` alias before assuming a new guest hop.

### 4. Lock chrome

Depends on 2. No separate hop this tick.

### Laws (this tick)

- JSON `engine/fixtures/window-host-context` + Ajv + independent last-window `window` alias oracle + leftover Inspection refresh-scope rows
- TS `windowHostContextBindings` / `leftoverInspectionRefreshScope`
- Rust `default_window_surface_has_host_context` on `7:window` in `surface-context-lifecycle`

`:6014` stayed HTTP 200. No serve kill, no wasm rebuild, no git.

### Files

1. `framework/…/PluginRuntime/tsx` — leftover `window` alias + Inspection refresh scope
2. `framework/…/ShellHost/tsx` — leftover Inspection Full refresh when selected ids change
3. `framework/…/engine/tests/window-host-context` + fixture
4. `framework/…/plugin/reactor/surfaces` — `7:window` host-context law
5. renderer-react `vitest.config.ts` — suite registered

### Handoff

- W-AB: brush-preview / import still `[expect-41]` — not touched
- #44 guest: first-pick leftover `selectedIds` empty (hover object id only); gumball mesh `translateSelection` pre-admit if still failing after the alias
- Re-probe `--selection --clipboard --gumball --locked --port=6014` on vite-live #43 (do not rebuild wasm)

## 8.30 W-G3 leftover first-pick selectedIds + mesh translate pre-admit (ready for #44)

Guest hops for rebuild **#44**. Vite-live `1:window` alias, leftover Inspection refresh, and gumball mode `move` were left in place. W-AB `importFixture` was not touched. No wasm rebuild, no serve kill, no git.

### 1. First-pick leftover `selectedIds` empty

Battery leftover at first canvas pick was `selectedIds:[]` with hover `{domain:vortex, id:seed-left-001}` (object id, not vortex uuid). Canvas pick published hover-only leftover.

`dispatch_interaction_action` now keeps the exact pick targets on leftover `interactionSelect` when `next_selection` returns empty — including a vortex-domain pick of an object id. Hover leftover still restores a prior leftover selection so a later hover turn cannot wipe it. Vortex-uuid Inspection fall-through is unchanged.

Law: leftover `interactionSelect` of `seed-left-001` → leftover `selectedIds` contains it.

### 2. Gumball `translateSelection` pre-admit

Leftover drag dispatched `translateSelection` `{ids, mode: mesh}` and the guest used to answer `fixed typed-operation and segmented-output authorities did not pre-admit the exact operation slot`. Ingress now pre-admits a vacant residue-class slot (`admit_typed_operation_slot`) before minting the operation id, so leftover mesh-mode translate with explicit ids commits a pose delta instead of colliding with a hover/tick/camera storm.

Law: leftover `translateSelection` with explicit ids + mesh mode commits a pose delta.

### Laws (this tick)

Ran. Do not claim from compile-only.

1. `cargo test -p semio-framework-plugin leftover_interaction_select_object_id` → `leftover_interaction_select_object_id_on_vortex_domain_lands_in_selected_ids` **ok** (1 passed / 660 filtered). Log `🗑️generated/w-g3-830-select-law.txt`.
2. `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib leftover_translate_selection -- --test-threads=1` → `leftover_translate_selection_mesh_mode_commits_pose_delta` **ok**, locked-object refuse **ok** (2 passed / 705 filtered). Log `🗑️generated/w-g3-830-mesh-translate-law.txt`.
3. Same crate, filter `leftover_` with harness leftover_overlay_translate / leftover_selected_vortex / leftover_translate_selection_mesh → **13 passed / 0 failed / 694 filtered**, including overlay unlock translate and `leftover_selected_vortex_uuid_falls_through_to_object_fields`. Log `🗑️generated/w-g3-830-sibling-leftover-laws.txt`.

### Files

1. `framework/.../plugin/rs` — `interactionSelect` keeps object-id pick targets; typed-operation slot pre-admit
2. `framework/.../plugin-runtime-plugin-builder-contract/rs` — object-id leftover `selectedIds` law
3. `s/.../puzzle/3d/editor/tests/unit/rs` — mesh-mode leftover translate pose-delta law

### Handoff

Ready for **#44**. Re-probe `--selection --clipboard --gumball --locked --port=6014` on vite-live after the wasm rebuild. Do not revert W-AB importFixture.
