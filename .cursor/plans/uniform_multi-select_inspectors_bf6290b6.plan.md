---
name: Uniform multi-select inspectors
overview: "Bring every technology's playground Inspection tab up to one consistent standard: batch-editable fields for the entire current selection (not just the first/only selected item), with field groups ordered most-specific (kind-specific) first and most-general (shared/base) last — mirroring the pattern `draw` and `puzzle/2d` already use."
todos:
  - id: phase0-shared-helper
    content: Add shared uiInspectorAllEqual + Mixed-value convention to framework/product/platform/core/index.ts
    status: completed
  - id: phase1-draw-raster-forms
    content: Upgrade draw, raster, forms inspectors to batch multi-select editing
    status: in_progress
  - id: phase2-flow-dag-map-presentation
    content: Upgrade flow, dag, gis/map, presentation inspectors from reject-multi to batch editing + grouped ordering
    status: pending
  - id: phase3-shooting
    content: Generalize shooting's selection model to sets of shot/asset ids with batch patch commands
    status: pending
  - id: phase4-trinity
    content: Make trinity's shared jack/rewrite inspector editable and batch-selectable
    status: pending
  - id: phase5-puzzle-gaps
    content: Close multi-edit gaps in puzzle/3d (vortices/attractions) and puzzle/5d (grips)
    status: pending
  - id: phase6-cad
    content: Extend CAD batch inspector fields beyond typology/hidden/locked
    status: pending
  - id: phase7-sketchpad
    content: Add sketchpad connection batch editing and type parent-chain inheritance ordering
    status: pending
  - id: phase8-semios
    content: Add a net-new Inspection tab to the Semios shell for media-graph nodes and app instances
    status: pending
  - id: phase9-verify
    content: Verify procedural/mindmap delegated fixes and confirm writer stays document-level
    status: pending
isProject: false
---

# Uniform Multi-Select Inspectors Across All Technologies

## Current state (from audit)

Every playground already exposes an "Inspection" tab (`FRAMEWORK_PANEL_TAB_INSPECTION_LABEL` in [framework/core/index.ts](framework/core/index.ts)) built via `build*InspectorTree`/`build*DetailsBody` functions, wired through `PureSidePanelTabDefinition` in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx). The shared helper `uiInspectorGroupsToTree` in [framework/product/platform/core/index.ts](framework/product/platform/core/index.ts:513-538) already documents the target contract ("ordered most-specific first, most-general last"), but only `draw` and `raster` actually use it. No technology has a shared "batch edit + Mixed value" helper — each place that supports it (`puzzle/2d`, `puzzle/3d`, `puzzle/5d`, `cad`, `sketchpad`) reimplements its own `allEqual`.

Maturity tiers found:
- **Full multi-edit reference**: `puzzle/2d` (`patchInspectorNodes`/`Handles`/`Edges` in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx:4391)).
- **Partial multi-edit**: `puzzle/3d` (objects batch, vortices/attractions read-only on multi), `puzzle/5d` (parts batch, grips single-only), `cad` (only typology/hidden/locked batch), `sketchpad` (pieces batch, connections read-only, no type-inheritance display).
- **Single-id-only ("first selected wins")**: `draw`, `raster`, `forms`.
- **Reject multi-select outright**: `flow`, `mathematical/graph/dag`, `gis/map`, `presentation`.
- **Single id+kind only**: `shooting`.
- **Exactly-one, fully read-only**: `trinity` (shared by `jack` + `rewrite`).
- **No Inspection tab at all**: `semios` shell.
- **Delegates to flow / puzzle-2d (fixed transitively)**: `procedural/2d`, `procedural/3d` (→ flow); `reasoning/mindmap`, `reasoning/mindmap/wires` (→ puzzle/2d).
- **Document-level, not entity-selection (no change needed)**: `writer` (Inspection shows document id/lang/uri, not a selectable-entity property set).

## Phase 0 — Shared infrastructure (do first, everyone depends on it)

In [framework/product/platform/core/index.ts](framework/product/platform/core/index.ts), next to `UiInspectorFieldGroup`/`uiInspectorGroupsToTree`:
- Add a single exported `uiInspectorAllEqual<T>(values: readonly T[]): boolean` helper (dedupe the four copies found in `puzzle/2d`, `puzzle/3d`, `puzzle/5d`, `sketchpad`).
- Add a small `uiInspectorMixedValue` convention doc (placeholder `"Mixed"` for text/select, `Number.NaN` + `uniform: false` for `numberStepper`/`vec3`) so every technology binds controls the same way instead of inventing its own.
- Keep `uiInspectorGroupsToTree` as the canonical way to render specific→general ordering; every technology below must route through it (not raw `uiDeclarativeSectionsToTree`).

## Phase 1 — Upgrade "first-id-only" technologies to batch edit

These already use `UiInspectorFieldGroup`/kind-specific group ordering (draw, raster) or need it added (forms):

- **`draw`** ([draw/play/index.ts](draw/play/index.ts:687)): replace `selectedIds[0]` lookup with mapping all `selectedIds` to layers; extend `drawPlayCmd("patchLayer", { layerId, field })` to `patchLayers({ layerIds, field, value })`; every `drawPlayInspector*Group` helper takes the layer array and uses `uiInspectorAllEqual` for Mixed placeholders. Keep existing group order (kind-specific → position → orientation → appearance → layer).
- **`raster`** ([raster/play/index.ts](raster/play/index.ts:488)): same treatment — `patchLayers` batch command, Mixed placeholders in pixel/adjustment/mask/layer groups, keep existing order.
- **`forms`** ([forms/play/index.ts](forms/play/index.ts:526)): refactor `formsPlayInspectorFields` off the flat `uiDeclarativeSectionsToTree` onto `UiInspectorFieldGroup[]` (kind-specific group first, base `label/kind/description/required` group last); add batch `patchQuestions({ questionIds, field, value })`.

## Phase 2 — Upgrade "reject-multi" technologies to batch edit + grouped ordering

All four currently early-return a "N selected" message and use raw `uiDeclarativeSectionsToTree`:

- **`flow`** ([flow/play/index.ts](flow/play/index.ts:428)): remove the `selectedNodeIds.length > 1` bail-out (L450-458); classify selection by `widget.kind`, build one `UiInspectorFieldGroup` per present kind (specific fields) plus a base `{id, kind}` group (general), batch via `patchFlowWidgets({ widgetIds, field, value })`.
- **`mathematical/graph/dag`** ([mathematical/graph/port/directed/dag/play/index.ts](mathematical/graph/port/directed/dag/play/index.ts:263)): same pattern, `patchDagNodes({ nodeIds, field, value })`.
- **`gis/map`** ([gis/map/play/index.ts](gis/map/play/index.ts:381)): classify selection into `selectedPositionIds` vs `selectedRouteIds` (like puzzle/2d's node/handle/edge split); render a Positions group and/or Routes group, each ordered geometry (lat/lon or points) → identity (label/kind); batch `patchPositions`/`patchRoutes`.
- **`presentation`** ([framework/product/presentation/play/index.ts](framework/product/presentation/play/index.ts:589)): tiles are homogeneous (no kind axis) — straightforward batch `patchTileCrops({ ids, field, value })` + `renameTiles({ ids, value })`, Mixed placeholders on crop x/y/width/height and name.

## Phase 3 — Shooting: generalize selection model

[shooting/play/index.ts](shooting/play/index.ts:230) stores one `selectedId`/`selectedKind` pair. Change controller state to `selectedShotIds: readonly string[]` + `selectedAssetIds: readonly string[]` (hierarchy already supports multi-highlight elsewhere in the repo's pattern); inspector renders a Shots group (when any shot ids selected) and/or Assets group, each with Mixed-aware batch `patchShots({ shotIds, field, value })` / `patchAssets({ assetIds, field, value })`.

## Phase 4 — Trinity: make fields real and batch-editable

[trinity/react/index.tsx](trinity/react/index.tsx:194) (`buildTrinityPlayInspectorTree`, shared by `jack` and `rewrite`) is 100% read-only `tree`/`description` items and requires exactly one selection. Rework to:
- Accept `selectedNodeIds.length >= 1`.
- Convert to `uiDeclarativeSectionsToTree`/`uiInspectorGroupsToTree` with real `field`+`input` nodes: `name` becomes editable and batchable; `kind` stays read-only (structural); surface any settable entries from `node.properties` as editable fields ordered kind-specific (from the node's own properties) before the general identity group (`id`/`kind`/`ports` read-only).
- Add a batch `patchTrinityNodes({ nodeIds, field, value })` command to both `trinity/jack/play/index.ts` and `trinity/rewrite/play/index.ts` controllers (they already share `setSelection`; add the patch handler alongside it), committing through each controller's existing fixture-commit path.

## Phase 5 — Close multi-edit gaps in the puzzle family

- **`puzzle/3d`** ([puzzle/3d/play/index.ts](puzzle/3d/play/index.ts:3496)): promote vortices and attractions from read-only "Mixed" text (multi-select) to real batch commands (`patchPuzzle3dVortices`, `patchPuzzle3dAttractions`), mirroring the existing `patchPuzzle3dObjects` pattern exactly.
- **`puzzle/5d`** ([puzzle/5d/play/index.ts](puzzle/5d/play/index.ts:483)): promote grips from "Select a single grip to edit" to batch `patchPuzzle5dGrips`, same pattern as `patchPuzzle5dParts`.
- Both already use the correct `uiInspectorAllEqual`/Mixed pattern for the entities they do batch-edit — reuse Phase 0's shared helper here instead of the local copies.

## Phase 6 — CAD: extend batch fields beyond typology/hidden/locked

[cad/js/renderer/play/index.tsx](cad/js/renderer/play/index.tsx:3352) (`buildCadPlaySelectionInspectorChildren`) already has the right shape (shared batch fields first, per-target sections after) but only batches `typology`/`hidden`/`locked`. Extend `patchCadPlaySelection` to also batch the common transform/property fields available on every selected target's kind, keeping order: kind-specific properties (per `typology`) → shared transform → hidden/locked (general).

## Phase 7 — Sketchpad: connections batch edit + type-inheritance ordering

[compose/client/lib/sketchpad/js/index.ts](compose/client/lib/sketchpad/js/index.ts:14478) (`buildSketchpadInspectionPanelBody`):
- Connections currently show gap as read-only "Mixed" text with no `onChange` — add batch `patchRouteConnections({ connectionIds, field, value })` mirroring the existing `patchRoutePieces`.
- Type section: per `compose/AGENTS.md`'s `parent^{type}` model, walk the parent-type chain and render the selected type's own fields first, then a general "Inherited" sub-group per ancestor (most specific type first, root/proto type last) — this is the one place in the repo where "sorted by inheritance from specific to general superclasses" is literal, not just a UI convention.

## Phase 8 — Semios shell: add its own Inspection tab

[semios/play/index.ts](semios/play/index.ts) has no Inspection tab today (`SemiosPlayController` only tracks `activeInstanceId`, no selection state). Add:
- Selection state for media-graph nodes and app instances (`selectedMediaNodeIds`, `selectedAppInstanceIds`).
- A `buildSemiosPlayInspectorTree` following the same group pattern: Media Graph Node group (label/position, kind-specific per node's backing program) and/or App Instance group (label/position), general identity (`id`) last; batch `patchMediaNodes`/`patchAppInstances` commands.
- Register it as a `PureSidePanelTabDefinition` in the playground renderer alongside the other technologies' Inspection tabs, using `FRAMEWORK_PANEL_TAB_INSPECTION_LABEL`.

## Phase 9 — Verify transitive fixes and leave-as-is technologies

- `procedural/2d` / `procedural/3d`: confirm their `build*PlayInspectorTree` delegation to flow's builder automatically inherits Phase 2's batch editing (no separate code change expected, verify by reading their thin wrapper files).
- `reasoning/mindmap` / `reasoning/mindmap/wires`: confirm they re-export `puzzle/2d`'s `Playground2d` and inherit Phase 5/reference behavior automatically.
- `writer`: leave the document-level Inspection tab as-is — it reflects the open document, not a selectable-entity collection, so "complete selection at once" doesn't apply the same way. No code change planned here beyond noting the exception.

## Execution notes

- Work will happen inside a single repo-MCP ticket (goals list + `ticket_open` at execution start, since this is one coherent initiative spanning many files but one clear goal).
- Every technology's existing tests in its `*/play/index.ts` (or `*/react/index.tsx`) test blocks (`if (import.meta.vitest) { ... }`) will be extended in place (per workspace rules: no new test files) to cover: (a) batch-patching N>1 selected ids, (b) Mixed-value rendering when values differ, (c) group ordering (kind-specific groups appear before general groups in the returned tree).
- No new files will be created; all changes land inside existing `play`/`react` `index.ts(x)` files using region/subregion comments for organization, plus the one shared helper addition in `framework/product/platform/core/index.ts`.
