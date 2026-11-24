# Saving an imported PDF (open, edit, Save)

Scope: PDF 1.7 base editor native Save (`✏️editor/💾️native-save`). Result: an existing PDF that was opened from a file (paged natural source, retained original objects) can be edited and saved; the saved file reopens with the edit and both pages intact.

All native runs went through `native-cargo.sh`, one test binary at a time, always filtered. Logs live in `🗑️generated` (delete when the ticket closes). Helpers added: `run-save.sh` (editor integration tests), `run-native-save-unit.sh` (native-save crate unit tests), `pdf-save-imported-oracle.py` (pymupdf check).

## How native Save is planned and written

`PdfGraphSaveJob` (`🧾️job.rs`) drives `PdfSaveProducer` (`📤️producer.rs`) over a `PdfNativeSaveProgram` (`🧱️program.rs`, `ClassicPdfSaveProgram`):

1. Admit a retained candidate (`PinnedPdfGraph`, `🧬️graph.rs`): trailer control only, the retained objects stay borrowed from the snapshot.
2. Ordinary graft (`🪡️graft.rs`): quotes and builds only the paid replacement rows for changed page geometry/rotation and document Info. Unchanged objects are never copied.
3. Quote (`🧮️graph-quote.rs`): bounds the total output and refuses what the writer cannot spell (at most 256 objects, 4096 control bytes and 128 value nodes per object).
4. Exact proof (`🔏️proof.rs`): lifts the candidate graph with the controlled lifter and compares it with the typed snapshot lane by lane, so a graft that does not say what the typed document says is never written.
5. Write (`🌊️graph-writer.rs`): one bounded control buffer, stream bodies borrowed from the retained rows, a fresh classic xref table and trailer. Output goes through paid 4 KiB pages; every step spends turn fuel, reports progress and observes cancellation.

## Causes

1. **Imported rosters are in file order, the planner required ascending object numbers.** `two-pages.pdf` is laid out `1 2 5 6 7 8 4 3`; the paged natural decode (and the cold decode) keep that order in `snapshot.objects`. The graft step refused it (`GraftRefusal::Source`, the first `windows(2)` ascending check), and the quote, the lookup (`binary_search`), the xref rows and the writer all assumed the same. The planner turned the refusal into `ReferenceRequired`, which the producer reports as "no reference writer is registered". Blank and generated documents are written in ascending order, which is why only imported documents failed. A debug probe at every refusal site printed `graft refusal Source objects=[1, 2, 5, 6, 7, 8, 4, 3]`.
2. **Edits outside the graft's vocabulary had no writer at all.** Inserted page text (a new content stream), fonts, images and so on are not geometry or Info. The exact proof refuses them (`Semantic`), and the code path that should then reconcile the whole document did not exist (`PinnedPdfGraph::prepare_reference` was unused).
3. **Latent framework hazard.** The maintenance ladder scheduled `MediaRefusal` whenever `media_submission_refusal_terminal_is_empty()` was false, and that predicate counts any live `media_export_preparation` or `media_download_preparation`. One upkeep turn between two Save submission turns therefore closed the in-flight Save preparation, and the next submit created it again, forever.

## Fixes

### 1. Ascending view over a file-ordered roster (`🧮️graph-quote.rs`, `🧬️graph.rs`, `🪡️graft.rs`)

- `ObjectOrder`: an inline, allocation free table (`u16` positions, at most `MAX_OBJECTS`) sorted by object number with `sort_unstable_by_key`, refusing a repeated number. It is a plain `Copy` leaf for retirement (`artifact_retire_leaf!`), so closing it costs one fixed copy.
- `GraphLoan` gained `order`, `base_len`, `base(i)` (the i-th smallest number) and `base_index(number)` (binary search); `len`, `object`, `lookup`, `added_info` and `GraphQuote::start` are defined over that view. The overlay `remove` index is an index in the same view.
- `PinnedPdfGraph` builds the order once (`retained_candidate`, `prepare_reference`) and `retained_control_extent` refuses rosters above `MAX_OBJECTS` up front.
- The graft no longer refuses on file order; it keeps its real refusals (object 0, more than 256 objects, wrong page tree, geometry and Info limits).

Effect: the retained rows are never moved or copied. The written file lists the objects ascending, so the bytes are not identical to the original layout, but every untouched object is carried over value-identical (checked below) and the xref and trailer are rewritten from scratch.

### 2. Reference reconciliation fallback (`🧱️program.rs`, `🧬️graph.rs`, `♻️close.rs`)

`ClassicPdfSaveProgram::refuse()` replaces every `ReferenceRequired` of the ordinary route (graft refusal, quote refusal, trailer, proof `Shape`/`Semantic`/`Deferred`, stuck grants). The first refusal switches the program to the reference route once; a second refusal is final and still reports the original "outside the bounded native Save qualification" message.

Reference route:

1. `PinnedPdfGraph::reference_extent` bounds the capacity of a whole reconciled graph (retained rows, op-count based estimate for the typed lanes, times two plus slack). It is checked against the turn ceiling (capacity and copy bytes), and a stall counter (`REFERENCE_TURNS`) turns a grant that can never carry it into the ordinary decline instead of an endless pending job.
2. One funded construction (`MediaConstruction::built`) runs `PinnedPdfGraph::prepare_reference`, i.e. the production `reconcile`: unchanged retained rows survive untouched, moved lanes are grafted, only what no graft can express is regenerated (same code `encode_pdf` uses). The actual capacity is reported (`reference_capacity`) and must stay under the quote. The step charges `capacity/16` fuel afterwards, so a big document delays the next step across turns instead of being free.
3. The reference graph replaces the retained candidate as the active graph; quote and writer run over its ascending view. The control trailer and exact proof are skipped (the reconciliation is the authority).
4. Close: `ClassicPdfSaveRetirement` and `OwnedGraphParts` carry the reference graph and `reference_birth`, retired through the same paid close as every other owner.

Limits that remain, by design of this bounded planner: at most 256 objects, 4096 control bytes and 128 nodes per object, the reconciliation itself is one atomic funded step (not yet resumable inside, bounded by the quote and the turn ceiling), encrypted and xref-stream documents decline, a snapshot without a retained object graph declines before the candidate is admitted (as before).

### 3. `MediaRefusal` only closes what belongs to a refusal (`🪜️maintenance-ladder`, `🧵️instance/📄️docs/📥️staging/🖼️render`)

New predicate `media_submission_refusal_retained()` (a refusal, its place, its retirement or its identity is retained). The maintenance ladder schedules `MediaRefusal` on that predicate. When a refusal is retained the unit still closes the failing preparation first and then the refusal (unchanged order in `close_media_submission_refusal`). The close ladder keeps `media_submission_refusal_terminal_is_empty()` (any preparation counts), so a closing app still tears everything down. A successful submission already takes its preparation itself.

## Tests and oracles

Editor integration (crate `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests`, `🧪️tests/🔬️unit`; open through the paged natural source under browser turn fuel, Save through the registered media export handle and its paged download):

- `natural_file_route_matches_independent_rotation_and_retains_other_pdf_objects`: rotate page 2 by 90, Save; lopdf (`oracle_apply_mutation` on the input, then `project_pdf_1_7` of both) must project identically (page count, rotation, boxes, text, objectGraph), then reopen: rotation, page 1 text, Info.
- `natural_file_route_edits_title_and_inserts_text_then_saves_both_pages` (new): `set-info-field` title and `insert-text` on page 1 (one-based), Save, lopdf reads 2 pages, a `Tj` on page 1 and none on page 2 and the title; reopen shows the title, the author unchanged, the text object, page 2 equal to the original, fonts equal.
- `maintenance_between_save_turns_keeps_the_live_media_export` (new regression): same Save with an upkeep (`maintenance_step`) turn between every two submission turns. With the old predicate it fails (`registered PDF Save handle after 32768 turns`, verified by temporarily restoring the old line); with the fix it passes.
- `natural_source_route_opens_a_pdf_under_the_browser_turn_fuel` (existing) still passes.

Native-save crate unit tests (`🧪️tests/🔬️original-save`, driven through the paid pages and the paid close, own decoder as reader):

- `imported_file_order_roster_saves_a_rotation_and_carries_every_other_object` (new): asserts the roster really is not ascending, rotates page 2 directly in the snapshot, Save, reopen; every object except page object 7 is value-identical to the input.
- `imported_text_edit_falls_back_to_the_reference_reconciliation_and_keeps_retained_objects` (new): title and page text edit, falls back (proof `Semantic`), reopen: content, title, author, fonts, page 2 and the font object unchanged.
- `registered_segmented_save_writes_the_large_original_image_and_reopens_exactly` (existing, 9.4 MB image) still passes; its body became `native_save(&snapshot)`.

pymupdf (1.26.7, `uv run --offline --with pymupdf==1.26.7 python pdf-save-imported-oracle.py ...`) opens the two files the tests dump (`SEMIO_PDF_SAVE_ROTATED_OUTPUT`, `SEMIO_PDF_SAVE_IMPORTED_OUTPUT`): see results below.

## PDF 1.4 and the other 1.7 subsets

They do not share this planner. Only the 1.7 base editor registers `semio_s_artifact_stdio_pdf_native_save::factory::PdfNativeExportFactory` (`native_media_export`); the 1.4 and the a/e/h/ua/vt/x editors come from the kit macros and save through the unsegmented `produce_media`/`encode_pdf` path, whose reconcile logic is exactly what the new fallback reuses.

## Results

| Test | Result |
| --- | --- |
| `natural_file_route_matches_independent_rotation_and_retains_other_pdf_objects` | was red (graft `Source` refusal), passes |
| `natural_file_route_edits_title_and_inserts_text_then_saves_both_pages` (new) | passes |
| `maintenance_between_save_turns_keeps_the_live_media_export` (new) | passes with the fix; fails without it (no Save handle after 32768 turns) |
| `natural_source_route_opens_a_pdf_under_the_browser_turn_fuel` | passes |
| native-save `imported_file_order_roster_saves_a_rotation_and_carries_every_other_object` (new) | passes |
| native-save `imported_text_edit_falls_back_to_the_reference_reconciliation_and_keeps_retained_objects` (new) | passes (about 44 s debug, the exact proof runs before the fallback) |
| native-save `registered_segmented_save_writes_the_large_original_image_and_reopens_exactly` | passes (9,437,850 bytes) |
| pymupdf 1.26.7 on the saved rotation file | accepts: 2 pages, page 2 `/Rotate 90`, same media boxes, page text and Info as the input |
| pymupdf 1.26.7 on the saved text file | accepts: 2 pages, title `Edited Title`, author unchanged, `Hello` on page 1, page 2 empty |

Totals: 7 Rust tests green (4 editor integration of which 2 new, 3 native-save unit of which 2 new), 2 pymupdf oracle runs, lopdf oracle inside the editor tests (page count, projection, operators, title). The saved rotation file is 842 bytes and lists objects 1 to 8 ascending with a fresh xref (`/Size 9`) and the input's `/Root` and `/Info`; only object 7 differs (`/Rotate 90`).

Not run (out of scope or owned elsewhere): the three large `natural_file_route_opens_large_*` tests and `paged_natural_decode_round_trips_a_large_original_image_exactly` (known red for the Encode-stage and fixture reasons in `📂️pdf-open-and-save-route.md`), the wasm rebuild (another agent), a browser journey. A concurrent agent's edit of `full-operation` briefly broke settlement mid-session (`publication-stalled`); it cleared without any change of mine.


## Files changed

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/💾️native-save/🧮️graph-quote.rs`, `🧬️graph.rs`, `🪡️graft.rs`, `🧱️program.rs`, `♻️close.rs`
- `.../💾️native-save/🧪️tests/🔬️original-save/🦀️.rs`
- `.../✏️editor/🧪️tests/🔬️unit/🦀️.rs` (helper `save_natural_bytes`, two new tests, env dumps)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs`, `🧵️instance/📄️docs/📥️staging/🖼️render/🦀️.rs`
- ticket helpers: `run-save.sh`, `run-native-save-unit.sh`, `pdf-save-imported-oracle.py`

No `[DEBUG]` probe of this work remains in source.

# Follow-up: no 256 object limit, what is resumable, which route takes which edit

## 1. Where 256 came from and what replaced it

One constant, `MAX_OBJECTS = 256` in `🧮️graph-quote.rs`, sized four fixed inline structures and gated three checks:

| Place | Was | Now |
| --- | --- | --- |
| ascending view of a file-ordered roster (`ObjectOrder`, my first fix) | inline `[u16; 256]` | `Option<Vec<u32>>`, absent for an already ascending roster (the common case), otherwise exactly `4 * objects` bytes, counted in the quoted capacity of the retained candidate (`order_extent`) and in `retained_control_capacity` |
| writer xref offsets | `[u32; 256]` | paid `Vec<u32>` (`offsets_birth` construction, exactly `4 * objects` bytes), retired through `ProgramClose` |
| graft `seen: [bool; 256]` | duplicate page guard | removed: a repeated page leaf is caught by the exact proof, and the graft already refuses a second changed page row |
| quote roster checks, `retained_control_extent` | at most 256 objects | none |
| per-object control prefix | fixed 4096 byte buffer, at most 128 value nodes | the buffer is sized by the quote: `GraphQuote::prefix()` is the largest prefix any object needs (at least 4096, at most 1 MiB), nodes are bounded by the prefix limit, and the node walk is charged as fuel (`take_extra`) |

The second row of that table mattered as much as the first: with 4096 bytes and 128 nodes the `/Kids` array of a flat page tree above about 100 pages was refused, so real documents failed long before 256 objects.

The object count is now bounded only by the budget the host grants, quoted exactly before anything is allocated. The program refuses cleanly (the old "outside the bounded native Save qualification" decline, never a stalled job) when the quote exceeds `original_turn_ceiling()`: the control prefix must fit one turn's copy and capacity grant, the offset table must fit its capacity grant. What still scales with the document and must fit one turn's grant (measured with the 1,600 page document below, 32 KB `/Kids` prefix, 3,204 objects):

- the exact proof lifts the typed pages into one vector: `pages * size_of::<PdfPage>()` in a single allocation step (about 1.1 MB for 1,600 pages, so a 1 MiB turn copy grant blocks it; the unit test grant is 32 MiB, the decode path of the browser already runs with 16 MiB capacity grants);
- the reference reconciliation needs about twice the document's retained bytes (`reference_extent`).

Tests (crate `semio-s-artifact-stdio-pdf-native-save`, `🧪️tests/🔬️original-save`, own decoder plus lopdf through the new dev dependency on the test oracle):

- `thousands_of_objects_rotate_insert_text_and_reopen`: generated flat document, 1,600 pages, 3,204 objects, file-ordered (written in descending number order). Rotate page 1001 (bounded graft route), Save, lopdf: 1,600 pages, `/Rotate 90` on that page only; reopen: typed pages equal. Then add text to page 1 (reference route), Save, lopdf: `Tj` operators on page 1, rotation kept; reopen: pages equal.
- `the_large_type1_fixture_rotates_inserts_text_and_reopens`: the existing `largeType1` law (48 pages) with two content streams per page.
- pymupdf 1.26.7 (`pdf-save-large-oracle.py` over the dumps written when `SEMIO_PDF_LARGE_OUTPUT_DIR` is set) accepts both saved large files: 1,600 pages, page 1001 rotated 90, title and author kept, media boxes and the text of pages 1, 2, 1000, 1001 and 1600 unchanged, `Hello` present only on page 1 of the text file.

I did not run the editor-level open of a multi-thousand-object file: the paged natural open and the Encode stage still hit the per-job turn caps recorded in `📂️pdf-open-and-save-route.md` for large files, which is the open path, not the Save planner.

## 2. What is resumable, and what is not

Every stage except one advances in bounded steps that spend turn fuel, report their stage, and observe cancellation at the next step.

| Stage | Granularity |
| --- | --- |
| retained candidate admission | one construction (the order table sort for a file-ordered roster is inside it, uncharged beyond the construction's own work unit) |
| ordinary graft | one page per step (`GRAFT_WORK`), one row per change |
| quote | one object per step (`QUOTE_WORK` plus the node walk) |
| control trailer, exact proof (lift of the retained graph, then comparison) | the controlled lifter and one comparison op per step |
| offset table and control buffer | one construction each |
| write | one control action (one object prefix) or at most one 4 KiB span per step, into paid 4 KiB output pages, `CHECKPOINT_BYTES` progress checkpoints |
| **reference reconciliation** | **one atomic step** |

The reconciliation is the production `reconcile` (lift of the whole retained graph, graft rounds, read back): a library function over whole-document maps. It cannot be sliced without rewriting it. What I changed: it only starts in a turn that is funded for its whole quoted work (`capacity / 16 + objects` fuel, gated, a stage name `pdf-save-reconcile` is set, a wait counter turns a grant that can never fund it into the clean decline), its capacity is quoted exactly before it runs, and cancellation is observed before it starts and at the next step after it. It cannot be interrupted while it runs. That is a gap against the "progress and cancellation for expensive operations" rule, bounded by the quote, not closed. Slicing it needs one of: a page-by-page graft built on the controlled lifter (the page content stage described below), or the incremental update design in section 4.

Tests for the resumable stages (same crate):

- `a_save_over_many_small_turns_writes_the_same_bytes_as_one_over_large_turns`: the same Save under a 16 KiB copy grant (16 items) instead of 32 MiB; the bytes are identical and the turn count is the larger one (more than 200 turns for 40 pages).
- `cancelling_a_large_save_midway_leaves_no_complete_file`: a 300 page document; cancelling during planning writes no byte at all, cancelling during writing leaves a strict prefix without `%%EOF`, and in both cases the program and every owner retire through the paid close (the test would panic on an unpaid owner). The "no partial file" guarantee itself is the job's: `PdfGraphSaveJob` publishes the export result only when the writer reports done, and a cancelled job closes and retires its output pages (`step_turn` returns the cancelled outcome); this test shows the program side of it. A full editor-level cancel of a registered Save handle was not added.

## 3. Which edit takes which route

| Edit (typed lane that moved) | Route |
| --- | --- |
| page rotation, media/crop/trim/bleed/art box, user unit of a page of a flat page tree (at most 32 entries in the dictionaries involved) | bounded graft: one replacement row, everything else borrowed; exact proof afterwards |
| document Info (title, author, subject, keywords, creator, producer, trapped, plain ASCII up to 128 bytes), removing Info | bounded graft |
| no change at all | bounded graft path with no rows (rewritten ascending with a fresh xref) |
| page content (inserted or changed text, shapes), fonts, images, annotations, page insert/remove/move, nested page trees, catalog lanes (outlines, labels, destinations, open action), forms, anything whose typed lanes the proof cannot reproduce from the graft | reference reconciliation (atomic), then the same quote and writer |
| encrypted documents, xref-stream/object-stream output, snapshots without a retained graph, objects beyond the 1 MiB prefix or the grant | declined with the "outside the bounded native Save qualification" payload |

## 4. Recommendation: incremental update (append-only) for imported files

Recommended as the long-term design for every imported document, not implemented here.

Why it fits better than rewriting the retained graph:

- Cost is proportional to the edit, not to the document: the original bytes are streamed through unchanged (a byte copy that is naturally chunked and cancellable), then only the new and replaced objects, one new xref section and a trailer with `/Prev` are appended. The reconciliation's one atomic step and its twice-the-document capacity disappear: the delta is exactly the objects the graft and the per-page lowering produce, which the bounded graft already computes incrementally.
- It is byte-faithful by construction (the strongest form of "untouched retained objects are carried over"), keeps signatures valid, keeps encrypted and xref-stream/object-stream files saveable (the appended section can be a classic table with `/XRefStm` or an xref stream matching the file), and needs no object-count table, no roster order, and no 256 or 1 MiB-per-object structure: unchanged objects are never parsed again by the writer.
- The ordinary graft rows become the appended objects directly; only the quote (offset of each appended object, which are few) and the xref subsections remain.

What it needs first: the original byte source on the save route (the paged natural source the open already holds, retained for the document's life, with a revision-bound lease and release when the document closes), a retained map object number to original generation/offset only for the replaced numbers, and a policy for growth (an explicit "Save as a compacted file" command that runs today's full writer, so histories of repeated appends do not grow without bound). Costs and risks: files grow with each save until compacted, replaced content streams stay as dead bytes, and the document must keep the original source alive across the session, which the local-first source custody already does for the open handle.

Order of work I would take: (1) per-page content graft on the controlled lifter so the common content edit leaves the atomic reconciliation, (2) the append-only writer on top of the same rows, (3) keep today's rewrite writer as the compaction path.

## Follow-up results

Native-save crate tests, each green in a filtered native run (the 4 earlier ones were re-run unchanged after the refactor): `imported_file_order_roster_saves_a_rotation_and_carries_every_other_object`, `imported_text_edit_falls_back_to_the_reference_reconciliation_and_keeps_retained_objects`, `registered_segmented_save_writes_the_large_original_image_and_reopens_exactly` (9.4 MB), `the_large_type1_fixture_rotates_inserts_text_and_reopens` (new), `cancelling_a_large_save_midway_leaves_no_complete_file` (new), `a_save_over_many_small_turns_writes_the_same_bytes_as_one_over_large_turns` (new), `thousands_of_objects_rotate_insert_text_and_reopen` (new, 3,204 objects, about 125 s in a debug build). pymupdf accepted both large saved files. Three harness findings worth keeping: the fixture caller's `business_fuel` is a lifetime stock (a 1,600 page proof exhausts 2^25, now 2^40), the proof's page vector and the reconciliation both need a per-turn copy grant of the order of the document (32 MiB in the test caller), and small turns are modelled with a small grant, not renewed fuel (`renew_each_turn` breaks the original clock binding).

Not verified after the last edits: the final combined run of all seven crate tests and the four editor integration tests. A concurrent agent is migrating PDF stream data to `PdfStreamBytes` (it already adapted `🧬️graph.rs` and `🌊️graph-writer.rs`, and the fonts crate did not compile at the end of my session), so every native build after that point failed in crates I did not touch. The seven crate tests had all passed individually before that migration; the editor-level integration tests were last green before this follow-up. Re-run both filters once the migration builds.

# Incremental update save

Status in one line: the append-only writer, the per-page graft planner and the program route exist in `✏️editor/💾️native-save/` and pass 8 program-level tests with lopdf and pymupdf; the retained original bytes are not yet carried by the snapshot, so the registered Save job still takes the compact route in production until that lane is wired (see "What is not wired").

## Design

Save has two routes, chosen once per Save by `ClassicPdfSaveProgram` (`Route::{Undecided, Increment, Compact}`):

- **Increment** (`🧷️increment.rs`, `📎️original-tail.rs`, append mode of `🌊️graph-writer.rs`): the retained original bytes are copied unchanged in 4 KiB steps, followed by only the changed and new objects, one new cross-reference section and a trailer with `/Prev`. A classic table continues a classic original, an xref stream (`/Type /XRef`, `/W [1 4 2]`, `/Index` runs, `/Prev`) continues a stream original. Object numbers and generations are never touched, new objects are numbered from `max(/Size, highest retained number) + 1`, free entries are never rewritten, objects that lived in object streams are re-emitted as plain objects that override the compressed entry. Encrypted originals decline (see below). A document nothing changed in is saved as the original bytes alone.
- **Compact** (unchanged): today's ordinary graft, exact proof, quote and full writer, plus the reference reconciliation. It is used for new documents (no original), and whenever the increment route is not applicable.

### Planner, one bounded step per unit (`IncrementPlan`)

1. `Tail`: read the last 4 KiB for `startxref` and the trailer, the first 4 KiB of the last section for its kind, `/Size`, `/Root`, `/Encrypt`. The tail root must equal the snapshot's root.
2. `Baseline`: hash the retained graph (ascending object order, stream bodies in 4 KiB chunks, 64 KiB per step) and compare with the digest taken when the document was opened. Raw object edits (`set-indirect-object` and friends) change the digest; they cannot be spelled by the typed graft, so the Save goes compact instead of losing them.
3. `Document`: one slice with zero pages lifts the catalog lanes, Info and document id; a moved catalog lane, a regenerating lane or a moved document id goes compact; a moved Info becomes one Info row (verified by lifting it back).
4. `Pages`: one step per page. A slice (the page, its tree ancestors with only that page as a kid, a synthetic catalog, the resource dictionaries) is lifted with the production lifter through a sparse `ObjectSource` over the retained rows (no clone of the document). If the lifted page and its resources equal the typed ones the page is clean. Otherwise the production `Graft` (new `Graft::sparse`, `Moved::between` made public) patches the slice, the patched slice is read back and must spell the typed page exactly (the same fixed-point law as `reconcile`). Only the page dictionary row and the new objects (content stream, annotations) become rows. This covers inserted and edited content, geometry and rotation, annotation edits.
5. `Coverage`: every typed resource id must have been seen by some page slice, else a resource was added or changed (image insert, image replace, font edits) and the Save goes compact.

Each step charges fuel (`max(4096, bytes read / 16)`), pays the retained rows it keeps through `consume_retained` and checks the rows against the turn's copy and capacity grant before keeping them (rows larger than the grant go compact). Cancellation is observed between steps by the job; nothing is published before the writer reports done (the job's existing rule), so a cancelled Save publishes nothing. Rows are retired through the same paid close (`IncrementRows` in `OwnedGraphParts`).

### Quote and writer

`GraphLoan` gained `resolver` (the original graph resolves references of appended rows), `GraphQuote::start_append` bounds `original + rows * 48 + 4096 + object bodies`, `ClassicGraphWriter::start_append(.., AppendFrame)` adds the phases `Copy`, `Separator`, `XrefRun`/`XrefRow` (classic runs) and `StreamOpen`/`StreamRun`/`StreamClose`/`StreamBody`/`StreamEnd` (stream section, rows written in batches that fit the control buffer). Every phase is one bounded step that spends fuel and reports progress through the existing producer and job.

### Faults

The compact-route decline now carries the coded reason `stdio.pdf.save-declined`; `PdfFault::SaveDeclined` (`📄️document/🚨️faults`) gives it an English and a German notice, so the shell shows a localized message and never the developer detail. Encrypted originals and documents whose compact write exceeds the grant end there, with nothing written.

## Tests and results

All in `💾️native-save/🧪️tests/🔬️original-save/🦀️.rs`, driven through the program, the paid pages and the paid close (`run-native-save-unit.sh`, one filtered native run, 453 s debug), lopdf (`semio_s_artifact_stdio_pdf_test_oracle`) as independent reader, pymupdf 1.26.7 (`uv run --offline`, `inc-oracle.py`) on the dumped files.

| Test | Result |
| --- | --- |
| `incremental_two_pages_title_and_text_append_to_the_original` (original exact prefix, under 4096 bytes appended, lopdf: +1 `Tj` on page 1, page 2 operators equal, all other objects value-identical) | pass |
| `incremental_rotation_appends_one_page_row` | pass |
| `incremental_without_edits_writes_the_original_bytes` | pass |
| `a_raw_object_edit_falls_back_to_the_compact_route` (digest refuses the increment, compact write reopens) | pass |
| `incremental_over_an_xref_stream_original_appends_a_stream_section` (`/XRef` section appended, lopdf reads it) | pass |
| `incremental_text_insert_on_1600_pages_completes_under_the_browser_grant` (`BROWSER_TURN`: 64 items, 512 KiB copy, 16 MiB capacity, 1 MiB release; over 1,600 funded turns; original prefix, under 4096 bytes appended, lopdf 1,600 pages with the text on page 1, reopened typed pages equal) | pass |
| `cancelling_an_incremental_save_midway_publishes_nothing` (cancel in the page plan: no byte; cancel in the write: never done, shorter than the full file) | pass |
| `a_second_incremental_save_stacks_on_the_first` (first file exact prefix of the second, both edits, lopdf reads the two-section chain) | pass |

pymupdf: `incremental-two-pages.pdf` 2 pages, title "Edited Title", "Hello" on page 1, page 2 empty; `incremental-xref-stream.pdf` 2 pages, "Hello" on page 2, title unchanged; `incremental-stacked.pdf` 2 pages, title "Second", "Hello" on both pages; `incremental-large.pdf` 1,600 pages, page 1 "Page 1 / Hello", page 2 "Page 2", title "Generated" (file 439,156 bytes).

The 7 older crate tests were not re-run in this pass (the compact route is unchanged apart from `GraphLoan.resolver` and the writer `step` signature; the crate and all its tests compile).

## What is not wired

- The snapshot does not carry the original bytes: `PdfSnapshot` has no lane for them, the natural-file decoder retires its paged source after decoding, and `ArtifactMediaExportJobRequest` holds only the snapshot. The program takes them through `ClassicPdfSaveProgram::with_original(OriginalBase { source, objects })`; `PdfGraphSaveJob` does not call it yet, so the registered Save still writes compactly. Closing that needs a persisted lane (original bytes in paged form plus the retained-graph digest from `increment::objects_digest`, filled by both decoders, ignored by value equality and diff) which touches the schema, diff, sqlite, text, proto and json files; it was not started because a snapshot lane is a cross-crate schema change while the stream-data migration is in flight.
- After a Save the host must rebase the retained original to the saved bytes (and the digest to the saved graph) before the next Save of the same open document; the test for a second save reopens the first result.
- Not expressible as an appended section (go compact): page insert, remove and move, nested page trees, outlines and other catalog lanes, forms, image insert and replace, font edits, raw object edits, document id edits, encrypted originals (decline).
- The compact route's reference reconciliation is still one atomic step; it is only reached when the increment route is not applicable.

# Production wiring

## Decision: local custody keyed by the retained-graph digest, not a snapshot lane

Two options were weighed.

- **Persisted snapshot lane** (original bytes as paged octets in `PdfSnapshot`). Rejected for now. It would double every snapshot clone, history retention and replication payload (the retained graph already holds every decoded stream). The Pack export, sqlite cell, text grammar, binary protocol, native JSON, diff, proto, graphql and ts twins would all need a second paged-bytes projection. The decisive point is rebasing: a Save never changes the document revision (the Save tests assert it), so "the saved file is the new base" is local, ephemeral state, not a shared event. In a lane it would need a new document mutation dispatched by the shell after every Save.
- **Retained natural-file source owned locally, addressed by content** (chosen). The original file is held in 4 KiB shared pages (`FilePages`, never one allocation) in a bounded local custody (16 entries, 128 MiB, least recently used out), keyed by an order independent SHA-256 of the retained object graph (`GraphDigest`). No schema, json, proto, graphql, ts or sqlite change was needed: the digest is recomputed from `snapshot.objects`, which every replica already has.

Multi-user: another replica receives the typed document and the retained graph, not the local file. Its Save finds no custody entry and goes compact, so it can always Save. A replica that opened the same file (same digest) appends. Correctness never depends on the custody: a miss, an eviction or a digest mismatch (raw object edits) is the compact route. Limit: custody is a cache, not exact document-lifetime ownership; closing the document does not evict.

## Wiring

- `🚪️io/📥️import/🗃️original-custody` (new): `FilePages`, `Stash` (one page per step), `GraphDigest` (4 KiB per step, row hashes summed so the roster order does not matter), `retain`, `find`, `rebase`, `Appender`.
- Both open routes and both decoders fill it. Controlled decoder (`NaturalPdfDecoder`, media-contiguous and paged natural source alike): new phases `StashFile` (after the file is prepared, source still alive) and `DigestGraph` (before publish), registered in `publish_snapshot`. Synchronous decoder (small files): `decode_natural_file` of the 1.7 editor registers after `decode_pdf`. Each step is one funded work unit and observes cancellation; the stash is unreported local memory.
- `IncrementPlan` now hashes first, then resolves its base: given (tests) or `custody::find(digest, document)`; a miss is `Compact::Retained`. New stage `Restore`: object numbers replaced by earlier appended sections are re-stated from the retained graph when the current edit equals the retained value (undo, title edited back), so stacking never keeps stale earlier sections.
- `ClassicPdfSaveProgram::with_document(parent_document_id)` is set by `PdfGraphSaveJob`; the writer's sink is teed so the appended bytes are collected; `commit()` (called when the Save completes, never on cancel) rebases custody: file = previous file plus appended pages (shared), digest unchanged (the retained graph does not change), touched numbers accumulated.
- Choice per Save: increment when custody holds the file, the digest matches and every edit is expressible; compact otherwise; `stdio.pdf.save-declined` when the compact route cannot write either (unchanged).
- PDF 1.4 has no native Save job. It would need the same custody fill in its decoder and an append writer over its retained objects.

## Tests (all through the registered app, `✏️editor/🧪️tests/🔬️unit`)

`natural_file_route_save_appends_to_the_opened_file_and_stacks_the_second_save`, `natural_source_route_save_appends_to_a_120_page_document_under_the_browser_grants`, `cancelling_an_opened_pdf_save_midway_publishes_nothing_and_keeps_the_base`, `a_new_pdf_document_still_saves_compactly`; regression: `natural_file_route_edits_title_and_inserts_text_then_saves_both_pages`, `maintenance_between_save_turns_keeps_the_live_media_export`. Files written with `SEMIO_PDF_INCREMENTAL_DIR` can be checked with `uv run --offline --with pymupdf==1.26.7 python inc-oracle.py`.

## Files changed

`🚪️io/📥️import/🗃️original-custody/🦀️.rs` (new), `🚪️io/📥️import/🦀️.rs`, `🚪️io/📥️import/🔁️controlled/🦀️.rs`, `✏️editor/🦀️.rs`, `✏️editor/💾️native-save/🧷️increment.rs`, `🧱️program.rs`, `🧾️job.rs`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.

## Results (native, debug, filtered, one cargo at a time)

All 6 editor tests pass (run 3: 4 of 6, the two failures fixed and re-run green: cancel 1/1 in run 4, 120-page 1/1 in run 6, 30 s). The stacking test also covers prefix, lopdf page count, `Tj` operators, title, custody equals the saved file, and reopening the second file. The native-save crate's own tests and the pymupdf oracle on the new files were not re-run.

Deviations and open items:

- The 1,600-page document could not be used at editor level. Open itself fails under the browser grants: 1,600 pages need a 1,977,200-byte contiguous object roster (`PDF decoder needs a 1977200-byte backing grant`), and 600 pages exceed the 4,096-turn open limit. Both are existing open-side limits (roster not paged), not Save. The editor test uses 120 pages under browser fuel and grants for open, edit and every Save turn. The 1,600-page Save under the browser grant remains proven at program level (`incremental_text_insert_on_1600_pages_completes_under_the_browser_grant`).
- After a cancelled Save the next submission needs upkeep turns between submits (`save_natural_bytes(.., upkeep = true)`) so the cancelled operation closes first.
- An increment quote refusal (stacked file over the output ceiling) still declines instead of falling back to compact.
