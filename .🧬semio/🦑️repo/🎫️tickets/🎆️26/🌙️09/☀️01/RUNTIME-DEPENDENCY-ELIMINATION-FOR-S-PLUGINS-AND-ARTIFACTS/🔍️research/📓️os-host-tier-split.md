# 🖥️ `semio-framework-os` host tier split — native SVG engines removed from wasip2

## Headline

The native-host SVG engine tier is no longer resolved for `wasm32-wasip2`:

```text
png       → nothing to print
resvg     → nothing to print
tiny-skia → nothing to print
usvg      → nothing to print
rustybuzz → nothing to print
```

Each result is from `cargo tree -p semio-framework-os --target wasm32-wasip2 -i <crate>` after
the change. `png`, `resvg`, `tiny-skia`, and `usvg` now live exclusively in:

```toml
[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]
```

The public API remains present on both targets. Lightweight SVG builders and DWG-to-SVG stay real
and target-neutral; the two operations that require the native SVG engine preserve their signatures
and return a precise `Err` in the wasip2 guest.

## Shared-checkout state and exact plugin counts

Before this source/Cargo change, both plugin manifests **already** contained their correct native-host
target dependency table in this shared checkout. `git diff` showed no local change to either manifest,
so this pass deliberately did not overwrite another worker's completed/concurrent manifest split:

```toml
[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]
semio-framework-os = { path = ".../🖥️host/📦️packages/🦀️rust", package = "semio-framework-os" }
```

That is the required puzzle split as well as animate's split. It removes the entire host crate from
both shipped component dependency graphs; the host-internal split is additionally important because
it makes the host crate itself valid without the native renderer tier when another wasip2 consumer
does depend on it.

Measured with the ticket's exact command before and after this pass:

```text
                                      Before host source split    After
semio-s-plugin-animate                             40              40
semio-s-plugin-puzzle                              67              67
```

The unchanged values are expected: the manifest guards were already active before this pass, so
`semio-framework-os` was already absent from these two component trees. The earlier ticket evidence
records the historical host-edge baseline as animate **88** and puzzle approximately **104–111**;
the pre-existing manifest split is what removed that edge before this host-internal hardening pass.

Neither plugin is at zero overall yet. Animate's remaining 40 crates are the `serde` and
`wit-bindgen`/`wasm-encoder`/`wasmparser`/`wit-component` families. Puzzle's 67 additionally include
its separate `vello`/`png`/font/image family. These are not reached through `semio-framework-os`:
`cargo tree -i rustybuzz` is clean for both plugins, and the exact host subtree crates above are clean.

## Per-symbol classification (`🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`)

The renderer-dependent public surface is the `media_export_raster` module, re-exported at crate root.
The host's unrelated public OS/workflow/registry APIs were not part of the `resvg`/`usvg` dependency
edge and are therefore not recategorized here.

| Symbol | Classification | Evidence |
|---|---|---|
| `media_accept_filter_kinds`, `OsMediaExportResult`, handler registration APIs | target-neutral | They operate on format metadata, JSON/bytes, and handler closures. No `resvg`, `usvg`, `tiny_skia`, or `png` type appears in their public signatures or bodies. |
| `dwg_drawing_to_svg` | target-neutral | It formats the first-party `DwgDrawing` polyline geometry into a `String`; it parses or rasterizes nothing and names none of the native renderer crates. |
| `Svg2dDocumentRenderer`, `register_2d_export_handlers` | target-neutral facade | The signature and registration remain valid on both targets. Its PNG/DWG closures call the stable functions below, which select the native implementation or the explicit wasip2 error. |
| Mesh registrar APIs | target-neutral | They delegate to first-party `MeshExporter`/`MeshImporter` and stdio DWG codecs, not the SVG renderer stack. |
| `wrap_svg`, `title_card_svg`, `pages_rects_svg`, `map_points_svg` | target-neutral | These public `media_export_simple` helpers only construct and escape SVG text from `serde_json::Value`. |
| `rasterize_svg_to_png_base64`, `encode_rgba_png` | **native SVG engine** | The real implementation calls `usvg::Tree`, `resvg::render`, `tiny_skia::Pixmap`/`Transform`, and `png::Encoder`. |
| `svg_to_dwg_bytes`, `collect_svg_children`, `collect_svg_path`, `transformed_svg_point`, `flatten_quad_into`, `flatten_cubic_into`, `flush_svg_polyline` | **native SVG engine** | This pipeline parses `usvg` nodes and `tiny_skia_path` segments before flattening curves to DWG polylines. |

`rasterize_svg_to_png_base64` and `svg_to_dwg_bytes` now each have two implementations with the
same public signature. Native targets retain the former engine verbatim. On wasip2 they return,
respectively, `SVG rasterization requires the native semio-framework-os host` and
`SVG-to-DWG conversion requires the native semio-framework-os host`. The wasip2-only unit test
asserts both errors; the existing native `svg_to_dwg_round_trip_produces_a_polyline` test remains
native-only because it exercises the real `usvg` parser and SVG geometry flattening.

## Are animate's three helpers guest-command reachable? No

The source trace differs materially from `raster-tier-split.md`'s
`export-video-from-deck` finding.

1. `animate_present_document_json_to_svg` calls `title_card_svg`; `animate_present_document_json_from_dwg`
   calls `dwg_drawing_to_svg` and `rasterize_svg_to_png_base64`. A repo-wide `rg` found no production
   caller for either animate helper outside its own definition and unit tests.
2. The present subset installs `io::io()` in
   `🗿️artifacts/🎬️present/…/🪆️subsets/✳️any/🦀️component.rs`. Its `entries()` registers only JSON,
   Markdown, PDF, PPTX, SVG, PNG, and text serializer/deserializer leaves. It does not register either
   helper, `title_card_svg`, DWG import, or a host media handler.
3. `AnimatePresentPlayApp::io()` returns the separate `present_io()` declaration. Its exported and
   imported format lists are empty; it declares only the `frames:in` raster port. It likewise does not
   reference either helper.
4. `AnimatePresentPlayApp::handle` has exactly one direct asynchronous arm:
   `PresentCommand::ExportVideoFromDeck(payload) => export_video_from_deck::handle_async(payload).await`.
   The remaining commands use the generated synchronous command table. `rg` finds neither helper nor
   any of the three host functions in that `Editor::handle` route or its command modules.
5. The component guest does have a real async dispatch path — reactor `poll` → `plugin_exchange` →
   `ArtifactApp::handle` — but no command or registered I/O leaf invokes these helpers.

**Conclusion:** unlike the video renderer, none of `title_card_svg`, `dwg_drawing_to_svg`, or
`rasterize_svg_to_png_base64` is currently reachable from animate's wasip2 guest command dispatch.
The host-only dependency guard is therefore behavior-preserving for the shipped animate component;
the stable wasip2 error APIs protect direct future consumers without reintroducing the renderer stack.

## Verification

- `cargo tree -p semio-framework-os --target wasm32-wasip2 -i {png,resvg,tiny-skia,usvg,rustybuzz}`
  (run one crate at a time) — all five report `warning: nothing to print.`
- Exact third-party count command — animate: **40 → 40**; puzzle: **67 → 67**, for the reason stated
  above.
- `cargo tree -p semio-s-plugin-animate --target wasm32-wasip2 -i rustybuzz` and the puzzle equivalent
  — both report `warning: nothing to print.`
- `git diff --check` for the two host files — clean.
- `cargo check -p semio-framework-os --target wasm32-wasip2 --message-format=short` reached
  `semio-framework-replication` but stopped before compiling the host crate on two pre-existing
  `E0119` errors in `🧰️framework/🔨️modules/🌱️value/🦀️component.rs`: conflicting
  `Serialize` and `Deserialize` implementations for `DslValue`. No diagnostic named either changed
  host file or its SVG engine symbols. This is not recorded as a passing build. Earlier queued retries
  were cancelled while other workspace builds held Cargo locks; the final run supplied the precise
  unrelated blocker above.

## Files changed

- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `🔍️research/📓️os-host-tier-split.md`
