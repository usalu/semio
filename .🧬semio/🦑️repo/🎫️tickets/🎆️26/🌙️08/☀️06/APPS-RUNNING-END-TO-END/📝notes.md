# Notes\n\n- Root cause for compose-desktop: electron-forge only walked up via npm/yarn/pnpm lockfiles, not bun.lock.\n- Applied bun patch to @electron-forge/core-utils@7.11.2 to include bun.lock/bun.lockb.\n- Catalog resolves process 3d -> process3d correctly (58 playgrounds).\n- puzzle 3d segments resolve null (alias is just 3d / puzzle3d) — package.json script uses puzzle:3d -> dev puzzle 3d which may fall through!\n
## GIS 2D (2026-08-06)

### Root causes fixed
1. **`ephemeralBox` function init** — treated function-typed `T` as lazy factories, so `resolveControlLabelId`'s identity resolver became `undefined` and crashed `FrameworkOsShellInner`. Init is now stored as-is; tests added in `🧩core`.
2. **Host `wasm_exports`** — gated behind `os-host-full` (uses `workflow` which is feature-gated).
3. **Plugin linker shim** — weak default `semio_plugin_bundle_installer_link_shim` for intermediate wasip2 cdylibs under `component-guest` feature unification.
4. **Surface ↔ puzzle** — `board-2d` is now optional (`default` feature); GIS uses `default-features = false`.
5. **Node-graph dag path** — `infinite_canvas::board::ports::directed_dag as dag`.
6. **GIS surface imports** — dep renamed to `framework_surface`; call sites use `framework_surface::{tiled_map,terrain}`.
7. **MapHost API** — `host.features.positions/routes`.
8. **`context_menu`** — dropped `&self` to match `DocumentApp` trait.
9. **Surface browser pkg** — real wasm rebuilt with `noDefaultFeatures` + `session-bindgen`; terrain bindgen gated so infinite path-mount does not duplicate `TerrainSession`.

### Verified
- `cargo build -p semio-s-plugin-gis --target wasm32-wasip2 --profile wasm-release` ✅
- Playwright smoke at http://127.0.0.1:6040/ — shell mounts, title `semio · gis · 2d`, `[DEBUG] plugin worker + gis (1 live)`, no `controlLabelIdResolver` crash.
- Headless-only `NoCompatibleDevice` (no WebGPU in Chromium headless) — expected.

### Still open (out of GIS-critical path)
- Puzzle 3d `PlayApp` source corruption blocks surface `board-2d` default feature / puzzle apps.
