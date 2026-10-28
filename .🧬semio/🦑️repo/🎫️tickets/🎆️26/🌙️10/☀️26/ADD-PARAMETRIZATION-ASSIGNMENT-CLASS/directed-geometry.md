# Directed Assignment Geometry

The shared graph geometry now declares every ordered pair of distinct K16 vertices exactly once in `AssignmentDirectedEdges`: 240 arcs. The existing twenty selected pairs preserve their lower-to-higher directions and supply both the dark graph overlay and the arrow-labelled writing ledger.

All graph edges connect named icon nodes. TikZ computes source and destination rectangle borders from the actual icon boxes; additional shortening keeps arrowheads outside those boxes. Edges occupy the PGF core background layer, preserving icon paint and keeping the native icon order. Opposite directions use separate bend-left curves. The K16 base uses 0.35 pt strokes, 0.9 mm Stealth tips and an 8-degree bend; the twenty selected directions use 0.8 pt strokes and 1.7 mm tips. The ledger keeps the existing sans-serif number font and uses only a mathematical arrow. Its writing rules end at 0.94 column units, giving 64.6 mm writing lines and a clear inner gutter. The original 32 mm oval radius, canonical sixteen icon choices, image scaling and measured full-width fit remain.

The worked-example graph accepts exactly six labels in this order: 1→2, 2→1, 2→3, 3→2, 1→3, 3→1. It draws six separate 16-degree curves with 0.8 pt strokes, 1.7 mm arrowheads and sloped 8.5 pt labels above the paths. Canvas-colour backing separates each label from nearby strokes. The graph uses a 32 mm vertical radius to accommodate the paired labels. The original native box, pyramid, elliptical cylinder, and four-row table are unchanged. The previous worked-example ledger and ledger-entry production macros were removed.

## Test-First Evidence

The retained language-neutral `directed-geometry.fixture.json` specifies the 240 directed arcs, twenty selected directions, six worked-example directions, minimum font size and stroke widths. The retained `directed-geometry-check.cjs` first failed because the directed-edge declaration was absent, then passed after implementation. It checks ordered-edge uniqueness/completeness, preserved selections, the six-argument graph API, removal of the old ledger API, named border-aware nodes, background layers, paired bends and 8.5 pt labels.

Independent production/PDF auditing and the German native render are coordinated by the parent and audit agent. Native dark verification will be appended after the page layout is settled.

## Native Bundle Constraint

The pinned native TeX bundle includes arrows.meta and PGF core layers but omits the optional TikZ backgrounds library. The implementation declares its own assignmentbackground layer through PGF core and orders it before main. It requires no dependency, bundle or cache changes.

## Final German Dark Native Regression

The official `deriveDarkPrintTexSource` helper produced the German dark driver. Registered Tectonic 0.16.9 compiled it successfully against the pinned local bundle with no box overflow or underflow. The three-page PDF is 133,862 bytes, has German document language and the expected title and author. The final PDF confirms that all 58 ledger digits use Anta while the twenty arrows use the mathematical arrow font, proving that the final font-preserving source change was consumed by the native compile.

The retained `check-directed-dark.py` passes metadata, three-page count, all-page character bounds, twenty ledger arrows, sans-serif ledger digits and six worked-example Form labels. The existing independent matrix verifier also passes both nine-by-nine exercise tables: writing rows are 6.14 mm, icon minimum padding is 2.53–2.61 mm, and maximum icon-centre displacement is below 0.031 mm.

All three pages were rendered with bundled PDFium/Pillow, and the contact sheet plus full-resolution worked-example and graph pages were visually inspected. The six K3 directed curves, above-path labels and reverse changes remain clear. The K16 base and twenty selected directions remain visible around the unchanged sixteen canonical icons. The final two-column ledger has twenty clear writing rules and a visible inner gutter. The independent directed-ledger verifier measures 64.60 mm rules and a minimum 4.08 mm gutter, with every ledger arrow matching its ordered fixture. Existing font/backend warnings remain; the 8.5 pt mathematical glyphs select nearby available TeX font sizes while the label text stays at 8.5 pt. No missing glyphs, clipped labels, overlapping writing rules, overfull or underfull boxes were observed.

Temporary compile, cache and rendered review files stay under the ticket generated directory for the parent's final cleanup. Final delivery remains parent-owned.
