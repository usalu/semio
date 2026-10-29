# Compact Slab Operation Diagnostic R75

Fresh isolated current452/d2f3source worker, SL-1 only, unchanged coordinates and GEOS1e-8 containment/overlap assertions. Main validation workers untouched. Geometry1451polygons and expected9-ring Polygon stored once in generated/slab-operation-geometry-r75.json; one compact pre-call marker is overwritten for each original difference/intersection in original loop order. No full-operand per-operation writes or extra phase writes. Retained input slab-operation-r75.py. No producer/test edits.

| Observation UTC | Operation | Indices | Ordinal |
|---|---|---|---:|
|07:04:48.375|intersection|296,325|4756|
|07:05:48.250|intersection|931,992|10634|
|07:06:48.223|difference|1450 minus expected|15123|
|07:07:23.728|difference|1450 minus expected|15123|

The last marker remained unchanged across35.505seconds, with no result/pass and no exception reported. All earlier markers necessarily returned to allow this advance. This localizes a slow original GEOS difference call for the final SL-1 polygon against the complete expected polygon; it does not prove a permanent hang. The observer was intended180s but disposed at approximately200s due observation/tool cadence. Do not report an exact180s kernel duration. Worker disposed and only diagnostic interrupted; no SLR work started becauseSL1did not finish. Main method can also encounter later slabs/roof fixture; those remain unprofiled.

Frozen last operands are generated/slab-operation-last-operands-r75.json, SHA256`1df367fde3aa1a374e5e5de77518c5d335b59d1ed53ef3063126c64ba38a4c70`. Left ring has5coordinates including closure. The expected geometry has9rings. These frozen operands permit an independent containment proof or bounded exact same-call reproduction without rebuilding the entire method. Source provenance is in the geometry capture. No passing claim, algorithm substitution, snap or tolerance relaxation follows from this diagnostic.

## Exact Predicate Corroboration

Fresh isolated interpreter loaded only the frozen last operands. Both polygons valid. `expected.covers(left)` returnedTrue in0.004367399999864574seconds. Independent analytic rectangle/hole-bbox proof agrees: left is rectangle[0.08,21.44]..[18.91,21.55] contained in expected outer rectangle[-0.15,-0.05]..[19.15,21.55]; all eight hole boxes have maxY≤18.079, strictly below left minY21.44. Therefore outside material area is exactly zero for this captured pair.

A generic conservative short circuit is mathematically sound: when expected.covers(polygon), outside difference is empty and area0; otherwise evaluate the original polygon.difference(expected).area<1e-8 unchanged. Keep individual validity check first. This is an exact topological predicate proof, not a coordinate snap, approximate fallback or error suppression. It does not weaken the original tolerance and does not require changing any producer geometry. The measured fast predicate applies only to this frozen pair; whole-method timing/pass remains pending owning-agent execution. Diagnostic interpreter disposed.
