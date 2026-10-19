# Domain and Dashboard Manifest Edges — Source Only

This inventory reads current handcrafted manifests and HEAD. No dependency resolver, build or lock regeneration ran. Alias/package names are resolved through the current root workspace declaration where applicable.

## Changed Package Edges

### `semio-s-fem-node`

`✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-dsl` → `semio-framework-dsl` {"path": "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-value` → `semio-framework-value` {"path": "../../../../../../../🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-pack-json` → `semio-framework-pack-json` {"path": "../../../../../../../🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-replication` → `semio-framework-replication` {"path": "../../../../../../../🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-dsl-record` → `semio-framework-dsl-record` {"path": "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-dsl-record-derive` → `semio-framework-dsl-record-derive` {"path": "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"version": "1.0.140"}
- `dev-dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}

### `semio-s-fem-material`

`✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-dsl` → `semio-framework-dsl` {"path": "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-dsl-record` → `semio-framework-dsl-record` {"path": "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-pack-json` → `semio-framework-pack-json` {"path": "../../../../../../../🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-value` → `semio-framework-value` {"path": "../../../../../../../🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-dsl-record-derive` → `semio-framework-dsl-record-derive` {"path": "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-replication` → `semio-framework-replication` {"path": "../../../../../../../🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"version": "1"}
- `dev-dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-results-state`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-artifact-fem-3d-scene` → `semio-s-artifact-fem-3d-scene` {"path": "../../../../../../../../../../../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🎬️scene/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-analyses-model` → `semio-s-fem-analyses-model` {"path": "../../../../../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust"}
- `dependencies`: `pack` → `semio-framework-pack` {"workspace": true}
- `dependencies`: `semio-framework-diagnostic` → `semio-framework-diagnostic` {"workspace": true}
- `dependencies`: `semio-framework-dsl-record` → `semio-framework-dsl-record` {"workspace": true}
- `dependencies`: `semio-framework-dsl-record-derive` → `semio-framework-dsl-record-derive` {"workspace": true}
- `dependencies`: `semio-framework-os-kernel` → `semio-framework-os-kernel` {"workspace": true}
- `dependencies`: `semio-framework-pack-json` → `semio-framework-pack-json` {"workspace": true}
- `dependencies`: `semio-framework-plugin` → `semio-framework-plugin` {"workspace": true}
- `dependencies`: `semio-framework-ui-locale` → `semio-framework-ui-locale` {"workspace": true}
- `dependencies`: `semio-framework-value` → `semio-framework-value` {"workspace": true}
- `dependencies`: `semio-framework-value-derive` → `semio-framework-value-derive` {"workspace": true}
- `dependencies`: `semio-s-fem-app-surface` → `semio-s-fem-app-surface` {"workspace": true}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-framework-async` → `semio-framework-async` {"workspace": true}
- `dev-dependencies`: `semio-framework-async-macros` → `semio-framework-async-macros` {"workspace": true}

### `semio-s-artifact-fem-2d-editor-results-window`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-artifact-fem-2d-results-state` → `semio-s-artifact-fem-2d-results-state` {"path": "../../🎚️config/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-artifact-fem-2d-editor-terminology` → `semio-s-artifact-fem-2d-editor-terminology` {"path": "../../../../../../🗣️terminology/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-artifact-fem-2d-editor-session` → `semio-s-artifact-fem-2d-editor-session` {"path": "../../../../../../🧵️session/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-analyses-model` → `semio-s-fem-analyses-model` {"path": "../../../../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-editor-model-window`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-analyses-model` → `semio-s-fem-analyses-model` {"path": "../../../../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-editor-commands`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-editor-panels`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-artifact-fem-2d-results-state` → `semio-s-artifact-fem-2d-results-state` {"path": "../../../🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-analyses-model` → `semio-s-fem-analyses-model` {"path": "../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-editor`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-artifact-fem-2d-results-state` → `semio-s-artifact-fem-2d-results-state` {"path": "../../🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-analyses-model` → `semio-s-fem-analyses-model` {"path": "../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-editor-interaction`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-geometry` → `semio-framework-geometry` {"path": "../../../../../../../../../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-editor-session`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-ui-scene` → `semio-framework-ui-scene` {"path": "../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-app-surface` → `semio-s-fem-app-surface` {"path": "../../../../../../../../../../⚙️engine/🖥️app-surface/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-artifact-fem-2d-results-state` → `semio-s-artifact-fem-2d-results-state` {"path": "../../../🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-viewer`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-model`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-mutations`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-dsl` → `semio-framework-dsl` {"path": "../../../../../../../../../../../../../🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-model` → `semio-s-fem-model` {"path": "../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🏗️model/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-artifact-fem-2d-engine` → `semio-s-artifact-fem-2d-engine` {"path": "../../../../../../../../⚙️engine/◻️2d/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-2d-load`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-2d-mutations` → `semio-s-artifact-fem-2d-mutations` {"workspace": true}

### `semio-s-artifact-fem-2d-analysis`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-2d-mutations` → `semio-s-artifact-fem-2d-mutations` {"workspace": true}

### `semio-s-artifact-fem-2d-mesh-topology`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-2d-mutations` → `semio-s-artifact-fem-2d-mutations` {"workspace": true}

### `semio-s-artifact-fem-2d-mesh`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-2d-mutations` → `semio-s-artifact-fem-2d-mutations` {"workspace": true}

### `semio-s-artifact-fem-2d-boundary`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-2d-mutations` → `semio-s-artifact-fem-2d-mutations` {"workspace": true}

### `semio-s-artifact-fem-2d-material`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-2d-mutations` → `semio-s-artifact-fem-2d-mutations` {"workspace": true}

### `semio-s-artifact-fem-2d`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-geometry` → `semio-framework-geometry` {"path": "../../../../../../../🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust"}
- `dev-dependencies`: `semio-framework-trace` → `semio-framework-trace` {"workspace": true}
- `dev-dependencies`: `csv` → `csv` {"version": "1"}
- `dev-dependencies`: `semio-framework-plugin` → `semio-framework-plugin` {"features": ["artifact-app-testing"], "workspace": true}
- `dev-dependencies`: `semio-framework-os-kernel` → `semio-framework-os-kernel` {"features": ["protocol-laws", "mutation-testing"], "workspace": true}
- `dev-dependencies`: `semio-framework-async` → `semio-framework-async` {"workspace": true}

### `semio-s-artifact-fem-3d-editor-results-window`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-artifact-fem-3d-editor-model-window` → `semio-s-artifact-fem-3d-editor-model-window` {"path": "../../../🧱️model/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-analyses-model` → `semio-s-fem-analyses-model` {"path": "../../../../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-editor-commands`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-editor-panels`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-editor`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-analyses-model` → `semio-s-fem-analyses-model` {"path": "../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-editor-visual-job`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-artifact-fem-3d-scene` → `semio-s-artifact-fem-3d-scene` {"path": "../../../../../🎬️scene/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-editor-session`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-artifact-fem-3d-scene` → `semio-s-artifact-fem-3d-scene` {"path": "../../../../🎬️scene/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-artifact-fem-3d-editor-results-window` → `semio-s-artifact-fem-3d-editor-results-window` {"path": "../../../🎭️modes/✏️edit/🪟️windows/📊️results/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-app-surface` → `semio-s-fem-app-surface` {"path": "../../../../../../../../../../⚙️engine/🖥️app-surface/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-scene`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🎬️scene/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-io`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-model`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-mutations`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-dsl` → `semio-framework-dsl` {"path": "../../../../../../../../../../../../../🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}

### `semio-s-artifact-fem-3d-load`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-3d-mutations` → `semio-s-artifact-fem-3d-mutations` {"workspace": true}

### `semio-s-artifact-fem-3d-analysis`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-3d-mutations` → `semio-s-artifact-fem-3d-mutations` {"workspace": true}

### `semio-s-artifact-fem-3d-mesh-topology`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-3d-mutations` → `semio-s-artifact-fem-3d-mutations` {"workspace": true}

### `semio-s-artifact-fem-3d-mesh`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../../../../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-3d-mutations` → `semio-s-artifact-fem-3d-mutations` {"workspace": true}

### `semio-s-artifact-fem-3d-boundary`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-3d-mutations` → `semio-s-artifact-fem-3d-mutations` {"workspace": true}

### `semio-s-artifact-fem-3d-material`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../../../../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-s-artifact-fem-3d-mutations` → `semio-s-artifact-fem-3d-mutations` {"workspace": true}

### `semio-s-artifact-fem-3d`

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-fem-node` → `semio-s-fem-node` {"path": "../../../../⚙️engine/📍node/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-fem-material` → `semio-s-fem-material` {"path": "../../../../⚙️engine/🧱️material/📦️packages/🦀️rust"}

### `semio-s-plugin-energy-engine-kernel`

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-math-dense` → `semio-framework-math-dense` {"path": "../../../../../../../../../🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust"}

### `semio-s-plugin-energy-engine-bestest`

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-s-plugin-energy-engine-sim` → `semio-s-plugin-energy-engine-sim` {"features": ["testing"], "workspace": true}

### `semio-s-plugin-energy-engine-num`

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔢️num/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-framework-math-dense` → `semio-framework-math-dense` {"path": "../../../../../../../../../🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust"}

### `semio-s-plugin-energy-engine-precompute`

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-math-dense` → `semio-framework-math-dense` {"path": "../../../../../../../../../🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust"}
- `dev-dependencies`: `semio-s-plugin-energy-engine-bestest` → `semio-s-plugin-energy-engine-bestest` {"path": "../../../🏛️bestest/📦️packages/🦀️rust"}
- `dev-dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}

### `semio-s-plugin-energy-engine-sim-restore`

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️restore/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-s-plugin-energy-engine-sim` → `semio-s-plugin-energy-engine-sim` {"features": ["testing"], "workspace": true}
- `dev-dependencies`: `semio-s-plugin-energy-engine-site` → `semio-s-plugin-energy-engine-site` {"workspace": true}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}

### `semio-s-plugin-energy-engine-sim`

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-os-kernel` → `semio-framework-os-kernel` {"path": "../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-framework-job` → `semio-framework-job` {"features": ["testkit"], "workspace": true}
- `dev-dependencies`: `semio-s-plugin-energy-engine-bestest` → `semio-s-plugin-energy-engine-bestest` {"workspace": true}
- `dev-dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}

### `semio-s-artifact-energy-model-editor-results`

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}

### `semio-s-artifact-energy-model-mutations`

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-s-plugin-energy-engine-model` → `semio-s-plugin-energy-engine-model` {"path": "../../../../../../../../../../🔨️modules/⚡️simulation/⚙️engine/🔋️model/📦️packages/🦀️rust"}

### `semio-s-artifact-energy-model-session`

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-math-rank` → `semio-framework-math-rank` {"path": "../../../../../../../../../../../../🧰️framework/🔨️modules/🧮️math/📶️rank/📦️packages/🦀️rust"}
- `dev-dependencies`: `semio-s-plugin-energy-engine-sim` → `semio-s-plugin-energy-engine-sim` {"features": ["testing"], "workspace": true}
- `dev-dependencies`: `semio-framework-job` → `semio-framework-job` {"features": ["testkit"], "workspace": true}
- `dev-dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}

### `semio-s-artifact-energy-model`

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-s-plugin-energy-engine-sim` → `semio-s-plugin-energy-engine-sim` {"features": ["testing"], "workspace": true}
- `dev-dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}
- `dev-dependencies`: `csv` → `csv` {"version": "1"}

### `semio-s-plugin-energy-epw`

`✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `csv` → `csv` {"version": "1"}

### `semio-s-plugin-puzzle-jobs`

`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework` → `semio-framework` {"optional": true, "workspace": true}
- `dev-dependencies`: `serde_json` → `serde_json` {"workspace": true}
- `dev-dependencies`: `semio-framework` → `semio-framework` {"workspace": true}
- `dev-dependencies`: `semio-framework-job` → `semio-framework-job` {"features": ["testkit"], "workspace": true}

### `semio-s-plugin-puzzle-retained`

`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-framework-job` → `semio-framework-job` {"features": ["testkit"], "workspace": true}
- `dev-dependencies`: `semio-s-plugin-puzzle-jobs` → `semio-s-plugin-puzzle-jobs` {"features": ["testing"], "workspace": true}

### `semio-s-artifact-puzzle-2d`

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-s-plugin-puzzle-jobs` → `semio-s-plugin-puzzle-jobs` {"features": ["testing"], "workspace": true}
- `dev-dependencies`: `semio-framework-async` → `semio-framework-async` {"workspace": true}
- `dev-dependencies`: `semio-framework-plugin-ownership` → `semio-framework-plugin-ownership` {"workspace": true}

### `semio-s-artifact-puzzle-5d`

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-job-publication` → `semio-framework-job-publication` {"path": "../../../../../../../🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/📦️packages/🦀️rust"}
- `dependencies`: `semio-s-plugin-puzzle-jobs` → `semio-s-plugin-puzzle-jobs` {"path": "../../../../🎮️commands/📤️jobs/📦️packages/🦀️rust"}
- `dev-dependencies`: `semio-s-plugin-puzzle-jobs` → `semio-s-plugin-puzzle-jobs` {"features": ["testing"], "workspace": true}
- `dev-dependencies`: `semio-framework-async` → `semio-framework-async` {"workspace": true}

### `semio-s-artifact-puzzle-3d-retained-action`

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-s-artifact-puzzle-3d-mutations-object` → `semio-s-artifact-puzzle-3d-mutations-object` {"path": "../../../../../🧬️schema/🧬️mutations/🧊️object/📦️packages/🦀️rust"}

### `semio-s-artifact-puzzle-3d`

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`

- `dev-dependencies`: `semio-framework-async` → `semio-framework-async` {"workspace": true}
- `dev-dependencies`: `serde` → `serde` {"workspace": true}

### `semio-s-fem-test-support`

`✏️s/🔨️modules/🏗️fem/⚙️engine/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-job` → `semio-framework-job` {"features": ["testkit"], "workspace": true}
- `dependencies`: `semio-framework-trace` → `semio-framework-trace` {"path": "../../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"}

### `semio-framework-repo-dashboard-repo-domain`

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🏛️repo-domain/📦️packages/🦀️rust/Cargo.toml`

- `dependencies`: `semio-framework-repo-todos` → `semio-framework-repo-todos` {"path": "../../../../📝️todos/📦️packages/🦀️rust"}
- `dependencies`: `semio-framework-repo-identity` → `semio-framework-repo-identity` {"path": "../../../../🪪️identity/📦️packages/🦀️rust"}

## Deliberate Source Move

Removed the unchanged old Puzzle5d editor/📸️snapshot/🦀️.rs because latest stash relocates its productive typed PlaySnapshot authority to schema/🎞️play-snapshot/🦀️.rs; the restored root/schema modules select the new schema owner. This removes an unreferenced duplicate source, not the PlaySnapshot implementation.

## Subsequent Browser Ownership Edges

- `semio-framework-repo-dashboard-terminal` dependencies: `dashboard_cli` → `semio-framework-repo-dashboard-cli`, path `../../../🧭️cli/📦️packages/🦀️rust`.
- `semio-framework-repo-dashboard-cli` dependencies: `ui_tui` → `semio-framework-ui`, path `../../../../../../../🔨️modules/🖱️ui/📦️packages/🦀️rust`, feature `tui-terminal`.
