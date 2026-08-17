# W1 glTF Relation Mutation Remediation V3

## Scope

- `bind-node-child.v1`
- `unbind-node-child.v1`
- `bind-scene-root-node.v1`
- `unbind-scene-root-node.v1`

## Implemented Leaf Semantics

- Each Rust descriptor owns a distinct `plan_inverse` path. It decodes and validates the typed inverse payload against the after-state, then plans the inverse phase; it never decodes an inverse envelope as a forward mutation payload.
- Both unbind commands now validate the selected target node index before testing relation membership.
- Every inverse records the exact relation sequence produced by the forward mutation (`expectedChildren` or `expectedNodes`). Inverse application rejects a changed post-forward sequence, as well as forged concrete touched paths, stale position, missing container, and duplicate relationship state.
- Rust, TypeScript, JSON Schema, GraphQL, and Protobuf inverse definitions share the same explicit expected-sequence field.
- Four shared canonical vectors now exercise successful plan/apply/replay/undo, malformed JSON, out-of-range index, missing target reference, forged touched paths, stale forward diff, altered after-state inverse rejection, and canonical JSON serialization.

## Validation Evidence

- TypeScript runtime vectors: **passed 4/4** on 2026-08-16. The runtime command emitted one `[DEBUG] TS relation vector passed` line per leaf.
- Contract vector JSON parses: **passed 4/4** on 2026-08-16.
- Rust focused contract command: `cargo test -p semio-s-plugin-stdio canonical_vector_enforces_forward_inverse -- --nocapture` is **blocked before test execution** by 1,018 unrelated current-tree errors in OBJ, DXF, IFC, LAS, PLY, PDF, raster, and other artifacts still consuming the pre-`MutationOutcome` API. The exact compiler log is `🧪️w1-gltf-relation-rust-contract.log`.
- Rust library check is running in the isolated `🎯️target/gltf-relations` target and is affected by the same cross-artifact mutation API migration. No relation-source diagnostic has been reported so far; its log is `🧪️w1-gltf-relation-rust-library.log`.

## Registration Decision

The four relation modules are present in Rust glue only so their isolated source and contract code can compile. They are intentionally **not yet registered** in `GLTF_MUTATION_LEAF_DESCRIPTORS` or the immutable JSON manifest. The required focused Rust runtime gate has not executed, so no leaf is accepted and no registry membership is claimed.

## Required Handoff

After the umbrella Rust crate is buildable, rerun the focused Rust contract command. Only after it passes, add the four descriptors to the root immutable registry and manifest, then run registry dispatch and text/binary transport gates.
