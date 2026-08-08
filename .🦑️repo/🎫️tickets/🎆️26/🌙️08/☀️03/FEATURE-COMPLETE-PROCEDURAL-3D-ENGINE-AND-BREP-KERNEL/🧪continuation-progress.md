# FEATURE-COMPLETE-PROCEDURAL-3D — continuation progress

Date: 2026-08-07

## Verified runtime

### Playground `:6018` (`dev:procedural:3d`, `SKIP_PLUGIN_BUILD=1`)
- Full 8-example probe **all green** (`example-shots-run23-full.log`):
  hex, rect-extrude, sphere-cut, fillet, fuse, face-sweep, wire (edges-only mesh), shell.
- Generate mode: layout OK; **Add Generation works** → `Generation 1` + HEIGHT/RADIUS/SIDES form (`playground-generate-add.log`).
- No `Invalid 3 panel layout` after Canvas `defaultLayout` Record fix.

### Demonstrator `:6029` Generator pane
- **Layout crash fixed** (`Invalid 3 panel layout: 68%, 32%` gone).
- Three panels: Generationen / Formular / Vorschau.
- Empty preview no longer crashes on `session.attachCanvas` (editor stub replaced; TextEditor + GraphWasmCanvas hardened; real `framework_editor_bg.wasm` built).
- **Add Generation E2E green** with Generator booting live `procedural3d` via `#generator` + cleared storage (`add-generation-by-id-run6.log`, `demonstrator-generator-run12.log`): Generation 1 + HEIGHT/RADIUS/SIDES, no generate Renderfehler.
- Landing change in `♻️mit-bestand/.../demonstrator/📦️index.tsx`: Generator pane uses `resolvePlaygroundBoot("procedural3d")` / `pluginFilter=procedural3d` so it does not depend on the stale Aug-4 demonstrator wasm. Other panes stay on the demonstrator bundle.
- Full demonstrator wasm rebuild remains blocked by puzzle/gis compile errors (needed if we want aggregator/other panes refreshed from source).

## Code fixes this continuation
- Canvas: `defaultLayout` as panel-id Record; axis group id includes child count/sizes; stale 2-key resize events ignored; safe `setLayout`.
- ResizablePanelGroup: reject array `defaultLayout`.
- TextEditor: ignore wasm session without `attachCanvas`.
- GraphWasmCanvas: guard missing `attachCanvas`.
- `buildEngineWasm` freshness: require `framework_editor_bg.wasm` (not stub JS).
- Built real `@semio-tech/framework-editor-rs` pkg (~24MB wasm).
- Demonstrator Generator boots procedural plugin module.

## Kernel progress this continuation
- Boolean mesh fallback now prefers `solid_from_triangle_soup` over convex hull (keeps non-convex cuts); section docstring honesty fixed.
- Fillet/chamfer rebuild via arc/inset **triangle strips** (soup first, hull fallback). Native `semio-s-3d` lib tests still **402 passed**.


## Kernel progress (continued, Aug 7 evening)
- Imprint same-edge double-hit now re-resolves the survivor edge after the first `split_edge`.
- Facade honesty: `arc_curve` → trimmed NURBS arc; `interpolate`/`approximate` → NURBS; `nurbs_surface_from_grid`/`coons_patch` → real NURBS surfaces; `curve_curvature` uses analytic/finite-diff curvature; `validate` returns structured JSON; `deconstruct` returns vertices+edges+faces; `solid_face_loops` restored to CAD wire API `(positions, face_loops)`.
- SSI sampling fallback emits degree-1 NURBS through hit points (not a collapsed line).
- `split_solid_by_plane` prefers classified triangle soups.
- `BrepDocumentOpEngine` handles box/sphere/cylinder/validate.
- Native `semio-s-3d` lib tests: **407 passed** (`native-wave-tests-run29.log`).

## Still open (plan waves B/C + close)
1. BREP fillet/chamfer still approximate strip-MVP (`🎨️blend`); true rolling-ball topology surgery pending.
2. Boolean still mesh-classify+soup for general contact (AABB fast paths + soup stitch); full imprint→stitch topology surgery pending.
3. STEP Unsupported composite branches + rational degree elevation still open.
4. Rebuild demonstrator wasm once puzzle/gis compile (optional for Generator; required for other panes freshness).
5. `ticket_close` when repo MCP is available (absent this session).

## Probe artifacts
- `example-shots-run23-full.log` / `example-shots-report.json`
- `demonstrator-generator-run6`…`run12`, `playground-generate-add.log`
- `add-generation-by-id*.log`, `framework-editor-wasm-build.log`
- `plugin-stage-demonstrator-layout.log` (failed: puzzle)
