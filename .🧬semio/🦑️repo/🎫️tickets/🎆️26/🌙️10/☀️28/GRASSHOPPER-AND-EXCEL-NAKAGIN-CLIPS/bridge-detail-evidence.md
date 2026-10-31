# Bridge Detail Evidence

## Source Geometry

The repository native bridge assets are inspected in place by inspect-bridge-cad.py and inspect-bridge-glb.py. No source asset is copied to the ticket. The full 3dm contains22 physical Breps plus443 wireframe curves:2 base solids,1 metal plate and19 rod objects. Five post objects are exact duplicate placements. The1:200 model retains those22 Breps; the1:500 model simplifies to13. The GLB contains264 line primitives and22 physical mesh objects (115 triangle primitives), agreeing with the physical CAD layer partition.

| Observable component | Native geometry |
|---|---|
| Main span/width | X0..5.25m, Y−2.3..0m |
| Flat bases | Both have topZ0, bottomZ−.2m |
| Expansion gap | X2.5..2.75m between base halves |
| Metal wearing plate | Z0...01m across bases and gap |
| Left return | X0..1.25m, Y−4.8..−2.3m |
| Diagonal side frames | Four sloping tubular legs forming V profiles, not crossed X members |
| High end centreline | Z2.3585786437626903m, approximately .05m tube radius |
| Guardrail | Top centreline1.25m, midrail.8m |
| Rails/posts | Top tube radius.04m; posts/midrail radius.03m |
| Steps | No stair-tread physical objects or rising deck geometry |

The native GLB triangle projection was opened and inspected in axon and elevation. The archived [Domus520 section](https://www.domusweb.it/en/from-the-archive/2010/12/21/kurokawa-s-capsules.html) also shows horizontal connecting decks with V-shaped diagonal side members. Diagonal lines are therefore represented as structural side-frame rods; no bridge staircase or endpoint height difference is inferred.

## Implemented Profile and Registered Footprint

The bridge_geometry schema records the source-reference dimensions and the actual closed plate, bases, tubular V side frames, guardrails and native south L-return. bridge_assembly(level) reads the current bridge_elevation(level) and source-core face endpoints; no absolute bridge height is embedded. Main width is now2.30m and centreY is−1.45m. The calibrated GLUON central-plane cut independently supports approximately2.21m width and centre−1.44m. See source-reference-recovery.md and bridge-footprint-recovery.md for the registration evidence.

Each A-end return is1.25m wide and extends2.50m south: under current endpoints it occupies X[−8.35,−7.10],Y[−5.10,−2.60]. The same morphology is positively visible on all three scan deck projections. Main south A guards start beyond the return junction, and an overhead skew crossbar joins the shifted south mast. No guard crosses the walk connection. Return side/end guards remain physical. Explicit body routes stop0.39m before the closed end guard, retaining0.60m passage width and2.10m headroom.

The main deck remains a continuous flat wearing plate over two concrete bases separated by the native0.25m gap. The return has its own plate/base and perimeter guards. Native joined objects are decomposed into semantic physical members, so model part counts are not asserted to equal native Brep counts. All reconstructed solids are closed and positive-volume.

## Placement and Clearance Limits

Native5.25m reference span differs from the5.60m source-core gap. Dynamic endpoints remain attached to the authored core faces while width, rail profiles and return dimensions follow the recovered native reference. This endpoint adaptation is explicitly reconstructed. The earlier1.25m/−0.9m working footprint is superseded, and the native return is now implemented.

Core approach openings use the same width/offset. The actual0.60m body route passes through each portal atY−1.00, then aligns with the deck atY−1.45 only outside the core, clearing the source-derived chamfer pier. No pier or clearance threshold was removed. Endpoint datums belong to the independent global dimension study.

## Final Validation

Seven NumPy/rhino3dm bridge contracts pass in2.838s: exact width/offset/return bounds and datum, split bearings, closed positive-volume members, full return-junction clearance, and portal transition geometry. Independent AJV accepts authored data and rejects12 impossible bridge parameter sets.

The integrated circulation snapshot passes18 tests in24.588s, covering office strips, all three bridge_return terminals, all bridge approaches/decks, every stair-centre path, actual all-part mesh barriers, full body-support coverage and independent NumPy/NetworkX graph agreement. These are bounded geometric checks of authored routes, not structural certification or proof of a fully surveyed historical reconstruction.
