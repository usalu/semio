# S-Dev Pipeline Analysis: `bun ./📜️script.ts dev s`

## 1. Call Chain

**Entry point:** `/Users/ueli/Documents/semio/📜️script.ts` line 173-179
```typescript
function runFrameworkOsPlaygroundDev(plugin: string, rest: string[] = []): void {
  runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:dev", "--", plugin, ...rest], {
    cwd: WORKSPACE_ROOT,
    env: frameworkOsPlaygroundDevEnv(ensureFrameworkOsPlaygroundCatalog(), plugin),
    ...daemonBudgetOpts(),
  });
}
```

- Calls: `bun nx run @semio-tech/framework-os-dev:dev -- s`
- **Project config:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json` lines 12-22
- Runs: `bun ./📜️script.ts dev s` (with forwardAllArgs=true)
- **Handler:** `DevScript.run(["s"])` at `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` line 1824-1972

## 2. Plugin "s" Registry Resolution

**Catalog location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts` line 72

```typescript
{ variant: "s", pluginId: "s", cratePath: "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust", 
  aliases: [], ports: { react: 6070, wgpu: 6066 }, 
  userPorts: { react: [6072,6073], wgpu: [6067,6068] }, 
  examples: [], engines: [], assets: [] }
```

**Plugin registry entry:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts` line 69

```typescript
{ pluginId: "s", cratePath: "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust", 
  wasmOut: "semio_s_plugin_space.wasm", role: "plugin", capabilities: ["documents.write"],
  host: { landingAppId: "home", hostAppId: "studio" }, 
  executionMode: "isolated", ... }
```

**Key fact:** Plugin "s" has `host: {...}` property — it's a **host plugin**.

## 3. Build Scope Decision (All Plugins vs Selected)

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` lines 1148, 1176-1180

```typescript
// Line 1148 in resolvePluginBuildTargets():
if (!filterPlugin || isHostPluginFilter(filterPlugin)) return entries;
if (entries.length === 0) {
  throw new Error(`no program build targets for filter ${JSON.stringify(filterPlugin)}`);
}
return entries;

// Lines 1176-1180 in preparePluginBuildTargets():
if (filterPlugin && !isHostPluginFilter(filterPlugin)) {
  console.log(`program build scope: ${targets.map((target) => target.pluginId).join(", ")}`);
} else {
  console.log(`program build scope: all (${targets.length} plugin crates)`);
}
```

**Determination:** `isHostPluginFilter("s")` returns `true` (per line 517 checking the plugin's `host` metadata), so:
- ✅ **Builds ALL plugins** in the catalog (~58 plugin crates)
- Prints: `program build scope: all (58 plugin crates)` (actual count varies)

## 4. Build Pipeline Stages

### 4a. Vite Startup (Non-Blocking)

**Location:** Line 1950-1963 in DevScript.run()

```typescript
const viteDone = runViteBunxDev(this.root, viteSegments, {
  portEnv: "S_OS_PORT",
  defaultPort,
  fixedPort: true,
  env: {
    SEMIO_PLUGIN: plugin,
    SEMIO_RENDERER: renderer,
    VITE_SEMIO_RENDERER: renderer,
    VITE_SEMIO_PLUGIN: resolvedFilter.pluginId,
    ...(resolvedFilter.appId ? { VITE_SEMIO_APP_ID: resolvedFilter.appId } : {}),
    ...(resolvedFilter.brand && !process.env.SEMIO_BRAND ? { SEMIO_BRAND: resolvedFilter.brand } : {}),
    ...frameworkOsLockedPrefsEnv(),
  },
});
```

- Does NOT await initially — Vite starts listening while plugin builds stream in
- **Port:** `process.env.S_OS_PORT` or default `6070` (from catalog)
- **Renderer:** `react` (default, can be overridden by `SEMIO_RENDERER`)

### 4b. Engine WASM Builds (React Only)

**Location:** Line 1896 (reactor path, streaming) or 1900 (non-streaming)

```typescript
await buildEngineWasm(plugin, renderer);
```

**Function:** Line 1549-1567

Builds unconditionally (unless `SKIP_ENGINE_BUILD=1`):
1. **framework-surface node-graph wasm** → `./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts wasm`
2. **framework-editor wasm** → `./🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts wasm`
3. **flow-core wasm** → `./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts wasm`
4. **Per-variant engines** (line 1561-1566): Plugin "s" declares no engines (`engines: []` in catalog)

**Each call:** `cargo/wasm-pack build` with budget tracking.

### 4c. Plugin Compilation (Streamed During Vite)

**Location:** Line 1964-1970 (after viteDone promise is spawned but NOT awaited initially)

```typescript
if (leaseRole === "holder") {
  await buildPluginsStreaming(filterPlugin);
  const filterPluginId = resolveCatalogFilterPluginId(filterPlugin);
  const catalogEntries = generatePluginRegistry(repoRoot, filterPluginId ? { filterPlaygroundPlugin: filterPluginId } : {});
  const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin);
  watchPluginRebuilds(targets);
}
await viteDone;
```

#### 4c-i: Plugin Registry Preparation

**Location:** Line 1160-1182 (`preparePluginBuildTargets`)

- Generates `@semio-tech/plugin-registry:generate` if catalog doesn't exist
- Reads plugin metadata and dependency graph
- Logs: `program build scope: all (58 plugin crates)` for host filter "s"
- Returns all PluginRegistryEntry targets

#### 4c-ii: Plugin Cargo Compilation

**Location:** Line 1191-1201 (`buildPlugins` → `buildPluginCatalog`)

For each target in all-plugins list:

1. **Cargo build** (line 973, `buildPluginCargo`):
   ```typescript
   cargo rustc -p semio-s-plugin-space --target wasm32-wasip2 --profile dev
       -- -C link-arg=-zstack-size=8388608
   ```
   
   - **Target:** `wasm32-wasip2` (WebAssembly Component Model, WASI Preview 2)
   - **Profile:** `dev` (default) or `wasm-release` if `SEMIO_BUILD_MODE=ship`
   - **Stack size:** 8 MiB (`PLUGIN_WASM_STACK_BYTES = 8 * 1024 * 1024`)
   - **Artifact path:** `target/wasm32-wasip2/dev/semio_s_plugin_space.wasm` (line 977)
   - **CARGO_TARGET_DIR:** Respects `$CARGO_TARGET_DIR` env, defaults to `./target`

2. **Materialize** (line 988-1012, `materializePlugin`):
   - **jco transpile** (async) → extract core wasm, JS bindings
   - **wasm-opt** optimization
   - **Output:** `🔌️plugin-modules/s/` directory
     - `semio_s_plugin_space_component.core.wasm` (core after jco extraction)
     - `semio_s_plugin_space_component.js` (jco transpiled)
     - `semio_s_plugin_space.js` (plugin bridge)
     - `🟨️host-shim.js` (plugin host wrapper)
   - Write hot-swap marker: `🔌️plugin-modules/.hot-swap`

**Concurrency:** 
- Cargo stage: Serial (shared `target/` lock)
- Materialize stage: Bounded to 4 concurrent (default, `SEMIO_MATERIALIZE_CONCURRENCY` override)
- Best-effort in streaming mode: failures don't abort remaining targets

## 5. Vite Configuration & Port 6070

**Config file:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts`

### Server Configuration (Line 105-113)

```typescript
server: {
  port: Number(process.env.S_OS_PORT ?? 6066),  // Default 6066, overridden to 6070 by catalog
  strictPort: true,
  fs: { allow: [repoRoot, pluginModulesDir, installedExtensionsDir, rendererModulesDir] },
  watch: {
    ignored: ["**/📇️registry/🤖️generated/**", "**/🤖️generated/**", "**/.vscode/launch.json"],
  },
}
```

- **Port resolution:** `process.env.S_OS_PORT` (from launch.json's `S_OS_PORT=6070`) or defaults to `6066`
- For "s" plugin in React mode, catalog defines `ports: { react: 6070 }` → Vite listens on **6070**

### Entry Point (Line 73-75, Plugins 115-123)

```typescript
root: playDir,  // ./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/
```

Entry via `semioHostHtmlVitePlugin` (line 115-123):
```typescript
entry: "/🟦️component.ts",
```

Resolves to: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️component.ts`

This is the shell/host React component that loads plugins dynamically.

### Plugin Serving (Lines 131-143)

```typescript
pluginModuleDirNames = isHostPluginFilter(plugin) || !resolvedPluginId ? undefined : ["_vendor", "_shard", resolvedPluginId];
```

For host "s": `pluginModuleDirNames = undefined`

Result: Serves **entire** `/plugin-modules/` directory at `/plugin-modules/` route:
- `/plugin-modules/s/` → wasm + js for space plugin
- `/plugin-modules/puzzle/` → wasm + js for puzzle plugin
- ... (all ~58 plugins)

### Hot Reload (Lines 126-128)

- `semioBackboneVitePlugin()`: File/folder I/O via SQLite (for document storage)
- `semioBlobVitePlugin()`: Blob endpoints
- `semioPluginHotSwapVitePlugin()`: SSE at `/plugin-modules/watch` for hot-reload on plugin rebuild

## 6. Artifact Locations Summary

| Stage | Input | Output Path | Format |
|-------|-------|-------------|--------|
| Cargo compile | `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/src/lib.rs` | `target/wasm32-wasip2/dev/semio_s_plugin_space.wasm` | WASM component |
| jco transpile | ↑ Cargo artifact | `🔌️plugin-modules/s/semio_s_plugin_space_component.core.wasm` | Extracted core |
| jco + wasm-opt | ↑ | `🔌️plugin-modules/s/semio_s_plugin_space_component.js` | JS bindings |
| Bridge gen | Metadata | `🔌️plugin-modules/s/semio_s_plugin_space.js` | Runtime bridge |
| Host shim | Template | `🔌️plugin-modules/s/🟨️host-shim.js` | Plugin host API |
| Hot-swap marker | All plugins | `🔌️plugin-modules/.hot-swap` | JSON: `{pluginId, rebuiltAt}` |
| Vite dev dist | Entry | (in-memory) | React shell |
| HTML | semioHostHtmlVitePlugin | http://127.0.0.1:6070/ | React root, loads `/🟦️component.ts` |

## 7. Engine WASM Artifacts

These are built by `buildEngineWasm` and placed in npm/wasm-pack output directories (not dev's `🔌️plugin-modules`):

- `./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/pkg/` → `@semio-tech/framework-surface-*-rs`
- `./🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/` → `@semio-tech/framework-editor-rs`
- `./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/` → `@semio-tech/flow-core`

Vite dev server bundles these into the React app (optimizeDeps exclude list at line 155).

## 8. Sequencing & Port Timeline

1. **t=0ms**: Launch dev script, set env
2. **t=50ms**: Ensure catalog exists (run `@semio-tech/plugin-registry:generate` if needed)
3. **t=100ms**: Start Vite server on port 6070 (non-blocking spawn)
4. **t=150ms**: Build framework engines (surface, editor, flow-core) → wasm-pack output
5. **t=1000ms+**: Build all ~58 plugins (streaming) via `buildPluginsStreaming`:
   - Cargo builds in series (one per time)
   - Materialize (jco/wasm-opt) runs bounded-parallel (4 at a time)
   - Hot-swap marker written per plugin (browser SSE endpoint polls this)
6. **t=5000ms+**: First plugin appears in `🔌️plugin-modules/s/`, browser fetches it
7. **t=30000ms+**: All plugins built, dev session fully functional

**Port 6070 is live and accepting HTTP from t=150ms onward**, but displays "waiting for host program" until plugin "s" (the host) finishes materialize around t=2000ms.

