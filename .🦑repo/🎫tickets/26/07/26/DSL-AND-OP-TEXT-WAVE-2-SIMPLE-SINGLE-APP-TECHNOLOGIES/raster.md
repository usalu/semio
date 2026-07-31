# raster — DSL + OpText (Wave 2)

## Design

- `.raster` DocumentDsl grammar (hand-rolled lexer: Word/Str/LBrace/RBrace/LBracket/RBracket/Eof),
  mirrors the `note`/`draw` pilot style (`note_text` module) — see `mod raster_text` in
  `raster/plugin/rs/lib.rs`, inserted inside `pub(crate) mod domain` right after `//#region 🔖Operations`.
- Document: `raster id=".." schema=".." [title=".."]` / `camera x=.. y=.. zoom=..` / optional
  `assets { key { mime=".." data=".." } }` / optional `layers { <layer>* }`.
- Layer common header: `id name visible opacity blend x y scaleX scaleY rotation` (transform flattened).
  - `pixel`: + optional `width`/`height`/`image=".."`, optional trailing `mask { enabled linked invert [width] [height] }`.
  - `group`: + optional trailing `mask { .. }`, then optional `{ <layer>* }` children.
  - `adjustment`: + `kind=".."`, optional trailing `params { key=value ... }`.
- `params` (`serde_json::Map<String, Value>`) uses a small recursive value grammar (number/bool/null/
  string/`[ ]` array/`{ key=value }` object) — no JSON-in-a-string escape hatch, so any adjustment kind's
  arbitrary params round-trip.
- Op-text (one line each): `add-layer index=N [parent=".."] <layer>`, `remove-layer id=".."`,
  `patch-layer id=".." [name=..] [visible=..] [opacity=..] [blend=..] [x=..] [y=..] [width=..] [height=..] [kind=..]`,
  `move-layer id=".." index=N [parent=".."]`, `set-camera x=.. y=.. zoom=..`,
  `replace-document <compact document>`.
- Dropped fixture-only fields not on the actual struct: JSON had `filters`/`clipToBelow` on some layers,
  and top-level `brushSize`/`brushOpacity` — none of these exist on `RasterLayerNode`/`RasterProjection`
  today (serde silently ignored them), so they are correctly absent from the DSL too.
- `SEMIO_EXAMPLE_JSON` renamed to `SEMIO_RASTER_EXAMPLE_TEXT` (`include_str!("../../example/semio.raster")`).
  Added `semio_example_document()` (parses DSL, falls back to `empty_raster_document()`) and
  `semio_example_json()` (JSON re-serialization for the framework-generic `App::example(..)` call site,
  which contractually needs JSON — out of scope in `framework/plugin`).
- `raster/example/semio.raster.json` deleted; `raster/example/semio.raster` handcrafted (generated once
  via a scratch python script from the old JSON to preserve the base64 emblem asset byte-for-byte, then
  hand-reviewed) — see `convert_raster_fixture.py` in the scratchpad (not part of the repo).

## Tests added (existing `//#region 🧪Tests` → `mod tests`, new `//#region 🔖DslAndOpText` subregion)

- `raster_dsl_round_trips_representative_document` — hand-built doc covering pixel+mask, nested
  group+group, adjustment with params (number/string/bool/null/array-of-arrays/nested object).
- `raster_dsl_round_trips_semio_example_document`.
- `raster_op_text_round_trips_every_variant` — one call per `RasterOperation` variant (AddLayer x2,
  RemoveLayer, PatchLayer x2, MoveLayer x2, SetCamera, ReplaceDocument).
- `raster_document_text_round_trips_store_with_applied_operation` — `DocumentVcsStore` with an
  `AddLayer` applied, `vcs::test_support::assert_document_text_round_trip`.

## Verification

- `cargo test -p raster-plugin --lib`: TBD (fill in once green).
- `cargo check -p raster-plugin --target wasm32-unknown-unknown`: TBD.

## Constraints honored

- Only touched files under `raster/`.
- No vcs/framework changes.
- No new crate deps (vcs + serde_json already deps of raster-plugin).
