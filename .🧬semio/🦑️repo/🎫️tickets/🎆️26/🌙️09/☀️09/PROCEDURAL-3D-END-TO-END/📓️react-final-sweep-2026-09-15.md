# 🧹 React final sweep — generation3d, 2026-09-15 (lane `react-final-sweep`, React :6023)

Closes the seven leftovers the React battery carried into its last green run: the three red rows
(`menus · export-formats`, `status-states` ×4, `panel-i18n` ×4), the three open defects named by
`📓️role-switch-regression-2026-09-14.md` §6 and `📓️boot-camera-framing-2026-09-15.md` §6, the guest
teardown panic `📓️react-remaining-reds-2026-09-15.md` §12 left undiagnosed, and the undo/redo chords
that had never been pressed on a history with anything in it.

**That last one uncovered a further defect and it is not fully closed**: pressing `mod+z` after a
slider edit rewinds the DOCUMENT and leaves the 3d preview showing the old geometry (§7.1). The rule
that fixes it landed and the redo half is proven; the undo half is not, and its row is left RED rather
than softened.

Guest restaged at **10:49**, **11:47** and **11:54** with this lane's Rust
(`🗑️generated/react-sweep/restage.txt`, `restage-2.txt`, `restage-3.txt`;
`semio_s_plugin_procedural_component.core.wasm` 11:54, 74.5 MB).
Host TS is served live off :6023 with `SEMIO_VITE_HMR=0`; every module this lane changed was `touch`ed
and re-fetched through `/@fs/…` before any verdict was taken.

---

## 1. `menus · export-formats` — the submenu was CLIPPED AWAY, and the menu never moved focus

### Root cause, measured

`🐍️submenu-reach-recon.mjs` (new) opens the flow graph's context menu on :6023, hovers
`menu.group.transfer`, and reads the export row's geometry, its ancestors' computed overflow and what
`elementsFromPoint` answers at the row's own centre:

```
row  exportDocument  rect {x:703, y:874, w:171, h:16}   visibility: visible
hit at its own centre  DIV.absolute inset-0 z-30   rect {x:6, y:61, w:1075, h:907}   ← the node-graph canvas
ancestors:
  DIV z=1   pos=relative  ox=visible oy=visible  rect {700,836,177,54}   (submenu chrome body)
  DIV z=50  pos=relative  ox=auto    oy=auto     rect {700,836,177,54}   (submenu ContextMenuChrome)
  DIV       pos=absolute                          rect {700,836,177,54}   .absolute.ms-tiny.start-full
  DIV       pos=relative                          rect {546,820,154,16}   (the group row)
  …
  DIV z=50  pos=fixed    ox=auto    oy=auto      rect {543,782,160,86}   ← the ROOT ContextMenuChrome
```

The root menu's chrome is `max-h-layout-command overflow-y-auto` (`ContextMenuChrome`,
`🎯️targets/⚛️react/🟦️.tsx`). CSS resolves an `overflow-x: visible` beside a non-visible `overflow-y`
to `auto`, so the chrome is a **scrollport** in both axes — and the submenu panel, a `position:absolute`
child anchored at `start-full` (x 700…877) of a box that ends at x 703, is clipped away to a 3 px
sliver. `getBoundingClientRect` still reports the full rect, which is why Playwright resolved the
element and then timed out on it: the actionability hit test at the row's centre answers the node-graph
canvas underneath. **The row was not merely awkward to reach; it was not on screen.**

The keyboard half was a second, independent cut. The arrow walk itself was already right —

```
after open        activeRow=null                 activeElement=""
after ArrowDown   activeRow=reorganize           activeElement=""
after ArrowDown   activeRow=menu.group.transfer  activeElement=""
after ArrowRight  activeRow=importDocumentRequest activeElement=""
after ArrowDown   activeRow=exportDocument       activeElement=""
```

— but the active row lived in React state only. `document.activeElement` stayed `<body>` for the whole
walk, so a screen-reader user was told nothing at all, and any focus-based consumer (this probe among
them) could not see the walk happen.

### Fix — one owning layer, `🖱️ContextMenu/🟦️.tsx`

1. **A submenu is a floating surface, not a child of the scrollport.** `ContextMenuSubmenuRow` now
   measures its own anchor row and portals its panel into `useShellFloatingSurfaceHost()` at viewport
   coordinates, exactly as the root controller already does. The placement is a pure rule,
   `contextMenuSubmenuPlacement({anchor, panel, viewport, gap})` — beside the anchor, flipped to the
   start side when the end side would overflow, top clamped into view.
2. **A submenu carries its own `role="menu"`.** It had none: nested inside the root's menu element it
   inherited reachability by accident, and a portaled panel without it would have been an *outside*
   pointer target — `isContextMenuPointerTarget` closes the menu on a pointerdown it does not
   recognise, so the first click would have dismissed the menu instead of dispatching.
3. **The active row takes real DOM focus.** Every row hands its element to the controller under
   `contextMenuPathKey(path)`; a layout effect focuses the active row (the menu element itself when no
   row is active), and the previously focused element is restored on close. Rows carry `tabIndex={-1}`
   and a group row now announces `aria-haspopup="menu"` beside its `aria-expanded`.
4. **A menu holds focus, so an action over the document's text selection must have CAPTURED it.**
   Taking focus collapses the DOM text selection, and `TextSelectionContextMenuHost`'s `Copy`/`Cut`
   read that selection when they run — so the focus move silently emptied the clipboard. The host now
   reads the selection when the menu OPENS and hands it to the action
   (`copyDomTextSelection(captured?)`, `cutDomTextSelection(target, captured?)`). This was found by the
   pre-existing in-source law, which went red on the focus change and is green again.

### Proof

`menus` on :6023 after the fix:

```
export-formats  reached=true  keyboardReached=true
                hitsOwnCentre={"own":true,"hit":"SPAN.truncate"}
                download={"filename":"generation3d.stl","bytes":3810,"head":"solid generation3d-previ"}
```

The row is now reachable by pointer (hover + click, owning its own centre pixel), reachable by keyboard
(the arrow walk both marks it and focuses it), and activating it produces real bytes. The step asserts
those four facts — its subject is the MENU. The seven-format roster it used to try to read is owned by
`viewer-actions · viewer-export-lists-every-format`, which drives the picker itself with a real
download and a signature check per format; re-driving that pane from inside a context-menu walk only
added a second failure surface (the window engagement toggle, which is not on screen in the state a
menu walk leaves the shell in) for a fact a registered row already holds.

---

## 2. `status-states` — `stale` had no producer, `error` had no route

### `stale` is dead, and is now deleted rather than left red

`build_flow_status_json` (`🌊️flow/🖥️host/🦀️.rs`) emits `Error`, `Blocked`, `Computing`, `Queued` and
`Ok` — and nothing else. The flow-tick-coalescing lane removed the last `Stale` producer on 09-14 when
it stopped calling an already-recomputed dirty node stale (that was what held `nodes_done` flat for a
whole chain). Grepping the whole tree for another producer of the word as a NODE EVAL STATUS finds
none: `DagNodeEvalStatusKind::Stale` was reachable only from the same `status_json` word, and the
`computing_json` `stale` array is a different channel (`set_computing_progress`, which writes
`computing_stale`, not `node_eval_status`).

A status word a user can never be shown is dead vocabulary, so it is gone from every layer that
declared it:

- `NodeEvalStatus::Stale` (`🌊️flow/🖥️host/🦀️.rs`) and the `"stale"` arm of `preview_chain_status`'s
  census match;
- `DagNodeEvalStatusKind::Stale`, its `"stale"` sync arm and its paint arm
  (`♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`);
- `status_stale` (`Stale` / `Veraltet`) from `✏️editor/🗣️terminology/🦀️.rs` and from
  `🧫️fixtures/🗣️terminology.json`;
- the `"stale"` row of `node_status_label` (`✏️edit/🪟️windows/🕸️flow/🦀️.rs`);
- the two `*:stale` steps and the word from `🐍️status-states-probe.mjs`, and `status_stale` from
  `🐍️panel-i18n-probe.mjs`'s `ownedElsewhere`.

The flow-window law now drives all five surviving tags in both locales and additionally pins that a
word the census cannot emit falls through to `Evaluated` rather than resolving to a label.

### `error` was reachable all along — the probe's drive was aimed at the wrong node

`🐍️error-status-recon.mjs` (new) selects an input widget in the Artifact panel, opens Inspection and
types a degenerate value into the widget's own number field:

```
rows      panel:procedural-play-graph/height  "Column Height Evaluated"  (+ radius, sides, …)
fields    panel:procedural-play-inspector/procedural-play-inspector.value.input  type=number value=6
type "0"  tags {"height":"ok","radius":"ok","sides":"ok","profile":"ok","extrusion-axis":"ok",
                "extrude":"error","column-preview":"ok"}
          row  "ExtrudeCurve Error"
```

`Column Height → 0` makes `extrude` answer `error` in one hop and the outline paints the word. The
probe drove `radius` first, which was never observed to fail; the drive order is now
`height, radius, sides` with that measurement quoted at the call site.

**And the drive never ran at all.** `driveError` opened the Artifact panel with an unconditional
`clickId("framework.panel.artifact")` — a panel tab TOGGLES, and `runPass` had already opened that very
panel, so the click SHUT it. The outline rows were then read off a closed panel, `wanted` came back
empty, and both locales recorded `attempts: []`: the step reported "no error state" for a drive that
had never happened. Every panel open in this probe now goes through an idempotent
`openPanel(tabId, bodyMarker)` that leaves an already-open panel alone, clicks the tab at its LEFT edge
and confirms the panel's own body is on screen before returning.

Measured after both fixes — `error` painted in BOTH locales:

```
statesEn ["ok", "computing", "blocked", "queued", "error"]
statesDe ["ok", "error", "computing", "queued", "blocked"]
status-states 11/11, 232 s, 0 page errors
```

---

## 3. `panel-i18n` — four German fields, all reachable, none driven

All four are real states this probe simply never entered. None is dead.

| field | German | why it was unreached | drive added |
|---|---|---|---|
| `no_selection` | `Keine Auswahl` | `inspection::render` reaches it only on its SECOND branch — a node IS selected and no fixture widget carries that id. Only a DELETE leaves such a selection (proven by `🐍️inspection-i18n-probe.mjs`); this probe never deleted anything. | new region: select a widget row, `Delete`, open Inspection, harvest |
| `preview_hint` | `(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)` | **a product defect, not a probe gap** — see below | the host now paints it; the probe additionally waits for `window:generation3d-generate-preview` by surface id before harvesting |
| `graph_empty` | `(keine Knoten)` | the outline's placeholder rows — and the outline lives in the Artifact panel, which was not the open panel when the empty document was harvested | open the Artifact panel (marker `procedural-play-graph`) before harvesting `No example` |
| `graph_unwired` | `(keine Leitungen)` | same | same |

### `preview_hint` was on the wire and painted by nobody

The generate preview publishes it whenever it delivered no meshes and no instances —
`generate_preview_status_json(…, empty.then(|| labels.preview_hint.as_str()))`
(`🧬️generate/🪟️windows/👁️preview/🦀️.rs`) — and it reaches the browser: `data-status-json` on
`window:generation3d-generate-preview` carries
`hint: "(evaluate a generation to preview output)"`, which is how the role-switch probe could read it
off the DOM. But `world3dComputeStatusV1` (`🖱️ui/🎬️scene/🟦️.ts`), the ONE reader React's
`WorldComputeStatusPane` uses, never decoded a `hint` field at all, and `WorldComputeStatusPane`
returned a zero-size `sr-only` element for every idle surface. **A user staring at an empty generate
preview was told nothing, in either locale.** This is the same shape as `📓️window-gaps-followup`'s F1a
— a channel the producer fills and the host drops — and the producer's half was even already under
law: `generate_preview_entry_state_is_an_idle_status_host_with_a_hint` is green and has been, which is
exactly why nobody noticed the reader.

Fixed at the reader and at the pane: `World3dComputeStatusV1` carries `hint` (bounded at 200
characters — this is chrome text, not a document), and an IDLE surface that has one paints it as
`role="status"` text instead of the silent placeholder. The pane keeps `pointer-events-none` so an
empty viewport is annotated, never covered.

Measured after the fix — all four reached, `unreached: []`, `26/26`:

```
preview_hint  -> 8-no-example        "…Generation auswerten, um die Ausgabe in der Vorschau zu sehen)…"
no_selection  -> 6b-inspection-no-selection
graph_empty   -> 8-no-example
graph_unwired -> 8-no-example
```

`preview_hint` turns up on the EDIT preview of an empty document as well as in generate mode — the
same `empty` branch, the same string, now on screen in both.

---

## 4. The role-switch leftovers

### 4a. `aria-keyshortcuts` disagreed with the badge — one rule now decides both

`mod` was resolved THREE times and only two of them agreed with each other. The dispatcher
(`parseOwnedHotkeyChords(keys, isAppleHotkeyPlatform(platform))`) and the visual badge
(`formatKeybindingShortcut`, `isAppleUiPlatform()`) are platform-aware; `ariaKeyshortcutsText` hard-coded
`mod → "Control"`. On macOS the role buttons therefore rendered `⌘️⌥️V` — the chord that actually
fires — while publishing `aria-keyshortcuts="Control+Alt+V"`, which does nothing.

One owning source: `keybindingPlatformUsesMetaV1(platform?)` in
`🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts` is now the only rule that decides what `mod` means.
`isAppleUiPlatform` is gone into it, `isAppleHotkeyPlatform` delegates to it, and `ariaKeyshortcutsText`
resolves `mod` through it. Both it and `formatKeybindingShortcut` gained an optional explicit
`platform`, so the badge and the ARIA text can be proved to name the same physical key off a real
machine rather than off whatever the test runner happens to be.

**Incidental defect found by that law**: `formatKeybindingShortcut` mapped `left`/`right`/`up`/`down` to
arrow glyphs but the chords are spelled `arrowleft`/`arrowright`, which fell through to the default and
rendered the mode-cycling badge as `⌘️⌥️Arrowright`. The four `arrow*` aliases are now cases.

### 4b. `plugin-ui.owner-close-budget-exhausted` — the ladder was priced as a SETTLE

`closeUiOwner` bounded itself with `PLUGIN_UI_CONTINUATION_LIMIT` (4 096), whose own docstring says it
"bounds ONE settle" — a count of host round trips for a single turn. A whole-instance UI close retires
every surface cell, every wire cursor and every node of every retained surface: a quantity that scales
with the DOCUMENT, not with a turn. The two are unrelated numbers, and the failure named neither a
phase nor a count.

Two changes, both following this codebase's own precedent for budgets
(`retainedUiIntakeStepCeiling`, and `OwnedUiPatchIntake`'s byte-aware stall rule):

- The ceiling is derived from what the close has to RETIRE:
  `retainedUiCloseStepCeilingV1(surfaces) = max(1, surfaces) × maxNodes × PLUGIN_UI_CLOSE_STEPS_PER_NODE`,
  where `surfaces` is the instance's own retained-surface count at the moment the ladder starts and the
  per-node factor (16) is read off `OwnedUiInstance.closeStep`'s twelve branches — observe the cell,
  drain its page and receipt outbox, retire its wire (input retirement, release), close its surface
  child-step by child-step, unlink — each of which `closeChild` reports to the outer ladder as another
  `pending`. `📓️close-ladder-budget-2026-09-12.md` §3.1 is right that raising a budget is not a fix:
  this is not a raise but a re-derivation of a number that was measuring a different quantity, and it
  lands together with a STRICTER stall rule.
- The real guarantee is byte-aware, not a step count: every progressing branch of
  `OwnedUiInstance.closeStep` reports the bytes it released, so 32 consecutive steps releasing NOTHING
  is the stall, and the fault names the phase (`plugin-ui.owner-close-stalled:<phase>`). The ceiling
  fault now names its phase and step count too.

**Not reproduced.** Two teardown drives on the current tree — `🐍️role-switch-runtime-probe.mjs` (5
switches, mid-chain and post-convergence) and `🐍️role-switch-regression-probe.mjs` at the original
1280×800 (3 instances, the exact shape that produced it) — carry no `owner-close-*` line,
`roleSwitchFailed: []`, `pageErrors: []`. So the fix removes the CLASS (a silent 4 096-step spin with no
diagnosis) rather than a byte this lane watched fail.

---

## 5. `WorldOrbitGated.reportCamera` invented the world origin

`reportCamera` and `captureNavigationSnapshot` read `useThree().controls` and fell back to
`targetScratch.set(0, 0, 0)`. The r3f store's `controls` slot is momentarily null between a rig
remount and re-registration, and a report taken in that window published the WORLD ORIGIN as the user's
camera target — overwriting a pose the boot framing had just computed, and letting the gesture
classifier read a pan that never happened.

The fix removes the guess rather than widening it. `start`/`end` are fired BY the controls object, so
`WorldOrbitControlsBridge` now hands that object to its callbacks and both readings take their target
from the gesture's own controls. The rule is pure and law-tested:
`worldCameraReportTargetV1(controls)` answers the controls' target or `null` — never a substitute — and
refuses a non-finite component for the same reason. A report with no target is not made at all.

The two SEEDING call sites that share the shape (`WorldOrbitCameraViewApplier`,
`WorldProjectionApplier`) are deliberately untouched: they compute a NEW pose around the current orbit
target, where an origin default is a seed rather than a reading. They are named in §11.

Live proof that the camera lane still reports real poses rather than fewer of them: the closing
battery's `interact` row is `fit 8/8` and `orbit 8/8`, and every framed camera names the delivered
centroid rather than the origin — e.g. `Sphere Cut With Torus` `target [0.0235, 0, 0]`,
`Face Sweep Extrude` `target [1, 0.75, 2]`, `Box Fillet Preview` `target [1, 1, 1]`.

---

## 6. The guest teardown panic — hypotheses eliminated, guard landed, not reproduced

`ordered-map root must be explicitly retired before drop`
(`🌱️value/🗂️ordered/🦀️.rs:81`) inside `flow-extension-brep`, once, at the end of a run.

**Eliminated by reading and by existing evidence:**

- *The parked-evaluation registry's bare drops.* `cancel_evaluation` drops the removed
  `RetainedEvaluation`, `cancel_all_evaluations` bulk-drops through `jobs.clear()`, and
  `retain_evaluation_job`'s eviction drops the evicted one — three bare drops of a
  `Box<dyn OperatorJob>`, and `OperatorJob` has no retirement door at all while its sibling `Operator`
  has `retire_cold`. But the whole tree contains exactly ONE `step_plan` implementation
  (`BrepBooleanOperatorJob`), which owns `GeometryHandle`/`BrepBooleanJob`/`bool`/progress and no
  `Dictionary` — and `🔬️evaluate-budget` already parks a job and calls `cancel_all_evaluations()`,
  green. So that path cannot be the owner of a live ordered root.
- *A bare `Dictionary` drop.* `Dictionary::drop` routes through `release_shared()` and fails with its
  OWN message ("final Dictionary ownership must be explicitly retired or owned by a cold boundary"),
  which is not the message that was seen. The `:81` message means a raw `OrderedMap` with a live root.

**Landed:** a teardown law in the viewer's eval-chain suite,
`destroying_a_converged_instance_retires_every_ordered_root` — it converges a real preview chain with
the geometry kernel served, performs the teardown a role switch performs (the whole-registry cancel,
then the instance close) and drops the app. Every `OrderedMap` in the process panics on a bare drop, so
the law needs no assertion of its own: one unretired root anywhere in that path aborts it.

```
[DEBUG] teardown after convergence: hops=1 meshes=3 parked evaluations retired=0
test viewer::generation3d::component::eval_chain_tests::destroying_a_converged_instance_retires_every_ordered_root ... ok
```

**A peer had already fixed one instance of exactly this panic, in exactly the code path the run that
produced it took.** `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`'s `render` carries a
comment naming this ticket:

> `generation_fixture_for` CLONES the document fixture, so the patched copy owns its own `layout`
> ordered-map root and must be retired before it leaves scope — a bare drop aborts the plugin actor
> with `ordered-map root must be explicitly retired before drop`. Reachable only once a generation is
> SELECTED, which is why no earlier render test ever hit it.

— and `gen_fixture.retire_cold();` now stands two lines below the clone. The `gaps` run that produced
the 09-15 sighting is the one whose `export-after-generate` step ADDS a generation and selects it,
which is the only route into that branch. That is a strong attribution, and this lane did not verify it
(it would need the pre-fix wasm rebuilt, and that wasm is gone).

**Not reproduced here**, and said plainly: neither the two browser teardown drives of §4b nor this
law produced the panic on the current tree. The law is a guard, not a diagnosis.

---

## 7. `keyboard-verbs` — undo and redo, pressed on a history with something in it

`📓️react-remaining-reds-2026-09-15.md` §12 named this exactly: the two chord rows read the real cursor,
but on this app's quiet boot it reads `canUndo:false, canRedo:false` with only config snapshots in it,
so both rows had only ever taken the UNARMED branch. The ARMED branch — the one that matters — was
pinned by a vitest law and by nothing that had ever run in a browser.

Four rows added to `🐍️editor-verbs-keyboard-probe.mjs`, driven the way a user drives it:

- `edit-target` — a widget row is selected in the Artifact panel and its own number field is on screen
  over a painted preview.
- `edit-arms-history` — a value is TYPED into that field. The shell's history must arm `canUndo`, and
  the preview must deliver different geometry.
- `undo-armed-history` / `redo-armed-history` — the chord must invoke its verb, the published
  `data-history-json` cursor must MOVE, and the preview's delivered geometry must return to the
  pre-edit / post-edit payload.

### What the rows had to be stated in, and why the first two attempts were wrong

**Not a count.** Raising `Column Height` from 6 to 8 re-extrudes the same topology: vertex and triangle
totals are byte-identical before and after an edit that really reached the kernel.

**Not `positions` alone.** The first attempt summed `data-meshes-json`'s `mesh.data.positions` and read
0 on a painted preview. A WIRE mesh publishes `positions: []` beside a full `edgePositions`, and
`positions ?? edgePositions` keeps the empty array — `[]` is not nullish. Measured with
`🐍️preview-delivery-shape-recon.mjs` (new): `window:procedural-preview`, 3 meshes, 3 689 chars, first
entry `{"id":"eval-profile@wire#0","role":"wire","data":{"positions":[],…,"edgePositions":[0.5,0,0,…]}}`.

**Not the cursor's SIGN.** `shellHistoryCursorDomV1` projects an APPEND-ONLY reduction of every
`HistoryPatch` the guest sent, so an undo APPENDS its own entry: measured 17 → 18 on a real undo, and
the cursor also climbs on its own while the probe hovers and selects. The pre-existing rows' armed
branch requires `after.cursor < before.cursor`, which this shell never does — they stayed green only
because they had never been armed. The cursor is therefore asserted as "it moved"; the DIRECTION is
read where the shell actually states it (`canRedo` becomes true, and only an undo can make it so).

**Not a payload that merely DIFFERS.** The first working attempt waited only for the mesh payload to
change, which returns on the FIRST partial delivery of a new evaluation — measured
`box "-0.5,-0.433,0,0.5,0.433,0"` at 442 chars, the profile wire alone with the extruded solid still
computing. The undo pressed on that reading raced the edit's own convergence and the height-8 geometry
arrived AFTER it (`box "…,0.433,8"`), so both armed rows read the wrong document. `settledBox` now
requires THREE consecutive identical boxes on a preview that is not cancellable before any of the four
readings is taken.

The snapshot gained `previewBox` (the corrected bounding box — the settled reading the rows assert
against), `previewDigest` (FNV-1a over every delivered mesh payload, as evidence) and `fixtureValues`
(the sliders' values straight off the document, panel-independent, unlike `inspectorValue`, which is
`null` whenever the Inspection panel is not the open one). Measured extents:
`height 6 → -0.5,-0.433,0,0.5,0.433,6`, `height 8 → -0.5,-0.433,0,0.5,0.433,8`.

### 7.1 What the row found: `mod+z` moved the document and not the preview

With the settle gate in place the row reported, cleanly:

```
edit-target        box -0.5,-0.433,0,0.5,0.433,6   fixtureValues {height: 6, radius: 0.5, sides: 6}
edit-arms-history  box -0.5,-0.433,0,0.5,0.433,8   fixtureValues {height: 8, …}   canUndo true
undo-armed-history box -0.5,-0.433,0,0.5,0.433,8   fixtureValues {height: 6, …}   canRedo true   ← RED
                   invoked ["undo"]   cursor 17 → 18
```

**The document reverted and the preview did not.** A user pressed `mod+z`, the slider went back to 6,
and the 3d view kept the 8-unit column — and the console carried neither a `toolRunStart` nor a
`flowEvalTick` after the chord, against the four the slider edit itself had produced.

**Root cause.** The preview is re-armed by `owe_attached_previews_for_mutations`, which reads a
GESTURE'S OWN EMIT (`emit.artifact_mutations`). `undo` and `redo` are FRAMEWORK-RESERVED
(`HISTORY_ACTION_IDS` in `🔌️plugin/🦀️.rs`, intercepted before `dispatch_action` ever reaches the app),
so they author no app emit and the app never learned its document had moved. The existing docstring
already says a hand-kept roster "can only approximate this" — this is the route it could not see.

**Fix — the rule is the document, not the route.** `applied_document_edits_digest(history)`
(`🧵️preview-eval/🦀️.rs`) folds every command log row that carries an `edit_id` into one number over
`(seq, applied)`: a gesture's edit appends an applied entry, an undo flips one to unapplied, a redo
flips it back, `revertToCommand` and a checkpoint checkout move several. `PreviewEvalRunLink` remembers
it, and `preview_eval_run_effects` — the one poll BOTH surfaces go through — owes every attached
preview window an evaluation whenever it moved. The emit route stays the fast path a gesture takes
without waiting for a poll; this is the backstop that makes the rule "the document moved".

The debt is recorded and a live run woken; the one-request LATCH is deliberately not released, because
a gesture that moved the stack has already asked for its run on its own emit — releasing it there made
the very next poll ask for a second, which
`a_shell_dispatched_example_switch_republishes_both_windows_and_rearms_the_eval_chain` catches by name
("the refresh poll must ask for no second run"). Both halves are under law
(`a_history_verb_owes_the_previews_the_evaluation_a_gesture_would_have`).

**Half-landed, and the remaining half is named exactly.** Measured twice on the 11:54 guest, identical
both times:

```
undo   invoked ["undo"]                                        box …,0.433,8   fixture height 6
redo   invoked ["redo","toolRunStart","flowEvalTick","flowEvalTick"]  box …,0.433,8   fixture height 8
```

The REDO chord re-arms (it did not before: `["redo"]` alone), so the rule works. The UNDO chord records
its debt and **the start that pays it does not arrive until the NEXT gesture's poll** — the
`toolRunStart` on the redo line IS the undo's debt being collected, one gesture late, by which time the
document is back at `height 8` and the preview never shows the undone geometry at all.

So what is left is not the rule but the CADENCE: no `pending_effects` poll follows a framework-reserved
history verb closely enough to read the debt it records. That is the same "a recorded debt with no poll
to read it" hazard `📓️preview-rearm-after-inspector-edit-2026-09-14.md` names — and on this one route
the app cannot do what that lane did (carry the ask on the gesture's own emit), because
`dispatch_action` never sees the gesture. It needs the framework to poll a reserved history verb's app
the way a command's own settle does. `keyboard-verbs · undo-armed-history` is left RED naming exactly
that, rather than softened into a green.

---

## 8. Laws, with output

### 8.1 `🖱️ContextMenu` — a new component suite, registered in the ui-react vitest config

```
cd 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript
SEMIO_TEST_LEVEL=long bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts \
  --testNamePattern="context menu submenu reachability"
→ Test Files 1 passed | 25 skipped   Tests 7 passed
```

| law | what it pins |
|---|---|
| places a submenu beside its anchor row and flips only when the end side would overflow | `contextMenuSubmenuPlacement` — the pure placement rule |
| clamps a submenu whose anchor sits at the bottom edge back into the viewport | the same rule's top clamp |
| keeps a submenu out of the scrolling menu chrome that would clip it away | the panel is NOT inside an `.overflow-y-auto` ancestor and not inside the controller's own container — the defect itself |
| gives a submenu its own menu role, so a pointer inside it is not an outside dismiss | `role="menu"` on the submenu, and `#exportDocument` closes onto one |
| moves real DOM focus onto the active row, not only the painted mark | ArrowDown/ArrowDown/ArrowRight/ArrowDown walks and `document.activeElement.id` is `exportDocument` |
| copies the selection the menu was OPENED on, because opening it took focus and collapsed that selection | `copyDomTextSelection(captured)` |
| announces a group row as a popup owner | `aria-haspopup="menu"` + `aria-expanded` |

The pre-existing in-source law `TextSelectionContextMenuHost opens copy on right-click over selected
text` is the one that CAUGHT the focus regression: it went red on the focus change and is green again
after the capture. That is this lane's "fails before" evidence for §1's fourth point.

### 8.2 `🔀️surface-switch` — one rule for the chord, both spellings

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
SEMIO_TEST_LEVEL=long bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts \
  ../../../../🧪️tests/🔀️surface-switch/🟦️.ts --silent=false --reporter=verbose
→ [DEBUG] surface-switch boot=10 group=4 roleTargets=4 switch=10 work=3 quiesce=3 sealed=5
          busyLabel=2 modeSteps=6 keybindings=4 twin=33 PASS
→ Test Files 1 passed   Tests 1 passed
```

Each of the four keybinding rows and the user-override row now assert `aria` AND `ariaApple` AND
`badge` AND `badgeApple`, plus that the two spellings name the same physical key on each platform
(`Meta+` ↔ `⌘️`, `Control+` ↔ `Ctrl+`). The fixture and its schema carry the three new fields.
**Failed before the arrow-glyph fix** with `'⌘️⌥️Arrowright' !== '⌘️⌥️→'`.

### 8.3 `🔬️engine-contract` — the camera report and the empty-surface hint

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
SEMIO_TEST_LEVEL=long bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts \
  --testNamePattern="world3d compute status pane|a camera report never invents a target"
→ Test Files 1 passed | 47 skipped   Tests 6 passed
```

- `a camera report never invents a target` — `worldCameraReportTargetV1` answers the controls' own
  target, `null` for no controls / no target / a non-finite component, and `{0,0,0}` only when the
  controls really hold the origin.
- `carries the producer's own empty-surface hint, and an idle surface that HAS one is not silent` —
  the hint survives the reader, is bounded at 200 characters, is `""` for a non-string and for a
  producer that sends none, and every shared fixture row still answers a string.

### 8.4 Rust — generation3d

```
CARGO_PROFILE_WASM_DEV_DEBUG=false cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- flow_graph_node_status_is_localized \
  destroying_a_converged_instance_retires_every_ordered_root --nocapture
→ warning: … generated 220 warnings          ← the expansion really ran
→ test editor::…::flow::tests::flow_graph_node_status_is_localized ... ok
→ [DEBUG] teardown after convergence: hops=1 meshes=3 parked evaluations retired=0
→ test viewer::…::eval_chain_tests::destroying_a_converged_instance_retires_every_ordered_root ... ok
→ test result: ok. 2 passed; 0 failed; 469 filtered out

… --lib -- terminology
→ labels_resolve_native_english_and_german_from_the_shared_view_state ... ok
→ german_is_a_translation_rather_than_a_copy_of_the_english ... ok
→ every_user_visible_label_is_declared_in_every_locale_exactly_as_the_shared_fixture_says ... ok
→ test result: ok. 3 passed

… --lib -- a_history_verb_owes_the_previews
→ [STATS] historyRearm edited=9544456440082886626 undone=10166710752253488033 effects=1 second=0
→ test preview_eval::tests::a_history_verb_owes_the_previews_the_evaluation_a_gesture_would_have ... ok

… --lib -- preview_eval   (the whole suite that law joined)
→ 21 passed; 0 failed

… --lib -- eval_chain      (the whole suite the teardown law joined)
→ 6 passed; 0 failed

… --lib -- flow::          (the whole flow-window family the `stale` deletion touches)
→ 8 passed; 0 failed

cargo test … --test example-geometry   → 18 passed; 0 failed
cargo test … --test io-round-trip      → 45 passed; 0 failed
```

### 8.4b The close ladder's budget — `🧪️tests/🔌️plugin-runtime`

```
SEMIO_TEST_LEVEL=long bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts \
  --testNamePattern="plugin ui close ladder budget|dispatch queue before its actor"
→ Test Files 1 passed | 47 skipped   Tests 3 passed
```

- `prices a close by the surfaces it has to retire, never by a settle's round trips` — the ceiling
  scales linearly with the retained-surface count, an instance with none still gets one document of
  room, and it is strictly greater than `PLUGIN_UI_CONTINUATION_LIMIT`.
- `keeps the stall rule far tighter than the backstop, so a stuck ladder is NAMED rather than ground
  through` — the zero-progress bound is below both the old settle bound and the new ceiling.

### 8.5 Rust — the framework crates the `stale` removal touches

```
cargo check -p semio-framework-os-flow --all-targets --keep-going     → exit 0, 382 warnings
cargo check -p semio-framework-os-infinite --target wasm32-unknown-unknown --keep-going
                                                                      → exit 0, 22 warnings
cargo check -p semio-framework-os-infinite --all-targets --keep-going  → exit 0, 132 warnings
cargo test -p semio-framework-os-flow --lib -- census chain_status status_json
→ [DEBUG] flow node census nodes_done per hop: [2, 3, 4]
→ [DEBUG] flow chain ledger: parked=PreviewChainStatus { nodes_done: 2, nodes_total: 4, in_flight: 1,
          working: true } ratios=[0.5, 1.0] settled={ nodes_done: 4, nodes_total: 4, in_flight: 0 }
→ the_node_census_advances_as_a_chain_walks_and_never_calls_a_recomputed_node_stale ... ok
→ the_chain_ledger_is_live_at_a_hop_boundary_and_its_census_only_grows ... ok
```

The wgpu board is checked for **its own target** (`wasm32-unknown-unknown`, 22 warnings = the
`cfg(wasm32)` code really compiled); a native-only check would not have compiled
`DagNodeEvalStatusKind`'s paint arm at all.

---

## 9. Battery

All runs on :6023, with the host TS this lane changed re-fetched through `/@fs/…` first. Five roots:
`react-sweep` (the first gate pass, 10:49 guest), `react-sweep2`/`react-sweep3` (the re-runs of what
that pass found), `react-sweep4`/`react-sweep5` (`keyboard-verbs` against the settle gate and then the
undo re-arm), `react-sweep-full` (a full run abandoned at `role-switch`, see below) and
`react-sweep-closing` (the closing full run, 11:54 guest).

**Why `react-sweep-full` was abandoned.** At 12:22 its `role-switch` row went red on ONE page error —
`SyntaxError: The requested module '🧰️framework/📦️packages/🟦️typescript/🟦️.ts' does not provide an
export named 'hostEffectInvocationV1'`. That symbol is a peer's, added to `🔨️modules/🛂️manifest/🟦️.ts`
mid-run and re-exported by the barrel's `export *`; the browser was reading a vite transform of the
manifest module from before it existed. Touching that module and the barrel restored it
(`/@fs/…/🛂️manifest/🟦️.ts` then names the symbol), and the run was restarted into
`react-sweep-closing` rather than carried on with a stale module in the graph. The twelve rows it had
already finished (`boot` … `status-parity`, 10 green / 2 red, 0 page errors) agree with the closing run
row for row.

### 9.1 The gate — `--only=menus,status-states,panel-i18n,role-switch,keyboard-verbs,journey`

| row | before this lane | after | evidence |
|---|---|---|---|
| `journey` | green 23/23 | **green 25/25**, 114 s | 8 examples × both roles, every mesh oracle |
| `role-switch` | green | **green**, 28 s | 5 switches, `noActor: 0`, `pageErrors: 0`, no `owner-close-*` |
| `menus` | RED 4/6 | **green 4/4**, 52 s | `export-formats` reached by pointer AND keyboard, `generation3d.stl` 3 810 bytes |
| `status-states` | RED 11/15 | **green 11/11**, 232 s | `statesEn`/`statesDe` both carry `error`; `stale` deleted from the vocabulary |
| `panel-i18n` | RED 25/29 | **green 26/26**, 114 s | `unreached: []` — `no_selection`, `preview_hint`, `graph_empty`, `graph_unwired` all on screen in German |
| `keyboard-verbs` | green 14/14 (both history rows UNARMED) | **RED 15/16** | the four new armed rows; `undo-armed-history` is the defect of §7.1 |

`react-sweep2 · panel-i18n` additionally tripped its two gate steps on two page errors, and those were
this lane's own doing: a `touch` of a served module mid-run produced `[DEBUG] hot-swap procedural` at
70.6 s → `destroyApp` → `[os-shell] remounted window refresh failed … no actor for instance 1`. The
`react-sweep3` re-run with no edit in flight is `0` page errors.

### 9.2 The full battery

`SEMIO_BATTERY_URL=http://127.0.0.1:6023/?plugin=generation3d SEMIO_BATTERY_ROOT=react-sweep-closing
bun 🐍️react-battery.mjs`, started 12:23:28, finished 12:58:32, **24 probes, 22 green, 2 red, 2 103 s
of probe time, `pageerrors: 0` on every single row**. Scoreboard:
`🗑️generated/react-sweep-closing/scoreboard.json`. Every row, in run order:

| # | row | verdict | s | what the row read | page errors |
|---:|---|---|---:|---|---:|
| 1 | `boot` | 🟢 | 101 | 2 surfaces up, 98 console lines, no fault | 0 |
| 2 | `journey` | 🟢 | 110 | 23/23 rows converged — 9 edit examples + 9 viewer examples, each against its mesh oracle, plus generate-mode, generate-added, back-to-edit and the role switch | 0 |
| 3 | `interact` | 🔴 | 123 | 53/56 — `payload 8/8`, `selection-reset 8/8`, `hover 8/8`, `orbit 8/8`, `fit 8/8`, `select 7/8`, `inspector 6/8`. RED: `Rectangle Wire Preview · select`, `Rectangle Wire Preview · inspector`, `Hexagonal Mushroom Column · inspector` — §11 | 0 |
| 4 | `generate-mode` | 🟢 | 30 | 11/11 | 0 |
| 5 | `flow-window` | 🟢 | 71 | 2 draws, the graph host mounted on `window:procedural-main`, not hidden | 0 |
| 6 | `flow-wire` | 🟢 | 98 | 6 nodes → cut → 5; the cut paints `extrude` `error: missing input: vector` and 442 meshes; the redraw press lands and restores `ok` with 626; `staleAim: 0` (nothing aims at the deleted tag) | 0 |
| 7 | `flow-reorganize` | 🟢 | 127 | 7 nodes moved and re-read | 0 |
| 8 | `cancel-preview` | 🟢 | 10 | 4 frames, 4 rows | 0 |
| 9 | `keyboard-verbs` | 🔴 | 167 | 15/16 — `historyJsonPublished: true`; `edit-arms-history` invokes `patchFlowWidgets, toolRunStart, flowEvalTick, flowEvalTick` and `redo-armed-history` invokes `redo, toolRunStart, flowEvalTick, flowEvalTick`. RED: `undo-armed-history`, whose `undo` invokes `undo` alone — §7.1 | 0 |
| 10 | `io-surface` | 🟢 | 53 | 6 rows | 0 |
| 11 | `export-encoding` | 🟢 | 50 | real bytes per format: `stl 3 810`, `dwg 1 096` | 0 |
| 12 | `status-parity` | 🟢 | 198 | 10 rows over all four surfaces (`procedural-main`, `procedural-preview`, `generation3d-generate-preview`, `procedural-view-preview`) | 0 |
| 13 | `role-switch` | 🟢 | 28 | `noActor: 0`, `traces: 0`, `pageErrors: 0` — no `owner-close-*` fault (§4b) | 0 |
| 14 | `i18n-a11y` | 🟢 | 59 | 5 tags; `dark` and `de` both survive a reload | 0 |
| 15 | `customization-persistence` | 🟢 | 25 | `localeKept`, `appearanceKept` | 0 |
| 16 | `gaps` | 🟢 | 193 | 12/12 | 0 |
| 17 | `preview-chrome` | 🟢 | 48 | 16/16 | 0 |
| 18 | `status-states` | 🟢 | 230 | 11/11 — `statesEn [ok, computing, blocked, queued, error]`, `statesDe [ok, error, computing, queued, blocked]`; five tags, both locales, no `stale` (§2) | 0 |
| 19 | `menus` | 🟢 | 50 | 4/4 — the export submenu reached by pointer AND by keyboard (§1) | 0 |
| 20 | `viewer-actions` | 🟢 | 83 | 13/13 — this is where the seven-format roster lives, with a download and a signature per format | 0 |
| 21 | `graph-keyboard` | 🟢 | 42 | 13 rows — arrow walk, `Enter` activate, `Escape` clear, `Delete` (which re-arms the chain: `deleteSelection, toolRunStart, flowEvalTick, flowEvalTick`) and `undo` | 0 |
| 22 | `outline-selection` | 🟢 | 37 | 10/10 | 0 |
| 23 | `panel-i18n` | 🟢 | 114 | `reached: 26`, `checkedHere: 26`, **`unreached: []`** of 31, the other 5 owned by `status-states` (§3) | 0 |
| 24 | `inspection-i18n` | 🟢 | 56 | `missing: []`, 23 catalogue items | 0 |

**The gate is NOT fully green, and this lane does not claim it is.** Two rows are red:
`keyboard-verbs · undo-armed-history` — this lane's own seventh item, a real product defect it diagnosed
and half fixed (§7.1) — and `interact`, which is red on three steps of two examples that nothing here
touched (§11). Everything else on the board is green with zero page errors, including all six gate rows
of §9.1 except that one step, and including every row that was green before this lane started.

---

## 10. Files

**Framework — UI kit**
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx` (submenu portal + placement rule +
roving focus + focus restore + captured selection),
`…/🖱️ContextMenu/🧪️tests/🧩️component/🟦️.tsx` (new),
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (registers it),
`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`
(`keybindingPlatformUsesMetaV1`, `mod` in `aria-keyshortcuts`, the four arrow glyphs, explicit
`platform` on both spellings),
`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx` (delegates),
`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` (`World3dComputeStatusV1.hint`).

**Framework — os renderer**
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
(close-ladder ceiling + zero-progress rule + named faults),
`…/🧱️elements/🌐️World3dHost/🟦️.tsx` (an idle surface with a hint paints it),
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`
(`worldCameraReportTargetV1`, the gesture's own controls, `threeVec3ToCad` widened).

**Framework — Rust**
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` (`NodeEvalStatus::Stale` gone, census
match),
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`
(`DagNodeEvalStatusKind::Stale` gone).

**App — generation3d**
`✏️editor/🗣️terminology/🦀️.rs`, `🧫️fixtures/🗣️terminology.json` (`status_stale` deleted),
`✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs` + its `🔬️unit` law,
`🧵️preview-eval/🦀️.rs` (`applied_document_edits_digest`),
`🧵️preview-eval/⏯️tool-run/🦀️.rs` (`PreviewEvalRunLink.applied_edits`, the owe-on-move),
`✏️editor/🦀️.rs` + `👁️viewer/🦀️.rs` (both `pending_effects` hand the digest in),
`🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` (the history-rearm law),
`👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs` (the teardown law).

**Laws and fixtures**
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔀️surface-switch/🟦️.ts` +
`…/🧱️elements/🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json` (three new per-platform fields and
their schema), `…/🧪️tests/🔬️engine-contract/🟦️.ts`, `…/🧪️tests/🔌️plugin-runtime/🟦️.tsx`.

**Probes (ticket folder)**
New recons: `🐍️submenu-reach-recon.mjs` (the clipped submenu),
`🐍️export-form-recon.mjs` (what a menu-dispatched export paints),
`🐍️error-status-recon.mjs` (the route to an `error` node status),
`🐍️preview-delivery-shape-recon.mjs` (what the preview publishes about its geometry).
Changed: `🐍️menu-dump-probe.mjs`, `🐍️status-states-probe.mjs`, `🐍️panel-i18n-probe.mjs`,
`🐍️editor-verbs-keyboard-probe.mjs`, `🐍️react-battery.mjs` (`keyboard-verbs` budget 12 → 24 min).

**Evidence** `🗑️generated/react-sweep/` … `react-sweep5/` and `🗑️generated/react-sweep-full/`.

---

## 11. Not claimed

- **`plugin-ui.owner-close-budget-exhausted` was never reproduced on this tree** (§4b). Two teardown
  drives — 5 role switches on `🐍️role-switch-runtime-probe.mjs` and the original 1280×800 shape on
  `🐍️role-switch-regression-probe.mjs` — are clean, so what is proven is that the ladder is no longer
  priced as a settle and that a stall is now NAMED in 33 steps; it is not proven that this lane removed
  the byte that failed on 09-14.
- **The close ladder's stall BRANCH is pinned by reading, not by a driven test.** §8's laws pin the
  ceiling's shape (derived from the retained surfaces, far above a settle's bound) and that the
  zero-progress rule is far tighter than it. Driving `closeUiOwner` against an owner that answers
  `pending` with no bytes would need a fake `OwnedUiInstance`, which this lane did not build.
- **The guest teardown panic is not diagnosed** (§6). Two hypotheses are eliminated with evidence and a
  guard law landed, but the original sighting has not recurred and its cause is still open. A peer has
  independently fixed one instance of the SAME panic in
  `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs` (`generation_fixture_for` clones the
  document fixture and its `layout` root is now `retire_cold`ed, with the ticket named at the site) —
  that fix is on the restaged guest and is a plausible owner of the 09-15 sighting, since the run that
  produced it had just added and selected a generation. This lane did not verify that attribution.
- **`WorldOrbitCameraViewApplier` and `WorldProjectionApplier` keep their origin default** (§5). They
  compute a NEW pose around the current orbit target, where an origin default is a seed and not a
  reading; they are not camera reports and were left alone rather than changed on a resemblance.
- **No repo-wide `tsc --noEmit` was run.** Neither changed TypeScript package declares a per-package
  project, and the root one is shared with every peer. The changed modules are exercised by vitest and
  by the battery, and each was fetched back through `/@fs/…` after editing so a transform failure could
  not hide — but a type-only error in a path no test takes would not have been caught.
- **Pre-existing reds this lane did not touch.** The whole `@semio-tech/ui-react` suite is
  `14 failed | 739 passed` on this tree, and every failure predates this lane: twelve of them throw
  `ENOENT … 🧰️framework/🎨️styling/🖌️ui/🎨️.css` — a file that does not exist — (`celebrateElements` ×2,
  `icon hover animations` ×5, `control chrome` ×2, `UIIntroduction appearance` ×2 and ContextMenu's own
  `keeps the U-gap transparent`), one throws `ENOENT … 🛂️manifest/🧫️fixtures/🖱️tutorial-local-interaction.json`,
  and `UIDialog accessibility · keeps a nested owned kind picker focusable` is unrelated. The
  ContextMenu suite's other 22 tests, including `TextSelectionContextMenuHost opens copy on right-click
  over selected text` and this lane's own 7, are green. Separately, two `semio-framework-os-flow`
  `wasm_session::domain_laws` tests fail on a bundled `flow_core.js` whose import paths no longer match
  the source. And `🔬️engine-contract` is `3 failed | 622 passed` on this tree — `every pane of one
  document names its OWN window surface` (already named as pre-existing by
  `📓️react-remaining-reds-2026-09-15.md` §12: the fixture was not updated when
  `worldSurfaceSelectionDomV1` gained `selectionMode`/`showEdges`), `styles the projection pane body
  like window options` and `scopes each generation window's own actions to that window` — none of them
  touched here, and this lane's two new tests are among the 622.
- **Two `semio-framework-os-infinite` dag tests and five `semio-s-artifact-procedural-generation3d`
  `--lib` tests fail on this tree and none of them is this lane's.** The dag pair
  (`dag_host_slider_overlay_preserves_language_neutral_field_labels`, a `Float(0.0)` vs `UInt(0)`
  number-type mismatch, and `selected_nodes_cursor_censuses_and_emits_one_byte_per_grant`, 20 bytes vs
  16) touch neither `DagNodeEvalStatusKind` nor anything the `stale` deletion moved; the generation3d
  five are the VCS/envelope/preview trio `📓️window-gaps-followup-2026-09-14.md` §8 already named
  (`two_instances_converge_disjoint_widget_moves`,
  `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`,
  `generation_preview_is_one_app_transient_shared_by_two_generation_windows`,
  `every_panel_publishes_its_body_in_generate_mode_as_well_as_edit`) plus
  `a_shell_dispatched_example_switch_republishes_both_windows_and_rearms_the_eval_chain`, which passes
  in isolation and in every filtered run and fails only in the whole-`--lib` order (the flow operator
  registry is process-global). Every suite this lane owns — `preview_eval` 21/21, `eval_chain` 6/6,
  `example_switch` 6/6, `terminology` 3/3, `flow::` 8/8 — is green.
- **`semio-framework-os-flow --lib` ABORTS as a whole (SIGABRT), and not because of anything here.**
  `a_coalesced_tick_parks_a_whole_wave_and_paints_every_member_computing` passes in isolation
  (`census=[("add","ok"),("left","computing"),("preview","ok"),("right","computing"),("slider","ok")]`)
  and the whole-suite run dies in `FlowHost::drop` → `ArtifactStore::drop` →
  `ArtifactStoreDisplacedRetirements::drop` with "panic in a destructor during cleanup" — a host DROPPED
  where that module's own docstring says "a host is CLOSED, never dropped". Narrowing to
  `-- host::tests` reproduces it with a first panic at `🌊️flow/🌿️vcs/🦀️.rs:694` and a second in
  `an_invalidated_session_hands_an_ephemeral_host_a_re_dispatching_baseline`, so it is the family's own
  shared-state hygiene, not one test. The `--lib` aborts of this crate are on record from 09-13; this
  lane removed one enum variant and one match arm and touched no `Drop`. The two census laws it does
  own (`the_node_census_advances_as_a_chain_walks_and_never_calls_a_recomputed_node_stale`,
  `the_chain_ledger_is_live_at_a_hop_boundary_and_its_census_only_grows`) and the ten `retirement` laws
  are green when run as their own filters.
- **`interact` is RED 53/56 on the closing run and this lane did not investigate it.** The three steps
  are `Rectangle Wire Preview · select`, `Rectangle Wire Preview · inspector` and
  `Hexagonal Mushroom Column · inspector`; every other step of every example is green, including
  `hover 8/8`, `payload 8/8`, `selection-reset 8/8`, `orbit 8/8` and `fit 8/8`. Nothing this lane
  changed touches selection or the inspection panel. **One hypothesis, offered and NOT verified**:
  `Rectangle Wire Preview` is the wire-only example, the click HOVERS it (`hovered: "rect@wire"`,
  `offered: ["rect@wire#0", "rect@wire", "eval-rect@wire#0"]`) and selects nothing — and §7 measured
  that a wire mesh publishes `positions: []` with its geometry in `edgePositions`. A picker that builds
  its hit geometry from `positions` alone would hover a wire and never select it. Worth a lane; this
  one did not open it.
- **The `menus` row no longer reads the seven-format roster** (§1). That is a deliberate re-scoping, not
  a dropped assertion: `viewer-actions · viewer-export-lists-every-format` holds the roster with a real
  download and a per-format signature check, and it is a registered battery row.
- **wgpu is untouched and unmeasured.** The `stale` deletion reaches the wgpu dag board
  (`DagNodeEvalStatusKind`), which is why it is checked for `wasm32-unknown-unknown` — but no wgpu
  battery was run and no wgpu browser reading was taken. `🐍️wgpu-battery.mjs` on :6118 is the first
  thing to pick up here.
- **Editing a served module while a battery row is running causes a hot-swap mid-run, and that is what
  `react-sweep2 · panel-i18n`'s two page errors were.** The row's own subject was green
  (`reached: 26/26`, `unreached: []`) and the gate steps went red on
  `[DEBUG] hot-swap procedural` at 70.6 s → `destroyApp` → `[os-shell] remounted window refresh failed
  … no actor for instance 1`. That is the same hot-swap-teardown shape
  `📓️react-remaining-reds-2026-09-15.md` §12 names, arrived at by a `touch` of this lane's own, not by
  the probe. Every verdict quoted in §9 is from a run with NO source edit in flight.
- **`keyboard-verbs · undo-armed-history` is left RED, and the defect behind it is only half fixed**
  (§7.1). The rule landed (`applied_document_edits_digest`, under law, both halves), and the REDO chord
  demonstrably re-arms on the 11:54 guest; the UNDO chord records its debt and the start that pays it
  arrives one gesture late, because no `pending_effects` poll follows a framework-reserved history verb
  closely enough. Closing it needs the FRAMEWORK to poll a reserved history verb's app, which is a lane
  of its own and not one of this lane's seven. Weakening the row to green would have hidden a defect a
  user meets the first time they press `mod+z`.
- **This lane owned port 6023 only.**
