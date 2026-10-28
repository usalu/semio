# Example Table Vertical Centering Content Audit

The German and English assignment content files are unchanged from the completed unnamed-chip task. Their SHA-256 values are e27a8e3e3f08829b66572608520249fbffdebe450b487aa4817eb9c3a7da4cdd and d81636004fd518abe2e23b58c8abe581326e5ee0524eaaaf56dad7ba46cbb8fd, respectively.

Both languages retain three empty AssignmentTable title arguments, their localized section and subsection headings, all original worked-example labels and values, the outside bare K3 graph, six filled numeric/category change lines, both exercise matrices, the complete graph, the selected blank ledger, and the same two explicit page breaks.

The worked table remains schema-configured as three type columns, eight parameter rows, a 16 mm header, 6 mm row height, 2 mm padding, and a 27 mm label column. Four body rows contain length, width, height, and form; four blank rows follow. The categorical row includes ascenders and descenders, and the length/width/height labels include mathematical italic symbols. Centering must therefore measure each filled text box height and depth rather than move only its baseline. Neither content arguments nor data need modification.

Coordination with the style owner confirmed a focused shared filled-cell text-box centering change. Header/icon builders, symmetric row struts, and blank-row builders remain outside that change. No production content edits are planned.

The retained chip-content-native.py provides English metadata, three-page, automatic-chip, title-absence, overflow, and all-page rendering checks. The retained chip-pdf.verify.py independently checks border geometry and unchanged worked/exercise table shape, but neither previous oracle proves vertical centering. Final English validation will add the new vertical oracle after the style is frozen and the native route is ready.

Native English compilation and final rendered review are complete.

## Final English Native Verification

Pinned Tectonic 0.16.9 compiled the original English main file successfully with the registered offline bundle, shared generated/tectonic-cache, and compiler-input first and last in the search paths. The result is three pages and 127103 bytes, with title Finger Exercise: Parametrization, author Architecture, and en-US catalog language. There are no overfull or underfull boxes and no out-of-page text.

The strict independent raster-ink oracle passed all sixteen filled worked-example cells. Signed vertical ink-center offsets range from -0.0512 mm to +0.0370 mm; maximum absolute offset is 0.0512 mm, below the 0.25 mm contract. Italic parameter labels and categorical words with descenders are included. The underlying native cell centers and row heights remain unchanged.

The preserved-chip and matrix oracle passed: all three automatic right chips remain connected, no left title or ghost outline reappears, the worked example retains four filled and four blank rows and a 16 mm header, and the two exercise matrices retain their 9 by 9 grids, 6.14 mm row pitch, and centered canonical artwork. All three focused verification processes exited successfully.

All three freshly rendered full-resolution English pages were visually inspected. The first table text is vertically centered and readable in every filled cell. Header icons, border positions, table dimensions, graph, six filled ledger lines, both task matrices, complete graph, twenty blank ledger lines, headers, footers, and page numbering remain clean and correctly placed. No clipping or overlap was found.

No production content file was edited for this task. Final byte hashes for both content files match the baseline recorded above. No dark build or duplicate graph test was required for the theme-independent text-box change.

Temporary native PDF, logs, renderings, and machine-readable metadata remain under the ticket generated folder for parent delivery and cleanup.

English PDF SHA-256: 97ff1b2b5f7dc50fbd6134757d82a6a4b351a8e875176ed5fc33dd62eb64c325
