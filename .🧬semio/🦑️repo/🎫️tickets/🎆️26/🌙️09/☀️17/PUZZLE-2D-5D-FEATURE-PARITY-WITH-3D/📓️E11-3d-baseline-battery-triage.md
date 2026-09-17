# E11 — 3d baseline battery triage (2026-09-17, PASS=65 FAIL=14 FAULTS=63)

Sources read:
- Console: `.🧬semio/…/☀️17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/🗑️generated/battery-3d-baseline-1.txt`
- Probe report: `.🧬semio/…/☀️02/PUZZLE-3D-END-TO-END/🗑️generated/probe-2026-09-17T13-15-13.md` (+ `.ndjson`, screenshots — newest `probe-*` set)
- Previous best: `.🧬semio/…/☀️02/PUZZLE-3D-END-TO-END/🗑️generated/battery-2026-09-13-64-6013-B55.txt` (PASS=89 FAIL=10 FAULTS=0, Sep 13 02:01) — confirmed by grepping `PASS=` across all three `…-B55.txt` runs (62→84/13, 63→83/14, **64→89/10**).
- `📓️E6-battery-matrix.md` §3.1 (prior triage of the 89/10 run's 10 FAILs).

This run booted, ran 79 of the previous run's 99 verdicts, and ended cleanly (`done booted=true …`). `first-hard-fault-at=none`, `guest-death-faults=0` in both runs — no crash, only collateral network noise and product/latency gaps.

## Headline finding: the host reloaded/navigated at least 3 times mid-run

The battery ran against a **live** host (fleet editing `os/dev` TS while the probe drove :6013). Direct evidence of full-page reloads *during* the 28.5-minute run:

- `[42.8s] step activate-perspective: FAILED TimeoutError: click: Timeout 5000ms exceeded.` — first step, already unhealthy.
- `[106.4s] step camera-gestures: FAILED Error: evaluate: Execution context was destroyed, most likely because of a navigation` — a real browser navigation, ~70s into the run.
- A burst of **52** `requestfailed net::ERR_CONNECTION_REFUSED` against `@fs/…/🖱️ui/🧱️elements/*.tsx` (Button, Dialog, Table, Tree, Panel, Scene, …) plus `📇️catalog.json`/`🎨️ui-preferences` config imports — the fault counter jumps from 4 (at `reboot-mutate`, t=277.3s) to 57 (at `relocate`, t=927.5s), i.e. the whole burst lands inside that ~650s window (covers `selection-surfaces` → `gumball-drag`). This is Vite re-optimizing/reconnecting after an HMR full reload, not a guest bug.
- `[1587.3s] step undo-unwind: FAILED Error: evaluate: Execution context was destroyed, most likely because of a navigation` — a third navigation, immediately upstream of the `undo-redo` FAIL.
- `ensurePanel framework.panel.history STAYED-SHUT … retry-click=ok … waitedMs=20084` (line 12-15) and `ensurePanel framework.settings STAYED-SHUT … retry-click=ok … waitedMs=20379` (line 84-87) — panel-open clicks silently miss and need a 20s-later retry, consistent with the host being mid-re-render/reconnect, not steady state.

This single mechanism (c) — **stale probe expectations racing a live-edited, reloading host** — is the direct or indirect cause of the large majority of both the new FAILs and all 63 FAULTS. It is not evidence of a puzzle-3d guest regression.

## FAIL table (14)

| # | Verdict | Evidence (quoted) | vs. previous best (89/10) | Classification | Suspected owner |
|---|---|---|---|---|---|
| 1 | `locked-refusal-notice` | `locked=true flag="locked true" grabbed=false poseChanged=false notices=["",""] waitedMs=30107` | **Same FAIL in 89/10** (`grabbed=true …notices=[] waitedMs=30135`) — long-standing | (a) product defect | Lock/gesture-refusal UI never renders the on-screen notice — puzzle3d editor panel, refusal-notice component (§3.1 of E6 already flags this as real) |
| 2 | `gumball-scene-delta` | `sceneDelta=false poseLen=785 waitedMs=30491` | **Same FAIL in 89/10** (`poseLen=799 waitedMs=31007`) — long-standing, but this occurrence sits inside the 280–920s reload-burst window (t=705.6s) | (a)/(c) mixed — long-standing product gap, this run's read possibly also confounded by the reload | Gumball drag→scene-delta dispatch; re-verify on a clean (non-reloading) serve before trusting the magnitude |
| 3 | `gumball-handle-enter` | `handle=null entered=false moveEntered=false taps=0` (t=705.6s, same reload-burst window) | New (not a named verdict in the 89/10 FAIL list) | (c) stale-live-host mismatch | WebGL canvas hit-testing for the gumball handle — desynced by the module-reload burst |
| 4 | `brush-preview-place` | `instances=3 preview=null` — hover *does* update correctly through 12 polls (`hover=seed-left-001:v9`…), but the click at aim `seed-left-001:v0` produces `brush-place hop":0, "addBrushObject":0` — the click never dispatches | New | (a)/(c) — hover pipeline is healthy, only the click→place dispatch is dead; timing (t=778.5s) is just past the reload-burst tail, so a clean rerun is needed to separate "real dispatch bug" from "post-reload desync" | `puzzle3d` editor brush-place hop handler (client-side raycast→dispatch), or the `interactionHover`/`brush-place` wire if the click event never reaches it |
| 5 | `projection-measures-present` | `ids=[]` immediately after the `camera-gestures` navigation failure (`measures rail unfoldControl=0 {"measures":[],"containerPresent":false}`) at t=112.9s, 6.5s after the navigation | New | (c) stale-live-host mismatch | Read raced the reload; the measures rail hadn't rehydrated yet |
| 6 | `add-object-trigger-present` | `trigger=0` — probe queries `[id="shell-menu.action.openAddObjectDialog"]`, same selector as the 89/10 run where it found `trigger=2` | New (was PASS, `trigger=2`) | (c) primary / (b) alternate — most likely DOM not yet rehydrated post-reload; but a 2→0 (not partial) drop also fits a host ID-scheme change (the console shows a live `DuplicateSiblingKey` bug on `seed-left-001:v0..v10`, so a window-scoping fix to element IDs is plausible and would break this literal-ID query) | `.🧬semio/…/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts:3157` selector vs. whatever currently emits `shell-menu.action.openAddObjectDialog` in the shell menu (framework shell, not puzzle3d) — needs a clean-serve rerun to disambiguate |
| 7 | `locale-flips-document-labels` | `en= de=` (both empty) | New | (c) stale-live-host mismatch | Document/artifact panel body read empty both times — same STAYED-SHUT-adjacent panel-hydration issue seen for Settings/History |
| 8 | `locale-de-document-section-label` | `terminology=native expected="Objekte" stored=null control="Terminologie Nativ" de=` | New | (c) | same as #7 |
| 9 | `locale-no-english-leak` | `de=` (empty read, cascades from #7/#8) | New | (c), downstream of #7 | same as #7 |
| 10 | `clipboard-copy-writes-a-fragment` | `copyTurns=0 copyEffects=[] clipboardWriteLogLines=0 waitedMs=15151` — mod+c never reaches the guest | New | (b)/(d) — waited to the 15s budget with zero turns, i.e. not a late arrival, a non-arrival; candidate real host gap in shortcut routing, but under load-70 this could also be starvation | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8566` `handleAppKeydown` — verify mod+c is in `session.app.keybindings` / not swallowed by `isEditableTarget`/reserved-chord checks |
| 11 | `clipboard-paste-reaches-the-guest` | `pasteTurns=0 pasteEffects=[] waitedMs=15122` — "with no retained fragment the shell opens the STAGED paste form instead of executing (`🏛️ShellHost/🟦️.tsx handleAppKeydown`)" | New | (b), downstream of #10 | same file as #10 |
| 12 | `catalogue-panel-opens` | `tab=framework.panel.catalogue` present in tabs list, click did not open body (STAYED-SHUT pattern seen twice elsewhere in this run for Settings/History, both eventually recovered on retry — catalogue wasn't retried) | New | (b)/(d) — shared-host panel-open race, exact same signature as the Settings/History near-misses that DID self-heal on retry | Framework panel-tab-button click → panel-body mount path (same code as the two STAYED-SHUT recoveries above); probe does not retry this one specifically |
| 13 | `undo-redo` | `example=Concrete Forest` (bare) — immediately downstream of `[1587.3s] step undo-unwind: FAILED … navigation` | New (PASS in 89/10) | (c), direct causal chain | Third mid-run reload; document/history state was mid-reload when undo-redo ran |
| 14 | `battery-faults` (aggregate) | `raw=63 hard=0 collateral=63 distinct=3` | New (FAULTS=0 in 89/10) | (c) — all 63 are the reload/dep-reoptimization noise below | Dev-serve churn from concurrent editing, not puzzle3d |

**Not a regression** (long-standing, both runs): #1, #2.
**Real product-suspect** (worth a clean-serve confirmation, not clearly reload noise): #4 (brush click dispatch), #10/#11 (clipboard), #12 (catalogue panel open).
**Reload/latency artifacts** (re-run on a quiescent host before spending fix effort): #3, #5, #6, #7, #8, #9, #13, #14.

**8 fixes since the 89/10 baseline**, confirmed PASS in this run: `relocate-pose-delta`, `outliner-show-restores`, `example-switch`(+`example-switch-instances`), `export-only`, `export-names-the-example`, `import-same-file-idempotent`, `import-distinct`, `import-distinct-records-history`. The live-editing fleet has been net-positive on the export/import/relocate/outliner cluster E6 flagged as real product gaps.

## FAULTS (63 raw, 0 hard, 63 collateral, distinct=3)

All three groups are benign in the sense that `battery-hard-faults PASS` and `guest-death-faults=0` — none crashed the guest or hard-faulted. None matches the task's example `[stale] … unactivated` banner text (that banner exists in this run's plugin-descriptor dump for `animate`/`architect`, lines ~1752 of the probe `.md`, but the probe does not classify it as a "fault" at all — it's unrelated log noise, not one of the 3 groups below).

| Group | Message (verbatim) | Count | Benign banner or real? |
|---|---|---|---|
| A | `http 404: http://127.0.0.1:6013/🧩️extension-modules/watch` | ~6 | Benign — known dev-serve endpoint 404 (extension hot-reload watch channel not wired for this plugin), recurs at every reload/navigation boundary (boot, projection-options, undo-redo) |
| B | `requestfailed net::ERR_ABORTED: http://127.0.0.1:6013/🧩️extension-modules/watch` | ~2 | Benign — same endpoint, request aborted by the same reload events as group A |
| C | `requestfailed net::ERR_CONNECTION_REFUSED: http://127.0.0.1:6013/@fs/…` (52 distinct framework `🖱️ui/🧱️elements/*.tsx` + config/catalog module paths) | ~52 | **Real signal, but of dev-serve churn, not a guest bug** — this is Vite's whole UI-element module graph failing to fetch in one contiguous burst (t≈277–927s), i.e. the dev server was mid-restart/re-optimize while the live-editing fleet touched a shared framework file. It is the network fingerprint of the "host reloaded" finding above, not a puzzle3d fault. |

(Printed collateral list enumerates 60 lines against a header-reported `collateral=63`; the 3-item gap is inside the probe's own running counter — e.g. the `boot`-time fault reflected in the counter but not re-printed verbatim in the list — and is immaterial to the grouping above.)

## Why 79 verdicts instead of 99 (20 fewer)

Every one of the 20 verdicts present in the 89/10 run but absent here is explained by an upstream **step-level** failure or **gating verdict** FAIL in *this* run — nothing silently vanished:

| Missing verdicts | Cause |
|---|---|
| `camera-pan`, `camera-zoom`, `camera-per-window`, `camera-lane-responsive`, `camera-emits-no-artifact-history`, `projection-control-flips`, `projection-repaints-camera` (7) | `[106.4s] step camera-gestures: FAILED Error: evaluate: Execution context was destroyed, most likely because of a navigation` — the whole step aborted, so its verdicts never ran |
| `suggestions` (1) | `[836.8s] step suggestions-open: FAILED TimeoutError: boundingBox: Timeout 30000ms exceeded.` |
| `volume-brush-arm`, `volume-brush-add-target-volume`, `volume-brush-target-volume-attribute`, `volume-brush-voxel-dims` (4) | `[880.2s] step volume-brush: FAILED TimeoutError: boundingBox: Timeout 30000ms exceeded.` |
| `undo-unwind` (1) | `[1587.3s] step undo-unwind: FAILED Error: evaluate: Execution context was destroyed, most likely because of a navigation` |
| `catalogue-add-object-kind`, `catalogue-add-selects-new-object`, `catalogue-drag-drop`, `catalogue-kind-rows-present` (4) | gated on `catalogue-panel-opens` FAIL — panel body never opened, so the downstream catalogue checks were skipped |
| `add-object-dialog-opens`, `add-object-instance-count-increases`, `add-object-kind-options-are-dynamic` (3) | gated on `add-object-trigger-present` FAIL (`trigger=0`) — probe code (`🔍️browser-probe.ts:3160-3163`) explicitly returns early when `found===0` |

7+1+4+1+4+3 = 20, matching exactly. Four additional steps failed outright (`activate-perspective`, `pick-object`, `context-menu`, `marquee-click`, `fill-tab`) but happened not to gate any named verdict, so they cost time/evidence quality (and are themselves further symptoms of a slow/reloading host under load ~70) without reducing the verdict count.

## Prioritized fix list

1. **Re-run the battery against a quiescent host** (no concurrent TS edits, no reload) before triaging any further — items #3–#9, #13, #14 and the `battery-faults` FAULTS group are very likely to clear or change shape on a stable serve; spending guest-code effort on them now risks chasing reload artifacts. Owner: whoever runs the next baseline; mechanism: same `🔍️browser-probe.ts` battery, but confirm `git log -1` on the touched `🖱️ui/🧱️elements/*` framework files shows no commits during the run window.
2. **`locked-refusal-notice`** (#1) — real, long-standing. Owner: puzzle3d editor's locked-object gesture-refusal UI (find where `notices` is populated for a grabbed-but-locked drag; currently stays `["",""]`). Fix: render the refusal notice text when `locked=true && grabbed=true/false`.
3. **`gumball-scene-delta`** (#2) — real, long-standing, confirm on clean serve. Owner: gumball drag→scene mutation dispatch (`poseLen` never changes after a drag). Fix: trace why the gumball's pointer-move handler doesn't emit a pose delta for this fixture.
4. **`brush-preview-place`** (#4) — hover pipeline is healthy (12 successful hover polls with correct `hoveredVortexFullId`), only the click→place hop is dead (`brush-place hop:0`, `addBrushObject:0`). Owner: puzzle3d brush-place click handler. Fix: instrument why a canvas click at a hit-tested vortex position doesn't emit the `brush-place`/`addBrushObject` hop, independent of hover state.
5. **`clipboard-copy-writes-a-fragment` / `clipboard-paste-reaches-the-guest`** (#10/#11) — `copyTurns=0`/`pasteTurns=0` at the full 15s budget is a non-arrival, not a late one. Owner: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8566` (`handleAppKeydown`). Fix: verify mod+c/mod+v are registered in `session.app.keybindings` for puzzle3d and aren't being eaten by `isEditableTarget`/`reservedChords` before reaching the app dispatch; re-run in isolation (not inside the full battery) to rule out load.
6. **`catalogue-panel-opens`** (#12) — same STAYED-SHUT signature Settings/History both hit and self-healed on retry; catalogue wasn't retried by the probe. Owner: framework panel-tab-button click → panel-body mount path (shared with Settings/History). Fix candidates: either the host has a real race between tab-activate and body-mount under load, or the probe should adopt the same retry-on-STAYED-SHUT pattern it already uses for Settings/History for catalogue too — file both: host race in the ShellHost panel-dock code, and a probe hardening follow-up in `browser-probe.ts`.
7. **`add-object-trigger-present`** (#6) — disambiguate reload-timing vs. ID-scheme drift. Owner: cross-check whether `shell-menu.action.openAddObjectDialog` is still emitted as a literal (non-window-scoped) ID; if the DuplicateSiblingKey fix (see console tail: `seed-left-001:v0..v10` "two children with the same key" spam) is mid-flight and moving IDs to a window-scoped form, `browser-probe.ts:3157`'s literal-ID query needs updating to match.
8. **DuplicateSiblingKey console spam** (`seed-left-001:v0`…`v10`, "two children with the same key") — not counted in FAULTS (console error, not a network fault) but recurs dozens of times in the console tail and correlates with the vortex/brush IDs implicated in #4/#3. Worth a dedicated look even though outside this run's FAIL/FAULT ledger — likely the outliner/vortex-list rendering the same vortex id twice per re-render (owner: puzzle3d outliner or the vortex-list host bridge).
