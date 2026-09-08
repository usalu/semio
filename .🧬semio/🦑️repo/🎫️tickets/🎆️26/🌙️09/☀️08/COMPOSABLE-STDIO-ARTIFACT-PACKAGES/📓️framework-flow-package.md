# Framework Flow Artifact Package

The semantic schema audit found Flow documents bundled in the OS Flow host package. The Flow snapshot, typed parameters, mutations, codecs, structural diffs, and retained value ownership now have an independent artifact declaration at `🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust`; implementation stays in its domain taxonomy. Pure renderer translation helpers depend on the extracted DAG data contract; node sizing remains in the Flow host. The host composes the artifact through private imports.

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

The final Infinite runtime dependency was isolated to host widget-size measurement. That computation now belongs to FlowHost; the pure widget-to-node data projection takes its measured dimensions explicitly. Existing three host calls calculate the same dimensions, while the package label-law test supplies a fixed size because it asserts authored label preservation. Flow artifact no longer depends on Infinite or semio-framework; only the lower DAG data/graph/neural contracts remain. Fresh compilation is pending.

## Selected Runtime Dependency Gate

The current `cargo tree -p semio-framework-artifact-flow-flow --edges normal` result resolves the Flow document through the DAG artifact, kernel, graph, neural engine, replication, and styling contracts. It contains no framework Flow/Infinite host, OS host, plugin composition crate, or rendering runtime. This gate selects the package defaults independently, avoiding the workspace-wide feature union.

The host-dependent node sizing function now belongs to the existing Flow host. The pure artifact projection accepts the computed dimensions; three host callers preserve their previous sizing behavior. The artifact label test supplies explicit dimensions. Public pure metadata helpers support the host calculation without an artifact-to-host dependency. Runtime test and host compiler gates remain pending.

## First Complete Standalone Runtime Run

The Nx package target compiled and executed 37 tests: 29 passed, including the new language-neutral package case and serde_json oracle; eight failed. Five copy tests incorrectly loaded their case catalog as source content after the shared fixture was separated. Their include now points to the canonical retained fixture and the neutral sourceFixture reference matches. Three exact JSON comparisons used integer lexemes for fields declared as f64; the typed graph-parameter, slider and layout/camera fixture values now retain floating-point lexemes. Dynamic payload numbers and integer wire indices are unchanged. The runtime retry is `framework-four-artifacts-runtime-test-2.txt`.

The adjacent Playbook and Space Nx targets did not run tests because ongoing manifest edits made the locked graph stale. Those failures provide no runtime evidence. The combined offline runtime diagnostic resolves the current graph; final Nx verification remains required.

The next real run reached 35/37 passing. The five selected-copy tests and graph-parameter fixture now pass. The remaining nested cluster fixture needed the same declared f64 lexemes in camera/layout fields. The diff rejection test exposed a real handwritten FlowCollectionDelta decoder bug: it silently defaulted missing required removed/inserted/replaced fields. The shared value derive now supports generics, so the duplicated handwritten codecs were replaced by strict ToValue/FromValue derives aligned with the adjacent schema and serde. The existing negative fixture test now independently verifies required-field/unknown-field rejection with typed serde_json too. Fresh runtime retry: `framework-four-artifacts-runtime-test-4.txt`, with no-fail-fast so all four packages report results.

The final native runtime check passed all 37 Flow artifact tests. This includes selected-copy fixture loading, numeric kinds, required collection-delta fields, and rejection of unknown fields against the existing serde oracle. See the combined final check recorded in `📓️validation.md`.
