---
name: Extract Graph Canvas, Add Dag
overview: Extract the generic graph-canvas engine out of puzzle/2d into infinite/canvas (canvas concerns) and the mathematical/graph bundles (graph engine), leaving puzzle/2d with only puzzle tooling (brush/fill/palette), then build DAG's own Rust-rendered IO-node canvas with a playground dev harness and fixture.
todos:
 - id: ticket-baseline
   content: Read repo://goals, open a repo MCP ticket for the extraction + dag; capture cargo/vitest/dev baseline; route temp files into the ticket folder.
   status: completed
 - id: phase1-canvas
   content: "Phase 1: Expand CanvasExtension into a real paint/hit-test trait and move generic canvas concerns (VelloThemePalette, generic icon codec sans metabolism, world-raster tiling, scene cache, grid, node/handle/edge/wire/label paint primitives) from BoardHost into infinite/canvas; keep infinite_canvas + dependents compiling."
   status: completed
 - id: phase2-graph-engine
   content: "Phase 2: Grow mathematical/graph into the rich generic board engine by absorbing BoardHost graph logic (stores, kind catalogs, LinkCompatRule, sync_descriptor, parse_fixture_v1, selection, hit-testing, edge curves/tips, link gestures, pointer, drain_events, GraphPortMode); repoint port/directed/normal layouts; cargo tests green."
   status: completed
 - id: phase3-session-react
   content: "Phase 3: Provide a generic board BoardSession (engine + extension hook) and move the generic Puzzle2dRenderer/reconciler/text-overlay/CanvasWasmBridge mount into infinite/canvas/react-renderer as a generic GraphCanvas + hooks + fixture/scene builders."
   status: completed
 - id: phase4-slim-puzzle
   content: "Phase 4: Slim puzzle/2d/rs + puzzle/2d/react to generic-engine + Puzzle2dExtension (brush/fill/palette/metabolism/original-style) only; repoint all consumers (puzzle/5d, platform + playground renderers, mindmap, wires, sketchpad, storybook); full cargo + vitest + dev boots green."
   status: completed
 - id: phase5-dag-render
   content: "Phase 5a: In the dag crate add cdylib + DagExtension painting the IO-node spec (rectangle, horizontal input labels left, vertical center name, horizontal output labels right, port-routed cycle-guarded edges) and a DagSession wasm API; extend in-file dag tests."
   status: completed
 - id: phase5-dag-harness
   content: "Phase 5b: Add dag/react canvas host (single canvas, no overlay), dag/play playground specialization (DagPlayHost region, boot subpath, PUZZLE_PLAY_ENTRY=dag), and dag/fixture/*.dag.json."
   status: completed
 - id: phase5-dag-wiring
   content: "Phase 5c: Wire dev - vite-elements-assets kind/subpath/markers, root package.json workspace + dev:dag script, root script.ts dag case, and a launch.json dag dev entry (new port)."
   status: completed
 - id: phase6-validate-close
   content: "Phase 6: Validate runtime via [DEBUG] logs / Playwright probe (rectangle IO nodes with port labels + vertical name + edges on one Rust canvas); run all cargo tests + vitest + affected dev boots; close the ticket with file list + summary."
   status: completed
isProject: false
---

# Extract Generic Graph Canvas From Puzzle 2D, Add DAG Dev + Fixture

## Goal

Per the directive: puzzle/2d must keep ONLY puzzle-specific tooling (brush, fill, palette, metabolism icons). Everything generic (graph engine, node/handle/edge/wire model, kind catalogs, selection, hit-testing, link gestures, fixture parsing, rendering, camera/LOD/GPU) moves to the infinite canvas ([infinite/canvas](/Users/ueli/Documents/compose/infinite/canvas)) and the graph bundles ([mathematical/graph](/Users/ueli/Documents/compose/mathematical/graph)). DAG, being flow-agnostic, then gets its own Rust/vello renderer for the Node spec (rectangle: horizontal input labels left, vertical name middle, horizontal output labels right) on a single infinite canvas, with a playground dev harness and a fixture.

## Target architecture

- `infinite/canvas/vello` (`infinite_canvas`): owns ALL canvas concerns. Promote `CanvasExtension` from a marker into a real trait with paint + hit-test + interaction hooks. Move generic canvas pieces out of `BoardHost`: vello theme palette, generic icon codec (typst/emoji/raster/inline-SVG; not metabolism), world-raster tiling, scene cache, grid, plus reusable node/handle/edge/wire/label PAINT primitives. Camera, LOD, GPU session, text, geometry already live here.
- `infinite/canvas/react-renderer`: becomes the real r3f-like host it claims to be. Move the generic `Puzzle2dRenderer`/reconciler/text-overlay/`CanvasWasmBridge` mount from [puzzle/2d/react/index.tsx](/Users/ueli/Documents/compose/puzzle/2d/react/index.tsx) here as a generic `GraphCanvas` + session bridge.
- `mathematical/graph` (`mathematical_graph`): the thin `GraphEngine` is replaced/grown into the rich generic board engine absorbed from `BoardHost`: node/handle/edge/wire stores, kind catalogs (`NodeKindDef`/`HandleKindDef`/`EdgeKindDef`/`WireKindDef`/`EdgeTipDef`), `LinkCompatRule`, selection (rect/lasso/modes), hit-testing, `edge_curve`/tips, link gestures, pointer/drag, `drain_events`, `GraphPortMode`, `sync_descriptor`, `parse_fixture_v1`. Generic interaction stays free of brush.
- `puzzle/2d/rs` (`puzzle_2d`): slimmed cdylib = generic engine (rlib deps) + `Puzzle2dExtension` providing brush/fill/fixture-drop/palette/metabolism/original-element-style + the puzzle `BoardSession` exposing the brush/fill wasm methods. ~1,400 lines of brush/fill stay; ~5,800 lines leave.
- `puzzle/2d/react`: slimmed to the generic `GraphCanvas` + puzzle-only React (palette, brush UI, metabolism enrichment).
- `mathematical/graph/port/directed/dag`: gains a `DagExtension` with custom Rust node painting (rectangle + per-port horizontal labels + vertical center name + port-routed edges, cycle-guarded via existing `would_create_cycle`) and its own cdylib `BoardSession`, built on the generic engine. New `dag/react` host + `dag/play` harness + `dag/fixture/*.dag.json`.

```mermaid
graph TD
  canvas["infinite/canvas (canvas: camera, LOD, GPU, text, icons, theme, paint primitives, CanvasExtension)"]
  reactrenderer["infinite/canvas/react-renderer (generic GraphCanvas + session bridge)"]
  graph["mathematical/graph (generic board engine: nodes/handles/edges/wires, kinds, selection, hit-test, fixtures)"]
  normal["graph/port/directed/normal (DirectedPortGraphEngine, layouts)"]
  dag["graph/port/directed/dag (DagExtension + dag cdylib + react + play + fixture)"]
  puzzle["puzzle/2d (Puzzle2dExtension: brush/fill/palette/metabolism only)"]
  canvas --> graph
  canvas --> reactrenderer
  graph --> normal
  normal --> dag
  graph --> puzzle
  canvas --> dag
  canvas --> puzzle
```

## Guiding constraints

- Keep the repo green after EACH phase (cargo tests + vitest + dev boot). Move code in regions/subregions; no parallel/duplicate files, no migration shims, no legacy aliases (greenfield).
- Generic code must not reference puzzle/elements/metabolism. DAG stays flow-agnostic. All DAG UI is drawn in Rust on one canvas (no DOM overlay widgets).

## Phases

### Phase 0 - Ticket + baseline

- Read `repo://goals`, open a repo MCP ticket (e.g. `Extract Generic Graph Canvas From Puzzle 2D And Add Dag`); route temp logs/scripts into its folder.
- Record baseline: `cargo test -p puzzle_2d -p mathematical_graph -p mathematical_graph_port_directed -p mathematical_graph_port_directed_dag` and the puzzle/2d + wires + flow dev boots.

### Phase 1 - Canvas extraction into infinite/canvas

- Expand `CanvasExtension` ([infinite/canvas/vello/lib.rs](/Users/ueli/Documents/compose/infinite/canvas/vello/lib.rs) ~1114) into a real extension trait (node/scene paint hook + optional hit-test/interaction overrides) consumed by the generic engine.
- Move from `BoardHost`: `VelloThemePalette` (783-820), generic `board_icon_codec` minus metabolism (64-241), `world_raster_tiling`, scene cache, grid, and node/handle/edge/wire/label paint helpers into `infinite_canvas` modules.
- Verify `cargo test -p infinite_canvas` + dependents compile.

### Phase 2 - Generic graph engine into mathematical/graph

- Grow [mathematical/graph/lib.rs](/Users/ueli/Documents/compose/mathematical/graph/lib.rs) `GraphEngine` (or a new `BoardHost` there) to absorb the generic graph logic from `BoardHost` (243-6528): stores, kind catalogs + `LinkCompatRule`, `sync_descriptor` (4568), `parse_fixture_v1` (4766), selection/preselect (3627-3764), `resolve_hit_world` (4422), edge curves/tips, link gestures, pointer (5920-6290), `drain_events` (3570), `GraphPortMode`.
- Keep schema parsing generic in the graph crate (the `puzzle.2d.fixture/v1` reader becomes the generic board reader). Defer any schema rename to avoid churn.
- Move generic standalone fns (6564-6607) and re-point `mathematical/graph/port/directed/normal` layouts.
- `cargo test -p mathematical_graph -p mathematical_graph_port_directed` green.

### Phase 3 - Generic WASM session + React renderer

- Provide a generic board `BoardSession` (generic engine + extension hook). Decide host crate: keep one shared generic cdylib reused by puzzle/dag/wires/mindmap (preferred, mirrors how wires/mindmap already rebuild puzzle wasm), each domain adding a thin cdylib + extension.
- Move generic `Puzzle2dRenderer`/reconciler/text-overlay/`CanvasWasmBridge` mount from [puzzle/2d/react/index.tsx](/Users/ueli/Documents/compose/puzzle/2d/react/index.tsx) (12448+, 13005+, 6623+) into [infinite/canvas/react-renderer/index.tsx](/Users/ueli/Documents/compose/infinite/canvas/react-renderer/index.tsx) as a generic `GraphCanvas` + hooks; keep fixture/scene-descriptor builders generic.

### Phase 4 - Slim puzzle/2d + repoint consumers

- Reduce `puzzle/2d/rs` to generic-engine + `Puzzle2dExtension` (brush 2235-3322, fill 2584-2842, fixture-drop, palette/metabolism, original-element-style) + puzzle `BoardSession` brush/fill wasm methods.
- Reduce `puzzle/2d/react` to generic `GraphCanvas` + puzzle tools; keep `Puzzle2dFixtureV1`/kind-catalog exports where puzzle-owned, re-export generic ones from the new home.
- Repoint consumers: [puzzle/5d/react](/Users/ueli/Documents/compose/puzzle/5d/react/index.tsx), [framework platform renderer](/Users/ueli/Documents/compose/framework/product/platform/renderer/react/index.tsx), [framework playground renderer](/Users/ueli/Documents/compose/framework/product/playground/renderer/react/index.tsx), [mindmap react/play](/Users/ueli/Documents/compose/reasoning/mindmap/react/index.tsx), [wires react/play](/Users/ueli/Documents/compose/reasoning/mindmap/wires/react/index.ts), sketchpad + storybook aliases ([vite-elements-assets.ts](/Users/ueli/Documents/compose/ui/styling/vite-elements-assets.ts)).
- Full `cargo test` + all `vitest` + puzzle/2d, wires, mindmap, flow dev boots green.

### Phase 5 - DAG renderer + dev + fixture (the deliverable)

- In [mathematical/graph/port/directed/dag/lib.rs](/Users/ueli/Documents/compose/mathematical/graph/port/directed/dag/lib.rs): make crate-type `["rlib","cdylib"]`, add wasm deps, and implement `DagExtension` node painting per spec - rectangle body, input `IoPortSpec.label`s as horizontal text on the left edge, output labels horizontal on the right edge, `IoNodeSpec.name` as vertical (rotated) text in the middle, edges routed port-to-port via `io_node_handle_angles`, cycle-guarded by `would_create_cycle`. Add a `DagSession` wasm API (attach/size/render/pointer/camera/load-fixture). Extend the dag tests in-file.
- Add `dag/react` (canvas host over `GraphCanvas` with `DagSession`, single `<canvas>`, no overlay widgets), `dag/play` (playground specialization: `DagPlayHost` region in the playground renderer, boot subpath `@semio-tech/framework-playground-renderer-react/.../dag`, `PUZZLE_PLAY_ENTRY === "dag"`), and `dag/fixture/*.dag.json` (`dag.fixture/v1` of IO nodes + edges).
- Wire dev: extend [vite-elements-assets.ts](/Users/ueli/Documents/compose/ui/styling/vite-elements-assets.ts) `PlaygroundRendererPuzzleKind` + boot-subpaths + host markers; add root [package.json](/Users/ueli/Documents/compose/package.json) workspace + `dev:dag` script; add `dag` case in root [script.ts](/Users/ueli/Documents/compose/script.ts) DevScript; add a `dev` DAG entry in [.vscode/launch.json](/Users/ueli/Documents/compose/.vscode/launch.json) (new port, e.g. 6017) following existing grouping.

### Phase 6 - Validate + close

- Confirm runtime with temporary `[DEBUG]` logs / a Playwright probe in the ticket folder: dag dev renders rectangle IO nodes with left input labels, vertical center name, right output labels, and port-routed edges on one Rust canvas.
- Run all cargo tests + vitest + every affected dev boot. Close the ticket with the file list and summary.

## Key risks

- `BoardHost` (Rust) and `Puzzle2dCanvas` (React) are large monoliths with many consumers; extraction is sequenced so each phase keeps builds/tests green before the next.
- Promoting `CanvasExtension` to a real paint/interaction trait is the linchpin enabling both the slimmed puzzle extension and the DAG custom node rendering.
