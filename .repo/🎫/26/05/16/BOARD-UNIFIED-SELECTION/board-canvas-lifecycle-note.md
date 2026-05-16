# Board canvas pointer lifecycle (2026-05-16)

## Cause

`BoardCanvas` used `useLayoutEffect` with `if (!canvasRef.current || rendererRef.current) return` and deps that included selection (or any prop that changed often). On each re-run React first ran the **previous** cleanup (`dispose()`). Combined with the early return (no new cleanup returned), that disposed the live `BoardRenderer` while the canvas stayed mounted — WASM pointer/wheel handling died; fixture drag still worked because it is wired on the outer `div`.

## Fix

- Recreate `BoardRenderer` only when `renderMode` changes; keep `setSelectionOptions` / `setWorldRasterTilingOption` for other prop updates.
- Drop the `rendererRef.current` guard so the effect always returns a stable cleanup contract.
- `flushSync(() => setContextRenderer(null))` before `dispose()` so `BoardHostSubtree` tears down the host mount (and clears the scene) before `session.free()`.

## Tests

Vitest: `does not dispose BoardRenderer when only selection props change` plus existing suite (`bunx vitest run --config vitest.config.ts` in `elements/client/lib/board`).
