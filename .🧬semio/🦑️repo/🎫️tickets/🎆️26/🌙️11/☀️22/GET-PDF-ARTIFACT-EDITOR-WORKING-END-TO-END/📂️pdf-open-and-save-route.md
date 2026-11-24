# PDF open and Save route (2026-11-23)

Native evidence: every run went through `native-cargo.sh`, one test binary at a time, always filtered (`-p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- <filters> --test-threads=1`). Logs: `🗑️generated/route*.log`, `🗑️generated/final*.log` (delete when the ticket closes).

## Problem 1: browser Open File never publishes `file-opened`

### What the native route showed

The browser open goes through the sealed paged natural source (`NaturalFilePublication` -> `advance_original_reserved_natural_source` -> `advance_reserved_paged`), not through `consume_media`. No native test used that route, so the earlier native "open works" results never covered it. A new test now does (below). Its first run did not hit any work quote at all: it stalled **silently** inside `advance_reserved_paged`:

- state at the stall: preparation present, input donated, request/body/job absent, staged headers `[0,2,3,5,6,7]` true, header `1` false;
- `prepare_reserved_header` copies header 1 (the tool id) but stages only `[0,2,3,5,6,7,8]`; the media route stages header 1 itself in `prepare_reserved_media_header`, the action route consumes it in `begin_action`, and the paged route (natural source and picker, both through `advance_reserved_paged`) did neither;
- `assemble_reserved_request` requires `[0,1,2,3]` and returns `Ok(false)` forever, so the request, the job and `file-opened` never appear. No denial, no error, no progress: an unbounded silent loop of host turns.

### Fix (framework)

`🧵️instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs`, `advance_reserved_paged`: stage header 1 right after `prepare_reserved_header`, exactly like `prepare_reserved_media_header` does for the media route.

### The 1,048,577 quote

Searched for the requester and could not pin it natively:

- every decoder quote path compares against the turn's `fuel_remaining` (`OriginalWorkLoan::admits` reaches `OriginalJobWorkRecipient`, `ActualWorkTurn`, the encoder recipient and the job native-turn context; all of them test `observed + additional <= fuel`);
- with a temporary probe in `OriginalWorkLoan::admits` (printing a backtrace on the first denied quote) the open of `two-pages.pdf` through the paged natural route, under the browser turn fuel of 1,044,891 (`(1 << 20) - 3_685`, the value in the console), produced **zero denied quotes** and completed (the new regression test passes);
- in the browser console the denial starts right after the Save's typed operation retires (`typed-operation slots live=0/64`) and the quote repeats at constant `fuel=1044891`, so it is a fresh quote per turn, not a growing one;
- the staged wasm is older than the tree (it still carries the `ready=true ...` debug print that no source file contains any more) and wasm32 changes `usize`-derived quantities (`usize::BITS`, `size_of`), which a 64-bit native run cannot reproduce.

So: **the silent stall above is the reproducible and now fixed cause of "Open File never publishes"; the 2^20+1 quote is not reproducible against the current tree natively and has to be re-checked against the rebuilt wasm.** I did not add a generic "clamp the quote" because a quote above one turn's fuel is only wrong relative to the turn's starting fuel, which the recipient (`OriginalJobWorkRecipient`) does not know; a clamp at that point would also turn legitimate "not enough left in this turn, retry next turn" denials into faults. If the rebuilt wasm still denies, the probe line to re-add is in `🌱️value/🎟️control/🧬️contract/🎟️work-action/🤝️loan/🦀️.rs` (`admits`): print `additional`, `available` and `std::backtrace::Backtrace::force_capture()` when `observed + additional > available`; the first frame above `OriginalWork::prepare` is the requester.

### Regression test

`natural_source_route_opens_a_pdf_under_the_browser_turn_fuel` (editor unit tests): opens `two-pages.pdf` through the paged natural source (`NaturalFilePagedSource::retain`, `DispatchCtx::advance_original_reserved_natural_source`, then `settle_registered_typed_operation`) with the browser's turn fuel, asserts two pages, settles once with a `Full` UI dirty scope, no `LoadDocument` effect, and closes the app. Before the fix it stalled for all 32,768 turns (`mounted PDF media intake stalled`); after the fix it passes (6 s). `open_registered_pdf` became `open_registered_pdf_through(.., PdfIntake, fuel)` (`PdfIntake::{Media, NaturalSource}`); `open_registered_pdf` keeps its signature for the existing tests. The decoder-level test `paged_natural_decode_of_the_browser_contract_terminates_with_every_page` (turns <= 128) stays.

## Problem 2: Save never gets a handle in the rotation test

### Gate

`submit_media_export_checked` -> `receive_owned_media_export` -> `submit_owned_media_export` all passed their gates (fuel, deadline, cancel, grant all fine, registries admitted, slot 0 vacant, progress fits). The creation branch ran on every call, `media_export_preparation` was `None` on every call: the freshly created preparation (66,256 bytes) was destroyed between two submit calls.

### Cause: test harness, with a latent framework hazard

The harness loop called `app.maintenance_step(..)` after every not-ready submit. The maintenance ladder unit `MediaRefusal` is scheduled whenever `media_submission_refusal_terminal_is_empty()` is false (`🪜️maintenance-ladder` unit 11), and that predicate counts **any** `media_export_preparation` as pending refusal cleanup. So maintenance retired the live Save preparation immediately after each creation, and the next submit created it again, forever (32,768 turns, command pipeline idle). The standard fixture `submit_fixture_media_export` never interleaves maintenance and works; the browser Save works because its turns are not interleaved with maintenance either.

Fix: harness only. The Save admission loop of `natural_file_route_matches_independent_rotation_and_retains_other_pdf_objects` no longer calls `maintenance_step` between submits (same shape as `submit_fixture_media_export`). The test now also opens (and reopens) through the paged natural source, i.e. the browser route.

Latent framework hazard (not changed, needs the owner of the maintenance ladder): the `MediaRefusal` rung should only close a preparation that belongs to a retained refusal or to a closing app, not a live in-flight export, otherwise any maintenance turn between two Save turns cancels the Save. Suggested fix: a separate predicate (`refusal slots or retirement present`) for the maintenance unit, keep `media_submission_refusal_terminal_is_empty` for the close ladder.

### Next blocker behind it (not fixed)

With a handle, the poll now fails: `media.payload: the PDF object graph is outside the bounded native Save qualification and no reference writer is registered for Save`. Source: `✏️editor/💾️native-save/🧱️program.rs:92`, the ordinary-graft step (`self.graft.step(..)`) returns `Source`, i.e. the retained original source graph of the imported document is not available to the graft after import and edit, so `PlanDecision::ReferenceRequired` and the producer declines. Blank documents save in the browser (journey), imported documents with retained objects do not. This is a PDF Save feature gap (retaining the original source across import, edit and Save), independent of the two problems above; the rotation test will stay red until it is closed.

## Large documents (not resolved, measured)

Final run (`final1.log`): the three `natural_file_route_*` large tests and `paged_natural_decode_round_trips_a_large_original_image_exactly` still fail; the paged-natural fix does not touch them (they run the media route and the contiguous decoder). `natural_file_route_opens_reexports_and_reopens_large_sampled_images` was not part of that run (its name does not match the `natural_file_route_opens_large` filter).

Measured with temporary probes (removed again):

- `natural_file_route_opens_large_retained_vector_document_and_reexports` (49,279 byte input): the Decode stage needs about 40,000 job calls (not 32,768) because each call advances only about 13 to 20 work units although the job turn has 4,096; the decoder returns early between allocation and transfer events. After the decode completes (98 objects, 4,787,830 owned bytes) the job enters the Encode stage (pack export of the snapshot) which advances **one work unit per job call** (`publication_work` 1,585,600 after 1,630,208 calls and still counting). So the settlement cap of 32,768 turns ("close owner Begin" is only the not-yet-started close owner, the job is still pending) is hit long before Encode can finish; the Encode stage turn efficiency (`advance_pack_export` / `note_publication_work` in `📜️declaration/📥️natural-file`, framework sqlite pack export) is the real blocker for large documents and is in the pack/sqlite export owner's area.
- `paged_natural_decode_round_trips_a_large_original_image_exactly` (9,437,877 bytes): harness defect. `OriginalWorkFixture::new()` has a total source work stock of 2^20 units (not renewed per turn). The first turn spends it (`fuel=1,044,479` at the first step); every later turn has `fuel=0`, `should_yield()` is true, and the decoder's first loop iteration, which always runs, advances exactly one unit (`delta=1`, phase `prepare-file`, `bytes/project/normal/decode/5`). 4,000,000 continuations therefore cover about 4 M of the 9.4 M units needed. A try with a 2^32 unit stock (a temporary `OriginalWorkFixture::with_fuel`) did not change the outcome within 18 s, so another cap (the per-turn `StepBudget` path in `🏃️headless/🎟️caller`) also applies; reverted, needs the fixture owner. The contract of the helper (`resumable(len, 16_384, 1, ..)`, one unit per step) is also far below the browser contract (4,096).

## Test results

| Test | Result |
| --- | --- |
| `natural_source_route_opens_a_pdf_under_the_browser_turn_fuel` (new) | passes (6 s); stalled for 32,768 turns before the header fix |
| `paged_natural_decode_of_the_browser_contract_terminates_with_every_page` | passes |
| `natural_file_route_matches_independent_rotation_and_retains_other_pdf_objects` | Save handle obtained now; fails at the Save poll (`Source` graft refusal, `ReferenceRequired`) |
| `natural_file_route_opens_large_retained_vector_document_and_reexports` | fails (settlement cap, Encode stage one unit per call, see above) |
| `natural_file_route_opens_large_retained_type1_document_and_reexports` | fails (same) |
| `natural_file_route_opens_reexports_and_reopens_large_sampled_images` | not run in the final pass |
| `paged_natural_decode_round_trips_a_large_original_image_exactly` | fails (fixture work stock, see above) |

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs` (stage header 1 for the paged routes)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`PdfIntake`, `BROWSER_TURN_FUEL`, `open_registered_pdf_through`, new test, rotation test: paged intake, no maintenance in the Save loop). No other source files are left modified: every probe and the attempted fixture/contract changes for the large image test were reverted.

All temporary `[DEBUG]` probes added during this work were removed again (denied-quote probe, Save gate/stage/create/prep dumps, natural dispatch, reserved paged, decline and reference-required prints, the `SEMIO_DEBUG_TURN_FUEL` override).
