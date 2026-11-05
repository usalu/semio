# Value Equality Dependency Cycle

Native Cargo admission reproduced the production cycle intrinsic -> equality -> retained-clone -> intrinsic before this change. Intrinsic requires actual typed equality implementations, so removing that dependency would break production source. Equality used retained-clone only for source custody and physical work controls; it did not require concrete clone cursors.

Equality now depends directly on existing clone-source for RetainedCloneRef, RetainedCloneBinding and test RetainedCloneSource. It imports grants, progress, steps and admission from their existing control-contract owner. The unchanged 64 KiB bulk page bound moves to that shared control contract and the clone traits module explicitly reexports the same item. No cursor algorithms, original authority, retirement order, progress accounting or cancellation behavior changed. No new external dependency, adapter, fallback or compatibility implementation was added.

## Validation

A repository-wrapped read-only path-dependency DFS from the equality manifest passed across 7 unique production manifests and 14 path edges without a cycle. Equality source and list-equality source contain no remaining higher retained-clone crate imports. The shared page constant remains exactly 64 * 1024. This is static dependency evidence, not a compiler result.

The existing language-neutral structural equality fixture has 15 cases and 8 cancellation cuts. Existing actual-work JSON has 2 bank cases. Rust regressions use serde_json as the independent JSON/value oracle and verify grant denial, actual allocation accounting, original source custody, cancellation and child-first bounded closure. Their source imports were corrected to the actual lower owner; fixture semantics remain unchanged.

Native owns all Cargo validation after the edit-ready handoff, including the exact equality manifest library tests if admission permits; results are pending and retained in native-validation.md. No concurrent Cargo or dependency sync was started by this agent. An initial redundant heavyweight read was cancelled through its own process after the light registered-tool read completed.

## Exact Touched Files

- `🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/📋️list/🟰️equal/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/🧪️tests/actual.rs`
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📜️clone-and-cursor-traits/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/SHOOTING-PLUGIN-END-TO-END/value-cycle.md`
