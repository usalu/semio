# Phase 9ab — Unused Compression Declarations

## Outcome

The root workspace manifest declared `flate2` and `libz-sys`, but no member manifest inherited
either declaration. The live dependency census therefore reported both as root-only external
identities even though no workspace package used them. Both unused declarations were removed from
`Cargo.toml`; no implementation or compression behavior changed.

## Verification

- `rg` over every `Cargo.toml` found no `flate2.workspace = true` or `libz-sys.workspace = true`
  consumer and only the two root declarations.
- `cargo metadata --no-deps --format-version 1` completed successfully after the edit.
- `bun ./📜️script.ts verify dependencies list rust` completed successfully.
- The live Rust census fell from 75 to 73 unique external identities: 70 runtime and 3 test-only.
- The census now reports no dependency whose only user is the root `Cargo.toml`.

This is declaration cleanup only. Transitive compression crates used by retained external packages
are not claimed as removed, and the Phase 9 zero-external-dependency gate remains open.
