# 🌐️ luna-web-state audit — web path readiness after terra-jco-spike verdict

**Scout**: luna-web (read-only)  
**Date**: 2026-08-20  
**Ticket**: 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME  
**Scope**: audit the web bridge generator and materialized bridges against the collapsed async world

## 1. Bridge Generator State (🌐plugin-web-materialize.ts)

### transpilePluginComponent (line 431-449)
- **Current jco invocation**: NO `--async-mode` flag (correct per terra-jco-spike verdict)
- **Maps**: both `pure` and `host-async` to `./🟨️host-shim.js`
- **Flags passed**: `--map "semio:framework/pure=./🟨️host-shim.js" --map "semio:framework/host-async=./🟨️host-shim.js"`
- **Status**: ✅ Correct. Line 432-437 docstring confirms: "NO `--async-mode` flag — confirmed byte-identical to jco's bare/"sync" default for a component whose every WIT function is already `async func`"

### pluginComponentBridgeSource (line 359-379)
- **Current destructure**: `{ reactor, jobs, checkpoint, describe } = await import("./${componentBase}.js")`
- **Exports**: `createActorApi(actorId)` function
- **API shape**: 
  ```typescript
  {
    poll: async (events, budget) => reactor.poll(events, budget),
    startJob: async (job, kind, input) => jobs.startJob(job, kind, input),
    stepJob: async (job, budget) => jobs.stepJob(job, budget),
    cancelJob: async (job) => jobs.cancelJob(job),
    checkpoint: async () => checkpoint.checkpoint(),
    restore: async (state) => checkpoint.restore(state),
    describe: async () => describe.describe(),
    resolveEffect/rejectEffect: (for effect routing)
  }
  ```
- **Status**: ✅ Correct. Takes `actorId` parameter, binds it via `hostShim.__bindHostBridge(actorId)` before returning API

### hostShimSource (line 563-719)
- **Implementation**: NOW handles both `pure` (interface pure) AND `host-async` (interface host-async) in one file
- **Pure interface** (lines 568-581): log, nowMs, traceSpan
- **Host-async interface** (lines 583-717):
  - Module-scoped state: `boundActorId`, `effectSeq`, `pendingEffects`
  - Helper functions: `__bindHostBridge`, `__resolveEffect`, `__rejectEffect`, `streamToByteGenerator`, `effectRequest`, `postFireAndForget`
  - 24 effect exports: storageRead, storageWrite, storageDelete, blobLoad, blobWrite, blobRead, httpFetch, documentRead, documentWrite, linkResolve, registryQuery, ioCompose, ioRun, cacheDerive, cacheRead, invokeExtension, openWindow, openDialog, dispatchAction, spawnPluginInstance, requestFileOpen, requestMediaFrames, requestCapability, spawnJob
  - 2 fire-and-forget exports: emit, emitPatch
- **Status**: ✅ Correct. Fully async-lifted, per-actor binding via `boundActorId`, effect-request/effect-complete flow implemented

## 2. Materialized Bridges (Dev Plugin Modules)

### Count
- **Total stale bridges found**: 109 × `🟨️host-shim.js` files in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules` + `🔌️extension-modules`

### Staleness Status: ALL STALE (pre-rewrite output)
Example from `🧑️‍💻️dev/🔌️plugin-modules/mathematical/🟨️host-shim.js`:
- ❌ Contains old functions: readDocument, writeDocument, openWindow, invokeAction, readAsset, networkFetch
- ❌ Contains old backbone functions: backboneSend, backbonePoll, backboneStatus
- ❌ NO `createActorApi` function
- ❌ NO `host-async` implementation
- ❌ NO effect-request/effect-complete flow
- ❌ NO `__bindHostBridge`, `__resolveEffect`, `__rejectEffect`

**Evidence of staleness**: All 109 files use `runSerialized` retry/reload pattern (DROPPED per important.md), old import surfaces (read-document, write-blob, network-fetch), and synchronous backbone relay — NOT the new jco-marshaled effect-request architecture.

### Banned Symbol Analysis (exchange function)

**Type Definition Files with exchange export**: 106 files
- Path pattern: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️{plugin,extension}-modules/*/interfaces/semio-framework-plugin.d.ts`
- Each exports: `export function exchange(instanceId: number, commands: Array<Uint8Array>): Array<Uint8Array>;`
- **Status**: ❌ BANNED. Per important.md "Replace, never wrap": `exchange` (WIT + all callers) must not exist at exit.

### PluginWorkerClient Search
- **Status**: ✅ NOT LIVE. Found only in:
  - Ticket folder archives (`26/08/07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT/original-component.ts`)
  - Comments in active code (documenting it was deleted)
  - NOT in `🧊️wgpu/🟦️typescript/🟦️boot.ts` (already removed)

## 3. Host-Shim Architecture: What Exists vs What's Needed

### Current Generated hostShimSource (line 563-719)
- ✅ Implements both `pure` AND `host-async` in one module
- ✅ Per-actor binding via `boundActorId` module-scoped state
- ✅ Effect-request/effect-complete Promise correlation
- ✅ Stream adaptation for async generators
- ✅ 24 async effect imports + 2 fire-and-forget doors

### Missing in Current 109 Materialized Bridges
- ❌ `createActorApi(actorId)` function signature
- ❌ `__bindHostBridge(actorId)` binding
- ❌ `__resolveEffect(requestId, value)` resolution
- ❌ `__rejectEffect(requestId, message)` rejection
- ❌ `effectRequest(effect, params)` Promise factory
- ❌ All 24 async effect imports (storageRead, httpFetch, etc.)
- ❌ All per-actor correlation plumbing

## 4. Vite Configuration: _shard Inclusion

**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts`  
**Line 69**: 
```typescript
const pluginModuleDirNames = isHostPluginFilter(plugin) || !resolvedPluginId ? undefined : ["_vendor", "_shard", resolvedPluginId];
```
**Status**: ✅ YES, `_shard` is included. Line 63-68 docstring confirms: "_shard is `🌐plugin-web-materialize.ts`'s generated `🟨️shard-worker.js` bundle — every actor of every plugin now activates through the ONE pooled shard-worker pool"

## 5. TypeScript Test Suites and Runners

### Vitest-Based Test Suites (all use bun + nx)

| Package | Config File | Test Files | Runner | Status |
|---|---|---|---|---|
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript` | `🧪️vitest.config.ts` | In-source: `🧵️shard-client.ts`, `📬️mailbox.ts`, `🧵️turn-scheduler.ts` | vitest (bun/node) | ✅ Live |
| `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript` | `🧪️vitest.config.ts` | In-source: `🟦️component.ts`, `🟦️backbone-worker.ts`, `🟦️effect-backbone.ts` | vitest (bun/node) | ✅ Live |
| `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript` | `🧪️vitest.config.ts` | (likely in-source) | vitest (bun/node) | ✅ Live |

### Plugin Web Package Test Suites
- **Path**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/`
- **Status**: ❌ NO test suite. Directory contains ONLY `🌐plugin-web-materialize.ts` (single generator file, no package.json, no vitest.config, no tests)

### Web/Dev/Playground Test Suites
- **Path**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/`
- **Config**: `🧪️vitest.config.ts` exists
- **Status**: Check that file for test file list

---

## Summary of Findings

### Stale Bridge Count
**109 stale materialized bridges** across `🧑️‍💻️dev/{plugin,extension}-modules/*/🟨️host-shim.js` — all pre-rewrite output using old backbone/document/import surfaces, no async effect routing.

### Banned Symbol Hit List

| Symbol | Count | Locations | Status |
|---|---|---|---|
| `exchange` function | 106 | `*/interfaces/semio-framework-plugin.d.ts` | ❌ LIVE — must be removed |
| `PluginWorkerClient` | 0 (live code) | Only in comments/archived | ✅ Already deleted |

### Action Items for Web Path Completion

1. **Regenerate all 109 materialized bridges** — `transpilePluginComponent` and `hostShimSource` are correct, but their output was never written to disk for dev plugins. Re-run the materialize step to populate bridges with new `createActorApi(actorId)` shape, per-actor `boundActorId` binding, and async effect routing.

2. **Remove 106 exchange type definitions** — all `*/interfaces/semio-framework-plugin.d.ts` exports per important.md rule "Replace, never wrap: exchange must not exist at exit".

3. **Add JSPI capability gate** — confirm `shardWorkerSource()` lines 108-112 (the `typeof WebAssembly.Suspending` check) is deployed before worker bootstrap. This is the explicit gate terra-jco-spike verdict requires.

4. **Verify async world materialization** — once `world actor` lands with all `async func` exports/imports, the plugin-web-materialize.ts mappings stay unchanged (already future-proof), but the generated bridges will require `jobs-async` / `checkpoint-async` field-name updates (noted in report line 6 of terra-jco-spike verdict as cross-packet follow-up, already flagged).
