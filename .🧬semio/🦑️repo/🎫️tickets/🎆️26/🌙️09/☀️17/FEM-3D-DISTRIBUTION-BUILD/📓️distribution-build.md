# FEM 3D distribution build

## Objective

Produce distribution-ready FEM 3D artifacts under each package's `dist/` tree:

| Layer | Nx target | Output |
|-------|-----------|--------|
| Artifact library | `@semio-tech/fem-3d-rs:build` | `✏️s/.../🧊️3d/📦️packages/🦀️rust/dist/build` |
| Plugin WASM | `@semio-tech/fem-plugin:component-release` | `.../fem/📦️packages/🦀️rust/dist/component-release` |
| Browser bundle | `@semio-tech/fem-plugin:materialize-release` | materialized support tree for fem |
| React distribution (CDN) | `@semio-tech/framework-os-dev:build-fem3d-react-release` or `@semio-tech/fem-plugin:build-fem3d-site` | `✏️s/🔌️plugins/🏗️fem/dist` (same Vite app as `serve-fem3d-react-dev`, `SEMIO_BUILD_MODE=ship`) |

## Progress

- `@semio-tech/fem-3d-rs:build` — **green** (2026-09-17): `dist/build` contains `libsemio_s_artifact_fem_3d.rlib`, `libsemio_s_artifact_fem_3d.rmeta`, `deps/` (236 entries).
- `@semio-tech/fem-plugin:component-release` — **green** (2026-09-17): `dist/component-release/semio_s_plugin_fem.wasm` (~16.0 MB, wasm-release + thin LTO).
- `@semio-tech/framework-os-dev:build-fem3d-react-release` — **green** (2026-09-17): published 1603 files (includes wasm-release fem plugin via `materialize-release` dependency chain).
- **CDN output path** (2026-09-17): `[[package.metadata.semio.playground]]` `distDir` on `fem3d` → single deployable tree at `✏️s/🔌️plugins/🏗️fem/dist` (`🌐️.html`, `🧶️bundles/`, `🔌️plugin-modules/`, …).

## Notes

- First `@semio-tech/fem-plugin:component-release` attempt failed after ~17m with a non-zero Cargo exit and no rustc diagnostics in the nx log; immediate retry with `--skip-nx-cache` succeeded (~13m). Likely a transient link/LTO race while another wasm build was active.
- Release WASM is ~16 MB vs ~81 MB for `component-dev`.
