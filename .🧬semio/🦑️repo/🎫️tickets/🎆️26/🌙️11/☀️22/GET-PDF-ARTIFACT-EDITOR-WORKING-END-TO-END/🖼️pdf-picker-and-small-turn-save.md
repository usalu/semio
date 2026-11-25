# PDF PNG picker end to end and small-turn Save (2026-11-24)

Follows `🧩️pdf-multi-resource-and-worker-jobs.md` (picker cause 3, native-save open item), `🏁️media-in-terminal-publication.md` (real-`PluginRuntime` shell simulation) and `💾️pdf-save-imported.md`. Every cargo run went through `native-cargo.sh` with `watchdog-cargo.py`, one cargo at a time, always filtered; no wasm rebuild. Logs under `🗑️generated/` (`small-turn-*.log`, `native-save-all.log`, `picker-*.log`).

## 2. `a_save_over_many_small_turns_writes_the_same_bytes_as_one_over_large_turns`

### Verdict: deterministic regression, not flaky, not the framework

Reproduced with the filter alone (`small-turn-1.log`, same `Some(false)`). Temporary `[DEBUG]` traces in `ClassicPdfSaveProgram` (removed) showed the path:

```
refuse route=Compact quote=true control=true proof=false blocked=(NeedsAllocation { bytes: 26891 }, 14032)
reference turns            <- reference reconciliation can never be built under 16 KiB copy
```

The exact proof lifts the candidate graph with the controlled lifter. Under the 16 KiB `SMALL_TURN` grant one lift unit may allocate 14,032 bytes (copy grant minus the inline body and `size_of::<PdfSnapshot>()`). The first font admission of the 40 page document asked for 26,891 bytes: `32 * size_of::<PdfFont>()` (840 B) plus the name and base font, because the multi-resource lifter (same day, `🧩️pdf-multi-resource-and-worker-jobs.md` §1) reserves the whole font table for 32 fonts at the first font. The same eager pattern was added for images (32 `PdfImage`), the role table (`pages + 32 + 64` roles, plus the new navigation role), graphics states and property lists (`MAX_STATES` each). A grant that cannot carry a unit twice makes the proof refuse; the reference route then cannot fit either (stall counter, `REFERENCE_TURNS`), so the plan declines. None of the framework changes named in the brief (scope plan top-up, multi-action receiving slice, worker step grant 4,096, `close_chained`) is involved: the Save program and test caller call none of them, and the large-turn Save of the same document passes.

### Fix: lifter tables grow when the document needs them, under the same quote

- `modules::lexer::{table_growth, grow_table}` (`🔤️lexer/🧱️containers`): one shared rule. The quote is the net growth (at least doubling, at least four slots, zero while capacity holds), and `grow_table` performs exactly that growth before the push. Nothing is reserved for a maximum ahead.
- Fonts (`🔤️fonts/🎚️controlled-opentype`): standard and embedded admission quote `table_growth(fonts, 1)` (+ `ROLES_PER_FONT` roles for embedded fonts) inside their allocation and grow before publishing; `font_slots`, `reserve_font_table` and `EXTRA_ROLES` are gone; an embedded font no longer defers (`ReferenceRequired`) when the role table is full.
- Images (`🖼️images/🎞️controlled`): the image and its role grow their tables in the publishing unit; `reserve_images` is gone; `reserves_empty` reports only sample backing.
- Lifter (`🧭️controlled`): `reserve_pages` reserves exactly the pages, then the content roles (one per page), then the page references, each in its own unit; the content role push, the metadata role splice, graphics states and property lists quote and grow their tables.

Result: the small-turn Save passes (`small-turn-4.log`, 88 s) and writes the same bytes as the large-turn Save. Whole native-save unit target: **15 / 15 pass** (`native-save-all.log`, 956 s). The lifter-sensitive editor laws `registered_pdf_with_several_fonts_images_and_inherited_resources_opens_like_both_reference_readers` and `registered_pdf_font_change_preserves_encoded_text_and_the_next_line` still pass (`picker-1.log`).

## 1. `registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit`

### The rewritten law (real `PluginRuntime`, simulated shell)

`ShellSide` in the PDF editor unit tests plays the browser shell against a runtime-hosted instance opened through the lifecycle (`pdf17_shard_host`, `shard_open_instance`): one event per turn, every typed-operation result page acknowledged (`shell_ack_of`), every other effect collected; `cross` reproduces `createOriginalMediaExchange` (offer until the exact receipt answers, keep turning while `Pending`, re-offer on `Retry`), `pick` reproduces `sendOriginalMedia` (Begin, 4 KiB `ORIGINAL_MEDIA_COMMAND_KIND` command pages, Complete, Cancel after a non-accepted frame). The document load, the action dispatch and undo/redo go to the host instance directly (the fixture route), everything else runs through `poll_kernel` turns.

The law asserts: `insert-image` (page 1, one-based) publishes exactly one `RequestFileOpen` (`image/png,.png`, raw bytes, single file, 512 KiB bound) whose arguments are only the `originalMedia` capsule (loan of this instance and document, hex operation/generation/revision, kind 240); a picker whose document changed before the file arrived (a rectangle on page 2) answers `Stale`, the shell cancels it and nothing is published; a second picker carries its own loan and the picked PNG publishes the image and soft mask (`PdfImageBody::Artifact` planes, names `SemioImage3/4`) and the `q cm Do Q` paint on page 1 only, page 2 untouched; replaying the delivered loan is `Stale`; one undo restores the loaded document exactly, redo the edit; the registered native Save carries both sample planes and the page-one paint (lopdf `reference_image_sample_streams`, `reference_page_resources`, `reference_page_operator_names`).

### Product bugs found and fixed on the way

| # | Symptom in the law | Cause | Fix |
| --- | --- | --- | --- |
| a | `natural-file import requires its owned declared media and no wire` | the reserved preparation always called `build_reserved_tool_job`; nothing ever called `build_original_picker_job`, so a picked file went to the natural-file importer | `ReservedPreparation::advance_builder` routes `ArtifactReservedToolInput::OriginalPicker` to `A::build_original_picker_job` (`🧵️instance/👷️jobs/📥️staging/🏷️reserved`); docstrings of both trait methods corrected |
| b | `import-media` stays in `Worker` (1 M identical steps) | `PngGraphEdit::append_placement` charged its copy bytes (4,336) as fuel; a worker step is loaned 4,096 | charge work units (one per placement row); bytes stay in the retained receipt (`🧩️graph-edit.rs`) |
| c | `original media requires its registered document actor, pages and causal issuer` (admission absent) | PDF document store owners had no original edit message encoder | `PdfMutation::document_store_owners` wraps its wire in `store::original_edit::with_original_edit_message`; `store` re-exports `original_edit` |
| d | same refusal (app contains `#`) | `is_atomic_backbone_tool` split `{app}#original-media` at the first `#`, and the store refused any app with `#`; every surface app id is `coordinate#role`, so no real app could publish a picked file | parse at the last `#` (like `parse_surface_app_id`), drop the store's `#` refusal |
| e | `max_file_bytes` 524,288 vs 24,546 | fixture held the inline-route bound; the shipped picker admits the paged PNG lane (`PNG_GRAPH_MAXIMUM_BYTES`) | fixture `🖼️local-png/🔣️.json` `bounds.maxFileBytes = 524288` |
| — | `artifact store has no installed owner disposer` | harness: the document was loaded before the runtime had driven the store constructor | `ShellSide::settle` also waits for `store_constructor_is_pending()` before the load |

### Still failing (open)

After fixes a to e, the stale-picker half of the law passes and the second picker's file is received, decoded and admitted for publication, then the turn fails with `original batch metadata and causal custody require the original Store batch abort port`. Traced (`picker-10..12`): the pending artifact publication is put into closing by `publish_mounted_typed_operation_unit` because `typed_operation_document_is_fresh` rejects the mounted `import-media` operation ("pending publication rejected a stale immutable document root"); it is neither `PreparationRefused` nor a domain failure. Closing a batch stage that already holds commit metadata has no implementation (the abort port), so the stale rejection surfaces as that invariant.

Hypothesis: the reserved picker operation is mounted with a generation/revision that is not the live store's after the rectangle, its undo and the first picker's cancel (the reserved preparation validates only the loan's revision, not the generation that `validate_commit` checks), so a correct picker is rejected as stale. The run that prints the mounted generation/base against the live store (`picker-13`) did not compile: a concurrent agent's in-progress edit broke `🌱️value/🛬️decode/🧳️owned/🧬️protocol` (`E0424`/`E0425`), not a file of this work. Next step: rerun `picker-13` once that crate builds; if the generation differs, mount the reserved picker operation on the live generation at donation (or validate the loan against it there); independently, the batch-stage abort port must exist so a stale publication closes instead of raising an invariant.

## Files changed

- `🔨️modules/🔤️lexer/🧱️containers/🦀️.rs`, `🔨️modules/🔤️lexer/🦀️.rs` (growth helpers)
- `🔨️modules/🔤️fonts/🎚️controlled-opentype/🦀️.rs`, `🔨️modules/🖼️images/🎞️controlled/🦀️.rs`, `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (tables grow on demand)
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (picker law rewritten against the real `PluginRuntime`, `ShellSide` shell simulation), `🧪️tests/🧫️fixtures/🖼️local-png/🔣️.json`
- `✏️editor/📄️document/🖼️image-authoring/🧩️graph-edit.rs` (placement fuel)
- `🧬️schema/🧬️mutations/🦀️.rs` (original edit message encoder)
- framework: `🔌️plugin/🧵️instance/👷️jobs/📥️staging/🏷️reserved/🦀️.rs` (picker builder routing), `🔌️plugin/📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs` and `…/🪟️surface/✏️artifact-editor-trait/🦀️.rs` (docstrings), `🏪️store/🦀️.rs` (`original_edit` re-export), `🏪️store/⚙️engine/…/🖱️store-cursor-operations/🦀️.rs` (no `#` refusal), `🔨️modules/📡️replication/🚪️io/💾️binary/🔗️causal/🧩️atomic-group.rs` (last `#`)
- all `[DEBUG]` traces of this work removed (`program.rs`, `🧵️reserved-picker.rs`, `📬️format-work.rs`, `🧩️graph-edit.rs`, typed-operation unit, store bundle and publication)
- `✏️editor/💾️native-save/🧱️program.rs`: only temporary `[DEBUG]` traces, removed again (no net change)

## Round 2 (2026-11-25): the picker law passes

Every cargo run went through `native-cargo.sh` with `watchdog-cargo.py`, one at a time; no wasm rebuild. Logs `🗑️generated/picker-r2-*.log`, `r2-*.log`. Two runs were lost to other agents' in-flight edits (`🌱️value/🛬️decode/🧳️owned/🧬️protocol` debug histogram, `🔌️plugin/🏛️ownership` projection rename, a `surface_contexts.len()` probe in the editor unit tests, the `⬇️lift` catalog split); each was repaired by its owner within the five-minute wait.

### 1. The hypothesis was wrong: the mount is bound to the live root

`[DEBUG]` at the picker admission (`picker-r2-3`) printed `op 384 base … gen 3 mounted_gen 3 live_gen 3` and the live revision equal to the base: the reserved picker operation is mounted on the live generation and revision after the rectangle, its undo and the cancelled first picker. No rebinding was needed; the loan check at `take_input` plus the phase-7 frontier check of the reserved mount already bind it.

The actual cause was in the publication unit (`📬️typed-operation-unit`): the per-turn document freshness gate ran **before** the branch that retires a closing pending publication. A picked file commits through the original route (`advance_original_commit`), which keeps its `commit_metadata` (the swapped history stacks, revision accumulator and displaced reservation) on the stage after the commit. The committed publication then had to retire on later turns, and every one of them first re-ran the freshness gate against a store frontier that had moved on, so a committed edit was "rejected as stale"; closing it then reached the generic stage close, which refuses Store custody (`… require the original Store batch abort port`), and nothing ever called the Store's existing abort port (`ArtifactStore::prepare_apply_batch_close`, used only by the tool-run finalize).

### 2. Fixes

| Where | Change |
| --- | --- |
| `🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs` | `ArtifactStoreBatchStage::holds_store_custody` and `ArtifactStoreBatchPublication::holds_store_custody` (commit metadata or causal seed custody); the generic stage close names it |
| `🏪️store/⚙️engine/🧱️store-components/…/🏗️store-owners-retained-reads-and-retirements/🦀️.rs` | `handoff_batch_publication_close` keeps a publication that still holds Store custody with its driver (`false`), so maintenance never receives an owner it cannot close |
| `🔌️plugin/🧩️composition/📬️publication/🦀️.rs` | `PendingArtifactStorePublication::holds_store_custody` (artifact, config and draft lanes) |
| `🔌️plugin/🧵️instance/🏪️stores/📥️staging/🧾️typed-operations/🦀️.rs` | `InstanceStores::return_publication_custody`: one paid unit of the issuing store's abort port per turn (artifact, config, draft store) |
| `🔌️plugin/🧵️instance/🧾️typed-operations/…/📬️typed-operation-unit/🦀️.rs` | the closing branch runs **before** the cancellation and freshness gates (a closing publication only retires, it is never re-validated), returns Store custody first through `return_publication_custody`, then the generic close; a cancelled publication the Store keeps is abandoned (`Canceled`) and closes locally; the stale gate answers the coded, localized `interactive-job.stale` fault ("The document changed while this action was being applied — try again." / "Das Dokument hat sich beim Anwenden dieser Aktion geändert — erneut versuchen.") instead of `plugin.internal`, and a stale publication the Store keeps is abandoned and closes the same way |
| `🔌️plugin/📜️declaration/🎞️media/📥️original-media/🧾️picker.rs` | another transfer (new `seq`) under a loan that already delivered its file, or whose document moved, answers `Stale` before the transport shape check; it answered `Refused` (the replay half of the law) |
| PDF editor unit tests | the picker law's seed is a real PDF file graph (`decode_pdf(encode_pdf(composed))`, both XObjects painted so the fixture names `SemioImage1/2` survive) instead of a bare semantic snapshot: the native Save declines (`stdio.pdf.save-declined`) a document without an object graph by design (`retained_control_extent` needs objects and a trailer `Root`), a state no opened or new document has |

Run `picker-r2-4` showed the custody path working: `closing op 384 custody true` for seven turns (metadata and seed custody returned through the Store), then the generic close. Run `picker-r2-8`: **the picker law passes** (102.7 s): first picker `Stale`, second picker `Accepted` with image, soft mask and paint on page 1 only, replay `Stale`, one undo restores the loaded document, redo the edit, and the registered Save (proof deferred, reference reconciliation) carries both sample planes and the page-one paint (lopdf).

### 3. Framework law

New companion crate `semio-os-store-original-batch-abort-tests` (`🏪️store/🧪️tests/🛑️original-batch-abort/`, fixture `🧪️tests/🧫️fixtures/🛑️original-batch-abort/🔣️.json`, registered in the root `Cargo.toml`): a staged batch holding commit metadata and a causal seed is kept by the Store (handoff refused), refused by the generic close, returned through `prepare_apply_batch_close`, then accepted by the Store's maintenance handoff; generation, revision, history and snapshot are unchanged and the next gesture is admitted on the same frontier. It stages the custody directly (`empty_batch_stage`) because the shared store fixtures cannot drive the original commit route any more: `FixtureStoreReceiving` refuses original work (`original work requires its original caller loan` / `… original issuer and resolved quote`), `close_durable_publication` and cold maintenance refuse factory retirement (`factory state lacks a native work quote`), and `OriginalJobFixtureCaller::admit` fails its own payload-authority heap assertion (`left: None, right: Some(0)`). These are pre-existing breaks of the store test harness; `semio-os-store-tests-03` and `semio-os-store-durable-group-tests` do not compile at all (70 and 143 errors in their own laws). The real original commit route is covered end to end by the PDF picker law.

Final run of the law (`r2-store-abort.log`): every fixture expectation holds (handoff refused, generic close refused, custody returned, handoff accepted without custody, generation/revision/history/snapshot unchanged, maintenance queued, next gesture admitted); the binary then fails in the shared test-store teardown (`close_test_store` → `close_owned_step_cold`: `factory state lacks a native work quote`), the same harness break as above, which no store law in this tree passes today.

### 4. Verification (all `[DEBUG]` traces of this round removed; one pre-existing trace of another agent in the publication unit restored)

- `run-round2-picker-suite.sh` → `r2-editor-neighbours.log`: **7 / 7 pass** (263.9 s): the picker law, `registered_pdf_page_edit_publishes_and_undoes_redoes`, `registered_pdf_draft_publishes_once_refuses_stale_and_undoes_redoes`, `registered_pdf_with_several_fonts_images_and_inherited_resources_opens_like_both_reference_readers`, `natural_file_route_edits_title_and_inserts_text_then_saves_both_pages`, `media_in_of_a_small_pdf_then_edit_and_save_appends_to_the_opened_file`, `a_progressing_media_in_is_never_faulted_as_stalled`.
- `r2-native-save.log` (native-save unit target, once): **13 pass, 1 fail, 1 killed**. `the_large_type1_fixture_rotates_inserts_text_and_reopens` fails with `the Save plan settles or is cancelled: Some(false)` (the plan declines, `r2-type1.log`), and `thousands_of_objects_rotate_insert_text_and_reopen` exceeded the 15-minute watchdog. Round 1 passed 15 / 15; this round changed nothing in `💾️native-save` (only temporary traces in `🧱️program.rs`, removed again), while the lifter (`⬇️lift`, `🔤️fonts`, `🖼️images/🎞️controlled`, `🔗️xref`, `🗜️filters`, `🖋️content`) and `🏭️factory.rs` / `🧷️increment.rs` were edited concurrently tonight by the font-lifter work. Open for that work: the Type 1 proof declines and the 1,600-object Save slowed down.
- Pre-existing, not touched: the source-anchor law `🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧾️source-rejection-anchors-and-host-lane` searches the publisher for text it no longer contains (`let grant=`, `pending.begin_close()`), before and after this round.

### Files changed in Round 2

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/🏗️store-owners-retained-reads-and-retirements/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🏪️stores/📥️staging/🧾️typed-operations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📥️original-media/🧾️picker.rs`
- new: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🛑️original-batch-abort/` (`🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`), `🏪️store/🧪️tests/🧫️fixtures/🛑️original-batch-abort/🔣️.json`; root `Cargo.toml` member
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (picker law seed)
- temporary only, restored: `…/🏃️mounted-run-and-pump/🦀️.rs`, `…/🎟️picker-publication/🦀️.rs`, `✏️editor/💾️native-save/🧱️program.rs`; the companion `semio-os-store-durable-group-tests` and `semio-os-store-tests-03` registrations tried on the way were removed again
- ticket: `run-round2-picker-suite.sh`
