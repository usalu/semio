# IFC4 Parameterized Curve Profiles

## Contract And References

The schema-first language-neutral profile contract is `🧪️tests/📜️profiles.json`. Circular and elliptical outlines use 72 counterclockwise points with implicit closure. The maximum source-space chord deviation is at most 0.001 times the larger semi-axis; inherited nonuniform mapping scales this error with the geometry. Each sample consumes original caller work and existing vertex/triangle limits govern extrusion. Profile Position must be an IfcAxis2Placement2D and swept profiles must be AREA.

Sources: [IFC4 ellipse profile](https://standards.buildingsmart.org/IFC/RELEASE/IFC4/FINAL/HTML/schema/ifcprofileresource/lexical/ifcellipseprofiledef.htm) defines semi-axis 1 along local X and semi-axis 2 along local Y, without reordering. [IFC4 hollow circle](https://standards.buildingsmart.org/IFC/RELEASE/IFC4/FINAL/HTML/schema/ifcprofileresource/lexical/ifccirclehollowprofiledef.htm) defines outer radius and wall thickness strictly less than the radius. IFC4 has no IfcEllipseHollowProfileDef entity; elliptical holes require an arbitrary profile with voids.

## Validation

Tests authored before implementation; source execution results and native gate will be recorded below. Existing Three.js is the independent test-only curve and extrusion oracle; it introduces no runtime dependency.

## Implementation And Executed Results

Independent Rust and TypeScript projectors now support IfcCircleProfileDef and IfcEllipseProfileDef, and IfcArbitraryClosedProfileDef with a complete IfcCircle or IfcEllipse curve. Shared profileCurve/profile_curve helpers also accept the existing closed polyline representation for the enclosing profile-with-voids implementation. Circle hollow sampling is delegated to the main multi-ring work through ellipseProfile/ellipse_profile. Trimmed/composite conics remain unsupported.

Schema-first fixtures were written and executed before implementation: the three new tests failed with unsupported circle profiles, invalid-dimension mismatch and lack of curve work. Full conic arbitrary profiles were added in a second red/green cycle. Final source execution passed 20/20 permanent TypeScript tests with zero strict TypeScript diagnostics. Six neutral source graphs cover circle, ellipse, translated/rotated tall ellipse, negative extrusion direction, arbitrary full circle and arbitrary full ellipse. Three.js EllipseCurve independently produced matching double-precision outline coordinates; Three.js ExtrudeGeometry independently produced equal triangle counts and matching volume/area within its Float32 buffer precision.

Rust regressions consume the same six source graphs and invalid-definition rows, verify counts/bounds/outward volumes/chord errors, and exercise cancellation/work caps. Rust source was inspected after the concurrent integrity repair and contains one profile implementation plus both helpers. Native compilation/execution remains unverified behind the existing Windows native-build cache case-sensitivity gate; no build infrastructure was changed.

Changed files: geometry/🟦️.ts, geometry/🦀️.rs, geometry/🧪️tests/🔬️unit/🟦️.test.ts, geometry/🧪️tests/🔬️unit/🦀️.rs, new geometry/🧪️tests/📜️profiles.json and geometry/🧪️tests/📜️profiles.feature, and this report.
