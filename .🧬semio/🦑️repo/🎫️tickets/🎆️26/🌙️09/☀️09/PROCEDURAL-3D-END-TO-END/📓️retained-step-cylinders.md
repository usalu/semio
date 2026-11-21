# Retained Exact STEP Cylinder Import

The registered original `brep.io.importStep` route now recognizes a closed three-face cylinder from directly authored STEP Part 21 instances. It uses the existing exact native cylinder recipe and transaction. The same retained reader continues to recognize the box, triangular prism and orthogonal cell branches.

The supported cylinder has one outward axis-aligned `CYLINDRICAL_SURFACE`, two oppositely oriented planar end caps, two full `CIRCLE` trim edges, and one straight authored `SEAM_CURVE` used twice in opposite senses. The wall must alternate the two circular trims and the two seam uses. Every wall edge and cap must share the original vertex and edge identities, preserve face/curve orientation, and agree on the authoritative native plane/cylinder/circle placement and radius. The seam must carry two distinct STEP axial `PCURVE` records one complete angular turn apart. The angular check permits 1e-10 radians for decimal serialization of the periodic parameter. The native radius, axis, height and base are certified separately and use the authored analytic inputs.

The reader accepts independently authored Cartesian-axis cylinders with exact analytic planes and closed circle trims, including cylinders translated away from the origin. The geometry remains a native cylinder with the existing exact query provenance. Split arcs, open cylinders, oblique axes, general offset cap planes, bores, toroidal surfaces and other STEP curved shells remain beyond this route. The retained STEP writer still serves its planar domain. No new Engine, Registry, Session or runtime dependency was needed for cylinder import.

## Verification

- The language-neutral fixture references two existing Open CASCADE-authored STEP files. It specifies their analytic bases, axes, radii, heights, native topology, world bounds, areas and volumes.
- The cached native witness received an original one-event read grant at each stage. It verified the authored analytic geometry, exact world bounds and analytic measures, native two-vertex/three-edge/three-face publication, and allocator-observed cleanup. It rejects a missing `SEAM_CURVE` and a half-turn parameter seam.
- A pinned, independently downloaded Open CASCADE WASM reader reopened both authored files and matched three faces, bounds, surface area and volume within its declared tessellation tolerance. Its artifacts were removed from ticket scratch storage after the run.
- The registered `original_step_import_preserves_authored_full_cylinders` test also checks the original Flow job, native handle publication and exact measure routes. The linked Flow test is present and awaits an official Flow run.
