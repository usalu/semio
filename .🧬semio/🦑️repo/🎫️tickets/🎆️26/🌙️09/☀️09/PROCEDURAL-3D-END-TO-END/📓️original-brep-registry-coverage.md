# Original BRep Registry Coverage

I compared the BRep operator IDs in the six registration owners with `NODE_KERNEL_METHOD` and traced the measurement, feature, and exchange families to their operator planning code. The public sets match with 93 distinct IDs. Mesh registration is a separate call and falls outside this count.

| Registration owner | Distinct operator IDs |
| --- | ---: |
| Primitives, curves, schemas and deconstruction | 17 |
| Surfaces and sweeps | 14 |
| Boolean, transforms and solid features | 23 |
| Intersection and evaluation | 13 |
| Measurement, utility and topology | 18 |
| Exchange | 8 |

| Finding | Scope and risk |
| --- | --- |
| Closest Point and Distance already schedule original-face jobs | The face cursor covered planar line and conic trims. It refused original spherical faces even though the same cap and wall faces already had exact area. This assignment reused their authored chart validator and retained the installed operator schemas and result channels. |
| Several registered feature operators still use immediate admission | `brep.solid.fillet`, `brep.solid.filletVariable`, `brep.solid.chamfer`, `brep.solid.chamferAsymmetric` and `brep.solid.defeature` fall into synchronous evaluation. The separately named retained edge-selection operators have different source and selection semantics. Aliasing the whole-solid forms would change which edges the original graph processes. |
| Heal remains immediate. Convert to NURBS now admits a funded cuboid subset | Heal still mutates live solid topology synchronously. The retained Convert to NURBS cuboid subset preserves source topology and stages degree-one spline geometry through publication or rollback. Other solid kinds and standalone curves receive a typed refusal. |
| DWG uses immediate admission | It follows the mesh codec path, separate from the retained original STEP, STL and OBJ operations. |
| Retained admission and geometry coverage are separate | Surface Closest UV has retained admission and exact nearest for supported analytic surfaces. A general rational-surface implementation needs bounded global minimization on the authored parameter domain. Strict unsupported domains remain a part of the operator contract. |

The registry count is a source audit. The full official registered Flow suite was deferred under the current disk gate.
