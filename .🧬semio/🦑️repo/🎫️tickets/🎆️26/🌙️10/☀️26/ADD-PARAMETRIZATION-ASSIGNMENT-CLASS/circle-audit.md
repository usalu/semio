# Directed Circle and Height Audit

The v9 language-neutral schema and fixture require the selected perimeter 1→2→…→16→1, preserve the four chords and 20 ordered ledger entries, and set the global vertical radius to 53 mm. The local worked example remains unchanged.

The TypeScript semantic verifier passes. It checks the authored ordered ring, visits all 16 vertices before returning to 1, and independently uses d3 Cartesian products plus mathjs sums to confirm one incoming and outgoing perimeter edge at every vertex. Flipping the closing edge to 1→16 fails both the traversal and degree checks.

The actual PDF verifier now requires the 16→1 overlay and ledger arrow, a 53 mm vertical radius, visible height 1.5±0.03 times the previous 83.6 mm, full text width, all 240 directed curves and arrowheads, and all 20 selected directions in order. The fresh German native PDF passes all directed, matrix and mixed-form verifiers. Complete visible height is 125.603 mm including number labels, 1.50243 times 83.6 mm; the icon bounds alone are not the full visible graph. The graph remains about 170 mm wide and the document has three pages.

Native checks confirm 240 base directed cubic paths, 20 selected overlays, 266 native arrowheads across K16 and K3, correct source/target endpoints including 16→1, unobscured arrowheads, noncoincident reverse paths, exact ordered ledger arrows, and writing gutters. Both page-two matrices remain 9×9 with 6.14 mm writing rows; their centered artwork has minimum insets 2.13–2.22 mm. Four mixed parameter rows, six edge-local categorical K3 labels, continuous dimensions and signed closed-cycle arithmetic remain intact.

Changed files: production test cases schema, cases fixture, feature contract, TypeScript verifier; ticket verify-directed-pdf.py and this audit. No production geometry or bilingual content was edited by the audit owner.
