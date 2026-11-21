# Mesh Union of Placed Stock

Partial Union already partitioned both convex operands. A cutter support could leave its current cell entirely inside or entirely outside, which previously caused a refusal. The shared branch now skips a wholly retained support and retires a wholly excluded cell without copying its unused polygon inventory. A support that cannot certify either side still refuses.

The two placed Box fixtures complete in both operand orders, including a skew positive overlap and a shared-axis overlap with distinct spans. Trim subdivisions can share the same rounded f32 line tolerance used by the Difference partitioner. Publication still checks every surviving authored plane, reciprocal incidence, connectedness, and finite positive volume.

| Case | Area | Volume |
| --- | ---: | ---: |
| Skew bump | 97.67981824350454 | 64.44499242681269 |
| Distinct spans | 98 | 64.5 |

All nine Rust Clipping laws passed. The 17-test Clipping and packaging suite passed with 4,705 assertions, including independent Manifold in both operand orders. The authored facade passed both cases with one-item material, UV and color projection. Flow passed 14 source-scoped retained tests. Generate3d passed five source-scoped tests, including both fixture orders at fuels 1, 7 and 1,024.

Repository cache collection removed an older compiler lane during the initial proof. The final authored, Flow, and Generate3d proofs reused a coherent pinned set of 141 compiled dependencies in the generated ticket directory. The helper retained only derived compiler artifacts and validated the selected dependency signatures before those runs.
