# Public Orthographic Plan Boundary Audit

Read-only capsule_views source review; root owns the P/S layout/API correction and deferred renderer edits. No physical model/source edit or heavy render.

## Concrete Outer Boundary Mismatch

capsule_views explicitly documents physical casing and finished interior, sets physical_casing metadata and measures physical2.60×4.20m bounds. It does not label the plan a nominal analytical schematic. The outer box is nevertheless drawn as four sharp rectangle corners.

The actual physical casing is a full3D radius.25 rounded box. Its horizontal middle-height section and top-projection silhouette have roundedXYcorners. Each drawn rectangle corner atX±1.30,Y−.10/4.10 lies outside that body by sqrt(.25²+.25²)−.25=.1035533906m. This is a concrete physical corner error, not uncertain historical precision or merely the old nominal2.5×4 dimensions.

## Existing Geometry-Derived Route

The model's standard-library _profile already constructs exact rounded rectangular quarter paths. Applying it to casing.width/casing.length/current corner_radius with the actual quarter segmentation, then translatingY by casing.back, provides the compatible outerXYboundary. Prefer reusing that construction or intersecting actual source mesh faces at the declared plan height, and protect equality against actual middle-height casing vertices. Preserve the physical width/length dimensions and unchanged aperture geometry; no external runtime library is needed.

A closed rounded casing envelope is still not a complete material section through real doors/windows. The existing family window overlay/API correction is separately root-owned; actual section material/opening loops should remain distinguishable from envelope/usable-boundary annotations.

## Inner Rectangle Scope

The current usableXYbounds are explicitly capsule_interior(left/right/back/front). The implementation's clear lining corner radius is anXZprofile/floor-ceiling detail, not an independently authoredXYvertical-corner radius. Therefore the inner rectangle can represent the authoritative usable boundary; it should not be arbitrarily rounded using interior_corner_radius. If advertised as an actual material section, derive its loops from actual lining/source openings rather than inventing an extra plan radius. This review establishes no new physical-model defect or historical dimension change.

## Saved Public Orthographic Repair — Source Reviewed

Root's saved capsule_views now obtains capsule_window(parameters,layout). That same actual circle supplies sheet metadata, plan opening and the branch choosing front versus side diameter dimensions. P/S use actual side-window longitudinal centre and normal; end-wall families use the actual front centre/offset. Both orthographic reference drawings explicitly retain bolts, preserving all four attachment-head parts without claiming they must all be visible through an opaque body.

The plan outer boundary now uses physical casing.width/length/current corner radius through the shared model _profile with the full-detail quarter policy and Y translation by casing.back. The authoritative inner usable rectangle remains unchanged; no unsupported XY lining radius is introduced. Overall2.60/4.20 dimensions are fabrication bounding extents, appropriately distinct from the rounded outline. Their virtual bounding-box witness corners need not imply those points are metal surface vertices.

The new portable1/4/8-quarter boundary test compares the actual shell's projected SciPy convex hull, not merely two calls to the same profile helper. The all-six-family public test independently checks actual aperture metadata/plan position and reference bolt-part inventory. Reported red evidence identified both the old rectangular corner defect and missing P/S family/bolt propagation. The combined green run is root-owned and still in progress at this source reread; no result or final image acceptance is assumed. No further concrete source contradiction was found here, and physical M remains unchanged.
