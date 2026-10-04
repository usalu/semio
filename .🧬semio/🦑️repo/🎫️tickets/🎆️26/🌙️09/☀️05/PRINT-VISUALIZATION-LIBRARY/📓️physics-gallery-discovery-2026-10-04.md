# Physics Gallery Discovery

Root independently consumed the current source-covered physics30 light PDF with pypdf (eight pages), rasterized pages 1, 3, 4, 5 and 7 in both themes with Poppler at 110 dpi, and actually viewed all ten images. These outputs are the current discovery wave, not a completion gate.

Both themes visibly reproduce the following concrete defects:

- Page 1: force/free-body/vector decomposition diagrams remain around the lower-left origin; downward weight arrows pass beneath their frame and cross the next caption area. The initial motion square and t0 label also cross the lower-left boundary.
- Page 4: the potential-energy parabola rises outside its frame across its caption. The highest three energy-level labels overlap because their physical energy positions are too close for their text height.
- Page 5: the conduction band and its caption sit above the plotting canvas through the figure caption.
- Page 7: lens/mirror diagrams draw substantially outside the figure to the left of the page, losing object/ray geometry; the Poincare sphere is centered on the lower-left origin and extends below its frame.

Trajectory, orbital and spacetime diagrams on page 3 and field/wave examples on page 5 render visible data within their plotting rectangles; this report does not claim their numerical semantics or whole-document visual completion. Physics repair belongs in the existing native owner, with neutral coordinates/bounds/text-separation controls and independent D3/PDF.js oracles. No generic compatibility translation wrapper is authorized by this audit. Exact actual-view records: [{"theme":"light","owner":30,"renderExit":0,"pdf":"C:\\git\\semio\\🧰️framework\\🛍️products\\📓️print\\📦️packages\\🟦️typescript\\dist\\documents\\viz-30\\⚛️viz-30.pdf","viewed":true,"pages":[1,3,4,5,7],"sha256":"9d11c8a2910f2f4b66ee454c21b7878c7ddbc0f8902ddb5d3a8db4d413ae3494"},{"theme":"dark","owner":30,"renderExit":0,"pdf":"C:\\git\\semio\\🧰️framework\\🛍️products\\📓️print\\📦️packages\\🟦️typescript\\dist\\documents\\viz-30\\⚛️viz-30-dark.pdf","viewed":true,"pages":[1,3,4,5,7],"sha256":"6efd466ff47bca97b5c31ca391704191cffae408950e1e3a1ab45d244a911e1c"}].


## Root Implementation Review While Repair Is Live

Read-only review of the current existing physics owner observed viewport fitting around force/motion/optics/sphere geometry while raw physical numerical reporting remains separate. The proposed label helper places PGF interrupt-picture within a measurement hbox. Root requested reuse of the already runtime-proven guide/hierarchy pattern that interrupts the picture before creating a private font-selected hbox; earlier hierarchy diagnostics exposed nullfont failures with alternative measurement scope. This is an implementation review concern to resolve in actual paired physics GREEN, not a new claimed runtime failure. Authored ranges remain authoritative; absent ranges and label placement are being corrected by the same owner.


## Root Shared Caption Parsing Review

Read-only current physics source inspection verified every common record parser caller receives a literal clist item `{##1}`. The levels-only repair had ten duplicated literal seq splits, while force/motion and other family captions still used the expanding common parser. Root requested neutral protected math force/motion coverage and consolidation through the existing common helper. Actual force-caption RED16814 failed exit1 in33.2s at cases.tex12 with undefined control sequence, confirming the shared authoring defect. Current candidate245c16ff uses one token-preserving Nnn helper and its former level duplicates call that helper. Unrelated drawing bodies remain unchanged. Protected energy gamma endpoint check58316 is still pending and its first TeX pass showed no error; no endpoint failure or speculative endpoint edit is claimed. Paired full physics66417/stock30-88613 are live before source release.
