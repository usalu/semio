# Parking Oracle Performance Audit R61

Read-only bounded source/fixture audit,2026-10-29. No test execution, full geometry build, fixture change, product/test edit or tolerance relaxation. The active cohort is left untouched.

## Specific Active Test Path

ParkingContract.test_physical_oracle_matches_native_height_intersections in model tests/🚗️parking.py162–193 first calls api._vehicle_parts(api.load_model()) at168. _vehicle_parts at current model.py1814–1830 immediately calls full building_geometry(model), enumerating all domains/storeys before the test clips to car height. This direct route does not use the lru-cached vehicle_obstacles path at1833–1843. Calling vehicle_obstacles earlier therefore does not necessarily save this direct oracle build. The active retained feature-cohort runner has no geometry-cache replacement.

The oracle then computes an independent SciPy ConvexHull per height-overlapping mesh, intersects section profiles with the height strip, projects tapered parts, and groups actual/fixture polygons by(tag,kind,role,rounded clipped z0,z1). Group-set equality is asserted at192. At193 each group forms two full unary_union geometries followed by symmetric_difference.area<1e−8. This is a genuine global overlay hotspot inside each group, but there is no per-phase/per-group timing in the frozen test. Test-level started status alone cannot distinguish full geometry rebuild, mesh projection, group union or final symmetric difference. The interrupted624s duplicate supplied no result; it cannot locate the stall or establish a test failure.

## Fixture-Bounded Workload

Read the frozen2542-obstacle fixture only. It contains339comparison groups. Largest group W-G-CHAUFFEUR/walls/solid/0..1.65 has120polygons/480vertices; CW-1/curtain/frame/0.9875..1.0125 has106polygons/330vertices; CW-1/frame/0..0.025 has104polygons/322vertices. Multiple circular columns and tube-service groups each contain96sectors/384vertices. These shared-boundary/near-collinear families are plausible overlay hotspots, especially curtain end-plane slivers after exact ownership subtraction, but their size alone is not a measured diagnosis. No source precision should be rounded away to accelerate them.

## Exact Bounded Alternative, Only If Authorized After Diagnosis

Keep the SAME actual independent projection, group identity and1e−8area assertion. Build an STRtree over comparison-group polygons. For each actual polygonP, query only expected polygons whose AABBs intersectP, and compute residualP minus those exact candidates, either sequentially or by their ROI-clipped local union. Repeat in the opposite direction. AABB filtering is an exact broad phase: a disjoint box cannot affect the difference.

To preserve the ORIGINAL symmetric-difference area exactly, do NOT blindly sum all per-polygon residual areas. Same-side input polygons can overlap, so that sum can count a missing patch repeatedly. Sum<=1e−8 would be a sufficient stronger check, but is not mathematically equivalent to the old tolerance unless same-side disjoint interiors are independently established. For exact equivalence, union residuals only within their STRtree-connected local components, sum those disjoint component-union areas separately forA\B andB\A, then assert their total<1e−8. Empty residuals need no union at all. This avoids unioning complete almost-identical global surfaces while preserving the exact set equation and strict tolerance.

Likewise, a source snapshot may be built once per immutable oracle input and passed to the direct physical-part enumeration, but only after preserving fresh-model/event tests and proving snapshot identity. Replacing the oracle with production vehicle_obstacles output would destroy the independent projection check. Do not make a frozen test change merely to conceal a slow run.

## Next Evidence Needed

If the active test continues stalled, first observe bounded phase timing/progress around the single full geometry build, mesh projection and current group identity/count, without changing the contract. A measured expensive group can then justify the exact bounded residual method above. This report is advice only; no implementation, fixture regeneration or altered pass condition was introduced.
