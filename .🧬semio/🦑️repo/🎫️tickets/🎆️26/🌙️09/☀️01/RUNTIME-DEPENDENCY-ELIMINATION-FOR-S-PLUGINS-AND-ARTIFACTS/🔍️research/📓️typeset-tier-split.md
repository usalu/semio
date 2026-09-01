# 🎯️ typeset's typst-tier split — animate's typst trap closed, a second unrelated edge found

## Headline

Animate's third-party crate count on `wasm32-wasip2`: **262 → 88** (measured with the ticket's exact
command, `cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 --edges normal --prefix none
| sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l`).

The typst trap named in the brief is fully closed:

```
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i typst
warning: nothing to print.
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i typst-svg
warning: nothing to print.
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i typst-assets
warning: nothing to print.
```

**`-i rustybuzz` does NOT report clean** — the brief's own verification checklist asked for it, and
the honest result is it's still there, but for a completely different, pre-existing reason unrelated
to typst (see "A second, unrelated edge" below). This is reported plainly rather than glossed over.

## Root cause and per-symbol classification (`🧰️framework/🔨️modules/🔤️typeset/🦀️.rs`)

Unlike `♾️infinite`'s dead `typst`/`typst-svg` (deleted outright by `typst-wasip2-split.md`),
`semio-framework-typeset` is genuinely used — its only consumer, `✏️s/🔌️plugins/🎞️animate`'s
`⚙️engine/🔤️text` module, calls `MarkupTypesetter::render_svg`, `svg_natural_size`, and
`svg_outline_paths` to render `Text`/`MathText`/`Integer`/`Paragraph`/`Code`/`DecimalNumber`
Sobjects and `Axes::with_tick_labels` (in `⚙️engine/📐️geometry`), `📷️camera`'s table/matrix cell
labels, and `🎛️config`'s config labels — real, load-bearing typesetting, not dead weight.

| Symbol | Classification | Evidence |
|---|---|---|
| `MarkupTypesetter` (trait) | target-neutral | `fn render_svg(&self, markup: &str) -> Option<String>` names no `typst::*`/`usvg::*` type. |
| `TypstTypesetter` (struct) | target-neutral | Zero-sized marker, no fields, no `typst::*` type. Kept as a single unconditional definition — only the `impl MarkupTypesetter for TypstTypesetter` block differs per target. |
| `default_typesetter()` | target-neutral | Returns `TypstTypesetter {}`, no typst/usvg type in signature or body. |
| `impl MarkupTypesetter for TypstTypesetter` (native) | **genuine engine** | Calls `typst_markup_to_svg`, which drives the real Typst `World`/`compile::<PagedDocument>` pipeline. |
| `typst_asset_font_list`, `TYPST_FONTS`, `typst_compile_markup_to_svg`, `typst_markup_to_svg` | **genuine engine** | Every one directly names `typst::*`/`typst_assets::*`/`typst_svg::*` types (`Font`, `FontBook`, `Library`, `World`, `PagedDocument`, `LazyHash`, `FileId`, `Source`, `VirtualPath`, `Bytes`, `Datetime`). |
| `svg_natural_size`, `svg_outline_paths` (native) | **genuine engine** | Both call `usvg::Tree::from_str`/`usvg::Options` directly. |
| `map_svg_point`, `collect_svg_paths` | **genuine engine** | Private helpers used only by `svg_outline_paths`'s native body; `collect_svg_paths` matches on `usvg::Node`/`usvg::tiny_skia_path::PathSegment` directly. |

Action taken, mirroring `raster-tier-split.md`'s exact shape: the trait, the zero-sized struct, and
`default_typesetter()` stay unconditional. Every fn body naming `typst`/`typst-svg`/`typst-assets`/
`usvg` — the `impl MarkupTypesetter for TypstTypesetter`, the four typst-compile helpers,
`svg_natural_size`, `svg_outline_paths`, `map_svg_point`, `collect_svg_paths` — gained
`#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`. Each of `svg_natural_size`/
`svg_outline_paths`/the `impl` block also gained a `#[cfg(all(target_arch = "wasm32", target_env =
"p2"))]` twin with the identical public signature that unconditionally returns `None`.

## Is typesetting guest-reachable? Yes — traced, same chain the raster split already proved

`Text::new`/`MathText::new`/etc. (in `⚙️engine/🔤️text`) and the `Axes`/`📷️camera` table-label/
`🎛️config` label builders that call them are Sobject-library builders invoked exclusively from a
user-authored `Scene::construct` implementation. Traced their only call path, one hop at a time,
confirmed by `grep`:

1. `Scene::construct` runs inside `run_construct`/`preview_scene_loop`
   (`⚙️engine/🎬️scene/🦀️component.rs`).
2. The only caller of `preview_scene_loop` repo-wide (confirmed:
   `grep -rn "run_construct\|compile_and_play\|preview_scene_loop" ✏️s/🔌️plugins/🎞️animate`) is
   `⚙️engine/🎥️video/🦀️component.rs`'s `render::render_scene` — the exact file
   `raster-tier-split.md` already gated for `VelloRenderer`.
3. From there the chain is identical to the one `raster-tier-split.md` already traced and verified
   hop-by-hop: `render_scene` → `compile_scene_to_assets` (`⚙️engine/🦀️component.rs`) →
   `export_video_from_scene` → `export_video_from_deck::handle_async` →
   `Editor::handle`'s single async match arm, `PresentCommand::ExportVideoFromDeck` — confirmed by
   re-reading `Editor::handle`'s full `match command` block
   (`✏️editor/🦀️component.rs:595`): it has exactly one async arm, `ExportVideoFromDeck`; every other
   `PresentCommand` variant (`SetActiveExample`, `EngagementInput`, `SetLocale`, `NoOperation`,
   `AddTile`, `SeedGrid`, ...) is dispatched through the separate sync table earlier in the same
   file and never touches `⚙️engine`/scene construction at all.

**Conclusion: typesetting is guest-reachable through the identical `export-video-from-deck` →
`Editor::handle` chain `raster-tier-split.md` already proved reachable.** A bare `cfg`-gate deleting
`render_svg`/`svg_outline_paths` outright would have broken that chain's compilation (the `Text`
Sobject constructors call them unconditionally today), so the raster split's "two implementations,
identical public API" pattern was reused rather than a delete.

## Why `None` (not an `Err`) is the correct, honest wasip2 outcome — not a new invented behavior

Unlike `wgpu`/`vello`, there is no WASI capability gap forcing this split — `typst`/`usvg` are pure
Rust and nothing prevents them compiling for `wasm32-wasip2` in principle. This is a **deliberate
CLAUDE.md "no third-party runtime dependency in the shipped component" elimination**, not a
technical impossibility, and the module docstring says so explicitly rather than implying a WASI
limitation that isn't real.

`MarkupTypesetter::render_svg`'s doc contract already was "`None` on a compile failure or an empty
document" — the wasip2 stub returning `None` unconditionally is squarely inside that pre-existing
contract, not a new behavior. Downstream, `svg_to_vobject` (`⚙️engine/🔤️text/🦀️component.rs:365`)
already has an empty-svg branch (`fallback_text_rect()`) for exactly this case. And critically, the
video-export command that is the only guest-reachable path to this code **already fails for an
unrelated, already-established reason on wasip2** — `semio-framework-raster`'s `VelloRenderer` (per
`raster-tier-split.md`) returns `Err(RasterError::Adapter(...))` because wasip2 has no GPU device,
and that failure happens in the same `render_scene` call as scene construction, before the
renderer ever runs. So this fallback never changes any export's real-world outcome; it just keeps
`Scene::construct` (which runs before the renderer is reached) compiling and running without a
panic if it were ever invoked ahead of the GPU check.

## Cargo.toml narrowing

`🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust/Cargo.toml`:

```toml
[dependencies]
semio-framework-geometry = { path = "../../../📐️geometry/📦️packages/🦀️rust", package = "semio-framework-geometry" }

[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]
typst = "0.14.2"
typst-svg = "0.14.2"
typst-assets = { version = "0.14.2", features = ["fonts"] }
usvg = "0.45.1"
```

No change was needed in `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` — it still depends on
`semio-framework-typeset` unconditionally (correct: the crate itself is a first-party dependency on
every target, it just internally narrows what it links, exactly like `semio-framework-raster`).

## Before / after `cargo tree -i` evidence

```
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i typst           → nothing to print
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i typst-svg       → nothing to print
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i typst-assets    → nothing to print

# capability preserved on native, where it can actually run:
$ cargo check -p semio-framework-typeset          → Finished, 0 errors (native, real typst/usvg path)
```

Before/after crate count, ticket's exact command:

```
262  →  88
```

## A second, unrelated edge found — `rustybuzz` is NOT gone, and here is exactly why

The brief's verification checklist asked to confirm `-i rustybuzz` is also clean. It is not:

```
$ cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i rustybuzz
rustybuzz v0.20.1
└── usvg v0.45.1
    ├── resvg v0.45.1
    │   └── semio-framework-os v0.1.0 (…/💻️os/🖥️host/📦️packages/🦀️rust)
    │       └── semio-s-plugin-animate v0.1.0 (…/🎞️animate/📦️packages/🦀️rust)
    └── semio-framework-os v0.1.0 (…/💻️os/🖥️host/📦️packages/🦀️rust) (*)
```

This `usvg@0.45.1`/`rustybuzz` is a **completely different edge** from the one this pass closed —
same crate name and version as typeset's own `usvg`, coincidentally, but reached through
`semio-framework-os` (the `🖥️host` native-host framework crate, package `semio-framework-os`),
which `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` depends on **unconditionally** (no
target table) for three real functions used in `🚪️io/🦀️component.rs`: `title_card_svg` (deck
thumbnail export), `dwg_drawing_to_svg` and `rasterize_svg_to_png_base64` (DWG import). This is the
source of the remaining ~88-crate tail: `resvg`/`usvg`/`rustybuzz`/`ttf-parser`/`fontdb`/`png`/
`gif`/`image-webp`/etc. (the full remainder list is in the "Not touched" section below), plus a
`wit-bindgen`/`wasm-encoder`/`wasm-metadata`/`wasmparser`/`wit-component`/`wit-parser` family from
elsewhere in that same dependency subtree.

**This is not this pass's trap, and not a new problem this pass introduced** — confirmed the same
`semio-framework-os = { path = "…/🖥️host/📦️packages/🦀️rust" }` unconditional dependency already
exists in `🧩️puzzle`'s own `Cargo.toml`
(`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:102`), and puzzle's 104-crate count —
already reported "clean" by `raster-tier-split.md` for the `wasm-bindgen`/`js-sys`/`web-sys`/
`vello`/`wgpu` edges specifically, not for a zero-crate total — already carries this same
`resvg`/`usvg`/`rustybuzz` family. The brief's "-i rustybuzz" check appears to have assumed
rustybuzz's only edge was through typst; that assumption doesn't hold for animate because this
second, pre-existing, host-crate edge was already there before today, shared with puzzle, and out of
this pass's scope (`semio-framework-typeset`, named explicitly in the brief).

Splitting `semio-framework-os` (a large native-host crate: title-card/DWG/PNG rasterization plus
whatever else it mounts) the same way raster/typeset were split is a real, tractable next step —
same "target-neutral API, native-only engine" shape is very likely to apply again — but it is a
separate crate, a separate trace (need to confirm guest-reachability of `title_card_svg`/
`dwg_drawing_to_svg`/`rasterize_svg_to_png_base64` through `io()`'s `MediaCodec` region), and a
larger dependency family than typeset's four crates. Flagged as a follow-up rather than attempted in
this pass, which was scoped to the typst trap specifically named in the brief.

## Build results

- **`cargo check -p semio-framework-typeset`** (native) — `Finished` in **8m 31s, 0 errors**.
  Confirms the real Typst/usvg path still compiles clean natively, unchanged.
- **`cargo check -p semio-s-plugin-animate`** (native) — **BLOCKED**, exit with 793 errors, **all**
  in `semio-s-plugin-stdio` (`E0277`/`E0308`/`E0599` against `serde::Serialize`/`Deserialize` in the
  `bmp`/`svg`/`xml`/`gltf`/... mutation-diff files) — confirmed by `grep -oE '\-\-> [^:]+'` over the
  full log: zero mentions of `typst`, `typeset`, or `🔤️text`. This is the same pre-existing,
  actively in-progress, uncommitted `stdio` wave `typst-wasip2-split.md` and `raster-tier-split.md`
  both already hit (793/2218/2989-error range across the three docs, consistent with an
  in-progress, growing wave — not a regression from this pass).
- **`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-animate`** — **BLOCKED**, 71 errors,
  **all** in `semio-framework-actor`/`semio-framework-replication`/`semio-framework-ui-contract`/
  `semio-framework-ui-runtime`/`semio-framework-job` (`E0277`: `ActorId`/`SchemaId`/`ArtifactId`/
  `MutationId` : `serde::Serialize`/`Deserialize` not satisfied) — confirmed by `grep -oE '\-\-> [^:]+'`
  over the full captured log: zero mentions of `typst`, `typeset`, `🔤️text`, or `🖌️raster`.
  `Compiling semio-framework-typeset v0.1.0` and `Compiling semio-framework-raster v0.1.0` both
  appear in the log with **no associated errors** before the build fails downstream at
  `replication`. Matches the ticket brief's own warning: "Other agents are live in `framework-async`,
  `replication` and `stdio`" — this is that concurrent wave (a broader serde-fanout in progress
  across `actor`/`replication`/`ui-contract`/`ui-runtime`/`job`, not just `framework-async`'s
  previously-reported 8 errors), not something this pass touched or can resolve.
- **`cargo tree -i`** (the metadata-only, lock-free, cannot-go-stale check, unaffected by either
  breakage above since it never invokes rustc) is the primary evidence: `typst`/`typst-svg`/
  `typst-assets` all confirmed absent from animate's `wasm32-wasip2` tree.

## Files touched

- `🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust/Cargo.toml` — `typst`/`typst-svg`/
  `typst-assets`/`usvg` moved from unconditional `[dependencies]` to a
  `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` table.
- `🧰️framework/🔨️modules/🔤️typeset/🦀️.rs` — top docstring rewritten to document the tier split;
  `MarkupTypesetter`, `TypstTypesetter`, `default_typesetter()` stay unconditional;
  `impl MarkupTypesetter for TypstTypesetter` split into a native-only real implementation and a
  wasip2-only `None`-returning implementation; `svg_natural_size`/`svg_outline_paths` each split the
  same way; `typst_asset_font_list`/`TYPST_FONTS`/`typst_compile_markup_to_svg`/
  `typst_markup_to_svg`/`map_svg_point`/`collect_svg_paths` and their `typst`/`usvg`-naming imports
  gated native-only; tests that require the real engine (`typst_plain_text_compiles_to_svg`,
  `svg_outline_paths_extracts_at_least_one_path`, `svg_natural_size_matches_fixture_dimensions`,
  `svg_outline_paths_flips_y_and_scales_exactly`, and the `FIXTURE_SQUARE_SVG` const they share)
  gated native-only; `typst_empty_markup_is_none_or_svg` and `svg_outline_paths_none_on_garbage_input`
  left unconditional (their assertions tolerate `None` unconditionally, so they pass under either
  implementation).

## Deliberately left alone

- The `semio-framework-os` (`🖥️host`) → `resvg`/`usvg@0.45.1`/`rustybuzz`/`ttf-parser`/`fontdb`/
  `png`/`gif`/... family — a completely different, pre-existing edge, already present in `🧩️puzzle`
  too, out of scope for this pass (see "A second, unrelated edge" above). This is the honest reason
  animate's count is 88, not near-zero.
- `preview::preview_scene_window_winit` (behind `feature = "preview-window"`, not default,
  pulls `winit`) — untouched, same as `raster-tier-split.md` left it.
- The `🗄️stdio` (793-error) and the broader serde-fanout (`actor`/`replication`/`ui-contract`/
  `ui-runtime`/`job`, 71-error) concurrent waves — explicitly not this pass's to fix.

## What is proven vs. not proven

**Proven**: `typst`/`typst-svg`/`typst-assets` are completely absent from
`semio-s-plugin-animate`'s `wasm32-wasip2` dependency graph (`cargo tree -i` evidence above).
`semio-framework-typeset` compiles clean natively (0 errors) with the real Typst/usvg path intact.
Typesetting's guest-reachability was traced hop-by-hop through the identical
`export-video-from-deck` → `Editor::handle` chain `raster-tier-split.md` already proved, and the
`None`-on-wasip2 fallback was shown to be within `MarkupTypesetter`'s pre-existing documented
contract, not a new invented behavior, and to never change a real export's outcome (which already
fails via `RasterError::Adapter` first). Crate count: 262 → 88.

**Not proven**: an end-to-end `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-animate`
or native `cargo check -p semio-s-plugin-animate` completing at 0 errors — both are blocked by two
different, unrelated, in-progress, uncommitted concurrent waves (`🗄️stdio` natively, a broader
serde-fanout wave across `actor`/`replication`/`ui-contract`/`ui-runtime`/`job` on wasip2), neither
introduced or touched by this pass, confirmed by grepping every error's `-->` path in both full logs.
`semio-framework-os`'s `resvg`/`usvg`/`rustybuzz` family remains linked into animate's (and puzzle's)
`wasm32-wasip2` component — a real, separate, out-of-scope-for-this-pass tier-split candidate for a
follow-up wave.
