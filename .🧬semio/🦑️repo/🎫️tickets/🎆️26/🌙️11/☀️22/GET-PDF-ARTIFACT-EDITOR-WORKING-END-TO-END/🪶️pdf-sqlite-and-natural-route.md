# PDF sqlite snapshot registration and natural-file route

Native runs: `sh native-cargo.sh test ...` with `bun` from `.cache/b/t/m/bun/*/` on PATH.

## (a) sqlite snapshot: `Lang` and codec registration

### Counts (`semio-s-artifact-stdio-pdf --lib`)
| | pass | fail | failure classes |
|---|---|---|---|
| before | 22 | 16 | 14 x "document codec registration conflicts for schema stdio.pdf" (13 of them from the shared one-time registration, one a direct registration), 1 x `Lang` Str vs Text, 3 x value-ownership |
| after | 22 | 16 | 0 registration conflicts, 0 `Lang` mismatches; all 16 are now the value/IO ownership framework (below) |

The pass count is unchanged only because every test that used to die on registration now reaches the next, independent blocker.

### Registration root cause (proved with `[DEBUG]` traces, removed again)
- `sqlite_snapshot_pdf14_owned_io_preserves_arbitrary_schema_and_text` (1.4 `🔬️unit`) called `register_native_snapshot_codec(dialect, store::artifact_codec::<..>(STDIO_PDF_DOCUMENT_SCHEMA))` directly. That codec carries `reference: Some(..)`.
- The shared one-time `root_tests::register_sqlite_test_declaration` registers the declaration's `native_artifact_codec` for the same schema with `reference: None`.
- `same_document_codec` compares the reference, so whichever registered second failed deterministically with a conflict. Under parallel test threads the direct registration won the race; single-threaded the declaration won, which is why `--test-threads=1` showed no conflict.
- Fix: the test now registers through the single idempotent `register_sqlite_test_declaration()`. Every PDF test registers through that one `OnceLock`, so registration is order independent and a failed test cannot poison later ones. The framework registry itself is unchanged: first owner wins, a differing second owner is a refusal.

### `Lang` root cause
`PdfUaBuilderConstruction` seeded `/Root/Lang` as `PdfObject::Str(bytes)`, while the decode path (retained-text lift) and the sqlite snapshot represent it as `PdfObject::Text`. Fix: the seed emits `Text(lang)` and `has_nonempty_lang` accepts both text forms. The `agree()` assertion in `sqlite_snapshot_pdf17_ua_initial_retains_genuine_typed_raw_profile_and_both_public_forms` now passes; that test fails further down (295:604) on the ownership blocker.

### Remaining 16 (not fixed, out of the PDF crates)
All fail inside `semio-framework-value` encode/decode ownership, which is in flight:
- "forwarded native storage requires its quoted allocation effect" (`🌱️value/🛫️encode/🦀️.rs:256`, `charge()` on a forwarded allocation port with no quoted effect).
- "native construction / native encoder requires its original recipient" (`🌱️value/🛬️decode/🏭️construction`, `🛫️encode/🏭️construction`).
- `encode_native(..).is_ok()` in `sqlite_snapshot_pdf_native_full_entity_row_admission`.
The sqlite codec paths call these through `io_export_sqlite_snapshot` with the test helper `with_snapshot_file_decoding`, which does not yet supply the recipient / quoted effect.

## (b) natural-file route (`semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests`, filter `pdf17::tests::natural_file_route` and `paged_natural`)

Crate name is `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests`. A full run of this crate never ends: the `history_edits_end_to_end` / `conflict_history_edits_end_to_end` tests spin at 200% CPU for over 30 minutes (value/UI ownership pumps never settle), so use the filter.

| | pass | fail |
|---|---|---|
| before (filtered, 7 tests) | 2 | 5 |
| after | 2 | 5 |

Before: 4 `natural_file_route_*` failed with "reserved media source is absent", the paged large-image decode with "did not publish within 4000000 continuations". The "PDF decoder made no progress" message did not occur on rerun after the import-loop fix.

### Fixed: "reserved media source is absent"
`consume_media` builds the media only when `reserved_media_due` is true (header 4 staged). In the same turn `advance_reserved_media` then stages header 4, and `prepare_reserved_media_input` sees header 4 ready with `media == None` and raised an invariant error. Fixes:
- `🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs`: `prepare_reserved_media_input` returns `Ok(false)` (wait for the next funded turn) when the media is not supplied yet.
- `🪢️members/📥️staging/🎞️media-transfer/🦀️.rs`: a media value built but not consumed in the turn gives its bytes back to the artifact instead of dropping them (the natural path `mem::take`s them).

### Fixed: "editor did not declare a loaded-parent child projection"
The PDF editors and viewers never declared `child_restore_projection`, which the live envelope load now requires. Declared (empty, derived from the snapshot's `ArtifactCompositionFields`) in: 1.7 base editor, 1.7 kit macro `pdf17`, 1.4 kit macro `pdf14`, 1.7 base viewer, 1.7 vt viewer.

### Changed: controlled decoder idle handling
`NaturalPdfDecoder::advance` (`🚪️io/📥️import/🔁️controlled/🦀️.rs`) now continues inside the turn on an unpaid `More` instead of ending the host turn (a moving cursor costs no turn; a stuck location still fails after 64 idle steps). The 1.7 browser-contract test still passes. It did not cure the large-image stall below.

### Still failing (new causes behind the fixed ones)
- `natural_file_route_matches_independent_rotation_and_retains_other_pdf_objects`: the open now succeeds; the next step fails with "captured shared original changed its Work or destination" (original-work/ownership framework, line 91 of the test, `settle_registered_typed_operation`).
- `natural_file_route_opens_large_retained_{vector,type1}_document...` and `..._large_sampled_images...`: the import settles forever ("original fixture settlement stopped after 32768 turns ... close owner Begin").
- `paged_natural_decode_round_trips_a_large_original_image_exactly`: the paged decoder does not publish within 4,000,000 continuations. Not caused by idle yields (unchanged after the continue change); a large single stream appears stuck in the decoder or the grant (16 KiB per turn). Needs a native probe with a per-phase counter.

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🪢️members/📥️staging/🎞️media-transfer/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`
- `.../🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/🦀️.rs`
- `.../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📥️import/🔁️controlled/🦀️.rs`
- `.../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs`, `✏️editor/🧰️kit/🧾️pdf17/🦀️.rs`, `✏️editor/🧰️kit/🧾️pdf14/🦀️.rs`
- `.../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/🦀️.rs`, `🧾️vt/👁️viewer/🦀️.rs`

No schema, json, proto, graphql or ts changes were needed: the sqlite schema already stores `Text` for `Lang` and the TS twin builds it as text.

## Follow-up round (b only): not resolved

Machine state: every native run spent over an hour queued behind other agents' builds (shared build dir, constant dependency edits), so the native probe for item (3) never reached its test binary. The probe was a `[DEBUG]` counter in `NaturalPdfDecoder::advance`; it is removed again, nothing debug remains.

### Decoder change vs the browser hot loop (verified by reading and by the earlier run)
- The 64 bound holds: `self.stall` and `self.stall_at` live on the decoder, so consecutive unpaid `More` steps at one `trace()` location still fail with "PDF decoder made no progress" at 65, now inside one `advance` call instead of across 65 turns. A paid step resets both.
- `paged_natural_decode_of_the_browser_contract_terminates_with_every_page` passed after the change (nat3 and nat4 runs); its `turns <= 128` assertion is untouched. A moving cursor can no longer burn a turn per unpaid step.
- Residual risk: in the lift phase `trace()` is "-", so a legitimate run of more than 64 consecutive unpaid steps there errors. That was already true before the change.

### Item (1), "captured shared original changed its Work or destination"
Raised only at `🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs:8` (`invalid()`), reached from `close`/`close_original_into` when `work.follow_between_actions` fails, when `original.original_work().binding_identity()` does not match the current `effect`, or when `elected/alias` is closed with an `effect` that did not `performed(1)`. So the close effect used to retire the shared original was issued from a different Work binding or source turn than the one that captured it. In the rotation test this happens in `settle_registered_typed_operation` right after the first dispatch, i.e. the shared original captured during `dispatch_fixture_action` is closed by a maintenance turn with another turn identity. This is in the arc-custody/work-binding contract that another agent is changing; no root fix made.

### Item (2), large documents never settle
`settle_registered_typed_operation` ends with 32768 turns, `pending true`, `maintenance empty false`, `close owner Begin`, grant 64 items / 512 KiB copy / 16 MiB capacity / 1 MiB release. `Begin` means the close owner never took its first step, so the maintenance turn is either not admitted (`MountedOwnerTurnV1::admit` returns `None`) or admitted with a demand it cannot fund. Together with item (1) this is the same arc-custody close family: the closes that follow the first dispatch cannot start because the captured original does not accept the maintenance effect. Not reproduced under a probe; not fixed.

### Item (3), large paged image
Not reproduced under a probe. Static reading: the decoder allocates the stream backing with `allocation_bytes = min(maximum_capacity_bytes, maximum_copy_bytes - copy_body)` (or `maximum_release_bytes` when reserving an empty output), so with the stock 512 KiB copy grant a multi-megabyte image can only be admitted through the empty-output path; a repeated `Yield` with `blocked` unchanged is the candidate. Next step is the native probe: print phase, `completed_work_units`, `trace()` and `blocked` every 20000 calls from `advance`, filter `pdf17::tests::paged_natural_decode_round_trips`.
