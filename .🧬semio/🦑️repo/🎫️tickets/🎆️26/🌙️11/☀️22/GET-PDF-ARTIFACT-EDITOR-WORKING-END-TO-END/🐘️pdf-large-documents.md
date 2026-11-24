# PDF large documents (2026-11-24)

Native evidence only, every run through `native-cargo.sh`, one cargo at a time, always filtered, crate `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib`. Numbers are debug native builds; wall time is the settlement of the open (`settle_registered_typed_operation`), the turn count is host turns (one fixture turn is one guest call with fresh fuel and grant). Runner: `large-documents-run.sh <log name> <test filters..>` (writes `🗑️generated/<log name>.log`; the logs were deleted after the numbers below were taken).

## Root causes (all measured, none guessed)

### 1. One item per worker step (decoder 13 to 20 units per call, Encode one unit per call)

`bounded_ownership::original_lifecycle_grant` clamps `maximum_items` to `.min(1)`, correct for a retirement or handback step (one owner action is one item). `drive_worker_step` ran the business producer inside that same wrapper and loaned `grant` (the clamped one) to the job. Probes showed `items=0` after the first charged unit of every step:

- decoder (`NaturalPdfDecoder::advance`): the loop broke on `grant.maximum_items == 0` after the first allocation or object, so a job step advanced 1 to 20 units although the contract allows 4,096;
- pack export: every codec event reports `copied_items: 1`, so one step ran exactly one event (`iters=2 why=noprogress`, `dfuel=1`).

Fix (`🧵️retained-command/🔄️full-operation`, `drive_worker_step`): the producer is loaned `RetainedCloneGrant { maximum_items: cx.retained_grant().maximum_items, ..grant }`, i.e. what the turn still owns, and its receipt is checked against that loan. Retirement and handback steps keep the one-item clamp.

Result: decoder 3,000 to 3,800 units per call (the 4,096 contract minus allocation breaks).

### 2. The Pack export was byte-level, one funded action per codec event

`BorrowedPackDocumentCursor` emits one event per byte per pass (three segment passes, a measure pass and an emit pass; roughly seven events per raw byte), and every event was one owner action charging one item. With 64 items per host turn (`MountedOwnerPolicyV1::INTERACTIVE_COMPONENT`) that is at most 64 events per turn; the 4.78 MB vector snapshot needs about 23.6 M events. Measured: 2.74 M events were still inside the measure pass (zero bytes written) when the run hit the 32,768-turn cap.

Fix (schema-neutral, framework):

- `BorrowedPackDocumentCursor::advance_batch` (`🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor`): up to `maximum_events` consecutive events as ONE owner action. Every event is still quoted against the remaining grant and recorded as its own work unit (`effect.performed(1)` per event), the byte currencies (`copied_bytes`, `retained_capacity_bytes`, `released_bytes`) sum exactly, and the batch is one copied item (a byte-level event owns no item). It stops before an event the grant, the output slice or the remaining work cannot fund, so an allocating event always starts the next action. A failing event carries the summed receipt of the batch.
- `OriginalSnapshotPackExport::advance` (`🏪️store/🚪️io/💾️binary/📸️export`): the staging slice is 4,096 bytes instead of 64 (final-page donation, page pop on close), the cursor runs `advance_batch` with 4,096 events and a 64 KiB copy allowance on top of its first event's quote.
- `NaturalFileImportJob` (`📜️declaration/📥️natural-file`): `PACK_EXPORT_ACTIONS_PER_STEP = 4,096` chained actions per job step instead of 64; the turn's items, copy, fuel and deadline end the step earlier.

### 3. A progress checkpoint after every 4,096 units cost two host turns per step

Each job step ended in a `Checkpoint` outcome; the outcome handback (acknowledge, then resume) switches the worker turn between the preparation and the close grant, and `drive_typed_operation_worker` deliberately hands the turn back when the close mode changes (one receipt family per turn). Measured ratio before the fix: 12,306 turns for 5,767 job steps (2.1 turns per step).

Fix (`natural-file`): `PUBLICATION_STEPS_PER_CHECKPOINT = 16`. A checkpoint (and so a progress event) is published every 16 steps, that is every 65,536 work units of Encode or Decode, and at once on every stage change. A step without an outcome ends the host turn without any handback. The coalescing quantum is the existing `publication_checkpoint_due` law with a 16 times larger quantum.

### 4. The fixture's work stock modelled nothing the host does

`OriginalWorkFixture::new()` is `with_source(.., fuel = 1 << 20)`: one lifetime stock, spent by the first turns; afterwards every turn had `fuel = 0`, `should_yield()` was true and the decoder's always-run first iteration advanced one unit (4,000,000 continuations cover 4 M of the 9.4 M units). Two more defects sat in the same test: its contract was `ToolExecutionContract::resumable(len, 16_384, 1, ..)` (one work unit per step, the browser's is 4,096), and its per-turn grant was the whole lifetime stock.

Decision: the finite stock of `new()` models a whole-operation allowance for small operations and stays. What the browser does is different: every turn is issued fresh fuel (the host Interactive lane budget, `OriginalFixtureCaller::turn_fuel()`), and the whole operation draws on one initiating-event lifetime (`HostLifetimeStock`: 58,720,256 fuel). New:

- `OriginalJobFixtureCaller::renew_each_turn(turn_fuel, lifetime_fuel)` (`🧵️job/⏱️context/🧪️testing/🎟️caller`): each turn starts from `min(turn_fuel, lifetime left)`, the lifetime is debited by what the turn spent, a turn after the lifetime gets none.
- `OriginalWorkFixture::hosted(lifetime_fuel)` (`🔌️plugin/🧪️testing/🎟️caller/👷️work`): the browser's funding for the work fixture; the test passes `HostLifetimeStock::FUEL`.
- The paged decoder helpers use `hosted(..)` and `NATURAL_SOURCE_TOOL_CONTRACT` (4,096 units per step, 8,192 items, 32 MiB output), so the fixture and the browser quote the same contract.

### 5. Decoder copy model overcharged byte-wise stream work 128 times (9 MiB image)

`COPY_PER_WORK = 128` bytes of copy per work unit is the worst case for a token and is what fits 4,096 units into the 512 KiB per-turn copy grant. For an uncompressed stream body one unit moves one byte, so the 9.4 M units of the 9 MiB image charged 1.2 GB against the 1 GiB lifetime copy (the browser lifetime copy is 528 MB): the copy stock was spent at 9,095,759 of about 9.4 M units (`decode advance call=3000: copy=30 why=grant`, then 13,000 yielding turns). Fix: the controlled decoder asks the cursor chain whether the next unit only copies one identity-stream byte (`PdfFilePreparationCursor::copying_identity_bytes` down to `PdfStreamDataCursor::copies_identity_bytes`), charges one byte of copy per unit for exactly that phase and hands that phase a bulk work grant (`CosGrant.work` up to the step's remaining units, the copy grant and the fuel; the stream data cursor already honours a work budget). Every other phase keeps 128 bytes and one unit per `next` call.

### 6. A Close receipt narrower than the policy grant was refused

`receipt_grant_matches` accepted a narrowing only under an armed host turn grant. A turn that already holds live retained capacity (4.29 MB of Pack pages in the Type1 import) can only be granted the capacity that is left, so the Close receipt grant was `maximum_capacity_bytes 12,488,908` against the policy's 16,777,216 and the fixture refused it (`registered fixture Close receipt differs from its original caller authority`). The Preparation receipt check already accepted a narrowing; the Close, history and tool-run checks now do too. The refusal message carries both grants and the progress.

## Measured turn counts and wall time

Before: none of the four tests reached the end.

| test | before | after items clamp only | after items clamp and batched Pack | after checkpoint coalescing (final) |
| --- | --- | --- | --- | --- |
| `natural_file_route_opens_large_retained_vector_document_and_reexports` | cap (32,768 turns, over 4 min, 2.94 M Encode calls and still in the measure pass) | cap at 32,768 turns, 21 s, 2.74 M events done | 12,306 turns, 24.8 s | **946 turns, 21.0 s** |
| `natural_file_route_opens_large_retained_type1_document_and_reexports` | cap | cap | 16,487 turns, 36.7 s | **1,249 turns, 27.9 s** |
| `natural_file_route_opens_reexports_and_reopens_large_sampled_images` (4 settlements: plain and mixed, open and reopen) | not run, same Encode wall | not run | 4,689 / 4,689 / 4,752 / 4,752 turns, 8.3 to 8.7 s each | **367 / 367 / 375 / 375 turns, 7.1 to 9.2 s each** |
| `paged_natural_decode_round_trips_a_large_original_image_exactly` (decoder only, 9,437,877 bytes) | 4,000,000 continuations at one unit each, no publish | unchanged | stalled at 9.1 M of 9.4 M units (copy stock) | **4,610 turns, 0.7 s** (one 4,096-unit step per turn; open, save and reopen all exact) |
| `natural_source_route_opens_a_pdf_under_the_browser_turn_fuel` (new, two pages) | stalled (header 1, see the open and Save note) | - | - | 68 turns, 0.25 s |
| `paged_natural_decode_of_the_browser_contract_terminates_with_every_page` | 27 continuations | - | - | 1 continuation |

Final caps in the tests: `LARGE_DOCUMENT_TURN_LIMIT = 4,096` for every media-route open (asserted through `settle_registered_typed_operation_within`, 3.3 times the worst measured 1,249 and one eighth of the 32,768 ceiling) and `LARGE_IMAGE_DECODE_TURN_LIMIT = 8,192` for the decoder-only 9 MiB image (4,610 measured). The wall time that remains is the work itself: 23.6 M (vector) and about 33 M (Type1) codec work units, 0.8 M units per second in a debug native build.

## Progress and cancellation

- Progress: a `Checkpoint` outcome (`applied_progress` = decode units plus publication units) is published every 65,536 units and at every stage change (Decode to Encode, Encode to AdoptGenesis, to Publish). The existing `natural-source` law (`naturalWorkUnitsPerStep = 4,096`) is unchanged.
- Cancellation, asserted on a 9 MiB document: new test `paged_natural_decode_of_a_large_original_image_honours_cancellation_mid_operation`. After 16 turns (about 55,000 units of 9.4 M) the root cancel token is cancelled; the next `advance` returns `Yield` without consuming work, and `begin_close` plus the bounded close path retire every byte (`terminal_is_empty`, the fixture's `finish()` proves the stock closes). The close itself still costs one funded work unit per turn: 7,189 close turns for the 16 turns of work done, asserted below 32,768. The batch actions of the Pack export honour cancellation too (`advance` refuses to resume once the token is cancelled, `advance_batch` stops on a non-admitted event) and the export closes through `close_pack_export`; its close pops up to 4,096 staged bytes per action (was 64).
- The exact cancel-mid-Encode of a retained Pack is not exercised natively; the decoder cancellation is the asserted one.

## Not done, measured and open

1. Close granularity: the retirement of a partially built graph is one unit per host turn by design (`close_original_effect_step` quotes one work unit per action); chaining several native close steps inside `NaturalPdfDecoder::close_step` was tried and gained nothing (7,189 to 7,079 turns) because the close effect only funds that one unit. A large cancelled import therefore retires in proportion to its size (about 8 units per turn). This belongs to the close ladder, not to the open.
2. Contiguous stream backing: under the browser's per-turn grants (512 KiB copy, 1 MiB release) a single contiguous retained stream above about 1 MiB cannot be allocated (`allocation_boundary = copy - copy_body`, or the release grant for an empty reservation). The 9 MiB original image therefore decodes in the fixture, whose paged-decoder grant is `capacity 16 MiB, copy 1 GiB`, but the browser would answer `PDF decoder needs a 9437184-byte backing grant` for an uncompressed stream of that size. Fixing it needs paged stream output (`PagedBytes`) in `PdfStreamDataCursor`; not attempted.
3. `drive_typed_operation_worker` still ends the turn when the close mode changes. With checkpoints coalesced this costs about one turn per 4,096-unit step; the next lever is a larger `max_work_units_per_step`, which is pinned by `natural-source-limits.json` and the 7.5 ms step deadline.
4. `BorrowedPackDocumentCursor` tests live in a crate that is not a workspace member (`semio-framework-pack-record-encode-cursor`), so `borrowed_pack_document_batched_actions_match_the_canonical_oracle_in_fewer_actions` was reviewed but could not be run natively here; the batch is proven end to end by the three media-route tests (re-exported PDFs decode to the same pages and objects, the independent oracle reads the same text and image samples).
5. The wasm was not rebuilt.

## Open and Save note (unchanged from `📂️pdf-open-and-save-route.md`)

The silent paged-natural stall (header 1 never staged) and its regression test `natural_source_route_opens_a_pdf_under_the_browser_turn_fuel` remain; that test passes in 68 turns here.

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs` (producer item grant)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs` (4,096 export actions per step, checkpoint coalescing for Encode and Decode)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/📸️export/🦀️.rs` (4,096-byte staging, batched cursor action)
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs` (`advance_batch`) and its `🧪️tests/🦀️.rs` (batched oracle test)
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs` (`renew_each_turn`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/👷️work/🦀️.rs` (`OriginalWorkFixture::hosted`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs` and `🧪️artifact-app-laws/🦀️.rs` (`settle_registered_typed_operation_within`, narrowing receipt grants)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📥️import/🔁️controlled/🦀️.rs` (identity-stream bulk grant and one-byte copy charge)
- `.../🧱️base/🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs`, `.../🔗️xref/📦️normal-projection/🦀️.rs`, `.../🗜️filters/🎚️decoded-stream/🦀️.rs`, `.../🗜️filters/🪣️stream-data/🦀️.rs` (`copies_identity_bytes` chain)
- `.../🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (hosted paged helpers, turn limits, cancellation test)

No `[DEBUG]` logging remains; the final run of the clean tree (`natural_file_route_opens`, `paged_natural_decode`, `natural_source_route_opens`) passed 7 of 7 in 90 s. Runner script: `large-documents-run.sh`.

## Paged streams and cancel close

### Paged stream output (open item 2 of the previous round)

Decoded stream bytes are now born, held, read and retired in 4 KiB pages, so no step needs a contiguous allocation above the browser turn grant (512 KiB copy, 1 MiB release, `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT`).

- Schema: `PdfObject::Stream.data` is `PdfStreamBytes = PagedBytes<{isize::MAX}>` (`stream_bytes(&[u8])` builds one for fixtures). Canonical tree, Value, controlled Value and retained-clone/retire/receive derive through the existing `PagedBytes` owners. New `ArtifactCanonicalPagedOctets` and `ArtifactCanonicalJsonTree::canonical_tree_octet_source` project stream octets to the borrowed Pack encoder as `IntrinsicBytesSource`, so re-export reads across pages.
- Framework: `PagedBytes::cold_reservation_bytes`, `cold_free_slots`, `cold_reserve_page`, `cold_push_slice`, `cold_retire_page` (one funded action per page: discard the leaf octets, then release the empty backing) and `PagedList::discard_plain_tail_leaf_cold`.
- `PdfStreamDataCursor` reserves one page per `NeedsAllocation` quote, pushes staged octets into the reserved slots and retires page by page on close; the doubling `previous` backing is gone. `PdfDecodedStreamCursor` pages its multi-stage inputs and retires them page by page. Normal projection, retained graph (raw streams, object streams, retiring values), compressed length and the xref stream section hold and release paged bodies.
- Consumers read across pages: image sample cursor and the sampled/DCT identity hashes (`ByteReadSource`, 4 KiB reads), DCT lookup (`&PdfStreamBytes` as `ByteReadSource`), object streams and ToUnicode/content tokens (`ByteReadSource`), native save stored-Flate and raw writer (4 KiB pages), foreign artifact identity, Pack export (`IntrinsicBytesSource`), image and shading preview and lifters (one `to_vec_owner` where a decoder needs a slice). The sqlite snapshot cell stays a contiguous `Cell::Blob` built by `to_vec_owner` because its owned `SqliteValue::Blob` is `Vec`-based.
- Page content: a stream is joined into the quoted `joined` buffer (the former borrowed `Single` input is gone), so a content stream above the copy grant still needs a paged tokenizer. Open.
- Tests: `identity_stream_output_is_born_and_retired_in_bounded_pages` (new, stream-data); `paged_natural_decode_*` helpers now run under `INTERACTIVE_COMPONENT` grants; `open_registered_pdf` opens through the sealed paged natural source with browser fuel and the interactive grants, so every media-route large test is a browser-grant test. `owned_cos_stream_*` pointer-reuse assertions became paged-octet assertions.

Measured with the browser grants (debug native): `paged_natural_decode_round_trips_a_large_original_image_exactly` 9 MiB: open 4,611 turns, reopen of the re-exported file 4,611 turns, all exact, 12.6 s for both tests together. Previously only the fixture's 1 GiB-copy grant passed.

### Cancel close

`NaturalFileDecodeCursor::close_chained` (trait default method, used by the natural-file close effect and by the test) runs quoted close actions inside one funded effect: every action gets `effect.nested(remaining grant)`, cumulative progress is subtracted, and the chain stops when the grant, the work or the quote is not admitted. Before this change: 7,189 close turns for 16 work turns (0.98 s test). The first chain stopped after any action without progress, and a close that discovers a `NeedsRelease` quote makes no progress, so it still took 7,215 turns (measured, test unchanged at the 32,768 bound). The chain now also continues when the decoder's own quote changed (`requoted != quoted`); that last change compiled but could NOT be measured: the single global cargo gate kept the run queued for over an hour, so I stopped it. The cancellation bound is therefore still `LARGE_DOCUMENT_TURN_LIMIT * 8` and the turn count after the requote fix is unmeasured. Next step: run `timed-run.sh t6 paged_natural_decode_of_a_large_original_image_honours_cancellation`, print the turns, and tighten the bound to the measured value.

### Pack batched oracle test

`semio-framework-pack-record-encode-cursor` is a member of the framework workspace (`🧰️framework/Cargo.toml`), not of the PDF workspace; `pack-cursor-test.sh <filter>` runs it there. `borrowed_pack_document_batched_actions_match_the_canonical_oracle_in_fewer_actions` passes (1 of 15, 0.63 s).

## Paged object roster

Status: measured and designed, the paged tables are NOT implemented.

### Measured before (debug native, browser grants, generated flat document, 2 objects per page, `sd-base` run, 849 s wall incl. compile)

| pages | turns (intake + settle) | wall | decode steps | encode steps |
| --- | --- | --- | --- | --- |
| 120 | 841 (201 + 640) | 15.9 s | 130 | 3,215 |
| 600 | 3,548 (201 + 3,347) | 89.2 s | 659 | 18,345 |
| 1,600 | not measurable, the probe panicked before reporting | - | - | - |

Sizes: `PdfPage` 632 B, `PdfIndirectObject` 120 B, `PdfAdmittedStreamRole` 600 B. Settlement turns are about 5.3 per page at 120 and 5.6 at 600: the growth is linear, and 600 pages (3,548) is just below the 4,096 limit. Per-page cost is Encode: about 30 Encode steps per page against about 1.1 decode steps. Decode is cheap (about 1,900 units per page).

### Findings

- The roster `Vec<PdfIndirectObject>` is 120 B per object, so at the contract maximum of 8,192 objects it is exactly 1 MiB, the release grant; it fits. The 1,977,200-byte quote most likely belongs to the lift page table (`reserve_pages`: pages x 632 B plus roles x 600 B, with 1,600 pages about 1.0 MB plus 1.1 MB), not the roster. Not yet confirmed by the 1,600 run.
- Any `Vec` backing above 1 MiB can be neither reserved nor retired under the browser grants (empty reservations are bounded by the release grant). `PdfSnapshot.pages` (1.0 MB at 1,600) and `admitted_stream_roles` (600 B each, plus 250 reserved) exceed that; so does the retained clone of the snapshot per edit.
- `XrefIndex` growth (32 B slots, doubling) quotes 512 KiB at 4,097 entries, over the 524,160-byte copy boundary; the page-tree visited set has the same limit.
- The lift parses every page content twice (`Mirror` stage) and stores the operators again in `admitted_stream_roles`, which roughly doubles the Pack that Encode exports byte by byte.

### Design (open)

1. `PdfSnapshot.pages` and `PdfArtifact.pages` become `PagedList<PdfPage, {usize::MAX}>` (wire shape and the JSON, TS, GraphQL and proto documents unchanged; the framework has the value, retire, clone, receive, Pack and DSL impls). `admitted_stream_roles` likewise. About 700 `.pages` use sites; the model and xref crates need a `semio-framework-value-persistent` dependency.
2. Lift: reserve one payload page per funded unit (`next_allocation_bytes` then `reserve_one_cold`), take the page limit from `/Count`, push with `push_reserved`.
3. `XrefIndex` slots become a `PagedList<Option<XrefRow>>`; growth and `close_step` work one page per unit (`discard_plain_tail_leaf_cold`, `release_empty_page_leased`).
4. Turn cost: drop the duplicate operators and the second parse, and batch Encode events within the turn.
5. Editor tests for 1,600 pages (open, insert text on page 1, Save incremental, reopen) and cancellation mid-open.

### Tests added

`open_turns_are_proportional_to_the_page_count_at_120_pages` and `..._600_pages` (bound: 256 + 8 turns per page), through the registered app under the browser contract. They have not been re-run since the debug output was replaced by assertions.

### Encode turn cost: what binds a turn (measured, 120 pages, two instrumented runs)

- Every Encode job step ends by `yield` with fuel 4,096 to 0: a step is bound by the 4,096-unit step contract (`naturalWorkUnitsPerStep`, pinned to the 7.5 ms step deadline), not by copy, capacity or items. A step runs 1 to 7 Pack actions and uses only 1 item per action (no hidden item overhead) and about 12 to 16 KiB of copy.
- A host turn ends after about 4 to 9 steps although it still holds 26 to 49 items and 310 to 510 KiB of copy: the cut is the turn's wall-time deadline (debug native build), not a grant. So "batch Encode events up to the available fuel" is already exhausted; more batching cannot lower the turn count.
- The only remaining lever is the number of events per page: 3,215 Encode steps x 4,096 = 13.2 M events for 120 pages, about 110 k events per page, against about 1,900 decode units per page. The duplicate operators (page content plus `admitted_stream_roles`) and the second lift parse (`Mirror` stage) are part of it, but removing them changes the role schema; not done.
- The temporary `[DEBUG]` instrumentation is removed again. Items B (paged tables), C (failed-open close), D (1,600-page and cancellation tests) are not done.
