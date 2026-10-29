# Browser Journey Version2 Implementation Review

2026-10-28. Read-only review of browser-journey.mjs/json, media-oracles.mjs, media-oracles.test.mjs and launch-layout.py against browser-final-audit.md and relevant current UI contracts. Requested browser-media.test.mjs does not exist; actual self-test file is media-oracles.test.mjs. No syntax checks, builds, tests or live browser run performed in this review.

## Required Correction

P1: stageAction kind uses row.locator(select), one(select), selectOption. The current staged renderer uses the framework custom Select. Its trigger is a button with role=combobox/data-slot=select-trigger; its options are div role=option/data-slot=select-item/data-value. There is no native select element. The browser journey necessarily times out at its first authored addFrame form despite three media self-tests passing.

Correct through normal UI events: find one trigger inside the action.addFrame.arg.kind row, read aria-controls, click trigger, locate the visible listbox with that exact ID, click its option with data-value=rect, and verify aria-expanded=false. If needed reopen and assert the rect option aria-selected=true, then Escape before recording the history baseline. Source: staged-args lines301-314; framework UI Select lines405-418,658-663,773-777. Use data-value rather than translated label. This preserves locale independence and real user activation.

## Strong Witnesses Implemented

- Artifact observations now scope to exactly one visible active Artifact panel. New imported Blueprint window IDs must be absent from the full preimport set; the normal tab is focused and restoration requires that owner plus exact saved frame/page IDs. The later original-only page and story marker are excluded. This closes the old hidden-instance/global-tree false pass.
- SVG and PNG export baseline before edits and edited output afterward. The independent sharp reader requires authored dimensions and the exact dark interior patch;95percent of pixels must change and both formats must agree. PDF requires the short authored marker, exact page count/dimensions and absence of stock Hello layout. These are real edited-document witnesses, not extension/header checks.
- Creation observes a multiset difference with exactly one added frame and no removed frame, then compares exact frame/page arrays on undo and redo. Artifact remains open and no panel toggles or selection changes occur between submission and redo. Keyboard activation follows explicit Execute focus. The form is staged before opening Artifact, so overlay geometry is bypassed only through legitimate keyboard focus/Enter rather than synthetic dispatch or force-click.
- Missing/duplicate/disabled controls fail. All recorded steps check console errors, page errors, visible surface fault markers and host fault/phase fields. Baseline is included. Zero-arg transfer/export actions legitimately return submitted=row; listeners are registered before UI invocation. Detached importer uses the actual filechooser and rejects multiple-file mode.
- Launch script delegates all commands through repo, pins current catalog, and runs the new journey in the workspace. It does not secretly compile or test outside repo.

## Limits and Optional Tightening

The immediate story-marker assertion reads the controlled textarea value after blur; it alone cannot prove guest publication completed. Current later PDF and archive marker assertions independently establish that commit, so this does not create a final false pass. Preserve those later assertions.

The600second deadline closes Chromium; SIGINT/SIGTERM also close it. Browser-dependent work then rejects and finally writes failure/cleans listeners. However Node-side sharp/pdfjs work is not canceled by browser.close, and Chromium launch itself happens before the deadline is installed. For a strict whole-process bound, use an outer launcher timeout/signal forwarding with the existing repo process facilities or add a final hard-exit grace timer after cancellation. Record deadline versus user signal explicitly in report.failure for diagnosis. This is lower priority than the deterministic Select blocker and does not invalidate completed witnesses.

The allowlist currently suppresses any entire console line containing contributions push refused empty pack even when its type is error. Prefer an exact known-message match if the line is ever widened to contain other diagnostic content; do not broaden this exception.

The three self-tests validate real independent media readers and negative cases. They do not mount staged controls, exercise locator ownership or deliver keyboard/filechooser events, so they could not detect the Select mismatch. A live server run remains necessary after correction.

No live pass is claimed. Root and fonts writer were notified of the Select correction.

## Follow-up Source Checks

Owner window locator is correct: framework Window renders GhostRegionShell id={id} data-slot=window (lines288-289), and Dock publishes the identical raw window ID. Spawned window IDs are therefore safely addressed with exact attribute matching after import. The engagement toggle suffix .engagement.toggle matches childElementId output. AddFrame argument keys kind/x/y match its declaration; no booleans are present in the current contract. Generic boolean support is unnecessary for this journey. Zero-arg export/transfer/addPage actions execute on row press; staged addFrame uses the populated form and asserts it did not take the row-only branch.

Current layout Canvas2dHost does not publish data-status-json/data-phase/data-fault; its inner WasmCanvas also renders only wrapper and canvas, with no status attributes. Null phase/fault is therefore expected for that host, not a ready-state guarantee. Do not add a mandatory phase=ready check against invented attributes. Keep owner-scoped painted canvas dimensions/screenshots plus console/pageerror and shell/window fault markers. If generic hosts are checked, Board2dHost publishes data-board-status-json (not data-status-json), whose fields are snapshotParsed/refusalReason; read that actual attribute and fail nonempty refusalReason or snapshotParsed=false. World3dHost uses data-status-json. Parse a present malformed JSON status as failure rather than silently treating it as unavailable. These changes improve generic fault coverage; the retained layout Canvas2d host has no status payload to validate.

Inspector textarea source is uniquely scoped: one visible Inspection panel, then global story.story-1.content parent, then its short single chunk textarea. Input commit is blur via Trigger::Commit and patchDocument with fingerprinted field. The current short marker does not create multiple chunks. No private hooks or script-dispatched UI mutations are used in this journey. Keyboard Execute after focusing an existing DOM button is a trusted Playwright user event.
