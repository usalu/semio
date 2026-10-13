# 2026-10-12 stdio editor fixes: png, step/ifc print, pptx details, pdf17

Executor: stdio-fix (Sonnet). All cargo ran in the foreground through `🚦️cargo-slot.sh` with `SLOTS=2` and the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/stdio-fix/target` (deleted at the end; logs stay in `play-fleet/stdio-fix/`). One process per test for the hub law (`each.py`, `RUST_MIN_STACK=67108864`, 300 s alarm). No git-modifying command, no ticket operation, no sub-agent. Every edit was a complete, compiling file change.

## Result

hub `semio-hub-stdio` `editor_catalog` (177 tests, one process per test): **69 -> 88 passed** (SQLite laws 68 -> 87 of 88; the 88th, `sqlite_snapshot_semio_any_editor`, is the known debug-build 300 s timeout; plus the neutral catalog law). The 89 failures:

| count | cause | owner |
| --- | --- | --- |
| 60 | `settle_registered_typed_operation` runs into its 30 s deadline (png, pptx, pdf14/17, step, ifc, gltf, json, md, csv, bmp, ... every editor that reaches the publication step) | plugin/store (see 5) |
| 17 | semio subsets: `host-addressed detail action ... unsupported artifact publication lane` (their snapshot trees lack `RetainedClone`, see 4) | stdio-semio |
| 11 | `needs a meaningful edit fixture` (docx x3, las, svg x3, tiff x2, xml x2) | editor owners |
| 1 | `sqlite_snapshot_semio_any_editor` 300 s debug timeout | none |

Every task below now passes everything that precedes the shared settle step.

## 1. png: "every PNG mutation has a direct binary owner"

Root cause (verified): the `ReplaceImage` variant of `PngMutation` (binary tag 12 in `📡️.protocol.semio` and `🥋️.ksy`, `binaryTag: 12` in its leaf `🔣️.json`) had NO direct leaf in either registry: `🚪️io/💾️binary/🧬️mutations` (panic "direct binary owner") and `🚪️io/📝️text/🧬️mutations` (panic "direct text owner", seen in the png lib test `schema_owned_png_replacement_diff_and_inverse_...`). The editor emits `ReplaceImage` for every details edit outside `/image/gamma` and a single sample, so `mutation.encode_op()`/`print_op()` panicked on the first png edit. It is not a consequence of the bulk-clone/paged-validation work (#53-#55).

Fix (`📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any`): new leaves `🚪️io/💾️binary/🧬️mutations/🔄️replace-image` and `🚪️io/📝️text/🧬️mutations/🔄️replace-image` registered in both `REGISTRY`s (same payload shape as the other png leaves: JSON of the payload, text `replace-image payload=<hex>`); text grammar rule `replace-image-op` now uses `payload` like every sibling (it said `image`), `.ebnf`/`.g4`/`.abnf`/`.spicy` list the opcode and tag 12; `replace_image::test_case()` joins `demo_mutation_cases()` (so `protocol_walk_law` and the grammar laws cover it). Fixture `🌎️hub/🧩️compositions/🗄️stdio/🧫️fixtures/✏️editor-catalog/🔣️.json` got the png entry (`/image/interlace` -> `true`, which takes the `replace-image` route).

Proof: png `--lib` 68/77 -> **71/78** (new law `every_mutation_kind_has_a_direct_binary_and_text_owner_that_roundtrips` and `png_mutation_inverse_sum_law_holds_for_every_leaf` pass; the text-owner failure is gone); `preparation_law` **4/4** (ReplaceImage and paint through the real sealer: post snapshot equals `apply_mutation`, inverse restores the base); hub `png_editor` now passes the fixture lookup, command replay, binary and text mutation round trips, inverse and the localized details render and stops only at the shared 30 s settle (5). The Apply/Undo/Redo through the registered app (`history_edits_end_to_end`) still ends in the K1 close-ladder witness ("registered fixture did not reach its exact terminal-empty witness"), so it is not provable through the app harness yet. Remaining png lib failures (7): K1 witness (3), `diff_grammar_conformance_law` (`PngDiff` prints `rects=[ ]`, the diff grammar rejects it), two retained-publication fuel/profile tests.

## 2. step/ifc `print_dsl`: "native encode has no available explicit retirement slot"

Root cause: the three `print_text` of step and ifc (`📐️step/.../🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native`, `🏗️ifc/.../4️⃣4/...` and `.../🔖️2x3/.../🧱️base/...`) are original callers of a native encode but built a bare `NativeEncodeControl` (and `SqliteSnapshotControl::new`) without a retirement recipient (the encode twin of #66).

Fix, generic: new `semio_s_artifact_stdio_contract::native_print::print_as_original_caller(limits, |control, owner| ...)` (`📇️registry/🧬️contract/🖨️native-print`): installs the empty `NativeEncodeRetirementRecipient`, runs the operation under `SqliteSnapshotControl::run_as_original_caller` (SQL recipient), then closes the native recipient from the full five-axis grant (forgotten rather than aborting if it stalls). The three `print_text` now call it.

Second layer behind it: `StepValue`/`IfcValue` had a hand-written `RetireOwned` without `controlled_retirement_supported`, so the cold `close::<T>` refused ("typed owner has no controlled retirement authority"); both now `#[derive(RetireOwned)]` and the hand impls are deleted.

Third layer for the editor tests: 30 editors (step x7, ifc x5, gltf, semio x17) used `snapshot_editing_bounded_first_step_tool_proofs!` without the document lane's publication authority, so every details action failed with `interactive-job.publication-authority-missing`. New macro `snapshot_editing_publication_authority!()` (contract `editing`) = the generic `mutation_apply_preparation_factory`; added to step, ifc and gltf. step/ifc/gltf needed `RetainedClone` on their snapshot trees (derive added to every snapshot type with `RetireOwned`, and to the shared `Part21Document/Header/Instance/Decimal`). The 17 semio subsets were tried and reverted: their snapshot trees lack `RetainedClone` and need their owner (17 failing editor tests, listed above).

Proof: hub `sqlite_snapshot_step_*` 7/7 and `sqlite_snapshot_ifc*` 5/5 pass (were 0/12); step `--lib` 212 -> **258/294**, ifc `--lib` 189 -> **231/244** (whole-process run, `RUST_MIN_STACK=256 MiB`). The step/ifc/gltf editor tests now reach the shared settle (5).

## 3. pptx x3: "detail controls do not render"

Root cause (verified with row-level instrumentation, removed again): the `Details` first paint ends the root window after the `Schema` row. `tree_window_indexed_rows` prices a row by its whole subtree; the pptx `opc` row costs 3301 reconcile items against 3290 left, because its nested default-open windows (which spend the shared budget row by row) consume the whole remaining item budget and leave nothing for the enclosing row's own controls. The row is dropped, and with it every later row, so no control binding is rendered (xlsx, which has the same shape, already lost its third root row this way).

Fix in the framework window ledger (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `tree_window_indexed_rows` + `TREE_WINDOW_ROW_OWN_ITEMS = 512`): while a row is being built, the windows nested in it may spend only `items - min(items / 4, 512)`; after the row is built the exact subtree price is charged as before, so flat windows (table kit, Home rows) are unchanged. Law: contract unit test `a_nested_window_leaves_its_enclosing_row_its_own_price` (nested heavy rows + parent overhead + trailing row; before the fix: parent cost 3381 > 3328 items, 0 rows; after: both rows). Plugin lib tree-window/table-kit/Home-row laws: 43/44 (the one failure, `tool_job_outcome_original_pages_close_under_independent_common_grants`, is the pre-existing ABORT of r14/r15). Contract `editing::` tests 70/72 (the 2 failures are the known unrelated `patch`/`rules` ones).

Proof: hub `pptx_editor`, `pptx_strict_editor`, `pptx_transitional_editor` now pass the details render and stop at the shared settle (5).

## 4. pdf17: SQLite decode "exceeded declared stage workload" and the tool-proof failure

Not product-code absence, three independent stale-by-one-field drifts of the 32-field `PdfSnapshot` (`admittedStreamRoles` was added as field 32):

- `from_record_controlled` (`🚪️io/📝️text/📸️snapshot`) and `to_value_controlled` (`🧬️schema/📸️snapshot`) declared `begin_stage(31)` / `object_encoding_controlled(31)` and then stepped 32 times -> "native decoding exceeded declared stage workload" (7 hub tests). Now 32; fixture `🛫️encoding.json` `fieldCount` 31 -> 32.
- `write_dictionary` (`🧩️cos`) inserted a `pdf_cos_value` row one cell short of the 13 columns -> "SQLite row column count" (hub + 18 lib tests). Fixed.
- Tool proof (7 hub tests, "tool factory proof rejected tool 'set-mesh-data'"): the 7 `pdf17` editor rosters (`snapshot_editing_command_roster!` + `tools:`) still named `set-mesh-data`, the page module's action is `set-mesh-reference`. Roster fixed in base, a, e, h, ua, vt, x.

Test debt behind them: `🧫️fixtures/🛂️semantic/📖️full` golden regenerated from the independent `bun:sqlite` reader of the real export (rows 1280 -> 1289, bytes 336428 -> 337504, +artifact references/samples/glyph rows, -image codec rows) and `Lang` of the UA profile is `PdfObject::Text("und")` (was `Str`).

Proof: pdf `--lib` sqlite subset 20/40 -> **40/40**; whole pdf `--lib` 905 -> **930/1018**; zero tool-proof failures left; hub `sqlite_snapshot_pdf17_*` 7/7 pass; the 7 `pdf17_*_editor` tests now reach the shared settle (5).

## 5. Shared blocker found: `settle_registered_typed_operation` 30 s

60 hub editor tests (all editors that survive their own laws) end with "registered fixture typed operation did not retire within 30 seconds". A `sample` of a running `json_any_editor` shows the test thread busy inside `PluginApp::advance_typed_operation_publication` (memmove), i.e. a progress/performance problem of the publication driver in the plugin fixture harness, not an idle deadlock. Owner: plugin/store (pl-tests), not changed here.

## Commands and results

- hub `editor_catalog`: before 69/177, after **88/177** (table above).
- png `--lib` + `preparation_law`: 71/78 + 4/4. pdf `--lib`: 930/1018. step `--lib`: 258/294. ifc `--lib`: 231/244. contract `editing::`: 70/72. plugin tree-window subset: 43/44.
- Native `--lib` build of everything above (via the hub test build) and `cargo check --target wasm32-wasip2 --lib` for contract, step, ifc, gltf, png, pdf and `semio-framework-plugin` (hub workspace): Finished, 0 errors.

## Files touched

- png: `🚪️io/💾️binary/🧬️mutations/{🦀️.rs,🔠️.abnf,🌶️.spicy,🔄️replace-image/🦀️.rs}`, `🚪️io/📝️text/🧬️mutations/{🦀️.rs,📖️.grammar.semio,🔤️.ebnf,🅰️.g4,🔄️replace-image/🦀️.rs}`, `🧬️schema/🧬️mutations/{🦀️.rs,🔄️replace-image/🦀️.rs}`, `🚪️io/🧪️tests/🔬️codec/🦀️.rs`.
- contract: `🖨️native-print/🦀️.rs` (new), `🦀️.rs`, `✏️editing/🦀️.rs` (macro), `✏️editing/🪟️details/🧪️tests/🔬️unit/🦀️.rs`, `📐️part21/🦀️.rs`.
- step: `🧬️schema/📸️snapshot/🦀️.rs`, `🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs`, 7 `✏️editor/🦀️.rs`. ifc: both snapshot files, both `🚦️native/🦀️.rs`, 5 `✏️editor/🦀️.rs`. gltf: snapshot + `✏️editor/🦀️.rs`.
- pdf17 base: `🧬️schema/📸️snapshot/🦀️.rs`, `🚪️io/📝️text/📸️snapshot/🦀️.rs`, `🚪️io/🪶️sqlite/📸️snapshot/{🧩️cos/🦀️.rs,🦀️.rs,🧫️fixtures/🛂️semantic/📖️full/🔣️.json}`, `🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧫️fixtures/🛫️encoding.json`, 7 subset `✏️editor/🦀️.rs`.
- framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (window ledger).
- hub: `🌎️hub/🧩️compositions/🗄️stdio/🧫️fixtures/✏️editor-catalog/🔣️.json` (png entry).
