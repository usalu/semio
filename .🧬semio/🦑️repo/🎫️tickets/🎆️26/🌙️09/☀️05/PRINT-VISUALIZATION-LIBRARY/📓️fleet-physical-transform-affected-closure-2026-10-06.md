# Scientific Physical Transform Affected Closure — 2026-10-06

Exact lexical callsite inventory below uses nearest declared SemioVizFamily; helper routines may be invoked by additional branches, so family label is source-context ownership, not closed runtime dispatch census.

| Source | Family context | Operation |
|---|---|---|
| semio-viz-scientific-3d.sty:281 | sci-3d | physical viewport |
| semio-viz-scientific-biology.sty:684 | sci-pathway | physical viewport |
| semio-viz-scientific-biology.sty:782 | sci-pedigree | physical viewport |
| semio-viz-scientific-biology.sty:895 | sci-punnett | physical viewport |
| semio-viz-scientific-biology.sty:967 | sci-lifecycle | physical viewport |
| semio-viz-scientific-biology.sty:1037 | sci-circos | physical viewport |
| semio-viz-scientific-biology.sty:1200 | sci-ideogram | physical viewport |
| semio-viz-scientific-biology.sty:1577 | sci-seqlogo | physical viewport |
| semio-viz-scientific-biology.sty:1713 | sci-contact-map | physical viewport |
| semio-viz-scientific-biology.sty:1931 | sci-nomogram | physical viewport |
| semio-viz-scientific-biology.sty:2036 | sci-schematic | physical viewport |
| semio-viz-scientific-chemistry.sty:68 | sci-molecule | physical viewport |
| semio-viz-scientific-chemistry.sty:88 | sci-molecule | nested structural fit |
| semio-viz-scientific-chemistry.sty:353 | sci-orbital | physical viewport |
| semio-viz-scientific-chemistry.sty:354 | sci-orbital | physical viewport |
| semio-viz-scientific-chemistry.sty:477 | sci-phase-diagram | physical viewport |
| semio-viz-scientific-chemistry.sty:575 | sci-electrochem | physical viewport |
| semio-viz-scientific-chemistry.sty:802 | sci-crystal | physical viewport |
| semio-viz-scientific-engineering.sty:107 | sci-circuit | physical viewport |
| semio-viz-scientific-engineering.sty:599 | sci-smith | physical viewport |
| semio-viz-scientific-engineering.sty:1031 | sci-truss | physical viewport |
| semio-viz-scientific-engineering.sty:1138 | sci-dimensioned | physical viewport |
| semio-viz-scientific-field.sty:798 | helper | nested structural fit |
| semio-viz-scientific-field.sty:887 | helper | physical viewport |
| semio-viz-scientific-geometry.sty:128 | sci-construction | physical viewport |
| semio-viz-scientific-geometry.sty:429 | sci-transform | physical viewport |
| semio-viz-scientific-geometry.sty:489 | sci-tiling | physical viewport |
| semio-viz-scientific-geometry.sty:718 | sci-fractal | physical viewport |
| semio-viz-scientific-geometry.sty:866 | sci-mesh | physical viewport |
| semio-viz-scientific-geometry.sty:1183 | sci-bracket | physical viewport |
| semio-viz-scientific-geometry.sty:1262 | sci-sports-series | physical viewport |
| semio-viz-scientific-mathematics.sty:1183 | sci-abstract-diagram | physical viewport |
| semio-viz-scientific-mathematics.sty:1225 | sci-grid-diagram | physical viewport |
| semio-viz-scientific-mathematics.sty:1314 | sci-braid | physical viewport |
| semio-viz-scientific-mathematics.sty:1464 | sci-set | physical viewport |
| semio-viz-scientific-mathematics.sty:1605 | sci-upset | physical viewport |
| semio-viz-scientific-mathematics.sty:1820 | sci-optimization | physical viewport |
| semio-viz-scientific-mathematics.sty:1829 | sci-optimization | physical viewport |
| semio-viz-scientific-mathematics.sty:1985 | sci-probability-tree | physical viewport |
| semio-viz-scientific-mathematics.sty:2159 | sci-simplex | physical viewport |
| semio-viz-scientific-mathematics.sty:2231 | sci-transition | physical viewport |
| semio-viz-scientific-physics.sty:96 | sci-force | physical viewport |
| semio-viz-scientific-physics.sty:537 | sci-feynman | physical viewport |
| semio-viz-scientific-physics.sty:550 | sci-feynman | nested structural fit |
| semio-viz-scientific-physics.sty:1244 | sci-optics | physical viewport |
| semio-viz-scientific-physics.sty:1341 | sci-poincare | physical viewport |
| semio-viz-scientific-signal.sty:472 | sci-staff | physical viewport |
| semio-viz-scientific-signal.sty:714 | sci-tonnetz | physical viewport |

Changing physical_begin/end affects many scientific owners, not only diagrams. Matrix maps x=W/(xmax-xmin), y=H/(ymax-ymin) with mx(0)/my(0) translation; reversed domains/ranges must preserve signed factors. Cropped explicit windows transform geometry outsidecanvas rather than automaticallyclip; keep that contract while reserving actualvisibleink. Outerchrome draws before transform and must remain unchanged. Nested structuralfit must retain order (localfit then viewport), labels/arrowheads/strokes and lowlevel system transform scaling need independentcomparison.

Alternatives: tracked normalcoordinate transform preservesPGF bbox but may change text/stroke scaling; retainlowlevelpaint transform plus explicitlytrackedactualbbox can preservephysicalink but must union labels/strokes and negativeclosureextensions, not canvas-onlyrectangle. Any systemicchoice requires exactPDFpoint/matrix regressions from mixed84+Construction140, branchsciencecontrols and trueomission Surface24/Physics24; structural Feynman/molecular/circuit, reversed/cropped window, nestedfit, autoheightnegativelegend and explicitheight control must remain. Carrieractualpanel+D3extent measures layout; probes alone cannot certifyreservation.

