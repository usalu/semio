# Interior convex Mesh cutter support rings

The joined longitudinal Mesh route now handles a single closed ring of active supports within the existing six-wall bound. It solves each nonparallel pair before sampling stock provenance. A pair outside the active cutter is discarded only when another halfspace excludes both ends of its longitudinal edge. Each remaining junction must lie strictly inside the stock and the other cutter halfspaces. The active adjacency must form one connected ring with degree two at every support. The same directed cycles, cavity construction and stock cap bridges apply to the certified cutter shape.

Two masked support extensions can meet at the same point on a stock edge. The pentagonal through-opening reproduced separate IDs at this physically identical point, which produced zero-length edges and caused tessellation after reimport to overfill a cap. The cursor now shares the stock-edge pair across those coincident events and rejects zero-length boundary segments. It also requires a single excluding halfspace across an entire discarded segment, or checks that the whole retained segment belongs to the cutter side of every other plane. Tessellation uses the same normalized tolerance for ear orientation and triangle-edge containment. That prevents a roundoff-sized orientation at a collinear corner from accepting an ear whose chord passes through a vertex.

| Neutral fixture | Intersect volume | Intersect area | Difference volume | Difference area | Difference shell count / genus |
| --- | ---: | ---: | ---: | ---: | --- |
| Enclosed pentagonal slab | 0.75 | 5.618033988749895 | 13.25 | 52.09016994374947 | 2 / 0 each |
| Pentagonal through-opening | 1.5 | 8.23606797749979 | 12.5 | 48.70820393249937 | 1 / 1 |
| Hexagonal pocket | 0.875 | 6.207106781186548 | 13.125 | 49.179242736186126 | 1 / 0 |

The missing native route was reproduced before the change. All 10 focused native tests, the rebuilt authored Mesh test and all 19 portable tests with 5,605 assertions passed. Manifold independently verified the exact geometry and topology. The authored test checks every cutter face owner, named and raw UVs, the original stock owner of the two aperture polygons, weighted source positions and an affine vertex channel after closed reimport. The new fixtures permit one Float32 unit in the last place for raw UV interpolation at nonbinary stock-edge samples.

The proof covers a strictly interior support ring with one opening or cavity. The [remote-contact follow-up](📓️mesh-remote-stock-contacts.md) handles reflex tangency and stock-edge coincidence when another support strictly excludes the contact. Active triple ties, unstable support pairs, active stock-edge contacts and unclosed graphs still require a separate geometry certificate.
