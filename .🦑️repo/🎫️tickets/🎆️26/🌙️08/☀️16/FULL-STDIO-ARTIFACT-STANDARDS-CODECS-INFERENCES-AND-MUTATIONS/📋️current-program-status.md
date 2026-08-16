# Current Program Status

## Acceptance Rule

A capability is counted only when its schema-owned leaf has executable Rust and TypeScript behavior, complete phase/result facets, a single shared vector executed by both implementations, typed rejection and stale-state coverage, direct diff/inverse or inference laws, descriptor registration, and a current-tree runtime gate. Physical files without those properties are candidates, not support.

## Framework Boundary

- Artifact definitions, strict registries, codec/resource contracts, request-aware inference execution, fallible store replay, and typed contribution surfaces are implemented in the active combined tree.
- `cargo check -p semio-framework-plugin` passed after the mutation-outcome contract migration.
- `cargo test -p semio-framework-plugin --lib dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying` passed one focused test and filtered 216.
- `Mutation::diff` plans through a typed outcome, but the generic `MutationDiff::apply` trait is still infallible. A current source census finds 125 Rust files implementing `MutationDiff` and 964 `.apply(&...)` call sites. Its repository-wide typed-application migration remains a P0 contract item; artifact runtimes must not hide this with fallback/no-op behavior.
- The history schema now carries outcome messages and conflicts, but `ArtifactEnvelope` does not yet carry those store-owned ledgers into `print_document_spr`; the compile bridge currently emits empty lists. Durable message/conflict persistence remains P0 and must be closed before the runtime gate.

## glTF Inferences

- The public aggregate computation file is removed.
- Sixty-seven physical inference leaves and service identities exist.
- Rust geometry behavior is split behind leaf-owned result construction; TypeScript execution is being remediated leaf by leaf.
- No inference leaf is accepted until its Rust and TypeScript shared-vector gates can run on the restored mutation/runtime boundary.

## glTF Mutations

- The authoritative domain matrix contains 222 semantic commands.
- No glTF mutation leaf is currently accepted. The four bind/unbind relation candidates were demoted when a forged-path audit showed their diff/inverse application trusted serialized `touchedPaths` instead of validating the concrete path recomputed from command fields.
- `create-scene` now carries collection-count, default-scene, and insertion-anchor preconditions, but remains unaccepted: its command root still defines a command-private adapter rather than the schema-owned descriptor contract, and its shared vector does not execute forged touched-path rejection in Rust and TypeScript.
- `change-material-alpha-mode` and `change-material-double-sided` now carry expected pre-state/forward-state values, reject stale and forged paths, use one shared Rust/TypeScript vector each, and expose schema-owned leaf descriptors. They are mounted candidates, not accepted support, until the current Rust registry/codec/runtime gates execute.
- The mounted-candidate Rust check is currently blocked before stdio by three framework-plugin API-drift errors: missing `AppFrame::Invocation.messages`, missing `AppFrame::Error.report`, and a removed `ArtifactStore::snapshot_with_conflicts` call. The framework remediation lane owns that boundary; no glTF pass is claimed from the blocked run.
- Runtime acceptance remains pending until the descriptor registry mounts these leaves, verifies concrete paths, propagates typed apply rejection, and passes the current-tree Rust gates.
- The legacy closed 28-variant dispatch, fixed binary tags, and payload switch are being replaced with a descriptor-driven registry. Rejected candidate folders are not registered.

## Remaining Program

- Finish and verify all glTF mutation and inference leaves.
- Execute artifact waves 1 through 11 for the remaining 35 catalog artifacts, including complete STEP/EXPRESS, IFC, CAD, document, image, media, Semio, and EPW runtimes.
- Run independent frozen-tree standards, architecture/OCP, runtime/security, fuzz, performance, schema-parity, WIT, cross-platform, and policy audits before ticket closure.
