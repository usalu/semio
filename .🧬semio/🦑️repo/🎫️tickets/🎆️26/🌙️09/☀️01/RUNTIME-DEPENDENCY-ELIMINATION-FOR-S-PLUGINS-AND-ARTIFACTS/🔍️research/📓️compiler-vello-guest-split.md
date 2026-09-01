# 🎯️ compiler text-shaping split closed; vello/CPU-drawing cluster investigated and correctly left alone — 76 → 67/68/67

## Headline — before/after (`cargo tree`, lock-free, cannot go stale)

```bash
for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity; do
  cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
    | sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
done
```

| plugin | before (this session) | after |
|---|---|---|
| `semio-s-plugin-puzzle` | **76** | **67** |
| `semio-s-plugin-flow` | **76** | **68** |
| `semio-s-plugin-trinity` | **76** | **67** |

`rustybuzz`, `ttf-parser`, `read-fonts@0.29.3`, `skrifa@0.31.3`, `font-types@0.9.0`,
`unicode-bidi-mirroring`, `unicode-ccc`, `unicode-properties`, `unicode-script` are gone from all
three plugins' `wasm32-wasip2` graphs (confirmed with full untruncated `-i` output — every one
prints "nothing to print", not truncated). `base64` is additionally gone from puzzle's and
trinity's graphs (flow still carries a *different, unrelated* `base64` edge — see "Not moved" below,
out of my safe scope this session). Native/browser tiers are unaffected — `rustybuzz`, `base64`
still resolve there, confirmed by `cargo tree -i` with no `--target`.

## Cluster 1 — text shaping (`semio-framework-compiler`): CLOSED, host-gated with an honest wasip2 arm

### Classification: verdict (b) — genuinely guest-reachable for *measurement*, not for painting

Traced the only caller of `compiler::compile_{snippet,text,emoji,code}_to_svg` repo-wide: three of
the four (`compile_snippet_to_svg`/`compile_emoji_to_svg`/`compile_text_to_svg`) are called from
`♾️infinite/🖼️canvas/🦀️component.rs`'s `resolve_math_src`/`resolve_emoji_body`/`resolve_text_body`,
which feed `board_resolve_icon_kind` → `BoardResolvedIcon::SvgPlain`. `compile_code_to_svg` has zero
callers anywhere outside the compiler crate's own tests (confirmed dead-for-now, left in place —
public API surface, not proven dead on every future target).

`board_resolve_icon_kind`'s guest-reachability was already established by a predecessor session
(`🔍️research/📓️infinite-host-deps-split.md`, its "Fix attempted and reverted" section): it is called
from `preview_media_natural_size` (`🎲️board/…/🕸️dag/🦀️component.rs`), itself reachable from
`Widget::InputImage { src }`/`DagPreviewContent::Image { src }` → `image_widget_size`/
`measure_preview_content` → `widget_to_dag_node` → `🖥️host/🦀️component.rs`'s DAG-node rebuild,
driven by `set_image_src` — an ordinary, unconditional, guest-reachable mutation handler. Since
`Icon::Math`/`Icon::Emoji`/`Icon::Text` are legal decodings of any `src` string (the decoder doesn't
distinguish "image widget" from "icon" provenance), `compiler::compile_*_to_svg` sits on this same
real path. **Not dead, not host-only by call-chain — the previous ticket's own analysis already
proved this.**

### The key fact that makes gating safe anyway: what the guest-reachable path actually consumes

`preview_media_natural_size`'s only use of the resolved SVG is
`canvas::svg_icon::svg_icon_content_bounds_from_str(&s)` — and that function **already has a
two-arm split** from an earlier pass (`🔍️research/📓️intrinsic-size-wiring.md`): its `wasm32-wasip2`
arm reads only the outer `<svg>` tag's declared `width`/`height`/`viewBox` attributes via
`semio-framework-intrinsic-size` (a first-party, oracle-verified XML-attribute parser) — **never**
the painted glyph geometry. Icon *painting* (`IconPaintCache::get_or_build`, the only consumer that
ever reads real glyph paths) is unconditionally `None` on this target (already gated, prior
session). So the guest-reachable need is exactly "a plausible declared box size," never real glyph
shaping — the same class of relaxation the ticket's `label_byte_world_x` precedent already
established for map labels.

This is the exact trap `📓️infinite-host-deps-split.md` correctly avoided fighting blind: a bare
`#[cfg]` deletion of `compile_snippet_to_svg` etc. would have been a genuine capability loss (no
declared box at all, breaking `widget_to_dag_node`'s compile or silently zeroing every math/emoji/
text-icon-sized widget). The fix instead follows `raster-tier-split.md`/`typeset-tier-split.md`'s
"two implementations behind one identical public API" pattern.

### The fix

`compiler`'s `text`/`math`/`svg` submodules (the real `rustybuzz`-backed shaping/outline/math-table
engine and its SVG serializer) are gated `#[cfg(not(all(target_arch = "wasm32", target_env =
"p2")))]` at the module-mount level in `📦️glue.rs` — `world`/`syntax` (font-byte storage and pure
DSL-grammar parsing, zero `rustybuzz`/`base64` reference, confirmed by grep) stay unconditional so
`compile_snippet_to_svg`'s wasip2 arm can still validate notation syntax and preserve the existing
`Err(CompileError::Syntax(_))` contract for malformed input.

`🦀️component.rs` (the facade): `Fonts`/`fonts()`/`render_to_svg` and the native bodies of all four
`compile_*_to_svg` fns are now native-only. A new wasip2-only sibling, `estimate_svg(char_count,
options)`, estimates the declared box from character count using the same character-width-to-
font-size ratio (`0.62`) already established for the identical class of heuristic fallback elsewhere
in this ticket (`♾️infinite`'s `label_advance`) — disclosed in the function's own docstring as an
intentional precision difference, not a stub: the returned `<svg>` carries **no** glyph paths, since
nothing on this target ever paints one (painting is unconditionally host-only). `compile_snippet_to_
svg`'s wasip2 arm still calls `crate::syntax::parse_formula` first, so invalid notation still
produces `Err` exactly as before. `SnippetOptions`/`SvgSnippet`/`CompileError` (already fully
target-neutral, per the crate's own pre-existing docstring — no `rustybuzz::`/`ttf_parser::` type
ever appears in the public API) needed **zero** changes, so **zero callers outside this crate were
touched** — `♾️infinite/🖼️canvas/🦀️component.rs`'s `resolve_math_src`/`resolve_emoji_body`/
`resolve_text_body` are byte-for-byte unchanged.

Six of the crate's own tests assert real glyph-shaped output (`<path`/`<rect`/`<image`) and were
gated native-only (the wasip2 arm's empty `<svg>` would fail them, correctly — they test the native
engine specifically); `invalid_syntax_is_a_syntax_error_not_a_panic` and
`repeated_calls_reuse_the_lazily_parsed_fonts` assert nothing that depends on real shaping and stay
unconditional (both pass under either arm).

### Cargo.toml

`🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/Cargo.toml`: `rustybuzz`/`base64` moved from
unconditional `[dependencies]` to `[target.'cfg(not(all(target_arch = "wasm32", target_env =
"p2")))'.dependencies]`.

### Verification

```
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i rustybuzz   → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i rustybuzz   → nothing to print
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i rustybuzz   → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i ttf-parser  → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i "skrifa@0.31.3"    → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i "read-fonts@0.29.3" → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i "font-types@0.9.0"  → nothing to print
# native — capability preserved where it can actually run:
$ cargo tree -p semio-s-plugin-puzzle -i rustybuzz
rustybuzz v0.20.1
├── semio-framework-compiler v0.1.0 (…/📚️compiler/📦️packages/🦀️rust)
│   └── semio-framework-os-infinite v0.1.0 (…/♾️infinite/📦️packages/🦀️rust)
│       └── semio-s-plugin-puzzle v0.1.0 (…/🧩️puzzle/📦️packages/🦀️rust)
├── usvg v0.45.1
    └── … (unrelated pre-existing edge, see "Not moved" below)
```

`rustfmt --edition 2021 --check` on both edited compiler files — exit 0, no diff.
`cargo metadata --no-deps` — exit 0, parses clean.

**`cargo check -p semio-framework-compiler`** (native): `Finished dev profile, 0 errors` (2.02s;
`rustybuzz v0.20.1` checked, confirming the native arm still compiles/links it).
**`cargo check -p semio-framework-compiler --target wasm32-wasip2`**: `Finished dev profile, 0
errors` (23.90s); grep of the full log for `Checking rustybuzz`/`Compiling rustybuzz` → **zero
matches**, confirming it is not even built for this target, not merely unlinked.

## A second, independent win found while tracing cluster 1: `base64` in `os-infinite` itself

`icon_codec::decode_data_url_svg`/`decode_raster_icon_bytes` (`🖼️canvas/🦀️component.rs`, both
unconditional, both real callers of `board_resolve_icon_kind`'s guest-reachable path) called the
third-party `base64` crate directly for `data:image/svg+xml;base64,...`/`data:image/{png,jpeg,jpg,
webp,gif};base64,...` decoding. This ticket already built and oracle-verified a first-party RFC 4648
codec (`semio-framework-io-base64`, `🧰️framework/🔨️modules/🚪️io/🔤️base64`, already adopted by seven
other s-plugins per `verified-outcomes.md`'s "First-party replacements … proven" table) — a direct,
zero-risk drop-in (`base64_standard_decode` has the identical `Result<Vec<u8>, _>` shape the call
sites already treated as `.ok()?`).

Swapped both call sites to `base64_codec::base64_standard_decode` (dependency renamed `base64_codec`
in `Cargo.toml`, matching the convention every other adopting crate already uses), removed the
unconditional `base64 = "0.22.1"` dependency entirely from `os-infinite`'s `Cargo.toml` (native *and*
wasip2 both drop it now — this one wasn't a target-gate, it was a full first-party replacement, so
no target split was needed at all). Also deleted one genuinely dead `use base64::Engine;` at the top
of `🌍️world/🦀️component.rs` (grepped: zero uses of `Engine`/`.encode(`/`.decode(` with a base64
receiver anywhere in that file — the only `.decode()` there is `image::ImageReader::decode()`).

```
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i base64  → nothing to print
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i base64  → nothing to print
```

**Not moved: `semio-s-plugin-flow` still shows `base64` (68, not 67).** `cargo tree -i` traces it to
a *completely different, unrelated* edge: `semio-framework-os-flow`'s own `📐️brep-geometry/
🦀️component.rs` (`decode_base64`/`encode_base64`, real BREP STEP/OBJ/STL/GLB import/export, called
from `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep`) declares `base64 = "0.22"` unconditionally and
imports `base64::Engine` directly. `🌊️flow/` is explicitly reserved territory this session — the
ticket brief names "os-kernel's `🌊️flow` subtree" as a live concurrent agent's area, and that file
additionally depends on `semio_s_plugin_stdio` (the other ticket's own 1400+-file in-flight wave) —
so this was left untouched rather than risk colliding with either. Same fix shape (`base64_codec`
swap) would apply cleanly there; flagged as a trivial follow-up once that territory is free.

## Cluster 2 — CPU drawing/raster decode (`vello`, `vello_encoding`, `peniko`, `kurbo`, `color`): investigated, correctly left alone

Traced `vello 0.7.0`'s own **published** `Cargo.toml` (not assumed from its feature list): `peniko`,
`skrifa` (`0.40.0`, a *second*, newer instance than the one `rustybuzz` pulled — independently
resolved, not removed by cluster 1's fix), `png`, `vello_encoding`, `bytemuck`, `log`, `thiserror`,
`static_assertions` are **all non-optional `[dependencies.*]` entries** — present regardless of
`default-features = false` or which Cargo features are enabled. Only `wgpu`/`vello_shaders`/
`futures-intrusive`/`wgpu-profiler` are truly optional. This is why `default-features = false`
(the shape `♾️infinite`'s Cargo.toml already deliberately uses for wasip2, per its own docstring)
still leaves `skrifa`/`read-fonts`/`font-types`/`png`/`flate2`/`miniz_oxide`/`fdeflate`/`crc32fast`/
`adler2`/`simd-adler32`/`color`/`moxcms`/`polycool`/`pxfm`/`guillotiere`/`linebender_resource_handle`
/`svg_fmt`/`arrayvec`/`euclid`/`num-traits`/`smallvec` resolved on `wasm32-wasip2` — confirmed with
`cargo tree -i` for every representative crate in this family (all resolve back through `vello`, not
through `image`/`compiler`).

The ticket brief's own note — "`♾️infinite`'s wasip2 target table deliberately keeps `vello`/
`vello_svg` with `default-features = false` for target-neutral `vello::Scene`/`peniko`/`kurbo`
drawing-command types" — was verified still accurate: `vello::Scene`/`peniko`/`kurbo` types are used
unconditionally throughout `🎲️board` (69 non-test, non-`#[cfg]` references to `Scene` alone in that
subtree, not exhaustively re-audited this session given the explicit instruction to read, not
re-litigate, that comment). Removing `vello` itself would require replacing `vello::Scene`'s own
type — the crate's *drawing-command* representation, not merely its renderer — with a first-party
equivalent throughout `🎲️board`'s unconditional code. That is a first-party rewrite at least as
large as the blake3/DEFLATE/parry3d replacements this ticket already shipped, with no
`preview_media_natural_size`-style "only the declared box matters" escape hatch available (unlike
cluster 1, `Scene` values are genuinely constructed and threaded through unconditional code, not
merely produced-then-measured) — a distinct, larger deliverable, not attempted this session per the
brief's own "if genuinely guest-reachable and needs real first-party work, say so with scope" rule.
**Correctly left alone, not overlooked.**

### The `image`/`png` terrain tail — confirmed still under a live peer's territory, not touched

`🗺️surface/🏔️terrain/🦀️component.rs` (path-mounted into `os-infinite`) still requires `image =
{ version = "0.25", default-features = false, features = ["png"] }` **unconditionally** (top-level
`[dependencies]`, not target-gated) for Terrarium RGB elevation-tile PNG decode. The Cargo.toml's own
comment confirms this is deliberate and shared with `semio-framework-surface`. Per the ticket brief
("a peer reintroduced it … entangled in a shared struct field and under open architectural review …
do not fight a live peer over it") this was checked but not touched — `image`/`png` remain present in
all three plugins' wasip2 graphs for this reason, independent of (and layered under) `vello`'s own
`png` pull.

## Full reachable subtree gated (cluster 1) — not just the entry point

Per the ticket's own stated lesson ("gating a caller doesn't gate its callees' compilation"), every
item that could reference `rustybuzz`/`ttf_parser`/`base64` was traced and gated, not just the
facade fns:

- `📦️glue.rs`: `pub mod text;` / `pub mod math;` / `pub mod svg;` mounts — gated.
- `🦀️component.rs`: `use crate::math::FontContext;` / `use crate::svg::{FontSet, SvgOptions};` /
  `use crate::text::Font;`, `struct Fonts`, `fonts()`, `render_to_svg()` — gated; new
  `estimate_svg()` + all four `compile_*_to_svg` wasip2 arms added.
- Six tests in `🦀️component.rs` that assert real glyph output — gated.
- `os-infinite`'s `Cargo.toml`: no change needed for `compiler` itself (it's first-party, not
  third-party — the goal only requires eliminating *third-party* links; `compiler` staying an
  unconditional first-party dependency is correct and required either way).

Confirmed nothing was missed: `grep -rn "compiler::text::\|compiler::math::\|compiler::svg::"` across
the whole repo, before and after, matches only inside `📚️compiler/` itself.

## Build verification — full, both crates, both targets, foreground

```
$ cargo check -p semio-framework-compiler
   Finished `dev` profile [unoptimized] target(s) in 2.02s        (0 errors)

$ cargo check -p semio-framework-compiler --target wasm32-wasip2
   Finished `dev` profile [unoptimized] target(s) in 23.90s       (0 errors, rustybuzz not compiled)

$ cargo check -p semio-framework-os-infinite
   Finished `dev` profile [unoptimized] target(s) in 12.55s       (0 errors, 63 pre-existing warnings)

$ cargo check -p semio-framework-os-infinite --target wasm32-wasip2
   error[E0433]: cannot find module or crate `typst_assets`
     --> …/🖼️canvas/🔨️bin/dump_guestslim_typst_fonts.rs:22
   error: could not compile `semio-framework-os-infinite` (bin "dump-guestslim-typst-fonts")

$ cargo check -p semio-framework-os-infinite --target wasm32-wasip2 --lib
   Finished `dev` profile [unoptimized] target(s) in 4.22s         (0 errors)
```

The bin-target failure is **pre-existing and untouched by this session**: `dump-guestslim-typst-
fonts` (`required-features = ["render"]`) needs `typst-assets`, which a *prior* pass (documented in
this same Cargo.toml's own comment, "🔤️ `typst-assets` … lives here too, same reasoning") already
put behind the native-only target table specifically because that bin is "a native dev-tool `[[bin]]`
never built as part of the wasip2 component." My session touched neither the `render` feature nor
`typst-assets`' target table nor the bin declaration — `git diff` on the Cargo.toml shows only the
`base64_codec` swap and the compiler-dependency-adjacent comment. `cargo check --lib` (what the
shipped `wasm32-wasip2` component actually builds) is 0 errors, confirming the library itself is
unaffected; the bin target was never wasip2-buildable and isn't part of the plugin's shipped
component regardless.

`os-kernel`/`os-flow`/`os-infinite`'s many pre-existing dead-code/unnecessary-qualification warnings
in both runs are unrelated to this session (none name a symbol or file this pass touched).

## What is proven vs. not proven

**Proven**: puzzle/trinity `wasm32-wasip2` third-party crate count **76 → 67**, flow **76 → 68**
(lock-free `cargo tree`, cannot go stale). `rustybuzz` and its exclusive dependency tail
(`ttf-parser`, `read-fonts@0.29.3`, `skrifa@0.31.3`, `font-types@0.9.0`, four `unicode-*` shaping
crates) are completely absent from all three plugins' wasip2 graphs. `base64` is completely absent
from puzzle's and trinity's wasip2 graphs. `semio-framework-compiler` and `semio-framework-os-
infinite` both compile clean (0 errors) natively and for `wasm32-wasip2` (`--lib` for the latter, the
actual shipped surface). Native/browser capability is unchanged — `rustybuzz`/real shaping/painting
still resolve and still work on those targets, confirmed by `cargo tree -i` with no `--target` and by
the native `cargo check` passing with `rustybuzz` actually compiled. Zero callers outside
`semio-framework-compiler` were touched.

**Not proven**: an end-to-end `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-{puzzle,
flow,trinity}` at 0 errors — not attempted this session, consistent with every other doc in this
ticket's own documented pattern of plugin-level builds being blocked by unrelated concurrent waves
(`🔌️plugin`+`🏪️store`, the last plugin-manifest wave, `🌊️flow`'s own subtree — all explicitly named as
other live agents' territory in this ticket's brief). `cargo tree -i` is lock-free and metadata-only,
unaffected by any of that, and is the primary evidence above.

## Deliberately left alone (do not re-attempt without new information)

- `vello`/`vello_encoding`/`peniko`/`kurbo`/`color`/`skrifa`/`read-fonts`/`font-types` (the second,
  vello-owned instance) and their whole raster-codec tail (`png`/`flate2`/`miniz_oxide`/`fdeflate`/
  `crc32fast`/`adler2`/`simd-adler32`) — genuinely needed for `vello::Scene`/`peniko`/`kurbo`
  drawing-command types used unconditionally in `🎲️board`, per the ticket brief's own explicit
  warning and this session's independent confirmation via `vello`'s published Cargo.toml. Would need
  a first-party `Scene`-equivalent rewrite, out of scope for this pass.
- `image`/`png` (terrain elevation-tile decode in `🗺️surface/🏔️terrain`) — a live peer's
  architectural-review territory, per the brief.
- `🌊️flow`'s own unrelated `base64` edge (`📐️brep-geometry`) — reserved concurrent-agent territory
  this session, plus entangled with `semio-s-plugin-stdio`'s own in-flight wave. Same fix shape as
  this session's `os-infinite` swap would apply once that territory is free.
- The proc-macro/`wit-bindgen`/`wasm-tools` tail (`syn`, `quote`, `wit-component`, `wasm-encoder`,
  `indexmap`, …) still listed by `cargo tree --edges normal` — re-confirmed this session (via `-i
  indexmap`) to resolve exclusively through `wit-bindgen-rust-macro (proc-macro)` → `semio-framework-
  plugin`, i.e. host-only/not actually linked, per `verified-outcomes.md`'s own established
  measurement correction. `🔌️plugin` is separately reserved concurrent-agent territory this session
  regardless.
- `serde`/`serde_json`/`serde_core`/`itoa`/`memchr`/`zmij` — the ticket's own explicitly out-of-scope
  later wave.

## Files touched

- `🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/Cargo.toml` — `rustybuzz`/`base64` moved to
  `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]`.
- `🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/📦️glue.rs` — `text`/`math`/`svg` module
  mounts gated native-only.
- `🧰️framework/🔨️modules/📚️compiler/🦀️component.rs` — `Fonts`/`fonts()`/`render_to_svg` gated
  native-only; all four `compile_*_to_svg` split into native + wasip2 (`estimate_svg`) arms; six
  glyph-content-asserting tests gated native-only.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml` — unconditional
  `base64 = "0.22.1"` replaced with `base64_codec = { …, package = "semio-framework-io-base64" }`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️component.rs` —
  `icon_codec::decode_data_url_svg`/`decode_raster_icon_bytes` swapped from `base64::engine::…` to
  `base64_codec::base64_standard_decode`; dead `use base64::Engine as _;` removed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` — dead
  `use base64::Engine;` removed (confirmed unreferenced by grep).

No file in `✏️s/🔌️plugins/🧩️puzzle`, `✏️s/🔌️plugins/🌊️flow`, `✏️s/🔌️plugins/🔱️trinity`, or
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/` was touched.
