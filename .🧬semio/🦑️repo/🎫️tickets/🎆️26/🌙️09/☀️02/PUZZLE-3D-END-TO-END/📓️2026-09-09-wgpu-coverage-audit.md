# Wgpu Coverage Audit — Puzzle3D End-to-End (2026-09-09)

## Verdict (lead)

**Use the React path (`ShellHost`/`World3dHost`, port 6013) to verify "every window and every tool of puzzle3d works", not the wgpu path (port 6113) — today.**

The wgpu retained renderer is **not a stub**: it has a real, substantial `world3d_states` surface implementation (GLB asset streaming, orbit-camera authority, bespoke pointer/wheel dispatch, sun-lit instanced mesh drawing with per-instance `selected`/`hovered` GPU state) and a **fully native, non-React** retained widget toolkit (`UiNode::{Slider,NumberStepper,Select,Tree,...}`) that renders the fill slider, steppers, selects, trees, panels and chrome directly in Rust — it does **not** fall back to React's `UiNodeView` for chrome. That is a genuinely pure-wgpu chrome path, more complete than the question's framing assumed.

But two concrete, confirmed gaps make it unusable for a full puzzle3d tool-by-tool pass right now:

1. **The 3D transform gumball has zero Rust/wgpu implementation anywhere in the repo.** Grepped every `.rs` file under the renderer and `ui` module trees for `gumball` — every hit is in a React `.tsx` file (`🏛️ShellHost/🟦️.tsx`, `🌐️World3dHost/🟦️.tsx`, `📌️ChromePanels/🟦️.tsx`, the react-target package). The *button* that activates "transform" mode is generic and wgpu-tested (`UtilityNode` wire format), but the on-canvas draggable gizmo that actually performs the transform does not exist outside React/three.js.
2. **`puzzle3d` has zero bespoke wiring in the wgpu renderer crate.** The crate depends directly on the `puzzle` plugin crate (`package = "semio-s-plugin-puzzle"`), but every one of its ~10 call sites is `puzzle::editor::puzzle2d::engine::...` (board authority, wheel/pointer-down/up/move/leave) — grepping the same 14186-line file for `puzzle3d`/`puzzle_3d` (case-insensitive) returns nothing. Puzzle3D on wgpu is entirely dependent on the *generic* world-3d protocol working end-to-end; unlike puzzle2d, nothing in the renderer special-cases it, so its correctness has not been separately proven by anything this audit found.

Ranked effort to close the gap (see §5 for detail): implement the 3D transform gizmo (gumball) in the native wgpu scene layer is the largest missing piece; brush/volume-brush *preview* rendering inside the 3D viewport is unconfirmed and needs a dedicated check; marquee/box-select inside a 3D viewport was not found either (only 2D ink/map marquee exists natively). Everything else asked about (fill slider, steppers, selects, trees, context menu, panels, GLB mesh instancing, orbit camera, sun lighting, asset streaming) has real, cited native code.

---

## 1. ComponentKind / world-3d coverage in wgpu

### React dispatch table (baseline)
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:287-319` — `resolveComponentSceneHost(kind: ComponentKind)`, a `switch` over 14 kinds (`canvas-2d`, `world-3d` → `World3dHost` at line 291-292, `node-graph`, `text-editor`, `table`, `paint-2d`, `tiled-map`, `board-2d`, `icon-render`, `ink-canvas`, `graph-timeline`, `block-list`, `diff-view`, `event-feed`).

`World3dHost` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`, 5323 lines) imports `three`/`GLTFLoader` at lines 10 and 13 — it is a **fully independent WebGL/three.js implementation**, wired only into the React `ComponentSceneHostRegistry`. It has no wgpu-target sibling directory (unlike other elements — see below).

### wgpu side: real, but structurally different
The wgpu renderer crate (`semio-framework-os-renderer-wgpu`, Cargo name confirmed at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml:2`) does not use a `ComponentKind` enum/switch analogous to React's. Its 14186-line lib body (`🧊️renderer/🦀️.rs`, wired in via `include!` at `📚️library/🦀️.rs:8`) declares per-element modules with `#[path]` into sibling `🧱️elements/<Element>/🎯️targets/🧊️wgpu/🦀️.rs` files — i.e. some framework elements have **both** a React `.tsx` and a Rust wgpu-target implementation, others (like `World3dHost`, `🎛️UtilityTree`, `📌️ChromePanels`) have **only** the `.tsx`.

The wgpu-side analog is a data-carrying `SurfaceKind` enum plus per-surface state maps on `ShellState`:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2240` — `pub world3d_states: AdmittedSurfaceMap<World3dState>` (alongside `node_graph_states`, `tiled_map_states`, `board2d_states`, `icon_render_states` — no `canvas2d_states`/`paint2d_states`/`text_editor_states` etc. found as bespoke maps).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1281-1283` — `scene_has_bespoke_pointer_dispatch(kind: SurfaceKind) -> bool { matches!(kind, SurfaceKind::World3d | SurfaceKind::NodeGraph | SurfaceKind::TiledMap | SurfaceKind::Board2d) }`, documented as the exclusion list so these 4 kinds are driven "directly by the OS event loop" instead of the generic scene handlers.
- Main renderer file, `🧊️renderer/🦀️.rs:108-116`: imports `begin_world3d_dynamic_retirement, enqueue_world3d_event, finish_world3d_asset, publish_world3d_asset_mesh_lease, ..., step_world3d_draw_rebuild, step_world3d_interaction, step_world3d_snapshot, ...` from `infinite_world::world` (i.e. the shared `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` crate) — real GLB asset streaming (`RendererAssetFormatCursor::Glb` / `GlbStructureCursor`, lines 370-489, validates the glTF binary header/chunk structure byte-by-byte), a per-frame `World3dSnapshot` transaction phase (lines 11390-11434: `step_world3d_draw_rebuild` → `step_world3d_snapshot`), and orbit-camera authority dispatch (lines 11394-11521, wheel/pointer handling explicitly excluded from the generic path per the bespoke-dispatch list above).
- Orbit camera parity: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8977-9019` converts between `OrbitController` and `TutorialCameraState::Orbit` for tutorial/keyframe recording — a real, working orbit camera model.
- Sun lighting: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:557,632-644,6468-6480` — `WorldEnvironmentSunRecord { enabled, azimuth, elevation }`, resolved into the scene-pass light direction. Real.
- **Actual GPU mesh instancing lives one layer deeper**, in the shared `semio-framework-ui` (`ui_wgpu`) crate, not in the renderer crate itself: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🖥️gpu.rs:301` — `ensure_mesh_step(&mut self, key, version, lease: kernel_3d_scene::Mesh3dLease)`; `✍️draw.rs:2614,2626,3093` — `World3dGpuInstance::from_instance(model, color, instance.selected, instance.hovered)`, i.e. per-instance selection/hover state is threaded all the way into the GPU instance buffer. This is real instanced-mesh rendering, not a placeholder.
- **The generic widget-tree paint step for any `ComponentScene` node is a placeholder**, however: `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1226-1277` (`render_component_scene_step`) only walks `surface_id`/`controller_id` byte-by-byte (bounded-fuel cursor discipline) then, at phase 3, paints **one flat rounded rectangle** (`ctx.draw.push_rounded(..., ctx.theme.panel, ...)`) and finishes — it never touches mesh/camera data. The actual pixel content for `world-3d` is composited separately (via the `ui_wgpu` GPU pipeline above, keyed off `world3d_states`), not through this widget-tree call — worth knowing if anyone goes looking for "the" scene-paint function and stops here.

**Picking**: no ray-cast/hit-test against 3D geometry was found (` grep -i "pick\|ray_cast\|hit_test" ` across `🖥️gpu.rs`/`✍️draw.rs` returns nothing 3D-specific; the only `hit_test`/`hit_test_node`/`hit_test_subtree` functions, `🎯️targets/🧊️wgpu/🎯️events.rs:114-246`, are 2D widget-tree hit-testing for the panel/chrome UI, not 3D scene picking). Instances do carry `selected`/`hovered` flags into the GPU buffer, but this audit found no evidence of where those flags get *set* from a 3D pointer ray in the wgpu path — they may only ever be set from document/selection JSON pushed by the plugin, never computed client-side. Not confirmed either way; flag as open.

**Conclusion for §1**: `world-3d` is *not* absent or a pure stub in wgpu — there is a real, generic (not puzzle3d-specific) native implementation of GLB instancing, orbit camera, sun lighting and asset streaming. But it is also not proven feature-complete: no gumball, no confirmed 3D picking/marquee, and puzzle3d itself has no bespoke wiring anywhere in this crate (contrast with puzzle2d's ~10 `puzzle::editor::puzzle2d::engine::...` call sites in the same file, e.g. `🧊️renderer/🦀️.rs:11335,13459,13550,13602,13607`).

---

## 2. Chrome widgets: pure wgpu or hybrid React fallback?

**Pure wgpu — not hybrid.** The wgpu-target `Interpreter` element has its own complete retained widget system, independent of React's `UiNodeView`/`Interpreter/🟦️.tsx:960-1080` fallback:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1-9` (file docstring): *"framework/products/os/modules/renderer/engine/elements/🟦️Interpreter/component.rs — wgpu interpreter implementation ... Maps framework UiNode trees to ui_wgpu widget nodes."*
- Lines 1587-1607 — `ui_node_kind_tag(node: &UiNode)`, an **exhaustive** match over `Stack, Text, Button, Separator, Input, Select, Toggle, KeyValue, Slider, NumberStepper, Ring, IconSelect, Field, Section, Group, Tree, Image, ComponentScene, ExternalSlot` (a new `UiNode` variant fails to compile until wired here, per the doc comment).
- Lines 1512-1518 — important scoping note: *"shell chrome/navbar/footer/dock are rendered by this crate's own immediate-mode widgets code directly into the composited canvas frame, never through `UI_ENGINE` at all"* — i.e. there are actually **two** native (not React) chrome layers: the retained `UiNode` tree (panels/inspection/artifact/catalogue/settings — the ones with `UiNode::Slider`/`NumberStepper`/`Tree`) and a separate immediate-mode layer for navbar/dock/footer, both wgpu-native.

**Fill slider on the wgpu path, concretely**: the puzzle3d schema declares `fillCount: Int!` (`✏️editor/🎚️config/🧬️schema/🔣️.json:11,65`; `.../🔗️.graphql:8`; `.../🟦️.ts:16,213`). The plugin's utility-ribbon items (select/brush/transform/fill) are emitted as `UtilityNode`s through the shared, schema-first wire format — confirmed cross-target-tested for wgpu specifically at `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-utilities-utility-node-wire-format/🦀️.rs:27,40,65-74` (`derive_utility_nodes(...)`, asserting `Toggle`/group hoisting for `"transform"`/`"brush"` ids). Combined with `UiNode::Slider`/`NumberStepper` being natively rendered (§ above), a user on the wgpu path would: click/drag the same slider widget, painted and hit-tested entirely by Rust (`🎯️events.rs` hit-test + the retained `UiNode::Slider` paint code), which dispatches a `setFillCount` action — the puzzle3d editor's own action handling (`✏️editor/🦀️.rs:2462,3056,5934,5996,6006,6632,6729,6738,7054,7231`, `set_fill_count::begin/step`) is renderer-agnostic (it runs inside the plugin's wasm component either way), so this part should work identically to React.

Where it diverges from "works" is anything requiring the **3D gumball** (missing, §1/§5) — e.g. "transform" utility mode has a working toggle button but no on-canvas manipulator to actually drag.

---

## 3. Native wgpu app (`🛠️dev🧩️puzzle🏙️3d🧊️wgpu🖥️native`)

`bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle3d` resolves through:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📋️project.json:163-176` — `native` target, `nx:run-commands`, `"command": "bun ./📜️script.ts native dev"`, `forwardAllArgs: true`.
- `.../📜️script.ts:317-347` (`NativeRunScript.run`) — `filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || "s"` → `"puzzle3d"`; runs `bun <os-dev script.ts> plugin puzzle3d` with `SEMIO_RENDERER: "wgpu"`, then launches `semio-wgpu-native` with `["--plugin", "puzzle3d", ...appArgs]` (line 343). `resolveNativeAppArgs(catalog, filterPlugin)` (line 338) pulls the app-selection args from the shared playground catalog.
- The native binary is built from the **same** `🧊️renderer/🦀️.rs` lib body (via the `[[bin]]` target in the same `Cargo.toml`, path `💾️binary/🦀️.rs`, sharing `ui_wgpu`/`ui_host` etc.) — i.e. it is not a separate, lesser implementation; it shares 100% of the component coverage (and the same gaps) analyzed in §1/§2, just running through `winit` instead of a browser canvas/OffscreenCanvas.
- `⌨️native-entrypoint/🦀️.rs:88` — `let plugin_filter = arg_value("--plugin").unwrap_or_else(|| "studio".to_string());` — confirms `--plugin puzzle3d` (injected by script.ts) is what actually selects the plugin; a bare positional arg passed directly to the compiled binary (bypassing script.ts) would be silently ignored, since this file never reads a bare positional — worth knowing if anyone tries to invoke `semio-wgpu-native` directly instead of through `bun nx run ... native`.

**Conclusion for §3**: identical component coverage to the browser wgpu path; same verdict and same gaps (§1, §5) apply.

---

## 4. Build health

Read-only check at audit time (**2026-09-09 00:19 CEST**), no build/test run performed:

- `semio-framework-ui-backend-webgpu` (`🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/`): 8 files (`🌆️scene_target.rs`, `🌐️backend.rs`, `🔌️gpu_context.rs`, `🧮️gpu_uniforms.rs`, `🎞️frame.rs`, `📈️buffers.rs`, `🔤️gpu_types.rs`, `🗃️resources.rs`, `🚦️surface_state.rs`, `🧊️surface_adapter.rs`) all mtime **2026-09-08 20:04:25/26** — a single bulk edit ~4h15m before the audit, **not** hot (>15 min). `git status --porcelain` on the directory is clean; last commit touching `🔌️gpu_context.rs` is `025ec86a4` at 2026-09-08 20:58:18.
- `raw_display_handle` fix (findings §38) confirmed present and applied: `🔌️gpu_context.rs:103` — `raw_display_handle: Some(wgpu::rwh::WebDisplayHandle::new().into())`.
- `🌉️ProgramBridge` wgpu target (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`): mtime **2026-09-08 22:28:09**, ~1h51m before the audit — also not hot. `git status --porcelain` clean; last commit `de617a7c1` at 2026-09-08 23:25:57.
- Neither crate shows in-flight edits from the `CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS` peer ticket at the moment of this audit (both dirs clean, both last-touched by already-committed changes, both >15 min stale). This can change quickly since that ticket is confirmed to be editing `📺️renderer` (per its `window-context-ownership.md`, modified per the session's git-status snapshot) — re-check mtimes before relying on this if time has passed.
- No stale cargo-check logs were found lying around for this crate to corroborate compile health one way or the other; per the read-only/no-build constraint, compile status of the current tree is **not verified** by this audit.

---

## 5. Verdict and ranked gaps

**Use React (`ShellHost`/`World3dHost`, port 6013) for the "every window, every tool" pass.** It is the actively complete implementation: three.js-based `World3dHost` with gumball, full utility ribbon (`🎛️UtilityTree/🟦️.tsx`), panels (`📌️ChromePanels/🟦️.tsx`), all driven by the same schema-first plugin document — nothing in this audit surfaced a missing tool on the React side.

**wgpu path (port 6113 / native) gaps, ranked by estimated effort to close:**

1. **Highest effort — 3D transform gumball.** Confirmed absent repo-wide in Rust (`grep -rli gumball` across the renderer + `ui` module trees returns only `.tsx` files: `🏛️ShellHost/🟦️.tsx`, `🌐️World3dHost/🟦️.tsx`, `📌️ChromePanels/🟦️.tsx`, `📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`). Needs a new on-canvas gizmo (translate/rotate/scale handles), ray-picking against gizmo geometry, and drag-to-transform math wired into the native `world3d_states`/GPU-instance pipeline — a genuinely new feature, not a port.
2. **Medium-high, unconfirmed — brush / volume-brush preview rendering inside the 3D viewport.** The *utility toggle* for brush/volume-brush is generic and wgpu-wire-format-tested (`🌍️world3d_snapshot.rs:78` has a `Brush` variant; `targets-wgpu-component-utilities-utility-node-wire-format` tests cover the button), but this audit did not find (or rule out) where the actual brush/fill preview overlay gets drawn into the 3D scene natively. Needs a dedicated check before use.
3. **Medium, unconfirmed — marquee/box-select in a 3D viewport.** Only 2D marquee (`ink`/`map`) was found natively (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:321,328,1920,3185-3417`); no 3D-scene marquee/box-select was found. May not be needed if puzzle3d's 3D selection model is exclusively click-to-pick, but that wasn't confirmed either.
4. **Low-medium — 3D picking confirmation.** GPU instances already carry `selected`/`hovered` (`✍️draw.rs:2614,2626,3093`) but this audit found no client-side ray-cast that *sets* those flags from a pointer click in wgpu; likely just needs tracing where selection state is populated from, not new implementation, unless it turns out to be missing.
5. **Low — puzzle3d-specific renderer wiring.** Unlike puzzle2d (`puzzle::editor::puzzle2d::engine::...`, ~10 call sites), puzzle3d has none. If the generic world-3d protocol is truly sufficient (as designed per the closed ticket `WGPU-INFINITE-WORLD-CHUNKING-POOLING-LOD-GRID-PARITY`, goal `🎯️puzzle3d🎯️puzzle3dplay`), this may need no code — only verification once gaps 1-3 are closed.

**Files most relevant if picking this up:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (scene dispatch, `render_component_scene_step`, bespoke-pointer-kind list)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`ShellState.world3d_states`, orbit camera)
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{🖥️gpu.rs,✍️draw.rs,📦️prepared.rs}` (actual GPU mesh-instance pipeline, `kernel_3d_scene`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` (shared world-3d asset/environment/sun domain logic)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` (puzzle3d plugin itself — zero renderer-side special-casing today)
