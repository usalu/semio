# Leibniz University Hannover Footer Logo

Added the LUH logo as the default chip in the footer's bottom-left zone. Both 444×131 PNG variants are unchanged copies of the Zwischenbericht originals. The existing theme resolver chooses the dark image for dark documents.

The print compiler's shared source catalog now stages the logo directory for every document. The footer uses a locally fixed left-to-right layout on both page sides; header mirroring retains its existing behavior. The existing footerlogos command can replace or clear the logo list.

## Verification

- Added language-neutral light/dark cases to the page-chrome fixture and Gherkin.
- Confirmed the new tests fail before implementation.
- The focused page-chrome suite passes: default selection, customizable logo list, physical-left placement, shared library inclusion, and both theme variants.
- SHA-256 comparison confirms the shared PNG bytes equal their Zwischenbericht originals. The independent sharp decoder confirms dimensions and identical decoded pixels.
- Updated the quick pipeline's expected shared library roots and integrated the logo checks.
- Whitespace diff check passes.
- Full quick pipeline remains blocked before tests execute: source admission cannot stat C:/git/semio/.cache/b/l/fixed.dir.
- The direct authored-script route also fails before tests: @semio-tech/framework-transient cannot be resolved.
- PDF rendering remains unverified because of these repository setup failures.

The retained verify.ts runs both border and logo checks through repo tools exec bun. Shared source files and concurrent assignment-catalog edits were preserved. No generated ticket files remain.
