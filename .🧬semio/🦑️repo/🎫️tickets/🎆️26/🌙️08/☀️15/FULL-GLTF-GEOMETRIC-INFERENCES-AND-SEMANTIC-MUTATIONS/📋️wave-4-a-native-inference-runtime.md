# Wave 4-A Native Inference Runtime

## Outcome

Implemented the native, domain-neutral, type-erased artifact inference runtime and registered GLTF end to end.

- Canonical snapshot pack bytes enter a type-erased service.
- The GLTF bridge decodes `GltfSnapshot`, invokes `GltfBuilder` through `ArtifactInferrer`, and encodes the result with the existing canonical GLTF binary inference codec.
- Runtime metadata records owner, artifact kind, artifact schema and version, document schema and version, inference schema and version, algorithm version, and policy version.
- `ArtifactDeclaration` owns executable inference services alongside inference schema descriptors and performs owner/kind/schema consistency checks before registration.
- Registry enumeration is deterministic through `BTreeMap` ordering.
- An identical metadata/function-pointer registration is idempotent. A differing function, owner, schema, or version for the same `(artifact_kind, inference_schema)` key returns `ArtifactInferenceRegistrationError`; declaration registration fails loudly on that error.
- Cold execution copies the service entry out of the global registry before inference, so the registry read lock is not held during geometry work.

## Public API

- `ArtifactInferenceServiceMetadata`
- `ArtifactInferenceExecutionError`
- `ColdArtifactInference`
- `ArtifactInferenceService`
- `ArtifactInferenceServiceKey`
- `ArtifactInferenceRegistrationError`
- `ArtifactInferenceServiceRegistry`
- `register_artifact_inference_service`
- `artifact_inference_service`
- `list_artifact_inference_services`
- `infer_artifact_cold`
- `ArtifactDeclarationBuilder::inference_services`
- GLTF `gltf_inference_service`

The adjacent Wave 4-B wire API is re-exported from the same shared plugin component after coordination with its owner; its implementation is not part of this lane.

## GLTF Identity

| Field | Value | Version |
|---|---|---:|
| owner | `stdio` | — |
| artifact kind | `s.stdio.gltf` | — |
| artifact schema | `s.stdio.gltf` | 1 |
| document schema | `stdio.gltf` | 2 |
| inference schema | `s.stdio.gltf.inference` | 2 |
| algorithm | GLTF geometric inference | 1 |
| policy | GLTF analysis policy | 1 |

`artifact_kind()` and the artifact declaration now use the explicit canonical `GLTF_ARTIFACT_KIND_ID` rather than conflating the artifact-kind lookup with the document schema.

## Inline Tests

1. Registry order independence and identical-registration idempotence.
2. Rejection of a function-identity conflict with identical metadata.
3. Rejection of a version conflict for the same lookup key.
4. Cold GLTF snapshot-pack execution.
5. Decoded canonical binary result equality with direct typed `ArtifactInferrer` output.
6. Repeat cold execution byte determinism.

## Validation Evidence

### Passed

```text
cargo test -p semio-framework-plugin artifact_inference_registry --lib
2 passed; 0 failed
```

```text
cargo test -p semio-s-plugin-stdio cold_native_inference --lib
1 passed; 0 failed; 3450 filtered out
artifacts::gltf::component::tests::cold_native_inference_decodes_snapshot_pack_and_matches_typed_result ... ok
```

The focused GLTF test confirms runtime behavior, not just compilation: canonical pack decode, typed builder inference, canonical binary encode/decode, decoded parity, and deterministic bytes all executed.

### Nx Gate

```text
bun nx run '@semio-tech/stdio-plugin:test-quick'
```

The Nx gate compiled `semio-framework-plugin` and `semio-s-plugin-stdio` successfully. Both attempts then exceeded the existing 30-second `cargo nextest` quick budget while blocked on the shared Cargo build-directory lock during concurrent agent validation. The focused Cargo tests above completed and passed after the lock cleared. No compile or test assertion failure was reported by Nx.

## Exact Production/Test Files Changed

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
   - native service metadata, errors, entry, deterministic registry, global API;
   - inline registry tests;
   - `ArtifactDeclaration` storage, builder method, validation, and registration;
   - public re-exports.
2. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️component.rs`
   - explicit identity/version constants;
   - GLTF type-erased cold bridge;
   - declaration registration;
   - inline cold execution/parity test.

No GLTF geometry, mutation, inference facet, inference codec, snapshot codec, schema leaf, store, glue, WIT, or UI file was modified by Wave 4-A. The existing framework schema/store APIs were sufficient and therefore were not widened.

## Ownership Boundary

- Wave 4-A owns native executable service registration, deterministic duplicate handling, declaration wiring, and GLTF cold execution.
- Wave 4-B owns serialized wire request/result contracts, WIT guest/host exports, descriptor transport, and projection/cache invalidation.
- Geometry owns inference computation and the binary inference codec; this lane only calls those public APIs.
