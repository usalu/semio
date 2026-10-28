# Authored Normal Interchange Plan

The admitted IFC triangulation now checks original normal rows, while current `gltf.ts` still drops authored normals on export and import with explicit diagnostics. This is the next root feature; the TFS admission checkpoint remains independently closed source evidence, not a normal-transport claim. Actual public tests first produced0pass/3fail/25assertions for two untouched Khronos NORMAL imports and an actual authored IFC NORMAL export. A fourth red proves a binary64 triangle with Three area0.5 collapses to area0 after float32 rounding. Additional existing position-stage observer laws expose swallowed original Error/undefined failures on import; abort remains separately tested. All these precede normal production changes, with logs in `🗑️generated/gltf-normals-*-red.txt`.

## Primary and Existing Authority

The [Khronos glTF2.0.1 specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#meshes) identifies NORMAL as normalized float VEC3 and requires equal attribute accessor counts. Position bounds describe the actual stored float values. Missing normals permit renderer flat-normal generation. The observed published page identifies commit8e798b02d254cea97659a333cfcb20875b62bdd4. Its requirements differ from IFC's finite IfcParameterValue triples; zero and nonunit IFC triples remain formally admissible in the existing neutral corpus.

[Three Matrix3.getNormalMatrix](https://threejs.org/docs/pages/Matrix3.html#getNormalMatrix) documents inverse transpose of the linear matrix. Installed Three0.182.0 is a test-only independent moderate-range oracle. No external runtime type or matrix class will enter the owned interface. Existing official Khronos sample sources/provenance, loader oracle and real IFC STEP sources remain available for end-to-end tests.

## Owned Representation and Transformation

Retain source-coordinate output and add optional first-party normal vectors plus a separate per-corner normal-index roster to the internal triangle mesh. This preserves IFC's distinct domains: raw CoordIndex addresses Normals; PnIndex optionally maps points. Shared point coordinates with different raw normal identities must survive GLB. Never merge pairs by rounded position or normal value. The original source snapshot and STEP remain unchanged.

The pure owned normal transform consumes an owned affine matrix and triple. It uses inverse-transpose direction followed by normalization. Translation and uniform positive SI scaling do not change direction. A reflected frame uses the same covector transform, while the existing central orientation stage swaps both coordinate and normal corner indices exactly once. Zero source normals remain zero in the derived mesh; they cannot become fabricated directions.

For extreme finite columns, normalize each matrix column before cofactors and rescale the input normal components by those column magnitudes. Relative log ratios may avoid premature overflow/underflow in division. Multiply normalized cofactors by the original determinant sign to recover inverse-transpose direction. Singular, nonfinite or unrepresentable results refuse the operation explicitly. Ordinary cases compare against actual Three; extreme cases need a separate high-precision test-owned reference.

## Actual Exchange

IFC export expands only the derived GLB vertex domain by exact (coordinate-index,normal-index) pairs when authored referenced normals are usable. NORMAL and POSITION accessors then have equal counts. Preserve corner order and existing world Z-up→glTF Y-up rotation. A referenced zero normal causes an explicit per-mesh omission diagnostic while represented geometry remains available; unused zero normals do not erase valid referenced data. Normalize nonunit directions without claiming bitwise semantic losslessness. Existing generic IFC semantic loss diagnostics must describe the final behavior accurately.

glTF import admits the actual optional dense NORMAL float VEC3 accessor under existing accessor limits and validates count and normalized finite nonzero directions. It transforms normals using the actual composed node matrix, applies the inverse Y-up basis, and authors actual IfcTriangulatedFaceSet.Normals beside coordinate rows. It preserves Closed unset and central reflected triangle handling. Material/tangent/texture/animation limitations remain explicit and separate.

Output logical grants must charge normal triples and per-corner normal indices before growth. Export pair growth and byte buffers must use the original byte grant. Float32 positions must be checked after rounding so an otherwise finite binary64 triangle cannot silently collapse in encoded output. Private output checks do not assert global solid/intersection validity for open meshes.

## Test-First Scope

Author closed neutral data/schema/feature before implementation and obtain real missing-NORMAL red through public IFC→GLB and glTF→IFC routes. Include Pn many-to-one hard edges, raw normal index ordering, reflected/nonuniform mappings, source nonunit/zero/unused zero, canonical normal edits with both inverses and exact STEP, complete accessor counts/bounds, unsupported malformed normal carriers/count, float32 collapse, output limits and original error/undefined/abort at actual new phases.

Reuse original Khronos Box/other retained sample assets and compare loader normal matrices per actual world vertex/corner. Use the existing third-party interfaces for physically authored IFC values and official glTF validation if already available. No new runtime dependency, fake complete-spec certificate, normal recomputation that overwrites authored directions, or build/compiler-policy change belongs to this slice.
