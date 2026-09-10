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
