# Certified sphere distance

Flow Distance measures the minimum separation of two spherical solids or of a spherical solid and a certified oriented box. It resolves live kernel provenance through the existing measure operation. It reports zero for touching, overlapping, and nested material. The box path projects the sphere center onto the three certified axes and measures its gap to the finite box before subtracting the radius. Sphere pairs subtract both radii in a fixed order for identical results after swapping operands. Both paths refuse nonfinite results.

The language-agnostic fixture covers four sphere pairs and five sphere-box pairs, including a rotated and translated box. The native Flow test constructs each solid through its public operator, transforms it, and queries both operand orders. Its linked run is pending managed build capacity. The compact native witness compiles the production registry distance method and analysis source using the pinned native vector and Value crates. It reproduced both refusals before the fix and passes all nine rows in both directions. The separate Three.js oracle passed nine assertions against the same fixture.

- [Native red test](./🗑️generated/sphere-distance-red.log)
- [Native green test](./🗑️generated/sphere-distance-green.log)
- [Three.js oracle](./🗑️generated/sphere-solid-distance-oracle.log)

This support covers full certified spheres and certified rectangular material. Trimmed spheres, general curved solids, voids, and compounds require their own boundary distance algorithms.

## Finite curved primitives

Sphere Distance now also consumes the exact signed point distance of a certified finite cylinder, cone, or torus. The sphere's radius reduces the outside material gap. The same operation handles separated cylinder rims, cone sides, bases and apices, and material near the torus hole. Contact and overlapping material return zero in either operand order.

The registered fixture uses eight curved rows in addition to the nine sphere and box rows. Its native witness failed before the curved dispatch, then passed all three groups using the production primitive distance formulas. Three.js independently passed all seventeen rows using point-to-box, finite line segment, and sphere queries in the radial plane. The full retained Flow test remains registered and awaits the managed link.

- [Curved red witness](./🗑️generated/sphere-curved-distance-red.log)
- [Curved native green witness](./🗑️generated/sphere-curved-distance-green.log)
- [Combined Three.js oracle](./🗑️generated/sphere-curved-distance-oracle.log)

## Parallel finite cylinders

The same retained Distance operation measures two cylinders with common or opposing axes by combining the closed radial disk gap and axial interval gap. The six additional rows cover separated caps, separated side walls, diagonal rims, contact, overlap and a translated reversed cylinder. Both operand orders passed the native analytic witness. Three.js independently checks the disk and interval distances; the complete suite passed 23 assertions. Skew cylinder axes need a different surface minimization.

- [Native red witness](./🗑️generated/cylinder-distance-red.log)
- [Native green witness](./🗑️generated/cylinder-distance-green.log)
- [Third-party fixture oracle](./🗑️generated/cylinder-distance-oracle.log)

## Nested spherical cavities

A spherical probe now measures the exact distance to material around a certified spherical void. The native shell proof compares the exterior and cavity signed distances before subtracting the probe radius, so a sphere inside the void sees its cavity wall. The neutral cases cover eccentric centers, tangent material, spheres outside the exterior, and spheres overlapping a rectangular shell. The registered Flow test makes the shells through the retained Cut operator and checks both operand orders.

The native witness reproduced the prior refusal and now passes five groups. Three.js independently passed all 30 assertions for the combined solid distance fixtures. Full linked verification awaits the managed native compiler.

- [Native cavity red test](./🗑️generated/sphere-cavity-distance-red.log)
- [Native cavity green test](./🗑️generated/sphere-cavity-distance-green.log)
- [Three.js cavity oracle](./🗑️generated/sphere-cavity-distance-oracle.log)

## Cylinders aligned with a box axis

The retained Distance operator combines the finite axial interval gap with the disk-to-rectangle gap when a cylinder shares a certified box axis. The fixture covers diagonal separation, tangent cap rims and a rotated and translated box. Both operand orders pass the native witness. The combined native source witness now passes six groups, and the independent Three.js suite passes 33 assertions across four tests. General skew pairs still require a different minimizer.

- [Aligned box native proof](./🗑️generated/sphere-cavity-distance-box-green.log)
- [Aligned box Three.js oracle](./🗑️generated/cylinder-box-distance-oracle.log)

## Orthogonal Boolean results and box shells

The registered Flow Distance operator now measures separation between a spherical probe and finite material retained by an orthogonal Boolean or box Shell. For a sphere outside the material, the exact point-to-material distance at its center, reduced by its radius, gives the gap. A sphere touching or overlapping the material returns zero. The material certificate excludes internal cell seams from the boundary and preserves notch and cavity walls. Rigid placement maps the center into the shell frame before measuring the same Euclidean distance.

The shared neutral fixture covers eleven probes across Fuse, a notched Cut, an enclosed orthogonal cavity, and a rotated and translated box Shell. It exercises corner and diagonal edge gaps, contact, overlap, internal seams and a spherical probe inside a cavity. The authoring-route test constructs each solid through the registered Flow operator and checks both operand orders. Its managed binary has not run during compiler storage reclaim.

The [native source witness](sphere-material-distance-check.py) reproduced four unsupported-admission failures before the change. With the new dispatch, it passed all four geometry groups and both operand orders on all eleven probes, using the production material-cell Boolean, exposed-boundary distance, and sphere gap functions. Manifold independently constructs the Boolean or shell, and Three.js computes the point-to-triangle distance with Manifold ray parity for membership. The full oracle passed six tests and 171 assertions. Scoped whitespace and compiler-source preflight on WASI, Apple Silicon, Windows x64 and Linux x64 passed. The native registry uses 3,988 of 4,000 permitted source lines.
