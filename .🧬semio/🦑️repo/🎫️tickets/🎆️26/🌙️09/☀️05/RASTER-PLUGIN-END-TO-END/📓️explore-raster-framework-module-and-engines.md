# Explore: Raster plugin's framework-side dependencies (module, engines, surface kind)

Read-only audit. All paths repo-relative from `/Users/ueli/Documents/semio`.

## 0. Headline finding

**The framework module literally named `🖌️raster` (`semio-framework-raster`) is NOT used by the `🖨️raster` plugin at all.** It is a headless Vello/wgpu scene-to-RGBA8 rasterizer consumed only by the `🎞️animate` plugin's video encoder. The `🖨️raster` plugin (a paint/layers app) instead depends on the generalized `paint2d` **surface kind** (`SurfaceKind::Paint2d` + `Paint2dScene`), which is a completely different subsystem in `semio-framework-ui-contract`/`semio-framework-plugin`/the wgpu+react renderer targets. Anyone auditing "framework support for raster" needs to look at the paint2d surface machinery, not the `🖌️raster` crate — a naming collision, not a missing feature.

---

## 1. Framework module `🧰️framework/🔨️modules/🖌️raster/`

- Crate: `semio-framework-raster` (`🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/Cargo.toml:1-31`). Package glue at `🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/🦀️.rs:1-6` just re-exports `../../🦀️.rs` (the real implementation, `🧰️framework/🔨️modules/🖌️raster/🦀️.rs`, 334 lines).
- What it provides: `VectorScene`/`FillOp`/`StrokeOp`/`DrawOp` (zero-`wgpu` value types, unconditional even on wasm32) plus `SceneRasterizer` — a headless wgpu+Vello device that rasterizes a `VectorScene` to a `Vec<u8>` RGBA8 buffer (`🦀️.rs:105-175`). Not a paint-surface/UI concept; it's an offscreen GPU-to-pixels renderer, relocated from `🎞️animate`'s old `VelloRenderer` per ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` (module docstring, `🦀️.rs:1-18`).
- External runtime deps: `vello 0.7.0` and `wgpu 27.0.1` (`Cargo.toml:26-28`), both external crates — explicitly permitted under CLAUDE.md's "external libs behind an interface" rule because every public type (`VectorScene`, `RasterError`, etc.) is first-party; no caller needs to import `vello`/`wgpu` (module docstring, `🦀️.rs:1-6`).
- wasm32 compatibility: `SceneRasterizer` and every private fn it alone uses is gated `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` (`🦀️.rs:108,124,130,177,193,210,215,220,235`), and the Cargo dependency table mirrors this exactly (`Cargo.toml:26`). The module docstring calls out, by name, that a bare `cfg(not(target_arch = "wasm32"))` would NOT exclude `wasm32-wasip2` (since `target_arch = "wasm32"` alone is true for it) and would leak `wgpu`/`vello`'s `wasm-bindgen`/`js-sys`/`web-sys` transitive edge into the shipped guest component — the exact bug class the module cites as already caught once in `🧩️puzzle` (`🦀️.rs:9-18`). No `std::fs`, no threads outside wgpu's own internals.
- Standalone workspace: **no** `[workspace]` table in either the crate's own Cargo.toml or the raster plugin's — both are plain members of the root workspace (root `Cargo.toml:109` lists the path, `Cargo.toml:152` aliases it in `[workspace.dependencies]`).
- Dependents (repo-wide grep for `semio-framework-raster`/`semio_framework_raster`): only
  - root `Cargo.toml:109,152` (workspace membership/alias)
  - `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml:53`
  - `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️.rs:661,667,675` (`RasterError`, `SceneRasterizer`, `VectorScene`)

  **Zero references anywhere under `✏️s/🔌️plugins/🖨️raster/`.**

---

## 2. Raster plugin's actual framework dependencies

`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:1-44`:
- `[dependencies]` (all unconditional, no `[target.…]` table at all in this file):
  `semio-framework-value-derive`, `semio-s-plugin-stdio` (default-features=false, `full-artifact-catalog`), `semio-framework-os-kernel`, `semio-framework-os` (the "os host" crate), `semio-framework`, `semio-framework-plugin` (feature `component-guest`), `semio-framework-dispatch-macros`, `semio-framework-schema`, `semio-framework-job` (workspace), `framework_hash` (= `semio-framework-hash`), `base64_codec` (= `semio-framework-io-base64`).
- Component metadata: `[package.metadata.component] package = "semio:raster"`, `crate-type = ["cdylib", "rlib"]` — this is a wasm32-wasip2 guest component.
- `stdio` dependency: `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:63` has an **empty** `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` placeholder table (no native-only deps currently).
- `os-host` (`semio-framework-os`, at `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`), which raster pulls in unconditionally, itself DOES have native-only sections:
  - `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` (lines 49-64): `png 0.17.16`, `resvg 0.45.1`, `tiny-skia 0.11.4`, `usvg 0.45.1`, optional `zip 2.4` — all correctly excluded from the wasm32-wasip2 build raster actually ships as.
  - `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` (lines 66-74): `tokio`, `semio-framework-actor`, `semio-framework-plugin-host`, `semio-framework-async` — same, correctly gated out.
  - Line 31 is the one worth flagging on first read: `ui_wgpu = { path = "…", package = "semio-framework-ui", features = ["wgpu"] }` is **unconditional** (no target gate). This looks like it could leak the real `wgpu` crate into raster's wasm32-wasip2 build. **It does not** — verified in `semio-framework-ui`'s own `Cargo.toml:27-35`: the `"wgpu"` feature only enables `dep:serde`, `dep:serde_json`, `dep:semio-framework-os-kernel`, `dep:semio-framework-value-derive`, `dep:bytemuck`, `dep:semio-framework-job` — the crate's own comment states this explicitly: *"Names the target, not the `wgpu` crate — that one only arrives with `wgpu-engine` below"* (line 27-28). The actual `wgpu = {…, optional = true}` dependency (line 90) is only pulled in by the separate `wgpu-engine` feature (line 37-52), which os-host never enables. So this is a deliberately engineered non-leak, not a blocker — but it is exactly the kind of thing that would silently break if someone "simplified" os-host's feature list without reading this comment.

Conclusion for item 2: none of raster's *own* Cargo.toml lines are native-only; the native/wasm split is entirely pushed one level down into `os-host`'s and `stdio`'s target tables, both of which correctly exclude native-only code from the wasm32-wasip2 build raster plugin actually ships as.

---

## 3. Surface kind: `paint2d`

### Declaration
`SurfaceKind` (15 variants) lives in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🗺️surface.rs:74-132`, wire name `"paint-2d"` for `Paint2d` (lines 102-104). `SurfaceProps{ kind, doc_schema, doc, bindings }` is the opaque envelope (lines 143-165); this contract crate never parses `doc.bytes` (module doc, lines 1-63) — the actual `Paint2dScene` shape is owned by `semio-framework-ui-scene`/`semio-framework-plugin`.

### Generic `SurfaceRegistry` (framework's declarative registry, `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗺️surface.rs`) — **dead for production surfaces**
`.register::<S>(kind)` is called nowhere outside this file's own unit tests (`registry.register::<DeadlineSurface>(SurfaceKind::World3d)` at line 750, `registry.register::<RecordingInputSurface>(SurfaceKind::NodeGraph)` at line 777 — repo-wide grep for `.register::<` confirms these are the *only* two call sites in the entire framework). Real surfaces (including paint2d) are NOT wired through this registry; they're dispatched via hand-written `match SurfaceKind::…` arms in the renderer engine (below). Worth flagging as either dead scaffolding or a not-yet-adopted abstraction — not a raster-specific blocker, but a framework inconsistency an auditor should not mistake for "the" surface dispatch mechanism.

### Renderer engines — the real paint2d path
- **wgpu target, Scenes element** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`): full `SurfaceKind::Paint2d` handling — hit-testing (lines 1424, 1448, 1466), scene decode dispatch (line 1513), `render_paint_2d(scene, bounds, ctx)` (line 1914), and it's included in the full enumerated-kinds test list alongside Canvas2d/TextEditor/InkCanvas/etc. (line 1964). Not a stub.
- **wgpu low-level target** (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs:3936,4435`, `…/🎯️events.rs:2124`): `SurfaceKind::Paint2d` used in scene-component construction and event tests.
- **react target**: `Paint2dHost` — a real, 688-line component (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx`), re-exported from the react target barrel (`…/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx:411,1121-1123`), and covered by react-target tests including a raster-specific fixture (`…/🔬️index.test.ts:4174-4197`, `surfaceId: "raster.play.viewport"`, `componentKind: "paint-2d"`).

**No missing surface kind in the react renderer — paint2d is fully wired on both the wgpu and react targets.**

### "Editor engine" / "flow-core" / "surface engine" — what these actually are
These three names come from `buildEngineWasm` in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:930-949`, which for a react-renderer dev session unconditionally builds three wasm-bindgen engines before any per-plugin engine:
1. **Surface/graph engine** — `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts` (crate `semio-framework-surface`, "paint/terrain/node-graph/tiled-map surface family" per its own docstring, line 2). Output dir: `🕸️bindings` (via `runWasmPackWebBuild`, `outputDirectory: "🕸️bindings"`, script.ts:15).
2. **Editor engine** — `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts` (framework-editor, output to `pkg/`).
3. **Flow-core** — `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts` (output to `🕸️bindings`, alongside a hand-bundled browser bridge — see its `bundleBrowserModule`, script.ts:12-26).

The raster plugin's own playground metadata (`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:17-20`) declares `engines = ["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"]` — i.e. it names the surface/graph engine as its extra per-variant engine. Since `buildEngineWasm` already builds that exact engine unconditionally for every react-renderer session (step 1 above, before consulting per-variant `engines`), **raster's own declaration is redundant** — it adds nothing that wasn't already going to be built. Not a bug, just worth knowing when reasoning about what `engines=[…]` actually gates.

### Artifact freshness — is `SKIP_ENGINE_BUILD=1` safe right now?
- `SKIP_ENGINE_BUILD` only affects the react-renderer path (`if (renderer !== "react" || process.env.SKIP_ENGINE_BUILD === "1") return;`, os/dev script.ts:941) and is referenced in three other spots: root `📜️script.ts:186,201` (dev-serve convenience for a peer holding the shared cargo lock) and a library test (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts:2096-2099`).
- Checked mtimes right now (2026-09-06):
  - Surface/graph engine: `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface_bg.wasm` — built Sep 5 13:37. **No `.rs` file under the crate is newer.**
  - Editor engine: `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/framework_editor_bg.wasm` — built Sep 5 13:40. **No `.rs` file under the crate is newer.**
  - Flow-core: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core_bg.wasm` — built Sep 4 21:55 (older, but still **no `.rs` file newer than it**; the bindings dir's `package.json`/`🖥️flow-host.js` were touched Sep 5 19:5x by the declarations step, not a rebuild of the wasm itself).
  - **Verdict: as of this snapshot, `SKIP_ENGINE_BUILD=1` is safe** — none of the three engines has a source edit not yet reflected in its built artifact. This is a point-in-time read only; per repo convention (concurrent live devs), re-check mtimes immediately before relying on it rather than trusting this report.

---

## 4. `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` — the "native raster" invocation

Full path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` (670 lines). There is no subcommand literally named `native raster`; rather `native` (`.register("native", NativeRunScript)`, line 644) takes the plugin/variant as its first positional segment, defaulting to `"s"` or `$SEMIO_PLUGIN` — so `bun script.ts native raster` runs with `filterPlugin = "raster"`.

`NativeRunScript.run` (lines 326-350):
1. Delegates to `NativeBuildScript` with the same segments.
2. `NativeBuildScript.run` (lines 301-318): `cargo build -p <crateName> --bin semio-wgpu-native --features native-bin` (optionally `--release` on `--dist`/`--release`), then recurses into `os/dev`'s own `plugin` command (`bun osDevScript plugin raster` with `SEMIO_RENDERER=wgpu SEMIO_PLUGIN=raster`) to build the raster plugin's own wasm component.
3. Back in `NativeRunScript`: starts the asset server if the raster variant declares assets (`variantAssetSpecs`), resolves native app args from the playground catalog, then execs `semio-wgpu-native --plugin raster <appArgs> [--smoke]` with `SEMIO_PLUGIN_MODULES` pointed at the built plugin output root.
4. `nativeBinaryPath` (lines 295-299) resolves to `target/{debug,release}/semio-wgpu-native[.exe]` under `$CARGO_TARGET_DIR` or `target/`.

So: it builds the native wgpu shell binary (`semio-wgpu-native`), builds the raster wasm plugin component via the shared os/dev build loop, then launches the native binary against that plugin — the standard "run one plugin natively through the wgpu target" flow, parameterized by plugin id rather than a dedicated raster-only subcommand.

---

## 5. Camera/zoom history vs current code

Three closed tickets, read via `🎫️ticket.json` summaries:

- **`26/07/01/RASTER-ZOOM-CAMERA-FIX`**: root cause was camera being part of the WASM compositor's sync JSON, causing full layer re-parse/asset re-upload on every wheel tick. Fix: compositor sync omits camera/session fields; canvas keeps a local session camera; only persists to the document when it actually changes.
- **`26/07/01/RASTER-ZOOM-COMPOSITE-FIX`**: composite window keeps session camera in React/WASM without document re-sync on zoom; navigator uses a fit-to-bounds overview camera with a React viewport overlay driven by the composite's camera+viewport; WASM selection chrome disabled in navigator; pick/hover/pan paths restored per-window.
- **`26/07/31/RASTER-CAMERA-AS-SESSION-ONLY-VIEW-ACTION`** (the big one): moved camera pose fully OUT of the VCS-tracked document into session-only runtime state; reclassified `setCamera`/`setCameraZoom` from `ActionKind::Operation` to `ActionKind::View` across the whole raster crate tree; deleted `RasterOperation::SetCamera` entirely (no compat shim); `RasterProjection` (document struct) no longer has a camera field; `RasterCamera` moved to the ui-crate's runtime/config type. 18/20 raster-ui tests passing at close (2 pre-existing unrelated icon-catalog failures, flagged separately).

**Current code matches all three tickets exactly:**
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1-6` (module docstring) states plainly: "...the session-only free camera) now lives in `crate::editor::raster::config::RasterConfig`, written via `RasterConfigMutation`s."
- `raster_scene()` (same file, lines 95-108) builds `Paint2dScene{ camera_json: dsl::os_pack::json::to_json_string(&runtime.camera), … }` where `runtime: &RasterConfig` — camera comes from config/runtime state, not the document.
- `document_sync_json()` (lines 55-68) filters out `assets`/`brushSize`/`brushOpacity` from the document JSON but no longer needs to filter `camera` — it was never part of `RasterSnapshot` to begin with post-migration, consistent with the ticket's "document struct no longer has a camera field at all."
- `set-camera` command handler (`…/🎭️modes/✏️edit/🎮️commands/📷️set-camera/🦀️.rs:16-18`): `Ok(Emit::config(vec![RasterConfigMutation::SetCamera{ camera: payload.camera.clone() }]))` — emits a **config** mutation (not a `RasterMutation`/document op), matching the View-kind/session-only reclassification.
- Composite and navigator window definitions still declare `surface_kind: SurfaceKind::Paint2d` (`…/🪟️windows/🖼️composite/🦀️.rs:24`, `…/🪟️windows/🧭️navigator/🦀️.rs:22`) and are covered by tests asserting exactly that (`🦀️.rs:56-61`, `🦀️.rs:49-52`).

No drift found between the ticket notes and the code on disk.

---

## 6. ComponentTree / UiNode: declaring a paint2d window vs. a canvas-2d window

**Raster (paint2d)** — `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs`:
```rust
// definition(): surface_kind: SurfaceKind::Paint2d  (line 24)
pub fn render(document: &RasterDocument, config: &RasterConfig) -> UiNode {
    build_paint_2d_scene(RASTER_PLAY_SURFACE_COMPOSITE, RASTER_PLAY_CONTROLLER_ID,
        raster_scene(document, config, config.active_utility_id.as_str(), "composite"))
}
```
One typed struct (`Paint2dScene`) built by a shared helper (`raster_scene`, §5 above) and handed to `build_paint_2d_scene(surface_id, controller_id, scene) -> UiNode`.

**Draw plugin (canvas-2d)**, the closest sibling for comparison — `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs:84,114-118`:
```rust
pub fn render(document: &DrawingSnapshot, config: &DrawingConfig, preview: &DrawingGesturePreview, active_utility: &str) -> UiAssemblyResult<BuiltNode> {
    // …builds `records: Vec<DslValue>` of raw scene-graph nodes via `overlay_record`/`artboard_scene_records`…
    scene_surface(
        DRAWING_PLAY_SURFACE_ID,
        semio_framework_ui_contract::SurfaceKind::Canvas2d,
        &Canvas2dScene { camera_x: config.camera.x, camera_y: config.camera.y, zoom: config.camera.zoom, layers_json: dsl::json::to_json_string(&records), snapshot: None },
    )
}
```
Same shape overall (typed scene struct → generic `scene_surface(id, kind, &scene)` helper → `UiNode`/`BuiltNode`), but two concrete differences worth flagging to whoever maintains raster:
- **Camera representation**: drawing bakes `camera_x`/`camera_y`/`zoom` directly as scalar fields on `Canvas2dScene`, read straight off `config.camera` (drawing never went through a "camera as session-only view action" migration, or did it before raster and already lands camera in config either way). Raster instead serializes the whole camera as one opaque `camera_json` string field on `Paint2dScene` (`raster_scene`, `🦀️.rs:101`) — an intentional difference (Paint2dScene's payload also carries `document_sync_json`, `assets_json`, `selection_json`, `active_utility`, `brush_size/opacity`, `composite_viewport_json`, i.e. a much richer envelope than Canvas2dScene's four fields), not a bug, but the two "2D surface" plugins do NOT share one camera wire-shape.
- **Return type**: raster's window `render()` returns `UiNode` directly via `build_paint_2d_scene`; drawing's returns `UiAssemblyResult<BuiltNode>` via `scene_surface` (fallible assembly, richer node type). Both patterns exist side-by-side in the framework's plugin-authoring surface (`semio_framework_plugin::{build_paint_2d_scene, scene_surface, …}`) — worth knowing if writing new code, since which one applies depends on the surface kind's own helper API, not a free choice.

Lowpoly's UV window (`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️.rs`) also uses paint2d — a second real-world consumer of `build_paint_2d_scene`/`Paint2dScene` beyond raster, useful as a cross-check if raster's usage ever looks suspicious in isolation.

---

## 7. Prioritized blockers / mismatches (framework side)

1. **None found that would actually block raster today.** Every concrete "does the framework support raster" question resolves positively: paint2d is fully wired on both wgpu and react targets (§3), the wasm32-wasip2 build excludes every native-only dependency correctly including the one line that looked suspicious at first glance (`os-host`'s unconditional `ui_wgpu` feature — verified safe, §2), and the camera/zoom architecture in code matches all three closed tickets exactly (§5).
2. **Naming trap (documentation/onboarding risk, not a code bug)**: the framework module named `🖌️raster` is unrelated to the `🖨️raster` plugin. Anyone told to "check the raster framework module" who reads `semio-framework-raster` instead of the paint2d surface machinery will draw entirely wrong conclusions. Worth a docstring cross-reference or a rename if this keeps causing confusion (out of scope for this read-only audit to fix).
3. **Dead generic `SurfaceRegistry`** (`🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗺️surface.rs`): `.register::<S>(kind)` has zero production call sites, only its own tests. If someone is hunting for "where paint2d gets wired up" they'll find this file first via a `SurfaceKind` grep and waste time on a dead end before finding the real dispatch in `…/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`. Low priority, framework-wide (not raster-specific) cleanup candidate.
4. **Redundant engine declaration**: raster's playground metadata (`Cargo.toml:17-20`) names the surface/graph engine as its extra build dependency, but `buildEngineWasm` already builds that engine unconditionally for every react-renderer session before consulting per-variant `engines` at all. Harmless (no double-build, `Set` dedup at `os/dev script.ts:947`), but the declaration currently documents nothing raster actually needs uniquely — if raster ever needs the editor or flow-core engine instead, note those are already unconditional too, and the `engines=[…]` field is effectively only useful for an engine outside that unconditional trio.
5. **Engine artifact freshness is a snapshot, not a guarantee**: confirmed safe to `SKIP_ENGINE_BUILD=1` at time of writing (§3), but this repo has concurrent live devs constantly rebuilding — re-verify mtimes immediately before relying on it in any follow-up work rather than trusting this file.

---

## Files touched/read (no edits made — read-only audit)

- `🧰️framework/🔨️modules/🖌️raster/🦀️.rs`, `🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📋️project.json,📜️script.ts}`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}`, `✏️s/🔌️plugins/🖨️raster/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/{🖼️composite,🧭️navigator}/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️set-camera/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🗺️surface.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗺️surface.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/{🟦️.tsx,🔬️index.test.ts}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` (`buildEngineWasm`, `linkedSessionEngines`)
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts` + `🕸️bindings/`
- `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts` + `🕸️bindings/`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️31/RASTER-CAMERA-AS-SESSION-ONLY-VIEW-ACTION/🎫️ticket.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️01/{RASTER-ZOOM-CAMERA-FIX,RASTER-ZOOM-COMPOSITE-FIX}/🎫️ticket.json`
- root `Cargo.toml` (workspace member list + `[workspace.dependencies]` alias lines)
