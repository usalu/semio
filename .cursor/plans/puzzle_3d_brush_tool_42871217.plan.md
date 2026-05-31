---
name: Puzzle 3D Brush Tool
overview: Add a "Brush" tool to puzzle 3d that previews and flushes a new, compatible, non-colliding object snapped onto a free vortex (coincident point, opposite direction), with Tab cycling and a right-click compatible-objects menu, committing both the object and its attraction when the cursor leaves the vortex.
todos:
  - id: catalog
    content: Extend ObjectKind with explicit template fields (meshUrl/meshByLod/scale/vortices) and populate placeable kinds in the nakagin fixture JSON meta.kindCatalogs.objects
    status: in_progress
  - id: logic
    content: "Add pure brush logic to react index: brushCompatibleCandidates, computeBrushPlacementPose, boxesIntersect, applyBrushPlacementToFixture (in a 🖌️Brush region)"
    status: pending
  - id: canvas
    content: Add brushActive/onBrushPlace CanvasProps, BrushWindowBridge gesture (proximity/enter-leave, Tab cycle, right-click menu), BrushPreview ghost, and BrushContextMenu overlay; gate orbit/marquee
    status: pending
  - id: play
    content: Add activeTool state, setActiveTool + addBrushObject commands, snapshot field, and Select/Brush toolbar toggles in the play controller
    status: pending
  - id: host
    content: Wire brushActive and onBrushPlace from the playground renderer host into PlayCanvas
    status: pending
  - id: tests
    content: Extend existing react + play vitest blocks to cover pose/compat/collision/fixture and the new play commands; verify runtime in the play app
    status: pending
isProject: false
---

# Puzzle 3D Brush Tool

## Decisions (confirmed)
- On commit: add the new object AND create the attraction between source and target vortices.
- Placeable templates are explicit kinds: extend the object-kind catalog (`kindCatalogs.objects`) with the geometry needed to instantiate (`meshUrl`, local `vortices` with `vortexKind`/`position`/`direction`/`radius`, optional `scale`). Objects carry only a kind id; nothing is inferred from existing instances.

## Domain recap (existing)
- Fixtures are CAD (Z-up). Object pose = `origin: Vec3` + `orientation?: Quat` + `scale`. Vortex has local `position` + `direction`. Attractions connect vortex full ids `objectId:vortexId` (`AttractionProps`, [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) ~203).
- `blockedVortexFullIds` already = vortices that are part of an attraction (`blockedVortexFullIdsFromAttractions` ~2069). "Vortex not part of an attraction" = NOT in this set.
- Compatibility via `vorticesAttractionCompatibleForDrag(attractingCtx, attractedCtx, kindCompatibility, kindCatalogs)` (~2142) and `kindCompatibility` from fixture `meta`.
- Registry exposes `getVortexWorld(fullId)`, `kindCatalogs`, `kindCompatibility`, `blockedVortexFullIds`, object groups (`RegistryValue` ~2514). CAD/Three conversion helpers exist (`cadVec3ToThree`, `cadQuatToThree`, ~2855). Object world AABBs via `Box3.setFromObject` (`boundsFromObjectGroups` ~2976).
- No existing tool-mode, pose alignment, collision test, context menu, or Tab handling — all new.

## 1. Catalog: explicit placeable templates
In [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) extend `ObjectKind` (~227) with optional template fields:
- `meshUrl?: string`, `meshByLod?: readonly LodMeshEntry[]`, `scale?: number | Vec3`
- `vortices?: readonly { vortexKind: string; position: Vec3; direction?: Vec3; radius?: number }[]`

Populate these for placeable kinds in [puzzle/3d/fixture/nakagin-capsule-tower.3d.json](puzzle/3d/fixture/nakagin-capsule-tower.3d.json) under `meta.kindCatalogs.objects` (mesh + local vortices/directions), reusing values already present on the matching instances but written explicitly into the catalog.

## 2. Pure logic (exported + unit-tested in the existing vitest block)
Add a `//#region 🖌️Brush` region with:
- `brushCompatibleCandidates(target, kindCatalogs, kindCompatibility)` → for target vortex kind, returns ordered candidate descriptors `{ kindId, sourceVortexIndex }` for every catalog object-kind vortex passing `vorticesAttractionCompatibleForDrag(targetCtx, candidateCtx, …)`.
- `computeBrushPlacementPose({ sourceLocalPosition, sourceLocalDirection, scale, targetWorldPositionCad, targetWorldDirectionCad })` → `{ origin: Vec3, orientation: Quat }` in CAD. Math: `R` = quaternion mapping `sourceLocalDirection` → `-targetWorldDirectionCad` (`setFromUnitVectors`); `origin = targetWorldPositionCad − R·(scale·sourceLocalPosition)`. Guarantees coincident vortex point and opposite direction.
- `boxesIntersect(a, b)` AABB overlap helper (with small tolerance) for collision filtering.
- `applyBrushPlacementToFixture(fixture, payload)` (mirrors `applyConnectToFixture` ~1766): append the new `FixtureObjectV1` (generated object id + generated vortex ids) to `objects` and the attraction to `attractions`; marks a structure change so `fixtureRevision` bumps and the ghost becomes a real object.

## 3. Canvas gesture + preview ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx))
New `CanvasProps` (~481): `brushActive?: boolean`, `onBrushPlace?: (payload: BrushPlacePayload) => void`. Templates read from existing `kindCatalogs`. Thread through `Inner`/`RegistryProvider` and `PlayCanvas` (~5427/5459) like other props.

`BrushWindowBridge` (modeled on `AttractionWindowBridge`/`MarqueeBridge` window listeners, ~4305/4334), active only when `brushActive`:
- pointermove: project registered vortices to screen (reuse `pickNearestScreenVortex` ~2738) to find the nearest FREE vortex (`!blockedVortexFullIds.has(fullId)`) within its projected radius. Track enter/leave of a target vortex.
- on enter: compute target world position/direction in CAD from the target object's store record (origin/orientation + vortex local pos/dir); build compatible candidates; pick the first non-colliding one (mount ghost → `Box3.setFromObject` vs existing object AABBs via `boxesIntersect`, auto-advancing past colliding candidates); set brush preview state.
- Tab keydown: `preventDefault()` + cycle to the next compatible (non-colliding) candidate; recompute preview.
- contextmenu while inside target: `preventDefault()` + open menu listing ALL compatible candidates; selecting sets the candidate.
- on leave (target lost) with an active preview: commit via `onBrushPlace`, then clear preview.

Rendering/UI:
- `BrushPreview` component mounted in `Inner`'s scene renders the candidate ghost (reuse `MeshBody`) at the computed pose (CAD→Three), translucent/highlighted style.
- `BrushContextMenu` HTML overlay in the `Canvas3D` wrapper div (the element already holding `onContextMenu` ~5543), positioned at the cursor.

Gating while `brushActive`: in the right-button orbit `onPointerDown` (~4149) and `onContextMenu` (~5543), when the cursor is inside a free vortex, suppress orbit and route to the brush menu; disable `MarqueeBridge` left-drag selection.

## 4. Play controller ([puzzle/3d/play/index.ts](puzzle/3d/play/index.ts))
- Add `activeTool: "select" | "brush"` (default `"select"`) field, expose on `Puzzle3dPlaySnapshot` (~1090).
- `run` command `setActiveTool` (~782 switch) + `addBrushObject` command that calls `patchFixture(f => applyBrushPlacementToFixture(f, payload))`.
- Toolbar: add a Select/Brush toggle group in `rebuildShellMode` (~741) next to the relocate toggles, dispatching `setActiveTool`.

## 5. Host wiring ([framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) ~956)
- Pass `brushActive={snap.activeTool === "brush"}` and `onBrushPlace={payload => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "addBrushObject", payload)}` into `PlayCanvas`. `kindCatalogs`/`kindCompatibility`/`blockedVortexFullIds` are already supplied.

## 6. Tests (extend existing files only)
- In [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) vitest block (~5600+): `computeBrushPlacementPose` (coincident point + opposite direction), `brushCompatibleCandidates`, `boxesIntersect`, `applyBrushPlacementToFixture` (object + attraction appended).
- In [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) tests (~1600+): `setActiveTool` snapshot, `addBrushObject` fixture growth + revision bump.
- Validate runtime in the 3d play app with `[DEBUG]` logs (enter/leave, candidate cycling, commit) before removing them; confirm visually via the play viewport.

## Process (repo rules)
Read `repo://goals`, open a ticket via the repo MCP, keep any temp logs/scripts inside the ticket folder, use regions/subregions, no new files for the above edits, close the ticket with a summary on completion.