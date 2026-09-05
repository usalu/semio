# Lowpoly App Boot Chain: End-to-End Trace

## 1. Package Script Entry Point

**File:** `/Users/ueli/Documents/semio/package.json` (line containing `dev:lowpoly`)

```
"dev:lowpoly": "bun ./📜️script.ts dev lowpoly",
```

This invokes the root `📜️script.ts` with the `dev` command and `lowpoly` as the first argument.

## 2. Root Script Processing Chain

### Step 2a: DevScript.run() — `/Users/ueli/Documents/semio/📜️script.ts:475-520`

The `DevScript` class handles the `dev lowpoly` segments:

1. **Line 485:** Check if segment is `s` — NO
2. **Line 489:** Check if segment is `multi` — NO
3. **Line 502:** Call `resolvePlaygroundDevApp(segments)` which resolves `"lowpoly"` to a playground app
4. **Line 502:** Call `runFrameworkOsPlaygroundDev(playgroundApp.app, playgroundApp.rest)`

### Step 2b: resolvePlaygroundDevApp() — `/Users/ueli/Documents/semio/📜️script.ts:177-182`

Calls `resolveFrameworkOsPlaygroundPlugin(ensureFrameworkOsPlaygroundCatalog(), segments)` which:
- Loads the playground catalog from `./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json` (line 2726 in `🟦️.ts`)
- Returns `{ plugin: "lowpoly", rest: [] }`

### Step 2c: runFrameworkOsPlaygroundDev() — `/Users/ueli/Documents/semio/📜️script.ts:194-202`

**File:** `/Users/ueli/Documents/semio/📜️script.ts:194-202`

Invokes:
```
bun nx run @semio-tech/framework-os-dev:dev -- lowpoly
```

With environment variables set by `frameworkOsPlaygroundDevEnv()` (line 199):
- `SEMIO_PLUGIN: "lowpoly"`
- `SEMIO_RENDERER: "react"` (default, line 2760 in `🟦️.ts`)
- `S_OS_PORT: "6078"` (for react; 6178 for wgpu, from catalog line 2755)

## 3. NX Target Definition

**File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:16-23`

The nx target `@semio-tech/framework-os-dev:dev`:

```json
"dev": {
  "executor": "nx:run-commands",
  "options": {
    "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript",
    "command": "bun ./📜️script.ts dev",
    "forwardAllArgs": true
  }
}
```

Changes directory to the framework-os-dev package and runs `bun ./📜️script.ts dev lowpoly`.

## 4. Framework OS Dev Script Processing

**File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1827-1977`

Class `DevScript.run()` execution path for `lowpoly`:

### 4a. Initialization (line 1828-1833)
- Ensure Apple Developer Dir (macOS)
- Publish shard worker: `publishShardWorker()` — outputs to `./🔌️plugin-modules/🧵️shard/🟨️shard-worker.js`

### 4b. Plugin Resolution (line 1851-1859)
- Plugin: `lowpoly`
- Renderer: `react` (default from env, line 1853)
- Port: `6078` (from catalog entry, line 2755 in library `🟦️.ts`)
- Stream plugin builds: `true` (because renderer === "react", line 1858)

### 4c. Plugin Build Lease (line 1861-1898)
- Check if this is a shared dev server (multi-user hub scenario)
- If holder: will call `buildPluginsStreaming()` after Vite starts
- If follower: will skip building and wait for holder to complete

### 4d: Engine WASM Build (line 1896-1903)
- Call `buildEngineWasm(plugin, renderer)` — builds renderer-specific WASM
- For lowpoly with react: builds the react renderer's WASM assets

### 4e: Vite Dev Server Start (line 1939-1957)
- Call `runViteBunxDev()` which:
  - Starts Vite on port **6078** (S_OS_PORT default)
  - Sets environment:
    - `SEMIO_PLUGIN: "lowpoly"`
    - `SEMIO_RENDERER: "react"`
    - `VITE_SEMIO_RENDERER: "react"`
    - `VITE_SEMIO_PLUGIN: "lowpoly"` (resolved from catalog)
    - `VITE_SEMIO_APP_ID: "s.lowpoly.lowpoly@1/*#editor"` (if available)

### 4f: Plugin Build Streaming (line 1945-1948)
- If lease holder:
  - Call `buildPluginsStreaming()` — builds ~37-crate lowpoly plugin in background
  - Call `generatePluginRegistry()` — regenerates catalog
  - Call `watchPluginRebuilds()` — watches for source changes and re-builds

### 4g: Await Vite (line 1950)
- Wait for Vite to exit (keeps dev server alive)

## 5. Lowpoly Plugin App IDs

**File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🔣️.json:1-30`

Only one app ID is declared:

```json
{
  "manifest": {
    "apps": [
      {
        "id": "s.lowpoly.lowpoly@1/*#editor",
        "role": "editor",
        "dialect": { "artifactKind": "s.lowpoly.lowpoly", "standard": "1", "subset": "*" }
      }
    ]
  }
}
```

Comparison with procedural:3d:
- `dev:procedural:3d` is: `bun ./📜️script.ts dev procedural 3d`
- `dev:lowpoly` is: `bun ./📜️script.ts dev lowpoly` (no second arg)
- **Lowpoly does NOT take an app-id argument; it defaults to the single declared app ID**

## 6. Windows and Modes Declaration

### 6a: Modes

Two modes are declared in lowpoly:

#### Mode 1: `edit`
**File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs:15-24`

```rust
pub fn layout() -> WindowLayout {
    create_default_layout(&[model::LOWPOLY_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Model".into()]))
}
```

| Mode | Window ID | SurfaceKind | Size Share |
|------|-----------|-------------|-----------|
| `edit` | `lowpoly-main` | `World3d` | 100% |

#### Mode 2: `paint`
**File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🎨️paint/🦀️.rs:23-32`

```rust
pub fn layout() -> NamedLayout {
    create_named_layout(
        LOWPOLY_PLAY_LAYOUT_PAINT,
        "Paint",
        create_default_layout(&[model::LOWPOLY_PLAY_WINDOW_MAIN.into(), uv::LOWPOLY_PLAY_WINDOW_UV.into()], "row", Some(&[60.0, 40.0]), Some(&["Model".into(), "UV".into()])),
        "builtin",
        Some("paintbrush".into()),
        None,
    )
}
```

| Mode | Window ID | SurfaceKind | Size Share |
|------|-----------|-------------|-----------|
| `paint` | `lowpoly-main` | `World3d` | 60% |
| `paint` | `lowpoly-uv` | `Canvas2d` | 40% |

### 6b: Window Definitions

**Model Window:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️.rs:57`

```rust
surface_kind: SurfaceKind::World3d,
```

**UV Window:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️.rs:37`

```rust
surface_kind: SurfaceKind::Canvas2d,
```

## 7. Environment Variables

### Key Environment Variables Read

**File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`

| Env Var | Purpose | Line | Default/Source |
|---------|---------|------|-----------------|
| `SEMIO_PLUGIN` | Plugin filter for catalog/build | 1859 | Set to "lowpoly" by parent script |
| `SEMIO_RENDERER` | Renderer choice (react/wgpu) | 1853 | "react" (line 1853) |
| `S_OS_PORT` | Dev server port | 1861 | 6078 (catalog lookup) |
| `SKIP_PLUGIN_BUILD` | Skip building plugin (only render) | 1858 | Unset (enables streaming builds) |
| `DEVCONTAINER` | Check if running in devcontainer | 1910 | From process.env |
| `SEMIO_BUILD_MODE` | Build mode (dev/ship) | N/A in dev | Not set in dev (development mode) |

**File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1939-1957`

Additional environment variables passed to Vite:
- `VITE_SEMIO_RENDERER: "react"` — renderer choice for build
- `VITE_SEMIO_PLUGIN: "lowpoly"` — plugin id for build
- `VITE_SEMIO_APP_ID: "s.lowpoly.lowpoly@1/*#editor"` — app id if available

## 8. Prebuilt Plugin Modules and Staleness

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/💠️lowpoly/`

### File Mtimes

| File | Mtime | Age |
|------|-------|-----|
| `semio_s_plugin_lowpoly_component.core.wasm` | Aug 17 18:29 | **19 days old** |
| `semio_s_plugin_lowpoly_component.js` | Sep 4 17:14 | **1 day old** |
| `🔣️.json` | Sep 4 11:17 | **1 day old** |
| `🛂️.descriptor.semio` | Sep 5 03:26 | **Current** |

### Rust Source Mtimes (Sample)

**Source files (Sep 1-5, 2026):**
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/🦀️.rs` — Sep 5 01:06
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — Sep 3 02:41
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` — Sep 5 01:06
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` — Sep 4 11:13

### Staleness Assessment

**WASM Module is STALE:**
- `.core.wasm` built Aug 17 18:29
- Most source files updated Sep 3-5
- **Difference: 18+ days old**
- A `dev lowpoly` run will trigger `buildPluginsStreaming()` to rebuild it

**JavaScript/Descriptor modules are mostly current:**
- `.js` rebuilt Sep 4 17:14 (1 day ago)
- `.descriptor.semio` refreshed Sep 5 03:26 (current)
- Not stale but will be regenerated if plugin builds

## Summary of Boot Chain Sequence

1. User: `bun run dev:lowpoly`
2. Root script: `bun ./📜️script.ts dev lowpoly`
3. DevScript resolves "lowpoly" via catalog lookup
4. Root script: `nx run @semio-tech/framework-os-dev:dev -- lowpoly`
5. NX target changes dir and runs: `bun ./📜️script.ts dev lowpoly`
6. Framework OS DevScript:
   - Publishes shard worker
   - Resolves: plugin="lowpoly", renderer="react", port=6078
   - Starts Vite on port 6078
   - Triggers `buildPluginsStreaming()` (rebuilds stale WASM)
   - Vite serves React shell listening for plugin module loads
   - Watch for file changes and rebuild on modification

Total ports engaged: **6078 (Vite React)**
Plugin modules directory: `./🔌️plugin-modules/💠️lowpoly/`
App entry point: `s.lowpoly.lowpoly@1/*#editor`
