# Audit C2 — Remaining Puzzle 3D end-to-end gaps (#45b full battery)

Ticket `26/09/02/PUZZLE-3D-END-TO-END` (moon `🌙️09/☀️02`), read-only audit 2026-09-11 (Composer 2.5 auditor).
**Goal is not complete.** C1 used a partial ndjson + aborted battery tail; **C2 uses the complete rendered probe markdown only.**

---

## Evidence stack

| artifact | wasm / serve | what it proves |
|---|---|---|
| `🗑️generated/probe-2026-09-11T14-48-04.md` | **#45** (`:6013`, B0+B2+B3+B8+B10 host partial) | **Full battery finished** — `1089.2s`, **160 verdict lines** (80 unique: 40 PASS / 40 FAIL), `faults (raw 0, hard 0, collateral 0, distinct 0)`, `battery-hard-faults PASS`. No `AliasCapacity` / slot-retirement storms. |
| `🗑️generated/battery-2026-09-11-45b-6013.txt` | same run | Step timeline companion to the md. |
| `📓️2026-09-11-wave-B12-windowconfig-roundtrip.md` | #45 + vite-live host | Six `window-option-*` + sun toggle **PASS** on isolated re-probe after measure-draft fix (supersedes #45b FAIL rows for that lane). |
| `📓️2026-09-11-wave-B12-window-config-lane.md` | laws only on #45 | Guest publish/refresh round-trip landed; browser on `:6014` **not run** (port contention). |
| `📓️2026-09-11-wave-B13-selection-landing.md` | #46 + vite-live host (`:6014`) | Inspection hash-bust when `leftover.ids` non-empty; hover-carry guest law. Browser: leftover still `selectedIds:[]` after `interactionSelect` — pick publication miss, not hash-bust. |
| `📓️2026-09-11-wave-B14-fill-planner.md` | vite-live host | `activeToolId` on leftover + third `worldFillBuildShouldTick` arg; laws green; browser `--only=fill-tab` **not run**. |
| `📓️2026-09-11-wave-B14-settle-semantics.md` | #46 + vite-live host (`:6013`) | Removed `empty-required stop`; `catalogue-drag-drop` **PASS**; `catalogue-add-object-kind` still FAIL (one-command lag). |

**#46** is on disk (deploy ~17:13); post-B12/B13/B14 full-battery re-run on `:6013` is the gating evidence C2 does not have yet.

---

## Status legend (wave column)

| tag | meaning |
|---|---|
| **#45b FAIL** | Still red in the full battery md — baseline for this table. |
| **B12 closed** | Isolated re-probe PASS on same wasm #45 + vite-live host; full-battery row stale. |
| **B12 partial** | Host/guest fix landed; needs wasm #46 scope class or probe locator update. |
| **B13 partial** | Host hash-bust proven; guest leftover publication still empty on #46 probe. |
| **B14 partial** | Vite-live settle/fill fixes; browser not re-run on full fill lane in this wave. |

---

## FAIL-only table — all 40 unique verdicts from #45b

| # | verdict | #45b evidence (summary) | wave since C1 | next hop |
|---|---|---|---|---|
| 1 | `projection-repaints-camera` | `data-camera-json` unchanged after projection measure nudge | B12 §2.3: `data-camera-json` now guest pose; may need longer wait or guest `setProjection` repaint | Re-probe `--only=projection-options` on vite-reloaded `:6013`; if still FAIL → guest projection publish → measures rail |
| 2 | `window-option-puzzle3d-play-grid-visible` | `before==after` checked=true | **B12 closed** — PASS isolate (`b12-window-options-6013.txt`) | Confirm in post-B12 full battery; no further guest hop for toggles |
| 3 | `window-option-puzzle3d-play-grid-snap` | `before==after` checked=false | **B12 closed** | same as row 2 |
| 4 | `window-option-puzzle3d-play-lod-auto` | `before==after` checked=true | **B12 closed** | same as row 2 |
| 5 | `window-option-puzzle3d-play-vortex-show` | trigger text `Selected` unchanged | **B12 closed** | same as row 2 |
| 6 | `window-option-puzzle3d-play-vortex-direction` | trigger text `Outwards` unchanged | **B12 closed** | same as row 2 |
| 7 | `window-option-puzzle3d-measure-sun-enabled` | `before==after` checked=false | **B12 closed** | same as row 2 |
| 8 | `settings-steppers-present` | `ids=["puzzle3d-play-settings"]` only tree row | **B12 partial** — four steppers live under `panel:puzzle3d-play-settings/…` in browser; probe locators stale (`uiNodeDomId` prefix) | Update `🔍️browser-probe.ts` §19 selectors to `panel:<bodyKey>/…` or `[id$="…"]`; then `--only=settings-panel` |
| 9 | `settings-grid-spacing-bumps` | `plusButtons=0` | **B12 partial** — `Stepper` `onDelta` only when declared; mousedown/mouseup now dispatches `setGridSpacing` | Probe locator fix (row 8); optional: active-pane `windowId` in panel context (B12 §5.1) |
| 10 | `settings-value-reaches-window-rail` | `settings=null`; rail slider draft only | **B12 partial** — rail value now published (not draft-only) per roundtrip doc | Probe fix + verify settings bump targets focused pane not `puzzle3d-main` roster fallback |
| 11 | `add-object-trigger-present` | `trigger=0 menu=[]` | B11 menu promotion in tree, not in #45 wasm | **#46** + `--only=add-object-dialog`; manifest `openAddObjectDialog` row |
| 12 | `locale-control-present` | `switched=false`; no language control | B10 DE law green; control discoverability unproven | `--only=locale-switch` on stable actor (#46); settings language control id |
| 13 | `fill-history-entry` | `entries=0` after fill apply | **B14 partial** — leftover `activeToolId=fill` + tick call site; not browser-proven | Vite reload + `--only=fill-tab,fill-apply-max` on #46; confirm `fillBuildTick` + count > 0 |
| 14 | `inspection-object-fields` | `id=null populated=false` | **B13 partial** — hash-bust when ids present; #46 probe never got `selectedIds` | Guest leftover publication after `interactionSelect` (clickForest + Frame); re-probe `selection-surfaces` logging `selectedIds:["seed-left-001"]` |
| 15 | `inspection-locked-flag-row` | `lockChrome=false` | **B13 partial** — same hop as §14 | same as row 14 |
| 16 | `clipboard` | `delta=0`; `wireEffectToFriendly` drops `send-message` | B14 settle unblocks mutation frames; clipboard mapping still unmapped | After §14 selection: map clipboard `send-message` in `wireEffectToFriendly`; `--only=clipboard-copy-paste` |
| 17 | `locked-flag-row` | `lockChrome=false` | cascade from §14 | row 14 first |
| 18 | `locked-refusal-notice` | `notices=[]` | cascade from §14 | row 14 + lock toggle with selection |
| 19 | `gumball-scene-delta` | `poseLen=266` unchanged; handle enter PASS | B9 dirty-scope on #46; needs live selection | §14 + `--only=gumball-drag` on #46 |
| 20 | `brush-preview-place` | `preview=null`; utility stays `select` | B9 per-window utility **not in #45** | **#46** + `--only=brush-stroke`; confirm `activeUtility=brush` |
| 21 | `volume-brush-arm` | `activeUtility=select` | B9 + B11 host gesture vite-live | **#46** + vite reload + `--only=volume-brush` |
| 22 | `volume-brush-add-target-volume` | `before=0 after=0` | **B14 partial** — settle fix lets mutations complete; arm still blocked by utility | utility arm (row 21) then re-probe; guest typed-op scheduling lag may need wasm hop |
| 23 | `volume-brush-voxel-dims` | `before=null after=null` | cascade from row 21 | row 21 first |
| 24 | `relocate-pose-delta` | `beforeLen=afterLen=266`; arm PASS | needs selection + mutation settle | §14 + `--only=relocate` |
| 25 | `engagement-brush-verb` | `activeUtility=select` | B9 utility publication | **#46** + `--only=engagement-bar` |
| 26 | `engagement-fill-verb` | `#tool.fill aria-pressed=null` | **B14 partial** — tool leftover carry | vite reload + fill tab probe |
| 27 | `context-menu-selection-precondition` | `outlinerRows=1 selected=0` | B11 context hits **#46** | §14 selection first |
| 28 | `context-menu-opens` | `rows=0` | cascade | §14 + B11 `defaultPrevented` guard on #46 |
| 29 | `context-menu-object-vocabulary` | all object rows missing | cascade | same as row 28 |
| 30 | `context-menu-zoom-row-action-is-registered` | zoom row absent | B11 `focusSelection` vs `zoomToSelection` checklist drift | register row action id + probe expectation |
| 31 | `outliner-hide-control-present` | `byText=0 byLabel=0 controls=[]` | B13: **PASS** `outliner-hide-control-present` on #46 isolate — #45b row stale | Re-probe `outliner-rows` on #46; if PASS → `outliner-hide-applies` hop |
| 32 | `catalogue-add-object-kind` | `before=1 after=1` | **B14 partial** — document now accumulates on #46 but probe reads one command early | Guest typed-op completes on **next** command ingress (`B14-settle-semantics` §6); wasm scheduling hop |
| 33 | `catalogue-add-selects-new-object` | `added=0`; selection on dock tabs not object | cascade + settle lag | row 32 + §14 |
| 34 | `catalogue-drag-drop` | `before=1 after=1` | **B14 closed** on #46 isolate — **PASS** | Confirm in full battery post-B14 host reload |
| 35 | `duplicate-selection` | `before=1 after=1` | B14: edits land (`before=2 after=2` on isolate) but probe timing | §14 + settle completion on same turn or probe wait |
| 36 | `duplicate-reselects-clone` | selection on window tabs only | cascade | row 35 |
| 37 | `focus-selection` | camera JSON unchanged | B11 empty-doc fix on **#46** | §14 + `--only=selection-keybindings` |
| 38 | `delete-selection` | `before=1 after=1` | B14: instances accumulate (`before=4 after=4`) — mutation works, selection absent | §14 first |
| 39 | `export-only` | `download=none` | unchanged | guest `exportFixture` download path on #46; `--only=export-import` |
| 40 | `import-distinct` | `before=1 after=1` | B9 import taps on #46 unverified in full battery | `--only=export-import` on #46; read `puzzle3d.import.*` console |

---

## Top remaining user-visible blockers (ranked)

Post-#45b full battery + wave deltas B12–B14 (browser proof still uneven):

1. **Viewport pick does not publish `selectedIds` into leftover / Inspection** — blocks §6, §16, §21–§22, gumball commit, clipboard, context menu, duplicate/delete/focus, catalogue select-new. B13 fixed the Inspection cache skip **when ids exist**; #46 probe still shows empty leftover after `interactionSelect`. Root: guest leftover publication + click target (not reverting B9).

2. **Fill tab arms but produces no objects** (`fill-history-entry` `entries=0` on #45b) — user sees Fill do nothing. B14 vite-live leftover `activeToolId` + tick gate is the hop; needs `--only=fill-tab,fill-apply-max` after reload.

3. **Brush / Volume Brush utilities stay `select`** (`brush-preview-place`, `volume-brush-arm`, `engagement-brush-verb`) — B9 per-window utility publication rides **#46**; #45b confirms not shipped in that wasm.

4. **Mutation completes one command late** (B14 settle semantics) — `catalogue-add-object-kind`, `duplicate-selection`, `delete-selection` still FAIL probe `after=` reads even though document accumulates on #46. Guest typed-op scheduling (`typed_operation` vs command ingress) needs wasm rebuild.

5. **Add Object dialog unreachable** (`add-object-trigger-present`) — B11 manifest fix in tree; needs **#46** browser proof.

6. **Export download** (`export-only` `download=none`) — import half works (`import-same-file-idempotent` PASS on #45b).

7. **Projection repaint probe** (`projection-repaints-camera`) — may be timing or honest guest-pose read after B12 `data-camera-json` change; lower user pain than selection if orbit/pan/zoom PASS.

8. **Settings panel probe false negatives** — steppers work in browser (B12); users blocked only if they rely on battery green. Fix probe locators, not guest body.

9. **Window options** — **dropped from top tier**: B12 isolate PASS on #45 for all six toggles/selects + sun; full-battery rows are stale pending one post-B12 run.

10. **Locale switch** — `locale-control-present` FAIL; DE labels unproven in battery (`undo-redo` PASS shows history lane healthier than #44).

---

## What #45b proved (not in FAIL table)

Windows (§1), camera gestures + per-window + no history on #45 (§2), projection controls present/flip (§3 partial), sliders `grid-spacing`/`lod-value`, suggestions (§13), engagement input present (§14 partial), relocate arm + no hard fault (§11 partial), example switch + undo-unwind + undo-redo (§5/§20), import same-file idempotent (§24 partial), **zero hard faults** (B8 retirement holds).

---

## Suggested verification sequence

1. Vite reload on `:6013` (picks up B12 measure drafts, B13 Inspection bust, B14 settle + fill leftover without new wasm).
2. `component-release` only if guest hops needed (B13 hover-carry, B14 typed-op scheduling, B12 `WindowOption` scope blast-radius).
3. `🔍️browser-probe.ts --battery --reload-between-groups --port=6013` → expect window-option rows green, fault count 0, FAIL count well below 40.
4. If mutate group noisy: isolates in order — `selection-surfaces`, `fill-tab,fill-apply-max`, `brush-stroke`, `catalogue-panel`, `add-object-dialog`.

---

## Goal verdict

**Puzzle 3D is not end-to-end for a user.** The #45b full battery (0 hard faults, 40 FAIL / 40 PASS) proves the shell and camera baseline; the edit surface (selection, tools, fill, add-object, export) remains broken or probe-stale. B12 closes window-option toggles on re-probe; B13/B14 narrow but do not close selection or fill in browser. **#46 + post-wave full battery** is still required before any checklist row is **proven**.
