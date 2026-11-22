# Retained tails on joined reflex Difference

Mesh Difference now joins the stock tails to a cut where two nonparallel longitudinal cutter planes share one existing reflex edge and one or both transverse caps move. The cursor builds the retained and removed profile rings from the same split stock segments and cutter wall intervals. The removed rings become the exposed transverse cutter faces. The stock tail walls reuse the same seam vertices so that the retained walls, exposed cutter faces, and uncut stock meet along matching edges.

At a moved cap, both retained branches can share the correct seam vertex. If both caps move, the coincident branches have separate middle edge indices between the seams. If one cap stays at the stock boundary, separate indices at that end preserve the distinct branches instead. The cursor requires a single closed cyclic link at each indexed vertex before it publishes the result.

| Neutral Difference fixture | Retained tails | Volume | Area | Components | Genus |
| --- | ---: | ---: | ---: | ---: | ---: |
| Right perpendicular slab | 2 | 12 | 54.47213595499958 | 1 | 1 |
| Left perpendicular wedge | 1 | 12 | 46.47213595499958 | 1 | 0 |
| Rotated wedge | 1 | 13 | 46.06449510224598 | 1 | 0 |

The native regression reproduced the prior support-contact refusal. All ten focused native tests then passed, including reciprocal incidence, cyclic vertex links, cap winding, cutter ownership, and Euler characteristic. The linked authored Mesh test checked the named and raw corner UVs, cutter materials, closed reimport, and component counts. The portable suite passed 18 tests with 5,357 assertions. Manifold independently reproduced area, volume and components at the exact fixture coordinates. For the two-tail slab, Manifold merges the coincident branches and reports genus zero. Shifting both longitudinal cutter faces outward by 0.0001 separates those branches and independently reproduces genus one, with area and volume converging to the exact values.

The joined route still requires one shared existing reflex edge, two longitudinal supports and simple boundary rings. Overlapping contact events, unstable or coincident stock edges, and output with multiple vertex-link fans follow the existing typed refusal.
