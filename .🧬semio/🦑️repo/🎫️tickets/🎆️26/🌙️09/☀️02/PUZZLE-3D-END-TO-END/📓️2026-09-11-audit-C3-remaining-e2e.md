# Audit C3 — Remaining Puzzle 3D end-to-end gaps

Ticket `26/09/02/PUZZLE-3D-END-TO-END` (moon `🌙️09/☀️02`), read-only audit 2026-09-11 ~21:00 (Composer 2.5 auditor).
**Goal is not complete.** Runtime artifacts only — not wave intent.

---

## Evidence stack (newest first)

| artifact | serve / wasm | what it proves |
|---|---|---|
| `🗑️generated/battery-2026-09-12-48-6013.txt` | **#48** `:6013` (Claude release battery) | **INCOMPLETE** — aborted ~849 s at `catalogue-drag-drop` (no summary line, no `export-import` / `undo-*` group). Mirrors `probe-2026-09-11T18-51-12.ndjson` through catalogue. |
| `🗑️generated/probe-2026-09-11T18-51-12.ndjson` | `:6014` vite **dev**, wasm materialized **~20:53** (per `📓️2026-09-11-cursor-coordination.md`) | Partial battery through catalogue (~61 verdict records). Pre-B20 quick-action chip in read group (`add-object-trigger-present` FAIL at t=115 s). |
| `📓️2026-09-11-wave-B20-add-object.md` | `:6014` isolate `probe-2026-09-11T18-59-41` (**ndjson not retained** in `🗑️generated/`) | Add Object dialog **PASS** isolate: `trigger=2`, 13 kinds, instances `1→2`. |
| `🗑️generated/probe-2026-09-11T14-48-04.md` | **#45b** `:6013` | Last **complete** full battery: `1089.2 s`, **40 PASS / 40 FAIL**, `faults (raw 0, hard 0)`. Baseline for regressions. |
| Waves B12–B22 reports | source + selective browser | See §B20 delta and wave column below. |
| `📓️2026-09-11-cursor-coordination.md` (~21:00) | — | Ground truth for ports, B20 PASS, B19/B21/B22 status, next hops. |

**Port rule:** `:6014` = vite dev (Cursor); `:6013` = release battery lane — do not conflate without noting serve profile.

---

## Status legend

| status | meaning |
|---|---|
| **proven** | Browser PASS on best post-B12 evidence (`18-51-12` / B20 isolate / #45b where no regression). |
| **source-fixed-unproven** | Fix + laws in tree; no post-fix full-battery PASS on wasm that includes the fix. |
| **broken** | Browser FAIL on best available run. |
| **unknown** | Step not reached (incomplete #48 battery) or contract/probe ambiguity. |

---

## 1. Requirement table — checklist §1–§25

| § | requirement (summary) | evidence | status |
|---|---|---|---|
| **1** | Top + Perspective windows; fixed 2-pane layout | `18-51-12.ndjson`: `window-both-present`, `one-canvas-each`, `same-document-both-views`, `distinct-camera` PASS; `#45b` same | **proven** |
| **2** | Camera orbit / pan / zoom; per-window | `18-51-12`: orbit/pan/zoom/json/per-window PASS; `camera-emits-no-artifact-history` **FAIL** (`before=0 after=6` history rows) — `#45b` had PASS (probe contract drift) | **proven** (gestures) / **unknown** (no-history row) |
| **3** | Projection pane options | Measures + control flips PASS (`18-51-12`, `#45b`); `projection-repaints-camera` FAIL both runs | **broken** (repaint probe) |
| **4** | Window options (grid, LOD, vortex, sun, select) | `#45b`: six toggles FAIL; `18-51-12` / `#48` partial: all six + sun **PASS** (`📓️wave-B12-windowconfig-roundtrip.md`) | **proven** |
| **5** | Example switcher; empty-doc `addObjectKind` | `#45b`: `example-switch` PASS; Nakagin / empty-doc not battery-proven; `#48` incomplete before replace group | **source-fixed-unproven** |
| **6** | Selection (viewport, tree, marquee, same-kind, clear) | `#48` battery L429: `selectedIds:["seed-left-001"]` on surfaces after outliner click; `18-51-12`: `inspection-object-fields` FAIL `id=null`; viewport pick path still does not land Inspection | **broken** |
| **7** | Hover highlight | No dedicated verdict; `brush-hover-storm` screenshots exist; vortex hover dispatches in suggestions step — visible hover UI not proven | **unknown** |
| **8** | Gumball / transform | `gumball-handle-enter` PASS; `gumball-scene-delta` FAIL `poseLen=266` unchanged (`18-51-12`, `#45b`) | **broken** |
| **9** | Brush utility | `brush-preview-place` FAIL `preview=null`; `engagement-brush-verb` FAIL `activeUtility=select` (`18-51-12`, `#45b`); B17 carry **vite-live**, B22 mesh queue **host #47** | **broken** |
| **10** | Volume Brush | `volume-brush-arm` FAIL `activeUtility=select`; add-target + voxel-dims FAIL (`18-51-12`, `#45b`); B17/B11 gestures in tree | **broken** |
| **11** | Relocate utility | `relocate-arm` PASS; `relocate-pose-delta` FAIL `beforeLen=afterLen=266` (`18-51-12`, `#45b`) | **broken** |
| **12** | Fill tool | `#45b`: `fill-history-entry` FAIL `entries=0`; `18-51-12`: **PASS** `entries=10` (B14); `engagement-fill-verb` FAIL `aria-pressed=false` | **proven** (planner/history) / **broken** (engagement arm read) |
| **13** | Vortex suggestions | `suggestions` PASS (`18-51-12`, `#45b`); hover-preview candidates not battery-tested | **proven** (open path) |
| **14** | Engagement bar | Input + placeholder PASS; `engagement-brush-verb` / `engagement-fill-verb` FAIL (`18-51-12`) | **broken** (verbs) / **proven** (chrome) |
| **15** | Context menu | `context-menu-selection-precondition` **PASS** on `#48`/`18-51-12` (B18); `context-menu-opens` / vocabulary / zoom-row **FAIL** `rows=0` | **broken** |
| **16** | Inspection panel | `inspection-object-fields`, `inspection-locked-flag-row` FAIL (`18-51-12`, `#45b`); B19 overlay **source+laws**, browser empty on `18-51-12` | **broken** |
| **17** | Artifact / outliner | `outliner-panel-opens` PASS; `outliner-hide-control-present` FAIL `controls=[]`; checklist §17 `value:true` toggle bug not probed | **broken** (inline hide) |
| **18** | Catalogue add + drag-drop | Rows PASS; `catalogue-add-object-kind`, drag-drop FAIL `before=after` (`18-51-12`, `#45b`); B21 source fix unbuilt in browser | **broken** |
| **19** | Settings panel steppers | `#45b`: steppers FAIL; `18-51-12`: steppers + grid bump **PASS** (B12); `settings-value-reaches-window-rail` FAIL (settings `10.5` vs rail `12.5`) | **proven** (steppers) / **broken** (rail sync) |
| **20** | History undo / redo / checkpoint | `#45b`: `undo-unwind`, `undo-redo` PASS; B19: `undo-unwind` FAIL on #47 Nakagin in full battery; `#48` incomplete | **unknown** (#48) / **source-fixed-unproven** (B21) |
| **21** | Copy / cut / paste | `clipboard` FAIL `delta=0` (`18-51-12`, `#45b`) | **broken** |
| **22** | Delete / duplicate / focus | `#45b` full battery: all FAIL `before=after`; B19 `--only=` on #47: duplicate PASS, delete still FAIL; `#48` incomplete | **broken** (full battery) |
| **23** | Add Object dialog | `#45b` / `18-51-12` read group: `add-object-trigger-present` FAIL; **B20 isolate PASS** (`📓️wave-B20-add-object.md`, `18-59-41`) | **proven** (isolate) / **broken** (full-battery read group on pre-chip run) |
| **24** | Import / export | `#45b`: `import-same-file-idempotent` PASS; `export-only` FAIL `download=none`; `import-distinct` FAIL; `#48` incomplete | **broken** (export) / **proven** (same-file import) |
| **25** | Locale DE labels | `locale-control-present` FAIL `switched=false` (`18-51-12`, `#45b`) | **broken** |

### Tool rows (explicit)

| tool | evidence | status |
|---|---|---|
| **Fill** | `fill-history-entry` PASS `entries=10` on `18-51-12`; tab arms; engagement fill verb FAIL | **proven** (planner) / **broken** (typed engagement arm) |
| **Brush** | `brush-preview-place` FAIL; utility stays `select`; B17 vite-live unproven in this wasm profile | **broken** |
| **Volume brush** | Arm + paint + dims FAIL (`18-51-12`, `#45b`) | **broken** |
| **Relocate** | Arm PASS; pose delta FAIL | **broken** |
| **Gumball** | Handle hit PASS; scene pose unchanged | **broken** |

---

## 2. FAIL-only table — still-red user-visible hops (ranked)

Evidence: `#48` partial + `18-51-12.ndjson` where available; else `#45b` `probe-2026-09-11T14-48-04.md`. Ordered by user pain × blast radius.

| rank | hop / verdict | symptom | evidence file |
|---|---|---|---|
| 1 | **Inspection empty despite selection** (`inspection-object-fields`, `inspection-locked-flag-row`, `locked-flag-row`) | User picks object; Inspection shows empty; lock chrome absent — blocks edit feedback | `18-51-12.ndjson` §6/§16; B19 source-only |
| 2 | **Brush / Volume Brush never arm** (`brush-preview-place`, `volume-brush-arm`, `engagement-brush-verb`) | Utilities stay `activeUtility=select`; no preview mesh | `18-51-12.ndjson` §9–§10, §14 |
| 3 | **Gumball drag does not move scene** (`gumball-scene-delta`) | Handle hover OK; `poseLen=266` unchanged after commit | `18-51-12.ndjson` §8 |
| 4 | **Relocate drag no pose delta** (`relocate-pose-delta`) | Arm OK; world JSON static | `18-51-12.ndjson` §11 |
| 5 | **Context menu does not open** (`context-menu-opens`, vocabulary, zoom-row) | Right-click after selection: `rows=0` | `18-51-12.ndjson` §15 |
| 6 | **Clipboard copy/paste** (`clipboard`) | `delta=0`; no copy control | `18-51-12.ndjson` §21 |
| 7 | **Catalogue add / drag** (`catalogue-add-object-kind`, `catalogue-drag-drop`) | Instance count stuck at 1; B21 fix not in browser | `18-51-12.ndjson` §18; `#45b` |
| 8 | **Delete / duplicate / focus** (full battery) | `before=after` on instance count | `#45b`; `#48` incomplete |
| 9 | **Export download** (`export-only`) | `download=none` | `#45b` |
| 10 | **Locale switch** (`locale-control-present`) | No discoverable language control | `18-51-12.ndjson` §25 |
| 11 | **Settings → window rail sync** (`settings-value-reaches-window-rail`) | Stepper bumps; measures rail value diverges | `18-51-12.ndjson` §19 |
| 12 | **Outliner inline hide** (`outliner-hide-control-present`) | No Hide/Lock controls found by probe | `18-51-12.ndjson` §17 |
| 13 | **Engagement fill verb** (`engagement-fill-verb`) | `brush`/`fill` typed verbs do not arm tool/utility | `18-51-12.ndjson` §14 |
| 14 | **Projection repaint** (`projection-repaints-camera`) | `data-camera-json` unchanged after measure nudge | `18-51-12.ndjson` §3 |
| 15 | **Mesh-upload latency** (B22 residual, not a named battery verdict) | User click can wait **~7–11 s** behind one mesh page even after back-pressure | `📓️2026-09-12-wave-B22-brush-mesh-upload.md` §5 |

**Dropped from top tier since #45b:** window-option toggles (B12 **PASS** on `18-51-12`), settings steppers (B12 **PASS**), fill planner producing history rows (B14 **PASS** `entries=10`).

---

## 3. B20 isolate vs full battery

| lane | B20 isolate (`18-59-41`, `📓️wave-B20-add-object.md`) | Full / partial battery (`18-51-12`, `#48` incomplete) |
|---|---|---|
| `add-object-trigger-present` | **PASS** `trigger=2` (Perspective quick-action chips) | **FAIL** `trigger=0` at t=115 s — read group ran **before** B20 chip + pre-20:53 wasm in that session |
| `add-object-dialog-opens` | **PASS** | not re-run in partial (read step ended at trigger) |
| `add-object-kind-options-are-dynamic` | **PASS** 13 kinds | — |
| `add-object-instance-count-increases` | **PASS** `1→2` | — |
| All other checklist rows | not in isolate scope | **Still red** — selection, tools, clipboard, context menu, catalogue mutate, export, locale, etc. |

**Conclusion:** B20 closes **only** the Add Object entry path (trigger → dialog → dynamic kinds → instance delta) on `:6014` isolate. It does **not** close catalogue add, full-battery read-group trigger, or any downstream selection/tool hop. A **post-B20 full battery on wasm including B20+B21 guest** is still required.

---

## 4. Wave ledger since C2 (B12–B22)

| wave | browser status | notes |
|---|---|---|
| **B12** WindowConfig round-trip | **proven** on `18-51-12` / `#48` partial | Closes six toggle/select FAILs from `#45b` |
| **B13** Inspection hash-bust | host proven; guest Inspection still empty | Superseded by B19 guest overlay |
| **B14** Fill planner | **proven** `fill-history-entry` on `18-51-12` | `engagement-fill-verb` still FAIL |
| **B14-settle** | isolate improvements on #47 | Full battery still FAIL catalogue/delete |
| **B17** Brush arm carry | **vite-live**; `18-51-12` still `activeUtility=select` | Needs confirm on dev reload |
| **B18** Probe hardening + outliner precondition | **proven** `context-menu-selection-precondition` | Context menu body still FAIL |
| **B19** Inspection leftover overlay | **source+laws**; browser FAIL on `18-51-12` | Rematerialize 20:07; re-probe blocked per coordination |
| **B20** Add Object chips | **proven** isolate only | Full battery read group stale |
| **B21** One-command lag | **source only**; needs **#48** wasm | Predicts catalogue/delete/undo improvements |
| **B22** Brush mesh upload | **host #47** measured; guest on **#48** | Residual ~0.64 s/page ingress |

---

## 5. #48 battery status

`battery-2026-09-12-48-6013.txt` is **incomplete** (no terminal summary; stops mid-catalogue ~849 s). Treat `#48` full-battery claims as **pending** — not evidence for goal completion.

Observed through abort (vs `#45b`):
- **New PASS:** window options (B12), settings steppers (B12), fill history (B14), context-menu precondition (B18), camera gestures (still PASS).
- **Still FAIL:** inspection, brush, volume brush, gumball, relocate, clipboard, context menu open, outliner hide, catalogue add, add-object in read group, locale, settings rail sync, projection repaint.
- **Regression to watch:** `camera-emits-no-artifact-history` FAIL on `18-51-12` vs PASS on `#45b` (contract ambiguity, not user crash).

---

## 6. Suggested verification sequence (post-#48 materialize + B21 guest)

1. Finish or restart `#48` battery on `:6013` to completion (`export-import`, `undo-*` groups).
2. Re-run `--only=selection-surfaces,brush-stroke,gumball-drag,add-object-dialog` on `:6014` dev wasm **≥20:53** with B19 rematerialized guest.
3. Compare FAIL count to `#45b` (40) — expect material drop on window options, fill history, settings steppers; selection/tools remain gating until B19+B17+B21 land in browser.

---

## Goal verdict

**Puzzle 3D is not end-to-end for a user.** Shell, camera, window options, fill planner, and Add Object (isolate) show progress; the edit surface — Inspection, brush/volume brush, gumball, relocate, context menu, clipboard, catalogue mutations, export, locale — remains broken or unproven on the newest runtime artifacts. **Do not mark the goal complete.**
