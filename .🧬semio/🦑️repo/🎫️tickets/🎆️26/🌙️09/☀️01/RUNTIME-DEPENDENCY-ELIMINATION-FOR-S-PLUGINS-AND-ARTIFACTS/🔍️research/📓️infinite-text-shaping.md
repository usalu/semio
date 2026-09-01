# 🏷️ `mod text` traced and closed — 97 → 76 for puzzle/flow/trinity; verdict (b), host-gated

## Headline — before/after (`cargo tree`, lock-free, cannot go stale)

```bash
for p in semio-s-plugin-puzzle semio-s-plugin-flow semio-s-plugin-trinity; do
  cargo tree -p $p --target wasm32-wasip2 --edges normal --prefix none \
    | sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
done
```

| plugin | before | after |
|---|---|---|
| `semio-s-plugin-puzzle` | **97** | **76** |
| `semio-s-plugin-flow` | **97** | **76** |
| `semio-s-plugin-trinity` | **97** | **76** |

`usvg`, `fontdb`, `vello_svg`, `svgtypes`, `tiny-skia-path` (and `roxmltree`, both resolved
versions) are gone from all three plugins' `wasm32-wasip2` graphs, confirmed with full (untruncated)
`-i` output per version where a bare name was ambiguous — see "Verification" below. `rustybuzz`
correctly stays (pulled by `compiler` for Math/Emoji/Text icon generation, explicitly out of scope
per `infinite-host-deps-split.md`).

## 1. Call-chain trace — what `mod text` actually asks `usvg` for, and who calls it

`mod text` (`🧰️framework/…/♾️infinite/🖼️canvas/🦀️component.rs:901` before this pass) builds a tiny
`<svg><text>…</text></svg>` string per label and parses it with `usvg::Tree::from_str` for two
distinct purposes, not one:

1. **Real glyph-shaped painting** — `append_label`/`append_label_tspans` call
   `render_svg_tree_literal` (from `svg_icon`) to rasterize the shaped `<text>` tree into a `Scene`.
   This *is* real glyph shaping+painting, i.e. verdict-(b) territory on its face.
2. **`usvg`-shaped advance measurement** — `label_byte_world_x`/`label_span_world_x` (via private
   `label_line_layout`/`label_prefix_advance_svg`) parse the same kind of `<text>` SVG purely to read
   back the shaped bounding box for caret/hit-test positioning — no `Scene` touched.

Traced every real caller of both groups, repo-wide (`grep -rn`, not the broken search tool):

- **`append_label`/`append_label_tspans`** — called only from `paint_*` helper functions in
  `➡️directed/🕸️dag/🦀️component.rs` (flow's `DagHost`: `paint_variadic_plus_controls`,
  `paint_node_name_vertical`, `paint_preview_image_content`, `paint_cluster_affordances`,
  `paint_node_name_horizontal`, `paint_note_caret_bar`, `paint_node_visual`) and from
  `✍️editor/🦀️component.rs`. `intrinsic-size-wiring.md` already traced `DagHost::paint_scene`'s own
  callers precisely: flow's `wasm_session` browser bridge (already `#[cfg(not(all(target_arch =
  "wasm32", target_env = "p2")))]`-gated in `📦️glue.rs`), `🗺️surface/🕸️node-graph`'s wrapper
  (confirmed absent from every plugin's wasip2 `cargo tree` output), and `#[cfg(test)]` blocks.
  `semio-framework-editor` — confirmed this session — is **not a dependency of `semio-s-plugin-
  puzzle`/`-flow`/`-trinity` at all** on `wasm32-wasip2` (`cargo tree -p <plugin> --target
  wasm32-wasip2 -i semio-framework-editor` → `did not match any packages`), so its own use of
  `append_label` is irrelevant to these three plugins regardless of reachability.
- **`label_byte_world_x`/`label_span_world_x`** — same paint-tree callers (`paint_note_caret_bar`)
  plus one genuinely different, non-painting path: `hit_byte_in_note_line` (private, in
  `➡️directed/🕸️dag/🦀️component.rs`) is called by `DagHost::begin_note_edit`, a `pub fn` that
  converts a click's world-x into a caret byte offset. `begin_note_edit`'s *only* real caller
  repo-wide is `🌊️flow/🌉️wasm/🦀️component.rs`'s `FlowBridge` dispatch (`domain.host.begin_note_edit(…)`)
  — and that whole module is mounted as `wasm_session` in flow's `📦️glue.rs` behind
  `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` (`"target_arch = "wasm32" is TRUE
  for wasm32-wasip2 too"`, the same gate class this ticket exists to enforce). So on
  `wasm32-wasip2`, nothing ever calls `begin_note_edit`, hence nothing ever calls
  `label_byte_world_x` through that path either.

**Conclusion: every real caller of every `usvg`-touching function in `mod text`, for all three named
plugins, is either inside the already-proven-host-only paint tree, behind an already-`p2`-gated
browser bridge, or in a crate that isn't even linked into these plugins' `wasm32-wasip2` builds.**
None of it is genuinely guest-reachable today.

## 2. Verdict: (b), with honest host-gating — not (a), not (c)

- **Not (c) dead code** — `mod text` is real, exercised code on native/browser (map labels, note
  editing, code-line rendering in the editor). Deleting it would be wrong.
- **Not (a) "only needs measurement"** — `append_label`/`append_label_tspans` genuinely rasterize
  shaped glyphs into a `Scene`; that is real painting, not a measurement query. A first-party
  measurement-only implementation (à la `semio-framework-intrinsic-size`) would not replace what
  these two functions actually do.
- **(b) applies, and the call-site check the ticket asks for came back clean**: nothing on
  `wasm32-wasip2` can reach real shaping/painting today, for any of puzzle, flow, or trinity. So the
  correct move — matching the exact working pattern `intrinsic-size-wiring.md` already established
  for `IconPaintCache::get_or_build` — is two implementations behind one identical public API:
  - `append_label`/`append_label_tspans` (paint, no return value) → `wasm32-wasip2` arm is a literal
    no-op. Not a stub for exercised behavior: nothing on this target could ever have observed a
    painted label, so "paint nothing" is the value every caller already implicitly gets today.
  - `label_byte_world_x` (measurement, returns `f64`) → `wasm32-wasip2` arm reuses this **same
    module's own existing "no real shaping" fallback shape**: `label_prefix_advance_svg`'s native
    body already falls back to the character-width heuristic
    (`label_advance`/`label_extent`/`label_text_inset`, no `usvg` involved) whenever
    `usvg::Tree::from_str` fails or produces empty bounds. The `wasm32-wasip2` arm is that identical
    heuristic, applied directly instead of as a failure fallback — a disclosed, intentional precision
    difference (character-width estimate vs. real glyph advance), not a stub, and byte offsets still
    map to a monotonically increasing world x. `label_span_world_x` needed no change at all — it just
    calls `label_byte_world_x` twice and resolves correctly per target automatically.

## 3. A second blocker found while tracing: `svg_icon` itself was still unconditional

Gating `mod text` alone would **not** have moved the `cargo tree` count. `svg_icon`'s own render
pipeline (`usvg_options_icons`, `to_affine`/`to_bez_path`, `render_path`/`render_group` and their
`_literal` siblings, `svg_icon_content_bounds`, `render_svg_tree_themed`, `append_svg_str`/`_themed`)
and the `SvgDocument` struct/impl (`pub struct SvgDocument(pub(crate) backend::usvg::Tree)`,
`parse_icons`, `content_bounds`, `render_themed`, `render_literal`) were **all still unconditional
`pub fn`/`pub struct` items**, referencing `usvg::*` types directly in their signatures — compiled
into every target's build (including `wasm32-wasip2`) regardless of whether `IconPaintCache::
get_or_build`'s *caller* was gated. Gating a caller does not gate its callees' own compilation; that
was the gap between `intrinsic-size-wiring.md`'s "painting closed" claim and the `cargo tree` number
staying at 97.

Traced every real caller of `svg_icon`'s render pipeline and of `SvgDocument`/`append_svg_document`
repo-wide: **all of them** are inside `IconPaintCache::get_or_build`'s already-`not(all(wasm32,
p2))`-gated native arm (`➡️directed/🦀️component.rs:783,794,801,812`), or `#[cfg(test)]` blocks
(`puzzle`'s `🔣️icons/🦀️component.rs`). `Dock`'s unrelated `Silhouette::content_bounds` (a different
type, same method name) was checked and ruled out as a false-positive grep hit. So this whole
pipeline got the identical host-only treatment as `mod text`, for the identical, independently
re-confirmed reason.

## 4. What was changed

`🧰️framework/…/♾️infinite/🖼️canvas/🦀️component.rs`:

- `vello_backend`'s `pub use vello_svg;` / `pub use vello_svg::usvg;` re-exports, the top-level
  `pub(crate) use renderer::vello_backend::usvg;` alias, and `SvgDocument`/`append_svg_document`'s
  export from `pub use renderer::{…}` — all gated `not(all(wasm32, p2))`.
- `SvgDocument` struct + its `impl` block, and `append_svg_document` — gated `not(all(wasm32, p2))`
  (host/browser-only; only real callers are `get_or_build`'s native arm and tests).
- `svg_icon` module: every `usvg`-touching item (25 items — statics, private helpers, and the public
  `usvg_options_icons`/`render_svg_tree_literal`/`svg_icon_content_bounds`/`render_svg_tree_themed`/
  `append_svg_str_themed`/`append_svg_str`) individually gated `not(all(wasm32, p2))`.
  `svg_icon_content_bounds_from_str`'s existing two-arm split (from `intrinsic-size-wiring.md`) was
  left untouched.
- `mod text`: `usvg_options_map_labels`, `escape_xml_attr`, `color_to_svg`, `LabelLineLayout`,
  `label_line_layout`, `label_prefix_advance_svg` gated `not(all(wasm32, p2))` (native-only; no
  wasip2 counterpart needed — nothing on that target calls them once `label_byte_world_x` has its own
  arm). `label_byte_world_x` and `append_label`/`append_label_tspans` split into two arms each (see
  §2). `label_extent`/`label_advance`/`label_text_inset` (already `usvg`-free) and
  `label_span_world_x` untouched/unconditional.

`🧰️framework/…/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️component.rs`:

- Split the `use super::canvas::{append_svg_document, Affine, FillRule, RasterImage, Rect, Scene,
  SvgDocument};` line so `append_svg_document`/`SvgDocument` (now native-only in `canvas`) are
  imported under a matching `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`; the
  target-neutral types stay unconditional.

`🧰️framework/…/♾️infinite/📦️packages/🦀️rust/Cargo.toml`:

- Removed `vello_svg` entirely from the `all(target_arch = "wasm32", target_env = "p2")` target
  table (previously present with `default-features = false`, which still pulled `usvg` per
  `intrinsic-size-parser.md`'s own finding that `vello_svg` hard-depends on `usvg` regardless of its
  feature flags). `vello` itself stays on both targets (target-neutral `Scene`/`peniko`/`kurbo`
  drawing-command types, used unconditionally elsewhere in `🎲️board`). Updated the surrounding
  docstrings to reflect the corrected, now-fully-traced picture (painting **and** `mod text`, not
  painting alone).

No file in `✏️s/🔌️plugins/🌊️flow`, `✏️s/🔌️plugins/🧩️puzzle`, or `✏️s/🔌️plugins/🔱️trinity`, and no
file in `semio-framework-editor`, was touched — every real caller of every gated symbol stayed
byte-for-byte unchanged, matching `intrinsic-size-wiring.md`'s own "zero callers changed" pattern.

## 5. Verification performed this session

### `cargo tree` (lock-free, cannot go stale)

Headline table above: 97 → 76 for all three plugins, identical count (confirms the fix lives in the
shared `os-infinite` crate, not per-plugin). Full, untruncated `-i` output for every crate name that
was ambiguous across the whole workspace (both `usvg@0.45.1`/`usvg@0.46.0`, both
`roxmltree@0.20.0`/`@0.21.1`, both `svgtypes@0.15.3`/`@0.16.1`) — every single one resolves to
"nothing to print" for all three plugins on `wasm32-wasip2`:

```
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i usvg              → ambiguous, both versions checked individually → nothing to print (either)
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i "usvg@0.45.1"     → nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i "usvg@0.46.0"     → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i fontdb            → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i vello_svg         → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i svgtypes          → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i tiny-skia-path    → nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i "roxmltree@0.20.0"→ nothing to print
$ cargo tree -p semio-s-plugin-flow    --target wasm32-wasip2 -i "roxmltree@0.21.1"→ nothing to print
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i fontdb/vello_svg/svgtypes/tiny-skia-path → nothing to print (all)
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i usvg/fontdb/vello_svg/svgtypes/tiny-skia-path → nothing to print (all)
$ cargo tree -p semio-s-plugin-puzzle  --target wasm32-wasip2 -i rustybuzz         → rustybuzz v0.20.1 ← compiler ← os-infinite ← plugin (expected, unchanged)
$ cargo tree -p semio-s-plugin-trinity --target wasm32-wasip2 -i rustybuzz         → rustybuzz v0.20.1 ← compiler ← os-infinite ← plugin (expected, unchanged)
```

### `cargo metadata --no-deps` — exit 0, parses clean with both Cargo.toml edits.

### Syntax verification — `rustfmt --edition 2021` on both edited `.rs` files

`rustfmt` fully parses a file (including its `include!`-mounted generated icon-name file) before it
can report anything; a genuine parse error (unbalanced braces, malformed `#[cfg(...)]`) would have
made it fail outright instead of emitting a diff. Both files parsed cleanly; `rustfmt` was then
applied to fix minor import-ordering it flagged (`git status` confirms only these two files changed,
no generated file was touched).

### Native/wasip2 `cargo check -p semio-framework-os-infinite` — attempted, both target-independent-blocked by an unrelated, pre-existing peer defect

```
error[E0277]: the trait bound `DslValue: serde::Deserialize<'de>` is not satisfied
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:200/231
error: could not compile `semio-framework-ui` (lib) due to 14 previous errors
```

Ran **twice** natively and once for `--target wasm32-wasip2`, all three foreground, all three fail
identically at `semio-framework-ui`'s `layout` module (`hover_args: Option<DslValue>` /
`#[derive(…, Deserialize)]` on a struct containing it) — an unconditional `pub mod`, not gated by any
feature, so it fails the same way on every target. `semio-framework-os-infinite`'s own source is
never reached by any of these three runs. Confirmed via full-output grep that **zero** error lines
name any file this session touched (`🖼️canvas/🦀️component.rs`, `➡️directed/🦀️component.rs`, or the
`os-infinite` `Cargo.toml`) or any symbol this session introduced (`svg_icon`, `SvgDocument`,
`append_label`, `label_byte_world_x`, `mod text`). `git log` on the offending file shows its last
commit predates this session; `git status` shows no uncommitted changes to it from this session. This
matches the ticket's own documented "known unrelated noise... live peer edits from other agents on
this same ticket" pattern (the in-flight `ArtifactStore`/`DslValue`-bound seam-5 migration
`verified-outcomes.md` already flags as "in progress") — **not** a defect introduced by this pass, and
not something in scope to fix here. The `cargo tree` evidence above is lock-free and metadata-only, so
it is unaffected by this unrelated compile blocker and remains the authoritative measurement.

## 6. Remaining gap to reach `draw-fsm`'s ~11-crate baseline

`rustybuzz` (and nothing else `usvg`-related) is the entire remaining delta traced this session,
pulled unconditionally by `compiler::compile_{snippet,emoji,text}_to_svg` for Math/Emoji/Text icon
generation — explicitly scoped out of this pass by `infinite-host-deps-split.md` as "a separate,
larger task" (shaping, not painting or measurement). The 76-crate figure for all three plugins is
`rustybuzz`'s own dependency tail plus the framework's remaining serde/tokio/etc. surface documented
elsewhere in `📓️verified-outcomes.md` — no `usvg`/`vello_svg`/`fontdb`/SVG-parsing crate remains
anywhere in any of the three plugins' `wasm32-wasip2` graphs.
