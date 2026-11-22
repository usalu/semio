# L6 Inventory: Puzzle Plugin, Dev, Hub and Storybook Surfaces

Read-only audit (no builds run). Sources: every `📋️project.json` under `✏️s/🔌️plugins/🧩️puzzle`, `✏️s/🧑‍💻dev/🧩️puzzle`, `🌎️hub/🧩️compositions/🧩️puzzle`, the framework storybook project (`dev-storybook-puzzle*`) and the framework-os-dev project (`verify time-travel`, called by the puzzle 3d/5d targets). `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle` does not exist on disk. The only variant project is `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services` (not puzzle).

## 1. Project targets (from `📋️project.json`)

| Project | Manifest | Target | Executor | options.command (or options keys) | Plugin test folder covered |
|---|---|---|---|---|---|
| `@semio-tech/puzzle-5d-react` | `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/📋️project.json` | `verify-lifecycle` | repo:script | `repo ./📜️script.ts verify time-travel --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🖐️5d-lifecycle/🟦️.ts --renderer react --locales en,de` | 🖐️5d-lifecycle |
| `@semio-tech/puzzle-5d-react` | `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/📋️project.json` | `test` | repo:script | `repo ./📜️script.ts test` | 🎚️config (shared vitest include; does not include 🧪️compose5d-preparetopologymodel) |
| `@semio-tech/puzzle-js` | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json` | `test` | repo:script | `repo ./📜️script.ts test` | 🎚️config (shared vitest runner); 🧪️transform-gesture; 📚️storybook-index (nested 🧪️tests); 🚀️renderer-boot-variants; 🔬️typescript-package-commands (runner) |
| `@semio-tech/puzzle-js` | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json` | `publication-authority-audit` | repo:script | `repo ./📜️script.ts publication-authority-audit` | 🔬️typescript-package-commands; 🧫️fixtures/🔏️publication-authority |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `test-store-close-source` | repo:script | `repo ./📜️script.ts test-store-close source` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `test-store-close-native` | repo:script | `repo ./📜️script.ts test-store-close native` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite-native` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite native` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite-source` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite source` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `verify-snapshot-sqlite-source` | repo:script | `repo ./📜️script.ts verify snapshot-sqlite-source` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `build` | repo:script | `repo ./📜️script.ts build` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `check` | repo:script | `repo ./📜️script.ts check` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `test` | repo:script | `repo ./📜️script.ts test` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | repo:script | `repo ./📜️script.ts graph-generate` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `graph-wire-check` | repo:script | `repo ./📜️script.ts graph-wire-check` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `preview-generated` | repo:script | `repo ./📜️script.ts preview-generated` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `test-presence-source` | repo:script | `repo ./📜️script.ts test-presence source` | — |
| `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `test-presence-native` | repo:script | `repo ./📜️script.ts test-presence native` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `build` | repo:script | `repo ./📜️script.ts build` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `check` | repo:script | `repo ./📜️script.ts check` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `test` | repo:script | `repo ./📜️script.ts test` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | repo:script | `repo ./📜️script.ts graph-generate` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `graph-wire-check` | repo:script | `repo ./📜️script.ts graph-wire-check` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite-source` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite source` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `verify-snapshot-sqlite-source` | repo:script | `repo ./📜️script.ts verify snapshot-sqlite-source` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite-native` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite native` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `preview-generated` | repo:script | `repo ./📜️script.ts preview-generated` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `verify-optional-fields` | repo:script | `repo ./📜️script.ts verify optional-fields` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `verify-inline-capacity` | repo:script | `repo ./📜️script.ts verify inline-capacity` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `verify-catalog-fields` | repo:script | `repo ./📜️script.ts verify catalog-fields` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `verify-edit-acceptance` | repo:script | `repo ./📜️script.ts verify edit-acceptance` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `verify-import-acceptance` | repo:script | `repo ./📜️script.ts verify import-acceptance` | — |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `verify-original-work-construction` | repo:script | `repo ./📜️script.ts verify original-work-construction` | — |
| `@semio-tech/puzzle-3d-rs-compute-ownership` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏛️ownership/🧮️compute/📋️project.json` | `test` | repo:script | `repo ./📜️script.ts test` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `verify-import-acceptance` | repo:script | `repo ./📜️script.ts verify import-acceptance` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `build` | repo:script | `repo ./📜️script.ts build` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `check` | repo:script | `repo ./📜️script.ts check` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `test` | repo:script | `repo ./📜️script.ts test` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | repo:script | `repo ./📜️script.ts graph-generate` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `graph-wire-check` | repo:script | `repo ./📜️script.ts graph-wire-check` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite-source` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite source` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `verify-snapshot-sqlite-source` | repo:script | `repo ./📜️script.ts verify snapshot-sqlite-source` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `test-snapshot-sqlite-native` | repo:script | `repo ./📜️script.ts test-snapshot-sqlite native` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `preview-generated` | repo:script | `repo ./📜️script.ts preview-generated` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `verify-original-work-construction` | repo:script | `repo ./📜️script.ts verify original-work-construction` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `verify-original-fill-fixture` | repo:script | `repo ./📜️script.ts verify original-fill-fixture` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `verify-original-fill-fixture-native` | repo:script | `repo ./📜️script.ts verify original-fill-fixture native` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `verify-runtime-lifecycle` | repo:script | `repo ./📜️script.ts verify runtime-lifecycle` | — |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `test-browser-lifecycle` | repo:script | `repo ./📜️script.ts verify time-travel --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️3d-lifecycle/🟦️.ts --renderer react --locales en,de` | 🧊️3d-lifecycle |
| `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `test-browser-wgpu-clipboard` | repo:script | `repo ./📜️script.ts verify time-travel --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️3d-wgpu-clipboard/🟦️.ts --renderer wgpu --locales en,de` | 🧊️3d-wgpu-clipboard |
| `@semio-tech/s-puzzle-composition-tests` | `✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json` | `verify-puzzle-fill-policy-self-tests` | repo:script | `repo ./📜️script.ts verify puzzle-fill-policy-self-tests` | 🔬️interactivity-puzzle-fill-p4e, 🔬️interactivity-puzzle-fill-run-job, 🔬️interactivity-puzzle-fill-trace (via plugin 🧪️testing re-exports) |
| `@semio-tech/s-puzzle-composition-tests` | `✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json` | `verify-browser-contribution` | repo:script | `repo ./📜️script.ts verify browser-contribution` | none plugin (dev 🧪️tests/🔗️browser-contribution) |
| `@semio-tech/puzzle-plugin` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json` | `wasm` | repo:script | `repo ./📜️script.ts wasm` | — |
| `@semio-tech/puzzle-plugin` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json` | `test` | repo:script | `repo ./📜️script.ts test` | — |
| `@semio-tech/puzzle-plugin` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json` | `fixtures-lint` | repo:script | `repo ./📜️script.ts fixtures lint` | none plugin (hub 🧪️tests/🔬️rust-package-commands) |
| `@semio-tech/puzzle-plugin` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json` | `canonical-architecture` | repo:script | `repo ./📜️script.ts canonical-architecture` | 🔬️tool-job-puzzle-reserved-routes (via plugin 🧪️testing re-export) |
| `@semio-tech/puzzle-plugin` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json` | `test-guest-io` | repo:script | `repo ./📜️script.ts test-guest-io` | none plugin (hub 🧪️tests/🧩️guest-io) |
| `@semio-tech/hub-puzzle-retained-test` | `🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/📋️project.json` | `test-ownership` | repo:script | `repo 📜️script.ts test-ownership` | none plugin (hub 🧵️retained) |
| `@semio-tech/hub-puzzle-retained-test` | `🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/📋️project.json` | `test-native` | repo:script | `repo 📜️script.ts test-native` | none plugin (hub 🧵️retained) |
| `repo-storybook` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📋️project.json` | `dev-storybook-puzzle` | repo:script | `repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📜️script.ts dev storybook puzzle` | — |
| `repo-storybook` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📋️project.json` | `dev-storybook-puzzle-2d` | repo:script | `repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📜️script.ts dev storybook puzzle 2d` | — |
| `repo-storybook` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📋️project.json` | `dev-storybook-puzzle-3d` | repo:script | `repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📜️script.ts dev storybook puzzle 3d` | — |
| `repo-storybook` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📋️project.json` | `dev-storybook-puzzle-5d` | repo:script | `repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📜️script.ts dev storybook puzzle 5d` | — |

## 2. Cargo and npm manifests without project targets (puzzle scope)

| Package | Manifest | Kind | Stale path deps |
|---|---|---|---|
| `(workspace)` | `✏️s/🔌️plugins/🧩️puzzle/Cargo.toml` | Cargo workspace | — |
| `semio-s-plugin-puzzle-jobs` | `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-plugin-puzzle-retained` | `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-puzzle-mutation-bridge` | `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.toml` | Cargo crate | — |
| `semio-s-plugin-puzzle-board-scene` | `✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board/🎬️scene/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `(workspace)` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.toml` | Cargo workspace | — |
| `semio-s-artifact-puzzle-2d-v1` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-board-host` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-engine` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-tambour-single-storey` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🌱️metabolism/🧩️tambour-single-storey/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-tambour-end-storeys` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🌱️metabolism/🧱️tambour-end-storeys/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-interaction` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-v1-editor-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit-2/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-unit-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-history-edit-runtime-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️history-edit-runtime/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-retained-panels` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/📌️panel-mounts/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-retained` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-inferences` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-schema` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-snapshot` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mutations` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-10` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-8` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🧮replace-edge-geometry/🧪️tests/📍️repositions-edge-1/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-3` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🆔change-manifest-id/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-9` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🧪️tests/📇️installs/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-connect-handles-preparation` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/🧪️tests/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-4` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-text-inverse` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-7` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🧪️tests/⚓️derives/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-2` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🔒change-node-locked/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-6` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/🧪️tests/🚫️rejects/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-mut-any-5` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-records` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️records/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `@semio-tech/puzzle-2d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🟦️typescript/package.json` | npm package | — |
| `semio-s-artifact-puzzle-2d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-2d-graph-manifest` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🛂️manifest/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `(workspace)` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.toml` | Cargo workspace | — |
| `semio-s-artifact-puzzle-5d-v1` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-dispatch-panels` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎬️dispatch/📌️panel-mounts/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-dispatch` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎬️dispatch/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-puzzle5d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-interaction` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-catalog` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-host-admission-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️host-admission/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-v1-unit-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit-2/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-testkit` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️testkit/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-store-preparation-commands` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️kit/💾️store-preparation/🎮️command-mounts/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-store-preparation` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️kit/💾️store-preparation/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-retained` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-schema` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-snapshot` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-diff` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-kind-catalogs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️kind-catalogs/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any-4` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any-5` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🧪️tests/🌱️appends-part-c/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any-6` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🧪️tests/📍️moves-part-a/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any-8` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🧪️tests/🚫️rejects/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any-2` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any-3` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️change-description/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-diff-2` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any-7` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/🤝️adds/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-mut-any-part2d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️part2d-leaves/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-5d-document-records` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `@semio-tech/puzzle-5d-ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🟦️typescript/package.json` | npm package | — |
| `semio-s-artifact-puzzle-5d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `(workspace)` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.toml` | Cargo workspace | — |
| `semio-s-artifact-puzzle-3d-precompute-geometry` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-precompute` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-precompute-brush` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-precompute-fill` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-config` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-session` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-modes` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-transform` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🔄️transform/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-commands-runtime` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-commands-runtime-2` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🪜️runtime-2/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-dispatch` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-commands-document` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-presence` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-panels` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-snapshot` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-terminology` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-retained-action` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-retained-document` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗂️document/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-editor-window` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-viewer` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-examples` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-io` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-inferences` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-model` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-diff` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-mutations-selection` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-mutations-vortex` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-mutations-volume` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-mutations-catalog` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-mutations` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-mutations-reference` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-mutations-object` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-mutations-pose` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `@semio-tech/puzzle-3d-ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🟦️typescript/package.json` | npm package | — |
| `semio-s-artifact-puzzle-3d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-s-artifact-puzzle-3d-host` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/🧰️owned/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-hub-puzzle` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |
| `semio-hub-puzzle-component` | `🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust/Cargo.toml` | Cargo crate | — |

## 3. Plugin test folder `✏️s/🔌️plugins/🧩️puzzle/🧪️tests` and owning target

| Test folder | Owning target(s) | Status |
|---|---|---|
| `⏪️time-travel` | none (`framework-os-dev:verify time-travel` only runs a scenario passed via `--scenario`; no puzzle target passes this one) | ORPHAN |
| `◻️storybook-2d` | none (no target or import reaches it; storybook targets do not reference it) | ORPHAN |
| `🎚️config` | shared vitest config for `puzzle-js:test` and `puzzle-5d-react:test` | infrastructure |
| `📏️inline-import` | none (only `🔣️.json` fixture, no reader found) | ORPHAN |
| `📚️storybook-index` | `puzzle-js:test` (vitest include, nested 🧪️tests) | covered |
| `🔬️interactivity-puzzle-fill-p4e` | `s-puzzle-composition-tests:verify-puzzle-fill-policy-self-tests` | covered (via plugin 🧪️testing re-export) |
| `🔬️interactivity-puzzle-fill-run-job` | `s-puzzle-composition-tests:verify-puzzle-fill-policy-self-tests` | covered (via plugin 🧪️testing re-export) |
| `🔬️interactivity-puzzle-fill-trace` | `s-puzzle-composition-tests:verify-puzzle-fill-policy-self-tests` | covered (via plugin 🧪️testing re-export) |
| `🔬️tool-job-puzzle-reserved-routes` | `puzzle-plugin:canonical-architecture` (hub, via plugin 🧪️testing re-export) | covered (cross-package) |
| `🔬️typescript-package-commands` | `puzzle-js:test`, `puzzle-js:publication-authority-audit` (runner module) | covered (runner, not a scenario) |
| `🖐️5d-lifecycle` | `puzzle-5d-react:verify-lifecycle` | covered |
| `🚀️renderer-boot-variants` | `puzzle-js:test` (vitest include) | covered |
| `🧊️3d-lifecycle` | `puzzle-3d-rs:test-browser-lifecycle` | covered |
| `🧊️3d-wgpu-clipboard` | `puzzle-3d-rs:test-browser-wgpu-clipboard` | covered |
| `🧊️storybook-spatial` | none (no target or import reaches it) | ORPHAN |
| `🧪️compose5d-preparetopologymodel` | none (only `🧪️testing` re-export in plugin package.json exports; `.tsx` not in any vitest include) | ORPHAN |
| `🧪️transform-gesture` | `puzzle-js:test` (vitest include) | covered |
| `🧫️fixtures` | data only: read by `puzzle-js:publication-authority-audit` | data |

Test folders under `🧪️tests` that are data-only or not reached by a target are called out as ORPHAN above.

## 4. Stale references (path in manifest/command that does not exist)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` namedInputs.nativeSources: `{workspaceRoot}/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🔨️modules/🏠️host/🧰️owned/🧪️tests/♻️store-close/**/*`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` target `build-inputs` cmd path: `️distribution/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/📋️project.json` target `test-storybook-catalog` cmd path: `🧪️tests/🧪️unit-runner/🟦️.ts`
