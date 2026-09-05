# Baseline runtime evidence — 2026-09-05

Coordinator-run commands and observations. Logs live in the session scratchpad and are copied into `🗑️generated/` at close.

## Registered gates

| Gate | Command | Result | Evidence |
|---|---|---|---|
| OS dev quick tests | `bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache` | RED | vitest run killed by the 30 s quick budget: `[budget] … --config 🧪️tests/🟦️.ts … exceeded 30000ms — killed`. No test results printed. Lane A owns the repair. |
| Plugin registry check | `bun nx run @semio-tech/plugin-registry:check --skip-nx-cache` | RED (infrastructure) | ~20 min repo walk then `ENOENT: scandir …/target-block/debug/deps/rustcAWEOX6` in `📚️library/🔍️discovery/🟦️.ts:8754` (`discoverCatalogPackages`). A concurrent lane's isolated Cargo target root vanished mid-walk. Lane B owns skipping `target*` roots and tolerating vanished entries. |
| stdio native check (census) | `RUSTC_WRAPPER="" CARGO_TARGET_DIR=target-s-e2e cargo check -p semio-s-plugin-stdio --keep-going` | running | Peer semio-f4 verified the `#[path]` mount drift is gone from the main crate (remaining hits are in the separate `semio-s-plugin-stdio-test-oracle` crate and test-only fixtures). Peer semio-08 reports the gltf mapping mismatch fixed at 03100691d5. Result to be recorded in `📓️stdio-check-census.md`. |

## Served React shell (`dev s served`, port 6070)

Started via `.claude/launch.json` `s-react-served` (`bun ./📜️script.ts dev s served` → `nx run @semio-tech/framework-os-dev:dev -- s` with `SEMIO_RENDERER=react SKIP_PLUGIN_BUILD=1`).

- 00:00 registry refreshed (59 plugin crates, 60 playgrounds, 45 framework packages), `.vscode/launch.json` regenerated.
- 00:00–08:00 the `dev s` process (pid 95024) has no child process, ~7 % CPU, port 6070 not listening. No vite child, no engine `wasm` child. Suspected in-process `generatePluginRegistry` repo walk inside `ensurePluginRegistry` (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1121-1128`) crawling every `target*` root, the same walk that takes ~20 min in the registry check.
- Root cause found and fixed: (1) the served launcher derived `S_OS_PORT` from the wgpu renderer (6066) because `frameworkOsPlaygroundDevEnv` read the renderer before applying the `served` override; (2) `ensurePluginRegistry`/`writePlaygroundSession` re-walked the repository 4-5 times in-process (80-210 s each under load ~120) after the external `generate`; (3) discovery entered every `target-*` Cargo root; (4) `buildEngineWasm` (`wasm-pack build … framework_surface`) blocked on the shared `target/debug/.cargo-lock` held by a peer's 39-min `cargo check -p semio-hub`. Fixes: renderer/port override honoured, `readGeneratedCatalogProjection` replaces the in-process walks, `isDiscoverySkipDirectory` + `readdirVanishing`, and `served` now also sets `SKIP_ENGINE_BUILD=1`.
- 05:55 fourth boot answered HTTP 200 (Vite "ready in 415982 ms" under load 100, then a restart on a peer edit to `📇️directory/🟦️.ts`); the browser tab loaded a blank document with `504 Outdated Optimize Dep` while Vite re-optimised, and the server was gone before a reload.
- The machine rebooted between ~06:10 and ~08:40 (uptime 4:49 at 13:26); every orphaned build and dev server died with it.
- 13:25 fifth boot (detached, log `dev-s-served-2.txt`): Vite "ready in 7136 ms" with lane E's slimmer config graph, then `📚️library/🔍️discovery/🟦️.ts changed, restarting server` at 13:25:20 (load average 327). Browser evidence still pending.

## Concurrency on the machine

- An orphaned `cargo rustc -p semio-s-plugin-puzzle --target wasm32-wasip2 --profile wasm-dev` (pid 96183, parent reparented to launchd) holds the shared `target-demonstrator-dev/wasm-dev` lock with a live rustc child compiling stdio; ticket `26/08/28/DEMONSTRATOR` has `-p semio-s-plugin-process` queued behind it. Not killed on purpose; peers semio-08 (PROCESS-END-TO-END) and semio-f4 (PROCEDURAL-3D-END-TO-END) consume its outputs.
- Other live tickets today: `26/09/05/BLOCK-PLUGIN-END-TO-END` (session ⚪2adc84fa) owns the block plugin; this ticket does not dispatch block work.
- `26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY` owns the stdio format-by-format rename; this ticket does not touch stdio Rust sources.

## First real runtime evidence — 14:05 (sixth boot, detached, log `dev-s-served-3.txt`, load average 60-70)

Page: `document.body.innerText` empty, readiness beacon never set, no shell error banner (React never mounted). Console (all `[DEBUG]`-prefixed shell logs):

| Fault | Message | Attribution |
|---|---|---|
| Fatal boot | `PluginRuntime: shard 0 lost, restoring actors: s#1` then `Framework OS boot failed … shard 0 terminated` (`🎭️actor/📮️shard-client/🟦️.ts:1860`) | Watchdog `checkHeartbeats` (`:1973-1990`): a shard with a pending turn older than `DEFAULT_HEARTBEAT_TIMEOUT_MS = 5000` and no fresher heartbeat collects one miss per window; `HEARTBEAT_MISSED_LIMIT = 3` → `terminate()` + `rebuild()`. The primary `s` actor's first turn (component instantiation on a loaded machine) exceeded ~15 s of heartbeat silence, so the host killed its own boot shard. |
| Router | `AppRouter.build failed … plugin "demonstrator" contributes a surface for "s.cad.cad@1/*" without depending on owner "cad"` (×8) | `🎠️kernel/🟦️.ts:615-640` requires `manifest.dependencies` (runtime manifest) to name the owner; demonstrator's runtime manifest lacks them although the registry row lists `dependsOn: [cad, gis, procedural, process, puzzle, sourcing, stdio]`. One offending plugin throws and takes the whole router down. |
| Descriptor | `plugin.descriptor-identity-mismatch: expected cad-extension-aec-building(-structure), received empty` | The 4 CAD placeholder descriptors (lane B census). |
| Descriptor | `plugin.descriptor-invalid: /🔌️plugin-modules/<sourcing-module-beams, flow-extension-draw, playbook-module-procedural, playbook>/🔣️.json returned HTML` | Missing owner pairs (19 rows census); Vite answers the SPA fallback. |
| Load | `timeout loading puzzle / reasoning-mindmap / lowpoly / flow-extension-primitive` (`ShellHelpers/🟦️.tsx:960`) | Module fetch/instantiate exceeded the per-plugin load timeout under load. |

Consequence for the definition of done: item 1 (boot with the full catalog) fails today for three independent reasons: watchdog kills the boot shard under load, the router is not resilient to one bad manifest, and 23 catalog rows have no valid descriptor. Lanes G (shard liveness) and H (router resilience + demonstrator manifest) dispatched 14:15; descriptors wait for the Wave 2 rebuild.

## Headless evidence attempts — 14:20-14:35

- Lane A's `bun ./📜️script.ts verify catalog --out …/🗑️generated/coordinator/catalog-smoke-1` against 6070 (S_CATALOG_SMOKE_PROGRAM_MS=20000): Playwright `goto` timed out at 300 s waiting for `load` — the unbundled dev module graph does not finish loading within five minutes on this machine (load average 50-70). The harness should wait for `commit` and poll the readiness beacon instead of `load`.
- Coordinator probe `boot-probe.ts` (scratchpad; `waitUntil: "commit"`, polls the beacon, logs request/response/failure counts and console errors): first run resolved Bun's own Playwright without browsers (`chromium_headless_shell-1243` missing); second run hit `ERR_CONNECTION_REFUSED` because lanes editing `🔌️vite-plugins.ts`/`⚙️vite.config.ts` restarted the shell at 14:27-14:28. Third run queued behind an HTTP-200 wait with `PLAYWRIGHT_BROWSERS_PATH=node_modules/.cache/ms-playwright` (the dev script's own convention, `📜️script.ts:4091`).
- 15:12 seventh boot (`dev-s-served-4.txt`, after a listening-but-dead Vite was force-killed): lane H's router isolation is live (`AppRouter excluded plugin "demonstrator": surface.contribution-not-permitted …` instead of a global failure) and lane G's worker error logging is live (`shard 0 worker error: redacted "error" event with no message …`). Standalone spawn of the shard worker from the page: script 200 text/javascript, no error/no message within 60 s, JSPI functions present on the main thread. So the shard dies during activation, not at load; forwarded to lane G with a request for in-worker fault reporting.
- 15:20-15:31 worker-construction probes (page-level `Worker` wrapper): at 15:14 the shell constructed its four shard workers at `/plugin-modules/_shard/🟨️shard-worker.js` (Vite answers the SPA HTML → immediate `error` event with a redacted message); by 15:30 `PluginRuntime/🟦️.tsx:48` (`ShardClient.createWorker`) spawns `/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js` and no error event fires within 44 s. The stale ASCII path was the fatal boot cause; a rerun is measuring whether the beacon now appears.
- 15:32 eighth boot (`dev-s-served-5.txt`) with lane G's in-worker fault reporting live: `shard 0 worker fault [handler/first-step] actor=s#1 module=/🔌️plugin-modules/🪐️s/🌉️bridge.js: unreachable RuntimeError: unreachable` → `PluginRuntime: actor s#1 trapped` → `Framework OS boot failed Error: unreachable`, beacon dataset set to `s` (error path) at 31 s. The cached `s` core (`semio_s_plugin_space_component.core.wasm`, built 2026-09-02 10:56, 63 MB) traps on its first turn against today's framework host, i.e. a stale-ABI core. Next: rebuild the `s` crate into the cache (`SEMIO_PLUGIN_ONLY=s` plugin build in `target-s-e2e`).
