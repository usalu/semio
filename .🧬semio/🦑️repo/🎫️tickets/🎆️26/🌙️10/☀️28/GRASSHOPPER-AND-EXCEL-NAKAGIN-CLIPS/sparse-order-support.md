# Sparse Support-Interval Ordering

The physical model is unchanged. The renderer previously projected every visible vertex against every distinct normal direction before its overlap sweep, storing two global item-by-axis tables. The independent performance audit identified the resulting matrix allocation as a concrete source cost.

The replacement computes each item's own support interval once, then projects only the actual overlapping candidates against the other face's representative axis. Work is split into at most 128 candidates per block. It uses actual source vertices, the same representative axes, the same separation error bound and the same forward/backward predicates. GH's separate exact projected-overlap depth correction remains intact. No geometry proxy, role filter, colour tolerance or new runtime library was added.

Portable JSON precedes implementation and covers variable polygon lengths, opposite axes, unknown normals, empty candidate lists and a 135-candidate block boundary. Its explicit values are checked against the former dense NumPy computation. The new function was absent for the observed red test (five failing subcases), then the isolated dense-oracle test passed in 0.192 seconds. A real drawing-engine test exposed an empty-input AxisError before the guard; empty/background identity and dense-oracle tests both pass after it, two assertions in 0.004 seconds with terminal exit 0. Background objects are returned unchanged.

The new cold/warm whole-building profile and strict raster/ray checks are still required. This report establishes the interval calculation and allocation change; it does not claim a measured whole-tower speedup or completed visual acceptance.
