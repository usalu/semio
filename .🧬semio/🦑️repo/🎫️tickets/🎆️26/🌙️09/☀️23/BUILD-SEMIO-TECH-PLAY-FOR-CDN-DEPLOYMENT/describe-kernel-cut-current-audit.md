# Descriptor emitter dependency closure after wAzFtU9

The live emitter graph still reaches `semio-os-kernel` through five direct declarations. The new scan follows local path and workspace dependencies through the kernel and document manifests. The earlier audit stopped at those two owners. [Earlier store closure audit](describe-store-closure.md)

| Kernel consumer | Existing owner and source change |
| --- | --- |
| `semio-plugin-describe` | Depend on `semio-os-pack` and select it as the existing `store` alias in `🖨️describe/🛂️descriptor-emission/🦀️.rs`. This preserves the same `store::pack_rt::{encode_wire_value,decode_wire_value}` implementations and the current source assertion on `descriptor_pack`. |
| `semio-plugin-guest-contract` | In `🧬️guest-contract/🦀️.rs`, use `semio_os_store_codec_registry::lifted_io::IoKey` for the current public `IoRouteConflict.key` field. It is the type already re-exported by the kernel's I/O facade. |
| `semio-plugin-interpreter-runtime` | Use `semio_os_vcs_entity_identity::control` for receipt, receiving, projection, receiver and authority types. The runtime root continues to re-export `OriginalOperationReceiving`. Use `semio_os_channel` for the current channel constants, admission function and mismatch faults, and `semio_os_identity` for WASI entropy. Twelve current Rust files reference the kernel, including runtime tests and their fixture calls. |
| `semio-plugin-interpreter-imports` | In `🧠️interpreter/🚪️io/📞️imports/🦀️.rs`, use `semio_os_vcs_entity_identity::control::OriginalOperationReceiving` and `semio_os_identity::{entropy_u64,fill_entropy}`. These are the existing controlled identity and entropy functions. |
| `semio-plugin-abi` | The coordinated ABI mapping selects `semio-os-io` for analysis and composition types, `semio-os-store-text-io` for `ArtifactStoreSnapshot`, `semio-os-pack` for pack and runtime codec names, `semio-framework-replication` for mutation types, `semio-os-vcs-entity-identity` for authority and observers, `semio-framework-io-control` for direction and run control, and `semio-os-channel` for the guest channel constant. Macro expansions must resolve public ABI re-exports through `$crate` so consumer crates inherit the same types through the ABI dependency. |

The five manifest changes can reuse the existing crate paths. The current describe manifest also declares the kernel as a separate mutation-testing development dependency. The production graph excludes development declarations. Retaining the test declaration leaves the kernel listed in describe's root lockfile row. Removing it would require validating that the mutation test suite no longer relies on its feature selection.

## Reachability evidence

The ticket [Bun manifest scanner](audit-describe-dependencies.mjs) substituted all five declarations in memory, using the exact seven dependencies listed for the ABI. Normal mode follows nonoptional dependency tables. Conservative mode also explores optional declarations and target and build dependency tables without evaluating feature or platform conditions. Both exclude development dependencies. Counts cover repository-local manifests.

| Graph | Current | Simulated | Kernel or store document after substitution |
| --- | ---: | ---: | --- |
| Normal | 341 | 297 | Neither reachable |
| Conservative | 358 | 305 | Neither reachable |

Both simulated graphs also omit `semio-os-store-member-open-store`, which owns the composition record source appearing in the retained tails of attempts 3 through 5. The store engine, engine core, engine live and engine bundle are present in both current graphs and absent in both simulations. The engine, core and live currently enter through the kernel. The engine itself brings in the bundle.

The current JSON pack, DSL and retained value original-slot owners remain reachable after the simulation. Those areas appear in other retained attempt tails, so this change would address one cluster of churn without making the emitter independent of concurrent edits. The emitter still needs to run component descriptor observation and owned codec verification under its interpreter.

## Lockfile scope and verification

The directly selected descriptor tool has one checked-in lockfile, the repository root. The production rewiring also changes consumers beyond that lock. Checked-in package rows appear in 176 lockfiles for guest contract, 176 for ABI, 21 for runtime and 11 for imports, with 176 unique lockfiles across the five names. A valid implementation would update the affected package edges for every consumer lock, accounting for existing development edges and any graph entries that become unreachable. The ABI target-specific and exported-macro consumers would require separate validation on host and guest. The descriptor differential and codec-pair tests cover the behavior the release must preserve.

Only this note and the ticket scanner were written. The simulation verified manifest reachability. Rust compilation, Cargo resolution and the runtime tests were not run.
