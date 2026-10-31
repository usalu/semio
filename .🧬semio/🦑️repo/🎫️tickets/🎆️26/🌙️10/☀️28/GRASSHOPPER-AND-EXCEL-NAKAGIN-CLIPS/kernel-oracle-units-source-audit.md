# Kernel Oracle Units Source Audit

Read-only review of the corrected permanent ten-test helper and retained cached2443 benchmark. No implementation/test/case edits and no runtime execution.

No concrete dimensional or camera-transform gap found in the inspected corrected paths.

- Shared fragment tests hits lines35-40: cross and along have length-squared units; epsilon is coordinate spacing in length units and is multiplied by edge length before all cross/along comparisons. Zero-length edges use Euclidean delta norm against epsilon. Tiny edges no longer receive an absolute squared threshold. The clamped segment-distance regression47-57 independently computes Euclidean closest distance and excludes an out-of-bounds point before checking hits.
- Actual lineage camera test98-106 normalizes towards, chooses a least-aligned basis seed, normalizes their cross product, and uses [horizontal, towards cross horizontal, towards] as an orthonormal row matrix. Both vertices and normals multiply matrix.T. hits solves depth in this camera frame; last painted depth is compared to maximum physical depth at109-114.
- Permanent area helper18-21 sums oriented edge cross vectors before taking norm. Cached benchmark validate-lineage-bsp.py areas30-38 likewise sums fan cross vectors before norm. Both preserve concave signed cancellation rather than overcounting magnitudes of fan triangles.
- Portable cases test169-183 requires every emitted fragment finite and strictly positive-area, alongside conserved per-source/polygon material. Cached2443 areas explicitly asserts len>=3 and area>0 for both input and output, then compares exact source/polygon key sets and maximum area discrepancy below1e-10.

Proof scope: validate-lineage-bsp.py checks actual cached component termination/node consumption, positive material and conservation; it contains no independent ray sampling despite its first docstring. Actual-camera ray checks are in the permanent lineage test. This is a precise scope distinction, not a failed geometric assertion. Runtime35286 remains execution-owned; carrier oracle2641 remains GH-owned. No handle was polled or duplicated.
