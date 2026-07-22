# Get Aggregator Working End to End

## Symptom

1. Windows showed `Fehlendes Fenster: puzzle3d-main-perspective` (and the Top pane likewise empty of body).
2. After that race was fixed, Top + Perspective panes still showed chrome only — transparent WebGL canvases over `bg-window` beige, no reference plane / meshes.

## Root causes

### A. Missing window body race

Default layout seeds two instances of `puzzle3d-main`:

- `puzzle3d-main-top`
- `puzzle3d-main-perspective`

`modeWindows` renders each extra instance from `windowUiByWindowId[instance.id]`. A concurrent `refreshUi` that only knows the bare kind id (`puzzle3d-main`) replaces the whole record via `mergeRecordPreservingIdentity`, wiping the Top/Perspective bodies while `extraWindowInstances` / `effectiveModeLayout` still reference them → "Fehlendes Fenster".

Race on boot:

1. Session-switch `refreshUi` fetches all panes and writes extras.
2. Boot `setActiveExample` starts another `refreshUi` from a render whose closure still has `extraWindowInstances = []` (or after the session-switch refresh set `layoutSeedKeyRef` so the second call no longer seeds from the default layout).
3. Second refresh applies only `puzzle3d-main` and erases the instance-keyed UI.

### B. Blank 3D canvases

Three stacked bugs:

1. **Pending projection take-on-read** — React Strict Mode mount→unmount→remount consumed `pendingWorldProjectionByWindowId` on the discarded pass; the second mount fell back to the shared scene camera.
2. **Stale render-time camera in layout effects** — `WorldProjectionContentFrame` / `WorldOrbitCameraViewRigSeed` closed over `useThree(s => s.camera)` from render, still the Canvas default `PerspectiveCamera`, while sibling `OrthographicCamera makeDefault` had already updated the live store. Top never got a pixel frustum (`left: null`); pose applied to the wrong camera.
3. **`frameloop="demand"` without post-load invalidate** — reference textures and GLBs resolve after the 4-frame mount kick; React commits meshes/textures but no draw is scheduled, so panes stay fully transparent.

## Fix

### Missing window body

1. Seed default-layout extras + shell layout synchronously when creating the boot session (host + playground paths).
2. Keep `extraWindowInstancesRef` updated on every seed/split/drop/retitle write; `refreshUi` and action dispatch read the ref, never the render-closure snapshot.
3. Drop `extraWindowInstances` from `refreshUi` deps so seeding extras does not itself schedule a second full refresh.

### Blank 3D canvases

1. Sticky `peekPendingWorldProjection` (clear only on pane close), not take-on-read / clear-on-apply.
2. Layout effects read the live store via `getThree().camera` / duck-type `isOrthographicCamera` (no `instanceof` across duplicate `three` copies).
3. `invalidate()` after reference texture media lands, after GLB scene clones, and a short demand-frameloop pump (~24 frames) on canvas mount.

## Verification

Playwright on `http://127.0.0.1:6023/` (skip intro):

- Top: `left: -207.5…` (ortho frustum applied), floor plan + grid + markers.
- Perspective: reference plane + GLB mesh on grid.
- Host screenshots: `host-0-v2.png`, `host-1-v2.png` in this ticket folder.
