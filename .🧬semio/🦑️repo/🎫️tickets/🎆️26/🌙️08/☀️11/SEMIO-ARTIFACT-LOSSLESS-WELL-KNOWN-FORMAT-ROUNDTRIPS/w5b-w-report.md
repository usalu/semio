# W5b — 🖍️draw svg/dwg Pattern Extraction Report

Agent: W5b implementer, `🖍️draw` plugin. Write scope: `✏️s/🔌️plugins/🖍️draw/**` only (stdio read-only).

## Summary

Deleted `🖍️draw`'s entire ad-hoc SVG string builder and its ad-hoc DWG entity-walking codec from
the engine (`draw_document_to_svg`, `rgba_to_svg_color`, `fill_style_to_svg`,
`path_segments_to_svg_d`, `escape_svg_text`, `draw_path_segment_to_dwg`,
`dwg_path_segment_to_draw`, `draw_document_json_to_dwg_bytes`, `draw_document_json_from_dwg`).
Replaced the SVG path with a real bridge: `draw_document_to_semio_drawing()` builds a real
`SemioDrawingSnapshot` (stdio's `s.stdio.semio/v1/drawing` subset — canvas/styles/recursive
`DrawNode` layers) from draw's own domain document, then `draw_document_to_svg()` dispatches it
through stdio's real `semio/drawing↔svg` bridge via `semio_framework::io_dispatch` and decodes the
real `SvgSnapshot` result back into SVG markup via stdio's own `write_svg_xml`. No hand-rolled
SVG/DWG bytes remain in this plugin.

DWG has **no stdio_gaps workaround** — see below; that side of the ad-hoc writer was deleted
outright with no bridge-based replacement, per instructions (report the gap, don't patch it).

## Files touched (all within `✏️s/🔌️plugins/🖍️draw/**`)

| File | What changed |
|---|---|
| `🗿️artifacts/🖍️draw/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` | Deleted the ad-hoc `MediaExport` region outright; added `SemioDrawingBridge` region (`draw_document_to_semio_drawing`, `matrix_to_semio_transform`, `to_semio_path_segment`, `solid_fill_to_semio_rgba`, `intern_semio_style`, `decode_data_uri_bytes`, `semio_draw_node_from_scene_node`, `draw_document_to_svg`, `draw_document_json_to_svg`); `draw_vector_media` now consumes the `Result`-returning bridge; test region ported (see below) |
| `🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🎹️composer/🦀️component.rs` | `compose_export_svg` (the production `s.draw@1/* → s.stdio.svg@1.1/*` composer entry, reached from the OS media-export dispatch) rewritten to build a `SemioDrawingSnapshot` and call `io_dispatch` — replaces a call into the degenerate `📤️export/…/svg` leaf, which only ever wrapped this artifact's own DSL text disguised as SVG bytes |
| `🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs` | DWG import leaf: deleted its call into the now-removed `draw_document_json_from_dwg`; replaced with an honest degenerate stub (same shape as this subset's existing svg/pdf/png import siblings), documented as a stdio_gap |
| `🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`, `.../📤️export/🧵️serializers/…/json/…` | Pre-existing (foreign, already-landed) breakage: `JsonSnapshot.value` is now stdio's own lexeme-preserving `JsonValue` model, not `serde_json::Value`; these two leaves still called `serde_json::from_value`/`to_value` against it. Fixed by round-tripping through stdio's own real RFC8259 text codec (`parse_json_text`/`write_json_text`/`write_json_pretty`) instead of writing a duplicate structural converter |
| `📦️packages/🦀️rust/📦️glue.rs` | Pre-existing (foreign, already-landed) breakage: `#[path]` for `commands::document` pointed at `🎮️commands/📄️document/…`, a directory that no longer exists (renamed to `📄️artifact/` by the repo-wide document→artifact terminology rename, commit `c31024cc6c` — a raw string literal invisible to identifier-based codemods). Fixed the path string only; kept the Rust module name `document` since `🦀️component.rs` still imports `commands::document` by that name |
| `🎛️apps/🖍️draw/📌️panels/🗂️layers/🦀️component.rs` | Pre-existing (foreign, already-landed) breakage: referenced `FRAMEWORK_PANEL_TAB_DOCUMENT_ID`/`_LABEL`, renamed in the framework to `FRAMEWORK_PANEL_TAB_ARTIFACT_ID`/`_LABEL` by the same rename wave. Updated both references |

The glue.rs/panel/json fixes are **not** part of the SVG/DWG extraction task itself — they were
blocking `cargo check -p semio-s-plugin-draw` outright (the whole crate failed to compile before
touching anything SVG/DWG-related) and are lagging call-sites of already-landed foreign refactors
within my own write scope, so I completed them per the ground rules ("lagging call-sites of landed
foreign refactors may be completed"). None of them touch stdio.

## Deleted ad-hoc code (LOC)

`draw_document_to_svg` (old hand-rolled SVG string builder), `rgba_to_svg_color`,
`fill_style_to_svg`, `path_segments_to_svg_d`, `escape_svg_text`, `resolve_draw_document_artboard`
(kept, still legitimate domain geometry — reused by the new bridge), `apply_draw_transform_point`,
`draw_path_segment_to_dwg`, `dwg_path_segment_to_draw`, `draw_document_json_to_dwg_bytes`,
`draw_document_json_from_dwg` — **~170 lines of ad-hoc SVG/DWG codec deleted outright**, zero
feature flags, zero commented-out fallback. `git diff --stat` for the engine file: 366 lines
changed (roughly balanced insert/delete — the new bridge is comparable in size to what it
replaced, but every line is now real conversion logic into stdio's neutral types, not SVG/DWG byte
emission).

## Test coverage ported

- Deleted: `dwg_export_import_round_trips_a_path_and_text_layer`,
  `draw_document_json_to_dwg_bytes_errors_on_invalid_json_and_skips_invisible_layers`,
  `draw_document_json_from_dwg_falls_back_to_single_empty_layer_when_no_entities` — all three
  exercised the deleted ad-hoc DWG functions directly; there is no bridge-based replacement path
  for DWG (see stdio_gaps below), so nothing to port them onto.
- Ported: `draw_document_to_svg_renders_shape_text_image_and_gradient_nodes` →
  `draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing`. Same
  shape/text/image/gradient coverage (a solid-filled+stroked rect, a linear-gradient rect, a text
  node with `<a & b>` content to exercise escaping, an image node with a real base64 asset), but
  instead of substring-matching hand-rolled SVG markup (which no longer exists), it decodes the
  real bridged SVG text back into stdio's own typed `SvgElement` tree (`parse_svg_xml` +
  `svg_element_from_xml_node`, the same pair stdio's own drawing↔svg bridge tests use) and asserts
  on structure: filled rect → `Path` with `fill` starting `rgba(255,...)`; gradient rect → `Path`
  with no `fill` attribute at all (gradients are honestly dropped, not fabricated as `fill="none"`
  the way the old renderer did); text → `Text` node containing the exact string; image → `Unknown`
  element named `"image"` with a `data:image/png;base64,...` href. Also keeps the original invalid-JSON
  error-path assertion (`draw_document_json_to_svg` on malformed input still errs).

## stdio_gaps

1. **No `s.stdio.semio/v1/drawing ↔ dwg` bridge.** Confirmed by directly inspecting stdio's
   `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/` tree: only `svg` (1.1), `dxf` (r12), `pdf`
   (1.7) leaves exist, matching the master plan's own format lattice row
   (`drawing↔svg/dxf/pdf`). DWG only bridges from `s.stdio.semio/v1/cad`, standard `ac1024` — a
   different hub subset entirely, and `io_compose_via`'s max-2-hops invariant (hub resolvable
   directly from sources, target resolvable from the hub's own output alone) rules out chaining
   `drawing→…→cad→dwg`. draw's own `s.stdio.dwg@ac1018` dialect (a third, distinct DWG standard
   from `ac1024`) makes this doubly unreachable. This is a real gap against acceptance scenario
   (b) in the master plan ("draw → semio/drawing → svg AND dwg"), which currently cannot be
   satisfied for the dwg half without stdio growing a `drawing↔dwg` bridge (or draw switching its
   own DWG dialect to `ac1024` and going through `cad`, which would be a much bigger domain-model
   change than this ticket's scope). Draw's own DWG import leaf is now an honest degenerate stub
   (matches its svg/pdf/png siblings) until this lands.
2. **`DrawNode::Text` has no font-size field.** Draw's own `DrawTextBody.size` (font size) has
   nowhere to go in semio/drawing's `Text { value, at, style }` shape — `style` (`DrawStyle`) only
   carries fill/stroke/stroke-width/opacity, no typography. Font size is silently lost crossing the
   bridge (position `at` is preserved). Minor, but a real fidelity gap worth a follow-up subset field.
3. **`DrawStyle` has no `blend_mode`/`fill_rule`, and `Group`/`Image` nodes carry no opacity slot
   at all** (only `Path`/`Text` reference a style). Draw's per-layer blend mode, fill rule, and
   image/group opacity are all honestly dropped at the bridge, not fabricated.

None of these were worked around locally — they're reported here per the ground rules.

## Exit checklist

`cargo check -p semio-s-plugin-draw` — **PASSES**, full output in `w5b-w-draw-cargo-check.txt`:

```
warning: `semio-s-plugin-draw` (lib) generated 4 warnings (run `cargo fix --lib -p semio-s-plugin-draw` to apply 2 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1m 03s
```

(4 warnings are all pre-existing/unrelated: an unused `ArtifactBuilder` import and an elided-lifetime
lint in the `✳️any` subset composer, an unused glob re-export in `glue.rs`, and a dead `artifact`
field on `DrawEngine` — none touched by this ticket, none new.)

`cargo test -p semio-s-plugin-draw --lib` — **BLOCKED, confirmed foreign.** Full output in
`w5b-w-draw-cargo-test.txt`. Across 3 separate invocations (plus a `cargo check -p
semio-s-plugin-draw --tests` and a control run of `cargo test -p semio-s-plugin-stdio --lib`, which
depends on nothing in this ticket), the failure is always inside `semio-framework-os-kernel`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`,
`🔨️modules/📡️spr/🎮️command|🔀️crdt|🔗️causal|🧪️testkit/🦀️component.rs`) — **never** inside any
`✏️s/🔌️plugins/🖍️draw/**` file — and the specific error changes between consecutive runs (E0063
missing `MutationMeta` fields → E0308/E0277 `Clone` bound → E0405 unresolved name), which is only
possible if the source is being edited between invocations. `git status --porcelain` on those exact
files confirms they are currently modified (uncommitted) by another concurrent session:

```
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔀️crdt/🦀️component.rs
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔗️causal/🦀️component.rs
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/benches/protocol.rs
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs
M  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs
```

This is "Concurrent Cargo Workspace Churn" (matches prior repo history of this exact failure
class) — `🧰️framework/**` is entirely outside this ticket's write scope, not touched by this
agent, and I did not chase it further per the standing guidance to poll rather than chase. As a
direct consequence, **the new/ported test code (inside `#[cfg(test)] mod tests`) has not been
machine-verified by an actual compiler run** — `cargo check` (default profile) does not compile
`#[cfg(test)]` code, and every attempt to reach it (`cargo test`, `cargo check --tests`) hit this
same unrelated blocker. I did a careful manual line-by-line review of the new/ported test against
the exact `SvgElement`/`XmlAttr` shapes and match-ergonomics patterns already used (and presumably
compiling) in stdio's own `semio/drawing↔svg` bridge tests, and I'm confident in it, but this
should be re-run once the framework churn settles — a fresh `cargo test -p semio-s-plugin-draw
--lib` is the natural verification step for whichever agent closes this wave.

## Files in this ticket folder from this agent

- `w5b-w-report.md` (this file)
- `w5b-w-draw-cargo-check.txt` (verbatim `cargo check -p semio-s-plugin-draw` output, PASS)
- `w5b-w-draw-cargo-test.txt` (verbatim `cargo test -p semio-s-plugin-draw --lib` output, blocked by foreign framework churn — see above)
