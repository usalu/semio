# Mixed Mesh Difference at reflex contacts

The clipping cursor now joins a longitudinal reflex cut to one or both transverse stock tails. It traces the branches of a transverse opening separately and publishes them at their common seam vertex. When Difference retains the pinched arms, each moved cap joins the arms to its stock tail. With two shared end vertices, the two retained longitudinal edges receive separate indexed interior samples at the same world position. This keeps both edge incidences distinct and preserves the handle through the two stock tails.

| Difference fixture | Volume | Area | Connected components | Genus |
| --- | ---: | ---: | ---: | ---: |
| Removed pinched arms, two tails | 11 | 55.23606797749979 | 1 | 0 |
| Removed pinched arms, one tail | 11 | 43.23606797749979 | 1 | 0 |
| Retained pinched arms, two tails | 10 | 60.47213595499958 | 1 | 1 |
| Retained pinched arms, one tail | 10 | 44.47213595499958 | 1 | 0 |

The two native red cases reproduced the previous refusal for opposite sides of the reflex support. All 10 native clipping tests passed after implementation. They check reciprocal edge incidence, a single cyclic link at each seam vertex, the number of indexed vertices along the pinch, the Euler characteristic and exact cutter face ownership. The linked authored test passed both Intersect operand orders and Difference, including named and raw UVs, material ownership, closed reimport, connected component count and Euler characteristic.

The portable Clipping and packaging suite passed 18 tests and 5,128 assertions. Manifold confirmed the exact area, volume and component count of each Difference fixture. At exact coincidence in the retained-arm two-tail case, Manifold reports genus zero. A cutter offset of 0.0001 separates the arms and yields one genus-one component with converging volume and area. The native and authored checks verify genus one at the exact plane with distinct branch incidence. The scoped whitespace check passed.

The follow-up composes transverse caps and independent longitudinal reflex supports. [Independent-support verification](./📓️mesh-reflex-independent-supports.md)
