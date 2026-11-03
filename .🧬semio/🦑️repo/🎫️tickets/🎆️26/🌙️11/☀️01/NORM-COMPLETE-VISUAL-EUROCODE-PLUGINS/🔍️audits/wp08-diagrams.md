# WP-08 2D Diagrams Window (A8)

## Delivered
- `NORM/🎬️structural-scene/📈️diagram-kit/` (mounted as `pub mod diagram_kit` in the crate root, kit has no dependency on `Diagram2d`):
  `📉️curves` (sampling with exact breaks, log space, EN 1993-1-1 6.49 chi, ISO 834 / external / hydrocarbon fire curves),
  `🧿️interaction-polygon` (convex hull, signed area, containment, utilisation ray U=|OA|/|OB|),
  `📜️spectra` (EN 1998-1 3.2 elastic and design spectra, TB/TC/TD exact samples and markers),
  `🌀️sn-curves` (EN 1993-1-9 detail category, knees at 2e6/5e6/1e8, cut-off, allowable cycles),
  `🧱️stress-blocks` (EN 1992-1-1 3.1.7 rectangular block, lambda/eta, eps_cu3, parabola-rectangle).
  Tests: `🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🥒️.feature`, oracle `🧪️tests/🔮️oracle/🐍️.py` (numpy + shapely).
- `SURFACE/🪟️visual-windows/📈️diagrams2d/` (mounted as `pub mod diagrams2d`):
  `🖼️canvas-lowering` (Diagram2d to Canvas2d `layers_json`: nice 1-2-5 and decade ticks, linear/log/sqrt, grid with minors, polylines, polygons/bands with default hatch per tone, scatter with per-tone marker shapes, limits, rays to boundary, section outline with strain/stress profiles and neutral axis, markers with utilisation labels, notes, legend, clipping, Okabe-Ito colours, dash redundancy, selected/hovered emphasis),
  `🎯️hit-test` (pick targets in pixel space, priority marker > point > legend > line > area, `DiagramSelection` with bound check and focus entity, tooltip, keyboard `focus_order`/`step_focus`),
  `📊️table-twin` (locale-formatted `TableTwin`, marker table, accessible description),
  parent `🦀️.rs` (locale number/percent formatting: decimal comma in DE, thin-space grouping; `diagram_scene` returning `Canvas2dScene` + targets).
  Tests: `🧪️tests/🦀️.rs`, `🧪️tests/🥒️.feature`, `🧪️tests/🔮️oracle/🐍️.py` (shapely distances, numpy logspace/arange), golden `🧪️tests/🧫️fixtures/🔣️layers.json`.

## Notes and hand-offs
- Layer JSON follows the drawing-contract node form (`transform`, `node{kind:path|text}`, `fill`, `stroke{dash}`). Verify with the Canvas2d host once A7/A6 wire windows.
- Surface3 series are skipped in canvas lowering (3D sub-surface belongs to the viewport window); Sqrt/Log domains are resolved from data.
- Direct manipulation of field-bound markers: `Marker.binding` is carried but dragging is not wired (A10 handle machinery).
- Mount line applied in `SURFACE/🦀️.rs` and logged in `📌️important/mounts.md`; the kit mount is logged in `requests.md`.
- Oracle script spawned as `python3 -I` like other norm tests (needs numpy/shapely on PATH python = locked `.venv`).

## Verification status
Nothing was compiled or run (repo MCP hung; `repo run` queued >30 min and the first attempt failed with an unknown-project id). Per the coordinator, builds are left to the integrator via `@semio-tech/norm-artifact-contract-rs`; compile notes are in `📌️important/requests.md`. My queued build shells were cancelled and `🗑️generated/wp08` was deleted. Only the kit's Python oracle was run standalone.
`🧪️tests/🧫️fixtures/🔣️layers.json` is a placeholder `{}`: regenerate once with `SEMIO_UPDATE_GOLDEN=1`.
