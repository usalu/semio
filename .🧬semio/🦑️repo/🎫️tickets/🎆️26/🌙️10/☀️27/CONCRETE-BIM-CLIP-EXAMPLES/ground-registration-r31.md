# Primary Ground Registration R31

## Control Method

The CMN visitor-plan pixels are registered with an independently solved affine transform using four outer structural grid intersections, the actual two hall-piloti dots at authored world axes [10.898,6.006] and [8.128,6.006], and the helical stair centre [6.23842,9.263605]. These structural identifications do not use the red visitor-room labels8/9. Seven control residuals are0.0273,0.0211,0.0215,0.0072,0.0156,0.1149 and0.1072 m. Maximum residual0.115 m;1.5 pixel reading uncertainty corresponds to0.130 m. Trace coordinates are therefore inferred registrations, not measured construction dimensions.

The source input is ground-registration-r31.json; reproducible NumPy/Matplotlib implementation is ground-registration-r31.py. Generated ground-registration-r31.png was visually inspected: left CMN primary pixels, middle FLC19414 design drawing, right all current ground columns/axes and current native glass against independently registered boundaries. The FLC design uses its own four structural-axis controls. Its broad curve and garage diagonal corroborate the defect, while its different stair and room arrangements remain a separate design phase.

## Canonical Coordinated Target

Hand-authored target polyline, metres with ±0.15 m registration uncertainty:

[3.15,10.76], [3.20,9.99], [3.42,9.16], [3.80,8.32], [4.35,7.47], [5.08,6.65], [6.01,5.89], [6.90,5.30], [7.85,4.87], [8.64,4.60], [10.50,4.60], [11.38,4.89], [12.36,5.34], [13.27,5.95], [13.82,6.17].

The actual glazed envelope stops at the garage junction [13.82,6.17]. The eastern conceptual boundary to about[16.0,10.75] is separately recorded and is not extended as curtain glazing. The diagonal garage boundary runs from the ramp edge[10.90,8.20] to[13.82,6.17]. The primary source supports the boundary role of the eastern arc; exact physical assembly/material there remains unresolved.

R33 owns the garage front/spine, ground spaces, service interiors and fixtures, and flat west/north/east outer ground walls. R31 owns the canonical curved curtain, main/service bays, obsolete host-strip removal, and eastern envelope physical resolution. Main entrance endpoint interval is[8.64,4.60] to[10.50,4.60]. Native service portal placement is being registered from the west tangent and the primary exterior image; no door is placed by car clearance.

## Curtain and Portal Schema Direction

Replace the car-derived centre/radius teaching dependency with explicit source-authored paths and independently evidenced member stations. Native curtain-host IfcOpeningElement geometry is proposed so real doors cut their curtain host, including fanlights above the portal. Current opaque wall strips W-G-ENTRY/W-G-SERVICE-ENTRY do not establish actual masonry construction. Every physical pane must be planar between its registered members; curved path tessellation is not a pane schedule. Visible photo stations, inferred widths/depths/rail heights and unresolved occlusions remain distinct.

## Additional Primary Evidence

[CCA PH1985:0889](https://www.cca.qc.ca/en/search/details/collection/object/362303) establishes narrow vertical members, an intermediate rail and textured glazing, photographer unknown and1931 or later. [Fondation Le Corbusier villa record](https://www.fondationlecorbusier.fr/oeuvre-architecture/realisations-villa-savoye-et-loge-du-jardinier-poissy-france-1928-1931/) links primary modern photographs. Gallery image11 (Paul Kozlowski, date not specified) clearly shows the service door at the west curved-glass/opaque-green-wall junction and fine glazing members. It does not prove every1931 material or joint. Research contact sheets and original downloaded images are generated outputs only.

## State

No production geometry has been changed by this registration. The inherited CW radius3.5/centre[8.5,6] is contradicted by two independently registered source plans. Earlier camera fits against that inherited curve are superseded exploratory results, not source measurements. The broad registered curve improves photograph fitting substantially; column-identity alternatives and occluded member positions remain under review. Native geometry, source topology, portable tests, all representations and parking must move together in the later coordinated cohort.

## TDD Native Curtain Host Contract

New portable model/tests/🪟️curtains.json plus co-located unittest module are authored before implementation. They fail on the missing public opening_geometry domain, the old curved-glass producer, and strict schema rejection of curtain_wall/curtain_path. The planned shared interface must retain the real finite portal void, disjoint planar panes/member parts, all seven representations and independent GEOS volume agreement.
