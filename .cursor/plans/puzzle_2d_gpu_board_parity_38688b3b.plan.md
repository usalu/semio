---
name: Puzzle 2D GPU Board Parity
overview: Root-cause the slowness and wrong/non-LOD styling by mounting the real WASM/WebGPU `BoardSession` (already built, currently unused) directly on the puzzle-2d canvas, exactly mirroring the just-finished GIS 2D architecture, instead of the JS-side JSON-layers Canvas2D reimplementation.
todos: []
isProject: false
---

## Root cause

The current puzzle 2D React path (`canvas-2d-host.tsx`) re-parses a hand-rolled JSON layer list every animation frame and redraws it with plain `CanvasRenderingContext2D` calls. This is a from-scratch, partial reimplementation of rendering that already exists, fully-featured, in Rust:

- `[puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs)` defines `BoardSession` (wasm-bindgen) whose `renderFrame()` is _purely_ GPU:

```554:558:puzzle/2d/rs/lib.rs
    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }
```

`render_frame_gpu` calls `host.build_vector_scene()` (full LOD, theme colors, bezier edges, icons — the exact fidelity that's missing today) and presents it via `cavas::gpu_session::CanvasGpuSession` (a real `wgpu` WebGPU surface). This compiles to `@semio-tech/puzzle-2d-rs` (`puzzle/2d/rs/script.ts` → `bun script.ts wasm`) but **nothing in the framework currently imports it** — it's dead code.

- The plugin (`puzzle/plugin/rs/d2/mod.rs`) instead serializes a crude flat `canvas_layers_json` (id/kind/color/xy/wh) every render, and the previous ticket's `canvasPointerDown/Move/Up/Wheel` handlers round-trip _every pointer event_ through the WASM plugin boundary, replaying it against a second `BoardHost` and re-serializing the whole scene — this is the performance bottleneck and the reason styling can never match the Rust renderer's real fidelity.

This is exactly the situation GIS 2D was just in, and it was just fixed by mounting the real `MapSession` (`@semio-tech/gis-2d-rs`) directly via `[gis-map-host.tsx](framework/renderer/react/components/gis-map-host.tsx)`. Puzzle 2D needs the same treatment: mount `BoardSession` directly, let it own rendering + pointer/camera interaction at native GPU speed, and only sync discrete committed events back to the plugin's document.

Confirming this won't affect other apps: `canvas-2d`/`Canvas2dScene` is a **generic, shared** component kind also used by note, draw, mindmap, layout, lowpoly, raster-placeholder, presentation and procedural plugins (verified via grep for `"canvas-2d"`). Only puzzle 2D (`d2/mod.rs`) will switch to a new dedicated `puzzle2d-board` kind — `canvas-2d-host.tsx` and everyone else stays untouched.

## Architecture (mirrors GIS 2D)

```mermaid
flowchart LR
    Plugin["d2/mod.rs plugin\n(document truth: fixture, selection, undo)"]
    Scene["Puzzle2dBoardScene\n(fixtureJson, cameraJson, kindCatalogsJson, selectionJson, interactive)"]
    Host["Puzzle2dBoardHost.tsx\n(new React component)"]
    Session["BoardSession (wasm)\nattach_canvas -> WebGPU"]
    Canvas["HTMLCanvasElement"]

    Plugin -->|render| Scene --> Host
    Host -->|syncDescriptorJson / setCamera / setKindCatalogsJson| Session
    Session -->|renderFrame each RAF, GPU| Canvas
    Canvas -->|pointerDownScreen/MoveScreen/UpScreen/wheelScreen\n(overview pane only)| Session
    Session -->|drainEventsJson: camera/select/nodeDragEnd/...| Host
    Host -->|dispatch "applyBoardEvents" eventsJson| Plugin
```

Puzzle 2D renders 3 panes (Overview/Detail/Selection) from one fixture. Only Overview is interactive (`puzzle2d_pointer_pane_is_interactive` already encodes this); Detail/Selection mount their own `BoardSession` too (for real GPU fidelity) but skip pointer wiring — same as static "camera-framed previews" today, just GPU-rendered.

## Concrete changes

**1. `[framework/core/rs/lib.rs](framework/core/rs/lib.rs)`** — add a new surface kind, mirroring `GisMapScene`/`build_gis_map_scene` (lines 2142-2177, 2409-2436, 2876+):

- `SurfaceKind::Puzzle2dBoard` (`#[serde(rename = "puzzle2d-board")]`).
- `Puzzle2dBoardScene { fixture_json, camera_json, kind_catalogs_json, selection_json, interactive: bool, ... }`.
- `build_puzzle2d_board_scene(...)`, and add one more `None` param to the shared `component_scene(...)` helper + its ~9 existing call sites.

**2. `[framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx)`** — mirroring `GisMapScene`/`createMapSession` (lines 2300-2340, 2929-2945):

- `Puzzle2dBoardScene` TS type + `puzzle2dBoard?: Puzzle2dBoardScene` on `UiComponentSceneNode`.
- `createPuzzle2dBoardSession()` dynamically importing `@semio-tech/puzzle-2d-rs/pkg/puzzle_2d.js`, returning `new mod.BoardSession()`.

**3. New `framework/renderer/react/components/puzzle-2d-board-host.tsx`** — mirroring `[gis-map-host.tsx](framework/renderer/react/components/gis-map-host.tsx)` almost exactly:

- Mount `<canvas>`, `attach_canvas(canvas, w, h, dpr)`, RAF loop calling `renderFrame()` only (cheap/GPU).
- Sync on scene change: `syncDescriptorJson(fixtureJson)`, `setKindCatalogsJson`, `setCamera`, and canvas theme via the existing `syncSessionCanvasTheme`/`useCanvasThemeSync` helpers from `@semio-tech/ui-styling` (already generic — `BoardHost` shares `CanvasThemePalette` with the DAG/normal graph engine these were built for).
- When `scene.interactive`: forward raw pointer/wheel events straight to `pointerDownScreen/pointerMoveScreen/pointerUpScreen/wheelScreen`; after each gesture, call `drainEventsJson()` and `dispatch("applyBoardEvents", { eventsJson })`.
- When not interactive (Detail/Selection panes): render-only, no listeners.

**4. `[framework/renderer/react/ui-interpreter.tsx](framework/renderer/react/ui-interpreter.tsx)`** — register `case "puzzle2d-board": return <Puzzle2dBoardHost .../>` next to the existing `"gis2d-map"` case (line 61).

**5. `[puzzle/plugin/rs/d2/mod.rs](puzzle/plugin/rs/d2/mod.rs)`**:

- `render_canvas` (line 937): emit `build_puzzle2d_board_scene(...)` instead of `build_canvas_2d_scene`; drop `canvas_layers_json` (line 808, now dead).
- Delete `sync_host_render_frame` (928) and `puzzle2d_pointer_pane_is_interactive` (922) and the 4 `canvasPointerDown/Move/Up/Wheel` match arms (1851-1921) — interaction moves client-side.
- Refactor `apply_host_events(host: &mut BoardHost, envelope)` (384-478) into `apply_board_events_from_json(events_json: &str, envelope: &mut Puzzle2dPlayEnvelope)`: the per-event match arms (394-465) are untouched (they only read `payload`, not `host`); drop the trailing `host.selection`/`host.camera` re-sync (467-477) since browser-drained events already self-describe every change (`camera`, `select` arms already exist).
- Add one new command arm: `"applyBoardEvents" => { apply_board_events_from_json(events_json, &mut envelope); vec![set_document_op(&envelope)] }`.
- Keep `sync_host_from_envelope`/`self.host`/`puzzle2d_engagement` untouched — still used for engagement-panel LOD/status text and default camera framing, unrelated to per-frame rendering.
- Update the two Rust tests currently exercising `canvasPointerDown`/`canvasPointerUp` (~~2221-2224) and the `canvas_layers_json` unit test (~~2181) to instead exercise `applyBoardEvents` and assert `"puzzle2d-board"` appears in the rendered scene JSON (replacing the `"canvas-2d"` assertion at line 2040).

**6. Build/deps wiring**:

- Add `"@semio-tech/puzzle-2d-rs": "workspace:*"` to `framework/renderer/react/package.json` (matching `gis-2d-rs`), run `bun install` to update `bun.lock`.
- Register `puzzle/2d/rs`'s `bun script.ts wasm` build in whatever aggregate dev-build step already builds `gis/2d/rs` (check `framework/product/os/dev/script.ts`) so the pkg is produced automatically.

## Verification

- Rebuild `puzzle/2d/rs` wasm package, run `cargo test -p puzzle-plugin` (note pre-existing unrelated `component_export_anchor` macro error from the last ticket — re-check if still blocking) and the updated Vitest suite.
- Manual check in the dev server: Overview pane pans/zooms/selects/drags at native frame rate with correct theme colors, LOD-driven detail, and bezier edges matching premigration; Detail/Selection panes render the same fidelity read-only.
- Reopen ticket `2026/07/09/PUZZLE-2D-REACT-PARITY` (per AGENTS.md, same task) rather than opening a new one, and close it with a summary + full file list when done.
