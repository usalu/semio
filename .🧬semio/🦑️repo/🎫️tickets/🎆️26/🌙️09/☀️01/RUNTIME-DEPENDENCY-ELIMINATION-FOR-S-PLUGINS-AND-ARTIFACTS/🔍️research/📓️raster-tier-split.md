# 🎯️ animate's wgpu-tier split — the last plugin closes clean

## Headline

`cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i {wasm-bindgen,js-sys,web-sys}` all
report **"nothing to print."** All four s-plugins verified in this ticket now match:

```
semio-s-plugin-flow      ✅ clean (all three)
semio-s-plugin-puzzle    ✅ clean (all three)
semio-s-plugin-draw-fsm  ✅ clean (all three)
semio-s-plugin-animate   ✅ clean (all three)   ← this pass
```

`semio-framework-raster`'s own `vello`/`wgpu` are also confirmed absent from animate's wasip2 tree
(`cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i vello` /`-i wgpu` both "nothing to
print"), while still present on the native tree (see Verification below) — the tier split removed
the edge without removing the capability where it can actually run.

## Per-symbol classification (`🧰️framework/🔨️modules/🖌️raster/🦀️.rs`)

| Symbol | Classification | Evidence |
|---|---|---|
| `FillOp`, `StrokeOp`, `DrawOp`, `VectorScene` (+ `impl`) | target-neutral | Built only on `semio_framework_geometry::{BezPath, Affine}` plus plain `[f32; 4]`/`f64`. Zero `wgpu::`/`vello::` token anywhere in their definitions. |
| `RasterError` (+ `Display`/`Error` impls) | target-neutral | Every variant holds a plain `String` or is a unit variant. Zero `wgpu::`/`vello::` token. |
| `SceneRasterizer` (struct + `impl`) | **genuinely GPU** | Fields are `wgpu::Device`/`Queue`/`Texture`/`TextureView`/`Buffer` and `vello::Renderer` directly; `new`/`render` call `wgpu::Instance::request_adapter`/`request_device`, `vello::Renderer::new`/`render_to_texture`. |
| `align_bytes_per_row`, `create_target_texture`, `build_vello_scene`, `read_pixels` | **genuinely GPU** | Each names a `wgpu::`/`vello::` type in its signature or body; each is a private fn used only by `SceneRasterizer`. |

Action taken: `FillOp`/`StrokeOp`/`DrawOp`/`VectorScene`/`RasterError` stay unconditional.
`SceneRasterizer` and its four private helper fns gained
`#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`. The crate's own `vello`/`wgpu`
dependencies moved from unconditional `[dependencies]` into a
`[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` table in
`🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/Cargo.toml` — the exact shape `wgpu-tier-split.md`
used for `ui_wgpu`/`vello`/`vello_svg` in `infinite`. Two tests
(`align_bytes_per_row_pads_to_wgpu_alignment`, `scene_rasterizer_renders_expected_pixel_count`) that
exercise the gated code were gated the same way; `vector_scene_push_order_is_stable` (pure
`VectorScene` push-order assertion) stays unconditional.

## Confirming the video renderer is NOT host-only like `render_world_3d` — a real design finding

The ticket's brief pointed at precedent: `wgpu-tier-split.md` gated `render_world_3d` cleanly because
grepping its only callers showed they all live in `📺️renderer/🧑️‍🎨️engine` — the OS's native/browser
rendering host, never a plugin. **The same trace for `pub mod renderer` in
`⚙️engine/🎥️video/🦀️component.rs` does NOT reach the same conclusion.** Traced call chain, one hop
at a time, each confirmed by `grep`:

1. `VelloRenderer` (the renderer module, line 653) is used by `render::render_scene` (unconditional
   call, line 440) and `preview::preview_scene_window_winit` (behind `feature = "preview-window"`,
   not a default feature — out of scope, untouched).
2. `render_scene` is called by `compiler::compile_scene_to_assets` in the SIBLING file
   `⚙️engine/🦀️component.rs:65`.
3. `compile_scene_to_assets` is called by `export_video_from_scene` (`⚙️engine/🦀️component.rs:423`,
   the `//#region 🔖️VideoExport` block).
4. `export_video_from_scene` is called from exactly one place repo-wide (confirmed by
   `grep -rn "export_video_from_scene" ✏️s`):
   `🎮️commands/🐚️export-video-from-deck/🦀️component.rs:15`, inside
   `async fn export_video_from_deck`, itself called only by
   `pub async fn handle_async(payload: &ExportVideoFromDeck)`.
5. `export_video_from_deck::handle_async` is called from exactly one place
   (`grep -rn "handle_async" ✏️s/🔌️plugins/🎞️animate`):
   `✏️editor/🦀️component.rs:596`, inside the `AnimatePresentPlayApp` `Editor` trait impl's
   `async fn handle(command: &PresentCommand, ...)` — `PresentCommand::ExportVideoFromDeck(payload) =>
   export_video_from_deck::handle_async(payload).await`.
6. `Editor::handle` is the plugin's own async command-dispatch entry point — the same
   `command_from_action`/`ArtifactApp::handle` surface the ticket's seam 6 already names as guest
   command dispatch. `#![allow(async_fn_in_trait)]` in `📦️glue.rs` (with the docstring "O1's
   universal-async ruling") confirms async trait fns are the deliberate, repo-wide dispatch
   convention, not an accident.
7. Confirmed the guest DOES run async code on `wasm32-wasip2`, not just natively: grepping
   `🔌️plugin/⚛️reactor/🦀️component.rs` shows a real `pub async fn poll<PA: PluginApp>` (the "async WIT
   poll", per its own `async_actor_poll_awaits_exchange_and_render_work` test) driven by a
   thread-local `executor::ReactorExecutor` — a cooperative, no-OS-threads async scheduler that
   exists specifically because a wasm32-wasip2 guest has none. `poll` → `plugin_exchange` →
   `command_from_action`/`ArtifactApp::handle` is the real, shipped guest dispatch path.

**Conclusion: `export-video-from-deck` IS reachable from wasip2 guest command dispatch.** Unlike
`render_world_3d`, simply `#[cfg]`-gating `VelloRenderer`/`pub mod renderer` away for wasip2 would
NOT be a no-op — it would break compilation of every caller up this chain (`render_scene` →
`compile_scene_to_assets` → `export_video_from_scene` → `handle_async` → `Editor::handle`'s match
arm), all of which must keep compiling under wasip2 regardless (that match arm is inside a trait impl
that must stay exhaustive, and `Editor::handle` is unconditionally part of the shipped component).

## The fix actually applied — not a stub, an honest environment-limitation error

`wasm32-wasip2` genuinely cannot open a GPU device: WASI Preview 2 defines no graphics API. So even
had `SceneRasterizer` compiled for that target, `SceneRasterizer::new`'s
`instance.request_adapter(...)` would find no adapter and return `RasterError::Adapter` at runtime —
exactly the same outcome native CI already produces in a headless environment (see the raster crate's
own `[DEBUG] no wgpu adapter in this environment — skipping GPU assertion` test fallback, an
established precedent in this same crate). The capability was never functionally available on this
target; the fix makes that pre-existing reality explicit instead of leaking `wgpu`/`vello` (and their
`wasm-bindgen`/`js-sys`/`web-sys` transitive edge) into the link graph to reach the same outcome.

`⚙️engine/🎥️video/🦀️component.rs`'s `pub mod renderer` now has two `VelloRenderer` definitions with
the identical public API (`async fn new(width, height) -> Result<Self, VideoError>`,
`fn render_capture(&mut self, ...) -> Result<Vec<u8>, VideoError>`), selected by
`#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` / `#[cfg(all(target_arch = "wasm32",
target_env = "p2"))]`:

- **Native/host** (unchanged behavior): wraps `semio_framework_raster::SceneRasterizer` exactly as
  before.
- **`wasm32-wasip2`**: a zero-sized marker struct. Both methods return
  `Err(video_error_from_raster(RasterError::Adapter("wasm32-wasip2 has no GPU device access ...")))`
  — the SAME `video_error_from_raster` conversion fn the native path already used (kept unconditional
  — it only pattern-matches `RasterError`/`VideoError`, both target-neutral), so the SAME error
  effect (`Effect::DownloadMediaExport { filename: "animate-video-export-error.txt", ... }`) reaches
  the user that `export_video_from_deck::handle_async`'s existing `Err(error) => ...` arm already
  produces for any other render failure. Nothing new was invented at the error-handling layer; this
  reuses the exact contract already established for "no adapter found."

Because both `VelloRenderer` variants export the identical public surface, every caller up the chain
— `render_scene`, `compile_scene_to_assets`, `export_video_from_scene`,
`export_video_from_deck::handle_async`, `Editor::handle`'s match arm — needed **zero changes** and
keeps compiling unconditionally on both targets. This is the direct reason the fix could stay
bounded to `⚙️engine/🎥️video/🦀️component.rs` (plus one `#[cfg]` on a native-only test in the sibling
`⚙️engine/🦀️component.rs`) instead of cascading `#[cfg]` gates through five call sites and a trait
impl.

Helper fns used only by the native `render_capture` body (`build_vector_scene`, `scene_affine`,
`paint_mobject`, `color_to_rgba_array`, `color_from_style`) were gated the same way as
`VelloRenderer`'s native impl — confirmed by grep that none has another caller.
`static_layer_hash`/`frame_hash` (pure hashing over `CapturedFrame`, called by `render_scene`'s cache
logic regardless of target) and `CapturedFrame` itself stay unconditional — verified they name no
`wgpu::`/`vello::` type and their `Sobject` trait import (needed for `mobj.id()`/`.transform()`/etc.
method resolution) must stay unconditional too, since `static_layer_hash` uses it outside any gate.

Two existing tests that assert real GPU pixel output (`renderer::tests::vello_renderer_produces_rgba_buffer`,
`render::tests::render_scene_writes_last_frame`) and one in the sibling file
(`compile_scene_to_assets_writes_mp4`) were gated native-only — their assertions are meaningless
against a `VelloRenderer` that always reports "no adapter" by design. `compile_present_site_writes_static_bundle`
in the same test module (writes the static site bundle, never touches the renderer) was left
unconditional.

## Cargo.toml narrowing

`🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/Cargo.toml`:

```toml
[dependencies]
semio-framework-geometry = { path = "...", package = "semio-framework-geometry" }

[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]
vello = { version = "0.7.0", features = ["wgpu", "wgpu_default"] }
wgpu = "27.0.1"
```

No change was needed at `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` — it depends on
`semio-framework-raster` unconditionally (correct: the crate itself is still a first-party
dependency on every target, it just internally narrows what it links).

## Before / after `cargo tree -i` evidence

Before this pass (from `📓️verified-outcomes.md`'s plugin table): `semio-s-plugin-animate` was not
yet checked with `cargo tree -i wasm-bindgen` on `wasm32-wasip2` — it was known to still name
`vello`/`wgpu` unconditionally via `semio-framework-raster`'s Cargo.toml, the traced edge this
ticket's brief specified:
`wasm-bindgen ← js-sys ← wasm-bindgen-futures ← wgpu ← semio-framework-raster ← semio-s-plugin-animate`
(also via `vello ← semio-framework-raster`).

After (this pass):

```
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i wasm-bindgen  → nothing to print
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i js-sys        → nothing to print
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i web-sys       → nothing to print
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i vello         → nothing to print
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i wgpu          → nothing to print

$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i {wasm-bindgen,js-sys,web-sys}
  → nothing to print (re-verified, unchanged)
$ cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i {wasm-bindgen,js-sys,web-sys}
  → nothing to print (re-verified, unchanged)
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i {wasm-bindgen,js-sys,web-sys}
  → nothing to print (re-verified, unchanged)

# native target — the capability is still real where it can actually run
$ cargo tree -p semio-s-plugin-animate -i vello
vello v0.7.0
└── semio-framework-raster v0.1.0 (.../🖌️raster/📦️packages/🦀️rust)
    └── semio-s-plugin-animate v0.1.0 (.../🎞️animate/📦️packages/🦀️rust)
$ cargo tree -p semio-s-plugin-animate -i wgpu
wgpu v27.0.1
├── semio-framework-raster v0.1.0 (.../🖌️raster/📦️packages/🦀️rust)
│   └── semio-s-plugin-animate v0.1.0 (.../🎞️animate/📦️packages/🦀️rust)
└── vello v0.7.0
    └── semio-framework-raster v0.1.0 (.../🖌️raster/📦️packages/🦀️rust) (*)
```

## Build results

- **`cargo check -p semio-framework-raster --target wasm32-wasip2`** — `Finished` in 1m15s, **0
  errors**, no warnings printed. Confirms the tier split compiles clean on the shipped target without
  needing `vello`/`wgpu` at all.
- **`cargo check -p semio-framework-raster`** (native) — `Finished` in 1m29s, **0 errors** (one
  pre-existing, unrelated future-incompatibility note about the `block` crate, a transitive `metal`
  dependency on macOS — not this pass's code). Confirms `vello`/`wgpu` still resolve and the native
  `SceneRasterizer` path is unbroken.
- **`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-animate`** and
  **`cargo check -p semio-s-plugin-animate`** (native) — **both BLOCKED**, identically, by a
  pre-existing, actively in-progress, unrelated wave: `git status --short ✏️s/🔌️plugins/🗄️stdio` shows
  **1404 modified files**, an uncommitted peer session mid-refactor (matches
  `📓️verified-outcomes.md`'s own note: "🗄️stdio ~563 real call-site files ... in flight; last seen
  2217 errors mid-conversion" — this run saw 2218, one more, consistent with that session still
  progressing). Every error in both runs is `error[E0277]`
  (`RemovePoint`/`SetPoint`/`InsertPoint`/... : `serde::Serialize`/`serde::Deserialize` not
  satisfied) in `✏️s/🔌️plugins/🗄️stdio`'s `☁️las`/`🎨️svg`/`🎞️pptx`/`📄️pdf`/`🧿️semio` artifact mutation
  files — confirmed by `grep -oE '\-\-> [^:]+'` over the full captured logs (both native and wasip2):
  **zero matches** for `🖌️raster`, `🎥️video`, or `semio_framework_raster` in either log. `stdio` is a
  direct, unconditional dependency of `semio-s-plugin-animate` on every target, so `cargo check`/
  `build` never reaches `animate`'s own compilation unit while it fails — this is not something
  `--keep-going` can route around (a dependent crate cannot type-check without its failed
  dependency's `.rmeta`).
- Given the plugin-level build could not be reached, both edited files were independently confirmed
  syntactically valid with `rustfmt --edition 2021 --check` (exit 0 on both — the only diffs shown
  were pre-existing formatting on lines this pass never touched), and every `#[cfg]` gate was
  hand-traced against its actual callers (documented above) rather than assumed.
- **`cargo tree -i` for all three target crates on all three plugins** (the metadata-only, lock-free,
  cannot-go-stale check, unaffected by the stdio breakage since it never invokes rustc) is unaffected
  and is the primary evidence for this ticket: **clean for animate, flow, puzzle, and draw-fsm.**

## Files touched

- `🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/Cargo.toml` — `vello`/`wgpu` moved from
  unconditional `[dependencies]` to a
  `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` table.
- `🧰️framework/🔨️modules/🖌️raster/🦀️.rs` — top docstring rewritten to document the tier split;
  `SceneRasterizer` (struct + `impl`), `align_bytes_per_row`, `create_target_texture`,
  `build_vello_scene`, `read_pixels` gated `#[cfg(not(all(target_arch = "wasm32", target_env =
  "p2")))]`; the two tests exercising them gated the same way.
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs`
  — `pub mod renderer`: split `VelloRenderer`/`StaticBackgroundCache`/`impl VelloRenderer` into a
  native-only variant (unchanged behavior) and a new wasip2-only zero-sized variant that reports
  `RasterError::Adapter` honestly; gated `build_vector_scene`/`scene_affine`/`paint_mobject`/
  `color_to_rgba_array`/`color_from_style` and their now-conditional imports (`Color`, `Affine`,
  `SceneRasterizer`, `VectorScene`) native-only; kept `RasterError` import, `video_error_from_raster`,
  `CapturedFrame`, `static_layer_hash`, `frame_hash`, and the `Sobject`/`Sobjects` imports
  unconditional; gated the `renderer` module's GPU-assertion test and `render::render_scene`'s
  GPU-assertion test native-only.
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️component.rs`
  — gated `compile_scene_to_assets_writes_mp4` (calls the real renderer transitively) native-only;
  left `compile_present_site_writes_static_bundle` (never touches the renderer) unconditional.

## Deliberately left alone

- `preview::preview_scene_window_winit` (behind `feature = "preview-window"`, not a default feature,
  pulls in `winit`) was not touched — out of scope for this pass, and `winit` on `wasm32` would need
  its own tier-split investigation if that feature is ever enabled for a wasip2 build.
- `typst`/`typst-svg`/`typst-assets`/`usvg` in `semio-framework-typeset` were not touched — already
  confirmed clean by `wave-animate-render.md` and unaffected by this pass's `cargo tree -i` results.
- Removing serde from `semio-s-plugin-stdio` (the 1404-file concurrent wave blocking the plugin-level
  build) is explicitly a different, already-tracked slice of this same ticket — not attempted here.

## What is proven vs. not proven — stated plainly

**Proven**: `semio-framework-raster` compiles clean on both `wasm32-wasip2` and native, with the
correct dependency set on each (`cargo tree -i` evidence above, plus two direct `cargo check` runs).
The `wasm-bindgen`/`js-sys`/`web-sys` edge is gone from `semio-s-plugin-animate`'s wasip2 dependency
graph, matching flow/puzzle/draw-fsm. The video-export command's guest-reachability was traced and
confirmed by grep at every hop, not assumed.

**Not proven**: an end-to-end `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-animate`
or native `cargo check -p semio-s-plugin-animate` completing at 0 errors — both are blocked by the
unrelated, in-progress `🗄️stdio` wave (1404 files, uncommitted, a different session's work) on
**both** targets identically, so this is not a regression introduced by this pass and not something
this pass can resolve. Re-running both commands once that wave lands is the natural way to close this
gap — the `cargo tree -i` results are the strongest evidence available in the meantime, being
lock-free, metadata-only, and unaffected by the stdio breakage entirely.
