# Note Completion Checks

The acceptance target is the repository-owned Note editor running the real guest with a fresh browser profile. The ticket's existing feature plan also names rich text, table cells, resizing, multi-block gestures, erasers, clipboard and keyboard shortcuts. Test results below describe the coverage available for each area.

## Current status

The repository-owned Note server on port 6080 is serving the managed attempt 57 guest. Native Cargo linked all 880 units and the artifact build recorded zero failures across 11 stages. The published component receipt is `5a55b3eb29a25e7471746adeccb90bbf58d2ed811ac7fa2a215a9af49878f61d`, byte-for-byte the linked candidate. It records source digest `88ac9542166edeff4be3a4ba88f288950e6385cac93a4186f64afbfe037f5ece`. The staged and HTTP-served WebAssembly core agree at 99,295,879 bytes and SHA-256 `ae502d654040428593176e396578bbe2b6dba6c574600700354593dec7afe326`. After publication, the repo operation reported status 75 when a concurrent Cargo registry change invalidated its post-build snapshot. The outer wrapper exited nonzero. The artifact report and independently verified published bytes confirm that packaging completed.

Fresh Chromium passed all six scenarios in the main acceptance batch and all three focused canvas scenarios against that guest.

| Area | Observed result |
| --- | --- |
| Editor and history | All 21 full editor checks passed. Chromium created Text, Table and Math blocks, committed bold text and edited a table cell. The Artifact tree reflected selection. Delete, Undo and Redo updated the document. |
| Drawing and erasing | Pencil added one stroke in Canvas and Navigator. Point Eraser split it into two. Stroke Eraser removed one fragment. Undo restored both fragments, then the original stroke, then the empty drawing. |
| Geometry | Group movement translated both blocks by 68 × 41. Eight resize samples grew one block from 280 × 120 to 332 × 147 while its neighbor kept its size. Undo restored Canvas and Navigator. |
| Clipboard and camera | Block copy and paste created a second block from a 329-byte clipboard payload. Anchored zoom moved from 1 to 1.08000004 with 0.0612 px of pointer error and survived a 12 × 8 SVG paste. |
| Archive | The full editor downloaded a new four-block archive, reloaded into a fresh Demo, and restored the same bold text, edited table cell, Math and SVG. Independent history-only, Math-only and mixed archives also reopened. Both focused restored documents accepted a subsequent edit. |
| Viewer and cancellation | Viewer displayed one block with zero editor controls. Import displayed an active progress bar. Cancelling preserved the current document and allowed a subsequent edit. |

The 212 mounted renderer tests passed across seven suites, and the separate media and file suite passed all 15 checks. The 22 language-neutral tests passed with 168 assertions across 12 files. The mounted runs cover ink editing, clipboard, marquee and operation scheduling. The neutral checks use SQLite as an independent oracle for retained grant and ordering contracts.

All four renderer contract generators completed. The project-owned typecheck reached its two-minute execution limit. The pinned compiler completed directly with the same renderer project configuration and reported 27 diagnostics in other renderer code and fixtures. Two supplementary native replay fixture tests compile and link, then fail at their authority or payload admission assertions. The standard graph-backed dev launch could not complete its preflight while the shared graph builder held admission. Live verification used the repository-owned React dev activation. The separate WGPU browser route remains unverified.

See [ink tool authority](note-ink-tool-authority.md), [native archive grant](note-native-archive-grant.md), [native fixture evidence](note-native-replay-fixture-authority.md), [media picker verification](note-original-media-picker-cancellation.md), and [standard launch preflight](note-standard-launch-preflight.md).

## Earlier checkpoints

| Area | Earlier evidence | Then required |
| --- | --- | --- |
| Dev contribution and launch | Ajv verifies the launch declaration. Attempt 45 served a sealed Note guest on port 6080 and Chromium mounted Canvas and Navigator. | Repeat against the final staged guest. |
| Artifact creation and editing | Chromium on attempt 45 added Text, Table and Math after the Store Closing fix. The mounted React tests cover editing text on blur, Escape, table cell navigation and targeting rendered cells. The live editor opens both fields, but the native `updateBlock` derivation omits paragraph and cell changes. | Add all three blocks, edit text and a table cell, then observe the settled artifact and Canvas. |
| Selection, resize and movement | Attempt 45 reflected the Text selection in the Artifact tree. The settled Chromium probe translated both selected blocks in Canvas and Navigator, then restored them with Undo. An isolated resize published 280 × 120 → 332 × 147 and Undo restored both views. The mounted suite covers the fix for compounded pointer samples. | Repeat the multi-sample resize and group movement on the final guest. |
| Drawing and erasing | Attempt 45 Pencil reached the retained ordered-map assertion. The rebuilt guest compiles the pending-effect view borrowing correction. The mounted point-eraser fixture clips a sparse segment and preserves nested fragment order. | Switch tools, draw, erase and undo through the real guest. |
| Clipboard and camera | The current browser anchors optimistic zoom at the pointer. A diagnostic run recorded the camera command holding the Note lane, so clipboard is also under a fresh run with no wheel event. Chromium pasted and rendered a 12 × 8 SVG in isolation. A separate run copied the settled Text selection and pasted its Note clipboard payload as a second block. The mounted React tests cover image data URLs, SVG priority and the read-only Navigator. | Verify camera completion together with a subsequent document action on the final guest. |
| History and keyboard | Attempt 45 created a Table and removed it with Undo, then selected and deleted Text, restored it, reapplied Delete with Redo and restored Text again with Undo. | Verify Delete, Undo and Redo after the settled publication paths. |
| Archive | An earlier browser exported a Math archive. Attempt 45 clears the Pack cap and worker underreport failures on import, then reports a Note replacement initializer receipt mismatch. A later initializer correction is in the attempt 46 build. | Export after edits, import from a clean session, verify text and image content and observe an open new document. |
| Other renderer | WGPU browser transport tests passed. The WGPU renderer needs its own published module for a live smoke. | Validate the separate renderer if completing the WGPU lane. |

## Attempt 51 startup diagnosis and attempt 52

The attempt 51 published guest loaded and activated, but Chromium's instance-open turn hung on the first live instance. The V8 CPU profile identified a cyclic registry scan in the native picker step. The three idle shard workers kept responding while the active worker was blocked. The picker now stops before revisiting any earlier slot. A language-neutral four-case fixture also validates the bounded scan with a separate SQLite ordering oracle.

To verify causality while the source build ran, an isolated browser context streamed a copied Wasm with just the equivalent cursor check at the profiled loop. The copied module validated with V8 and never replaced the published product. The instance-open request returned after 37 continuation polls. The previously stalled editor then completed the Demo replay and mounted both the Canvas and Navigator with no shell error. Attempt 52 is the source-built publication used for final acceptance.

The isolated guest also added Text and Table from the real Canvas Actions pane and committed and undid a typed table cell. The Bold toolbar reproduced a distinct React focus bug. The mounted red suite saw a formatting pointerdown take focus and immediately publish one premature edit. Both editor overlays now keep their pointer events out of Canvas gesture recognition, while a pointerdown on the Canvas still commits by blurring the editor.

The focused Pencil run reached the typed pending-effect path and reproduced the original ordered-map Drop assertion in that separate runtime path. The hot frame decoded to the fresh ViewModel clone in the background pending-effect driver. That driver and its matching surface-refresh branch now borrow the registered view under their existing instance guard. The already-fixed exchange branch remains a borrow too. Attempt 52 was still compiling when these two Rust fixes landed; its exact source claims require verification before acceptance.

The toolbar regression went from one failure and 41 passes to 42 passes. A separate Chromium run confirmed that clicking Bold kept the editor focused, created actual bold markup, and saved the bold run on blur. The same browser run passed the table-cell commit and Undo checks.

The archive trace decoded the 805-byte Math archive and opened and acknowledged a second Note instance. Its load returned an initializer fault because the Note document-store owners had no native semantic replay producer. The existing shared cold-mutation factory is now registered for both replay and single-item publication in the editor and viewer. The host additionally reports an import error before waiting for failed-instance cleanup without releasing the transfer gate early. The 29-case transfer suite passed after the notification-ordering change.

The broader isolated Chromium run confirmed Math creation, selection in the Artifact tree, Delete/Undo/Redo, a saved table-cell marker, and SVG paste with decoded dimensions of 12 × 8. Pointer-centered zoom moved from 1.0 to 1.08 with 0.0612 px anchor error. A fresh image-only run downloaded an archive, but export after the longer gesture sequence waited behind an interactionHover command and exceeded the browser test timeout. React previously sent every pointer sample even when its hover target was unchanged. React now matches the WGPU renderer's reuse of the previous hover target. The neutral sequence checks repeated targets, background clear, pointer leave, and re-entry, with SQLite independently deriving the four transitions.

The hover change passed the mounted suite with 43 tests and matched the existing WGPU transition behavior. In a separate long run, a later export still encountered a slow setCamera command, although the native manifest step reached its finished phase. The transcript distinguishes that phase from the final native reply. Final acceptance places a real SVG document edit after wheel zoom and verifies the resulting changed scene retains the anchored zoom. The isolated mixed-document export also runs without the camera step so that archive evidence can be compared with the longer sequence.

With the hover fix and without the camera or Pencil steps, Chromium downloaded a 4,044-byte mixed document archive after a normal series of edits. The live saved scene had four blocks, including a restored Bold marker, the authored table cell, Math, and the decoded 12 × 8 SVG. The archive SHA-256 was 4729a5f727f315000284bffafd3dc6bd701bc00e3c23ec285242fa18e55df0e3. A reload returned the empty Demo. The open Artifact dock covered the Canvas Actions button at that point; the acceptance driver now explicitly closes the visible Artifact dock before clicking import. The mixed archive is kept as a generated acceptance input until the ticket closes.

On the actual attempt 52 publication, fresh Chromium passed the Note shell boot, Text, Table and Math creation, bold editing, table-cell edit and Undo, Artifact row selection, Delete/Undo/Redo, in-app block clipboard, multi-selection translation in Canvas and Navigator, and eight-sample resize from 280 × 120 to 332 × 147 with an unchanged neighbor. This evidence uses the published guest and no isolated Wasm replacement.

The same guest's Pencil flow still reached an ordered-map destructor, with a different Wasm frame sequence in the SurfaceVisible mounting path. The mixed archive reached replay on the new producer, then refused an actual narrowed close grant. Native diagnostic epoch 99847 recorded a 1,584-byte copy demand against a 932-byte issued grant, with 792 bytes capacity and depth 7 required. The outer source had 21 MB of copy grant, which belongs to a different scope. These two additional failures are under repair before another publication.

A real attempt 52 viewer switch and the one-Math archive independently trapped during history input selection. The published Rust unit did not match the newer replicated native replay source. The current shared change waits for a published projection through is_some rather than treating an admitted but unpublished slot as input. The next build must verify that exact source claim and retest both paths.

In the attempt 52 long interaction sequence, camera zoom moved optimistically to 1.08 with 0.0612 px anchor error, but the following SVG did not publish within 85 seconds. A focused fresh-session check separates that path from the longer interaction history and its resource use.

Two fresh attempt 52 Chromium runs reached WindowConfig's native canonical sealing continuation after a later document edit was queued behind wheel zoom. The late Wasm breakpoint loop reached retained authority closure, and the independent raw Wasm memory sample read phase byte 5. The typed operation never reached presenter or window-generation refresh. The runtime continued responding to empty polls.

In the old camera guest, the retained authority closure received 3 Work units and required 4 for the 53-byte selected close. The debugger reached the comparison and recorded both values without changing native behavior. The new canonical quote delegates to the actual child's Work demand. The attempt 53 retry compiler dep-info matches that source.

A separate real Chromium session on attempt 52 opened the Tasks panel during mixed archive import and observed the active progress bar. Cancel closed the transfer, preserved the original empty Demo, and allowed a subsequent normal Text block edit. The completed archive case uses a separate session.

The isolated captured attempt 53 retained the anchored zoom after publishing a real 12 × 8 SVG edit. The focused case observed zoom 1.0800000429153442 and offset [−5.11999989, −10.17879963] at both steps, with 0.0612 px pointer anchor error. The first test incorrectly compared the newly updated block count a second time; the corrected test uses the immutable initial count and passed.

The standalone shell reopened the archived one-block Math document in the captured guest and accepted an additional Text block. The test selected Close and discard only for its fresh, empty Demo. The detached profile had no document data folder. The older native guest then warned about zero progress during background retirement of the predecessor, and the new document remained editable.

The combined current-source neutral test run passed 9 tests and 86 assertions across archive polling cadence, receiving grants, maintenance wake, native transpiler preflight, camera authority Work and Pencil window projection. The browser runner and isolated binding helper also pass syntax checks.

The actual published attempt 54 passed anchored zoom after SVG paste and, in the full editor session, a Pencil stroke in both Canvas and Navigator with Undo restoring both. The focused empty Demo case selected Pencil visually but did not produce a stroke or reach its eraser checks. The actual mixed and Math archive baselines did not complete on that earlier native scheduler. The older full acceptance run was stopped after the verified Pencil and Undo checkpoints so the corrected guest could build.

Managed attempt 55 compiled 880 Rust units and sealed compiler provenance. Its compiler dependency claims match the current native sources for archive maintenance scheduling, archive replacement initializer diagnostics, replay receiving grants, cleanup progress validation, and scoped window projection. Guest publication and browser checks are tracked separately.

The nine current-source neutral suites passed 16 tests and 135 assertions for mount ownership, archive waking and polling, standalone adoption, replay grant narrowing, camera authority and batching, native preflight, and window projection.

The Note browser host now routes Pencil and Eraser using the mounted surface's own shell utility register. All 47 focused renderer tests pass. The live attempt 55 browser opened an empty Demo and completed drawing, point erasure, stroke erasure, and Undo in Canvas and Navigator. See note-ink-tool-authority.md.

The actual attempt 55 mixed archive avoided the old Source copy exhaustion and reached store initialization, which reported that native construction exceeded the original remaining Job grant. Viewer and the focused Math archive failed during live cleanup of their prepared instance. A direct Viewer worker receipt showed an issued three-item grant and a four-item callback receipt, with 323 copied bytes and 224 bytes of retained capacity. The managed guest and browser suite therefore remain under native initialization repair.

A fresh browser on attempt 55 also cancelled the active mixed-archive transfer from the Tasks panel, preserved the initial Demo, and added a Text block afterward.
