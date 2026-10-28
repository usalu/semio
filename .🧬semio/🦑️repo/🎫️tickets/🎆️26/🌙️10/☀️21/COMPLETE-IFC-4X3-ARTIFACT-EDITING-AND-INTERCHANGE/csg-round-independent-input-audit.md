# Circular CSG Primitive Independent Input and Oracle

The frozen neutral corpus has 20 explicit rows: 11 supported and nine refusal cases. Its closed schema contains dimensions and their physical carrier kinds, primitive and product placements, SI prefix, mapping frame/scales, sampling policy, expected bounds, segments and polygon/ideal volumes. No production source was imported or edited during the broad135 reader.

## Primary declaration and geometry

The cached released buildingSMART lexical pages specify bottom-disk centre at Position.Location, local XY disk and positive local Z height. Cone apex is [0,0,Height]. Cylinder uses Radius; cone uses BottomRadius. IfcCsgPrimitive3D owns Position: IfcAxis2Placement3D. Height and radius are IfcPositiveLengthMeasure. The pinned ADD2 EXPRESS source has these declarations at lines 5519–5527 and 10115–10125.

Primary cached pages are generated/csg-primitives/IfcRightCircularCylinder.htm (SHA256 0222a311f3263098b7c3f48486a262f76a2211e53684e6d658f370b845220f0a) and IfcRightCircularCone.htm (b468b5d184fd457f6b0fc042a11941b5f0442158b50416ea11b3a6916da152cb), from the existing pinned buildingSMART release evidence. Sphere and pyramid semantics are outside this corpus.

## Independent observations

`csg-round-three-oracle.ts` uses installed Three revision182 behind an owned array/scalar interface. All 11 supported rows match authored bounds and polygon volumes within 2e-6 relative/absolute tolerance. Default sampling is 72 segments; tighter absolute tolerance gives224, coarse tolerance gives8. It applies the actual moderate placement/mapping matrices and SI scaling. Cone volume is one third the corresponding polygonal cylinder; reflected nonuniform scale has absolute determinant24.

All measured meshes have closed opposite-direction edge incidence after identifying Three's known parametric seam/cap duplicates. This is explicitly symbolic constructor topology, not an arbitrary coordinate welding algorithm. Actual Float32 coordinates determine measured bounds/volume. Three's centre-fan cap triangle count is not a required implementation output. Reflection makes the raw signed volume negative; a producer must apply its established orientation correction once.

`py -3.13 csg-round-physical-independent.py` used installed IfcOpenShell0.8.5 without PYTHONPATH or installation. Nineteen rows serialize and reparse with exact FILE_SCHEMA IFC4X3_ADD2; the wrong string radius is rejected by the real constructor. Full EXPRESS emits no diagnostics for all11 supported rows or the segment-cap and positive-subnormal rows. Four nonpositive dimension cases fail IfcPositiveLengthMeasure.WR1; the foreign Position fails its entity role; zero mapping scale fails ScaleGreaterZero. Thus six physical rows have diagnostics. Physical source hashes, actual object IDs, placement/mapping matrices and raw normalized diagnostics are retained in generated/csg-round-independent/physical.json.

Normal IfcOpenShell geometry was deliberately not rerun or claimed to agree: the parent's complete default cylinder/cone products already record unsupported/empty normal-backend geometry. Three and analytic polygon measures supply independent geometry evidence; physical EXPRESS supplies separate declaration evidence. Sampling resource and normal-SI precision refusals are implementation admission policies, not schema invalidity.

## Inputs and boundaries

The author script, JSON, schema, feature and both oracle scripts remain ticket-owned inputs. Generated Three and physical outputs are under generated/csg-round-independent. Neither corpus nor oracle provides MVD/view certification, full CSG/boolean support, native execution, or a universal floating-coordinate error certificate. Future production tests must independently preserve original progress/cancellation, output grants and finite/nondegenerate projected triangles.
