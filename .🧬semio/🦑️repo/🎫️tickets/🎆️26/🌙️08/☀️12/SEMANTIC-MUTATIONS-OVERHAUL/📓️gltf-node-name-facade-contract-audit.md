# glTF Node Name Facade Contract Audit

## Availability

At 2026-08-27, the retained neutral fixture `🧪️gltf-node-name/🧫️fixtures/🔣️laws.json` is present and readable. An earlier root read reported `ENOENT`; that earlier availability failure is retained as an observation, not represented as a current missing file.

The workspace itself has Ajv 2020 but no GraphQL/protobuf parser executable or package. Root then supplied an isolated test-only ticket package at `🧪️language-surface-oracles/`, with GraphQL 16.11.0 and protobufjs 7.5.4. Its local package/lock/cache boundary leaves the workspace package and lockfiles unchanged. `🧪️gltf-node-name/📜️script.ts` imports those exact local parsers; no production dependency or Cargo invocation was added.

## Required Facade Invariants

All representations of the public change-node-name envelope must preserve these facts from the canonical JSON Schema:

- `node` is an unsigned 32-bit integer, including values above GraphQL's signed `Int` maximum.
- apply has exactly one required nullable `value`; restore has exactly the two required nullable `before` and `after` witnesses.
- an optional-string carrier has exactly one state: a present string or an explicit `absent: true` marker. Neither state, both states, or `absent: false` are invalid.
- the phase and payload are coupled: apply has apply only; restore has restore only. Neither, both, and mismatched phase/payload forms are invalid.
- a Protocol Buffers `oneof` must be present and selected; a decoded message with no selected `state` or no selected `phase` is invalid before it maps to the leaf operation.

## Current Red Gaps

`🔗️.graphql` uses `Int!`, which GraphQL 16.11.0 coercion rejects at canonical node `4294967295`. Its optional-string input accepts no state, both `present`/`absent`, and `absent: false`; its phase envelope accepts absent, both, or phase-mismatched payloads.

`🛰️.proto` accepts the `uint32` maximum and rejects two simultaneously selected oneof members under protobufjs 7.5.4. It nevertheless verifies absent embedded messages, an empty optional-string state, an empty phase, and `absent: false`. Unlike GraphQL it has no independent declared phase to mismatch, so a restore-only object verifies as a valid restore rather than carrying an apply/restore mismatch. Explicit representation validation is required before mapping decoded objects to the leaf operation.

The retained `🧪️gltf-node-name/🧫️fixtures/🔣️facade-parser-laws.json` has 11 schema-validated vectors: valid apply/restore, uint32 maximum, missing nullable state and carrier, missing restore witness, false/both optional states, no phase payload, phase mismatch, and multiple phase payloads. `🧪️gltf-node-name/🧪️facade-parser-green.log` records the actual parser run: nine vectors diverge from the canonical JSON Schema. `🧪️gltf-node-name/🧪️facade-parser-red-restore.log` is retained as the first expectation correction, where GraphQL's non-null restore witness rejected omission while protobufjs accepted its absent field.

No GraphQL/protobuf production facet was edited for this audit. The Rust test-only compile corrections were separately released after the registered build exposed them.
