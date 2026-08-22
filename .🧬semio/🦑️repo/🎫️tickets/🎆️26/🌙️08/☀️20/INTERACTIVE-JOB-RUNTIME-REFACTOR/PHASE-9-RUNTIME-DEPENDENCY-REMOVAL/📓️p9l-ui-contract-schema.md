# P9l UI Contract Owned Schema Projection

## Outcome

`semio-framework-ui-contract` no longer depends on `ts-rs`. Its 79 wire-facing TypeScript declarations now come from an explicit, versioned `SchemaMetadata` table in the contract crate. Validation rejects version zero, duplicate names, and declaration/name mismatches before the stable renderer emits the committed mirror.

The generated TypeScript contract is byte-identical to the previous projection apart from the provenance header and two documentation lines that no longer describe `ts-rs` behavior. Runtime serde wire shapes and all public Rust types are unchanged.

## Implementation

- Removed all 79 conditional `ts_rs::TS` derives and type attributes.
- Replaced the optional `ts-rs` feature dependency with an owned `typegen = []` feature.
- Replaced scratch `bindings/*.ts` generation and JavaScript consolidation with one Rust-owned deterministic renderer.
- Kept `generate` and read-only `check` behind the existing Nx/`📜️script.ts` commands.
- Locked transparent wire newtypes to `string`/`number` projections in an owned-schema test.
- Kept the existing declaration order and documentation in the committed mirror.

## Verified Gates

- Focused typegen export: 1/1 passed.
- Nx `test-quick`: 87/87 passed.
- Nx `check-wasm`: `wasm32-wasip2`, `wasm32-unknown-unknown`, and wasip2+typegen passed.
- Nx `check`: metadata validation and committed-mirror byte comparison passed.
- Clippy for all targets with typegen and `-D warnings`: passed.
- Release check with typegen: passed.
- Rustfmt check: passed.
- Cargo tree and package source census contain zero `ts-rs`/`ts_rs`/`#[ts]`/`TS` derive sites.
- Dependency ratchet: clean at 234 current versus 238 baseline, with no new third-party dependency.

## Remaining Boundary

`ts-rs` remains a workspace dependency because framework trace/kernel/interaction/UI/IO/manifest and OS-kernel consumers remain. This packet removes the UI-contract direct consumer and scratch codegen path; workspace deletion follows after those independent consumers migrate.
