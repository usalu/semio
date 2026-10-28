# K16 Content and Twenty Selected Changes

The bilingual assignment contains one complete graph for all sixteen types with global numbering A1–8 and B9–16. Page two retains both compact matrices unchanged. The third page holds the oval graph, twenty selected changes and reflection. Students fill only the marked pairs: the sixteen distinct ring edges and four random chords through the middle. Signed changes read from the lower number to the higher one; the reverse changes invert every sign. The cycle check explicitly uses a closed cycle of marked edges. Concurrent course and custom header changes are preserved.

The language-neutral fixture and strict JSON Schema remain version 5 for this unpublished iteration. The contract contains sixteen vertices, 120 unordered base edges, all 64 cross-table base edges, global icon numbering and a full-text-width horizontal ellipse with 32 mm vertical radius. Exactly twenty marked pairs feed one two-column ledger of ten rows at 6.3 mm pitch. The Gherkin scenarios state the same outcome independently of the implementation language.

The four chords are 1–10, 2–12, 4–10 and 5–13. They are reproducible from seed 20261027 using an LCG and descending Fisher–Yates shuffle over lexicographically ordered candidate pairs with cyclic distance 6–8. An independent d3-array shuffle reproduces the same four pairs. Every selected pair is unique, belongs to the complete K16 graph, and occurs exactly once in the ledger. All four middle chords connect the two tables.

Schema and regression tests were authored before each implementation change. The first K16 regression failed against the original K8 geometry with 28 !== 120. The twenty-change revision then failed against the old all-120 ledger because it did not consume the selected-edge list. The independent d3-array Cartesian product still enumerates all 120 base pairs, mathjs verifies sixteen choose two, and AJV rejects changes to cardinality, numbering, geometry, selected edges and page order.

The standalone lxml SVG oracle passed through the bundled Python runtime, confirming all 16 canonical source icons. The registered TypeScript verifier independently checks the SVG hashes and embedded vector PDF provenance. Final selected-geometry source verification passed through the registered Bun verifier after integration. Native English compilation and visual review completed through the correct canonical compiler-reference search precedence; both independent native PDF validators passed. See k16-english.md for the measured dimensions and final visual review.

Changed files:

- Both parametrization-*.content.tex files in the assignment template.
- The language-neutral cases JSON, schema JSON, Gherkin feature and TypeScript verifier in the parametrization-assignment tests.
- This retained ticket report.

