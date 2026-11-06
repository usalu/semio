# Cargo Normalization Result

The first pass changed 31 manifests, including nine target package names; the raster workspace pass changed three more. Five identical root workspace duplicate mappings were removed. Explicit custom aliases were preserved. The first-pass generated file list was inadvertently overwritten by a no-op second pass; the list below reconstructs 30 of the 31 first-pass files from the write timestamp interval and includes the root manifest. One first-pass file remains unidentified, and concurrent edits after the scan may affect attribution.

Fresh read-only TOML census: 4462 manifests, 0 direct and workspace path package-name mismatches. This census does not validate feature references or Rust imports.

`repo tools exec cargo -- metadata --locked --offline --format-version 1` failed before dependency resolution: the feature `durable-group-testing` in the store one-item-owners manifest references the old `semio-framework-os-store-one-item-publication/durable-group-testing` dependency name. Rust imports and manifest feature references require follow-up. No native tests were run.

## Manifest files changed

- Cargo.toml
- ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🎛️live-editor/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪺️inspector-forms/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️session/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/✏️editor/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/📦️packages/🦀️rust/Cargo.toml
- 🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎛️shell-channel/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔭️inference-tools/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📥️receiving/📝️forms/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🧬️codec/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🧪️tests/🪟️native-window/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🏪️stores/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧱️construction/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍎️tests-01/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🥝️tests-16/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🪝️tests-12-2/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧩️expand/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.toml
- ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml

## Feature Reference Pass

2 manifests changed. 0 unresolved observations.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/📦️packages/🦀️rust/Cargo.toml



Offline locked metadata passed the previous manifest-feature error and then failed during the configured compiler version probe: compiler ingress refused access without the repository launcher and authentic enforcing store. Registry and lock resolution were not reached.
