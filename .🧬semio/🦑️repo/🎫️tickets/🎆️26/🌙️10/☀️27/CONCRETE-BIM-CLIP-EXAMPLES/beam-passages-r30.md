# Fixed Drain Beam Passages R30

## Source Contract

The source authors four fixed vertical 100 mm reserved profiles on BEAM-PRIMARY-F-2-7, BEAM-PRIMARY-F-4-10, BEAM-PRIMARY-R-2-7 and BEAM-PRIMARY-R-4-10. Their outlines exactly equal the corresponding native floor/roof opening outlines. The nominal drain designation of 80 mm is sourced; physical OD80 mm, wall3 mm, bore100 mm and concentric registration remain inferred. The nominal radial envelope gap is10 mm; polygonal facets slightly reduce its shortest geometric value.

The reservations belong to beam geometry, independently of any current pipe route. They cut only the beam's finite vertical material extent. Moving a pipe must not move its structure reservation. No new voids are inferred in remote beams, cellar beams or footings.

## Independent Proof Interface

Portable input is analysis/tests/🕳️passages.json. The co-located Python unittest module uses actual public solid_geometry/service_geometry products and checks all110 beam products, including RIB-CELLAR and BEAM-CELLAR-STAIR. Physical parts, shared parts and full net-passage parts are each inspected without treating intentional ownership overlap as an obstruction.

GEOS tests pairwise occupied cross-sections with holes. Exact full-height projections establish constant sections where applicable; general native convex solids use independent SciPy hull edges intersected with horizontal planes, partitioned at all vertex heights and integrated with NumPy Gaussian nodes. An explicit filled-plug case has known positive volume, and a tetrahedron slice has independently known area. Closest material gaps use the separate GEOS/SciPy distance oracle. Fixed-cut identity is independently checked as raw beam cross-section minus the authored opening polygon at every material height band.

## Results

All five source/portable contracts passed in27.231 s in a fresh worker. Every actual pipe-versus-beam candidate passed for physical, shared and full net-passage domains across all110 beam products; no additional cellar or remote beam obstruction was identified. Four explicit reservations exactly match their native slab-opening profiles, remain unchanged after pipe rerouting, and equal their raw finite beam material minus the fixed profile.

Minimum independently measured material distances for the four full beam passages are0.009994645448 m,0.009994645448 m,0.009994645446 m and0.009994645446 m. These are the faceted realization of an inferred nominal10 mm radial gap, not a sourced installation minimum.

The final raw-cut proof avoids large global unions: independent STRtree partition overlap, containment and summed area use a1e-10 m GEOS precision grid with1e-8 m² identity tolerance. Actual absence checks retain1e-10 m² section and1e-10 m³ material-intersection thresholds. The portable filled plug produces the expected positive volume, so a filled beam cannot pass by the hosting policy.

Live source SHA256:393446bfae97f9abcd5b151e644367e72ea9fea0d0b9e26c9c3e0ac253f7783e. The proof adds no runtime dependency or source geometry change. Both clash and passage suites are registered once in the standard main load_tests entry.
