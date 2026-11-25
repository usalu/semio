# PDF Editor Browser Journey (standalone host, port 6260)

2026-11-22. Status: the standalone PDF 1.7 editor shell is up and renders in a real browser. Opening a real PDF and saving a new document do NOT complete yet (details and diagnosis below).

## Commands

Repo route (what the coordinator asked for), attempted twice, stalled in the graph read on a busy, live-edited tree:

```
cd /Users/ueli/Documents/semio && repo ./📜️script.ts dev playground stdio-pdf-1-7 react prototype
```

First attempt reached `Planning 1 targets` after about 8 minutes (capacity waits under load 20+), then sat in `Reading the project graph slice ... taken 0 of 2`. Second attempt (`repo run @semio-tech/framework-os-dev:serve-stdio-pdf-1-7-react-prototype --exclude-dependencies`) the same. Per the ticket rule, fell back to native.

Native fallback (working, `launch.json` entry `stdio-pdf-react`, port 6260):

```
sh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END/serve-pdf-native.sh
```

`serve-pdf-native.sh` runs `serve-pdf-native.ts` with the repo bun. The `.ts` mirrors the managed Vite service shape (`⚡️caching/🌐️vite/🟦️.ts`): `createServer` in middleware mode on one shared `node:http` listener, config loader `native`, the variant `🏗️builder/🌐️vite/🟦️.ts`, and the serve env (`SEMIO_PLUGIN`, `VITE_SEMIO_PLUGIN`, `VITE_SEMIO_APP_ID=s.stdio.pdf@1.7/*#editor`, `SEMIO_BUILD_MODE=prototype`, `SEMIO_VITE_CACHE_DIRECTORY`, `SEMIO_VITE_HMR=0`, `S_LOCAL_ONLY=1`, `S_DATA_DIR`).

It uses the already staged prototype activation (`.cache/b/p/m/browser/prototype/modules/📘️stdio-pdf`, wasm built 2026-11-21), so no wasm build is needed to boot.

Journey: `repo <ticket>/pdf-pane-journey.ts [en|de|open]`; probes: `probe-pdf-pane.ts`, `explore-pdf-pane.ts` (CDP pause sampling of the plugin worker).

## What works

- Page loads, beacon `ready:stdio-pdf-1-7`, shell titled `stdio · pdf · 1.7`, Editor/Viewer toggle, Page, Text layer, Object and Details panes, full Actions rail (Insert Text, Insert Page, Set Document Info, ... Save File, Open File).
- Executing an action form works on a new document (Insert Page opens its Page/Width/Height form and responds immediately).

## What fails and why

1. Native-host console noise: `WebSocket ... /semio-stream-mux failed` (the backbone mux is served by the local hub; the native host has none, `Remote: detached`, `local only`). Filtered in the journey.
2. Open File with `2️⃣two-pages.pdf` hangs in the plugin worker. Evidence (`🗑️generated/explore-long.txt`): after about 15 minutes the shell reports `natural-file.import-and-retirement-failed ... plugin.command-ingress-stalled ... continuations=100936/16 ... command-pending, actor more-work`. CDP pause sampling of the worker shows the wasm calling `semio:framework/pure.operation-clock-us` (via `operationProgress` / `operationAllocation`) in a hot loop at 100 percent CPU. So the staged wasm's controlled natural decode (`🚪️io/📥️import/🔁️controlled/🦀️.rs`, `NaturalPdfDecoder::advance`, grant `work: 1` per step) never reaches `Complete` for a two-page file, a livelock or an extremely fine work granularity. The component wasm is stale (2026-11-21) relative to the Rust crates the other agent is changing, so the root-cause fix belongs with the PDF Rust crate work and needs a rebuilt component to re-verify. Not changed here.
3. Save File on a new document: `shell.documentTransfer.file-save-failed ... natural-file.no-data-folder` without `S_DATA_DIR`, and `natural-file.archive-not-persisted` with `S_DATA_DIR` set. The native host has no local hub session (`withDevLocalHubSessionV1` returns null for local-only), so the catalog cannot persist the archive. Needs either the real repo route (hub session) or a hub-less persistence path; unresolved.
4. New PDF shows `Page width 0 / Page height 0` and `Empty canvas` (fresh document has a zero-size page); likely expected for the empty genesis, but a PDF editor should start with a default page size. Not changed.

## Fixes made on the way

- `devContribution` for the playground rows is now owned by the composition json (single source), as requested by the coordinator:
  - `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/📋️contract/🟦️.ts`: `CompositionPlaygroundV1.devContribution?`, closed-object validator gained an optional-field list.
  - `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/📋️contract/🧬️schema/🔣️.json`: optional `devContribution` property.
  - `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts`: playground row generator emits `devContribution`.
  - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧩️composition/🔣️.json` (row `stdio-pdf-1-7`) and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🧩️composition/🔣️.json` (three `stdio-xlsx*` rows) carry it.
  - `prepareStdioComposition` ran green (`{"contributions":38,"apps":180,"receipts":35}`) and the generated PDF and office `Cargo.toml` rows contain `devContribution` (the earlier loss was the generator overwriting hand edits).

## Evidence

`🗑️generated/probe.png`, `🗑️generated/pdf-journey/*`, logs `host-native.log`, `explore-long.txt`.

## Follow-up round (save, blank page, journey)

### Hub / data folder finding
There is no separate hub to start for local save. `S_LOCAL_ONLY=1` makes `withDevLocalHubSessionV1` return no hub; document persistence goes through the backbone folder store, which the Vite dev server itself serves (`semio-backbone-vite-plugin`: `GET|PUT /semio-backbone?uri=folder://<S_DATA_DIR>/os/...` plus the stream channel `/semio-stream-mux` from `devStreamMuxServer(server.httpServer)`). So the launcher needs `S_DATA_DIR` (set to `.cache/b/p/m/stdio-pdf-data`) and a Vite server that has a real `httpServer`.

### Root cause fixed in the launcher
`serve-pdf-native.ts` used Vite middleware mode on a private `node:http` listener, so `server.httpServer` was `null`; the mux plugin then served a private channel nothing could connect to (every `/semio-stream-mux` WebSocket handshake got a 200). It now uses `createServer({... hmr:false})` and `server.listen()`; the WebSocket errors are gone, and `/semio-backbone` PUT/GET answer 200 (events and bootstrap db files appear under the data folder). `serve-pdf-native.sh` is unchanged and still starts everything zero-touch (launch.json entry `stdio-pdf-react` unchanged).

### Save still not green
Save File on a new document still ends with `natural-file.archive-not-persisted`: the archive is written to the folder, but the shell's verified readback (`awaitVerifiedReadbackV1` in `persistAttachedProgramDocument`) never receives an `archive` event from the backbone worker (instrumented, `accepts` is never called, instrumentation reverted). Next step: trace the `localDocumentArchive` message through `store/worker/♻️artifact-lifecycle` and `ShellHost/🧵️backbone-worker` for a freshly created document (is the new document registered with the worker?).

### Blank document page size (fixed in source, not yet verified in browser)
`blank_pdf_snapshot()` (`🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📤️export/🦀️.rs`) encoded `PdfSnapshot::default()`, which has zero pages, so the editor showed a phantom page 0 x 0 with an empty canvas. It now authors one blank US Letter page (`PdfPage::default()`, 612 x 792, same as the 1.7 base builders). Unit test added: `blank_document_is_one_letter_page_and_its_own_fixed_point` in `…/📤️export/🧪️tests/🔬️unit/🦀️.rs`. Not run: `repo cargo test -p semio-s-artifact-stdio-pdf-io-export` is refused (`package ID specification did not match any packages`: the crate is not in the root workspace; the pdf artifact workspace needs its own route) after a 14 minute capacity wait. The browser sees the fix only after the other agent's wasm rebuild.

### Journey
`pdf-pane-journey.ts` en/de new-document steps: the title edit (`set-info-field`) is refused by the staged wasm (`original-turn.native-return-refused: semantic`), probably also caused by the zero-page blank document; and the save step then times out as above. Re-run after the wasm rebuild and the readback fix. Insert-text and reopen steps are not yet in the script.

## End-to-end run (2026-11-23)

Evidence: `🗑️generated/pdf-journey/` (`*-1-new.png`, `*-2b-inserted.png`, `open-1-fixture.png`, `console-*.log`, `journey.json`).

### Build and host

- `rebuild-pdf-wasm.sh` now works end to end: cargo through `native-cargo.sh` (lane pool, `wasm32-wasip2`, profile `prototype`), then `stage-pdf-component.ts`, which strips the 275 MB `name` section (`strip-component-names.mjs`) and runs the repo's own transpile steps (`semio-native-transpile`, async-result and asset-URL rewrites, preview2 shim relocation to `../🪞️vendor/...`), then swaps `.js`, `.d.ts`, `interfaces/`, `*.core.wasm` and the receipt hash in the staged module. A full wasm build takes about 45 minutes. `jco transpile` cannot be used: it traps (`out of bounds memory access`) on a 280 MB component.
- Host on port 6260 (`serve-pdf-native.sh`) restarted and serving the rebuilt component.

### Result

| Step | State |
| --- | --- |
| New document shows a real page | PASS: Page width 612 / height 792, white Letter canvas (en). |
| Page, Text layer, Object, Details panes, localized labels | PASS (Title, Author, ... fields present). |
| Title edit, insert text, save, reopen, export/import, two-page open (en and de) | BLOCKED, see below. |

Blocking fault: the first mutation of the guest (title commit, insert text, save) ends in a typed `semantic` failure of the original-turn framework (`original-turn.native-return-refused: semantic`, then `file-save-failed`, `tree window refresh failed`). The fault text is not surfaced to the host (the WIT failure carries only the tag). Earlier in the day it also hit the very first host event; after rebuilding from a later tree state it moves to the first mutation, so the value/ownership framework (the in-flight "native encoder requires its original construction recipient" family, owned by `GET-FEM-PLUGIN-WORKING-END-TO-END`) is the cause, not the PDF artifact or the shell. Open File of `two-pages.pdf` no longer hot-loops but yields no `file-opened` notice within 60 s.

### Journey script changes

`pdf-pane-journey.ts`: insert-text through the real form (`framework.window.pdfPage.action.insertText.arg.<key>.input`, `...insertText.execute`), page-size check through the Page width/height inputs, reopen checks title and inserted text, new `transfer` step (Export Document, Import Document), full console trace per locale. Known gap: the German run still checks the English input labels for page size.

### Next

Surface the `OriginalTurnPrimary::message()` in the WIT poll failure (or log it through the guest `log` import) so the host shows the actual fault, then fix it in the framework, rebuild with `rebuild-pdf-wasm.sh`, rerun `repo <ticket>/pdf-pane-journey.ts`.

## Status after the reboot (2026-11-23, afternoon)
- Native repro of the first edits (blank PDF 1.7: title, insert text) passes on the current tree; the browser fault was a stale wasm. The journey still has to be rerun against a fresh build.
- `.cache/b/p/m` was empty and `repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype` died on a stray entry in the unmounted native root. Fixed in the repo (stranded entries are quarantined, regression test), see `🧭️pdf-guest-edit-refusal.md`. The run is in flight (`/tmp/claude-501/devhost.txt`, `nohup`): cold graph read takes about 30 minutes, then the dev host builds the wasm and serves port 6260.
- Not done yet: restage (needs the dev target, or the self-sufficient staging fallback), `serve-pdf-native.sh`, `pdf-pane-journey.ts` en/de, natural-route leftovers.

## Native fallback staging (2026-11-23, later)
- The products image `.cache/b/p/fixed.dmg` (33 GB, UDRW) was simply not attached after the reboot and still held the previously staged `browser/prototype/modules` (854 MB: `📘️stdio-pdf`, `🧵️shard`, `🪞️vendor`). `mount-products.sh` attaches it at `.cache/b/p/m` (what the repo's own recovery does once its graph read finishes). Mounted by hand once.
- The staged `🌉️bridge.js` and `🟨️.js` were older than the current sources (staged-guest budget, return-escrow job outcome, the new semantic-fault log), so swapping only the component files would have mismatched. `stage-pdf-component.ts` now also regenerates both from source with `pluginComponentBridgeSource` and `hostShimSource` (same materialization pipeline) and refuses with a clear message when no staged module exists.
- `rebuild-pdf-wasm.sh` started (waited behind two other agents for a lane, then compiling with `-j 1`); log `/tmp/claude-501/wasm.txt`. Host start and the en/de journey follow when it stages.

### Graph read not converging (assessment)
`repo run` is not a query: the dev target must plan against a fresh graph, and with no sealed generation (rebooted machine) the slice read walks the file inventory (217,413 source observations, 535 owners, "n of 600" counters that restart). On a worktree that is edited every second by 50 machines the inventory observation keeps being invalidated, so it can stall for tens of minutes. Real repo bug (class): a run should use the sealed/previous generation and only re-read changed owners, instead of restarting the whole inventory on concurrent edits. Not pinned to a file here: the progress line comes from the resident service's graph slice reader (`checking the file inventory of the tree`).

## Journey evidence after the fresh wasm (2026-11-23, evening)
Host `serve-pdf-native.sh` on 6260 serves the rebuilt and re-staged component (sha256 `fa75f5a1…`), bridge and host shim regenerated from source. Journey `pdf-pane-journey.ts`, evidence in `🗑️generated/pdf-journey/`.

| Step | en | de |
| --- | --- | --- |
| New document: Letter canvas 612×792 | PASS | PASS |
| Set title (Object pane) | PASS (title in saved PDF) | PASS (stored as PDFDocEncoding hex; journey check now accepts it) |
| Save File: `%PDF-1.7`, 584-879 bytes | PASS | PASS |
| Insert text | the guest refused `pdf page 1 does not exist`: the form takes a zero-based page. With `0` it is accepted (`textShown` still false right after submit, text appears after reopen) | same |
| Reopen the saved file | file chooser opens now; the guest never publishes a `file-opened` notice within 120 s | same |
| two-pages.pdf, export/import | not reached | not reached |

The old `original-turn.native-return-refused: semantic` is gone: the first edits of a fresh document no longer fail in the browser.

### Open File stall (root cause candidate)
The console of the open shows 11,358 repeated `[DEBUG] work quote denied ready=true observed=0 additional=1048577 fuel=~1044891`: a decoder work quote of 1 MiB + 1 can never be admitted under a turn fuel just below 1 MiB, so every later turn repeats the same denial. This is the same family as the native large-document stalls (`close owner Begin`, 32768 turns). The quote logic is in `🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs` (`admit_work`) and the host turn fuel in the original admission stock; owned by the value/ownership work. Fix direction: quote at most the turn's fuel ceiling in chunks (or raise the ceiling above the largest indivisible quote).

### UI finding: page numbers
The insert-text form's `page` field is zero-based while the panes say `Page 1 / 2` (thumbnails, text layer, inspector heading all use `page + 1`). User-facing numbers must be one-based. The surface is wide (`edit_from_action` page argument, inspector commit args, authoring pick-source args, `go-to-page`, presence, about 140 test call sites and the natural-file fixture whose rotation page is an index), so I did not flip it piecemeal; a half flip would make some actions one-based and others zero-based. Plan: one-based `page` in every declared page argument, converted once in `page_index()`, new `PdfFault::PageNumberOutOfRange` (en/de, params page and last) replacing the English `pdf page N does not exist` at the four lookup sites, inspector/authoring/presence callers pass `index + 1`, tests and fixtures updated together. Journey keeps `0` until then.

## End-to-end run (fresh build)

2026-11-24. Wasm rebuilt through `rebuild-pdf-wasm.sh` (lane `w`, `-j 1`, sha256 `34a06abf…`, includes other agents' work up to 03:10), re-staged with regenerated bridge and host shim, host `serve-pdf-native.sh` on 6260. Evidence: `🗑️generated/pdf-journey/` (`en-1-new.png`, `en-2b-inserted.png`, `de-*.png`, `*-open-*.png`, `console-*.log`, `journey.json`, saved `*.pdf`).

| Step | en | de |
| --- | --- | --- |
| New document: Letter canvas 612×792 | PASS | PASS |
| Set title (Object pane) | PASS (in the saved file) | PASS (PDFDocEncoding hex) |
| Insert text on page 1 (one-based) | PASS, text on the canvas about 2 s after submit, canvas pixels change | PASS |
| Save File | PASS, `%PDF-1.7`, text in the content stream | PASS |
| Reopen the saved file | FAIL: no `file-opened` notice in 90 s (see Open File) | FAIL (same) |
| Open `two-pages.pdf` | FAIL (same stall) | not rerun |
| Edit + save the opened PDF | not reachable (`transfer-busy` while the open is pending) | not reachable |
| Export Document / Import Document | FAIL: no download within 90 s | not rerun |

### Diagnosed on the way
1. Host lost wake-up (fixed): `drainTypedOperations` dropped a request that arrived while another drain ran; a typed operation admitted during that window never got its completion polled. `🚚️handle-turn-drive/🟦️.ts` now re-arms the running drain (`rearmedDrains`).
2. Intermediate wasm (before 03:10): the first edit of the session (title) and the following insert-text both ended in `interactive-job.publication-stalled` ("advanced 4096 publication units without moving its ladder: waiting on the pending publication is closing"): the edit reached the document and the saved file, but the operation never completed, so no `OperationCompleted` with `Full` scope reached the shell (text layer stayed "Untitled", canvas blank). Visible in the console as lane 11 result pages. The fresh build does not reproduce it (two probe runs, three journey runs); I added guest `[DEBUG]` probes at the publication abandon and the stale-root branch and they never fired, so the cause is not pinned. Probes removed again. Native regression test added: `typed_edit_after_a_reserved_shell_note_settles_through_the_registered_typed_operation` (passes; new law helper `settle_reserved_action_as`, `feature-owner.json` hashes of the two helpers are stale).
3. Open File quote (`additional=1048577`): zero `work quote denied` lines on the fresh wasm. The console instead shows another agent's probe `[DEBUG] big work quote 49999995 available=Some(49999995)` (a quote equal to the whole 50 M turn fuel) once per open and per export, plus `unsupported backtrace` (std backtrace is unavailable on wasm32, so a backtrace probe cannot locate the requester there; use a native run or name the call site in the message). The open and the export then sit pending with the shell warning `stdio-pdf interaction has held stdio-pdf#2's lane for 5000 ms` and later `has not answered "an interaction" for 205 s`. So the Open/Export stall persists but is not the old denial loop: a decode or encode stage that quotes all remaining fuel does not finish. Owner: the decoder/encoder work-quote path (`🔨️modules/🧵️job` retirement/decode, pack export stage; see `📂️pdf-open-and-save-route.md`, large-document section).
4. Imported-PDF save with retained objects (`ReferenceRequired`): not reachable in the browser yet because the open stalls; natively unchanged. The journey step `open-en`/`open-de` already asserts the contract (valid PDF, or a localized alert with `role=alert|status`), and currently records `silent failure` only because the save is refused as `transfer-busy` (that notice is localized: "Another file is still being opened — wait for it or cancel it in Tasks."). It is displayed, but the shell also shows a persistent status line for the stalled interaction with a Cancel command, which is accessible.

### Journey changes
`pdf-pane-journey.ts`: steps `en`, `de`, `open-en`, `open-de`, `transfer`, `probe` (diagnostic: `PROBE_TITLE`, `PROBE_TWICE`, `PROBE_CLICK`, `PROBE_POKE`, `INSERT_WAIT_MS`); insert text polls until the text is visible (reports `visibleAfterMs`) and compares canvas pixels; open steps insert on page 1 and save, accepting either a valid PDF or a localized accessible error.

### Open items
- Open File of any PDF and Export Document stall on the fresh wasm (quote of the whole fuel, see 3). Needs a native reproduction of the paged natural open under the 50 M browser turn fuel, not the 1 MiB fuel the existing regression test uses.
- Verify item 2 stays fixed after the next rebuild (cause unknown).
- The journey's reopen check reads title/text from the stale pane; once Open works, assert a fresh canvas hash.

## End-to-end verification (2026-11-24, final pass)

Evidence is under `🗑️generated/pdf-journey/` (screenshots `en-*`/`de-*`, saved PDFs, `journey-en-open-transfer-details.json`, `journey-de-open-details.json`, the two stdout files, `trap-frames-first-wasm.txt`, `wasm-build-final-tail.txt`, `console-*.log`). Logs of the native runs are `🗑️generated/final-*.log`, `final2-*.log`, `final3-*.log`, `range.log`, `ab-*.log`.

### 1. Regression pass (native, one cargo at a time, `native-cargo.sh`, test binaries behind `watchdog-cargo.py`)

| Suite | Result |
| --- | --- |
| testkit `original_mutation_archive` | 10 / 0 |
| libs: `semio-s-artifact-stdio-pdf` 38, `-io-export` 19, `-mut-family-11` 32, `-12` 120, `-13` 79, `-14` 73, `-15` 59, `-mutation-round-trip-tests` 12, `-native-save` 15 (414 s), `-page-editor-authoring-tests` 20, `-page-editor-tests` 65, `-presence-lane` 10, `pdf14-canvas` 8 | all pass, 0 fail (592 tests) |
| snapshot-value `editor::pdf14`, `pdf14a`, `pdf14x` | 34 / 0 |
| `natural_file_route` / `paged_natural_decode` / `closes_mid_reserved_commit` / `media_in_of_a_small_pdf` / `shard_fuel_open` / `natural_source_route` | 7 / 3 / 9 / 2 / 4 / 2, all pass |
| four history laws (`history_edits_end_to_end`, `conflict_history_edits_end_to_end` for `pdf17`, `17a`, `17e`, `17h`) | 8 / 8 (pdf17 in the filtered run, the other six directly from the built binary because the 25 minute watchdog ended the filtered binary; 1,198 s) |
| framework `semio-framework-job-return-budget` (budget, stock, scope), `-job-context`, `-job` | 15 / 3 / 10, all pass |
| `semio-plugin-job-receiving` (Pack receiving, value copies, transaction preparation) | 9 / 1: `original_value_copies::underfunded_admission_stays_pending_and_a_funded_turn_resumes_it` fails. A/B run of the same binary with the old shortcut of `OriginalReturnScopePlan::quote` behind a temporary env switch fails identically, so it is not the scope plan change (switch removed again). Not investigated further. |
| `semio-os-dsl-module-tests` receiving | 2 fail + 1 hangs (killed): `original_command_receiving_operation_original_record_and_owned_fields_resume_one_item`, `original_operation_failure_preserves_real_probe_cpu_independently_of_physical_progress`, `original_operation_close_quotes_and_funds_the_selected_binding_after_take`. They drive `cursor.step` directly; the working tree carries uncommitted edits of other sessions in exactly that code (`🌱️value/🛬️decode/🧳️owned/🏗️record-machine`, `🌱️value/✨️derive/🛬️receive`, `🗣️dsl/🫴️receiving/📦️pack-value`). Not mine, not touched. |
| broader `editor::pdf17` (not on the requested list): 41 pass, 4 fail | `controlled_pdf_terminal_publication_requires_both_owned_headers` (`original page allocation attempt is occupied` while closing the decoder), `registered_pdf_font_change_preserves_encoded_text_and_the_next_line` (`PDF semantic import requires the reference reader for Resources`: the synthetic input needs the reference route), `registered_pdf_page_and_authoring_actions_undo_and_redo_step_by_step` and `registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit` (`original fixture settlement stopped after 32768 turns ... close owner Begin`, the arc-custody close family already named in `🪶️pdf-sqlite-and-natural-route.md`). All four predate this ticket's last changes (same signatures in earlier notes). |

New native law: `an_out_of_range_page_edit_is_refused_with_the_range_fault_through_the_registered_typed_operation` passes. It shows the refusal `stdio.pdf.page-number-out-of-range` (params `page`, `last`) is raised at **settlement** of the typed operation, not at dispatch, and that the document stays unchanged.
Helper fix: `try_open_registered_pdf_with` now closes the app on every `Err` (it used to drop it, so a failed open surfaced as a drop panic of the store slot instead of its reason).

### 2. Rebuild and host

`rebuild-pdf-wasm.sh` (lane `w`, `-j 1`) twice; the final wasm is sha256 `71f92eba…` (staged component 282,757,849 bytes), bridge and host shim regenerated by `stage-pdf-component.ts`, host `serve-pdf-native.sh` on 6260 restarted for each wasm. The first final wasm (`27b56b88…`) trapped on Open; its trap was symbolicated without a rebuild: `trap-frames.ts` pauses on exceptions in the workers and prints the wasm frames, `wasm-function-names.mjs` maps the function indexes to Rust names through the `name` section of the unstripped component (`lane-w/t/wasm32-wasip2/prototype/semio_hub_stdio_pdf_component.wasm`). The top frames: `abort` < `panic_fmt` < `resolve_ready::<…StdioPdfApp as PluginApp>::take_typed_operation_completion` < `advance_typed_operation_output_inner` < `plugin_continue_typed_operations`.

### 3. Root cause found and fixed: the first completion of an Open trapped the guest

`take_typed_operation_completion` is driven with `resolve_ready` (synchronous exchange) and awaited `refresh_cache`, whose first loop awaited `plugin_job_yield_once()` while a command prune was pending. An import is a whole-document replacement; `retire_displaced_document_rows` requests exactly such a prune, so the very first completion after an Open returned `Pending` and `resolve_ready` panicked (`wasm unreachable`, "future was not ready on its first poll"). Natively the maintenance step between turns had always finished the prune first, which is why no native law saw it.
- `🧵️instance/🪢️members/📥️staging/🧾️typed-operation-results`: the completion is withheld (witness stays queued, the non-empty outbox keeps the host driving) while a prune is pending, one bounded prune step per call.
- `🧵️instance/🖼️render/📥️staging/📜️history-view` (`refresh_cache`): no yield any more; at most `COMMAND_PRUNE_STEPS_PER_REFRESH` = 1,024 bounded steps, stops on a step that makes no progress; the rest stays with the maintenance ladder.
Verified in the browser: the trap is gone (`resolve_ready` no longer appears in any console).

### 4. Journey (`pdf-pane-journey.ts`, extended)

New or changed steps: out-of-range page number (localized range fault expected), edit of the opened two-page file with title plus text, the incremental route check (original bytes are a byte prefix of the saved file) and a reopen of the saved file, Export/Import archive round trip (`exportArtifactDocument` / `importArtifactDocument`), the Details-pane title (`setSnapshotValue` on `/info/title`), and a "no console errors beyond known noise" verdict per session. Run: `en open-en transfer details-en` and `de open-de details-de`.

| Step | en | de |
| --- | --- | --- |
| New document 612×792, set title, insert text on page 1 (one-based), visible on the canvas (canvas hash changes), Save valid `%PDF-1.7` with title | PASS (879 bytes, text after 1.8 s) | PASS |
| Out-of-range page number shows the localized range fault | **FAIL**: nothing is shown | **FAIL** |
| Reopen the saved file | **FAIL**: `file-open-failed`, see 5 | **FAIL** |
| Open `two-pages.pdf` (both pages render) | **FAIL**: same | **FAIL** |
| Edit it (title + text) and Save, incremental prefix check | **FAIL**: the open failed, the save then writes a fresh 874 byte document (not a prefix of the 963 byte original) | **FAIL** |
| Reopen the incrementally saved file | **FAIL** | **FAIL** |
| Export / Import archive round trip | PASS (2,577 byte archive, text back after Import Document) | not rerun (language independent action ids) |
| Title from the Details pane of an untitled document | **FAIL**: nothing is shown | **FAIL** |
| No console errors: new document, transfer, details | PASS, PASS, PASS | PASS (details) |
| No console errors: after the failed opens | FAIL (the open failure itself) | FAIL |

Counts: en 4 of 12 verdicts pass (`journey-en-open-transfer-details.json`), de 2 of 10 (`journey-de-open-details.json`).

### 5. What fails and why (open)

1. **Open File** (reopen, `two-pages.pdf`, incremental Save and everything after it). After fix 3 the open no longer traps; it fails with `original-turn.semantic InvariantViolated: original media import retired without a terminal publication` (`advance_original_media_import`, `🔌️plugin-app-impl/🎞️media-transfer`): the import's typed operation is no longer in `tool_operations` when the `MediaIn` custody looks for its terminal page. The console shows the operation's slot born and drained inside the same exchange (`typed-operation slots instance=2 live=1/64`, then `live=0/64`), so the operation is acknowledged or retired by another driver before `progress.terminal` was captured (candidate: the host operation drain acknowledging the page that the media command owns). The native real-runtime law stops at `Effect::SpawnJob`, so it cannot see this; the next step is to extend `media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_*` through the job host to the terminal page. `Document load: Loading document: 479 of 480` is the mount progress of the same import. This blocks the incremental-route browser proof (the incremental Save itself is proven natively: `natural_file_route_save_appends_to_the_opened_file_and_stacks_the_second_save`, 120 pages `natural_source_route_save_appends…`).
2. **Out-of-range page** and **Details-pane title**: both are silent in the browser. Natively the range refusal exists as a fault at settlement (new law above), so the failing link is the shell: a typed-operation fault page carries the canonical Fault JSON (`code`, `params`, `severity`), but `consumeTypedOperationEffects` decodes it as text and throws a plain `Error("typed-operation failed: …")` (or parks it for a call that has returned), the drain's `catch` only pushes `{instanceId, error}` onto `turnOutcomes`, and `appFaultNoticeV1` (the localized notice from the app's `faultNotices`) is reached only for a `SemioFaultError` raised at dispatch. Fix direction: decode the page into a `SemioFaultError` in `🔌️PluginRuntime/📬️typed-result` and route drain failures of an instance to the shell's `showTransientNotice` path. The Details route needs the same check once the fault is visible (the value argument is JSON-encoded in the journey).
3. Named open items unchanged: large documents under browser grants (open-side roster not paged), append-only cache is session-only.

### 6. Housekeeping

- `🧰️framework/…/🔌️plugin/🧪️tests/📦️units/📋️feature-owner.json`: `remainingFunctions` regenerated with `regen-feature-owner.ts` (the bun units test derives it from the facade and its mounted law files). `settle_history_verb_as` and `settle_reserved_action_as` are not in that list any more: commit 755 moved them into `🧪️artifact-app-instance-laws` (own crate, not mounted by `#[path]`), so their stale hashes are gone with that move; the function-list assertion of the units test passes now (it failed before). The units test still fails further on (`semio-plugin-testing-caller` dependency list, surface-carrier laws), caused by other sessions' crate changes, untouched.
- `[DEBUG]`: removed `println!("[DEBUG] segmented native Save …")` from the native-save `original-save` test; the temporary range-path print and the A/B env switch of this pass are removed as well. No `[DEBUG]` line of this ticket remains in the PDF artifact. Left alone (other sessions): `lifetime spent` in the fixture dispatch helper, the `ladder zero`/`live cleanup zero progress`/`big work quote`/`follow unit` probes, the host `job completion turn`/`isolated job …`/`ui install`/`refresh target` probes (FEM and mutation-latency tickets).

### 7. Changed files (this pass)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🪢️members/📥️staging/🧾️typed-operation-results/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/📜️history-view/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️feature-owner.json`
- PDF editor unit tests (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`): app closed on a failed open, the range-fault law
- native-save `🧪️tests/🔬️original-save/🦀️.rs` (`[DEBUG]` print removed)
- Ticket: `pdf-pane-journey.ts`, `run-final-regression.sh`, `run-final-framework.sh`, `run-final3.sh`, `run-range.sh`, `run-ab.sh`, `after-regression-wasm.sh`, `after-regression-wasm2.sh`, `regen-feature-owner.ts`, `trap-frames.ts`, `wasm-function-names.mjs`, `explore-details*.ts`, `explore-range.ts`, this section.

## End-to-end verification, round 2 (2026-11-24, wasm rebuilt from the current tree)

Evidence: `🗑️generated/pdf-journey2/` (first run, stale descriptor), `pdf-journey3/` and `pdf-journey4/` (`journey.json`, console logs); build logs `🗑️generated/wasm-build-r2*.out`.

### Result

| Step | en | de |
| --- | --- | --- |
| New document, title, insert text visible, Save valid PDF | not reached (pane boot error) | not reached |
| Reopen saved file / open `two-pages.pdf` / edit + incremental Save / reopen | not reached | not reached |
| Export / Import archive round trip | not reached in the final run (PASS in the first run only, stale descriptor) | not reached |
| Out-of-range page alert (localized) | not reached (first run: alert fired as the generic text `The operation was refused (stdio.pdf.page-number-out-of-range).`, i.e. fault transport works, catalog entry was missing) | same, generic German text |
| Details-pane unknown field alert / title of untitled document | not reached | not reached |
| Console | `pageerror: configured host app has no panel leaf` (first run), boot error `actor stdio-pdf#1 stopped without publishing requested UI surfaces (missing=["1:pdf.thumbnails"], status=idle)` (final runs) | same |

So (a), (b) and (c) are not yet proven in the browser: the pane no longer gets past boot with the real descriptor.

### What round 2 found

1. **The staged descriptor was stale.** `stage-pdf-component.ts` copied the new wasm next to the descriptor (`🔣️.json`, `🛂️.descriptor.semio`) of the products image, which never changed. The real pipeline (`browser-bundle` materialization) runs the module's own `describe()` and writes both. Consequences of the stale one: `faultNotices` was `[]` for every app (so every typed-operation fault showed the generic notice instead of the PDF catalog text), and window kinds added since (the thumbnail strip, first window of the default layout) were not requested, which masked item 2. Fixed in the ticket stage script: it now runs the describe probe (`PLUGIN_DESCRIPTOR_PROBE_SOURCE`, `finalizePluginDescriptor`) and rewrites `🛂️.descriptor.semio`, `🔣️.json` and the receipt's `semanticDescriptorSha256`. With it the 16 PDF fault notices (en/de) are in the served manifest. The committed `🌎️hub/…/📘️pdf/🧩️component/🔣️.json` is also stale (no fault notices, no thumbnail window); it is regenerated by the repo pipeline, not touched here.
2. **Boot stops at the first window.** With the real descriptor the first default-layout window is `pdf.thumbnails`. The shell refresh sends `SurfaceVisible`; the guest answers one turn, 0 patches, 0 effects, status `idle` (temporary `[DEBUG] settle stop` trace in `settlePluginTurn`, removed). `plugin_render_surface` returns without a tree when `app.render` answers `None`. Not the empty strip: I added `the_thumbnail_strip_of_a_document_without_pages_renders` (passes natively) and gave the strip a "No pages" / "Keine Seiten" row for a document without pages (`empty_label`), rebuilt the wasm (sha `72a11e4a…`) and the boot error is unchanged. Next step for whoever continues: trace why `render` answers `None` for the first surface of a fresh instance before its document is mounted (the stale-descriptor flow started with the main page window, which evidently did not hit this) and whether the shell may request the window surfaces before the blank document exists.

### Known limits (recorded, not solved)

Documents with two or more fonts or XObjects on one page are refused by the controlled lifter; large documents under browser grants; the append-only cache is session-only.

### Changed files (round 2)

- `stage-pdf-component.ts` (descriptor refresh), `pdf-pane-journey.ts` (`PDF_JOURNEY_OUT`), `run-page-editor-tests.sh`, this section.
- `✏️s/…/📖️pdf/…/7️⃣1.7/…/🧱️base/✏️editor/🖼️page/🔭️view/🖼️thumbnails/🦀️.rs` (`empty_label`, empty-state row) and its unit tests (`the_thumbnail_strip_of_a_document_without_pages_renders`).
- Temporary `[DEBUG]` trace in `PluginRuntime/🔁️turn-settle/🟦️.ts`: added and removed.

## End-to-end verification, round 3 (2026-11-24)

Evidence: `🗑️generated/pdf-journey5/` (full journey on wasm `e703aa09…`, `journey.json`, screenshots, `console-*.log`), `pdf-journey5-open/` (open steps after the standalone panel fix), `probe-boot*.log` + `probe.png` (boot probe), build logs `wasm-build-r3*.out`, native logs `surfaces1..5.log`, `thumb-sections.log`, `lanes*.log`, `close-trace*.txt`, descriptor emission `regen-descriptor.log`.

### Why `pdf.thumbnails` published nothing (root cause)

Not `render` and not the descriptor: natively every declared window publishes (law below, 20 of 20 requests for 1.7 and 1.4). In the browser a `[DEBUG]` trace of the shell intake (added, then removed) showed the guest DID publish `1:pdf.thumbnails`, and the shell's retained-UI intake **rejected** the patch: `plugin-ui.intake-rejected:validation:Owned UI graph invariants violated (invalidRowTarget) node=1`. The strip's rows carried a record `Trigger::Activate` binding on a `TreeItem` (select page, move earlier/later); the retained UI contract (`row_target_is_valid`, Rust and TS alike) allows a tree row to activate only through its `RowTarget`. The rejected surface was dropped, the next refresh asked for it again, the guest's patch tracker had nothing new to publish, and the boot stopped with `missing=["1:pdf.thumbnails"]`. The intake rejection itself never reached the boot error (it is swallowed between the first and the second refresh; recorded, not fixed).

Second defect found after the fix: the strip put its rows directly under the `Tree` root, but a tree renders only its `TreeSection` children, so the window painted "No data". The rows now live in one localized section ("Pages"/"Seiten").

### Fixes

1. Thumbnail strip (`🖼️page/🔭️view/🖼️thumbnails`): rows activate through `semio_plugin::activation_target` (select, move-page), never a record `Activate` binding; rows sit in one tree section; unit tests assert the shape (`strip_shape`: one section, one row per page / the empty-state row) — page-editor tests 2/2.
2. Generic surface law (PDF editor unit tests, real `PluginRuntime`): `every_declared_window_publishes_its_surface` requests every declared window kind in default-layout order (`Event::SurfaceVisible` with the packed window view, acknowledging patches like the shell), in en and de, on a fresh instance and again after `MediaIn` mounted a document, and applies every published patch through the contract's own `UiPatchApplyProducer` (`ShardShell`), so a patch the shell would reject fails the plugin's tests. Laws: `every_declared_pdf17_window_publishes_its_surface_before_and_after_the_document_mounts`, `…pdf14…`. Surfaces: 20/20 for each. Kept generic over `PluginApp` inside the PDF test module; lifting it into `artifact_app_laws` would rebuild the whole framework test chain, so it stays with the plugin for now.
3. Shell, standalone host: Open File threw `configured host app has no panel leaf` (`📤️document-transfer-tabs/🔶️scope`, `focusSpawnedProgram`): a standalone program host has no host app; the first panel now comes from the mounted session's own app. Open, reopen and the fixture open publish `file-opened` again.
4. Details pane: `setSnapshotValue` on `/info/title` of an untitled document was refused before any capacity was measured (`snapshot_edit_is_admitted` asked the snapshot for a pointer it does not hold yet), surfacing only as `tree window refresh failed … retained command work refused the command`. The stdio contract now exports `snapshot_edit_payload_is_admitted` (payload only; `snapshot_edit_value_is_admitted` = payload ∧ path), and the PDF 1.7 editors admit edits inside the whole-value lanes `special_edit` answers (`WHOLE_VALUE_LANES`, `addresses_whole_value_lane`), so the title is set and an unknown field reaches its localized `stdio.pdf.unknown-field` refusal. Law: `a_details_value_inside_a_whole_value_lane_is_admitted_before_the_snapshot_holds_it`.
5. Staging: the component bridge now requires the newer shard worker (`original-turn.succession-unoffered`); `stage-pdf-component.ts` also writes `shardWorkerSource()` (the `support` step of `@semio-tech/framework-plugin-web`).
6. Journey: notices are recorded with `role`/`aria-live` when they appear (the 20 s refused-insert wait let the transient notice expire before the check), range lines read the notice text after the code, page errors are written with their stack.
7. Descriptor sync check: `🌎️hub/…/📘️pdf/🧪️tests/🛂️committed-descriptor` describes the assembled bundle natively (the `describe` export's path) and compares JSON and pack with the committed pair, naming differing paths and the regeneration command.

### Result (`pdf-journey6`, full journey on the staged wasm `e703aa09…`, which carries fix 1a — row targets — but not the later tree section and Details fixes; TS fixes 3, 5, 6 are live)

| Step | en | de |
| --- | --- | --- |
| Pane boots (all five default windows publish) | PASS | PASS |
| New document 612×792, set title, insert text on page 1 visible on the canvas, Save valid `%PDF-1.7` | PASS | PASS |
| Out-of-range page: localized PDF notice, `role=alert`, `aria-live=assertive`, nothing leaked | PASS ("Page 99 does not exist. Enter a page number from 1 to 1.") | PASS ("Seite 99 existiert nicht. …") |
| Reopen the saved file (title and text back) | PASS | PASS |
| Open `two-pages.pdf`: `file-opened`, canvas painted | PASS | PASS |
| … both pages listed (`Page 1 / 2`, `Page 2 / 2` in the thumbnail strip) | FAIL: strip shows "No data" (rows outside a tree section; fixed in source, needs the next wasm) | FAIL (same) |
| Edit the opened file and Save: saved file starts with the original bytes | FAIL: full rewrite (874/897 bytes, original 963) — see below | FAIL |
| Reopen the saved file (title, text) | PASS on title and text; page count unproven (strip) | same |
| Export / Import archive round trip | PASS (2,579 bytes, text back) | (language independent) |
| Details pane: title of an untitled document | FAIL: refused before capacity (fixed in source + native law, needs the next wasm) | FAIL |
| Details pane: unknown-field notice | not reached (the step above times out first) | not reached |
| No console errors beyond known noise (new document, opened PDF, transfer) | PASS, PASS, PASS | PASS, PASS |

Counts: 14 of 22 verdicts pass (`🗑️generated/pdf-journey6/journey.json`).

### Remaining failures and their causes

1. **Next wasm not built.** Fixes 1b (tree section) and 4 (Details admission) are proven natively but the wasm with them could not be produced tonight: the shared tree did not compile for a while (`semio-framework-value-native-receiver-protocol`, then `ControlledFontResources::dependencies`, both other sessions' work in progress), and once it did, `rustc` for `semio-hub-stdio-pdf` (the 20-app monomorphization) was SIGKILLed three times while other sessions' processes held the memory (`wasm-build-r3c/r3d.out`, `wasm-build-loop-*.out`, `mem-r3e.log`). Next step: `sh rebuild-pdf-wasm.sh` on a quiet machine, then rerun `pdf-pane-journey.ts` (expected to turn the strip, reopen-pages and both Details rows green).
2. **Incremental Save of an opened file.** The browser opens through `importAppMedia` (`AppCommand::MediaIn`), which mounts the document without the original bytes as a retained natural source, so Save re-encodes the whole document. The append-only incremental route is proven natively only for the natural-source route (`natural_file_route_save_appends…`). Product gap: the shell's open must use the paged natural-source route (or `MediaIn` must keep the original as the natural source) for the incremental Save to exist in the browser.
3. **Committed descriptor cannot be regenerated.** The published emitter (`semio-plugin-describe describe`, run by `regen-pdf-descriptor.ts` and settled with `settleComponentDescriptorV1(…, "write")`) emits a pair the catalog refuses: `emitted descriptor forms exceed the fixed boundary` (`CATALOG_DESCRIPTOR_MAX_BYTES` = 4 MiB). Cause: `inspector_window_definition` copies the page window's whole action list (≈200 KB) into `pdf.object` of each of the seven PDF 1.7 editors (`windowKinds` per 1.7 editor 685 KB, committed 363 KB). Fix direction: the object window declares exactly the verbs its fields and buttons commit (field inputs, page actions, entry forms, authoring controls, delete), with a law that every binding the inspector renders is declared. Until then the new `committed_descriptor` test fails by design (stale pair; not run natively tonight — the hub crate is the memory-heavy compile).
4. **Native close after mounted surfaces.** The surface law publishes 20/20 for 1.7 and 1.4, but closing the instance afterwards never reaches `Retired` (16,383 `MoreWork` turns, sources `close_cleanup`, `reconcile`, `lifecycle`; patch tracker and pending authority each keep `closing=1`; `close-trace.txt`), the same with every surface hidden and every patch acknowledged first. The law asserts the surfaces before the close, so a surface failure is reported, but the test still fails at teardown. Not diagnosed further (needs a framework trace of which close participant stays incomplete).
5. **Swallowed intake rejection.** The shell's intake rejection of the first refresh never became the boot error; the second refresh reported only "missing". Recorded, not fixed.

### Changed files (round 3)

- `✏️s/…/📖️pdf/…/7️⃣1.7/…/🧱️base/✏️editor/🖼️page/🔭️view/🖼️thumbnails/🦀️.rs` (row targets, tree section) and `…/✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs` (`strip_shape`).
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (generic shard harness, `ShardShell`, surface laws for 1.7 and 1.4, close/hide with acknowledgements, Details admission law); `…/🔬️pdf-snapshot-value/📦️packages/🦀️rust/Cargo.toml` (dev-dependency `semio-ui-contract-validation`).
- `…/7️⃣1.7/…/🧱️base/🧬️schema/🧬️mutations/🦀️.rs` (`WHOLE_VALUE_LANES`, `addresses_whole_value_lane`), `…/✏️editor/🦀️.rs` and `…/✏️editor/🧰️kit/🧾️pdf17/🦀️.rs` (`snapshot_edit_is_admitted`).
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🛠️tool-job-factories/🦀️.rs` (`snapshot_edit_payload_is_admitted`), `…/🧬️contract/🦀️.rs` (re-export).
- `🧰️framework/…/🏛️ShellHost/🧱️built-node-stores/🟦️.ts` (`spacePanelOwnerLeafIdV1`) + `🧪️tests/🟦️.test.ts` and fixture (bun 1/1), `…/📤️document-transfer-tabs/🔶️scope/🟦️.tsx`.
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧪️tests/🛂️committed-descriptor/🦀️.rs`, its `📦️packages/🦀️rust/Cargo.toml` (`[[test]]`, dev-dependencies).
- Ticket: `stage-pdf-component.ts` (shard worker), `pdf-pane-journey.ts`, `probe-boot.ts`, `regen-pdf-descriptor.ts`, `rebuild-until-green.sh`, this section. Temporary `[DEBUG]` traces in `PluginRuntime/🧩️module-loader`, `🔁️turn-settle`, `🧺️handle-ui-intake` (+`lease.ts`) and in the law: added and removed.

## End-to-end verification, round 4 (2026-11-25)

Evidence: build logs and 20 s memory samples `🗑️generated/wasm-r4{a..g}.{out,mem.log,peaks.txt}`, journeys `🗑️generated/pdf-journey7/` (full, wasm `b2c26327…`), `pdf-journey7b/` (open and Details with per-step console logs), `pdf-journey7c/` (open, strip given 15 s), `pdf-journey8/` (full, wasm `ebfd8ade…` with the Save port, before the attachment drain), `pdf-journey8b/`, **`pdf-journey9/` (final, full)** and `pdf-journey9b/` (Details with the catalog text), strip probe `probe-strip.log`, native laws `mis-1..5.log`. Scripts: `measured-wasm-build.sh`, `sample-rustc-memory.sh` (20 s `top`, crate name from `ps`, plus a guard that stops only this build's own rustc below 12 % free), `peak-rustc-memory.py`, `await-quiet-tree.sh`, `run-round4-journey.sh`, `round4-chain.sh`, `probe-strip.ts`/`run-probe-strip.sh`, `run-media-in-save.sh`.

### 1. Wasm build within memory

Host and headless Chrome were stopped before every build (nothing listened on 6260, no Chromium). Peak physical footprint (`top` MEM, 20 s samples) of the large rustc processes of this build:

| crate (wasm32-wasip2) | before (r4d, repository `-Z threads=0`) | after (r4f, `-Z threads=1`) |
| --- | --- | --- |
| `semio_hub_stdio_pdf` (20-app monomorphization) | **10.0 GB** (6.2 GB 30 s after start, 10 GB at 2 min) | **5.94 GB** (r4f); **8.76 GB** in one 20 s sample of r4g at 3.5 min, 3.1 to 5.5 GB in every other sample |
| `semio_hub_stdio_pdf_component` (cdylib, link) | 6.79 GB, then **SIGKILL** during the link (`wasm-component-ld` 2.7 GB beside it, a foreign 6.2 GB host rustc) | **5.08 GB** (r4f), 4.40 GB (r4g), linked both times |
| `semio_s_artifact_stdio_pdf_model` | 4.54 to 5.06 GB | 3.92 GB |
| `semio_s_artifact_stdio_binary_io_registry` | 2.32 GB | 1.66 GB (host) / below 2 GB |
| build | killed | r4f 32 min 52 s (`b2c26327…`), r4g 34 min 11 s (`ebfd8ade…`, 285,180,288 bytes); no guard stop, no kill |

Cause at root: the repository's `.cargo/config.toml` runs every rustc with `-Z threads=0` (one front-end thread per core, 10 here). The parallel front end also translates the first batch of codegen units (one per thread) into LLVM modules at once, so the hub crate reached 6 GB within 30 s and 10 GB within two minutes, independent of `-j 1` and `codegen-units`. Fix in `rebuild-pdf-wasm.sh`: the build overrides `[target.wasm32-wasip2] rustflags` with the repository's own value (read from `.cargo/config.toml`, so `--max-memory` and `-zstack-size`, the component ABI, stay as configured) and only `threads=0` replaced by `threads=1`. A per-package `profile.*.package.*.rustflags` override does not work: cargo appends target rustflags after profile rustflags, so the last `-Z threads=0` won (r4e, verified on the rustc command line, killed by me). The derive-expansion levers of `🧠️pdf-model-compile-memory.md` (hoisting `RetainedClone`, per-field arms) were not needed for this crate: the peak was the parallel front end, not expansion. Opt-level, `codegen-units` and `embed-bitcode=no` (already set) stay unchanged. The r4g spike shows the hub crate still has one short phase near 9 GB with a single front-end thread (20 s samples hide its length); the next lever, if it is killed again, is the monomorphization itself: the eight profile editors (`pdf14a`, `pdf17a`, `pdf17e`, …) are distinct types, so every framework generic is instantiated for each of the 20 apps in this one crate.

Other build reliability fixes: `native-cargo.sh` gate is now FIFO (ticket files under `locks/queue`, oldest live ticket first; installed atomically so running waiters kept their copy): before, a waiter that slept on the memory check lost every race and this build waited behind other sessions' test runs for over an hour. `await-quiet-tree.sh` starts a build only after the PDF sources did not change for 5 minutes: r4a, r4b and r4c failed on other sessions' half-written lifter split (`semio-s-artifact-stdio-pdf-lifter`, `operator_bytes`, `DocumentLiftGrant`), not on memory. Note: other sessions compile outside this gate (`semio_hub_norm` 6.7 GB, `semio_hub_gis_component` 5.6 GB, `semio_s_artifact_energy_model_snapshot` 8.6 GB, a 5.2 GB native test binary were sampled beside this build); the gate cannot see them.

### 2. Journey (final: `pdf-journey9` + `pdf-journey9b`, wasm `ebfd8ade…`, all TS fixes live)

| Step | en | de |
| --- | --- | --- |
| Pane boots (five default windows) | PASS | PASS |
| New document 612×792, title, insert text visible, Save valid `%PDF-1.7` (now through the native segmented Save) | PASS | PASS |
| Out-of-range page: localized notice, `role=alert`, `aria-live=assertive`, nothing leaked | PASS | PASS (was FAIL) |
| Reopen the saved file (title, text) | PASS | PASS |
| Open `two-pages.pdf`: `file-opened`, canvas painted | PASS | PASS |
| … both pages in the strip (`Page 1 / 2`, `Page 2 / 2`) | FAIL: strip shows `Page 1 / 1` (see 5) | FAIL (same) |
| Edit the opened file and Save: saved file starts with the original bytes | FAIL: 876 bytes, written by the native Save but compactly (see 3) | FAIL: 899 bytes |
| Reopen the saved file (title, text; pages) | title and text PASS, pages FAIL (strip) | same |
| Export / Import archive round trip | PASS | (language independent) |
| Details pane: title of an untitled document | PASS (was FAIL) | PASS |
| Details pane: unknown-field notice (`stdio.pdf.unknown-field`, `role=alert`, assertive) | PASS: "The document has no field nonexistent." | PASS: "Das Dokument hat kein Feld nonexistent." |
| No console errors beyond known noise (new, opened, transfer, details) | PASS, PASS, PASS, PASS | PASS, PASS, -, PASS |

Counts: 18 of 24 verdicts pass (journey 9 with the Details rows of 9b); all six failing verdicts trace to 3 or 5. Round 3: 14 of 22. The journey's unknown-field expectation now is the catalog text (`PdfFault::UnknownField`: "The document has no field {field}." with `field = nonexistent`), not a pointer it never rendered; the Details step reopens the action form when its second press closed it.

Intermediate: `pdf-journey7` (wasm `b2c26327…`) failed the de range notice and both unknown-field notices through the refresh path of 4; `pdf-journey8` (port declared, no drain) failed every Save with `media export completed against a stale revision or generation` and the browser context closed afterwards.

### 3. Incremental Save of an opened file (remaining failure 2): cause corrected and fixed

Round 3 blamed the `MediaIn` route for not filling the original-file custody. Measured, that is not the cause:

* Natively, through the real `PluginRuntime`, `MediaIn` already retains the original: the new law `media_in_of_a_small_pdf_then_edit_and_save_appends_to_the_opened_file` (shard harness `shard_media_in_open`, now taking a `mounted` continuation that edits and saves the runtime-hosted app as its instance) asserts `custody_file == two-pages.pdf` right after the mount and passed that assertion (`mis-3.log`). Both decode routes (`decode_natural_file` → `retain_file`, controlled decoder `StashFile`/`DigestGraph`) fill it, whichever port carries the bytes.
* In the browser a temporary `[DEBUG]` trace (custody insert/find, Save route; removed) showed `custody insert digest=[28,14,e9,34] bytes=963` on open, then **no** `custody find` and no Save route at all: the browser Save never reached `PdfGraphSaveJob`.
* Why: the shell takes the revision-bound native export only when the app's `exportMediaFile` action names a declared `out` port (`declaredMediaFileExportV1`). The PDF 1.7 editor declared the action (`outputPort = artifact:native`) but its definition's `io.ports` was empty (the served descriptor: `"ports": []`), because `create_pdf17_editor` pushed `export_action()` and never set `definition.io` (the app's `io()` for the runtime came from the macro, the descriptor from the definition). The shell then fell back to the reference route (`exportAppMedia` → `encode_natural_file`), which always rewrites the whole document.
* Fix (decided against re-routing the open through the natural-source route: the open was never the defect, both routes keep the original): `semio_s_artifact_stdio_pdf_native_save::factory::declare_native_save` declares the port (`app_io()`) and the action together, `create_pdf17_editor` calls it. The served descriptor now lists `artifact:native` (`out`, `data/document`).
* That exposed the next defect: the first Save of an unkept document keeps it (`keepProgramDocument`: local catalog write, then `openDocumentRef` attaches the instance, whose attachment lane loads the archive into the instance asynchronously). The native export was submitted while that load was still running, and the guest refused its completion (`media export completed against a stale revision or generation`, `pdf-journey8`). `saveNaturalFile` (`📤️document-transfer-tabs/🔶️scope`) now drains the instance's document attachment lane before it persists and exports, so the export binds the settled revision; new-document Save is green again on the native route (`pdf-journey8b`, `pdf-journey9`). Law `the_pdf17_save_action_names_a_declared_native_output_port` applies the shell's rule to the serialized definition (exactly one `out` port with the action's port id, the codec's format and media type).
* Still open in the browser: the opened file's Save now runs the native program but writes compactly (876/899 bytes instead of 963 + an appended section). Natively the same open → edit → Save appends. The browser differs by the keep/attach of 3b: the opened document is kept and its archive loaded back into the instance before the Save, and the increment plan answers compact when the custody misses (digest of the reloaded `snapshot.objects` differs from the opened file's, or the reloaded snapshot carries no retained objects) or an edit is inexpressible; the temporary trace that would tell which was removed before this build. Next step: a native law that keeps and reloads the archive (`loadAppDocumentArchive`) between the MediaIn open and the Save, then the fix at whichever of the two it shows.
* Native result (`mis-5.log`, one gated run): both laws pass, 2/2 in 94 s: open via `MediaIn` (real `PluginRuntime`, shard budget) → custody holds the 963 original bytes → title + text edit → Save starts with the original bytes, appends under 16 KiB, lopdf reads 2 pages, the `Tj` and the title → second edit (page 2) → Save stacks on the first.

### 4. Shell: guest UI patch rejection (remaining failure 5) and refusals that failed a refresh

* Rejection: `🧺️handle-ui-intake/rejection.ts` (new). A refused patch raises `PluginUiIntakeRejectedError` (`code = plugin-ui.intake-rejected`, `origin` = surface, parsed contract rule and node from the retained-UI contract's own text, localized `notice` en/de) and is recorded per instance and surface (logged once as `console.error("[os-shell] <plugin>#<surface>: guest UI patch refused", {rule, node, …})`), cleared when the surface installs and forgotten when the instance closes. `refreshUi` explains a failed refresh with it (`explainUiIntakeFailure`): the boot error that used to read `stopped without publishing requested UI surfaces (missing=["1:pdf.thumbnails"])` is now the rejection, with the original failure as `cause`. `windowFaultFromError` carries the `notice`; the shell's error card shows it in the UI language (`windowFaultTextV1`, no default language: any other locale shows the diagnostic) with `data-semio-window-fault-code` and `-origin`, the diagnostic as `title`. Tests (bun): `rejection.test.ts` over the language-neutral `rejection-cases.json` validated by `rejection.schema.json` with Ajv (third party), `rejection.feature`; 4/4, plus `lease.test.ts` 7/7; vitest `🩺️window-fault` 14/14 and the PluginRuntime intake tests 5/5.
* Refusals through refreshes (now green in en and de): a typed-operation fault page is owned by the first live call that sees its sequence 0. When an action's refusal settled while a window refresh was draining, the refresh adopted the operation and threw the fault (`tree window refresh failed SemioFaultError: …`), so no localized notice appeared (de range fault, both unknown-field cases). `withTypedOperationCall(…, adopts = false)` marks calls that dispatch no operation; `refreshUi` uses it, so such a fault goes to the instance's notice fan-out. An unowned fault that an observer received is no longer also logged as `interactive-job.unattributed-result-fault` (it is logged only when nobody received it, as before when no shell listens). Bun: `fault-notices.test.ts`, new case per fixture row (en and de notice text, no throw, nothing reported); 13/13.

### 5. Thumbnail strip after Open File (open)

`probe-strip.log`: before and 15 s after opening `two-pages.pdf` the only strip rows in the DOM are `Page 1 / 1` of instance 1, while the open mounted the file into a new instance 2 (`typed-operation slots instance=2`, custody insert under instance 2). The standalone host opens a file as a spawned program (round 3 fix 3) and the default layout's Pages window keeps showing the host instance's untitled one-page document; the canvas, Object and Details windows that the journey edits are instance 2's. Not changed in this round: the layout has to bind the Pages window (and every document window) to the instance the opened file lives in.

### Changed files (round 4)

- `✏️s/…/📖️pdf/…/7️⃣1.7/…/🧱️base/✏️editor/💾️native-save/🏭️factory.rs` (`declare_native_save`), `…/✏️editor/🦀️.rs` (`create_pdf17_editor`), `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`shard_media_in_open` continuation, `edit_instance_pdf`, laws `media_in_of_a_small_pdf_then_edit_and_save_appends_to_the_opened_file`, `the_pdf17_save_action_names_a_declared_native_output_port`).
- `🧰️framework/…/🔌️PluginRuntime/🧺️handle-ui-intake/rejection.ts`, `rejection.test.ts`, `rejection-cases.json`, `rejection.schema.json`, `rejection.feature` (new), `…/🧺️handle-ui-intake/🟦️.ts`, `…/🧩️module-loader/🟦️.ts`, `…/📬️typed-result/🟦️.ts` + `fault-notices.test.ts`, `…/🏛️ShellHost/🩺️fault/🟦️.ts`, `…/🏛️ShellHost/🖼️canvas-footer/🔸️canvas/🟦️.tsx`, `…/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx` (attachment drain before Save).
- Ticket: `rebuild-pdf-wasm.sh` (threads=1 target override), `native-cargo.sh` (FIFO queue), `pdf-pane-journey.ts` (per-step console logs, DOM page rows, strip convergence wait, Details form reopen), new scripts listed above, this section. Temporary `[DEBUG]` traces in `🗃️original-custody/🦀️.rs` and `💾️native-save/🧱️program.rs`: added and removed (the wasm `b2c26327…` still prints them).

## End-to-end verification, round 5 (2026-11-25)

Evidence: wasm build `🗑️generated/wasm-r5b1.{out,mem.log,peaks.txt}` (r5a failed on another session's half-written `retained-graph` `close_normal`, not on memory), native laws `r5-reload-1.log` (with the temporary traces), `r5-reload-3.log` (final), journeys **`pdf-journey10/`** (one browser for all steps) and **`pdf-journey15/`** (one browser per step, final harness), open diagnostics `pdf-journey10a/`, `pdf-journey10b/`, `probe-open.log`, vitest `r5-natural-scope.log`, `r5-opened-program.log`. Scripts: `build-with-retries.sh` (new: up to three measured builds, at most 2 minutes of quiet wait each), `probe-open.ts` + `run-probe-open.sh` (new), `pdf-pane-journey.ts` (changed, see below).

### 1. Wasm build within memory

Built from a tree that contains the media-transfer acknowledgement fix (`🩹️media-in-regression.md`): `r5b1`, 08:48:55 to 09:38:53 (about 50 min), staged `7653cb9f…` (285,191,177 bytes). Host and headless Chrome were stopped. Peak physical footprint (20 s `top` samples) of the largest rustc processes: `semio_hub_stdio_pdf` **7.44 GB**, `semio_hub_stdio_media` 6.18 GB (another session's build that ran alongside, outside the gate), `semio_s_artifact_stdio_pdf_model` 4.14 GB, `semio_hub_stdio_pdf_component` 3.81 GB (link), `semio_s_artifact_stdio_binary_io_registry` 1.95 GB. No guard stop, no kill.

### 2. Journey (wasm `7653cb9f…`)

| Step | en | de |
| --- | --- | --- |
| Out-of-range page: localized notice, `role=alert`, `aria-live=assertive` | PASS | PASS |
| New document, title, insert text, Save `%PDF-1.7` | PASS (880 bytes) | PASS (902 bytes, journey 10); journey 15: the browser was killed by the host warden during the Save (see 5) |
| Reopen the saved file (title, text) | PASS | PASS (journey 10) |
| Open `two-pages.pdf`: `file-opened`, canvas, **strip shows `Page 1 / 2` and `Page 2 / 2` and no row of another document**, page width 595, title "Original Title" | PASS | PASS |
| Edit the opened file and Save: **saved file starts with the original 963 bytes** | PASS (1,438 bytes) | PASS (1,461 bytes) |
| Reopen the saved file (title, text, `Page 2 / 2`) | PASS | PASS |
| Export / Import archive round trip | PASS (2,578 bytes) | PASS (2,577 bytes) |
| Details pane: title of an untitled document | PASS | PASS |
| Details pane: unknown-field notice, `role=alert`, assertive | PASS | PASS |
| No console errors beyond known noise (new, opened, transfer, details) | PASS ×4 (journey 15) | PASS ×4 |

Counts: the 24 verdicts of round 4 all passed on this build: journey 10 passed 25 of 26 verdicts, and the one failure was the input ledger's refusal line for the out-of-range insert that the range step provokes on purpose. It is now classified (see 4). Journey 15 (final harness, with de Export/Import added) passed 24 of 26 product verdicts plus the two new de transfer verdicts. The two failures are the de new-document step, whose browser process the host warden killed by SIGKILL (`cancel cap`, 4,255 MB over the 4,096 MB cap of the `browser` kind; a rerun of the step alone, `pdf-journey15de`, was killed the same way). Round 4: 18 of 24.

### 3. Causes and fixes

**Wrong page list after Open File (failure 1).** A standalone program host (no host app, `hostMode` false) published an opened file as a *spawned* panel entry (`focusSpawnedProgram`). The canvas followed `activeSpawnedId`, but `focusedProgramV1` takes a spawned entry only in host mode, so the layout's windows (Pages strip and the rest) stayed bound to the primary session, which was the untitled instance 1. Decision: there is one rule for every Open path (`📂️opened-program`, schema-first neutral cases with Ajv). A host keeps the opened program as its focused tab, as before. A standalone program host *adopts* it as its one program through the session switch (`adoptPreparedPrimarySession`), and every window then follows it. Import Document already worked this way. The predecessor is persisted and then retired; it is never kept as a hidden tab. `publishOpenedProgram` in `📤️document-transfer-tabs/🔶️scope` serves Open File, reopening a kept file and Import Document. `keepArtifactOpeningV1` awaits an asynchronous publish, so the adoption runs only after the archive and the catalog entry are durable. If the switch refuses, the opened instance is retired and unlisted.

The adoption exposed two more defects, both fixed:

* **Retirement closed an instance while its attachment was still loading.** Persisting the untitled predecessor keeps it and attaches it, and the attachment lane then loads the archive back asynchronously. The switch closed the instance during that load: `runtime close cleanup faulted for instance 1: … zero-progress` (`pdf-journey10a`). The fix: `persistProgramDocument` drains the instance's attachment lane after keeping it, as the Save path did since round 4 (`pdf-journey10b`: gone).
* **The predecessor's edits were checkpointed after it was gone.** `useCheckpointOnCloseV1` fires once the session key changed, so its `commitCheckpoint` addressed the program that replaced it. The input funnel refused it (`owner-mismatch … dialog origin is not current`), and the left document stayed uncommitted. The fix: `publishOpenedProgram` checkpoints the primary's uncommitted edits while it still owns the canvas, before the switch seals it.

**Save of an opened file was a full rewrite (failure 2).** The leading hypothesis is refuted. The new native law `media_in_then_archive_reload_then_edit_and_save_appends_to_the_opened_file` opens through `MediaIn` on the real `PluginRuntime` and then reads the document archive and loads it back into the same instance, as `keepProgramDocument` plus the attachment lane do. The round trip keeps the document identity, every retained object and the custody digest (`[28,14,e9,34]`, 8 rows before and after). The custody still holds the 963 original bytes, and the Save appends. In the browser, temporary `[DEBUG]` traces (custody insert/find, Save route, baseline digest; added and removed again, but still printed by the staged wasm `7653cb9f…`) showed the following on this build: `custody insert … bytes=963` on open, then on Save `route=true`, `baseline digest=[28,14,e9,34]`, `custody find … held=[([28,14,e9,34], "")]` (a hit), then `custody insert … bytes=1438` (rebase). The saved file starts with the original bytes in en and de. So the archive reload was never the cause. On this build the browser Save takes the incremental route once the opened document is the session's program. The round-4 compact files came from the mixed binding of failure 1, in which the session program was still instance 1 while the canvas showed instance 2. That was not proven again on the old build. The new law stays as the regression for the reload.

### 4. Harness changes (`pdf-pane-journey.ts`)

* Open step: also fails on any page row that does not end in `/ 2` (`staleRows`). Page width and title are read from input values: the Object pane now shows them as editable fields, so `innerText` never contained them.
* Export/Import runs in en and de (`transfer-en`, `transfer-de`).
* Console verdict: the input ledger's own refusal line for the out-of-range insert (`insert-text refused: dispatch-failed … — stdio.pdf.page-number-out-of-range`) is listed as `intended`. The range step provokes this refusal and checks it as an announced localized notice, and the ledger logs every refusal by design. Every other refusal, error or warning still fails the verdict.
* One browser per step, Playwright's signal handlers off. A disconnect the journey did not ask for is reported as a failing verdict with the step it hit.

### 5. Open: the browser tree of the PDF editor exceeds the warden's browser cap

Since about 09:40 the host warden (`🛡️warden`, enforce mode, `~/Library/Caches/semio/host-capacity/warden/alarms.jsonl`) kills headless browser trees above 4,096 MB (kind `browser`, `signal SIGKILL`). It also suspends them under memory guard. The journey browser running one PDF editor page reached 4,104 to 4,281 MB at eight cancels, during a new-document Save or an Open, and was SIGKILLed (`<process did exit: exitCode=null, signal=SIGKILL>`, `pdf-journey14`). Journeys 11 to 14 and the de new-document step of 15 died this way. The suspected cost is several shard workers, each with its own instance of the 285 MB component. The custody trace's `held=0` on reopen shows a fresh module memory per program instance. That cost is a product memory concern and was not changed in this round. The warden was not changed either.

### 6. Tests

* Native (`r5-reload-3.log`, one gated run, debug traces removed): 5 of 5 pass: `media_in_then_archive_reload_then_edit_and_save_appends_to_the_opened_file` (new), `media_in_of_a_small_pdf_then_edit_and_save_appends_to_the_opened_file`, `media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_under_the_shard_budget` and `…_under_the_interactive_grant`, `the_pdf17_save_action_names_a_declared_native_output_port`.
* Vitest: `💾️natural-scope` 38/38. New: standalone adoption after durability, refused switch retires and unlists, a kept file reopens as the program or needs no switch, the predecessor's checkpoint comes before adoption. Reattachment is now drained before the instance may close. Fixed at its cause: the readback test now awaits the probe that follows the folder `read`. `📂️opened-program` 8/8 (new: Ajv over `🔣️.json`, one case per row).

### Changed files (round 5)

- `🧰️framework/…/🏛️ShellHost/📂️opened-program/🟦️.ts`, `🔣️.json` (new); `…/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx` (`publishOpenedProgram`, checkpoint before adoption, attachment drain in retirement persist, pure `keptOwnerHeld`); `…/🏛️ShellHost/🗂️local-catalog/📥️artifact/💾️opening/🟦️.ts` (asynchronous publish); `…/🧪️tests/📂️opened-program/🟦️.ts` (new), `…/🧪️tests/💾️natural-scope/🟦️.tsx`; `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (suite registered).
- `✏️s/…/📖️pdf/…/7️⃣1.7/…/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`reload_document_archive`, the reload law). Temporary `[DEBUG]` traces in `💾️native-save/🧱️program.rs`, `🧷️increment.rs`, `🚪️io/📥️import/🗃️original-custody/🦀️.rs`: added and removed.
- Ticket: `pdf-pane-journey.ts`, `build-with-retries.sh`, `probe-open.ts`, `run-probe-open.sh`, this section.

## End-to-end verification, round 6 (2026-11-25)

Evidence: `🗑️generated/pdf-journey/` (final, 36 of 36), earlier runs of the same round `pdf-journey-r6a/` (27 of 36, handed-over wasm), `pdf-journey-r6b/` (32 of 36, rebuilt wasm, before the tree toggle fix), `pdf-journey-r6c/` (36 of 36 after the toggle fix and script fixes), probes `pdf-journey-expand/`, `pdf-journey-a11y/`; per run console logs `console-*.log`, memory samples `r6-mem.log` (and `r6a/b/c-mem.log`), wasm build `wasm-r6xw1.{out,mem.log,peaks.txt}`, descriptor `r6xd1.log`, `round6-chain.log`, vitest `r6-tree-windows.log`, `r6-interp-tree-windows.log`. Host: `serve-pdf-native.sh` (port 6260) from `run-round4-journey.sh`, one browser per step. The first run used the staged wasm of `r6w-stage.log` as handed over.

### Step table

| Step | en | de |
| --- | --- | --- |
| Out-of-range page: localized notice, `role=alert`, assertive | PASS | PASS |
| New document, title, insert text | PASS | PASS |
| Reopen saved file (title, text) | PASS | PASS |
| New document: no console errors | PASS | PASS |
| Open `two-pages.pdf` (strip `Page 1 / 2`, `Page 2 / 2`) | PASS | PASS |
| Edit opened file, incremental Save (original bytes kept) | PASS | PASS |
| Reopen incrementally saved file | PASS | PASS |
| Opened PDF: no console errors | PASS | PASS (r6a and an earlier de a11y run: intermittent `zero-progress`, see 3) |
| Export then Import archive | PASS | PASS |
| Transfer: no console errors | PASS | PASS |
| Details pane: title of an untitled document | PASS | PASS |
| Details pane: unknown-field notice, alert, assertive | PASS | PASS |
| Details: no console errors | PASS | PASS |
| a11y: inspector titles are real headings with a level | PASS | PASS |
| a11y: page row starts collapsed, ArrowRight expands, move row named | PASS | PASS |
| a11y: move row focusable, named, visibly focused, Enter commits the move | PASS | PASS |
| a11y: last page offers only its earlier move, ArrowLeft collapses | PASS | PASS |
| a11y: no console errors | PASS | PASS |

Final run: en 18 of 18, de 18 of 18 (36 of 36). First run of the round: 27 of 36 (en 4 and de 4 a11y verdicts, plus de opened-PDF console).

### 1. Fixes

**The framework interaction verbs were missing from the PDF editors' roster (guest Rust, wasm rebuilt).** A keyboard or pointer pick on a page row dispatches `interactionSelect`, which the shell refused: `app … dropped action "interactionSelect" dispatched from window kind "pdf.thumbnails": no window kind declares it`. Cause: `install_object_interaction` adds the `objects` interaction domain to the already built `AppDefinition`, after `build_definition` ran its injection pass (`try_build_definition` injects the six interaction verbs only for domains declared before the build). Since reserved verbs left the window rosters (`🗂️pdf-descriptor-action-sets.md`) the descriptor carried none of them (the committed descriptor had zero occurrences). `install_object_interaction` now pushes `interaction_action_definitions` that the roster does not declare (`🎬️actions/🦀️.rs`, shared by the 1.4 canvas). The `pdf17-editor` action set now holds `interactionSelect`, `interactionHover`, `clearSelection`, `selectAll`, `setSelectionMode`, `setInteractionGranularity`; the committed descriptor pair was settled (`settle-pdf-descriptor.ts`, `round6-chain.sh`): 1,300,331 JSON bytes, 304,518 pack bytes. Rebuilt wasm staged (`r6xw1`, three components, descriptor 1,299,041 bytes).

**A disclosure toggle in a window tree never showed (host TS).** A tree in a window body is host-controlled: `onOpenStateChange` goes to the shared scheduler, which records the open state and asks for a refresh, but `TreeView` derives its open rows in a memo keyed on the store revision. A guest whose answer to the toggle leaves the document unchanged (the thumbnail strip) never bumps the revision, so the row stayed collapsed: every toggle reported `open=true` again, `aria-expanded` stayed `false` (the Details tree could not collapse either, `pdf-journey-expand`). `createTreeWindowContextV1` now exposes `subscribe` and `revision`, bumped by every `setOpen`; `TreeView` subscribes with `useSyncExternalStore` and re-derives its open map on it. Tests: `🛠️ShellHelpers/🧪️tests/🪟️tree-windows` 17/17 (new law: subscribers hear every toggle, in order, until they unsubscribe), Interpreter tree window laws 24/24 matching `-t "tree window"`.

**Journey script (wrong, not the product).** The page-row focus indicator check required an outline or box-shadow; this design system paints keyboard focus as a stroke of the shared-border compositor overlay (`data-shared-border-state="focus"`, `🎨️.css`: native ring off by design), so the check now accepts that segment. The "move committed" check compared row sizes and canvas hashes, which are identical for two equal pages; it now inserts a marker text on page 1 and reads which page heading precedes it in the Text/Details window (page 1 to page 2). The last-page check counted the move row after ArrowLeft had collapsed it, and now counts before, and also requires the row to have expanded. New diagnostic step `expand` (disclosure click, ArrowRight, move-row click, a second tree).

### 2. Memory

Largest single process seen by the 15 s `top` sampler across the runs: 2,164 MB (headless browser renderer, first run), 1,641 to 2,012 MB in the later runs, against the warden's 4,096 MB cap for kind `browser`. No `kill` or `cancel` alarm for this ticket's processes in `alarms.jsonl` during the round (19 `suspend`/`resume` pairs under memory guard and one `danger` suspend, all resumed within seconds). Wasm build `r6xw1`: see `wasm-r6xw1.peaks.txt` (largest rustc `semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests` 3.82 GB and `semio_s_artifact_stdio_pdf_model` 2.83 GB, both from another session's gated builds; the three hub crates 1.7 to 2.0 GB under `threads=1`).

### 3. Remaining gaps

- **Intermittent `plugin.internal.zero-progress` while retiring the predecessor after Open File** (`runtime close cleanup faulted for instance 1 … elapsed 10500us, ceiling 8000us`, logged as `switchToPluginApp: predecessor … retirement failed`). It hit the de open and de a11y steps once each in the first run, never in en and not in the two later full runs. The rule (`runtime_close_stall_verdict`, `🔁️cleanup-turns`) faults after eight consecutive empty close steps and a real-time stall credit of 8 ms, so a loaded browser turns a ladder that is merely waiting into a fault; the faulted instance leaves quarantine and stays un-retired. Not fixed: a wasm rebuild per hypothesis is too costly and the cause (what the ladder waits on after the attachment drain) needs a native reproduction. The verdict `de opened PDF: no console errors` is the one that can flake.
- The Details tree could not collapse before the toggle fix; other plugins' trees whose guest answers a toggle with an unchanged document were affected the same way and are fixed by it, but only the PDF editor was re-run.
- The committed descriptor pairs of other plugins are still stale against the reserved-action move (`🗂️pdf-descriptor-action-sets.md`).

### Changed files (round 6)

- `✏️s/…/📖️pdf/…/7️⃣1.7/…/🧱️base/✏️editor/🖼️page/🎬️actions/🦀️.rs` (`install_object_interaction`).
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/🔣️.json`, `🛂️.descriptor.semio` (settled).
- `🧰️framework/…/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx` (`createTreeWindowContextV1`), `…/🗣️Interpreter/🪟️tree-windows/🟦️.ts` (`TreeWindowContextValue.subscribe`/`revision`), `…/🗣️Interpreter/🧱️node-tree/🟦️.tsx` (`TreeView`), `…/🛠️ShellHelpers/🧪️tests/🪟️tree-windows/🟦️.tsx` (new law). Temporary `[DEBUG]` traces in the first and third file: added and removed.
- Ticket: `pdf-pane-journey.ts`, `round6-chain.sh` (new), this section.
