# Acceptance 2026-11-27 (Process Plugin End to End)

## Criteria (derived from manifests, fixtures, tests and callers)
| # | Criterion | Evidence source |
|---|---|---|
| A1 | Discovery: hub component `semio:process` declares the `process3d` playground row (`s.process.process3d@1/*#editor`, ports react 6022 / wgpu 6122); registry generation lists it and the four extensions `process-extension-{concrete,metal,robotic,wood}` contributing `process.machines` | `🌎️hub/🧩️compositions/🏭️process/🧩️component/📦️packages/🦀️rust/Cargo.toml`, `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json`, `🔌️plugins.json`, test `✏️s/🔌️plugins/🏭️process/🧪️tests/🧭️dev-route` |
| A2 | Loading: the hub wasm builds for `wasm32-wasip2`, is transpiled/published as a browser plugin module, and the dev route boots it (title `semio · process · 3d`, one canvas, drawn pixels, no console errors) | `hub-build.sh`, `publish-hub.ts`, browser scenarios |
| A3 | Registration: plugin `process` with apps `#editor` and `#viewer`, examples, capabilities match `registration.json` | `🌎️hub/🧩️compositions/🏭️process/🧪️tests` |
| A4 | Entry points: example select (Demo / Concrete Forest / No example), roles Editor and Viewer, Actions panel actions (e.g. Increase), undo | `process-interactions.json`, `process-role-cycle.json` |
| A5 | Core workflow results: the selected example renders a non-empty 3D view, actions change the document and undo restores it | same |
| A6 | Lifecycle and recovery: cancelling a running whole-document load stops it, leaves a consistent document and a usable app; role switches cycle without faults | `process-cancel.json` / `process-cancel-load.json`, native laws `a_cancelled_replay_leaves_the_runtime_instance_live_and_accepting_actions` (`folder-reload-route`) |
| A7 | Native tests of bridge, extensions, process3d crates and hub-process crates pass | `process-rust-tests.md` |
| A8 | TS package typecheck, lint and tests pass | `process-typescript-checks.md` |

## Results
Filled in at close-out below.
