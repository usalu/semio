# PDF integration status (2026-11-24)

All runs through `native-cargo.sh`, one cargo at a time, bun on PATH, `RUST_MIN_STACK=67108864`. Disk was full once (35 GB freed by deleting the stale `lane-1` build dir of `🗑️generated/native-target`).

## 1. Whole-workspace compile (`check --workspace --tests`)

Run 1 stopped on one framework error (`Drop` impl for `NativeEnumReceiver` missing `N: RetireOwned`, already fixed by another agent while I waited). Run 3 then showed the real test-target errors; run 5 compiled clean (109 crates, 0 errors).

| Error | Cause | Fix |
| --- | --- | --- |
| `glyph_location` is private (4x, `fonts/🔠️truetype` unit test mounted in `schema`) | `pub(crate)` method used from a test in another crate; the first assertion was a tautology | method is `pub` with a docstring; tautology removed |
| `inverse` not found on `PdfDiff` (1.4 mutations test) | `ColdDiffAlgebra` not imported | `use protocol::ColdDiffAlgebra` |
| `apply`/`inverse` not found on `PdfRoleDiff` (testkit archive test) | the role wire type is no longer the generic triple | test goes through `to_indexed()` |
| unresolved import `text::snapshot::field` (`inferences` producer test) | unused import of a module that does not exist | import removed |
| `io::text` not found (`export` unit test mounted in `schema-4`) | wrong crate path for `retained_text` | fully qualified through `semio_s_artifact_stdio_pdf_inferences` |
| `&&[u8; 20]` expected `&[u8]` (`modules/🖼️images`) | extra reference after the paged stream type | `stream_bytes(data)` |
| missing `Display` for `TypedOperationResultLane`, `semio_framework_hash` import in native-save | not present any more (fixed by others before this run) | none needed |

## 2. Requested suites

| Suite | Result |
| --- | --- |
| `semio-s-artifact-stdio-pdf --lib` | 38 pass / 0 fail |
| `-io-export` | 19 / 0 |
| `-mut-family-11`, `-12`, `-13`, `-14`, `-15` | 32 / 120 / 79 / 73 / 59, all 0 fail |
| `-mutation-round-trip-tests` | 12 / 0 |
| `-page-editor-tests` | 65 / 0 |
| `-page-editor-authoring-tests` | 20 / 0 |
| `-presence-lane` | 10 / 0 |
| `semio-s-artifact-stdio-pdf14-canvas` | 8 / 0 |
| `-native-save --lib` (whole lib, includes the incremental/segmented unit tests) | 15 / 0 (493 s) |
| snapshot-value `editor::pdf14` | 34 / 0 (includes the pdf14, 14a, 14x history, conflict, inputs-resolve and `closes_mid_reserved_commit` laws) |
| snapshot-value history laws 1.7 (`pdf17`, `17a`, `17e`, `17h`) | **before fix 8 fail, after fix 8 / 8 pass**; `pdf17ua`, `17x`, `17vt` passed all along |
| snapshot-value `natural_file_route` | 7 / 0 |
| snapshot-value `paged_natural_decode` | 3 / 0 (the cancellation-mid-operation test is one of them, so the cancel path of the chained close ladder ran) |
| snapshot-value `closes_mid_reserved_commit` | 9 / 0 (1.4 and 1.7 variants) |

### The 1.7 history laws (`cold replay edit refused after a failed in-place application`)

Root cause (found with `[DEBUG]` probes in `ColdReplayEdit::advance`, removed again; `cold-apply/🦀️.rs` has no net change): the history harness drafts a candidate input, the leaf `setPageBox` with page index 1 on a one-page document built a diff without looking at the base, `outcome.admit()` admitted it, and the in-place `apply_diff_cold_into` then failed with `mutation.apply.missing-target` (`pages/pages/1`) after the post root was already marked failed; the next `advance` reported the generic refusal. The 1.4 leaves refuse such a target in `diff` (`mutation.target-missing`), the 1.7 leaves did not.

Fix: `diff::outcome_over(base, diff)` in `🔺️diff/🧾️pdf-diff-aggregate` validates the freshly built diff against the base with the same `validate_pdf_diff` the apply uses and returns an empty diff plus a `mutation.target-missing` error otherwise (the rejected diff is retired). Fifteen page-addressed 1.7 leaves use it: set-page-content, append-page-content, set-page-crop-box, set-page-media-box, set-page-box, set-page-user-unit, set-page-rotation, remove-page, insert-page, insert-content, replace-content, remove-content, set-annotation, insert-annotation, remove-annotation. Re-ran all mutation lib suites after the change (above table plus the workspace sweep below): no regression.

## 3. Workspace sweep of everything else (`test --workspace --lib --tests`, minus the suites above)

First sweep: 1001 pass / 35 fail in 9 targets. After the fixes below: 7 targets still failed, 13 tests; those 13 are worked in "Round 2" at the end of this file.

| Failing test(s) | Cause | What I did |
| --- | --- | --- |
| 9 pdf14 window tests (`own14_page_view_...`, `render_keeps_each_own_page_draft_and_dimensions` x3, `render_lists_each_own_page_...` x5) | `UI arena has no accepting lifetime registration`: the tests project a built tree without holding the fixture UI arena | `FixtureUiArena::admit()` at the start of each (passes) |
| 3 pdf14 `definition_declares_the_own_resolved_page_actions` | stale expectation: the 1.4 page window declares only `set-page` and `replace-page-text` since commit 755 | expectation updated (passes) |
| 7 `render_paints_the_page_text_on_the_canvas` (profile_windows, 1.7 variants) | stale test: the scene's layers travel in lanes and text is painted as glyph paths, not as the string | project the tree and decode with `decode_fixture_scene_with_lanes`; assert the text layer `p0:t0` with `cubic` segments (7 / 7 pass; `first_surface` helper removed) |
| `stream_data::identity_stream_output_is_born_and_retired_in_bounded_pages` (filters) | the test assumed two retirement steps per page; only leaf pages hold octets: steps = pages + leaves | exact law `steps == pages + ceil(len / 4096)` (passes) |
| `pdf14_operations_round_trip_through_the_pack_document_with_exact_float_words` (schema-2) and `pdf17_operations_round_trip_...` (testkit) | **real bug, not fixed**: a `-0.0` field is decoded as `0.0` after `encode_op` / `decode_op` (text codec `print_op`/`parse_op` keeps it, so only the binary Pack document path loses the sign; 1.4 `ResizePage.height`, 1.7 `PdfPage.media_box`). Encoder (`TAG_F64` bits), decoder (`read_f64_le`) and `FromValue` read exact in isolation; the loss is somewhere between them in the wire value path (`variants_binary` / `tagged_value_binary` -> `encode_wire_value_into` -> `decode_wire_value`). Not bisected further. | open, first suspect list above |
| 9 `export::component::tests::*` mounted in `schema-4` (`a_structural_page_edit...`, `bachelor_thesis_reads...`, `demo_pdf_asset_is_the_natural_file...`, ...) | structural: the same test file runs in `io-export` (19 / 0) and, through the `🪞️mirror` mount, in `schema-4`, where `fixture_path!` (manifest-relative, written for the `io-export` depth) misses its fixtures and then poisons a `LazyLock`. These tests never compiled in `schema-4` before today. | open: stop mounting `io/export` tests in the `schema-4` mirror (or make the fixture paths file-relative); the same tests pass in `io-export` |
| `bachelor_thesis_tests::analyzer_to_builder_round_trip_reproduces_equivalent_pages` | `foreign PDF artifact pdf:s.stdio.font.type1:... has not been admitted` while encoding the rebuilt snapshot (testkit, 7 min test) | open |
| `bachelor_thesis_tests::lossless_structural_flow_law_...` | `token count exceeds max_tokens limit (4000001 > 4000000)` in the snapshot DSL round trip (one token over the limit) | open |
| `page_content_contracts::controlled_page_projection_matches_reference_across_borrowed_and_split_streams` | `left: 0, right: 100` at `modules/⬇️lift/.../📜️page-content/🧪️tests` line 133 | open, not investigated |
| `aggregate_operation_contracts::a_details_edit_dispatches_the_concrete_kind_of_the_field_it_names` | `SetValue` on `/info/title` of a document without a title fails with `object key 'title' does not exist` (optional keys are absent in the value tree now); the Details route would need `InsertValue`, or the test an `info` with a title | open |
| `original_archive_objects_restore_actual_snapshot_and_role_lane` | restored objects keep an extra `5 0 Null` object (`left` has two objects, `right` one) | open, not investigated |
| `original_archived_set_image_donates_into_concrete_mutation` | `original receiving made no progress: quote copy_bytes 396402 against grant maximum_copy_bytes 32768`: the donation of a `SetImage` quotes the image octets as one copy; likely the paged stream type's receiving quote | open |

## 4. Changed files (all under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`)

- `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🔠️truetype/🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖼️images/🦀️.rs`
- `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🗜️filters/🪣️stream-data/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🦀️.rs`
- `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📤️export/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧾️pdf-diff-aggregate/🦀️.rs` (`outcome_over`)
- `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/{✏️set-page-content,➕️append-page-content,✂️set-page-crop-box,📐️set-page-media-box,🖼️set-page-box,📏️set-page-user-unit,🔄️set-page-rotation,🗑️remove-page,📥️insert-page,🖋️insert-content,🔁️replace-content,🧻️remove-content,📝️set-annotation,📌️insert-annotation,📍️remove-annotation}/🦀️.rs`
- `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📩️archive/🦀️.rs` and `.../🧪️tests/🔬️unit/🦀️.rs` (debug probe added and removed)
- `🏅️standards/7️⃣1.7/🪆️subsets/{🧱️base,🗄️a,📐️e,⚕️h,🧾️vt,♿️ua,🖨️x}/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs` (canvas test)
- `🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/4️⃣1.4/🪆️subsets/{🧱️base,🗄️a,🖨️x}/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs` and `.../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs`
- `🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs`
- framework (net no change, `[DEBUG]` probes added and removed): `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs`


## 5. Round 2 (the 13 remaining failures)

Verification run 1 (`run-round2-fixes.sh`, `schema-2`, `schema-4`, `testkit`, `io-export`, snapshot-value, filtered): `io-export` 19 / 0, snapshot-value (`editor::pdf14`, `natural_file_route`, `paged_natural_decode`, `closes_mid_reserved_commit`) 50 / 0, `schema-2` (float words) 4 / 0, `schema-4` (`export::component`) 19 / 0 in 184 s, testkit `page_content_contracts` 3 / 3 and bachelor thesis `lossless_structural_flow_law` pass. The testkit run then hung in `analyzer_to_builder_round_trip_reproduces_equivalent_pages` for 80 minutes and was killed by hand (see item 3). Run 2 (`run-round2-recheck.sh`, behind `watchdog-cargo.py`, which SIGKILLs any test binary older than 600 s) re-verifies the rest; its result is recorded in the table below.

| # | Test(s) | Root cause | Fix | Status |
| --- | --- | --- | --- | --- |
| 1 | `pdf14/pdf17_operations_round_trip…exact_float_words` | `renormalize_whole_number_floats` (pack bridge) rewrote every whole float to an integer; `-0.0 >= 0.0` took the `UInt(0)` branch. Encoder, wire and decoder were exact. | Negative zero keeps its float word. Policy: `-0.0`, NaN (payload kept), +/-inf are never integer-normalised, exact IEEE bits travel (`TAG_F64`). | pass (1.4 in `schema-2`, 1.7 twin in testkit) |
| 2 | 9 `export::component::tests` in `schema-4` | The same test file is mounted in `io-export` and, through the mirror, in `schema-4`; its `fixture_path!`/`fixture_*!` paths were manifest-relative for one crate depth. | The five fixtures resolve from the repository root (`repo_bytes!`/`repo_text!`), one mount, both crates. | pass (19 / 0 in `schema-4`) |
| 3a | `analyzer_to_builder_round_trip…` (Type1 not admitted) | The builder-only rebuild had no native custody: every font/image program is a reference whose body lives in the original graph; the rebuilt snapshot had no objects. | The test seeds the builder with the original stream objects (`PdfBuilder::from_snapshot`, no trailer, so the writer still lowers afresh). The streams are put on the built snapshot (no trailer, so the writer still lowers afresh). Two earlier attempts hung and were found by sampling the hung test binary (`sample`), not by rerunning: one `InsertObject` per stream re-projects the whole graph per edit, and seeding the builder before the ~1,000 resource mutations made every mutation deep-clone the whole snapshot (`apply_diff_cold` clones the snapshot; `PagedBytes::clone` copies byte by byte) - quadratic in the custody bytes. Also fixed a real quadratic: `NativePdfArtifactResources::resolve` hashed every graph stream for every reference; graph streams are now identified once per kind. Test: 55 s. | pass |
| 3b | `lossless_structural_flow_law…` (4,000,001 > 4,000,000 tokens) | The limit was a guess; the check fires at the first token over, so 4,000,001 was only a lower bound. Measured with a `[DEBUG]` probe: the thesis snapshot DSL is 206,745,591 bytes and 45,010,438 whitespace words, so it never fitted. | A token or node spans at least one byte, so both budgets equal the input length; the existing 272 MiB byte budget bounds the work (the thesis uses 73 % of it; a document above 285 MB would be refused by the byte budget). `[DEBUG]` probe removed. | pass |
| 4 | `controlled_page_projection_matches_reference…` | Test law was stale. Before, a single stream was borrowed contiguously and a split page was joined into a new buffer, so `acquired(split) - acquired(single) == joined.len()` proved the join copy. Stream data is paged now (`stream_bytes`), so one stream is copied page by page into the same joined backing: both cases copy the joined bytes once. | The law is now `acquired(split) == acquired(single)`, `released` equal, `>= joined.len()`. The split case is still distinguished by its separator: the operator list of the two-stream page must equal the reference parse of `left + "\n" + right` (a missing whitespace separator would merge the last token of one stream with the first of the next), and `referenced_content_arrays_join_mixed_members_in_original_order` asserts the exact joined octets `20\n30 m 75\n90 l S`. | pass |
| 5 | `a_details_edit_dispatches_the_concrete_kind…` | `SetValue` on an absent optional key: the rule table reads the value tree, which omits absent keys. | The PDF `special_edit` now owns every whole-value lane at any depth (`/info/title`, ...): set of an absent known field creates it, remove of a present one works. An unknown or mistyped field under a whole-value lane is refused, never a silent no-op: code `stdio.pdf.unknown-field`, parameter `{field}`, English "The document has no field {field}." and German "Das Dokument hat kein Feld {field}." in the editor fault catalog (`PdfFault::UnknownField`, covered by `fault_notices_cover_every_raised_code_in_both_languages`, which checks both languages and matching placeholders). Tests added for set-absent, remove-present, remove-absent, unknown field (code and name). The 1.4 `special_edit` is not changed. | pass |
| 6 | `original_archive_objects_restore_actual_snapshot_and_role_lane` | Read from the `[DEBUG]` outcome print (removed): the inverse `RemoveObject` was refused with `mutation.apply.stream-role-required`, "semantic stream role payload omits a consumed logical dependency". A role's inputs always include its own identity (`dependencies_of` starts with it), but the fixture's restored role and the test's base role carried `dependencies: []`; the stale refusal left object 5 in place. | The fixture (`pack-donation.json` inverse) and the two test roles now carry their own identity as dependency. | pass |
| 7 | `original_archived_set_image_donates_into_concrete_mutation` | Not the stream type: `PdfImageBody::Samples { values: Vec<u32> }` makes the decoded wire value 12,288 nodes x 32 B = 393,216 B (+ header = 396,402 B); the pack-value receiving quotes the whole conversion as one turn against the 32 KiB grant, so it could never be funded. | The conversion quote is prepaid page by page (`paid`), one `body`-sized page per turn, before the conversion runs. Honest limit: the conversion (`M::from_value`) is still ONE atomic step; what bounds it is the decoded value's size (`decoded_value_retained_bytes`, the quote itself) and the stock/capacity grant; between pages it is cancellable and every turn stays within its grant, but the final step is not interruptible. Making it genuinely incremental needs a retained-clone `FromValue` (the store law `pack_value_conversion_heap_growth_stays_within_its_quote` already names this need). | pass |

Regression pass after all Round 2 edits (`run-round2-regression.sh`, libs, unfiltered): `semio-s-artifact-stdio-pdf` 38 / 0, `io-export` 19 / 0, `mut-family-11` 32, `-12` 120, `-13` 79, `-14` 73, `-15` 59 (all 0 fail), `mutation-round-trip-tests` 12 / 0, `native-save` 15 / 0 (534 s), `page-editor-authoring-tests` 20 / 0, `page-editor-tests` 65 / 0 (includes the bilingual fault catalog law with `UnknownField`), `presence-lane` 10 / 0, `pdf14-canvas` 8 / 0. Snapshot-value (filtered) 50 / 0 and `schema-2` 4 / 0, `schema-4` 19 / 0 passed in the first Round 2 run. Not run: `semio-os-dsl-module-tests` (the pack-value receiving tests live in the root workspace; the receiving change is covered by the archive donation test and the snapshot-value and mutation suites above).

Updated table (all 13 tests): items 1 (2 tests), 2 (9 tests), 3 (2), 4 (1), 5 (1), 7 (1) pass; item 6 passes (testkit `original_mutation_archive` 10 / 0 after one rerun that an unrelated compile error of another agent had blocked). All 13 failing tests pass; no `[DEBUG]` probe of mine remains.
