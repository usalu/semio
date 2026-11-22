# L14 Editor Activation Recipe (Read-Only Exploration, 2026-11-22)

## Activation closure
Source: `playgroundPreparationTargets` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs`.

The closure, in order:
1. `@semio-tech/plugin-registry:session-<variant>`
2. `@semio-tech/framework-plugin-web:support`
3. `semio-os-infinite:fonts`
4. The engine wasm `@semio-tech/puzzle-plugin:wasm`. It runs `repo ./📜️script.ts wasm` in `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust` and produces `pkg/semio_puzzle.js` and `semio_puzzle_bg.wasm`.
5. `<project>:component` for each plugin in the runtime component closure. For puzzle this is probably only `semio-hub-puzzle-component`.
6. `prepare <variant> react prototype`
7. `activate <variant> react prototype`. This publishes `.cache/b/p/m/browser/prototype/runtime/react/<variant>`.

## Component build
- Targets come from `componentBuildTargets` in `📥️inference/🧩️component-targets/🟨️.mjs`.
- Each runs `repo 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts component --manifest <crate>/Cargo.toml`.
- It is a wasm32-wasip2 cdylib, about 230 MB.
- Puzzle component crate: `🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust`.
- No staged component exists in `.cache/b/p/m/browser/prototype/modules`.
- The `☀️18/DEV-PUZZLE-5D…/🗑️generated/w/{t,b}/wasm32-unknown-unknown/…/semio_hub_puzzle_component.wasm` builds are not wasip2.

## Disk gate
- `admitBuildDiskV1` lives in `🏗️build/🌡️host-capacity/💽️disk/🟦️.ts`, and `judgeDiskV1` in `🧹️disk-recovery/🟦️.ts`.
- The reserve is 6 GiB × safetyFactor 2 = 12 GiB, plus a 1.5 GiB cushion.

## Lifecycle scenarios
| Editor | Scenario | react port | wgpu port |
| --- | --- | --- | --- |
| 2D | `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts` | 6012 | 6112 |
| 3D | `🧊️3d-lifecycle/🟦️.ts` | 6013 | 6113 |
| 5D | `🖐️5d-lifecycle/🟦️.ts` | 6014 | 6114 |

## Runner
- `@semio-tech/framework-os-dev:time-travel` runs `repo ./📜️script.ts verify time-travel`, which dispatches to `runTimeTravelCli` in `🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts`.
- Flags: `--scenario <path> [--serve <url>] [--renderer react|wgpu] [--locales en,de] [--out <dir>]`.
- Without `--serve`, it activates first via `ensureDevServe`.
- The locale flag is `--locales`, not `--lang`.
