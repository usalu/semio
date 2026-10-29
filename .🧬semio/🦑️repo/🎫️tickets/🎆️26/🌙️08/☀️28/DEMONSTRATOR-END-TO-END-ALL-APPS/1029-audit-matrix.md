# 1029 Audit Matrix: wasm32-wasip2 check of the mit-Bestand activation closure

STATUS: RED. 25/25 batches finished. Failing batches (latest attempt): hub cad-aec-building cad-spatial-shape flow-brep playbook-procedural process-concrete process-metal process-robotic sourcing-beams sourcing-slabs sourcing-windows

## 1. Component to guest package to workspace

Components come from `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json` (11 pluginIds, 21 extensionIds). Plugin guests are the hub compositions (`semio-hub-<id>-component`, same as the 1028 cad lane). Extension guests are the `🧩️component` crate inside each extension's own Cargo workspace root. Framework targets resolve to the `componentDir` crate that `buildAdmittedWasmWebV1` compiles (the same pattern as `rsDir`/`componentDir` in each project's `📜️script.ts`).

Paths are repo-relative. Each workspace root and each component package was confirmed to exist and to be a member of its workspace (members are listed explicitly).

### 1.1 Plugins (hub workspace, batch `hub`)

| Cases id | Kind | Guest package(s) | Workspace manifest | Batch |
|---|---|---|---|---|
| cad | pluginId | `semio-hub-cad-component` | `🌎️hub/Cargo.toml` | hub |
| demonstrator | pluginId | `semio-hub-demonstrator-component`, bundle crate `semio-hub-demonstrator` | `🌎️hub/Cargo.toml` | hub |
| energy | pluginId | `semio-hub-energy-component` | `🌎️hub/Cargo.toml` | hub |
| fem | pluginId | `semio-hub-fem-component` | `🌎️hub/Cargo.toml` | hub |
| flow | pluginId | `semio-hub-flow-component` | `🌎️hub/Cargo.toml` | hub |
| gis | pluginId | `semio-hub-gis-component` | `🌎️hub/Cargo.toml` | hub |
| playbook | pluginId | `semio-hub-playbook-component` | `🌎️hub/Cargo.toml` | hub |
| procedural | pluginId | `semio-hub-procedural-component` | `🌎️hub/Cargo.toml` | hub |
| process | pluginId | `semio-hub-process-component` | `🌎️hub/Cargo.toml` | hub |
| puzzle | pluginId | `semio-hub-puzzle-component` | `🌎️hub/Cargo.toml` | hub |
| sourcing | pluginId | `semio-hub-sourcing-component` | `🌎️hub/Cargo.toml` | hub |

### 1.2 Framework wasm targets (batches `framework` and `hub`)

| Target | Guest package(s) | Workspace manifest | Batch |
|---|---|---|---|
| `framework-surface-rs:wasm` | `semio-framework-surface-component` (lib `semio-framework-surface`) | `🧰️framework/Cargo.toml` | framework |
| `framework-editor-rs:wasm` | `semio-framework-editor-component` (lib `semio-framework-editor`) | `🧰️framework/Cargo.toml` | framework |
| `framework-os-node-graph-rs:wasm` | `semio-framework-os-node-graph-component` (lib `semio-framework-os-node-graph`) | `🧰️framework/Cargo.toml` | framework |
| `puzzle-plugin:wasm` (`semio-hub-puzzle`) | `semio-hub-puzzle-component` (lib `semio-hub-puzzle`) | `🌎️hub/Cargo.toml` | hub |
| `semio-framework-os-flow-browser-package:wasm` | `semio-framework-os-flow-component` (lib `semio-framework-os-flow`). The project dir has no Cargo.toml of its own. Its script builds `componentDir` = `🌊️flow/🧩️component`. | `🧰️framework/Cargo.toml` | framework |

### 1.3 Extensions (one workspace root each, one batch each)

| Cases id | Guest package | Workspace manifest | Batch |
|---|---|---|---|
| cad-extension-aec-building | `semio-s-plugin-cad-aec-building-component` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/Cargo.toml` | cad-aec-building |
| cad-extension-aec-building-energy | `semio-s-plugin-cad-aec-energy-component` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/Cargo.toml` | cad-aec-energy |
| cad-extension-aec-building-structure | `semio-s-plugin-cad-aec-structure-component` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/Cargo.toml` | cad-aec-structure |
| cad-extension-spatial-shape | `semio-s-plugin-cad-spatial-shape-component` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/Cargo.toml` | cad-spatial-shape |
| flow-extension-bim | `semio-s-plugin-flow-bim-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.toml` | flow-bim |
| flow-extension-brep | `semio-s-plugin-flow-brep-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.toml` | flow-brep |
| flow-extension-dictionary | `semio-s-plugin-flow-dictionary-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.toml` | flow-dictionary |
| flow-extension-draw | `semio-s-plugin-flow-draw-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.toml` | flow-draw |
| flow-extension-list | `semio-s-plugin-flow-list-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.toml` | flow-list |
| flow-extension-logic | `semio-s-plugin-flow-logic-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.toml` | flow-logic |
| flow-extension-math | `semio-s-plugin-flow-math-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.toml` | flow-math |
| flow-extension-primitive | `semio-s-plugin-flow-primitive-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.toml` | flow-primitive |
| flow-extension-text | `semio-s-plugin-flow-text-component` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.toml` | flow-text |
| playbook-module-procedural | `semio-s-plugin-playbook-procedural-component` | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.toml` | playbook-procedural |
| process-extension-concrete | `semio-s-plugin-process-concrete-component` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.toml` | process-concrete |
| process-extension-metal | `semio-s-plugin-process-metal-component` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.toml` | process-metal |
| process-extension-robotic | `semio-s-plugin-process-robotic-component` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.toml` | process-robotic |
| process-extension-wood | `semio-s-plugin-process-wood-component` | `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.toml` | process-wood |
| sourcing-module-beams | `semio-s-plugin-sourcing-beams-component` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.toml` | sourcing-beams |
| sourcing-module-slabs | `semio-s-plugin-sourcing-slabs-component` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.toml` | sourcing-slabs |
| sourcing-module-windows | `semio-s-plugin-sourcing-windows-component` | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.toml` | sourcing-windows |

Skip rule: none of the 23 workspaces above is a 3d puzzle workspace (`🧩️puzzle/🗿️artifacts/🧊️3d`), so the skip is a no-op for this matrix. The hub batch does include `semio-hub-puzzle` (the `puzzle-plugin:wasm` target), so the hub batch covers the puzzle plugin itself.


## 2. Results per batch

Each batch is one `repo cargo ... check --manifest-path <ws> --target wasm32-wasip2 --keep-going -p ...` run, logged to `🗑️generated/1029/matrix-<batch>.txt` with rc in `.rc`. A `-r2` log is the second attempt for that batch and supersedes the first. Crate rows come from cargo's `could not compile` lines. Components that depend on a failing crate are blocked by cargo (no own compile result) and are not listed as failing crates.

### hub

Result: rc=1, RED

| crate | errors (cargo count) |
|---|---|
| `semio-framework-plugin-time-travel-turns` | 15 |
| `semio-framework-plugin-instance-typed-ops` | 16 |
| `semio-s-fem-node` | 2 |
| `semio-s-fem-material` | 2 |
| `semio-s-artifact-procedural-2d-editor-config` | 7 |
| `semio-s-artifact-procedural-2d-editor-presence` | 5 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-framework-plugin-instance-typed-ops` (4): error[E0277]: the trait bound `M: store::ReferenceSpaceMember` is not satisfied at `products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📦️packages/🦀️rust/../../📥️staging/📬️publication/👶️child-operation-unit/🦀️.rs:12:82`
- `semio-framework-plugin-instance-typed-ops` (4): error[E0277]: the trait bound `OriginalStoreSlot<<A as ArtifactApp>::Snapshot, _>: SpaceMember` is not satisfied at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📦️packages/🦀️rust/../../📥️staging/🪆️child-group/🦀️.rs:149:37`
- `semio-framework-plugin-instance-typed-ops` (2): error[E0277]: the trait bound `semio_framework::Fault: From<MemberValueError>` is not satisfied at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📦️packages/🦀️rust/../../📥️staging/🪆️child-group/🦀️.rs:96:176`
- `semio-framework-plugin-time-travel-turns` (3): error[E0308]: mismatched types at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/📦️packages/🦀️rust/../../🏃️turn-driving-and-replay-steps/🦀️.rs:132:257`
- `semio-framework-plugin-time-travel-turns` (2): error[E0004]: non-exhaustive patterns: `semio_framework_plugin_time_travel::TimeTravelQueryOutput::CommitCloseDemand(_)` not covered at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/📦️packages/🦀️rust/../../📖️ref-reads/🦀️.rs:10:15`
- `semio-framework-plugin-time-travel-turns` (2): error[E0277]: the trait bound `M: store::ReferenceSpaceMember` is not satisfied at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/📦️packages/🦀️rust/../../🏃️turn-driving-and-replay-steps/🦀️.rs:141:82`
- `semio-s-artifact-procedural-2d-editor-config` (6): error[E0433]: cannot find module or crate `semio_framework_pack_json` in this scope at `s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📦️packages/🦀️rust/../../🦀️.rs:98:26`
- `semio-s-artifact-procedural-2d-editor-config` (1): error[E0277]: the trait bound `Generation2dConfigMutation: DslOwnedReceivingVariants` is not satisfied at `s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📦️packages/🦀️rust/../../🦀️.rs:174:1`
- `semio-s-artifact-procedural-2d-editor-presence` (2): error[E0277]: the trait bound `CameraJson: NativeDefault` is not satisfied at `���️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/📦️packages/🦀️rust/../../🦀️.rs:19:177`
- `semio-s-fem-material` (2): error[E0433]: cannot find `semio_framework_value_snapshot_projection` in the crate root at `✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/📦️packages/🦀️rust/../../🦀️.rs:17:94`
- `semio-s-fem-node` (2): error[E0433]: cannot find `semio_framework_value_snapshot_projection` in the crate root at `✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/📦️packages/🦀️rust/../../🦀️.rs:11:94`

### framework

Result: rc=0, GREEN (latest attempt framework-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### root-flow

Result: rc=0, GREEN

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### root-nodegraph

Result: rc=0, GREEN (latest attempt root-nodegraph-r2)

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### cad-aec-building

Result: rc=1, RED (latest attempt cad-aec-building-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| `semio-s-artifact-cad-cad-model-diff` | 2 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-s-artifact-cad-cad-model-diff` (2): error[E0433]: cannot find `semio_framework_diagnostic` in the crate root at `rtifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/../.././././././././../../🚪️io/📝️text/🔺️diff/🦀️.rs:11:1`

### cad-aec-energy

Result: rc=0, GREEN (latest attempt cad-aec-energy-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### cad-aec-structure

Result: rc=0, GREEN

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### cad-spatial-shape

Result: rc=1, RED (latest attempt cad-spatial-shape-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| `semio-framework-plugin-guest-contract` | 3 |
| `semio-framework-plugin-registration` | 1 |
| `semio-framework-2d-text` | 1 |

Top distinct errors per crate (up to 3, count, message, first location):

- `(unattributed)` (1): error: Original resource loan deadline exhausted at ``
- `(unattributed)` (1): error: failed to write `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/pef1792/b/wasm32-wasip2/prototype/build/semio-framework-plugin-instance-s... at ``
- `semio-framework-2d-text` (1): error: extern location for semio_framework_value does not exist: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/pef1792/b/wasm32-wasip2/prototyp... at `🧰️framework/🔨️modules/◻️2d/📝️text/📦️packages/🦀️rust/../../🦀️.rs:23:53`
- `semio-framework-plugin-guest-contract` (1): error: extern location for semio_framework_value_derive does not exist: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/pef1792/b/prototype/build... at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️guest-contract/📦️packages/🦀️rust/../../🦀️.rs:6:5`
- `semio-framework-plugin-guest-contract` (1): error[E0463]: can't find crate for `semio_framework_os_kernel` at `���️os/🔨️modules/🔌️plugin/🧬️guest-contract/📦️packages/🦀️rust/../../🎟️original-actor-context/../../⚛️reactor/🎟️context/🌉️call/📤️failure/🦀️.rs:5:5`
- `semio-framework-plugin-guest-contract` (1): error[E0463]: can't find crate for `semio_framework_plugin_abi` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️guest-contract/📦️packages/🦀️rust/../../🦀️.rs:10:9`
- `semio-framework-plugin-registration` (1): error[E0463]: can't find crate for `semio_framework` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/📜️registration/📦️packages/🦀️rust/../../🦀️.rs:2:5`

### flow-bim

Result: rc=0, GREEN (latest attempt flow-bim-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### flow-brep

Result: rc=1, RED

| crate | errors (cargo count) |
|---|---|
| `semio-s-artifact-stdio-step-snapshot` | 18 |
| `semio-framework-os-flow-extensions` | 1 |
| `semio-s-artifact-stdio-semio-brep-viewer` | 3 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-framework-os-flow-extensions` (1): error[E0308]: mismatched types at `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/📦️packages/🦀️rust/../../🗃️evaluation-output-and-retained-evaluation/🦀️.rs:22:236`
- `semio-s-artifact-stdio-semio-brep-viewer` (2): error[E0277]: the trait bound `SemioBrepSnapshot: NativeReceive<RecordValue>` is not satisfied at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👁️viewer/📦️packages/🦀️rust/🦀️.rs:128:1`
- `semio-s-artifact-stdio-semio-brep-viewer` (1): error[E0277]: the trait bound `SemioBrepSnapshot: BorrowedDslRecord` is not satisfied at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👁️viewer/📦️packages/🦀️rust/🦀️.rs:128:1`
- `semio-s-artifact-stdio-step-snapshot` (18): error[E0433]: cannot find module or crate `semio_framework_pack_json` in this scope at `�plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust/././././././../../🦀️.rs:23:26`

### flow-dictionary

Result: rc=0, GREEN (latest attempt flow-dictionary-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### flow-draw

Result: rc=0, GREEN

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### flow-list

Result: rc=0, GREEN

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### flow-logic

Result: rc=0, GREEN

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### flow-math

Result: rc=0, GREEN

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### flow-primitive

Result: rc=0, GREEN

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### flow-text

Result: rc=0, GREEN

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### playbook-procedural

Result: rc=1, RED (latest attempt playbook-procedural-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| `semio-s-plugin-flow-brep-mesh` | 4 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-s-plugin-flow-brep-mesh` (1): error[E0061]: this method takes 1 argument but 2 arguments were supplied at `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/📦️packages/🦀️rust/../../🧰️mesh-operator-io/🦀️.rs:177:19`
- `semio-s-plugin-flow-brep-mesh` (1): error[E0277]: `?` couldn't convert the error to `neural_engine::EvalError` at `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/📦️packages/🦀️rust/../../🧰️mesh-operator-io/🦀️.rs:177:42`
- `semio-s-plugin-flow-brep-mesh` (1): error[E0423]: cannot initialize a tuple struct which contains private fields at `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/📦️packages/🦀️rust/../../📇️registration/🦀️.rs:159:195`

### process-concrete

Result: rc=1, RED

| crate | errors (cargo count) |
|---|---|
| `semio-s-artifact-stdio-step-mutations` | 21 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-s-artifact-stdio-step-mutations` (21): error[E0433]: cannot find module or crate `semio_framework_pack_json` in this scope at `dio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust/././././././../../🧩insert-entity/🦀️.rs:8:26`

### process-metal

Result: rc=1, RED (latest attempt process-metal-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| `semio-s-artifact-process-process3d-model` | 2 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-s-artifact-process-process3d-model` (1): error: cannot find attribute `dyn_enum` in this scope at `ess/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/../.././📸️snapshot/🪚️machines-and-catalogs/🦀️.rs:282:3`
- `semio-s-artifact-process-process3d-model` (1): error: cannot find macro `__semio_dispatch_MachineCatalog` in this scope at `ss/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/../.././📸️snapshot/🪚️machines-and-catalogs/🦀️.rs:302:31`

### process-robotic

Result: rc=1, RED (latest attempt process-robotic-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| `semio-s-artifact-stdio-step-cc4` | 7 |
| `semio-s-artifact-stdio-step-inferences` | 6 |
| `semio-s-artifact-stdio-semio-flow` | 11 |
| `semio-s-artifact-stdio-semio-brep` | 1 |

Top distinct errors per crate (up to 3, count, message, first location):

- `(unattributed)` (1): error: failed to write `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/p3aa732/b/wasm32-wasip2/prototype/build/semio-s-artifact-process-process3... at ``
- `(unattributed)` (1): error: failed to write `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/p3aa732/b/wasm32-wasip2/prototype/build/semio-s-artifact-stdio-step-geome... at ``
- `semio-s-artifact-stdio-semio-brep` (1): error[E0463]: can't find crate for `serde_derive` which `semio_framework_os_kernel` depends on at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/📦️packages/🦀️rust/../../🦀️.rs:7:1`
- `semio-s-artifact-stdio-semio-flow` (2): error[E0463]: can't find crate for `semio_framework_os_kernel` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/📦️packages/🦀️rust/../../🦀️.rs:8:1`
- `semio-s-artifact-stdio-semio-flow` (1): error: extern location for semio_framework_value_derive does not exist: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/p3aa732/b/prototype/build... at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/📦️packages/🦀️rust/../../🦀️.rs:11:1`
- `semio-s-artifact-stdio-semio-flow` (1): error: extern location for semio_s_artifact_stdio_semio_flow_snapshot does not exist: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/p3aa732/b/w... at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/📦️packages/🦀️rust/../../🦀️.rs:21:17`
- `semio-s-artifact-stdio-step-cc4` (1): error[E0463]: can't find crate for `semio_framework_plugin` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/📦️packages/🦀️rust/././././../../🚪️io/🦀️.rs:15:10`
- `semio-s-artifact-stdio-step-cc4` (1): error[E0463]: can't find crate for `semio_framework_schema_derive` which `semio_framework_schema` depends on at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/📦️packages/🦀️rust/🦀️.rs:9:1`
- `semio-s-artifact-stdio-step-cc4` (1): error[E0463]: can't find crate for `semio_s_artifact_stdio_contract` at `�️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/📦️packages/🦀️rust/././././../../🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs:9:5`
- `semio-s-artifact-stdio-step-inferences` (2): error[E0463]: can't find crate for `semio_framework_os_kernel` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust/🦀️.rs:7:1`
- `semio-s-artifact-stdio-step-inferences` (1): error: extern location for semio_framework_value_derive does not exist: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/p3aa732/b/prototype/build... at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust/🦀️.rs:10:1`
- `semio-s-artifact-stdio-step-inferences` (1): error[E0463]: can't find crate for `semio_s_artifact_stdio_step_snapshot` at `plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust/././././././../../🦀️.rs:11:5`

### process-wood

Result: rc=0, GREEN (latest attempt process-wood-r2; first attempt rc=1)

| crate | errors (cargo count) |
|---|---|
| (none) | no `could not compile` line in log |

Top distinct errors per crate (up to 3, count, message, first location):

- (no error lines; see log for runner failures)

### sourcing-beams

Result: rc=1, RED

| crate | errors (cargo count) |
|---|---|
| `semio-s-artifact-stdio-semio-object-snapshot` | 4 |
| `semio-s-artifact-stdio-semio-presentation-snapshot` | 10 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-s-artifact-stdio-semio-object-snapshot` (1): error[E0277]: the size for values of type `(dyn NativeValueSource + 'static)` cannot be known at compilation time at `/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/📦️packages/🦀️rust/././../../🦀️.rs:29:131`
- `semio-s-artifact-stdio-semio-presentation-snapshot` (2): error[E0277]: the trait bound `schema::snapshot::component::PlaceholderKind: DslField` is not satisfied at `️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/📦️packages/🦀️rust/././../../🦀️.rs:97:44`
- `semio-s-artifact-stdio-semio-presentation-snapshot` (1): error[E0277]: the trait bound `schema::snapshot::component::PlaceholderKind: BorrowedDslField` is not satisfied at `️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/📦️packages/🦀️rust/././../../🦀️.rs:97:44`

### sourcing-slabs

Result: rc=1, RED

| crate | errors (cargo count) |
|---|---|
| `semio-s-artifact-stdio-semio-object-examples` | 1 |
| `semio-s-artifact-stdio-semio-presentation-mutations` | 1 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-s-artifact-stdio-semio-object-examples` (1): error: ArtifactChild field requires an explicit #[child(kind = "…")] or #[child(capability = "…")] at `🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/📚️examples/📦️crate/📦️packages/🦀️rust/././../../../../🧬️schema/🔺️diff/🦀️.rs:25:5`
- `semio-s-artifact-stdio-semio-presentation-mutations` (1): error[E0275]: overflow evaluating the requirement `std::ptr::Unique<semio_framework_value_native_owned::owned::tree::NativeReceiveSlot<(dyn semio_framework_value_native_owned::o... at `�️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/📦️packages/🦀️rust/🦀️.rs:58:1`

### sourcing-windows

Result: rc=1, RED

| crate | errors (cargo count) |
|---|---|
| `semio-s-artifact-stdio-semio-object-examples` | 1 |
| `semio-s-artifact-stdio-semio-presentation-mutations` | 1 |

Top distinct errors per crate (up to 3, count, message, first location):

- `semio-s-artifact-stdio-semio-object-examples` (1): error: ArtifactChild field requires an explicit #[child(kind = "…")] or #[child(capability = "…")] at `🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/📚️examples/📦️crate/📦️packages/🦀️rust/././../../../../🧬️schema/🔺️diff/🦀️.rs:25:5`
- `semio-s-artifact-stdio-semio-presentation-mutations` (1): error[E0275]: overflow evaluating the requirement `std::ptr::Unique<semio_framework_value_native_owned::owned::tree::NativeReceiveSlot<(dyn semio_framework_value_native_owned::o... at `�️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/📦️packages/🦀️rust/🦀️.rs:58:1`

## 3. Notes for the coordinator

- Workspace keys: the flow family (`semio-framework-os-flow`, `semio-framework-os-flow-component`) and the node-graph crates (`semio-framework-os-node-graph`, `semio-framework-os-node-graph-component`) declare the repo root (`/Cargo.toml`) as their `workspace`, not `🧰️framework/Cargo.toml`. Checking them from the framework manifest fails with "package ID specification ... did not match any packages", which aborted the first framework batch before any compile. Their batches therefore use the root manifest (`root-flow`, `root-nodegraph`). The surface and editor crates resolve to the framework workspace and are checked there (`framework`).
- Reruns: the first framework batch was superseded by `framework-r2`. Batches with a `-r2` log had their first attempt fail only in shared framework crates or in infrastructure (`ENOSPC` on a lock file, and a jobserver `mkfifo refused`, both transient). Their `-r2` log is the latest attempt.
- Not rerun: `hub`, `flow-brep`, `process-concrete`, `sourcing-beams`, `sourcing-slabs`, `sourcing-windows` (their failing crates include non-framework plugin or stdio crates, so they do not qualify under the rule), and `process-robotic-r2`/`process-metal-r2`/`playbook-procedural-r2`/`cad-aec-building-r2` (their latest attempt fails on non-framework crates).
- Blocked components: cargo does not report a compile result for a component whose dependency failed under `--keep-going`. A `-p` package with no own error line is therefore GREEN only if it is listed as `rc=0` by its batch. For a batch that is RED with no crate of that component in the list, the component's status is "blocked" or "green", not verified. Transitive closure was not computed.
- Source edits: none. The auditor did not edit source code, `AGENTS.md`, or the ticket state.
