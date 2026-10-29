# Physical Vehicle and Parking Revalidation R22

Source: 386 authored elements after the hollow cellar hoist/coal construction. No building, vehicle, jib or portal geometry was moved. The assumed car remains 4.6 × 1.75 × 1.65 m. Historical three-car capacity remains the separately documented CMN statement; three assumed modern cars remain unverified, without a global infeasibility claim.

## Physical Contract

Public `part_band_footprints(part,z0,z1)` clips occupied native geometry to the open vehicle-height band before projection. Ordinary concave prisms retain their contour; compatible convex lofts interpolate the two clipped rings. Native XZ/YZ section extrusions project their clipped occupied intervals, including disconnected components, rather than their full silhouette. Zero-height contact at the car floor is excluded.

Public `vehicle_obstacles(model,z0,z1)` returns isolated records cached by exact model JSON and height band. It enumerates physical columns, walls with actual apertures, typed door/window/curtain filling parts, structural/finish slabs, screens, stairs, details, furnishings, beams, foundations, railings, services, actual inclined ramp flights and landings. The original wall-centreline collision substitution has been removed. Native circular-profile chord error is retained conservatively. AABB lower bounds and nearest-candidate ordering accelerate exact polygon distances without changing their result.

The reviewed snapshot contains 1,061 height-overlapping projections. Covers COVER-HOIST (top 0) and COVER-COAL (top −0.13) contribute no vehicle-volume obstacle. This does not certify hatch opening, goods delivery or hoist operation while cars are parked.

## Static Result

The former positive control [12,14,90°] overlaps HOIST-JIB's base/post (largest projected intersection 0.0324 m²). It is now a required rejected case, alongside the prior invalid three-car layouts.

A bounded deterministic search evaluated candidate second-car centres x11.8…13, y14.2…15.2 at 0.1 m stations and angles85/90/95/100/105°. It found139 valid candidate poses in the first replay (about0.65 s indexed search after geometry preparation). Selected unchanged full-size cars:

- centre[13.79,9.5], angle90°;
- centre[12,14.4], angle90°.

Independent JavaScript and NumPy/GEOS validation confirms both static footprints: minimum garage-boundary distance0.225 m, minimum column-surface distance0.117 m, minimum other physical-part distance0.150 m and car-pair gap0.302654919 m. Required margins remain0.075/0.075/0.025 m respectively. This is a two-car positive control, not a claim of maximum capacity2 or manoeuvrable ingress.

## Canonical Arrival: Intermediate Source Failure

The default public `car_path` (120 samples, dense collision check) and `clearance` were executed. Current exposed teaching rainwater drops DP-2 and DP-4 intersect the assumed exterior route: minimum0, certified lower bound−0.116194305, `collision_free=false`, `clearance.fits=false`. Entry pair2/1 still has nominal side margin1.3475 m. The complete result is truthful; the building and route were not shifted.

The services agent is independently replacing those unsupported exposed drops using primary evidence for rainwater inside columns2/4. Therefore this intermediate failure is recorded here, not frozen as an acceptance expectation. A stable synthetic service-crossing regression tests the physical collision mechanism. Re-run source binding, native snapshot review, parking and canonical route after that publication.

## Verification and Inputs

Language-neutral cases in `🚗️parking.json` include flush closed cover, sloped section clipping, disconnected section projection without a false bridge, concave prism and translated loft. JavaScript polygon-clipping and Python GEOS/NumPy independently agree. Eight Python tests pass, including current native-part snapshot conformance, rejected old hoist-overlapping control, isolated cache results and a synthetic pipe collision. JavaScript validation passes.

Retained input `parking-r22-input.json` and bounded test-only replay `parking-r22-solver.py` support progress and cancellation; all generated snapshots/results remain under `🗑️generated`. The source fingerprint was reviewed against physical additions, not blindly refreshed. Section-only source parts are handled identically in the JavaScript/Python fingerprint filters.
