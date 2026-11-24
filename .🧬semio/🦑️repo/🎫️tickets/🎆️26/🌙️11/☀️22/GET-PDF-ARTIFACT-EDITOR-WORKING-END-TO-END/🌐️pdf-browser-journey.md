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
