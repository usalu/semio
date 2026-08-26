# Cargo Stem Residual Classification

## Evidence status

This is a read-only classification of the retained pre-transaction-v2 inventory. It does not authorize a physical move or a fixed-contract widening.

The exact `semantic-stem-unresolved` Cargo census is 126 leaves:

| Basename | Count |
| --- | ---: |
| `Cargo.toml` | 73 |
| `Cargo.lock` | 53 |

| Location class | Count |
| --- | ---: |
| Governed canonical or embedded ticket roots | 122 |
| Workspace-nested non-ticket locations | 3 |
| Repository root | 1 |

## Mechanism packets

### Ticket-local Cargo evidence

The 122 ticket leaves are retained build/test evidence beneath exact `.🧬semio/🦑️repo/🎫️tickets/🎆️YY/🌙️MM/☀️DD/<ticket>/…` roots. They should not create semantic directories named `Cargo`. The safe schema mechanism is two ticket-governed fixed filename contracts, one each for `Cargo.toml` and `Cargo.lock`, using the same canonical/embedded ticket prefix grammar as the narrowed cache-tag authority. This authority must not match production paths.

### Repository lock

The sole repository-root residual is `Cargo.lock`. It needs one exact repository-root Cargo lock contract, parallel to the existing root `Cargo.toml` contract.

### Three workspace-nested decisions

These must not be swept into a broad exception:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊gpu/Cargo.toml` is a real `[package]` manifest for `semio-framework-os-renderer-wgpu`. The package taxonomy must explicitly decide whether target-specific crates are owned package boundaries or whether this crate moves to a canonical Rust package owner.
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/Cargo.toml` declares an intentionally isolated fixture workspace/package.
- The adjacent `jcoprobe` `Cargo.lock` is its generated Cargo lock authority. The fixture schema needs an exact external-package boundary or a canonical package relocation; a global fixture exception would be too broad.

## Next tests

The fixed-contract packet must prove all 122 ticket paths plus root `Cargo.lock`, reject equivalent production lookalikes, and use a third-party Cargo TOML parser or `cargo metadata` fixture parity. The three nested decisions require separate language-neutral path vectors and actual Cargo metadata evidence before schema admission.
