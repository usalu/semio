# Play Declarative UI

**Goal:** Detach elements play bundles from DOM/React; plugins contribute `UiNode` trees and commands; host renders.

**Bundles:** geometry/play (prior), geometry/spatial/play, scene/play, board/play.

## Summary

- Extended `@elements/ui-shell` `UiNode` with `board`, `panel`, and `ShellWindowMeasure`; added `registerDeclarativeSidePanelBody`.
- Host renderer (`ui-declarative-renderer.tsx`) registers surface hosts for scene3d, board, and panel.
- `WorkbenchView` resolves declarative window bodies and side panels; LOD measures dispatch via `CommandBus`.
- Migrated spatial, scene, and board play bundles: framework-free `index.ts` + thin `react.tsx` host adapters.

## Files

- `elements/client/lib/ui/ui-protocol.ts`
- `elements/client/lib/ui/index.ts`
- `elements/client/lib/react/ui-declarative-renderer.tsx`
- `elements/client/lib/react/index.tsx`
- `elements/client/lib/geometry/spatial/play/index.ts`
- `elements/client/lib/geometry/spatial/play/react.tsx`
- `elements/client/lib/geometry/spatial/play/index.html`
- `elements/client/lib/system/renderer/react/scene/play/index.ts`
- `elements/client/lib/system/renderer/react/scene/play/react.tsx`
- `elements/client/lib/system/renderer/react/windows/board/play/index.ts`
- `elements/client/lib/system/renderer/react/windows/board/play/react.tsx`
- `elements/client/lib/system/renderer/react/windows/board/vitest.config.ts`
