# Retained Placed Box Union and Pocket

Positive-volume Union of two retained Box operands now falls back from the certified common-frame footprint path to the original planar boundary arrangement. The arrangement clips the authored faces of both operands, suppresses buried cells, compacts collinear candidate vertices, and certifies reciprocal edge uses before staging a Solid. The native Flow fixture runs both operand orders and checks the resulting geometry and shaded-preview metrics. Disjoint placements continue to produce exact compound members.

The same arrangement now permits five active cutter supports for Cut. A rotated elevated tool that ends inside the stock opens an exact blind pocket. The compacted shell retains the five inward tool supports and publishes the live stock supports and edge joins. The existing bounded six-support refusal remains covered.

| Case | Vertices | Edges | Faces | Surface area | Volume |
| --- | ---: | ---: | ---: | ---: | ---: |
| Skew positive-overlap Union | 24 | 38 | 16 | 97.67981824350454 | 64.44499242681269 |
| Shared-axis distinct-span Union | 20 | 32 | 14 | 98 | 64.5 |
| Rotated distinct-span Cut | 20 | 32 | 14 | 98 | 63.5 |

The compact native source suite passed all 44 tests. The independently evaluated Manifold and Three fixture suite passed 19 tests with 226 assertions. The source-closure audit kept the affected owner packages within their copy bounds and the scoped whitespace check passed. The registered Flow test remains in the managed Rust integration run.
