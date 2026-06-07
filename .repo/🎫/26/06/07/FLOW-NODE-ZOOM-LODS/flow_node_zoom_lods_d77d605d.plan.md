---
name: Flow Node Zoom Lods
overview: "Give DAG/flow nodes six zoom LOD tiers (mirroring puzzle 2d): zoomed out shows a horizontal centered name (or a bare block at the coarsest tier), and zooming in progressively reveals three visible sections (input | name | output), the rotated vertical name, port handle dots, and input/output port labels."
todos:
  - id: lod-bands
    content: Add DAG_LODS/DAG_LOD_SCALE, DagDrawLod enum + predicates, and pub dag_draw_lod(zoom) in dag/lib.rs Lod region
    status: completed
  - id: paint-branch
    content: "Branch paint_scene on LOD: horizontal name (overview/compact), section dividers + vertical name + handles (normal+), port labels (detail+), control/value text gating, minimap block, and [DEBUG] band-change log"
    status: completed
  - id: horizontal-helper
    content: Add paint_node_name_horizontal and paint_section_dividers helpers
    status: completed
  - id: flow-chrome
    content: Gate flow slider/note/preview chrome in paint_flow_widget_chrome behind Normal+ via dag::dag_draw_lod
    status: completed
  - id: tests
    content: "Extend dag tests: dag_draw_lod band mapping + paint_scene smoke at overview/micro zoom"
    status: completed
  - id: validate
    content: Rebuild wasm, run dev:flow + probe, confirm [DEBUG] LOD transitions and visual collapse/expand
    status: completed
  - id: ticket
    content: "When repo MCP is back: read repo://goals, open FLOW-NODE-ZOOM-LODS ticket under best goal, close with summary"
    status: completed
isProject: false
---

# Flow Node Zoom LODs

## Goal
Render DAG nodes (used by the flow canvas) at six camera-zoom LOD tiers like puzzle 2d, so far zoom collapses a node to a horizontal name and near zoom expands it into the three-section input/name/output layout with port labels.

## LOD scheme (reuse `infinite_cavas::lod`)
Declare the same six bands puzzle 2d uses, with identical thresholds, in the DAG crate. Node appearance per tier (zoom increasing):
- `minimap` (`<0.15`): rectangle fill only, no stroke/text/handles.
- `overview` (`<0.35`): rect fill+stroke + horizontal centered name. No sections, ports, or handle dots.
- `compact` (`<0.55`): same as overview (horizontal name), slightly larger text clamp.
- `normal` (`<1.25`): two visible vertical section dividers (input | name | output) + rotated vertical name in the middle + handle dots. No port labels yet.
- `detail` (`<2.5`): adds input/output port labels.
- `micro` (`>=2.5`): full fidelity (port labels + value/option/media hint text for slider/select/screen).

Handle dots (`snap.handles` loop) and per-kind controls (slider track, select control, value text) become gated by tier instead of always drawing.

## 1. Add LOD bands + resolver to the DAG crate
In [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs), add a new `// #region 🔖Lod`:
- `use infinite_cavas::lod::{Lod, LodScale};`
- `const DAG_LODS: &[Lod; 6]` (minimap/overview/compact/normal/detail/micro) with `max_zoom` `0.15 / 0.35 / 0.55 / 1.25 / 2.5 / f64::INFINITY` and id/name/description, plus `const DAG_LOD_SCALE: LodScale`.
- `pub enum DagDrawLod { Minimap, Overview, Compact, Normal, Detail, Micro }` with `from_scale_index(usize)` and helper predicates (`shows_sections`, `shows_port_labels`, `shows_handles`, `shows_name`, `name_is_vertical`).
- `pub fn dag_draw_lod(zoom: f64) -> DagDrawLod` calling `DAG_LOD_SCALE.resolve_index(zoom)`.

## 2. Branch `paint_scene` rendering on LOD
In `DagHost::paint_scene` (lines ~891-984):
- Resolve `let lod = dag_draw_lod(cam.zoom);` once.
- Skip the rect stroke + all text + handles for `Minimap` (fill only).
- Add helper `paint_node_name_horizontal(scene, center_screen, name, px, fill, halo)` that centers text (offset origin by approx `-len*px*0.62/2`, `-px/2`).
- For `Overview`/`Compact`: draw rect + `paint_node_name_horizontal` only.
- For `Normal`+: keep `paint_node_name_vertical`; add helper `paint_section_dividers(scene, aff, node, stroke)` drawing two vertical world-space lines insetting the name column (e.g. at `node.x +/- hw*0.4`), only when the node has inputs/outputs.
- Call `paint_port_labels` only when `lod.shows_port_labels()` (`Detail`/`Micro`).
- Slider/select/screen branches: draw their control geometry at `Normal`+ and their value/option/media-hint text at `Detail`+.
- Gate the trailing handle-dot loop (lines ~976-983) behind `lod.shows_handles()` (`Normal`+).
- Add a `[DEBUG]` band-change log: add a `Cell<i8>` field (e.g. `last_logged_lod`) to `DagHost`; in `paint_scene` log `"[DEBUG] dag draw lod=<label> zoom=<z>"` only when the resolved index changes (interior mutability, `paint_scene` stays `&self`).

## 3. Make flow chrome follow the same LOD
In [flow/core/lib.rs](flow/core/lib.rs) `paint_flow_widget_chrome` (lines ~686-746):
- Resolve `let lod = dag::dag_draw_lod(cam.zoom);` (re-export already available via `pub use ... as dag`).
- Only paint slider track/thumb/labels, note text, and preview text at `Normal`+ so the flow overlay matches the node's three-section appearance and disappears when nodes collapse to a horizontal name.

## 4. Tests (extend existing `#region Tests`, no new files)
- DAG crate test: assert `dag_draw_lod` maps sample zooms to the expected band (`0.1`->Minimap, `0.3`->Overview, `0.5`->Compact, `1.0`->Normal, `2.0`->Detail, `5.0`->Micro).
- DAG crate smoke test: `paint_scene` runs without panic at a low zoom (overview) and high zoom (micro) for the demo fixture.

## 5. Runtime validation
- Run `dev:flow` and the existing probe [.repo/🎫/26/06/07/FLOW-RUNTIME-LOADABLE-MODULES/validate-flow-runtime.mjs](.repo/🎫/26/06/07/FLOW-RUNTIME-LOADABLE-MODULES/validate-flow-runtime.mjs) (extend it inside the ticket folder) to wheel-zoom the canvas and confirm the `[DEBUG] dag draw lod=...` band transitions, verifying nodes collapse to a horizontal name when zoomed out and expand into sections + port labels when zoomed in. The DAG playground ([mathematical/graph/port/directed/dag/play/index.ts](mathematical/graph/port/directed/dag/play/index.ts)) shares this renderer and will reflect the same behavior.

## Ticket / process
- The repo MCP server is currently erroring, so opening the ticket and reading `repo://goals` is blocked. Once it is available: read `repo://goals`, open a ticket (e.g. slug `FLOW-NODE-ZOOM-LODS`) under the most appropriate goal, keep all temp probe edits inside the ticket folder, and close it with a summary on completion.
- No `launch.json` change needed (pure rendering change; `dev:flow` already exists). WASM rebuild of `flow_core`/dag pkg required for the browser to pick up the Rust changes.

## Notes / decisions
- Implemented in the shared DAG renderer per your choice, so both flow and the DAG playground get LODs.
- Thresholds reuse puzzle 2d's exact values for cross-surface consistency; flow's default camera zoom of `1.0` lands in `normal`, so the default view keeps the three-section layout.