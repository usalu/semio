# IFC Boolean and Clipping Adapter Independent Design Review

## Source Facts and Narrow Initial Slice

The released catalog declares IfcBooleanResult.Operator, FirstOperand and SecondOperand on IfcBooleanResult itself; it derives Dim from the first operand. IfcBooleanOperand selects BooleanResult, CsgPrimitive3D, HalfSpaceSolid, SolidModel and TessellatedFaceSet. Its actual WHERE rules require equal dimensions and require Closed to exist and be TRUE for any tessellated operand. IfcBooleanClippingResult adds no direct tuple fields: its inherited operator must be DIFFERENCE, first operand must be SweptAreaSolid/SweptDiskSolid/another ClippingResult, and second operand HalfSpaceSolid. These are additional constraints, not interchangeable class labels or a general BooleanResult-to-Clipping promotion.

Pinned ADD2 EXPRESS SHA256 f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860 contains SELECT3316, Clipping4348 and Boolean4358. Source graph reads must use exact owner-qualified slots and the existing canonical indexed row, preserving unknown/ambiguity and carrier admission. Semantic Boolean rendering is separate from selected Body Clipping MVD membership and whole schema validity.

Start with finite IfcBooleanResult operations on actual already-supported closed finite operands: Block, Cylinder/Cone primitives, existing closed faceted/TFS/PFS and supported area extrusions, plus nested finite BooleanResult. Explicitly refuse unsupported operand declarations/profile families rather than omit or approximate. Operations UNION/INTERSECTION/DIFFERENCE need actual closed input admission and the parent's shared cooperative Boolean kernel. Clipping/half-space is a separate bounded next slice, because an infinite operand is not a finite triangle mesh.

## Original Physical Corpus

Fresh byte census covers unchanged56 originals (46 official plus10 certification): exactly one IfcBooleanResult, no IfcBooleanClippingResult or HalfSpace/PolygonalBounded/BoxedHalfSpace. Per-file hashes/IDs are generated/ifc-boolean-adapter-independent.json. This is bounded lexical evidence, not a universal official absence or applicability claim.

The real positive graph is bath-csg-solid.ifc SHA4592f3029faa12625494ada734fcfe46e4fc91a33d001198e9e3144721870b95: product225→mapped representation→CSGSolid208→BooleanResult207 DIFFERENCE(Block200, ExtrudedAreaSolid206). Block is2000×800×800mm at identity; cutter is centred rounded rectangle1800×600mm/radius200mm, displaced[1000,400,100]mm, +Z extrusion700mm. Plain installed IfcOpenShell0.8.5 observation retains operand/product placement matrices and exact ADD2 header. The ideal analytic volume is1.28−.7*(1.08−(4−PI)*.04) cubic metres; it is not a newly measured mesh certificate.

The cutter is IfcRoundedRectangleProfileDef, currently outside the supported profile adapters. Thus a finite Boolean adapter alone does not render this original; a separate genuine rounded-rectangle profile evaluator is needed. Do not replace it by a plain rectangle or relabel this original as a new adapter pass. Its top cut is flush/coincident with the block top and is a meaningful future coplanar robustness case. Authored Block–Block positive STEP fixtures provide an honest first producer target while the original remains an explicit unsupported profile boundary.

## Shared Stream, Frames and Authority

Reuse the existing representation cursor for external representation mapping and its active-path cycle control; do not create a second graph/index. A Boolean AST needs its own bounded explicit stack: enter actual owner/operator/operand references, evaluate first/second in source order, run the kernel and return one result. Active-path IDs catch cycles while permitting repeated DAG operands. A global visited set must not discard a repeated source occurrence. Every stack/source-sized mesh operation needs original limits, checkpoints and captured caller scope.

All finite operands must share one coordinate space and SI scale before the operation. Primitive Position and extrusion frames remain actual source data. External mapped frame reflection must be applied consistently once for kernel-valid outward inputs and once for final published orientation only if not already baked; never run the central reversal twice. Pick and test one explicit convention. Independently reflected one operand, reflected whole result, oblique negative sweep, nested mapping and repeated shared operands are required regressions.

IfcCsgPrimitive3D is directly legal as an operand. Current CSG adapter accepts an actual CsgSolid root before reading its primitive. It cannot be reused by fabricating a fake CsgSolid wrapper. Factor a source-owned primitive evaluator accepting the actual primitive row and unchanged graph authority, then call it from both real CsgSolid and Boolean operand routes. IfcMappedItem is not an allowed BooleanOperand member; mapping support at representation level does not authorize mapped-item operands.

Kernel output growth must obey the original cumulative logical value limit before retained output expands/publishes, not just a final count. Candidate/intersection/sort traversal and finite cleanup must remain cooperative. Empty intersections/full subtraction need explicit empty-result outcome policy; do not forge a degenerate triangle to represent emptiness. A disconnected regular Boolean UNION must not accidentally inherit the one-connected-ClosedShell rule unless that is an explicitly narrower supported family. Generic input solid validation and IFC declared-shell rules are distinct.

## Clipping Semantics and Primary Provenance

Pinned primary [IfcHalfSpaceSolid](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/IfcHalfSpaceSolid.htm) identifies the selected side using its surface normal: AgreementFlag TRUE means the subset the normal points away from; FALSE means the side it points into. For a +Z plane, TRUE therefore selects below and FALSE above. DIFFERENCE removes that selected subset. This is the critical sign test; do not infer it from a variable name.

The primary also requires the surface to split the domain into two and treats ordinary half spaces as unbounded. Infinite half spaces cannot be approximated by an arbitrary large box. A later exact plane-clipping evaluator must justify finite operand bounds, cap topology, orientation, error/overflow and wholly inside/outside/coincident cases; polygonal-bounded/boxed variants additionally need their genuine separate roles. No view badge or diagram supplies missing implementation.

The three exact pinned BooleanResult/Clipping/HalfSpace page URLs, original byte lengths and SHA256 observations are generated/ifc-boolean-primary.json, authored by ifc-boolean-primary.py. No production/shared Boolean kernel source was read or edited during its active rewrite.

## Test-First Adapter Audit Contract

Closed language-neutral inputs should include overlapping/touching/disjoint/equal Block pairs for all three operators; nested and repeated DAG operands; cycle/unknown/wrong-role/wrong-arity; source real-carrier errors; both tessellation Closed TRUE and FALSE/unset; valid finite mapping/reflection/SI; output and work frontier denial; empty result; source/caller mutation and initial/interior/final exact Error/undefined/typed failures. Independent actual IfcOpenShell full EXPRESS and normal geometry, Three volume/topology and existing test-owned Manifold comparator can corroborate authored finite positives with raw unsupported cases preserved. Actual public IFC→mesh→GLB/Khronos and canonical operand/dimension mutation/diff/inverse/STEP restore remain required.

No runtime dependency, new SDK public type, build change, native proof or full IFC/MVD Boolean support is proposed by this report.
