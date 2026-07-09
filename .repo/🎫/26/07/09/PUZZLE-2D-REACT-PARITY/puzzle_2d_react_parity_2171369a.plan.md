---
name: Puzzle 2D React Parity
overview: Restore premigration parity for the puzzle 2D React renderer by fixing a critical camera-transform sign bug that pushes all rendered content off-canvas, aligning the overview pane's pointer hit-testing with what is actually rendered, and giving the generic Canvas2d layer schema real shape/color/selection fidelity.
todos:
 - id: ticket
   content: Check repo://goals and open/reopen a ticket for Puzzle 2D React parity
   status: completed
 - id: part-a-camera
   content: Fix camera translate sign + dpr mixing in canvas-2d-host.tsx JsonLayersCanvasSession, add pure transform helpers + tests
   status: completed
 - id: part-a-wheel
   content: Implement cursor-anchored wheel zoom matching infinite_cavas::camera::wheel_screen
   status: completed
 - id: part-b-sync
   content: Sync BoardHost camera/size to the render camera before handling canvasPointerDown/Move/Up/Wheel in d2/mod.rs
   status: completed
 - id: part-b-raw-coords
   content: Dispatch raw CSS-pixel pointer coords from canvas-2d-host.tsx instead of pre-converted world coords
   status: completed
 - id: part-b-tests
   content: "Add d2/mod.rs test: click at rendered node position selects it"
   status: completed
 - id: part-c-color-role
   content: Emit color/role fields for node/handle/edge/wire layers from kindCatalogs (with deterministic fallback) in d2/mod.rs
   status: completed
 - id: part-c-draw
   content: Draw real circle/rect shapes, selection highlight, handle styling, edge vs wire dashing in canvas-2d-host.tsx
   status: completed
 - id: part-c-tests
   content: Extend index.test.ts and d2/mod.rs tests for new color/role/selected fields
   status: completed
 - id: validate
   content: Run vitest + cargo test, rebuild plugin, manually verify in running dev server, close ticket with summary
   status: completed
isProject: false
---

# Puzzle 2D React Parity Restoration

## Empirically confirmed root cause (Part A) — canvas renders nothing visible

Opened the running dev server (`http://127.0.0.1:6012/`, terminal already running `bun run dev:puzzle:2d`) and loaded the "Concrete Forest" example. The Document panel correctly lists the one fixture node ("Hexagonal Cut Concrete Forest Left"), but the canvas shows only the checkerboard background — exactly the "no canvas is showing at all" symptom.

By monkey-patching `CanvasRenderingContext2D` draw calls in the live page I confirmed [framework/renderer/react/components/canvas-2d-host.tsx](framework/renderer/react/components/canvas-2d-host.tsx) _is_ computing and issuing a `strokeRect`/`fillText` for the node — just at device pixel `(3430, 1650)` on a `1099×1159` canvas, i.e. **far outside the visible viewport**.

Root cause: `JsonLayersCanvasSession.renderFrame()` does

```12:1:framework/renderer/react/components/canvas-2d-host.tsx
ctx.translate(width / 2 + this.camera.x * this.dpr * zoom, height / 2 + this.camera.y * this.dpr * zoom);
ctx.scale(zoom * this.dpr, zoom * this.dpr);
```

i.e. `screenX = width/2 + zoom·dpr·(camera.x + worldX)` — it **adds** `camera.x` instead of subtracting it. Every other camera implementation in this codebase uses the canonical subtractive form:

```2872:2878:mathematical/graph/port/directed/normal/rs/lib.rs
pub fn world_to_screen(&self, p: Point) -> Point {
    infinite_cavas::camera::world_to_screen(&self.camera, &self.viewport(), p)
}
```

```1551:1557:infinite/cavas/rs/lib.rs
pub fn world_to_screen(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
    Point::new((p.x - camera.x) * camera.zoom + viewport.width as f64 / 2.0, ...)
}
```

Since `puzzle/plugin/rs/d2/mod.rs::puzzle2d_pane_camera` always centers the camera on the fixture's content bounds (never `(0,0)`), this sign error means **any non-empty fixture** renders off-screen. It only looked "fine" for the default "Empty" example because `camera = (0,0)` happens to make the sign irrelevant.

A secondary, currently-latent bug: `renderFrame`/`toCanvasCoords` mix CSS-pixel pointer input with device-pixel (`canvas.width`, already `dpr`-scaled) canvas dimensions. Invisible at `dpr=1` (the test browser) but wrong on real HiDPI screens.

### Fix

- Rewrite the camera math in [canvas-2d-host.tsx](framework/renderer/react/components/canvas-2d-host.tsx) to work entirely in logical/CSS pixel space (`this.logicalWidth/logicalHeight`), applying `dpr` once via a leading `ctx.setTransform(dpr,0,0,dpr,0,0)`, then `translate(logicalWidth/2 - camera.x*zoom, logicalHeight/2 - camera.y*zoom)` + `scale(zoom,zoom)` — matching the canonical formula exactly.
- Fix `toCanvasCoords` to the matching inverse, and flip the pan-drag sign in `pointerMove` so dragging still feels natural under the corrected transform.
- Implement cursor-anchored wheel-zoom, matching `infinite_cavas::camera::wheel_screen` (`camera.x = world_before.x - (sx - width/2)/next_zoom`), instead of the current zoom-in-place.
- Extract the point transforms into small pure exported functions and add cases to the existing `Canvas2dHost` test in [framework/renderer/react/index.test.ts](framework/renderer/react/index.test.ts) (do not add a new test file) asserting a node centered under the camera maps inside the viewport.

## Part B — Overview-pane clicks/drag target the wrong world position

`canvas-2d-host.tsx` pre-converts pointer coordinates to "world space" via `toCanvasCoords` (using the _framing_ camera, `Canvas2dScene.cameraX/Y/zoom`, which `puzzle2d_pane_camera` computes purely from content bounds) before dispatching `canvasPointerDown/Move/Up/Wheel`. But [puzzle/plugin/rs/d2/mod.rs](puzzle/plugin/rs/d2/mod.rs)'s handlers feed that value straight into `BoardHost::pointer_down_screen`, which performs its _own_ independent screen→world conversion using `self.host.camera` (defaulted, only touched via the separate `setCamera` command) and a hardcoded fixed viewport:

```334:336:puzzle/plugin/rs/d2/mod.rs
fn sync_host_from_envelope(host: &mut BoardHost, envelope: &Puzzle2dPlayEnvelope) {
    host.set_size(BOARD_DEFAULT_WIDTH, BOARD_DEFAULT_HEIGHT, 1.0);
```

The two cameras/viewports never agree (the render camera is deliberately zoomed/framed to fit content; the interactive host camera starts at `(0,0,1)` against a fixed `1024×768`), so hit-testing never resolves to the coordinates that were actually rendered — clicking a visible node will not select it.

### Fix

- In `d2/mod.rs`, immediately before handling `canvasPointerDown/Move/Up/Wheel` (guarded by `puzzle2d_pointer_pane_is_interactive`, the only interactive pane), sync `self.host` to the exact frame that was rendered: `self.host.set_size(width_from_args, height_from_args, 1.0)` and `self.host.set_camera(cx, cy, zoom)` from `puzzle2d_pane_camera(&envelope.fixture, PUZZLE2D_PANE_OVERVIEW)`.
- In `canvas-2d-host.tsx`, stop pre-converting: dispatch the _raw_ CSS-pixel `x, y` (already available in `pointerDown/Move/Up`) instead of `toCanvasCoords(x, y)` — Rust now performs the (correctly synced) conversion itself. `width`/`height` are already included in the dispatched payload; just need to be read on the Rust side.
- Extend the existing `#[cfg(test)] mod tests` in `d2/mod.rs` with a case that dispatches `canvasPointerDown`/`Up` at the screen position corresponding to a known node (given the render camera) and asserts it becomes selected.

## Part C — Visual fidelity: real shapes, selection highlight, kind color

`node_canvas_layer`/handle-layer/`edge_line_layer` in `d2/mod.rs` only emit generic bounds (`{id, kind: shape, x, y, width, height, selected}`); `canvas-2d-host.tsx`'s fallback drawing path ignores `selected` entirely (not even in the `CanvasLayerRecord` type) and always draws a rectangle regardless of `kind`, colored by array _index_ (`hsla(index*47%360,...)`) rather than anything meaningful — so a circle node renders as an arbitrarily-colored square, and selecting it produces no visual feedback. Edges and wires also both flow through the same line layer with no visual distinction.

### Fix

- `d2/mod.rs`: add `color` (from `kindCatalogs` when the fixture declares one — confirmed schema already supports per-kind hex `color`, see `puzzle/2d/rs/lib.rs` catalog tests — else a deterministic hash-of-kind-id fallback, not the current index-based rainbow) and a `role` (`"node" | "handle"`) discriminator to node/handle layers, and an edge-vs-wire role to `edge_line_layer`.
- `canvas-2d-host.tsx`: extend `CanvasLayerRecord` with `color`/`role`/`selected`; draw an actual filled circle (`ctx.arc`) when `kind === "circle"` vs. a rounded rect otherwise; use `layer.color` for fill when present; draw a distinct accent stroke + halo when `layer.selected`; render `role === "handle"` layers smaller/dimmer than nodes; dash wire lines vs. solid edge lines.
- Extend the existing tests in `index.test.ts` and `d2/mod.rs` to assert the emitted JSON carries `color`/`role`/`selected` correctly for a selected circle node.

## Validation

- Check `repo://goals` and open (or reopen an existing) ticket for this work before editing, per repo workflow.
- `bun nx test framework-renderer-react` for the camera-math + `Canvas2dHost` fidelity tests.
- `cargo test -p puzzle-plugin` for the `d2` pointer-sync + layer-color tests.
- Rebuild the puzzle2d plugin WASM and use the already-running dev server (`bun run dev:puzzle:2d`, `http://127.0.0.1:6012/`) to load Concrete Forest and, with temporary `[DEBUG]`-prefixed logs (removed before finishing), confirm: the node renders as a visible colored circle inside the canvas, clicking it selects it (Document + Inspection panels reflect the selection), dragging moves it, and wheel-zoom stays anchored under the cursor.
- Close the ticket with a summary of every file touched.
