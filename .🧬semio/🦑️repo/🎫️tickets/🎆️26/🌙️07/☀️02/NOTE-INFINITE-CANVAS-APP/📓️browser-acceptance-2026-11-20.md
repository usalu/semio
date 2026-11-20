# Browser Acceptance — 2026-11-20

Live interaction acceptance is blocked. `repo run @semio-tech/framework-os-dev:dev-note-react-prototype` was admitted but waited on the pre-existing shared project-graph load. The canonical React port is 6080; HTTP requests to ports 6080, 6041 and 6180 could not connect. The earlier direct launch recipe `repo tools exec --light bun -- ./📜️script.ts serve note react dev` now exits with `Original parent Script process invocation required`. The computer-use browser inventory was empty, and creating an in-app browser returned `Browser is not available: iab`. No editor interactions were verified.

A subsequent launch check confirmed that the installed Playwright Chromium headless shell starts successfully (version 156.0.8078.4). Ports 6080, 6041 and 6180 still refused connections while the guest build remained active.

The canonical server was also attempted through the owned ServeScript without waiting for the shared graph. Browser support, fonts, and the Note session were prepared, but serving requires the Note browser guest. Guest compilation encountered and progressed past several PDF and framework errors; the seventh incremental build is active after a Note asset map fix, with 670 units reached and no new diagnostic at the latest checkpoint. The server has not started and live interaction acceptance remains pending. The ticket probe now asserts artifact addition, selection, delete and undo, pencil and undo, and reload persistence, saving diagnostics before assertions.

Subsequent passes progressed through the PDF, framework, Note schema, presence and SQLite compiler errors as those sources were corrected. The ninth pass found a PDF indirect-object derive error, also corrected; the tenth pass is active. Concurrent source changes and compiler capacity pressure have required repeated incremental builds. No guest or listening server has been observed, and no live interactions have been verified.

The tenth pass hit a transient BRep manifest duplicate that was subsequently corrected. The eleventh compiled the Note descriptor and then hit a stale transitive instance-docs API; the referenced methods are now present. The twelfth pass is waiting on shared compiler capacity. The live acceptance remains unverified.

The twelfth pass cleared the previous PDF and instance-docs errors, then exposed missing Note mutation DSL derives, which were added. The thirteenth stopped at Cargo admission; after a selection fix, the fourteenth attempt entered planning. A fresh check still could not connect to port 6080, so the acceptance probe has not run.

The fourteenth pass cleared Cargo admission but captured a stale PDF lexer dependency; the manifest and locks were corrected. The fifteenth pass is active and reached approximately 703 units without diagnostics. The guest is not staged, so the editor URL remains unavailable.

The fifteenth pass reached Note schema and found a missing typed DSL receiver for text children, now fixed in the store owner. The sixteenth found a stale engine call, corrected by its owner. A seventeenth attempt is active; the server remains unavailable and live acceptance is pending.
