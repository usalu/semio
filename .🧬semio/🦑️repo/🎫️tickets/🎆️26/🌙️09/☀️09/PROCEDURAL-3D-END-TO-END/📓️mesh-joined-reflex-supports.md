# Joined longitudinal supports at a reflex edge

The Mesh clipping cursor now handles a full-height prism intersection or difference where two nonparallel longitudinal cutter planes share one reflex edge of the stock. It constructs the supported boundary from the exposed stock segments and cutter wall intervals. At a pinched apex, it follows the directed boundary cycles and gives the cycles separate indexed vertices with the original source provenance.

The neutral law exercises mirrored perpendicular plane pairs and a rotated plane pair whose supports are both tangent at the reflex edge. The rotated case also has ordinary stock vertices on the supporting planes.

| Fixture | Intersect volume | Intersect area | Intersect components | Difference volume | Difference area | Difference components |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Right wedge | 4 | 16 | 1 | 10 | 38.47213595499958 | 2 |
| Left wedge | 4 | 16 | 1 | 10 | 38.47213595499958 | 2 |
| Diagonal wedge | 2 | 16.12899020449196 | 2 | 12 | 41.65685424949238 | 1 |

The native regression reproduced the rejection that a reflex contact meets another support before the implementation. The completed runs passed all 10 focused native tests, the linked authored integration test, and all 18 portable tests with 5,306 assertions. The production clipping tests verify exact areas and volumes, both Intersect operand orders, separate seam indices, cyclic vertex links, and cutter face ownership and winding. The linked public authored Mesh façade verifies raw and named UVs and cutter materials, then reimports the closed output and checks component count and Euler characteristic. Manifold independently matches area, volume and components for every fixture and both Intersect orders. Every component has genus zero.

This route requires one common existing reflex edge. The [transverse follow-up](📓️mesh-joined-reflex-transverse-caps.md) covers moving one or both caps for Intersect. The [retained-tail follow-up](📓️mesh-joined-reflex-retained-tails.md) covers moving one or both caps for Difference. The [interior-crossing follow-up](📓️mesh-interior-support-crossings.md) covers a junction strictly inside the profile when both transverse caps move. Coincident stock edges and additional reflex contacts on an individual support still require a broader arrangement.
