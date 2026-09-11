# Wave B11 — Editor actions: Add Object dialog, catalogue add/drop, Volume Brush, selection verbs, context menu

Implementation pass, 2026-09-11. Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Scope: checklist §10, §15, §18,
§22, §23 — the defect families `📓️2026-09-11-wave-B1-battery-extension.md` §5 numbered 7, 8, 10, 12 and
14, traced against `📓️2026-09-11-audit-A2-unproven-sections.md`.

No git write, no worktree, everything in the FOREGROUND, every quoted line is real output from this
session. Probe runs are in `🗑️generated/probe-2026-09-11T15-*`.

---

## 0. Headline

Two host defects were found that neither B1 nor A2 suspected, and one of them is bigger than the section
it was found under:

1. **The plugin context menu has never been reachable in the React renderer at all.** `ShellHost` builds
   `requestContextMenu` and publishes it through `PluginSurfaceActionsContext`, but **no**
   `<InterpretedUiNode>` call site passes it as a prop — so every `ComponentSceneHost`'s
   `requestContextMenu` was `undefined` and `openSurfaceContextMenu` was dead code. A right-click on a
   world/board/canvas surface could only ever produce ShellHost's *window-level* fallback menu. That is
   the real §15 root cause (the guest's ignored `surface.hits` is only the second half). Fixed, and
   proven live with a `[DEBUG]` tap.
2. **The Volume Brush Alt+click gesture is fixed and proven live**, and the remaining `FAIL` is
   downstream: the dispatch leaves the page, reaches the guest with the right window and utility, and the
   invocation settles **without a Document frame**. In the same page NOTHING mutates the document —
   `addObjectKind` from the catalogue behaves identically. That is the shared settle/publication lane
   (`settle puzzle#1 empty-required stop … status=more-work`), not this wave's surface.

| lane | result |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `0` lines matching `^error`, `88 warnings` (the crate's pre-existing dead-code set — they prove the crate really expanded) |
| guest `--lib` suite | `687 passed; 2 failed` — both peer-owned, quoted in §7 |
| renderer-react vitest | `Test Files 3 failed \| 20 passed (23)` / `Tests 9 failed \| 863 passed (872)` — **exactly B4's 9 pre-existing failures**, no new one |
| ui-react vitest | `Test Files 5 failed \| 17 passed (22)` / `Tests 5 failed \| 702 passed (707)` — none in Tree/drag, all pre-existing |
| `bun x tsc --noEmit` (both react targets) | zero NEW errors in touched files (§7) |
| browser probe (`:6013`, wasm #44, vite-live host) | five `--only=` runs on fresh pages; verdicts per defect below |

**Host-live now** (vite serves the renderer from source, no wasm rebuild): the Volume-Brush gesture, the
`requestContextMenu` wiring, the ShellHost fallback guard, the Tree drag-payload mirror.
**Rides #45**: every guest change (the manifest reorder, `focusSelection`'s empty-selection rule, the
context-menu `hits` read, the target-volume notice).

---

## 1. §23 — "Add Object" dialog never opens

### Root cause (NOT what the brief assumed)

The brief assumed "the trigger dispatches `shell.openActionPane` instead of the guest
`openAddObjectDialog`". One level up: **two different actions published the same `"Add Object…"` label,
and the wrong one came first.**

- `✏️editor/🦀️.rs` (pre-wave) `.mutation("addObjectKind", puzzle3d_localized_phrase(|l| l.object,
  |w| format!("Add {w}"), …))` — palette-visible by default (`ActionDefinition::new` sets
  `in_palette: true`, `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:905`) and **arg-carrying** (it owns the
  `objectKind` select through `.action_args("addObjectKind", …)`).
- `✏️editor/🦀️.rs` (pre-wave) `.shell_action("openAddObjectDialog", … format!("Add {w}…"))` — the row
  that actually emits `Effect::OpenDialog { dialog_id: "addObject" }` (`🦀️.rs:3407`).
- `ShellHost/🟦️.tsx:9766` rewrites any arg-carrying action to `action: "shell.openActionPane"` and
  appends `"…"` to its label (`:9763`), so `addObjectKind` renders as the literal string `"Add Object…"`
  — byte-identical to the dialog opener's own label.
- `contextMenuEmitOverBudget` (`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:214`) keeps only the first
  `CONTEXT_MENU_PRIMARY_BUDGET = 5` non-destructive leaves at top level (`:70`) and buckets the rest.
  `openAddObjectDialog` was declared 21 rows later, so it landed inside `menu.group.more` ("More ›")
  while `addObjectKind` took primary slot 2. B1's dump is the proof:
  `"shell-menu.action.addObjectKind=2 Add Object…"` at top level, no `openAddObjectDialog` row at all.

So a user (and the probe's `/add object/i` filter) clicked the FIRST `"Add Object…"` row and got a bare
action pane. The dialog was unreachable, not broken.

### Fix

`✏️editor/🦀️.rs` `create_puzzle3d_app`, create-verb block:

- `openAddObjectDialog` moved to the FIRST create slot and declared as
  `ActionDefinition::bounded_catalog(…, ActionKind::Shell).category("create")`, so it takes a primary
  slot instead of being folded into "More ›".
- `addObjectKind` is now
  `…bounded_catalog(…, ActionKind::Mutation).category("create").in_palette(false)`. It is the dialog's /
  the catalogue row's / the viewport drop's parametrized verb; its `objectKind` select IS the dialog, so
  publishing a second, identical user-facing row was the whole defect. Direct dispatchers are untouched —
  `in_palette` gates only palette/menu vocabulary (`ShellHost/🟦️.tsx:9758`).

The dialog itself needed nothing: `app_definition_declares_the_add_object_dialog` and
`the_add_object_dialog_offers_every_object_kind_of_both_examples` already pass, i.e. **the kind select is
already dynamic** (A2 §23 is right; the 09-09 checklist's "hardcoded single option" is stale).

### Law

`exactly_one_add_object_row_is_menu_vocabulary_and_it_opens_the_dialog` — exactly one palette-visible
action carries an "Add Object" label, it is `openAddObjectDialog`, it is `ActionKind::Shell`, it is
declared *ahead of* `addObjectKind` (the primary-budget ordering the defect turned on), and dispatching it
really emits `Effect::OpenDialog { dialog_id: "addObject" }`.

### Probe verdict

```
--only=add-object-dialog   →   add-object trigger=0 menu=[]
                               verdict add-object-trigger-present FAIL trigger=0 menu=[]
```

Still failing at #44, and the reading changed for a good reason: the probe's fallback path right-clicks
the viewport and reads whatever menu appears. Before this wave that was ShellHost's window-level menu
(which carried the misleading `addObjectKind` row); now World3dHost legitimately claims the right-click
(§5) and the guest answers with zero rows at #44, so no menu appears at all. **Rides #45.** The probe
needs no edit — its FIRST locator is already
`[id="shell-menu.action.openAddObjectDialog"], [data-menu-action="openAddObjectDialog"]`.

---

## 2. §18 — catalogue add / drag-and-drop

### 2a. `draggable:false` is **by design**, not a defect

B1 read `(el as HTMLElement).draggable` (`🔍️browser-probe.ts:2150`) — the LIVE property, i.e.
`effectiveDraggable` (`🌳️Tree/🟦️.tsx:2134`). Under the default UI driver a transfer-only row is
deliberately grip-armed:

- `deriveTreeDragRoles` (`🌳️Tree/🟦️.tsx:526-534`) gives a row with `dragData` the single role
  `["transfer"]` (the catalogue row sets it at `📌️panels/🛍️catalogue/🦀️.rs:110-117`);
- `renderTreeDragHandles` renders a real `DragHandle` (`data-slot="drag-handle"`,
  `data-drag-role="transfer"`, `iconKind="move"`) whose `onPointerDown` calls `armDrag`;
- `effectiveDraggable` turns `true` only once that grip is pressed.

That is an explicit, tested contract, not drift:
`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` —
`it("default driver keeps catalogue transfer on the move handle only")` asserts exactly
`/data-slot="tree-item-row"[^>]*draggable="false"/`, and its sibling
`"compact driver collapses drag roles onto the row surface"` asserts the opposite for
`COMPACT_UI_DRIVER`. I built the surface-drag change, found those two laws and **reverted it**: flipping
it would overturn a deliberate per-driver design for every palette in every app. The honest statement is
**the row IS draggable, from its move grip; `data-draggable="true"` is the declared flag and
`el.draggable` is a false negative outside an armed drag.** A2 §18's "rows are draggable" holds.

### 2b. The synthetic drop could never have carried a payload — now it does (**proven live**)

`🔍️browser-probe.ts:2168` builds its `DataTransfer` from
`source.getAttribute("data-drag-payload") ?? source.id ?? ""`, and **no such attribute existed anywhere
in the repo** (`rg data-drag-payload` → 0 hits), so it fell back to the row's DOM id, which is not JSON,
so `parsePuzzle3dCatalogueDragPayload` rejected it and `commitCatalogueDropAt` returned before
dispatching. B1's line shows it:
`catalogue-drag-drop FAIL ran=true payload=panel:puzzle3d-play-kinds/Hexagonal Cut Concrete Forest Left`.

**Fix** (`🌳️Tree/🟦️.tsx`): new exported pure helper `treeRowDragPayloadAttributes(dragData)` emitting
`data-drag-mime` / `data-drag-payload`, spread onto both tree-row shells next to the existing
`data-draggable`, fed from `item.dragData` in `TreeDataItemView`, re-exported from the ui-react barrel.
Same principle as the world host's `data-instances-json`: a native HTML5 drag cannot be driven by
synthetic pointer moves, so the bytes a real `dragstart` would put on the `DataTransfer` must be readable
from the row.

**Proven live** — the same probe step now reads the real payload:

```
catalogue drag-drop {"ran":true,"payload":"{\"objectKind\":\"Hexagonal Cut Concrete Forest Left\",\"meshUrl\":\"/mesh/🧊️hexagonal-cut-concrete-forest-left.glb\"}"}
```

**Law** — `it("a transfer row mirrors its drag payload onto the DOM")`:
`Tests 1 passed | 706 skipped (707)`.

### 2c. Adding still produces no object — and the cause is NOT the catalogue

Every hop is real at this HEAD:

- row → `ui::tree_item(...).try_on_with(Trigger::Activate, "addObjectKind", {objectKind})`
  (`📌️panels/🛍️catalogue/🦀️.rs:88-102`);
- `treeItemToTreeData` maps that binding to `onClick: () => dispatchTrigger(context, record, "activate")`
  (`🗣️Interpreter/🟦️.tsx:1245`);
- `handleSelectItem` really calls it — `item.onClick?.(event, …)` (`🌳️Tree/🟦️.tsx:3257`), not a
  double-click-only path;
- the guest arm lands: `add_object_kind_honors_drop_origin` and
  `add_object_kind_materializes_the_declared_kind_default` both **ok** (including with integer
  coordinates — see §3);
- and the browser confirms the dispatch leaves the page:

```
[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"addObjectKind"}
[DEBUG] command ingress lane {"instanceId":1,"actionId":"addObjectKind","seq":24,"lane":"Interactive"}
[DEBUG] performInvocation settled {"…","actionId":"addObjectKind","frames":2,"frameKinds":["Invocation","Ephemeral"],"historyCursor":null,"historyUpserts":0,"historyCanUndo":null,"effects":0}
```

`frameKinds` carries **no Document frame**: the invocation settled without publishing an artifact
operation. Identical shape for `addTargetVolume` (§3). See §6.

---

## 3. §10 — Volume Brush: Alt+click adds no target volume

### Root cause

The commit gesture lived on an **invisible r3f plane inside the canvas**: `WorldVoxelGroundPlane`
(`World3dHost/🟦️.tsx:3161`, pre-wave) — a 10000×10000 `<mesh>` with `meshBasicMaterial visible={false}`
whose `onClick` required `event.nativeEvent.altKey`. It is mounted at the very END of the scene graph,
BELOW `WorldInstancesLayer` and `WorldVolumeLayer`, so the object under the cursor is always the nearer
intersection — and the probe (like a user) Alt-clicks *on the forest object*, exactly where the gesture is
occluded. Every other ground gesture in this host had already been moved off the canvas for this reason:
catalogue drop (`resolveCatalogueDropOrigin`), the engagement pointer path, and B5's relocate drag all
raycast the ground from the HOST's own pointer handlers via `raycastGroundPoint`. The Volume Brush was the
last one still inside the canvas.

Secondary: `add_target_volume` returned **silently** without `args.origin`
(`🎮️commands/➕️add-target-volume/🦀️.rs:8-10`), so a miss was indistinguishable from a dead gesture.

### Fix

**Host** (`World3dHost/🟦️.tsx`), new `//#region WorldVolumeBrushGesture` beside the relocate region:

| what | kind |
|---|---|
| `world3dVolumeBrushOriginV1(ground, gridFactor)` | new exported pure helper — `snapWorldPointToGrid(ground, true, gridFactor)`, `null` on a ray miss |
| `world3dVolumeBrushCommits(volumeBrushMode, altKey)` | new exported pure helper — the whole arming predicate |
| `voxelGroundOriginAt(clientX, clientY)` | new `useCallback`; ONE raycast shared by the hover ghost and the commit |
| `handlePointerDown` | one new branch after relocate: armed + Alt → set the ghost, `handleVoxelPlace(origin)`, return (so no marquee opens) |
| `handlePointerMove` | one new line: while armed, `setVoxelHoverOrigin(voxelGroundOriginAt(...))` — it does **not** return, so orbit/marquee are untouched |
| `useEffect` on `volumeBrushMode` | clears a stale ghost when the utility disarms (the r3f `onPointerOut` that did it is gone) |
| `WorldVoxelGroundPlane` | **deleted** (component + its single JSX use). `WorldVoxelPreviewBox` is unchanged, now fed by the host hover. |

Both helpers re-exported from the renderer-react barrel next to `world3dRelocateDispatchArgsV1`.

**Guest**: a dispatch with no usable `origin` now raises exactly one localized notice
(`ctx.notice(|l| l.target_volume_origin_required…)` + `ctx.abort = true`). New EN/DE label pair in
`🗣️terminology/🦀️.rs` (`"Point at the ground plane to place a target volume"` / `"Zeigen Sie auf die
Bodenebene, um ein Zielvolumen zu platzieren"`, both `native` and `reuse` axes authored).

### Laws

- vitest `🔬️engine-contract` —
  `it("Volume-Brush Alt+click reads the ground through the host, not an occludable canvas plane")`:
  `Tests 1 passed | 868 skipped (869)`.
  *(Re-running it: `-t "Volume-Brush Alt+click"` matches NOTHING — vitest's `-t` is a regex and `t+` is a
  quantifier. Use `-t "reads the ground through the host"`.)*
- cargo — `alt_click_adds_one_target_volume_sized_by_the_utility_voxel_dims` (arm the utility, drive the
  three `setVoxelDims` axes, dispatch `addTargetVolume` at a ground origin, assert +1 volume whose scale
  follows W<D<H, **and** that an integer-valued origin like the live browser's `[0,10,0]` places one too)
  and `add_target_volume_without_a_ground_point_raises_one_notice_and_places_nothing`.

### Probe verdict — gesture PROVEN LIVE, publication is downstream

`verdict volume-brush-add-target-volume FAIL before=0 after=0` still, but a temporary `[DEBUG]` tap in
`handleVoxelPlace` (added, run, **removed**) shows the whole chain working:

```
warning: [DEBUG] b11 voxel-place origin=[0,10,0] surface=1 window=puzzle3d-main-perspective
warning: [DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"addTargetVolume"}
warning: [DEBUG] command ingress lane {"instanceId":1,"actionId":"addTargetVolume","seq":34,"lane":"Interactive"}
debug:   [DEBUG] puzzle3d.utility.publish action=addTargetVolume window=Some("puzzle3d-main-perspective") utility=volumeBrush map_hit=true
warning: [DEBUG] performInvocation settled {"…","actionId":"addTargetVolume","frames":2,"frameKinds":["Invocation","Ephemeral"],"historyCursor":null,"historyUpserts":0,"historyCanUndo":null,"effects":0}
```

The Alt+click raycasts the ground, snaps to the lattice, dispatches, and the guest receives it on the
right window with the right utility. What is missing is the **Document frame** — see §6. (In the very
first run of the session the same dispatch settled with `"historyUpserts":1,"historyCanUndo":true`, so the
publication is intermittent, not absent.) `volume-brush-arm` and `volume-brush-voxel-dims` PASS, so the
guest instance-scope bug was never what blocked this — the occlusion was.

---

## 4. §22 — duplicate / delete / focus selection

### 4a. `focusSelection` — real guest defect, fixed

`Puzzle3dFocusSelectionWork::step`'s `Publish` stage began
`if self.matched == 0 { return …Complete(Emit::default()) }`, and `matched` only counts objects inside the
selection set. With nothing selected the work completed empty: no camera move, no notice, nothing in the
console — `f` was a dead key on every boot before the user had picked anything, exactly as B1 measured
(`focus-selection FAIL before={…} after={…}` byte-identical).

**Fix**: new `Puzzle3dFocusSelectionWork::frames(&self, object_id)` —
`self.selected.is_empty() || self.selected.contains(object_id)` — used by both the `SumObjects` and
`DistanceObjects` stages, so an empty selection frames the WHOLE document. No extra capacity: the two
object scans are already what `extent` declares (`objects.len() * 2 + 1`). The same subject rule is
mirrored in `apply_puzzle3d_focus_selection` (the non-retained twin) so the two lanes cannot disagree.

**Law**: `focus_selection_frames_the_whole_document_when_nothing_is_selected` — parks the camera far away,
dispatches `focusSelection` with an empty selection, asserts the camera moved, that `result.mutations` is
empty (§22's "focus emits no artifact history entry") and that the document is byte-identical.
`focus_selection_hostile_static_law_rejects_hidden_whole_collection_work` still passes.

### 4b. `duplicateSelection` / `deleteSelection` — not defects; the precondition is

Both arms are correct and both refuse deliberately: `duplicate_selection` →
`ctx.refuse_without_selection` (`🎮️commands/👯️duplicate-selection/🦀️.rs:15`), `delete_selection`
likewise (`🎮️commands/🗑️delete-selection/🦀️.rs:13`), and the existing law
`selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice` already pins that each
raises exactly one real-prose notice, emits no mutation and does not force a full refresh.
`duplicate_selection_reselects_the_created_clones` (the brief's "duplicate must reselect the clone")
passes unchanged — the clone ids ride `Emit.interaction_writes` via `ctx.replace_selection`.

So B1's `duplicate-selection FAIL before=1 after=1` / `delete-selection FAIL before=2 after=2` are the
**empty-selection precondition** (the same one that makes `context-menu-rows` report
`entitySelected=[]`), a §6 viewport/outliner-select problem, plus §6's own no-document-mutation blocker.
B1's note that Delete worked once (`before=1 after=0`) is consistent with that and inconsistent with a
dead verb. §4a is deliberately written so focus no longer depends on a selection at all.

### Probe verdict

`focus-selection` rides **#45**. `duplicate-selection` / `delete-selection` stay blocked on §6.

---

## 5. §15 — context menu: two host root causes and one guest one

### 5a. `requestContextMenu` was never wired into any surface host (**the big one**)

`ShellHost/🟦️.tsx:4205` builds `requestContextMenu` and publishes it at `:9807` through
`PluginSurfaceActionsContext`. `World3dHost` takes it as a **prop**
(`World3dHost({ node, onAction, requestContextMenu })`) and passes it to `openSurfaceContextMenu`. But
`rg "<InterpretedUiNode"` finds three call sites — `ShellHost:9267` (windows), `ShellHost:9232`
(spawned), `ShellHelpers:1911` (panels) — and **not one of them passes `requestContextMenu`**. The prop is
optional, so it was always `undefined`, `World3dHost`'s `onContextMenu` returned at
`if (!requestContextMenu) return;` (before it ever calls `preventDefault`), and the entire
plugin-context-menu branch was dead code in the React renderer. Everything B1 read as "the puzzle3d menu
never appears" was ShellHost's *window-level* fallback menu arriving because the event was never claimed.

Proven, not deduced: a temporary `[DEBUG]` tap at the top of `openSurfaceContextMenu` (added, run,
**removed**) printed **nothing** on a viewport right-click before the fix, and after it:

```
warning: [DEBUG] b11 surface-menu kind=world3d hasRequest=true hasFallback=true
```

**Fix** (`🗣️Interpreter/🟦️.tsx`, `InterpretedUiNode`): read the context and default the prop to it —
`requestContextMenu: requestContextMenu ?? usePluginSurfaceActions()`. One `??` wires every surface host
at once, which is what that context exists for. **Law**:
`test("InterpretedUiNode hands every surface host the shell's plugin context-menu resolver")`.

### 5b. The shell fallback opened on top of the surface menu

`ShellHost`'s window-level `contextmenu` listener skipped only
`isContextMenuPointerTarget(event.target)`; it did **not** check `event.defaultPrevented`, while every
`ComponentSceneHost` claims a right-click by calling `event.preventDefault()` synchronously before
awaiting its plugin menu. The builder's own docstring defines the fallback as *"shown for any right-click
no inner surface claimed"*, so this was a straight omission — and with 5a fixed it would have meant two
menus on every viewport right-click.

**Fix**: `if (event.defaultPrevented) return;` in that listener. **Proven live**: the same probe step that
used to dump ten `shell-menu.action.*` rows now reports `context-menu rows=[]`.

I also tried making an EMPTY plugin answer fall back to the shell menu and **reverted it** — the existing
law `test("openSurfaceContextMenu keeps an empty plugin answer off the shell fallback")` pins the
opposite, deliberately. The comment now says so at the call site.

### 5c. Guest: `surface.hits` was never read

`Puzzle3dContextSelection::from_surface` iterated **only** `surface.selection`. Its own docstring claims
*"The surface's own hits/selection still win when present, so a right-click on an unselected entity still
targets what was clicked"* — but `hits` appeared nowhere in the file, while `World3dHost` does send it
(`hits = target ? [{ domain: target.kind, id: target.id }] : []` from `resolveWorldContextMenuTarget`).
So a right-click on an unselected object produced `Vec::new()` — no `duplicate` / `select-same-kind` /
`zoom` / `hide-show` / `lock-unlock` / `delete` row anywhere.

**Fix**: `from_surface` folds `surface.hits` in after `surface.selection`, through a shared
`bucket(domain)` accessor, and only into a bucket the selection left EMPTY. Both halves of the contract
hold: a right-click on an unselected entity targets what was clicked, and a right-click inside a live
multi-object selection keeps the whole selection as the subject (so "Delete (3 objects)" can never
silently narrow to the row under the cursor).

**Laws**: `right_clicking_an_unselected_object_opens_that_object_menu_not_an_empty_one` (walks the whole
menu tree including group children and requires all six §15 row ids) and
`a_hit_inside_the_selection_keeps_the_whole_selection_as_the_menu_subject`. New testkit helper
`context_menu_for_hit(app, domain, id)` beside `context_menu_for_selection`.

### Probe verdict

```
--only=context-menu-rows   →   context-menu rows=[]
                               verdict context-menu-opens FAIL rows=0
                               verdict context-menu-object-vocabulary FAIL missing=[duplicate, select-same-kind, zoom, delete, hide-show, lock-unlock] present=[]
```

5a and 5b are live and proven (see the two quoted taps). 5c rides **#45**. One further live observation
worth an owner: with the resolver now wired, the `[DEBUG] b11 surface-menu` tap printed the ENTRY line but
**never** the matching `resolved items=` line during the whole step — `plugin.contextMenu(...)` did not
settle at all on that instance while the brush-mesh upload storm was running (same page shows
`registerBrushMesh … command ingress did not complete within 1024 continuations`). Until that resolves,
§15 will show an empty menu even on #45.

---

## 6. The one blocker everything else is waiting on (handed over)

In a FRESH page with a single `--only=` step and **zero** hard faults, **no document mutation lands at
all**: `addTargetVolume` (§3) and `addObjectKind` from both the catalogue click and the catalogue drop
(§2c) each settle with `frames: ["Invocation","Ephemeral"]` — no Document frame, `historyUpserts: 0` — and
the surrounding console is full of

```
warning: [DEBUG] settle puzzle#1 empty-required stop continuation=1 status=more-work drain=true
error:   [DEBUG] action failed registerBrushMesh {…} Error: [DEBUG] plugin puzzle: command ingress did not complete within 1024 continuations (observed statuses: command-pending, idle)
error:   [DEBUG] action failed registerBrushMesh {…} Error: typed-operation failed: typed-operation pending publication rejected a stale immutable document root
```

i.e. settle stops while the plugin still reports `more-work`, so the frame that would carry the artifact
operation never arrives. Intermittently it DOES (the session's first `addTargetVolume` settled with
`historyUpserts: 1`). This is the settle/publication lane (B4/B7/B8 territory), not §10/§18, and it is
what keeps `volume-brush-add-target-volume`, `catalogue-add-object-kind` and `catalogue-drag-drop` red
even with their gestures proven.

---

## 7. Files changed and quoted outputs

### Files

Guest (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`):

- `🦀️.rs` — `Puzzle3dContextSelection::bucket`/`from_surface` (hits); `Puzzle3dFocusSelectionWork::frames`
  + both scan stages; `apply_puzzle3d_focus_selection` empty-selection rule; `create_puzzle3d_app`
  create-verb block.
- `🎮️commands/➕️add-target-volume/🦀️.rs` — notice instead of a silent return, plus a docstring.
- `🗣️terminology/🦀️.rs` — `target_volume_origin_required` (EN/DE, native+reuse).
- `🧪️tests/🔬️testkit/🦀️.rs` — `context_menu_for_hit`.
- `🧪️tests/🔬️unit/🦀️.rs` — six new laws.

Host:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` —
  `//#region WorldVolumeBrushGesture`, `voxelGroundOriginAt`, the two pointer-handler branches, the
  disarm effect, `WorldVoxelGroundPlane` removed.
- `…/🧱️elements/🗣️Interpreter/🟦️.tsx` — `InterpretedUiNode` reads `usePluginSurfaceActions()`;
  `openSurfaceContextMenu` tidied (behaviour unchanged, comment added).
- `…/🧱️elements/🏛️ShellHost/🟦️.tsx` — `defaultPrevented` guard on the fallback listener.
- `…/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — re-export of the two world helpers.
- `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — two new laws + imports.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` — `treeRowDragPayloadAttributes`, the two row
  spreads, the `dragData` prop.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — re-export +
  `registerTests2` dependency.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` — new law.

Ticket: this report. Probe artifacts under `🗑️generated/probe-2026-09-11T15-*` (kept).

### cargo

```
$ cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
warning: `semio-s-artifact-puzzle-3d` (lib) generated 88 warnings (run `cargo fix --lib -p semio-s-artifact-puzzle-3d` to apply 85 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.76s
$ … | grep -c '^error'
0
```

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 exactly_one_add_object focus_selection alt_click_adds_one_target add_target_volume_without right_clicking_an_unselected a_hit_inside_the_selection duplicate_selection_reselects context_menu
running 13 tests
test editor::puzzle3d::component::tests::a_hit_inside_the_selection_keeps_the_whole_selection_as_the_menu_subject ... ok
test editor::puzzle3d::component::tests::add_target_volume_without_a_ground_point_raises_one_notice_and_places_nothing ... ok
test editor::puzzle3d::component::tests::alt_click_adds_one_target_volume_sized_by_the_utility_voxel_dims ... ok
test editor::puzzle3d::component::tests::context_menu_at_selects_object_groups_flags_and_keeps_delete_last ... ok
test editor::puzzle3d::component::tests::context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden ... ok
test editor::puzzle3d::component::tests::context_menu_at_selects_vortex_and_prepends_suggest_objects ... ok
test editor::puzzle3d::component::tests::duplicate_selection_reselects_the_created_clones ... ok
test editor::puzzle3d::component::tests::every_context_menu_row_dispatches_a_declared_action ... ok
test editor::puzzle3d::component::tests::exactly_one_add_object_row_is_menu_vocabulary_and_it_opens_the_dialog ... ok
test editor::puzzle3d::component::tests::focus_selection_frames_the_whole_document_when_nothing_is_selected ... ok
test editor::puzzle3d::component::tests::focus_selection_hostile_static_law_rejects_hidden_whole_collection_work ... ok
test editor::puzzle3d::component::tests::object_context_menu_owns_puzzle_rows_not_shell_fallback ... ok
test editor::puzzle3d::component::tests::right_clicking_an_unselected_object_opens_that_object_menu_not_an_empty_one ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 676 filtered out; finished in 1.11s
```

Full crate suite: `test result: FAILED. 687 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out;
finished in 104.24s` — `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin`
and `two_instances_converge_disjoint_object_edits_via_backbone`, both peer-owned and neither touching this
wave's surface. The backbone one fails on `attach_backbone`, before any action runs:

```
panicked at …🔬️unit/🦀️.rs:3962:76:
attach b: Fault { … code: FaultCode("module.vcs"), … message: "validation failed: remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized" … }
```

### vitest

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts
 Test Files  3 failed | 20 passed (23)
      Tests  9 failed | 863 passed (872)
```

The 9 are byte-for-byte B4's pre-existing set: 6 × `🧩️package-integration` (generated-worker
byte/provisioning laws), 1 × `🔬️engine-contract` (`buildNoteShellCommandAction`), 2 × `🔌️PluginRuntime`.
None appears in this wave's diff.

```
$ SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts
 Test Files  5 failed | 17 passed (22)
      Tests  5 failed | 702 passed (707)
```

The 5: `.storybook/🧭️scope-resolution` and `📦️react-package-export` (the pre-existing
`Cannot bundle built-in module "bun:sqlite"` / package-entry-path breakage), two `🕸️Diagram` 20 000-node
force laws, one `📨️UIDialog` focus law, one `UIIntroduction appearance` law. Both existing Tree drag laws
pass.

### tsc

`🖱️ui/…/⚛️react`: **zero** lines matching `Tree/` or the package barrel.
`📺️renderer/…/⚛️react`: only pre-existing, peer-owned lines — `World3dHost(3251,12)`/`(4109,12)`
`pickEnabled`, `World3dHost(4446,42)` `addEventListener("contextmenu")`, four `ShellHost` lines
(`1903`, `7765`, `7766`, `🗨️dialog-origin/…`), and `Interpreter(1457,164)` `import.meta.dir` (the same
`ImportMeta` symptom the ui barrel already reports at 10870/10926). None is in this wave's hunks.

---

## 8. Probe recipe for the next pass

`:6013` was occupied by another agent's `--battery` for most of this wave (`pgrep -f "browser-probe"`
non-empty; `curl` to the port timed out at 8 s, 90 s and 120 s while it ran). It freed up at the end and
five `--only=` runs were made on fresh pages. For the re-run after #45:

1. `--only=add-object-dialog --port=6013` — expect `add-object-trigger-present` to resolve on its FIRST
   locator and the dialog to open with a multi-option kind select.
2. `--only=context-menu-rows --port=6013` — expect the six object rows **without** a prior selection,
   because the right-click hit alone now seeds the guest menu (§5c). If the menu is still empty, check
   whether `plugin.contextMenu` resolves at all (§5's closing observation).
3. `--only=volume-brush --port=6013` — the gesture is already proven; this flips only once §6 lands.
4. `--only=catalogue-panel --port=6013` — `catalogue-drag-drop` now carries a real payload; same §6 gate.
5. `--only=selection-keybindings --port=6013` — `focus-selection` should flip on its own, with no
   selection required.

Two time-savers, both cost real minutes to rediscover:

- vitest's `-t` is a **regex**: `-t "Volume-Brush Alt+click"` silently matches nothing (`t+` is a
  quantifier) and reports `869 skipped` as if the suite were gated.
- the probe's `el.draggable` read is `effectiveDraggable`, not the declared flag — read `data-draggable`
  (and now `data-drag-payload`) before calling a palette row "not draggable".

## 9. Handed to other waves

- **§6 — no document mutation lands, and viewport/outliner selection never lands.** Both blockers are
  above this wave's surface and gate five remaining verdicts.
- **`plugin.contextMenu` does not settle on a busy instance** (§5) — newly reachable, newly visible.
- **Transfer-drag ergonomics** (§2a) — `treeRowSurfaceDragEnabled` was built and reverted because
  `🧪️owned-locale-detector-retirement/🟦️.tsx` pins the opposite contract per UI driver. Making catalogue
  rows draggable without arming the grip is a UI-driver decision, not a puzzle3d one.
