# FND-LOWER-METADATA-10 — Lower Mutation Metadata Contract

## Scope

Moved the fourteen-field `MutationLeafDescriptor`, its five wire enums, error types, and const validators from OS command to the product-neutral `protocol::mutation` contract. OS command and the OS SPR facade retain explicit public reexports; they do not define a second metadata type.

The five existing actual-source descriptor tests and their twenty neutral roster vectors remain at the OS command façade, now resolving the canonical lower implementation. The lower owner contains a genuinely borrowed `BorrowedLeaf<'a, T>` metadata test; it infers the constants from a `BorrowedLeaf<'a, &'local u32>` value rather than assuming a static payload. The pending registered Nx replay is the execution evidence for this strengthened lower-owned unit test.

## Frozen Lower API

```rust
pub trait MutationLeaf {
    const DESCRIPTOR: MutationLeafDescriptor;
    const PROVENANCE: MutationSourceProvenance;
}
```

`MutationSourceProvenance` has exactly `workspace_token: [u8; 32]`, `mutation_root`, `owner`, `source_path`, `descriptor_path`, and `taxonomy_path`. It is `Clone + Copy + Debug + PartialEq + Eq`, with neither `Default` nor a serde derive. No base `Mutation` item, mutation-kind supertrait, derive, registry, or production leaf changed in this packet.

## Structural Evidence

The qualified source scan after the move finds each metadata enum/struct/trait declaration only in `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs`. OS command contains direct reexports only; the SPR façade reexports the lower contract symbols.

`git diff --check` is clean for all three edited Rust paths. The coordinating root observed its fresh actual lower-crate build exit 0; its retained log is `🧪️lower-metadata-build-retry.log`. This packet did not start Cargo.

## Paired Compiler Oracle

The lower-owned neutral compiler contract is at `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧬️mutation-leaf-contract/`. Its schema requires `providesDescriptor`, `providesProvenance`, `borrowedGeneric`, `expectedCompile`, and `E0046` for every negative case.

The ticket harness `🧪️lower-metadata-contract/📜️script.ts` validated the schema with Ajv, then compiled every case exactly once with matching-hash protocol and serde rlib+rmeta pairs passed together to `rustc`. It records source, argv, compiler output, runtime output, and result JSON for each case.

Observed standalone compiler-oracle command, exit 0 (this is direct Bun execution of the ticket harness, not a registered Nx target):

```text
SEMIO_LOWER_METADATA_DEPS='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️derive-contract-target/debug/deps' SEMIO_LOWER_METADATA_PROTOCOL_HASH=9326ffd3ad988ba0 SEMIO_LOWER_METADATA_SERDE_HASH=9726de5488b8f586 bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️lower-metadata-contract/📜️script.ts'
```

Retained evidence: `🧪️lower-metadata-contract/🧫️run-Yfcu3a`. Results: `borrowed-generic-complete` compiled and ran `insert-page:42`; `missing-descriptor` and `missing-provenance` each failed compilation with `E0046`; all three completed with no signal or process error.

The registered workspace target is `bun nx run @semio-tech/framework-replication-rs:test`; it remains root-owned. A prior retry selected two lower tests and passed with 211 filtered tests (`🧪️lower-metadata-registered-retry.log`); the earlier wrong filter selected zero tests (`🧪️lower-metadata-registered.log`) and is retained only as non-passing selection evidence. Root will replay the registered Nx target after the strengthened lower-owned unit test.

## Source Release

Release is the current shared working tree. The exact changed paths are sent to the coordinator for its root gate.
