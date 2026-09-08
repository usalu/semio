# Framework Flow Artifact Package

The semantic schema audit found Flow documents bundled in the OS Flow host package. The Flow snapshot, typed parameters, mutations, codecs, structural diffs, and retained value ownership now have an independent artifact declaration at `🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust`; implementation stays in its domain taxonomy. Existing renderer translation helpers currently depend on Infinite, but there is no Flow host dependency. The host composes the artifact through private imports.

Test-first evidence: the AJV fixture schema passed and the missing standalone package assertion failed before declarations were implemented. Compiler and runtime tests are pending.

Direct consumer migration updated 118 Rust source files and 8 nearest owning manifests. The old host public artifact/parameter/retained mounts were replaced by private composition imports.

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml`

The first isolated framework Flow Nx test run failed while Infinite compilation used a Cargo dependency graph captured before the new DAG package was registered. The source now names that new dependency. This is an in-flight graph failure, not a passing current-graph test; rerun after the DAG package test is required.
