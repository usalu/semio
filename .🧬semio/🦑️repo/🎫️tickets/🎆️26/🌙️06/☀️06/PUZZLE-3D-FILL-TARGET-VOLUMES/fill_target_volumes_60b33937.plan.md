---
name: Fill target volumes
overview: 'Extend the puzzle 3d fill tool with persisted "target volumes": oriented boxes drawn with a CAD-box-style 3-point + height tool, selectable/movable/scalable via the gumball, that constrain fill suggestions to objects whose bounding box is fully contained in the union of volumes.'
todos:
 - id: ticket
   content: Read repo://goals and open ticket PUZZLE-3D-FILL-TARGET-VOLUMES under best goal
   status: completed
 - id: world-primitive
   content: Add shared WorldVolume* oriented-box primitive (props, pose/transform helpers, BoxItem, Layer, worldVolumesContainAabb) in infinite/world/r3f
   status: completed
 - id: fixture
   content: "Extend Fixture with targetVolumes[]: parse/encode, pure operations (add/update/remove/relocate), selection snapshot targetVolumeIds + pick kind"
   status: completed
 - id: render-layer
   content: Render WorldVolumeLayer in puzzle 3d scene wired to registry selection/hover + onTargetVolumeRelocate; canvas props fillEditTargetVolumes/onTargetVolumeRelocate/onTargetVolumeDraw
   status: completed
 - id: draw-tool
   content: Implement 3-point + height box-draw session (grid picks, vertical Z projection, CAD-box preview ghost, commit) with draw UI store + tool-active ref gating marquee
   status: completed
 - id: fill-submode
   content: Add fillEditTargetVolumes controller state + commands and engagement options/status (Edit target volumes, Delete volume)
   status: completed
 - id: fill-constraint-ts
   content: Add targetVolumes to BrushFillSequenceArgs and AABB-containment gate in createBrushFillSequenceStepper.tryPlaceOne + invalidation on volume change
   status: completed
 - id: fill-constraint-rs
   content: Pass volumes through preparePuzzle3dFillSession/startPuzzle3dFillBuild/setScene and implement containment in Rust fill_step_one
   status: completed
 - id: host-plumbing
   content: "Wire playground host: fillEditTargetVolumes prop, onTargetVolumeRelocate/Draw, volume selection, capture targetVolumes into fill session prep"
   status: completed
 - id: tests
   content: Extend existing test files (infinite, puzzle/3d react, rs) and verify runtime with [DEBUG] logs; run nx tests; close ticket
   status: completed
isProject: false
---

## Fill Target Volumes

Mirror the existing `references` feature end-to-end (shared world primitive -> fixture field -> selection -> gumball relocate -> parse/encode), add a 3-point+height draw tool as a fill sub-mode, and gate fill placements by AABB containment in TS + Rust.

### Ticket (first, per repo rules)

Read `repo://goals`, then `ticket_open` slug `PUZZLE-3D-FILL-TARGET-VOLUMES` under the best goal. All temp logs/scripts go in that ticket folder. Close with `ticket_close` listing touched files.

### 1. Shared oriented-box primitive — [infinite/world/r3f/index.tsx](infinite/world/r3f/index.tsx)

New `#region Volume` mirroring the `WorldReference*` block (lines ~1656-1860):

- `WorldVolumeProps extends WorldEntityFlags` = `{ id; origin: Vec3; orientation?: Quat; scale?: number|Vec3; color?; opacity?; relocate?; relocateActive? }`. The box is a unit cube scaled by `scale` (so `scale` encodes width/depth/height); gumball move/rotate/scale map directly.
- `WorldVolumeRelocatePayload`, `applyWorldVolumePose(group, volume)`, `applyWorldVolumeTransform(volume, after)` — copies of the reference helpers.
- `WorldVolumeBoxItem`: translucent `boxGeometry args={[1,1,1]}` + `lineSegments` edges (`depthWrite=false`, `DoubleSide`), pointer select/hover, mounts `UnifiedGumball` when selected (reuse `WorldReferenceGumball` pattern). Honors `worldEntityRenderMode`/`worldEntitySelectable`.
- `WorldVolumeLayer` (maps `WorldVolumeProps[]` -> items) with `selectedIds/hoveredId/onSelect/onHover/onRelocate/relocateActive/translationSnap`.
- Containment helper `worldVolumesContainAabb(volumes, aabbMin, aabbMax)`: transform the 8 AABB corners into each volume's local frame (inverse pose), inside if all corners satisfy `|local.axis| <= scale.axis/2` for any single volume (union, AABB-strict). Work in three space (same space as collision bodies) to avoid CAD/three axis confusion. Add to the existing test region: pose round-trip + contain true/false cases.

### 2. Fixture wiring — [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx)

- Import `WorldVolumeLayer`, `applyWorldVolumeTransform`, `WorldVolumeProps`, `WorldVolumeRelocatePayload` (next to the existing reference imports, lines 70-74).
- Extend `Fixture` (line 1260) with `targetVolumes: WorldVolumeProps[]` (default `[]`); parse in `parseFixture` (line ~1612, mirror `references` flatMap) and include in any encode path.
- Pure operations mirroring `addReferenceToFixture`/`updatePuzzle3dReferenceInFixture`/`applyReferenceRelocateToFixture` (lines 1507-1532): `addTargetVolumeToFixture`, `updatePuzzle3dTargetVolumeInFixture`, `removeTargetVolumeFromFixture`, `applyTargetVolumeRelocateToFixture`.
- Selection: add `targetVolumeIds` to `SelectionSnapshot` (line 452), `EMPTY_SELECTION_SNAPSHOT` (483), `{ kind: "targetVolume"; id }` pick (492/888), `selectionFromPick` + merge (498-575), equality checks.
- Canvas props: add `fillEditTargetVolumes?: boolean`, `onTargetVolumeRelocate?`, `onTargetVolumeDraw?(volume)` near `onReferenceRelocate` (line 1069).
- Scene: render `<WorldVolumeLayer .../>` in `Inner` (next to the references layer), wired to registry selection/hover and `onTargetVolumeRelocate`. Volumes always render as faint guides; select/gumball enabled only when `fillEditTargetVolumes`.

### 3. Draw tool: 3 points + height — [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx)

Concise box-draw session reusing CAD box semantics (reference: [cad/asset/modelDefinition/spatial.shape/interaction/box.json](cad/asset/modelDefinition/spatial.shape/interaction/box.json) three-point states; preview math `computeBoxPreviewLayout` in [cad/js/kernel/brepjs/index.ts](cad/js/kernel/brepjs/index.ts)):

- New external store `puzzle3dTargetVolumeDrawUiStore` + `puzzle3dTargetVolumeToolActiveRef` mirroring `puzzle3dBrushUiStore`/`puzzle3dBrushToolActiveRef` (lines 7311-7330) to gate marquee/select (add to the guards at 9018 / 10405).
- Phases: `p0` (pointerdown) -> `edge` (pointerdown sets p1, defines base edge direction+length) -> `width` (pointerdown sets p2, perpendicular width => oriented base rect: origin=center, orientation=quat from edge dir on grid plane, base scale=[len,width,~0]) -> `height` (pointermove projects pointer ray onto a vertical Z line through center, sets Z size; pointerdown/Enter commits). XY picks via `puzzle3dClientToGridPlaneCad` (line 1704) with grid snap; add a small `projectRayToVerticalZLineCad` helper (port of CAD `projectRayToVerticalZLine`).
- Live preview: render a `WorldVolumeBoxItem`-styled ghost (`raycast={()=>null}`) from the in-progress pose.
- Commit -> `onTargetVolumeDraw(volume)` -> host calls `addTargetVolumeToFixture`, then auto-selects it so the gumball appears for immediate move/scale.

### 4. Fill sub-mode + engagement — [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) and [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx)

- Controller: add `fillEditTargetVolumes: boolean` to `Puzzle3dPlayShellController` + `Puzzle3dPlaySnapshot`; commands `setFillEditTargetVolumes` / `toggleFillTargetVolumeEdit`; `deleteSelectedTargetVolume`. Add `addTargetVolume`/`relocateTargetVolume` patch cases (use the pure operations).
- `buildPuzzle3dPlayEngagement` (lines 7664-7776): when `activeTool === "fill"`, add an `options` toggle "Edit target volumes"; while editing, set status hint ("Pick 3 points then drag height; select a volume to move/scale") and a "Delete volume" option when a target volume is selected. Keep the fill-count slider available.
- Activating edit sets `puzzle3dTargetVolumeToolActiveRef` and enables interactive volumes + draw tool via the `fillEditTargetVolumes` canvas prop.

### 5. Constrain fill suggestions (TS + Rust)

- TS: add `targetVolumes?: readonly WorldVolumeProps[]` to `BrushFillSequenceArgs` (line 4266). In `createBrushFillSequenceStepper.tryPlaceOne` (after `brushPreviewFromCandidate`, ~4355), when volumes are non-empty compute the preview world AABB (`brushPreviewWorldMatrix` line 3837 + posed mesh bbox, as in `brushCollisionMeshExtentOk` line 3855) and `continue` if `!worldVolumesContainAabb(...)`. Enforced before the existing collision check.
- Rust: [puzzle/3d/rs/lib.rs](puzzle/3d/rs/lib.rs) — add target volumes to `SceneConfig`/`set_scene` and implement the same AABB-in-oriented-box test in `fill_step_one` (~1360-1509) right after target enumeration.
- Plumbing: pass volumes through `preparePuzzle3dFillSession` (line 628) -> `startPuzzle3dFillBuild` (line 501) -> worker `setScene` JSON `buildPrecomputeSceneJson` (line ~4498). Source volumes from the captured base fixture.
- Invalidation: add `invalidatePuzzle3dFillForTargetVolumesChange()` (mirror `invalidatePuzzle3dFillForDistributionChange`) called when volumes are added/edited/removed while fill is active, forcing a rebuild + re-apply of the pending count.

### 6. Host plumbing — [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)

- Pass `fillEditTargetVolumes={snap.fillEditTargetVolumes}` and wire `onTargetVolumeRelocate` -> `relocateTargetVolume`, `onTargetVolumeDraw` -> `addTargetVolume`, plus target-volume selection into the existing selection bridge (mirror `onReferenceRelocate`, canvas props ~2104).
- Include `targetVolumes` in `fillBaseCaptureRef` and call `preparePuzzle3dFillSession(base, ..., overlapBudget, base.targetVolumes)`; re-prepare when volumes change (extend the `fillSessionPreparedRef` reset logic at ~2009-2018).

### 7. Tests & verification (extend existing files only)

- `infinite/world/r3f`: pose round-trip, `worldVolumesContainAabb` true/false, `applyWorldVolumeTransform`.
- `puzzle/3d/react`: `targetVolumes` parse/encode round-trip; box-draw pose math from 3 points; fill sequence with a volume keeps every placement contained; selection snapshot with `targetVolumeIds`.
- `puzzle/3d/rs`: `fill_step_one` containment test.
- Run puzzle 3d play; confirm at runtime with temporary `[DEBUG]` logs: draw a box (3 points + height) inside fill edit mode, move/scale it via gumball, leave edit mode, and verify new fill suggestions appear only inside the volume. Run `bun nx run @semio-tech/puzzle-3d-react:test`, `@semio-tech/infinite-world-r3f:test`, `@semio-tech/puzzle-3d-rs:test`.

### Notes

- No new launch.json entries required (reuses puzzle 3d play); the edit toggle/draw are declarative engagement options.
- All new code in `#region`/subregions, concise, emoji-prefixed docstrings, no external lib imports outside `sceneHostPort`/existing ports.
- "Target volume" = the persisted oriented box ("voxel"); AABB-strict containment; oriented boxes per the confirmed decisions.
