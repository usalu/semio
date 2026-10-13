# 2026-10-12 editor catalog complete: 177 of 177

Executor: edfix (Sonnet). Cargo ran in the foreground of its own slot through `🚦️cargo-slot.sh` (`SLOTS=2`) with the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/edfix/target` (deleted at the end, together with the 2.6 GB test binaries; logs and result tables stay in `⚡️cache/edfix/`). The hub law ran one process per test (`each-catalog.sh`, 4 parallel, 300 s alarm, `RUST_MIN_STACK=67108864`). No git-modifying command, no ticket operation, no sub-agent. Every edit was a complete, compiling file change.

## Result

hub `semio-hub-stdio` `editor_catalog`, 177 tests, one process each:

| | before | after |
| --- | --- | --- |
| ok | 160 | **177** |
| FAILED | 16 | 0 |
| TIMEOUT | 1 | 0 |

Before: 11 `needs a meaningful edit fixture`, 5 `snapshot-edit.lossy-conversion`, and the 300 s `sqlite_snapshot_semio_any_editor`. After: the slowest tests are the pdf17 editors (55 to 96 s in debug, 4 tests in parallel); `sqlite_snapshot_semio_any_editor` takes 22 s in the catalog run (29 s alone).

## 1. The 11 editors without an edit fixture

Each editor now has a fixture row in `🧫️fixtures/✏️editor-catalog/🔣️.json` (`snapshotEdits`; the law's two supported shapes, `setSnapshotValue` and `insertSnapshotValue`, were enough, so the law needed no new action). Three editors could not be edited through the generic details pane, which was a product fault, not a fixture gap.

| editors | edit | note |
| --- | --- | --- |
| svg `*`, basic, tiny | insert `/doc/root/children/-` = `<desc>Edited</desc>` | fixture only |
| las | set `/header/systemIdentifier` = `Edited` | needed the no-op fix below |
| tiff `*` | insert `/ifds/0/entries/-` = Artist tag 315 `Edited` | fixture only |
| tiff baseline | set `/ifds/0/entries/2/values` = bits per sample `[16,16,16]` | the baseline vocabulary is two computed gestures; needed the no-op fix below. The result is a soft baseline-conformance finding by design (`unsupported_bits_per_sample_is_flagged_soft`) |
| xml `*` | insert `/doc/root/children/-` = text `Edited` | product fix: the initial document had no root |
| xml valid | set `/doc/declaration/standalone` = `true` | product fix: the blank document had no declaration |
| docx `*`, strict, transitional | set `/opc/contentTypes/defaults/1/contentType` = `text/xml` | product fix: the vocabulary spoke tuples |

Product fixes behind them:

- **xml any initial document.** `XmlAnyEditor::initial_snapshot()` was `XmlSnapshot::default()`, a document without a root element, so no detail path existed and nothing could be inserted. New `XmlSnapshot::with_empty_root()` (a well-formed `<root/>`, like the valid subset's blank) is now the initial snapshot of the xml any editor and viewer.
- **xml valid blank document.** `blank_valid_xml_snapshot()` now carries `<?xml version="1.0" encoding="UTF-8" standalone="no"?>`. The valid subset refuses every edit that lands on an invalid document, and its vocabulary (rename-document-element, set-standalone, declare-entity, subsets, text) had no gesture reachable from `<!DOCTYPE root><root/>` through a path that exists in the snapshot JSON.
- **docx content-type rows.** The retained OPC snapshot spells a row as `{name, contentType}`; the details vocabulary (`content_type_edit`) still decoded and edited `(String, String)` tuples (`defaults/0/1`), so editing the row the details pane shows was refused (`no kind edits`) and an inserted row in the visible shape failed to decode. It now owns a `ContentTypeRow` object (`deny_unknown_fields`) and addresses `/contentType` and `/name`. xlsx/pptx keep tuples in their snapshots, which is why their fixtures read `defaults/0/0`.
- **Unchanged value must publish nothing** (the law's own rule). The las header/VLR special and the tiff baseline special emitted a mutation for an unchanged field. Both now compare the decoded edit with the current value and return no mutation, as the xml and docx specials already did.

## 2. The 5 lossy reopens (jpg any/baseline, pdf17 ua/vt/x)

Diagnosis, by decoding the editable source and walking it against its own typed re-encoding: **the round trip loses nothing and JPEG lossiness is not involved.** The first divergence of every failure is an octet string:

- jpg: `/image/pixels: Array([UInt(255) x4]) vs Bytes([255,255,255,255])`
- pdf17 ua: `/objects/0/value/value/4/value/value: Array([117,110,100]) vs Bytes([117,110,100])` (the `Lang` text `und`)

`Vec<u8>` encodes as `DslValue::Bytes`; JSON can only spell it as an array of numbers, so the source parser yields `Array`. The contract's `values_equivalent` (used by `snapshot_from_edit_source` and by the patch `equivalent_at`) treated those as different and raised `snapshot-edit.lossy-conversion`. pdf17 base/a/e/h pass only because their initial documents hold no octet string. So the law's expectation was right and the contract predicate was wrong: no per-format lossiness declaration was needed, and none was added.

Fix (`📇️registry/🧬️contract/✏️editing/🦀️.rs`): `values_equivalent` equates `Bytes` with the array of its octets, element-wise through the same exact-number rule (so `257` or `1.5` do not match). Law: `an_octet_string_equals_the_json_array_of_its_octets` over `🧫️fixtures/🟰️octet-equivalence/🔣️.json` (6 cases, both argument orders, cross-checked against `serde_json` as the independent oracle): 1 passed.

The law now also names the first typed divergence in its own failure message (`first_divergence`), which is how this was found.

## 3. `sqlite_snapshot_semio_any_editor` (debug timeout)

Timing (law instrumented, since removed): export 1.0 s, **import 580 s** for the binary payload, against 14 s for the brep subset alone and 20 ms for table. Sampling (`sample`, 10 ms) of the brep import: 88 percent of the test thread was under `StageOwner::retire_workspace` (closing the SQLite import workspace); of the active samples, `factory_ticket_demands` (quoting the four currencies of the next retirement turn) was **80 percent** and the actual `close_factory_ticket` step 8 percent. Top-of-stack: `PagedList<Box<dyn RetirementCursor>>::{leaf,get,height,page_items}` plus `Leaf<bool>`/`Deferred<bool>` cursors.

Two causes, both in the retirement of the schema-validation scratch (`SchemaValidationStorage`, one per subset reconstruction, thousands of tokens for the 183 tables of the any snapshot):

1. Every scalar field of a `Token`, `Column`, `Table` and `(usize, usize)` group was its own deferred cursor (about 29 turns per token). A token owns one allocation, its text.
2. Each retirement turn quoted the next demand 16 times (four axis calls each deriving all four) plus once more for the depth admission. The `ErasedSnapshotRetirement::next_demand` doc already says why one pass is required.

Fix:

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🏛️validation/🦀️.rs`: `Token` retires through the single byte cursor of its text (`retirement::bytes`); `Column`, `Table` and the new `Group` are `Copy` leaves (`artifact_retire_leaf!`), so a `Vec` of them drains in bulk.
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs`: `factory_ticket_demands` asks `next_demand` once.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`: the boxed close step checks only the depth axis (`artifact_retirement_box_depth`) instead of quoting all four.

Result in the same test, same loaded machine: import **580 s to 10.9 s** (binary payload) and 8.2 s (text payload); 199,569 retirement turns for the big import (3.2 s quoting demands, 6.3 s stepping); whole test 29 s alone, 22 s in the catalog run. The three changes were landed together, so their individual shares are not separated. The pre-fix turn count was not recorded.

Not done, and a larger follow-up: the same per-scalar-cursor cost applies to every `derive(RetireOwned)` struct of plain fields. A derive that sets `retirement_element_copy_bytes` for all-plain structs would collapse those turns across the whole codebase.

## Verification run

- hub `editor_catalog`: 160/16/1 before, 177 ok after (one process per test, above).
- Before the final build, in isolation with an earlier binary: svg x3, las, tiff x2, xml x2, jpg x2, docx x3, pdf17 ua/vt/x, `sqlite_snapshot_semio_any_editor` each passed. The final 177 run is the authoritative one.
- `cargo check --lib`, native and `--target wasm32-wasip2`, all exit 0: `semio-s-artifact-stdio-contract`, `-xml`, `-las`, `-tiff`, `-docx`, and `semio-framework-value` + `semio-framework-os-kernel` + `semio-framework-io-sqlite-snapshot` (the `semio-hub-stdio` change is the test file only).
- contract `an_octet_string...`: 1/1. docx `edit_rules` unit tests: 6/7 (the content-type row law passes; the one failure, `children_insert_and_remove_under_the_parents_address`, replays `InsertXmlNode`/`RemoveXmlNode`, which this work did not touch; not run on a pristine tree).
- `semio-framework-value --lib`: 307/307. `semio-framework-io-sqlite-snapshot --lib`: 54/54. os-kernel `--lib sqlite_snapshot`: 57/58 (`sqlite_snapshot_native_schema_lazy_recursive_physical_text_never_materializes_ordinary_metadata`, a DSL producer test on a shared static counter; unrelated to retirement; not run on a pristine tree).
- xml `--lib`: 93/94. The failure, `set_standalone_is_exact_in_every_declaration_combination`, parses literal declaration-less sources, so it does not touch the blank document: `SetStandalone::inverse` returns no step when the base has no declaration, hence undo cannot remove the declaration the forward step created. Pre-existing logic fault, left open (needs a declaration-removing leaf in the valid vocabulary).
- las `--lib`: 53/54 (`sqlite_snapshot_las_cohort_actual_typed_files_preserve_full_domain`: "owning test artifact directory", environment). tiff `--lib` does not compile (stale tests reference `tiffAnyEditor_retained_reduce`, `crate::editor`), also with `component-app-assembly`; not part of this work.

## Hazards met

- A peer's half-applied csv admission edit (`control` removed, still used, `🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs:17`) blocked the hub build for about 40 minutes; three automatic retries got through once it settled. The `unused_qualifications` sweep rewrote `std::mem::size_of` in files this work also touched, so their `git diff` is larger than this work.
- A cold hub test build took 18 to 28 minutes under the fleet; cargo slot waits added up to 9 minutes each.

## Files touched

- hub law and fixture: `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/✏️editor-catalog/🦀️.rs`, `🌎️hub/🧩️compositions/🗄️stdio/🧫️fixtures/✏️editor-catalog/🔣️.json` (+97 lines, 11 editors).
- contract: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`, `.../✏️editing/🧪️tests/🔬️unit/🦀️.rs`, new `.../🧫️fixtures/🟰️octet-equivalence/🔣️.json`.
- xml: `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/{🧬️schema/📸️snapshot/🦀️.rs, ✏️editor/🦀️.rs, 👁️viewer/🦀️.rs}`, `.../✅️valid/🧬️schema/🦀️.rs`.
- las: `☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/✏️editor/🦀️.rs`. tiff: `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs`.
- docx: `📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️edit-rules/{🦀️.rs, 🧪️tests/🦀️.rs}`.
- framework: `🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`, `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🏛️validation/🦀️.rs` (`🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs` was instrumented and restored; its remaining diff is the peer sweep).
- cache (deleted binaries and target; kept): `⚡️cache/edfix/{res-before,res-after}/results.tsv` and the build/test logs.
