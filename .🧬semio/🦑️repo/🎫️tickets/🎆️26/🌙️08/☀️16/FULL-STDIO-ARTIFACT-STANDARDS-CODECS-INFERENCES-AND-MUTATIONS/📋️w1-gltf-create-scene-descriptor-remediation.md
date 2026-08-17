# W1 glTF Create-Scene Descriptor Remediation

## Scope And Freeze Boundary

This remediation changes only the unmounted `create-scene.v1` leaf and ticket-local verification evidence. It does not alter the mutation registry, schema root, glue, dispatch, text/binary transports, relation leaves, or material leaves. The leaf remains **unregistered and not accepted** pending the integrated runtime gate.

## Delivered Leaf Contract

- `create-scene/🦀️component.rs` exports `pub const DESCRIPTOR: GltfMutationLeafDescriptor` using the frozen common contract. The private `GltfCreateSceneDescriptorAdapter`, its private plan type, and serialized-path inspection entry points are removed.
- Descriptor planning canonical-decodes the mutation payload, validates it, derives the command-owned forward diff and inverse, canonical-encodes both, and returns concrete paths calculated by the diff leaf.
- Descriptor diff/inverse application decodes, applies the command phase, then returns paths recalculated from command semantics. Both phase leaves reject a forged serialized `touchedPaths` value before any mutation is applied.
- Mutation, diff, and inverse validate the incoming default-scene reference. Invalid indexed references reject atomically as `gltf.mutation.reference-out-of-range`.
- Forward planning preserves the exact pre-state collection count, default scene, and insertion anchor. Inverse planning preserves the exact forward collection count, inserted empty scene, post-insertion anchor, default-scene restoration rule, and concrete paths.
- Diff and inverse retain the same canonical command ID; phase is the typed phase field rather than an `.inverse` command identity.

## Facet Parity

The three physical phase folders contain **15 typed phase facets**: Rust, TypeScript, JSON Schema, GraphQL, and Proto for mutation, diff, and inverse. The leaf has 19 source/contract files in total.

- The inverse JSON/GraphQL/Proto facets now represent `expectedSceneCountAfter` and `expectedNextSceneAfter` in addition to its exact expected scene/default values.
- `GltfCreateSceneValueV1` and `GltfCreateSceneAnchorV1` are owned once by the diff GraphQL/Proto facet. The inverse facet references those shared command types instead of redeclaring them.
- All phase schemas retain a parameterized descriptor pattern (`document/scenes/{position}`) while every planned/applied vector asserts concrete indexed paths.

## Canonical Executable Vector

`create-scene/🧪️contract/🔣️component.json` is the sole vector source consumed by both implementations. It covers malformed payload decoding, range and default-reference rejection, direct mutation, diff derivation/application, replay/default/anchor stale rejection, forged forward paths, inverse derivation/application, inverse index/scene/anchor stale rejection, forged inverse paths, canonical IDs, and JSON serialization.

The Rust vector invokes `DESCRIPTOR.plan`, `apply_diff`, `plan_inverse`, and `apply_inverse`, as well as the typed phase derive/apply functions. The TypeScript vector imports the same JSON file and runs the equivalent mutation/diff/inverse laws.

## Verification

Passed:

- `rustfmt --edition 2021 --check` for all five owned Rust leaf/contract files and the ticket Rust harness.
- `CARGO_TARGET_DIR=.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/🎯️create-scene-vector cargo test --manifest-path .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/Cargo.toml` — 1 Rust descriptor/vector test passed; 0 failed. The ticket-local harness compiles the exact unmounted leaf files against the frozen descriptor shape and a minimal scenes-only snapshot harness.
- `bun .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/verify_w1_a_gltf_create_scene.mjs` — common-descriptor shape, no forbidden transitional core symbols, exact wire-field parity, no duplicate GraphQL/Proto command types, and the canonical TypeScript vector passed.

Not green:

- A standalone TypeScript static check reaches two pre-existing errors in the unowned shared `🔒️top-level-collections-private/🟦️component.ts:40`: TS2352 casts `GltfDocument` directly to `Record<string, unknown>`. The create-scene Bun vector itself executes successfully; this shared-helper type error was not modified in this leaf-only shard.
- The plugin's integrated Rust crate/runtime gate was intentionally not run: this leaf is unmounted by scope, and registry/glue ownership belongs to integration. Ticket-local vector success is evidence for mount readiness, not final acceptance.

## Handoff

`create-scene.v1` is mount-ready for the integration owner through `create_scene::DESCRIPTOR`. Registering it is outside this shard. It must remain **not accepted** until the integration owner mounts the leaf and runs the repository Rust/transport runtime gate.
