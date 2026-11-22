# Axial Mesh booleans for matching concave prisms

Mesh Intersect and Difference now accept two orthogonally extruded concave prisms with exactly the same transverse profile. After the existing prism certificates pass, the cursor compares profile vertices under cyclic indexing, accounts for opposite cap order, and checks that both cap layers preserve the profile and have uniform axial levels. The clipping step uses the two real transverse cutter faces with their original indices. Interpolation samples stay on the stock's axial edges and land at the cutter's exact authored cap coordinates.

| Neutral fixture | Intersect volume | Intersect area | Difference volume | Difference area | Difference shells |
| --- | ---: | ---: | ---: | ---: | ---: |
| Contained axial interval | 6 | 22 | 6 | 28 | 2 |
| Partial overlap on a rotated axis with opposite cutter cap order | 3 | 14 | 3 | 14 | 1 |
| Axial cap contact | 0 | 0 | 3 | 14 | 1 |

The old native cursor refused both concave operands before evaluating their profiles. The positive native checks require opposing edge incidence, correct cutter cap winding and ownership, and the expected Euler characteristic. The actual authored façade checks both Intersect orders and Difference for named and raw UVs, cutter cap materials, empty intersection at face contact, and a closed reimport. It also checks the typed refusal for a laterally offset profile. Manifold independently verifies volume, area and genus-zero component counts.

All nine focused native unit tests, both native integration tests, the linked authored test, and all 20 portable tests with 5,691 assertions passed. This proof covers exact matching orthogonal profiles. The [Union follow-up](📓️mesh-matching-concave-union.md) covers overlapping, touching, contained and separated axial intervals with the same profile. The [nested-prism follow-up](📓️mesh-nested-concave-prisms.md) covers distinct concave profiles with strict axial and transverse containment.
