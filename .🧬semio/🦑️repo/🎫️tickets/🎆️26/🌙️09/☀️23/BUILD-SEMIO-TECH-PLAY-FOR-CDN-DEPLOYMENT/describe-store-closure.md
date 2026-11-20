# Why the host descriptor build admits store document history

The second recorded exit-75 attempt for run `wCsXqmg` reports `cargo:source-digest-mismatch` on the store document hydration fold. Its retained log is `.cache/b/r/m/components/runs/wCsXqmg/l/job-describe-tool.log.2.tail`. The mismatch occurred during source admission and rechecking for the host emitter. A crate can invalidate that build through its declared sources even if the binary never calls that code.

## Production dependency paths

The emitter selects the `semio-plugin-describe` binary in `plugin/describe/emitter/contract`. A repository-local Cargo manifest scan found five incoming normal nonoptional edges to `semio-os-kernel` from the emitter and its reachable dependencies. The following trails include each incoming edge.

| Trail from `semio-plugin-describe` | Kernel dependency declaration |
| --- | --- |
| `describe → os-kernel` | `plugin/describe/packages/rust/Cargo.toml:35` |
| `describe → guest-contract → os-kernel` | `plugin/guest-contract/packages/rust/Cargo.toml:33` |
| `describe → guest-contract → plugin-abi → os-kernel` | `plugin/io/packages/rust/Cargo.toml:30` |
| `describe → interpreter-runtime → os-kernel` | `plugin/interpreter/runtime/packages/rust/Cargo.toml:28` |
| `describe → interpreter-runtime → interpreter-imports → os-kernel` | `plugin/interpreter/io/imports/packages/rust/Cargo.toml:24` |

The declarations above are paths under the OS modules tree with decorated path names omitted. The OS package manifest declares `semio-os-store-document` as an unconditional dependency at line 76. The document crate mounts history hydration in its root module and hydration mounts the reported `fold` source at line 50. Both mounts are unconditional. The describe-only graph therefore admits this source without enabling descriptor integration tests, Wasmtime, or store synchronization.

The static scan resolved local path and workspace dependencies and stopped at kernel or document. It inspected 243 reachable manifests when following normal nonoptional declarations. A second scan followed optional and target-specific normal and build dependencies as well, conservatively ignoring feature conditions, and inspected 250 manifests. It found the same five incoming edges and no route to the document crate bypassing the kernel. Development dependencies were excluded from both scans. Verification covered manifest and source inspection.

## Existing narrow owners

- The descriptor emitter’s direct kernel import supplies only `store::pack_rt` in `plugin/describe/descriptor-emission`. `semio-os-pack` owns that module directly and the kernel re-exports it. Using the pack crate would preserve the encoder/decoder, canonical value type, and wire output. The existing alias can preserve the `store::pack_rt` call spelling checked by the unit test.
- `semio-framework-manifest-plugin` owns the canonical descriptor and package descriptor types re-exported by `semio-framework`. `semio-framework-kernel-identity` owns `Budget`. The main framework crate had no normal nonoptional path to store document history in the static scan. Narrowing these imports alone would leave document history in the graph.
- In the guest contract, the public `GuestCodecPairDecl.key` type is the existing `IoKey` owned by `semio-os-store-codec-registry` through its `lifted_io` module. The kernel re-exports the same type.
- Interpreter runtime and interpreter imports also obtain entity identity, channel version admission, and identity entropy through the kernel. Their existing narrow owners include `semio-os-vcs-entity-identity`, `semio-os-channel`, and `semio-os-identity`. The ABI’s kernel edge has wider usage, including protocol and pack names, receiving authority and public `ArtifactStoreSnapshot` bounds. The snapshot trait is defined by `semio-os-store-text-io`. ABI macro call sites and their paths would need verification in a full change.

The existing emitter compiles the component in `OwnedRuntime`, obtains the observed descriptor, and reuses that component for owned codec verification. Those steps are part of descriptor emission. Dropping the direct kernel dependency would leave the other four paths intact. Removing document history from production source admission requires cutting all five paths while retaining the same public types and runtime observations.
