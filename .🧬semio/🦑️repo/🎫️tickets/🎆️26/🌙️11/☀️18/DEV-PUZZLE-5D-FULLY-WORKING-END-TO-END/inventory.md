# Puzzle 5D Inventory

| Project | Path | Key targets |
| --- | --- | --- |
| `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust` | build, check, test, graph-wire-check, test-snapshot-sqlite*, verify-* |
| `@semio-tech/puzzle-5d-react` | `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript` | test, verify-lifecycle (`🧪️tests/🖐️5d-lifecycle`) |
| `repo-storybook` → `dev-storybook-puzzle-5d` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook` | dev launch on port 6010, depends on prepare-storybook |
| `@semio-tech/hub-puzzle-retained-test` | `🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained` | test-ownership, test-native |
| `semio-hub-puzzle` (cargo, no project.json) | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust` | `@semio-tech/puzzle-wasm` npm package |

The dev stories live in `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️5d-timeline`. The dev tests are in `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔺️diff-parsers`.

The test scope `🧪️tests/🧪️compose5d-preparetopologymodel` is imported by the 5d-react target and has no project of its own.

Root aggregate groups in `📋️project.json` are `puzzle-spatial-tests` and `snapshot-sqlite-parent-baselines`.

The tree `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🗿️artifacts/🖐️5d` holds only `🏅️standards` and may be stray. It still needs a check.
