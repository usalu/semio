# glTF Import Materialization Independent Audit

Read-only source checkpoint of io/gltf.ts new materialization loops, gltf-materialization neutral cases/schema/test, source-isolation phase expansion and the existing genuine Khronos/Three suite. No new confirmed implementation defect in the reviewed loops. No production edit or duplicate full sweep was performed.

Position materialization fuses world transform, Y-up to IFC Z-up conversion and exact IFC scalar construction in one original-batch loop. The coordinate order remains [x,-z,y], matching the previous adapter and independent Three projection. Its completion checkpoint occurs before the positions are attached to an IFC point-list entity. Transform overflow/nonfinite values still refuse through ifcReal.

Explicit indices use existing private accessor rows; implicit indices are their ordinal. No second full numeric index array/default list is built. The validation loop checks every index is safe, nonnegative and below the private position count, before any 1-based BigInt triangle index is emitted. Divisibility by three is checked first. JavaScript array lengths bound the index arithmetic well below safe-integer overflow; adding one remains safe after the range check.

Triangle emission reads indexAt(index*3+offset) directly with offsets [0,1,2] or [0,2,1] for reflected transforms. This reproduces the prior winding swap without a whole reflection pass. The fixed three-item map is bounded independently of source mesh size. Captured original batchSize governs positions, index validation and triangles, including a completion checkpoint for each. The graph remains local; cancellation is rethrown by the catch and never returns its partially accumulated graph. Final import completion is awaited before artifact publication.

The authored neutral cancellation cases exercise both explicit and implicit paths at batchSize=1 and a later interior prefix of all three phases, rebind caller control and abort the original signal, assert no completion/publication and verify caller input unchanged. At review time these cases did not independently exercise reflected-route cancellation or completion-prefix cancellation; those are coverage suggestions, not observed production failures, because the completion checkpoints and reflected emission share the inspected control path. Root was informed before finalizing this report.

Existing official sample tests load pinned physical Khronos files with SHA checks, parse independently through Three.GLTFLoader and compare every position plus the full index sequence, including determinant-based reflected winding, exact mesh/vertex/triangle counts and bounds. They cover the actual nonindexed/indexed and NegativeScale paths. Root owns their current focused execution evidence.

This auditor reran the original gltf-source-mutation-probe.ts after materialization changes, including its independent actual Three parse of physical Box GLB: caller byte mutation at accessor completed=1 still leaves emitted maximum [0.5,0.5,0.5], matching untouched Three geometry within approximately 1e-16 transform error. The same probe confirms caller metadata mutation leaves Name=Mesh. Generated probe files now represent corrected after-results.

Metadata attribute lookup/diagnostic scans, IFC-to-glTF final indices/buffer emission and GLB JSON synchronous primitives are separate remaining frontiers. Their plan is gltf-export-json-frontier-audit.md; this report does not imply those loops became cooperative. Native owners/gates are unchanged and authentic compiler ingress still refuses before rustc.

## Authored coverage expansion

Root expanded the neutral matrix to three routes (indexed, implicit, reflected indexed), three phases and both interior/completion boundaries, for nineteen tests including schema admission. The earlier reflected/completion coverage suggestions are now addressed by authored tests. Current run evidence is pending parent completion; this update does not claim that matrix has executed successfully.

Parent completed the stable registered checkpoint after expanded cancellation coverage: 603 pass, 12 gated skips, zero failures, 12,486 assertions across 38 suites in 128.52 seconds; focused strict checking passes. This is parent-owned TypeScript/reference evidence, not native Rust execution.
