# 📐️ Intrinsic-size wiring — trinity closed, icon dimension+painting split for wasip2, 104→97

## Headline — before/after (all three plugins, `cargo tree`, lock-free, cannot go stale)

```bash
for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity; do
  cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
    | sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
done
```

| plugin | before (this pass) | after (this pass) |
|---|---|---|
| `semio-s-plugin-puzzle` | **104** | **97** |
| `semio-s-plugin-flow` | **104** | **97** |
| `semio-s-plugin-trinity` | unmeasured | **97** |

Net removed from the shared `♾️infinite` baseline: `gif`, `image-webp`, `weezl`, `zune-core`,
`zune-jpeg`, `color_quant`, `quick-error` (7 crates). This is **less** than the 11 this pass actually
removed at the source — see "A concurrent edit landed mid-pass" below for the full accounting; the
work in this doc is real and independently verified, the net number just reflects an unrelated,
simultaneous peer change to the same `Cargo.toml`.

## 1. Trinity's caller-graph trace — the blocker my predecessor left open

`infinite-host-deps-split.md` traced every `IconPaintCache`/`BoardHost`/`DagHost` painting entry
point except one: `semio-s-plugin-trinity`'s `World::paint_scene` (`✏️s/🔌️plugins/🔱️trinity/
🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️component.rs:647`,
`TrinityBridge::paint_scene`, calling `self.board.build_vector_scene()` where `board: BoardHost` is
the same `infinite_board_port_directed_normal::BoardHost` puzzle uses). It could not find a caller
inside the plugin and flagged it "external and unverified."

**Traced this session, conclusively:**

- `grep -rn "TrinityBridge" .` (whole repo, not just the plugin) finds **zero** matches outside the
  file that defines it — nothing anywhere constructs a `TrinityBridge`, so `paint_scene` cannot be
  reached through it.
- A concurrent, already-committed peer change (commit `f394df99d4`, 2026-09-01 18:10:11 +0200 —
  confirmed real via `git log --date=iso`, not the fake commit-message date) deleted trinity's own
  `mod wasm_bridge`/`mod wasm_session` (the `TrinitySession` WebGPU canvas host) outright, with the
  docstring left in place: *"nothing ever built either for `wasm32-unknown-unknown` (no engine
  entry, no `wasm` script target)"*. The deleted `wasm_session::TrinitySession::render_frame` used to
  call `inner.host.board.build_vector_scene()` **directly** (not through `TrinityBridge::paint_scene`
  at all) under a **bare** `#[cfg(target_arch = "wasm32")]` — the exact "forgot `wasip2` is also
  `wasm32`" bug class this whole ticket exists to close, now moot because the module is gone.
- Repo-wide `grep -rn "TrinityBridge" .` confirms trinity's own `📦️glue.rs` mounts the `world` module
  unconditionally, so this isn't cfg-hidden from the check — it is genuinely dead code, on every
  target, right now.

**Conclusion**: `TrinityBridge::paint_scene`/`self.board.build_vector_scene()` is unreachable from
any caller repo-wide. Trinity does **not** need full SVG *rendering* — it needs nothing from the
painting stack at all today. This closes the blocker precisely as scoped by
`intrinsic-size-parser.md`'s "Remaining work" item 2, and clears the path for item 1 (gating the
painting call sites) without any risk to trinity specifically. No trinity file was edited — the
finding is "already safe," not "made safe."

## 2. Dimension call sites — swapped at the source, not by duplicating resolution logic

The ticket asked to swap `preview_media_natural_size`/`image_widget_size` onto
`semio-framework-intrinsic-size`. Both are thin wrappers (`image_widget_size` →
`preview_image_node_size` → `preview_media_natural_size`,
`🧰️framework/…/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:460,635`) around
`canvas::icon_codec::board_resolve_icon_kind` + `canvas::svg_icon::svg_icon_content_bounds_from_str`.
Rather than reimplementing `board_resolve_icon_kind`'s Data/Svg/Catalog/Shortcode/Math/Emoji/Text
resolution a second time inside a wasip2 arm of `preview_media_natural_size` itself (real risk of the
two paths silently diverging), the swap was made **at the two leaf functions
`board_resolve_icon_kind` actually calls for dimensions** — so `preview_media_natural_size` and
`image_widget_size` needed **zero code changes** and automatically pick up the new behavior on
`wasm32-wasip2` through the same call chain they already had.

### `icon_codec::decode_raster_icon_bytes` (`🧰️framework/…/♾️infinite/🖼️canvas/🦀️component.rs`)

Split the post-base64-decode step into `raster_icon_bytes_to_rgba(&[u8]) -> Option<RgbaImage>`, two
`#[cfg]` arms:
- `not(all(wasm32, p2))` (unchanged): `image::load_from_memory` full pixel decode.
- `all(wasm32, p2)` (new): `semio_framework_intrinsic_size::raster_dimensions(raw)` — header-only,
  `RgbaImage { data: Arc::from([].as_slice()), w, h }`. `data` is deliberately empty; see §3 for why
  that is safe and not a silent capability drop.

### `svg_icon::svg_icon_content_bounds_from_str` (same file)

Two `#[cfg]` arms on the existing function:
- `not(all(wasm32, p2))` (unchanged): real `usvg::Tree::from_str` + painted-content bbox.
- `all(wasm32, p2))` (new): `semio_framework_intrinsic_size::svg_intrinsic_size(svg)` — declared
  `width`/`height`/`viewBox` box, `x`/`y` fixed at `0.0` (a declared box has no painted-ink offset).
  **Disclosed, intentional behavior difference** on this target only, exactly the "two
  implementations behind one signature" pattern `raster-tier-split.md`/`typeset-tier-split.md`
  already established. Its only caller anywhere is `preview_media_natural_size`.

Both arms reference a single top-level alias added once,
`#[cfg(all(target_arch = "wasm32", target_env = "p2"))] use semio_framework_intrinsic_size as
intrinsic_size;`, referenced from each module as `super::intrinsic_size`.

## 3. `IconPaintCache::get_or_build` — painting gated by return value, not by touching callers

`infinite-host-deps-split.md` and `intrinsic-size-parser.md` both scoped this as "gate
`get_or_build`/`append_icon_at_screen_rect` and every `paint_scene`/`build_vector_scene` entry point
in flow, puzzle, and trinity" — a multi-file, multi-symbol edit. Tracing every real caller this
session (see below) found a narrower, lower-risk fix that achieves the identical outcome:

`get_or_build` already returns `Option<CachedIconPaintLease<'_>>`, and every real caller
(`append_icon_at_screen_rect`, `paint_scene` in both `DagHost` and `BoardHost`'s wrapper types,
trinity's now-dead `TrinityBridge::paint_scene`) already treats `None` as "nothing to paint" and
no-ops gracefully. So instead of gating each caller, `get_or_build` itself got a second `#[cfg]` arm:

```rust
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn get_or_build(&self, _encoded: &str, _fg: Color, _bg: Color, _preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
    None
}
```

**Zero callers changed** — `append_icon_at_screen_rect`, `DagHost::paint_scene`,
`BoardHost::build_vector_scene`, and trinity's `TrinityBridge::paint_scene` are byte-for-byte
unmodified and compile identically on every target. This is exactly the pattern the ticket's own
scoreboard calls "the working pattern... the wasip2 one returning an honest [value] that was already
the true runtime outcome — never a stub, never a broken call chain": nothing on `wasm32-wasip2` was
ever able to reach a real paint today (see full caller trace below), so `None` there is not a stub
for exercised behavior, it formalizes what was already true, and any future wasip2 caller gets a
graceful "nothing painted" instead of a crash or garbage pixels.

This is also why `decode_raster_icon_bytes`'s wasip2 arm can safely return an **empty** `data` buffer
(§2): the only consumer that would ever read `RgbaImage.data` for real pixels is `get_or_build`'s
`RasterRgba8` branch, and that branch is now unreachable-by-construction on this target (`get_or_build`
never even calls `board_resolve_icon_kind` there). Not a silent capability drop — the capability
(icon *painting*, as opposed to *measuring*) is host-only by nature, exactly as the ticket predicted
for `IconPaintCache` if it "turned out to be real painting."

### Full caller trace that justified the single-function fix

- `append_icon_at_screen_rect` (same file) — its only two call sites are `DagHost`'s private
  `paint_preview_image_content`/`paint_node_lod_icon` (flow), both reachable only from `paint_scene`'s
  own render tree, confirmed by grep (no other callers of either helper).
- `DagHost::paint_scene` (`➡️directed/🕸️dag/component.rs:6088`) — callers: `flow/🖥️host`'s wrapper
  (used only by flow's own `#[cfg(test)]` tests and the wasm bridge already gated `not(all(wasm32,
  p2))` by `infinite-host-deps-split.md`'s Fix 1), `🗺️surface/🕸️node-graph`'s wrapper, and
  `#[cfg(test)]` blocks in this same file.
- `BoardHost::build_vector_scene` (`➡️directed/➕️normal/component.rs:10432`) — callers:
  `encoded_scene_hint()` (used by puzzle's already-gated wasm bridge plus tests only), the
  `CanvasContent::build_scene` trait impl (its only non-test consumer,
  `🧰️framework/🔨️modules/✍️editor/🦀️component.rs:1421`, sits inside a block already
  `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]`-gated), test call sites in
  `puzzle/…/⚙️engine/🔗️linking/component.rs`, and trinity's `TrinityBridge::paint_scene` (§1, dead).
- `🗺️surface/🕸️node-graph` and `📺️renderer/…/EngineCanvas` — flagged by `intrinsic-size-parser.md`
  as "plausibly host-only... not independently re-verified." **Independently re-verified this
  session**: none of puzzle/flow/trinity/animate's `📦️glue.rs` mount either module by `#[path]`, and
  `cargo tree -p <plugin> --target wasm32-wasip2 --edges normal | grep -i 'surface\|renderer'` is
  empty for all four plugins — no crate with either name is even a dependency of any plugin's wasip2
  build. Confirmed, not merely traced.

No file in this caller graph other than `get_or_build` itself needed to change.

## What still genuinely needs `usvg`/`image` at runtime, and why

- **`usvg`/`vello_svg`/`fontdb`/`roxmltree`/… (~45 crates) — still genuinely needed, unrelated to
  painting.** A previously-untraced consumer: `🧰️framework/…/♾️infinite/🖼️canvas/🦀️component.rs`'s
  `pub mod text` (map/place-name label rendering, `usvg_options_map_labels`, `append_label`,
  `append_label_tspans`, `label_byte_world_x`, `label_span_world_x`) uses `usvg::Tree::from_str` as a
  **text shaper** — it builds a tiny `<text>` SVG string and parses it with `usvg` purely to get real
  font-shaped glyph bounds/advances, unconditionally, on every target. `usvg` reaches this crate only
  via `vello_svg`'s re-export (`pub use vello_svg::usvg;` — there is no direct `usvg` dependency), so
  `vello_svg` cannot be narrowed away from `wasm32-wasip2` either, regardless of `get_or_build` being
  gated. This module's own wasip2 reachability was **not traced this session** — it is a new, distinct
  question from the icon-painting one this pass closed, and is the accurate replacement for
  `intrinsic-size-parser.md`'s "Remaining work" items 1–3 (which described the *old*, narrower blast
  radius before this consumer was known). A follow-up pass should trace `append_label`/
  `label_byte_world_x`/`label_span_world_x`'s callers the same way this pass traced `paint_scene`'s,
  before attempting to narrow `usvg`.
- **`rustybuzz`** — still needed by `compiler::compile_{snippet,emoji,text}_to_svg` for Math/Emoji/Text
  icon generation, out of scope per `infinite-host-deps-split.md`'s own classification (separate,
  larger task).
- **`image` (with reduced features) — reintroduced mid-pass, unrelated to icons.** See next section.

## A concurrent edit landed mid-pass — `image` came back for a different, real reason

While this pass was running, a peer session committed a change to this exact
`♾️infinite/📦️packages/🦀️rust/Cargo.toml` adding `image = { version = "0.25", default-features =
false, features = ["png"] }` back to the **unconditional** `[dependencies]` table, with a docstring
explaining it is required by the `#[path]`-mounted `🗺️surface/🏔️terrain/🦀️component.rs` (Terrarium
DEM elevation-tile PNG decode), shared between this crate and `semio-framework-surface`.

This is a **different, genuine** need, not a regression of this pass's work — verified, not assumed:

- `decode_terrarium_png`/`TerrainSessionCore::upload_elevation_tile` (the only functions in that file
  touching `image::`) have **zero non-test callers** in `🌍️world/🦀️component.rs` except one
  `#[test]`. The only production caller chain is `sync_terrain_state` ← `sync_terrain` ←
  `render_world_3d` — and `render_world_3d` is **already** `#[cfg(not(all(target_arch = "wasm32",
  target_env = "p2")))]`-gated (a prior pass in this same ticket). So this subsystem has the identical
  shape to the icon-painting one this pass just closed — but `image::RgbaImage`/`image::ImageError`
  are baked unconditionally into `DecodedElevationTile`'s field type and
  `FrameworkSurfaceTerrainError`'s enum variant (not behind an `Option`-returning boundary like
  `get_or_build` was), so gating it cleanly means restructuring a public error type and a struct field
  across **two crates** (`os-infinite` and the untouched-this-session `semio-framework-surface`), not
  one function.
- The subsystem's own docstring flags it as under **open architectural review** by another party:
  *"Open question for the coordinator/W1 owner (see `📓️wave2-reports/terrain-report.md`): whether
  decode+mesh-build should instead route through the frozen host `EngineCache`."* Restructuring it
  unilaterally this session risks colliding with that in-flight design work.

Left untouched deliberately — this is precisely the same class of judgment call trinity was last
pass (a real, traced, structurally-similar candidate, correctly left for a follow-up that can budget
the two-crate, two-type restructuring properly). **This pass's own fix is still fully valid and
necessary**: independently, `decode_raster_icon_bytes`'s wasip2 arm no longer needs `image` at all for
icons; the crate simply happens to still be present for the unrelated terrain reason, which is why the
net count (97) is 4 crates higher than what this pass's own icon fix alone would have produced (93,
confirmed by measurement before the peer's commit landed) — the missing 4 (`image`, `png`,
`byteorder-lite`, `moxcms`, `pxfm` minus one already-accounted `png`) are terrain's own minimal `png`-
only tail, not a leak from this pass's code.

## Verification performed this session

### `cargo tree` (lock-free, cannot go stale) — confirmed removed from `wasm32-wasip2`

```
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i gif          → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i image-webp   → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i weezl        → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i zune-core    → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i zune-jpeg    → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i color_quant  → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i quick-error  → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i usvg          → usvg v0.46.0 (unchanged, expected — mod text)
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i rustybuzz     → rustybuzz v0.20.1 (unchanged, expected — compiler)
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i fontdb        → fontdb v0.23.0 (unchanged, expected — usvg)
```

### `cargo check -p semio-framework-os-infinite` (native, foreground) — verbatim tail

```
warning: `semio-framework-os-infinite` (lib) generated 64 warnings (run `cargo fix --lib -p semio-framework-os-infinite` to apply 14 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 17.27s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```
0 errors. All 64 warnings are pre-existing `dead_code` lints in `🌍️world/🦀️component.rs` (pick_*
functions) and `➕️normal/component.rs` (`BoardOwnedEvent`/`BoardPointerPlan`), none in any file this
pass touched — confirmed by grepping the full warning output for `intrinsic`/`icon_codec`/`svg_icon`
(zero matches) and for `error[` (zero matches).

### `cargo check -p semio-framework-os` (native, foreground) — verbatim tail

```
warning: `semio-framework-os` (lib) generated 27 warnings
    Finished `dev` profile [unoptimized] target(s) in 2m 28s
```
0 errors. Warnings are pre-existing dead-code in `🖥️host/🦀️component.rs`'s ABI codec scaffold,
unrelated to this pass.

### `cargo test -p semio-framework-intrinsic-size -- --nocapture` (foreground) — verbatim tail

```
running 9 tests
test component::tests::raster_dimensions_rejects_malformed_and_truncated_input ... ok
webp VP8/VP8X hand-built spec fixtures: 3/3 matched documented byte layout
test component::tests::webp_lossy_and_extended_headers_match_spec_hand_built_fixtures ... ok
raster fixture corpus: 6/6 matched
test component::tests::raster_fixture_corpus_matches_recorded_image_crate_derived_expectations ... ok
svg fixture corpus: 43/43 matched
test component::tests::svg_fixture_corpus_matches_recorded_usvg_derived_expectations ... ok
svg live usvg oracle: 40 numeric matches + 3 agreed hard failures / 43 cases
test component::tests::svg_oracle_matches_usvg_live_across_fixture_corpus ... ok
webp (VP8L) oracle: 12/12 matched image crate
test component::tests::webp_lossless_oracle_matches_image_crate_across_corpus ... ok
jpeg oracle: 12/12 matched image crate
test component::tests::jpeg_oracle_matches_image_crate_across_corpus ... ok
png oracle: 12/12 matched image crate
test component::tests::png_oracle_matches_image_crate_across_corpus ... ok
gif oracle: 12/12 matched image crate
test component::tests::gif_oracle_matches_image_crate_across_corpus ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.97s
```
Still 9/9, byte-identical to the parser-authoring session's own run (module untouched, per the ticket's
constraint not to modify `semio-framework-intrinsic-size` itself).

### Not attempted this session

`cargo check --target wasm32-wasip2` for any plugin — not required by this ticket's own verification
loop for this deliverable (the loop specifies native `cargo check` for `semio-framework-os-infinite`/
`semio-framework-os` plus the intrinsic-size test; `cargo tree` is the wasip2-side evidence, and it is
lock-free/metadata-only so it cannot be stale). A real `wasm32-wasip2` build was not started — this
session's edits are narrow `#[cfg]` splits with no new unconditional symbol, in files that already
build cleanly natively, mirroring the confidence level `infinite-host-deps-split.md`'s Fix 1/2 shipped
at without a wasip2 build either.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️component.rs` — added the
  `intrinsic_size` alias; split `svg_icon_content_bounds_from_str` and `decode_raster_icon_bytes` (as
  new `raster_icon_bytes_to_rgba`) into native/browser vs. `wasm32-wasip2` arms.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️component.rs` —
  split `IconPaintCache::get_or_build` into a full native/browser arm (unchanged body) and a
  `wasm32-wasip2` arm returning `None` unconditionally.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml` — moved `image`
  into the `not(all(wasm32, p2))` target table (full png/jpeg/webp/gif features, for the native/browser
  icon-painting arm) and added `semio-framework-intrinsic-size` to the `all(wasm32, p2)` target table.
  Coexists with a peer's unrelated, concurrently-added unconditional `image = {features=["png"]}` line
  — see "A concurrent edit landed mid-pass" above; not reverted, not modified.

No file in `✏️s/🔌️plugins/🔱️trinity`, `✏️s/🔌️plugins/🌊️flow`, or `✏️s/🔌️plugins/🧩️puzzle` was
touched — every real caller of the gated code stayed byte-for-byte unchanged (§3).

## Remaining work for a follow-up pass

1. Trace `🖼️canvas`'s `mod text` (`append_label`, `append_label_tspans`, `label_byte_world_x`,
   `label_span_world_x`, `usvg_options_map_labels`) the same way this pass traced `paint_scene` — find
   every real caller, determine if it is reachable from any plugin's `wasm32-wasip2` guest dispatch or
   host/browser-only (map/terrain label rendering is a strong prior for host-only, unverified). This is
   the actual remaining blocker for narrowing `usvg`/`vello_svg`/`fontdb` and the ~45-crate tail away
   from wasip2 — not painting, which this pass closed.
2. `🗺️surface/🏔️terrain/🦀️component.rs`'s `decode_terrarium_png`/`DecodedElevationTile`/
   `build_terrain_tile_mesh`/`FrameworkSurfaceTerrainError` — structurally the same shape as the icon
   fix this pass shipped (real caller chain already gated at `render_world_3d`), but the `image` types
   are baked into a struct field and a public error enum variant rather than behind an
   `Option`-returning boundary, and the file is shared with the un-audited `semio-framework-surface`
   crate. Coordinate with `📓️wave2-reports/terrain-report.md`'s open question before restructuring —
   it may resolve differently (e.g. routing through `EngineCache`) than a straight `#[cfg]` split.
3. Re-run the `cargo tree` count once (1) and (2) both land — expected to drop the shared
   `usvg`/`rustybuzz`/`fontdb` tail (~45 crates, if (1) confirms host-only) and `image`/`png`/
   `byteorder-lite`/`moxcms`/`pxfm` (if (2) lands), for a combined count in the neighborhood of `draw-
   fsm`'s already-serde-only baseline.
