# IFC4 Parameterized Profile Review

Read-only review 2026-10-21 of repaired IFC4 any/🧬️schema/📐️geometry/{🦀️.rs,🟦️.ts} and profile contracts/tests. Line numbers refer to the read snapshot and may move during active ring work. Native code was not compiled; findings below are static.

## Findings

1. **P2: Invalid profile Position silently becomes identity.** Rust profile line 514 / TS profile line 277 uses generic reference/ref, which returns None/undefined for every non-reference value. Thus IFCCIRCLEPROFILEDEF(.AREA.,$,42,2.) is rendered exactly as if Position were $. The IFC4 optional Position is either omitted ($) or a reference to IfcAxis2Placement2D; a scalar, aggregate, string or derived marker must not erase user intent silently. Introduce a checked optional-reference reader distinguishing permitted unset from wrong kind, and use it for parameterized profiles and the axis RefDirection read. Add malformed present optional-field vectors. This is not a requirement for full EXPRESS compliance; it prevents concrete invalid input from being silently rendered as another shape.

2. **P2: Axis2Placement2D validates z value, not required dimension.** Rust axis lines 231 and 240–243 / TS axis 84,93–96 accepts a 3-coordinate location (x,y,0) and 3-coordinate direction (x,y,0); coordinates() erases original arity into padded Point. New ellipse_profile only checks entity type and final z (Rust 497,505 / TS 266,271). Official IFC4 WHERE rules explicitly require Location.Dim=2 and RefDirection.Dim=2. Inspect original point/direction coordinate list arity before padding for the 2D axis branch; add three-dimensional zero-z location/direction rejection vectors. Also use the same common validation for rectangle placement; otherwise rectangle still permits full IFCAXIS2PLACEMENT3D while ellipse rejects it.

3. **P2: Valid tiny curves fail existing polygon predicate absolute threshold.** New circle/ellipse dimensions accept every positive finite axis, but Rust polygon 333–374 / TS polygon 160 onward still rejects normal magnitude <1e-12 and ear cross-products <=1e-12. A circle radius 1e-7 has total normal around 6e-14 and is unconditionally diagnosed degenerate despite positive valid dimensions and representable geometry. Very thin eccentric ellipses hit ear tests earlier. The forthcoming ring normalization should correct this; ensure a retained neutral circle radius 1e-7 and highly eccentric ellipse exercise that path. Likewise translated profiles at huge coordinates can suffer area cancellation in extrusion Rust 562 / TS 309; calculate signed area from points translated to a local origin. Do not only fix polygon projection while keeping extrusion winding dependent on untranslated shoelace sums.

## Sound Parts / No Defect Found

The new sampling itself consumes one tick per sample, and downstream vertex/triangle emission already consumes budgets. The curve sampling is bounded to 72 points; no new unbounded loop was found. Axis parameter indices (position index 2, first radius index 3, second index 4) and local placement composition match official IFC4 parameterized-profile semantics. IFC4 permits SemiAxis2 > SemiAxis1; the tall ellipse fixture is valid (do not add a first-axis-major restriction from IfcEllipse curve to IfcEllipseProfileDef).

72 equal-angle samples satisfy the declared pre-transform chord error relative to max(a,b): 1-cos(pi/72)=0.0009517784 < 0.001. The fixture explicitly states pre-inherited-transform tolerance, so nonuniform mapped scaling is not a hidden violation of that stated contract. CURVE_RELATIVE_TOLERANCE is currently descriptive while CURVE_SEGMENTS is independently hard-coded; a contract test should assert the inequality so edits cannot invalidate it silently.

Repair verification: fresh Rust read has one Projector declaration and one mod tests footer; the previously reported injected suffix copy is no longer present in this read snapshot.

## Tests Worth Adding

Current 📜️profiles.json has four valid cases: origin circle, origin ellipse, a 90-degree placed tall ellipse, negative extrusion direction; invalid cases are zero radius, negative ellipse dimension, CURVE profile and wrong 3D placement entity. Existing test-only Three EllipseCurve/ExtrudeGeometry oracle covers points, mesh measures and triangle counts. Missing vectors are malformed present optional reference, 3D zero-z location/direction, arbitrary non-cardinal rotation, valid tiny/high-eccentric curves, huge translated profile, and curve cancellation at every step rather than selected work checkpoints. Three float32 ExtrudeGeometry makes it a poor huge-coordinate measure oracle: use its double-precision Curve points plus translation-invariant analytic signed area/volume, or IfcOpenShell, for those numeric tests.

## Primary IFC4 Sources Checked

- [IfcEllipseProfileDef](https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcprofileresource/lexical/ifcellipseprofiledef.htm): semi-axes are positive lengths along local x/y; Position optional, missing means identity. No relative major/minor ordering WHERE rule exists.
- [IfcParameterizedProfileDef](https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcprofileresource/lexical/ifcparameterizedprofiledef.htm): Position is optional IfcAxis2Placement2D and places profile in swept solid xy plane.
- [IfcAxis2Placement2D](https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcgeometryresource/lexical/ifcaxis2placement2d.htm): formal LocationIs2D and RefDirIs2D rules require original coordinate dimensions, not a numerical zero z.

No ring module review was attempted; it remains under implementation.
