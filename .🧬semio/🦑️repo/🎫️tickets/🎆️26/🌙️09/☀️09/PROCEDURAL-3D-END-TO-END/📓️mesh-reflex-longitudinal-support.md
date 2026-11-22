# Reflex contact in longitudinal Mesh Booleans

The shared clipping cursor now splits the cutter wall at each ordered reflex contact. It gives the two adjacent boundary chains separate indexed vertex pairs at the same world coordinates. Each pair follows its own source sidewall and cap cycle. When the retained side excludes both original edges at the contact, the same support produces an uninterrupted cutter wall.

The neutral fixtures exercise one reflex contact in both Intersect operand orders, Difference for either half-space, two reflex contacts on the same support, and an intersection whose transverse cutter caps move both ends. The authored façade preserves exact cutter material owners and both named and raw UVs. The round trip checks the number of vertex-connected components and requires opposing edge uses throughout each closed component.

| Case | Intersect volume / area / components | Difference volume / area / components |
| --- | --- | --- |
| One reflex contact, retained arms | 6 / 26.47213595499958 / 2 | 8 / 28 / 1 |
| One reflex contact, retained base | 8 / 28 / 1 | 6 / 26.47213595499958 / 2 |
| One reflex contact with moved caps | 3 / 19.23606797749979 / 2 | — |
| Two reflex contacts, retained arms | 8 / 36.94427190999916 / 3 | 12 / 40 / 1 |

The native regression reproduced the previous reflex refusal. All 10 native clipping tests passed after the change, the linked authored test passed, and the 18-test portable suite passed with 5,076 assertions. Manifold independently verified the volume and area for every operation and the expected component counts for the full-height cuts. For the moved caps, Manifold returns two components when the convex cutter is first and one when the prism is first. The exact native and authored checks require two components in both orders. The scoped whitespace check passed.

The retained construction supports a single longitudinal support with distinct reflex contacts. The follow-up covers Difference with transverse stock tails. A contact involving multiple longitudinal supports requires additional seam construction. [Mixed Difference verification](./📓️mesh-reflex-difference-tails.md)
