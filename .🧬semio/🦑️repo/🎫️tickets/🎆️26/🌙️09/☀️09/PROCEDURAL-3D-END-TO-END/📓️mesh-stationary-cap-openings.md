# Stationary cap openings in Mesh Difference

A strictly interior triangular cut now opens through one or both stationary stock caps. The clipping cursor takes two disjoint extensions of a cutter support from the opening to the paired stock interval endpoints. Each bridge lies within the original stock profile and outside the cut. They split the cap into two simple rings that reuse the stock and opening vertex indices. The cursor verifies each ring's orientation and the cyclic links of the complete output before it releases any polygons.

The two cap polygons keep the original stock cap material and authored chart. Stock edge samples and the interior weighted samples survive the named and raw UV transfer and the closed reimport. Reversed cutter faces still own the triangular wall and the recessed lid where a cap moves.

| Neutral Difference fixture | Volume | Surface area | Boundary shells | Genus |
| --- | ---: | ---: | ---: | ---: |
| Blind triangular opening | 13.5 | 48.88634951737267 | 1 | 0 |
| Triangular through-opening | 13 | 49.30056307974577 | 1 | 1 |

The old clipping cursor refused the clockwise interior ring. It now distinguishes this opening from an ordinary cap and an enclosed cavity. All ten focused native tests and the linked authored Mesh test passed, including exact ownership of both pieces of each stationary cap and an affine vertex channel after reimport. All 19 portable tests passed with 5,513 assertions. Independent Manifold results matched the exact volume, area, shell count and genus for both fixtures.

This assembly uses one strictly interior opening with two certified support extensions. The [four-support follow-up](📓️mesh-four-support-interior-cuts.md) applies the same assembly to interior rectangular cuts. Multiple openings, contact with the stock profile, and junctions that cannot produce two such bridges still require a broader cap construction.
