# Circle German Dark Verification

The official dark-source derivation generated a three-line driver from the canonical German source. Registered Tectonic 0.16.9 compiled it through the existing compiler reference projection and local encoded bundle. Compilation exited successfully. No checked source, fonts, icons, class, packages, or templates were copied into the ticket.

| Property | Result |
| --- | --- |
| Native PDF pages | 3 A4 |
| Native PDF bytes | 133948 |
| Catalog language | de-DE |
| Title | Fingerübung Parametrisierung |
| Author | Architektur |
| Overfull / underfull diagnostics | 0 / 0 |
| Text outside page bounds | 0 |
| Ledger arrows | 20 |
| Anta ledger digits | 58 |
| Actual visible graph height | 126.243 mm |
| Ratio to prior 83.6 mm | 1.51008 |

## Independent Checks

check-circle-dark.py reads native metadata with pypdf and native text bounds and ledger typography with pdfplumber. It passed all assertions.

The independently maintained verify-directed-pdf.py passed on the actual dark PDF: all 240 directed cubic background arcs, the 20 exact selected directions forming a closed perimeter plus four chords, all 266 native arrowheads including the worked example, distinct antiparallel curves, endpoint and arrowhead clearance outside icons, 16 vector icons, full-width graph fitting, ordered ledger, ledger gutter, and all six directed worked-example arcs. Its actual visible-height measurement includes number labels.

## Visual Review

review-k16-pdf.py rendered every page through bundled PDFium. The contact sheet and full-resolution third page were inspected. The selected ring runs clockwise through every numbered icon and closes from 16 to 1. The taller full-width graph, instructions, complete two-column ledger, final prompt, header and footer remain separated and fit without clipping or overlap. The two matrices and worked example retain their layouts. Arrowheads remain visible beyond node pads.

The host emits its existing Fontconfig diagnostic during native compilation; no TeX layout diagnostic was emitted.

Retained authored inputs: circle-geometry.case.json, circle-geometry.verify.cjs, and check-circle-dark.py. Retained reports: circle-geometry.md and circle-dark.md. Derived driver, compiler outputs/cache, and rendered review images remain confined to the ticket generated directory for the parent agent to remove after integration.
