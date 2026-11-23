# 🚦️ Layout End To End: Native Lane on g1-m04 (2026-11-22, claude-code)

## Why Native

`repo run @semio-tech/framework-os-dev:activate-layout-wgpu-prototype --dry-run` waited 15 min on "Waiting for the repo graph service" and exited 1. Every target is therefore run through `launch-native.ts`, which builds the repo script envelope and runs the target's own command:

`repo tools exec --light bun -- <ticket>/launch-native.ts <cwd> <project:target> <script> <args…>`

`LAYOUT_RENDERER` selects `wgpu` (the default) or `react`. If the `repo tools exec` wrapper hangs, run the bun binary under `~/.cache/semio/runtimes/bun/*/bun` directly.

## Disk Incident

At about 15:20, `/home` reached 0 B free and every lane stalled. `repo cache cas gc` failed too, although its own error message says it works on a full disk. Recovery took three steps:

1. Remove the compiler caches `~/.cache/{go-build,zig,cursor-compile-cache}`, about 0.9 GB.
2. Run `repo cache cas gc`, which freed 2.4 GiB.
3. Run `repo cache gc` and `repo cache dedupe`.

A separate lane is fixing the cas gc startup on a full disk (`opus1122-cas-gc-full-disk.md`).

## Chain

| Step | Command | State |
| --- | --- | --- |
| session | registry `./📜️script.ts session layout` with `SEMIO_PLUGIN_CATALOG=✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` | done: 1 plugin staged |
| support | plugin-web `./📜️script.ts support` | done |
| component | driver-stages `component --manifest 🌎️hub/🧩️compositions/📏️layout/🧩️component/📦️packages/🦀️rust/Cargo.toml` | rerunning after the disk incident |
| wgpu wasm, generators, fonts | ui-rs generate, font assets, deps-wasm-opt, deps-browser-wasm, wasm build, three generators, infinite fonts | rerunning |
| prepare | dev `./📜️script.ts prepare layout wgpu prototype` | blocked on component |
| activate / serve 6179 / journey | — | pending |

## Reports

- `opus1122-journey-selector-audit.md`: engagement-toggle id fixed in `browser-journey.mjs`
- `opus1122-examples-audit.md`
- `opus1122-graph-service-diagnosis.md`
- `opus1122-chromium-webgpu.md`: WebGPU works headless on this host (SwiftShader; `--use-angle=vulkan` gives the AMD adapter)
- `opus1122-ts-tests.md`, `opus1122-rust-layout-tests.md`, `opus1122-react-lane.md`: pending
