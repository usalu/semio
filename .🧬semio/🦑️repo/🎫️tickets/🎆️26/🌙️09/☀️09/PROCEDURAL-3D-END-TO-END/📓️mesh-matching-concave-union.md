# Mesh Union for matching concave axial prisms

Mesh Union now accepts two exactly matching orthogonal concave prism profiles. For overlapping intervals, the existing transverse Difference retains the left tail with stock edge samples at its new cap. Union removes that cap and the matching right cap, then welds the two ordered profile rings using the right operand's original vertex mapping. At exact face contact it welds the original caps. Containment returns the containing operand, and separated intervals retain both closed exterior shells. A welded result passes reciprocal incidence before publication.

| Neutral fixture | Union volume | Union area | Boundary shells |
| --- | ---: | ---: | ---: |
| Contained axial interval | 12 | 38 | 1 |
| Partial overlap on a rotated axis with opposite cutter cap order | 9 | 30 | 1 |
| Exact axial cap contact | 6 | 22 | 1 |
| Separated axial intervals | 6 | 28 | 2 |

The existing native Union refused the concave operands. The new integration exercises both operand orders and verifies the precise source face set of each domain. At contact, the weld retains the authored stock vertex identity. At overlap, each shared vertex is reconstructed from its original stock edge weights and retains the matching right vertex mapping. The actual authored façade preserves face materials, named and raw UVs, and different affine vertex channel values on the two sides of the join after closed reimport. Laterally offset concave profiles still return a typed refusal.

Nine focused native unit tests, both native integration tests, the linked authored Mesh test and all 20 portable tests with 5,746 assertions passed. Manifold independently matched exact area, volume, boundary shell count and genus in both operand orders.
