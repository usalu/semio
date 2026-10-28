# Closed Inner Cycle

The four additional highlighted paths now form the clockwise cycle 1→5→10→13→1. The outer cycle 1→2→…→16→1 remains intact. The same selection feeds the graph and the twenty ordered answer labels, so the last four writing rows match the new inner loop exactly.

There are still 240 base directed arcs and exactly twenty highlighted directions. The inner cycle uses four nonadjacent, clockwise-ordered vertices and has no self-crossing. The taller K16, shared L/B/H example, two-line K3 captions, two compact capsule matrices and three-page layout are unchanged. Production editing was limited to the four selected inner edges.

## Reproducible Selection and Tests

The existing seed 20261027 feeds an unsigned LCG and full Fisher-Yates shuffle of vertices 1–16. Each sample takes four vertices, sorts them clockwise and requires cyclic gaps between three and five. The first accepted sample is attempt 19, vertices 1,5,10,13. This produces four interior edges distinct from the outer ring.

Version-eleven schema and language-neutral fixtures pass. An independent D3 shuffle reproduces the same sample; independent degree and traversal checks verify the inner cycle has one incoming and one outgoing edge at every participating vertex. Negative cases reject the flipped closure, disconnected or duplicate selections and ring-edge overlap.

## Native PDF Verification

- Independent German directed PDF audit confirms all 240 base arcs, twenty selected arcs, 266 native arrowheads, exact ordered ledger, both closed cycles, icon clearance, full width and 125.603 mm graph height.
- Mixed-example and matrix audits pass, preserving the numeric/categorical examples, all six K3 labels and both centered nine-by-nine grids.
- Native German light, English light and German dark compilation succeeds. Final German log has no overfull, underfull or missing-character errors.
- All three final German pages were rendered and individually inspected. English and dark versions were also independently rendered and reviewed. Both cycles are clearly closed; the inner paths do not self-cross, and all twenty answer rows fit.
- Frozen bilingual content hashes remain unchanged. Ledger writing rules remain 64.60 mm with a 4.08 mm minimum inner gutter.

## Final Artifact

- output/pdf/Fingeruebung-Parametrisierung.pdf
- Three A4 pages, 134057 bytes
- SHA-256: af9bdd0bdb90ad8d1755f40aedc580bde6f6ce7abef36c3ab6a80841b64db15f

Authored inputs and audit reports remain in this ticket. Temporary compiler outputs and renders are removed after verification. Original resources, existing styling and concurrent repository changes are preserved.
