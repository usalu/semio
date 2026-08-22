# P9m Framework Owned Schema Projection

## Outcome

The root `semio-framework` crate no longer depends on `ts-rs`. Its 172 exported TypeScript declarations are described by the crate-owned, versioned `SchemaMetadata` table and rendered in deterministic declaration order. The committed TypeScript mirror is validated from Rust without scratch bindings or a third-party derive/code-generation path.

## Implementation

- Replaced conditional `ts_rs::TS` derives and attributes with explicit schema metadata owned by the framework crate.
- Replaced the optional `ts-rs` dependency with an owned `typegen = []` feature.
- Kept the existing `generate`, read-only `check`, and tiered test entry points behind Nx and `📜️script.ts`.
- Added validation for nonzero versions, unique declaration names, and declaration/name agreement.
- Added a committed-mirror parity assertion over all 172 declarations.

## Verified Gates

- Framework release check with `typegen`: passed.
- Framework type-generation checks for `wasm32-wasip2` and `wasm32-unknown-unknown`: passed.
- Nx framework `generate`, `check`, and test targets: passed.
- Owned metadata count and committed-mirror comparison: 172 declarations, passed.
- Source and manifest census contains no `ts-rs`, `ts_rs`, `#[ts(...)]`, or `TS` derive in the framework crate.

## Boundary

The generated declarations remain a wire-contract artifact. Runtime serde removal is a separate codec migration and is not conflated with this code-generation dependency packet.
