# Wave: heavy render/typeset stack (`typst*`, `usvg`, `vello`, `wgpu`) in `🎞️animate`

Status: DONE at the manifest level (all six crates gone from the plugin's `Cargo.toml`, both new
framework crates build/test/lint clean in isolation). The plugin-level `cargo build` /
`cargo build --lib --target wasm32-wasip2` could NOT be verified to completion — both are
currently blocked by a pre-existing, unrelated, actively-in-progress breakage in
`semio-framework`/`semio-framework-os-kernel` from a different concurrent session (see
"Verification" below for full evidence this is not this wave's code).

## Usage inventory (measured, not assumed)

Both call sites relocated verbatim (no behavior change) from
`✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml`'s unconditional
`typst`/`typst-svg`/`typst-assets`/`usvg`/`vello`/`wgpu` lines into two new framework crates.

### wgpu + vello — headless video frame capture

`.../⚙️engine/🎥️video/🦀️component.rs`, `pub mod renderer`, struct `VelloRenderer`:
- `wgpu` symbols used: `Device`, `Queue`, `Instance`/`InstanceDescriptor`/`Backends`,
  `RequestAdapterOptions`/`PowerPreference`, `DeviceDescriptor`/`Features`/`Limits`/
  `MemoryHints`/`Trace`, `Texture`/`TextureView`/`TextureDescriptor`/`TextureViewDescriptor`/
  `TextureDimension`/`TextureFormat`/`TextureUsages`/`Extent3d`, `Buffer`/`BufferDescriptor`/
  `BufferUsages`, `CommandEncoderDescriptor`, `TexelCopyTextureInfo`/`TexelCopyBufferInfo`/
  `TexelCopyBufferLayout`, `Origin3d`, `TextureAspect`, `MapMode`, `PollType`. 27 distinct symbols
  (matches the ticket's "~27" estimate).
- `vello` symbols used: `Renderer`/`RendererOptions`/`AaSupport`, `Scene`, `RenderParams`/
  `AaConfig`, `peniko::Color`, `peniko::Fill`, `kurbo::Stroke` (via `vello::kurbo`). 9 symbols.
- Callers: `preview_scene_window_winit` (feature `preview-window`) and the `render_scene`/export
  path both go through `VelloRenderer::new`/`render_capture` only — no other file in the plugin
  names a `wgpu::`/`vello::` type.

### typst + typst-svg + typst-assets + usvg — math/markup typesetting → vector paths

`.../⚙️engine/🔤️text/🦀️component.rs`, `pub mod text`:
- `typst` symbols used: `Library`, `World`, `LibraryExt`, `Bytes`, `Datetime`, `Abs`,
  `PagedDocument`, `FileId`, `Source`, `VirtualPath`, `Font`, `FontBook`, `LazyHash`,
  `diag::FileError`/`FileResult`, `compile`. 15 symbols.
  - `typst-svg`: `svg_merged`. `typst-assets`: `fonts()`.
- `usvg` symbols used: `Options`, `Tree`, `Node` (`Group`/`Path` variants),
  `tiny_skia_path::PathSegment` (`MoveTo`/`LineTo`/`QuadTo`/`CubicTo`/`Close`). 8 symbols.
- The plugin already had a *local* interface (`TextRenderer` trait / `TypstTextRenderer` struct)
  isolating `typst`/`typst-svg`/`typst-assets` at the *code* level, plus a stdio-SVG-codec
  validation round-trip (`render_markup_to_svg_snapshot`/`typst_markup_to_validated_svg`) — real,
  deliberate behavior this wave preserves exactly. What was missing was crate-level isolation: the
  plugin's own `Cargo.toml` still linked all four crates directly. `usvg` had no isolation at all
  (`svg_to_vobject`/`collect_svg_paths` used it inline).

## Framework interfaces

Both new crates sit at `🧰️framework/🔨️modules/<name>/📦️packages/🦀️rust`, matching the existing
`🔢️hash`/`📐️geometry`/`🧮️math` single-purpose-module pattern (`[lib] path = "🦀️.rs"` →
`#[path = "../../🦀️.rs"] mod component; pub use component::*;`), and both build on
`semio_framework_geometry::{BezPath, Affine}` — the pre-existing kurbo-backed facade whose own
docstring already anticipated this ("Escape hatch for the renderer bridge crate to interop with
`kurbo`/`vello`" on `BezPath::to_kurbo`/`Affine::to_kurbo`).

### `semio-framework-raster` (`🧰️framework/🔨️modules/🖌️raster`)

```rust
pub struct FillOp { pub path: BezPath, pub transform: Affine, pub color: [f32; 4] }
pub struct StrokeOp { pub path: BezPath, pub transform: Affine, pub color: [f32; 4], pub width: f64 }
pub enum DrawOp { Fill(FillOp), Stroke(StrokeOp) }
pub struct VectorScene { pub ops: Vec<DrawOp> }
impl VectorScene {
    pub fn new() -> Self;
    pub fn push(&mut self, op: DrawOp);
    pub fn fill(&mut self, path: BezPath, transform: Affine, color: [f32; 4]);
    pub fn stroke(&mut self, path: BezPath, transform: Affine, color: [f32; 4], width: f64);
}

#[derive(Debug)]
pub enum RasterError { Adapter(String), Device(String), Render(String), ReadbackChannelClosed, ReadbackMap(String) }
impl std::fmt::Display for RasterError { ... }
impl std::error::Error for RasterError {}

pub struct SceneRasterizer { /* private: wgpu::Device, Queue, vello::Renderer, Texture, TextureView, Buffer, sizes */ }
impl SceneRasterizer {
    pub async fn new(width: u32, height: u32) -> Result<Self, RasterError>;
    pub fn render(&mut self, scene: &VectorScene, background: [f32; 4]) -> Result<Vec<u8>, RasterError>;
}
```

`vello`/`wgpu` are named only inside this crate's `component.rs` (implementation, not signatures).

### `semio-framework-typeset` (`🧰️framework/🔨️modules/🔤️typeset`)

```rust
pub trait MarkupTypesetter { fn render_svg(&self, markup: &str) -> Option<String>; }
pub struct TypstTypesetter;
impl MarkupTypesetter for TypstTypesetter { ... }
pub fn default_typesetter() -> TypstTypesetter;

pub fn svg_natural_size(svg: &str) -> Option<(f64, f64)>;
pub fn svg_outline_paths(svg: &str, scale: f64, flip_y_offset: f64) -> Option<Vec<BezPath>>;
```

Split into "markup → SVG string" (`typst`/`typst-svg`/`typst-assets`, unchanged from the plugin's
original `TypstTextRenderer`) and "SVG string → vector paths" (`usvg`, generalized from the
plugin's original `svg_to_vobject`/`collect_svg_paths`/`map_svg_point`) as two separate functions
— deliberately, not fused into one call — so the plugin's existing stdio-SVG-codec validation step
can still sit between them exactly as before (Typst SVG → stdio `parse_svg_xml`/`write_svg_xml`
round-trip → `svg_outline_paths`), with zero behavior change to that validation.

`typst`/`typst-svg`/`typst-assets`/`usvg` are named only inside this crate's `component.rs`.

## Proof: no third-party type leaks through either interface

```
$ grep -n "^pub \|    pub fn\|    pub async fn" 🧰️framework/🔨️modules/🖌️raster/🦀️.rs | grep -iE "wgpu|vello"
(empty)
$ grep -n "^pub \|    pub fn" 🧰️framework/🔨️modules/🔤️typeset/🦀️.rs | grep -iE "typst|usvg"
(empty — the only matches for "typst" are the first-party `TypstTypesetter` identifier and
`default_typesetter()`, neither of which names a `typst::*` type)
```

`SceneRasterizer`'s six device/pipeline fields (`device: wgpu::Device`, `queue: wgpu::Queue`,
`renderer: vello::Renderer`, `target_texture`, `target_view`, `readback_buffer`) are all private
(no `pub`), confirmed by inspection of the struct definition.

## What moved

- `VelloRenderer` (video engine) now wraps `semio_framework_raster::SceneRasterizer` — its own
  public API (`new(width, height)`, `render_capture(...)`) is unchanged, so every caller
  (`preview_scene_window_winit`, the export path, both test modules) needed zero changes beyond
  the internals of `video/component.rs` itself.
- `build_vello_scene`/`paint_mobject`/`scene_affine`/`read_pixels`/`create_target_texture` moved
  into `semio-framework-raster`, generalized from `Sobjects`/`Camera`/`AnimateConfig` domain types
  to the first-party `VectorScene`/`DrawOp` vocabulary. The plugin's `build_vector_scene`/
  `paint_mobject`/`scene_affine` (renamed, same shape) now build a `VectorScene` instead of a
  `vello::Scene`.
- `TypstTextRenderer`/`TextRenderer` (text engine) moved to `semio-framework-typeset` as
  `TypstTypesetter`/`MarkupTypesetter` (renamed only because "text" already means something
  narrower — plain-text `Sobject` — elsewhere in this same file). `typst_asset_font_list`,
  `typst_compile_markup_to_svg` (and its `AnimateTypstWorld`), `typst_markup_to_svg` all moved.
  `svg_to_vobject`'s usvg tree walk (`collect_svg_paths`/`map_svg_point`) moved as
  `svg_outline_paths`; `typst_markup_to_validated_svg`/`render_markup_to_svg_snapshot` (the
  stdio-SVG-codec validation step) stayed in the plugin unchanged except for the renderer type.

## Bug found and fixed during test-driven development

`read_pixels`'s `copy_texture_to_buffer` used `bytes_per_row: Some(4 * width)` verbatim from the
original plugin code. That is only valid when `4 * width` is itself a multiple of
`wgpu::COPY_BYTES_PER_ROW_ALIGNMENT` (256) — true for the plugin's own existing test (`64×64`,
`4*64=256`) but false for a smaller fixture (`32×32`, `4*32=128`), which is exactly what this
wave's own GPU fixture test used. First run failed with wgpu's real validation error ("Bytes per
row does not respect COPY_BYTES_PER_ROW_ALIGNMENT"). Fixed by padding the readback buffer/copy to
`align_bytes_per_row(width * 4)` and stripping the padding back out per row before returning the
tightly-packed `Vec<u8>`. This was a real latent bug in the moved code, not something introduced
by the move — the original plugin code had the same bug, just never exercised it because every
caller happened to use dimensions that were multiples of 64. Covered by a new pure unit test
(`align_bytes_per_row_pads_to_wgpu_alignment`, no GPU needed) plus the existing GPU fixture test
at the exact `32×32` size that exposes it.

## Kurbo note (scope fence coordination)

`vello::kurbo::Stroke` (video renderer) and `kurbo::Affine`/`.to_kurbo()` call sites
(`scene_affine`, `static_layer_hash`) are fully absorbed into this wave's rewrite: the
`vello::kurbo::Stroke` construction now happens entirely inside `semio-framework-raster`
(`build_vello_scene`), and `scene_affine`/`static_layer_hash` were simplified to use
`semio_framework_geometry::Affine` directly (`Affine::new(...) * camera.transform`,
`mobj.transform().as_coeffs()`) instead of round-tripping through `.to_kurbo()` — both `Affine`
methods that already existed on the framework type before this wave. By the time this edit
landed, the `kurbo` peer had already removed the bare `kurbo = "0.13.1"` line from
`✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` entirely (confirmed via re-read
immediately before every edit, per the scope fence) — their own wave report is
`📓️wave-text-and-path.md` in this same `🔍️research` folder.

## What remains and why (out of scope for this slice)

- **W10 (link-level)**: the ticket's own "scope correction" section notes that even a spotless
  plugin manifest still links third-party code transitively through `semio-framework`/
  `semio-framework-os-kernel`/`semio-framework-geometry` (`gltf`, `blake3`, `serde`, `kurbo`, ...).
  `semio-framework-raster`/`semio-framework-typeset` add `wgpu`/`vello`/`typst*`/`usvg` to that
  same transitive-but-first-party-manifest set. Making the framework's OWN dependency-tree
  first-party (hand-rolled BLAKE3, hand-rolled typesetting/GPU rasterization) is explicitly out of
  scope for this slice — the ticket says plainly "Do NOT attempt to reimplement a typesetting
  engine or a GPU compute rasterizer from scratch. That is not the right answer."
- **Differential oracle in `[dev-dependencies]`**: the ticket's general test-driven rule asks for a
  differential test against the third-party crate as a dev-only oracle, verifying a from-scratch
  reimplementation. That pattern doesn't apply cleanly here — `typst`/`usvg`/`vello`/`wgpu` ARE the
  production implementation behind the interface, not something reimplemented from scratch, so
  they stay `[dependencies]` (not `[dev-dependencies]`) in their respective framework crates, by
  design and explicit ticket instruction. In their place: exact-coordinate fixture tests
  (`svg_outline_paths_flips_y_and_scales_exactly` against a hand-authored SVG with known geometry,
  independent of Typst's actual output) and a pure unit test for the wgpu alignment fix.
- **`kurbo` and `image`**: explicitly out of scope per the scope fence (owned by two other
  concurrent agents on this same file). Both lines were gone from
  `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` by the time this wave's edits landed.

## Verification (verbatim tails)

### `cargo test -p semio-framework-raster`

```
running 3 tests
test component::tests::align_bytes_per_row_pads_to_wgpu_alignment ... ok
test component::tests::vector_scene_push_order_is_stable ... ok
test component::tests::scene_rasterizer_renders_expected_pixel_count ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.71s

   Doc-tests semio_framework_raster

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

A real wgpu adapter WAS available in this environment (`scene_rasterizer_renders_expected_pixel_count`
ran the actual GPU path, not the `[DEBUG] no adapter — skipping` fallback branch) — confirmed by
the padding-fix regression having a real failure to fix in the first place.

Both raster commands used an isolated `CARGO_TARGET_DIR` (a scratchpad dir) and `RUSTC_WRAPPER=""`
to avoid the shared workspace's `target/` lock contention — at the time of this wave, `ps aux`
showed 80-200 concurrent `rustc`/`cargo` processes from other sessions and `sysctl vm.swapusage`
showed 24.3/25.6 GB swap used, load average 100-130. `cargo check -p semio-framework-raster`
against the SHARED (non-isolated) target dir sat with near-zero CPU progress for over an hour under
that contention before being cancelled in favor of the isolated-dir run.

Also caught during this wave's first raster test run: `read_pixels`'s `bytes_per_row: Some(4 *
width)` is only valid when `4 * width` is a multiple of `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT` (256)
— true for `64×64` (the plugin's own pre-existing test) but false for `32×32` (this wave's new
fixture), and the first run failed with wgpu's real validation error before the alignment fix
below was applied — see "Bug found and fixed" above.

### `cargo test -p semio-framework-typeset`

```
running 6 tests
test component::tests::svg_outline_paths_none_on_garbage_input ... ok
test component::tests::svg_outline_paths_flips_y_and_scales_exactly ... ok
test component::tests::svg_natural_size_matches_fixture_dimensions ... ok
test component::tests::typst_empty_markup_is_none_or_svg ... ok
test component::tests::typst_plain_text_compiles_to_svg ... ok
test component::tests::svg_outline_paths_extracts_at_least_one_path ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

   Doc-tests semio_framework_typeset

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

(Isolated target dir; cold build of typst's full dependency tree took 33m under the contention
described above.) `cargo clippy -p semio-framework-typeset --all-targets -- -D warnings` and the
equivalent `-p semio-framework-raster` command both finished with **zero warnings** (`Finished
`dev` profile [unoptimized] target(s) in ...` and no `warning:` lines from either crate — the
`raster` run's one real finding, `manually reimplementing div_ceil`, was fixed by switching
`align_bytes_per_row` to `u32::div_ceil` before the clean run shown here).

### `cargo build -p semio-s-plugin-animate` (native) — BLOCKED, not this wave's code

Three separate foreground attempts (isolated target dir, full output captured to a log file each
time) all failed identically, and NOT in this wave's code:

```
error[E0277]: the trait bound `SpaceHistoryMutation: protocol::FromValue` is not satisfied
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🦀️.rs:31:10
[... 42 more error[E0277]/error[E0599] in the same Mutation/ToValue/FromValue/serde::Serialize
     family, all inside semio-framework-os-kernel/semio-framework ...]
error: could not compile `semio-framework-os-kernel` (lib) due to 75 previous errors; 29 warnings emitted
```

Evidence this is unrelated concurrent churn, not this wave's regression:
- `grep -n "🖌️raster\|🔤️typeset\|semio-s-plugin-animate\|semio_framework_raster\|semio_framework_typeset"`
  over the full captured build log: **zero matches**, across all three attempts. Every one of the
  43-166 errors (the count grew between attempts, from 43 to 166, as the other session's edit
  progressed further) is `error[E0277]`/`error[E0599]` about `Mutation`/`protocol::ToValue`/
  `protocol::FromValue`/`serde::Serialize`/`serde::Deserialize` trait bounds, in files this wave
  never opened.
- `git status --short 🧰️framework/🔨️modules/📡️replication 🧰️framework/📦️packages/🦀️rust
  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust` shows those exact modules as modified-but-
  uncommitted throughout this wave's session, including a `Cargo.toml` change in
  `📡️replication` (matching the plan's own W10 note that `base64` is being re-homed out of
  `📡️replication/⚙️codec`) — a different concurrent session mid-refactor of the
  `Mutation`/`ToValue`/`FromValue` trait surface everything in the workspace derives against.
- Cannot be worked around from this wave's side: `semio-s-plugin-animate` depends on
  `semio-framework-os-kernel`/`semio-framework` unconditionally (for reasons unrelated to
  video/text), so cargo never reaches this crate's own compilation step at all while its upstream
  is broken.

### `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-animate` — BLOCKED, same cause

Same story one dependency layer further up:

```
error: could not compile `semio-framework` (lib) due to 166 previous errors; 8 warnings emitted
```

`grep` over the full log for this wave's crate/file names: zero matches, same as above.

**Neither native nor wasm32-wasip2 plugin-level build could be confirmed green in this session.**
That is a genuine gap in this wave's own verification, honestly reported rather than papered over
— it is not, however, evidence against this wave's change: three attempts spread over roughly two
hours all failed at the identical, unrelated `Mutation` trait-bound error signature, in crates this
wave never touched, while a different session's uncommitted edit to exactly those crates sat in
`git status` the whole time. Re-running `cargo build -p semio-s-plugin-animate` (and the
wasm32-wasip2 variant) once that other session's edit lands is the natural way to close this gap.

### `grep -rnE '^(typst|typst-svg|typst-assets|usvg|vello|wgpu) ?=' ✏️s --include=Cargo.toml`

```
(no output — exit 1)
```

Empty, as required. Re-checked repeatedly across this wave's session (including after each of the
three blocked build attempts above, in case a concurrent edit to the animate plugin's own
`Cargo.toml` reintroduced one of the six lines) and stayed empty throughout.
