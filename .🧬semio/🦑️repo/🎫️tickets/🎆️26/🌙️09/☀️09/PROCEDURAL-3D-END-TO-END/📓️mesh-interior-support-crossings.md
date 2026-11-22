# Interior crossings of two longitudinal Mesh supports

The Mesh clipping cursor can now assemble Intersect and Difference when two nonparallel longitudinal cutter planes meet strictly inside a concave stock profile and both transverse caps move. The cursor computes their junction in the stock profile, checks positive clearance from its edges, and requires the junction to fall strictly inside the stock interval paired on each support. The directed boundary assembler splits both support intervals at the same indexed junction. Difference uses those same junctions and edge splits to attach both stock tails.

The new vertices carry nonnegative interpolation weights from at most four original stock vertices. The cursor reconstructs and checks each weighted position before it gives either vertex to the boundary assembler. The native test independently checks each weight sum and reconstructed position in both Intersect operand orders and in Difference.

| Neutral fixture | Intersect volume | Intersect area | Difference volume | Difference area |
| --- | ---: | ---: | ---: | ---: |
| Axis-aligned rectangle | 1 | 7 | 13 | 50.47213595499958 |
| Rotated triangular wedge | 0.5 | 4.414213562373095 | 13.5 | 48.88634951737268 |

Each result is one closed genus-zero shell. The old route rejected the axis-aligned fixture because a stock crossing on one support was outside the second support. After the change, all 10 focused native tests, the linked authored Mesh integration test, and all 18 portable tests with 5,388 assertions passed. Manifold independently verifies area, volume, components and genus at the exact fixture coordinates. The authored checks cover both Intersect orders, Difference, exact cutter ownership and winding, named and raw UVs, and closed reimport.

The [stationary-cap follow-up](📓️mesh-stationary-cap-sampling.md) covers generated interior corners that reach one or two stationary concave caps and validates their authored face samples. Boundary junctions at an existing reflex edge continue to use the prior joined route, and independent supports continue to use their existing route. The [three-support follow-up](📓️mesh-three-support-crossings.md) covers two separated interior junctions between a third support and an opposing pair.
