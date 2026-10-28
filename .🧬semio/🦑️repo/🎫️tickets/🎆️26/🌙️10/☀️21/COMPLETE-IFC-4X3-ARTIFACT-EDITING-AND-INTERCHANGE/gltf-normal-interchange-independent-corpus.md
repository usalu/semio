# Authored Normal Interchange Independent Corpus

Ticket-only source/schema/feature/oracle preparation. No production normal implementation, public route success, native execution or runtime dependency is claimed.

## Frozen Inputs

| Input | SHA256 |
|---|---|
|gltf-authored-normal-cases.json|8a777cc8b6cd688bc087a4a69ca7dc4301b838430d0dd4236ee086e9756b5d69|
|gltf-authored-normal.schema.json|12d4478cf1cb5b336b99e6c17cd59b9fb0fab21f99e7b43b632991a05bd1cedb|
|normal-matrix-cases.json|3ea977a4f62a01cd11b4945b0ce0fa6e4d156553258df6f47c0cb30a5917c6b9|
|normal-matrix.schema.json|3ef4dc882e937c98aec49c09e6b7e6d85564d27c77f78fccf6d722ffb489bef8|

Features are gltf-authored-normal.feature and normal-matrix.feature. `gltf-normal-neutral-author.py` authors the complete current explicit data/schema, including closed-form expectedDirection vectors. Prior pre-direction matrix input/schema observations are retained under generated/gltf-normals/before-direction-*.json. The expected-direction author/check script is retained as research input, not a production migration.

Eight source cases: smooth-unit; Pn permutation; hard-edge6raw normals→4coordinate rows/2triangles; nonunit normalize; referenced zero explicit whole NORMAL omission; unused zero retains data; extra valid unused normal; missing Normals. Every row has explicit points/triangles/normals/PnIndex/ClosedFALSE and output status/count expectations. Raw CoordIndex selects normals before point mapping. Exact pair identity is(point index,normal index), not equal numeric normal/position values. Source cardinality remains separate from GLB attribute cardinality.

Ten matrix rows: identity, Z90rotation, reflection, nonuniform scale, shear, extreme-axis, extreme-balanced, extreme-reflected, singular and zero normal. Matrices explicitly store COLUMNS. Every row carries expectedDirection as an owned triple or null: null singular means reject matrix; null zero means retain representable zero without claiming a nonzero direction. The source-zero interchange omission policy is distinct from malformed geometry. The direction formula is normalize(A^-T n), with a common-scale cofactor implementation allowed. Expected moderate formulas include[6,4,3]/sqrt61 and[8,12,21]/sqrt649, not compiler-generated production output.

## Primary Authority

The current [Khronos glTF2.0 specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#meshes), observed version2.0.1/commit8e798b02d254cea97659a333cfcb20875b62bdd4, defines NORMAL as a normalized VEC3 float attribute and requires consistent vertex attribute counts. It also distinguishes absent normal handling from authored data. The [Three Matrix3 documentation](https://threejs.org/docs/pages/Matrix3.html#getNormalMatrix) defines the normal transform as inverse transpose of the linear matrix. glTF float32 encoding is a transport boundary; sensible IFC finite ParameterValue triples need not already be unit directions. Existing pinned IFC source and normal formal-domain audit remain separate.

## Actual Independent Results

`gltf-normal-independent-oracle.ts` keeps Three and the validator behind a test-owned interface returning only arrays/counts. Strict Ajv2020 validates both closed corpora. All8 independently authored GLBs pass the already available Khronos gltf-validator2.0.0-dev.3.10 with zero errors. Three0.182.0 loads all8 with expected POSITION/NORMAL counts and retained authored directions within2e-7 after Float32 encoding. The hard-edge output has6 vertices, not4. Referenced-zero and missing-normal GLBs omit NORMAL; unused-zero/extra rows retain complete usable referenced attributes. This is a constructed output oracle, not successful output from the unfinished own normal exporter.

`normal-matrix-independent-oracle.py` uses existing test-owned mpmath1.3.0 at1200 decimal digits, interpreting actual binary64 input values. Eight invertible/nonzero cases produce directions and singular/zero have no nonzero direction. Authored expected vectors agree within4ULPs, including tiny representable1e-300. The five moderate cases additionally agree with actual Three inverse-transpose within8 binary64 epsilons. Generated matrix-mp.json retains90-digit directions and determinant signs. Three is deliberately not used as the extreme-range authority.

`gltf-normal-physical-oracle.py` uses installed IfcOpenShell0.8.5 behind its own owned observation interface. All8 actual authored ADD2 physical graphs parse and full EXPRESS emits zero statements. Physical.json retains exact source normal values, PnIndex, source IDs and file hashes. Referenced zero and nonunit source normals are formally admissible; sensible shading and omission/normalization policy are still separate.

No install was required. The cached validator package/index.js was present and its actual API ran. Original unchanged Khronos sample provenance remains commit15cd02d304e54cb42a4db4eefe719f76190f4b68. Fresh bounded validation/loading reports BoxInterleaved24positions/24normals and NegativeScaleTest3958positions/3958normals, both zero validator errors/warnings. Original SHA256 values match the existing manifest: Box b2ae631f118f1d13f829cdf9d9dc0fe7cb582de20b8c51d17f81f77a1cbf290c; NegativeScale ea8c41fd0630f03adcf7a5c06bf86d7b850fa510b6f07cc51eb1fccc205a50f2. Original files were referenced in place, not retagged/copied as sources.

## Exact Cofactor Design Review

For invertible A, the actual cofactor matrix C satisfies A^-T=C/detA. Thus normalization can use C*n followed by the exact determinant sign. Do not accidentally use the adjugate instead of C or apply reflection twice. Column/normal mantissa-exponent decomposition and common powers of two cancel in normalized direction; no per-vertex9matrix parse is needed for a prepared transform.

Final conversion MUST preserve relative component exponents against one common maximum component bit length. Normalizing each component independently destroys ratios; shifting every component to a single top53 cutoff can prematurely erase small but representable directions. Per-component significand plus relative exponent is appropriate. Binary64 exponent limits bound intermediate cofactor/dot integers to roughly4200/6300bits (fixed arithmetic, not source-size work), with ordinary cases much smaller. The public implementation still needs its own test-first cancellation/output/grant, singular/nonfinite and Float32 collapsed-position laws; these oracles supply no original funding/host lifetime certificate.

All scripts remain ticket inputs; generated files are confined to generated/gltf-normals. All processes terminated. No full glTF/IFC/MVD/native or bitwise source-normal fidelity certificate is claimed.

## Supplemental Fixed Arithmetic Edges

`normal-matrix-edge-independent.py` independently observes3 extra inputs through mpmath1.3.0 at1600 decimal digits; it does not modify the frozen10 corpus. Generated matrix-edge-mp.json records exact binary64 input columns and90-digit directions. Identity with minimum subnormal nonzero normal yields[1,0,0]; uniform1e-300 diagonal yields normalized[1,1,1]; a1e200 nearly parallel pair separated by actual Math.nextafter-equivalent one ULP yields approximately[-.7071067811865475,.7071067811865476,0]. These distinguish sourcezero from squared-length underflow, determinant underflow and large cofactor cancellation. They are supplemental input/oracle evidence, not a production failure assertion or independent resource proof.
