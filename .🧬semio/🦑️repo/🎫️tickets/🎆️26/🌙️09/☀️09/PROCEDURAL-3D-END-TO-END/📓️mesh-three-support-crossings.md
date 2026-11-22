# Three longitudinal supports with separated interior crossings

The shared Mesh clipping cursor now handles an opposing pair of longitudinal cutter supports joined by a third support at two separate points strictly inside the stock profile. It collects the pairwise junctions before publishing them, checks that each lies strictly inside the remaining cutter halfspace, and inserts each at its ordered position in both incident stock intervals. It selects the resulting segments against all three halfspaces. The same directed cycles assemble the intersection, the retained complement, and the Difference tail seams.

The neutral fixtures exercise the same three walls with either both transverse caps moved or both stock caps stationary. The native checks require both generated junctions at each layer, stock-source weights that reconstruct them, opposing edge incidence and cyclic vertex links. The authored checks verify raw and named corner UVs, cutter material ownership, an affine vertex channel after reimport, and one closed genus-zero shell.

| Neutral fixture | Intersect volume | Intersect area | Difference volume | Difference area |
| --- | ---: | ---: | ---: | ---: |
| Three supports with a shorter cutter slab | 1 | 7 | 13 | 51.47213595499958 |
| Three supports with both stock caps stationary | 2 | 10 | 12 | 44.47213595499958 |

The native regression reproduced the old support-plane refusal before the change. All 10 focused native tests, the linked authored Mesh test, and all 19 portable tests with 5,461 assertions passed. Manifold independently matched exact area, volume, components, and genus for both Intersect operand orders and Difference in each fixture.

The new topology family requires two separated interior junctions and one parallel pair. The [three-nonparallel-support follow-up](📓️mesh-three-nonparallel-supports.md) covers a strictly interior triangular Intersect and enclosed Difference. Interior junctions touching a stock edge and coincident junctions still require broader certification. Existing independent supports and the two-support reflex route retain their prior handling.
