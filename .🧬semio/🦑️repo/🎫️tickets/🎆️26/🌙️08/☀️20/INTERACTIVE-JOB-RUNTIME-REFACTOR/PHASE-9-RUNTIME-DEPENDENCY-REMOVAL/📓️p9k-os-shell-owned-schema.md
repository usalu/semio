# P9k OS Shell Owned Schema

## Result

The OS shell Rust package no longer depends on `ts-rs`. Its TypeScript mirror is generated from a repository-owned, versioned schema table colocated with the shell wire types.

## Implementation

- Replaced every `ts_rs::TS` derive and `#[ts(...)]` attribute in the shell component with 30 `SchemaMetadata` rows.
- Every row carries a nonzero schema version, canonical type name, and complete TypeScript declaration.
- The validator rejects zero versions, duplicate names, and declaration/name mismatches.
- The renderer preserves deterministic declaration order and emits the committed `🤖️generated/🟦️shell.ts` file byte-for-byte.
- The package `typegen` feature is now dependency-free.
- The Bun/Nx typegen command invokes the owned Rust export test directly and no longer creates or consolidates `ts-rs` scratch bindings.

## Verification

- `bun nx run @semio-tech/framework-os-shell-rs:typegen --skip-nx-cache`: passed; 1 export test passed.
- `bun nx run @semio-tech/framework-os-shell-rs:test-quick --skip-nx-cache`: passed; 10/10 tests passed, 1 test skipped by profile.
- `cargo check --release -p semio-framework-os-shell --features typegen`: passed.
- `cargo check --target wasm32-unknown-unknown -p semio-framework-os-shell --features typegen`: passed.
- `cargo clippy -p semio-framework-os-shell --all-targets --features typegen -- -D warnings`: passed.
- `cargo fmt --check -p semio-framework-os-shell`: passed.
- `cargo tree -p semio-framework-os-shell --features typegen | rg 'ts-rs|ts_rs'`: zero matches.
- Source census for `ts-rs`, `ts_rs`, and `#[ts(` in the complete shell subtree: zero matches.
- Generated declaration census: exactly 30 `export type` declarations.

## Scope Boundary

This packet removes `ts-rs` only from `semio-framework-os-shell`. Other workspace consumers remain Phase 9 work; the workspace-wide dependency audit is therefore not claimed complete.
