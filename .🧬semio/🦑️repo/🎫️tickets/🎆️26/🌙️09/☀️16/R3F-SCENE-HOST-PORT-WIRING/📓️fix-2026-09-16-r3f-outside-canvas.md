# R3F hooks outside Canvas (2026-09-16)

## Symptom

All `world-3d` surfaces (procedural 3d, CAD, demonstrator, aggregator, puzzle, …) showed:

`R3F: Hooks can only be used within the Canvas component!`

Stack often pointed at `@react-three/drei` `PerspectiveCamera` inside `WorldCanvas` / `WorldProjectionRig`.

## Cause (two layers)

1. **App hooks** — `World3dHost`, tool-run trace, and `Scene` called `useThree` / `useFrame` / `useLoader` from a direct `@react-three/fiber` import while `WorldCanvas` mounted Canvas via `sceneHostPort.fiber`. With dedupe/prebundle drift that yields two fiber stores.

2. **Vite prebundle vs aliases** — Dev configs used `playgroundSceneHostResolveAliases` (pin fiber/drei to one ESM entry) **and** `optimizeDeps.include: ["@react-three/fiber"]`. The prebundled `.vite/deps` fiber is a second store; drei's `PerspectiveCamera` (always imported through `@react-three/drei`) calls `useThree` on that copy while `Canvas` uses the aliased module.

CAD renderer already routed app hooks via `sceneHostPort.fiber`.

## Fix

- Extended `SceneHostPort.fiber` with `useLoader`; routed `World3dHost`, `tool-run-trace`, and `Scene` fiber hooks through `sceneHostPort.fiber`.
- Wired `threeHostPort.canvas` to `sceneHostPort.fiber.canvas` (same constructor for `HostThreeCanvas` and `WorldCanvas`).
- Added `playgroundSceneHostOptimizeDeps()` — keeps `three` warm, **excludes** `@react-three/fiber` and `@react-three/drei` from prebundle; applied to os dev and mit-bestand demonstrator Vite configs.

## Verify

1. Delete stale optimizer output: `.🧬semio/🦑️repo/⚡️cache/vite/mit-bestand-demonstrator` (or restart dev server after the config change).
2. Reload demonstrator CAD pane; 3D windows should render instead of `ShellFaultBoundary` / R3F hook error.
3. `bun nx run @semio-tech/ui-styling:test` — `playgroundSceneHostOptimizeDeps` unit test.
