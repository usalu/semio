# Inferred Native Test Input Coverage

## Confirmed Native Hash Gap

The native Node/Nx hash probe completed in 35.9s without executing any test or compiler. It selected the inferred DOCX transitional mutation subject target. Native Nx collected **556 file inputs** for hash `5554831827136213098`. None of the **35** checked compilation manifests were included: the workspace `Cargo.toml`, `rust-toolchain.toml`, the subject manifest and 32 additional local dependency manifests. The collected paths were normalized to workspace-relative paths; sample inputs include `.gitignore`, `.nxignore` and `nx.json`.

The test runner selects the nearest ancestor `📦️packages/🦀️rust/Cargo.toml` in `rustSutCrate`, then links it from its generated host. For this case that subject is the DOCX artifact package, outside the narrower transitional subset owner. Its local dependency closure was computed by the same Cargo resolver already qualified against native Cargo metadata in the cache contracts. The probe establishes a missing-input defect; it does not claim a real application test cold/warm run or a compiler execution.

Probe: `🔬️native-inventory/🦀️test-inputs/📜️script.ts`. Logs/data: `🗑️generated/native-inferred-rust-test-inputs-node.log` and `🗑️generated/native-inferred-rust-test-inputs.json`. Run native Nx graph/hash APIs with Node inside a selected Nx target; Bun graph construction failed because its worker could not require the core plugin's top-level-await ESM module.

## Required Repair

Share subject/host/contributed-package selection between runtime execution and Nx inference. Add the complete selected native source and toolchain inputs, preserve phase distinctions (oracle-only work must not build the subject), attach existing generator prerequisites in the outer task graph, and expose project dependency edges for affected selection. Native `nativeSources` named inputs and `nativeDependencyRoots`/`nativePreparation` already provide most Cargo dependency logic. Do not recursively invoke the core project normalizer or another repository-wide scheduler from the test plugin.

The test plugin currently hashes only the case, subset owner, contribution globs, testing-domain files and shared globals. It adds no Cargo dependency edges or prerequisites. The runtime also creates standalone native host manifests and builds without `--locked`; immutable host dependency resolution and artifact ownership need qualification before claiming complete test caching.

## Implementation Notes

- Test plugin: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs`.
- Runtime selection and host materialization: the same domain's `📜️script.ts`, `rustSutCrate` and `materializeRustHost`.
- Core Cargo dependency helpers and native input groups: library `🟨️.mjs`. Generator contracts are source data in taxonomy `generatorContracts`.
- `oracleHostPackagesFor` includes contributions from **all matching ancestors**, despite its nearest-contributor docstring. Contribution directories support owner-specific taxonomy overrides. A per-owner ancestry walk can resolve relevant package declarations without scanning the entire repository. Preserve this behavior when sharing selection code.
- Core project identifiers come from an authored `📋️project.json` name, otherwise the Cargo package name. Referencing native projects' named inputs must use those identifiers, not crate aliases.

## Checked Manifests

- `Cargo.toml` — missing
- `rust-toolchain.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/Cargo.toml` — missing
