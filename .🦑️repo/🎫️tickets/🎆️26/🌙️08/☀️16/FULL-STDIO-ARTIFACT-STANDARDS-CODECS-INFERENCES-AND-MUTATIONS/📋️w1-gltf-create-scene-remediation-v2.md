# glTF Create-Scene v1 Remediation

## Delivered Source Changes

- The exact `create_scene` Rust glue module mounts the common descriptor, command-private mechanics, phase modules, and shared Rust contract.
- Rust and TypeScript create-scene phases use only command-local scene sequence/reference mechanics. They do not import the generic top-level collection repair or `family_diff` seam.
- Forward and inverse payloads carry complete expected scene sequences. Validation rejects stale distant scenes, default-scene drift, forged touched paths, replay, and forged canonical empty scenes before mutation.
- All command indices/counts/versions use the u32 domain at the Rust, TypeScript, JSON Schema, GraphQL, and Proto boundaries. Rust converts u32 to usize through checked helpers.
- JSON Schema phase identifiers are unique (`.mutation`, `.diff`, `.inverse`) while `x-semio.id` remains `s.stdio.gltf.mutation.create-scene.v1`.
- The sole canonical JSON fixture now covers default remapping, append without a default scene, distant stale forward/inverse state, forged paths, and exact inverse restoration. Both Rust and TypeScript contracts consume it.
- The Rust root assembly and schema manifest register the common descriptor. The TypeScript registry is descriptor-based for this path and the generic text/binary envelope resolves it without a closed mutation union or tag switch.

## Local Evidence

- `bun .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/verify_w1_a_gltf_create_scene.mjs` passed. It executes the shared TypeScript vector, validates root/glue/static assembly, and verifies descriptor text/binary round trips plus diff application.
- JSON parsing passed for all three phase schemas, the canonical vector, descriptor assembly, generic envelope schema, and text transport schema.
- `rustfmt --edition 2021 --check` passed for the create-scene Rust leaf/common descriptor/contract, mutation root, Rust dispatcher, and binary transport. The generated shared glue file was not globally formatted because rustfmt traverses unrelated pre-existing module formatting differences.
- Cargo and Nx were not run, per the serialized runtime gate constraint.

## Gate Handoff

1. Run the serialized stdio Cargo no-run gate after the runtime/store owner resolves its current disjoint errors.
2. If it reaches glTF, first check the exact `create_scene` glue mount and u32 dispatcher change; run the ticket-local Rust vector target after compilation succeeds.
3. Keep the known unrelated inference parity issue separate: `registry::executable_mappings()` is empty while the artifact manifest advertises executable inference leaves. Do not count it as create-scene acceptance.
