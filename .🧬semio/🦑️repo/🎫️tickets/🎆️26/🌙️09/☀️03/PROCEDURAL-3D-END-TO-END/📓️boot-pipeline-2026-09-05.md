# Boot Pipeline Trace: `bun run dev:procedural:3d`

**Date**: 2026-09-05  
**Command**: `bun run dev:procedural:3d` → `bun ./📜️script.ts dev procedural 3d`

---

## 1. Ordered Boot Stages

### Stage 1: Command Resolution
- **File**: `/Users/ueli/Documents/semio/📜️script.ts:474-514`
- **Function**: `DevScript.run(segments: ["procedural", "3d"])`
- **Resolution**:
  - Line 500: `resolvePlaygroundDevApp(["procedural", "3d"])` matches playground catalog
  - From `🤖️generated/🎮️playgrounds.ts:53`, variant `"generation3d"` has aliases `["procedural 3d"]`
  - Resolves to: `app: "generation3d"`, `rest: ["3d"]`
  - Line 502: Delegates to `runFrameworkOsPlaygroundDev("generation3d", ["3d"])`

### Stage 2: Framework OS Dev Script Invocation
- **File**: `/Users/ueli/Documents/semio/📜️script.ts:194-202`
- **Function**: `runFrameworkOsPlaygroundDev(plugin: "generation3d", rest: ["3d"])`
- **Action**: Spawns `bun nx run @semio-tech/framework-os-dev:dev -- generation3d 3d`
- **Environment Setup** (line 199):
  - `SEMIO_PLUGIN: "generation3d"`
  - `SEMIO_RENDERER: "react"` (default)
  - `VITE_SEMIO_RENDERER: "react"`
  - `VITE_SEMIO_APP_ID: "s.procedural.generation3d@1/*#editor"` (from playground entry)

### Stage 3: Framework OS Dev Nx Target
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:12-23`
- **Nx Target**: `@semio-tech/framework-os-dev:dev`
- **Command**: `bun ./📜️script.ts dev generation3d 3d` (cwd: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript`)

### Stage 4: OS Dev Script Router
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1827-1975`
- **Class**: `DevScript`
- **Function**: `run(segments: ["generation3d", "3d"])`

#### Sub-stage 4a: Shard Worker Publishing
- **Line**: 1834
- **Function**: `publishShardWorker()`
- **Output**: Writes `🔌️plugin-modules/🧵️shard/🟨️shard-worker.js`
- **Purpose**: One package-agnostic shard worker for all plugins' ShardClient pool

#### Sub-stage 4b: Plugin Registry Preparation
- **Lines**: 1851-1869
- **Variables**:
  - `variantSegment`: `"generation3d"`
  - `filterPlugin`: `"generation3d"` (after line 1851)
  - `renderer`: `"react"` (line 1854, from env or default)
  - `plugin`: `"generation3d"` (line 1855)
  - `defaultPort`: Resolved by `frameworkOsPlaygroundDefaultPort(playgroundCatalog, "generation3d", "react")`
    - From playground entry (line 53): `react: 6018`
  - `streamPluginBuilds`: `true` (line 1864: renderer === "react" && SKIP_PLUGIN_BUILD !== "1")

#### Sub-stage 4c: Plugin Build Lease Management (Optional)
- **Lines**: 1869-1894
- **When**: `streamPluginBuilds` is true
- **Action**: Acquires/manages plugin build lease via port-based locking
- **Purpose**: Prevents duplicate builds when multiple dev sessions run the same variant

#### Sub-stage 4d: Registry Generation & Engine WASM Build
- **Lines**: 1898-1904
- **Conditions**:
  - If `leaseRole === "holder"` OR `streamPluginBuilds || SKIP_PLUGIN_BUILD === "1"`:
    1. **Line 1898**: `await ensurePluginRegistry("generation3d")`
    2. **Line 1899**: `await buildEngineWasm("generation3d", "react")`
    3. **Line 1900**: If holder, mark lease ready

#### Sub-stage 4e: Vite Server Start
- **Lines**: 1950-1966
- **Function**: `runViteBunxDev(this.root, viteSegments: ["3d"], {...})`
- **Port**: From `S_OS_PORT` env or `defaultPort` (6018)
- **Environment Setup**:
  - `SEMIO_PLUGIN: "generation3d"`
  - `SEMIO_RENDERER: "react"`
  - `VITE_SEMIO_RENDERER: "react"`
  - `VITE_SEMIO_PLUGIN: "procedural"` (pluginId from catalog)
  - `VITE_SEMIO_APP_ID: "s.procedural.generation3d@1/*#editor"`
  - Locked prefs env from `frameworkOsLockedPrefsEnv()`

#### Sub-stage 4f: Streaming Plugin Builds (If Holder)
- **Lines**: 1967-1972
- **Function**: `buildPluginsStreaming("generation3d")`
- **Watch Rebuilds**: `watchPluginRebuilds(targets)`
- **Effect**: Plugins compile in background while Vite already listens

---

## 2. Environment Variables That Change Behavior

### Global Build Control

| Variable | Default | Purpose | File:Line |
|----------|---------|---------|-----------|
| `SEMIO_BUILD_BUDGET_MS` | Defined by `buildBudgetMs()` | Timeout for cargo builds | `📜️script.ts:966` |
| `SEMIO_BUILD_MODE` | `"dev"` | Build profile selection (set to `"ship"` in BuildScript) | `BuildScript.run():1980` |
| `SEMIO_PLUGIN_ONLY` | None | Rebuild only ONE plugin crate (for hot-swap iteration on host) | `dev/script.ts:1131-1139` |
| `SKIP_PLUGIN_BUILD` | None | Skip all plugin cargo builds; use pre-built artifacts | `dev/script.ts:1864` |
| `SKIP_ENGINE_BUILD` | None | Skip engine WASM builds (framework-surface, framework-editor, flow-core) | `dev/script.ts:1554` |

### Renderer Selection

| Variable | Default | Purpose | File:Line |
|----------|---------|---------|-----------|
| `SEMIO_RENDERER` | `"react"` | Renderer backend: `"react"` (Vite) or `"wgpu"` (trunk) | `dev/script.ts:1854` |
| `VITE_SEMIO_RENDERER` | Mirrors `SEMIO_RENDERER` | Vite-visible renderer choice | `dev/script.ts:1960` |

### Plugin & App Selection

| Variable | Default | Purpose | File:Line |
|----------|---------|---------|-----------|
| `SEMIO_PLUGIN` | `DEFAULT_HOST_VARIANT` (`"s"`) | Active playground plugin ID | `dev/script.ts:1853` |
| `PLAYGROUND_APP_KIND` | None | Fallback app ID if `SEMIO_PLUGIN` unset | `dev/script.ts:1853` |
| `SEMIO_BRAND` | Resolved from playground | Shell brand ID override | `vite.config.ts:24` |

### Port & Network

| Variable | Default | Purpose | File:Line |
|----------|---------|---------|-----------|
| `S_OS_PORT` | Playground-specific (6018 for generation3d) | React dev server port | `dev/script.ts:1856`, `vite.config.ts:119` |
| `S_OS_MCP_PORT` | `"6300"` | OS MCP gateway port (not used for dev:procedural:3d) | `dev/script.ts:672` |
| `S_LOCAL_RELAY_URL` | None | Local relay endpoint for collaborative sessions | `vite.config.ts:121` |
| `S_LOCAL_RELAY_SECRET` | None | Relay authentication header | `vite.config.ts:126` |

### Component Builds

| Variable | Default | Purpose | File:Line |
|----------|---------|---------|-----------|
| `SEMIO_PLUGIN_PROFILE` | Auto-selected wasm profile | Override wasm-dev/wasm-release profile | `dev/script.ts:98-99` |
| `SEMIO_PLUGIN_SYMBOLS` | None | Strip=none (keep debug symbols) if `"1"` | `dev/script.ts:105` |
| `SEMIO_MATERIALIZE_CONCURRENCY` | 4 | Parallel jco/wasm-opt processes during materialize stage | `dev/script.ts:1059-1062` |
| `CARGO_TARGET_DIR` | `./target` (repo root) | Cargo output directory override | `dev/script.ts:969` |

### Development Infrastructure

| Variable | Default | Purpose | File:Line |
|----------|---------|---------|-----------|
| `DEVCONTAINER` | `"false"` | Bind to `0.0.0.0` instead of `127.0.0.1` if `"true"` | `dev/script.ts:1906`, `vite.config.ts:118` |
| `DEVELOPER_DIR` | Set in project.json | macOS Xcode tools (set by os-dev nx target) | `project.json:19` |
| `SDKROOT` | Set in project.json | macOS SDK root (set by os-dev nx target) | `project.json:20` |

---

## 3. React Renderer Port Mapping for App `generation3d`

### Port Resolution

| Mapping | Value | Source |
|---------|-------|--------|
| **Variant** | `generation3d` | `playgrounds.ts:53` |
| **React Port** | `6018` | `playgrounds.ts:53`, `ports.react` field |
| **WGPU Port** | `6118` | `playgrounds.ts:53`, `ports.wgpu` field |
| **Port Env** | `S_OS_PORT` | `dev/script.ts:1856`, `vite.config.ts:119` |
| **Fallback** | `6066` | `vite.config.ts:119` (only if `S_OS_PORT` unset AND not playground-resolved) |

### Port Authority Definition

- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts`
- **Line**: 53 (for `generation3d` variant)
- **Generation**: Auto-generated by `@semio-tech/plugin-registry:generate` from plugin catalog discovery

---

## 4. Plugin Component (WASM) Build & Cache Locations

### Built Component Output Location

**Directory**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🌀️procedural/`

**Files Written**:
- `semio_s_plugin_procedural_component.core.wasm` — Jco-extracted core WASM (78.7 MB, from `target/wasm32-wasip2/{profile}/`, line `dev/script.ts:970`)
- `semio_s_plugin_procedural_component.js` — Jco transpiled component bindings
- `semio_s_plugin_procedural_component.d.ts` — TypeScript typings
- `🌉️bridge.js` — Runtime bridge source (line `dev/script.ts:998`)
- `🟨️.js` — Host shim source (line `dev/script.ts:987`)
- `🔣️.json` — Descriptor (from plugin crate's owner root)
- `🛂️.descriptor.semio` — Hashes & metadata

**Mtime Check**:
```bash
ls -la 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🌀️procedural/
```
Expected output (current state):
- `semio_s_plugin_procedural_component.core.wasm`: Sep 1 11:06 (stale, needs rebuild)
- `semio_s_plugin_procedural_component.js`: Sep 4 17:18 (recent)

### Cargo Artifact (Pre-Materialize)

**Location**: `target/wasm32-wasip2/{profile}/semio_s_plugin_procedural.wasm` (or specified `CARGO_TARGET_DIR`)
- **Profile**: Auto-selected from `pluginWasmProfile()` (wasm-dev for dev, wasm-release for ship)
- **Line**: `dev/script.ts:970`
- **Accessed by**: `buildPluginCargo()` → `materializePlugin()` (line 982, 994)
- **Size**: ~60 MB (before jco extraction reduces to core.wasm)

### Cached/Stale Detection

- **Hot-Swap Marker**: `🔌️plugin-modules/🌉️hot-swap.json` (line `dev/script.ts:1002`)
- **Stale Check**: `pluginBuildOutputsPresent()` (line `dev/script.ts:1889`) scans `🔌️plugin-modules/` for pre-existing plugin dirs
- **Purpose**: Follower processes wait for holder's outputs before serving; if none appear, they build themselves

### Force Rebuild of Procedural Component Only

**Method 1: SEMIO_PLUGIN_ONLY**
```bash
SEMIO_PLUGIN_ONLY=procedural bun run dev:procedural:3d
```
- Rebuilds only the `procedural` plugin, ignoring host/filter scoping (line 1131-1139)

**Method 2: Delete Output & Restart**
```bash
rm -rf 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🌀️procedural/
bun run dev:procedural:3d
```
- Forces a fresh build since `pluginBuildOutputsPresent()` will fail (line 1889)

**Method 3: Touch Procedural Source**
```bash
touch ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/src/lib.rs
bun run dev:procedural:3d
```
- File watcher (`watchPluginRebuilds()`, line 1972) detects change and rebuilds

---

## 5. Registry Generation Validation (Taxonomy Schema)

### Validation Layers

#### Layer 1: Taxonomy Load & Authority
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:111-139`
- **Function**: `loadCatalogTaxonomy()` (line 111)
- **Checks**:
  1. Loads `🔣️taxonomy.json` (shared repo-wide contract)
  2. Validates `pluginAreas` array is non-empty and all areas exist in `taxonomy.areas` (line 124-127)
  3. Merges area states to most permissive (legacy > mixed > clean, line 131-139)
  4. Affects whether findings warn vs. fail (line 139)

#### Layer 2: Component Package Discovery
- **File**: `registry/📜️script.ts:170-172`
- **Function**: `discoverComponentPackages(repoRoot, packages)`
- **Filter**: Only packages with `lang === "🦀️rust"` AND `role` in `["plugin", "extension"]`
- **Source**: Shared `discoverCatalogPackages()` from repo-lib (line 170)

#### Layer 3: Component Package ID Validation
- **File**: `registry/📜️script.ts:76-102`
- **Function**: `parseComponentPackageId(text: string, manifestPath: string)`
- **Checks**:
  1. Manifest ≤ 64 KiB (line 78, `COMPONENT_MANIFEST_MAX_BYTES`)
  2. Exactly one `[package.metadata.component]` section (line 86-88)
  3. `package = "semio:<lowercase-alnum-or-hyphen>"` (line 101, `COMPONENT_PACKAGE_ID` regex)
  4. Fails the gate if malformed (line 100-102)

#### Layer 4: Registry Catalog Rendering
- **File**: `registry/📜️script.ts:1873`
- **Function**: `renderCatalogFiles(repoRoot)`
- **Outputs**: `entries`, `playgrounds`, `frameworkPackages`
- **Written to**: `🤖️generated/` (line 1874-1878)

#### Layer 5: Descriptor Hash Verification (Check Gate Only)
- **File**: `registry/📜️script.ts:1963-2017`
- **Function**: `validatePluginDescriptorGate()` (part of CheckScript, line 2958+)
- **Checks** (for each plugin):
  1. Descriptor file exists at owner root (line 189-195)
  2. `pluginId` matches component package (line 1964)
  3. `extends` matches first dependency for extensions (line 1965)
  4. Extension-request activation events name real extension points (line 1965-1966)
  5. Built WASM SHA-256 matches descriptor's `hashes.wasmSha256` (line 1966)
- **Severity**: Currently WARN-ONLY (no plugins emit descriptor yet; see line 1968 comment)

### What Makes Registry Generation Fail

1. **Missing Taxonomy**: `loadCatalogTaxonomy()` throws if `🔣️taxonomy.json` invalid or pluginAreas missing
2. **Malformed Manifests**: `parseComponentPackageId()` fails if manifest > 64 KiB or package ID doesn't match regex
3. **Missing plugin.rs Files**: `discoverComponentPackages()` returns empty if no `📦️packages/🦀️rust/` Cargo.toml found
4. **Stale Plugin Crates**: If a plugin's Cargo.toml references a non-existent component dependency
5. **IO Failures**: `writeFileSync()` (line 1878) fails if output dir not writable

---

## 6. Current Pre-Built Artifacts & Staleness

### Plugin Components (as of 2026-09-05)

```bash
ls -la 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🌀️procedural/
```

| File | Mtime | Size | Status |
|------|-------|------|--------|
| `semio_s_plugin_procedural_component.core.wasm` | Sep 1 11:06 | 78.7 MB | ⚠️ STALE (5+ days old) |
| `semio_s_plugin_procedural_component.js` | Sep 4 17:18 | 562 KB | Recent |
| `semio_s_plugin_procedural_component.d.ts` | Sep 1 11:06 | 3.6 KB | ⚠️ STALE |
| `🌉️bridge.js` | Sep 1 22:58 | 9.4 KB | ⚠️ STALE |
| `🟨️.js` | Sep 1 17:03 | 6.8 KB | ⚠️ STALE |
| `🔣️.json` | Sep 4 17:18 | 992 KB | Recent |
| `🛂️.descriptor.semio` | Sep 1 11:06 | 215 KB | ⚠️ STALE |

**Implication**: Core WASM (11:06 Sep 1) is 5+ days older than descriptor (17:18 Sep 4), indicating SHA mismatch risk if registry `check` gate runs.

### Framework Engine WASMs

#### ✍️editor (Framework-Editor)
```bash
ls -la 🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/
```
- `framework_editor_bg.wasm`: Sep 4 21:25 (recent)
- Built via `buildEngineWasm()` (dev/script.ts:1560)

#### 🗺️surface (Framework-Surface-Node-Graph)
```bash
ls -la 🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/pkg/
```
- **Missing**: No `pkg/` dir present
- Must rebuild on first `bun run dev:procedural:3d` (line 1558)

#### 🌊️flow (Flow-Core)
```bash
ls -la 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/pkg/
```
- **Missing**: No `pkg/` dir present
- Must rebuild on first `bun run dev:procedural:3d` (line 1562)

### Registry Artifacts

```bash
ls -la 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/
```
- `🎮️playgrounds.ts`: Generated at every registry refresh
- `🟦️.ts`: Generated at every registry refresh
- Auto-regenerated, not manually tracked (line 1873-1878)

### Hot-Swap Marker (Most Recent Operation)

```bash
cat 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/♻️hot-swap.json
```
- Expected: `{"pluginId": "procedural", "rebuiltAt": <unix-ms>}`
- Written last by materialize stage (dev/script.ts:1002-1003)
- Shows which plugin was most recently rebuilt

---

## Summary

**Boot Flow**: User command → Root script.ts → Framework-os-dev nx target → Dev script → (Lease mgmt) → Plugin registry generate → Engine WASM build → Vite server (port 6018) → Streaming plugin builds.

**Key Ports**: React on **6018**, WGPU on 6118 (for `generation3d`/`procedural 3d`).

**Plugin Output**: `🔌️plugin-modules/🌀️procedural/` (semio_s_plugin_procedural_component.{core.wasm,js,d.ts,js} + bridge + shim).

**Staleness Risk**: Core WASM (Sep 1) older than descriptor (Sep 4); engine WASMs for surface & flow-core missing entirely—first run rebuilds them.

**Registry Validation**: Taxonomy contract → component discovery → package ID regex → manifest bounds check → hash verification (check gate only, currently warn-only).
