# Reflex contacts with independent longitudinal supports

The shared Mesh clipping cursor now retains reflex contacts from multiple independent longitudinal supports. Each contact must lie strictly inside the other supporting planes. As the cursor assembles caps and sidewalls, it selects contacts from the active supports and still inserts other support crossings along incident source edges. This preserves separate pinched branches, the identity of each cutter face, and the closed seams at transverse stock tails.

The neutral law covers opposing supports, nonparallel supports, simultaneous reflex contacts on two separate supports, and transverse tails. Measures are volume, area and connected components in that order.

| Fixture | Intersect | Difference | Difference genus |
| --- | --- | --- | ---: |
| Opposing supports, base band | 6 / 23 / 1 | 8 / 39.47213595499958 / 3 | 0 |
| Opposing supports, tip band | 4.875 / 22.604101966249686 / 2 | 9.125 / 36.86803398874989 / 3 | 0 |
| Opposing supports, base tails | 3 / 17.5 / 1 | 11 / 60.97213595499958 / 1 | 2 |
| Opposing supports, tip tails | 2.4375 / 16.177050983124843 / 2 | 11.5625 / 56.295084971874736 / 1 | 2 |
| Nonparallel supports | 8.5 / 30 / 1 | 6.5 / 30.47213595499958 / 3 | 0 |
| Reflex contacts on both supports | 8 / 28 / 1 | 12 / 52.94427190999916 / 4 | 0 |
| Reflex contacts on both supports, with tails | 4 / 22 / 1 | 16 / 82.94427190999916 / 1 | 3 |

The native regression reproduced the previous single-support refusal before the change. All 10 native clipping tests then passed, including reciprocal edge use, cyclic seam links, exact cutter owners, separate contact indices and Euler characteristic. The linked authored façade passed both Intersect orders and Difference for every fixture, preserving raw and named UVs and exact cutter material ownership through a closed reimport. The complete portable clipping and packaging suite passed 18 tests and 5,252 assertions.

Manifold independently confirmed the exact volume and area of the new cases. With moved caps on the tip band, it reports the two pinched intersection components when the cutter is first. The native and authored checks require both operand orders to return two. At coincidence it reports genus one on the enclosed base-tail case and the case where both supports have reflex contacts. Separating the pinched branches by moving the relevant supports by 0.0001 yields the same limiting geometry with genus two and genus three respectively, matching the exact indexed native topology.

The [joined-support follow-up](📓️mesh-joined-reflex-supports.md) covers two nonparallel supports sharing one reflex edge at stock cap positions. General intersecting removal contours still require a broader arrangement.
