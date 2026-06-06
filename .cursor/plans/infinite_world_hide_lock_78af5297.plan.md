---
name: Infinite World Hide Lock
overview: Add a shared persisted hide/lock mechanism to @infinite/world/r3f, consume it in CAD and puzzle 3d (hidden = unrendered but revealed-on-tree-hover as hovered; locked = dimmed + non-interactive), and expose it through the shared Tree UI with per-row toggles and a selection context menu.
todos:
  - id: ticket
    content: Open/reopen repo MCP ticket; read repo://goals and associate with the most fitting goal.
    status: completed
  - id: engine
    content: Add WorldEntityFlags type + pure helpers (selectable/rendered/renderMode) + dim constant to infinite/world/r3f, with tests.
    status: completed
  - id: persist-puzzle
    content: Add hidden/locked to puzzle ObjectProps/VortexProps/AttractionProps; parse/serialize in parseFixtureV1; thread through fingerprints, ObjectRecord, fixtureToRecords.
    status: completed
  - id: persist-cad
    content: Store per-entity hidden/locked in CAD Model.metadata via AttributeStore helpers (covers object + vertex/edge/face).
    status: completed
  - id: render-puzzle
    content: "Wire puzzle ObjectItem/Vortex/CableBatch: hide unless revealed-on-hover, dim+non-interactive when locked; prune selection; exclude from marquee/select-all."
    status: completed
  - id: render-cad
    content: Wire CAD pick-target flag filtering (selectable + render), locked dim in targetStyle, committed-mesh hide/dim, selection guards + pruning.
    status: completed
  - id: icons
    content: Add lock-open to VENDORED_ICON_IDS and regenerate ui assets.
    status: completed
  - id: ui-tree
    content: Add tree row hide/lock toggle actions + selection context menu to ui/react Tree; muted hidden-row styling.
    status: completed
  - id: platform-plumb
    content: Add actions/contextMenu to UiTreeItemNode and map through platform + playground tree renderers.
    status: completed
  - id: wire-hierarchies
    content: Attach actions/context menu in puzzle and CAD hierarchy builders; add toggleHidden/toggleLocked controller/chrome commands.
    status: completed
  - id: validate
    content: Extend in-file tests; run world/puzzle/cad/ui tests; smoke both plays with [DEBUG] logs confirming hide/reveal/lock + context menu.
    status: completed
isProject: false
---

## Decisions (from clarifications)
- Persisted in the scene documents (puzzle `FixtureV1`, CAD `Model.metadata`).
- Applies to all selectable entity kinds (CAD vertex/edge/face/object; puzzle object/vortex/attraction).
- `hidden` = not rendered, but revealed in 3D as a hovered piece while its tree row is hovered.
- `locked` = rendered dimmed/greyed AND not selectable/hoverable/editable.

## Workflow
- Open a ticket via repo MCP first (read `repo://goals`, associate with the puzzle3d/world goal used by the closed unify ticket `🎯puzzle3d🎯puzzle3dplay`, or a more fitting one). Keep temp files under the ticket folder. Add code into existing files using `//#region` blocks. Extend existing test regions only. Close ticket with summary + touched files when done.

## 1. Shared mechanism in `@infinite/world/r3f`
In [infinite/world/r3f/index.tsx](infinite/world/r3f/index.tsx) add a new `//#region 👁️EntityFlags`:
- `export interface WorldEntityFlags { readonly hidden?: boolean; readonly locked?: boolean }`
- `export const WORLD_LOCKED_OPACITY_SCALE = 0.35` (dim factor) and a desaturation hint.
- Pure helpers (consumed by both apps, keyed by the app's existing stable entity key string):
  - `worldEntitySelectable(flags)` -> `!flags?.hidden && !flags?.locked`
  - `worldEntityRendered(flags, revealed)` -> `!flags?.hidden || revealed`
  - `worldEntityRenderMode(flags, { hovered, selected, revealed })` -> `{ asHover: boolean; dim: boolean }` (hidden+revealed => render as hover; locked => dim, never selected outline).
- Add assertions for these helpers in the file's existing tests region.

Rationale: the engine stays the generic layer; storage stays per-doc; reveal-on-hover reuses each app's existing hover-sync store (CAD `AppPointerFocusStore`, puzzle `RegistryHover`/`hoverFocus`).

## 2. Persistence

### Puzzle 3d ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx))
- Add `hidden?`/`locked?` to `ObjectProps` (293), `VortexProps` (278), `AttractionProps` (321).
- `parseFixtureV1` (~1413): read + serialize the new fields for objects, vortices, attractions.
- Include flags in `fixtureAppearanceFingerprint` so `syncAppearanceFromFixture` picks up changes.
- Carry flags into `ObjectRecord` (2063) via `fixtureToRecords` (2090); attractions already pass through the store.
- Thread flags `ObjectItemById` (2806) -> `ObjectItem`/`Vortex`; `Attractions`/`CableBatch` for attraction flags.
- Rust [puzzle/3d/rs/lib.rs](puzzle/3d/rs/lib.rs): no change — serde ignores unknown keys; collision/brush intentionally unaffected by visibility (note this decision in the ticket).

### CAD ([cad/js/core/index.ts](cad/js/core/index.ts))
- Store per-entity flags in `Model.metadata` (`AttributeStore`, already round-trips in `ModelJson.metadata`), keyed by entity id, fields `{ hidden, locked }`. This uniformly covers objects AND sub-entities (vertex/edge/face) with no schema churn.
- Add `AttributeStore`/`Model` helpers: `getEntityFlags(id)`, `setEntityFlag(id, flag, value)`, bumping `revision`.
- No change required to [cad/schema/json/model.json](cad/schema/json/model.json) (metadata is free-form), but document the convention.

## 3. Rendering wiring

### Puzzle 3d
- `ObjectItem` group `visible` (5945): `worldEntityRendered(flags, objectPointerHovered)`.
- When hidden+hovered -> force `resolveMeshStyle({ hovered: true })`; when locked -> dim via `worldEntityRenderMode` (reduce material opacity, suppress `Outlines`).
- Pointer handlers (`onPointerOver`/selection commit): bail when `!worldEntitySelectable(flags)` (locked/hidden not pickable in canvas; hidden only reachable via tree hover).
- Same pattern for `Vortex` (6250) and attraction `CableBatch` (6288): hide unless revealed, dim when locked.
- Selection: exclude locked/hidden from marquee + select-all (`filterSelectionByPlaygroundKinds` neighborhood in [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts)); prune selection when an entity becomes hidden/locked.

### CAD ([cad/js/renderer/index.tsx](cad/js/renderer/index.tsx))
- New `filterSpatialPickTargetsForEntityFlags(targets, flags)`; apply when building `selectablePickTargets` (~4327) so hidden/locked are not ray-pickable/hoverable.
- `resolveSpatialPickTargetsToRender` (2374): skip hidden targets unless the key is the current hover key (revealed) or pinned selection.
- `targetStyle` (2308): add a `locked` branch (dim opacity via `WORLD_LOCKED_OPACITY_SCALE`, desaturated, thin line, no select emphasis).
- `CommittedMeshLayer`/`resolveSpatialSceneVisibility` (796): hide faces/edges for hidden solids/objects; dim for locked.
- Reject locked targets in `dispatchSelectionTargets`/`selectHierarchyTarget`; prune selection on hide/lock.

## 4. UI

### Shared Tree ([ui/react/index.tsx](ui/react/index.tsx))
- Per-row toggles: reuse `TreeDataItem.actions` (`TreeSectionAction` button kind, 8177) with `eye`/`eye-off` and `lock`/`lock-open` icons. Show persisted-active toggles always; reveal the rest on `group-hover`.
- Add right-click support to tree rows: optional `TreeDataItem.contextMenu?: ContextMenuItem[]` (or builder); wrap the row body in the existing `ContextMenu` (644). Menu acts on selection (if the row is selected, apply to all selected ids; else select then apply).
- Muted styling for hidden rows (extend `treeRowStateClasses`, 8705).
- Hover reveal needs no new code: existing `onPointerEnter` -> hover store -> renderer already drives the 3D reveal once step 3 renders hidden-but-hovered as hovered.

### Icons ([ui/assets/script.ts](ui/assets/script.ts))
- Add `"lock-open"` to `VENDORED_ICON_IDS` (151) and regenerate via the `build ui assets` launch entry (`eye`/`eye-off`/`lock` already vendored).

### Platform plumbing ([framework/product/platform/core/index.ts](framework/product/platform/core/index.ts))
- Add `actions?` and `contextMenu?` to `UiTreeItemNode` (140); map them in `uiTreeItemsToTreeData` and `uiTreeNodeToTreePanelConfig` ([framework/product/platform/renderer/react/index.tsx](framework/product/platform/renderer/react/index.tsx), [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)).

### Per-app hierarchy wiring
- Puzzle [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) `buildPuzzle3dPlayHierarchySections` (1228): attach `actions` + `contextMenu` per row; add controller commands `puzzle3dPlayToggleHidden/Locked` extending `patchPuzzle3d*`/`updatePuzzle3d*InFixture`.
- CAD [cad/js/renderer/play/index.tsx](cad/js/renderer/play/index.tsx) `buildCadPlayHierarchySections`/`cadPlayPrimitiveChildTreeItem` (356/303): attach `actions` + `contextMenu`; add chrome callbacks `toggleHidden/toggleLocked` writing `Model.metadata` + bumping revision; keep highlight registration so locked entities still cross-highlight.

## 5. Tests + validation
- Extend in-file test regions: `@infinite/world/r3f` (helpers), puzzle/3d/react (fixture parse round-trip of flags; render visibility/selectable), cad renderer (pick-target flag filtering; locked style; metadata round-trip), ui/react (tree actions + context menu render).
- Run world/puzzle/cad/ui tests via existing launch.json test entries (no new entries expected).
- Smoke both plays with `[DEBUG]` console logs: hide an object (disappears, tree row muted), hover its tree row (reappears as hovered piece), lock an entity (dimmed, cannot select in 3D), right-click selection -> hide/lock all. Confirm runtime behavior in console before claiming done.

## Notes / risks
- CAD sub-entity flags rely on stable kernel ids in `geometry`; verify ids persist across load (`materializeInlineObjectPrimitives`).
- Keep `pinnedPickTargetKeys` solid<->object bridging intact when filtering.
- Greenfield: puzzle's existing per-object `visible?` becomes redundant with `hidden`; fold/remove rather than keep both (no back-compat).
