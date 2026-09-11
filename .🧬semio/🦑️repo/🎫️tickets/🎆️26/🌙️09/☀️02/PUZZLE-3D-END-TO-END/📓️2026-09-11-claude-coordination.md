# 🧩️ Claude coordination (2026-09-11 14:15, claude-code / Fable 5.1, session 8acfb894)

Third coordination session on `26/09/02/PUZZLE-3D-END-TO-END`. Works in conjunction with the cursor
fleet (`📓️2026-09-10-cursor-coordination.md`, last write 12:27 — W-G3 §8.30 + W-AB #44 hops "in flight",
no #44 wasm built: newest release wasm is `dist/release/🔌️plugin-modules/🧩️puzzle` 2026-09-10 23:23 = #43)
and one other live claude session (`semio-a5`, task unknown). Repo MCP failed to connect
(`invalid initialize params`) — ticket bookkeeping is manual on disk.

## Ground truth at start (14:06)

| item | state |
|---|---|
| HEAD | `46c3cb9de0` 2026-09-11 12:39 (auto-commit); puzzle 3d guest tree clean vs HEAD |
| serves | `:6013` (pid 26789, orphaned `serve puzzle3d react release`, 1 day) and `:6014` (cursor session) — both serve the same release page + #43 wasm from `dist/release` |
| load | 17 / 27 / 35, no cargo running |
| build dir | shared `.🧬semio/🦑️repo/⚡️cache/cargo` since 2026-09-11 — never set `CARGO_TARGET_DIR` |
| last battery | #43 on :6014 (2026-09-10 21:24Z): boot/example/undo/redo/clipboard/suggestions/gumball-handle PASS; FAIL: first-pick `selectedIds` empty → Inspection empty, lock chrome, gumball `translateSelection` pre-admit fault, brush preview wipe, importFixture no-op; `1:window` host-context storm fixed vite-live |

## Interpretation of the task

"Every window works" = the two viewports (Top, Perspective) and every panel (Tool/Fill, Inspection,
Artifact/outliner, Catalogue, Settings, History) on both examples (Concrete Forest, Nakagin).
"Every tool works" = checklist §1–§25 of `📓️2026-09-09-user-feature-checklist.md`, browser-proven by
`🔍️browser-probe.ts --battery` (extended where the battery has no step yet).
"Optimize architecture where performance limits are reached" = the documented ceilings (fill tick
cadence / OOM, scene-lane publication, intake budget, `PUZZLE_COMMAND_WORK_ITEMS` on Nakagin,
command-page authority) get a design + implementation, not a bigger constant.

## Fleet rules given to every agent

- No modifying git commands, no worktrees, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, foreground builds only.
- All scratch/output under `🗑️generated/` of this ticket; never sweep it; never close/reopen the ticket.
- Serves: this session owns `:6013`; never kill by pattern, only by port/pid. `:6014` belongs to the cursor session.
- Temporary logs carry `[DEBUG] `; every wave writes its own `📓️2026-09-11-wave-<id>-….md` here.
- Verify by running (cargo test / vitest / probe), never "written, not run".

## Phase A — read-only audits (Sonnet), launched 14:20

| id | scope | report |
|---|---|---|
| A1 | which of the queued #44 guest/host hops already exist at HEAD (first-pick `selectedIds`, translate pre-admit, brush live-target encode, importFixture fold, lock chrome, `1:window` alias) — exact gaps with file:line | `📓️2026-09-11-audit-A1-pending-hops.md` |
| A2 | checklist sections never browser-proven (§2 camera, §3 projection, §4 window options, §10 volume brush, §11 relocate, §14 engagement bar, §18 catalogue, §19 settings, §22 delete/duplicate/focus, §23 add object, §24 export, §25 locale) — wiring host↔guest, ranked defects | `📓️2026-09-11-audit-A2-unproven-sections.md` |
| A3 | performance ceilings still in force + architecture designs (not constants) | `📓️2026-09-11-audit-A3-perf-ceilings.md` |
| A4 | battery coverage vs checklist; concrete new probe steps with selectors | `📓️2026-09-11-audit-A4-battery-coverage.md` |
| A5 | live peer edit map on shared host files (last 24 h, git log + mtimes) | `📓️2026-09-11-audit-A5-peer-map.md` |

## Phase B — implementation waves (Opus), launched after A1/A2 + baseline battery

(filled in as launched)

## Log

- 14:06 baseline battery started on :6013 (`🗑️generated/battery-2026-09-11-baseline-6013.txt`).
- 14:33 cold shared-cache `component-release` build started (pid 1962, `🗑️generated/build-2026-09-11-component-release-warm.txt`) — the shared build-dir had zero wasm-release units, so the first build is cold and warms the cache for every later rebuild.
- 14:40 baseline battery on :6013 (#43 wasm, vite-live host at HEAD) → `🗑️generated/probe-2026-09-11T12-10-48.md`:
  PASS boot / example-switch / undo-unwind / undo-redo / gumball-handle-enter / suggestions.
  FAIL inspection-object-fields (id=null), inspection-locked-flag-row, clipboard (delta=0, history rows present),
  locked-flag-row, locked-refusal-notice, gumball-scene-delta (poseLen=266), brush-preview-place (preview=null),
  import-distinct (1→1). faults=16, ALL one message: `fixed typed-operation and segmented-output authorities did
  not pre-admit the exact operation slot` on suggestionsTick ×10, engagementAbort ×3, setCamera ×2, transformBegin ×1.
  → the pre-admit fault is the convergent blocker (gumball/transform, suggestions ticks, camera). Wave B0 launched on it.
- 14:58 A1 landed (`📓️2026-09-11-audit-A1-pending-hops.md`): 7/8 queued #44 hops are PRESENT at HEAD (first-pick
  selected, brush live-target latch, importFixture fold + new `openImportFixture` host-arm, Inspection fall-through +
  lock flag_row, `1:window` alias, v102_1 fix, history paging). Only the typed-operation slot pre-admit (B0) is absent.
  ⇒ the next wasm build is a verification build for everything except B0. Gotcha for waves: `grep` returns zero on
  `🔌️PluginRuntime/🟦️.tsx` (very long lines) — use `rg` or python.
- 14:58 A5 landed (`📓️2026-09-11-audit-A5-peer-map.md`): renderer `🧱️elements/**` is hot under PROCEDURAL-3D-END-TO-END
  (ShellHost `setContributions` block live-uncommitted), `🧑‍💻dev` tooling is being rewritten by NX-COMPLETE-TASK-CACHING.

## Phase B — implementation waves (Opus)

| id | launched | scope | report |
|---|---|---|---|
| B0 | 14:45 | typed-operation / segmented-output slot pre-admit fault (16/16 baseline faults; blocks gumball, suggestions ticks, camera) — plugin host Rust `dispatch_typed_command_inner` | `📓️2026-09-11-wave-B0-pre-admit-slot.md` |
| B1 | 15:20 | battery extension per A4: steps for §1–§4, §10, §11, §12, §14, §15, §17–§19, §22–§25, `--only`, NDJSON verdicts, fault classes, blast-radius groups, `data-camera-json` on World3dHost | `📓️2026-09-11-wave-B1-battery-extension.md` |
| B2 | 15:22 | A3 ceiling #1: page-priced grants for the remaining retirement ladders (SurfaceReconcileTerminal, MountedTreeTerminal, transport/table-rows) + `settlePluginTurn` zero-progress fast-fail | `📓️2026-09-11-wave-B2-continuation-ceiling.md` |
| B3 | 15:24 | A3 ceiling #3: fixed owners backed by ≤64 KiB lazily allocated sub-pages (guest geometry containers) | `📓️2026-09-11-wave-B3-chunked-owner-pages.md` |
| B4 | 15:26 | A3 ceiling #2: attribute the ~5.4 s main-thread cost per window on Nakagin refresh, then paged first publication / lane memo / projection skip | `📓️2026-09-11-wave-B4-main-thread-refresh.md` |

A3 also re-verified `worldRelocate` on Nakagin as NOT a live defect (extent ≈1 617 items < 4 096), so checklist §26 item 1 is closed.
- 15:33 warm build finished: `wasm-release` 10m 32s, 101 crates into the shared cache. Materialize/activate chain for verification build **#44-pre** (HEAD guest, B0 not yet in) started (`🗑️generated/deploy-2026-09-11-44pre.txt`).
- 15:52 #44-pre materialized (transpile+descriptor, Activated (changed)); served core.wasm hash == disk (5e59b06fb69c, 14:32 build) WITHOUT a serve restart — vite serves the file fresh, so no port recycling is needed. Battery #44-pre launched on :6013 (`🗑️generated/battery-2026-09-11-44pre-6013.txt`).
- 15:50 A2 landed (`📓️2026-09-11-audit-A2-unproven-sections.md`): Relocate utility DEAD at the host (no gesture dispatches `worldRelocate`) → wave B5 launched; Import/Export IS wired (checklist + reverification were wrong); window DOM ids are camelCase (`puzzle3dMainTop`); fold toggles `#framework.window.<id>.measures.unfold` etc.; engagement bar spans two panes; camera pose has no DOM exposure (B1 adds `data-camera-json`).
- 16:05 battery #44-pre (`🗑️generated/probe-2026-09-11T12-33-00.md`, 393 s): locked-flag-row flipped to PASS; still FAIL
  inspection-object-fields on the FIRST pick (the same Inspection panel is fully populated at the locked step — second
  pick works), clipboard (probe finds no Copy control: `copyBtn=0 copyById=0`, census delta 0 although history shows
  Copy/Paste rows), gumball-scene-delta, brush-preview-place (after arming Brush the world lane still reports
  `utility=select`, hover=null → guest never republishes `activeUtility` on utility switch), import-distinct (3→3).
  faults=9: pre-admit ×6 (setCamera ×4, engagementAbort ×2 — B0), NEW `framework route 'interactionHover' has no exact
  pending spawn slot`, `1:framework.panel.history: AliasCapacity`, one interactionSelect framework-route fault.
  → wave B6 launched: hop-by-hop browser diagnosis + guest/host fixes for first-pick Inspection, utility republish,
  clipboard, import; B0 owns the pre-admit family.
- 16:20 B0 landed (`📓️2026-09-11-wave-B0-pre-admit-slot.md`): root cause = ingress minted a global operation id then asked whether its residue class (id % 64, direct-mapped, probe-free) was free → hash collision, not capacity; fix = reserve the first vacant slot then mint a congruent id (`allocate_operation_id_in_slot`), same for envelope ingress + media export; law green. Peer refactor leaves 6 unrelated plugin-crate test reds. Rebuild #44 started (pid 41729, `🗑️generated/deploy-2026-09-11-44.txt`).
- 16:45 B5 landed (`📓️2026-09-11-wave-B5-relocate-gesture.md`): Relocate utility wired end to end (grab → ground-plane ghost
  drag → ONE absolute `worldRelocate {objectId, position}`; Escape aborts); guest refuses locked/hidden with the
  `selection_locked` notice; Settings section titled `Settings — <window id>`; checklist §26 item 8 was stale (already
  `Mutation`). DOM id of the Relocate toggle is the raw `worldRelocate`. Vitest + cargo laws green.
  ⚠️ B5 found 4/5 guest undo laws failing (`dispatch("undo")` succeeds, document unchanged) — an unknown peer is
  mid-refactor of the typed publication path in plugin `🦀️.rs` (removed `_ if failed` retirement guards, deleted
  `async fn dispatch_typed_command`) → wave B7 launched to attribute and fix. Guest laws need `RUST_MIN_STACK=134217728`.
| B6 | 16:10 | hop-by-hop browser diagnosis: first-pick Inspection, Brush `activeUtility` republish, clipboard control + paste, import fold; new `pending spawn slot` / `AliasCapacity` faults | `📓️2026-09-11-wave-B6-lane-diagnosis.md` |
| B7 | 16:50 | guest undo-law regression (peer typed-publication refactor suspected) | `📓️2026-09-11-wave-B7-undo-regression.md` |
- 17:05 B2 landed (`📓️2026-09-11-wave-B2-continuation-ceiling.md`): the dominant unpriced ladder was the retained TABLE
  ladder (32×32 = 1 060 units, ONE per turn → 1 092 turns for one outliner-scale table); every ladder now takes the
  page grant; `settlePluginTurn` fails fast after 128 zero-progress continuations naming the pending surface (bound
  derived from the document contract). Mixed-surface law 1093 → 5 turns. Guest-side → rides the next wasm (#45).
  Note: root `bun ./📜️script.ts test long --run …` now errors (NX-COMPLETE-TASK-CACHING changed the router); vitest
  lanes run from the react target package (B2 report §6).
- 17:20 B4 landed (`📓️2026-09-11-wave-B4-main-thread-refresh.md`): attribution on a live Nakagin switch — projection
  132 ms ×51 (1 275 nodes), intake 124 ms, World3dHost 3 renders / 13 ms: the audit's three candidates were all small.
  Real defect: `buildUiRefreshRequest` sent cached hashes but no projector ever honoured them → every refresh rebuilt every
  tree and defeated `InterpretedUiNode`'s memo. Fix: `uiRefreshSectionUnchanged` in PluginRuntime (retained + owned
  responses answer `{key, hash}`) → projection 4 ms ×3, 95 % of bodies skipped. Two vitest laws. Host-only (live now).
  ⚠️ B4 saw the switch "DID NOT land" in its probes with a flood of `[DEBUG] settle … empty-required stop` warns
  (HEAD, cursor session) ×2 505–6 624 per switch (108–244 ms) — must be gated/removed in the close-out sweep; the #44
  battery decides whether example-switch still passes with B2's fast-fail live.
- 17:35 #44 deployed (wasm-release 12m 29s incremental; served hash 8bef0b857f6c == disk, B0 fault string present). Battery #44 queued behind a running agent probe (`🗑️generated/battery-2026-09-11-44-6013.txt`).
- 17:50 B7 landed (`📓️2026-09-11-wave-B7-undo-regression.md`): NOT a production regression — HEAD 46c3cb9 split every
  non-clipboard framework-reserved verb into admit (SpawnJob Isolated) + host-driven job, and the puzzle testkit's
  `dispatch` never routed the admission through `settle_reserved`, so 30 laws asserted on a document that never moved.
  8-line testkit fix: guest suite 638/35 → 668/5, all undo laws + framework `reserved_undo` 9/9 green. Residual reds:
  two `📐️geometry` sub-page laws + Nakagin fill law (B3 in flight), `two_instances_converge…` (known foreign),
  `open_vortex_suggestions_…_interactive_ceiling_for_nakagin` (needs an owner → next wave).
- 18:05 B3 landed (`📓️2026-09-11-wave-B3-chunked-owner-pages.md`): `FixedOwnerVec/Map` now claim lazily sized sub-pages
  (largest power-of-two slot count ≤ 64 KiB: objects 442 KB → 8 × 55 KB, cells 459 KB → 8 × 57 KB …); refusal seals the
  owner honestly; 4 unreachable!s in `CollisionSpatialIndex::step` → `Rejected(Capacity)`; 4 laws red→green (Nakagin fill
  under a fragmented-guest reservation policy). Still over the ceiling (listed, not fixed): brush-mesh Vecs 786 KB,
  import raw 256 KB, command output 256 KB, envelope decode 256 KB, wit-bindgen turn future 189 KB. Guest → next wasm.
- 18:20 battery #44 ran with B1's extended probe (70 steps, 1 455 s, `🗑️generated/probe-2026-09-11T13-15-23.md` + `.ndjson`):
  NEW PASS: window-both-present, one-canvas-each, same-document-both-views, distinct-camera, camera-json/orbit/pan/zoom/
  per-window, window-options-lane-responsive, settings-panel-opens, add-object-trigger, fill-history-entry, suggestions,
  volume-brush-target-volume-attribute, relocate-arm, context-menu-opens, outliner/catalogue open, catalogue-kind-rows,
  catalogue-add-selects-new-object, import-same-file-idempotent, undo-unwind.
  FAIL: every window-option toggle/slider (no value change), settings steppers absent, add-object dialog never opens,
  locale control absent, engagement input absent, context-menu object vocabulary missing (only shell rows), outliner hide
  control absent, catalogue add / drag-drop no delta, delete/duplicate/focus no delta, export no download, volume-brush
  & brush never arm in the lane (`activeUtility=select`), relocate no pose delta, plus the old inspection/clipboard/
  locked/gumball/import lanes; undo-redo fails late.
  faults=45 hard: B0's NEW message `every fixed typed-operation and segmented-output slot already owns a live operation`
  ×39 (engagementAbort ×20, setCamera ×19, openVortexSuggestions ×2) — i.e. typed-operation slots are never retired and
  the 64-slot table saturates ~half-way through the run, after which EVERY typed action fails → most late FAILs are
  collateral. Plus `1:framework.panel.history: AliasCapacity` ×1 and `framework route … no exact pending spawn slot` ×3.
  → wave B8 launched on the slot retirement leak (highest-impact defect in the whole run).
- 18:55 B6 landed (`📓️2026-09-11-wave-B6-lane-diagnosis.md`): (a) Brush/Volume-Brush disarm = HOST defect: every leftover
  overlay publication replaced the world overlay without `activeUtility` → fixed (`leftoverOverlayCarryingUtilityV1`),
  utility now survives pick/orbit/hover storm, brush-place hop 2; preview still null = GUEST id-scope miss
  (`render_body` keys the utility publication by window KIND, host map by window INSTANCE, `map_hit=false`).
  (b) First-pick Inspection: host hops all fire; the guest re-renders the panel with the pre-pick hash for 73 s → the
  persisted vortex selection is invisible to the guest's own render (silent `unwrap_or_default` reads now named).
  (c) Clipboard NOT broken: framework `mod+c/x/v`, paste creates `object-1`, but the world lane reflects it ~12 s late
  because nothing dirties the window surface after a mutation (same for gumball-scene-delta). (d) Import: real guest
  no-op (`effects:0 historyUpserts:0`). (e) `no exact pending spawn slot` fixed (pending_reserved direct-mapped on
  job % 64, now re-mints into a vacant class, law red→green). (f) `AliasCapacity` = alias credits exhausted by the
  unbounded history panel. B1's probe is mid-refactor (steps registered into a plan nothing iterates) — B6 used its
  own `🔍️b6-lane-probe.ts`. ⚠️ B6 restored plugin `🦀️.rs` from a 90 s old copy once — B8 must re-verify its hunks.
  → wave B9 launched on the guest half: utility publication instance scope, first-pick selection render, importFixture
  fold, window-surface dirty after mutations, history panel alias budget.
- 19:25 B1 landed (`📓️2026-09-11-wave-B1-battery-extension.md`): probe now 70 steps (§1–§25), `--only`, groups + `--reload-between-groups`,
  NDJSON, hard/collateral faults, history deltas by entry id; `data-camera-json` / `data-target-volumes-json` on World3dHost.
  Camera §2 browser-PROVEN (orbit = Alt+right-drag, middle = pan). Product defects with DOM evidence: `🌳️Tree` wraps every
  row checkbox in `<label onClick={preventDefault}>` → NO tree checkbox can ever toggle (grid/snap/lod-auto/select-*);
  projection measures + sun group render nowhere; vortex show/direction selects no-op; no `puzzle3d-play-settings` tab;
  Add Object dispatches `shell.openActionPane` and opens no dialog; outliner Hide no-op; catalogue rows `draggable:false`,
  add does nothing; engagement pane never renders; Alt+click adds no target volume; duplicate/focus nothing; German
  locale EMPTIES the outliner. Contract decision (coordinator): camera is per-window session-only view state
  (ticket 26/07/31) → `setCamera` must NOT add an artifact history entry; the World3dHost docstring is wrong.
  → waves B10 (framework chrome: Tree checkbox, engagement pane, settings tab, measures/sun, locale outliner, outliner
  Hide, camera history contract) and B11 (Add Object dialog, catalogue add/drag, volume-brush target volume, vortex
  selects, duplicate/focus/delete) launched. Some late-run FAILs may be slot-saturation collateral (B8) — both waves
  verify in a fresh session with `--only`.
| B8 | 18:25 | typed-operation slot retirement leak (39/45 #44 faults), pending-spawn / alias families | `📓️2026-09-11-wave-B8-slot-retirement-leak.md` |
| B9 | 19:00 | guest lanes: utility publication per window instance, first-pick selection render, importFixture fold, window dirty after mutations, history alias budget | `📓️2026-09-11-wave-B9-guest-lanes.md` |
| B10 | 19:35 | framework chrome: Tree checkbox, measures/sun rail, vortex selects, settings tab, engagement pane, outliner Hide, DE locale outliner, camera history contract | `📓️2026-09-11-wave-B10-framework-chrome.md` |
| B11 | 19:37 | editor actions: Add Object dialog, catalogue add/drag, volume-brush target volume, duplicate/delete/focus, context-menu vocabulary | `📓️2026-09-11-wave-B11-editor-actions.md` |
- 20:05 B8 landed (`📓️2026-09-11-wave-B8-slot-retirement-leak.md`): the `Retiring` stage of a typed operation had NO branch
  in the turn driver — only maintenance stage 0/24 (one stage per turn, ~240 turns per release) ever freed a slot, while
  admission was synchronous → every typed operation leaked from action 0. Fix: one release site
  `retire_typed_operation_unit`, driven by the turn driver, the close ladder and maintenance; diagnostics
  `live_typed_operation_slots()` on the runtime-diagnostics console channel. Laws red→green (200 actions → slots 0).
  Peer refactor leaves 146 plugin-crate reds (zero regressions vs baseline). `AliasCapacity` is a different table
  (cap 8, history panel) → B9 item 5; `no exact pending spawn slot` already fixed at HEAD by a peer.
  Build #45 started (pid 90195, B0+B2+B3+B8; B9/B11 guest halves ride #46).
- 20:35 #45 deployed (wasm-release 6m 38s; served a532fbe68602 == disk, B8 diagnostics string present). Battery #45 running with `--reload-between-groups` (`🗑️generated/battery-2026-09-11-45-6013.txt`).
- 20:50 B9 landed (`📓️2026-09-11-wave-B9-guest-lanes.md`): (1) utility publication resolved the window from
  `config.window_ids.first()` while every instance renders under the kind's body key → `puzzle3d_addressed_window_id`
  at the three sites, per-instance law; (2) first pick: `Puzzle3dPlaySnapshot::new` `unwrap_or_default()` substituted an
  EMPTY typed document when the typed decode failed (`kindCompatibility` projected as `null`), `interaction_topology`
  read that half and `validate_state` pruned every id → both halves fixed, 4 laws; (3) importFixture is an identity
  no-op by construction (effects:0 excludes every refusal branch) — probe's "distinct" file likely not distinct; taps
  kept; (4) HOST: `dispatchDirectBrowserActorCommand` applied no dirty scope → `browserActorDispatchUiScopeV1` +
  refresh on mutations (paste/undo/gumball now reflect in the same turn, vite-live); (5) `AliasCapacity`: history
  Commands section rows were paged but aliases were not (31 interactive rows per arena page) → per-page
  `PanelRowBudget`, 200 revertible entries law. Guest suite 687/2 (pre-existing). Guest halves ride #46.
- 21:10 B10 landed (`📓️2026-09-11-wave-B10-framework-chrome.md`): Tree `<label onClick={preventDefault}>` = HTML legacy-canceled
  activation (reverts checkedness, suppresses `change`) → now stopPropagation only, 5 laws; projection/sun measures:
  id family renamed to `<prefix>-measure-projection…` + both group roots were `default_open: false` (collapsed groups
  render NO children); `defaultDock` hardcoded the bottom corners so app-declared Settings/Display tabs were dropped;
  engagement `Pane` never spread its id + two independent folds → one fold + focus; camera `View` dispatch no longer
  logs a history row. Outliner Hide, vortex selects, DE locale: no source defect found (laws green) — B1's empty DE
  tree correlates with an `actor-activation.revoked` in the same step. B10 could not probe: :6013 unresponsive.
- 21:12 :6013 vite (pid 26789, 1 day old) was wedged at 97 % CPU, `curl` → 000 for 20 s, battery #45 stalled 549 s at a
  reload → killed by pid (own serve) and restarted (`🗑️generated/serve-6013-2026-09-11.txt`, ready in 382 ms).
  Partial #45 verdicts before the wedge: NEW PASS projection-measures-present, projection-control-flips,
  window-options-emit-no-history, camera-emits-no-artifact-history, grid-spacing, lod-value; camera-orbit/zoom and
  projection-repaints-camera flipped to FAIL (host mid-edit by B10 during the run). Clean re-run #45b started
  (`🗑️generated/battery-2026-09-11-45b-6013.txt`).
| B12 | 21:20 | WindowConfig lane round-trip: window-option toggles/selects/sun, settings steppers → window rail, camera lane reflected in `data-camera-json`, World3dHost consumers | `📓️2026-09-11-wave-B12-windowconfig-roundtrip.md` |
- 21:45 battery #45b clean run (`🗑️generated/probe-2026-09-11T14-48-04.md`, 1 089 s, 80 steps): **FAULTS=0** (45 → 0 — B8's
  retirement fix + B0's admission + B6's pending-spawn fix hold in the browser), PASS=40 FAIL=40. Remaining FAILs map to
  waves in flight: window options / settings / projection-repaints (B12), first-pick Inspection / brush preview /
  volume-brush arm / engagement-brush-verb (B9 guest → #46), add object / catalogue / duplicate / delete / focus /
  context menu (B11), plus four unowned: `fill-history-entry entries=0` (was PASS on #44 — suspect B10's "op-less View
  dispatch logs no history row" or the reload grouping), `export-only download=none`, `locale-control-present`
  (probe finds no locale control), `engagement-fill-verb #tool.fill aria-pressed=null`. → wave B13 on those four.
| B13 | 21:50 | fill history row, export download, locale control reachability, engagement fill verb | `📓️2026-09-11-wave-B13-export-history-locale.md` |
- 21:52 build #46 started (pid 13280, B9 guest lanes + whatever B11/B12 have landed; `🗑️generated/deploy-2026-09-11-46.txt`).
- 22:15 #46 deployed (wasm-release 4m 46s, Activated (changed)). Battery #46 queued behind agent probes (`🗑️generated/battery-2026-09-11-46-6013.txt`).
- 22:30 B11 landed (`📓️2026-09-11-wave-B11-editor-actions.md`): the plugin context menu NEVER worked in the React renderer —
  `requestContextMenu` was published via context but no `<InterpretedUiNode>` passed it as a prop (one `??` fix +
  `defaultPrevented` guard; shell fallback no longer opens on top); Add Object: manifest gave the arg-carrying
  `addObjectKind` the primary slot and folded the dialog into "More ›" (fixed); catalogue `draggable:false` is a
  deliberate per-driver contract (move grip) — added the missing `data-drag-payload` DOM mirror; volume-brush Alt+click
  lived on an occluded r3f plane → host `raycastGroundPoint` path, `WorldVoxelGroundPlane` deleted; `focusSelection`
  frames the whole document when nothing is selected; guest now honours `surface.hits`. 6 cargo + 3 vitest laws.
  ⚠️ B11: `addTargetVolume` / `addObjectKind` reach the guest and settle with NO Document frame (`historyUpserts:0`)
  amid `settle puzzle#1 empty-required stop … status=more-work` — the host's empty-required continuation stop (HEAD,
  cursor session) aborts the settle while the guest still reports more-work, so the mutation's publication is dropped.
  → wave B14 launched on settle semantics (also removes the `[DEBUG]` warn flood B4 measured at 108–244 ms/switch).
| B14 | 22:35 | settle semantics: empty-required stop drops in-flight mutations (no Document frame); warn flood | `📓️2026-09-11-wave-B14-settle-semantics.md` |
- 22:55 battery #46 (`🗑️generated/probe-2026-09-11T15-13-33.md`, 1 199 s): FAULTS=0, PASS=41 FAIL=39 (+locked-flag-row).
  B9's guest lanes did NOT flip in the browser (inspection first pick, brush/volume-brush arm `activeUtility=select`,
  `context-menu-selection-precondition selected=0`): selection/utility leftovers never land — consistent with B11's
  finding that the host's empty-required settle stop drops in-flight publications (B14). Waiting on B12/B13/B14.
- 23:00 verified: #46 served wasm DOES contain B9 (strings `import.ingress`, 6 × `b9 ` taps, B8 diagnostics) — the non-flip is a law-vs-browser gap on the publication side, B14's lane.
- 23:20 B12 landed (`📓️2026-09-11-wave-B12-windowconfig-roundtrip.md`): the WindowConfig lane was never broken (695 ms idle
  round trip); the rail controls just showed no optimistic draft while the dispatch was in flight → `🎚️measure-controls`
  draft for toggle/select (3 laws); guest `Puzzle3dScopeClass::WindowOption` (19 verbs no longer repaint the whole
  shell); `data-camera-json` = guest-published pose, `data-viewport-camera-json` = live rig; `ContainerView` dropped
  authored ids for section/group/field (fixed); `NumberStepperView` always bound `onDelta` to an unbound trigger → NO
  Settings stepper had ever dispatched (fixed); steppers `uniform: true`. All six `window-option-*` verdicts PASS live.
  Open: (a) per-window camera is `Puzzle3dCamera::default()` (zeros) until the first `setCamera` — the guest must
  publish a real initial pose per pane; (b) Settings edits land on the base kind `puzzle3d-main`, not the focused pane —
  DECISION: Settings targets the last-focused window instance (`activeWindowId`), the guest must not bake the roster's
  first id; (c) probe §19 selectors predate the surface-prefixed `uiNodeDomId` ids; (d) camera-orbit/zoom probe waits
  (1.2–1.6 s) are shorter than the debounced round trip. → wave B15 (camera initial pose, settings scope, probe ids/waits).
| B15 | 23:25 | initial per-window camera pose, Settings → last-focused pane, probe §19 ids + camera waits + selection precondition | `📓️2026-09-11-wave-B15-camera-settings-probe.md` |
- 23:45 B14 landed (`📓️2026-09-11-wave-B14-settle-semantics.md`): the empty-required stop sat BEFORE B2's zero-progress
  accounting and ended every mutation turn at continuation 1 (`required=[]` = satisfied by definition), so the
  Document frame never arrived and operations parked (slots 14→18 live). Fix: one publication-free streak with two
  outcomes — quiesced when nothing is outstanding, B2's stalled fault (naming the operation) when surfaces were
  required; both ungated warns removed. 3 laws red→green. Browser: catalogue-drag-drop PASS, document accumulates
  edits with real `create-object` history rows; example switch LANDS in 5.6 s (HEAD: 86 s, never landed),
  longtaskCpu 14.2 s → 0.2 s, warns 2 505 → 17. Residual: an edit lands ONE COMMAND LATE — the add's own turn ends
  `idle` with its operation parked and only the next command-ingress turn drives it (refresh/drain turns do not) →
  guest reactor scheduling → wave B16.
| B16 | 23:50 | guest reactor: freshly admitted typed operation parks until the NEXT command-ingress turn (edits land one command late) | `📓️2026-09-11-wave-B16-parked-operations.md` |
- 23:52 battery #46b (host-live B10/B11/B12/B14) started to re-baseline before #47 (`🗑️generated/battery-2026-09-11-46b-6013.txt`).
- 00:20 (09-12) battery #46b (host-live) crashed at the locale step (`Execution context was destroyed … navigation` inside
  `snapshot()` — a page navigation during the locale switch; probe hardening + root cause → B17). Before the crash:
  all six `window-option-*` verdicts flipped to PASS (B10 Tree fix + B12 drafts, host-live); `camera-orbit/pan/zoom`
  and `camera-emits-no-artifact-history` now FAIL honestly because `data-camera-json` mirrors the guest pose (zeros
  until B15's initial pose + the `setCamera` history guard ride #47).
- 00:25 correction: the #46b crash is the probe's own `--reload-between-groups` path (`snapshot()` raced `page.reload()` at 'reloading page before group mutate'), not a locale navigation — probe hardening goes to B17 after B15 releases the probe file.
- 00:40 B13 landed (`📓️2026-09-11-wave-B13-export-history-locale.md`): fill-history-entry PASS (B10's bottom-anchor mount
  duplicated the framework History tab → `shellRendersPanelTabItself` filter); Export WORKS (7 542 B `puzzle-3d.json`;
  probe route is the context menu on a dead actor); locale control is on `framework.settings` + real no-default-language
  leak fixed (`syncShellLabelLocale`, chrome flips DE⇄EN live); engagement `fill <n>` chain law green + `aria-pressed`
  added to `PanelTabBar` (tabs had no ARIA state). All host-live.
  ⚠️ SYSTEMIC: the guest actor traps ~30 s after boot with `plugin.reactor-close-authority: "native close terminal
  unavailable"` (present in #45b and #46 batteries; not matched by the probe's FAULT_RE, so FAULTS=0 hid it) — every
  battery step after ≈t+40 s measures a DEAD guest. → wave B17 launched (highest priority).
| B17 | 00:45 (09-12) | guest actor traps ~30 s after boot: `plugin.reactor-close-authority: native close terminal unavailable` (`⚛️reactor/🚪️lifetime/🦀️.rs:346`) | `📓️2026-09-11-wave-B17-reactor-close-trap.md` |
- 01:15 B15 landed (`📓️2026-09-11-wave-B15-camera-settings-probe.md`): opening per-pane camera is now DERIVED in the guest
  (`framed_camera` twin of the host autofit; publishing it would violate the lane's single-writer contract — 52 laws
  red — so `setCamera` stays the only writer); new schema-first `focusedWindowId` on the view context (JSON schema, TS,
  Rust ViewModel, wgpu shell) so Settings addresses the pane the user looks at; probe: camera-orbit/zoom PASS
  (poll up to 5 s), settings-steppers-present + grid-spacing-bumps PASS (surface-prefixed ids). Honest reds for #47:
  window-distinct-camera, settings-value-reaches-window-rail (B12 `uniform: true`), camera-emits-no-artifact-history
  (B10 guard). NEW product defect: clicking the outliner entity row `panel:puzzle3d-play-document/seed-left-001`
  selects nothing (`data-status-json` empty on both surfaces) — may be the dead actor (B17) or the parked operation
  (B16); re-check on #47 before assigning.
- 01:40 B16 landed (`📓️2026-09-11-wave-B16-parked-operations.md`): `has_runnable_work` read `AwaitingAck => !result_page_presented`,
  false the instant the result page was handed out, so the guest answered `Idle` on every turn that published
  anything and the un-parking ACK (an Event::Message on the host's NEXT continuation) never came until a new command;
  fix: a mounted operation is runnable until removed, `next_advanceable_typed_operation` skips AwaitingAck, result
  page scan finds the owning operation. 2 laws red→green; plugin crate zero regressions vs baseline.
  Build #47 started (pid 56339: B10 guard + B12 + B15 + B16 guest; `🗑️generated/deploy-2026-09-12-47.txt`); B17 rides #48.
| B18 | 01:50 (09-12) | probe hardening (reload race, guest-death FAULT_RE, guest-alive verdict), outliner row selection, context-menu precondition path | `📓️2026-09-12-wave-B18-probe-hardening.md` |
- 02:05 #47 deployed (5m 20s, Activated (changed)); battery #47 queued (`🗑️generated/battery-2026-09-12-47-6013.txt`, no reload grouping until B18 hardens the probe).
- 02:20 B17 landed (`📓️2026-09-11-wave-B17-reactor-close-trap.md`): B13's "guest dies after 30 s" was wrong — the trap
  appears in ZERO batteries and only in three `--only` probes, always right after `[DEBUG] hot-swap puzzle …`: the
  trigger is a plugin HOT-SWAP fired by a peer deploy into a live page; "Agent disconnected" is the presence chip.
  Three stacked causes: `is_retired` answered `Ok(false)` on lock contention (un-minting a receipt),
  `terminal_is_empty` erased the Fault into one opaque sentence, and `reloadPlugin`'s catch disposed the handle the
  session had just adopted (the actual killer). Fixes: Fault provenance, latched idempotent terminal witness,
  `committed` flag. 2 laws red→green; hot-swap live on :6013 now keeps the committed handle. Guest halves ride #48.
  Probe regex fragments: `actor-activation\.revoked|plugin-handle\.(closed|retirement-failed)|hot-swap rolled back`
  (the fault code crosses the console as a byte map, never as text) → B18.
- 02:35 battery #47 (`🗑️generated/probe-2026-09-11T16-46-55.md`, 1 205 s): FAULTS=0, PASS=46 FAIL=34. NEW PASS: fill-history-entry,
  outliner-hide-control-present, settings-steppers-present, settings-grid-spacing-bumps, all six window-option-*.
  BUT the document-mutation lanes B14 had browser-PROVEN on #46+host-live (catalogue-drag-drop PASS, duplicate/delete
  census moving) are red again on #47, selection still never lands (`surfaceStatus=["1=","1="]`), and undo-unwind
  regressed (example stays Nakagin). Something between B14's proof (23:30) and #47 (B15 `focusedWindowId` host+guest,
  B16 guest scheduling, B17 F3 host hot-swap commit) re-broke the publication path → wave B19 (bisect + fix).
| B19 | 02:40 (09-12) | bisect the mutation-lane regression between B14's #46 proof and #47 (suspects B15 focusedWindowId, B16 scheduling, B17 F3, B10 guard); may run one or two deploy chains itself | `📓️2026-09-12-wave-B19-mutation-lane-regression.md` |
- 03:15 B18 landed (`📓️2026-09-12-wave-B18-probe-hardening.md`): probe navigation-safe (`evalSafe`, reload re-runs boot),
  guest-death regex + `guest-alive-<group>` verdicts + `first-hard-fault-at`; outliner rows: `TreeItem`'s expandable
  layout wired `onClick` to the 12 px label only and no row published `aria-selected` → fixed in `🌳️Tree` (4 laws);
  context-menu path fixed (no clearing pick, projected instance position). On #47: context-menu-selection-precondition,
  context-menu-opens (5 rows), context-menu-object-vocabulary (all six), zoom→focusSelection all PASS.
  Handed over: `focusSelection` accepted but camera bit-identical; outliner Hide leaves the row unchanged; BOTH world
  surfaces report `data-surface-id="1"` and publish empty `selectedIds`/`data-status-json` although the leftover
  overlay carries `["seed-left-001"]` → wave B20 (world surface identity + focusSelection + Hide), parallel to B19.
| B20 | 03:20 (09-12) | world surfaces both `data-surface-id=1` / empty selection status, focusSelection camera, outliner Hide | `📓️2026-09-12-wave-B20-world-surface-identity.md` |
- 03:50 B19 landed (`📓️2026-09-12-wave-B19-mutation-lane-regression.md`): NO regression — the mutation lanes were red on
  #45b/#46/#47 alike; B14's PASS was an `--only` reading, and the same replay on #47 is strictly better (duplicate,
  duplicate-reselects-clone, volume-brush add/voxel-dims flipped to PASS); undo-unwind passes in isolation. All four
  suspects cleared with file:line. What is real: B14's one-command lag (the `addObjectKind` settle runs 23
  continuations to `idle` with no Document frame; the row arrives in the NEXT command's settle) and
  `registerBrushMesh` dispatches taking 5.6–14.7 s EACH, serialised per actor, so late-battery verdicts (3.5 s budget)
  measure latency (74 started / 59 settled on #47). B16's laws are VACUOUS: the `KeyedTestApp` `compositeEdit` ladder
  is refused at `TestCountOneItemPreparationFactory::preflight` (Fault lane) — never proven against a landing
  mutation. New equivalence law `an_admitting_host_call_hands_back_the_terminal_lane_it_earned` becomes the pin once
  the fixture reaches Terminal. → B21 (fixture → Terminal, then fix the lag natively) and B22 (registerBrushMesh cost).
| B21 | 03:55 (09-12) | repair the KeyedTestApp compositeEdit fixture so B19's law pins the one-command lag, then fix the lag natively | `📓️2026-09-12-wave-B21-one-command-lag.md` |
| B22 | 03:57 (09-12) | `registerBrushMesh` 5.6–14.7 s per dispatch serialised per actor → mesh by URL / bulk lane / non-blocking upload | `📓️2026-09-12-wave-B22-brush-mesh-upload.md` |
- 04:35 B21 landed (`📓️2026-09-12-wave-B21-one-command-lag.md`): the lag was NOT scheduling — a typed operation's edit reaches
  the command log only via `backfill_command_log` inside `refresh_cache`, and `take_typed_operation_completion` gated on
  `history_dirty_sequences.is_empty()` BEFORE that refresh, so the completion carried `history_patch: None` and the row
  surfaced on the next command (one added `refresh_cache().await?`). Fixture preflight refused any mutation with a
  `description` (dropped); B19's law extended with history patches → red→green; latest-wins rebase discarded its key
  and could wait forever on the shared key registry (fixed, 3 of B16's recorded reds now pass); six puzzle3d import
  laws were green ON the lag (moved to the completion lane); puzzle3d pin
  `a_mutating_verb_lands_its_object_and_its_history_row_inside_its_own_settle` red→green. Build #48 started (pid 96722:
  B17 + B21 (+ whatever of B20/B22 compiles); `🗑️generated/deploy-2026-09-12-48.txt`).
- 04:55 #48 deployed (6m 15s, Activated (changed)); battery #48 queued with `--reload-between-groups` (B18-hardened probe; `🗑️generated/battery-2026-09-12-48-6013.txt`).
- 05:20 B20 landed (`📓️2026-09-12-wave-B20-world-surface-identity.md`): `Interpreter` set `surfaceId: String(record.id)` (a
  per-document DFS integer → node `1` in every window) → `surfaceHostIdentityV1` (surface = owning window, controller =
  record id, pane = authored key) + `data-selection-json`/`data-window-instance-id` on World3dHost; both panes now
  publish `selectedIds:["seed-left-001"]` with distinct identities; volume-brush-arm PASS; context-menu-zoom moves the
  camera of the pane the menu opened over (focus-selection verdict needs a selection precondition — probe). Outliner
  Hide: guest law green (`[1,1,1]`→`[0,0,0]`, eye→eye-off) and the browser logs `change-object-hidden … new-hidden=true`
  applied, yet the guest re-renders 8× at scale `[1,1,1]` — the mutation never reaches the document it renders from.
  KEY DATUM: the WHOLE selection-scoped mutating family (duplicate, delete, setSelectionFlag, addTargetVolume,
  translateSelection, worldRelocate) is red together while catalogue-drag-drop commits → B21's `refresh_cache`
  completion fix is the first candidate (in #48); if #48 still shows it, next wave: rendered-document vs mutated-document.
- 05:45 B22 landed (`📓️2026-09-12-wave-B22-brush-mesh-upload.md`): measured — default example 18 pages @ p50 0.64 s;
  document scale 202 pages enqueued at once, 40 settled in 420 s, p90 36 s, max 88 s, a click waited 44 s behind 15
  pages; the cost is 84 worker round trips per page inside the serialised command-ingress transaction. No host→guest
  lane a command arm can consume exceeds 8 KiB (`http-request` unmapped, `InvokeExtension` lands on the same bus), so
  the wave made bytes cross ONCE per geometry (digest index; every `dist/mesh/*.glb` is the same capsule) and
  back-pressured the run to one outstanding page; retransmit no longer kills the run; refused alias falls back.
  Host-live: queue depth 162 → ≈2, click wait 44 s → 10.7 s (0.17 s on the default example); guest digest index rides
  #49. 3 Rust + 5 vitest laws. Residual: ~84 round trips / 0.64 s per page (next ceiling), page scan cursorised into
  23 work items, a real pull lane needs `http-request` mapping + guest AsyncTask. ⚠️ a live peer rewrote
  `✏️editor/🦀️.rs` at 19:36 — coordinate before editing it.
| B24 | 05:50 (09-12) | per-command cost: ~84 host↔worker round trips / 0.64 s per 8 KiB command in the ingress transaction | `📓️2026-09-12-wave-B24-command-ingress-round-trips.md` |
- 06:20 battery #48 (`🗑️generated/probe-2026-09-11T18-51-12.md`, 1 154 s): FAULTS=0, guest alive in every group,
  PASS=51 FAIL=32 (+context-menu-selection-precondition, guest-alive ×3, reboot ×2). The mutation family is still red
  in the FULL battery although B18/B19/B20/B22 proved the same lanes PASS in fresh-page `--only` runs → the full run
  measures latency behind the serialised brush-mesh pages (B22: 0.64 s/page, back-pressure host-live, digest index
  rides #49) with 3.5 s verdict budgets. Started a per-lane fresh-page pass (`🗑️generated/lanes-2026-09-12-48.txt`)
  to split functional reds from latency reds before assigning the next wave.
- 06:50 B24 landed (`📓️2026-09-12-wave-B24-command-ingress-round-trips.md`): the 84 round trips were the GUEST's pacing —
  `plugin_exchange` ran `PluginCommandIngress::step()` once per turn (4 turns to decode one page) and
  `plugin_continue_typed_operations` advanced exactly ONE unit per turn (≈78 turns); the host adds nothing. Fix:
  `PluginCommandIngress::advance(72)` per turn + `TypedOperationGrant` (many units per turn bounded by an unacked
  result page, the turn's frame capacity, 8 ms, 256 units). 52 → 4 turns per command (13×); laws
  `a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page` (1/2/4/8 pages → ≤ pages+1 turns) + vitest
  census. Guest-only → rides #49 (after the lane pass finishes on #48).
- 07:05 lane pass interim (fresh page per lane, #48): FUNCTIONAL reds even in isolation — first-pick Inspection (empty),
  locked flag row + refusal notice, gumball scene delta, brush preview, volume-brush add target volume, relocate pose
  delta; PASS in isolation — clipboard, volume-brush arm + voxel dims, relocate arm, gumball handle enter. The whole
  SELECTION-SCOPED mutation family fails while create paths commit (B20's datum) → wave B23 launched with four
  hypotheses (B4 refresh-skip hash source, typed-vs-projection document, leftover selection identity, kind-vs-instance
  addressing).
| B23 | 07:05 (09-12) | rendered document ≠ mutated document for the selection-scoped mutation family | `📓️2026-09-12-wave-B23-rendered-vs-mutated-document.md` |
- 08:25 B23 landed (`📓️2026-09-12-wave-B23-rendered-vs-mutated-document.md`): H1/H2/H4 killed with evidence; every guest half
  green natively; the pick genuinely disappears between dispatch and render but the two witnesses were gated on
  `runtime_diagnostics_enabled()` (env read, never true in a jco component) → now `interaction_selection_loss_v1`
  verdicts on a visible channel + `data-guest-selection-json` (unmerged guest lane) on the pane; new law names the
  short-circuit no testkit exercises: `plugin_mount_surface → SurfaceContexts → plugin_render_surface`, where
  `SurfaceContexts::get` rebuilds a body's ViewModel from ONE shared `view_state` that every mount overwrites.
  → B25: build that testkit route in semio-framework-plugin, reproduce the pick loss, fix the shared view_state.
| B25 | 08:30 (09-12) | build the `plugin_mount_surface → SurfaceContexts → plugin_render_surface` testkit route; fix the shared `view_state` every mount overwrites | `📓️2026-09-12-wave-B25-surface-contexts-view-state.md` |
- 08:32 lane pass on #48 (fresh page per lane, `🗑️generated/lanes-2026-09-12-48.txt`): PASS in isolation — clipboard,
  volume-brush arm/voxel dims, relocate arm, gumball handle, context-menu precondition/opens/vocabulary/zoom-registered,
  outliner opens + hide control, catalogue opens/rows/drag-drop, duplicate + reselects clone, example-switch-instances,
  undo-unwind, import-same-file. FUNCTIONAL reds — the selection-scoped family (first-pick Inspection, lock row/notice,
  gumball delta, brush preview, add target volume, relocate delta; B25), catalogue-add-object-kind, delete-selection,
  outliner-hide-applies, focus/zoom camera, undo-redo, export download, import-distinct (B9 says identity; probe file).
  Chained: deploy #49 (B20/B22/B23/B24 guest) after the lane pass → full battery (`🗑️generated/deploy-2026-09-12-49.txt`,
  `battery-2026-09-12-49-6013.txt`).
| B26 | 08:40 (09-12) | residual lanes: export route + per-example filename, genuinely distinct import, redo, focus/zoom preconditions, catalogue row add, outliner hide row | `📓️2026-09-12-wave-B26-residual-lanes.md` |
- 09:15 B25 landed (`📓️2026-09-12-wave-B25-surface-contexts-view-state.md`): `SurfaceContexts` (`⚛️reactor/🪟️surfaces`) held
  ONE `view_state` that every `insert` overwrote, so every body rendered against whichever sibling mounted LAST (the
  shell always sent distinct contexts per surface); also the old `get` returned `None` → the `surface X has no host
  context` fault class. Fix: `SurfaceRole { Window(id) | Panel | Section }` fixed at mount, each binding keeps its own
  projected ViewModel, `update_view` replays per role, alias = another Window binding. New law
  `🔬️surface-view-state-routing` red→green (quoted clobber: Inspection rendered with `focusedWindowId: -top, locale:
  de`). 28/28 related laws; plugin crate zero new reds; puzzle selection family 64/1 (peer's). Rides #50 with B23.
- 23:45 CEST (2026-09-11) — TIMESTAMP CORRECTION: every log line above marked "(09-12) 0x:xx" was written with a wrong
  clock; real wall-clock is 2026-09-11 evening CEST (B17 ≈ 20:20, B19 ≈ 21:50, B20 ≈ 22:20, B21 ≈ 21:35, B22 ≈ 22:45,
  B23 ≈ 23:25, B24 ≈ 22:50, B25 ≈ 23:40). Filenames dated 2026-09-12 stay as they are.
- 23:43 second vite wedge on :6013 (pid 6648 at 100 % CPU, http 000, lane `add-object-dialog` hung 48 min) after the
  B23/B25/B26 host edit burst — same signature as 21:12. Recycled by pid, back in 30 s (`🗑️generated/serve-6013-2026-09-11-b.txt`).
  The lane loop resumes (locale-switch, settings-panel, projection-options) and the chained #49 deploy + battery follows.
- 00:30 (2026-09-12) B26 landed (`📓️2026-09-12-wave-B26-residual-lanes.md`): export/import probe routes drove the shell's
  FALLBACK context menu (suppressed over a viewport) → now via the Actions pane `#action.exportFixture` /
  `#action.openImportFixture`; per-example download name (`active_example_id` on `Puzzle3dConfig`, `Config` lane added
  to `setActiveExample`'s contract; rides #50); import-same-file was passing vacuously (import never ran); undo-redo:
  two probe defects + a real host defect (`navbarExampleIdFromHistoryUpserts` restored a `remembered` id only the
  select's own onValueChange wrote → `rememberedExampleIdFromDispatchV1`, host-live); focus/zoom get an orbit-away
  precondition. Two PRODUCT defects located, not fixed: (a) catalogue rows sit UNDER `nav#ui.navbar`
  (`elementFromPoint` → navbar child), so no real press reaches them (drag-drop bypasses hit-testing);
  (b) outliner Hide: `setSelectionFlag` applies (both surfaces publish zero scale) but the panel body is never
  re-taken because `browserActorDispatchUiScopeV1` yields `none` when the ADMISSION reply reports
  `mutationCount: 0` (a typed operation completes later). → wave B27.
- 00:40 the lane loop and the chained #49 command deadlocked on each other's zsh wrapper argv (both matched `pgrep -f browser-probe`); killed both, relaunched #49 deploy + battery with a self-excluding pattern (`browser-pro[b]e`).
| B27 | 00:45 (09-12) | catalogue rows under the navbar (layout); admitted typed operations never refresh panels (scope must follow the completion) | `📓️2026-09-12-wave-B27-navbar-overlap-admitted-refresh.md` |
- 00:45 #49 relaunched: deploy (B20/B22/B23/B24/B25/B26 guest) → full battery (`🗑️generated/deploy-2026-09-12-49.txt`, `battery-2026-09-12-49-6013.txt`).
- 01:25 (09-12) #49 deployed (B20/B22/B23/B24/B25/B26 guest) and full battery (`🗑️generated/probe-2026-09-11T22-07-1*.md`,
  1 120 s, 88 steps): FAULTS=0, PASS=53 FAIL=35 (+undo-unwind, +add-object-trigger; B26's new steps add reds:
  add-object dialog/kind/count, export-names-the-example, import-distinct-records-history). The selection-scoped
  family is still red in the FULL run → fresh-page lane pass on #49 started (`🗑️generated/lanes-2026-09-12-49.txt`)
  to measure B25's effect in isolation; B27 (host-live) in flight.
- 01:40 (09-12) lane pass on #49 (`🗑️generated/lanes-2026-09-12-49.txt`): B25 flips the READ side — inspection-object-fields,
  inspection-locked-flag-row, locked-flag-row now PASS in isolation; add-object dialog/kind/count PASS (B11/B26);
  catalogue-drag-drop, volume-brush arm/dims, relocate arm PASS. Still red in isolation: the selection-scoped
  MUTATIONS (gumball delta, relocate delta, add target volume, duplicate, delete, outliner hide → B27's
  completion-scope refresh is the candidate), locked-refusal-notice, brush preview, catalogue row add (B27 navbar),
  export download none + import never reaches the guest (B26's new Actions-pane route unverified), and three
  REGRESSIONS vs #48 lanes: context-menu-opens rows=0, engagement-input-present input=0, gumball-handle-enter
  entered=false (host mid-edit by B26/B27 or probe change) → wave B28 on the host-live routes + regressions.
| B28 | 01:45 (09-12) | host-live routes + regressions: export/import via the Actions pane, context-menu rows, engagement input, gumball handle enter, locked refusal notice | `📓️2026-09-12-wave-B28-host-routes-regressions.md` |
- 02:25 (09-12) B27 landed (`📓️2026-09-12-wave-B27-navbar-overlap-admitted-refresh.md`): the catalogue rows were FOLDED, not
  covered — all four sections `default_open: false` so every row had a 0×0 rect and `elementFromPoint(0,0)` hit the
  navbar (navbar z-index is 0 by CSS; the `z-navbar` class was dead source, now `z-base`); guest `objects` section
  opens by default (rides #50), layout law from `STYLING_METRICS.chrome`. The completion lane already refreshes
  (Hide flips in ~1.9–9.2 s; `outliner-hide-applies` PASS live); real residual was 18 completions per idle 10 s each
  paying `applyHostEffects` → `typedOperationCompletionRefreshV1` (no pass for completions that dirtied nothing).
  `outliner-show-restores` is a probe timing artifact (fixed 3 s wait vs 9 s Hide). Build #50 started (pid 3280).
- 02:50 (09-12) #50 deployed (4m 02s; B26 export name + B27 catalogue fold). Full battery waits for B28 (owns the probe file and the port).
