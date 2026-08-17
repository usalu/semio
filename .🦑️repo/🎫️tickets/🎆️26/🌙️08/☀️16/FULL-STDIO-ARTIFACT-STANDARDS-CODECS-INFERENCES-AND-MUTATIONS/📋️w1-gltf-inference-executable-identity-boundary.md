# glTF Inference Executable Identity Boundary

## Current-tree finding

The glTF artifact definition declares 67 inference leaves as `implemented` with `executable_registration: true`. The artifact declaration independently registers 67 real `ArtifactInferenceService` values, each containing the canonical inference-schema ID and its request-aware function pointer. The stdio definition registry nevertheless implements `executable_mappings` as an unconditional empty map and therefore rejects the schema before those services can satisfy the declaration parity gate.

This is a real identity-plumbing defect. It must not be hidden by marker `fn()` values, synthesized hashes, an always-present identity, or a downgrade while the executable services remain present.

There is also an independent status-ledger defect. The source currently marks all 67 leaves `implemented` and executable, while the checked-in ledger test still expects the earlier 15 declared / zero registered / zero implemented state and the umbrella acceptance record says TypeScript/shared-vector closure is incomplete. Executable registration, implementation, and verification are separate axes; the validator currently collapses registration into implementation by requiring `executable_registration == (implemented || verified)`. That equality must be removed. The final per-leaf status must be derived honestly from completed Rust and TypeScript/vector evidence, not chosen merely to make registry parity pass.

## Required contract

`ArtifactInferenceService` is the authoritative executable value. It must expose its process-local executable identity through the same crate-owned `ArtifactExecutableIdentity` used by `ArtifactCapability`. The identity must be derived from the service's actual `ArtifactInference` function pointer; callers must not cast or depend on that external function type.

The minimal owned API is:

```rust
impl ArtifactInferenceService {
    pub fn executable_identity(&self) -> ArtifactExecutableIdentity;
}
```

The stdio glTF assembly must then build a map from each service's `metadata().inference_schema` to `service.executable_identity()`. Definition assembly must compare that exact key set with the 67 schema-owned executable registrations and attach the matching identity to each inference capability. Duplicate service IDs must reject rather than overwrite.

## Acceptance

1. Exactly 67 schema inference IDs, service metadata IDs, inference descriptors, and executable-identity keys match.
2. Every identity is derived from the actual request-aware service function pointer.
3. Duplicate IDs and missing/extra rows reject with a typed assembly error before registry mutation.
4. No marker executable, address hash, compatibility alias, default, or identity synthesis exists.
5. The stdio runtime-capability exact-set parity gate and inference text/binary service round trips pass on the current tree.
6. Registration, multi-implementation completion, and verification remain independently counted; no validator equality conflates them.

## Scheduling

The framework plugin component is currently part of the repository-wide fallible `MutationDiff::apply` migration. This identity patch must land only after that writer freezes the shared component, then be verified serially with the stdio gates.
