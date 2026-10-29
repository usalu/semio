# Actual Parking Oracle Runtime Profile R62

Read-only isolated runtime profile,2026-10-29. No production/fixture/test source changes. The ORIGINAL ParkingContract.test_physical_oracle_matches_native_height_intersections method is called unchanged in a fresh worker with source SHA256c53b7cbe645fb7660d4662799f9e3d79c928dc91d1b3d700bfff7693f5e2e208 asserted. Runtime wrappers around api._vehicle_parts,SciPy ConvexHull and shapely.ops.unary_union preserve every original return value and tolerance. No other worker's modules were patched. Retained input parking-oracle-profile-r62.py; phase events/progress and active union GeoJSON are under generated/parking-profile-*-r62.json.

## Measured Hotspot

Full _vehicle_parts completes in60.3453s and yields11,976records. The logged380mesh ConvexHull calls consume0.12718s total (the wrapper reports each ten calls). The first656unary_union calls complete; the slowest completed union takes0.02275s. Union657 starts atelapsed128.6899s with104actualpolygons and remains active on subsequent observation. Captured exact actual polygons under generated/parking-profile-current-union-r62.json.

Only one frozen expected group has104polygons: CW-1/curtain/frame,z0..0.025. Active polygon bounds follow the curved hall frame (approximatelyX3.13..13.83,Y4.58..10.76), matching that family. This specifically identifies global overlay of the actual bottom curtain-frame projection as the observed hotspot, rather than the full source build, SciPy mesh clipping or the earlier tube sectors. The wrapper logs union-start BEFORE calling the originalGEOS union; absence of union-end is direct phase evidence.

Instrumentation also writes progress and input snapshots, so total profiled elapsed time includes its filesystem overhead; individual original-call timings are the relevant measurements. No testPASS or corrected-geometry claim follows from a stalled/cancelled profile. The planned bound is cancellation once this one union has remained unchanged180s, or600s overall. Current profile is still running when this section is written; final disposition will be appended.

## Final Bounded Disposition

The same original union657(104polygons) remained unchanged for201.85s, at330.54s overall. Terminated only this isolated profiling worker at the declared180s-union bound. No union-end, oracle result or testPASS was produced. Exact current union polygons and656completed-union events are retained for a bounded equivalent-overlay diagnosis. The concurrently running actual feature cohort was not interrupted by this audit. Final cancellation metadata is generated/parking-profile-cancelled-r62.json. This now supplies measured justification to consider R61’s exact bounded residual method under a separate authorized change; this profile itself did not alter any contract.
