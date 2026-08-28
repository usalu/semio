# No-State Publication Fixture Implementation

## Scope

This packet converts `NoConfigMutation`, `NoPresenceMutation`, and `NoTransientMutation` into uninhabited aggregates and replaces the five former `Noop` publication emissions in the owned plugin test paths with source-owned concrete leaves. `InteractionConfigMutation` and all runtime-peer regions remain untouched.

## Source ownership

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
  - `No*Mutation` aggregates are empty enums with `Mutation::DESCRIPTORS = &[]`.
  - Their text and binary decoders reject every input; value-only operations are exhaustive uninhabited matches.
  - The retained cancellation, linearization, stale-root, and fairness presence paths use `PublicationPresenceMutation` and the exact replacement leaf.
  - `TestApp` publishes concrete presence and transient replacements from `TestCommand::Increment`; it preserves the preflight/publish/ack/retirement, cancellation, and latest-wins paths.
  - The fixture is mounted at the component root before inline `app`, so its canonical path is resolved from the owning `🦀️component.rs` directory rather than an inferred inline-module directory.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️publication-fixtures/**`
  - Physically authored `PublicationPresence` and `PublicationTransient` snapshot roots own direct `ChangePublicationPresence` and `ChangePublicationTransient` leaves.
  - Each snapshot has a structural diff with `Option<u64>` replacement: default is identity, `absorb` takes the later explicit replacement, and each leaf changes an exact `u64` revision. Its inverse emits the previously observed revision.
  - Each leaf owns exact text (`change-publication-… <u64>`) and binary (`tag + eight big-endian bytes`) codecs. The aggregate delegates to that leaf; descriptors declare the matching opcode, stable tag, and `text`/`binary` surfaces.

`change` was verified in `protocol::APPROVED_VERBS`; `advance` is absent. No `Advance…` leaf or `advance-publication-*` descriptor remains in this fixture tree.

## Schema-first evidence

The retained neutral schema and fixture are:

- `🧪️no-state-publication-fixtures/🛂️schema.json`
- `🧪️no-state-publication-fixtures/🧫️fixtures/🔣️cases.json`
- `🧪️no-state-publication-fixtures/📜️script.ts`

The script uses Ajv 2020 to validate the exact two `change-publication-*` records, compares all fourteen descriptor fields against the actual descriptor JSON, verifies all canonical leaf/source/schema paths, proves the component-root mount is before `app`, checks the actual three no-state source declarations for empty rosters without a `Noop` construction, validates the three-by-three reject matrix, and checks the safe JSON numeric payload matrix against both leaf schemas. The Rust unit matrix additionally covers serde's exact `u64::MAX` acceptance and `u64::MAX + 1` rejection, which JavaScript numeric values cannot represent exactly.

The original pre-source red is retained at `🧪️no-state-publication-fixtures/🧫️run-avqMUG/🔣️results.json`; it failed only because the planned fixture roots were absent. The latest green run is retained at `🧪️no-state-publication-fixtures/🧫️run-htEWOi/🔣️results.json`; it includes a BigInt reference implementation for the exact `u64` text/binary vectors and strict aggregate operation/payload envelope validation.

Executed command:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️no-state-publication-fixtures/📜️script.ts
```

It exited `0` and emitted `[DEBUG] no-state publication neutral passed=true`.

## Runtime boundary

No Cargo or `rustc` command was run. The source Rust gate is pending root’s serialized compiler execution. The relevant registered tests are `publication_fixture::tests::no_state_mutations_have_empty_rosters_and_reject_all_codec_input`, `publication_fixture::tests::publication_leaves_apply_inverse_preserve_identity_diff_and_expose_full_rosters`, `publication_fixture::tests::publication_leaf_and_aggregate_codecs_are_exact_and_u64_serde_rejects_invalid_numbers`, `retained_latest_wins_cancellation_guards_real_store_publication_and_preserves_committed_ack`, `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair`, and `a_command_reaches_both_ephemeral_lanes_without_touching_history`.
# Public Runtime Client Pending Artifact Release

`🧪️no-state-public-plugin-client/🦀️public-client.rs` is a genuine external Plugin client: it imports the public no-state types, requires each public `Mutation::DESCRIPTORS` roster to be empty, and rejects text, binary, and serde unit/object forms. It does not recreate any Plugin type.

`🧪️no-state-public-plugin-client/🦀️fixture-client.rs` mounts the real canonical publication-fixture root. Its six selected tests cover the three no-state boundary assertions plus real leaf and aggregate serde, full `u64` raw-JSON boundaries, identity, absorb, apply, inverse, text, binary, descriptor, and malformed-codec laws. Its run mode requires an explicit coherent Plugin/kernel/serde/serde_json artifact map; it has not been executed because the Plugin compiler slot is still owned by root. Its static preparation command ran successfully and retains the exact 16-file source closure in `🧪️no-state-public-plugin-client/📜️script.ts`.

Aggregate envelope schemas now exist at the two real aggregate mutation roots. They require exactly `operation` and `payload`, constrain the sole operation discriminator, and reference the owning leaf payload schema. The retained Ajv matrix validates successful and missing/wrong/unknown operation-envelope cases separately from leaf payload cases.

## Derive Repair And Pending Adoption Census

The first peer Plugin compile stopped before tests because the four publication fixture sources addressed a non-dependency `dsl_derive`. The actual Plugin path is its existing kernel alias `dsl`: `🦀️component.rs` is mounted by the package glue with `extern crate semio_framework_os_kernel as dsl`, and the kernel's DSL facade reexports `MutationLeaf` and `Mutations`. The two direct leaves now derive `dsl::MutationLeaf`; the two aggregate roots now derive `dsl::Mutations`. The public fixture client mirrors that exact alias. The neutral replay retained at `🧪️no-state-publication-fixtures/🧫️run-ZfKBGA/🔣️results.json` checks all four sources reject `dsl_derive::` and use the kernel DSL facade. No Rust compiler command was run in this lane.

The peer's reported E0046 adoption sites were reread, rather than treated as an exhaustive Plugin inventory:

- `🦀️component.rs:6881` — `DummyMutation`, a one-leaf `set-count` DslOps test mutation.
- `:7121` — `TxnMutation`, three `set-count*` DslOps test variants with a foreign-step case.
- `:7559` — `SurfaceMutation`, a one-leaf `set-count` DslOps surface test mutation.
- `:28210` — `fixture_channel!` emits `$mutation`, a test-only `SetValue` mutation used by its generated channel fixtures.
- `:29349` — `WireTestOp::Add`, the contribution-wire test operation paired with `WireTestMutationKind`.
- `:33148` — `TestMutation`, the two-variant TestApp count/label mutation.
- `:33358` — `TestConfigMutation`, the TestApp config replacement/snapshot mutation.
- `:38392` — `ChildrenTestMutation`, an empty children-fixture mutation.
- `:769` — `composer_entry_of` is a generic builder/erasure boundary, not a mutation definition; its `ArtifactPack` snapshot bound means adopted fixture operations must retain their real codec bounds through this call path.

The existing concrete counter-oriented test fixtures should be adopted and derived in place for this separate mandatory metadata transaction; adding manually fabricated descriptors would evade the source/provenance contract. The `MutationKind` test bounds at these test callsites must be rerun after that adoption, because a derived aggregate requires direct leaf implementors. This census deliberately excludes the separately owned `InteractionConfigMutation`, lifecycle/runtime work, and the unrelated Plugin include at line 17684.
