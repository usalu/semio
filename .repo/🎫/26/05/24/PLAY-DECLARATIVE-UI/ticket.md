# Play Declarative UI

**Status:** closed (2026-05-24)

## Summary

Enforced canvas-only declarative window bodies in `@elements/ui-shell`: scene windows are a single fullscreen `scene3d`, board windows a single `board`. Chrome (toolbars, side panels, window measures) uses VS Code–style `WorkbenchMode.tools` and shell APIs only.

Fixed spatial/geometry/scene/topology play hosts that stacked status strips and in-window buttons on top of the 3D viewport. Removed duplicate status chrome inside `SpatialSurface`. Stabilized declarative window React components so workbench `generation` updates no longer remount the golden-layout window (fixes scene reload on every toolbar click). Toolbar plain buttons use `type="button"`.

## Files

- `elements/core/index.ts`
- `elements/renderer/react/index.tsx`
- `elements/renderer/react/ui-declarative-renderer.tsx`
- `elements/spatial/play/index.ts`
- `elements/spatial/react/index.tsx`
- `elements/renderer/react/windows/geometry/play/index.ts`
- `elements/renderer/react/windows/geometry/geometry-play-host.tsx`
- `elements/renderer/react/windows/scene/play/index.ts`
- `elements/renderer/react/windows/scene/scene-play-host.tsx`
- `elements/renderer/react/windows/topology/play/index.ts`
- `elements/renderer/react/windows/board/play/index.ts`
