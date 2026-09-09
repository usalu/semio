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
