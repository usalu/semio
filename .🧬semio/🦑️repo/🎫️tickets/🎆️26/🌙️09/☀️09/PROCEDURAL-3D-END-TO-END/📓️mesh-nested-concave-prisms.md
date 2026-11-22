# Mesh booleans for strictly nested concave prisms

Mesh Intersect, Difference, and Union now accept two orthogonal concave prisms with distinct transverse profiles when one is strictly inside the other. The cursor certifies axial clearance, then visits one projected pair of stock and cutter edges per step. It requires disjoint boundaries with positive clearance and verifies that the inner profile lies within the stock. Intersect retains the original inner operand, Union retains the original outer operand, and outer-minus-inner uses the existing closed cavity publisher with reversed cutter faces. Inner-minus-outer returns empty.

| Neutral fixture | Intersect volume | Intersect area | Difference volume | Difference area | Union volume | Union area |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Scaled concave profile | 1.5 | 9.5 | 10.5 | 47.5 | 12 | 38 |
| Seven-corner notch on a rotated axis | 0.6875 | 5.582106781186548 | 8.3125 | 35.582106781186546 | 9 | 30 |

Each outer-minus-inner result has an outward stock shell and an inward cavity shell. Both shells have genus zero. The native test also checks cancellation while the profile certificate is in progress. A separate fixture has every cutter vertex inside the stock but an edge across the stock notch. Manifold independently measures a true intersection volume of 2.875 against the cutter's 3.125. Mesh reports the typed containment refusal for this fixture.

The pre-change native test reported a matching-profile refusal. After the change, nine focused native units, both registered native tests, the rebuilt authored Mesh test, and all 20 portable tests with 5,812 assertions passed. Manifold corroborates exact areas, volumes and shell topology. The authored test checks cap and wall materials, raw and named UVs, and distinct per-operand vertex attributes through closed reimport.

This route requires strict axial and transverse clearance. The [separation follow-up](📓️mesh-separated-concave-prisms.md) covers distinct disjoint profiles and axially separated operands. Crossed profile boundaries within an overlapping axial interval and cutters that open at a stock cap still require their own construction.
