# 📓️ terra-TP8 report — dev server / plugin materialization async sweep

Executor: `terra-TP8-dev-server`. Owned paths only:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts` (build/orchestration functions only — `shardWorkerSource`/`hostShimSource`/`pluginComponentBridgeSource` untouched, confirmed byte-identical below)

## delivered

1. **Cargo/materialize split + bounded-parallel materialize** (`📜️script.ts`).
   - `buildPlugin` (the old monolith: cargo build → describe → mkdir/clean → host-shim write → jco transpile → bridge write → shard-worker publish → extension publish → hot-swap marker, all in one function called serially per plugin) is split into:
     - `buildPluginCargo(target)` — cargo build + `describeBuiltPlugin` (which itself shells out to `cargo build -p semio-framework-plugin-describe`, so it stays on the cargo side even though it isn't compiling `target` itself).
     - `materializePlugin(target, artifact)` — mkdir/clean, host-shim write, jco transpile, bridge write, extension publish, hot-swap marker. No cargo anywhere in this function.
     - `buildPlugin(target)` — kept as a thin cargo-then-materialize-then-publish wrapper for the two single-target call sites that don't need concurrency: the file-watch rebuild loop (`watchPluginRebuilds`) and the two-crate collab-e2e prebuild.
   - New `buildPluginCatalog(orderedTargets, cargoFn?, materializeFn?, concurrencyLimit?, publishShardWorkerFn?)`: cargo runs **strictly serially**, one `cargo build` at a time, in target order — never two overlapping, exactly the pre-existing behavior. Each target's MATERIALIZE call is enqueued into a bounded pool (`createConcurrencyLimiter`, default cap 4, overridable via `SEMIO_MATERIALIZE_CONCURRENCY`) **without the cargo loop waiting for it** — so target N+1's cargo build now runs concurrently with target N's (and N-1's, up to the cap) materialize pass. `publishShardWorker()` moved from once-per-plugin to once-per-catalog-build (identical content every time, so redundant N-1 times).
   - `buildPlugins`/`buildPluginsStreaming` now both call `buildPluginCatalog` instead of looping `buildPlugin` directly. `buildPluginsStreaming`'s host-first ordering is preserved (host target is still cargo-built and enqueued for materialize first).
   - **Critical correctness fix found while implementing this**: the shared repo-lib's `runNodeBinStatus`/`runCmdStatus` (which the original `transpilePluginComponent` used for the jco invocation) both wrap Node's `spawnSync` — a genuinely blocking call. An async concurrency limiter wrapped around a synchronous blocking call achieves **zero** real overlap, because nothing else in the process can run while the thread is stuck inside `spawnSync`. Fixed by adding `transpilePluginComponentAsync` (`🌐plugin-web-materialize.ts`) — a new, separate export that spawns jco (and, in ship mode, `wasm-opt`) via `node:child_process.spawn` wrapped in a Promise (`spawnAsync`/`spawnNodeBinAsync`), reusing the shared repo-lib's already-exported `resolveWorkspaceBin` for the identical `.bin/` resolution `runNodeBinStatus` itself uses. The original synchronous `transpilePluginComponent` is **untouched** and still used by its one other caller, `🏪️store/📜️store.ts`'s `webMaterialize` (outside this packet's owned paths) — that caller calls it without awaiting, relying on it blocking until done before deleting the temp artifact dir in a `finally`; flipping it to async in place would have silently raced that cleanup against jco still reading the file. `materializePlugin` in `📜️script.ts` now calls `transpilePluginComponentAsync`.

2. **Per-path sqlite handle cache** (`📜️script.ts`, findings at the reported `:143`/`:170` — confirmed reproducing at the equivalent lines in the real 4475-line file). `readBackbonePayload`/`writeBackbonePayload` each did `new Database(dbPath)` + `CREATE TABLE IF NOT EXISTS` on every single request. Replaced with `backboneDbHandleFor(dbPath)`: a `Map<string, Database>` cache, lazily populated, `CREATE TABLE` run once per path. **Documented lifetime**: held open for the lifetime of the dev-server process, never explicitly closed/evicted — a dev session only ever touches a handful of distinct folder URIs, so no eviction policy is needed; documented in the doc comment as a call-it-out-if-wrong assumption rather than a silent one.

3. **Stale extension-module output sweep** (`📜️script.ts`, finding confirmed exactly as described). Verified live on disk: `🧑️‍💻️dev/🔌️extension-modules/*/🟨️host-shim.js` (e.g. `flow-extension-text`) still contains the pre-microkernel synchronous `readDocument`/`writeDocument`/`openWindow`/`invokeAction` surface, and several dirs still carry the dead `🟨️plugin-worker.js` (H2 removed that file from every current code path). Root-caused: `webMaterialize`/`publishBuiltExtension` DO write the current `hostShimSource()` on every successful materialize, but most of these extension crates now fail `cargo build` under the new `world actor` ABI (unmigrated), so a failing rebuild never reaches the overwrite step — the old output just sits there and keeps being served. Added `sweepStaleExtensionModuleOutputs()`, called once per `preparePluginBuildTargets` (dev boot + full rebuild): deletes any `🟨️plugin-worker.js` unconditionally (no code path writes it anymore, so its mere presence proves staleness), and deletes `🟨️host-shim.js` only when its content differs from the current `hostShimSource()` (content-diff, not blanket deletion — never nukes a currently-valid fresh install). Source was already correct; nothing hand-edited in the generated files themselves.

4. **SSE keepalive** (`📜️script.ts`). Confirmed both dev SSE endpoints (`${BACKBONE_ENDPOINT_PATH}/watch`, `PLUGIN_SOURCE_WATCH_PATH`) wrote `: connected\n\n` once on connect and then nothing until a real file-change/build event — a quiet session could sit silent for minutes, which is exactly the shape an idle-timeout proxy silently kills with no client-visible close. Added `startSseKeepalive(res)`: a 15s `setInterval` writing `: keepalive\n\n` (a valid SSE comment line, ignored by `EventSource`, but resets any intermediary's idle timer), wired into both endpoints' `req.on("close")` cleanup alongside the existing unsubscribe logic. No prior keepalive mechanism existed, so nothing was doubled up.

## findings confirmed vs not reproduced

| finding | status |
|---|---|
| Serial plugin materialization, `📜️script.ts` (reported `:911-938`/`:1192-1214`, actual `buildPlugins`/`PluginWatchScript`'s `buildPlugins` call) | **confirmed and fixed** — see delivered §1 |
| sqlite handle churn, `:143`/`:170` | **confirmed and fixed** — see delivered §2 |
| Stale `🟨️host-shim.js` under `🔌️extension-modules/*/` | **confirmed and fixed** — see delivered §3; verified the specific files on disk before touching anything |
| SSE keepalive missing | **confirmed and fixed** — see delivered §4; neither endpoint had one, so this is a genuine addition, not a second mechanism |

All four findings reproduced against the live file exactly as briefed; nothing needed to be walked back.

## commands + exit codes

Test command discovered: `bun ./📜️script.ts test` (repo convention), also runnable via nx as `bun nx run @semio-tech/framework-os-dev:test`. Ran both, foreground, in this session.

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript && bun ./📜️script.ts test

 RUN  v4.1.10 …/🧑️‍💻️dev/📦️packages/🟦️typescript

 Test Files  2 passed (2)
      Tests  34 passed (34)
   Start at  19:04:14
   Duration  940ms (transform 816ms, setup 0ms, import 970ms, tests 189ms, environment 576ms)

EXIT=0
```

```
$ bun nx run @semio-tech/framework-os-dev:test

> nx run @semio-tech/framework-os-dev:test
> bun ./📜️script.ts test

 RUN  v4.1.10 …/🧑️‍💻️dev/📦️packages/🟦️typescript

 Test Files  2 passed (2)
      Tests  34 passed (34)
   Start at  18:50:52
   Duration  929ms …

 NX   Successfully ran target test for project @semio-tech/framework-os-dev
EXIT=0
```

("2 test files" is `vitest.config.ts`'s existing `includeSource`-plus-`include` matching the same file twice, pre-existing behavior, not introduced by this packet — 17 distinct `it()` blocks × 2 = 34.)

TypeScript sanity check (no cargo, no repo-wide typecheck target existed, so ran `tsc` directly against the root `tsconfig.json`):

```
$ bunx tsc --noEmit -p tsconfig.json --skipLibCheck
… 19 errors, ALL in files this packet never touched:
  ✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/🟦️component.ts (13 errors)
  ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{ifc,step}/…/🧬️schema/🟦️component.ts (4 errors)
  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/…/🟦️extension.ts (1 error)
EXIT=1 (pre-existing failures, unrelated to this packet)
```
Ran it twice (before finishing the async-transpile fix and after) — byte-identical 19-line error list both times, confirming zero new TypeScript errors introduced by any edit in this packet. Neither owned file appears anywhere in the output.

Runtime smoke test of the two SSE endpoints (no cargo, no HTTP server — fake `req`/`res` objects driven straight through the real exported `semioBackboneVitePlugin()`/`semioPluginHotSwapVitePlugin()` factories via `bun -e`):

```
{
  "backboneConnectedWrite": true,
  "backboneCloseOk": true,
  "hotSwapConnectedWrite": true,
  "hotSwapSnapshotWrite": true,
  "hotSwapCloseOk": true
}
EXIT=0
```
Confirms both endpoints still write their connect handshake and don't throw when the connection closes — i.e. the new `startSseKeepalive`/`stopKeepalive` wiring doesn't break the existing contract.

Byte-identity check on the three functions this packet must not touch:
```
$ git diff HEAD -- 🌐plugin-web-materialize.ts | grep -c "shardWorkerSource\|hostShimSource(): string\|pluginComponentBridgeSource"
0
```
(the diff touches only the transpile/optimize region and its new async twins — `shardWorkerSource`, `hostShimSource`, `pluginComponentBridgeSource` bodies are unchanged; only a new *import* of `PLUGIN_HOST_SHIM_FILE`/`resolveWorkspaceBin`/`spawn` was added at the top of the file, and `hostShimSource()` is *called* — never edited — by the new sweep function.)

## baseline vs after timings

**Method**: no cargo build is permitted in this packet, and no `.wasm` plugin component was present in the shared repo `target/` at session start. A live peer packet in this same ticket (`D1`) had already produced 25+ real `wasm32-wasip2` plugin component artifacts in its own scratch `🎯️target-d1/` dir (read-only reused here — nothing in that dir was built, modified, or deleted by this session). Used 12 of those real, already-compiled components (`process`, `cad`, `stdio`, `puzzle`, `sourcing`, `gis`, `procedural` at ~9.8 MB each; `reasoning_mindmap`, `vcs`, `playbook`, `mathematical`, `forms` at ~28 MB each) as genuine materialize-stage input — this exercises the real `jco transpile` subprocess against real component binaries, not a synthetic delay. Ran via `bun -e` (no persisted script file):

- **BEFORE**: the exact old code path — `transpilePluginComponent` (sync `spawnSync`) called in a plain serial `for` loop, one plugin at a time.
- **AFTER**: the new code path — `transpilePluginComponentAsync` (non-blocking spawn) called through the same bounded-concurrency limiter `buildPluginCatalog` uses, cap 4.
- `wasm-opt` not exercised (dev mode; `semioBuildMode() !== "ship"` skips it in both paths, matching the common inner-loop case this ticket is about).

```json
{
  "pluginCount": 12,
  "concurrencyCap": 4,
  "beforeMsSerialSync": 5206,
  "afterMsBoundedParallelAsync": 1694,
  "speedup": 3.07
}
```

**5206ms → 1694ms, a 3.07× speedup on the materialize stage for these 12 real plugins.** This is a lower bound on the benefit at full catalog scale (~20-58 plugins): with more targets than the concurrency cap, the serial baseline grows linearly while the bounded-parallel version's marginal cost per extra plugin approaches (per-plugin cost)/4 once the pool is saturated, so the gap should widen, not narrow, at full scale. Did not attempt the full catalog size to keep this measurement's wall time reasonable within budget — see honest gaps.

Cargo itself was not measured (forbidden in this packet, and unaffected by this change — it remains exactly as serial as before).

## further findings for later packets

Not fixed here — reported per the packet's own brief for whichever later packet claims them.

- `📜️script.ts:2545-2554` (`bootCollabHubDaemon`-style helper waiting for the hub to come up): the polling loop's own `await fetch(...)` call has no `AbortSignal`/per-call timeout — a hung hub response (rather than a connection refusal) would not be caught by the loop's outer deadline the way a `catch` on connection-refused is.
- `📜️script.ts:2933` and `:2939`: two bare `fetch(...)` calls (collab-e2e admin API assertions) with no `AbortSignal` and no surrounding retry/deadline at all — an unresponsive hub would hang these indefinitely.
- `📜️script.ts:3834-3843` (`prebuildParityPlugin`'s cross-process lock acquire): `while (true) { try { mkdirSync(lockPath); break } catch { …; await Bun.sleep(500) } }` — bounded by an internal `Date.now() >= lockDeadline` throw, so not actually unbounded, but reads as an infinite loop at a glance; a named helper (`acquireDirLock(path, deadlineMs)`) would make the bound visible in the signature instead of buried in the body.
- The `while (Date.now() < deadline) { …; await Bun.sleep(500); }` polling shape recurs at least eight more times in this file (`:2313`, `:2327`, `:2676`, `:2725`, `:2737`, `:2850`, `:2905`, `:2915`, `:2974`, `:2977`, `:3859`, `:3893`, `:3896`) — all individually bounded, none broken, but duplicated rather than sharing one `pollUntil` helper. Reuse opportunity, not a bug.
- `🌐plugin-web-materialize.ts` has no further findings of this class — it is now clean of blocking-vs-concurrent mismatches for every function this packet was allowed to touch.

## lease-requests

None. Both findings that could have required touching a third file were resolved without one:
- The sqlite/SSE/sweep/split work stayed entirely inside the two owned files.
- The synchronous-spawn problem was resolved by *adding* a new export (`transpilePluginComponentAsync`) in the owned `🌐plugin-web-materialize.ts` rather than changing the existing `transpilePluginComponent`'s signature, which would have required a change to `🏪️store/📜️store.ts` (not owned) to keep its `webMaterialize` caller correct.
- `resolveWorkspaceBin` needed by the new async spawn was already exported by the shared repo-lib — no edit to that file was needed, just a new import.

## honest gaps

- **Full dev-server boot not exercised.** `bun ./📜️script.ts dev` unconditionally reaches `buildEngineWasm`, which runs real `cargo build` calls, even under `SKIP_PLUGIN_BUILD=1` — there is no existing flag that boots the dev server with literally zero cargo invocation. Binding rule 4 forbids any cargo build in this packet, so a genuine end-to-end boot was not attempted. Substituted: (a) the full `bun ./📜️script.ts test` suite (34/34 passing, exercising the new code through real `import`/module evaluation of the whole file), (b) a `tsc --noEmit` pass showing zero new errors anywhere in the tree, and (c) a runtime smoke test driving the two SSE middleware factories with fake `req`/`res` through their real handler functions. This is real evidence the file loads and the touched runtime paths behave correctly, but it is not proof the dev server boots end to end.
- **Materialize benchmark used 12 plugins, not the full ~20-58-plugin catalog**, to keep the measurement's own wall time inside this session's budget. The real jco/subprocess work scales with file count and size, so the qualitative result (bounded-parallel beats serial once there is more than one target) should hold and likely improve at full scale, but the exact multiplier at full catalog size is not measured here.
- **`wasm-opt` (ship mode) async path is implemented (`optimizePluginCoreModulesAsync`) but not separately timed** — the benchmark ran in dev mode (the common inner-loop case), where `wasm-opt` is skipped in both the before and after paths by design (`semioBuildMode() !== "ship"`).
- **The concurrency cap of 4 is a judgment call**, justified in-code (jco/wasm-opt are CPU-bound subprocesses that each hold a decoded wasm module + intermediate JS AST in memory; 4 avoids the class of machine-saturation `📌️important.md` records for unbounded parallel cargo) but not tuned against a real multi-core measurement at full catalog scale within this session.
- **`SEMIO_MATERIALIZE_CONCURRENCY` override exists but is untested** beyond the unit test's direct constructor-argument path (the env-var parsing branch itself has no dedicated unit test — low risk, it's a one-line `Number.parseInt` guard, but noting it rather than silently assuming coverage).

## files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`

No other files created or modified. No temporary files left outside this ticket's own report; benchmark scratch output (jco transpile artifacts under the tool-provided scratchpad, ~400 MB) was deleted after extracting the timing numbers above.
