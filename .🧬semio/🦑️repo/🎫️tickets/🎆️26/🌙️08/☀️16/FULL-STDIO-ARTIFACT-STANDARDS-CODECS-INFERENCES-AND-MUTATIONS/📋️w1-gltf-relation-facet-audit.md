# W1 glTF Relation Mutation Full-Facet Audit

Audit date: 2026-08-16. Scope is frozen to the four unregistered relation leaves.

| Leaf | Facets present | Typed inverse planner | Legacy `plan_inverse: plan` | Exact inverse after-state witness | Rust/TS vectors | Status |
|---|---:|---:|---:|---:|---:|---|
| `bind-node-child.v1` | 19 | yes | absent | `expectedChildren` | yes | source frozen, unregistered |
| `unbind-node-child.v1` | 19 | yes | absent | `expectedChildren` | yes | source frozen, unregistered |
| `bind-scene-root-node.v1` | 19 | yes | absent | `expectedNodes` | yes | source frozen, unregistered |
| `unbind-scene-root-node.v1` | 19 | yes | absent | `expectedNodes` | yes | source frozen, unregistered |

## Cross-Facet Parity

- Each leaf contains exactly one command root plus mutation, sparse diff, inverse, shared contract, Rust, TypeScript, JSON Schema, GraphQL, and Protobuf facets.
- Canonical ID and version remain the leaf’s mutation identity: `s.stdio.gltf.mutation.<semantic-slug>.v1` and version `1`.
- Inverse JSON Schema includes the canonical ID/version/`inverse` phase metadata; GraphQL and Protobuf include the required `expectedChildren` or `expectedNodes` field with the same collection element type (`Int!` / `uint32`).
- All Rust command, diff, and inverse payload structs reject unknown JSON fields. JSON Schema facets likewise forbid additional properties.
- Concrete touched paths are recomputed by diff/inverse application and checked against the serialized value. No relation source contains a `[DEBUG]` line.

## Shared Contract Coverage

Each leaf’s one canonical JSON vector is executed in TypeScript and consumed by Rust tests when the crate reaches test compilation. It covers:

- exact forward mutation, sparse diff apply/replay, inverse undo, and canonical JSON serialization;
- malformed JSON payload rejection;
- out-of-range container and missing target-reference rejection;
- forged touched-path rejection;
- stale forward-diff rejection;
- stale inverse rejection when the post-forward relation sequence has changed.

## Registry Freeze

The four descriptors are deliberately absent from the Rust descriptor array and immutable JSON manifest. Their Rust module glue is present only for future contract compilation. Full crate compilation is currently blocked outside this shard by the stdio-wide `MutationOutcome` adoption; see `🧪️w1-gltf-relation-rust-contract-rerun.log` and `🧪️w1-gltf-relation-rust-library.log`.
