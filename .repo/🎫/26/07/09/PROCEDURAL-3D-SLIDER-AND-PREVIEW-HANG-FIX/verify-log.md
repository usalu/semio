# Verify Log — Procedural 3d Slider and Preview Hang Fix

## Rust

- `cargo test -p kernel_3d_brepkit fixture_sphere_cut_torus_at_slider_max_completes` — **pass** (~36s)
- `cargo test -p mathematical_graph_port_directed_dag dag_host_slider_overlay_state_json_includes_slider_track` — **pass**
- `cargo test -p procedural-plugin preview_cache` — **pass** (viewport cache + patch widgets refresh)
- `cargo test -p procedural-plugin sphere_cut` — **pass** (~16s)

## TypeScript / Vitest

- `bun nx run @semio-tech/framework-renderer-react:test` — **27/27 pass** (includes slider overlay parse test)
- `bunx vitest run -t "keeps leaf inspector controls visible"` in `ui/js/react` — **pass**

## Browser (`browser-verify.ts`)

- Dev server: `PROCEDURAL_3D_PLAY_PORT=6018 SEMIO_RENDERER=react bun run dev:procedural:3d`
- In-canvas slider overlay visible on default hexagonal column fixture
- Graph wheel burst (12 events) completes in ~1.2s without tab stall
- Headless WebGPU `NoCompatibleDevice` and unrelated `forms-module-procedural` plugin panic filtered (environment noise)

## WASM rebuild

- `SEMIO_PLUGIN=procedural3d bun ./script.ts plugin` in `framework/product/os/dev`
- `bun ./script.ts wasm` in `flow/core`
