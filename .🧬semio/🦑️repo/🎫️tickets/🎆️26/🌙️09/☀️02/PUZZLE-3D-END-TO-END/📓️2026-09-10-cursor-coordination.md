# Cursor coordination fleet (2026-09-10, 00:25, cursor-chat / Fable 5)

A second coordination session joined per the dev's instruction; it works in conjunction with the
existing fleet (W-F fill OOM, W-G shell tab, W-H brush mesh, W-N landed, coordinator runtime
verification) and does not duplicate live waves.

## Waves owned by this session

| wave | model | scope | report |
|---|---|---|---|
| W-P2 | cursor-grok-4.6-xhigh | takeover of the stalled W-P paged scene lanes (TS half + intake stall `plugin-ui.intake-budget-exhausted`), only after 15 min file-cold liveness check | continues `📓️2026-09-09-wave-P-paged-scene-payload.md` |
| W-U | cursor-grok-4.6-xhigh | editor correctness: gumball undo coalesce, relocate ActionKind honesty, outliner continuation-row cursor, inspection ids bound, retained-jobs fixture | `📓️2026-09-10-wave-U-editor-correctness.md` |
| A1 | composer-2.5 | read-only re-verification of the 25-section user-feature checklist against current source | `📓️2026-09-10-checklist-reverification.md` |
| A2 | composer-2.5 | read-only audit of the order-dependent test failures (plugin-host patches pool, ui-runtime registry, 2 MiB stacks) | `📓️2026-09-10-order-dependent-tests-audit.md` — done 00:32 |
| W-O4 | cursor-grok-4.6-xhigh | implements A2's designs: patches output-pool test guard + drain seam, ui-runtime registry guard; `RUST_MIN_STACK` 128 MiB floor already landed in `runCargoTestBudgeted` (coordinator) | `📓️2026-09-10-wave-O4-test-isolation.md` |

00:40 takeovers: the prior fleet's W-F / W-G / W-H went silent 18–28 min (only the runtime-verification
coordinator is still writing). W-H died mid-refactor leaving the 6013 serve broken (three consumers
import the deleted `registeredPuzzle3dBrushMeshes`). Launched W-H2 (restore module graph + finish the
brush-mesh registry consumers, then probe boot) and W-F2 (fill OOM: retention fix, bounded-heap law,
carrier boxing, hot-loop eprintln cleanup). W-G takeover deferred until the serve boots again.

| W-H2 | cursor-grok-4.6-xhigh | continues `📓️2026-09-10-wave-H-brush-mesh-reannounce.md` |
| W-F2 | cursor-grok-4.6-xhigh | continues `📓️2026-09-09-wave-F-fill-oom.md` — first instance died (ping timeout) 01:21 leaving a non-compiling `Puzzle3dFillSession` (E0509 move-out-of-Drop) + written §3.2 plan; relaunched. Coordinator removed a duplicate `use std::collections::HashMap;` at editor.rs:23 to unblock the shared crate. |

## 10:00 — ONE root cause consolidates: framework reserved jobs never driven (undo + selection + everything downstream)

W-AB proved the pick chain host-side (marquee `setPointerCapture` delayed until 4 px drag so r3f sees
the click; AABB fallback must not fire empty-select on miss): a Perspective click on the Forest table
dispatches `interactionSelect {domainId: vortex, granularity: object, id: seed-left-001}` — but the
guest never publishes the InteractionView (table gray, Inspection empty, copy silent). Coordinator
grep: `interactionSelect` is ALSO a framework reserved job (`FrameworkInteractionSelectJob`,
plugin/🦀️.rs) — the same never-driven lane as `FrameworkUndoJob`. So ONE pump fix (W-G3, in flight)
should unblock: undo/redo, selection publication, and therefore copy→paste browser-proof, locked
refusal, gumball. W-AB rescoped to the three non-blocked items: real `exportFixture` activation +
download/import round trip, brush via the Perspective utility bar, suggestions via Alt+right-click on
a hovered vortex marker. Serve :6014 = wasm #33, healthy.

## 09:50 — undo root narrowed: framework reserved job never driven; W-G3 on the pump

Wasm #33 (instrumented) probed twice (`probe-2026-09-10T07-33-57.md`, `07-36-34.md`): the guest's
`commit_framework_history_route` eprintln NEVER fires, and no `spawn-job` effect is routed on undo
(coordinator traces in PluginRuntime saw zero spawns across 4 dispatches). So undo's ingress enqueues
the internal `FrameworkUndoJob` (`ArtifactReservedToolJob`, plugin/🦀️.rs ~:14483/:14531) and settles
command-complete, but nothing ever drives the reserved job to its commit — third face of the
"guest work needs a host pump" gap (fill ticks, isolated steps, now framework reserved jobs). W-G3
resumed with the full evidence chain. Temporary traces live: guest history-route eprintlns (#33),
PluginRuntime spawn-job/job-done console logs, ShellHost undo-route log — all [DEBUG]-prefixed,
strip at close-out.

## 09:28 — ⚠️ PEER SESSIONS: please scope serve kills to your own port

At 09:26 a peer session's `serve-release-direct.sh` run killed the :6014 vite (my serve) via its
kill-by-pattern step (`vite --configLoader` / `script.ts serve puzzle3d react release` match BOTH
ports). I restarted :6014 (wasm #32, cursor-session serve, harness-managed). If you redeploy :6013,
please kill by PORT (`lsof -tiTCP:6013 -sTCP:LISTEN`) or by your own pids — not by pattern. I never
touch :6013.

## 09:35 — FILL APPLY BROWSER-PROVEN (W-G3); wasm #32 live: switch label verified; undo still a store no-op → instrumented rebuild #33

W-G3 §8.9: the 120 ms `fillBuildTick` interval was a mirage — `createInFlightSkippingInterval` stayed
in-flight for the whole ~10 s command ingress, starving Isolated `step-job`. Host now pumps steps
continuously around `driveSpawnedJob` (32/admission, 2 ms guest budget intact) and only runs
`fillBuildTick` when a UI poll is due (every 128 steps). Browser proof `probe-2026-09-10T07-01-09.md`:
End → Count 1 (`1/1000`), history `create-object` + `SetFillCount {count:1}`, treeItems 4→23,
FAULT_RE=0. First fill unit ~77 s after activate (was ~50 min projection). Open: `setActiveTool ""`
mid-flow bounce; `requestContextMenu` skips host tool overlay.

Rebuild #32 deployed; coordinator recycled the (re-)wedged :6014 vite and probed
(`probe-2026-09-10T07-17-27.md`): **"Set Active Example↶" label + one document row browser-verified**
(W-AA's fix works). Undo remains a silent store no-op (4 dispatches route local → guest
command-complete → nothing changes). No skip diagnostics in console. Coordinator instrumented
`commit_framework_history_route` + `dispatch_group_history_action` in `🔌️plugin/🦀️.rs` with
`[DEBUG]` eprintlns (tail_group_id, undone/skipped, benign-collapse) — guest check green (1 m 16 s) —
and launched rebuild #33. Decision point after probe: which branch swallows the undo.

## 09:02 — W-AA guest fix landed (laws green); rebuild #32 launched

W-AA: the switch's coalesced Complete emit had no `description`, so History backfilled the label from
the first op (`delete-object id=seed-left-001`); now labeled "Set Active Example". `Resize Window` is a
HOST Shell chrome row, not a guest undo step (guest emits no `window_config_mutations`). New law
`set_active_example_history_is_one_set_active_example_row` + W-X chunk/undo family: 8 passed
(RUST_MIN_STACK=16777216). Report `📓️2026-09-10-wave-AA-undo-history.md`.

Rebuild #32 launched by the coordinator (harness-managed chain: component-release with retries →
support → materialize → prepare → activate; NOT the old rebuild-until-ok.sh, whose kill-by-pattern
would murder the peer's :6013 vite). After activate: restart the :6014 serve, then browser-verify
switch label + one-row + undo-restores (the store-side undo no-op suspect) on wasm #32.

## 08:45 — fill arms + ticks in browser (W-G3); serve :6014 wedged + restarted; cadence is the last fill gap

W-G3 landed: `hostArmedViewContext` on dispatch + full refresh after `setActiveTool` (arming), and
batched Isolated `step-job` 32/admission (was 1 — 5 steps/30 s starved `semio.puzzle3d.fill`). Guest
untouched. Laws: window-view-context hostArmed=3 TS+Rust twin, isolated-admission vitest 1/1.
Report `📓️2026-09-10-fill-build-host-tick.md` §8.8.

The vite serve on :6014 wedged during W-G3's last edit (97% CPU, HTTP 000, 105 min uptime) — the
coordinator killed ONLY the :6014 pair (38973/38975) and relaunched the identical serve
(harness-managed, answering 200). Coordinator re-probe (`probe-2026-09-10T06-41-09.md`): fill session
arms ("Cancel fill" row, treeItems 3→4), armed guest ticks print `active_tool=Some("fill")` (console
line-wraps — read multi-line), zero faults. Remaining: **tick cadence** — ~1 armed tick / 10 s vs
~320 ticks needed natively for first ready unit → Count pinned at 0. W-G3 resumed on host pump
frequency (not guest cap bumps).

## 08:15 — coordinator landed the shell-Undo host fixes; dispatch chain browser-proven; store-side gap remains

W-AA stalled 3× on recon, so the coordinator took defect 1: (1) Interpreter now mounts non-treeItem
control children of tree items into `TreeDataItem.control` (the History `.run` button + filter select
were silently dropped) with row-click forwarding to a single enabled activatable child; (2) framework
`mod+z`/`mod+shift+z`/`mod+y` chords in ShellHost after the app-keybinding loop. Browser-proven: all
four undo funnels reach the routing (`route:"local", canUndo:true` ×4) and settle guest
`command-complete` (`probe-2026-09-10T06-08-08.md`). BUT the store visibly does nothing: history stays
2 rows, Nakagin stays, order frozen — suspect `commit_framework_history_action`'s silent
`NothingToUndo/ForeignEdit → UiDirtyScope::None` branch or the coalesced-group undo path
(`🔌️plugin/🦀️.rs` ~:21979). W-AA is rescoped to guest history hygiene + one-undo-restores laws which
must expose this natively. Details in `📓️2026-09-10-wave-AA-undo-history.md` coordinator section.
Temporary `[DEBUG] undo route` log lives in ShellHost until undo is browser-green.

## 07:56 — W-Z checklist survey complete; W-AA + W-AB fix waves launched

W-Z dispositioned all nine items (`📓️2026-09-10-wave-Z-browser-checklist.md`), zero FAULT_RE hits, no
guest rebuild. Verdicts: item 9 closed (presence accent); items 1–8 defect/partial. Highlights:
shell Undo inert (`framework.history.undo.run` child never mounts; Meta+Z doesn't reach
`handle_action("undo")`), switch leaves TWO history rows (`Resize Window` + `delete-object` label
instead of "Set Active Example"), paste opens an Execute form instead of cloning (missing
`args.fragment`), export/import live as numbered workspace-menu rows `4 Export`/`5 Import`,
brush/gumball/lock/suggestions never reached their real chrome (Utilities bar / inspector /
vortex marker).

Fix fleet launched (both grok):
- **W-AA** — undo/history: shell Undo mounting + mod+z (host), switch label + one-row coalescing +
  one-undo-restores law (guest; rebuild flagged, not deployed).
- **W-AB** — interaction chrome: paste fragment auto-apply, marquee selection proof, export/import
  round trip via filechooser, locked refusal, brush + gumball via perspective Utilities, suggestions
  via vortex marker.
W-G3 still on fill arming/ticks. Serve :6014 stays up; deploys remain with the coordinator.

## 07:40 — red Perspective banner resolved: NOT a defect (checklist item 9 closed by coordinator)

DOM inspection on :6014 (`🗑️generated/tab-inspect.ts`): the Perspective `mode-dock-tab` carries no
destructive/red class and transparent computed background; its icon paths are `M3 18h18` +
`M5 18 12 6l7 12` + two circles = the perspective/vanishing-point glyph, NOT lucide `triangle-alert`
(no exclamation paths `M12 9v4`/`M12 17h.01`). The red highlight in screenshots tracks the
last-focused elements (Perspective viewport, Fill tab, Tool category, drag grips) — it is the
local-user presence/focus accent, by design. W-Z: strike checklist item 9.

## 07:25 — W-G3 tool body browser-proven; fill build never ticks in browser; W-Z checklist wave launched

W-G3 fixed the empty tool body with browser proof on :6014 (renderer-TS only, no wasm rebuild): apply
`drillOnOpen` on every closed-host open, forward through `Panel`, `resolvePanelBranchBodyLeaf` renders the
remembered/first leaf. Proof `🗑️generated/probe-2026-09-10T05-19-26*`: tab=tool.fill, treeItems=3, body
"Count | 0 | Hexagonal Cut Concrete Forest Left | 100% | Distribution", faults=0. Suite 639/1.

Coordinator follow-up probe (`probe-2026-09-10T05-22-32.md`): Count slider stays 0/1000 after 40+ s with
tab open and after keyboard End on the slider; the guest's leftover `[DEBUG] fill_build_tick` eprintln never
prints → **fillBuildTick is never dispatched in the browser** (guest chain proven natively by W-F2c). Either
the tool never ARMS (tab select ≠ setActiveTool; the in-tree `toggle-group-item#tool.fill` was removed as a
duplicate) or the host never schedules the retained fill job. W-G3 resumed on this (owns pressed-state/arming).

W-Z (grok) launched in parallel on the remaining checklist: example-switch undo one-step, clipboard,
marquee vs pick, import/export, locked-volume gumball refusal, brush, gumball drag, suggestions window,
and the red Perspective tab banner. Probing :6014 with flag-gated probe extensions; no serve restarts,
guest changes (if any) law-proven only, deploys stay with the coordinator.

## 06:40 — 🎉 NAKAGIN RENDERS (wasm #31); W-G3 launched on the empty tool body

W-X symmetry fix landed (shared packedTextLeaf/packed_text_leaf; suite 639/1 foreign-only). Rebuild
#31 + probe: boot clean, pick/context clean, **Nakagin Capsule Tower renders in both viewports**
(~50 s switch, zero faults) — screenshot probe-2026-09-10T04-30-13-example-switch.png. Remaining
browser gap: the fill tool body is EMPTY in the shell bottom panel (tab opens, no slider/params/toggle
in DOM) though guest tool bijection + fill family (59/0) pass natively — new wave W-G3 owns it plus
wave-G's two pressed-state defects. After W-G3: rebuild #32 + full checklist (fill ready/apply, switch
undo one-step, clipboard, marquee, import/export, locked refusal, brush, gumball, suggestions).

## 05:30 — all waves landed; #30 probed: packed-leaf asymmetry blocks boot; W-X resumed

W-F2c: fill family 59/0 (lib 635/5 at the time). W-Y: all six defects fixed-with-laws (lib 638/2,
both reds foreign at the time). Rebuild #30 probe: BOOT BLOCKED — repeating
`SyntaxError: Unterminated string in JSON at position 511` at `sectionValueFromBuiltNode`: W-X's
33-slice packed text leaves are not unpacked by the section-value consumer (also the native red
`reserved_refresh_section_payloads_admit_into_the_retained_section_carrier`, previously misattributed
as foreign). W-X resumed to make pack/unpack symmetric via shared helpers in every consumer.

## 04:55 — W-X switch pipeline landed; rebuild #29 probed; next wall: surface reconcile capacity

W-X fixed both switch defects (8-item chunks/kind, one coalesced undo, Interpreter successor-hash
cache; suite 620/10). Rebuild #29 + probe: the coalesced Publish lands (op 94, full scope), but the
refresh now dies at `retainedUiRefreshEffects` with SurfaceReconcileLimits refusals — world surface
129 nodes > max_nodes 128 (viewports keep the old scene); framework.panel.artifact 8 419 293 B >
8 388 608 max_bytes on interactionSelect; framework.section.measures NodeCapacity. W-X resumed on this
cluster (bound surface node/byte scaling at the cause; ui-runtime ♻️reconcile region — peer
coordinator + W-O4c guards live there, surgical edits only). Probe FAULT_RE now catches `Credits {`,
`NodeCapacity`, `SemioFaultError`.

## 04:25 — W-F2c retention landed; W-O4c done; wave W-Y launched

W-F2c: fill envelope retention FIXED (5 admissions / ~3.5 MiB over 320 ticks, was 2.8 MiB/tick);
resumed to chase the remaining `ready=0` cause (isolated FILL_JOB_KIND completes with an EMPTY
FillBuilder.sequence) + the 2 MiB stack law + full suite. W-O4c: patches 34/34 and ui-runtime 122/0,
single-thread + default + alone (deadlock/poison stay #[ignore], pass via --ignored). New wave W-Y
(grok) owns the A1 unowned list: worldRelocate Nakagin extent, clipboard, import/export, context-menu
fallback, marquee rectangle, locked-volume gumball refusal. W-X carve-out: setActiveExample work loop
+ host refresh/scene republish. Note: a foreign bun now serves :6013 (our serve was SIGTERMed); same
activated release build on disk, leaving it shared. Disk: 136 GiB free after cleanup.

## 03:45 — W-U3 landed; wasm #28 probed; disk incident; wave W-X launched

W-U3 closed all five editor-correctness defects (gumball coalesce, ActionKind honesty, outliner +
inspection paging via setPanelPage, fixture verdict) — suite 618/13. Disk hit 100% full (cause of the
silent serve deaths + cargo incremental corruption); freed ~104 GB of stale coordinator target dirs.
Rebuild #28 + probe: ZERO faults everywhere incl. Nakagin switch (arena fault gone), BUT the world
never re-renders after setActiveExample — two defects (one-op-per-roundtrip Migrated job decomposition
~1.5 s/item, and partial-windowBodies refreshes never republishing the world-3d scene surface).
Wave W-X (resumed W-U3 agent) now owns both. Details: 📓️2026-09-10-wasm27-browser-verification.md §0.

## 02:40 — wasm #27 browser verification (see 📓️2026-09-10-wasm27-browser-verification.md)

Boot + pick + context menu + Concrete Forest: **zero faults** — the scene-lane and intake fixes hold
in the browser. Nakagin switch: scene-surface/intake faults GONE, but the publish aborts at
`history-panel.row-action-args` — `UI_VALUE_ARENA` exhaustion caused by unpaged outliner/inspection
rows (~140 volumes × arg maps). That is W-U3 defects 3+4; history is the victim, not the hog. Do NOT
bump arena constants. Serve on :6013 now runs harness-managed (nohup'd serves were being reaped).
W-P2c verified all scene-lane laws green natively (866 B spine + 9 pages for a 73 439 B Nakagin doc).

## 01:53 — infra instability + coordinator-driven unblock

Between 01:21 and 01:51 the subagent service degraded: repeated `PING timed out` kills and agents
ending their turn after a single reconnaissance step. Net effect: W-F/W-H/W-U/W-P2/W-O4 and their
relaunches all died or stalled without finishing. The coordinator therefore drove the load-bearing
compile unblock directly (evidence: precompute was 26 min cold, error unchanged — agents were not
progressing):
- Removed a duplicate `use std::collections::HashMap;` at editor `🦀️.rs:23` (canonical grouped import
  survives at :56).
- Fixed the `Puzzle3dFillSession` E0509 in `⏳️precompute/🦀️.rs` `install_fill_session`: fields are now
  taken through a `&mut` borrow (`.take()` / `mem::take`) so the `Drop` type is never partially moved
  out of and its Drop is a clean no-op once emptied (this also completes W-F §3.2 step 1).
- The `📌️panels/🗿️artifact` `setPanelPage` errors and the ui-runtime `slot` E0425 resolved via peer
  convergence. **`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` is GREEN
  (35.8 s, 01:52).**

Rebuild #27 launched 01:53 (`rebuild-until-ok.sh` → `rebuild-27.txt`) from the green tree — carries the
scene-lane guest half, brush-mesh registry, and all landed fixes. Fresh waves launched on the green tree
(fresh launches survive the flaky infra better than resumes):
- W-U3 (grok) editor correctness — 5 defects
- W-F2c (grok) fill retention steps 2–5 + bounded-heap law + carrier boxing (step 1 done by coordinator)
- W-P2c (grok) scene-lane verification + intake stall
- W-O4c (grok) test isolation families 1–2

A1 done 00:32 (`📓️2026-09-10-checklist-reverification.md`): unowned defects ranked — worldRelocate Nakagin work-item cap, clipboard (copy/cut/paste), import/export UI, setActiveExample runtime reset, context-menu shell-fallback conflation, marquee rectangle vs pick, locked-volume gumball refusal. Queued as wave 2 of this fleet once W-U/W-H free the editor + host TS files.

Claim protocol: each wave re-reads files immediately before editing, appends progress to its report
early (stake), and backs off any file modified by a foreign agent within the last 10 minutes.
No wave rebuilds the wasm component or touches the running serve on 6013; rebuild #27 stays with the
coordinator(s).

## 10:55 coordinator — #34 deployed; undo STILL unreached; actor-ingress gate identified
- Rebuild #34 (W-G3's `drive_self_waking_ready` + ownership skip) built first-try, activated; :6014 recycled and serving.
- Probe `probe-2026-09-10T08-26-22.md`: 4 undo dispatches route local, each settles `command-complete`, ZERO `[DEBUG] history route` eprintln, navbar unchanged. Guest stderr confirmed flowing (cooperative-maintenance trace visible), so the commit path is genuinely unreached.
- New root-cause lead: browser undo goes through `dispatchDirectBrowserActorCommand(windowActionInvocation(...))` (ShellHost ~5717), NOT `plugin.handleAction`. Guest actor-ingress path gates on `registry.window_action(window_kind_id, action_id)` (plugin host 🦀️.rs ~25088) — undeclared reserved verbs likely no-op with a clean complete. W-G3's ownership skip only covered `handle_action_invocation`.
- W-G3 resumed: trace actor ingress, extend reserved-verb routing to that path, native law simulating actor admission of undo on a non-declaring window kind, no wasm rebuild (coordinator runs #35 after land).
- Serve :6014 = wasm #34, healthy. Do NOT kill by pattern; kill by port only.

## 11:05 coordinator — W-AB round landed; resumed on zoom probes + :6014 export dispatch
- Item 4: host download path FIXED with laws (`mediaExportEncodingText` coerce, in-document anchor, 10s URL revoke). Open: Export row never reaches `onAction` on :6014 (worked on :6013) — W-AB re-tasked to root-cause; if guest-side, folds into rebuild #35.
- Item 6: brush chrome PASS (Utilities unfold, Brush active + Overlap budget). Placement = vortex click in brush mode; default camera too far — W-AB adds camera zoom probe step.
- Item 8: suggestions gesture confirmed (Alt+right-click no-hover correctly → Workspace Menu); host hover arming landed. Needs zoomed hover — same probe step.
- Note for probes: `[id="framework.window...unfold"]` attribute selector required, `#id.with.dots` is wrong CSS.

## 11:15 coordinator — W-G3 landed actor-ingress fix; rebuild #35 launched
- Real gate found: `addressed_action_view` in `plugin_exchange` requires a live `window_instances` row; navbar/history undo addresses a window the guest view lacks → fault swallowed, `command-complete`, `handle_action_invocation` never ran. WIT `reactor::poll` also wasn't wrapped in `drive_self_waking_ready`.
- Fix: `admit_addressed_action_view` — strict window check for catalog actions, unprojected-view fallback for reserved verbs (undo/redo/interactionSelect/…), used by BOTH `plugin_exchange` and `plugin_handle_action`; `drive_self_waking_ready` now wraps WIT `reactor::poll` too. Laws 3/0 incl. new `reserved_undo_actor_ingress_admits_undeclared_window_kind`. Details §8.11 of 📓️2026-09-10-fill-build-host-tick.md.
- Rebuild #35 building now; activate will swap assets under :6014 — W-AB's fresh page loads after activate get #35 (superset, fine). Coordinator re-proves undo + selection after deploy.

## 11:35 coordinator — undo hang PINPOINTED: handleAction promise never resolves; reserved lane → spawn-job conversion
- Deploy chain fully verified clean: served wasm on :6014 hash-identical to dist #35, contains W-G3's fix (panic-string grep). NOT a staleness problem.
- Host funnel traces (probe 09-27-53): undo routes local → plugin found → `directActor:false` → `plugin.handleAction` called → promise NEVER resolves (no response, no rejection, no fault). Other actions settle fine around it. The reserved job parks on non-self-waking work (store history dispatch on later turn); nothing pumps it; no more-work/spawn-job emitted.
- Architectural verdict: in-stack reserved lane is structurally wrong in browser. W-G3 re-tasked: convert reserved verbs to the host-driven spawn-job lane (same machinery as fill, browser-proven), handleAction answers first-turn. Rebuild #36 after land.
- Unrelated: peer's half-landed rename `pluginUiIntakeStepCeiling`→`retainedUiIntakeStepCeiling` broke boot (missing export); coordinator followed the rename in PluginRuntime (4 refs). Boot restored.
- ShellHost now carries 5 more `[DEBUG] undo funnel*` traces + 1 `[DEBUG] actor dispatch` trace — close-out sweep targets.
- :6014 wedged once more mid-round; recycled (kill by port only).

## 11:40 coordinator — W-AB round 3 landed; resumed for final captures
- Landed: per-window Frame control (`world3dFrameCameraFromInstances`) + centroid law; context-menu rows expose `id`; Actions-pane Export DOWNLOADS `puzzle-3d.json` (7542 B) on :6014.
- Open, W-AB resumed: Frame re-capture, brush placement with framed camera, suggestions hover, workspace-menu Export row onSelect gap (Import row works, Export row inert — row-activation class of defect).
- Undo/selection remain expected-broken until W-G3's spawn-job conversion + rebuild #36.

## 12:15 coordinator — W-AB round 4: Export + Frame PROVEN; vortex hit-testing is the last W-AB thread
- Export FIXED + browser-proven on :6014 (menu rows inherited `pointer-events: none` from dimmed chrome; `pointer-events-auto` on leaf + active ContextMenuChrome body; puzzle-3d.json downloaded). Frame control PROVEN (instances-group AABB, table fills Perspective). Laws 4/0.
- Remaining: vortex marker hit-testing (blocks brush placement item 6 + suggestions menu item 8) — clicks on visible geometry never dispatch `addBrushObject`, hover never arms. W-AB resumed to root-cause the r3f pick path / hit proxies.
- Checklist item 4 residual: numbered Import lands `importFixture {}` (no file picker) — assess after reserved lane lands.
