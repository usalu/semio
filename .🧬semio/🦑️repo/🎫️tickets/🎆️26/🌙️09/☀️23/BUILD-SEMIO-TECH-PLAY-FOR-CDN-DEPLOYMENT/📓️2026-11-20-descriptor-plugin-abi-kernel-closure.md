# Describe emitter plugin ABI dependency

The remaining direct production edges to `semio-os-kernel` are the five in [the descriptor closure audit](describe-store-closure.md): describe, guest-contract, plugin ABI, interpreter runtime and interpreter imports. Cutting the ABI edge alone leaves the document and engine reachable through the four parallel edges.

## ABI owner map

These are the six Rust sources compiled as part of `semio-plugin-abi` that name the OS kernel. Paths below are relative to the OS plugin module.

| ABI source | Kernel use | Existing direct provider |
| --- | --- | --- |
| `io/🦀️.rs` | Composer, analyzer and payload types, including the confidence type used by the IO registry | `semio-os-io`, which defines the typed composition surfaces and re-exports the original erased registry types and payload |
| `io/🦀️.rs` | `ArtifactPack` and `PackError` | `semio-os-pack`, which re-exports the original framework pack types |
| `io/🦀️.rs` | `ArtifactStoreSnapshot` compiler bounds | `semio-os-store-text-io::ArtifactStoreSnapshot`, its defining crate |
| `io/🦀️.rs` | `ColdMutation`, `ColdMutationDiff`, `MutationOutcome` and `MutationApplyResult` | `semio-framework-replication`, which exports the same existing mutation and diff definitions |
| `io/authority/🦀️.rs` | `Observer` and `EntityIdentityAuthority` | `semio-os-vcs-entity-identity::control` |
| `io/authority/snapshot/🦀️.rs` | `IoNativeDirection` and `IoRunControl` | `semio-framework-io-control`, which defines both |
| `schema/actor-input/🦀️.rs` | `pack_rt` actor wire encoding and decoding | `semio-os-pack::pack_rt` |
| `schema/sqlite/🦀️.rs` | `pack_rt` SQLite wire diagnostics encoding and decoding | `semio-os-pack::pack_rt` |
| `schema/🦀️.rs` | `CHANNEL_VERSION` in the WASI p2 binding module | `semio-os-channel::CHANNEL_VERSION` |

Use explicit direct imports. The `store`, `protocol` and `pack` aliases in ABI's root are aliases for the whole OS kernel. The private `Mutation` and `MutationDiff` imports have no references in the mounted closure. The ABI's own `Confidence` enum serves decomposition, and the separately imported IO confidence is part of the analyzer and composer contracts. Both identities must stay intact. The mounted actor-input and SQLite unit tests also spell the kernel pack path and need the corresponding direct import.

## Exported expansions

`component_persistent_local!` uses `$crate::component_persistent_local::ComponentPersistentLocal` on wasm32-p2 and `std::thread_local!` elsewhere. Its reactor, extension and macro-generator consumers need no path change. `return_walk_template!` reads the caller's generated `super::return_types` and imports `semio_framework_value`. It is instantiated by the guest ABI and the host operation bridge through the guest-contract re-export. Neither expansion uses an OS-kernel alias. The generated Wasip2 export uses `$crate::component::wasip2`. `owned_abi` is a module re-export, with framework-kernel data types in its signatures. The SDK exports can keep the same bindings and canonical types.

## Scope and verification

The source cut is one production ABI manifest plus the six Rust files above. It makes no new type, facade, feature split or compatibility layer. Both OS IO and store text-IO retain space-history, which remains part of the necessary closure. The coordinated read-only manifest simulation found that combining the ABI's seven direct providers with the other four kernel-edge replacements reduced reachable local production manifests from **341 to 297**. The conservative optional, target and build-dependency scan went from **358 to 305**. The simulated graph excludes `semio-os-kernel`, `semio-os-store-document`, `semio-os-store-engine`, `-engine-core`, `-engine-live`, `-engine-bundle` and the member-open store. It still contains DSL, JSON and value owners, so their source churn remains visible.

There are **176 tracked non-ticket Cargo.lock files** containing a `semio-plugin-abi` stanza, including the repository, framework, application and hub workspace locks. The new dependency list changes the locked package stanza throughout the affected workspaces. This is a narrow source change but a broad locked-build update, and the ABI edge alone would deliver no descriptor closure reduction. A later landed change needs the coordinated five edges, sanctioned lock refresh, host describe and ABI checks and existing WASI p2 and host/guest binding consumers. No production files or locks were edited, and no Cargo command ran.
