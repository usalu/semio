# Audit A2 — DOM-to-guest trace for never-browser-proven checklist sections

Read-only. Scope: checklist §1, §2, §3, §4, §10, §11, §14, §15, §17, §18, §19, §22, §23, §24, §25 —
sections `🔍️browser-probe.ts --battery` never touches (confirmed: `grep -i` over the probe source for
grid/lod/vortex-show/projection/volume-brush/relocate/engagement/outliner/catalogue/settings/addObject/
locale/German returns nothing outside unrelated matches). Cross-referenced against
`📓️2026-09-10-checklist-reverification.md` (audit A1) — I do not re-litigate A1's backend/command
findings where they hold; I add the DOM→dispatch→guest chain, exact selectors, and two corrections A1
also missed (§11, §24). All line numbers are current-source (`main` @ this session's checkout), not the
checklist's original citations, which have drifted.

Host files: `ShellHost` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
(9982 lines); `World3dHost` = `.../🧱️elements/🌐️World3dHost/🟦️.tsx` (6112 lines); `ShellHelpers` =
`.../🧱️elements/🛠️ShellHelpers/🟦️.tsx` (4361 lines); `Window` = `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx`;
react-package = `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`.
Guest editor root = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (8431 lines).

---

## Cross-cutting mechanism found this pass (applies to §1–§4, §9–§11, §14, §23, §24)

Every window-instance container/fold-toggle DOM id is produced by
`childElementId("framework.window", id, ...segments)`
(`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🆔️ElementId/🟦️.tsx:51`), which runs each segment through
`elementIdSegment` (`:24-40`) — **dashes are stripped and the following letter is capitalized**, i.e.
kebab-case window instance ids become camelCase in the DOM. So:

- Window container: `#framework.window.puzzle3dMainTop`, `#framework.window.puzzle3dMainPerspective`
  — **not** `#framework.window.puzzle3d-main-top` as checklist §1 and the original source comment
  (`ShellHost/🟦️.tsx:9264` docstring) imply. Verified against a real assertion, not just the
  transform: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx:322-327`
  hardcodes `id="framework.window.puzzle3dMainTop"` and `data-element-alias="framework.window.puzzle3dMain"`
  (the window-KIND alias, also camelCased). **This is the correction a probe author most needs**: every
  selector in the original checklist that names a window-instance id (`puzzle3d-main-top`,
  `puzzle3d-main-perspective`) is wrong as a literal DOM `id` — use the camelCase form, or better,
  `elementIdSelector("framework.window.puzzle3d-main-top")` from the same module (`:59-64`), which
  matches both the literal id AND the `data-element-alias`.
- Window chrome overlays are **folded by default** and must be unfolded before any measure/utility/
  engagement/action-pane control inside them is interactable or even present in the DOM
  (`Window/🟦️.tsx:171-184`, `folded` defaults `true`). Toggle ids, all `childElementId`-derived
  (`Window/🟦️.tsx:332,410,349-368,388-390`), same camelCase rule applies to the `id` argument:
  - Measures rail (§2/§3/§4 window options): `#framework.window.puzzle3dMainTop.measures.unfold`
  - Utility bar (§9/§10/§11 utility options + utility switcher): `#framework.window.puzzle3dMainTop.utilityBar.unfold`
  - Engagement/action pane (§14 status + §23 "Add Object…" + §24 "Export"/"Import…"):
    `#framework.window.puzzle3dMainTop.engagement.toggle`
  - Search/command-line (§14 typed input): `#framework.window.puzzle3dMainTop.search.toggle`
  All four exist independently per window instance — Top and Perspective each need their own unfold.
- The clickable toggle element itself is `WindowPaneChromeToggle` with `id={chromeToggleId}`
  (react-package `:9500-9548`) — a real, addressable, single element, confirmed not just a `data-*` hook.

---

## §1 Windows — Top, Perspective — **WIRED** (source), camelCase-id correction above

Chain: mount → `ShellHost/🟦️.tsx:9264-9265` `id={childElementId("framework.window", instance.id)}`
`data-element-alias={childElementId("framework.window", kind.id)}` → guest
`main/🦀️.rs:31-32` `WINDOW_INSTANCE_TOP = "puzzle3d-main-top"` / `WINDOW_INSTANCE_PERSPECTIVE =
"puzzle3d-main-perspective"`, `main/🦀️.rs:35-37` template ids encode the orthographic-top vs
three-point-free-orientation projection presets → `main::render` (`:563-573`) builds `World3dScene::base(camera_json(...), ...)`.
No dead links found. `Split/close` correctly absent by design (`main::definition`, `actions: Vec::new()`
intentional — see the cross-cutting fallback note under §22 below, not a §1 concern).

**Probe:** `page.locator('[id="framework.window.puzzle3dMainTop"], [data-element-alias~="framework.window.puzzle3dMainTop"]')`
and the `...Perspective` twin, both present at boot with no click needed. Assert child canvas exists
(`WorldCanvas`/`data-surface-id` inside `World3dHost/🟦️.tsx:5828`) and `data-instances-json`/
`data-meshes-json` attributes (`:5833-5834`) are non-empty after an example loads.

## §2 Camera — orbit/pan/zoom — **WIRED** (source, matches A1), one real DOM-exposure gap found

Chain: pointer drag on canvas → `World3dHost/🟦️.tsx:4689` `dispatch("setCamera",
worldCameraSetCameraDispatchArgs(windowInstanceId ?? node.surfaceId, state))` (fired on gesture
release via `WorldOrbitGated.onCamera`, `:5295`) → guest `set-camera/🦀️.rs:6-12` parses `args.camera`
(malformed payload = silent no-op, unchanged from A1) → merged into per-window
`Puzzle3dWindowConfig` via the W-D2-fixed `🎚️config/🦀️.rs:100-104` byte-grant check.

**Defect (new, minor):** unlike instances/vortices/brush-preview
(`data-instances-json`/`data-vortices-json`/`data-brush-preview-json` at `World3dHost/🟦️.tsx:5833-5835`),
**`cameraJson` is never exposed as a `data-*` attribute** — grep for `data-camera` in `World3dHost/🟦️.tsx`
returns nothing; `scene.cameraJson` (`:4439`) only ever feeds the internal `OrbitControls` seed. A
Playwright probe cannot read post-orbit camera pose from the DOM at all; it must either (a) monkey-patch
`dispatch`/intercept the outgoing `setCamera` action args via `page.exposeFunction`/console hook, or
(b) screenshot-diff, or (c) read the Projection measure's `Select` value (§3) as an indirect proxy (only
covers projection family, not raw pose). Worth flagging to the dev as a probe-ability gap, not a user
bug.

**Probe:** unfold `#framework.window.puzzle3dMainTop.measures.unfold`, then
`page.mouse.move`+`down`+`move`+`up` inside the perspective canvas
(`[data-surface-id]` under `#framework.window.puzzle3dMainPerspective`). Assert no console error/hang
(the W-D2 fix's regression signature) and, if instrumented, that the intercepted `setCamera` args
changed.

## §3 Projection pane options — **WIRED** (source, matches A1)

Chain: `#framework.window.puzzle3dMainTop.measures.unfold` → measures rail → `Select`
`id={measure.id}` (`ShellHelpers/🟦️.tsx:3008-3024`, `windowMeasureSelectControl`) for
`puzzle3d-measure-projection-*` ids (built by `options/🎥️projection/🦀️.rs:17`,
`world3d_projection_measures`) → `onValueChange` dispatches `{...measure.onChange, args:{value}}`
tagged with `windowId` by `windowMeasuresChrome`'s `taggedOnAction`
(`ShellHelpers/🟦️.tsx:3208-3210`) → `setProjection`/`setProjectionParam` on the `WindowConfig` lane
(W-D2-fixed, same as §2).

**Probe:** unfold measures, `page.locator('#puzzle3d-measure-projection-<mode-select-id>')` (grep the
built id at `options/🎥️projection/🦀️.rs` for the exact suffix at build time — not hardcoded here since
it's assembled from a format string), switch orthographic↔threePoint, assert the pane's visual
projection changes (screenshot) since cameraJson has no DOM hook (§2's gap applies here too).

## §4 Window options — grid/LOD/vortex/sun/select — **WIRED** (source, matches A1), exact ids confirmed live

Chain: same rail as §3. Every measure id from the checklist is the **literal, unmodified** DOM `id` —
these do NOT go through `childElementId`'s camelCase transform (unlike §1's window container ids):
`Select`/`TreeCheckbox`/`Slider` all render `id={measure.id}` directly
(`ShellHelpers/🟦️.tsx:3009,3011,2949` for select/toggle/slider respectively), and `measure.id` is the
raw string the guest wrote (`puzzle3d-play-grid-visible`, `puzzle3d-play-lod-value`,
`puzzle3d-measure-sun-azimuth`, etc., per `main/🦀️.rs:68-79` wiring `window_measures`). So checklist
§4's id table is **directly usable as Playwright locators** once the rail is unfolded — no translation
needed, in contrast to §1.

**Probe:** unfold `#framework.window.puzzle3dMainTop.measures.unfold`, then e.g.
`page.locator('#puzzle3d-play-grid-visible')` (a `TreeCheckbox`, click toggles `checked`), assert no
hang (W-D2 regression signature) and that a second window instance's own rail is unaffected (per-window
isolation, `windowMeasuresChrome`'s `windowId` tag, `ShellHelpers/🟦️.tsx:3198-3212`).

## §10 Volume Brush utility — **WIRED** (source), new granular ids

Chain: utility bar → click "Volume Brush" utility button (`UtilityTree`, inside
`#framework.window.puzzle3dMainTop.utilityBar.unfold`) → `setActiveUtility{utilityId:"volumeBrush"}`
→ guest `volume-brush/🦀️.rs:11` `UTILITY_ID = "volumeBrush"`, options group id
`puzzle3d-play-utility-options-volume-brush` (`:39-51`, matches checklist) whose **children are three
sliders with their own literal ids not previously documented**: `puzzle3d-voxel-w`,
`puzzle3d-voxel-d`, `puzzle3d-voxel-h` (`volume-brush/🦀️.rs:20-33`, `format!("puzzle3d-voxel-{axis}")`),
each dispatching `setVoxelDims{axis}`. Alt+click paint path: `World3dHost` `WorldVoxelGroundPlane`
`onPlace={handleVoxelPlace}` (confirmed present, gated `volumeBrushMode` at `World3dHost/🟦️.tsx:6043-6044`)
→ `addTargetVolume` (`🦀️.rs:3318` per A1).

**Probe:** activate `volumeBrush` utility, unfold `...utilityBar.unfold`, drag
`#puzzle3d-voxel-w` slider, assert `setVoxelDims` args `{axis:"w"}`; Alt+click on
`[data-surface-id]` inside the perspective pane, assert a new target-volume entity appears in
`data-instances-json`/outliner (§17).

## §11 Relocate utility — **DEAD at the host DOM layer** — corrects A1 (more severe than "extent cap")

A1 marked this OPEN-UNOWNED citing only the Nakagin extent-cap fault. **This pass found the drag
gesture the checklist assumes ("drag an object to a new absolute position") does not exist in the React
host at all**, independent of document size:

- `world-relocate/🦀️.rs` (13 lines) declares `UTILITY_ID = "worldRelocate"` and no options — clicking
  the utility button in the utility bar correctly activates it (`setActiveUtility`, confirmed generic
  mechanism, `🦀️.rs:6836-6838` doc comment + `set-active/🦀️.rs:7-17`).
- The guest command handler is real: `world-relocate/🦀️.rs:16-40` `world_relocate(ctx, args)` expects
  `{objectId, position:[x,y,z]}` and moves+attracts the object — same absolute-position shape as the
  target-volume analogue.
- **`grep -n "worldRelocate" World3dHost/🟦️.tsx` and every other file under
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer` returns zero matches outside JSON plugin
  manifests.** The one analogous mechanism that DOES exist —
  `WorldVolumeLayer`'s `onRelocate={handleTargetVolumeRelocate}` (`World3dHost/🟦️.tsx:6040-6045`),
  dispatching `relocateTargetVolume` — is gated `relocateActive={activeUtility === "transform"}`, i.e.
  it fires for dragging a **target volume** while the **Transform** (gumball) utility is active, not
  for dragging an **object** while **worldRelocate** is active. There is no `onRelocate`/
  `objectDrag*`/equivalent prop anywhere on the object-instance mesh layer
  (`WorldInstanceMesh`/the group above `WorldVortexMarkers`, `World3dHost/🟦️.tsx:5990-6006`), and no
  `activeUtility === "worldRelocate"` branch exists anywhere in the file.
- Net effect: a user can select "Relocate" in the utility bar, but **no pointer gesture in the
  viewport ever calls `dispatch("worldRelocate", ...)`** — the feature is unreachable by any UI action,
  on either example, regardless of the Nakagin extent-cap bug (which would only matter if the dispatch
  could ever fire).

**Probe:** activate `worldRelocate` utility (`#framework.window.puzzle3dMainTop.utilityBar.unfold` →
click the Relocate button), then attempt `page.mouse` drag on a selected object in the viewport;
assert **no** `worldRelocate` action leaves the page (hook `dispatch`/network) — confirming dead, not
merely faulting. This is a stronger, cheaper reproduction than trying to trigger it on Nakagin
specifically (Concrete Forest, 1 object, is sufficient to prove the gesture never fires at all).

## §14 Engagement bar (command-line HUD) — **WIRED**, but spans two separate DOM panes — corrects checklist framing

The checklist treats "engagement bar" as one surface; in the actual component tree it is **two
independent `Pane` overlays** with two independent fold-toggle ids (see cross-cutting section):

- **Engagement pane** (top-left, `#framework.window.<id>.engagement.toggle`) — for puzzle3d this
  carries ONLY the status readout: `main/🦀️.rs:620` `status: Some(vec![WindowEngagementStatus{id:
  "puzzle3d-world-status", text: "<n> objects · <n> attractions"}])`, rendered as
  `data-slot="engagement-status-item"` (react-package `:10447-10450`). `options: None`, `control:
  None`, `controls: None` (`main/🦀️.rs:605-624`) — **puzzle3d declares no engagement options/control at
  all**, so this pane is otherwise empty text. It also hosts the Action Pane (`{actionPane}`,
  `Window/🟦️.tsx:372`) — see §23/§24.
- **Search pane** (top-middle, `#framework.window.<id>.search.toggle`) — the actual typed command-line
  input. Guest: `main/🦀️.rs:608-619` `WindowEngagementInput{id:Some("puzzle3d-engagement"),
  on_change:"engagementInput", on_submit:"engagementSubmit", on_repeat_last:"engagementRepeatLast",
  on_abort:"engagementAbort"}` → host `windowEngagementToSearchSpec` (`ShellHelpers/🟦️.tsx:1689-1712`)
  → `Search` component's `Input` renders `id={input.id}` = **`puzzle3d-engagement`** (literal, react-
  package `:10305`, since the id is neither empty/`"search-input"`/internal-chrome). Enter →
  `onSubmit`; Escape → `onAbort`; **Space, only when the field is idle/empty and outside session,
  triggers `onRepeatLast`** (`applySearchSpaceAction`, react-package `:9967-9980`) — not a button, not
  a generic "repeat" affordance; the checklist's "Repeat last" row undersells how narrow this trigger
  is.
- **`engagementControlSelect` is not wired through the generic `control`/`controls` field at all**
  (both `None` for puzzle3d). It is dispatched from the **Brush utility's** candidate-picker toggle
  group `on_change` (`🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs:58`,
  `puzzle3d_action("engagementControlSelect", None)`), which lives in the **utility bar's** options
  rail (§9's brush candidate picker), a third DOM location entirely. A probe hunting for "the
  engagement control" in the Engagement pane will find nothing; it must look at the Brush utility
  options instead.

**Probe:** unfold `#framework.window.puzzle3dMainTop.search.toggle`, type into
`#puzzle3d-engagement`, press Enter (submit), Escape (abort). Separately unfold
`...engagement.toggle` and assert `[data-slot="engagement-status-item"]` text matches
`/\d+ .* · \d+ .*/` (object/attraction counts). For control-select, activate Brush utility and use
its own toggle group in the utility-options rail, not the Search/Engagement panes.

## §15 Context menu (right-click) — **WIRED**, prior "zoomToSelection" defect confirmed FIXED in current source

Chain: right-click in viewport → `World3dHost/🟦️.tsx:5854-5876` builds `openSurfaceContextMenu(...,
mapWorldContextMenuSpecs, shellContextMenuFallback)` → guest `puzzle3d_context_menu_items`
(`🦀️.rs:2681-2751`, current lines, drifted from both prior audits' `2347-2419`/`2512-2568` citations)
→ rows via `puzzle3d_context_menu_row` (`:2614-2621`, `id/label/icon/action/args` set directly, no
registry validation — checklist's claim holds) → `mapContextMenuSpecs`
(`World3dHost/🟦️.tsx:1314-1343`) sets `id: spec.id` verbatim on the rendered `ContextMenuItem` →
`ContextMenu/🟦️.tsx:430,442,501-502` renders `id={item.id}` `role="menuitem"`.

**Confirmed fixed (current source, not just A1's claim):** every "Zoom to Selection" row now reads
`puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None,
false)` at `🦀️.rs:2692` (objects), `:2713` (vortex), `:2748` (reference) — all dispatch the real,
registered `"focusSelection"` action id, not the phantom `"zoomToSelection"` the 09-09 checklist
caught. Group rows: `.group("hand", ...)` (objects, hide-show/lock-unlock, `:2694-2701`) and
`.group("targets", ...)` (target volume, `:2723-2740`) — both correctly negate current state
(`!all_hidden`/`!hidden` etc., matching the outliner fix in §17, not the outliner's former bug).

**Probe:** `page.locator('[role="menuitem"]#zoom')` after right-clicking a selected object/vortex/
reference; assert it does NOT fault (no `dispatch_action: no handler` console error) and the camera
frames the selection. Row ids (`duplicate`, `select-same-kind`, `zoom`, `hide-show`, `lock-unlock`,
`delete`, `suggest`) are **not namespaced** — reused across selection kinds — so a probe must select the
target kind first to disambiguate which menu is open.

## §17 Artifact / outliner panel — **WIRED**, prior hide/lock-inverse defect confirmed FIXED in current source

Chain: row click → `interactionSelect` (`📌️panels/🗿️artifact/🦀️.rs:108-121`, `select_action`) or
inline hide/lock icon → `setSelectionFlag` via `flag_args` (`:122-131`) used by `hide_lock_actions`
(`:132-146`) called from `object_row`/`reference_row`/`target_volume_row`
(`:149,166,174`, current lines).

**Confirmed fixed (current source):** `flag_args(entity, id, flag, value)` is now called with
`!hidden`/`!locked` at the call sites (`hide_lock_actions`, `:135,140`) — the docstring at `:126-130`
explicitly narrates the fix ("A hardcoded `true` here made 'Show'/'Unlock' re-apply the state the row
was already in"). Matches A1; this pass additionally traced the row's own DOM id source:
`selectable_item`'s `try_id(id.as_ref())` (`:112-113`) uses the raw entity id (`object.id`,
`puzzle3d_vortex_full_id(...)`, etc.) as the tree-row id — **not namespaced under
`puzzle3d-play-document`** at the Rust layer; namespacing (if any) happens in the generic
`PanelTreeBuilder`/`Tree` renderer, not verified this pass.

**Probe:** hide an object via its outliner row's eye icon, then click the SAME row's icon again
(now "Show"); assert `hidden` flips back to `false` (previously stuck `true`). Row action icon has
`title`/`aria-label` from `labels.show`/`labels.hide` — usable as an accessible-name locator alongside
the row's own tree-item id.

## §18 Catalogue panel — **WIRED** end-to-end, including drag-drop

Chain (click path): catalogue row → `addObjectKind` action (checklist accurate).
Chain (drag path, newly traced): `Catalogue`/`Tree/🟦️.tsx:854,871-887` renders rows with
`draggable:true, dragData:{[mime]: JSON.stringify(payload)}` where `mime =
"application/x-semio-catalogue-item"` (`Tree/🟦️.tsx:854`) — **matches the guest constant exactly**,
`📌️panels/🛍️catalogue/🦀️.rs:24` `PUZZLE3D_CATALOGUE_DRAG_MIME = "application/x-semio-catalogue-item"`.
Drop target: `World3dHost/🟦️.tsx:5880-5883` `onDragEnter/onDragOver/onDrop` on the host div →
`onCatalogueDrop` (`:5760-5768`) reads `event.dataTransfer.getData(CATALOGUE_DRAG_MIME)` (with an
in-memory `getActiveCatalogueDragPayload()` fallback since `dragover` can't read `dataTransfer` until
drop, `Tree/🟦️.tsx:891-897`) → `commitCatalogueDropAt` (`:5705-5716`) → `dispatch("addObjectKind",
{objectKind, origin})`.

**Probe caveat:** native HTML5 drag-and-drop cannot be driven by plain `page.mouse` moves in
Playwright/Chromium (no real OS drag). Either dispatch synthetic `dragstart`/`dragover`/`drop`
`DragEvent`s with a constructed `DataTransfer` via `page.evaluate`, or exercise only the simpler
click-to-add path (`addObjectKind` from a row click) and leave true drag-drop as a manual QA item — the
guest→host wiring is proven real either way.

## §19 Settings panel — **WIRED**, DOM id needs a `.control` suffix not in the checklist

Chain: `settings_panel/🦀️.rs:26-38` `stepper_field(id, ...)` builds a `ui::field` with `try_id(id)`
wrapping a **child** `BuiltNode` whose own id is `format!("{id}.control")` carrying the actual
`Component::NumberStepper` (`:28-29`). So the field id (`puzzle3d-play-settings.overlap-budget` etc.,
matches checklist) is the **row/label container**, not the interactive control — the real
Playwright-clickable stepper is `#puzzle3d-play-settings.overlap-budget.control` (and the `.proximity-
radius`/`.chunk-size`/`.grid-spacing` siblings). Dispatch: `ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID)
.action(action, None)` (`:27`) — **no `windowId` in args**, unlike the per-window measures rail's
`taggedOnAction`. `set_chunk_size`/etc. mutate `ctx.scene.runtime.*` directly
(`set-chunk-size/🦀️.rs:6-9`), which resolves to whichever window the host's ambient `view.windowId`
names as current (host-tracked focus), not an explicit selection — this is the mechanism behind A1's
flagged "verify in a split-window test" ambiguity: Settings edits the **last-focused** window's config,
invisibly, with no per-window UI cue in the Settings panel itself.

**Probe:** focus Top pane (click into its `[data-surface-id]`), open Settings tab, change
`#puzzle3d-play-settings.grid-spacing.control`; assert Top's grid measure
(`#puzzle3d-play-grid-spacing` in Top's own measures rail) changed and Perspective's did not; then
repeat after focusing Perspective and confirm the opposite.

## §22 Delete / duplicate / focus selection — **WIRED**, but only via a non-obvious framework fallback

This pass initially suspected these hotkeys were dead: `main::definition()`'s `WindowKindDefinition.actions
= Vec::new()` (`main/🦀️.rs:52`), and `grep -rl ".window_kind_actions(" ✏️s` returns **zero hits across
every plugin in the repo** — meanwhile `ShellHost/🟦️.tsx:7716` builds the keybinding dispatch gate's
`actionById` strictly from `session.app.windowKinds.find(kind).actions`, `continue`-ing past any
binding whose action isn't in that map (`:7735-7738`). Tracing further into the SDK
(`🔌️plugin/🦀️.rs:5155-5661`, `try_build_definition`) found the resolving mechanism: an
**unowned-action fallback** (`:5334-5337`, relative to `try_build_definition`'s start) —
```
for action in &actions {
    if !explicitly_owned_action_ids.contains(&action.id) { window.actions.push(action.clone()); }
}
```
— every app-level action NOT claimed by `.window_kind_actions()`/`.window_kind_action_refs()` on ANY
window kind is auto-copied onto **every** window kind at build time. Since puzzle3d has exactly one
window kind and never calls `.window_kind_actions()`, this fallback silently populates
`puzzle3d-main`'s `actions` with everything declared via `.action_with()`/`.mutation()`/`.view_action()`/
`.shell_action()` in `create_puzzle3d_app` (`🦀️.rs:8170-8232` region) — including `deleteSelection`
(`:~8175`), `duplicateSelection`, `focusSelection` (`.view_action`, `:8202`), so **Delete/Backspace/
Ctrl+D/F are real**, just via a mechanism the checklist and A1 never mention. Also confirmed:
`ShellHost/🟦️.tsx:5943-5947` runs an active `undeclaredActionDiagnostic` gate
(`ShellHelpers/🟦️.tsx:318-327`) on every dispatch that DOES `console.error` (not silently) when an
action isn't found this way — so a probe/dev checking devtools console would already see any real gap
of this class.

**Probe:** select an object, press `F` (no modifier — matches `.keybinding("f", "focusSelection")`,
`🦀️.rs:8165`), assert camera reframes; `Delete`/`Backspace` remove selection; `Cmd/Ctrl+D` duplicates
with reselect (`duplicate_selection_reselects_the_created_clones`, cited by A1). All should fire with
**zero console errors** — any `semio: app "puzzle3d" dropped action "..."` line is the fallback failing
and should be treated as a regression, not expected behavior.

## §23 Add Object dialog — **WIRED**, DOM location clarified

Chain: `.shell_action("openAddObjectDialog", ...)` (`🦀️.rs:8197`, category unset i.e. default) →
surfaces via the generic **Action Pane** (`windowActionPaneNode`, `ShellHost/🟦️.tsx:9216` etc.) which
renders **inside the same top-left pane as Engagement** (`Window/🟦️.tsx:363-372`,
`{engagement}{actionPane}` both children of the `id={childElementId(..., "engagement")}` Pane) — so
"Add Object…" is reached by the **same toggle** as §14's status readout:
`#framework.window.puzzle3dMainTop.engagement.toggle`, not a separate button. Handler:
`🦀️.rs:3344,3574-3575` emits `Effect::OpenDialog{dialog_id:"addObject", ...}` → resolved by
`resolveDialogDefinition`/`session.app.dialogs` in ShellHost (checklist accurate on this leg). Kind
select confirmed dynamic (A1: `puzzle3d_object_kind_options()`, `🦀️.rs:7634`-region, law
`the_add_object_dialog_offers_every_object_kind_of_both_examples` — the 09-09 checklist's "hardcoded
single option" claim is stale/wrong against current source).

**Probe:** unfold `#framework.window.puzzle3dMainTop.engagement.toggle`, find the button whose
accessible name matches the localized `"Add {kind}…"`/`"Objekt hinzufügen…"` label
(`🦀️.rs:8197` phrase), click it, assert the dialog opens with a kind `<select>` offering more than one
option on both examples.

## §24 Import / export — **WIRED and functional** — corrects BOTH the 09-09 checklist and A1's reverification

Both prior audits concluded "import/export does not exist as a usable end-user feature" / "no UI path
to load arbitrary fixture JSON," reasoning from `setFixtureJson` (genuinely dead, `BatchOnlyPendingRewrite`,
and separately capped at `PUZZLE_COMMAND_RAW_BYTES`=8192 bytes — that finding is correct but is not the
real import mechanism). **Current source has three real, wired actions neither prior audit's grep
apparently caught:**

- `exportFixture` — `.action_with(ActionDefinition::bounded_catalog("exportFixture", ..., ActionKind::
  Shell).category("file"))` (`🦀️.rs:8171`) → handler `📤️export-fixture/🦀️.rs:8-16`
  `export_fixture(ctx)` serializes `ctx.scene.fixture` to JSON and pushes
  `Effect::DownloadMediaExport{filename:"puzzle-3d.json", mime_type:"application/json", data, ...}` →
  host `ShellHost/🟦️.tsx:4921-4931` handles `"downloadMediaExport" in effect` generically and calls
  `downloadMediaExport(filename, mimeType, data, encoding)` — a real browser file download, the same
  primitive already proven for tutorial-recording export (`:6485`) and theme export (`:7398`).
- `openImportFixture` — `.action_with(...category("file"))` (`🦀️.rs:8173`) is **specially intercepted**
  in `ShellHost/🟦️.tsx:5961-5968`: `void requestFileOpen("application/json,.json", "text", false)` opens
  a native OS file picker, then dispatches `importFixture` with `{payload: contents, name}`.
- `importFixture` — `.action_with(...ActionKind::Mutation).in_palette(false)` (`🦀️.rs:8172`, hidden from
  command palette on purpose, only reachable via the picker flow above) → handler
  `📥️import-fixture/🦀️.rs:9-32` `import_fixture(ctx, args)` parses `args.payload` (or `.json`/
  `.fixture` as fallbacks) into a `Puzzle3dFixture` via `FromValue`, replaces `ctx.scene.fixture`
  wholesale, `ctx.notice(...import_invalid...)` + `ctx.abort=true` on any parse failure (real error
  path, not silent).
- The `PUZZLE_COMMAND_RAW_BYTES` 8 KiB cap that dooms `setFixtureJson` on Nakagin's 128,755-byte DSL
  does **not** apply here: `importFixture` is absent from the precompute retained-command dispatch list
  (`🦀️.rs:7600`, which enumerates exactly `cycleBrushCandidate|cycleBrushCandidateBack|
  cancelFillBuild|fillBuildTick|registerBrushMesh|setFillCount|suggestionsTick`) — it's a normal
  action dispatch through the much larger generic `UiValue`/action-args arena, not the narrow retained-
  command wire. A round-trip export→import of Nakagin itself should work; this was not fully load-
  tested this pass (worth a browser confirmation) but the wire-cap objection specifically does not
  apply.
- A ticket-`🗑️generated` artifact from a previous pass, `import-file-picker-laws-cargo.txt`, is already
  present, consistent with someone else having independently found and started validating this same
  path — corroborating, not contradicting, this finding.

**Probe:** unfold `#framework.window.puzzle3dMainTop.engagement.toggle`, click "Export" (accessible
name from the native EN/DE label at `🦀️.rs:8171`), assert a `puzzle-3d.json` download fires (Playwright
`page.waitForEvent("download")`). Click "Import…", handle the native file-chooser
(`page.waitForEvent("filechooser")`) with a previously-exported file, assert the document round-trips
(object/attraction counts in the engagement status line, §14, unchanged).

## §25 Locale / terminology — **WIRED** (framework-level, not puzzle3d-specific)

`uiLocale` is a **shell-wide** preference (`ShellHost/🟦️.tsx:1977` destructured from
`shellState.uiPrefs`), set via `dispatch({type:"SET_UI_LOCALE", value})` (`:1993`, restored from
persisted locks/prefs on boot) — not a puzzle3d control. It threads into every label resolver puzzle3d
uses (`Puzzle3dLabels`/`app_labels!`, per A1's citation of `🧪️:1296-1314` `set_label_axes(De, Reuse)`).
This pass did not locate the actual locale-switch UI control (out of scope depth for this audit's time
budget — likely a generic OS/shell preferences surface, not under the puzzle3d editor crate at all) —
**flag as residual**: a probe needs to find wherever `SET_UI_LOCALE` is dispatched from (grep
`SET_UI_LOCALE` dispatch sites outside ShellHost's own reducer) before it can drive this section.

**Probe (partial):** once the locale switch is located and set to `de`, read the outliner panel
(§17, `puzzle3d-play-document`) section headers and assert they read the German terms (e.g.
`"Baukomponenten"` per the failing test A1 cites) rather than English.

---

## Ranked defects (user impact order)

1. **`worldRelocate` has zero host-side gesture wiring (§11)** — worse than A1's Nakagin-extent-cap
   framing: the Relocate utility is reachable and activatable but produces NO interactive behavior on
   EITHER example, always, not just on large documents. Highest-impact new finding this pass.
2. **Import/Export is real and was wrongly written off twice (§24)** — inverts prior guidance from
   "don't bother testing this, it doesn't exist" to "this is a real feature that needs its first
   browser confirmation." High impact because it changes what the team should even attempt to verify.
3. **Window-instance DOM ids are camelCase, not literal kebab-case (§1, cross-cutting)** — every
   selector a probe author copies verbatim from the original checklist for a window container or a
   fold-toggle will silently miss (`querySelector` returns null) unless corrected. Blocks writing ANY
   new probe for §1–§4/§9–§11/§14/§23/§24 until known.
4. **Camera pose has no DOM exposure (§2)** — blocks writing a real orbit/pan/zoom assertion without
   either instrumenting `dispatch` or falling back to screenshot diffing.
5. **Engagement bar is two panes, not one, and `engagementControlSelect` lives in a third place
   entirely (§14)** — a probe author following the checklist's "engagement bar" framing literally will
   look for the typed input in the wrong pane and never find the control-select at all.
6. **Settings panel silently edits the last-focused window with no visible scoping (§19)** — not a
   bug, but a real UX trap once WindowConfig is confirmed working; worth a product decision, not just a
   probe.
7. **Native HTML5 drag-and-drop (§18) is real but not Playwright-drivable by plain mouse gestures** —
   a probe author who tries `page.mouse` drag for the catalogue and gets nothing may misdiagnose it as
   broken when it's a tooling limitation, not a product defect.

## Probe step specs

- **§1**: assert `elementIdSelector("framework.window.puzzle3d-main-top")` and `...perspective` resolve
  on boot; no interaction needed.
- **§2**: unfold measures, drag-orbit perspective canvas, assert no hang; camera pose only verifiable
  via dispatch interception or screenshot diff (no DOM hook).
- **§3**: unfold measures, toggle the projection `Select`, assert visual projection change
  (screenshot; same no-DOM-hook caveat as §2 for verifying via camera state).
- **§4**: unfold measures, click `#puzzle3d-play-grid-visible` / drag `#puzzle3d-play-lod-value` /
  select `#puzzle3d-play-vortex-show` — ids are literal, no transform needed.
- **§10**: activate `volumeBrush` utility, unfold utility bar, drag `#puzzle3d-voxel-w/d/h`; Alt+click
  viewport, assert new target volume.
- **§11**: activate `worldRelocate` utility, drag a selected object; assert **no** `worldRelocate`
  dispatch ever fires (proves dead, cheaper than reproducing the Nakagin-only extent fault).
- **§14**: unfold `...search.toggle`, type/submit in `#puzzle3d-engagement`; unfold
  `...engagement.toggle` separately for the status line; control-select lives under the Brush utility,
  not either pane.
- **§15**: right-click a selection, `page.locator('[role="menuitem"]#zoom')`, assert `focusSelection`
  fires without a dispatch fault.
- **§17**: toggle an outliner row's hide icon twice, assert the flag round-trips (was stuck `true`,
  confirmed fixed in current source).
- **§18**: click-to-add is Playwright-trivial (`addObjectKind`); true drag-drop needs synthetic
  `DragEvent`+`DataTransfer` construction via `page.evaluate`.
- **§19**: focus one pane, edit `#puzzle3d-play-settings.<field>.control` (note the `.control` suffix),
  assert only the focused pane's own measure changed.
- **§22**: `F`/`Delete`/`Backspace`/`Cmd+D` on a selection; assert zero
  `semio: app "puzzle3d" dropped action` console errors (the fallback-mechanism regression signature).
- **§23**: unfold `...engagement.toggle`, click "Add Object…" (by accessible name), assert dialog opens
  with a multi-option kind select.
- **§24**: unfold `...engagement.toggle`, click "Export" and assert a `download` event; click
  "Import…" and handle the `filechooser` event with a prior export.
- **§25**: locate the `SET_UI_LOCALE` dispatch site (not found this pass), switch to `de`, assert
  outliner section headers read German terms.
