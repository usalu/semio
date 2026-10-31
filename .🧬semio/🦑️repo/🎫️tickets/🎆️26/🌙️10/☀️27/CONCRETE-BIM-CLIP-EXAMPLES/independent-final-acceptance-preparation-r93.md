# Independent Final Acceptance Preparation R93

Read-only review before final freeze. No whole-model capture or pair audit was started. No production or schema files were edited.

## Ramp Plan Decomposition Ink

Confirmed production renderer defect by direct inspection of `_plan_ramps`, line 462, in `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/📋️plan/🐍️.py`. The renderer constructs a polygon for every flight-owned convex part and assigns each polygon a nonzero INK stroke. Consequently shared part boundaries are drawn independently, including arbitrary convex decomposition edges. The same loop also applies semitransparent fill to each part; overlapping projected parts can darken their common projection. Source outlines do not identify those edges as construction joints. This is a renderer claim unsupported by the authored physical model.

Snapshot plan SHA256: `5958a5cda487edc134478287a32fa8aaa8a8054d9fcaf0c7429da2bb56433212`. Source review is not a final frozen visual acceptance.

Repair recommendation: compute the exact planar union of the existing owned part footprints per flight. Render its area once, with no part strokes, and stroke only the union exterior and interior rings. Keep every disconnected component and hole. Preserve all true recesses/cuts, actual flight ownership, height visibility, selection styling and directional arrow. Do not replace the union with a convex hull, source nominal strip, broad mask, rectangle, or uncut walking outline: each would erase actual geometry. A native deterministic boundary-union interface can remain dependency-free at runtime; validate its neutral polygon cases against Shapely/GEOS in tests.

Required controls before acceptance: two adjacent triangles forming one rectangle yield one outer boundary and zero diagonal; a concave notch retains its notch; a hole remains empty; disconnected components stay distinct; projected overlapping parts do not receive duplicate opacity; actual RF-2B produces exactly its GEOS footprint union and no stroked interior seams through the recording drawing interface. Compare boundary linework as well as filled area: an area-only test cannot detect the defect.

## Existing Repairs and Provenance

Read the garage-threshold and ramp-arrival reports. Their evidence distinguishes inferred geometry from surveyed source. The garage extension preserves existing FFL, leaf and opening while continuing the actual ground-bearing slab to its host exterior face; it does not claim an original track or seal. The ramp arrival fix uses actual occupied height for declared ramp_finish, leaving bedding hidden. Both descriptions are architecturally coherent within the explicit teaching approximation. I have not independently rerun their geometry here.

The physical atlas report explicitly declares stable=false and broad occupied bands without occlusion. It cannot certify final plans or final all-product-pair interference. Its dense partition edges likewise cannot be read as actual floor joints.

## Final Audit Readiness

Expanded runner and v2 freshness contract were read. Existing controls output contains 11 expected geometric cases, including retained positive controls and holes/nonconvex negative controls. The output itself has no final run identity or completion receipt, so this read does not promote queued command58397 to GREEN. Its owner must poll the actual session. Seven freshness controls remain separately required.

After explicit FINAL FREEZE: run normal-admission frozen Python3.12 controls, capture and audit; retain every positive/unresolved case with no tag waivers, filters or unsupported exclusions. Require before/after source and full producer identity stability, current capture identity and honest native SDK cell/air/bearing proof. A source or producer change during final work invalidates final certification and requires a new stable capture. Final visual review must inspect the production boundary rendering, not only SDK geometry.
