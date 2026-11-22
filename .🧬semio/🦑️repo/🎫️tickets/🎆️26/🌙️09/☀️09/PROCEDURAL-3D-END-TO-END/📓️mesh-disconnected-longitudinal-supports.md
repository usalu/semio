# Disconnected longitudinal Mesh Boolean cuts

The shared clipping cursor orders crossings along each longitudinal support and pairs the endpoints of the material intervals on that support. For each resulting interval it emits a cutter-owned wall with the operation's winding. Cap reconstruction connects outgoing and incoming boundary chains at the same endpoints, then walks every closed cycle. The cursor reuses each indexed crossing for the original prism sidewall, the cutter wall and the appropriate cap. This also applies where a cutter moves either transverse cap.

Four U-profile cases share a neutral fixture. Volumes and areas below are totals, followed by the number of closed components.

| Cut | Intersect volume / area / components | Difference volume / area / components |
| --- | --- | --- |
| Two arms | 2 / 12 / 2 | 5 / 22 / 1 |
| Base and partial arms | 5 / 22 / 1 | 2 / 12 / 2 |
| Two arms between transverse caps | 1 / 8 / 2 | 6 / 32 / 1 |
| Two independent supports | 3.5 / 18 / 1 | 3.5 / 22 / 3 |

The native regression reproduced the disconnected-profile refusal before the change. All ten native clipping tests then passed. They check reciprocal halfedges, winding, volume, area, Euler characteristic and the exact cutter owner of each created face. The linked authored Mesh façade test passed both Intersect operand orders and Difference for each fixture, including the number of faces owned by each cutter material, named and raw UVs and closed reimport. The portable Clipping suite passed 18 tests and 4,977 assertions. Manifold independently measured the four cases and verified their component counts and genus. The scoped whitespace check passed.

The change covers transverse and independent longitudinal supports for a certified translated simple prism. Oblique supports, coincident profile edges, tangent contacts and overlapping removed regions remain outside this construction.
