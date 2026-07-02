---
name: Layer Graph Crates
overview: Purge all graph logic from infinite_cavas, build a composable graph crate chain that mirrors the folder structure (graph -> port -> port/directed -> {normal, dag}) by splitting the board monolith down by layer, and make puzzle/2d depend only on the directed-port-normal leaf.
todos:
  - id: cavas-purge
    content: "Phase A: remove vcompute, scene_json, board_json_*, normalize_board_descriptor, CanvasNodePaint from infinite_cavas; rename board_icon_* to icon_*; keep camera/lod/text/raster/gpu/CanvasExtension/geom_sel; verify gis/map + mindmap + wires compile."
    status: completed
  - id: graph-geometry
    content: "Phase B: move graph geometry + generic CameraJson/NodeDescJson into mathematical_graph; drop pub mod board_host re-export; keep generic GraphEngine/GraphExtension."
    status: completed
  - id: port-crate
    content: "Phase C: create mathematical_graph_port at graph/port/ (handles/ports layer: HandleDescJson/HandleData/HandleKindDef/NodeKindHandleTemplate); repoint port/undirected to it."
    status: completed
  - id: port-directed-crate
    content: "Phase D: create mathematical_graph_port_directed at graph/port/directed/ (directed board base: engine aliases, Edge, directed scene descriptors, edge/board types, LinkCompatRule, layouts, GraphExtension re-export)."
    status: completed
  - id: normal-leaf-rename
    content: "Phase E: rename port/directed/normal crate to mathematical_graph_port_directed_normal; move BoardHost monolith + puzzle.2d.fixture specifics here; re-export the base."
    status: completed
  - id: puzzle-repoint
    content: "Phase F: make puzzle/2d depend on and import only mathematical_graph_port_directed_normal (re-export undirected layouts from the leaf)."
    status: completed
  - id: dag-repoint
    content: "Phase G: point dag at the new port/directed base; drop puzzle-specific layout imports."
    status: completed
  - id: verify-close
    content: "Phase H: update launch.json/Cargo.lock; run cargo test + vitest + dev boots (puzzle, dag) with [DEBUG]/Playwright validation; route temp files into the existing ticket folder; reopen and close the ticket."
    status: completed
isProject: false
---

# Layer Graph Crates: Canvas/Graph Separation + Composable Chain

## Goal (three directives)
1. `infinite_cavas` carries zero graph logic (no nodes/handles/edges/board/scene-descriptors).
2. `puzzle/2d` imports only the directed-port-normal leaf crate.
3. Graph crates compose along the folder tree: `graph -> port -> port/directed -> {normal, dag}`, with the board monolith split down by layer.

## Target dependency graph

```mermaid
graph TD
  cavas[infinite_cavas: canvas only]
  graph[mathematical_graph: generic GraphEngine + graph geometry]
  port[mathematical_graph_port: handles/ports - NEW]
  portUndir[mathematical_graph_port_undirected]
  portDir[mathematical_graph_port_directed: directed board base - NEW at port/directed/]
  normalLeaf[mathematical_graph_port_directed_normal: BoardHost + puzzle.2d.fixture - RENAMED leaf]
  dag[mathematical_graph_port_directed_dag]
  normalUndir[mathematical_graph_normal_undirected]
  normalDir[mathematical_graph_normal_directed]
  puzzle[puzzle_2d]

  graph --> cavas
  port --> graph
  portUndir --> port
  portDir --> port
  portDir --> normalUndir
  normalLeaf --> portDir
  dag --> portDir
  normalUndir --> graph
  normalDir --> graph
  puzzle --> normalLeaf
```

Key rename: the crate name `mathematical_graph_port_directed` moves from the leaf `port/directed/normal` to a NEW intermediate crate at `port/directed/`. The leaf is renamed to `mathematical_graph_port_directed_normal`. Because `dag` already imports `mathematical_graph_port_directed` for the engine/types, that import keeps working against the new base.

## Phase A - Purge graph logic from infinite_cavas
In [infinite/cavas/vello/lib.rs](infinite/cavas/vello/lib.rs) remove and relocate (to Phase B/D):
- `pub mod vcompute` (handle/edge geometry: `handle_position_on_circle`, `circle_handle_angle_toward`, `rectangle_handle_angle_toward`, `compute_edge_bezier_points`, `distance_point_to_cubic_bezier`, `distance_between`, `encode_board_stroke_scene`).
- `pub mod scene_json` + its re-export line (`CameraJson`, `NodeDescJson`, `HandleDescJson`, `EdgeDescJson`, `WireDescJson`, `SceneDescriptorJson`, `FixtureJson`, `fixture_edge_handle_ids_from_object`).
- `board_json_visible_option`, `board_json_visible_or_true`, `board_json_hidden_flag`, `normalize_board_descriptor_hidden_to_visible`.
- `trait CanvasNodePaint` (graph-specific paint).

Keep (canvas concerns, used by gis/map + mindmap): `vello`/`usvg` re-exports, `text`, `camera`, `lod`, `raster`, `canvas_content`, `gpu_session`, `icon_codec`, `theme`, `svg_icon_vello09`, `CanvasExtension`, `CanvasEngine`, and `geom_sel` (verify it is generic selection geometry; keep). Rename board-named-but-generic icon items (`board_icon_assets` -> `icon_assets`, `usvg_options_board_icons` -> `usvg_options_icons`).

Verify [gis/map/rs/lib.rs](gis/map/rs/lib.rs), [reasoning/mindmap/lib.rs](reasoning/mindmap/lib.rs), [reasoning/mindmap/wires/lib.rs](reasoning/mindmap/wires/lib.rs) still compile (they only touch retained canvas APIs).

## Phase B - mathematical_graph (generic) owns graph geometry + generic descriptors
[mathematical/graph/lib.rs](mathematical/graph/lib.rs):
- Add a `geometry` region with the functions moved from cavas; repoint internal refs (currently `cavas::vcompute::*`, `cavas::{compute_edge_bezier_points, distance_point_to_cubic_bezier, encode_board_stroke_scene}`) to local.
- Add generic `CameraJson` + `NodeDescJson` (port/edge-agnostic scene base).
- Remove `pub mod board_host;` and `pub use board_host::*;` (the monolith leaves this crate).
- Keep `GraphEngine<P,D>`, `Node`, `Handle`, `Camera`, `Selection`, `RenderSnapshot`, `BoardEvent`, `GraphPortModel`, and `GraphExtension` (it bounds `cavas::CanvasExtension`).

## Phase C - NEW crate mathematical_graph_port at graph/port/
Create `mathematical/graph/port/Cargo.toml` (name `mathematical_graph_port`, dep `mathematical_graph`) and `lib.rs` (`pub use mathematical_graph::*`). Move here the port/handle layer pulled from the monolith: `HandleDescJson`, `HandleData`, `HandleKindDef`, `NodeKindHandleTemplate`, and handle helpers. Repoint [mathematical/graph/port/undirected/Cargo.toml](mathematical/graph/port/undirected/Cargo.toml) + [lib.rs](mathematical/graph/port/undirected/lib.rs) to depend on `mathematical_graph_port`.

## Phase D - NEW crate mathematical_graph_port_directed at graph/port/directed/
Create `mathematical/graph/port/directed/Cargo.toml` (name `mathematical_graph_port_directed`, deps `mathematical_graph_port` + `mathematical_graph_normal_undirected` for force layout) and `lib.rs`. This is the shared directed-port base for both `normal` and `dag`. Move here:
- `DirectedPortGraphEngine`/`BoardEngine` type aliases, `Edge` type, `resolve_endpoint_node_id`.
- Directed scene descriptors: `EdgeDescJson`, `WireDescJson`, `SceneDescriptorJson`, `FixtureJson`, `fixture_edge_handle_ids_from_object`, `board_json_visible_*`, `normalize_board_descriptor_hidden_to_visible`.
- Directed edge/board types from the monolith: `EdgeData`, `EdgeKindDef`, `EdgeTipDef`, `EdgeStrokePattern`, `EdgeTipGeometry`, `WireData`, `WireKindDef`, `LinkCompatRule`, `CompatSpecificity`, `GraphPortMode`, plus shared node/style types (`NodeShape`, `NodeData`, `NodeKindDef`, `BoardElementStyleKind`, `VelloThemePalette`, `SelectionOptions`, `ActiveTool`, `Interaction`).
- The `force_graph`, `hierarchical_tree`, `redraw_layout` modules currently in the leaf [lib.rs](mathematical/graph/port/directed/normal/lib.rs), and the `GraphExtension` re-export.

## Phase E - Rename leaf to mathematical_graph_port_directed_normal (BoardHost home)
- Rename [mathematical/graph/port/directed/normal/Cargo.toml](mathematical/graph/port/directed/normal/Cargo.toml) to `mathematical_graph_port_directed_normal`; dep becomes the new `mathematical_graph_port_directed` base at `path = ".."`.
- Move the `BoardHost` monolith (the whole [mathematical/graph/board_host.rs](mathematical/graph/board_host.rs), the struct + impl, `parse_fixture_v1` for `puzzle.2d.fixture/v1`, `sync_descriptor`, brush/fill, pointer, paint, kind catalogs glue) into this leaf as `board_host.rs`; `lib.rs` does `pub use mathematical_graph_port_directed::*` + `pub mod board_host; pub use board_host::*`. The struct stays whole here (Rust cannot split one inherent impl across crates); only its building-block types/functions are distributed downward in Phases B-D.

## Phase F - Repoint puzzle/2d to the leaf only
- [puzzle/2d/rs/Cargo.toml](puzzle/2d/rs/Cargo.toml): drop `mathematical_graph`, `infinite_cavas`, `mathematical_graph_normal_undirected`; the lone graph dep is `mathematical_graph_port_directed_normal` (keep `gis_map`, `reasoning_mindmap`).
- [puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs): replace the three direct imports with re-exports from `mathematical_graph_port_directed_normal` (which transitively re-exports `cavas`, the engine, layouts). The leaf must re-export the undirected layout fns puzzle uses for mindmap/wires fixtures (`apply_undirected_*`) so puzzle needs no direct undirected dep.

## Phase G - dag against the base
- [mathematical/graph/port/directed/dag/Cargo.toml](mathematical/graph/port/directed/dag/Cargo.toml): point `mathematical_graph_port_directed` to `path = ".."` (the base) instead of `../normal`; drop now-unneeded direct `mathematical_graph` dep if the base re-exports it.
- [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs): drop puzzle-specific layout imports (`apply_edge_handle_snap_*`, `apply_redraw_layout_*`); import only engine/types from the base.

## Phase H - Wire, verify, close
- Update [.vscode/launch.json](.vscode/launch.json) and any rs debug configs that reference the old crate path/name. Rebuild to refresh `Cargo.lock`.
- Run `cargo test` across all graph crates + `puzzle_2d` + `dag`; run affected vitest; boot `dev:puzzle` and `dev:dag`, confirming runtime via `[DEBUG]` logs / the existing Playwright probe.
- Route all temp logs/probes into the existing ticket folder `.repo/🎫/26/06/07/EXTRACT-GENERIC-GRAPH-CANVAS-FROM-PUZZLE-2D-AND-ADD-DAG/`; reopen that ticket via repo MCP rather than opening a new one; close with the full file list when green.

## Risks
- The board monolith split touches ~6.3k lines; do it phase-by-phase keeping `cargo test` green after each crate, since a single inherent impl cannot span crates (struct stays in the leaf, only supporting items move down).
- The crate-name handoff (`mathematical_graph_port_directed`: leaf -> intermediate) must land atomically with the dag/puzzle Cargo edits or the workspace will not resolve.