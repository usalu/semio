# Three pairwise nonparallel Mesh supports

The shared Mesh clipping cursor now handles three pairwise nonparallel longitudinal supports that bound a region strictly inside a concave stock profile. It verifies each junction against the third halfspace and both incident stock intervals, then uses the same directed cycles as the earlier joined-support cases. The authored Intersect route handles the triangular cut with either moved or stationary transverse caps.

An enclosed Difference with two moved caps uses the removed cycle to emit a cavity with reversed winding. It retains the original outer stock rings, assigns the cavity walls and caps to the cutter faces, and checks the indexed links of both closed boundary shells before emission.

| Neutral fixture | Operation | Volume | Area | Boundary shells |
| --- | --- | ---: | ---: | ---: |
| Triangular cutter slab | Intersect, either order | 0.5 | 4.414213562373095 | 1 |
| Triangular cutter slab | Enclosed Difference | 13.5 | 50.88634951737268 | 2 |
| Triangular cutter, stationary stock caps | Intersect, either order | 1 | 6.82842712474619 | 1 |

Every boundary shell has genus zero. The native regression reproduced the prior support-plane refusal. All ten focused native tests and all 19 portable tests with 5,491 assertions passed. Manifold independently reproduced the exact area, volume, shell count, and genus. The linked authored Mesh test also checked original-stock interpolation, raw and named UVs, cutter materials, and the closed reimport with an affine vertex attribute.

The [stationary opening follow-up](📓️mesh-stationary-cap-openings.md) covers a triangular Difference that opens on one or both stationary stock caps. The new route certifies distinct interior junctions. Coincident junctions and contacts on a stock edge still require their own certification.
