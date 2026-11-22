# Retained closest point on a planar polygon face

`brep.measure.closestPoint` now queries the live native Face, using its plane, original coedges, edge parameter ranges, and outer and inner loops. The existing fixed analytic projection remains in use for standalone analytic geometry, supported vertices and edges, full conic disks, and certified primitives.

The cursor borrows the face from the original engine on every turn and verifies its native Face ID and topology revision. It spends one event on plane preparation, one per ring opening, one per coedge, and one to return the point. Each turn has a fixed-size copy budget and the cursor holds constant-size state. The Flow job can stall on an insufficient grant or be cancelled between turns. It publishes the resulting point through the existing `pointOut` channel.

The plane foot uses the dual of the original frame's in-plane axes. Half-open ray parity and the exact `orient2d` sign classify it against the outer ring and each hole. A foot in the closed material region is the closest point. Otherwise, the cursor compares points on the original finite line trims. The comparison uses Euclidean distances to the plane foot in world space, where the common normal offset cannot mask differences. Equal computed distances select the first coedge in authored traversal order.

The polygon path assumes the native face has validated simple, coplanar rings with non-overlapping holes. The cursor checks finite bounded UVs, nondegenerate segments, coedge ownership, vertex continuity, and bounded closure. Nonzero UV magnitudes must fall within `1e-120` and `1e140` to keep the orientation expansion in the normal floating-point range. Curved trims in a general planar boundary and nonplanar faces return `UnsupportedOwner`. A surface with nonlinear trims needs its own projection algorithm.

## Verification

- The language-neutral tilted, skew-frame fixture covers nine interior, exterior, notch, hole and boundary cases. One hole case distinguishes the world metric from the UV metric.
- Three.js triangulates the exact polygon with both holes and projects independently onto each triangle. The combined analytic and planar oracle run passed 7 tests and 36 assertions.
- The source-only Rust harness mounted the production cursor and robust orientation implementation under a minimal in-memory topology contract. It passed all nine geometric cases, the exact turn count, and curved-trim refusal. The source-only analytic harness also passed after mounting the shared dual-coordinate helper.
- The registered Flow test uses real native topology, alternates edge orientations, uses nonzero trim limits, drives the operator with one-item grants, and checks an underfunded turn. It also exercises mixed-trim refusal and cursor invalidation when the live face, engine, or shape revision changes. Repository Cargo validation is coordinated with the parent ticket.
- Repository source-closure admission measured 3,892 active lines for the BRep engine, 1,632 for analysis, 2,283 for the original geometry jobs, and 1,663 for the Session core. All were within the 4,000-line ceiling.

## Orthogonal solids and rigidly placed shells

The registered Flow Closest Point operator now projects directly onto the exposed rectangles of certified orthogonal material. The bounded material-cell walk projects onto each exposed rectangle and picks the minimum Euclidean distance. It ignores internal occupied-cell seams and includes cavity and notch walls. The same walk still supplies signed distance to Classify and spherical clearance. For rigidly placed shells, the query uses the certified rigid frame for both the local projection and its world-space result.

The neutral fixture adds eight nearest points across Fuse, notched Cut, enclosed Cut, and a rotated and translated box Shell. It covers exterior corners, a diagonal notch edge, points inside material, cavity walls, and placed shell walls. The authored Flow test constructs the geometry via the registered Boolean or Shell operators and reads the returned point through Closest Point.

The [production-source Rust witness](sphere-material-distance-check.py) passed all eight projections, eleven spherical clearance probes in both orders, and refusal when extreme coordinates produce a nonfinite distance. The independent Manifold and Three.js suite passed six tests and 179 assertions. The source preflight passed WASI, Apple Silicon, Windows x64 and Linux x64, with 4,000 active lines each for Boolean, Engine, and Native Registry. The authored Flow test has not been linked during compiler storage reclaim.
