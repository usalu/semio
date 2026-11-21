# Browser Acceptance — 2026-11-20

Live interaction acceptance is blocked. `repo run @semio-tech/framework-os-dev:dev-note-react-prototype` was admitted but waited on the pre-existing shared project-graph load. The canonical React port is 6080; HTTP requests to ports 6080, 6041 and 6180 could not connect. The earlier direct launch recipe `repo tools exec --light bun -- ./📜️script.ts serve note react dev` now exits with `Original parent Script process invocation required`. The computer-use browser inventory was empty, and creating an in-app browser returned `Browser is not available: iab`. No editor interactions were verified.

A subsequent launch check confirmed that the installed Playwright Chromium headless shell starts successfully (version 156.0.8078.4). Ports 6080, 6041 and 6180 still refused connections while the guest build remained active.

The canonical server was also attempted through the owned ServeScript without waiting for the shared graph. Browser support, fonts, and the Note session were prepared, but serving requires the Note browser guest. Guest compilation encountered and progressed past several PDF and framework errors; the seventh incremental build is active after a Note asset map fix, with 670 units reached and no new diagnostic at the latest checkpoint. The server has not started and live interaction acceptance remains pending. The ticket probe now asserts artifact addition, selection, delete and undo, pencil and undo, and reload persistence, saving diagnostics before assertions.

Subsequent passes progressed through the PDF, framework, Note schema, presence and SQLite compiler errors as those sources were corrected. The ninth pass found a PDF indirect-object derive error, also corrected; the tenth pass is active. Concurrent source changes and compiler capacity pressure have required repeated incremental builds. No guest or listening server has been observed, and no live interactions have been verified.

The tenth pass hit a transient BRep manifest duplicate that was subsequently corrected. The eleventh compiled the Note descriptor and then hit a stale transitive instance-docs API; the referenced methods are now present. The twelfth pass is waiting on shared compiler capacity. The live acceptance remains unverified.

The twelfth pass cleared the previous PDF and instance-docs errors, then exposed missing Note mutation DSL derives, which were added. The thirteenth stopped at Cargo admission; after a selection fix, the fourteenth attempt entered planning. A fresh check still could not connect to port 6080, so the acceptance probe has not run.

The fourteenth pass cleared Cargo admission but captured a stale PDF lexer dependency; the manifest and locks were corrected. The fifteenth pass is active and reached approximately 703 units without diagnostics. The guest is not staged, so the editor URL remains unavailable.

The fifteenth pass reached Note schema and found a missing typed DSL receiver for text children, now fixed in the store owner. The sixteenth found a stale engine call, corrected by its owner. A seventeenth attempt is active; the server remains unavailable and live acceptance is pending.

The owned server eventually listened on port 6080 and the Chromium baseline was run against the Note editor URL. Boot failed before any surface mounted: `data-semio-os-error=note`; the console reported `Window owner must declare its original disposer birth` and a refused `setActiveExample` dispatch. The probe saved its boot snapshot and stopped immediately. No interaction or persistence assertions ran.

## November 21 runtime

Attempt 40 compiled the real Note WASI guest, sealed its source proof and ran all eleven publication stages with zero native failures. The outer repository command subsequently exhausted retries for a moved shared generation and returned 75. The module receipt was already published and the repository activation accepted it. The browser component source digest is `2cf05d6d12f58d3347f3fbea0089961c70d17a8173bf139592a4237586d83607`. The owned Vite route returned HTTP 200 at the Note editor URL after activation issued a new artifact receipt.

Chromium reported `data-semio-os-ready=note` and mounted the actual Canvas and Navigator. The command form added a text block, then a second default text block, which Undo removed. A separate fresh tab added and rendered Math through the Artifact panel. A text, text, undo, Math sequence reported a publication stall once and displayed Math on a later refresh. The first test did not set the custom Kind combobox, so it did not prove Table creation.

Selecting the Pencil utility in the first trace left later hover, selection, camera and ink commands failing with the exact native publication-owner diagnostic. A fresh Math-and-pointer probe did not reproduce that diagnostic. The wheel now cancels browser scrolling through a non-passive listener scoped to the Canvas. Chromium reported that the new wheel event was canceled, and all 18 mounted renderer tests passed, including a fixture check that the Navigator leaves the wheel event alone. The restaged utility owners and the full browser interaction run are pending.

The second Chromium run selected Table in the actual custom combobox, produced a table in the Canvas, and removed it with Undo. It then reproduced the delayed Math publication. After selecting Text in the Artifact panel and focusing the Canvas, Delete removed the Text row from the Artifact panel, while the Canvas still showed two blocks. The utility-switch registration now has a focused neutral registry fixture and is in staging attempt 41.

A fresh Demo session opened with no blocks after a page reload, even after the first Math block had appeared. The explicit Export action downloaded 369 bytes of Note DSL containing that Math block. Export Document produced a separate 805-byte Note Pack archive, SHA-256 `63176c020a2e832986f47bcaf090fc178f2e747b2cbee027f8c73352580b150b`. Import Document accepts this archive format. In a fresh editor it admitted a new instance, then failed during Pack and SPR hydration with `Replay: limit exceeded: retained record-body physical credits`. The imported instance's cleanup then reported the refused previous outcome. The authored Demo reload and the editable archive are recorded separately in the browser probe.

## Attempt 42

The owned build linked 879 Cargo units and sealed 4,925 provenance files. All eleven staging steps completed and the owned Vite route served the Note editor after a new activation. Chromium mounted Canvas and Navigator with an empty Demo and reported `data-semio-os-ready=note`.

The first real Add Text, Table and Math commands all reported a publication stall. The diagnostic Text run reached operation 192, Publishing stage 1, ownership flags `0b01011100` and paid progress 694. Four thousand ninety-six observations left that state unchanged. The operation had a pending durable publication, with no child, group or result page. Math later appeared after an additional refresh. Further source-level evidence is needed to identify the blocked Store subphase.

The Pencil flow reached a different original ownership refusal on following gestures. The current artifact has exact registered utility and tool owners, but did not settle the retained caller. The canvas wheel still zoomed from 1 to 1.08 about the pointer, with 0.062 CSS pixels of observed anchor error.

The 805-byte archive no longer emitted the former Pack allocation refusal. A focused import with Vite HMR disabled failed with a subsequent native semantic turn refusal, and its cleanup reported the prior outcome. The original worker payload needs to be captured before the worker-to-main error transport truncates it.

## Attempt 43

The HMR-off Vite route accepted the newly published guest. Two fresh Chromium sessions opened Canvas and Navigator. The Text diagnostic showed a pending artifact Store owner in Closing, with no accepted receipt, no returned or outstanding document snapshot reads and no pending result page. At streak 4,096 the top-level turn still had more than seven million items, 1.73 GB of copy budget, depth 64 and over 47 million fuel. The likely constraint is inside retirement admission or scheduling. Snapshot read pressure is ruled out for this reproduced turn.

The first Pencil-related hover and selection now reach a guest panic at the retained ordered map’s terminal drop assertion. The preceding utility command had already held the lane for five seconds. The previous original emission capture error did not appear in this focused reproduction. Following commands trapped after the panic; no successful Pencil gesture or Undo was observed.

## Attempt 45

The published guest starts in Chromium and mounts both windows. Text and Table create successfully, Table Undo removes the block, and the Artifact panel adds rendered Math. The panel selects the exact Text row, Canvas Delete removes it, Undo restores it, and Redo removes it again. A second Undo restores it. These operations completed without the previous 4,096-unit publication fault.

The in-place edit needs a separate diagnosis. A fresh editor did expose the text overlay and accept the marker in its DOM, but the committed block remained blank. A table double-click did not reliably expose its editor. The test now uses separate block positions, tracks whether the editor was visible and logs focus loss.

A separate Pencil run still reproduces the ordered-map terminal panic. The browser now captures the exact Wasm instruction stack through a diagnostic-only stderr hook. The release core is stripped. Paste and export in the full sequence occurred after the trap and therefore provide no independent evidence about those operations.

An import-only run reaches a new cleanup continuation after the worker reservation correction. It precedes the semantic refusal at epoch 28877 and reports `original lifecycle child receipt differs from its whole producer receipt` with two copied items and 2,613 copied bytes. The focused diagnostic retained that cause before transport reduction. The imported block did not display.
