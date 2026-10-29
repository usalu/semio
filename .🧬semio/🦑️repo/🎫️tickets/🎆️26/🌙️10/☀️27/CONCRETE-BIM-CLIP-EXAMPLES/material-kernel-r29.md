# Material Kernel R29

The new three-zone garden posts exposed a real constructive-geometry defect, beyond excessive fragmentation. Convex difference accepted consecutive or explicitly closing duplicate vertices. A zero-length cutter edge classified the remaining material as both inside and outside, appending phantom material. Portable complete-cutter and half-cutter cases first failed with1.0m² and.5m² wrong occupied area. Exact adjacent/closing-vertex normalization in _ccw fixes them without dropping distinct curve facets or introducing an epsilon approximation.

Two separate portable cases also first showed that a provably disjoint or boundary-only cutter unnecessarily split a unit square into two pieces. Exact axis-aligned bounding-box rejection in _subtract now leaves it intact. Empty inputs are handled explicitly. Overlapping-cut geometry still follows the original clipping operation.

At449 source products, sourceSHA256e822635ae3ebd3c8861798601200da1d44ebdf73c322d83f19241697488dac3d, W-GARDEN-E originally contained4196parts and summed2.277667776455m³. Of these,4048were in the6cm shelf band3.86–3.92. Bounding-box rejection alone reduced fragmentation but still left wrong material; it was not accepted as a geometry-conservation success. Independent GEOS subtraction of the actual raw12leafparts and native occupied cutters showed that this clay band must be empty.

With duplicate-ring normalization and a fresh facade/cutter/ownership cache, the wall has36parts and2.170006838889m³. Independent GEOS tests across every source material/height band agree: largest symmetric difference1.7401e−12m²; largest summed-area error1.7200e−12m². The removed.10766094m³ was phantom/double-counted masonry, not a change to the authored wall silhouette or dimensions.

The controlled target-wall stage took7.54335s before and.17345s after; fresh cutter preparation is separately10.78364s. These are portable interpreter observations, not native rendering performance. Because the original output was wrong, this is a correctness repair with a performance benefit, not a claim of unchanged material volume.

All5 material-ownership tests pass12.966s, including original metadata/volume contracts, source column subtraction, the new portable difference cases and a new complete source garden-wall GEOS ownership oracle. Permanent files: model analysis/🐍️.py _ccw/_subtract, model tests/🪚️ownership.json and .py. No product external dependency was introduced. The generated benchmark is temporary; native render validation remains a separate gate.

The complete54-test shared geometry/orthographic feature cohort was rerun after this kernel repair and the initial R30 joint ownership publication. It completed with0failures/0errors. Later beam penetration/native-export changes still require their own identified follow-up; this pass is not presented as the final whole-goal cohort.

## Extended Ground-Wall Proof R30

The additional vehicle-height partition deltas are W-G-CHAUFFEUR280→190parts,W-G-ENTRY38→16,W-G-GARAGE-END177→68. Before accepting a derived parking snapshot, the independent raw-minus-native-cutters oracle was extended to these three walls as well as the garden wall. A nearcoincident GEOS operation stalled and that observation was cancelled without a pass claim.

The verification now uses a bounded adaptive test-only GEOS precision grid no larger than1e−10m. Grid size is reduced from the total input perimeter so the conservative accumulated area-displacement allowance remains below1e−9m²; the bound is added to the measured symmetric difference/summed-area error and must still fit the original1e−8m² criterion. Production geometry is unchanged by this verification precision. The progress-observable wall-ownership-r30.py input reports the active wall externally.

All5ownership tests, including all4source-wall band proofs, pass16.777s in the fresh interpreter. The current physical two-car control still has its original clearances; pending source/input-class changes are treated as later identified cohorts.


R34 final445 source: removed retired unsupported W-G-ENTRY from source_partition.additional_walls. Kept W-G-CHAUFFEUR and W-G-GARAGE-END plus coreW-GARDEN-E. All5independent material-ownership tests pass14.3s on the final facade cohort, including bounded GEOS snapping error and exact duplicate-cutter regression. No production subtraction change was needed. Retained wall-ownership-r34.py and generated progress/log output.
