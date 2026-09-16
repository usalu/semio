# R3F hooks outside Canvas (2026-09-16)

## Symptom

All `world-3d` surfaces (procedural 3d, CAD, aggregator, puzzle, …) showed:

`R3F: Hooks can only be used within the Canvas component!`

## Cause

`WorldCanvas` mounts `@react-three/fiber` through `sceneHostPort.fiber.canvas` (single pinned module via Vite `playgroundSceneHostResolveAliases`). `World3dHost`, tool-run trace, and `Scene` called `useThree` / `useFrame` / `useLoader` from a direct `@react-three/fiber` import. With dedupe/prebundle drift that yields two fiber stores: Canvas provides context on one, hooks read the other.

CAD renderer already avoided this via `sceneHostPort.fiber` PortWiring (`cad-js/renderer`).

## Fix

- Extended `SceneHostPort.fiber` with `useLoader`.
- Routed `World3dHost`, `tool-run-trace`, and `Scene` fiber hooks through `sceneHostPort.fiber`.
- Removed dead `@react-three/fiber` import from `framework-renderer-react` barrel.

## Verify

- Reload any app with a 3D window; viewport should render instead of the R3F hook error.
- `vitest` component tests under `World3dHost/🧪️tests/🧩️component` (mock fiber via `configureHostPorts` if needed).
