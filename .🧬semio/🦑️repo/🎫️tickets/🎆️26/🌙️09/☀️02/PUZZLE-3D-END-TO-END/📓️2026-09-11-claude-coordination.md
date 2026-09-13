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
- 04:45 (09-12) B28 landed (`📓️2026-09-12-wave-B28-host-routes-regressions.md`): REAL layout defect — the perspective
  window's pane-chrome toggle (`…engagement.toggle` at 490,35) is covered by the mode-dock tab button
  (`mode-dock-tab-1-puzzle3d-main-perspective`), six pointer clicks never unfold it, and `Pane` unmounts folded
  children, so a real user can never reach Export / Import / the typed engagement field; context-menu rows were never
  broken (the guest answers 6–7 rows in ~18 s behind serialised ingresses; 30-poll budget → all three PASS);
  `World3dHost` dispatched an undeclared `contextMenuAt` on every right-click (deleted, law); the typed field lives
  in the SEARCH pane (`engagement-input-present` PASS) but its controlled `input.value` stays `""` so every submit is
  empty; gumball hit stamp now includes the axis origin. A probe with `page.evaluate` between mouse down/up
  deadlocked 44 min and :6013 wedged a third time (pid 81904) → recycled by pid at 02:44, B28's two orphan probes
  killed, full battery #50 started (`🗑️generated/battery-2026-09-12-50-6013.txt`).
| B29 | 04:50 (09-12) | dock tabs cover window pane-chrome toggles; engagement Search controlled value never updates; `#tool.fill` needs the footer Tool category; probe mutation polling budgets | `📓️2026-09-12-wave-B29-dock-chrome-engagement.md` |
- 05:20 (09-12) battery #50 (`🗑️generated/probe-2026-09-12T00-44-50.md`, 1 247 s): FAULTS=0, PASS=51 FAIL=37. NEW PASS:
  export-only (B28's pane-toggle fallback), add-object dialog + dynamic kinds. NEW FAIL: example-switch / undo-unwind /
  undo-redo all read `example=` (EMPTY) and `export-names-the-example expected=.json` — the active example id is
  empty at boot and after a switch (B26's `active_example_id` is stamped only by the set_active_example arms; B26's
  host `rememberedExampleIdFromDispatchV1` reads it) → REGRESSION, wave B30. import-distinct now reaches the guest
  (taps present) but the census stays 1 → the fold, not the route. context-menu rows answer in 15–20 s (30 polls
  insufficient in the full run) — B29's 30 s budgets.
| B30 | 05:25 (09-12) | example id at boot / navbar label regression / export name; importFixture completion must republish the world lane | `📓️2026-09-12-wave-B30-example-id-import-lane.md` |
- 06:35 (09-12) B29 landed (`📓️2026-09-12-wave-B29-dock-chrome-engagement.md`): the window sat in the dock tab bar's band
  because `ui.css` `.window-silhouette-content-plane` zeroed only the padding of its clearance for any descendant
  `edgeless`/`dead-line-scroll` (matched through the mode-dock body) → guard scoped with `:not(:has([data-slot=
  "window"]))`; a second obstruction was World3dHost's hardcoded-English `Frame` overlay in the chrome row → moved to
  a localized top-right rail below the clearance (EN/DE); B28's synthetic click fallback deleted. Live: toggle
  uncovered, `export-only` PASS by a real click, `engagement-input-present` PASS. Engagement: Search was purely
  controlled by a value the guest never republished → `searchControlledLineV1` draft; ShellHost skipped tool
  reconciliation unless the Tool category was the root → `programArmedToolRevealV1`; `engagement-brush-verb`,
  `engagement-fill-verb`, `engagement-placeholder-has-no-dead-verbs` PASS. Probe: `settleFor(…, 30 s)` + `waitedMs`
  on every mutation verdict (delete-selection is a proven 30 s red). :6013 wedged a FOURTH time (bun 91282, 96 %) —
  sampling it before the recycle (`🗑️generated/vite-wedge-sample-2026-09-12.txt`). Newly visible reds: `engagement-abort`
  cannot leave the fill tool; probe `selectionState()` counts dock tabs; `every_advertised_engagement_verb_is_implemented`
  red at HEAD (peer).
- 06:40 (09-12) B30 landed (`📓️2026-09-12-wave-B30-example-id-import-lane.md`): NO element carried
  `playground.navbar.fixture` (the select spent its id on `.label/.select/.trigger`), so every reader fell through to
  the first combobox (empty) — the four #50 reds measured nothing; id now names the trigger (law); guest default
  `active_example_id = concrete-forest` at seeding (rides #51). Import lane: contract correct, completion carries the
  composite + document bodies, `import-distinct` PASS at ~17 s on a free serve; `openHistory()` toggles the panel
  SHUT on a second click (probe). Vite wedge ROOT CAUSE found by PROCEDURAL-3D-END-TO-END
  (`📓️vite-serve-wedge-2026-09-12.md`): chokidar consolidates one FSEvents stream over the whole repo and runs ~1 316
  prefix filters per write anywhere (probe outputs, cargo cache…) — fixed in `⚙️vite.config.ts` (`server.watch: null`
  + `semioSourceWatchVitePlugin`, unwatched `🗑️generated`/`.🧬semio`/`dist`…). :6013 recycled onto the fixed config
  at 04:37 CEST; #51 (B29 host + B30 guest) deploy + battery chained (`🗑️generated/deploy-2026-09-12-51.txt`).
| B31 | 06:45 (09-12) | `engagement-abort` cannot leave Fill; `delete-selection` 30 s red; gumball zero pose delta; probe `selectionState`/`openHistory` repairs | `📓️2026-09-12-wave-B31-delete-abort-gumball.md` |
- 07:20 (09-12) B29 follow-up: gumball-handle-enter PASS (grab at 0.6 of origin→tip), catalogue-add-object-kind PASS by a
  real press (1→2), `selectionState()` no longer counts dock tabs (`addObjectKind` does not select what it adds —
  honest red). The mutation commits LAND (history `move-object … new-origin=13.0994`, hide reaches both surfaces)
  but `data-instances-json` never changes and the panel body is never re-taken → the last convergent defect is the
  world-lane republication after a typed-operation completion (B27's `typedOperationCompletionRefreshV1` returning
  null for `{kind:"none"}` completions is the first hypothesis) → wave B32.
| B32 | 07:25 (09-12) | world lane / panel bodies not republished after typed-operation completions (gumball, relocate, hide, delete); `Agent disconnected` during mutations | `📓️2026-09-12-wave-B32-world-lane-after-completion.md` |
- 07:50 (09-12) battery #51 (`🗑️generated/probe-2026-09-12T02-44-30.md`, 1 265 s): FAULTS=0, PASS=50 FAIL=33. NEW PASS:
  camera-orbit, example-switch, undo-unwind, undo-redo, import-distinct-records-history (B30 navbar id). NEW FAIL vs
  #50: add-object dialog/kinds, engagement-input-present (unfold), export-only — all pass in fresh lanes → the FULL
  run's accumulated state (Nakagin 180 instances, armed Fill, presence banner, dock state) is what breaks them; the
  mutation verdicts burn the full 30 s in the full run while B29's fresh lanes land them in 6–21 s → wave B33 on the
  full-run vs fresh-lane gap (state pollution / tool arming / dock unfold under load).
| B33 | 07:55 (09-12) | why mutations and pane unfolds that pass in fresh lanes fail inside the full battery (state pollution, armed Fill, Nakagin load, presence) | `📓️2026-09-12-wave-B33-full-run-vs-fresh-lane.md` |
- 08:20 (09-12) B32 stalled (agent watchdog, no progress 600 s) while reading the publication contract table — relaunched as B32b with the same brief.
- 09:25 (09-12) B31 landed (`📓️2026-09-12-wave-B31-delete-abort-gumball.md`): delete's real defect was in the plugin host —
  `interaction_selection_snapshot`'s leftover overlay refilled an EMPTY store selection (only the reserved pick route
  ever wrote it), so every app-authored subtractive clear was undone on the next read → `leftover_after_app_selection
  _write_v1` (also fixes HEAD's `every_advertised_engagement_verb_is_implemented` red); `deleteSelection` gains the
  `Interaction` lane + `clear_selection`; the contract fixture was stale in five further ways and the publication audit
  script lacked two keys (now green for all three owners). Escape now disarms Fill (cancels the plan, `SetActiveTool ""`).
  Gumball: the probe dragged perpendicular to the axis, and World3dHost FABRICATED a 0.5 `translateSelection` whenever
  the pose didn't move (deleted, law). Lanes on #51: delete PASS (19.9 s / 10.8 s), duplicate PASS on a verified
  selection, import-distinct-records-history PASS (`openHistory()` was pressing Undo); `gumball-scene-delta` is one
  clean hop (a real delta does not move the census → B32b). A peer's `🔀️surface-switch` work briefly broke the boot
  (`createSessionWorkLedgerV1`).
- 09:30 (09-12) build #52 (compile only, pid 87736; B31 guest + plugin-host leftover fix) started; materialize/activate deferred until B32b/B33 finish their lanes (`🗑️generated/build-2026-09-12-52.txt`).
- 10:05 (09-12) B32b landed (`📓️2026-09-12-wave-B32-world-lane-after-completion.md`): NO dropping hop — translate /
  relocate / addTargetVolume all reach `data-instances-json`, but 10–14 s late, because of a refresh STORM: (1)
  `register_brush_mesh`'s `request_reupload` widened its `Quiet` scope to the viewport unconditionally on every
  idempotent repeat (23/33 completions viewport-scope, 23/31 refresh passes all-`unchanged`) → fixed, 3 laws
  (rides #53); (2) `dispatch_interaction_action` returns `UiDirtyScope::Full` for ALL six interaction verbs incl.
  `interactionHover` → pointer motion repaints the whole shell → wave B34 (app-declared interaction scope).
  Re-attribution: outliner hide/show PASS (13.4 s), gumball-scene-delta intermittent at the 30 s budget, relocate's
  drag never dispatches (`beginRelocateDrag` hit-test — B28 family), `Agent disconnected` = MCP bridge steady state.
| B34 | 10:10 (09-12) | interaction verbs dirty `Full` (hover repaints the whole shell) → app-declared interaction scope; relocate drag hit-test | `📓️2026-09-12-wave-B34-interaction-scope.md` |
- 10:15 (09-12) build #53 (compile only, pid 94221; B31 + B32b guest) started; deploy after B33 reports.
- 11:05 (09-12) B33 landed (`📓️2026-09-12-wave-B33-full-run-vs-fresh-lane.md`): ONE polluter — `brush-stroke`. Root cause:
  `World3dHost.dispatch` discarded `onAction`'s promise (the only thing that settles on `OperationCompleted`), so every
  "one round trip outstanding" gate was inert: a 70-move hover storm enqueued 72 `interactionHover` + 85
  `suggestionsTick` turns (11/10 settled) and `addTargetVolume` starved behind ~136 turns at ~3.5/s — the mechanism
  behind every "reply never arrived" red. Fix (host-live): `dispatchSettled`, single-flight tick lane, coalesced
  reference hover, gate release on settle. Proof with the polluter still first: volume-brush add, catalogue add,
  duplicate, delete, focus, engagement input, context-menu rows (8), outliner hide/show, add-object dialog,
  settings→rail ALL PASS; 3 engine-contract laws. Probe recipes: `consoleBuf` 4 000-line ring makes `slice(mark)`
  empty in long runs (`tail=[]`, `guestTaps=[]`); `context-menu-rows` must precede `brush-stroke`. Guest halves:
  brush preview gate `no-free-candidate free=0 pending=true`, typed `brush` verb does not arm. Stale vite transform
  seen twice (200 OK + missing-export SyntaxError → `touch` the file).
- 11:12 (09-12) #53 materialize + full battery chained (`🗑️generated/deploy-2026-09-12-53.txt`, `battery-2026-09-12-53-6013.txt`).
| B35 | 11:15 (09-12) | probe console ring (indexed), step ordering; brush preview gate `no-free-candidate`; typed `brush` verb | `📓️2026-09-12-wave-B35-probe-ring-brush-gate.md` |
- 11:45 (09-12) battery #53 (`🗑️generated/probe-2026-09-12T04-33-49.md`, 1 180 s): FAULTS=0, **PASS=63 FAIL=25** (was 50/33).
  NEW PASS: add-object dialog/kinds/count, catalogue add + drag-drop, context-menu-opens, duplicate + reselects clone,
  engagement abort/clear/fill-verb/input/placeholder, focus-selection (B33's dispatch gate + B31). Remaining 25:
  first-pick Inspection + lock chrome + clipboard + volume-brush arm + export + settings→rail + camera-history +
  projection-repaint + locale + outliner-hide-control in the FULL run only (pass in fresh lanes) → wave B36 (bisect
  round 2); gumball/relocate delta (B34), brush preview + `brush` verb + context-menu vocabulary ordering + import
  taps (B35), `addObjectKind` does not select what it adds, delete-selection census confounded by a late duplicate.
| B36 | 11:50 (09-12) | full-run bisect round 2: first-pick Inspection, lock chrome, clipboard, volume-brush arm, export, settings→rail, camera history, projection repaint, locale, outliner hide control | `📓️2026-09-12-wave-B36-full-run-bisect-2.md` |
- 12:55 (09-12) B34 landed (`📓️2026-09-12-wave-B34-interaction-scope.md`): app-declared interaction scope via a new
  `ArtifactApp::interaction_scope(verb, domains)` hook (default None → Full, zero churn for 40 apps); puzzle3d
  declares hover → world body only, select/clear/selectAll → world + inspector + outliner + history + measures,
  mode/granularity → world + measures. 6 laws. Measured before on #53: 22 of 26 refresh passes were `full`, 68 % of
  requested bodies answered `unchanged`; after needs #54 (guest). Relocate: the press point was 265 px from the
  object — the selection was a GATE on the press point although the args are a travel delta → an empty-ground press
  with a live selection now grabs the anchor as base point; `relocate-pose-delta` FAIL 30 s → PASS 2.7 s (twice),
  gumball-scene-delta PASS in the same run.
- 13:00 (09-12) build #54 (compile only, pid 20179; B34 guest interaction scope) started; deploy after B35/B36 finish their lanes.
- 13:30 (09-12) B35 landed (`📓️2026-09-12-wave-B35-probe-ring-brush-gate.md`): probe ring → monotonic `consoleSeq`
  cursors; `context-menu-rows` leads its group and disarms utilities → vocabulary + zoom-row PASS. Brush preview:
  the suggestions tick resolved its target from menu + hover only while the render used the latched
  `brush_live_target`; with hover cleared and `brush_cache` dropped by every accepted `registerBrushMesh`, the render
  asked for a vortex no tick would warm (`free=0 pending=true`) → third leg on the latch,
  `PUZZLE3D_BRUSH_WARM_TICKS = 2`, laws warm in 1 tick; `brush-preview-place` PASS (rides #54). Typed `brush`: the
  probe read the wrong pane (fixed, PASS in isolation); behind `brush-stroke` the effect arms the map but no render
  follows for 30 s — `applyHostEffects`' trailing `refreshUi` vs in-flight coalescing (ShellHost :5537/:4747) → host
  wave; `engagement-abort` was passing vacuously (open, B31's arm).
| B37 | 13:35 (09-12) | a refresh requested during an in-flight pass is dropped (effects like SetActiveUtility render only on the next keystroke) → exactly-one merged follow-up | `📓️2026-09-12-wave-B37-effect-refresh-coalescing.md` |
- 14:40 (09-12) B36 landed (`📓️2026-09-12-wave-B36-full-run-bisect-2.md`): 13 verdicts, four causes, two product. (1) the
  shell auto-reveals Inspection on a pick and the probe's "open the tab" click COLLAPSES it (rows 12 → 0 → 12) — same
  for History (`setCamera` emits no history: the product was right) and the Artifact panel; (2) delete's precondition
  select reads empty after `catalogue-panel` (probe); (3) export: Concrete 7.5 KB downloads, Nakagin 145 KB does not —
  `export_fixture` never uses the framework's segmented-download lane (32 MiB cap, host drain) → product; (4) locale,
  clipboard, duplicate-reselects fail in fresh lanes too (`selectionState()` reads a `selection` field
  `data-interaction-json` never carries). Product fixes: `WindowConfigOwnerRegistry::capture` falls back to
  `focused_window_id` (Settings rendered the default while writing the focused pane); `addObjectKind` selects what it
  adds (Interaction lane declared). Both ride #55. Peer-owned: three laws die on `dispatch(CLEAR_SELECTION)` no longer
  clearing.
| B38 | 14:45 (09-12) | probe: ensurePanel, settings by id, projection controls, selectionState from `data-selection-json`; product: segmented export download for large fixtures; clipboard fresh-lane red | `📓️2026-09-12-wave-B38-probe-recipes-export-segments.md` |
- 14:50 (09-12) build #55 (compile only, pid 55428; B34 + B35 + B36 guest) started; deploy + battery after B37/B38 report.
- 16:00 (09-12) B37 landed (`📓️2026-09-12-wave-B37-effect-refresh-coalescing.md`): the lost refresh was never asked for —
  `typedOperationCompletionRefreshV1` correctly answered `none` (arming a utility is host-owned) and ShellHost forwarded
  `none` verbatim; fix `hostEffectRefreshScopeV1` (declared scope ∪ what applying the effects earned) +
  `createUiRefreshCoalescerV1` replacing the hand-rolled loop (a rejected pass dropped the owed follow-up, joiners awaited
  the wrong pass, re-entrant requests started a second concurrent pass); 7 laws. `engagement-brush-verb` PASS isolated;
  behind `brush-stroke` the pane renders armed but the probe reads identical values for BOTH panes because the world
  record is assembled under one shared `recordKey: puzzle.3d.play.viewport` → wave B39 (per-instance world record).
| B39 | 16:05 (09-12) | world record keyed by the authored key `puzzle.3d.play.viewport` is shared by both panes on the host → per-window-instance record | `📓️2026-09-12-wave-B39-per-instance-world-record.md` |
- 16:10 (09-12) #55 materialize + full battery chained (`🗑️generated/deploy-2026-09-12-55.txt`, `battery-2026-09-12-55-6013.txt`); host live incl. B33/B37; B38/B39 in flight.
- 16:45 (09-12) battery #55 did not boot: pageerror `📤️SegmentedDownload/🟦️.ts does not provide an export named …` — B38's in-flight segmented-export host edit (or a stale vite transform); re-run after B38 lands (touch the module first).
- 17:20 (09-12) B38 landed (`📓️2026-09-12-wave-B38-probe-recipes-export-segments.md`): six probe recipes (`ensurePanel`
  never clicks an open tab, settings by id, real projection controls, `selectionState()` from `data-selection-json`,
  16-id census cap removed) → locale-control-present, catalogue-add-selects, duplicate-reselects, locked-flag-row,
  clipboard PASS on the unchanged guest; export over 64 KiB now streams the framework's segmented-download lane
  (guest `PuzzleCommandWorkStep::Download`, host buffered sink; 145 714 B reassembled byte-for-byte; rides #56); the
  whole segmented-download drain test corpus was in no runner (added). NEW product defect: the locale switch REVOKES
  the plugin actor (`actor-activation.revoked`, `no channel for instance 1`) → wave B40. Honest reds: `projection-
  repaints-camera` (a select flips, camera identical), `delete-selection` (census unchanged 30 s with a real selection).
| B40 | 17:25 (09-12) | locale switch revokes the plugin actor; projection option must repaint the camera; delete census | `📓️2026-09-12-wave-B40-locale-actor-revocation.md` |
- 17:30 (09-12) build #56 (compile only, pid 81673; B38 guest segmented export) started; battery #55b running (`🗑️generated/battery-2026-09-12-55b-6013.txt`).
- 18:05 (09-12) battery #55b (`🗑️generated/probe-2026-09-12T07-31-16.md`, 1 176 s, 95 steps): PASS=61 FAIL=34 but
  FAULTS=11 (5 hard, first at 86 s = the locale switch in the read group → actor revoked, B40) so the rest of the
  read group and the DE label verdicts measured a dead guest; NEW PASS: export-only + export-names-the-example,
  inspection-object-fields, inspection/locked flag rows, locale-control-present + no-english-leak, outliner hide
  control, context-menu vocabulary/zoom, clipboard + selection preconditions. The mutate group ran on a Nakagin
  document with 175 instances after brush strokes (`puzzle3d.brush.*` objects), which changes every census reading.
- 18:15 (09-12) #56 materialized (B38 guest segmented export); the full battery waits for B40 (locale switch kills the actor at the 86 s mark of every full run).
- 18:50 (09-12) B39 landed (`📓️2026-09-12-wave-B39-per-instance-world-record.md`): the record WAS per instance at every
  hop; the real defect was `World3dHost`'s leftover InteractionView overlay — ONE module-level `let` for every pane of
  every document, laid over each pane's record last (last-writer-wins: `-top`'s `setCamera`/`registerBrushMesh`
  leftover wiped `-perspective`'s arm) — plus two arm routes (action vs guest `setActiveUtility` effect) of which only
  the action published the overlay. Fix: per-instance overlay registry with declared scope (`window` / `document` /
  `allWindows`), `setActiveUtilityForWindow` as the single arm authority. Live on #56: the two panes diverged for the
  first time (`select` vs `volumeBrush`); `--only=brush-stroke,engagement-bar,volume-brush` → PASS=15 FAIL=0 FAULTS=0
  (engagement-brush-verb and engagement-abort green). Host-only.
- 18:55 (09-12) full battery on #56 with every step except `locale-switch` (B40 pending) started (`🗑️generated/battery-2026-09-12-56-nolocale-6013.txt`).
- 19:30 (09-12) B40 landed (`📓️2026-09-12-wave-B40-locale-actor-revocation.md`): changing the UI language HOT-SWAPPED every
  plugin — `establishPrimarySession` had `uiLocale`/`uiTerminology` in its deps → `PluginSource` subscription re-ran →
  a fresh SSE connect replays a full snapshot → every loaded plugin routed to `reloadPlugin` → `destroyApp`. Fix: refs
  instead of deps + `pluginAvailabilityRouteV1` (a replayed snapshot can never destroy a live instance; law
  installs=3 hot-swaps=0 drops=6) + a full `refreshUi` on locale change so guest bodies relabel. Live: locale switch
  keeps both windows, faults 12 → 0, DE document labels PASS. `projection-repaints-camera` = probe timing (score
  `value`, use `cameraSettled`); `locale-de-document-section-label` demands the `reuse` terminology word while the
  shell is on `native` ("Objekte" is correct); delete-selection PASS in isolation and behind the locale step.
  Handover: `framework.worldOrbit.projection` is a hardcoded id rendered once per world surface (duplicate ids).
| B41 | 19:35 (09-12) | duplicate `framework.worldOrbit.projection` id per world surface; probe scoring for projection repaint + DE section label | `📓️2026-09-12-wave-B41-projection-id-probe-scoring.md` |
- 19:37 (09-12) full battery WITH the locale step on #56 (host incl. B39/B40) queued behind the no-locale run (`🗑️generated/battery-2026-09-12-56-full-6013.txt`).
- 20:20 (09-12) battery #56 (no locale; `🗑️generated/probe-2026-09-12T08-08-54.md`): PASS=61 FAIL=29 but a NEW guest death at
  740 s: `shard 0 terminated by the host watchdog: the worker was silent for 18 603 ms; outstanding: cancelJob
  puzzle#1` — right after `engagement-abort` PASS (B31's Escape now cancels the in-flight fill plan via
  `Effect::CancelJob`) the guest's cancel turn never yields → watchdog kills the shard → actor revoked → every later
  verdict (outliner hide, catalogue, duplicate/delete, export, import, guest-alive) measured a dead guest. The mutate
  group also runs on 176 instances after the brush stroke (`puzzle3d.brush.*` objects painted on hovered vortices).
| B42 | 20:25 (09-12) | `cancelJob` for the fill plan is a non-yielding guest turn (18.6 s) → watchdog kill; cancellation and drops must be bounded per turn | `📓️2026-09-12-wave-B42-cancel-job-watchdog.md` |
- 20:50 (09-12) full battery #56 WITH locale (`🗑️generated/probe-2026-09-12T08-28-10.md`): PASS=56 FAIL=39, faults 15 —
  the locale switch no longer kills the actor (B40 holds); the same `cancelJob` watchdog kill at 755 s (B42) plus a NEW
  `shard 0 worker fault [handler/takeSegmentedDownloadChunk]` (B38's segmented export guest half on #56 faults in the
  bridge) → wave B43; the window-option / projection controls read "absent from the measures rail" because B41 is
  mid-rename of the measure ids (probe locators vs live ids) — re-measure after B41.
| B43 | 20:55 (09-12) | `takeSegmentedDownloadChunk` worker fault on the Nakagin export (segmented lane guest/bridge half) | `📓️2026-09-12-wave-B43-segmented-download-fault.md` |
- 21:40 (09-12) B41 landed (`📓️2026-09-12-wave-B41-projection-id-probe-scoring.md`): the projection pane, its switch rows
  and the measures rail produced ELEVEN duplicated DOM ids per extra pane (kind-level `id_prefix`) → host-side
  instance qualification (`childElementId`, `qualifyWindowMeasureIds`) mirroring `uiNodeDomId`, fold-state bleed
  between panes fixed, CAD's invalid `cad-orbit-projection` id fixed; 4 laws. Probe: `cameraSettled` for the
  projection repaint, DE label from the ACTIVE terminology; select triggers have no DOM value → product stamps
  `data-published-value` on every draft-bearing rail control (law). Live: `projection-options,locale-switch,
  window-content` 16/16 PASS, 0 faults. Note: `🎨️r3f` vitest config cannot load (strip-only TS import) — its in-source
  laws are unrun; a `ReferenceError` in the probe's `locked-refusal` step fixed.
- 21:45 (09-12) checkpoint battery on #56 with B39/B40/B41 host-live queued (`🗑️generated/battery-2026-09-12-56c-6013.txt`); B42/B43 in flight.
- 22:10 (09-12) checkpoint battery did not boot: `📤️SegmentedDownload/🟦️.ts does not provide an export named 'MAX_SEGMENTED_DOWNLOAD_BYTES'` — B43 mid-edit of the segmented-download contract; re-run after B43 (touch the module).
- 23:05 (09-12) B43 landed (`📓️2026-09-12-wave-B43-segmented-download-fault.md`): no limit fired — the bridge's
  `takeSegmentedDownloadChunk` handed jco's TAGGED `option<list<u8>>` (`{tag,val}`) to the worker unwrapped, so chunk 0
  was refused as `[object Object]` (the only guest option not routed through `unwrapOption`); new schema-first
  contract leaf `📮️shard-client/📤️segmented-download` (chunk bytes, outstanding, total, refusal codes) replaces four
  `4_096` literals; over-cap export refuses with a notice. Bridge law fails with the line reverted; worker law streams
  145 KB (36 chunks) and 32 MiB−1 (8 192 chunks). Served `🌉️bridge.js` restaged (no wasm rebuild). :6013 answers 500
  (`Failed to resolve import "@semio-tech/framework-renderer-react"` — a peer's install recreated `node_modules`
  under the running vite) → recycling the serve, then the checkpoint battery.
- 23:40 (09-12) serve boot broken by a peer's registry relayout (generated output moved into `🤖️generated/🧩️plugins/` at 12:34 CEST while the generator still emitted `../📦️deployment`): fixed the one emitted path at `📇️registry/📜️script.ts:696` to `../../📦️deployment`, regenerated, restarted :6013.
- 00:20 (09-13) second peer-drift repair: the ui relocation `📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx → 🎯️targets/⚛️react/🟦️.tsx` rewrote ShellScope's import to the thin test barrel (exports only 🖌️render) → pointed it at the relocated module.
- 00:30 (09-13) the same mis-rewrite hit every element importer (Tree, ContextMenu, Input, Stepper, …): repointed all of them at `🎯️targets/⚛️react/🟦️`.
- 00:45 (09-13) the repaired imports were served STALE by vite (`sed -i` replaces the inode; the new fs.watch-based watcher misses it) — `touch` on each file re-transforms; checkpoint battery #56f running.
- 01:05 (09-13) the package entry `⚛️react/📦️packages/🟦️typescript/🟦️.ts` had become a thin test adapter (only 🖌️render) while every `@semio-tech/ui-react` consumer resolves there → added `export * from "../../🟦️.tsx"` (the relocated target index).
- 01:40 (09-13) checkpoint battery #56g (`🗑️generated/probe-2026-09-12T11-06-48.md`, 1 744 s): booted, **FAULTS=0, PASS=64
  FAIL=27** (best full run so far; every host repair holds). The mutate group runs on a ~350-instance document (Nakagin
  180 + ~170 brush-painted objects) and every mutation burns its 30 s budget → the remaining ceiling is mutation
  latency vs document size (wave B44). Full-run-only reds still: engagement pane unfold obstruction, settings tab
  `puzzle3d.panel.settings` absent, export download none, volume-brush arm `select` (wave B45 bisect round 3).
| B44 | 01:45 (09-13) | mutation latency scales with document size (350 instances → 30 s per translate/delete): delta publication of the instances lane | `📓️2026-09-13-wave-B44-mutation-latency-large-document.md` |
| B45 | 01:47 (09-13) | full-run-only reds round 3: engagement unfold obstruction, settings tab absent, export in the long run, volume-brush arm | `📓️2026-09-13-wave-B45-full-run-bisect-3.md` |
- 04:10 (09-13) B42 landed (`📓️2026-09-12-wave-B42-cancel-job-watchdog.md`): the watchdog text was a misdiagnosis — no
  unyielding turn exists. (A) the shard worker had an early `if (kind === "cancelJob") { …; return }` BEFORE the
  requestId gate, so `ShellClient.cancelJob` (sent with a requestId) never got a reply/heartbeat and the watchdog killed
  the shard on the oldest outstanding request → branch deleted, 3 vm laws red→green (this is what killed #56 at 740 s).
  (B) cancellation was never incremental: puzzle3d implemented none of the `mounted_job_*` hooks and the session
  `Drop` drained up to 262 180 close units inline under the registry mutex (and could not finish) → process-wide reaper
  spending one `close_step` per granted turn, constant-time Drops; census worst_turn=1, `fill cancel abort` 91/0.
  Also repaired a peer codemod that left the puzzle3d testkit self-recursive (`context::meta`). Host part live; guest
  rides #57.
- 04:15 (09-13) build #57 (compile only, pid 47375; B42 guest reaper + whatever of B44 compiles) started; deploy after B44/B45 report.
- 05:20 (09-13) B45 landed (`📓️2026-09-13-wave-B45-full-run-bisect-3.md`): 8 verdicts, 4 causes — context-menu submenu rows
  appear ~100 ms after the probe sampled (probe poll); `unfoldPerspectiveUtilities` matched both the unfold and fold
  chips and FOLDED the bar (probe); an open History panel's full-height 3 px resize handle covered the Settings tab
  centre (product, `🖼️Panel`, law) + `ensurePanel` read attributes a tab never publishes (probe); the polluter for the
  rest is `fill-apply-max` when it lands ~150 objects → later small writes settle `historyUpserts:0` and change nothing
  in 30–45 s = B44's latency lane; Inspection `locked` row goes stale while the guest fires its notice. Export needs the
  next wasm. Blocker: :6013 down — PluginRuntime imports `🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts`, consolidated
  by a peer into that package's `🟦️.ts`.
- 05:35 (09-13) peer relocation `🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts → 🎭️actor/🖼️wire-turn/🟦️.ts` left PluginRuntime (and any other importer) on the old path → repointed by relative path, touched.
- 06:10 (09-13) third peer-drift repair: the relocated styling vite builder (`🎨️styling/🏗️builder/🌐️vite/🟦️.ts`, untracked) kept in-source test imports at `./🧪️tests/…` (tests live two levels up) and esbuild fails the config bundle → `../../🧪️tests/…`; serve restarted.
- 06:20 (09-13) #57 materialized (B42 guest reaper); checkpoint battery #57 started (`🗑️generated/battery-2026-09-13-57-6013.txt`).
- 07:30 (09-13) B44 landed (`📓️2026-09-13-wave-B44-mutation-latency-large-document.md`): the 30 s is NOT the lane payload
  (intake 282 ms, apply 0 ms) — a `deleteSelection` guest turn is 6.2 s at 180 objects and even `setCamera` 2.3–4.8 s:
  `fixture_geometry_fingerprint` materialised the whole fixture JSON 3× per refresh (54 % of a cache-hit render) and a
  mutation costs 104–235 host↔guest turns even at 1 object (one-item publication grant). Changes: per-object instance
  residency + `instancesDelta` lane (19th, all four pinned sides; delta 368 B vs 55 154 B full), structural fingerprint
  20.7 ms → 0.65 ms (32×), host in-place apply by id; Nakagin switch 1 235 → 808 ms, click→180 instances 2.76 → 1.69 s.
  NEW BLOCKING DEFECT: no selection can be established on Nakagin at all (six canvas picks return nothing, the outliner
  publishes zero entity rows) — every large-document mutation verdict measured a document with no selection → wave B46.
  Residual: the one-item publication grant (framework, Codex peer live in that file), `interactionSelect` 20× at 180
  objects, relocate gesture at 1 object.
| B46 | 07:35 (09-13) | selection impossible on Nakagin (canvas pick + outliner rows), `interactionSelect` 20× cost at 180 objects, relocate precondition | `📓️2026-09-13-wave-B46-selection-on-nakagin.md` |
- 07:40 (09-13) build #58 (compile only, pid 64404; B44 guest residency/delta lane + fingerprint) started; deploy after the #57 battery.
- 08:10 (09-13) battery #57 (`🗑️generated/probe-2026-09-12T14-31-16.md`, 1 377 s, 99 steps): **FAULTS=0, PASS=74 FAIL=25**
  (best full run). NEW PASS: the whole engagement family (input, brush/fill/clear verbs, abort, placeholder), export
  + example-named export, settings panel/steppers/bumps, window-options-emit-no-history, context-menu vocabulary.
  Remaining 25 = the mutate group on a 165-brush-object document (selection/latency: B44 guest in #58, B46 selection on
  large documents), clipboard in the long run, projection flips (qualified ids), settings→rail, camera history, locked
  refusal, brush preview, volume-brush arm (20 s), import-distinct (`guestTaps=[]`), outliner hide.
- 09:00 (09-13) #58 deployed (B44 guest) and battery (`🗑️generated/probe-2026-09-12T14-55-2*.md`, 1 562 s): FAULTS=0, PASS=73
  FAIL=25 — identical verdict set to #57 (the mutate lanes wait on B46's large-document selection). The other full-run
  reds get their own wave now: projection flips on the instance-qualified ids, settings→rail, camera history,
  clipboard in the long run, import-distinct `guestTaps=[]`, outliner hide, volume-brush arm 20 s, locked refusal.
| B47 | 09:05 (09-13) | full-run reds outside the selection family: projection flips (qualified ids), settings→rail, camera history, clipboard, import taps, outliner hide, volume-brush arm | `📓️2026-09-13-wave-B47-full-run-residuals.md` |
- 10:35 (09-13) B46 landed (`📓️2026-09-13-wave-B46-selection-on-nakagin.md`): B44's "no selection on Nakagin" was two reader
  defects (outliner rows arrive after 2.5 s; B44's probe read `ids` while the pane publishes `selectedIds`). REAL fixes:
  canvas pick/marquee were blind on GLB meshes (URL-backed mesh records carry no inline data → unit-cube AABB; now
  `GLB_MESH_LOCAL_BOUNDS` from `GlbInstanceMesh`; 0/20 → 14 candidates, hit found); the leftover overlay left
  `activeObjectId` null (gumball target / Inspection focus read nothing); `interactionSelect` built the topology TWICE
  per pick (two full fixture decodes, ~1 800 nodes) + linear id scans → generation-keyed memo + `membership()`;
  outliner rows clamp to the arena page and keep their select binding. Subset battery on #58 host-live: 17/17 PASS
  incl. outliner hide/show. STILL BROKEN: the guest never re-publishes its selection lane after a pick on Nakagin
  (`data-guest-selection-json` stays `[]` for 150 s; `puzzle.3d.play.document` never projects in that window —
  guest `plugin_refresh_ui` vs `UiDirtyScope::Partial`) and `registerBrushMesh` never drains (8 min re-announce loop).
| B48 | 10:40 (09-13) | guest selection lane not republished after a pick on Nakagin (`plugin_refresh_ui` vs Partial scope); `registerBrushMesh` re-announce loop never drains | `📓️2026-09-13-wave-B48-nakagin-selection-lane.md` |
- #59 deployed (B46 guest) 18:24; battery #59 queued (`🗑️generated/battery-2026-09-13-59-6013.txt`).
- 11:40 (09-13) B47 landed (`📓️2026-09-13-wave-B47-full-run-residuals.md`): PRODUCT — `uiIntentPayload` let a scalar gesture
  payload replace the authored args wholesale (a stepper sent `10.5`, evicting `{windowId}`) so the whole Settings panel
  was mute for scalar controls (fixed, law; `settings-value-reaches-window-rail` PASS); a full-viewport
  `div.ui-veil.z-tutorial` (z 10000, pointer-events auto) RE-ARMS after the welcome tour is skipped and makes the app
  inert — earlier full runs' `example-switch` silently did not switch (probe clears it; product defect open); the
  Inspection panel body (absolute z-30) covers the measures rail so Projection is unusable while Inspection is open
  (product, handed off); outliner Hide fails only when a SIBLING is selected (row-action explicit `{entity, ids}` lost
  in the row-action hop; tap on #59). PROBE — projection combobox polling, camera-history rows were the probe's own
  panel toggles, import fell back to a 2026-09-10 fossil export that REPLACED Nakagin. Clipboard ×3 PASS fresh.
| B49 | 11:45 (09-13) | tutorial veil re-arms after skip (inert app); Inspection panel covers the measures rail; row-action explicit args lost with a selection | `📓️2026-09-13-wave-B49-veil-inspection-overlap-row-args.md` |
- 12:30 (09-13) battery #59 (`🗑️generated/probe-2026-09-12T16-24-2*.md`, 1 446 s): FAULTS=0 but PASS=67 FAIL=32 — the whole
  pointer-driven read group is inert (camera orbit/pan/zoom `before == after`, no pick lands, mutate group on the
  1-object document) = the tutorial veil B47 found re-arming (z 10000, pointer-events auto); B47's probe-side
  `clearIntroductionVeil` did not cover this run's reload path. #59's verdicts are therefore confounded; B49 owns the
  product fix (Skip must dismiss the veil durably). New PASS anyway: camera-emits-no-artifact-history, clipboard
  copy/paste hops.
- 12:45 (09-13) correction on #59: the veil was cleared at 48 s and picks DO land (leftover `selectedIds:["seed-left-001"]`);
  what died is the CAMERA lane — orbit/pan/zoom `moved=false` for 30 s each while #57/#58 moved in ~1 s. `data-camera-json`
  mirrors the guest-published pose (B12), so `setCamera` → WindowConfig lane → republish regressed with #59's guest
  (B46: generation-keyed `interaction_topology_memo`, outliner page clamp, pick bounds are host) → wave B50 (bisect B46's
  guest hunks against #58). Verdicts downstream of the camera (projection flips, focus/zoom) are collateral.
| B50 | 12:50 (09-13) | #59 regression: `setCamera` no longer republishes the camera lane (orbit/pan/zoom dead) — bisect B46 guest hunks vs #58 | `📓️2026-09-13-wave-B50-camera-lane-regression.md` |
- 13:40 (09-13) B50 landed (`📓️2026-09-13-wave-B50-camera-lane-regression.md`): the camera lane is fine — `setCamera` is
  dispatched, run and settled, but the guest reactor LIVELOCKS after the first gesture: the re-admitted `1:window`
  alias surface holds a zero-byte output that never acks/retires, every later reconcile is refused-and-dropped
  (`more-work streak` 2 323 → 7 843, `sources=["reconcile"]`), so every retained surface stays at its boot revision
  (`hashes …:1`) — same wedge B48 is on (identical hashes hours apart); #59 only changed the surface traffic.
  Camera/projection/inspection reds on #59 are collateral; a camera refresh-route law now exists (passes natively).
| A6 | 13:50 (09-13) | read-only `[DEBUG]`/flood inventory for the close-out sweep (attribution per wave, keep/remove) | `📓️2026-09-13-audit-A6-debug-trace-inventory.md` |
- 14:10 (09-13) A6 landed (`📓️2026-09-13-audit-A6-debug-trace-inventory.md`): 419 production `[DEBUG]` hits in scope, 281
  pre-session, 138 session-added → 99 REMOVE, 22 KEEP (gated), 17 PEER; a NEW ungated flood `puzzle3d.brushPreview.bind`
  / `.assemble` (World3dHost :5026, Interpreter :1437; 174 lines = 14.5 % of a console tail); 44 ticket input scripts to
  keep. The sweep wave (B51) runs after B48/B49 land so it does not clobber live edits.
- 14:50 (09-13) B49 landed (`📓️2026-09-13-wave-B49-veil-inspection-overlap-row-args.md`): the welcome-tour veil never re-armed —
  the FIRST Skip could not be pressed: `resolveIntroductionPlacement` returned the centered branch unclamped (Skip at
  y −78) and `WindowChrome`'s `z-[1]`/`z-[2]` arbitrary utilities are not emitted by the stylesheet so the body plane
  swallowed every cap-row press → clamp all branches, real inline stacking, a press on a blocking veil ends the tour,
  session-durable dismissal (4 laws). Inspection over the measures rail → shell-root-keyed dock-column reserve
  consumed by `🪟️Window` (rail x 1130 → 833, listbox opens in 1 ms; law). Row-action args: the host sends
  `{entity, flag, ids, value}` verbatim — the three silent exits are in the guest's staged prologue (B48's lane).
  All host-live.
- 15:50 (09-13) B48 landed (`📓️2026-09-13-wave-B48-nakagin-selection-lane.md`): the guest UI freeze was never Nakagin-specific
  — `MountedReconcileGrant`'s `Drop` released less than `cancel` (never cleared `slot.output_index`, never closed the
  reserved `ready` output); `commit_source` has seven `Err` exits taking the Drop path (`let _ = grant.commit_source`)
  and `reserve_mounted_owned` refuses any slot with `output_index.is_some()` → one refused commit makes a surface
  permanently un-reservable (refuse → defer → re-dirty forever, `streak=62 288`, all 13 surfaces deferred) and leaks
  1 of 64 output slots per exit. Fix: one shared `release()`, refused commit/defer become named shell faults,
  `redirty_acknowledged_deferred_surfaces` drains all ready deferred surfaces per turn, `reserve_refusal` in
  `debug_state`. Mesh loop: three announce paths left the standing request unretired + host `claimReupload` gate moved
  by its own traffic → `retire_mesh_reupload` on all paths + `PUZZLE3D_MESH_REUPLOAD_CLAIMS = 2` (live: 60 commands in
  30 s then zero). ⚠️ a peer's `♻️reconcile` edit (17:07 CEST) no longer terminates its retirement ladder (3 `patches::`
  laws + B2's law red with B48 on and off) — #60 is built from that tree. Build #60 + deploy + battery chained.
| B51 | 16:00 (09-13) | close-out `[DEBUG]` sweep (gated: Rust after #60 compiles, host TS after the #60 battery) | `📓️2026-09-13-wave-B51-debug-sweep.md` |
- 17:05 (09-13) #60 deployed (B48 guest) and battery (`🗑️generated/probe-2026-09-12T18-29-0*.md`, 1 297 s): FAULTS=0,
  PASS=64 FAIL=33 — the camera is STILL frozen (orbit/pan/zoom before == after), i.e. the guest still livelocks after the
  first gesture: B48 warned that a peer's `♻️reconcile` edit (17:07 CEST) no longer terminates its retirement ladder
  (3 `patches::` laws + B2's mixed-surface law red with B48 on and off) and #60 was built from that tree → wave B52.
| B52 | 17:10 (09-13) | reactor retirement ladder no longer terminates on #60 (peer's ♻️reconcile edit; alias surface zero-byte output): bounded livelock + termination laws | `📓️2026-09-13-wave-B52-reconcile-ladder-livelock.md` |
- 17:55 (09-13) B51 landed (`📓️2026-09-13-wave-B51-debug-sweep.md`): 148 session-added `[DEBUG]` lines re-derived; 53 removed
  in scope (incl. the `brushPreview.bind/.assemble` flood and the guest `puzzle3d.import.*` taps), 46 remain classified
  KEEP-gated/KEEP-doc/PEER; `.catch` handlers and named fault reports kept with the prefix stripped; three
  literal-pinning laws repaired properly (peer contributions block restored verbatim + PEER; B31's gumball landmark made
  a permanent record). Findings: the biggest console families (`performInvocation`/`command ingress`, 43 % of a tail)
  are PRE-session in PluginRuntime (`grep` treats those long-line files as binary — use `grep -a`) → follow-up outside
  this ticket's sweep rule; two close-out paths a glob misses: `🗑generated/` without U+FE0F (59 files, 3.6 MB) and a
  corrupted nested `<U+FFFD>️26/…` tree. 0 errors on all three checks, renderer 1037/6 (baseline), ui-react 718/13.
- 19:20 (09-13) B52 landed (`📓️2026-09-13-wave-B52-reconcile-ladder-livelock.md`): the peer's `♻️reconcile` relocation is
  near-pure (no peer hunk broke the ladder); `outSome(0)` was output SLOT 0, not a zero-byte output. Root cause
  (ours, pre-existing): `retire_exact` priced the DOCUMENT retirement ladder with the caller's fixed 4 096-byte grant
  while it retires 6 416-byte `UiNodeRecord`s that a list page frees whole or not at all → `leaf-starved` forever,
  every surface deferred, the handback registry holds the owner, pool exhaustion refuses every reservation
  (`registry-reservation-unavailable` ×6 035). Fix: grant priced from the value's allocated bytes + a named stall
  fault at 4 096 no-progress steps. `patches:: reconcile_budget` 4 red + B2's law spinning → 42/42 green in 0.6 s.
  Guest-only → build #61 + deploy + battery chained.
- 20:50 (09-13) #61 deployed (B52 guest) and battery (`🗑️generated/probe-2026-09-12T20-23-3*.md`, 1 640 s): **FAULTS=0,
  PASS=71 FAIL=25** — the livelock is gone (camera orbit/pan/zoom, projection flips + repaint, settings→rail,
  camera-history all PASS). Remaining 25: the mutate group on a 161-brush-object document (gumball/relocate/duplicate/
  delete/catalogue add at 30 s — B54), Nakagin export `download=none` in the full run + the import chain behind it
  (B53), context-menu hand rows + zoom camera, locked refusal precondition, outliner hide control, volume-brush arm
  20 s, clipboard in the long run (B55).
| B53 | 20:55 (09-13) | Nakagin export in the full run (segmented lane end to end on #61) + import chain | `📓️2026-09-13-wave-B53-nakagin-export-full-run.md` |
| B54 | 20:55 (09-13) | mutation latency on the brush-painted document round 2 (turn count per mutation, one-item publication grant) | `📓️2026-09-13-wave-B54-mutation-latency-2.md` |
| B55 | 20:55 (09-13) | context-menu hand rows/zoom camera, locked refusal, outliner hide control, volume-brush arm, clipboard in the long run | `📓️2026-09-13-wave-B55-full-run-bisect-4.md` |
- 01:10 (09-14) B54 landed (`📓️2026-09-13-wave-B54-mutation-latency-2.md`): unit census splits B44's "104–235 turns" —
  publication ladder 17 turns (document-independent), page 3, retirement 1, worker 520–5 353 units (busy-wait poll
  count, not round trips). Fixes: the publication ladder runs a bounded slice per turn (21 → 4/5 turns, size-
  independent, store grant untouched), `Puzzle3dSceneInvalidation` per changed object (was a whole-document rebuild
  re-paid every 120 ms tick), `build_history_view` O(n²) → id index + op-line reuse; 3 laws + 1 rewritten; framework
  publication laws 20/3 vs baseline 18/5. Repaired a peer's uncompilable kernel hunk (borrow split, ledger kept).
  BROWSER CEILING on #61: after a selection on Nakagin, `deleteSelection` never lands in 196 s —
  `reserve_refusal=1:window:registry-reservation-unavailable`, six surfaces deferred, `sources=["reconcile"]`,
  settled=0 → the `1:window` ALIAS surface wedges the registry again (B48 residual 1/2). → wave B56: retire the alias
  (leftover patches must address the real window instance) so no synthetic surface can hold the registry.
| B56 | 01:15 (09-14) | `1:window` alias surface wedges the reconcile registry after a Nakagin selection: address leftover patches to the real instance and retire the alias | `📓️2026-09-14-wave-B56-window-alias-retirement.md` |
- 02:20 (09-14) B53 landed (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md`): the segmented export lane is GREEN live
  (145 714 B `nakagin-capsule-tower.json`, one blob, 180 objects re-parsed); the failing hop was the PRESS — the Export
  row sits at y=1990 in a 58…865 band and the probe never scrolled the rail; the click then stalls on the Nakagin
  main thread and the file lands 21–35 s later past the 20 s budget. Product fixes: `windowActionPaneNode` now filters
  `inPalette` (96 → 81 rows); the peer's `semioSourceWatchVitePlugin` mapped every macOS `fs.watch` `rename` to `add`
  while vite invalidates only on `change` → EVERY edited source module was served pre-edit for the life of the server
  ("host-live" was not live since the 04:37 recycle unless touched) — fixed to replay `add`+`change`. Probe recipe
  (scroll, click budget, 60 s download wait) → B55. → the serve gets recycled before #62 so the watcher fix is in.
- 03:25 (09-14) B55 landed (`📓️2026-09-13-wave-B55-full-run-bisect-4.md`): ONE product fix cleared six reds — the stylesheet
  carrying `@import "tailwindcss"` declared no `@source` for its own module, so `.z-menu/.z-dialog/.z-pane/.z-base/
  .z-navbar` and `max-h-layout-command` were never emitted: the context menu painted UNDER the world canvas
  (`elementFromPoint` → CANVAS), forced clicks hit the canvas, the paste form's Execute was unreachable. Probe: fill
  steps trail their group (ribbon reachable), `ensurePanel` makes the body the authority, gumball grab re-selects,
  every mutate verdict carries `instances=<n>`. B55's own full battery on #61 + host-live: **PASS=89 FAIL=10 FAULTS=0**.
  Remaining: `translateSelection` dispatched but never settles on the large document (B54/B56 lane), outliner-show-
  restores (row label restores, world scale does not — unowned), a `worldPick` dropped-action error per canvas press.
| B57 | 03:35 (09-14) | port B53's export recipe (scroll, click budget, 60 s download wait) + import chain, example-switch verdict, outliner Show not restoring world scale | `📓️2026-09-14-wave-B57-export-recipe-show-restore.md` |
- 05:10 (09-14) B56 landed (`📓️2026-09-14-wave-B56-window-alias-retirement.md`): `registry-reservation-unavailable` was never a
  property of `1:window` — it is whichever surface asks FOURTH: one reconcile reservation asks 8 MiB
  (`UI_RESIDENT_SURFACE_BYTES`) of a 32 MiB aggregate → exactly THREE concurrent reconciles process-wide while a
  puzzle3d session mounts thirteen surfaces (`registry=resident=3s/…/25 875 744 B of 33 554 432 B`, handback 378/384
  free). The alias is retired anyway (4 host + 3 guest sites, body-less patches refused by name, job progress now
  reaches the real panes, 25 % of every world publication gone); 25/25 laws. Peer breakages: react vitest lane and the
  repo-root `📜️script.ts` currently broken. → wave B58: size the reconcile reservation to the tree actually reconciled
  (`try_shrink` runs AFTER reservation today) so thirteen surfaces fit — the last convergent blocker for mutations.
| B58 | 05:15 (09-14) | reconcile reservation 8 MiB × 3 of 32 MiB starves every fourth surface: size reservations to the tree, all mounted surfaces reconcile | `📓️2026-09-14-wave-B58-reconcile-reservation-ceiling.md` |
