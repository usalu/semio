# Boundary Preflight Error Audit

The preflight passed an empty generated-input observer list to `readRustSourceClosureV1` and truncated error details to 120 characters. Three `Unsupported Rust compile token output requires an explicit generated-input observer` rows are scanner limitations to resolve with the proper observer: the energy mutation derive emits `include_str!(#relative)` from `quote!`, the value text snapshot derive emits `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #path))`, and the DSL expansion closure includes quoted `include_str!` token interpolations in mutation expansion/schema-index modules. These observations do not prove the corresponding generated inputs exist for every consumer.

The fourth row is a real absent path: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🔏️pack-authority-and-publication/📦️source/📦️packages/🦀️rust/Cargo.toml` declares `../../🦀️.rs`, which does not exist. `🧰️framework/Cargo.toml` still lists this package as a workspace member. A separate source exists under `config/📜️contract/🔏️pack-authority-and-publication/📦️source/🦀️.rs`, included as a module by its parent. No Cargo.toml or Cargo.lock dependency on package `semio-plugin-window-config-pack-source` was found. This suggests a stale orphan after a move, but the intended owner should reconcile it; pointing the old manifest at the new module would not establish an independently compilable crate.

No source was edited: the workspace manifest and orphan manifest are shared active changes and altering the Cargo workspace during the active release loop would invalidate its source inputs. No build was run.

## Orphan Reconciled

The superseding `📜️contract` crate owns and exports the current pack source through its actual module chain. The old package has no manifest consumers; its previous registrations in root workspace members and framework workspace exclusions, the orphan manifest and project descriptor, and its stale package entries in root and composition lockfiles were removed. The composition lock also contained a stale dependency edge to the obsolete package; it was removed. Correction: the framework workspace entry was an exclusion, not a member.

Static verification with Bun's TOML parser accepted both workspace manifests and both edited locks, found no stale package or path in those four files, resolved the current contract as a root workspace member and framework workspace exclusion, and confirmed the contract module chain exists. No Cargo or build was run; actual compilation remains the active release's responsibility.
