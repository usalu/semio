# Wave: text (parley/swash in 📏️layout) and path (kurbo in 🎞️animate)

Status: DONE. Both plugin-level `cargo check` runs are blocked by unrelated concurrent breakage
(see Verification below) — everything within this slice's own reach has been verified by actually
running it.

## Slice (a) — parley/swash in 📏️layout

### API surface found

`🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️text.rs` (`semio-framework-ui-render`,
aliased `ui_render` by convention — see `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/Cargo.toml`)
already wraps parley/swash/fontique completely: `TextSystem` (font registration/fallback, shaped
run cache, glyph atlas), `TextStyle`/`FontFamilyChoice`, `ShapedText`/`ShapedGlyph`,
`FontDependencyId`/`FontStatus`/`FontSource`/`FontFetch`, `measure`/`measure_wrapped`/`wrap`/
`shape`/`next_grapheme`/`previous_grapheme`/`selection_geometry`. The file's own docstring already
states no `parley::`/`swash::`/`fontique::`/`peniko::` type appears in a `pub` signature — verified
independently with `grep -n "^\s*pub " 🦀️text.rs | grep -iE "parley|swash|fontique|peniko"` (empty).

The layout plugin's call site,
`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️component.rs`'s
`LayoutEngine`, needed more than what `TextSystem` exposed: wrapped+aligned paragraph shaping
(font weight, relative line height, letter spacing, alignment) — `TextSystem` only had unwrapped
`shape`/wrapped-but-unaligned `measure_wrapped`/`wrap`. This was the real gap.

### Framework extension (`🦀️text.rs`)

Added, all first-party (no parley/swash in any signature):

- `TextAlignment` enum (`Left`/`Middle`/`Right`/`Justified`) — converted to `parley::Alignment` only
  inside a private `TextSystem::to_parley_alignment`.
- `TextRunStyle { base: TextStyle, weight: f32, line_height_relative: f32, letter_spacing_px: f32,
  alignment: TextAlignment }` — kept separate from `TextStyle` since `measure`/`wrap`/cursor-movement
  callers never need the extra fields.
- `TextSystem::shape_paragraph(&mut self, text: &str, style: &TextRunStyle, max_width: f32) ->
  ShapedText` — builds a `RangedBuilder` with FontStack/FontSize/FontWeight/LineHeight/LetterSpacing,
  `break_all_lines(Some(max_width))`, `align(...)`, then reuses the same glyph-extraction path as
  `shape` (refactored the shared tail into a new private `extract_shaped`).
- Falls back to a glyph-less placeholder `ShapedText` (reusing `placeholder_measurement`) while the
  style names an unresolved `FontDependencyId`, matching `measure`'s existing short-circuit.

No leaked type: verified again after the edit with the same `grep -n "^\s*pub "` filter — empty.

### Plugin rewire

`✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/Cargo.toml`: deleted the `parley`/`swash` lines, added
`ui_render = { path = "../../../../../🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust",
package = "semio-framework-ui-render" }` (same alias convention `ui-host`'s Cargo.toml already
uses).

`.../✏️editor/⚙️engine/🎬️scene/🦀️component.rs`:
- Removed `use parley::...`/`use parley::fontique::Blob`, `use std::borrow::Cow`, `use
  std::sync::Arc` (only needed by the deleted parley code).
- `LayoutEngine` now holds `{ text: TextSystem, font: FontDependencyId }` instead of
  `{ font_context: FontContext, layout_context: LayoutContext<[u8;4]>, fonts_ready: bool }`.
  `LayoutEngine::new` registers the embedded `LAYOUT_SANS` font synchronously via
  `text.request_font(...)` + `text.provide_font_bytes(...)` — both are plain sync calls in
  `TextSystem`, so this also fixes a latent bug: the old code's `self.ensure_fonts()` (an `async
  fn`) was called without `.await` inside `layout_story`, meaning the embedded font registration
  never actually ran (the future was constructed and immediately dropped). The new code has no
  such gap.
- `LayoutEngine::layout_story`/`layout_story_in_frame`/`alignment_from_str`/`default_paragraph`
  converted from `async fn` to plain `fn` — every call site in this file already called them
  without `.await` (`let (layout, _overset) = layout_story_in_frame(...)`, which cannot type-check
  against a `Future`-returning `async fn`), so this was already latently broken/inconsistent with
  the repo-wide async-convention debt tracked separately; making these four functions plain `fn`
  is the minimal fix consistent with their existing (already-non-awaiting) call sites, not a
  broader async-debt cleanup.
- The glyph-extraction loop in `build_display_list_for_page` (`for line in layout.lines() { for
  positioned in line.items() { if let PositionedLayoutItem::GlyphRun(run) = ... } }`) replaced with
  a direct `shaped.glyphs.iter().map(...)` over `ShapedText::glyphs: Vec<ShapedGlyph>`.
- Test `layout_story_in_frame_resolves_alignment_variants_and_detects_overset` updated:
  `layout.height()` → `shaped.height` (a field, not a method, on `ShapedText`).

### Differential-test design

In `🦀️text.rs`'s own `#[cfg(test)] mod tests` (parley is already a normal `[dependencies]` entry
of this framework crate — the "declare the oracle in `[dev-dependencies]`" rule targets a crate
that does *not* already depend on the third-party library; here it already does, as the platform
wrapper, so the oracle is used directly):

- `shape_paragraph_wraps_at_max_width_and_never_mid_char` / 
  `shape_paragraph_alignment_variants_respect_max_width_and_measure_positive_height` — fixture
  tables (text, max_width, expectation) exercising every `TextAlignment` variant.
- `shape_paragraph_agrees_with_an_independently_built_parley_layout` — builds a *second*,
  independent `Collection`/`FontContext`/`LayoutContext`/`RangedBuilder` pipeline directly against
  `parley` (not reusing any `TextSystem` internals) with the same style properties, and asserts our
  wrapper's glyph count and width/height agree within `0.01px` (tight, because both paths run the
  identical parley version on identical inputs — this is the "did the wrapper diverge from what it
  wraps" proof, not a cross-version tolerance).

## Slice (b) — kurbo in 🎞️animate

### API surface found

`semio-framework-geometry` (aliased `geometry`, already a direct dependency of the animate plugin)
already had a comprehensive kurbo-backed 2D vocabulary in
`🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs`: `Point`/`Vec2`/`Affine` (with
`translate`/`scale`/`rotate`/`as_coeffs`), `Rect`/`RoundedRect`/`Circle`/`Line`/`Arc`/`CubicBez`,
`PathEl` (Move/Line/Quad/Curve/Close, converts to/from `kurbo::PathEl`), and `BezPath` (wraps
`kurbo::BezPath`: `move_to`/`line_to`/`quad_to`/`curve_to`/`close_path`/`push`/`elements`/
`bounding_box`). This is the "already wraps it, just extend the gap" case the ticket predicted.

The only gap: a `kurbo::PathSeg`-equivalent (per-segment eval/arclen/subsegment) and an
affine-apply on a whole `BezPath` — both used directly via `kurbo::{ParamCurve, ParamCurveArclen,
PathSeg, Shape}` at the single call site
`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️component.rs`'s
`sobject` module (`trim_path_at_ratio`, `sample_path_points`, `transform_bezpath`, `lerp_affine`).
Confirmed via `grep -rln "use kurbo" ✏️s/🔌️plugins/🎞️animate` — exactly one file.

### Framework extension (`⚙️engine/🦀️.rs`, `semio-framework-geometry`)

Added, all first-party:

- `BezPath::path_segments(&self) -> Vec<PathSeg>` — walks `elements()` into `Line`/`Quad`/`Cubic`
  segments; a `ClosePath` becomes an implicit closing `Line` back to the subpath's `MoveTo` (skipped
  if already there). No arc-flattening `tolerance` parameter like kurbo's own `path_segments` takes
  — our `PathEl` has no `Arc` variant to flatten, so it's inapplicable.
- `BezPath::apply_affine(&self, affine: Affine) -> Self` — maps `affine` over every point of every
  element; replaces the plugin's local `transform_bezpath` that round-tripped through
  `to_kurbo()`/`kurbo::BezPath::apply_affine`.
- `BezPath::is_empty(&self) -> bool`.
- `PathSeg` enum (`Line`/`Quad`/`Cubic`, first-party — not a wrapper over `kurbo::PathSeg`) with
  `start`/`end`/`eval`/`subdivide_at`/`subsegment`/`as_path_el`/`arclen`, every method a from-scratch
  implementation (De Casteljau subdivision for eval/split; a line's arclen is the exact chord
  distance).
- `PathSeg::arclen(&self, accuracy: f64) -> f64` — **adaptive recursive subdivision by
  control-polygon length**, not a call into `kurbo`: at each level, compares the chord length
  (`start`↔`end`) against the control-polygon length (the sum of the control-point-to-control-point
  segment lengths — always ≥ the true arc length, since straight-line-cutting a curve's control
  polygon can only shorten it toward the chord). When that gap is within `accuracy`, returns the
  average of chord and control-polygon length as the estimate for that piece; otherwise splits at
  `t=0.5` and recurses with `accuracy` **halved per level** (so the sum of every leaf's error stays
  bounded by roughly the original `accuracy` — a geometric series, not a per-leaf budget that grows
  with depth), capped at depth 32 as a hard backstop. Documented in the method's own docstring.

### Plugin rewire

`✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml`: deleted only the `kurbo = "0.13.1"` line
(left every other line — including the `typst`/`vello`/`wgpu`/`image` entries another concurrent
wave is already mid-removing — untouched). `geometry` was already a direct dependency, so no new
line was needed.

`.../✏️editor/⚙️engine/🎬️scene/🦀️component.rs`'s `sobject` module: removed `use kurbo::{ParamCurve,
ParamCurveArclen, PathSeg, Shape};`; `trim_path_at_ratio`/`sample_path_points` rewired onto
`BezPath::path_segments()` + `PathSeg::{arclen, start, subsegment, as_path_el, eval}` (deleted the
`kurbo::BezPath`/`bezpath_from_kurbo` round-trip entirely — no compat shim); `paths()`/
`transform_bezpath` collapsed onto `BezPath::apply_affine` (the local `transform_bezpath` helper
function deleted outright); `lerp_affine` now reads `Affine::as_coeffs()` directly instead of
round-tripping through `.to_kurbo().as_coeffs()`.

**Scope note on `vello::kurbo`**: the ticket flagged `video/🦀️component.rs` as a possible
`vello::kurbo::Stroke` site to leave alone. By the time this slice landed, a concurrent wave had
already rewritten that file — `grep -rln kurbo ✏️s/🔌️plugins/🎞️animate` now returns only
`AGENTS.md` (not touched, per CLAUDE.md). No `vello::kurbo` call site remains to carve out.

### Differential-test design

New `path_seg_tests` module in `⚙️engine/🦀️.rs` (kurbo is already a direct `[dependencies]` entry
of `semio-framework-geometry`, same "already the platform wrapper" reasoning as slice (a) — used
directly as the oracle rather than added again under `[dev-dependencies]`):

- Fixture tables: `eval_matches_hand_computed_fixtures` (hand-computed Bernstein-basis expected
  points), `start_and_end_match_endpoints_for_every_variant`,
  `subdivide_at_endpoints_join_at_the_split_point_and_reproduce_original_endpoints`,
  `subsegment_full_range_is_identity_and_half_ranges_sum_to_whole_arclen`, `line_arclen_is_exact`,
  `path_segments_walks_a_multi_subpath_document_and_closes_each_subpath`,
  `path_segments_skips_a_redundant_close_when_already_at_the_start_point`,
  `apply_affine_translates_every_point_including_control_points`,
  `as_path_el_round_trips_through_a_fresh_bezpath`.
- `cubic_quarter_circle_approximation_matches_analytic_arc_length` — the standard
  `kappa≈0.5522847498` cubic-Bezier quarter-circle construction, checked against the analytic
  `π·r/2` (independent of kurbo entirely).
- `arclen_agrees_with_kurbo_param_curve_arclen_across_random_curves` — a constant-seeded in-test
  LCG (`Lcg`, `6364136223846793005`/`1442695040888963407` constants, never the `rand` crate)
  generates 32 deterministic pseudo-random quad/cubic curves; asserts our `PathSeg::arclen` agrees
  with `kurbo::ParamCurveArclen::arclen` (same `accuracy = 1e-4`) within a **relative** `0.5%`
  tolerance (relative, not absolute, because curve sizes span ~1–200 units in the fixture range).

## Verification

### Framework unit tests (executed)

`cargo test -p semio-framework-ui-render --lib text::` — **PASSED**, 11/11, including the new
`shape_paragraph_*` tests and the differential oracle test. Verbatim tail:

```
running 11 tests
test text::tests::atlas_insertion_queues_exactly_one_upload_and_a_repeat_glyph_queues_none ... ok
test text::tests::measuring_with_an_unloaded_custom_font_yields_pending_with_a_placeholder ... ok
test text::tests::utf8_utf16_index_conversion_round_trips_across_a_non_bmp_emoji ... ok
test text::tests::selection_geometry_covers_the_requested_range_with_at_least_one_rect ... ok
test text::tests::measuring_the_same_string_twice_serves_the_second_call_from_the_shape_cache ... ok
test text::tests::shape_paragraph_agrees_with_an_independently_built_parley_layout ... ok
test text::tests::measuring_ascii_run_is_stable_and_matches_shaped_advance ... ok
test text::tests::cursor_movement_never_lands_inside_the_emoji_grapheme_cluster ... ok
test text::tests::wrapping_breaks_at_max_width_and_never_mid_char ... ok
test text::tests::shape_paragraph_wraps_at_max_width_and_never_mid_char ... ok
test text::tests::shape_paragraph_alignment_variants_respect_max_width_and_measure_positive_height ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 118 filtered out; finished in 0.02s
```

(Run with `RUSTC_WRAPPER="" CARGO_BUILD_RUSTC_WRAPPER="" CARGO_TARGET_DIR=<scratchpad>/ui-render-target`
to bypass sccache serialization under heavy concurrent-build load — see
`project-sccache-serializes-concurrent-builds` — not a change to the repo's own build config.)

### `cargo test -p semio-framework-geometry` — BLOCKED by unrelated pre-existing breakage

`cargo test -p semio-framework-geometry --lib` fails to compile — but the 49 errors are all
`` `random::Rng`/`random::SplitMix64` is not a future `` in
`🧰️framework/🔨️modules/📐️geometry/🎲️random/🦀️.rs` (`.await` on a non-`Future` constructor inside
that file's own `#[cfg(test)]` block), a file this slice never touched. `git status` shows it as a
staged-but-uncommitted addition last modified 2026-08-21 — pre-existing async-convention debt (see
memory `project-semio-async-convention-debt.md`), unrelated to `PathSeg`/kurbo and out of this
slice's scope per the ticket's own "ignore unrelated recent changes" rule.

`cargo check -p semio-framework-geometry --lib` (production code only, no `#[cfg(test)]`) —
**PASSED** cleanly, confirming the new `PathSeg`/`BezPath::{path_segments,apply_affine,is_empty}`
code itself compiles with no errors:

```
    Checking semio-framework-geometry v0.1.0 (.../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 41.97s
```

Because the in-crate test suite could not run, the `PathSeg::arclen` algorithm (byte-identical
logic, `Point` substituted for a plain `(f64,f64)` tuple) was additionally verified in a standalone
scratch crate (`<scratchpad>/pathseg-verify`, `kurbo = "0.13.1"` as its only dependency) — actually
executed:

```
line exact OK
quarter circle OK: est=15.71016698073856 analytic=15.707963267948966
split sum OK: whole=20 a+b=20
differential oracle OK across 200 curves, max relative error = 0.000001
ALL PATHSEG VERIFICATIONS PASSED
```

200 random quad/cubic curves (vs. 32 in the in-crate test) all agreed with `kurbo::ParamCurveArclen`
to within 0.5% relative error (max observed: 0.0001%). This is real evidence the algorithm is
correct; it is not a substitute for the in-crate test actually running once `🎲️random/🦀️.rs` is
fixed by whoever owns that debt.

### Plugin builds — BLOCKED by unrelated concurrent breakage (not this slice's code)

`cargo check -p semio-s-plugin-animate` and `cargo check -p semio-s-plugin-layout` (both run with
`RUSTC_WRAPPER="" CARGO_BUILD_RUSTC_WRAPPER=""` and a dedicated `CARGO_TARGET_DIR` under the
scratchpad, to bypass sccache serialization under ~163 concurrent rustc/cargo processes from other
sessions — see `project-sccache-serializes-concurrent-builds`) both compiled deep into their real
dependency graphs — in both runs `semio-framework-geometry` (this slice's own crate) appears as
`Checking semio-framework-geometry v0.1.0 ... ` with **zero errors attributed to it** — and then
both hit the identical unrelated error in a transitive dependency, `semio-framework-replication`:

```
error[E0433]: cannot find module or crate `semio_framework_deflate` in this scope
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../⚙️codec/🦀️.rs:390:12
    |
390 |         Ok(semio_framework_deflate::deflate(raw))
    |            ^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_deflate`
...
error: could not compile `semio-framework-replication` (lib) due to 9 previous errors
```

Root-caused: `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml` declares
`semio-framework-deflate` as `optional = true` (`deflate = ["dep:semio-framework-deflate"]`), but
`⚙️codec/🦀️.rs` calls `semio_framework_deflate::{deflate,inflate,InflateOutcome}` unconditionally
(not behind `#[cfg(feature = "deflate")]`) — a mid-refactor gap in a completely different,
concurrently-running slice (the master plan's `📓️status.md` lists "base64 ×7 plugins" and
`semio-framework-hash`-adjacent slices as "running" right now; `🧰️framework/🔨️modules/🗜️deflate`
is a real, freshly-added crate, confirming this is that wave's in-flight work, not a typo in this
diff). Both `semio-s-plugin-animate` and `semio-s-plugin-layout` transitively depend on
`semio-framework-replication`, so both hit this identically, for reasons that have nothing to do
with `parley`/`swash`/`kurbo`. Per the ticket's own escape valve ("if so, verify your slice with
`cargo check -p <animate crate>` instead and SAY SO explicitly"), the same treatment was applied to
`layout`: **`cargo check -p semio-framework-geometry --lib` (PASSED, see above) and the fact that
`semio-framework-geometry` compiles cleanly inside both plugins' real dependency graphs (confirmed
twice) stand in as this slice's plugin-level verification.** `wasm32-wasip2` builds were not
attempted for either plugin — they would hit the same `semio-framework-replication` blocker before
ever reaching the code this slice touched, so running them would add no new information.

### Repo-wide grep gate

```
$ grep -rnE '^(parley|swash|kurbo) ?=' ✏️s --include=Cargo.toml
(no output)
```

## Honesty notes / unfinished

- `cargo test -p semio-framework-geometry` cannot currently pass end-to-end due to unrelated
  pre-existing async-convention debt in `🎲️random/🦀️.rs` (staged Aug 21, not this slice's file,
  not touched). Production code verified via `cargo check --lib` (passed) + a standalone
  differential harness (actually executed, 200 random curves, max relative error vs
  `kurbo::ParamCurveArclen` 0.0001%) instead.
- `cargo check -p semio-s-plugin-animate` / `-p semio-s-plugin-layout` (native) and the
  `wasm32-wasip2` builds could not be run to a clean finish for either plugin — both are blocked by
  an unrelated, currently in-flight `semio-framework-replication`/`semio-framework-deflate` wiring
  gap owned by a concurrent slice (see Verification above for the exact error and root cause). This
  slice's own crate (`semio-framework-geometry`) is confirmed error-free inside both plugins' real
  dependency graphs up to that point, and the layout plugin's Cargo.toml/source changes were
  reviewed line-by-line for stray `parley`/`swash`/`Layout<...>` references (none remain, grep
  clean). If the deflate wiring gets fixed by its owning slice, re-running
  `cargo check -p semio-s-plugin-animate` / `-p semio-s-plugin-layout` and the two
  `wasm32-wasip2` builds is the one remaining step to close this out end-to-end.
- The `LayoutEngine`/`layout_story`/`layout_story_in_frame`/`alignment_from_str`/
  `default_paragraph` functions were converted from `async fn` to plain `fn` because every existing
  call site already called them without `.await` (destructuring the return value directly) — this
  was necessary for the file to type-check at all and is the minimal fix, not a broader pass over
  the repo's separate async-convention-debt effort.
