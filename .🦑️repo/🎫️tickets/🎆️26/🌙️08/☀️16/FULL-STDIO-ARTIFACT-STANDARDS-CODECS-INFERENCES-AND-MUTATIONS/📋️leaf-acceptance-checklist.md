# Leaf Acceptance Checklist

## Mutation Leaf

A mutation leaf is accepted only when every item is true.

- The command has one canonical `.v1` ID and one cohesive semantic payload; no snapshot, whole-record replacement, option bag, generic collection operation, `Set*`, or no-op vocabulary is present.
- The command folder owns typed mutation, command-specific sparse diff, and base-derived inverse phases in Rust, TypeScript, JSON Schema, GraphQL, and Protobuf.
- Validation rejects missing indices/IDs, invalid references, duplicates, invalid order positions, non-finite values, no observable change, and command-specific invariants before mutation.
- Direct diff planning carries enough expected pre-state to reject sequential application to a stale after-state.
- Inverse reconstruction captures every authoritative value/reference changed by the forward operation and rejects when the current state no longer equals the expected forward result.
- Diff and inverse application recompute their concrete touched paths from typed fields and reject forged serialized paths; registry envelopes are compared with the recomputed paths.
- Reference repair is command-owned, exhaustive over the snapshot relationship matrix, deterministic, and restored exactly by inverse.
- A single canonical JSON vector is imported/deserialized and executed by Rust and TypeScript. The tests run mutation apply, diff derive/apply, inverse derive/apply, forward-stale, inverse-stale, forged-path, malformed/index/reference, serialization, and exact restoration laws.
- A command-root descriptor delegates only to the leaf phases. The schema mutation root assembles descriptors; runtime dispatch and codecs contain no command IDs, payload inspection, or command behavior.
- Text/binary transport round trips the generic versioned envelope, rejects unknown IDs/versions/phases/trailing data/budget overflow, and canonicalizes payload representation.
- Current-tree Rust and TypeScript tests, schema parity, registry parity, replay/undo/redo, and runtime evidence gates pass.

## Inference Leaf

An inference leaf is accepted only when every item is true.

- The leaf owns one canonical `.v1` result contract, computation entrypoint, dependencies, validity, quality, diagnostics, provenance, and unavailable behavior in Rust and TypeScript.
- Pure reusable math remains in private named kernels and cannot construct the public inference result.
- JSON Schema, GraphQL, Protobuf, text, and binary facets describe the complete typed result without opaque JSON or aggregate aliases.
- One canonical vector is executed by both implementations and covers analytic value, unavailable/malformed input, determinism, budget, and quality/provenance.
- Cold and incremental results are identical; dependency, policy, revision, and generation changes invalidate correctly; stale executions are rejected.
- The leaf owns its descriptor/service and cache identity. The inference root only assembles descriptors and the dependency DAG.
- Current-tree Rust and TypeScript, schema parity, cache/DAG, wire, and runtime evidence gates pass.
