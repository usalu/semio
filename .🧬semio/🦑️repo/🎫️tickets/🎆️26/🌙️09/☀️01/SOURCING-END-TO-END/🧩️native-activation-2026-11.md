# 🧩️ Native sourcing activation (2026-11-23)

Bypasses the repo graph (peer's 10 GB graph-plan, memory pressure). All commands from the workspace root; bun = `$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)`.

## Steps
1. `sh <ticket>/native-wasm-build.sh` (JOBS=2): per component `cargo build -p <crate> --lib --target wasm32-wasip2 --profile prototype` with the pinned nightly (`.cache/b/t/m/r/*/toolchains/nightly-2026-07-20-*/bin`), no rustc wrapper, target dir `<ticket>/🗑️generated/native-wasm`. Passes: `🌎️hub` (semio-hub-sourcing-component) and the three extension workspaces (beams, slabs, windows).
2. `bun <ticket>/native-stage-modules.ts`: calls the framework's own `transpilePluginComponentModulesAsync` (debug strip + `semio-native-transpile` + jco rewrites), copies into a sibling stage dir, writes host shim, `rewritePreview2ShimImports`, runs the module's `describe()` probe under node JSPI (`PLUGIN_DESCRIPTOR_PROBE_SOURCE`), `finalizePluginDescriptor` -> `🛂️.descriptor.semio` + `🔣️.json`, bridge (`pluginComponentBridgeSource`), receipt; renames into `.cache/b/p/m/browser/prototype/modules/<dir>`; compares the descriptor with the committed pair; publishes the activation receipt `runtime/react/sourcing/activation/🔣️receipt.json`.
3. `bun <ticket>/serve-sourcing-native.ts > 🗑️generated/serve-native.txt` (background): Vite on 127.0.0.1:6081.

## Problems and fixes
- Why not call `buildComponentClosureV1`/`describeComponentsV1`: they demand a script-process invocation envelope, resource leases and the store lanes (the stuck graph). The lower-level stage functions are called directly instead.
- `playgroundComponentClosureV1("sourcing")` throws (generated `🚀️playgrounds.json` not regenerated); ids are listed explicitly (`componentPassPlanV1` confirms 4 passes).
- Profile: first build used opt-level 0 and failed at component encode (`functions count exceeds limit of 1000000`). The driver's real prototype wasm-component profile (`renderCargoProfileConfigurationV1`, role component) is opt-level `s`, thin LTO, cgu 16, strip symbols, debug-assertions on; the script now passes exactly those selectors (minus host-only package overrides). The driver is correct; my hand-rolled profile was the bug.
- Vite config reads `runtime/react/sourcing/activation/🔣️receipt.json` at serve start (ENOENT crash); the stager publishes it (an empty one was seeded to bring the server up before modules exist).
- The `pgrep -f` waiter pattern matched itself; wait on pids instead.
- Plugin id `sourcing` is not in the generated catalog inventory until `repo run @semio-tech/s-dev-composition:generate` runs.

## Staged (prototype modules root `.cache/b/p/m/browser/prototype/modules/`)
| plugin | dir | wasm bytes | wasm sha256 | descriptor vs committed |
|---|---|---|---|---|
| sourcing | 🪵️sourcing | 221918908 | 24f0359a6822f6a100dc034e4cb621fcf465e1e7e1fd91930f4c8c2d7a9e113e | MATCH (75982 B) |
| sourcing-module-beams | 🪜️sourcing-module-beams | 38660357 | 9f3798ac057ad6023a9aac5d4d0d223845091a1e9d10096496da10be68d8e2f4 | MATCH (1718 B) |
Core wasm of sourcing is served as `application/wasm` (221830479 B) at `/🔌️plugin-modules/🪵️sourcing/`.

## Further findings
- Vite builds its `/🔌️plugin-modules/<dir>` static mounts from the activation receipt at server start: restart `serve-sourcing-native.ts` after every staging run (before that, wasm/json requests fall back to index.html or 404).
- The receipt's `sourceTreeDigest`/`provenanceSha256` are derived markers, not the driver's cargo-provenance Merkle roots.
- Slabs and windows built later (41 min each, shared target invalidated by peer edits) and are staged.
- Extensions are additionally installed (as `publishActivatedExtension` does) into `runtime/react/sourcing/extensions/<dir>` with preview2 imports rewritten to `../../🔌️plugin-modules/🪞️vendor/` and the install-meta record; Vite serves them at `/🧩️extension-modules/`.
- A peer's half-written `🏗️build/🕸️graph/📥️load/🟦️.ts` had literal newlines inside string literals (`+"<LF>")`); fixed to `"\n"` (2 sites). A missing peer module (`🔁️inventory-rounds`) resolved itself later.

| plugin | wasm bytes | sha256 |
|---|---|---|
| sourcing-module-slabs | 38611447 | bc232d5836adc2ccb229eff82c0c5c871fe77e6fe0b74cbb7233ac352973ebb4 |
| sourcing-module-windows | 38460896 | 644d428e6dc2fa464dd2c4625d94d0981b097e4d4e8e79f537f32fb25d338bc0 |
All descriptors MATCH the committed pairs. Beams was rebuilt (see table above; sha may differ).
