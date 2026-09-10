# Runtime verification — puzzle 3d on the React target (release guest)

Ticket 26/09/02/PUZZLE-3D-END-TO-END. Server: direct release chain (`serve-release-direct.sh`, port 6013,
`SEMIO_VITE_HMR=0`), component built 07:20-07:27 from the tree as of 07:20 (includes the example-order
swap, the locale/terminology dispatch injection, the MessageChannel yield, the proportional intake
budget, the int/uint wire tags). Browser pane emulated at 1440×900 (the pane itself stays hidden).
Every observation below is from the page's own console hooks (`console.error`/`warn`/`log` captured
in-page, `[DEBUG] cooperative-maintenance` filtered) plus React-fiber reads of `shellState`.

## 07:40 boot (reload of `/?plugin=puzzle3d`)

| t | observation |
|---|---|
| 0-30 s | title `semio · puzzle · 3d`, 2 windows (`puzzle3d-main-top`, `puzzle3d-main-perspective`), example picker = **Concrete Forest** (order swap landed) |
| 30.3 s | `[DEBUG] local interaction observation failed … actor puzzle#1 did not publish its requested UI surfaces within 4096 continuations (required=[], published=[], effects=0, status=more-work)` |
| 30.3 s | `[DEBUG] action failed setActiveExample {"exampleId":"concrete-forest"}` — same 4096-continuation exhaustion, `status=more-work` |
| 30.8 s | `[DEBUG] action failed noteShellCommand {"commandId":"shell.windowResize"}` — same, `effects=1` |
| ~40 s | both windows render (Top: ortho grid + hexagon footprint; Perspective: grid, gizmo, the Concrete Forest seed object); catalogue tree Objects/Vortices/Cables/Attractions |

The three boot failures are one symptom: every action turn during the first ~30 s returns
`more-work` for 4096 continuations without effects/patches (`settlePluginTurn`,
`🔌️PluginRuntime/🟦️.tsx:1058`). After boot settles, actions succeed (below). Root cause not yet
measured — instrumentation added to the reactor turn (`⚛️reactor/🔄️turn/🦀️.rs` `[DEBUG] reactor
more-work streak=…` naming which of executor/close-cleanup/typed-operation/reconcile/resumes/
command-ingress/lifecycle keeps the turn hot); needs the next component rebuild.

## Interactions after boot (all with zero faults unless noted)

| action | how | result |
|---|---|---|
| Utilities bar (Perspective) | `#framework.window.puzzle3dMainPerspective.utilityBar.unfold` | bar shows **Transform, Brush, Volume Brush, Relocate** |
| Tool category | `#framework.category.tool` | shows **Fill** (`#tool.fill`) |
| Fill panel | `#tool.fill` | panel tab opens with only the activate toggle (`#tool.fill.activate.toggle`) |
| Fill activate | inner `<button id="tool.fill">` | toggle → `pressed=true`, `data-state=on`; **no count slider, no cancel row, no distribution tree** (`shellState.windowUi.toolMeasuresByToolId === {}`) |
| Brush utility on/off | `#brush` | pressed toggles; no faults |
| Click on the seed object (Perspective, select utility) | pointer click at the object | `shellState.interaction.selection === {}` — no selection observed (hit or pick path unverified) |
| Example switch → Nakagin | picker option | picker label changes; **scene unchanged after 94 s**, no fault, no `[DEBUG] action failed` |
| Example switch → Concrete Forest | picker option | label changes; no shard traffic captured by the Worker/MessagePort hooks (hooks may not see the shard transport) |

## Root causes found so far

1. **Sections never reach the React shell** (framework, retained lane). `refreshUi`
   (`🔌️PluginRuntime/🟦️.tsx` ~1615) only turns `windows`/`panels` into `surface-visible` events and
   `ownedUiRefreshResponse` (~1320) only projects window/panel surfaces; the request's
   `engagements`/`measures`/`tools` sections are never produced (refresh cache holds only `window:*`
   and `panel:*`). Hence `windowMeasuresByWindowId === {}` and `toolMeasuresByToolId === {}`: the Fill
   tool has no count slider, the Projection pane and utility option groups have no measures, window
   engagements never arrive. Wave **W-M** (`📓️2026-09-09-wave-M-retained-sections.md`) makes the
   three sections first-class retained surfaces.
2. **Applied-edit ledger ceiling** (framework store). `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64`
   (`🌿️vcs/🦀️.rs:186`), `push_applied` refuses at 64 (`🏪️store/🦀️.rs` ~12487); retained tool
   operations publish `Emit.artifact_mutations` one edit per mutation. A Nakagin load (~182) and any
   fill > 64 placements hit the ceiling. Audit `📓️2026-09-09-applied-ledger-ceiling-audit.md` in
   flight; fix wave to follow (one Edit per Emit vs compaction vs capacity).
3. **Boot-time more-work spin** (open): see the boot table; instrumentation pending rebuild.
4. Peer churn: the 3d editor is uncompilable since 07:28 (`cannot find value config` ×4 at
   `✏️editor/🦀️.rs:2995-3060`, kind-weight work; the ownership peer's uncommitted edit) — the release
   component cannot be rebuilt until it heals.

## 08:55 rebuild #3 (W-M sections + reactor trace) — root cause of the more-work spin, measured

Sections now arrive: `windowMeasuresByWindowId` = 10 measures for each of `puzzle3d-main`,
`puzzle3d-main-top`, `puzzle3d-main-perspective`; `toolMeasuresByToolId.fill` = `puzzle3d-fill-count`,
`puzzle3d-play-distribution`; engagements for all three windows. Boot actions still fail after 4096
continuations, and the reactor trace (worker `eprintln`, read via the console buffer) says why:

```
[DEBUG] reactor more-work streak=4096 seen=4215 executor_deadline=false close_cleanup=false
        typed_operation=true reconcile=false resumes=false executor_pending=false command_ingress=false lifecycle=false
[DEBUG] typed-operation publication turn=4096 operations=["47:Worker:true:true"] latest_wins_empty=true
        effects=0 events=0 ui=0 query=started=true page_sent=true … inner=ready=true length=256 terminal_page=false
[DEBUG] cooperative-maintenance … pool=CooperativePoolSnapshot { pump_calls: 4096, selections: 514,
        no_selection: 3582, selected_by_lane: [3, 0, 0, 511, 0, 0], queued_by_lane: [0, 0, 0, 1, 0, 0] … }
```

- The only hot source is `typed_operation`: tool operation 47 (the boot `setActiveExample` retained job)
  sits in stage `Worker` for the whole 4096-turn budget. The local-interaction query is *not* the spin
  (page ready, waiting for the host ACK, `has_pending_work` false).
- `selected_by_lane[Interactive] = 3`: the process pool executed THREE interactive job steps in 4096
  reactor turns. On wasm the pool has no threads; a step submitted by `MountedWorkerJobSession::pump_one`
  runs only when `WorkerPool::pump` is called, and the only pump was `pump_runtime_live_cooperative_turn`
  — one pump per cooperative-maintenance job (lane 3, 511 runs), each of which calls
  `maintenance_step` once, whose stage 0 of 21 is the sole `drive_worker_step` site. Net cadence: one
  worker step per ~170 turns.
- Fix (framework): `🔌️plugin/🦀️.rs` `advance_typed_operation_publication_one` now drives a
  Worker-stage operation for a bounded slice (`drive_typed_operation_worker`, ≤256 pumps / 4 ms) and
  `⚛️reactor/🔄️turn/🦀️.rs` pumps the process pool inside every turn on wasm
  (`pump_process_worker_pool`, ≤64 pumps / 2 ms, folded into `more_work`). Rebuild #4 queued.

## 09:05-09:40 rebuilds #4-#6 (worker pump, W-B batching, fault-body retention, maintenance stage trace)

- The 4096-continuation spin is gone: boot actions now settle in 1-17 s instead of exhausting the budget.
- Rebuild #4 surfaced the hidden job fault as `typed-operation cancelled before its next publication unit`
  (the lease cancellation masked the body) → `terminal_fault` retention on the mounted operation.
- Rebuilds #5/#6 (consistent tree): the actor traps at ~17-23 s with
  `plugin.internal.interactive-ceiling: runtime live cleanup faulted for instance 1: the turn overran the
  interactive step ceiling (elapsed 8401-10201us, ceiling 8000us)`. The per-stage trace names the
  offender: `[DEBUG] maintenance stage=14 elapsed_us=8401` = `drive_store_replacement_jobs` — one unit
  of the boot example's whole-document store replacement exceeds the 8 ms law on the release wasm, and
  the cooperative-maintenance verdict (`runtime_live_cleanup_publish_turn`) is instance-fatal.
  Wave W-R2 (`📓️2026-09-09-wave-R2-store-replacement-step-budget.md`) slices that unit.
- Concern for the framework owner: a single overrun of any maintenance unit kills the instance
  (`RuntimeMaintenanceStatus::Fault(InteractiveCeiling)` → reactor turn error → actor trapped); a
  quarantine of the offending job would keep the app alive.

## 12:40 rebuild #7 (non-fatal ceiling verdict, W-P3, W-R2, W-T, host-owned tool switches)

- The actor no longer traps; sections and both canvases mount; the boot `setActiveExample` and the
  local-interaction read fail with the retained job's real fault: **`puzzle command wire payload is
  malformed`** — `RetainedPuzzleCommandJob`'s `Decode` phase runs `Puzzle3dCommand::decode_op`, which
  parses the externally-tagged map `{"SetActiveExample":{"window_id":…,"args":…}}` that `encode_op`
  writes, but the host's JSON action route (`dispatch_action` / `dispatch_command` →
  `admit_command_json`) admitted the tuple `["setActiveExample", args]` as the retained raw wire and
  transferred it to the app-owned factory. The native tests never saw it because the testkit uses the
  test-only `dispatch_typed`, which admits `encode_op(&command)`.
- Fix (framework, `🔌️plugin/🦀️.rs`): both JSON routes now build the typed command first and admit its
  exact `OpBinary` wire through `admit_command_wire` (the `dispatch_typed` shape); the dead
  `admit_command_json` was removed. Rebuild #8 waits behind a peer's in-flight `🫧️transient` refactor.

## 14:20 rebuilds #8-#10 — the second architectural wasm gap, measured

- Rebuild #8 (exact `encode_op` wire for JSON actions): the boot `setActiveExample` succeeds in 2.4 s,
  no faults, both scenes render, sections arrive. The next host action, the shell's
  `noteShellCommand` (a framework-reserved route), never returns.
- Rebuild #10 trace: `[DEBUG] turn 870 begin events=0` is followed by
  `[DEBUG] reserved job 'noteShellCommand' poll 1…4096: Submitted` with no later turn phase — the
  poll loop of `run_framework_reserved_job` spins INSIDE one reactor turn. `dispatch_action` →
  `dispatch_framework_reserved_action` is driven synchronously by `resolve_ready`, whose poll loop
  re-polls `plugin_job_yield_once` immediately and never pumps the cooperative pool, so the mounted
  worker step submitted by `pump_one` can never execute on wasm. Natively a pool thread runs it, which is
  why no test sees this. Every framework-reserved route (undo, redo, clipboard, history revert, shell
  notes, tutorial recording) hangs the actor on the React target.
- Fix (`🔌️plugin/🦀️.rs` `run_framework_reserved_job`): after a non-terminal poll the loop pumps the
  process pool itself on wasm (`pool.pump(now_ms)`), the same law `drive_typed_operation_worker` applies
  to typed operations. Rebuild #11 queued; per-turn phase traces reduced to every 256th turn.

## 17:10 rebuild #11 (reserved-route pump, exact wire, W-B/W-M/W-P3/W-R2/W-D2, composition impl)

- Boot: `setActiveExample` OK 2.4 s, `noteShellCommand` OK 1.6 s — the reserved-route hang is gone.
  Sections, both scenes, outliner and inspection render.
- `readLocalInteraction` and `readHistory` at boot still exhaust 4096 continuations in ~4 s
  (`typed_operation=true` streak of 5066 turns that then ends) — the boot example load's typed operation
  runs to completion but the two reads drain on it; W-A adds drain polling and completion routing.
- `registerBrushMesh` now fails with its real reason: `rejected 63997 raw bytes before decoding;
  maximum is 8192` — the world layer uploads the whole GLB in one command → W-M2 pages it.
- Example switch to Nakagin: `setActiveExample` returns OK (started) in 705 ms, the job runs 395 turns,
  no fault, but the document view never updates and History shows no edit: the completion
  (`Invocation { in_reply_to: 0, ui_scope }`) is dropped by the channel client (no waiter with seq 0)
  → W-A (`📓️2026-09-09-wave-A-async-completion-refresh.md`). History rows render as
  `framework.history.entry.[object Object]` (pack integer carrier) — same wave.

## 18:35 — History panel description overflow (`ui.fixed-capacity … history-panel.command-description`)

- Symptom (after Fill activation, rebuild #12): `refreshUi` and `setActiveTool` failed with
  `ui.fixed-capacity: fixed UI admission failed at history-panel.command-description`; every later
  shell refresh was rejected, so the tool switch looked dead from the user's side.
- Root cause: `ui_history_panel` joined all `CommandView::op_lines` (each `OpText::print_op`, the full
  DSL/JSON line of the edit's forward operations) into one `UiText` description. A fill/mesh edit
  prints far more than the 512-byte `UI_TEXT_MAX_BYTES`, so admission failed for the whole panel.
- Fix: `UiText::clipped(&str)` in the ui-contract crate (`🎬️action.rs`) — whole value when it fits,
  otherwise the longest char-boundary prefix plus `UI_TEXT_CLIP_MARK` (`…`); the history panel
  description now uses it. Full op lines stay in the history view (and the React shell's own join).
- Tests: contract unit test `ui_text_clipped_keeps_short_values_and_marks_long_ones_on_a_char_boundary`
  passes (run); plugin-host test `ui_history_panel_clips_an_oversized_operation_description` is
  written but the native test target is currently blocked by a peer's `🧪️tests/🧩️composition` file
  (`ArtifactStoreInitializationAuthority` etc. missing in `super`); `cargo check -p
  semio-framework-plugin` (lib) is clean. Runtime proof pending rebuild #13.
- Slider probe: focusing the `puzzle3d-fill-count` slider span and pressing ArrowRight ×20 produced
  no action calls (value stayed 0) — needs a pointer drag or the slider's own keyboard handling; retest
  after rebuild.

## 18:50 — Fill count slider stuck at 0: consequence of the history-panel fault, not a slider bug

- Live measure props read from the React tree: `{"id":"puzzle3d-fill-count","value":0,"min":0,
  "max":1000,"ready":0,"loading":true,"reveal":"puzzle3d-fill","onChange":{"action":"setFillCount"}}`.
  The slider is a reveal slider (`clampToReady`), so with `ready = 0` every pointer/keyboard value is
  clamped back to 0 and nothing dispatches — by design.
- `ready` is the planned fill count. Planning is driven by the shell's 120 ms `fillBuildTick` loop
  (`World3dHost`), which only runs while the main body's interaction JSON reports
  `activeUtility === "fill"` and `fillBuild.done === false`. That JSON is refreshed by the same
  `refreshUi` that failed on `history-panel.command-description`, so the world body never learned the
  fill tool was active, no tick was ever dispatched (no `fillBuildTick` in the call hook for >40 s),
  and the plan stayed at 0.
- Expected after rebuild #13 (history description clipped): refresh succeeds → interaction
  `activeUtility: "fill"` → ticks → `ready` grows → slider drag commits `setFillCount` on release.
- Rebuild #13 attempt 1: `semio-s-artifact-puzzle-2d`/`-5d` rustc processes died with SIGTERM
  (no compile error; a concurrent cargo in the same private target, see the lane-qualified target
  memory) — the retry loop continues.

## 19:15 — Rebuilds #12/#13 never reached the browser (stale materialized component since 17:07)

- Served module dir `🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules/🧩️puzzle` was
  last written 17:07:19; its `…core.wasm` still contains `history-panel.command-description`, while the
  freshly built `target-p3d/wasm32-wasip2/wasm-release/semio_s_plugin_puzzle.wasm` (18:51) does not.
- Cause: `nx run @semio-tech/puzzle-plugin:component-release` only runs cargo; the jco transpile +
  wasm-opt into the module dir is the dev script's `plugin <variant>` command (`buildPlugins` →
  `materializePlugin`). The serve chain's `materialize release` step does not exist ("unknown command")
  and `activate` reported `(unchanged)` on both #12 (18:17) and #13 (18:51).
- Consequence: every browser observation after 17:07 ran a wasm without the history clip, W-A's
  completion frame, W-M2's paged upload fixes and W-D3/D4's Rust fixes. The 18:5x re-test therefore
  reproduced the `ui.fixed-capacity … history-panel.command-description` fault unchanged (expected for
  the stale build) and the fill slider stayed at `ready: 0`.
- Fix: `🔨️materialize-and-serve.sh` (ticket + scratchpad) runs
  `SEMIO_BUILD_MODE=ship … 📜️script.ts plugin puzzle3d` with the private `CARGO_TARGET_DIR`, then
  restarts the release server; `🔨️rebuild-until-ok.sh` now chains that step after `component-release`.
- Correction (19:25): the dev script's `plugin puzzle3d` is the wrong tool (recompiles with other rustc
  flags, writes `🧑‍💻dev/🔌️plugin-modules`, which the react release serve does not read). The real
  cause was the recreated `🔨️serve-release-direct.sh` pointing `PLUGIN_SCRIPT` at the puzzle-js
  `📜️script.ts` (only `test`), so `support release` and `materialize release` both printed
  `unknown command` on every serve since the 17:56 restart. Fixed to
  `🔌️plugin/📦️packages/🟦️typescript/📜️script.ts`; `🔨️rebuild-until-ok.sh` restored to
  `component-release` → serve chain. Materialize of the 18:51 artifact launched 19:24.

## 19:40 — First run on a fresh wasm (materialized 19:26): history fault gone, fill plan never grows, guest OOM

- Boot (fresh profile hook): `readLocalInteraction` still exhausts 4096 continuations (W-I),
  `refreshUi` ok, boot actions now 1.5–2.4 s (were 22 s on the stale wasm). W-A's
  `OperationCompleted` arrives (`operation 35, uiScope full`) and the shell applies it.
- Peer breakage fixed on the way: `🔌️plugin/📇️registry/🟦️.ts` (19:03) imported `node:fs` for a
  new `readTrustedStdioCatalog`; the browser shell imports that module, so every boot died with
  `Module "node:fs" has been externalized`. Moved the reader to
  `📇️registry/✅️trusted-stdio-catalog/🟦️.ts` (no callers yet).
- Fill: Command tab → Fill tab dispatches `setActiveTool` (scope full) → `refreshUi` succeeds
  (no `history-panel.command-description` fault any more) → measures render
  (`puzzle3d-fill-count 0/1000`, distribution group) → the shell's 120 ms `fillBuildTick` loop
  runs (125 ticks/60 s, each tick a typed operation with a completion, `uiScope none`).
- BUT `ready` stays 0 / `loading: true` for the whole run, and after ~115 ticks the guest traps:
  `memory allocation of 98880 bytes failed` → `RuntimeError: unreachable` in `poll` →
  `PluginRuntime: actor puzzle#1 trapped`; every later action fails with `unreachable` (the tick
  loop keeps firing against the dead actor). So the fill planner never publishes a piece and the
  guest leaks/grows memory per tick until OOM — same family as the native `settle` stall W-S is on.
- Lead: `⚛️reactor/💼️jobs/🦀️.rs::spawn_job` only runs a kind registered in
  `BOUNDED_KIND_REGISTRY`; an async `KIND_REGISTRY` kind is `ExplicitStateMachineRequired` outside
  `cfg(test)`. `fill_build_tick` spawns `FILL_JOB_KIND` (`JobPlacement::Isolated`) — check which
  registry puzzle 3d uses and whether the React host steps isolated jobs at all.

## 20:05 — Nakagin switch: completion arrives but its full refresh was dropped (owner check)

- Fresh wasm + live TS: selecting "Nakagin Capsule Tower" dispatches `setActiveExample` (scope
  none, ~0.7 s) and W-A's `OperationCompleted` arrives (`operation 52, uiScope full`), but scene,
  outliner and history stayed on Concrete Forest and no `refreshUi` followed.
- Trace (`[DEBUG] completion apply`): `ownerCurrent: false, sessionCurrent: true`. The subscription
  built its owner with `captureEffectOwner(target, null)`; `isCurrentEffectOwner` requires
  `isCurrentDialogOrigin(owner.presentation)`, which is false for `null`, so `applyHostEffects`
  returned before its refresh. Every other owner site passes `captureDialogOrigin(session)`.
  Fixed in `🏛️ShellHost/🟦️.tsx` (completion subscription now captures the session's own origin).
- Also seen: the completion carries no `historyPatch` for the example switch
  (`history_dirty_sequences` empty at drain time) — history panel stays empty; to check after the
  owner fix. `readLocalInteraction`/`readHistory` still exhaust 4096 continuations on this wasm
  (W-I's fix needs rebuild #14).
- Peer breakage on the way: `💻️os/🟦️.ts:2393` imports `🧪️tests/🧊️mesh-pack-decode/🟦️.ts`
  (mesh pack codec, 19:49) which did not exist — Vite refused every boot and nx's project graph
  failed repo-wide. Wrote that module with five decoder laws (hand-encoded varint/tag bodies,
  symbol/inline texture, error paths, base64 chunk reassembly).

## 20:20 — Owner fix verified; hidden-pane timer throttling; Nakagin refresh now blocked only by the W-I starvation

- With `captureEffectOwner(target, captureDialogOrigin(target))` the boot completion (`operation 35,
  full`) and the Nakagin completion (`operation 227, full`) both reach `applyHostEffects refresh
  {"scope":{"kind":"full"}}` → `refreshUi` (trace `ownerCurrent: true`).
- That `refreshUi` (and the following `readHistory`) still fail with `did not publish its requested
  UI surfaces within 4096 continuations (required=[], published=[])` — the local-interaction /
  returned-read starvation W-I fixed in `🔌️plugin/🦀️.rs` (not in the 19:26 wasm). Rebuild #14
  launched 20:18 with W-I's Rust.
- Verification trap: the Browser pane is hidden (`document.hidden === true`), so Chrome throttles
  main-thread timers to ~1 Hz (measured 7 ticks in 3.6 s for a 10 ms interval; 887 ms lag) and
  intensive throttling after 5 min. That is why the example dropdown would not open and why the fill
  tick loop ran at ~2 Hz. For this session's verification a dedicated-worker timer shim is installed
  after each reload (`window.setTimeout/setInterval` routed through a Worker; 74 ticks/s afterwards);
  it is debugging-only and not part of the app.

## 20:35 — Rebuild #14 (W-I + W-S Rust): boot clean, Nakagin switch now blocked by an endless `reconcile`

- Boot: `readLocalInteraction` ok (27 s, serialized behind the boot actions), `refreshUi` ok,
  completion `operation 35` → full refresh applied, no faults; outliner shows Objects / References /
  Target volumes / Attractions.
- Nakagin: `setActiveExample` ok → completion `operation 227, full` → `refreshUi` and `readHistory`
  fail after 4096 continuations, `status=more-work`. Worker trace at the failure:
  `reactor more-work streak=8192 … typed_operation=false reconcile=true resumes=false
  executor_pending=false command_ingress=false lifecycle=false effects=0` — the retained-surface
  reconcile is the only pending source and never completes after the document switch (W-S's suite
  saw `ui.fixed-capacity: scene-surface.encode … 33527 bytes` on the same document; W-K owns the
  payload cap, but a reconcile that cannot publish must fault, not spin).
- Per-turn `[DEBUG] turn N begin/end` worker traces are still compiled in (two eprintln per turn);
  remove with the next rebuild.

## 20:55 — Rebuild #15 (W-J): fill job starts, then the guest traps again; tick loop over-queues

- Boot clean on #15 (no faults, `readLocalInteraction` 12 s). Fill activation: measures render, the
  bounded fill job now really starts in the browser (transient notice `Fill progress
  prepare-fixture 0/10000`), but within ~30 s the guest trapped again (`RuntimeError: unreachable` on
  `fillBuildTick`, actor dead); the panic text was already pushed out of the 500-line console buffer
  by the per-turn traces (removed in #16), so the cause is being re-captured.
- Tick loop defect (host): with unthrottled timers the 120 ms `fillBuildTick` interval enqueued
  252 actions + 90 `readHistory` in 35 s and overflowed the per-actor queue (`serializePerActor:
  … queue is full (>256 pending turns)`, 38 rejected). `createInFlightSkippingInterval` only skips
  while its own `run()` promise is pending, and `dispatch("fillBuildTick")` resolves before the
  guest turn completes, so ticks queue faster than turns finish. The tick must be gated on the
  previous tick's guest completion (or dispatched from the completion itself), and a tick must not
  trigger a `readHistory`.

## 21:05 — Pick works on #15
- Left click on the slab in the Perspective view: three `handleAction`s (one `full`), `refreshUi`
  ok, `readHistory` ok, `readLocalInteraction` ok in 0.3 s (was 4096-continuation failure before
  W-I); the slab highlights in both Top and Perspective views and the window focus moves.
- Boot cost note: the seven `registerBrushMesh` page uploads each trigger a `refreshUi` +
  `readHistory` pair (14 extra round trips, ~3 s each while the boot queue is busy).
- Context menu on the selected slab opens with real rows (Set Active Example, Add Object…,
  Duplicate Selection ⌘D, Translate Selection, Rotate Selection, Delete Selection ⌫). Defects:
  (a) the submenu group labels render as raw keys `menu.group.history` / `menu.group.hand` /
  `menu.group.selection` / `menu.group.more` (missing EN/DE labels for context-menu groups);
  (b) `Delete Attraction` and `Delete Target Volume` are offered for a plain object selection.
- Inspection panel shows no field group for the selected object (panel body empty) — to verify
  whether the panel is folded or the inspector body is not published.
- Duplicate Selection (context menu) on the selected slab: `handleAction` ok (scope none), completion
  `operation 355` with a partial scope (`windowBodies: [puzzle3d…]`), `refreshUi`/`readHistory` ok —
  but the outliner still lists one object and the history snapshot (`upserts`) contains only
  `interactionSelect` rows (cursor 7, `canUndo: true`): the duplicate produced no document edit and
  no command-log row. To reproduce natively: `duplicate_selection_reselects_the_created_clones`
  passes (W-D4), so the browser path differs (selection carried by the local-interaction lane?).
- Context-menu group labels: the wgpu target resolves `menu.group.<category>` through
  `ribbon_parent_label` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:303`); the React shell has no such
  resolution and shows the raw ids.
- History panel: after expanding the History tab the rows render with numeric ids (`framework.history.entry.1…9`: Resize Window, `delete-object id=seed-left-001` (boot edit, clipped op line), Activate Window, Select ×4, Toggle Panel, Switch Panel Tab) — the `[object Object]` carrier defect is gone; no Duplicate row confirms the no-op above.

## 21:40 — Rebuild #16: reconcile spin after Nakagin now has a tracker trace

- Nakagin switch on #16: completion `227, full` → `refreshUi` fails again after 4096 continuations,
  `published=["1:framework.panel.catalogue", …]` (some surfaces did publish), status stays
  `more-work`. Worker trace at streak 8192: `reconcile=true`, `patches: slots=11 producers=0 jobs=0
  terminals=2 producer_terminals=1 deferred=1 ready=1 output_fault=none`, `pending: slots=[]
  handback_empty=true exhausted=false`. So the publication queue is empty and idle while the tracker
  still holds two terminals, one producer terminal, one deferred surface and one ready output that
  `next_ready_index` never selects (it requires `published && !closing`); `close_step` steps only
  `close=true` terminals one slot per turn and returns early on any closing ready output.
- Fill on #16: the bounded job starts (`Fill progress prepare-fixture 0/10000` notice) but the tick
  loop floods the actor queue (>256 pending turns) so job slices starve (W-L owns the tick gating).
- Rebuild #17 (per-slot tracker trace: surface/generation/producer/job/reconciler/ack vs revision,
  ready published/closing/reservation/empty, terminal close/fault/empty, producer-terminal flags,
  deferred surfaces) launched 21:40; streak trace threshold raised to 2048 to stop the console flood.
- Browser-pane trap: the page had scrolled by 12 px (`scrollY=12`), so every ref/coordinate click on
  the navbar example trigger missed; `window.scrollTo(0,0)` before clicking fixes it.

## 22:00 — Reconcile spin root cause (rebuild #17 per-slot trace)

- Nakagin switch on #17, streak 8192, `reconcile=true`, pending queue empty. Tracker:
  `1:framework.panel.artifact#g103:---:ack1/rev0:outSome(3)`, `ready=[g103:--r-]` (not published,
  not closing, reservation held, pages present), `terminals=[g103:c--]`,
  `producer_terminals=[1:framework.section.measures:cARV]`, `deferred=[1:framework.section.tools]`,
  `close_cursor=48`.
- Cause: `drive_job_one`'s abandon branch (`slot.reconciler.is_some() && !job.is_ready()`) moved the
  artifact panel's reconcile job into a terminal and dropped the slot's `output_index` WITHOUT closing
  the ready output it had reserved. `next_ready_index` only selects `published && !closing` outputs
  and `close_step` only steps `closing` ones, so the output stayed forever, `has_work()` stayed true,
  and every later drain hit the 4096-continuation budget. Fix in `⚛️reactor/🩹️patches/🦀️.rs`:
  `close_output(state, index)` before retiring the abandoned job (rebuild #18).
- Why the reconciler was present while the job ran: `mark_rejected` (a host rejection of the stale
  patch during the document switch) installs a fresh `SurfaceReconciler` on the slot, which the
  abandon branch then treats as "newer work exists".

## 22:35 — Rebuild #19 abandon trace: the branch is the normal job retirement

- Boot on #19 (original abandon behaviour + trace): clean, no missing surfaces. The abandon branch
  fires for every surface at boot with `ready=(gen, published=false, closing=true, reservation=false,
  pages present)` — i.e. after an `Empty` transfer (tree equal to the acknowledged root) the job is
  retired through that branch, and after a `Published` transfer the output stays `published=true`
  for the host to take. Closing unconditionally (#18) therefore killed the published outputs of the
  four boot surfaces.
- Refined fix (rebuild #20): close the output in the abandon branch only when it is *stranded* —
  `!published` (a transfer that can never run because the slot already holds a reconciler; the
  Nakagin case `g103: published=false, closing=false, reservation=true`).
- #19 Nakagin (original behaviour, trace): same shape — `framework.panel.artifact#g26:---` with
  `ready=[g26:--r-]` (unpublished, reservation held), `terminals=[g27:c-e, g26:c--]`,
  `producer_terminals=[section.measures:cARV]`, `deferred=[section.tools]`; and `close_cursor=32` did
  NOT advance between streak 2048 and 4096 — `PatchTracker::close_step` is not reaching its cursor
  loop during the continuation turns (to check: early return before the cursor, or the call site).

## 23:00 — Rebuild #20: boot clean, Nakagin still spins; the strand is not created by the abandon branch

- #20 boot clean (no missing surfaces). Nakagin: same tracker shape — `artifact#g26:---:outSome(3)`
  with `ready=[g26:--r-]` — but the slot still holds `output_index`, so the abandon branch (which
  clears it) never ran for g26; the job reached its terminal (`g26:c--`) through another path (the
  closing-instance branch of `close_step` retires a slot's job into a terminal without touching an
  unpublished output whose close key differs). Rebuild #21 adds `close_stranded_outputs` to
  `close_step`: any unpublished, not-closing output whose slot has no producer/job is closed and
  its `output_index` cleared — the invariant "an output without a live publisher cannot stay open".

## 23:50 — Rebuilds #21/#22: the spin narrows to two job terminals drained too slowly

- #21 (stranded-output sweep): `ready=[]` after the switch, remaining `terminals=[g26:c--]`,
  `producer_terminals=[section.measures:cARV]`, `deferred=[section.tools]`.
- Cause of the producer terminal: `ComponentTreeProducer::close_step` completed only when the GLOBAL
  built-child retire pool was terminal-empty (no live reservations), impossible while surfaces are
  mounted, and no one else drained the pool. Fixed (`🖱️ui/🧠️runtime/…/🎭️present.rs`,
  `🧬️contract/…/🏗️builder.rs`): completion means "close queue empty" (live reservations excluded);
  the reactor turn drains one page per turn. ui-runtime laws green in isolation (whole-suite runs
  fail on shared-registry contention regardless of this change).
- #22: `producer_terminals=[] deferred=[]`, remaining `terminals=[g33:c--, g26:c--]` (tools and
  artifact job terminals, `close=true`, never empty). `PatchTracker::close_step` advanced its cursor
  one slot per turn over 64 slots, so a terminal received one retirement unit every 64 turns; a
  Nakagin-sized tree needs hundreds of units → tens of thousands of turns. Fixed: the cursor jumps to
  the first live closing terminal each turn (rebuild #23/#24).
- ui-runtime lib suite (`--test-threads=1`) shows the same 8+ failures with the close-page semantics reverted to HEAD (A/B run), so they are pre-existing order-dependent failures of that crate, not this change; every touched law passes in isolation.

## 00:20 — Rebuild #23: terminals drain, but the artifact terminal outlasts the 4096-turn budget

- Trace at streak 2048: `terminals=[g33:c--, g26:c--]`; at 4096: `terminals=[g26:c--]` — the tools
  terminal retired, the artifact panel's (hundreds of retire units) did not, and `has_work()` kept
  the actor in `more-work` for background retirement the host will never observe.
- Fix (rebuild #25): `reconcile_work` now uses `PatchTracker::has_publishable_work` (producers, jobs,
  published outputs, deferred/unadmitted surfaces, closing instances, output fault) — terminal and
  producer-terminal retirement is maintenance driven by `close_step`, which the turn now calls up to
  `PATCH_CLOSE_UNITS_PER_TURN = 8` times.

## 00:55 — Rebuild #25: the reconcile spin is gone; the real Nakagin fault surfaces

- Boot clean. Nakagin: `setActiveExample` ok (0.56 s) → completion → `refreshUi` returns in 0.37 s
  with `ui.fixed-capacity: fixed UI admission failed at scene-surface.encode: surface payload exceeds
  fixed capacity with 57281 bytes` — no 4096-continuation drain any more (terminal retirement is no
  longer "more work"; publishable work is). The world body cannot carry Nakagin in one 32 KiB
  payload → wave W-P (paged scene lanes).
- Fixes that closed the spin, in order: stranded-output sweep (#21), producer close semantics +
  reactor built-node drain (#22/#23), close cursor jumping to live terminals (#24), publishable-work
  classification + 8 close units per turn (#25).

## 23:20 → 00:10 — Fill activation, fill OOM, shell quirks (build #25 + HMR TS)

- **Fill activation fault fixed (TS).** `setActiveTool failed unknown action window instance puzzle3d-main` reproduced on a fresh boot. Cause: the `SET_ACTIVE_TOOL_ACTION_ID` branch of `🏛️ShellHost/🟦️.tsx` forwarded the bare session view state (`{ activeModeId }` only), so the plugin host's `addressed_action_view` → `ViewModel::for_window_instance` had no `windowInstances` to project onto. The utility/context-menu/generic-dispatch paths all build the base view state with `sessionWindowInstances` + `buildActiveUtilityByWindowId` and project with `windowViewContext`/`panelViewContext`; the tool branch now does the same (and `dispatchSpaceExtensionOp` too). Verified: `setActiveTool:68 ok`, `actionPane.activeToolId === "fill"`, the fill measures pane (Count slider, Hexagonal Cut group) renders, the `fillBuildTick` loop runs one tick per completed turn (~140–250 ms, in-flight gating holds). React target typecheck: 820 pre-existing errors in peer test files, none in ShellHost/ShellHelpers.
- **Fill guest OOM captured.** After 173 / 181 ticks (~25–30 s, two runs) the guest traps with `memory allocation of 16384 bytes failed` → `unreachable`; the plugin wasm is linked with `--max-memory=536870912`, so ~2.8 MB is retained per tick. The slider measure stays `ready: 0, loading: true` throughout — no plan slot ever becomes available. After the trap every later `handleAction` fails (`shard 0 worker fault … unreachable`); the tick loop keeps dispatching against the dead guest until reload. Capturing the message required freezing the tick loop from the in-page hook at the first `unreachable` (the console buffer floods in ~4 s). → wave W-F (Opus) launched with a native-law-first brief.
- **Native fill tests.** `cargo test … fill`: `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin` ok (771 turns, worst 1.0 ms); `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` overflows the 2 MiB test-thread stack (passes with `RUST_MIN_STACK=4194304`, fails at 2 MiB) — a > 2 MiB stack frame on the `fill_tool::measures` / `window_measures` path. → folded into W-F.
- **Shell quirks.** (1) First click on the ribbon tab `tool.fill` dispatches `setActiveTool { toolId: "" }` although no tool is active (tab renders pressed); the second click activates. (2) The Tool category needs two clicks before its items render; the second click fires eight sequential `handleAction`s of rising latency (181 → 2579 ms) — identified later as seven `registerBrushMesh` page uploads (650–2850 ms each, every one answering with a `notify` effect) plus `setCamera`, triggered when the Perspective window becomes active. → wave W-G (Opus) for (1)/(2); the brush-mesh upload cost and its notify refusals need a look once W-P lands.
- **Pick blocked by peer HMR state.** On the next boot the shell showed `plugin-ui.section-root-mismatch:#0`, no windows and an empty catalogue: the served wasm is build #25 while peers (W-N inspection/partial scopes, W-P paged scene lanes, W-G) are landing TS halves through vite HMR. Browser verification of pick/gumball/undo/duplicate resumes after rebuild #26 with the wave outputs.

## 00:05 → 00:40 (2026-09-10) — Build #26 served; intake stall on the Perspective surface

- **#26 boots** (`Activated puzzle3d react release: 1 completed components (changed)`; the served core carries `framework.section.catalogue`, the pool-pump trace is gone). Both windows render, the introduction tour shows (skipped via its Skip control). Boot turn timings are still serialized: `readLocalInteraction` 28.4 s / `refreshUi` 26.3 s / `setActiveExample` 27.9 s overlap.
- **Every refresh after window activation fails** with `plugin-ui.intake-budget-exhausted:1:puzzle3d-main-perspective:36864` (`🔌️PluginRuntime/🟦️.tsx` `acceptUiPatches`: budget = 4096 + max(bytes, ops×4096)×8 → the patch is one small op, and 36 864 intake steps pass in 127 ms, i.e. `OwnedUiPatchIntake.advance` never progresses — it waits for something the host loop never lets the guest deliver, which matches the paged scene lanes W-P is landing (Rust half in #26, TS half in flight). Consequence on the guest: the surface slot sits at `ack1/rev2` forever, the `surface-visible` re-admission lands in the `deferred` family, and the drain spins (`reconcile=true`, `deferred=[1:puzzle3d-main-perspective]`) until `did not publish … within 4096 continuations`. No `interactionSelect` is ever dispatched from a click (only `setCamera`), so pick/inspection/context-menu checks are blocked on this.
- **Guest-side half fixed now:** `PatchTracker::has_publishable_work` counts a deferred surface only when `deferred_surface_ready` holds (no slot, or no producer/job and ack ≥ rev) — a surface waiting on the host's acknowledgement is host-blocked and must not hold `more-work` (the ack arrives as its own lifecycle turn). Law `a_deferred_surface_awaiting_the_hosts_acknowledgement_does_not_hold_more_work` in `🩹️patches/🧪️tests/🔬️unit/🦀️.rs`; passes, as do the two earlier laws (`abandoned_reconcile_job_closes_its_ready_output_so_the_tracker_can_idle`, `a_closing_terminal_does_not_wait_behind_sixty_three_empty_slots_per_unit`) now that the plugin-host lib test target compiles again.
- **Plugin-host tracker suite is order-dependent:** `patches::tests` = 19 passed / 15 failed with 2 threads, and the same 15 fail single-threaded even with my three laws skipped (`mounted_output_admission_* … called Result::unwrap() on an Err value: SurfaceId("75:direct")` — the process-wide mounted-output pool is left full by an earlier test in the run: `effects_publish_in_admission_order…`/`generation_max…`/`issued_obsolete…`); every one of them passes alone. `issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner` also overflows the default 2 MiB test stack (needs `RUST_MIN_STACK=64 MiB`). Not caused by this ticket's tracker edits (they pass with the edits when run alone); recorded for the gates note.
- W-N landed (puzzle3d suite 613/9, gumball verbs refuse without selection at the cause, one scope table); W-F, W-G, W-H, W-P running. Next rebuild (#27) after W-P.

## 00:20 → 00:40 — The user-facing entry `bun dev:puzzle:3d`

- `bun dev:puzzle:3d` → `workspace:dev -- 3d` → root `📜️script.ts` `runFrameworkOsPlaygroundDev` → `runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:dev", "--", "puzzle3d"])`. **Failed before any build**: `NX Failed to load 1 Nx plugin(s): …📚️library/🟨️.mjs: require() async module … is unsupported` from `buildProjectGraphAndSourceMapsWithoutDaemon`. Cause: `runCmdInternal` (`📚️library/🏃️process/🟦️.ts`) resolved `nx` to `node_modules/.bin/nx`, bypassing the workspace's `package.json` `nx` script (the caching bootstrap that owns the daemon-served project graph the async ES-module inference plugin needs). Fix: `workspaceScriptExists(name)` — a name declared as a workspace script is never resolved to a same-named bin; law `process runner reaches workspace scripts before same-named bins` in `🧪️tests/⏱️process-budgets/🟦️.ts` (passes; asserts `bun nx --version` reaches the bootstrap).
- Second run: the bootstrap now runs `nx watch` for the dev target and fails with `NX Daemon is not running. The watch command is not supported without the Nx Daemon` → `Nx source watcher exited before readiness`. `bun nx daemon` confirmed no daemon; `bun nx daemon --start` brought it up (pid 22281). Third run in progress with the daemon up — if it boots, the remaining gap is that the bootstrap must start the daemon itself before watching (a user's first `bun dev:puzzle:3d` on a fresh machine must not depend on a manual `nx daemon --start`).

## 02:30 → 03:00 — Build #26 + W-P4/W-H3/W-G TS via HMR

- **Boot**: `refreshUi` 873 ms (was 26.3 s), `readLocalInteraction` 1.5 s (was 28.4 s), `setActiveExample` 1.2 s — W-P4's document-priced intake ceiling + 1024-step yield stride. Both windows render; the tour shows once.
- **Fill is armed at boot** (W-G restores the dock arrangement's `tool.fill` leaf and arms it): 50 `fillBuildTick`s in the first seconds; deactivated it by one press to keep the guest alive (fill OOM → W-F4).
- **Pick works on the first click** now: `interactionSelect` 409–499 ms, selection `seed-left-001` (object granularity) highlighted in both windows; no `registerBrushMesh` refusals at activation any more (W-H3 TS half).
- **Inspection panel** opens from its dock button: `INSPECTION → Schema puzzle3d.fixture / Domain architecture / Objects 1` (document tree; no per-object fields for the selection — checklist §16 expectation still open).
- **Context menu** on the selected object: Set Active Example, Add Object…, Duplicate Selection (mod+d), Translate Selection, Rotate Selection, History›, Hand›, Selection›, More›, Delete Selection (backspace).
- **Duplicate Selection** (#26 wasm, pre-W-N): `duplicateSelection` 282 ms, scope `none`, no effects, no visible clone, Objects still 1 — the W-D4/W-N no-op; W-N's cause-level fix is in the tree, verified after #27.
- **Undo / redo** (`cmd+z`, `cmd+shift+z` in the Perspective window): `undo` 224 ms / `redo` 245 ms, both scope `full` — dispatch and completion work; the history panel lists 12 rows (`delete-object id=seed-left-001 ↶`, Set Active Tool, Resize Window, Switch Panel Tab, …).
- **New admission fault**: every `refreshUi` after the duplicate fails with `ui.fixed-capacity: fixed UI admission failed at history-panel.command-label` — the history row label (`entry.label`, plus ` xN` when folded) is authored copy that can exceed 512 bytes; now `UiText::clipped` like the description (plugin host `ui_history_panel`), law `ui_history_panel_clips_an_oversized_command_label_and_its_folded_count`. Ships with the next wasm (#28).

## 09:20 → 09:50 — Build #27 (W-N + W-H3 Rust + history label clip; compiled 03:12, before W-F4 finished)

- Boot under a concurrent release compile (#28): `refreshUi` 5.0 s, `readLocalInteraction` 16.6 s, `setActiveExample` 11.3 s; both windows render after ~60 s; W-H3 re-pages the seven brush meshes once after the guest starts (`registerBrushMesh` ~2 s each, no refusals).
- **Restored dock profile arms Fill at boot** (W-G hydrate-arms); with Fill armed the world blocks picks by design, so a click only orbits (`setCamera`) and *Duplicate Selection* answers `notify: "Nothing is selected"` — W-N's cause-level refusal is live. From a user's seat this reads as "clicking does nothing" until the Fill tab is pressed off; recorded for the checklist (§12).
- **History label clip confirmed**: `undo` 181–217 ms / `redo` 194–205 ms (scope full) and every following `refreshUi` succeeds; history rows re-render.
- **Shell/guest tool desync**: the shell state showed no active tool while the guest still ran the fill (measures pane + ticks); pressing the Fill tab twice (`setActiveTool fill` → `setActiveTool null`) resynced it.
- **Fill on #27** (partially W-F4?): after arming, `Cancel fill` appeared and the inspection tree went `Objects 1 → 5` — the plan produced placements for the first time in a browser — then the guest died (`interactionSelect` → `shard 0 terminated`, `setCamera` → `actor-activation.revoked`); the trap text was lost in a 4 000-line console flood. Re-run on #28 (all of W-F4) with the freeze hook.

## 09:30 → 10:10 — Build #28 (W-F4 fill census + cancel-token fixes)

- **Fill now plans in the browser**: after arming, `fillBuildTick` runs at 280–450 ms per tick, the count slider reports `ready: 12–15` (first non-zero readiness ever observed in the app) and `Cancel fill` shows; the inspection tree stayed `Objects 1` because the count slider was left at 0 (reveal semantics).
- **New trap**: ~76 ticks (~40 s) after arming the host prints `[DEBUG] plugin job semio.puzzle3d.fill#490 exceeded its 65536-step host budget — cancelling`, and the very next turn traps the guest (`unreachable` → `shard 0 lost, restoring actors` → `actor-activation.revoked` on every later call). Two defects: (a) the host's isolated-job driver caps a bounded job's *lifetime* at 65 536 steps — a fixed host constant, not the plan's own size — so any real fill plan is cancelled mid-way; (b) the cancel path panics inside the guest. → wave W-F5.
- **Console flood**: 7 000+ lines per minute are the host's `[DEBUG] thunk start/done command-ingress` pair per turn (plugin TS package) plus the peer's per-tick `[DEBUG] fill_build_tick … changed= faulted=` eprintln; they push the trap text out of the 500-line buffer within seconds even with the tick loop frozen.

## 10:30 → 10:55 — Build #28, clean profile: example switching

- Clean profile (no restored dock arrangement): boot `refreshUi` 618 ms / `readLocalInteraction` 1.2 s / `setActiveExample` 915 ms; no Fill armed; both windows mount after ~100 s (the Perspective body always mounts last, ~1–2 min after Top under load).
- **Example switch renders nothing**: Nakagin → `Objects 180` in the inspection tree but an empty world; back to Concrete Forest → `Objects 1`, still empty. `setActiveExample` answers scope `none` both times and the scene surface is never republished after boot. → W-P5 (launched with this evidence).
- With Fill armed from a restored profile the guest dies ~40 s after boot (`65536-step host budget` cancel → `unreachable`), which had masked the example-switch finding on the previous runs. → W-F5.

## 11:00 → 11:15 — Build #29 (W-F5 guest half; host lifetime cap gone)

- Activation printed `(unchanged)` although the core wasm was rebuilt (8m17s, new mtime, the removed `fill_build_tick` trace string is gone): the activation receipt hashes the descriptor, not the module bytes — `Activated … (unchanged)` only proves an unchanged descriptor. The wedged `:6013` server (curl timeout 10 s, seen by W-P5 too) was restarted by the serve step.
- **Fill on #29**: armed after a clean boot (the first Fill press on the restored tab disarms, the second arms); the plan advanced for **184 ticks / 44 s** (250–350 ms per tick, count slider `ready: 31`) — past the old 76-tick lifetime cap. Then every tick answered `plugin.command-page-allocation: fixed command page authority could not reserve its exact 64 slots` (32×) and two seconds later the guest trapped (`unreachable`). → wave W-F6 (command-page retirement under a 4 Hz command stream; exhaustion must refuse, not trap).
- Boot on a clean profile: `readLocalInteraction` 1.2 s, `refreshUi` 0.7 s, `setActiveExample` 1.0 s; the Perspective body mounts ~60–90 s after Top (scene republish latency, W-S2).
