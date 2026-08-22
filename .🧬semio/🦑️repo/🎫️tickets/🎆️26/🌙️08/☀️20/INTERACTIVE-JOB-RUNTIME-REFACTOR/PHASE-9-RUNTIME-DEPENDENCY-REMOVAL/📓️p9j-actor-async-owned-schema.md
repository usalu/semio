# Phase 9j — Actor and Async Owned Schema Metadata

## Scope and inventory

The direct `ts-rs` manifest inventory contains the framework umbrella, OS kernel, OS shell, framework UI, UI contract, actor, and async packages. This packet owns only the two isolated low-level packages outside the concurrent UI/OS/plugin work: `semio-framework-actor` and `semio-framework-async`.

Both packages now have zero `ts-rs` source, attribute, manifest, documentation, or dependency-tree references. The workspace dependency remains because the five excluded direct consumers have not yet migrated.

## Owned schema design

Each crate owns a feature-gated `schema_metadata` region containing versioned `SchemaMetadata` rows. Every row names one wire type, carries an explicit non-zero schema version, and owns the exact TypeScript declaration for that type. Validation rejects zero versions, duplicate names, and declaration/name mismatches before generation.

The `typegen` feature is now dependency-free. Its generator renders the static rows in deterministic order and the existing Nx `typegen` target writes the committed mirror directly through the crate's export test. A second no-environment test path compares the rendered bytes with the committed generated file, so stale mirrors fail without silently rewriting the fixture.

The migration preserves every generated `export type` declaration byte-for-byte. Only generator attribution and the former derive-generated Rust-doc blocks were removed from the generated files. Actor's previously hand-corrected tagged-union projections remain unchanged.

## Package changes

- Removed all `ts_rs::TS` derives, field attributes, trait imports, and `TS::export` calls from actor and async.
- Removed both direct optional `ts-rs` dependencies and changed both `typegen` features to empty owned-generator switches.
- Replaced the per-type scratch-directory consolidation logic in both existing `📜️script.ts` files with the owned schema export path.
- Updated Rust and TypeScript package descriptions to identify the owned-schema mirror.
- Kept the public TypeScript wire shapes unchanged; the actor TypeScript behavioral suite confirms the generated mirror remains consumable.

## Verification

All commands used the ticket-local `🧪️target-tsrs` Cargo target unless the command was TypeScript-only.

- Feature-enabled native compile: `cargo check -p semio-framework-async -p semio-framework-actor --features semio-framework-async/typegen,semio-framework-actor/typegen` passed.
- Owned generators: `bun nx run @semio-tech/framework-async-rs:typegen` passed; `bun nx run @semio-tech/framework-actor-rs:typegen` passed. The first parallel run exposed a missing crate qualification in the actor test; it was repaired, and the authoritative actor rerun passed.
- Byte-stable mirror assertions: the two feature-enabled `exports_typescript_bindings` tests passed, 1/1 per crate.
- Rust quick gates: `bun nx run-many --target=test-quick --projects=@semio-tech/framework-async-rs,@semio-tech/framework-actor-rs --parallel=2` passed: async 43/43 and actor 89/89.
- TypeScript gate: `bun nx run @semio-tech/framework-actor:test-quick` passed, 46/46.
- Release feature compile: both crates passed together.
- Wasm feature compile: both crates passed for `wasm32-unknown-unknown`; actor emitted only its pre-existing async-constructor deprecation warning.
- Clippy: both crates passed together with all targets and both `typegen` features.
- Source/manifest/tree census: zero `ts-rs`, `ts_rs`, or `#[ts(...)]` matches across actor and async; neither package's `cargo tree` contains `ts-rs`.
- Declaration differential: the generated-file diff contains no added or removed `export type` lines.
- `cargo fmt --check -p semio-framework-async -p semio-framework-actor` is not recorded as green because of one unrelated pre-existing formatting difference in actor `📦️glue.rs:45`; neither owned `🦀️component.rs` appeared in the formatter diagnostic.

No framework UI, UI contract, OS, shell, stdio, plugin, plugin-host, Store, or DSL file was edited by this packet.
