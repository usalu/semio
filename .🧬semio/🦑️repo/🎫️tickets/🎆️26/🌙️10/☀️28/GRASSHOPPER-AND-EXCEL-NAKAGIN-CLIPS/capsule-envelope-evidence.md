# Native Capsule Physical Envelope

Current reconstruction decision: physical casing now follows these native2.60×4.20×2.50m surfaces while nominal studies remain2.50×4.00×2.50m. The chosen as-built P/S offset is.60m from independent component-relative GLUON evidence; native.95 remains a conflicting reference. See physical-casing-implementation.md for current code and gates. The inspection chronology below distinguishes what the native asset proves from later reconstruction choices.

## Scope and method

Read the original eight capsule GLBs in place under the repository metabolism representation assets. Decode only TRIANGLES primitives and actual POSITION buffer values; exclude mode3 line strips/wireframe curves, accessor summary bounds, CAD control points and collider assets. All nodes have no transform. Convert GLB Y-up to CAD Z-up as (x, −z, y). A direct Node buffer decoder and a retained NumPy inspection script independently repeat this calculation. Source files are not copied.

## Actual physical surfaces

All eight assets have physical extents X=−1.29999995..+1.29999995, Y=−2.09999990..+2.09999990, Z=−.200000003..2.29999995 metres: approximately2.60wide×4.20long×2.50high. Slash has1474 physical vertex entries; Backslash1448, J1472, L1474, P1470, S1468 (Z1470/Q1468 archive alternatives). Slash top and bottom are actual flat material surfaces at Z2.30 and−.20, not bounding-box inflation. The porthole glazing is a separate mesh; the other physical mesh contains shell/entry/window trim. No extra physical lower service-rail geometry projects beneath−.20 in these assets.

The earlier native Brep volume bounding box2.86057×4.46057×2.76057 and wireframe extrema2.628626×4.246242×2.527525 are therefore not the physical meshed envelope. They must not be used as evidence of a2.7m capsule height. The native2.6m width also differs from the nominal2.5m teaching width; this repository fixture is not a dimensional preservation survey and does not authorize changing canonical nominal dimensions.

## Museum dimensions and limits

[M+ A806 primary object record](https://www.mplus.org.hk/en/collection/objects/unit-a806-nakagin-capsule-tower-19701972-tokyo-japan-2024104/) was reread on2026-10-29. It publishes only “overall:255×270×420cm”, with no axis labels or dimensional-order convention in the object metadata. Thus2.70cannot responsibly be assigned to height. The restored architectural fragment may include finish/hardware, but this page does not define their contribution. No claim is made that hardware explains the measured scan gap.

The native physical body does not support a2.70m height or a hidden.20m downward rail extension. Its body2.50m high and source-backed current pitch2.80leave a nominal.30m same-phase inter-body interval; that arithmetic does not independently calibrate scan scale, underside details, registration or threshold datum. Existing reconstruction hardware uses explicitly inferred dimensions and cannot be circular evidence for the historical gap. No production dimensions, poses or energy inputs were changed.

Retained input: inspect-native-envelope.py. Generated numerical comparisons: capsule-physical-envelope.json and native-envelope-numpy.json.

## Exact physical glazing disks

The retained inspector now isolates planar physical triangle surfaces with two1.30m spans and onezero span, then independently least-squares fits their actual vertex coordinates with NumPy. This excludes trim, wireframes and control/accessor summary bounds. The separate64-vertex glazing disk is disambiguated from a56-vertex coplanar rear aperture surface where present. All8native assets were checked.

Slash/Backslash/J/L glazing centres are approximately(0,2.03433323,1.05000001)m in native centredCADcoordinates, radius.65m. Backslash also has an aperture plane atY2.03333330. P/S glazing centres are exactlyto float32precision(1.234876275,.950000008,1.050000007)m, radius.650000008m; their rear aperture disks are atX1.233333349 with the SAME longitudinalcentre.950000005. UnusedQ/Zmirror Xnegative with the sameY.95. The fitted native geometry therefore positively supports longitudinaloffset.95, not.63. Native Z1.05 is1.25m above the actual bodybottom−.20.

The source-correlated scan rim estimates.588–.724reported independently differ from the native asset. Their registration and local-body-centre uncertainty must be resolved before treating either as a measured historic exactoffset. The native asset alone does not authorize changing.95to.63; no production offset changed. Both methods' raw results are retained under generated numerical outputs. The NumPy inspection exits0.
