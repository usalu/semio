# W5b — 📏️layout svg/dwg pattern extraction

Agent scope: `✏️s/🔌️plugins/📏️layout/**` only. `✏️s/🔌️plugins/🗄️stdio/**` read-only (not modified).

Note on filename: the task instructions said to write the report to the literal path
`w5b--report.md`, but this wave dispatches one agent per pattern plugin (🗒️note, 📏️layout, 🌍️gis,
🎥️shooting, 🌀️procedural, 🖨️raster, 🖍️draw, 🧩️puzzle) — writing to that exact shared filename would
race/clobber the other 7 agents' reports. Wrote to `w5b-layout-report.md` instead (matches the plan's
own `w<N><agent>-report.md` convention); the closer should fold these into a combined `w5b--report.md`.

## Files changed

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/⚙️engine/🎬️scene/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs` (1-line unrelated pre-existing path fix, see
  "Foreign breakage encountered and fixed" below)

## What changed

### ⚙️engine/🦀️component.rs

- **`layout_document_json_to_svg`**: previously called `semio_framework_os::pages_rects_svg` — a
  hand-rolled SVG string builder in framework `os` (`format!("<rect .../>")` etc.). Now: deserializes
  the `Value` into a `LayoutSnapshot`, maps it to a real `SemioDrawingSnapshot`
  (`layout_snapshot_to_semio_drawing`, new), and composes real SVG text through stdio's real
  `s.stdio.semio/v1/drawing` → `s.stdio.svg` bridge via `io_dispatch` (`compose_svg_from_drawing`, new,
  `pub(crate)`). No SVG string is built in this crate anymore.
- **`layout_snapshot_to_semio_drawing`** (new): maps each `Page` onto a translated `DrawNode::Group`
  (pages laid out side by side, 24px gap — matches the old `pages_rects_svg` thumbnail arrangement),
  nesting the page boundary plus every visible frame as a rect-shaped `DrawNode::Path`
  (`rect_path_segments`, new, `pub(crate)` — the shared "rects-as-paths" primitive). `Frame::Rect`
  frames keep their real fill/stroke color; `Frame::Text`/`Frame::Image` frames get a neutral outline
  color (mirrors the existing blueprint-chrome colors `⚙️engine/🎬️scene` already uses for the same two
  frame kinds).
- **`layout_document_json_from_dwg`**: kept `dwg_rect_pages` unchanged (real geometric rectangle
  detection over an already-decoded `DwgDrawing` — this was never hand-rolled bytes; the input is
  produced by `semio_framework_os::dwg_from_bytes`, a real mesh-module DWG parser, before this plugin
  ever sees it). Its output is now funneled through a real `SemioDrawingSnapshot`/`DrawNode` tree
  (`dwg_drawing_to_semio_drawing`, new) before being mapped back into `Page`s (`path_bounds`, new — the
  exact inverse of `rect_path_segments`), instead of building `Page`s straight off a bespoke
  `Vec<(f64,f64,f64,f64)>` tuple list. See **stdio_gaps** below for why this direction doesn't go
  through `io_dispatch` the way the SVG-export direction does.
- Added `LayoutError::Svg(String)` variant (surfaces `compose_svg_from_drawing` failures to
  `⚙️engine/🎬️scene`'s `Result<String, LayoutError>`-returning exporters).
- Added `ensure_stdio_semio_drawing_registered` (`#[cfg(test)]`, `pub(crate)`, `std::sync::Once`-guarded)
  — registers stdio's real drawing-subset composer (which registers its own svg/dxf/pdf io leaves) into
  the shared `io` registry once per test binary. `cargo test` never runs the plugin-host boot path that
  would normally call this at runtime, so the tests do it themselves.
- Added 2 tests to the existing `mod tests` region: `svg_export_composes_through_semio_drawing_bridge`
  (real end-to-end proof through the actual `io_dispatch` registry — asserts `<svg`/`<path`/`</svg>`
  shape and the composed canvas dimensions: 2 demo pages × 400×500 + 24px gap = 824×500) and
  `svg_export_rejects_invalid_document_json`.

### ⚙️engine/🎬️scene/🦀️component.rs

- **`export_display_list_svg`**: previously hand-rolled an SVG string directly from a `DisplayList`
  (background `<rect>`, per-rect fill/stroke `<rect>`s, image-placeholder `<rect>`s, one small `<rect>`
  per glyph). Now: maps the `DisplayList` onto a real `SemioDrawingSnapshot`
  (`display_list_to_semio_drawing`, new — same visual fidelity as before: white background, one path
  per filled rect and one per stroked rect [both emitted when a `DisplayRect` has both], placeholder-vs-
  resolved image tint paths, one small filled path per glyph — same "glyph as a small box" fidelity the
  old code had; this engine still never emits real font outlines to SVG on either the old or new path)
  and composes it through the same `compose_svg_from_drawing` bridge `engine/🦀️component.rs` exports.
  Signature changed `-> String` to `-> Result<String, LayoutError>`. Its only caller,
  `export_document_svg`, already returned `Result<String, LayoutError>`, so this is a transparent
  propagate-with-`?` internally; its own 3 external call sites (`apps/layout/🦀️component.rs`'s
  `export_media`, `🌉️wasm/🦀️component.rs`'s `export_svg`, `🎮️commands/🐚️export/🦀️component.rs`'s
  `handle`) already treated `export_document_svg` as fallible, so none of them needed changes — verified
  by reading all 3 call sites.
- Updated the existing test `svg_export_contains_rect_and_wraps_a_valid_document` →
  `svg_export_contains_path_and_wraps_a_valid_document`: asserts `svg.contains("<path")` instead of
  `"<rect"`, with a comment explaining why (the drawing subset's SVG serializer
  (`semio-s-plugin-stdio`'s `SemioDrawingToSvg`) always lowers `DrawNode::Path` to an SVG `<path>`
  element — `DrawNode` has no rect-shaped variant, so the bridge's vocabulary has no `<rect>`). Also
  calls the new `ensure_stdio_semio_drawing_registered()` helper.

## Deleted

- Both hand-rolled SVG string builders (`layout_document_json_to_svg`'s body and
  `export_display_list_svg`'s body) — removed outright, no feature flag, no fallback path kept.
- The only call site of `semio_framework_os::pages_rects_svg` in this plugin. (`pages_rects_svg` itself
  lives in framework `os`, out of this agent's write scope, and is presumably still called by other
  W5b sibling plugins, e.g. 🌍️gis's likely use of the sibling `map_points_svg` helper in the same
  framework module — not deleted here.)

## stdio_gaps

1. **No `drawing↔dwg` bridge registered in stdio.** stdio's `s.stdio.semio/v1/drawing` subset
   (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🎹️composer/🦀️component.rs`)
   registers io leaves for svg/dxf/pdf only — matches the master plan's own lattice ("drawing↔svg/dxf/pdf";
   dwg is not listed there). `layout_document_json_from_dwg` therefore cannot route its DWG import
   through `io_dispatch` the way `layout_document_json_to_svg`'s export direction does. It still avoids
   any hand-rolled byte manipulation (see above) and now expresses the extracted geometry as a real
   `SemioDrawingSnapshot`/`DrawNode` tree rather than a bespoke tuple list, so the "page/rect model maps
   onto `DrawNode`" shape from the task brief is honored symmetrically on both directions — just without
   an actual cross-crate `io_dispatch` call on the import side. If a future wave adds a `drawing↔dwg`
   bridge to stdio, `layout_document_json_from_dwg` is the natural place to switch over.
2. `⚙️engine/🎬️scene/🦀️component.rs`'s other exporters — `export_document_png_cpu`, `export_document_pdf`,
   `scene_png_from_display_list` — are still hand-rolled (raw PNG raster fill via the `png`/`image`
   crates directly, and a hand-built minimal PDF byte stream). These were **not named in this ticket's
   scope** (only the two `*_document_json_to_svg`/`*_document_json_from_dwg`-shaped functions plus the
   scene SVG emitter were named), and stdio's `drawing`/`image` subsets do have real `drawing↔pdf` and
   `image↔png` bridges that a future wave could route these through the same way `export_display_list_svg`
   now routes through `drawing↔svg`. Left untouched to stay in scope.

## Foreign breakage encountered

`git status` at start of the task showed the plugin clean (only my own edits appear there now).
Two independent kinds of foreign breakage were hit while verifying:

1. **Fixed** (landed, non-dirty, one-line, within my write scope): `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs`
   line 439 pointed `pub mod document` at `../../🎛️apps/📏️layout/📌️panels/📄️document/🦀️component.rs`, but
   that directory no longer exists — it was renamed to `📄️artifact` by an unrelated, already-committed
   terminology-rename commit (`c31024cc6c`, 2026-08-10, "Rename framework-wide document contracts to
   artifact..."), and this one glue.rs reference was never updated. Confirmed via `git log`/`git show`
   that the commit landed (not mid-edit) and that `📄️artifact/🦀️component.rs`'s doc comment/content is
   verbatim the old "document panel" (spreads/pages/frames/layers/stories/links tree). Retargeted the
   `#[path = ...]` to `📄️artifact/🦀️component.rs`, kept `pub mod document;` (the Rust module name) so
   every existing `panels::document`/`document_panel` reference elsewhere keeps compiling unchanged.
   This blocked `cargo check -p semio-s-plugin-layout` from even starting to parse the module tree, so
   it had to be fixed to verify anything at all; it's a pure "lagging call-site of a landed foreign
   refactor" per the master plan's hazard-management rule, and is inside my own write scope.

2. **NOT fixed** (out of scope and/or actively mid-edit — reported here instead): after the glue.rs fix,
   `cargo check -p semio-s-plugin-layout` still fails with 6 errors, all in **pre-existing io leaf files
   under `🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/...` that I never touched** (not
   `⚙️engine/🦀️component.rs` or `⚙️engine/🎬️scene/🦀️component.rs`, the two files this ticket named):
   - `📤️export/…/🖊️dwg/🔖️ac1018/✳️any` and (transitively) stdio's `DwgSnapshot` — `git status` shows
     stdio's `🖊️dwg` artifact **currently dirty** (`MM` on its engine/snapshot/mutations files) — an
     active, in-progress foreign edit. Per the plan's hazard rules ("mid-edit files may not [be
     completed]") this was left alone.
   - `📤️export/…/📄️pdf/🔖️1.4/✳️any` and `📥️import/…/📄️pdf/🔖️1.4/✳️any` — stdio's `PdfSnapshot` no
     longer has a `page: PageDoc{width,height,text}` field at all; it's been replaced with
     `pages: Vec<PdfPage>` by a landed (git-clean, not dirty) foreign commit. Not a mechanical rename —
     `PageDoc` doesn't exist anymore and would need a real design decision for how a multi-page,
     structured `PdfPage` maps to/from `LayoutSnapshot`. Out of this ticket's named scope.
   - `📤️export/…/🔣️json/🔖️rfc8259/✳️any` and `📥️import/…/🔣️json/🔖️rfc8259/✳️any` — stdio's
     `JsonSnapshot.value` is now a real handcrafted `JsonValue` enum (the "object" subset's own
     lexeme-preserving typed value graph per the master plan), not `serde_json::Value` — landed
     (git-clean), also not a mechanical fix (needs a real `serde_json::Value ↔ JsonValue` converter).
     Out of this ticket's named scope.

   None of these 6 errors are in the two files this ticket asked me to rewire; I verified this by
   reading every error's file:line in the pasted output below. `cargo test -p semio-s-plugin-layout --lib`
   hits the identical 6 errors (same crate, same compile step) — pasted below too, for completeness, not
   because it's a different result.

## Verification

`cargo check -p semio-s-plugin-layout 2>&1 | tail -150` (full output also saved to
`w5b--layout-cargo-check.txt` in this ticket folder):

```
warning: `semio-s-plugin-stdio` (lib) generated 482 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 171 suggestions)
    Checking semio-s-plugin-layout v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust)
error[E0432]: unresolved import `semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc`
 --> .../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:3:5
  | no `PageDoc` in `artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot`

error[E0609]: no field `page` on type `&semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot`
 --> .../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:9:61

error[E0308]: mismatched types (JsonValue vs. serde_json::Value)
 --> .../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9:66

error[E0308]: mismatched types (JsonValue vs. serde_json::Value)
 --> .../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9:28

error[E0063]: missing fields `codepage` and `maintenance_version` in initializer of `DwgSnapshot`
 --> .../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs:10:8

error[E0560]: struct `PdfSnapshot` has no field named `page`
 --> .../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs:11:9

error: could not compile `semio-s-plugin-layout` (lib) due to 6 previous errors; 18 warnings emitted
```

`cargo test -p semio-s-plugin-layout --lib 2>&1 | tail -30` (full output also saved to
`w5b--layout-cargo-test.txt`):

```
error[E0432]: unresolved import `semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc`
error[E0609]: no field `page` on type `&semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot`
error[E0308]: mismatched types (JsonValue vs. serde_json::Value) ×2
error[E0063]: missing fields `codepage` and `maintenance_version` in initializer of `DwgSnapshot`
error[E0560]: struct `PdfSnapshot` has no field named `page`
error: could not compile `semio-s-plugin-layout` (lib test) due to 6 previous errors; 22 warnings emitted
```

Both gates fail for the identical 6 pre-existing/foreign reasons documented above — **zero** errors or
warnings attributable to my own new/changed code in `⚙️engine/🦀️component.rs` or
`⚙️engine/🎬️scene/🦀️component.rs` (confirmed by reading every error's file:line; the only warnings
those two files still emit — 3 `unnecessary qualification` lints around the pre-existing
`LayoutArtifactEngine` struct — predate this change and are outside the edited regions). I could not
produce a green `cargo test` run or pasted passing-test numbers for `svg_export_composes_through_semio_drawing_bridge`
etc. as a result — the whole crate must link to run any test binary. Recommend the closer either (a)
waits for whichever wave is mid-flight on stdio's `dwg`/`pdf`/`json` artifacts to land, or (b) assigns a
follow-up to bring layout's `✳️any/🚪️io` pdf/json/dwg leaves in line with stdio's current shapes (real
design work, not a one-line fix, and squarely inside `✏️s/🔌️plugins/📏️layout/**` so any future agent
can do it without touching stdio).
