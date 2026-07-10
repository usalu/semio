# Puzzle 5D React Parity — Verify Log

## Baseline diagnosis (pre-change, dev server :6014)

- 2D window (generic `canvas-2d` host): scene reached the host with correct camera + 12 layer records, but `layerBounds` requires `width`/`height` and d5 emitted circles with only `x/y/radius` → every layer fell into the corner-label fallback (`fillText("circle", …)` off-screen). Canvas showed only the checkerboard.
- 3D window: rendered after GPU wake (in-app browser suspends WebGL when backgrounded → transient "THREE.WebGLRenderer: Context Lost"); seed mesh + grip spheres visible, but no gumball/pick/context-menu (selection JSON lacked `granularity/transformTool/gumballActive/…`).
- Nakagin example could not parse at all: d5 struct expected `attracting`/`attracted` on fasteners while both 5d fixtures use `source`/`target` (premigration naming) → fell back to Concrete Forest.
- Round-tripping the document through `setDocument` dropped `part.3d.label`, `part.2d.iconKind`, `document.meta`, `document.label` (struct was lossy).
- Engine scene config passed the 5d `kindCatalogs` (with `grips` templates) straight to the puzzle-3d engine (which expects `objects` with `vortices`) → brush candidates/preview could never resolve.
- Brush/fill adoption (`fixture_from_engine_json`) reset every part's `2d` aspect to default → fill/brush wiped the flat board.

## Native tests

- `cargo test -p puzzle-plugin d5` → 32/32 passed (board projection, board events, gumball selection JSON, worldPick, setHover, transform tool, context menu, duplicate/zoom, engagements, measures, catalogue derivation, mixed selection classification, patchPart/patchGrip, merge preserves flat aspects, delete cascades, relocate proximity fastener, camera routing, nakagin parse).
- `cargo test -p puzzle-plugin` full: d5 32/32; two failures in d2/d3 are from another session's in-flight working-tree changes (`d2::renders_distinct_canvas_per_pane`, `d3::brush_placement_control_lists_free_candidates`; their diff, not touched by this ticket).
- `bun ./script.ts test --run` in framework/renderer/react → 47/47 passed.

## Wasm round-trip harness (`wasm-verify.ts`, transpiled component via `loadPluginModule`)

- 2D body renders a `puzzle2dBoard` scene (1 node Concrete Forest; 180 nodes / 179 edges Nakagin).
- Board `select` event → 3d selection with `gumballActive: true`, `transformTool: "move"`, `activeObjectId`, context menu with `duplicateSelection`/`zoomToSelection`.
- Board scene `selectionJson` mirrors the selection (paired 2d↔3d selection).
- 3d `setHover` → board scene `hoveredId` (paired 3d→2d hover).
- Fill tool → slider engagement; measures 3 (2d: LOD + suggestion + brush) / 2 (3d).
- Document tree has Fasteners section; catalogue derives Parts/Grips/Fasteners/Ropes from `kindCatalogs`.
- Brush candidate toggle-group stays pending until real meshes register (`registerBrushMesh` arrives from the browser at runtime; box fallback has no free candidates for the seed) — same tolerance as the puzzle-3d parity ticket.

## Browser verification (dev server :6014, react renderer)

- Blocked temporarily by another session's in-flight raster wiring (`@semio-tech/raster-rs` import in os-shell before workspace registration); resumed after their wiring landed.
- See screenshots/notes below.
