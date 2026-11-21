# PDF editor verification

The registered PDF 1.7 test opens the checked-in two-page PDF through the natural-file input port, rotates the second page through the editor action, exports through the same port, and reopens the bytes. Its independent reference uses lopdf. The inspector regression clears a title and checks that the exported author survives. The ten subset test suites now use the owned-action argument API.

The registered action and export changes still require a successful native test run. The natural-file input path currently opts into the existing single-turn decoder only within the scheduler’s work limit. Larger PDF imports require a resumable parser.

The complete TypeScript source roster passed 61 tests in 13 files. The font suite now checks the public source facade directly on cold workspaces. Added native regression cases for deleting and moving annotations, clearing title metadata, and preserving neighboring paint operations when editing a selected path.

MuPDF independently verified the before-and-after fixture streams for isolated fill changes, path deletion, and resizing a transformed Bézier curve. These comparisons validate the fixture geometry and paint. The Rust tests compare the editor’s actual mutations with the same fixtures. Document-property mutation construction now lives in a dedicated PDF editor crate.

The editor now reads PDF font metrics when positioning text, honors text rise, and preserves the following text’s baseline when a run is moved, resized, or deleted. MuPDF verified the absolute baseline and the reference results. Changes to a selected run’s font preserve subsequent font selection and use an unoccupied resource name.

The shared Stdio Rust test-support crate passes its focused compile check. Both PDF editor suites were submitted together through Cargo. Chromium is installed and launches. The shell activation retry passed the previously broken native-build import and is still checking its project graph.

A real Chromium instance painted the skewed form fixture using the production canvas renderer. All six sampled pixels matched MuPDF. On this macOS host, the default Chromium GPU subprocess stalled while loading its executable. The ticket probe runs Chromium’s in-process GPU on macOS and completed in two seconds. The full mounted shell journey is tracked separately.

MuPDF opened both incremental-update fixtures without repair. In each, the current revision frees object 3 and introduces object 9. The reader now carries free entries across older xref sections and repair scanning. The matching native conformance regression is queued.

Repository command startup also required a repair to the shared source-analysis schema. The portable owner pattern is present again, and its three focused tests passed.

The resolver now reads indirect stream lengths using the current xref revision. MuPDF independently extracted “prefix endstream suffix” from the shared document, which embeds a marker in its stream content. All six raster probes for skewed-image selection also matched MuPDF. A click on blank paper selects that page for inspector edits. An off-page click clears selection. The native editor and conformance tests cover these changes. Their current runs are still pending.

Replacing a text run or changing its font now preserves the next run’s baseline. MuPDF verified the original and edited coordinates. Cubic path bounds use derivative roots rather than fixed samples. The production Rust geometry and a 64× MuPDF raster both measured a height of four points for the off-sample extremum. During these checks, repository startup lost the public storage fresh-space export. Restoring the existing implementation’s reexport brought the launcher back.

Retained streams with an unknown or unreadable filter preserve their encoding declaration, decode parameters and original payload through materialization. MuPDF independently saved and reopened the same encoding and bytes after an unrelated metadata edit. Xref validation now reads only object headers. Import transfers the xref map and unfiltered stream bodies into their next owners directly, avoiding a second whole-document cache of ordinary objects.

Vector selection now follows painted strokes, open and closed outlines, and nonzero or even–odd fills. The standalone production geometry matched all six MuPDF raster probes, including empty stroke interiors and holes. Dashed strokes and cap/join details still share the editor’s simplified stroke treatment.

The text layer now exposes keyboard-activated page and object selection across the seven PDF 1.7 editors. The browser smoke clicks Page 2 on the canvas before changing rotation and activates it from the text layer after reopening. The PDF/JSON browser acceptance matrix rows passed 14 checks. An invalid JSON fixture was moved into the matrix’s own fixture set by adding its missing malformed input.

Page paint and hit testing now follow a convex quadrilateral clipping path, including a transformed rectangle, and restore the previous clip on Q. MuPDF matched six clip probe pixels and a vector drawn after restore. The native editor integration test is pending. General curves, multiple subpaths and text clipping still need the corresponding renderer support.

The production-source xref harness passed 43 tests. It covered typed normal-object length references, compressed object members, mixed xref revisions, row precedence, and cancellation during sparse-index migration. Package-level and mounted checks remain in progress through the repository build graph.

The parser harness now passes 47 tests. The retained graph reads supported stream filters directly from the original byte span, preserves undecodable raw payloads, and builds a generation-aware object directory as it admits decoded objects. Semantic lifting borrows that directory. The page inspector now exposes insert, remove, and move controls for the selected page using the existing transaction actions. These inspector bindings are covered by the native page test suite in progress.

## End-to-end acceptance

| Requirement | Evidence | Remaining verification |
| --- | --- | --- |
| Open an existing PDF and retain its original pages, text, resources, and properties | MuPDF and the 47-test production-source parser harness agree on selected retained objects, page order, text, filter output, and incremental revisions. | The registered artifact Open and shell journey must run against the current editor. |
| Edit text, images, vectors, page structure, and document properties | Native unit cases assert the actual typed mutations against shared fixtures. Independent MuPDF probes cover position, fill, clipping, image paint, and the supporting document properties. | The current editor Cargo suites are compiling and still need to report results. |
| Save, undo, and reopen through the normal mounted app | The registered native test uses the original file producer. The Chromium smoke drives real Open, page selection, rotation, Save, and reopen. Both are wired to current entry points. | Capture successful runs with an actual shell session and verify the saved bytes. |
| Open large files with scheduler progress and cancellation | Controlled lexer, xref, index, filter, and page-tree steps have independent grants and cancellation tests. The framework holds a completed snapshot over a checkpoint before publishing. | Wire fully bounded semantic work and publication into mounted large-file Open. |
| Render accurately on both renderers | Chromium painted the skewed-form fixture and matched MuPDF at six probes. Literal vector and clip geometry also passed isolated native checks. | The full native Canvas2d check and wider rendering tests need current results. |

The latest focused PDF harness passed 86 tests, using the real PDF lexer, xref, content parser, filter and retained-graph paths. Limited font, device-colour and external-resource shims support the isolated content checks. MuPDF independently confirmed the compressed object-stream members, the text and stroke round trip, and all 18 inline-image pixels. The paged export owner’s native birth quote was measured in a repository-built library as 6,088 copy bytes and 6,080 retained bytes, below the 32 KiB interactive copy ceiling. The registered PDF and mounted browser end-to-end tests still need a successful run. The latest browser build stopped on incompatible Pack feature units before either runtime component linked.

## Current mounted acceptance

The PDF Open owner now handles independently verified 48-page vector and Helvetica fixtures through its file and semantic cursors. Registered native tests cover the full typed Open and Save route for both shapes, including retained objects, trailer and provenance. The separate Chromium script exercises small Open, edit, download, reload and reopen, with independent modes for large vector and Type1 text. These tests have not yet run through the full mounted app. The component build has repeatedly re-admitted automatically after live workspace changes. The native gate has advanced beyond prior window-config and converted-receiving compiler errors and is still compiling the plugin owners.

The final focused graph and content suite passed 114 tests. The separate semantic tests passed 6 content, 5 page-program and 4 document cases. The 49,279-byte vector fixture took 435,663 one-unit turns and the 33,038-byte Type1 fixture took 605,487. The current semantic harness measured the inline controlled cursor at 24,704 bytes, with fixture stand-ins for the larger framework types. Existing page-content unit contracts now run in the PDF testkit owner, keeping the production lift source closure under its test line limit. The separate annotation, Type 3, embedded TrueType and coded-image paint fixtures cover the added rendering paths.

The focused lexer, filter, xref and content harness now passes 119 tests. Its new cases verify compressed indirect lengths, generation-sensitive lookup, bounded collision probes and cancellation of the separately admitted carrier and member. MuPDF independently opens the fixture without repair and returns both stream payloads across embedded marker bytes. The xref-stream test owner now uses 3,993 active lines after two index tests moved to the existing PDF testkit, which uses 3,747 active test lines.

The inspector now exposes the five page boundaries and user-unit scale. The native source probe and MuPDF agree on the cropped size and the media fallback after a clear. The page renderer now combines embedded horizontal TrueType outline clips at the end of a text object. MuPDF confirms the expected red glyph regions and the unclipped blue paint after graphics restore. PDF.js confirms Type 3 mode 7 paints its glyph procedures and mode 3 remains invisible.

The first governed native attempt reached the page editor with a stale dependency graph and rejected the newly added font and JPEG imports. The live source also used a Rust 2024 conditional in a Rust 2021 package. The conditional now parses in Rust 2021. The current native gate is configured to recapture both dependencies, run the moved testkit cases and check the page editor suite. The Chromium component build is also retrying from the current workspace after a source size change in the spatial kernel.

- [Compressed length receipts](📏️compressed-length.md)
- [Page boundary fixtures](📐️page-boundaries.md)
- [Glyph clip fixtures](🔠️glyph-clipping.md)

The controlled document lifter now accepts a qualifying indirect text note and a direct URI-action link from their exact retained object generations. Its focused source suite passed 15 cases, including partial cancellation, and MuPDF independently verified the shared annotation fixture. The mounted browser acceptance script now includes accessibility-driven text, vector, image, title and page box edits, uses the shell history rail to undo and redo the last edit, and checks the downloaded bytes through an independent MuPDF oracle. The script and both MuPDF reference inputs pass their focused checks. The full mounted run remains behind component compilation.

- [Controlled annotations](📋️controlled-annotations.md)
- [Mounted browser verification](📋️mounted-pdf-e2e.md)

The initial current-source browser generation warmed more than 740 units. Its admitted PDF lift source digest differed from the live annotation source, so the browser build released its stale compiler lease and rejoined the queue. The older native eight-package request can now take that lease when the host has sufficient capacity. Build caches were kept.

The staged physical reservation fix admitted the exact 33,038-byte Type1 specimen under one-unit mounted grants. It classified all 48 empty 32 KiB windows, used 186,848 work turns, and retained 188,261 bytes in the graph. All 48 decoded content streams matched MuPDF. The staged 38,096-byte image specimen likewise passed physical Graph preparation, typed image roles, full sample comparison and cancellation immediately after the history reservation. The parked source awaits the current package and browser baselines.

The staged CFF reader and SFNT outline interface passed the full FontTools contour and CID comparison. MuPDF also opened the 4,177-byte embedded-font PDF, recovered its text and baseline positions, and rasterized all four glyphs. [CFF outline receipts](📋️cff-outlines.md)

## Current Integration and Staged Work

The governed native source admission passed all 720 units after the annotation fixture regression moved from the full testkit closure into the existing snapshot-value tests. The native and WASI captures compiled the revised original-slot and declaration modules. The WASI compiler emitted the PDF xref, lift, fonts and page editor. Complete component receipts and mounted Open, edit, Save and reopen are still pending.

The real browser route needed an authored Specific development contribution. The product-owned Vite and browser entry are available. The production catalog generator and natural-file router passed with the isolated metadata line. Activation requires the actual Stdio and PDF components and there is no compatible prebuilt Stdio receipt in the normal cache. The Cargo metadata binding remains staged during compilation.

- [Mounted PDF runtime gate](📋️mounted-pdf-e2e.md)
- [Registered PDF and native gate](📋️native-editor-gate.md)
- [Mixed large-file text and image import](📋️mixed-resources.md)
- [Atomic page boundary draft fix](📐️page-boundary-draft-fix.md)
- [Raw CFF and OpenType glyph outlines](📋️cff-outlines.md)
- [Selected font and text mutation checks](📋️font-reassignment.md)

## Integrated Source Status

The baseline native replay passed 31 graph-projection tests. Its page editor binary passed 45 of 50, including Type 3, TrueType and annotation painting. Three fixture arena failures and one integer versus decimal assertion were corrected in the shared page tests. The fifth failure exposed the cold DCT stream custody defect. The live fix passed all seven focused Rust checks and PyMuPDF reproduced the coded-image fixture and pixel data. The refreshed governed Canvas check remains pending. [Cold image evidence](cold-dct-import.md)

Two registered tests independently passed their draft and rectangle edit undo and redo histories. A clean natural-file fixture exposed a missing runtime birth and initial cache step in its test setup. The registered Open and snapshot decoder fixtures were corrected for the next native pass. [Native gate evidence](📋️native-editor-gate.md)

The first full WASI capture compiled the PDF renderer and codec owners. Its independent BREP consumer and provider used different live source epochs and neither final component linked. The production browser target restarted after the coordinated PDF release. Actual mounted Open, edit, Save and reopen still need a successful browser run. [Mounted gate evidence](📋️mounted-pdf-e2e.md)

## Additional Acceptance Coverage

The live mixed-resource source check passed all 16 cases. Its 38,459-byte specimen has four pages with independent image and Helvetica text references. The production graph source probe independently matched all 48 Flate content streams with MuPDF and proved exact release of the standard history allocation on cancellation. These checks exercise the real source cursors beneath the application route. [Mixed resource receipts](📋️mixed-resources.md)

The registered native PNG law now checks the ordinary picker effect, captured revision, retained completion, straight RGB and soft mask, preservation of another page, rejection of a replayed stale revision, and one Undo and Redo. The law and its neutral fixture are live in test-only source. It has yet to run in the queued governed native gate. [Native gate evidence](📋️native-editor-gate.md)

The large JPEG import and renderer test are staged for the next coordinated source capture. The focused Rust path imported, cancelled, lowered and reopened the genuine 44,486-byte PDF. An independent MuPDF render confirmed exact encoded streams and page pixels. The new image and stream owners reach 57 Stdio component units and 44 PDF component units, so both publications must refresh after release. [Controlled JPEG evidence](controlled-dct-import.md) [Browser closure findings](browser-closure-audit.md)

The active WASI capture has compiled outputs from the four current PDF history schemas. Each emitted unit-cache dependency digest matched the live label input. This establishes compiled source custody for the five English and German labels. Mounted details and native history assertions are still pending.

The integrated-source lexer, stream filter, cross-reference, content and font harness passed 124 tests against the current repository-built native dependency lane. It exercised the current CFF module and xref stream normalization. The dedicated live mixed-resource and coded-image probes cover the separate controlled XObject owner.

Build recovery now supports one held-lane retry for a warm unsplit pass with multiple components. It also verifies independently linked component inputs when a different component fails. Repository driver laws proved that a stale source on an unlinked branch can be omitted and that a stale or missing shared dependency still refuses the linked proof. Both changes are live for fresh build processes. [Retry law](📋️component-retry.md) [Partial provenance law](📋️partial-cargo-provenance.md)
