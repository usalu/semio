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
