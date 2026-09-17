# 📓️ Wave 2G — the ◻️2d Playwright battery expanded to the full 3d matrix, plus the E7 probe fixes

Slice 2G. One file changed: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🔍️browser-probe.ts`
— **888 → 2 682 lines, 23 → 48 registered steps, 169 `verdict(…)` calls**. No product source was touched;
every product finding below is a hand-off, not a fix.

Paths are relative to `/Users/ueli/Documents/semio`. The probe is `PROBE2`; the 3d reference battery is
`.🧬semio/…/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts` (`PROBE3`).

---

## 0. TL;DR

- The three probe bugs E7 named are fixed and **two of them are verified green on :6012**:
  `24-import/import-round-trip` PASSes (`chooser:true`, 85 s) and `16-inspection/inspector-fresh-on-open`
  PASSes, both for the first time. `FAULTS=0` on every run since the fault-regex fix (was 2 every run).
- Every MISSING row of `📓️E6-battery-matrix.md` §2 now has a lane, plus the 2d-specific ones. Conventions
  kept: `--battery`, `--only=`, `--port=`, **new `--reload-between-groups`**, read → mutate → replace
  grouping with per-group `guest-alive-<group>` checkpoints, 30 s mutation polls with `waitedMs` on every
  verdict, per-step screenshots, refused-fixture dumps, and the exact summary line
  `battery PASS=n FAIL=n FAULTS=n HARD=n first-hard-fault-at=… guest-death-faults=n`.
- Last dry-run (`--only=windows,two-window-independence,history-panel,settings-steppers,context-menu-rows,duplicate,focus-selection`):
  **PASS=26 FAIL=1 FAULTS=0 HARD=0**. The one red is a real finding (`framework.history.revert` does not exist).
- `bunx tsc --noEmit --strict` clean (only `import.meta.dir`, a bun-ism).

---

## 1. The probe bugs (E7 §3, §4, §5)

### 1.1 `setActions()` is window-scoped and waits for the reloaded chrome — E7 §3, **verified fixed**

`actionsOpen()` queried `[id="action.undo"], [id="puzzle2d-engagement"]` **page-wide**. All three panes are
mounted and all three author the *same* ids (`puzzle2d_engagement` hands every pane
`id: "puzzle2d-engagement"`, `EDITOR2/🎭️modes/✏️edit/🦀️.rs`), so a Detail pane whose Actions pane happened to
be open answered "open" for the Overview one, the toggle click was skipped, and `import` raced a file chooser
against a row that was never in the DOM.

- `actionsOpen(windowId)` scopes to `[data-slot="window"][id="<windowId>"]`, matching what the framework's own
  key router already does (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` `shouldRouteKeysToWindowSearch`).
- NEW `actionsToggleReady(windowId)` polls the pane's own `framework.window.<Camel>.engagement.toggle` for
  present + visible + enabled before it is pressed (a `selectExample()` reload re-renders the pane and the
  toggle is transiently absent). `selectExample()` now awaits it itself, so every caller benefits.
- The click outcome is logged, never swallowed; three attempts, each reported with `waitedMs`.

Measured: `setActions 2d-overview want=true attempt=0 click=ok open=true waitedMs=11` →
`PASS 24-import/import-round-trip {"chooser":true,"before":1,"after":180,"edges":179,"parsed":"true","waitedMs":85558}`.

### 1.2 `FAULT_RE` no longer matches the dev-staleness banner — E7 §5, **verified fixed**

The `puzzle2d-` alternative is now `puzzle2d-(?!react-dev)` (it must keep matching the app's own
`Fault::from("puzzle2d-…")` codes), and a new `BENIGN_CONSOLE_RE`
(`contributions document sources|staged plugin module|\[stale\]|activate-puzzle2d-react-dev|transform freshness`)
gates `noteFault`. Every run since: `FAULTS=0` with the 59-line staleness banner still on the console.

### 1.3 `inspector-fresh-on-open` asks the BOARD first — E7 §4, **verified fixed**

Two new verdicts frame the inspector question, and the inspector question is not asked when they fail:

- `1-windows/board-painted-before-inspection` and `1-windows/board-stays-painted-through-the-step`, both read
  off `data-board-fixture-parsed` + a non-zero node count (`boardPainted()`).
- The retry click **re-queries** `nodeScreen(second)` instead of reusing a stale coordinate.

Wave 2A's `parse_fixture_json` fix (commit-after-admission) is what makes these green; the lane now says so
by name instead of reporting a blank board as an inspector defect.

### 1.4 Four more probe defects found while dry-running (all fixed)

| defect | symptom | fix |
|---|---|---|
| a `null` reading scored as "the value moved" | `published:"false"` → `null` (control left the DOM on an HMR reload) passed `option-…-changes` | `nudgeControl` requires `after !== null`, and scores on `data-published-value` when the control has one (the rail's rendered text is an optimistic draft) |
| collapsed groups read as missing controls | `puzzle2d-play-grid-snap`/`-grid-factor` absent — the GRID group is authored `default_open: Some(false)` and its children are not in the document at all | `unfoldMeasures` expands every rail group by its id-less `tree-gutter` chevron, each label once (the chevron is a toggle) |
| an unbounded `>` census scored a document swap as a placement | `suggestion-accept-places-a-node` PASSed `before=1 after=180` (an Enter fell through to the example picker) | every single-entity lane asserts `=== before + 1` |
| the shell reloading mid-battery | a dev serve under concurrent framework edits answers HMR with a full reload; every later verdict measured a blank document | the step loop detects `windows<3 \|\| canvases<3`, waits the boot out, and records `shell-remounted-before-<step>` as its own verdict |

---

## 2. Lane table — what each lane asserts, the hooks it needs, who owns it, dry-run verdict

`owner` is the slice the lane hands a red to. **Dry-run column:** `PASS`/`FAIL` = measured on :6012 today;
`FAIL(stale)` = the verb exists in source but the dev serve runs the previously staged plugin wasm, so the
red says nothing about the verb; `not run` = not dry-run in this session.

### 2.1 Lanes that measure chrome/product already live

| lane (step) | asserts | DOM hooks / verbs | owner | dry-run |
|---|---|---|---|---|
| `window-options` | grid-snap, grid-factor, per-pane LOD present **and** their *published* value moves; grid-visible + selectable-kind absent | `#puzzle2d-play-grid-snap`, `#puzzle2d-play-grid-factor`, `#2d-overview-lod` (DOM id `<windowId>/<authored>`), `data-published-value` | 2C (the two missing) | 3 PASS, 2 FAIL (missing by design today) |
| `two-window-independence` | wheel on Overview moves only its camera; all three panes publish one census; every board surface names its window instance | `data-board-camera-json`, `data-board-nodes/edges/handles`, `data-window-instance-id` | — | **3 PASS** |
| `history-panel` | the ledger panel opens with rows; undo/redo/checkpoint/revert controls exist | `framework.panel.history`, `framework.history.{entry.N,undo,redo,checkpoint,revert}` — sections `framework.history.{commands,actions}` render **shut**, so they are expanded first | framework | 4 PASS, 1 FAIL (`revert` does not exist; the panel offers Undo/Redo/Checkpoint/**Check In**) |
| `settings-steppers` | the app settings panel opens; the three authored steppers exist by id; one bumps | `puzzle2d.panel.settings`, `[id$="/puzzle2d-play-settings.{fill-count,suggestion-offset,grid-factor}.control"]`, `[data-slot="stepper-plus"]` (press-and-hold, never a synthesized click) | — | **5 PASS** |
| `context-menu-rows` | the full node vocabulary; the hide row alternates its label; an empty selection offers Select All | `[role="menuitem"]#{toggleHidden,toggleLocked,duplicate,focusSelection,selectSameKind,deleteSelection,selectAll}`, submenu group `menu.group.selection` | — | **3 PASS** |
| `duplicate` | `mod+d` adds exactly one node and the CLONE is re-selected | `mod+d → duplicateSelection`, `data-board-selection-json` | — | **2 PASS** |
| `focus-selection` | the context-menu row exists and moves the camera (after orbiting away first) | `[role="menuitem"]#focusSelection` | — | **2 PASS** |
| `selection-keybindings` | click selects; `mod+a` selects all; Escape clears; a click-marquee on empty canvas selects nothing | `mod+a → selectAll` | 2C (`mod+a` is unbound: only `delete,backspace` and `mod+d` are) | 3 PASS, 1 FAIL |
| `outliner-rows` | rows exist; rows carry Hide/Lock actions; hide applies; show restores | `[id^="panel:puzzle2d-play-document/"] [data-slot^="tree-action"]` | 2C | 1 PASS, 3 FAIL(stale) — `flag_row_action` is in source |
| `locked-refusal` | lock row → inspector flag reads `true` → drag refused → refusal notice → delete refused | `[role="menuitem"]#toggleLocked`, `[id$="puzzle2d-play-inspector.node.locked"]`, `data-board-positions-json` | 2C/2E | 3 PASS, 2 FAIL — **`deleteSelection` ignores the lock** (12 entities → 0) and no refusal notice renders |
| `engagement-grammar` | placeholder has no dead verbs; `move 50 25`, `rotate 45`, `scale 1.5` (on a ≥2-node marquee — the centroid verbs are no-ops on one node), `connect <h> <h>`, `fill 12` arms the tool AND sets the count, repeat-last | `#puzzle2d-engagement` (window-scoped), `typed` reported at Enter | 2A (line), 2C (repeat-last), 2D (connect) | 6 PASS, 3 FAIL |
| `fill-controls` | Start present; run reports progress; Pause/Step/Abort/Finalize present **once the run is live**; pause halts; step advances one placement; count change mid-run; abort ends and retracts | tool-run panel buttons by label, `#puzzle2d-fill-count` | framework tool-run | 6 PASS, 3 FAIL (`Step` adds nothing; `Abort` from Paused does not end the run) |
| `fill-weights` | both distribution groups present; per-kind sliders present; zeroing one kind changes the placed mix (both runs must share one baseline) | `[id*="puzzle2d-play-suggestion-distribution"]`, `[id*="puzzle2d-play-node-kind-"]` — inside the **Tool** panel, both groups authored shut | — | **3 PASS** |
| `fill-history` | fill places nodes; the whole run is **one** artifact ledger row; one undo restores the pre-fill document | `framework.history.entry.N` filtered by `CHROME_HISTORY_ROW` | 2A | **3 PASS** (`added:["Fill↶"]`) |
| `history-controls` | checkpoint usable; an edit after it lands; revert restores; undo changes; redo restores | `framework.history.{checkpoint,revert,undo,redo}` | framework | 4 PASS, 1 FAIL (no `revert`) |
| `hover` | local paint, guest echo, echo clears on exit, outliner-row hover paints the canvas | `data-board-hover-paint-id` (host), `data-board-hovered-id` (guest echo) | 2B | FAIL(stale) — 2B's report documents both hops as landed |
| `brush-place` | brush arms; a slot opens on a rim handle (hover *and* click attempted, `openedVia` named); `tab` / `shift+tab` cycle; the click places exactly one node | `[data-slot="toggle-group-item"]#brush`, `#puzzle2d-brush-placement`, `tab → cycleBrushCandidate`, `shift+tab → cycleBrushCandidateBack` | 2B | 1 PASS, 4 FAIL(stale) |
| `suggestions-menu` | the `suggestNodes` row on a free **handle**; the popup publishes its candidate page; hover previews without committing; accept places exactly one node and re-selects it; Escape closes | `[role="menuitem"]#suggestNodes` → `openHandleSuggestions`, `data-board-suggestion-menu-json`, `hoverSuggestion`, `acceptSuggestion` | 2B | FAIL(stale) |
| `clipboard` | precondition selection; `mod+c`+`mod+v` adds one node; `mod+x` removes the selection; the context menu carries copy/cut/paste | reserved `copy`/`cut`/`paste` hotkeys (the probe grants clipboard permissions), `[role="menuitem"]#{copy,cut,paste}` | 2D | 1 PASS, 3 FAIL |
| `create-edge` | an `createEdge` action row exists in the pane's Actions panel and adds one edge | `#action.createEdge` inside `[data-slot="window"][id="2d-overview"]` | 2D | 2 FAIL |
| `proximity-connect` | dropping a dragged node inside the proximity radius forms an edge; the radius setting exists | `applyBoardEvents{nodeDragEnd}` auto-attach, `[id$="/puzzle2d-play-settings.proximity-radius.control"]` | 2D | 2 FAIL |
| `target-regions` | the area brush arms; a drag paints a region; a fill stays inside it | `[data-slot="toggle-group-item"]#areaBrush`, `data-board-target-regions-json` | 2F | 3 FAIL |
| `rotate-gumball` | the board publishes a transform; a tangential drag on the handle rotates the selection without translating it | `data-board-transform-json` | 2E | 2 FAIL |
| `add-node-dialog` | a trigger exists; the dialog opens; its kind list is LIVE (not 3d's hardcoded `"Object"`); submitting adds one node | `#shell-menu.action.openAddNodeDialog`, `[data-slot="dialog-content"]` | 2C | 4 FAIL |

### 2.2 Pre-existing lanes kept and hardened

`example-inventory`, `windows`, `example-concrete-forest`, `example-nakagin`, `panels`, `camera-wheel`,
`click-select` (now loads a document and frames the board first), `marquee`, `drag-node`, `utilities`,
`fill`, `delete`, `undo` (its verdict renamed `undo-action-row-changes-document` — the History panel is a
second, independent route measured by `history-controls`, and two verdicts sharing a name cannot be joined
across runs), `context-menu`, `catalogue-add`, `select-same-kind`, `engagement-move`, `export`, `import`,
`settings`, `locale`, `explore-chrome`, `inspector-timing`, `guest-alive`.

---

## 3. Reconciliation table — selectors this probe predicts for slices whose report has not landed

2A and 2B have reported and their ids are already wired in. For 2C/2D/2E/2F the lanes are written from the
master plan's verb names and the 3d spelling; **if a slice ships a different id, only the string in the
`expectedSelector`/`expectedKeybinding`/`expectedVerb` field of the named verdict has to change** — every
one of them is printed in the verdict's own JSON, so a red names the string it looked for.

| slice | lane / verdict | predicted hook | 3d counterpart |
|---|---|---|---|
| 2C | `window-options/option-puzzle2d-play-grid-visible-*` | `#puzzle2d-play-grid-visible` | `puzzle3d-play-grid-visible` / `setGridVisible` |
| 2C | `window-options/option-puzzle2d-play-selectable-kind-*` | `#puzzle2d-play-selectable-kind` | `setSelectableKind` |
| 2C | `outliner-rows/outliner-rows-carry-hide-lock-controls` | `[id^="panel:puzzle2d-play-document/"] [data-slot^="tree-action"]`, row text `Hide`/`Show`, `Lock`/`Unlock` | 3d's outliner row actions |
| 2C | `add-node-dialog/*` | `#shell-menu.action.openAddNodeDialog`, dialog `[data-slot="dialog-content"]`, kind `[data-slot="select-trigger"]`, submit `#ui.dialog.submit` | `openAddObjectDialog`, `#objectKind` |
| 2C | `engagement-grammar/engagement-repeat-last` | `WindowEngagementInput.on_repeat_last` + `engagementRepeatLast`; driven as ArrowUp + Enter | `engagementRepeatLast` |
| 2C | `selection-keybindings/select-all-keybinding` | `mod+a → selectAll` | — |
| 2C/2E | `locked-refusal/locked-refusal-notice` | a transient notice matching `/lock\|gesperrt/` in `[data-slot="notice"], [role="status"], [role="alert"]` | `locked-refusal-notice` (red in 3d too) |
| 2D | `clipboard/*` | reserved `mod+c` / `mod+x` / `mod+v`; context rows `#copy`, `#cut`, `#paste` | `Puzzle3dClipboardJob` |
| 2D | `create-edge/create-edge-action-present` | `#action.createEdge` | `createAttraction` |
| 2D | `proximity-connect/proximity-radius-setting-present` | `[id$="/puzzle2d-play-settings.proximity-radius.control"]` | `setProximityRadius` |
| 2E | `rotate-gumball/*` | `data-board-transform-json` on the board surface | `data-instances-json` pose delta + `#move` gumball |
| 2F | `target-regions/area-brush-arms` | `[data-slot="toggle-group-item"]#areaBrush` | `volumeBrush` |
| 2F | `target-regions/area-brush-paints-a-region` | `data-board-target-regions-json`, rows `{x,y,width,height}` | `data-target-volumes-json` |

Already reconciled against a landed report: 2B's `suggestNodes` / `openHandleSuggestions` /
`hoverSuggestion` / `acceptSuggestion` / `closeHandleSuggestions`, `cycleBrushCandidate` +
`cycleBrushCandidateBack` on `tab`/`shift+tab`, `data-board-suggestion-menu-json`,
`data-board-hover-paint-id`. 2A's verbatim engagement line is measured by printing `typed` at Enter.

---

## 4. Product findings this probe surfaced (hand-offs, none fixed here)

1. **`deleteSelection` ignores the `locked` flag** — `8-locked/locked-node-refuses-delete`, measured
   `before=12 after=0` on a node whose inspector flag read `locked true` and whose *drag* was correctly
   refused. `puzzle2d_transform_selection` honours the lock; `🎮️commands/🗑️delete-selection/🦀️.rs` does not.
   → slice 2C.
2. **No locked-refusal notice** — the guest refuses the gesture silently. Mirrors 3d's own red
   (`📓️E6` §3.1), so it is a shared gap, not a 2d regression. → 2C/2E.
3. **`framework.history.revert` does not exist.** The History panel's COMMANDS section offers
   `undo`, `redo`, `checkpoint` and `checkin` ("Check In (1)", the VCS publish row — *not* a revert).
   "Roll the document back to the last checkpoint" has no control in the shell. → framework.
4. **Fill `Step` advances no placement** and **`Abort` from a Paused run does not end it** —
   `12-fill/fill-step-advances-one-placement` (`atPause=29 after=29 waitedMs=30131`) and
   `12-fill/fill-abort-ends-the-run` (`waitedMs=30380`, still `Paused · Searching an open handle (2/5)`).
   Both are framework `ToolRun` transitions 2d inherits. → framework / 2A.
5. **`fill <n>` does not retarget the count in the served build** —
   `14-engagement/engagement-fill-argument-sets-the-count` reads `100` after `fill 12` while the tool arms
   correctly. The arm that sets `ctx.scene.runtime.fill_count` is present in current source, so this is
   most likely staleness; re-measure after activation.
6. **`undo` via the pane's Actions row changes nothing** (`undo-action-row-changes-document`) while the
   same document undoes/redoes correctly through the History panel (`history-controls`, both green). Two
   routes, one working — worth a look. → framework.
7. **Handle ids never reach the DOM.** `board2dVitals` publishes node positions only, so a bare `connect`
   (whose precondition is *exactly two selected handles*) cannot have its precondition established from the
   browser. Handle ids DO appear in `data-board-selection-json` (`<nodeId>:link`), which is what the lane
   uses for the explicit two-operand form — but there is no way to aim a pointer at a named handle.
   → slice 2E, as a probe-observability item.

---

## 5. Commands run

| command | verdict |
|---|---|
| `bunx tsc --noEmit --strict --skipLibCheck --lib esnext,dom 🔍️browser-probe.ts` | clean (only `ImportMeta.dir`, a bun-ism, on every run) |
| `bun 🔍️browser-probe.ts --only=windows,two-window-independence,history-panel,settings-steppers,context-menu-rows,duplicate,focus-selection --port=6012` | **PASS=26 FAIL=1 FAULTS=0 HARD=0** (final run) |
| earlier dry-runs, `--only=` per lane family (window-options, hover, brush-place, suggestions-menu, clipboard, engagement-grammar, create-edge, proximity-connect, target-regions, rotate-gumball, locked-refusal, outliner-rows, fill-controls, fill-weights, fill-history, history-controls, add-node-dialog, export/import/inspector-timing) | every lane executed; no probe exception, no selector typo on existing chrome; `FAULTS=0` on all of them |

No `nx`, no server was started or recycled, no cargo (this slice owns no Rust). The scratch DOM-dump script
and its output are in `TICKET/🗑️generated/2G/` (`dump2d.ts`, `dump2d.txt`) — it is what established the
measures-rail and History-panel shapes.

---

## 6. Not verified / owed to integration

- **The dev serve runs the previously staged plugin wasm** (it prints "59 staged plugin module(s) are behind
  their source" at every boot). Framework/React changes arrive live over HMR — which is why 2A's verbatim
  engagement line measures green — but every guest-side verb landed today (2B's suggestions family, 2C's
  outliner row actions, 2D's `connect`/`create_edge`) is invisible until the coordinator re-activates
  `framework-os-dev:activate-puzzle2d-react-dev`. Every such red is tagged `FAIL(stale)` in §2 and must be
  re-measured after activation before it is read as a defect.
- **No full `--battery` run.** The lanes were exercised in `--only=` families; the group ordering,
  `STEP_LEADS_ITS_GROUP` / `STEP_TRAILS_ITS_GROUP` and the `guest-alive-<group>` boundaries were exercised in
  every run, but a single end-to-end battery (≈40–60 min with three fill runs) has not been taken.
- **`--reload-between-groups` is implemented and type-checked but never driven** — no run used more than one
  group boundary with the flag on.
- **`fill-weights/fill-weights-change-the-distribution`** compares kind histograms derived from outliner row
  labels. It now refuses to score unless both runs started from the same baseline census, but the histogram
  itself is a coarse instrument; treat a green as a smoke test, not as a distribution proof.
- **The serve reloads under the battery** when peers edit framework files. The step loop absorbs this
  (`shell-remounted-before-<step>`), but a reload *inside* a step still costs that step its verdicts; two of
  my dry-runs lost a lane that way. A battery for the record should be taken when the tree is quiet.
- Lanes not dry-run at all after their last edit: `suggestions-menu` (re-aimed at rim handles), `hover`
  (split into local paint + guest echo), `brush-place` (three rim radii), and the bounded-`+1` census
  predicates on `add-node-dialog`/`clipboard`/`duplicate`/`create-edge`/`proximity-connect`. They
  type-check; their gestures are the same ones the earlier runs drove.
