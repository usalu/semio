# W1-A glTF Top-Level Mutation Batch

## Accepted Leaf

`create-scene` is the only accepted leaf in this handoff. `delete-scene` remains an unaccepted candidate pending its separate audit.

| Measure | Count |
| --- | ---: |
| Accepted command leaves | 1 |
| Accepted physical phase folders | 3 |
| Typed phase facets (`TS`, `Rust`, `JSON Schema`, `GraphQL`, `Proto`) | 15 |
| Shared contract vectors | 1 |
| Shared contract consumers | 2 |

The canonical descriptor identity is `s.stdio.gltf.mutation.create-scene.v1` in all three phases. The `phase` field distinguishes mutation/diff/inverse; no phase receives a derivative command ID. Its command-root Rust descriptor adapter owns payload decoding, planning, replay, and touched-path inspection while delegating semantic work to its three phase modules; integration may bind it into its common registry without moving command logic into dispatch.

For creation, the forward diff is exactly `{ position, expectedSceneCount, expectedDefaultSceneBefore, expectedNextScene, scene }`; the inverse is exactly `{ position, expectedScene, defaultSceneBefore, expectedDefaultSceneAfter }`. The forward diff validates its collection-size, default-scene, and insertion-anchor preconditions before repairing references or inserting. It rejects a replay on the post-state, a changed default scene, and a changed anchor with `gltf.mutation.stale-diff`. Both phases derive concrete `document/scenes/<position>` and, when default-scene repair changes it, `document/scene`; the descriptor carries parameterized patterns only.

## Evidence

The shared vector at `create-scene/🧪️contract/🔣️component.json` records base, forward result, undo result, exact diff/inverse envelopes, and typed rejection cases. Its TypeScript contract imports that JSON vector and executes the laws; its Rust `#[test]` source serde-deserializes it, constructs a real `GltfSnapshot`, then runs mutation apply, `diff::derive` + diff apply, `inverse::derive` + inverse apply, serialization, and range, replay, default-scene, anchor, and inverse-staleness assertions.

Executed checks:

- `bun verify_w1_a_gltf_create_scene.mjs` — passed: phase facet parity, canonical JSON vector acceptance, range rejection, concrete paths, diff apply, post-state replay rejection, default-scene and insertion-anchor stale rejection, inverse restoration, stale-inverse rejection, and serialization.
- `rustfmt --check` over the create-scene descriptor, mutation, diff, inverse, and contract Rust sources — passed, providing Rust parse and formatting validation.
- Focused static scans — no `GltfDiff`, `familyDiff`, transitional derivation, generic operation variants, inverse ID suffix, `payload_json`, or clone-no-op apply in the accepted leaf.

No Cargo, Nx, or integration/dispatcher build was attempted: the inference gate remains source-only, and mutation root/glue ownership is reserved for integration.

## Unaccepted Candidates

The other 67 initially requested document/top-level leaves are intentionally **not accepted** in this batch, including `delete-scene` pending its independent audit. Existing generated collection candidates remain outside this report’s count; they require the same exact command-specific diff/inverse and facet parity standard before integration. The 16 earlier document core candidates likewise remain unaccepted because they still use the pre-parity diff/facet format.

No inference-owned files, mutation root/glue, transports, definition JSON, or nested command leaves were changed.
