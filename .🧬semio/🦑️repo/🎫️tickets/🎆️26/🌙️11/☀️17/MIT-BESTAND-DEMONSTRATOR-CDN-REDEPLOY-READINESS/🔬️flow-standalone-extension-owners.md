# Standalone Flow Extension Ownership

The standalone release producer compiles `semio-os-flow-component` as a `wasm32-unknown-unknown` dynamic library using the repository root lock. It links the Flow family, which explicitly links the browser Wasm implementation. The original locked Cargo inverse graph found one route from standalone Flow into actor input:

```text
semio-os-flow-component
  → semio-os-flow
  → semio-os-flow-extensions
  → semio-plugin
  → semio-plugin-guest-contract
  → semio-plugin-actor-input
  → semio-plugin-actor-input-wire
```

Flow's evaluation implementation used three type names from the SDK facade. The extension crate owns `ExtensionResourceOwner` and `ExtensionInvokeStep`, and the ownership crate owns `PluginLifecycleStep`. The unit test's `ArtifactInferenceGateway` also comes from the extension crate. The facade re-exports these same Rust definitions. Evaluation still implements the existing owner and advances the same owned request and retirement steps. The standalone browser does not implement the plugin actor input path.

The release producer's authored tests first failed on the actual manifest's old umbrella dependency. The built-in TOML parser identifies each actual source owner manifest. After switching to those two owners, both producer tests passed with 24 assertions, covering the seven exact C bridge exports required by the current public schema and the original producer and generated-bindings boundaries. The browser assembly and Flow family, bridge implementation, neural bindings and publication schema were outside the production edits.

The genuine pinned offline Cargo `--locked --unit-graph` produced a release `cdylib` root with 938 units and 685 distinct targets. It includes the Flow Wasm bridge and both owner crates. There are zero units for the umbrella SDK, plugin actor input or actor input wire. The old inverse graph and new exact unit plan are in `🗑️generated/flow-actor-input-chain.log` and `🗑️generated/flow-standalone-owned-unit-graph.json`. Build source tests are in `🗑️generated/flow-evaluation-owners-{red,green}.log`. The managed producer and proof of the exact new published bytes run separately through the browser owner.

The root lock changed together with the standalone inputs. The live FEM and GIS guest graphs and host descriptor graph had no Flow extension evaluation unit. Their process still used the live Hub lock, so the other rows waited until that guest terminated and its owner released the locks. The exact transaction verified both replacement owners already existed in every applicable lock, changed 41 package records, and found the root and Hub records already exact. A post-transaction structural scan after the independent actor decoder lock change found all 43 records exact and no remaining old record. See `🗑️generated/flow-evaluation-lock-{preflight,apply,verified}.log`.

Production paths:

- `Cargo.lock`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧩️extension-resources-topic-and-command/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/📥️request/🦀️.rs`

Test paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🏗️build/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🏗️build/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🏗️build/🧪️tests/🟦️.ts`
