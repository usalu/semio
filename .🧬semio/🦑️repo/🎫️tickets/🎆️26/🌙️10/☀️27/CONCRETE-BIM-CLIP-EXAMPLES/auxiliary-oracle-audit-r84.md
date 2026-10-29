# Auxiliary Oracle Local Difference Review R84

Read-only original test/source snapshot review and isolated portable GEOS comparison. No source/producer/test edits. R40 report localizes the slow active operation to unary_union of813primary polygons atz6.140000000000001; individual native polygon validity does not guarantee a cheap global overlay. No permanent-kernel-hang claim is inferred.

## Exact Reference Equivalence

For each unchanged source height band, let raw auxiliary polygons beAi and primary polygonsPj, including their holes. Set identity `(union Ai) minus(union Pj) = union_i(Ai minus P1 minus P2 …)` follows from difference distributing over union and successive subtraction. Primary overlaps do not change the identity; holes remain actual polygon holes and must not be converted into enclosing boxes. A primary whose bounding box is disjoint from the ORIGINAL raw polygon cannot affect any later remainder (which is its subset), so bbox culling per raw piece is exact. Use strict disjoint boxes; retain touching-box candidates or apply an exact disjoint predicate. No snap, coordinate perturbation, positive-area cutoff or new product kernel is necessary.

Therefore proposed reference may compute per-raw-piece sequential GEOS difference against bbox-local primary polygons, then unary_union of the resulting local remainders. It must still union the raw remainders because raw auxiliary pieces can overlap at joints. Keep original actual-owned union and1e-8XOR criterion, source z-level event set and midpoint rules. This changes the independent test oracle workload, not producer ownership or source physical material.

Original duplicate-primary assertion `area(actual intersection union Pj)<1e-8` also need not materialize a global primary union: sum of area(actual_piece intersection localPj) is conservative (nonnegative sum bounds union intersection), retaining1e-8 cannot falsepass but can be stricter where actual or primary pieces overlap. Alternatively union local intersection results then compare exact area to preserve equivalence. Never omit this second occupied-conservation assertion merely because the reference matches. Retain all existing tolerance values and no skip-on-GEOS-error.

## Portable Independent Checks

Retained neutral input auxiliary-oracle-cases-r84.json and reproduction auxiliary-oracle-reference-r84.py. Fresh isolated GEOS compared old global union/difference with proposed local sequential difference for ordinary rectangles, polygons with holes, overlapping raw and primary polygons, and remote bbox exclusions. Areas12/20/9m²; all symmetric differences exactly0, all outputs valid; selected primary calls1/2,1/1,4/6 respectively. These are reference-equivalence controls, not a completed actual source112-method result or guaranteed performance speedup. No actual captured813global union was rerun.

The source test currently uses slab-owned parts in primary plus beam-owned/shared and columns; changing physical primary scope is a separate semantic task. Preserve the original primary polygon domain during this workload correction, and do not accidentally replace it with bounding rectangles. Native mesh/section projections continue to use part_outline_at at the same z.

## Implemented Helper Independent Static Review

Reviewed checked `_local_primary_reference` after owner publication. STRtree queries are against original raw/owned polygon bboxes; sorted source indices preserve deterministic sequential subtraction. Raw remainders are unioned. Duplicate-primary check unions all owned/candidate-primary intersections, so overlapping primary candidates cannot double-count area. Filtering only intersection.area>0 excludes measure-zero boundaries, not positive thin material; all inputs retain validity/positive-area assertions. Exact set equivalence holds for ordinary, hole-bearing, overlapping, touching and far cases. Source z-event levels, actual part_outline_at/holes, material domains and original two1e-8source assertions remain. No source-specific exception, coordinate snap, cutoff or alternate product kernel found.

Owner recorded actual source facade contract plus callback2PASS33.185s with stable528b/be55 manifests. The implementation helper matches the independently reviewed set identities; no duplicate source geometry run was performed by reviewer.
