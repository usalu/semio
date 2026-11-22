# Four-support interior rectangular Mesh cuts

The longitudinal clipping cursor now handles a rectangular cutter inside a concave stock profile. It admits the two opposing pairs as a closed support ring with exactly two distinct junctions on each support. Each junction has to lie strictly inside the stock, strictly inside the two other halfspaces, and inside the paired stock intervals on both incident supports. The cursor uses the same retained and removed cycles for both Intersect operand orders, enclosed Difference cavities, and Difference with openings in stationary caps.

| Neutral fixture | Intersect volume | Intersect area | Difference volume | Difference area | Difference shell count / genus |
| --- | ---: | ---: | ---: | ---: | --- |
| Rectangular cutter slab | 1 | 7 | 13 | 53.47213595499958 | 2 / 0 each |
| Rectangular cutter through stationary caps | 2 | 10 | 12 | 48.47213595499958 | 1 / 1 |
| Rotated rectangular pocket with the upper cap stationary | 0.25 | 2.414213562373095 | 13.75 | 47.88634951737267 | 1 / 0 |

Before the change, the native clipping test reproduced a support-plane refusal. All 10 focused native tests, the rebuilt linked authored Mesh test, and all 19 portable tests with 5,559 assertions passed. Manifold independently matched volume, area, shell count and genus. Native and authored checks verify weighted stock vertex provenance, cyclic links, exact cutter ownership, the two stock-owned polygons at each opening, named and raw UVs, and an affine vertex channel after a closed reimport.

The route certifies two opposing pairs with four distinct interior junctions. The [convex support ring follow-up](📓️mesh-convex-support-rings.md) certifies active support adjacency without requiring opposing pairs and adds pentagonal and hexagonal fixtures. It retains the existing refusal for uncertain geometry, including crossed supports at the stock profile and additional simultaneous junctions.
