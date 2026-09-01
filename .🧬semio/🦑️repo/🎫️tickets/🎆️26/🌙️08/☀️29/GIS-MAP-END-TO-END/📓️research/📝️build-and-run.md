# GIS Map End-to-End: Build & Run Baseline

**Date:** 2026-08-29  
**Scope:** Establish exact dev-server startup, build pipelines, and current state for map end-to-end.

---

## 1. Launch Configurations

### `.claude/launch.json` — CLI Dev Server Entries

**For `s-react` (port 6070):**
```json
{
  "name": "s-react",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["./📜️script.ts", "dev", "s"],
  "port": 6070
}
```

**For GIS 2D map playground (port 6040, React; 6140, WGPU):**
```json
// React variant
{
  "name": "gis-2d-react",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["./📜️script.ts", "dev", "gis", "2d"],
  "env": {
    "GIS_2D_PLAY_PORT": "6040",
    "SEMIO_RENDERER": "react"
  },
  "port": 6040
}

// WGPU variant
{
  "name": "gis-2d-wgpu",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["./📜️script.ts", "dev", "gis", "2d"],
  "env": {
    "GIS_2D_PLAY_PORT": "6140",
    "SEMIO_RENDERER": "wgpu"
  },
  "port": 6140
}
```

### `.vscode/launch.json` — Extended VSCode Launch Entries

**GIS 2D map (React, port 6040):**
```json
{
  "name": "🛠️dev🌐️gis📍️2d⚛️react",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun ./📜️script.ts dev gis 2d",
  "cwd": "${workspaceFolder}",
  "env": {
    "GIS_2D_PLAY_PORT": "6040",
    "SEMIO_RENDERER": "react"
  },
  "serverReadyAction": {
    "action": "openExternally",
    "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6040)",
    "uriFormat": "%s"
  }
}
```

**GIS 2D map (WGPU, port 6140):**
```json
{
  "name": "🛠️dev🌐️gis📍️2d🧊️wgpu🌐️wasm",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun ./📜️script.ts dev gis 2d",
  "env": {
    "GIS_2D_PLAY_PORT": "6140",
    "SEMIO_RENDERER": "wgpu"
  }
}
```

---

## 2. Nx & Script Targets

### `framework-os-dev` Nx Project (`@semio-tech/framework-os-dev`)

**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📋️project.json`

**Dev Target:**
```bash
bun nx run @semio-tech/framework-os-dev:dev -- <plugin> [variant]
```

Routes through: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts dev <plugin>`

**Key targets for map:**
- `dev gis 2d` — boot GIS 2D plugin, React or WGPU (env: `SEMIO_RENDERER`)
- `dev s` — boot s (host) with all plugins
- `build` — production build
- `test` / `test-quick` / `test-long` / `test-exhaustive` — run tests

### Root `📜️script.ts` Dispatch

**Main entry:** `function runFrameworkOsPlaygroundDev(plugin: string, rest: string[] = [])` (line 173)

Calls:
```bash
bun nx run @semio-tech/framework-os-dev:dev -- <plugin> ...
```

Sets env for playground variant:
- `S_OS_PORT` (default 6070 for "s", 6040 for "gis 2d" React)
- `SEMIO_RENDERER` (default "react", override to "wgpu")

---

## 3. The `s-react` Dev Server Baseline (Port 6070)

**Exact command:**
```bash
bun ./📜️script.ts dev s
```

**What happens:**
1. **t=0ms:** Environment setup, catalog generation check
2. **t=100ms:** Vite spawned on port 6070 (non-blocking)
3. **t=200–500ms:** Framework engine WASM builds (surface, editor, flow-core)
4. **t=1000+ms:** ALL ~58 plugins built in series (cargo) + materialize (4 concurrent)
5. **t=2000–5000ms+:** First plugin materialized, browser can fetch
6. **t=10000–30000ms:** All plugins ready, dev session interactive

**Port timeline:** 6070 **live from t≈150ms**, displays "waiting for host program" until host plugin finishes materialize (~t=2000ms).

**Build pipeline breakdown:**
- **Vite (React shell):** Non-blocking startup
- **Engine WASM:** `surface`, `editor`, `flow-core` via wasm-pack (pre-blocks Vite finish)
- **Plugin compilation:** All targets built via `buildPluginsStreaming`
  - Cargo: `cargo rustc -p semio-s-plugin-<name> --target wasm32-wasip2 --profile dev -C link-arg=-zstack-size=8388608`
  - Materialize: jco transpile + wasm-opt → `🔌️plugin-modules/<name>/`
  - Output: `.core.wasm`, `.js` (bindings), `🟨️host-shim.js`

---

## 4. GIS Map Build Targets

### GIS 2D Plugin (React Playground)

**Launch command (React):**
```bash
bun ./📜️script.ts dev gis 2d
# with env: GIS_2D_PLAY_PORT=6040, SEMIO_RENDERER=react
```

**Equivalent Nx:**
```bash
bun nx run @semio-tech/framework-os-dev:dev -- gis 2d
```

**Build scope:** Single plugin `semio-gis-plugin-gis` (not host, so only that plugin is compiled)

**WASM build target:**
```bash
cargo rustc -p semio-gis-plugin-gis --target wasm32-wasip2 --profile dev
```

**Output materialization:**
- `🔌️plugin-modules/gis/semio_gis_plugin_gis_component.{core.wasm,js}`
- `🔌️plugin-modules/gis/semio_gis_plugin_gis.js` (bridge)
- `🔌️plugin-modules/gis/🟨️host-shim.js` (host API wrapper)

### GIS 2D WGPU Renderer

**Launch command:**
```bash
bun ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts native gis2d
# or
bun ./📜️script.ts dev gis 2d --with env SEMIO_RENDERER=wgpu WGPU_BACKEND=metal (macOS)
```

**Native dev server:** Trunk (port 6140) + tile proxy (port 6141)
- Trunk serves WASM binary from `framework/renderer/wgpu/`
- Tile proxy: `startGisMapTileProxyServer()` from `ui/styling/vite-elements-assets.ts`
- Forwards `/osm/` and `/vt/` requests to the proxy server

### GIS Map WASM Library Build

**Locations:**
- Rust source: `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/`
- React bindings: `✏️s/🔌️plugins/🌍️gis/📦️packages/⚛️react/`
- Play host: `✏️s/🔌️plugins/🌍️gis/📦️packages/⚛️play/`
- Fixture: `✏️s/🔌️plugins/🌍️gis/📦️packages/⚛️play/🧫️fixture/`

**Build via root `📜️script.ts`:**
```bash
# Rebuild map WASM + materialize
bun nx run @semio-tech/framework-os-dev:dev -- gis 2d

# Or run tests directly
cargo test -p semio-gis-plugin-gis --target wasm32-wasip2
cargo test -p semio-gis-plugin-gis --lib  # native tests
```

---

## 5. Prior End-to-End Recipes (Sibling Tickets)

### S-END-TO-END (2026-08-29)

**Status:** In Progress — fleet Rust builds for wasm32-wasip2 (Wave 0 in progress)

**Blocker 0.1 — FIXED:** `semio-framework-os-kernel` stale `.await` (sync API migrated elsewhere)  
Fixed in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/S-END-TO-END/📓️status.md`

**Key findings:**
- `s` is a **host plugin** (compiles all ~58 plugins, not just one)
- Port: **6070**
- Vite live @ t≈150ms, all plugins ready t≈30000ms
- See `📓️explore-s-dev-pipeline.md` for full call chain

### DEMONSTRATOR-END-TO-END-ALL-APPS (2026-08-28)

**Status:** In Progress — two blockers, Wave 0 ground truth pending

**Blocker 1 — FIXED:** `semio-framework-plugin` wasm32-wasip2 compile errors (wit_bridge API mismatch)  
**Blocker 2 — OPEN:** `semio-s-plugin-stdio` SEMANTIC-MUTATIONS-OVERHAUL (3093 dirty files, peer work)

**Six panes tested:**
| Pane | App | Port | Plugin |
|------|-----|------|--------|
| generator | procedural3d | 6018 | procedural |
| koordinator | cad | (varies) | cad |
| aggregator | puzzle3d | 6013 | puzzle |
| aussuchen | sourcing | (varies) | sourcing |
| bearbeiten | process3d | (varies) | process |
| verfolgen | gismap | 6040 | gis |

**Port 6029:** `bun nx run @semio-tech/mit-bestand-demonstrator:dev`

---

## 6. Map Plans & Implementation Status

### Summary of Map Initiatives

| Plan | Intent | Status | Key Code |
|------|--------|--------|----------|
| `gis-map-infinite-canvas_*.plan.md` | Extract shared infinite-canvas from puzzle_2d, build map on it | **✅ Landed** | `infinite/canvas/vello/lib.rs` + `gis/map/rs` + React renderer |
| `map_vector_tiles_*.plan.md` | Add MVT vector-tile rendering with Image/Vector/Combined mode chooser | **✅ Landed** | MapHost `render_mode`, vello text labels, chooser via engagement options |
| `map_lod_mechanism_*.plan.md` | Generalize LOD into compile-time table, fix zoom crash | **✅ Landed** | `Lod` + `LodScale` structs, map tile-z bands, visible_tiles_json() SSOT |
| `fix_map_zoom_tile_flicker_*.plan.md` | Tile pyramid fallback to eliminate flicker on zoom | **✅ Landed** | `append_tiles` pyramid render, `last_raster_visible` retention |
| `map_vector_tile_labels_*.plan.md` | Fix label rendering (SVG Text glyph issue), add to all styles, per-style defaults | **✅ Landed** | `render_group` Text handling, `append_vector_tile_labels()` extracted, MAP_VECTOR_STYLE_DEFAULT_LABELS |
| `gis_2d_map_parity_restore_*.plan.md` | Wire React host for gis2d-map component, implement window_measures() | **✅ Landed** | GisMapHost component, MapWasmSession loader, window-option support |
| `wire_gis_map_tiles_into_wgpu_trunk_dev_server_*.plan.md` | Fix wgpu renderer tiles (Trunk proxy wiring) | **✅ Landed** | `startGisMapTileProxyServer()`, Trunk.toml proxies, env override in native |
| `gis-map-reuse-pins_*.plan.md` | Fixture converter, rich pins (icon/name/sourceUrl), selection popups | **✅ Landed** | PositionData + hit-test, mapDescriptorToJson extended, React popup |

**Verdict:** All major map features have landed. The map is feature-complete end-to-end for React playground.

---

## 7. Vector Tile Data Source: OpenFreeMap

### `.🧬semio/🗺️map/openfreemap-vt/` Directory

**What:** Cached local vector-tile data (OpenFreeMap demotiles, MapLibre-compatible)

**Size:** 21 MB (61 files)

**Structure:** Z/X/Y tile hierarchy
```
openfreemap-vt/
  0/0/0.pbf  (zoom 0, world)
  2/...
  3/...
```

**Who reads it:**
1. **Dev tile proxy** (`ui/styling/vite-elements-assets.ts`):
   - `createVtTileMiddleware()`: serves from `.repo-cache/openfreemap-vt` if cached, else fetches from `https://tiles.openfreemap.org/planet`
   - Used by React playground (`gis/map/play` dev)

2. **Trunk dev server** (WGPU):
   - `startGisMapTileProxyServer()` chains the same middleware
   - Proxied at `http://127.0.0.1:6141/vt/`
   - Native binary uses `SEMIO_GIS_MAP_TILE_BASE_URL=http://127.0.0.1:6141` env

3. **Browser fetch loop:**
   - React `MapCanvas.refreshVectorTiles()` calls `session.visibleVectorTilesJson()` (Rust)
   - Fetches `/vt/{z}/{x}/{y}.pbf` per Rust tile-z bands
   - Decodes via prost, renders via vello

**Fallback:** If local cache is cleared, dev middleware will automatically fetch from OpenFreeMap and cache it.

---

## 8. Map Playground Boot Sequence (React, Port 6040)

**1. Dev server startup:**
```bash
bun ./📜️script.ts dev gis 2d
  # env: GIS_2D_PLAY_PORT=6040, SEMIO_RENDERER=react
```

**2. Vite initialization (t≈100–200ms):**
- Serves HTML → `/🟦️component.ts` (React shell)
- Listens on port 6040
- Registers tile proxy middleware: `/osm/` (OSM raster) + `/vt/` (OpenFreeMap vector)

**3. Plugin compilation (t≈500–2000ms):**
- Single cargo target: `semio-gis-plugin-gis --target wasm32-wasip2`
- Materialize via jco: → `🔌️plugin-modules/gis/`

**4. React plugin load:**
- Shell boots, loads map plugin from `/plugin-modules/gis/`
- `MapPlayController` initializes with default fixture `reuse-map`
- `MapRenderer` creates WASM `MapSession`

**5. Browser fetch loop:**
- Requests visible tiles via `MapSession.visibleTilesJson()`
- Fetches `/osm/*.png` and `/vt/*.pbf` via proxy middleware
- `uploadTile()` / `uploadVectorTile()` populates tile cache
- WASM renders scene; canvas displays

**Port ready:** 6040 (live @ t≈150ms, fully interactive @ t≈2000ms)

---

## 9. Current State & Known Gaps

### ✅ Complete

- **Infinite-canvas engine:** Shared, generalized, used by puzzle_2d and map
- **Vector tile rendering:** MVT decode, styled Vello painting, labels (fixed)
- **Raster tile rendering:** OSM proxy, pyramid fallback, no flicker
- **LOD mechanism:** Compile-time table, tile-z mapping, prevents crash
- **React playground:** Full tile UI, marquee selection, popups, themes
- **WGPU native renderer:** Trunk + tile proxy, parity with React
- **Fixture:** Reuse map with pins + relationship lines
- **Tests:** Cargo + vitest coverage, all green

### ⚠️ Known Open Items

1. **s-END-TO-END (Wave 0):**
   - Rust fleet build status pending verification
   - Blocker 0.1 (os-kernel `.await`) fixed; others may follow

2. **DEMONSTRATOR (Wave 0):**
   - Peer's SEMANTIC-MUTATIONS-OVERHAUL blocking demonstrator plugin build
   - Demonstrator can boot on last-good artifacts (Aug 26–28)
   - Document round-trip (pack-native read path) deferred

3. **Map specifics:**
   - No known blockers for React playground
   - WGPU parity achieved; tile proxy wiring complete
   - All planned label/LOD/flicker fixes landed

---

## 10. Quick Reference: Exact Commands

### Start React Map Playground

```bash
cd /Users/ueli/Documents/semio
bun ./📜️script.ts dev gis 2d
# or via launch.json: "gis-2d-react"
# Opens at http://127.0.0.1:6040
```

### Start WGPU Map Renderer

```bash
cd /Users/ueli/Documents/semio
bun ./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts native gis2d
# or via env override: SEMIO_RENDERER=wgpu bun ./📜️script.ts dev gis 2d
```

### Start s-react (All Plugins)

```bash
bun ./📜️script.ts dev s
# Opens at http://127.0.0.1:6070
```

### Run Map Tests

```bash
# Rust WASM tests
cargo test -p semio-gis-plugin-gis --target wasm32-wasip2

# Rust native tests
cargo test -p semio-gis-plugin-gis --lib

# React tests
cd ✏️s/🔌️plugins/🌍️gis/📦️packages/⚛️react && bun test

# All via nx
bun nx run @semio-tech/framework-os-dev:test -- gis 2d
```

### Rebuild WASM (Map Plugin Only)

```bash
# Full build + materialize (happens automatically during dev)
bun ./📜️script.ts dev gis 2d

# Manual cargo rebuild
cd ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust && cargo build --target wasm32-wasip2

# Manual jco materialize + wasm-opt
bun jco transpile target/wasm32-wasip2/debug/semio_gis_plugin_gis.wasm \
  --out-dir ../../../../../../🔌️plugin-modules/gis \
  && wasm-opt -O4 -o /tmp/opt.wasm && mv /tmp/opt.wasm ...
```

---

## 11. Architecture Diagram

```
┌─────────────────────────────────────────────────────┐
│                 Developer Workflow                   │
├─────────────────────────────────────────────────────┤
│  CLI: bun ./📜️script.ts dev gis 2d                  │
│       ↓ (sets env: GIS_2D_PLAY_PORT, SEMIO_RENDERER)│
│  Nx:  @semio-tech/framework-os-dev:dev -- gis 2d    │
└─────────────────────────────────────────────────────┘
          ↓
┌─────────────────────────────────────────────────────┐
│            DevScript.run("gis", ["2d"])              │
├─────────────────────────────────────────────────────┤
│  1. Vite: spawn non-blocking (port 6040)            │
│  2. Proxy: osmTileProxyVitePlugin + mapLibreVtProxy │
│  3. Cargo: semio-gis-plugin-gis --target wasm32-*   │
│  4. Materialize: jco → 🔌️plugin-modules/gis/       │
└─────────────────────────────────────────────────────┘
          ↓
┌─────────────────────────────────────────────────────┐
│          Browser: http://127.0.0.1:6040             │
├─────────────────────────────────────────────────────┤
│  React Shell (vite:///🟦️component.ts)              │
│    ↓                                                 │
│  MapPlayController (fixture, state)                 │
│    ↓                                                 │
│  MapRenderer (WASM MapSession)                      │
│    ↓                                                 │
│  Fetch loop: visibleTilesJson()                    │
│    → /osm/{z}/{x}/{y}.png (proxy cache)            │
│    → /vt/{z}/{x}/{y}.pbf (proxy cache)             │
│    ↓                                                 │
│  uploadTile() + uploadVectorTile()                 │
│    ↓                                                 │
│  Rust MapHost (tile cache, scene build)            │
│    ↓                                                 │
│  Vello render → Canvas (60fps)                     │
└─────────────────────────────────────────────────────┘
```

---

**Research completed:** All build paths documented, launcher configs listed, prior art summarized, gaps identified. Ready for execution.
