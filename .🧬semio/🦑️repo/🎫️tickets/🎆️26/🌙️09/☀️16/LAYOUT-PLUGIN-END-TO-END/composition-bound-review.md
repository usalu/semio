# Child Composition Container Bound Review

2026-10-28. Read-only exact three-line diff and current container/field-owner source review. No compiler, test, graph or production source edit.

## Result

No actionable defect found in lowering ChildComposition container from ReferenceSpaceMember+MemberFactory to SpaceMember+MemberFactory while retaining ReferenceSpaceMember on child_packs and child_head_packs. The diff changes only the struct bound and those two method where clauses; all semantic bodies remain identical.

## Exact Field Requirements

- ChildMemberRegistry<M> is structurally unconstrained over M; its fixed slots contain ChildMemberEntry<M>, and birth_demands/new are generic construction utilities.
- ArtifactFixedRegistry<Box<MountedPrivateChildGroup<M>>> requires the nested group type to be well formed. MountedPrivateChildGroup explicitly requires SpaceMember+MemberFactory, matching the lowered container exactly. Its existing Drop terminal-custody assertion remains under that same domain contract; no ReferenceSpaceMember requirement is hidden there.
- ChildMemberRetirement<M> is structurally unconstrained and its operational retirement impl requires SpaceMember, so the registry of those owners fits.
- ChildContentView/ChildContentRetirement and scalar counters, bools, pending CompositionPin vector and CompositionCoordinator carry no M-dependent reference-pack bound.
- No Drop implementation for ChildComposition was found in the bounded plugin source tree. The existing nested owner Drop/custody requirements are preserved.

The existing ChildComposition impl already uses SpaceMember+MemberFactory+static. birth_demands/new call the same generic registry/frame constructors as before and do not invoke a reference pack method. The bound correction does not add a factory, bypass original construction controls or change allocation/admission behavior.

## Reference-Pack Authority

child_packs calls member.envelope_pack_bytes; child_head_packs calls member.document_pack_bytes. Both now explicitly require M:ReferenceSpaceMember at the exact method boundary. Dialect/owner path projection, sorting, fault conversion and payload construction remain unchanged. A native SpaceMember-only caller can hold/run the composition but still cannot call either reference pack method without the stronger trait. No unchecked downcast, fake ReferenceSpaceMember implementation or implicit reference export route was added.

This is a structural container capability correction, not a relaxation of reference serialization. Generic consumers that actually request reference packs must prove the existing reference trait; any diagnostic there should be handled at that actual caller rather than strengthening every native composition container.

## Scope and Remaining Proof

The source review clears this exact bound split only. Native session56810/log13 remains coordinator-owned proof; no compile success is inferred. File attribution includes the new components path supplied by Rust, alongside its prior dispatch-import prerequisite, without attributing unrelated shared hunks. Findings sent to Rust and coordinator.
