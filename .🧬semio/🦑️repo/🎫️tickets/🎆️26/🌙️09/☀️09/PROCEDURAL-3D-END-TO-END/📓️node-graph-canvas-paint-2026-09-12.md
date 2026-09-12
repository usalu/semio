# Flow node-graph canvas — painting the real scene (2026-09-12)

Closes finding #1 and #2 of `📓️audit-flow-window-render-2026-09-12.md`: `render_frame` computed the real
vello `Scene` and discarded it, and the JS host painted a hard-coded black `fillRect` per widget at raw
world coordinates. Both are gone; the canvas now presents the picture the host's own LOD-aware vector
painter produces, camera and all.

Lane: the flow node-graph canvas paint path only. No `git` mutating command, no dev-server restart, no
wgpu-shell edits. The flow node-graph wasm (`@semio-tech/flow-core`, where `render_frame` lives) **was
rebuilt** — `bun nx run semio-framework-os-flow-core:wasm`, three times across the session, last at the
end of the lane; the running vite on 6018 picks the staged module up on reload.

---

## 0. Headline

| before | after |
|---|---|
| scene canvas: **0 ink pixels**, 1 distinct colour (untouched), or one solid black rectangle per widget at raw world coordinates | scene canvas: **468 618 ink pixels, 1 170 distinct colours, 0 pure-black pixels**, grid + node bodies + ports + wires + minimap, camera applied |
| `render_frame` dropped `scene` | `render_frame` **presents** it: WebGPU when the surface's canvas is bound to a device, otherwise the scene itself, encoded, for a 2D replay |
| `renderFlowCanvas` invented boxes | `renderFlowCanvas` replays the encoded scene; there is no placeholder branch left |

Live, `[DEBUG] flow surface context created 2d 864x1354 dpr=1 widgets=7 commands=47` — seven widgets,
forty-seven draw commands, all forty-seven replayed
(`🗑️generated/canvas-paint/after-default-wide/console.txt`).

---

## 1. What was added: a portable encoding of the scene the repo already builds

`Scene` is **already** a first-party command list in this repo — `Vec<SceneCommand>`, with `vello` reached
only at `Scene::vello_scene` on host/browser targets (`♾️infinite/🖼️canvas/🦀️.rs`, `vello_backend`'s own
docstring). The missing piece was never a renderer; it was an **exit** for that command list on a host
with no GPU device. So:

### 1.1 `canvas::draw_list` (`♾️infinite/🖼️canvas/🦀️.rs`, new `pub mod` inside `mod renderer`)

`write_scene_draw_list(&mut String, &Scene, DrawListOptions)` / `scene_draw_list_json(&Scene, …)` encode
every replayable command as one tagged flat JSON array:

```
{"version":1,"truncated":false,"commands":[
  ["f",rule,[r,g,b,a],[a,b,c,d,e,f],shape],                                   fill
  ["s",width,capStart,capEnd,[dash…],dashOffset,[r,g,b,a],[affine],shape],    stroke
  ["i",w,h,"<base64 rgba8>",[affine]],                                        image
  ["pl",rule,blend,alpha,[affine],clip], ["pc",rule,[affine],clip], ["po"]    layers
]}
```

Shapes stay **primitives** — `["r",…]` `["rr",…8]` `["ci",cx,cy,r]` `["ln",…]` `["cb",…8]` — and only
`Arc`/`BezPath` become a verb stream `["p",[0,x,y, 1,x,y, 2,…, 3,…, 4]]`. That is deliberate: a replaying
2D context then reaches its own `roundRect`/`arc`/`bezierCurveTo`, so a later zoom cannot facet a circle
(the same reason `RecordedShape` keeps shapes exact rather than flattening at record time), and the
payload stays small.

Three properties that matter:

- **The camera is already in it.** `DagHost::paint_scene` bakes `camera_content_affine(camera, viewport)`
  into every command's transform, so a replay applies the affine verbatim and never re-derives a camera.
  This is what kills finding #2 structurally rather than by patching the placeholder's coordinates.
- **Device-pixel scale is deliberately NOT baked in** (unlike `render::scale_scene_for_device_pixel_ratio`,
  which the GPU path still uses). The replayer supplies its own base transform, which keeps a draw list a
  pure function of scene + camera — and therefore directly comparable across implementations, which is
  what makes the conformance twin possible.
- **Zero new dependencies.** Numbers are formatted by hand; base64 (for the one command carrying opaque
  bytes) is fifteen first-party lines. `vello`/`wgpu`/`kurbo`/`peniko` are untouched and the module
  compiles on `wasm32-wasip2` too, where none of them exist.

Two additive accessors were needed to emit primitives instead of flattenings, both in
`📐️geometry/⚙️engine/🦀️.rs`: `RoundedRect::{rect,radii}`, `RoundedRectRadii::as_clockwise`,
`Circle::{center,radius}`, `Line::{p0,p1}`, `Arc::parameters`. Pure readers, no behaviour change.

### 1.2 `render_frame` now presents (`🌊️flow/🕸️wasm/🦀️component.rs`)

```rust
self.host.paint_scene(&mut scene, self.width, self.height, self.dpr);
let clear = self.host.dag.canvas_theme.raster_clear;
let presented = surface_canvas::present_surface_scene(surface.id.get(), &scene, clear, self.dpr);
…
if presented { payload.push_str(r#"{"version":1,…,"commands":[]}"#) }
else { canvas::draw_list::write_scene_draw_list(payload, &scene, …) }
```

The reply gained `present` (`"gpu"`/`"2d"`), `clear` (the host's own `raster_clear`, so the 2D path
grounds the canvas exactly as `vello::RenderParams::base_color` does on the GPU path) and `draw`.
`fixture` and `labels` are unchanged, so every existing consumer keeps working.

### 1.3 The WebGPU path — reused, not forked (`surface_canvas`, same file)

`CanvasGpuSession` (`♾️infinite/🖼️canvas/🦀️.rs`, `gpu_session`) is the browser vello/wgpu presenter every
other board surface in this repo already uses — `DagSession`, puzzle2d's `BoardSession`, the paint/map/
editor sessions. The flow session could not reach it for one reason: it speaks a **linear-memory byte
ABI** (`flow_bridge_send`/`flow_bridge_poll`), which cannot carry an `HtmlCanvasElement`.

So the canvas is bound **out of band**, keyed by the same `surface` id the ABI's `attachSurface` already
issues — no duplicated session state, no protocol change:

| export | role |
|---|---|
| `flowAttachSurfaceCanvas(surface, canvas, w, h, dpr) → Promise<boolean>` | binds the canvas to a `CanvasGpuSession`; resolves `false`, never rejects, when no device can be had |
| `flowResizeSurfaceCanvas(surface, w, h, dpr)` | swapchain resize, driven from the ABI's own `setSize` |
| `flowSurfaceCanvasPresentsOnGpu(surface)` | whether this surface is on the GPU |
| `flowDetachSurfaceCanvas(surface)` | released on `lost`/`device-lost`/`cancelled`/`rejected` |

`present_surface_scene` then rasterizes through the identical `render_frame(&scene, clear)` the dag
session uses, scaling for dpr only when dpr ≠ 1 so the common case costs no scene clone.

New browser-only deps on the flow crate: `js-sys`, `web-sys` (feature `HtmlCanvasElement`),
`semio-framework-async` (its owned `browser::future_to_promise`) — all already used by `♾️infinite` for
exactly this, all gated to `cfg(all(target_arch = "wasm32", not(target_env = "p2")))`.

### 1.4 The 2D replayer (`🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js`)

`replayFlowDrawList(context, drawList, base)` — exported, so it is testable on its own. Composes
`base × command.affine` itself (one `setTransform` per command instead of a save/transform/restore trio,
which also lets a clip pushed by a layer survive the next command), maps rule/cap/blend codes to their 2D
names, falls back to four exact corner arcs where `roundRect` is missing, and unwinds its own layer stack
before returning. `renderFlowCanvas` now: returns immediately when `present === "gpu"` (a canvas admits
exactly one context kind for its whole life, so it must not even ask for a 2D context there), otherwise
sizes the store, grounds it in the host's `clear`, and replays. **There is no third branch and no
placeholder.**

`attachFlowSurface` no longer acquires a device it never used. It calls
`flowAttachSurfaceCanvas` through `bindings` (the generated `flow_core.js` namespace — `source` carries
only the raw wasm exports, which cannot take an element) and returns `presentsOnGpu`. `navigator.gpu` is
checked in JS **before** the call: a wgpu bring-up failure inside the guest is an unrecoverable trap that
would take the whole session with it, so the overwhelmingly common deviceless case never reaches that
code. `🌐️flow-browser.js` threads `bindings` from `createFlowBrowserRuntime` to `FlowSession.attachCanvas`;
`🪪️WasmSessionLoader/🟦️.tsx` passes `core` (not `exports`).

### 1.5 One retained buffer in the guest

`FlowDomainAdapter.frame_payload: String` is cleared and refilled per frame. A guest that allocates a
fresh multi-kilobyte reply every frame fragments its own heap (dlmalloc hands nothing back) until it
aborts; this keeps the per-frame allocation count at one (the returned `Vec<u8>`, sized exactly) once the
buffer has reached its working size. `write_scene_draw_list` exists for this reason.

---

## 2. The second defect found on the way — and fixed

Wiring the real picture through made a pre-existing bug visible and fatal: **the canvas was painted at
most once per session, never after.**

`renderFlow` issued its render through `observeFlowTask(session, "renderCanvas", …)`, which keeps one task
per feature key and **cancels the previous one**. That is right for a query whose answer is superseded and
wrong for the paint: a render reply is what puts pixels on the canvas, and `syncSurfaceSize` has just
cleared the backing store. A burst of invalidations — mount, `ResizeObserver`, scene sync, theme sync,
all of which arrive together — therefore repeatedly emptied a canvas the engine had in fact painted.

Measured live before the fix (`🗑️generated/canvas-paint/diag-paint4/console.txt`):

```
15469 rf issue
15470 rf coalesce  … ×10
15627 dag draw lod=normal zoom=1.000      ← render_frame really ran
24674 flowpaint … commands=16 drawn=16    ← its reply, 9.2 s later
24718 dag draw lod=detail zoom=1.784      ← the re-issued render, now on the SYNCED graph
34174 rf reject Error: cancelled          ← killed by the next surface re-attach
```

Fix (`🕸️NodeGraph/🟦️.tsx`, `renderFlow`): an in-flight render is left alone; a request arriving during one
is remembered and re-issued when it settles. At most one extra frame, never a dropped one. The registry
entry (and therefore unmount cancellation) is unchanged.

Result, same probe after the fix: `flowpaint … 2d 47 47 7` at 54.8 s, 82 s, 119 s and 149 s — repeated
repaints of the real seven-widget graph.

---

## 3. Tests

### 3.1 Language-agnostic fixture + Rust law on the scene builder

`♾️infinite/🖼️canvas/🧪️tests/🎬️draw-list/🔣️.json` — one node-graph picture described once: camera
(`x=120, y=-40, zoom=1.25`), viewport, three grid lines, two cubic wires, three nodes, four ports, plus the
paint order and the camera law in its own `provenance` block.
`📐️expected-draw-list.json` — the encoding both implementations are pinned to (21 commands).
`🦀️.rs` paints that fixture with the canvas's own primitives (the same vocabulary `DagHost::paint_scene`
emits) and asserts:

```
test canvas::draw_list_tests::the_node_graph_fixture_encodes_to_the_shared_draw_list ... ok
test canvas::draw_list_tests::every_command_keeps_the_camera_affine_the_painter_baked_in ... ok
test canvas::draw_list_tests::a_zoomed_camera_moves_every_encoded_coordinate ... ok
test canvas::draw_list_tests::shapes_stay_primitives_so_a_replay_never_facets_a_circle ... ok
test canvas::draw_list_tests::a_scene_over_the_command_ceiling_truncates_loudly ... ok
test result: ok. 5 passed; 0 failed
```

### 3.2 vitest/node twin on the 2D fallback painter — draw calls, not pixels

`🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js` reads the **same two files** (from where the encoder lives, not a
copy) and replays the pinned draw list into a recording 2D context. It asserts the recorded call list:
ports → `arc` ×8, node bodies → `roundRect` ×6, wires → `bezierCurveTo` ×2, grid → `lineTo` ×3, exactly one
`clip`/`save`/`restore`, **zero `fillRect`** (the placeholder's signature); that every transform carries the
fixture's camera zoom and that a node body is *not* drawn at its raw world origin; that a device-pixel base
transform multiplies the camera rather than replacing it; and four refusals (unknown version, absent list,
absent context, unbalanced `po`) plus the `roundRect`-less `arcTo` fallback. Registered in
`semio-framework-os-flow-core:test-browser`:

```
[DEBUG] Flow draw-list replay reproduced every primitive of the shared fixture as its own 2D primitive
[DEBUG] Flow draw-list replay applied the camera affine and composed the device-pixel scale over it
[DEBUG] Flow draw-list replay refused every malformed list and unwound its own layer stack
NX   Successfully ran target test-browser for project semio-framework-os-flow-core
```

### 3.3 Laws over the REAL generation3d payload

`🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/{🔣️.json,🦀️.rs}` — the live `hexagonal-mushroom-column` flow fixture
(7 widgets / 6 synapses, committed as data with provenance, same precedent as the wgpu lane's copy;
re-validated by `FlowHost::parse_fixture_json` on both sides).

```
test wasm_session::draw_list_laws::the_generation_3d_flow_window_paints_a_scene_that_survives_encoding ... ok
test wasm_session::draw_list_laws::moving_the_camera_moves_the_encoded_picture ... ok
test wasm_session::draw_list_laws::render_frame_carries_the_draw_list_the_host_must_paint ... ok
test result: ok. 3 passed; 0 failed
```

The third drives the production `FlowDomainAdapter::render_frame` and asserts `present == "2d"`, a
non-empty `draw`, a four-component `clear`, and that `fixture`/`labels` survived. **Measured envelope at
966×814: 47 commands / 22 593 bytes** — two orders of magnitude under the 1 MiB ABI reply body, which is
what makes carrying the picture itself viable at all; the law pins it under 64 KiB.

### 3.4 Pinned in the React contract suite

`🔬️engine-contract/🟦️.ts`, hidden-tab attach lane: a frame carrying no draw list must now paint *nothing*
rather than invent a picture — `expect(fillRect calls).toEqual([])`.

### 3.5 Neighbouring suites

| command | result |
|---|---|
| `bun nx run @semio-tech/framework-renderer-react:test-long` | **965 passed, 3 failed** — `noteShellCommand`, `PluginRuntime` ×2, all peer lanes, all failing identically on the pre-change baseline taken at the start of this lane (`🗑️generated/canvas-paint/react-test-long.txt`, then 951/4 including a `🔀️surface-switch` failure its owner has since fixed) |
| `bun nx run semio-framework-os-flow-core:test-browser` | passed |
| `bun nx run semio-framework-os-flow-core:test-source` | **fails pre-existing** at `🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:36`, the Rust-source assertion `📓️node-graph-attach-2026-09-10.md` already flagged as not fixed |
| `cargo check -p semio-framework-os-flow --target wasm32-unknown-unknown` | 0 errors, 1 warning (the pre-existing unused `SpaceMember` import) |
| `cargo check -p semio-framework-os-infinite --lib --target wasm32-wasip2` | 0 errors — the guest target is untouched by the new module |
| `cargo check -p semio-framework-geometry` | 0 errors (its `--tests` build fails pre-existing on stray `.await`s in `🎲️random`'s suite, unrelated) |

### 3.6 Pixel-level smoke: no screenshot-diff pattern exists in this repo

Searched: no `toMatchImageSnapshot`, `pixelmatch` or `toHaveScreenshot` anywhere outside `node_modules`.
Rather than introduce a golden-PNG baseline (which would churn on every theme token change and cannot say
*why* it differs), `🐍️canvas-paint-probe.mjs` reads the canvas's own `getImageData` and asserts on
**pixel statistics**: ink coverage, distinct colour count, **pure-black pixel count** (the placeholder's
exact signature) and the ink bounding box. That is resolution- and theme-independent and names the two
failure shapes directly. Recommendation for whoever wants a golden image later: it belongs next to this
probe, not in vitest — jsdom implements no 2D context.

---

## 4. Runtime — 6018, before and after

Probe: `🐍️canvas-paint-probe.mjs` (new, this folder). Outputs under `🗑️generated/canvas-paint/`.

| run | scene canvas (index 0) | label canvas (index 1) |
|---|---|---|
| **before** (`🗑️generated/flow-window-audit/default/`, prior lane) | one solid black `fillRect` per widget at raw world coordinates | two node labels |
| **after, headless default, 1440×900** (`canvas-paint/after-default/`) | clear `221,218,205,255`, **393 022 ink px**, 810 colours, **0 pure-black px**, ink bounds the full 483×814 | 311 px |
| **after, headless default, 2560×1440** (`canvas-paint/after-default-wide/`) | clear `240,236,221,255`, **468 618 ink px**, 1 170 colours, **0 pure-black px** | 1 093 px |
| **after, `--enable-unsafe-webgpu --ignore-gpu-blocklist --use-angle=metal`** (`canvas-paint/after-webgpu/`) | `present=2d`, `commands=47 drawn=47 widgets=7` — see §4.1 | — |

`after-default-wide/4-zoomed-out.png` is the clearest frame: LOD grid, node bodies as rounded rectangles,
semicircular input ports, **wires** running between nodes, port labels, and the minimap widget with its red
viewport rectangle showing the whole seven-node column. `2-surface.png` in the 1440-wide runs shows the
same at the document's own camera.

**The black rectangle and the raw-coordinate placeholder are gone** — `pureBlackPixels: 0` in every run,
and the canvas geometry is unchanged (`data-surface-id`, both canvases `absolute inset-0`, same sizes as
the audit measured), so host-side picking keeps its contract.

### 4.1 Honest limit on the WebGPU half of the live check

The GPU path could **not** be exercised on the running 6018 serve, and the reason is worth recording
because it will bite the next lane too: vite resolves `@semio-tech/flow-core`'s entry through the
`node_modules` symlink, and the dev server's source watcher
(`🧑‍💻dev/…/🔌️vite-plugins.ts`, `UNWATCHED_REPOSITORY_SEGMENTS`) deliberately does not watch
`node_modules`. So a rebuilt `flow_core.js` — the file carrying the new wasm-bindgen wrappers — is never
invalidated, and `bindings.flowAttachSurfaceCanvas` is `undefined` in the page; `attachFlowSurface`
correctly falls back to the 2D replay, which is exactly what the `--enable-unsafe-webgpu` run shows.
Proven, not inferred: `🐍️flow-module-freshness-probe.mjs` (new) fetches every flow module the page loads
and reports whether the served bytes contain a given marker —

```
200 616614B marker=false  …/node_modules/@semio-tech/flow-core/flow_core.js     ← stale, never invalidated
200 203197B marker=true   …/🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js            ← fresh (real path, watched)
```

The GPU path will engage on the coordinator's next dev-server restart. It is covered meanwhile by the
attach laws (§3.2's four deviceless hosts plus the `navigator.gpu` guard law) and by the fact that
`CanvasGpuSession` is the identical presenter four other browser surfaces already ship.

Two further notes for anyone probing this serve: a `touch` (mtime only) does **not** invalidate a `.tsx`
module, a content write does; and the flow session answers a `renderFrame` behind whatever eval work is
queued — measured at **9–40 s** on a cold boot — so a probe that settles for 40 s reads a canvas that is
several syncs behind the outline tree.

---

## 5. Findings handed on (not this lane)

1. **`renderFrame` reply latency, 9–40 s.** `render_frame` executes promptly (its own
   `[DEBUG] dag draw lod=…` line is immediate) and the reply then queues behind `flowEvalTick`/extension
   round-trips on the single-threaded bridge pump. The canvas therefore lags the DOM label overlay, which
   paints synchronously. Not the payload: one 22 kB body is a single 64 kB page.
2. **The flow window remounts and re-attaches its surface every 20–40 s** during boot
   (`[DEBUG] node-graph attach called` at 14.4 s, 34.2 s, 114.8 s …), cancelling any render in flight. With
   (1) this is why the first correct frame can take a minute to appear.
3. **`RuntimeError: unreachable` in the guest at session teardown.** Observed twice, both times in the
   last frame before `Error: Flow session is closed`
   (`canvas-paint/diag-paint4/console.txt` 95 923 ms, `diag-paint5` 176 601 ms), never during steady-state
   painting, and it survived the retained-buffer change — so it is a close-path trap, not the reply
   payload. Not reproduced in the pre-change 40 s captures because none ran long enough to reach teardown.
4. **The scene carries glyph outlines** (`canvas::text::append_label` → `BezPath` fills) *and* the React
   label overlay paints the same strings with `fillText`. Both shells already do this; at the LODs measured
   the outlines are a small share of the 47 commands, but if label crispness is ever questioned, this
   double-paint is where to look.

---

## 6. Files

| file | change |
|---|---|
| `🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs` | additive shape accessors (`RoundedRect::{rect,radii}`, `RoundedRectRadii::as_clockwise`, `Circle::{center,radius}`, `Line::{p0,p1}`, `Arc::parameters`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs` | **new** `pub mod draw_list` (encoding, options, base64, shape/command writers), `pub use renderer::draw_list`, test mount |
| `…/♾️infinite/🖼️canvas/🧪️tests/🎬️draw-list/🦀️.rs` | **new** — 5 laws on the scene builder |
| `…/♾️infinite/🖼️canvas/🧪️tests/🎬️draw-list/🔣️.json` | **new** — the language-agnostic picture |
| `…/♾️infinite/🖼️canvas/🧪️tests/🎬️draw-list/📐️expected-draw-list.json` | **new** — the pinned encoding both sides read |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs` | `render_frame` presents (GPU or encoded draw list); **new** `surface_canvas` region (4 wasm-bindgen exports + non-browser stub); retained `frame_payload`; detach on lost/cancelled/rejected; swapchain resize from `set_size`; test mount |
| `…/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-async`; browser-only `js-sys`, `web-sys` (`HtmlCanvasElement`) |
| `…/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs` | **new** — 3 laws over the real generation3d payload |
| `…/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🔣️.json` | **new** — the committed `hexagonal-mushroom-column` flow fixture |
| `…/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js` | **new** — the JavaScript conformance twin |
| `…/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js` | `replayFlowDrawList` + helpers; `renderFlowCanvas` replays instead of inventing; `attachFlowSurface` binds the guest's presenter and returns `presentsOnGpu` |
| `…/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js` | `bindings` threaded through `createFlowBrowserRuntime` → `FlowSession.attachCanvas` |
| `…/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js` | attach laws refactored to the new contract (4 deviceless hosts + the `navigator.gpu` guard law) |
| `…/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts` | registers the twin in `test-browser` |
| `…/🌊️flow/🫀️core/🕸️bindings/*` | rebuilt (`…:wasm`) — `flow_core_bg.wasm`, `flow_core.js`, `🖥️flow-host.js`, `🌐️flow-browser.js`, declarations |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` | `renderFlow` coalesces instead of pre-empting (§2) |
| `…/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx` | passes `bindings: core` to the flow runtime |
| `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | no-placeholder assertion on the hidden-tab lane |
| `<ticket>/🐍️canvas-paint-probe.mjs` | **new** — pixel-statistics + hover/select + zoom probe |
| `<ticket>/🐍️flow-module-freshness-probe.mjs` | **new** — which flow modules the serve actually hands the page, and whether their bytes are the ones on disk |

Evidence: `🗑️generated/canvas-paint/{after-default,after-default-wide,after-webgpu,diag-paint2..5}/`
(`ink-*.json`, `interactions.json`, `console.txt`, `1-settled.png`, `2-surface.png`,
`3-after-interaction.png`, `4-zoomed-out.png`), plus `rust-laws-final.txt`, `rust-draw-list-law.txt`,
`rust-flow-draw-list-laws.txt`, `react-test-long.txt`, `baseline-wasm-check.txt`,
`flow-core-wasm-build.txt`.
