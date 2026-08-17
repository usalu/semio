# CAD Selection Realtime Follow-up

## Context

- Repo MCP `repo://goals`, `ticket_reopen`, and `ticket_close` were not registered in this session.
- Used this existing ticket because it already covers stale 3D scene selection updates.
- Closest listed local goal remained `🎯️r2602🎯️runningsketchpad`.

## Change

- Updated `cad/js/renderer/index.tsx`.
- `useHostState` now resolves controlled functional updates from a live ref instead of the controlled prop captured by the child pane render.
- This keeps rapid CAD renderer selection updates from being computed against stale per-pane selection maps while the parent play shell is still re-rendering.

## Verification

- `bun ./📜️script.ts test -- --run index.tsx` from `cad/js/renderer`:
  - `index.tsx` passed: 66 tests.
  - Existing dirty `play/index.tsx` document tests still failed in collection.
- Browser runtime on `http://127.0.0.1:6120/` with concrete fixture:
  - Four CAD canvases rendered.
  - Console showed `[DEBUG] bootstrapCadModules: spatial-shape, aec-building, aec-building-energy, aec-building-structure`.
  - Console showed concrete reference texture loads.
  - Canvas click produced visible workbench selection detail: `Selected targets 1`, target `face · hexagonal-cut-concrete-forest-left-face-58`.
