# OS `s` Frontend Shell — Audit Report

**Scope:** OS shell only (`🧰️framework/🛍️products/💻️os`, `✏️s` plugin list, renderer host). Not individual plugin internals.  
**Date:** 2026-09-23  
**Command logs:** `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/END-TO-END-OS-HUB-COLLABORATION-MCP/🗑️generated/os/`

---

## Executive Summary

The `s` OS frontend **can boot in the browser** (Vite + React) and serves HTML on a configurable port. All **34 top-level plugins** under `✏️s/🔌️plugins` are registered (plus 26 extensions → 60 total in the generated registry). However, **typecheck, registry check, and several test targets currently fail**, blocking a clean CI gate. First dev boot is **very slow** (~16 min cold, 132 Nx tasks even in `served` mode) because every plugin wasm must be built/materialized before Vite starts.

---

## 1. How to Run

### Primary launch config (recommended)

| Field | Value |
|-------|-------|
| **launch.json name** | `🛠️dev🪐️space⚛️react` |
| **Command** | `bun nx run workspace:dev -- s` |
| **Nx target chain** | `workspace:dev` → `bun ./📜️script.ts dev s` → `runFrameworkOsPlaygroundDev("s")` → `@semio-tech/framework-os-dev:dev` (resolved to `serve-s-react-dev` when `SEMIO_RENDERER=react`) |
| **Port** | `S_OS_PORT=6070` (launch.json); Vite default without override is `6066` |
| **Host** | `127.0.0.1` (browser) |
| **Renderer** | React via Vite (`SEMIO_RENDERER=react`) |

### Environment variables (launch.json)

```
S_OS_PORT=6070
SEMIO_PLUGIN=s
SEMIO_RENDERER=react
```

### Collaboration variants

| Config | Extra env |
|--------|-----------|
| `🛠️dev🪐️space👤️1⚛️react` / `👤️2` | `S_HUB_URL=http://127.0.0.1:8787`, `S_DATA_DIR=...` |
| `🧭️compound🖥️s⚛️react🗄️os-hub` | Starts `🛠️dev🗄️os-hub` + `🛠️dev🪐️space⚛️react` together |
| `🧭️compound🖥️s👥️users🗄️os-hub` | Hub + two user sessions |

### Hub server (separate process)

| Field | Value |
|-------|-------|
| **launch.json** | `🛠️dev🗄️os-hub` |
| **Command** | `bun nx run os-hub:dev` |
| **Port** | `OS_HUB_PORT=8787` |
| **Data** | `OS_HUB_DATA=${workspaceFolder}/.🧬semio/🌐hub/hub-dev/` |

### Other renderer modes

| Config | Host | Notes |
|--------|------|-------|
| `🛠️dev🪐️space🧊️wgpu🌐️wasm` | Browser (Trunk/wasm) | Port 6066 |
| `🛠️dev🪐️space🧊️wgpu🖥️native` | Native binary | Not browser |
| `🛠️dev🪐️space⚛️react📦️served` | Browser | Skips rebuild chain; serves pre-built `dist/` |

### Per-plugin dev (not full `s` shell)

Individual plugins use `🛠️dev📐️cad⚛️react` etc. — same `workspace:dev -- <plugin>` chain but `SEMIO_PLUGIN=<plugin>` filters to one plugin.

### What it serves

- **Browser SPA** at `http://127.0.0.1:<S_OS_PORT>/`
- Entry: Vite serves `🧰️framework/.../🧑‍💻dev/🌐️.html` with `/🟦️.ts` bootstrap
- **Not** Electron/Tauri for the primary dev path (native wgpu is a separate target)
- `dev s` loads **all 60 registered plugins** (space plugin is the OS host shell)

---

## 2. Architecture Map

```
📜️script.ts (root)
  DevScript.run(["s"]) ──► runFrameworkOsPlaygroundDev("s")
    └── nx run @semio-tech/framework-os-dev:dev
          └── 🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts
                ├── prepare: plugin-registry:generate + session-s
                ├── activate: 60× (component-dev + materialize-dev)
                └── serve: Vite (🏗️builder/🌐️vite/🟦️.ts)

🧰️framework/🛍️products/💻️os/
  🟦️.ts                          # OS kernel TS API (DirectoryClient `n`, bindings, frames)
  🖥️host/                         # Rust WASM host runtime
  🔨️modules/
    🧑‍💻dev/                       # Dev runner, Vite config, playground session
    📺️renderer/🧑‍🎨engine/        # React + wgpu render targets
      🧱️elements/🏛️ShellHost/    # Main shell UI, hub URL wiring
      🧱️elements/🖥️Board2dHost/   # 2D board host
      🧱️elements/🌐️World3dHost/   # 3D world host
    🔌️plugin/
      📇️registry/                 # Plugin discovery + generated catalog
        🤖️generated/🔌️plugins.json
        🤖️generated/🗿️artifacts/🦀️.rs
      🏗️build/                    # wasm build + materialization pipeline
      🏪️store/                     # Plugin installation + backbone worker
    🏪️store/👷️worker/🟦️.ts      # Hub sync client (DirectoryClient, WebSocket)
    📇️directory/                   # Hub directory protocol schemas

✏️s/
  🔌️plugins/<plugin>/
    📦️packages/🦀️rust/Cargo.toml  # [package.metadata.semio] — discovery source
    🗿️artifacts/<kind>/           # Per-artifact-kind implementations
      🏅️standards/🔖️1/🪆️subsets/✳️any/
        ✏️editor/ 👁️viewer/ 🧬️schema/ 🦀️.rs
```

### Plugin registration flow

1. **Discovery:** Scan `✏️s/🔌️plugins/**/📦️packages/🦀️rust/Cargo.toml` for `[package.metadata.semio]` (`role = "plugin"`, `package = "semio:<id>"`, `activationEvents`, `dependsOn`, etc.)
2. **Generation:** `bun nx run @semio-tech/plugin-registry:generate` → `🤖️generated/🔌️plugins.json`, `🗿️artifacts/🦀️.rs`, playground catalog, launch.json rows
3. **Build:** Per-plugin `component-dev` + `materialize-dev` (wasm → JCO transpile → browser modules)
4. **Activation:** `semioActivationVitePlugin` loads activated components into Vite module graph
5. **Host:** `space` plugin declares `[package.metadata.semio].host = { landing = "home", shell = "studio" }` — the OS host shell

### Registered vs folder plugins

| Category | Count | Status |
|----------|-------|--------|
| Top-level folders in `✏️s/🔌️plugins` | 34 | All registered |
| Registered `role=plugin` in `plugins.json` | 34 | Matches folders |
| Registered extensions (`role=extension`) | 26 | Under parent plugin trees |
| **Total registry entries** | **60** | No orphaned top-level folders |

### Artifacts

- **Location:** `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifactKind>/🏅️standards/<version>/🪆️subsets/<subset>/`
- **Surfaces:** Each subset has `✏️editor` (mutation) and `👁️viewer` (read-only) — addressed as `<kind>@<standard>/<subset>#<role>`
- **Schemas:** `🧬️schema/` (GraphQL, JSON, proto, TS, Rust) per surface
- **Loading:** Registry emits `on-artifact-kind:<kind>` activation events → plugin wasm activates → OS routes document open to the owning plugin's surface
- **Example:** `puzzle` has `🖐️5d`, `🧊️3d`, `◻️2d` artifact kinds with editor/viewer/config

---

## 3. Hub Connection Entry Points

| Layer | File | Mechanism |
|-------|------|-----------|
| **Env config** | `launch.json` | `S_HUB_URL=http://127.0.0.1:8787` |
| **Vite proxy** | `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` | `/_semio/hub/*` → `S_HUB_URL` (same-origin, forwards `Authorization`) |
| **Vite define** | same file | `VITE_S_HUB_URL`, `VITE_S_DATA_DIR` injected at build time |
| **Shell UI** | `📺️renderer/.../🏛️ShellHost/🟦️.tsx` | `readViteSEnv("VITE_S_HUB_URL")` → hub sign-in, directory scope |
| **Store worker** | `🏪️store/👷️worker/🟦️.ts` | `n` class (DirectoryClient), `openDirectory()`, WebSocket grants via `/_semio/hub` |
| **OS API** | `🟦️.ts` (framework-os) | `DocumentBinding { kind: "hub", n, spaceId }`, directory-open/bootstrap/scope commands |
| **Hub server** | `🌎️hub/` (out of scope) | `os-hub:dev` on port 8787 |

Without `S_HUB_URL`, the shell runs **local-only** (no hub proxy, no collaboration).

---

## 4. Verified Status (Commands Run)

> All commands used `NX_SOCKET_DIR=<ticket>/🗑️generated/os/.nx-sockets` to avoid sandbox socket issues.

### Typecheck

| Command | Result | Key error |
|---------|--------|-----------|
| `bun nx run @semio-tech/framework-os:typecheck` | **FAIL** | `Board2dHost/🧪️tests/👻️catalogue-drop/🟦️.tsx(13,101): error TS1002: Unterminated string literal` — truncated import line (concurrent edit) |
| `bun nx run @semio-tech/framework-renderer-react:typecheck` | **FAIL** | 100+ errors: missing `Bun` types, `import.meta.dir`, cross-repo test file inclusion, `bun:test`/`bun:sqlite` modules |

Log: `typecheck-framework-os-v2.log`, `typecheck-renderer-react-v2.log`

### Plugin registry

| Command | Result | Key error |
|---------|--------|-----------|
| `bun nx run @semio-tech/plugin-registry:check` | **FAIL** | `plugin registry catalog is stale: generated/🗿️artifacts/🦀️.rs` |

Log: `plugin-registry-check-v2.log`

### Tests

| Command | Result | Key error |
|---------|--------|-----------|
| `bun nx run @semio-tech/framework-os-dev:test -- quick` | **FAIL** (1/186) | `vite config module graph > stays inside declared module and source-byte bounds` — expected ≤40, got 58 |
| `bun nx run @semio-tech/framework-os-host-rs:test` | **FAIL** (dep) | `semio-framework-ui` wasm build: `cannot find module web_sys` |

Logs: `framework-os-dev-test-quick.log`, `os-host-rs-test.log`

### Dev server smoke test

| Command | Result |
|---------|--------|
| `S_OS_PORT=6099 SEMIO_PLUGIN=s SEMIO_RENDERER=react bun nx run workspace:dev -- s served` | **PASS** (after ~16 min cold build) |
| `curl http://127.0.0.1:6099/` | **HTTP 200**, 3540 bytes HTML |
| Vite banner | `VITE v7.3.6 ready` on `http://127.0.0.1:6099/` |
| Activation | `Activated s react dev: 60 completed components` |
| Runtime errors | None observed during startup |

Log: `dev-server-served.log`

**Note:** Even `served` mode still ran 132 Nx dependency tasks on cold boot (all plugin component-dev/materialize-dev). Subsequent boots should be faster with cache.

---

## 5. Prioritized Blockers

### P0 — Blocks typecheck / CI

1. **Truncated import in Board2dHost test** (concurrent edit in progress)
   - File: `🧰️framework/.../🖥️Board2dHost/🧪️tests/👻️catalogue-drop/🟦️.tsx:13`
   - Error: `TS1002: Unterminated string literal`
   - Fix: Complete the truncated `import { BoardSessionFactoryContext...` line

2. **Stale plugin registry artifacts**
   - File: `🔌️plugin/📇️registry/🤖️generated/🗿️artifacts/🦀️.rs`
   - Fix: `bun nx run @semio-tech/plugin-registry:generate` (then commit regenerated output)

### P1 — Blocks wasm host / renderer tests

3. **Missing `web_sys` in semio-framework-ui wasm build**
   - File: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3576`
   - Fix: Add `web_sys` to the wgpu target's `[target.'cfg(target_arch = "wasm32")'.dependencies]` or gate the `web_sys::window()` call behind `cfg`

4. **Renderer-react typecheck pulls entire monorepo**
   - Many errors from `📜️script.ts`, plugin test files, `import.meta.dir` (needs `moduleResolution: bundler` + `types: ["bun"]`)
   - Fix: Tighten `tsconfig` `include`/`exclude` for renderer package; add `@types/bun` to types array

### P2 — Quality / dev experience

5. **Vite config module graph budget exceeded**
   - File: `🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts` — bound is 40, actual is 58
   - Fix: Either refactor vite config imports or raise the budget with justification

6. **Cold dev boot takes ~16 minutes**
   - 132 Nx tasks (60 plugins × component + materialize) before Vite serves
   - Fix: Improve `served` mode to truly skip rebuild when dist is fresh; consider lazy plugin activation

7. **Hub collaboration untested in this audit**
   - Shell serves without hub; collab requires compound launch with `🛠️dev🗄️os-hub`
   - Fix: Run `@semio-tech/framework-os-dev:collab-e2e` once P0/P1 are resolved

### P3 — End-to-end gaps

8. **No `S_HUB_URL` in default `🛠️dev🪐️space⚛️react`**
   - Local-only by default; hub/collab is opt-in via compound configs
   - Documented in `ShellHost/🟦️.tsx` — intentional but blocks "working end to end" without compound launch

9. **Native host (Electron/Tauri) not part of primary dev path**
   - Browser-only for `dev s`; native wgpu is a separate launch row
   - Not a bug, but relevant for "desktop-first" goal

---

## 6. Quick Reference

```bash
# Full s shell (React, port 6070)
S_OS_PORT=6070 SEMIO_PLUGIN=s SEMIO_RENDERER=react bun nx run workspace:dev -- s

# Hub + s together (use launch.json compound, or manually):
bun nx run os-hub:dev          # port 8787
S_HUB_URL=http://127.0.0.1:8787 S_OS_PORT=6070 bun nx run workspace:dev -- s

# Regenerate plugin registry
bun nx run @semio-tech/plugin-registry:generate

# Typecheck OS
bun nx run @semio-tech/framework-os:typecheck

# Quick dev tests
bun nx run @semio-tech/framework-os-dev:test -- quick
```
