# Export Round-Trip Failures

Source: `/private/tmp/claude-501/-Users-ueli-Documents-semio/1d0bb5f7-211f-43c0-a647-88a04838a26d/scratchpad/pdf/set2.txt` (export crate around line 100084, editor history around line 100750). `decode_pdf` uses `read_clear_pdf_graph`, so the export lane failures are lift and write bugs, not the controlled decoder.

## Assertions

- `text_document_round_trips_and_shows_its_text` panics at export unit `assert_same_lanes` line 85. The only diff is `$.admittedStreamRoles: length 2 vs 0`.
- `rich_document_round_trips_every_lane`, `encryption_round_trips_with_every_algorithm`, and `xref_streams_and_object_streams_round_trip` panic at the same line. Full diff: `$.acroForm.defaultAppearance` only on the seed (`/F1 0 Tf 0 g`), `$.pageLabels[1].prefix` only on the seed (`A-`), `$.admittedStreamRoles: length 13 vs 0`.
- `document_stream_matches_the_whole_document_write` panics at `stream.page(page).unwrap()` line 405: `Unsupported("foreign PDF artifact pdf:s.stdio.font.truetype:sha256:8d8d58562dfe725ca4a38d2e92a4665806b85775be005b106cc635158455037b has not been admitted")`.
- `demo_pdf_asset_is_the_natural_file_of_the_demo_snapshot` panics at `assert_eq!(asset, encode_pdf(...))` line 518. Left is the 906-byte spaced SemioSans fixture baked into that binary. Right is the current compact Helvetica writer. `🎬️demo.pdf` on disk is already 715 bytes, Helvetica, `Kids[3 0 R]`. The fixture was not rewritten.
- `blank_document_is_one_letter_page_and_its_own_fixed_point` passed. It is already `decode(encode(empty page))`, has no content stream, and has no embedded font.
- Thesis and structural rewrite tests in the same log also fail `assert_same_lanes` only on `admittedStreamRoles` owner numbers. Those numbers are retained-graph bindings, the same class as `objects` and `trailer`, which `typed_lanes` already drops.
- `editor::pdf17e/h/ua/x::history_edits_end_to_end` panics at framework `ingress.rs:106`. Tool proof rejects `zoom-in` because `factory='BoundedNativeEditToolJobFactory'` does not match `registered_factory='semio_s_artifact_stdio_contract_editing::tool_job_factories::BoundedNativeEditToolJobFactory<...Pdf17EEditor>'` (`generated_migrated=false`). Owner, controller, and schema strings match. This is not an encode/decode bug.
- Natural open tests (`controlled_pdf_terminal_publication_requires_both_owned_headers`, `paged_natural_decode_of_the_browser_contract_terminates_with_every_page`, and one later unwrap) fail with `PDF decoder made no progress during prepare-file`. That binary message has no ` at {trace}` suffix. `NaturalPdfDecoder::advance` grants `CosGrant { work: 1 }` and counts every zero-work `More` in the same outer phase. Outer phase stays `PrepareFile` for the whole parse, so inner stage changes were counted as a stall. `read_clear_pdf_graph` uses work 4096 and has no stall counter, which is why export tests reach the lane diffs instead.

## Fixes

- `Lifter::text` now decodes `PdfObject::Str` with `decode_text_string`. Lower writes page-label `/P` and AcroForm `/DA` as `Str(encode_text_string(..))`. Those ASCII strings are PDFDocEncoding and round-trip. `admit_retained_text` still does not list `DA` or `P`, so the retained graph stays `Str` and the byte fixed point is unchanged. Field DA already survived through `text_value`.
- `typed_lanes` also clears `admitted_stream_roles`. Lift records an Operators role for every page `/Contents`. A hand-built seed has none. The testkit still requires those roles on the lifter snapshot, so recording them stays.
- `DocumentStream::begin` no longer clears `objects` on the headless snapshot. Fonts are lowered from the original snapshot, then `page` builds `FontCodec` from the copy. An empty graph cannot admit the embedded TrueType. Pages and trailer are still cleared.
- A zero-work `More` counts toward the stall only while `PdfFilePreparationCursor::trace()` is unchanged. The trace now includes the projection stage and object number (`project/normal/{stage}/{num}`), so a stage change is progress. Sixty-four identical traces still abort.

### Fixes, second pass (native run, before and after)

| Crate | Before | After |
| --- | --- | --- |
| `semio-s-artifact-stdio-pdf-io-export` lib | 15 pass / 4 fail | 19 pass / 0 fail |
| `mut-family-11` | 31 / 1 | 32 / 0 |
| `mut-family-12` | 93 / 27 | 120 / 0 |
| `mut-family-13` | 75 / 4 | 79 / 0 |
| `mut-family-14` | 67 / 6 | 73 / 0 |
| `mut-family-15` | 55 / 4 | 59 / 0 |

Root causes and fixes:

- **Spacing of retained text values.** `write_dict` in the lexer left no space before `Str` values but did before `Text` and `Date`, so a lowered `/Lang (de-CH)` and a retained `/Lang(de-CH)` differed. `Text` and `Date` now follow `Str`.
- **Duplicated font programs.** `decode_pdf` appended every admitted font-program stream to `objects` even when an identical stream already sat in the graph (23 extra unreferenced Type1 streams on the thesis). The first write kept them, the re-encode did not. Admitted objects equal to an existing one are now skipped; `NativePdfArtifactResources::resolve` finds the graph copy by identity.
- **Object-number bindings in lopdf vector comparison.** `lanes()` in the lopdf-vectors test now also clears `admitted_stream_roles`, same class as `objects` and `trailer`.
- **`diff_replace_page`** removed and re-added the whole page. It is now one sparse `PdfPageDiff` carrying only differing fields.
- **Output intents without a stream role.** `output_intent_rows` created the ICC `/DestOutputProfile` stream without its `ReferenceBody` role, so graph projection refused it. New helpers `role_rows` and `reference_body_role` in conformance-support add the role, and every owned-object drop (output intent, DPart root, struct tree root, `remove_catalog_entry_owned_rows`) now retires the roles of the dropped objects.
- **Stale inverse expectations.** The `remove_*` inverse tests (trim box, display doc title, lang, mark info, struct tree root, DPart metadata and root) asserted `entry_index: None` and empty placements. The inverse restores the exact position (`Some(1)`, real placements), as the "middle row" tests require.

Validation: the lopdf vector tests (27 operations) and `semio-s-artifact-stdio-pdf-test-oracle` comparisons in io-export pass.
