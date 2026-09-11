# Dev Boot and Plugin Wasm Component Pipeline — NX Caching Analysis

## 1. Launch Configurations (.claude/launch.json:line 1-231)

### Environment Variables Across Dev Servers
- **procedural3d-react** (line 36-47): `SEMIO_RENDERER=react`, `S_OS_PORT=6018`
- **procedural3d-wgpu** (line 50-63): `SEMIO_RENDERER=wgpu`, `S_OS_PORT=6118`
- **puzzle3d-react** (line 66-77): `SEMIO_RENDERER=react`, `S_OS_PORT=6013`
- **puzzle3d-wgpu** (line 80-93): `SEMIO_RENDERER=wgpu`, `S_OS_PORT=6113`
- **mit-bestand-demonstrator-fast** (line 165-180): Sets `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-engines`, `SEMIO_BUILD_BUDGET_MS=5400000`, `RUSTFLAGS=-Awarnings`, `CARGO_TERM_QUIET=true`, `SKIP_PLUGIN_BUILD=1`
- **mit-bestand-demonstrator-noengine** (line 204-217): Also sets `SKIP_ENGINE_BUILD=1`

**Key finding**: Dedicated private `CARGO_TARGET_DIR=target-engines` used for engine builds; plugin builds default to shared `target/` or `process.env.CARGO_TARGET_DIR`.

## 2. OS/Dev Script Entry Points

### React Renderer Path (devScript: line 1439-1500)
1. Calls `activatePlaygroundRuntime(plugin, profile)` (line 1449) → runs nx target `@semio-tech/framework-os-dev:activate-${variant}-react-${profile}`
2. Then calls `new ServeScript(this.root).run([plugin, renderer, profile, ...serverArgs])` (line 1450)
3. ServeScript (line 1413-1422): Reads activation receipt, runs `runViteBunxDev` with vite dev

### WGPU Renderer Path (devScript: line 1453-1498)
1. Calls `buildPlugins(plugin)` (line 1455) → direct cargo builds, not nx
2. Calls `buildEngineWasm(plugin, renderer)` (line 1456) → delegates to WGPU package script
3. Spawns wgpu trunk dev server at `WGPU_PACKAGE_ROOT` (line 1485-1494)

## 3. Plugin Wasm Component Build Pipeline

### Cargo Build Phase (buildPluginCargo: line 341-353)
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:341`
- **Command**: `cargo rustc -p <package> --target wasm32-wasip2 --profile <profile> -C link-arg=-zstack-size=8388608`
- **Profile Selection**: `pluginWasmProfile()` → calls `selectComponentWasmProfile("dev", override)` (line 128-129)
- **Artifact Output**: `${cargoTargetRoot}/wasm32-wasip2/${cargoProfileDir(profile)}/${packageName.replace(/-/g, "_")}.wasm`
- **Target Directory**: Uses `process.env.CARGO_TARGET_DIR` or `{repoRoot}/target` (line 344)
- **Special Handling**: If `ownedTargetRoot` is set (private build), uses that + disables incremental + clears wrapper (line 347)

### Materialization Phase (materializePlugin: line 362-386)
- **Location**: `🔌️plugin-modules/${moduleDirectoryName(target.pluginId)}/` (line 363)
- **Steps**:
  1. `transpilePluginComponentAsync()` (line 375): jco transpile from cargo wasm → .js + .core.wasm
  2. `describeBuiltPlugin()` (line 376): Runs descriptor probe, writes sha256 hashes
  3. `stagePluginDescriptor()` (line 377): Copies 🔣️.json and 🛂️.descriptor.semio from owner root
  4. Writes bridge JS files (MODULE_BRIDGE_FILE)
  5. `publishBuiltExtension()` (line 382): Copies extension artifacts to 🧩️extension-modules/
  6. Writes hot-swap marker (line 384)

### Catalog Build Orchestration (buildPluginCatalog: line 578-612)
- **Cargo stage**: Serial execution, one plugin at a time (line 591)
- **Materialize stage**: Bounded pool (default 4, configurable via `SEMIO_MATERIALIZE_CONCURRENCY`) (line 585)
- **Concurrency strategy**: Cargo builds serially to avoid shared target/ lock contention; materialize tasks enqueued into bounded pool WITHOUT waiting for previous target's full completion (line 598-607)
- **publishShardWorker()** called once at end (line 610)

### Plugin Component Targets (🟨️.mjs: line 705-733)
**For each plugin crate**, nx dynamically generates two cached targets:

```
component-<profile>:
  cache: true
  parallelism: false
  inputs: ["nativeSources", "^nativeSources", ...commandInputs, env: SEMIO_PLUGIN_SYMBOLS]
  outputs: {projectRoot}/dist/component-<profile}
  command: bun <LIBRARY_ROOT>⚡️caching/🦀️cargo/📜️script.ts native component <profile>
  
materialize-<profile>:
  cache: true
  dependsOn: [component-<profile>, @semio-tech/framework-plugin-web:support-<profile>, (release: workspace:deps-wasm-opt)]
  inputs: ["production", "^production", dependentTasksOutputFiles, web/typescript/**, deployment/catalog.json]
  outputs: {workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/<profile>/🔌️plugin-modules/<moduleDirectory>
  command: bun 🔌️plugin/📦️packages/🟦️typescript/📜️script.ts materialize <profile>
```

## 4. React Dev Activation Pipeline

### Dynamic Targets (🟨️.mjs: line 758-804)
For each playground variant, nx generates:

```
prepare-<variant>-react-<profile>:
  cache: false
  parallelism: false
  dependsOn: [
    @semio-tech/plugin-registry:session-<variant>,
    @semio-tech/framework-plugin-web:support-<profile>,
    semio-framework-os-infinite:fonts,
    <engines>:wasm,
    <selected-plugins>:materialize-<profile>
  ]
  command: bun ./📜️script.ts prepare <variant> react <profile>

activate-<variant>-react-<profile>:
  cache: false
  parallelism: false
  dependsOn: [prepare-<variant>-react-<profile>]
  command: bun ./📜️script.ts activate <variant> react <profile>

serve-<variant>-react-<profile> | dev-<variant>-react-<profile>:
  cache: false
  continuous: true
  dependsOn: [activate-<variant>-react-<profile>]
  command: bun ./📜️script.ts serve <variant> react <profile>

build-<variant>-react-release:
  cache: true
  inputs: ["production", "^production", dependentTasksOutputFiles (transitive), distribution inputs]
  outputs: {projectRoot}/dist/build-<variant>-react-release
  dependsOn: [prepare-<variant>-react-release, @semio-tech/assets:build]
```

### Activation Script (ActivationScript: line 1379-1411)
1. Reads activation receipt or computes new one
2. Computes digest from support files (preview2 shim, shard worker, fonts) + each plugin module
3. For changed plugins: calls `publishActivatedExtension()` (line 1404)
4. Publishes new receipt (line 1407)
5. **Output**: Runtime root + activation receipt files

## 5. Browser/WGPU Renderer Builds

### Plugin Component Output Location
- **File written by**: `materializePlugin()` → `transpilePluginComponentAsync()` (invoked in line 375)
- **Output path**: `{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/{profile}/🔌️plugin-modules/{moduleDirectory}/`
- **Files**: `{component}.js`, `{component}.core.wasm`, `🟨️.js` (host shim), `🏗️.js` (bridge)
- **Descriptor**: `🛂️.descriptor.semio`, `🔣️.json` (published alongside, read at runtime)

### Asset Caching
- **PLUGIN_MODULES_ROOT**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/` (defined: `🔌️vite-plugins.ts:22`)
- **browserModuleRoot(profile)**: Runtime path for module consumption (defined in plugin/📦️packages/🟦️typescript)
- **Shard worker**: Single shared instance at `🔌️plugin-modules/🧵️shard/🧵️.js`, rewritten on every plugin build (line 98-102)
- **Preview2 vendor**: `preview2-shim/**` vendored + rewritten in already-staged plugin JS (line 197-209)

## 6. Where Caching Works vs. Missing

### Cached Targets (nx)
✓ **component-dev / component-release**: Cargo wasm build cached per plugin
✓ **materialize-dev / materialize-release**: jco transpile + wasm-opt cached per plugin
✓ **@semio-tech/plugin-registry:session-<variant>**: Session generation cached
✓ **@semio-tech/framework-plugin-web:support-<profile>**: Web support assets cached
✓ **Engine wasm targets** (surface, editor, flow-core): Cached per engine package
✓ **build-<variant>-react-release**: Final vite dist build cached

### Uncached / Outside NX (Direct Cargo)
✗ **buildPlugins()** for WGPU renderer: Calls `buildPluginCargo()` DIRECTLY from DevScript, not via nx (line 1455)
  - Runs raw `cargo rustc` commands in dev script, bypassing nx task caching
  - All 50+ plugins rebuilt serially + materialized, even if plugin binaries unchanged
  - Cannot reuse cached component outputs from prior nx builds

✗ **Preparation stage** (prepare-*-react-*): Runs outside nx caching (cache: false) (line 788)
  - Runs `bun ./📜️script.ts prepare` directly, reads staged component outputs manually
  - No nx input/output validation or incremental checking

✗ **Activation stage** (activate-*-react-*): Runs outside nx caching (cache: false) (line 786)
  - Reads activation receipt, manually invokes `publishActivatedExtension()` per plugin

✗ **Shard worker & preview2 shim**: Rewritten on EVERY plugin build, even when source unchanged (line 97, 206-209)
  - Not input-gated; no cache key validation
  - Touches all plugins via hot-swap marker write (line 384)

## 7. Key Gaps: How to Cache Package-Level Builds for Dev Consumption

### Gap 1: WGPU Dev Path Bypasses NX
**Status**: Direct `buildPlugins(plugin)` call runs raw cargo + materialize, never checks nx cache
**Solution**:
- Export plugin targets from plugin crates with component-dev/materialize-dev
- Change DevScript WGPU path to run `nx run @semio-tech/framework-os-dev:plugin` (or new `plugin-materialize`) that depends on all selected plugins' `materialize-dev`
- Reuse cached materialize outputs instead of re-running cargo

### Gap 2: Materialize Outputs Not Reused Across Contexts
**Status**: materialize-* target outputs to `dist/<profile>/🔌️plugin-modules/` but dev script reads from `🔌️plugin-modules/` (PLUGIN_MODULES_ROOT)
**Solution**:
- Make dev materialize outputs point to live `🔌️plugin-modules/` path OR symlink/copy after cache hit
- Add nx target that guarantees `🔌️plugin-modules/` is populated from cache before dev starts

### Gap 3: Preparation & Activation Run Every Dev Session
**Status**: prepare-* and activate-* declared cache: false, never reused
**Solution**:
- Add cache: true to prepare-* if inputs are deterministic (plugin modules + web support + engines)
- Add inputs that capture all dependency outputs (transitive dependentTasksOutputFiles)
- Gate on activation receipt hash or module digest changes

### Gap 4: Shard Worker & Shim Rewritten Unconditionally
**Status**: publishShardWorker() and rewritePreview2ShimImports() run every build, no change detection
**Solution**:
- Add hash comparison: only rewrite if source or target changed
- Make shim/worker a standalone nx target with inputs: [shardWorkerSource version] and outputs: [shim path]
- Depend on that target in materialize instead of inline rewrites

### Gap 5: Private CARGO_TARGET_DIR Usage Limited to Fast Builds
**Status**: Only mit-bestand-demonstrator-fast uses target-engines; dev builds contend on shared target/
**Solution**:
- Make dev serve scripts set CARGO_TARGET_DIR to a dev-specific isolated dir (e.g., target-dev-<renderer>)
- Prevents concurrent dev sessions blocking each other on cargo lock
- Pair with nx task caching so second dev session reuses first's component outputs

## Summary

**Currently Cached** (2 paths):
- npm build-time: component-dev/release, materialize-dev/release, web support, engine wasm
- Ship build-time: build-<variant>-react-release vite output

**Currently NOT Cached** (3 paths that should be):
1. WGPU dev plugin builds — runs raw cargo serially, ignores nx caches
2. React prepare/activate stages — cache: false, re-scans every session
3. Shard/shim rewrites — unconditional file writes, no change detection

**Entry Points to Refactor**:
- `DevScript.run()` line 1439: Make WGPU path depend on nx plugin materialize targets
- `playgroundPreparationTargets()` 🟨️.mjs:787: Promote prepare-* to cache: true with full input closure
- `materializePlugin()` line 362: Add hash gating for shard/shim rewrites or promote to separate nx target
