# Descriptor Emitter Host Dependencies

The pinned Cargo offline locked unit graph for `semio-plugin-describe --profile release --bin semio-plugin-describe` selected 596 Darwin host units. This is the actual production selection without the Wasmtime differential dev dependencies. Cargo selected both the emitter binary and library with a direct dependency on `semio-framework` and `semio-os-kernel` in each.

| Graph observation | Selected units | Difference |
| --- | ---: | ---: |
| Actual current graph | 596 | 0 |
| Remove only all four emitter-owned umbrella edges | 596 | 0 |
| Disconnect every framework umbrella edge without restoring needed owners | 579 | −17 |
| Disconnect every OS kernel umbrella edge without restoring needed owners | 318 | −278 |
| Reconnect likely narrower OS owners while keeping the framework facade | 531 | −65 |
| Reconnect mapped owners after disconnecting both umbrellas | 521 | −75 |

The last two rows are reachability projections from the original graph, with the original feature sets. A fresh Cargo build would be needed to establish the exact savings after changing manifests. Every one of the six selected framework geometry units remains in both projections. No BREP or framework 3D unit is in the original emitter graph.

The emitter uses the manifest plugin's canonical descriptor and assembly failure ID, kernel identity's budget, the actor's retained-turn values and the OS pack bridge. It could import those owners directly. It would still select both facades through the guest contract, plugin ABI and interpreter runtime. Actor input also retains the framework facade. Interpreter imports separately retain the OS kernel.

The narrow OS projection reconnects the existing Pack, IO, store codec and text IO, entity identity, platform entropy, channel and replication mutation owners. For example, the guest contract's `IoKey` originates in the store codec registry. The interpreter's channel admission originates in the OS channel crate, and its authority signatures use the original VCS entity identity owner. The plugin ABI also consumes snapshot and mutation traits, codecs and IO vocabulary. Moving just the emitter's direct imports cannot eliminate those routes.

The geometry dependency has an independent path from the guest contract through kernel JSON, the kernel, UI declarations, UI scene and scene algebra. The manifest's app-action schema also reaches the scene geometry types. The DSL derive remains through the store codec registry and SPR in the narrowed projection. Replacing umbrellas therefore leaves the two dependency classes that have changed during recent emitter runs.

This was a read-only audit. The host graph and Cargo transcript are under `🗑️generated/descriptor-emitter-native-units.json` and `🗑️generated/descriptor-emitter-native-plan.log`. The inspected source selection starts at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/Cargo.toml`.
