# PDF Compositing

The native PDF painter currently drops isolated group ancestry and applies layer opacity separately to fill and stroke. The shared language-neutral compositing fixtures already specify overlap, nested opacity, sibling scopes, multiply blending, and fill/stroke overlap for canvas and SVG.

Implement isolated PDF transparency Form XObjects, applying group opacity and blend mode once when invoking each form. Each form owns only its used resources, preventing recursive resource dictionaries and duplication of unrelated assets. Leaf effects also require a form so fill and stroke are combined before opacity is applied.

The normative basis is [Adobe ISO 32000-1, section 11.6.6](https://developer.adobe.com/document-services/docs/assets/35e4369068f86065372c18787171a17e/PDF_ISO_32000-1.pdf): a transparency group is composited into its parent as one graphics object; an isolated group begins with a transparent backdrop; group execution resets blend mode, alpha constants and soft mask before painting the group's content.

## Implemented Source

The native PDF exporter now opens and closes shared group scopes using their common ancestry prefix. It emits compressed Form XObjects with an isolated RGB transparency group and applies opacity/blend once at each form invocation. Leaves with their own opacity or blend also receive forms; their geometry is borrowed rather than cloned. Paint-level fill/stroke alpha stays inside the form. Group metadata rejects reopened IDs, conflicting ancestry, invalid blend names and nonfinite/out-of-range opacity.

Each page/form records used resource names. A resource index resolves only those names when writing its dictionary, so nested forms do not scan or inherit unrelated resources and cannot refer to themselves. Existing page orientation and complete world transforms remain in effect.

Added native tests for all five shared compositing scenes, invalid scope metadata and acyclic/scoped resources. Updated the existing combined gradient/text/image/opacity test to inspect form paint streams as well as the page stream.

The native fixture test optionally writes its real exported PDFs into `SEMIO_DRAW_PDF_ORACLE_DIRECTORY`. A registered Draw TypeScript test uses PDF.js and @napi-rs/canvas to render those bytes and compares every pixel against an independently rendered Sharp SVG reference on white. An explicit canvas factory keeps intermediate PDF transparency surfaces on the same native canvas binding as the destination.

## Validation Status

Run 85503 (`tests-pdf-oracle-wiring.txt`) completed with exit 0: 197 passed, one skipped, 46,955 assertions across 27 files; the existing Ajv field-patch and publication-authority audits also passed. The skipped test is explicitly the new native-PDF pixel oracle because no native exports were supplied. This is wiring/regression evidence, not proof that the new Rust exporter compiles or renders correctly. The explicit canvas-factory refinement was made after this run and remains unexecuted.

Native run 5483 (`tests-native-compositing.txt`) predates this PDF work and remains live. Depending on when Cargo reads source, it may or may not include the new tests; inspect its final roster before claiming coverage. Component build 93519 (`build-draw-inspector-rows.txt`) also remains live. Neither was restarted, and no browser action occurred.

## Next Verification

After 5483 actually terminates, run the existing Draw native Nx test target with `SEMIO_DRAW_PDF_ORACLE_DIRECTORY` set to this ticket's `🗑️generated/pdf-compositing`. Use the existing ticket-local Nx cache and TMPDIR environment. The focused Cargo test filter is `io::export::pdf` if it matches the final test roster; otherwise use the complete library suite. Then run the registered Draw TypeScript Nx test target with the same oracle-directory value. Missing PDF files must fail the oracle rather than silently skipping once the variable is set. Inspect raster differences and fix them before claiming PDF parity.

## Remaining Scope

PDF TypeScript export itself is still an empty entry point. Native compilation, PDF pixel parity and browser download delivery remain unverified. PDF font embedding/Unicode, gradient-stop alpha, broader image formats and incremental export progress/cancellation remain incomplete. Native and raster renderer compositing parity, effect-bearing ungroup, and the rest of the end-user acceptance plan also remain open. This change does not narrow the overall goal.



## Blend Vocabulary Follow-Up

The shared corpus now has 37 scenes, and the PDF writer now uses the actual camelCase authored vocabulary for the four multiword blend modes. Native mapping tests were added before that correction. See [blend round trips](🎨️blend-roundtrip.md). The latest TypeScript suite passed 270 tests / 145,453 assertions with only the native PDF pixel oracle skipped. Native fixture generation remains pending on the active native build; the original five-scene count above describes the earlier checkpoint.
