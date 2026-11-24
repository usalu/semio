# Shell wgpu test crates chrome-1, chrome-3, input-2, input-3: handoff

Engine root `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`. Logs: `/private/tmp/claude-501/-Users-ueli-Documents-semio/0fec7092-b872-4c05-9d02-7a8aa493f8d0/scratchpad/shell-ci/` (`a1`..`a5` iteration runs, `b1` final run of the four crates, `b2`/`b3` final run of the already-green crates; runner `run.sh`, direct binary runner `bin.sh`, bilingual-law model `law.py`).

## Final counts

Every run under the shared cargo lock, `-j 3`, `--test-threads=2`, at most 4 packages per invocation.

| Crate | Before | After | Run |
|---|---|---|---|
| shell-tests-chrome-1 | 57 / 61 | 61 / 61 | `a4` (final source for this crate) |
| shell-tests-chrome-3 | 40 / 41 | 41 / 41 | `a4` |
| shell-tests-input-2 | 58 / 72 | 72 / 72 | `a5` (after the last edit, the find test's `own_surface`; `a4` had 71 / 72 with only that test failing) |
| shell-tests-input-3 | 78 / 79 | 79 / 79 | `a4` |
| shell-tests-chrome-2 | 10 | 10 / 10 | `b2` |
| shell-tests-chrome-4 | 102 | 102 / 102 | `b2` |
| shell-tests-chrome-6 | 95 | 95 / 95 | `b2` |
| shell-tests-input-1 | 73 | 73 / 73 | `b2` (66 s) |
| shell-input | 5 | 5 / 5 | `b3` |
| shell-view-chrome (`--features semio-renderer-wgpu-shell-view-chrome/testkit`) | 6 | 6 / 6 | `b3` |

Not one single final invocation: `b2`/`b3` (the green crates, run after every source edit) are final. The repeat of the four target crates as one invocation (`b1`) did not complete: its first attempt was killed during the link of input-3 (`exit status: 137`, rustc, not a test failure; the other three crates of that attempt had compiled), the second attempt hung 20 minutes in `resource-directory-admission:v1` after the kill and I cancelled it with SIGTERM (my own process), and the third attempt queued behind other agents' runs on the shared cargo lock for more than 45 minutes, so I withdrew it. The four target counts above therefore come from `a4` and `a5`, which ran on the final source of those crates (the only later source change is none: after `a5` I edited only this document).

## Clusters: root cause, files, kind

| Crate | Test(s) | Root cause | Kind | Files |
|---|---|---|---|---|
| chrome-3 | `no_capitalised_chrome_phrase_escapes_the_bilingual_tables` | The source reader now sees the whole Shell tree. It found (a) real EN/DE `Locale` tables outside the declared `ChromeI18n` region, (b) EN/DE pairs chosen by `if is_de` in the task-manager rows and the original-image import task, (c) English-only fault texts of Open File, form page, image source page and Save, returned as `Err(String)` and shown raw in the transient notice and in the document-opening banner. | product | see "Bilingual law" below |
| chrome-1 | `accepted_node_graph_note_caret_arms_resets_and_retires_with_its_exact_scene` | `drop_engine_surface` closed the attached engine surface with one fuel unit per close turn; a surface that owns a note-edit caret needs more than one unit per step, so the close ladder never progressed ("fixed terminal witness" after 262144 turns). | fixture | `E/🎯️targets/🧊️wgpu/🔗️engine-attach/🧰️fixtures/🦀️.rs` (`with_engine_close_context(1 << 20, ..)`) |
| chrome-1 | `accepted_event_feed_rows_publish_tabbable_buttons_read_only_text_and_exact_actions` | The test hand-built the control's action args with `action_args_json!` (serde_json map, keys sorted: `id`, `surfaceId`) while the product derives the expected action with `dsl_value!` (insertion order: `surfaceId`, `id`); `DslValue::Object` equality is order sensitive, so the exact-action comparison in `event_feed_accessibility_activate` refused. Production stages and compares with the same constructor, so the product is right. | stale test | `E/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🧩️accepted-control-publications/🦀️.rs` (control built with `dsl_value!`) |
| chrome-1 | `apply_drop_committed_queues_the_merged_action_into_input` | A positional `application/x-semio-tree-row-reorder` payload dropped on a non-tree drop target (a stack) queued the stack's own drop action with the decoded row payload. The engine only ever offers a reorder drag to tree rows (`nearest_accepting_drop_target`), and `apply_tree_row_drop` already refuses that mime; the commit entry point did not. | product | `E/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs` (`apply_drop_committed`) |
| chrome-1 | `retained_input_refusal_settlement_obeys_the_shared_draft_disposition_law` (case `date frozen refusal`) | A closed `date`/`color` picker field opened its popup on Enter even when the user had typed a draft, so the typed date never committed. The shared law corpus (`🧪️tests/🧫️fixtures/🚦️input-draft-disposition`) commits the typed draft on Enter. | product | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs` (`route_picker_key`: Enter with a draft differing from the published text falls through to the edit commit) |
| input-2 | nine `presented_input_authority_tests`, `find_publishes_filters_and_activates_with_a_physical_row_click` | Fixture closed its caller (`FixtureTurnV1::finish`, so the engine surface close) at the end of every paint while the test still held the documents; the close never terminated (demand `copy_bytes 112`). Input-1 pattern: the test owns one `PointerFixtureTurns`, paints through it, closes its documents, then `turns.finish()`. The find test additionally registers its published window surface with `turns.turn().own_surface(..)` and retires the shell documents before `finish`. | stale test (paint helpers) | `E/🧱️elements/🐚️Shell/🧪️tests/🎯️presented-input-authority/🦀️.rs`, `E/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs` (`publish_palette_chrome_in`) |
| input-2 | `picking_an_arg_carrying_command_opens_its_form`, `workspace_clipboard_keeps_copy_through_the_next_paste_shortcut` | The first test left its panel documents alive at arena close ("descendants remain borrowed"); the failure left a stale queued retirement on the thread, which made the second one fail with "stale UI value owner" when the thread was reused. The second passes alone. Fix: `retire_shell_documents(&mut shell)` at the end of the first test. | stale test | `E/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs` |
| input-2 | `workspace_clipboard_keeps_the_latest_effect_and_preserves_paste_placement` | Not the step-versus-turn-total comparison of the old clipboard handoff (that one is in `semio-ui-wgpu-laws` and is green). Here the Shell had no arm for `Effect::ClipboardWrite`: it fell into the "effect dropped" arm, so `DocumentState::workspace_clipboard` (read by `bind_workspace_paste` and the paste shortcut) was never written. | product | `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/⚙️settings-and-dock-persistence/🦀️.rs` (`apply_host_effects_until_load`: latest write wins) |
| input-2 | `retained_tree_reorder_marker_follows_the_physical_pointer_and_clears` | Same close problem as the presented-input tests: the wrapper `paint_tree_pointer_documents` finishes its caller, which closes the retained tree surface, so the drop preview (`retained_tree_drop_preview`) is gone while the test still drives the pointer. The test now keeps `PointerFixtureTurns` alive (`paint_tree_pointer_documents_in`), has its arena scope, closes its document, then `turns.finish()`. | stale test | `E/🧱️elements/🐚️Shell/🧪️tests/📐️retained-tree-reorder/🦀️.rs` |
| input-3 | `the_status_stacks_sit_and_size_like_react_notice_stacks` (two defects) | (1) Product: the route-admission row (horizontal) gave the first cell the whole budget and then forced every later cell to at least one glyph, so the frame exceeded `max-w-[90vw]` by one glyph width. The constant (90 percent) and the law are right; the layout now reserves one glyph per remaining cell before sizing a cell. (2) Test: once the width law passed, `frame.y + frame.h == 900 - footer_height` compared two derived `f32` for exact equality (`871.2001` vs `871.2`); it now compares within 0.01, like every other geometry assertion of that test. | product + stale test | `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs` (`status_stack_layout`), `E/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-parity-constants/🦀️.rs` |

## Bilingual law (chrome-3): decision and implementation

Decision: every user-visible literal goes through the EN/DE tables; no exemption was added to the law. The law is unchanged.

- Real `Locale` tables outside the declared region: `local_folder_text` and `local_files_title` (`🎛️prefs/📎️local-folders/🦀️.rs`) are exhaustive `match (.., Locale)` tables, so they are wrapped in the `//#region 🗣️ChromeI18n` markers, which the law names as the table (a per-`Locale` exhaustive match fails to compile for an untranslated locale, like `LocalizedLabel::native`).
- `if is_de` pairs: new rows `taskManager.transfer.{exportLabel,exportProgress,importOriginal,cancelImport}` and `document.open.fileLabel` in `🎯️targets/🧊️wgpu/📖️shell-vocab/🧵️task-manager-and-route-vocab/🦀️.rs`; the call sites (`📂️doc-open/🎨️theme-file-import-and-accessibility/{🦀️,🎞️original-picker}.rs`, the two `Open File` labels in `🔀️actions/🎬️shell-actions/🎯️dispatch-action/{📂️,🌐️}open.rs`) read them; the two `Cancel {file}` rows use the existing `taskManager.tasks.cancelTask`.
- Fault texts: the 32 English fault literals became stable fault codes (`open-file.no-declared-extension`, `form-page.out-of-range`, `image-source-page.needs-window`, `save.requires-attached-document`, ..., same style as the existing `natural-file.kept-archive-unavailable`). Their EN/DE reader texts live in the new vocabulary module `🎯️targets/🧊️wgpu/📖️shell-vocab/🧯️fault-vocab/🦀️.rs` (`shell_fault_text(code, is_de)`). They are resolved where they reach a reader: `classify_dispatch_fault_notice` (transient notice, `🎛️prefs/🗨️chrome-overlays-and-tour/🦀️.rs`) and `document_opening_banner_text` (`🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs`); an unknown text still shows as is. No test read the old English sentences (searched).
- Sites rewritten: `📂️doc-open/🎨️theme-file-import-and-accessibility/{📂️open,🌐️open,💾️save}.rs`, `🔀️actions/🎬️shell-actions/🎯️dispatch-action/{📂️open,🌐️open}.rs`, `.../🪟️layouts/🦀️.rs`. `ProgramBridge` copies of two of these texts (outside the Shell source set the law reads) were left alone.
- `law.py` (scratchpad `shell-ci`) re-implements the law over the same module tree and reports 0 offenders; the real law passes in the final run.

## Fixture and test-support changes (additive, shared)

- `E/🎯️targets/🧊️wgpu/🔗️engine-attach/🧰️fixtures/🦀️.rs`: engine close fuel 1 -> `1 << 20` (shared by every law that attaches an engine surface; a larger fuel only lets a step run, it cannot hide a hang).
- No other shared fixture changed. `PointerFixtureTurns`, `retire_shell_documents`, `paint_tree_pointer_documents_in` of `🧰️fixtures/🐚️shell` were used as they are.

## Cross-agent note

At 07:18 `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/📐️layout-stepping/🦀️.rs:292` held another agent's `eprintln!("[DEBUG] layout job {window_id} ..")` that did not compile (`SurfaceId` has no `Display`) and had blocked every cargo run for more than 20 minutes; I changed only `{window_id}` to `{window_id:?}`. That `[DEBUG]` line is theirs and still has to be removed by them. Later an in-flight `Box<IconExportBatch>` change in shell-pointer broke one run and was fixed by its owner.

## Still failing

Nothing in the four crates. Not run as part of this task: chrome-5, docs-*, windows-* (other owners).

## Verified by running vs reasoned

Verified by running: every count in the table; the root causes of the chrome-1 caret test (fuel), event-feed test (arg order: the debug print showed `[id, surfaceId]` against `[surfaceId, id]`), the date draft (debug print: `date` produced no commit command while `text`/`longText`/`number` did), the input-3 width (`871.2001` vs `871.2` appeared only after the width law passed), and the fact that `workspace_clipboard_keeps_copy_through_the_next_paste_shortcut` passes alone (so it was thread pollution of the previous failing test).

Reasoned only: that the bilingual law's decision also holds for future `Err(String)` sites (they must use a fault code of `shell_fault_text`); that `apply_drop_committed` refusing a positional payload on a non-tree target has no other caller relying on the old behavior (searched the call sites and the engine's drop acceptance, not run in other crates); that the `1 << 20` close fuel in `drop_engine_surface` cannot mask a real close hang (a hang spins for the turn ceiling regardless of the fuel); the German fault texts were not reviewed by a native speaker.

No `[DEBUG]` edit of mine remains (grep over the touched trees is clean).
