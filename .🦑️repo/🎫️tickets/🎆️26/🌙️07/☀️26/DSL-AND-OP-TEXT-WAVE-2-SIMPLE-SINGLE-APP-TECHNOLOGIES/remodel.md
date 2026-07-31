# remodel — DSL + OpText scratch notes

## Files touched
- remodel/rs/lib.rs
  - added `use vcs::{DocumentDsl, Operation, OperationDiff, OpText, TextError, TextSpan};`
  - added `//#region 🔖️Dsl` (private `mod remodel_text` hand-rolled lexer/parser/printer + `impl DocumentDsl for RemodelScene`, EXTENSION = "remodel")
  - added `//#region 🔖️OpText` (`impl OpText for RemodelOperation`)
  - extended `//#region 🧪️Tests` with `//#region 🔖️DslAndOpText`: `populated_scene_fixture()` helper (also now reused by the pre-existing `populated_scene_roundtrips_through_json`), `default_scene_roundtrips_through_dsl`, `populated_scene_roundtrips_through_dsl`, `every_operation_variant_roundtrips_through_op_text` (all 20 variants incl. None/Some branches), `store_roundtrips_through_document_text` (DocumentVcsStore + assert_document_text_round_trip)
- remodel/plugin/rs/lib.rs
  - added `use vcs::DocumentDsl;`
  - `create_remodel_app()`: `default_example` now built via `default_remodel_scene().print_dsl()` instead of `serde_json::to_string(...)`

## Grammar summary
Whitespace-insignificant, hand-rolled Word/Str/`{`/`}` lexer (same shape as note/draw pilots).
Top level: `remodel schema=<w> id=<w> { stream* asset* calibration{} gcp* params{} job{} results{} }`
- Dash `-` sentinel = None for every `Option<T>` (scalars AND whole optional structs: sparse/dense/trajectory/geoProducts/qc all "dash-or-fields").
- `MeshData` numeric buffers always base64-packed via existing `PackedF32`/`PackedU8` plus new local `pack_u32`/`unpack_u32` (no per-element text, ever).
- Op-text verbs are camelCase set-variant names (`setStreams`, `setAsset`, ... `setQc`), each one line, reusing the same construct print/parse functions as the DSL (grammar is identical, just flattened onto one line via dedicated `print_X_fields` helpers instead of the DSL's `print_X` keyword-prefixed wrappers).

## Verification
- `cargo check -p remodel_document --lib` — running via isolated CARGO_TARGET_DIR (shared target/ was busy).
- `cargo test -p remodel_document --lib` — pending.
- `cargo test -p remodel-plugin --lib` — pending.
- `cargo check -p remodel-plugin --target wasm32-unknown-unknown` — pending.
