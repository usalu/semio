# Moved transverse caps on a joined reflex intersection

Mesh Intersect now clips a joined-reflex prism at one or both transverse cutter caps. The joined boundary cursor works on the mapped cap vertices. Edge crossings retain weighted stock-face provenance when a cap moves, and separate reflex corners inherit stock-edge samples at the moved layer. The generated cap faces carry their cutter material and outward winding in either operand order.

| Neutral fixture | Moved caps | Volume | Area | Connected components |
| --- | ---: | ---: | ---: | ---: |
| Perpendicular joined slab | 2 | 2 | 12 | 1 |
| Rotated joined tail | 1 | 1 | 10.06449510224598 | 2 |

The native regression reproduced the previous support-contact refusal. After the change, all ten focused native clipping tests and the linked authored Mesh integration test passed. Both operand orders preserved cutter ownership and winding, cyclic seam vertex links, named and raw corner UVs, and a closed reimport. The portable suite passed 18 tests with 5,328 assertions, including independent Manifold matches for all three measures in both operand orders. Both rotated branches have genus zero.

The [retained-tail follow-up](📓️mesh-joined-reflex-retained-tails.md) covers Difference with one or two moved caps.
