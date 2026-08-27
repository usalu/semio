# FND-SOURCE-ROSTER-13 — Aggregate Source Validator

## Contract

`protocol::mutation::MutationLeafSourceScope` freezes the aggregate-owned workspace token, mutation root, taxonomy locator, source filename, and descriptor filename. `validate_mutation_leaf_source` first delegates to the existing complete descriptor validator, then fail-closes on unsafe authority facts, non-direct owners, any token-byte mismatch, provenance fact disagreement, or a source/descriptor path that is not the exact owner-plus-scope-filename value.

The scope has no default or wire serialization. The validator is `const`, allocation-free, compares each of the 32 workspace-token bytes, and preserves exact source spelling: no Unicode normalization, case guessing, historical filename against the current scope, or provider alias is accepted. It permits a different safe filename only when that independently supplied scope fact and the actual provenance suffix agree exactly.

## Neutral Contract

The schema-first fixture is `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧬️mutation-leaf-source-contract/`. It declares 26 authority/path cases and an exact permutation of all 32 workspace-token byte positions. The ticket harness expands that into 58 source probes, validates the fixture with Ajv, applies an independent exact-value/path reference, and generates one retained compiler/runtime artifact directory per probe.

Observed pre-artifact reference result: `source-roster-reference cases=58 failures=0`. The first fresh-artifact compiler attempt is retained as `🧪️source-roster-contract/🧫️run-tJAe23`; it exposed a NUL Rust-literal harness defect and an uppercase-`Compose` source-root error-order defect. Both are corrected in source.

Observed post-correction standalone compiler oracle, exit 0:

```text
SEMIO_SOURCE_ROSTER_DEPS='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️derive-contract-target/debug/deps' SEMIO_SOURCE_ROSTER_PROTOCOL_HASH=9326ffd3ad988ba0 SEMIO_SOURCE_ROSTER_SERDE_HASH=9726de5488b8f586 bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-roster-contract/📜️script.ts'
```

The final result was `cases=58 guard=E0080 failures=0`, retained at `🧪️source-roster-contract/🧫️run-q1Cvz4`. Every ordinary probe compiled and ran; its Rust result matched the independently calculated reference. Before compiling any probe, a dynamically generated Ajv schema independently validated the full scope, direct descriptor owner, and all six exact provenance facts for every vector; its boolean matched the reference outcome for all 58 cases. Complete Ajv outcomes are retained as `🧪️source-roster-contract/🧫️run-q1Cvz4/🔣️ajv-outcomes.json`. The separate invalid-provenance client asserted `RESULT.is_ok()` in const evaluation and failed compilation with `E0080`, proving that an invalid result cannot satisfy a const acceptance guard.

The root's registered lower-test invocation is currently blocked before test selection by an unrelated E0716 in `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/📡️transport/🧪️component.rs:31`; its retained log is `🧪️source-roster-lower-registered.log`. This packet did not edit that transport file and does not claim a full current registered-crate pass. No Cargo or Nx command was started by this packet.

## Source Boundary

Only the lower mutation contract, its dedicated neutral tests, OS command reexports, and OS SPR reexports are changed. `Mutation`, mutation-kind traits, aggregate derive, root policy, and production leaves remain untouched.
