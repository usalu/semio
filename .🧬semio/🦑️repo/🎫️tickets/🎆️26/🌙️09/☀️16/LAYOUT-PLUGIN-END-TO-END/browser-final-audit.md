# Final Browser Helper Audit

2026-10-28. Source inspection only. No build, test, browser execution or source edits. Read root, OS and layout AGENTS plus prior final-audit, browser-validation and export-journey. Paths named s/layout resolve to emoji-prefixed folders in this checkout.

## Active Artifact Scope

Use exactly one visible panel root:

```js
const artifact = page.locator(`[data-slot="panel"][data-panel-visible="true"][data-active-tab-id="framework.panel.artifact"]`);
assert.equal(await artifact.count(), 1);
const rows = artifact.locator(`[role="treeitem"]`);
```

Panel implements these attributes in framework UI elements/Panel lines473-478; mobile Layout lines67-71 follows the same contract. Its visible ID is framework.panelTab.framework.panel.artifact. Use descendant root layout-document if needed, but do not assume the root itself has role tree. Query row IDs only inside this visible panel, never document-global. The document panel has stable page rows layout-document.page.<id> and bare frame IDs. Inspector rows can coexist elsewhere and hidden/old bodies must not contribute.

Artifact is a focused-program panel, not an instance-tagged subtree. Do not invent a data-instance-id attribute on it. Record the active canvas owner before import using `[data-slot="mode-dock-tab"][data-active="true"]` data-window-id; Dock source lines121-125 defines it. Record all window IDs if the panel takes focus. Spawned imported windows use spawnedWindowInstanceIdV1(spawned.id, guestId), unlike the old layout-blueprint IDs; mode-windows lines47-84 shows the new owner and generated per-window engagement. After import wait for a new Blueprint window ID absent from the preimport ID set, then focus its ordinary tab, reopen Artifact, and resolve the panel locator anew. Require saved exact IDs and saved marker, and absence of the later original-only page/marker. Do not take the first engagement or the first global frame-1. Scope action pane to the new observed engagement/window. A changed window ID plus restored content is the ownership witness; the panel itself supplies no independent session ID.

## Edited Media Witness Through Existing UI

Public addFrame exposes kind (select rect/text/image), x and y only. It adds a 120x64 rect with fill [0.2,0.24,0.3,1], or a text frame referencing the first existing story. Its ID is frame-(page.frames.len()+1), so derive the actual newly introduced ID by set difference rather than assuming a globally unique generated ID. patchFrame is in_palette=false. There is no patchStory action. Do not expand fictitious Actions forms for either.

Simplest robust geometry marker: create a rect on page-1 with x=251, y=337 through addFrame. Scope controls to action.addFrame.arg.x and action.addFrame.arg.y and locate the descendant numeric input; kind is a select control, not fillable text. Execute only once and assert exactly one new frame ID. Export page-1 before adding another active page. Default fill yields a large distinctive interior patch; use a point well inside it, e.g. (280,360), and compare against a pre-edit export to prove that this location changed from baseline. If fixture already paints that location, choose another observed blank position. Assert at least an interior patch, not a single antialiased edge pixel.

For a textual marker, use Inspector, not a private dispatch: open framework.panel.inspection, find `layout-play-inspector.global.story.story-1.content` (or selected equivalent), and its descendant textarea whose ID begins `layout-play-inspector.global.story.story-1.content.span.` and ends `.input`. Current short demo text has one chunk. Fill a short marker such as LAYOUT-EDIT-731 and blur the textarea to commit. Inspector document_input uses Trigger::Commit with commit blur. It routes patchDocument with a fingerprinted story.<id>.content.chunk.<start>.<end>.<stamp> field. Re-resolve after each edit because chunk ranges and fingerprint change. Assert the textarea value after rerender. The selected text frame variant is layout-play-inspector.selected.story.<storyId>.content.span.<start>.<end>.input. This works through actual UI, without exposing internal patchFrame in palette.

Use both witnesses if possible: text recovered by pdfjs, and geometry patch recovered by independent PNG decoding. Current retained SVG encoder does not emit frame IDs or text nodes: engine/export/validate-plan-and-svg lines306-325 writes PaintShape paths/images, and the SVG header at254 is a white root rect with explicit width/height/viewBox. Therefore a textContent assertion for the marker in SVG is incorrect. Rasterize SVG using sharp and inspect the same authored rect interior as PNG; assert page dimensions and compare before/after pixels. PDF text marker plus page count proves edited document content; PDF rasterization can additionally validate the same geometric marker when an existing renderer is available. Require marker BEFORE all three final exports; stockdemo exports can remain baseline images but do not count as edited export proof.

## Exact Undo Without Extra Ledger Entries

Current add-frame-then-undo cannot prove the created document snapshot: it checks only a log and later count. Keep Artifact open throughout the measured sequence. Open and expand addFrame form before the baseline, make any panel positioning/docking gestures before baseline, then record exact row IDs; submit, poll the same already-mounted Artifact DOM without any clicks, assert newIds.length===1 and no removed IDs; press Mod+Z immediately; poll same panel and require exact baseline frame/page ID arrays. Press redo immediately and require the exact post-create arrays. DOM reads, screenshots and polling add no history entries. Do not call countWithPanel, togglePanel, change selection, or reopen action panes between submission and undo/redo. Focus Execute before submission so keyboard undo does not land in a text input. If the overlay covers the form, position panels through public UI before baseline; do not solve the obstruction by toggling after create. Existing forced click is a weak actionability witness and should be reported if retained.

## Helper Corrections

- submitAction must scope every arg control to the action argument row, use select interaction for kind, fail on missing/disabled inputs, and distinguish zero-arg row execution from staged-form execution. Current helper returns no-execute-control even after valid export row execution and logs absent args without failing.
- state must read frame/page/selected IDs from the one visible Artifact panel; return ownerWindowId separately. Assert all recorded steps, including artifact-panel-baseline, have no surface fault/error. Check host phase/fault fields, not only console regex.
- After import, await new owner plus restored content instead of fixed three-second delay and global page IDs. Re-resolve owner/action pane and avoid retained original locators.
- PDF Hello layout expectation must change to the authored marker after editing the story. SVG cannot recover text via DOM textContent because glyphs become paths.

No live pass is claimed. Native activation and live server6079 remain prerequisites owned by the main task.
