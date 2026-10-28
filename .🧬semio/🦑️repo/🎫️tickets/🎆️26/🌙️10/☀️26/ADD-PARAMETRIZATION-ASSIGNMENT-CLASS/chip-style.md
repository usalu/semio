# Assignment Matrix Title Chips

The three assignment matrices can now use an empty `AssignmentTable` title while preserving native table numbering and the continuous top border. The enclosing environment sets a group-local active-context boolean, so matrix chrome no longer depends on title text. Unwrapped matrices retain their plain border.

The shared muted-header renderer previously drew a padded, bordered left chip even for an empty title. Its title box now remains empty in that case. A protected expansion preserves rich title commands; named title construction, gap calculation, baseline, and right chip remain unchanged. No title text is covered or erased.

## Validation

`chip-title.case.json` defines language-neutral empty-title, named-title, and unwrapped-matrix cases. `chip-title.verify.cjs` failed before implementation because an empty title had no independent active context, then passed all three source contracts after implementation. Native PDF inspection will provide independent third-party validation of chip geometry and numbering.

## Files

- `semio-assignment.sty`: active-context boolean and matrix open/close conditions.
- `semio-window.sty`: omit the left chip for an empty stored title.
- Ticket: `chip-title.case.json`, `chip-title.verify.cjs`, and this report.

## Native Dark PDF Validation

The official dark derivation and pinned native Tectonic compilation completed with exit 0. `chip-pdf.verify.py` completed with exit 0 using independent pypdf/pdfplumber readers. All three automatic right chips (`1.1.a`, `1.2.a`, `1.2.b`) remain connected to the full-width matrix border; no left title text or left ghost frame remains. It also verified four filled plus four blank worked rows, both eight-row exercise grids, 16 mm headers, centered artwork, and the three-page layout.

`review-k16-pdf.py` completed with exit 0. Each of the three rendered pages was visually inspected: titleless matrices, native right chips, example graph and six separate ledger rows, and the complete cycle graph/ledger remain intact. The frozen source was unchanged throughout the dark build. Every process owned by this agent finished successfully.

Dark PDF SHA-256: b584adf02554e5c2be3f5bb1d404fcc1256d2728e2076aff21d185394435656a.
