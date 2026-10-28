# K16 Geometry With Twenty Selected Transitions

The shared geometry enumerates all 120 unordered pairs of sixteen vertices once, including the 64 pairs between Tables A and B. The complete K16 is drawn faintly. A separate shared enumeration highlights twenty transitions: the sixteen edges of the outer ring, followed by four repeatable random chords through the middle: (1,10), (2,12), (4,10), and (5,13). Their cyclic distances are seven, six, six, and eight. The same twenty-edge callback supplies the writing ledger, so no unselected pair requires an answer.

The authored selection uses Fisher-Yates with an unsigned 32-bit LCG, seed 20261027, multiplier 1664525 and increment 1013904223. Candidate pairs use ascending endpoints and cyclic distance six through eight. The first four shuffled candidates are sorted by endpoints for stable print order. This script retains the exact generation algorithm.

Vertices preserve canonical Table A then Table B order, the original image scale of 0.8, the original 32 mm vertical radius, and the original numbered-node styling. Measuring the natural width and adjusting only the horizontal radius fits the artwork and numeric labels to the full available text width. There is no vertical scaling.

The selected-edge ledger has two columns of ten rows, a 6.3 mm row pitch, and 68 mm writing lines at 170 mm text width. Its fixed 63 mm footprint fits below the graph on page 3. Both compact exercise tables are unchanged.

Validation: source contracts independently reproduce the random chord selection and compare the K16 edge set against D3. The root agent owns final native PDF verification and delivery. Shared-footer Unicode filename regression diagnosis is retained separately.

Actual PDF auditing found that unpainted horizontal label padding and macro-edge spaces reduced the visible graph to 165.18 mm despite its 170 mm TeX box. The corrected geometry removes only horizontal label padding and trims outer spaces in both width measurements. Vertical label padding, font, image scale and the 32 mm vertical radius remain unchanged. Auditing also found that a writing rule extended under the next column label; the two-column ledger now reserves an inner label area and a visible gutter while retaining 68 mm writing rules.

Final dark native verification passed: 170.001 × 84.242 mm outer graph bounds, 32 mm vertical radius, all 120 base edges, all 20 highlighted pairs and twenty matching clear ledger slots. The three-page PDF has no overfull or underfull boxes; both compact page-2 matrices pass independent geometry checks. See k16-dark.md for the complete regression and visual review.
