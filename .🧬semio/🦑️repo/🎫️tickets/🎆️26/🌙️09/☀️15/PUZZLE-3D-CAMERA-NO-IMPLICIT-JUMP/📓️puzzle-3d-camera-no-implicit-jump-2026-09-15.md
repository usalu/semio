# Puzzle 3D Camera No Implicit Jump

## Symptom

Viewport camera jumps (zoom, pan, reframe) when placing objects from the catalogue, during fill, or other edits — without `setCamera`, `focusSelection`, orbit, or gizmo view actions.

## Root causes

1. **`WorldAutoFit` fit key included the scene mesh roster.** Puzzle 3d publishes a revision-only `fit_json` lane (document identity, not geometry). Adding a catalogue kind registers a new mesh id, which changed the fit key and re-armed auto-fit even though the user had already been framed on that document.
2. **`WorldProjectionContentFrame` kept reframing on growing `contentBounds`** while `viewportOwned` was still false after the one-shot projection seed (orthographic Top pane especially). Fill provisional instances were already excluded from bounds; the continuous reframer still ran for committed geometry growth.

## Fix

- `world3dAutoFitKey(revision, sceneCameraAttachJson, autoFitBounds)` — bounds segment only when the producer publishes `boundsMin`/`boundsMax` (generation3d); no mesh list.
- After the first projection content seed, lock continuous projection reframing (`projectionContentFrameSeededRef`) and set `viewportOwned` so later edits do not soft-reframe the pane.

## Files

- `🧰️framework/…/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts`

Related prior work: `PUZZLE-3D-FILL-ORBIT-CAMERA-RESET` (fill provisional bounds + orbit navigate lock).
