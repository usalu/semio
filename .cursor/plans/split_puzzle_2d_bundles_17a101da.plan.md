---
name: Split Puzzle 2D Bundles
overview: "Decompose the monolithic puzzle/2d Rust crate and React renderer into a crate-per-bundle, trait-extended stack: a generic infinite canvas base (Rust + React renderer), reusable Rust-only extension layers (graph, map, mindmap), and puzzle/2d re-homed on top, with puzzle/2d building and tests green on the new structure."
todos:
 - id: ticket
   content: Read repo://goals, open/reopen a repo MCP ticket for the puzzle 2d bundle split under the most fitting goal.
   status: completed
 - id: cavas-rust
   content: "Build infinite/cavas/vello crate: move vcompute, geom_sel, scene_json, svg_icon_vello09, canvas slice of board_host (camera/tiles/grid/LOD/Theme/vello scene/WebGPU session); define CanvasExtension trait + generic CanvasEngine; add Cargo.toml (rlib) + workspace member; migrate generic tests."
   status: completed
 - id: graph-rust
   content: "Build mathematical/graph crate depending on cavas: GraphExtension trait, Node/Handle/Edge, force_graph/hierarchical_tree/redraw_layout, graph hit-test/render; Cargo.toml + tests."
   status: completed
 - id: puzzle-rust
   content: "Re-home puzzle/2d/rs on graph: Puzzle2dExtension impl, keep elements palette/metabolism icons/codec + build.rs, kind catalogs, brush, fixtures, concrete BoardSession + board_* wasm exports; update Cargo.toml path deps."
   status: completed
 - id: map-mindmap-rust
   content: "Implement gis/map (MapExtension: CanvasExtension) and reasoning/mindmap (MindmapExtension: GraphExtension, Topic/Relationship) as single-lib.rs extension crates with unit tests; add workspace members."
   status: completed
 - id: cavas-react
   content: "Build @semio-tech/infinite-cavas-react-renderer: move Adapters, GpuWasmBridge, ReactCanvas reconciler/host/Scene Sync/Hooks; add package.json/project.json/script.ts/vitest/tsconfig; migrate generic React tests."
   status: completed
 - id: puzzle-react
   content: "Re-home @semio-tech/puzzle-2d-react on the renderer: keep puzzle kinds/paint/store/objects/scene/fixtures; import @semio-tech/infinite-cavas-react-renderer; add workspace dependency."
   status: completed
 - id: wiring
   content: Wire launch.json test/build entries for new crates and the react-renderer (following existing grouping/order); ensure each bundle has exactly one script.ts and project.json calls script.ts.
   status: completed
 - id: verify
   content: Run cargo build/test per crate, both react vitest suites, the puzzle/2d wasm build, and dev:puzzle:2d runtime check; confirm map/mindmap compile + tests pass.
   status: completed
 - id: close
   content: Close the ticket with summary and the full list of created/updated/removed files.
   status: completed
isProject: false
---

## Goal

Turn the two monoliths into layered, reusable bundles where each higher level is a single Rust file extending the level below via traits:

- `infinite/cavas/vello/lib.rs` - generic canvas engine crate (rlib) + `infinite/cavas/react-renderer/index.tsx` - generic React reconciler/host.
- `mathematical/graph/lib.rs`, `gis/map/lib.rs`, `reasoning/mindmap/lib.rs` - Rust-only extension crates (each one `lib.rs`).
- `puzzle/2d/rs/lib.rs` + `puzzle/2d/react/index.tsx` - the concrete leaf bundle (cdylib wasm + puzzle React) re-homed on graph + cavas.

This must run inside a repo MCP ticket (read `repo://goals` first, then `ticket_open`), use regions/subregions in every file, keep one `script.ts` per bundle, and register launch.json entries following existing grouping.

## Extension model (the core design)

Crate-per-bundle, compiled as rlib libraries; only `puzzle/2d/rs` stays a `cdylib` producing wasm (wasm-bindgen cannot be generic, so the concrete `BoardSession` lives at the leaf).

- `infinite_cavas` defines `pub trait CanvasExtension` (hit-test contribution, scene/paint contribution, kind catalog, theme palette injection) and a generic `CanvasEngine<E: CanvasExtension>` owning camera, tiles, grid, viewport, LOD, vello scene assembly, selection box predicates, and the WebGPU surface plumbing.
- `mathematical_graph` defines `pub trait GraphExtension: CanvasExtension` adding `Node`/`Handle`/`Edge`, hit-testing and the `force_graph` / `hierarchical_tree` / `redraw_layout` algorithms.
- `puzzle_2d` provides a concrete `Puzzle2dExtension` implementing `GraphExtension`, contributing `NodeKind`/`HandleKind`/`EdgeKind`/`WireKind`, the elements palette/metabolism icons, brush, fixtures, and the wasm facades (`BoardSession`, `board_*` free functions).
- `gis_map` (`MapExtension: CanvasExtension`) and `reasoning_mindmap` (`MindmapExtension: GraphExtension`, Topic=Node / Relationship=Edge) are thin proof-of-extensibility crates with their own unit tests.

The React renderer splits the same way: the generic reconciler/`ReactCanvas`/`GpuWasmBridge` move to `@semio-tech/infinite-cavas-react-renderer`; puzzle kinds/paint/stores stay in `@semio-tech/puzzle-2d-react` which imports the renderer.

## Boundary map (from current regions)

Rust `puzzle/2d/rs/lib.rs`:

- To cavas: `vcompute` (geometry), `geom_sel` (selection predicates), `scene_json`, `svg_icon_vello09`, and the canvas slice of `board_host` (camera, tiles, grid, viewport, LOD, `Theme` struct, vello scene assembly, `BoardSessionInner`/WebGPU render loop). Generic deps `vello`, `vello_svg`, `typst*`, `image`, `fontdb`, `data-url`, `base64` move here.
- To graph: `force_graph`, `hierarchical_tree`, `redraw_layout`, plus `Node`/`Handle`/`Edge` (lines ~7426-7730) and their hit-test/render in `board_host`.
- Stays in puzzle/2d: `board_icon_assets`, `elements_board_palette` (build.rs include from `ui/styling/rs/board_vello_build.inc.rs`), `board_metabolism_icons`, `board_icon_codec`, concrete palette->`Theme` wiring (lines ~2737-2772), kind catalogs, brush, fixtures, `BoardSession` + `board_*` wasm exports. `build.rs` stays here (elements/metabolism specific).

React `puzzle/2d/react/index.tsx`:

- To cavas react-renderer: `Adapters`, `GpuWasmBridge`, generic `ReactCanvas` (regions ~9605-11421: Kinds/Context/Markers/Descriptor Build/Scene Sync/HostMountBridge/Canvas/Hooks) and the reconciler `Renderer` host plumbing (`HostKinds`/`PropApply`/`MountHelpers`/`HostMountInternals`).
- Stays in puzzle/2d react: `IconSelectorMode`, puzzle `Kinds`, `ElementsUiPuzzle2dPaint`, `Stores`, `Objects`, `Scene`, `DirectedGraphObservation`, fixtures, the puzzle-specific paint inside `Renderer`.

## Build / workspace wiring

- Add `infinite/cavas/vello`, `mathematical/graph`, `gis/map`, `reasoning/mindmap` to `[workspace].members` in `Cargo.toml`. Each gets a `Cargo.toml` (`crate-type = ["rlib"]`, path `lib.rs`). `puzzle/2d/rs/Cargo.toml` keeps `cdylib`/`rlib` and adds path deps on `mathematical_graph` (which deps `infinite_cavas`).
- React renderer bundle gets `package.json` (`@semio-tech/infinite-cavas-react-renderer`), `project.json`, `script.ts`, `vitest.config.ts`, `tsconfig.json` mirroring `puzzle/2d/react`. `@semio-tech/puzzle-2d-react` adds `@semio-tech/infinite-cavas-react-renderer: workspace:*`.
- Move generic Rust tests into the matching crate's `lib.rs` `#[cfg(test)]` (rule: extend existing test blocks, no new test files). Generic React tests move into the renderer's `import.meta.vitest` block.
- `launch.json`: add cargo test entries for the new crates and an `@semio-tech/infinite-cavas-react-renderer:test` entry, following the existing 3_dev group ordering.

## Verification (acceptance)

- `cargo build`/`cargo test` green for each new crate and for `puzzle_2d` (run, do not assume).
- `bun nx run @semio-tech/infinite-cavas-react-renderer:test` and `@semio-tech/puzzle-2d-react:test` green.
- `puzzle/2d/rs` wasm build still succeeds; `bun run dev:puzzle:2d` renders the nakagin fixture (confirm at runtime via console logs).
- `map` and `mindmap` crates compile with passing unit tests, proving a new extension is just one `lib.rs`.

## Risks / notes

- `board_host` (~5k lines) is the hard split; the `CanvasExtension`/`GraphExtension` trait surface must be sized so node/handle/edge/wire paint + hit-test live in graph/puzzle while camera/tiles/scene stay generic.
- The empty stub files already exist (`infinite/cavas/vello/lib.rs`, etc.), so this edits stubs and adds the required crate scaffolding rather than inventing a new layout.
- Intermediate crates are rlib-only (no wasm/script needed); only the leaf ships wasm, keeping "every extension is just a rust file" literally true.
