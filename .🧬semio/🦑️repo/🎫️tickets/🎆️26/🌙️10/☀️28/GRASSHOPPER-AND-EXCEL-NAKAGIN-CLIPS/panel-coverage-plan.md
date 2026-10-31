# Conformal Perforated Panel Coverage

The correction is saved and verified: **135 physical tests and 52 structural tests pass on model AST 87255d8965e795fc9c5a00fed16fb8de67184b94552a72006f5d4959cdbaa084**. Broad physical-run inputs were unchanged; current acceptance passes. Three dedicated coverage tests pass in 19.955 seconds, and strict AJV rejects three malformed coverage contracts.

The cap partition preserves exact outer rectangle/profile vertices and the original N-angle inner aperture polygon independently. A radial aperture sector can contain several outer-chain vertices; only planar cap triangles are partitioned. True outer/inner perimeter walls close the material. Lofted sharp-casing boundaries share corresponding edge-fraction breakpoints, preserving exact rectangles and planar wall lofts. No extra radial walls, changed window radius, resampled aperture or pose/dimension changes are introduced.

Lining front/back outlines now use their actual quarter-band profile. Shared clipped doorway vertices use the existing eleven-decimal material-weld identity before edge cancellation, retaining original profile coordinates.

Portable cases/schema precede implementation under cases/🧱️panel. Independent NumPy/Rhino tests verify exact outer corners, unchanged aperture vertices, rectangle/profile-minus-N-gon area, positive material volume, corner coverage rays, manifold edge incidence, coherent orientation and every lining patch. Coverage spans full/exterior, all six physical families and sharp/sub-skin/default radii.

## Red/Green Chronology

1. Independent actual-source witness (1.049, 4.1, 2.249) was uncovered despite lying 1 mm inside the intended flat corner.
2. Baseline two tests failed: shell corner absent by 50 mm and lining quarter vertex by 35 mm.
3. Initial conformal caps passed both targeted tests in 20.507 seconds.
4. Full 134 integration caught lining-back four-face edges; all other 132 assertions passed. This failed snapshot is retained separately.
5. Canonical clipping identities and an all-lining material regression passed three targeted tests in 22.867 seconds.
6. Final 135 physical tests all pass, including coverage three, shared 59 and circulation 18. Exact current fingerprint acceptance passes; parent structural 52 uses the same model.

The former 184-test result is historical and superseded. Final rendering and native replay are separate pending gates.
