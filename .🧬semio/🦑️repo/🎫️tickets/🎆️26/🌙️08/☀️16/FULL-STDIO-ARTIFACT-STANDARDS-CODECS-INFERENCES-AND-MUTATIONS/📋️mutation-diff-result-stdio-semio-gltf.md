# Mutation Diff Result Adoption for Semio and GLTF

## Scope

- The 19 Semio v1 subset diff implementations and the two GLTF diff implementations.
- Shared Semio indexed/name-keyed triple preflight.
- GLTF immutable mutation-dispatch propagation and legacy sparse-diff structural preflight.

## Contract

- Every owned `MutationDiff::apply` returns `protocol::MutationApplyResult<Snapshot>`.
- Candidate snapshots are local until all structural preflight succeeds.
- Missing, duplicate, overlapping, and out-of-range persisted collection operations are rejected.
- Semio subset-kind mismatch is a typed `mutation.apply.kind-mismatch`, never an unchanged-base success.
- A failed Semio absorb is represented by `SemioDiff::Rejected`; later absorb cannot manufacture success.
- GLTF descriptor application maps its typed registry error into `MutationApplyError` without an infallible bridge.

## Source Changes

- Added shared `validate_indexed_triple` and `validate_named_triple` functions.
- Replaced Presentation's duplicated triple types with the authoritative shared schema types.
- Added recursive Semio Value kind, collection, target, and insertion-position validation.
- Added top-level GLTF collection preflight and default-scene reference validation.
- Extended Semio diff text/binary codecs with a canonical rejection variant.

## Static Evidence

- `rustfmt --edition 2021 --check` passed for all 21 implementations and the shared triple schema.
- Scoped `git diff --check` passed.
- Runtime/Cargo evidence is pending the serialized repository-wide adoption boundary.

## Open Verification

- Compile-led conversion of affected test and production consumers.
- Focused malformed persisted-diff rejection tests.
- Full stdio library gate after all 58 stdio implementations are migrated.
