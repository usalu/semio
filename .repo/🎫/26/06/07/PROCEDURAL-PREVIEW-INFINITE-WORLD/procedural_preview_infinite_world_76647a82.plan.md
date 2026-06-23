---
name: Procedural Preview Infinite World
overview: "Upgrade the procedural play 3D preview pane (ProceduralPreview) from a minimal orbiting viewport into the full infinite-world canvas used by puzzle 3d and cad play: infinite LOD grid, viewport navigation gizmo, ortho/perspective projection switch, and camera view rig. No transform gumball."
todos:
  - id: imports
    content: Extend @infinite/world/r3f import in procedural/react with WorldLodBridge, WorldOrbitViewSnapGateProvider, WorldOrbitCameraViewRig, WorldOrbitViewControls, WorldOrbitProjectionSwitch, WorldLayer, applyOrbitProjectionToCameraState, and camera types
    status: completed
  - id: preview-stack
    content: Rewrite ProceduralPreview to compose the full infinite-world stack (grid bridge, snap-gate provider, camera rig, orbit gated, viewport gizmo, projection switch overlay) with component-local camera/projection state and seedKey
    status: completed
  - id: content-layer
    content: Move the visibleHandles BrepGeometryLayer mapping into a WorldLayer order=10 inside the new stack, preserving selection/hover/preview behavior
    status: completed
  - id: tests
    content: Extend the existing import.meta.vitest block in procedural/react for the upgraded viewport; run the procedural vitest suite and validate runtime behavior with temporary [DEBUG] logs
    status: completed
isProject: false
---

## Goal

Make the procedural play 3D preview pane look and behave exactly like the puzzle 3d / cad play viewports (infinite grid, viewport gizmo, projection switch, camera rig), reusing the shared `@infinite/world/r3f` engine. Per clarification: no transform gumball.

This is a single-component change in [procedural/react/index.tsx](procedural/react/index.tsx). No changes to `flow/*`, the play controller, or the playground renderer are required, because camera/projection state lives inside the component.

## Current state

`ProceduralPreview` ([procedural/react/index.tsx](procedural/react/index.tsx), ~line 867) is the minimal stack:

```882:905:/Users/ueli/Documents/compose/procedural/react/index.tsx
	return (
		<div className={className ?? "relative h-full w-full bg-zinc-900"}>
			<WorldCanvas frameloop="demand" cameraPosition={[8, 8, 6]} background="#18181b">
				<WorldCameraInvalidator />
				<ambientLight intensity={0.45} />
				<directionalLight position={[12, 18, 10]} intensity={1.1} />
				<WorldOrbitGated />
				{visibleHandles.map((entry, index) => ( ... ))}
			</WorldCanvas>
		</div>
	);
```

It is missing the infinite grid, viewport gizmo, projection switch, and managed camera rig.

## Reference composition (puzzle 3d `Inner`, lines 11074-11121)

The full stack to mirror: `WorldCanvas` (with `WorldOrbitProjectionSwitch` as overlay) -> `WorldLodBridge` (showLodGrid) -> `WorldOrbitViewSnapGateProvider` -> `WorldOrbitCameraViewRig` + `WorldOrbitGated` + `WorldOrbitViewControls` + lights + `WorldLayer` with content. All symbols are exported from `@infinite/world/r3f`.

## Changes — [procedural/react/index.tsx](procedural/react/index.tsx)

### 1. Adapters region (~line 31): extend the `@infinite/world/r3f` import

Add `WorldLodBridge`, `WorldOrbitViewSnapGateProvider`, `WorldOrbitCameraViewRig`, `WorldOrbitViewControls`, `WorldOrbitProjectionSwitch`, `WorldLayer`, `applyOrbitProjectionToCameraState`, and the types `WorldCameraState`, `OrbitCameraProjection`, `Vec3` (Vec3 already comes from `@geometry/brep/js`).

### 2. Rewrite `ProceduralPreview` (~line 867) to compose the full stack

- Hold camera state internally with `useState<WorldCameraState>` defaulting to a z-up perspective pose appropriate for the small brep scene (e.g. `{ position: [8, 8, 6], target: [0, 0, 0], up: [0, 0, 1], zoom: 1, projection: "perspective" }`), plus a `seedKey` counter (or string) that bumps when the gizmo/projection changes so `WorldOrbitCameraViewRig` re-seeds.
- Drop `cameraPosition` from `WorldCanvas` so the rig owns/manages the camera (matches puzzle/cad `managedCamera` pattern — `WorldCanvas.ownedCamera` is gated on `cameraPosition !== undefined`). Keep `frameloop="demand"` and `background="#18181b"`.
- Pass `overlay={<WorldOrbitProjectionSwitch projection={...} onProjectionChange={p => setCamera(applyOrbitProjectionToCameraState(camera, p)); bump seedKey} />}`.
- Inside `WorldCanvas`, wrap content in `WorldLodBridge` with `showLodGrid`, `automaticLod`, `distanceReference={100}`, `gridFactor` default, `gridDatum={[0,0,0]}` (mirrors cad `InteractionSpatialView`).
- Inside that, `WorldOrbitViewSnapGateProvider` containing, in order:
  - `WorldOrbitCameraViewRig state={camera} seedKey={seedKey} perspectiveFov={45}`
  - `WorldOrbitGated controlsKey={seedKey} projection={camera.projection} zoom={camera.zoom} onCamera={next => setCamera(next)}` (reports pose on orbit end)
  - `WorldOrbitViewControls onCameraChange={next => { setCamera(next); bump seedKey }}` (the bottom-right X/Y/Z gizmo; clicking snaps the view)
  - the existing lights (`ambientLight` + `directionalLight`; can keep current values)
  - `WorldLayer order={10} name="procedural.preview"` wrapping the existing `visibleHandles.map(... <BrepGeometryLayer/> ...)`
- Keep `WorldCameraInvalidator` for the demand frameloop.
- Preserve all existing geometry/selection/hover/preview props and `BrepGeometryLayer` behavior unchanged.

### 3. `BrepViewport` (~line 909)

No change needed — it delegates to `ProceduralPreview` and will inherit the new viewport automatically.

## Tests / validation — extend existing `import.meta.vitest` block in [procedural/react/index.tsx](procedural/react/index.tsx)

- Add/extend a render-smoke assertion that `ProceduralPreview` mounts the world stack (grid bridge + gizmo + projection switch present) without throwing, alongside the existing multi-handle/render-mode tests. Do not create new test files (extend in place per repo rules).
- Run the procedural vitest suite. Verify at runtime in procedural play that: the infinite grid renders, the bottom-right viewport gizmo snaps named views, the ortho/perspective switch toggles, and orbit/pan/zoom match cad/puzzle. Use temporary `[DEBUG]` logs for camera/projection changes during validation, then remove them.

## Ticket

Reopen the existing ticket `2026/06/07/PROCEDURAL-BREP-PLAYGROUND` (repo MCP `ticket_reopen`) since this continues the procedural preview window work; keep any temp files inside the ticket folder and close with a summary when done.

## Notes / decisions

- Camera + projection state are component-local (no controller/play-harness plumbing) to keep the change minimal and clean; this differs from puzzle/cad which persist camera to a fixture, but the user only asked for the viewport to match visually/behaviorally.
- No `UnifiedGumball` / `SpatialTransformGumball` (explicitly out of scope).
- Reuses shared engine only — no direct three/drei/fiber imports added (continues `sceneHostPort` discipline).
