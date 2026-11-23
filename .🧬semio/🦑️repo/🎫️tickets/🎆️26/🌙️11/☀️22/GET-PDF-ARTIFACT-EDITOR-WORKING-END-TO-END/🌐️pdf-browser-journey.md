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
