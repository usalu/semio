# Circle Geometry

The selected K16 edges now form a continuous clockwise circle: 1 → 2 → … → 16 → 1. The four selected chords remain 1 → 10, 2 → 12, 4 → 10, and 5 → 13.

The K16 vertical center radius increases from 32 mm to 53 mm. With the existing icon and label extent of approximately 19.6 mm, visible height increases from approximately 83.6 mm to 125.6 mm, a factor of 1.50239. The measured-width fitting algorithm stays intact. The worked example keeps its local 32 mm vertical radius and all six directed curves.

Only two production lines changed in parametrization-geometry.tex: the selected closing arc and the global vertical radius. All 240 directed arcs, 20 selected arcs, canonical icon order, bend angles, arrowhead sizes, matrix setup, and ledger coordinates remain intact.

## Validation

- Authored language-neutral circle-geometry.case.json before modifying production geometry.
- Authored and ran circle-geometry.verify.cjs against the old source: expected failure because selected closing arc was 1 → 16.
- Re-ran after the two source edits: passes exact selected ring and chords, all 240 unique directed arcs, global radius 53 mm, worked-example radius 32 mm, six K3 curves, existing bend angles, width fitting, and ledger bounding coordinates.
- Independent production and rendered PDF validation is assigned to the audit and integration agents.

## Native Dark Result

The actual rendered graph including number labels measures 126.243 mm high, 1.51008 times the prior 83.6 mm. Independent native PDF verification passes all directed arcs and arrowheads. The three-page dark layout passes metadata, text bounds, typography and visual inspection. Full evidence is retained in circle-dark.md.
