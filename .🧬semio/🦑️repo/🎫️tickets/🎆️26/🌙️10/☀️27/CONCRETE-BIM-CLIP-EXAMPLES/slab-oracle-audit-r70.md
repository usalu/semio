# Slab Oracle Audit R70

Read-only independent audit, 2026-10-29. No producer or test edits and no GEOS execution. Source SHA256 `d2f3c0d1bb97c93836fb61b53adfe94756c59c31217585899decf09cda4f59f2`.

## Frozen Operand Proof

Captured `left` and `right` only from the mutable `🗑️generated/slab-oracle-current-r69.json` snapshot; frozen JSON is `🗑️generated/slab-operands-r70.json`. SHA256 of exact UTF-8 frozen JSON: `644aa7f8c904c9e3ee6f56c65ddb9582a3b9c9652178d6e1d43b4748801da327`. This audit proves this pair only; it is not proof of all mutable snapshots, all source slab parts, or a completed partition contract.

Converted each JSON coordinate parsed as JavaScript binary64 to its exact signed integer over power-of-two denominator, then evaluated edge cross products with BigInt rational arithmetic. Thus no decimal rounding, tolerance, floating determinant, GEOS, or geometry library enters the proof. Left is a triangle. For its directed edge from vertex 2 `(8.038,9.156176618102)` to vertex 0 `(6.94604795491,8.373694848328)`, the third left vertex has strictly positive determinant; both endpoints have zero determinant. Every right vertex has strictly negative determinant. The triangle convex hull and right polygon convex hull therefore lie in strictly separated half-planes. Their intersection is empty, and exact intersection area is **zero**, not merely below an area tolerance. Convexity of the right polygon is not required for this separating-edge proof.

Exact right-vertex determinants in source order (numerator / denominator):

- `-105216731436056043908623200533 / 316912650057057350374175801344`
- `-220075311420619872657046373461 / 633825300114114700748351602688`
- `-249436228068771920559592408307 / 633825300114114700748351602688`
- `-59866488792510335444227347807 / 158456325028528675187087900672`

## Full Surface Versus Owned Material

Inspected `🧱️model/🐍️.py` lines 1327–1333: `full_parts` starts as the occupied slab surface before column ownership allocation. `_column_partition` subtracts column prisms from `parts`, returns their common material separately in `shared_parts`, and returns column sleeve holes as void prisms. `full_parts` retains column-owned common material but removes column sleeve holes. Consequently `parts` is exclusively slab-owned material, while `full_parts` represents physical slab reference material inclusive of shared column joints. Projected footprint comparisons also require care when holes or ownership vary along z.

The source-region partition contract in `🧱️model/🧪️tests/⚡️partitions.py` lines 66–67 builds its oracle from the original outline minus resolved slab voids, but selects owned `g["parts"]`. For slabs with column reserves this is a scope mismatch: the oracle does not subtract column-owned material, and even `full_parts` needs column sleeve holes reflected in its expected physical surface. The synthetic specimen uses no reserves and is unaffected by this distinction. Source surface coverage and exclusive ownership must be tested separately with corresponding oracles; deleting valid ownership subtraction or tolerating the missing area would not resolve the semantics.

The zero-area captured GEOS call is independent of that scope mismatch. It establishes that this specific allegedly slow intersection has an exact separating witness; it does not by itself identify why an external call is slow.

## Performance Followup

Existing `slab-oracle-deadline-r69.json` records 4,999 completed instrumented operations by 119.0946976 seconds, last intersection kernel duration 0.0002317000007678871 seconds (0.232 ms). This is continued progress, not evidence of a permanently stalled GEOS call. Wall average is about 23.82 ms per operation, approximately 103 times that final kernel measurement, but one kernel sample cannot establish the aggregate GEOS time distribution.

The diagnostic wrapper writes the entire mapped operand JSON once and overwrites phase JSON twice per binary operation: roughly 14,997 filesystem writes at 4,999 binary operations, plus JSON serialization/mapping. Native slab and union phase writes alter the exact write count. Kernel elapsed excludes those writes, while wall time includes them. Therefore this diagnostic cannot measure uninstrumented Python-per-pair overhead or justify attributing its wall average to GEOS. Heavy I/O is a concrete confounder.

The current progress file is empty (consistent with interruption during overwrite); the deadline summary omits native-slab counts and candidate counts. The old R23 capture gives SL-1 598 parts, but its source hash is `9418d0b426f084daa13f03d17720fba8cbd3aef0787552ef768a4b7e0660255d` with 448 products, so that number is not a current-source full-parts count. No current-source polygon count or STRtree candidate count is recoverable from the inspected captures. A fresh geometry worker was deliberately not launched for this short followup.

Static recommendation: Shapely 2 vectorized `intersection(left_array,right_array)` followed by vectorized `area` can evaluate exactly the same STRtree bbox candidate pairs (retain j>i), without adding a predicate filter or changing the 1e-8 overlap threshold. Individual polygon validity and polygon-minus-expected checks can remain unchanged. Bulk STRtree `query(polygons)` returns index pairs and would remove repeated Python queries; then vectorized intersection removes repeated Python geometry calls. This is batching of the same GEOS oracle, not an alternative geometry algorithm. Preserve pair identities so failures still identify their slab/part pair, and process bounded batches if candidate memory is large. However neither an exact batching speedup nor a Python-overhead bottleneck is established by the available evidence. First use uninstrumented elapsed timing or accumulate kernel time/counters in memory and write once at completion. The immediate correction remains the full/owned scope fix; no producer/test changes were made by this audit.

## Folded Correction Review R73

Static review of corrected `⚡️partitions.py`, SHA256 `40b3d24463ee3f966ce96925548453ade93bef414747b65eaf46cac7cd46e446`. Four test method names exactly match the original four; the temporary extra identity is absent. The existing source method now evaluates physical surface validity, containment, total area and pairwise overlap using `full_parts`, retains all original 1e-8 overlap/containment thresholds, and explicitly guards source reserve columns against holes. That guard scopes this source oracle to the current solid-column fixture; it does not claim a general projected-area oracle for future partial-height sleeve holes.

The folded existing-bearing-fixture check independently obtains shared structural volume from GEOS intersections of actual full-part XY polygons with actual column profiles, weighted by the actual z-interval overlap. It compares shared volume against both that reference and the retained 0.0207328 fixture, then checks owned+shared=full volume. No projected area is used as volume.

The additional unweighted owned+shared=full area assertion is valid specifically for this current fixture: every SL-R region has bottom6.34 and top6.51or6.57, while column10 has one solid profile spanning3.14..9.03 and column15 has its upper solid square profile spanning6.34..9.03. Thus each intersecting slab prism is cut across its complete z interval. No upper/lower owned remainder duplicates its shared XY footprint, so the XY-area partition is additive even though the source regions have differing thicknesses. This assertion is not a universal invariant for partial-height cutters. A future fixture introducing a cutter endpoint inside a slab interval would require z-sliced area checks or volume-only conservation.

Review found no source or geometry-kernel change in this correction. No test reruns were performed by this reviewer; execution and fresh full inventory remain with the owning agents.
