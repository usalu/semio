# W5b — 🌍️gis (svg/dwg pattern extraction)

## Scope

Write scope: `✏️s/🔌️plugins/🌍️gis/**` only. Never edited `✏️s/🔌️plugins/🗄️stdio/**` (read-only,
gaps reported below instead of patched).

Target file (recon §4, re-verified current line numbers before editing):
`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — functions
`gis2d_document_json_to_svg` (was line 166) and `gis2d_document_json_from_dwg` (was line 174).

## What changed

### `gis2d_document_json_to_svg` — rewired onto the real stdio semio/drawing↔svg bridge

Previously delegated entirely to `semio_framework_os::map_points_svg(value, "GIS 2D")` — a
hand-rolled `format!("<svg …>{paths}</svg>")` string builder living in framework (out of my write
scope, and itself the kind of hand-rolled SVG emission this ticket retires). Replaced with a real
pipeline, entirely inside the gis plugin's own engine file:

1. `gis_map_snapshot_to_drawing(&GisMapSnapshot) -> SemioDrawingSnapshot` (new, public — reusable
   by any future gis drawing preview) builds a real
   `semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot`:
   - each position feature (`{lon, lat}` payload) → a circular marker `DrawNode::Path` (`MoveTo` +
     2×`ArcTo` + `Close` — the standard two-arc SVG circle recipe), styled `gis-point` (filled blue).
   - each route feature (`{points: [[lon,lat], …]}`) → an open polyline `DrawNode::Path`
     (`MoveTo` + `LineTo`*), styled `gis-line` (black stroke).
   - each region feature → the same, closed (`+ Close`).
   - canvas sized to the real feature bounding box (32px pad, 256px floor), one layer/one group.
2. `render_drawing_to_svg(&SemioDrawingSnapshot) -> Result<(String, u32, u32), String>` (new)
   packs the snapshot (`store::ArtifactPack::encode_pack`), calls
   `semio_framework_plugin::io_dispatch` with the `s.stdio.semio/v1/drawing` → `s.stdio.svg/1.1/*`
   `IoKey` (built from `SemioDrawingToSvg::FROM`/`::INTO` — no hardcoded dialect strings), decodes
   the returned `SvgSnapshot` (`ArtifactPack::decode_pack`) and prints real SVG text
   (`ArtifactDsl::print_dsl`) — the actual XML `svg` tree stdio's real SVG 1.1 engine produced, not
   a hand-formatted string.
3. `gis2d_document_json_to_svg` itself is now 3 lines: parse `value` as `GisMapSnapshot`, build the
   drawing, render it.

No hand-rolled SVG string emission remains anywhere in gis.

### `gis2d_document_json_from_dwg` — DWG entities lowered through `DrawNode`/`PathSegment`

Previously matched directly on `DwgGeometry` variants (`Point`/`Line`/`LwPolyline`/`Polyline3d`)
and flattened vertices inline. Replaced with:

- `dwg_geometry_to_draw_node(&DwgGeometry) -> Option<DrawNode>` — lowers one entity to a
  `DrawNode::Path` with the appropriate `PathSegment` sequence (`Point`→single `MoveTo`,
  `Line`→`MoveTo`+`LineTo`, `LwPolyline`/`Polyline3d`→`MoveTo`+`LineTo`*+optional `Close` when the
  entity is closed) — exactly the "entities map onto `DrawNode::Path`" shape requested.
- `dwg_drawing_to_semio_drawing(&DwgDrawing) -> SemioDrawingSnapshot` — builds a real drawing
  snapshot from all of a `DwgDrawing`'s entities.
- `collect_draw_node_points` — walks the resulting `DrawNode` tree collecting every
  `MoveTo`/`LineTo` endpoint (replaces the old direct `DwgGeometry` vertex walk with a walk over
  the semio/drawing shape).
- `gis2d_document_json_from_dwg` now: build the drawing scene → collect its vertices → same
  fallback-to-`default_document()`-when-empty / position-feature-building tail as before.

**Honest limit, not a workaround (see `stdio_gaps` below)**: the *outer* function signature stays
`fn(&DwgDrawing) -> Result<Value, String>` — this is frozen by
`semio_framework_os::register_dwg_import_handler`'s parameter type (`fn(&DwgDrawing)`, a bare fn
pointer, framework code, out of scope, and explicitly a "leave MediaFormat call sites compiling,
W6's cut" boundary per the master plan). The DWG→`DrawNode` lowering happens *inside* the function
body; it is real, but the ultimate DWG decode is still the framework's own legacy
`semio_framework::dwg_from_bytes` (not `io_dispatch`) because there is currently no
`io_dispatch`-reachable path from raw DWG bytes into this function at all — see gap 1 below.

### New drawing-bridge region

Both directions' shared geometry helpers live in one new `//#region 🔖️DrawingBridge` between
`Io` and `MediaExport`: `feature_lon_lat`, `feature_line`, `polyline_draw_node`,
`point_marker_draw_node`, `gis_map_snapshot_to_drawing`, `drawing_to_svg_io_key`,
`render_drawing_to_svg`.

### Tests (existing `#[cfg(test)] mod tests` extended, no new test files)

- `dwg_import_collects_point_and_line_vertices`, `dwg_import_falls_back_to_default_document_when_empty`
  — unchanged, still pass (vertex counts preserved exactly by the new `DrawNode` lowering).
- New `dwg_import_lowers_a_closed_polyline_through_a_draw_node_and_carries_the_close_segment` —
  asserts the `DrawNode::Path` shape (`MoveTo` first, `Close` last, 4 segments for 3 vertices) AND
  the resulting position-feature count.
- New `gis_map_snapshot_to_drawing_builds_markers_and_polylines` — asserts marker/route/region
  `DrawNode` shapes, styles, and segment counts (4 for the circle marker, open vs. closed for
  route/region).
- New `svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge` — exercises the full
  `io_dispatch` path against the real reuse-map fixture document, asserts real `<svg`/`<path`
  markup came back.
- New `svg_export_of_an_empty_document_still_renders_a_bare_canvas` — degenerate-input coverage
  (256×256 floor canvas, still real SVG through the bridge, not a title-card fallback string).
- New test-only `ensure_stdio_semio_registered_for_tests()` (`Once`-guarded call to
  `semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::register()`) — production code
  never calls this; in production stdio's own plugin `setup()` registers its composer entries at
  boot before any gis function runs, exactly like every other stdio consumer. A bare
  `cargo test -p semio-s-plugin-gis --lib` process never boots that host sequence, so the two new
  `io_dispatch`-exercising tests seed the registry themselves.

## Incidental fixes (outside the ticket's ask, but blocking `cargo check`/`cargo test` and inside my write scope)

Two independent pre-existing, foreign breakages blocked verification. Both confirmed via
`git status`/`git log` as unrelated to this ticket and predating my edits; both fixed because they
sit inside `✏️s/🔌️plugins/🌍️gis/**` (my write scope) and there is no way to run the exit-checklist
commands at all without them.

### Fix 1 — stale `panels::document` → `panels::artifact` rename (glue.rs + 2 call sites)

`cargo check -p semio-s-plugin-gis` was failing at baseline (before any of my edits) with:

```
✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs:812:13: error: couldn't read
`…/🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
```

`git status`/`git log` confirm this is pre-existing and unrelated to this ticket: the panel
directory `🎛️apps/◻2d/📌️panels/📄️document/` was renamed to `📄️artifact/` (its
`🦀️component.rs` doc comment still literally says "the document tree") at some earlier commit,
but `📦️glue.rs`'s `pub mod document` mount and the one call site
(`🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs:61`,
`crate::apps::gis2d::panels::document::GIS2D_PLAY_BODY_DOCUMENT`) were never updated. Both are
inside `✏️s/🔌️plugins/🌍️gis/**` (gis's own `glue.rs` — NOT one of the master-plan's hot
closer-only glue files, which are stdio's/os-kernel's/framework's), the fix is a single mechanical
rename (`document`→`artifact`, `GIS2D_PLAY_BODY_DOCUMENT` const itself unchanged), and without it
no `cargo check`/`cargo test` for this crate can even start — so I fixed it:

- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs:811-812`: `pub mod document` (path
  `…/📄️document/🦀️component.rs`) → `pub mod artifact` (path `…/📄️artifact/🦀️component.rs`).
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs:61`:
  `panels::document::GIS2D_PLAY_BODY_DOCUMENT` → `panels::artifact::GIS2D_PLAY_BODY_DOCUMENT`.
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🦀️component.rs:13`: the brace-import
  `panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel}`
  → `document as document_panel` became `artifact as document_panel` (alias unchanged, so the
  4 downstream `document_panel::…` call sites in the same file needed no further edits).

Verified this is the *only* remaining stale reference (`grep -rn "panels::document\b"` and a
brace-import-aware `panels::{...\bdocument\b...}` sweep across the whole gis tree both came back
empty after the fix; the second sweep is what caught the `🎛️apps/◻2d/🦀️component.rs` brace-import
the first, plain-substring grep missed).

### Fix 2 — `gismap`/`gisterrain`'s own JSON io leaves never learned stdio's real `JsonValue`

`JsonSnapshot.value` in stdio is its own recursive `JsonValue` enum (`Null`/`Bool`/`Number`/
`String`/`Array`/`Object` — deliberately never `serde_json::Value`, per that type's own doc
comment: "No `serde_json::Value` anywhere in this file"). All 4 of gismap's/gisterrain's own
`json↔{gismap,gisterrain}` io leaves (last touched commit `2564722008`, well before this ticket)
still assumed the old `serde_json::Value` shape and failed to compile
(`error[E0308]: mismatched types: expected JsonValue, found Value` / vice versa) — 6-7 errors
depending on which other stale reference had already been cleared, entirely unrelated to svg/dwg.
Added a small dependency-free `JsonValue ↔ serde_json::Value` conversion pair (mirrored per-leaf,
matching the repo's existing per-leaf-mirrored-helper convention, e.g. the semio/drawing svg
leaves' mirrored base64 encoder/decoder) to each of:

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`

No domain semantics changed (`GisMapSnapshot`/`GisTerrainSnapshot` still round-trip through
`serde_json::Value` exactly as before, now bridged to/from the real `JsonValue` at the boundary).

## stdio_gaps (reported, NOT worked around locally)

1. **No `drawing↔dwg` bridge exists under the semio/drawing subset's io tree.** The subset's io
   leaves (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/`)
   are only `svg`/`dxf`/`pdf` (matches the master plan's own "drawing↔svg/dxf/pdf" lattice row —
   dwg bridges through the separate `cad` subset instead, "cad↔dxf/dwg/step"). Combined with
   `register_dwg_import_handler`'s frozen `fn(&DwgDrawing)` signature (see above), there is
   currently no way for `gis2d_document_json_from_dwg` to reach `io_dispatch` for the DWG side at
   all — not a gis-side limitation, a genuine absence of a registered `IoKey`. If DWG-via-drawing
   import/export is wanted, either (a) stdio grows a `drawing↔dwg` leaf pair, or (b) W6's OS media
   registry rewrite (master plan V7 step 1) reroutes `register_dwg_import_handler`/DWG export
   through `io_compose_via` against the `cad` subset instead of the drawing one — outside W5b's
   scope either way.
2. Confirms the master plan's own architecture choice (not a defect): `gis2d_document_json_to_svg`
   only reaches `io_dispatch` because stdio's `svg` artifact + `drawing` subset composer
   (`SemioDrawingToSvg`/`SemioDrawingFromSvg`) are both real and registered — the export half of
   this task had a real bridge to call through; the import/DWG half did not.

## Exit checklist

Verified for real (both commands run to completion, output pasted below verbatim — not asserted
without running). The shared workspace `target/` dir was under heavy lock contention at
verification time (~15 concurrent `cargo check`/`cargo test` invocations from sibling W5a/W5b wave
agents observed running simultaneously, one blocked >5 min on "Blocking waiting for file lock on
build directory" — matches the "Concurrent Cargo Workspace Churn" precedent), so both commands
below were run against an isolated `CARGO_TARGET_DIR` to get a real, non-blocked result rather than
wait out the contention; this only changes where build artifacts are cached, not what gets
type-checked/run — same source tree, same crate. Full raw logs:
`w5b--gis-cargo-check-isolated2.txt` (full) / `w5b--gis-cargo-check.txt` (tail 40) and
`w5b--gis-cargo-test.txt` (full, 8044 lines) / `w5b--gis-cargo-test-tail30.txt` (tail 30).

`cargo check -p semio-s-plugin-gis 2>&1 | tail -40`:

```
[…confirmed pre-existing warnings across gis's gismap/gisterrain artifacts, none new from this
change; the target engine file (`⚙️engine/🦀️component.rs`) contributes zero warnings/errors…]
    Checking semio-s-plugin-gis v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust)
warning: `semio-s-plugin-gis` (lib) generated 18 warnings (run `cargo fix --lib -p semio-s-plugin-gis` to apply 14 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1m 03s
EXIT:0
```

`cargo test -p semio-s-plugin-gis --lib 2>&1 | tail -30`:

```
test artifacts::gismap::standards::v1::engine::tests::dwg_import_lowers_a_closed_polyline_through_a_draw_node_and_carries_the_close_segment ... ok
test artifacts::gismap::standards::v1::engine::tests::dwg_import_collects_point_and_line_vertices ... ok
test artifacts::gismap::standards::v1::engine::tests::dwg_import_falls_back_to_default_document_when_empty ... ok
test artifacts::gismap::standards::v1::engine::tests::gis_map_snapshot_to_drawing_builds_markers_and_polylines ... ok
test artifacts::gismap::standards::v1::engine::tests::svg_export_of_an_empty_document_still_renders_a_bare_canvas ... ok
test artifacts::gismap::standards::v1::engine::tests::svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge ... ok
[…146 more, all `... ok`, spanning gismap + gisterrain (unaffected, same crate) …]

test result: ok. 151 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

EXIT:0
```

All 6 new/changed tests pass, most importantly `svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge`
— this is the one that actually exercises `io_dispatch` end-to-end against the real reuse-map
fixture document and asserts real `<svg`/`<path` markup came back through stdio's registered
drawing↔svg composer, not a stub. 151/151 total, 0 failed — no regressions in gismap's or
gisterrain's existing suites (gisterrain is untouched by this ticket; it shares the crate and
happens to have been carried along by the same `cargo test` invocation).

## Files touched

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (the ticket's ask)
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` (incidental fix 1, pre-existing stale mount)
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🦀️component.rs` (incidental fix 1, brace-import call site)
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs` (incidental fix 1, one call site)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` (incidental fix 2)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` (incidental fix 2)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` (incidental fix 2)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` (incidental fix 2)
