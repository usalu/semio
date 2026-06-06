# Puzzle3d Precompute Worker

## Summary

Background Web Worker + Rust/WASM (`parry3d`) precomputes brush collision-free candidates per free vortex and greedy fill sequences off the UI thread.

## Architecture

- `puzzle/3d/rs` — `Puzzle3dPrecomputeSession` (wasm-bindgen): collision, candidate enumeration, pose math, fill stepper
- `puzzle/3d/react/precompute.worker.ts` — JSON-RPC worker with idle `precompute_step` loop
- `puzzle/3d/react/index.tsx` — `Puzzle3dCollisionEngine` interface; `WasmCollisionEngine` (worker) + `MeshBvhCollisionEngine` (vitest/SSR fallback)
- Brush hover reads warm worker cache; `BrushPreviewGhost` is render-only
- Fill session uses worker `fill_progress` when `puzzle3dPrecomputeUsesWorker()` is true

## Files

- `puzzle/3d/rs/Cargo.toml`, `lib.rs`, `script.ts`, `project.json`, `package.json`
- `puzzle/3d/react/precompute.worker.ts`
- `puzzle/3d/react/index.tsx` (precompute region, brush/fill wiring)
- `puzzle/3d/play/index.ts` (fill session + host rules)
- `Cargo.toml` (workspace member)
- `.vscode/launch.json` (`🛠️dev🧩puzzle📷3d🦀rs`)
- `puzzle/3d/react/vitest.config.ts`, `project.json`

## Tests

- 306 vitest tests pass in `@puzzle/3d/react`
- Rust unit test `brush_candidates_allow_separated_boxes`
- WASM vs mesh-bvh parity test in vitest
