# Live Export and Reopen Journey

2026-10-28. Source-derived workflow only; no live pass claimed.

## Exact Window and Action Scope

Window.tsx derives the pane ID with childElementId(framework.window, windowId, engagement), and the toggle appends .toggle. childElementId camel-cases each supplied segment: windowId layout-blueprint therefore yields framework.window.layoutBlueprint.engagement, not a hyphenated ID. Use the actual observed Blueprint engagement ID E, then `[id=E] [data-slot=window-action-pane]` as the action rail and `[id=E.toggle]` as its toggle. Execute IDs are childElementId(framework.window, windowId, action, verb, execute), for example framework.window.layoutBlueprint.action.exportSvg.execute. The execute control is inside the same pane but does not start with E; scope it by containment and exact suffix .action.exportSvg.execute. Action row IDs are action.exportSvg, action.exportPng and action.exportPdf. Argument row IDs are action.<verb>.arg.<arg-id>. These names are locale-neutral.

Action panes execute zero-argument actions on row click and expand actions requiring a form. A generic helper must handle both modes; it must not require an Execute button after an action that already fired. Source: ShellHelpers/window-panes/buildActionCategoryTree.

## Supported Export Inputs and Host Route

The public action IDs are exportSvg, exportPng and exportPdf. Each typed payload has optional page_id; the host bridge accepts pageId, page_id, id or value. Empty args are valid. The retained export route defaults SVG and PNG to the addressed Blueprint/Preview active page, with the first document page as fallback. PDF without pageId exports the whole document; supplying pageId restricts it. Source: editor/export-tool-job/retained/page_id and editor/args-bridge.

These are migrated retained jobs and must travel through the shell action route. Calling each leaf command handle directly deliberately returns layout-export-job-only. Their complete-download publication drains segmented output through the shell download sink, creates a Blob with the declared MIME type, and clicks a hidden anchor. Use Playwright page.waitForEvent(download) registered BEFORE the UI action, then download.failure(), suggestedFilename() and saveAs() under this ticket generated folder. Do not accept a success console message as proof of a download.

Expected output signatures: SVG image/svg+xml with absent text encoding; PNG image/png and PDF application/pdf use base64 in the guest publication, but the downloaded file must contain decoded binary bytes. For an explicit page-1 export filenames are page-1.svg/page-1.png/page-1.pdf. For whole-document demo PDF the current native test expects Demo.pdf. Filenames sanitize the source to ASCII alphanumeric, dot, underscore or hyphen and cap 128 characters. Metadata additionally carries document identity and 64 hexadecimal revision digits internally; these are not a promise of an extra downloaded JSON file.

## Concrete Browser Sequence

1. Boot the selected layout component and assert the document-root ready beacon equals layout with no surface faults. Open Artifact by data-tab-id framework.panel.artifact. Establish exact page and frame stable IDs.
2. Before adding a blank active page, export the existing content page. The actual demo DSL has document name Demo, two 400 by 500 pages, text frame frame-text-1 containing Hello layout, rect frame-1 and a deliberately missing image link. Exporting a newly created blank page proves only file delivery. Explicitly choose page-1 where the UI offers pageId, or select the page-1 row before exporting SVG and PNG with defaults.
3. For each format register the download wait before clicking the action row/Execute control. Assert successful completion, expected extension, nonzero bytes and independent parse success. Keep all files under generated. Record the exact component and source fixture.
4. Export whole-document PDF before or after addPage, but derive expected page count from the current observed document: baseline demo has two, the current interaction journey adds a third. Assert all expected pages and visible text on the content page.
5. Exercise redo after a witnessed undo and assert the exact removed ID returns. Create an additional uniquely named marker through patchDocument or patchFrame using authored action controls. Record exact stable IDs plus marker before reload.
6. Reload the SAME page within the SAME browser context and origin, wait for readiness, reopen Artifact, and assert the exact marker and added IDs remain. A fresh BrowserContext destroys the storage witness. Do not select demo again after reload because example initialization may replace the document. If reload re-seeds the demo, record that as the persistence result and inspect the actual document binding before claiming a failure in the layout codec.

## Independent Existing Libraries

- SVG: use existing jsdom with contentType image/svg+xml; assert root namespace/localName, viewBox/dimensions and expected paint/text content. Existing sharp/libvips can rasterize it and detect nonempty content beyond a blank page.
- PNG: existing sharp can read metadata and decode pixels; assert PNG format, positive dimensions matching the exported page settings and nonuniform pixel samples. A header-only check cannot establish a usable image.
- PDF: existing pdfjs-dist legacy reader is already used by the portable export oracle; reuse its Node globals setup and worker configuration. getDocument on the downloaded bytes must succeed, numPages must match the live document, getPage/getTextContent must recover Hello layout on the relevant page, and viewport dimensions must be positive. The existing oracle imports reside in export/pdf-png-and-package-encoding/tests/oracles.mjs. Do not add runtime dependencies for these tests.

## Reopen Boundary

ShellHost initial-example readiness calls dispatchActiveExample when a non-host session is not documentInitialized. Therefore an automatic demo boot is not by itself evidence of persistent document reopening. A rigorous persistence claim also needs the same durable document runtime identity/binding before and after reload, or an explicit host artifact reopen route. This review did not establish an exact durable reopen URI or a layout JSON download action. Neither should be invented from export SVG/PNG/PDF. The reserved importer in the earlier audit admits fields:in; exported visual files are not the canonical persisted layout snapshot.

## Exact Archive Picker Interaction

Source confirmation: manifest document_transfer_action_definitions injects BOTH archive actions into every app window as palette-visible Shell verbs in transfer category. The window rail renders transfer category as action.category.transfer, with leaf rows action.exportArtifactDocument and action.importArtifactDocument. Both carry no args and fire on the row click: do not look for .execute controls. Scope to the live Blueprint engagement pane as above, scroll the row into view, and use a normal visible click. These routes are Chrome audience; browser interaction is the intended public UI route.

Concrete Playwright sequence, using a previously resolved visible pane locator and generated-path archivePath:

```js
const archiveDownload = page.waitForEvent("download", { timeout: 120000 });
await pane.locator("[id=\"action.exportArtifactDocument\"]").click();
const downloaded = await archiveDownload;
assert.equal(await downloaded.failure(), null);
assert.ok(downloaded.suggestedFilename().endsWith(".semio-archive"));
await downloaded.saveAs(archivePath);
const chooserEvent = page.waitForEvent("filechooser", { timeout: 15000 });
await pane.locator("[id=\"action.importArtifactDocument\"]").click();
const chooser = await chooserEvent;
assert.equal(chooser.isMultiple(), false);
await chooser.setFiles(archivePath);
```

The import picker is a transient detached input created with document.createElement(input), type=file, accept=.semio-archive plus archive MIME, then input.click(). It is NOT appended to the document, so page.locator(input[type=file]).setInputFiles is not reliable here. Playwright filechooser.setFiles supports this normal chooser interaction and triggers its onchange. requestFileOpen reads FileReader.readAsDataURL; importDocumentArchive decodes those bytes via decodeDocumentArchiveBytes, creates a new app instance and calls loadAppDocumentArchive with progress/cancellation. The resulting focus moves to the new spawned instance. Re-resolve the pane and Artifact tree after import; the old pane locator might still refer to the original document. Assert a newly observed owner plus the exact saved marker/frame IDs.

Use a unique marker BEFORE export, change/delete that marker AFTER export in the original, then import and require the original saved marker to return in the new document. This defeats a false pass from the old document remaining visible. The input funnel returns applied immediately before the async transfer completes, so wait for actual new document content, and reject imported-failed/unreadable/cancelled transfer notices.

## Media Oracle Review

The current media-oracles.mjs uses real jsdom, sharp/libvips and pdfjs-dist; its SVG/PNG decode plus nonuniform pixels and PDF page/text assertions are meaningful independent checks. Validate SVG/PNG on page-1 rather than a newly added blank page. The function treats any unknown format as PDF; callers should pass only the known three formats or add an explicit supported-format assertion. SVG/PNG currently require positive dimensions but do not check the expected 400 by 500 page dimensions, so a crop/scale regression could still pass; compare dimensions against the selected page where settings promise 1:1 raster output. PDF checks only page 1 text and numPages, which is enough for this delivery smoke test but not complete multipage fidelity.
