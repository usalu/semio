# 🎯️ `image`/`png` cluster closed with a first-party codec; `vello` cluster re-classified with
deeper evidence than the prior pass — confirmed guest-safe to replace, sized, not attempted

## Headline — before/after (`cargo tree`, lock-free, cannot go stale)

```bash
for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity; do
  cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
    | grep -v 'Updating\|^error\|^ ' | sed 's/ (\*)$//' | awk '{print $1}' \
    | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
done
```

| plugin | before (this session) | after |
|---|---|---|
| `semio-s-plugin-puzzle` | **67** | **63** |
| `semio-s-plugin-flow` | **68** | **64** |
| `semio-s-plugin-trinity` | **67** | **63** |

`image` is confirmed **completely absent** from all three plugins' `wasm32-wasip2` graphs
(`cargo tree -i image` → "nothing to print" on all three, full untruncated output). `image`'s own
pinned `png@0.18.1` instance and its exclusive tail (`byteorder-lite`, confirmed absent via `-i`) are
gone with it. `vello`'s own `png@0.17.16` instance (and `flate2`/`miniz_oxide`/`fdeflate`/
`crc32fast`/`adler2`/`simd-adler32`, all shared with `vello`, not `image`) remain — expected, see
Cluster 2. Native/browser tiers are unaffected: `image` still resolves there
(`cargo tree -p semio-s-plugin-puzzle -i image` with no `--target`, unchanged path through
`arboard`/`semio-framework-ui` and the pre-existing `not(all(wasm32,p2))`-gated `icon_codec`
raster-decode arm in `🖼️canvas/🦀️.rs`) — no capability lost on the targets that had it.

## Cluster 1 — `image`/`png` raster decode: CLOSED with a first-party codec, no gate needed

### What actually needed it (traced, not assumed)

Repo-wide grep of every `image::` reference inside `♾️infinite`'s own source tree found exactly
three call sites, not the "several" the ticket brief's scoreboard implied:

1. `🖼️canvas/🦀️.rs:1724` (`raster_icon_bytes_to_rgba`) — already `#[cfg(not(all(target_arch =
   "wasm32", target_env = "p2")))]`-gated (icon *painting*, host/browser-only; `IconPaintCache::
   get_or_build`'s only caller is unconditionally `None` on wasip2, per a prior pass).
2. `🌍️world/🦀️.rs:11310` (`apply_reference_image_bytes`) — already gated the same way, and
   independently confirmed dead-for-now (zero callers repo-wide) by `📓️infinite-host-deps-split.md`.
3. `🌍️world/🦀️.rs:13747-13752` — `#[cfg(test)]` fixture construction only.

None of these three needed touching — they were already correctly gated or test-only, and none was
the driver of the **unconditional** `image = { version = "0.25", default-features = false,
features = ["png"] }` at line 58 of `♾️infinite`'s own `Cargo.toml`. Tracing that line's own comment
("Required by the path-mounted `🗺️surface/🏔️terrain/🦀️.rs`") to the shared source file found the
real, sole unconditional consumer: `decode_terrarium_png(bytes) -> image::RgbaImage` in
`🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs`, decoding Mapzen/AWS Terrarium RGB-encoded
elevation-tile PNGs (`R*256 + G + B/256 - 32768`) for `TerrainSessionCore::upload_elevation_tile`.
That file is `#[path]`-mounted (not crate-dependency-mounted, to avoid a cycle — surface depends on
infinite) into **both** `semio-framework-os-infinite` and `semio-framework-surface`, so both
manifests declared `image` unconditionally for the same shared file.

### Classification: verdict — genuinely guest-reachable for real pixel data, and now replaceable outright

`upload_elevation_tile` is an ordinary, unconditional public method on `TerrainSessionCore`
(no `#[cfg]` anywhere in the file) — real elevation values are read per-pixel
(`sample_elevation`) to build the terrain mesh, not just measured for a bounding box the way
`compiler`'s SVG-icon sizing was (see `📓️compiler-vello-guest-split.md`'s cluster 1). This is a
**full RGBA decode need**, not a dimensions-only one — `semio-framework-intrinsic-size` (headers/
attributes only) would not have covered it. It maps exactly to `semio-framework-pixels`'s
`decode_png(bytes) -> Result<RasterImage, RasterError>`, which the ticket brief flagged as newly
landed and oracle-verified (12/12 vs `png` differential) but not yet checked against this call site.
It does: no third-party dependency at all (`[dependencies]` empty in
`🧰️framework/🔨️modules/🖼️pixels/📦️packages/🦀️rust/Cargo.toml`, `png` only in
`[dev-dependencies]` as the oracle), full RGBA8 output (`RasterImage { width, height, pixels: Vec<u8> }`,
row-major, 4 bytes/pixel — the exact shape `sample_elevation`'s pixel math needs), and it is already
adopted by four other plugins (`remodel`, `animate`, `draw`, `lowpoly`) via the identical
`path = "…/🖼️pixels/📦️packages/🦀️rust", package = "semio-framework-pixels"` dependency shape used
here.

### The fix — full replacement, not a target gate

Because the elevation-tile decode is needed on **every** target this file is mounted into (native,
browser, `wasm32-wasip2`), this is not a two-implementations-behind-one-API split like the
`compiler`/`raster`/`typeset` tier splits — it is a straight first-party swap, the same shape as the
ticket's `base64_codec` replacement in the same crate. `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/
🦀️.rs`:

- `FrameworkSurfaceTerrainError::Image(image::ImageError)` → `Image(semio_framework_pixels::
  RasterError)`, `From` impl updated to match.
- `decode_terrarium_png(bytes) -> Result<image::RgbaImage, _>` → returns
  `semio_framework_pixels::RasterImage` via `semio_framework_pixels::decode_png(bytes)?`.
- `sample_elevation(image: &image::RgbaImage, …)` → takes `&semio_framework_pixels::RasterImage`;
  pixel access changed from `image.get_pixel(x, y)` to a manual flat-buffer index
  (`pixels[(y*width+x)*4..][..4]`) — `RasterImage` has plain `pub width`/`pub height`/`pub pixels`
  fields, no `get_pixel` method, so this is the one real code-shape change (not a signature-only
  swap).
- `DecodedElevationTile.image: image::RgbaImage` → `semio_framework_pixels::RasterImage`.
- Three test-fixture helpers (`solid_terrarium_png`, `gradient_terrarium_png`,
  `sample_elevation_clamps_out_of_bounds_coordinates`) that built `image::RgbaImage`/encoded PNG
  bytes via `image::DynamicImage::…::write_to(…, ImageFormat::Png)` were rewritten against
  `semio_framework_pixels::RasterImage::new` + `semio_framework_pixels::encode_png` — same fixture
  shapes, same assertions, zero test behavior change (byte-for-byte same R/G-encoded elevation
  values written per pixel).

No caller of `TerrainSessionCore::upload_elevation_tile`/`terrain_tile_mesh_json`/`evict_terrain_tile`
anywhere in the repo needed to change — the public API of `terrain.rs` (method names, signatures
returning `bool`/`String`/JSON) is unchanged; only the *internal* pixel-buffer type moved.

### Cargo.toml

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`: the
  **unconditional** `image = { version = "0.25", default-features = false, features = ["png"] }`
  (line 58, the only unconditional `image` entry in this manifest — the other `image` declaration,
  `features = ["png", "jpeg", "webp", "gif"]`, was already correctly confined to the
  `not(all(target_arch = "wasm32", target_env = "p2"))` target table for icon-painting) is now
  `semio-framework-pixels = { path = "../../../../../../🔨️modules/🖼️pixels/📦️packages/🦀️rust",
  package = "semio-framework-pixels" }`, present on **every** target including `wasm32-wasip2`.
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml`: added the same
  `semio-framework-pixels` path dependency alongside the pre-existing `image` entry. `image` was
  **not** removed here — `semio-framework-surface`'s own `🎨️paint/🦀️.rs` and `🗺️tiled-map/🦀️.rs`
  (siblings path-mounted into the same crate, not into `os-infinite`) still call `image::
  load_from_memory`/`image::codecs::png::PngEncoder` directly and were out of scope this pass —
  `semio-framework-surface` is not in any s-plugin's `wasm32-wasip2` dependency graph at all
  (`cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i semio-framework-surface` →
  "did not match any packages"), so it does not affect the scoreboard; only `os-infinite`'s copy of
  the shared file needed to lose `image`, and it did.

### Verification

```
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i image   → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i image   → nothing to print
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i image   → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i byteorder-lite → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i "png@0.18.1"   → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i "png@0.17.16"
png v0.17.16
└── vello v0.7.0
    └── semio-framework-os-infinite v0.1.0 (…/♾️infinite/📦️packages/🦀️rust)
        └── semio-s-plugin-puzzle v0.1.0 (…/🧩️puzzle/📦️packages/🦀️rust)
# native/browser — capability preserved where it can actually run:
$ cargo tree -p semio-s-plugin-puzzle -i image        (no --target)
image v0.25.10
└── arboard v3.6.1
    └── semio-framework-ui v0.1.0
        └── … → semio-framework-os-infinite v0.1.0 (also the gated icon-paint arm) → semio-s-plugin-puzzle
```

`cargo metadata --no-deps` — exit 0, both edited manifests parse clean. `rustfmt --edition 2021
--check` on `🏔️terrain/🦀️.rs` — exit 0, no diff (valid syntax, matches existing style).
`cargo check -p semio-framework-pixels` and `cargo check -p semio-framework-pixels --target
wasm32-wasip2` — both `Finished`, 0 errors (confirms the API surface consumed here —
`decode_png`, `encode_png`, `RasterImage::new`, the `width`/`height`/`pixels` fields, `RasterError`
— is itself sound on both targets before relying on it).

### What is NOT proven, stated plainly

An end-to-end `cargo check -p semio-framework-os-infinite` (native) was attempted **three times**,
foreground, and blocked three times by three **different**, unrelated, live concurrent-peer
failures, none naming any file this pass touched:

1. First two attempts: `semio-framework-graph`'s build script threw `Invalid taxonomy schema:
   generatorContracts["wgpu-frame-worker"] tracked output "…🎯️targets/🧊️wgpu/…" is missing` — traced
   to an **uncommitted** (`git status` shows `MM`) in-flight edit of
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, unrelated to this ticket.
2. Third attempt (after the taxonomy edit apparently progressed): `semio-framework-ui` failed with
   `couldn't read …🧱️elements/☑️Select/🎯️targets/🧊️wgpu/🦀️component.rs: No such file or directory` (a
   file mid-move), then `semio-framework-graph` failed a second, different way —
   `ValueType: serde::Serialize`/`DeserializeOwned` not satisfied, tracing into
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/…` — matching the exact
   serde-to-`ToValue`/`FromValue` migration this ticket's own `verified-outcomes.md` documents as
   live and in-progress elsewhere in the tree right now.

Three attempts, three distinct unrelated symptoms, zero mentions of `🏔️terrain`, `🖼️pixels`, or
either edited `Cargo.toml` in any error — consistent with heavy, ongoing concurrent churn rather
than a regression from this pass. Per the ticket's own hard constraint ("if you are overruled, check
before re-reverting" / never fight a live peer), this was not chased further. As independent
positive evidence: `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm` — the
ticket's own named "currently clean" baseline — **succeeded, 0 errors**, `Finished dev profile
[unoptimized] target(s) in 54.65s`, confirming `semio-framework-os-kernel` itself (a dependency of
every plugin including `os-infinite`) is not broken by the same churn at that moment; the failures
above are specific to `os-infinite`'s own extra dependency edges (`graph`, `ui`) that `draw-fsm`
doesn't pull.

## Cluster 2 — `vello`/CPU-drawing: re-investigated with deeper evidence than the prior pass, confirmed replaceable-in-principle, sized, correctly NOT attempted this pass

The prior session (`📓️compiler-vello-guest-split.md`) traced `vello 0.7.0`'s own published
manifest and confirmed `peniko`, `skrifa`, `png`, `vello_encoding`, `bytemuck` are non-optional
regardless of `default-features = false`, and that `🎲️board` uses `vello::Scene`/`peniko`/`kurbo`
types unconditionally (69 non-test references to bare `Scene` alone), concluding a first-party
rewrite was out of scope for that pass. This session went one level deeper — tracing not just *that*
`Scene` is used unconditionally, but exactly *what operations* are performed on it and where real
rasterization happens — because the ticket brief specifically asked to separate "drawing-command
construction" from "rasterization."

### The abstraction boundary already exists — this is the key finding

`🎲️board`'s own source (`🔌️ports/➡️directed/🦀️.rs`, `➕️normal/🦀️.rs`, `🕸️dag/🦀️.rs`, `🎲️board/
🦀️.rs`) contains **zero** direct `vello::`/`peniko::`/`kurbo::` references — confirmed by grep. Every
one of the 69 `Scene` references resolves through `use super::canvas::{Affine, FillRule,
RasterImage, Rect, Scene};`, and `♾️infinite/🖼️canvas/🦀️.rs` already defines:

```rust
mod renderer {
    pub(super) mod vello_backend {
        pub use vello; pub use vello::kurbo; pub use vello::peniko; pub use vello::Scene; …
    }
    pub struct Scene(pub(crate) backend::Scene);   // ← first-party newtype, ALREADY wraps vello::Scene
    impl Scene {
        pub fn new() -> Self { … }
        pub fn fill(&mut self, rule: FillRule, transform: Affine, paint: impl Into<Paint>, …) { … }
        pub fn stroke(&mut self, stroke: &Stroke, …) { … }
        pub fn draw_image(&mut self, image: &RasterImage, transform: Affine) { … }
        pub fn append(&mut self, other: &Scene, transform: Option<Affine>) { … }
        pub fn push_layer(&mut self, …) { … }
        pub fn push_clip_layer(&mut self, …) { … }
        pub fn pop_layer(&mut self) { … }
        pub fn is_empty(&self) -> bool { … }
        pub fn path_count(&self) -> usize { … }
        pub fn retirement_step(&mut self) -> bool { … }       // pops one internal buffer entry
        pub fn retirement_is_empty(&self) -> bool { … }
        pub fn vello_scene(&self) -> &backend::Scene { &self.0 }   // the only "escape hatch"
    }
}
```

`Affine`/`Stroke`/`Cap`/`Color`/`Paint`/`FillRule`/`BlendMode`/`RasterImage` are **also** already
first-party newtypes over `kurbo`/`peniko` values, with explicit `From`/`to_kurbo()` conversions —
this is the exact "two implementations behind one API" shape the ticket's own precedent
(`raster-tier-split.md`, `typeset-tier-split.md`) established, just not yet finished for the
rendering backend itself.

### Classification: verdict (b) confirmed — guest builds commands and steps disposal; never rasterizes

Traced every method actually reachable from unconditional `🎲️board` code:

| method | what it does | guest-reachable? |
|---|---|---|
| `Scene::new`, `.fill`, `.stroke`, `.append`, `.draw_image`, `.push_layer`, `.push_clip_layer`, `.pop_layer` | record draw commands into the scene | yes — ~150 call sites, real production code (e.g. `➕️normal/🦀️.rs`'s hover/selection preview, edge/node rendering) |
| `.is_empty`, `.path_count` | inspect recorded-command count | yes — one production call (`encoded_scene_hint()` in `➕️normal/🦀️.rs:10482`, a diagnostic "hint", not an exact-value contract), remainder are test assertions |
| `reserve_opaque_scene_retirement`/`publish_opaque_scene_retirement`/`.retirement_step`/`.retirement_is_empty` | incrementally pop **one internal vello-encoding buffer entry per call**, spreading a large `Scene`'s `Drop` cost across ticks | yes — real production call sites in `🔌️ports/➡️directed/🦀️.rs:700-708` and `➕️normal/🦀️.rs:3610-3615,10362-10367`; this is precisely CLAUDE.md's "support progress and cancellation for all expensive operations" mandate, already implemented, just implemented in terms of `vello_encoding::Encoding`'s own internal field shape (`resources.glyph_runs`, `.glyphs`, `.normalized_coords`, `.color_stops`, `.patches`, `path_tags`, `path_data`, `draw_tags`, `draw_data`, `transforms`, `styles`) |
| `Renderer::new`, `.render_to_texture`, `RenderParams` (real rasterization) | GPU paint | **no** — exactly ONE call site in this crate (`🖼️canvas/🦀️.rs:1408-1446`), already inside the `not(all(wasm32,p2))`-gated wgpu/`JsValue` session code; a second, sibling call site exists in `📺️renderer/🧑️‍🎨️engine/…/EngineCanvas/🧊️component.rs` (a separate, host-only "wgpu-frame-worker" product, not part of any plugin's wasip2 graph) |
| `draw_glyph`/glyph-run population | text rasterization | **never called** anywhere in `🎲️board`/`🖼️canvas` — `glyph_runs`/`glyphs` buffers are only ever `.pop()`-ed (retirement) or `.is_empty()`-checked, never populated; real glyph painting happens exclusively through `vello_svg::append_tree`/`SvgDocument`, both already `not(all(wasm32,p2))`-gated |

**This confirms the brief's hypothesized verdict (a): the guest only ever builds and incrementally
disposes scene data; it never rasterizes, and on `wasm32-wasip2` the glyph-run buffers are always
empty in practice.** Unlike `📓️infinite-host-deps-split.md`'s `usvg`/`rustybuzz` finding (a genuine
"no first-party alternative without a real algorithmic reimplementation" result, since resolved by
`intrinsic-size`), nothing here needs vello's *rendering* — only its *value/encoding* types, exactly
the class of dependency the `raster`/`typeset`/`compiler` splits already replaced.

### Why this was NOT attempted this pass, with an honest size estimate

Two independent things distinguish this from a mechanical rename, both real:

1. **`retirement_step`'s granularity is coupled to `vello_encoding::Encoding`'s specific multi-buffer
   internal layout** (11 separate `Vec`s popped one at a time), not just to `Scene`'s public API. A
   first-party replacement needs its own internal representation to support equivalent single-step,
   O(1)-per-tick disposal — achievable (a `Vec<DrawOp>` popped one element at a time is *simpler*
   than mirroring 11 buffers, since glyph buffers are always empty on this target and don't need a
   guest-side representation at all), but it is a real design decision, not a mechanical port.
2. **The tree is under heavy, active concurrent churn right now** (three distinct unrelated build
   failures hit across three retries in Cluster 1's verification above, one of them touching
   `semio-framework-os-kernel-neural-engine`'s serde derives — the exact seam this ticket's own
   `verified-outcomes.md` documents as a live in-progress migration). `🖼️canvas/🦀️.rs` and
   `🎲️board/**` are large, frequently-touched, unconditionally-compiled files at the center of this
   churn. Landing an unverifiable multi-file rewrite of the rendering-command type on this exact
   surface, while `cargo check` cannot currently reach a clean baseline for reasons independent of
   this change, is the wrong risk/reward trade for this pass.

**Estimated size if implemented**: replace the `backend::Scene` field's type and the twelve `impl
Scene` method bodies (table above) with a first-party command-recording representation; convert to
a real `vello::Scene` only at the two already-host-gated render call sites (`🖼️canvas/🦀️.rs:1446`,
`EngineCanvas/🧊️component.rs:1229`) — no other call site in `🎲️board`/`🖼️canvas` needs to change,
since none references `vello::`/`peniko::`/`kurbo::` directly. Value types (`Affine`, `Stroke`,
`Cap`, `Color`, `Paint`, `FillRule`, `BlendMode`, `RasterImage`) are already first-party newtypes and
would only need their `to_kurbo()`/`.0` conversions retargeted or kept as-is if `kurbo`/`peniko` stay
as direct (non-`vello`) dependencies for value math — traced separately this session:
`peniko`/`kurbo`'s own forward dependency trees (`cargo tree -p peniko@0.6.1`, `-p peniko@0.4.1`)
resolve to only `color`, `arrayvec`, `polycool`, `smallvec`, `linebender_resource_handle` — **none**
of `skrifa`/`read-fonts`/`font-types`/`png`/`vello_encoding`/`bytemuck` come from `peniko`/`kurbo`
themselves, only from `vello`'s own sibling `[dependencies]` entries. So dropping `vello` +
`vello_encoding` while keeping direct `kurbo`/`peniko` dependencies (a smaller, self-contained,
already-mostly-wrapped value-type family) would shed roughly 19 crates per plugin (`vello`,
`vello_encoding`, `skrifa@0.40`, its exclusive `read-fonts`/`font-types` instance, `png@0.17.16`,
`flate2`, `miniz_oxide`, `fdeflate`, `crc32fast`, `adler2`, `simd-adler32`, `guillotiere`, `moxcms`,
`pxfm`, `svg_fmt`, `bytemuck`, `id-arena`, `static_assertions`, `core_maths`), taking puzzle/trinity
from **63 toward roughly 44** — leaving `kurbo`/`peniko`/`color`/`arrayvec`/`smallvec`/`polycool`/
`linebender_resource_handle` (~7 crates) as a smaller residual third-party value-type family, itself
a candidate for a later, fully-first-party pass if the ticket's "zero third-party" goal is pursued to
the end. **This is a genuine deliverable-sized piece of work** (on the order of the ticket's own
blake3/DEFLATE/parry3d first-party rewrites, per the prior session's own comparison, which this
session's deeper trace does not overturn) — correctly scoped and left for a dedicated pass, not
attempted here.

## Files touched this session

- `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs` — `image::` → `semio_framework_pixels::`
  throughout (production decode fn, error type, `sample_elevation`, `DecodedElevationTile`, three
  test fixtures). Zero change to any public method signature.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml` — unconditional
  `image` replaced with `semio-framework-pixels` path dependency.
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` — added the same
  `semio-framework-pixels` path dependency (additive; `image` itself untouched, still needed by
  `paint`/`tiled-map` in that crate, which is not part of any plugin's wasip2 graph).

No file in `✏️s/🔌️plugins/**`, `🎲️board/**`, or `🖼️canvas/🦀️.rs` was touched — the vello cluster
(Cluster 2) was investigated and classified but deliberately left unmodified, per the analysis
above.

## What is proven vs. not proven — stated plainly

**PROVEN** (lock-free `cargo tree`, cannot go stale): `semio-s-plugin-puzzle`/`trinity` wasip2
third-party count **67 → 63**, `semio-s-plugin-flow` **68 → 64**; `image` completely absent from all
three; native/browser `image` resolution unchanged (no capability regression);
`semio-framework-pixels` itself compiles clean natively and for `wasm32-wasip2`; both edited
manifests parse (`cargo metadata`); the edited `.rs` file is syntactically valid (`rustfmt --check`,
exit 0); `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm` succeeds end-to-end
(0 errors) at the same moment `os-infinite`'s own check is blocked, isolating the blocker to
`os-infinite`'s extra dependency edges, not to shared foundation crates or this pass's edits.

**NOT proven**: an end-to-end `cargo check -p semio-framework-os-infinite` or
`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-{puzzle,flow,trinity}` at 0 errors —
blocked three times, three different unrelated ways, by live concurrent peer edits (taxonomy schema,
a mid-move UI file, and an in-progress serde→`ToValue` migration in `neural-engine`), none naming any
file this pass touched. The `vello` cluster's "guest never rasterizes" classification is proven by
grep-traced call chains to real production call sites (not by inspection alone), but the
first-party-replacement itself was not implemented — see sizing above.
