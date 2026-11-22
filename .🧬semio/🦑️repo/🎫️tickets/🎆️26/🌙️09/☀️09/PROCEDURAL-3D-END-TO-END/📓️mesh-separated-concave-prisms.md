# Separated concave prism Boolean operations

The retained axial certificate now distinguishes strict containment from separation. When the axial intervals have positive clearance, it settles separation before visiting projected edges. Otherwise it checks both transverse boundaries, requiring positive clearance for each edge pair and checking polygon parity in both directions. This handles distinct concave footprints with overlapping axis-aligned bounding boxes and axial spans.

Intersect returns empty. Difference returns the original left operand. Union joins the two authored domains as two closed, independent exterior shells. The verified Union preserves each original wall and cap owner.

| Neutral fixture | Union volume | Union area | Boundary shells |
| --- | ---: | ---: | ---: |
| Distinct footprints separated inside the stock's reentrant corner | 9.5625 | 36.375 | 2 |
| Axially separated prisms with crossing footprints | 10.5625 | 38.629006963152456 | 2 |

The previous native route refused the first fixture before checking either projected boundary. Both new fixtures exercise both operand orders for all three operations. The rebuilt authored Mesh test checks empty Intersect, retained Difference, exact Union material ownership, named and raw UVs, and per-operand vertex attributes after closed reimport. Manifold independently verifies both Union shells, their areas and volumes, and the empty Intersect and retained Difference.

Nine focused native unit tests, both registered integration tests, the linked authored test, and 20 portable tests with 5,868 assertions passed. Profile boundary contact with overlapping axial spans still needs a separate certificate.
