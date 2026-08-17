# Current Program Status

## Acceptance Rule

A capability is counted only when its schema-owned leaf has executable Rust and TypeScript behavior, complete phase/result facets, a single shared vector executed by both implementations, typed rejection and stale-state coverage, direct diff/inverse or inference laws, descriptor registration, and a current-tree runtime gate. Physical files without those properties are candidates, not support.

## Framework Boundary

- Artifact definitions, strict registries, codec/resource contracts, request-aware inference execution, fallible store replay, and typed contribution surfaces are implemented in the active combined tree.
- `cargo check -p semio-framework-plugin` passes after the mutation-outcome and merge-command migrations.
- Focused plugin dispatch and merge-command tests each pass 1/1. `SetMergePolicy`, `ResolveConflict`, and `ReadConflicts` now use the authoritative store and canonical report/conflict payloads; invalid ordinals reject without fallback.
- `Mutation::diff` plans through a typed outcome. The generic `MutationDiff::apply` contract is now actively migrating to crate-owned `MutationApplyResult<P>` with structured code/message/target rejection. The framework/kernel production and test boundaries are intentionally mid-migration and Cargo is paused; no fallback, infallible adapter, or production `expect` is accepted.
- The framework `ArtifactBuilder::mutate` contract, its derived wrapper/macro, and all 54 non-stdio manual builders now return the intact `MutationOutcome<Self::Diff>` after applying `outcome.diff()`. Static/rustfmt parse checks pass; the current combined Cargo gate is queued behind the stdio half of the same breaking migration.
- `ArtifactEnvelope` now carries outcome messages and conflicts through SPR, reconstruction, remote merge, and the host backbone shape. Atomic quarantined-batch resolution, empty-store snapshot validation, stable operation identity, causal reconstruction, generation invalidation, strict history decoding, cursor validation, HLC advancement, receipt edit identities, full text operation metadata, durable cursor persistence, and conflict actor canonicalization have focused adversarial coverage.
- The frozen runtime V3 boundary is green: kernel library `952/952`, sync-feature check, exact Rust/TypeScript wire fixture `1/1`, host-full no-run, and the host backbone/workflow text+binary round-trip `1/1`. Source formatting and diff hygiene pass. The fallible `MutationDiff::apply` migration deliberately supersedes this compile boundary until its repository-wide adoption is complete.
- Non-glTF stdio has no remaining bare `Mutation::diff`, bare public apply-helper return, legacy builder signature, direct outcome-as-diff application, accidental double projection, or corruption marker. The stdio library compiled after that migration, and its complete baseline executed `3,436` passed / `75` failed / `3` ignored. Functional repairs are proceeding by artifact family without weakening fixtures or expectations.

## glTF Inferences

- The public aggregate computation file is removed.
- Sixty-seven physical inference leaves and service identities exist.
- Rust geometry behavior is split behind leaf-owned result construction; TypeScript execution is being remediated leaf by leaf.
- No inference leaf is accepted until its Rust and TypeScript shared-vector gates can run on the restored mutation/runtime boundary. Twenty TypeScript geometry leaves have executable kernels, but their shared typed result/vector gates remain incomplete.

## glTF Mutations

- The authoritative domain matrix contains 222 semantic commands.
- No glTF mutation leaf is currently accepted. The four bind/unbind relation candidates now use independent inverse planners, validate explicit references and indices, recompute concrete paths, reject forged serialized paths, and pass all four TypeScript contracts. They remain unregistered until their Rust contracts execute after the stdio outcome migration.
- `create-scene` now uses command-local Rust/TypeScript mechanics, exhaustive full-sequence stale and inverse guards, phase-unique schemas, a shared three-case vector, the common schema-owned descriptor, exact Rust glue mounting, an open TypeScript registry, and generic descriptor envelopes for both text and binary transports. Its Bun vector/registry/transport verifier and targeted formatting checks pass. The serialized Rust integration gate is pending the repository-wide fallible `MutationDiff::apply` migration, so it is still not counted as accepted support.
- `change-material-alpha-mode` and `change-material-double-sided` now carry expected pre-state/forward-state values, reject stale and forged paths, use one shared Rust/TypeScript vector each, and expose schema-owned leaf descriptors. They are mounted candidates, not accepted support, until the current Rust registry/codec/runtime gates execute.
- The three former framework-plugin API drift blockers are fixed and the plugin check/focused command test pass. The next current-tree blocker is the deliberate repository-wide typed mutation-application transition described above.
- Runtime acceptance remains pending until the descriptor registry mounts these leaves, verifies concrete paths, propagates typed apply rejection, and passes the current-tree Rust gates.
- The legacy closed 28-variant dispatch, fixed binary tags, and payload switch are being replaced with a descriptor-driven registry. Rejected candidate folders are not registered.

## Remaining Program

- Finish and verify all glTF mutation and inference leaves.
- Migrate every artifact helper to consume typed mutation outcomes, then migrate `MutationDiff::apply` itself to a crate-owned typed `Result` with no infallible adapter.
- Execute artifact waves 1 through 11 for the remaining 35 catalog artifacts, including complete STEP/EXPRESS, IFC, CAD, document, image, media, Semio, and EPW runtimes.
- Run independent frozen-tree standards, architecture/OCP, runtime/security, fuzz, performance, schema-parity, WIT, cross-platform, and policy audits before ticket closure.
