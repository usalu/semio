# Authored Normal Interchange Independent After Audit

All13 owned+23 reference hashes match before and after (`generated/gltf-normal-independent-source-before.json`, `...-after.json`). No additional confirmed P1/P2 was found in this bounded source/runtime review. Root may release the13 source files. All independent processes terminated.

## Actual Execution

- New normal/matrix85 tests pass,0fail,16701 assertions across2 files,1.437s (`generated/gltf-normal-independent-tests.txt`).
- Retained TFS/PFS/mapping/glTF/circle265 tests pass,0fail,7069 assertions across5 files,5.47s (`generated/gltf-normal-independent-affected.txt`).
- `gltf-normal-actual-oracle.ts` uses the ACTUAL IFC fixture→ifc4x3ToGltf→writeIfcGltfGlb producer for all8 frozen source cases. Cached Khronos validator2.0.0-dev.3.10 reports zero errors for every actual GLB. Independent Three loading verifies per-corner positions and authored normal directions/counts against the prior independent source oracle, not just accessor presence. Exact actual output GLBs/SHA/counts/indices/diagnostics are under generated/gltf-normals-actual/oracle.json. Hard-edge emits6 pairs; referenced-zero/absent emit no NORMAL, while unused zero/extra normal retain data. All actual corner positions match exact Float32 expectation; direction error stays within2e-7.
- Prior constructed-only GLB results, MP1200/1600 normal direction references, IfcOpenShell0.8.5 full EXPRESS0 for8 source graphs and unchanged official sample provenance remain in gltf-normal-interchange-independent-corpus.md. They were not overwritten or represented as new actual-producer evidence. The85 replay includes actual import of24/3958 original Khronos normals with real world matrices.

## Source Correctness

IfcNormalTransform captures exact finite affine cofactors once. Cyclic minors are the cofactor matrix C, with determinant sign applied once after C*n direction scaling. Exact binary64 mantissa/exponents avoid early determinant/cofactor overflow, underflow and cancellation; per-component top53 significands with exponents relative to one maximum preserve ratios and tiny representable directions. Zero input stays zero; nonfinite/singular/projective carriers refuse. Current explicit minimum-subnormal/uniform-tiny/nearparallel laws pass. This is a bounded fixed arithmetic implementation, not arbitrary precision output or a shader/Float32 error certificate.

TFS capture preflights positions, point indices, original normal triples and normal corner identities before growth. Normals are indexed by raw CoordIndex, while coordinates use optional PnIndex. Original source/STEP values remain unchanged. Private exact-coordinate closed topology stays distinct from this authored normal domain. Central reflected world publication swaps both point and normal corner indices exactly once, while normal direction uses proper A^-T. Positive SI unit scaling cancels from normalized direction.

Export expands exact(point-index,normal-index) pairs, admits index bytes and each24-byte new pair before growth and checks final cumulative byte limits. A referenced zero emits explicit per-mesh omission rather than a fabricated normal; unused zero has no erasure effect. Nonunit IFC normals normalize only derived output. Original logical41/42, private pair167/168 and public8-product1343/1344 boundaries pass. Float32 bounds use encoded rounded values and actual encoded triangles are checked for finite/nondegenerate output before publication; the genuine collapse regression passes. No global open-surface intersection constraint is introduced.

Import requires the actual dense float VEC3 NORMAL/count matching POSITION, finite normalized nonzero direction under explicit1e-5 tolerance, actual composed node transform and inverse basis[x,-z,y]. Closed remains unset. Existing unsupported material/other semantics remain separately diagnosed. Original control scope rethrows caller Error/undefined before unsupported-domain catches; actual new phases plus old position callbacks pass initial/interior/completion failure/abort and source-immutability laws.

Canonical normal edit, central mutation/diff inverses, exact STEP and restored GLB, reflected/nonuniform coupled corner identities, malformed normal7 and official sample hashes pass. The optional normal lists remain finite ParameterValue triples, not a fabricated IFC unit-vector/cardinality equality rule.

## Boundary

This closes the frozen authored-normal transport slice and tested original control/logical-byte limits. Existing material/tangent/skin/animation and generic IFC semantic losses are not repaired by normal transport. No native implementation/compiler/runtime/receipt, whole glTF/IFC validity, universal matrix-rounding precision, complete view or certification claim is made. Source physical evidence and actual emitted GLB conformance are separate observations.
