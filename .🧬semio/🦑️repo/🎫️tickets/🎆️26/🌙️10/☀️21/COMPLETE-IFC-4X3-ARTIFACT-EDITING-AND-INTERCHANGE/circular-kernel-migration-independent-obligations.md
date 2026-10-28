# Circular Kernel Migration Audit Obligations

This is a static pre-producer audit, not a closure of moving production. Frozen20 and supplemental18 inputs remain unchanged. Current circle.ts, csg.ts and geometry.ts were read as source text only.

## Retain source authority at adapters

Existing Circle adapter requires exact IfcExtrudedAreaSolid tuple4 and exact IfcCircleProfileDef tuple4, no complex constituent projections, AREA ProfileType, optional actual 2D/3D placements, actual IfcDirection tuple1 containing three ratios, positive radius/depth and nonzero transverse normalized direction.z. New primitive adapter must use actual owner-qualified IfcCsgPrimitive3D.Position and concrete Height/Radius or Height/BottomRadius through current canonical field/query helpers. It must not create an extrusion/profile or pretend primitive attributes occupy Circle's tuple slots. Existing CsgSolid exact tuple and real placement point/direction role checks should survive expansion from Block. Preserve Block's60 logical-value admission and source unsupported diagnostics.

Adapters should finish source-role/arity checks before handing numeric data to the shared evaluator. A kernel receiving numbers alone cannot certify a source owner, profile AREA marker, required Position or valid raw numeric carrier. A nullable top radius is also insufficient to distinguish a cone from a degenerate cylinder accidentally; use an explicit closed shape policy and reject unexpected combinations. This is a proposed design constraint, not a defect observed in code yet.

## Missing exact tests before closure

1. Compare all existing Circle mesh positions and indices before/after extraction at unchanged72/default and24/704 world-policy cases; a changed triangulation should not be hidden behind volume-only assertions if preservation is promised.
2. Add the combined oblique negative-Z, rotated/offset profile and reflected frame row from csg-round-supplemental-independent-audit.md. Assert positive signed emitted volume, not only absolute volume. Separately keep negativeZ without reflection and reflection with positiveZ, so two accidental missing flips cannot cancel in the combined case.
3. Assert translated profile base centre is transformed once, and extrusion direction stays in solid local coordinates rather than profile coordinates. Current code applies profileFrame to base points, then adds depth*direction before solid/world transform. An extracted frame*profile matrix must not rotate the extrusion displacement with profile rotation.
4. For an oblique sweep, certify the actual composed circular basis while retaining transverse-height volume and positive normal-SI transverse requirement. Radius*depth alone is not volume; abs(normalizedDirection.z) is essential. Height vector translation must not enter the2D basis norm.
5. Exact grant rows at n72: cylinder1283 refuses/1284 admits; cone644 refuses/645 admits. Cumulative two distinct products sharing one representation must charge each output independently. Use valid SET Items, not duplicate item references. Source capture limits must be large enough to reach the intended mesh frontier.
6. Exercise original callback failure/cancellation before first source field, at sampling, first/interior/last ring point, each cap/side family, final generator completion and central reflection. Original undefined/typed class failures must remain exact; denied output cannot publish a partial mesh. A reflected fixture is needed to reach mesh-orientation progress rather than only local generation.
7. Demonstrate maxSegments, maxValues and chord policy are captured originals when caller mutates its options in progress. Kernel input must be the admitted captured policy, not a fresh read from mutable options.
8. Repeat world collapse at1e20, extreme edge difference overflow, subnormal SI dimensions/transverse height, singular mapped frame and minimum-capacity rejection. Projected triangle finite/nondegenerate checks must occur before publication; ideal chord certification cannot authorize collapsed output.

Central geometry publication currently counts coordinates/indices, validates represented determinant sign, reverses triangle winding (and normalIndices when present) cooperatively and then debits remaining values before pushing a mesh. New circle/primitive kernel must emit local outward winding only; central represented reflection remains the sole frame-sign owner. CSG primitives currently author no normals; no implicit authored normal transport is claimed.

## Audit readiness

Later frozen review should include shared kernel, both source adapters, captured policy callsites, public dispatcher, data/test consumers and exact source manifest references. Actual selected baseline and independent Three/physical evidence are complementary: Three's constructor topology is symbolic at known seams, while SDK primitive meshing remains unsupported. No new runtime dependency, native claim or whole CSG/MVD claim is justified by this plan.
