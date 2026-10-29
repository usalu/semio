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

