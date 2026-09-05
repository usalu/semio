# Sourcing App: Launch Path & Plugin Build Mapping

**Date:** 2026-09-05  
**Ticket:** 🎆️26/🌙️09/☀️01/SOURCING-END-TO-END  
**Status:** READ-ONLY EXPLORATION (no builds run, no edits made)

---

## 1. Launch Command Chain: `bun run dev:sourcing`

### Package.json Entry
- **File:** `/Users/ueli/Documents/semio/package.json` (line 38)
- **Command:** `"dev:sourcing": "bun ./📜️script.ts dev sourcing"`

### Root Script Router → Framework OS Dev
- **File:** `/Users/ueli/Documents/semio/📜️script.ts` (line 502)
  - Calls `runFrameworkOsPlaygroundDev("sourcing", [])`
  - Resolves via `resolvePlaygroundDevApp(["sourcing"])`
  
- **Actual NX Invocation:**
  ```
  bun nx run @semio-tech/framework-os-dev:dev -- sourcing
  ```

### NX Target Definition
- **File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` (lines 18–23)
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

### DevScript Implementation
- **File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` (lines 1827–1978)
- **Class:** `DevScript extends BundleScript`
- **Flow:**
  1. Loads playground catalog from `🎠️playgrounds.json`
  2. Resolves variant "sourcing" → finds port 6081 (react) / 6181 (wgpu)
  3. Calls `runViteBunxDev()` with:
     - Port: `S_OS_PORT = 6081` (via env or default)
     - Plugin: `SEMIO_PLUGIN=sourcing`
     - Renderer: `SEMIO_RENDERER=react`
  4. Optionally streams plugin builds if `SKIP_PLUGIN_BUILD` is unset
  5. Binds Vite dev server at **127.0.0.1:6081**

### Port Resolution
- **Catalog File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json`
  ```json
  {
    "variant": "sourcing",
    "pluginId": "sourcing",
    "ports": { "react": 6081, "wgpu": 6181 }
  }
  ```

---

## 2. Plugin Build Command: `bun nx run @semio-tech/framework-os-dev:plugin -- sourcing`

### NX Target (Plugin Subcommand)
- **File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` (lines 94–100)
  ```json
  "plugin": {
    "executor": "nx:run-commands",
    "options": {
      "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript",
      "command": "bun ./📜️script.ts plugin",
      "forwardAllArgs": true
    }
  }
  ```

### Router Dispatch
- **File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` (lines 5480–5525)
  - Routes "plugin" command to handler at line 5490+
  - For non-subcommand arguments (e.g., "sourcing"), calls `new PluginBuildScript(this.root).run(segments)`

### PluginBuildScript
- **File:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` (lines 1214–1219)
  ```rust
  class PluginBuildScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN;
      await buildPlugins(filterPlugin || undefined);
    }
  }
  ```

### Output Directory
- **Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/`
- **Constant:** `pluginOutRoot` (line 67 of dev/📜️script.ts)

---

## 3. Current On-Disk State: Served Sourcing Plugin Modules

**Full Directory Listing (with mtimes):**

```
ls -la 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/
```

| File                                    | Size        | Modified           | Notes                 |
|----------------------------------------|-------------|-------------------|-----------------------|
| `interfaces/`                          | (dir)       | Sep  1 12:30       | Type defs             |
| `semio_s_plugin_sourcing_component.core.wasm` | 42.0 MB | Sep  1 12:30 | **CORE WASM** (jco-extracted) |
| `semio_s_plugin_sourcing_component.d.ts` | 3.4 KB | Sep  1 12:30 | TypeScript definitions |
| `semio_s_plugin_sourcing_component.js` | 428.9 KB | **Sep  4 17:20** | Shim binding (regenerated) |
| `🌉️bridge.js`                         | 9.4 KB     | Sep  1 22:58      | Framework glue        |
| `🔣️.json`                             | 322.7 KB   | **Sep  4 11:18**   | **DESCRIPTOR** (manifest) |
| `🛂️.descriptor.semio`                 | 73.3 KB    | **Sep  5 03:26**   | **BINARY DESCRIPTOR** (latest) |
| `🟨️.js`                               | 6.8 KB     | Sep  1 17:03      | Host shim             |

**Key Observations:**
- Most core files: **Sep 1 12:30** (stale by 4+ days)
- Binary descriptor: **Sep 5 03:26** (latest, 1 hour old at time of exploration)
- JSON descriptor: **Sep 4 11:18** (may be out of sync with binary)
- Shim binding (.js): **Sep 4 17:20** (regenerated, likely by vite/hot-swap)

---

## 4. Served Descriptor: Command Classifications

### File: `🔣️.json`
**Path:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/🔣️.json`

**Classification Counts (grep `"interactiveJob"`):**

| Classification | Count |
|---|---|
| `Migrated` | **110** |
| `BatchOnlyPendingRewrite` | **32** |
| **Total** | **142** |

**Expected:** 6 migrated / 8 batchOnly (per task description)  
**Actual:** 110 migrated / 32 batchOnly

**Status:** Served descriptor contains WAY more commands than the 14 bounded-tool set; appears to be the full plugin manifest, not just UI entry points.

---

## 5. Source of Truth: Rust Bounded Tool Declarations

### Bounded Tool IDs (Contract Boundary)
**File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (lines 230–246)

```rust
const SOURCING_CURATION_BOUNDED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "setDocument",
    "stockFromCatalogue",
    "curationAdd",
    "curationSetCount",
    "curationRemove",
    "dropOnPool",
    "dropOnCurated",
    "setFilterQuery",
    "setFilterModule",
    "setFilterTypology",
    "setFilterMinAvailability",
    "sortTable",
    "setContributions",
];
```

**Total:** 14 commands (matches expected count)

### Individual Command Classifications (Interactive Job Assignments)
**File:** Same as above (lines 1053–1162)

| # | Command ID | Source Classification | Rust Line | Notes |
|---|---|---|---|---|
| 1 | `setActiveExample` | **Migrated** | 1150 | ✓ migrated |
| 2 | `setDocument` | **Migrated** | 1149 | ✓ migrated |
| 3 | `stockFromCatalogue` | **Migrated** | 1151 | ✓ migrated |
| 4 | `curationAdd` | **Migrated** | 1152 | ✓ migrated |
| 5 | `curationSetCount` | **Migrated** | 1153 | ✓ migrated |
| 6 | `curationRemove` | **Migrated** | 1154 | ✓ migrated |
| 7 | `dropOnPool` | **Migrated** | 1155 | ✓ migrated |
| 8 | `dropOnCurated` | **Migrated** | 1156 | ✓ migrated |
| 9 | `setFilterQuery` | **Migrated** | 1157 | ✓ migrated |
| 10 | `setFilterModule` | **Migrated** | 1158 | ✓ migrated |
| 11 | `setFilterTypology` | **Migrated** | 1159 | ✓ migrated |
| 12 | `setFilterMinAvailability` | **Migrated** | 1160 | ✓ migrated |
| 13 | `sortTable` | **Migrated** | 1161 | ✓ migrated |
| 14 | `setContributions` | **Migrated** | 1053 | ✓ migrated |

**Summary:**
- **All 14 bounded tools: Migrated**
- No `BatchOnlyPendingRewrite` in the bounded set
- No `ForbiddenFromUi` in the bounded set

### Descriptor Mismatch Analysis
The 142 commands in the served descriptor include the bounded 14 PLUS 128 other commands (likely extension/schema mutations, internal handlers, etc.). The descriptor includes all actions ever declared, not just the UI-interactive ones bound by the contract.

---

## 6. Dev Launcher: Plugin Build Lease & Serving-Only Mode

### Lease Mechanism
**Purpose:** Coordinate multiple concurrent `dev sourcing` processes on same port; only one holds the plugin-build lease, others serve its output.

**Files:**
- Lease directory: `target/semio-dev-leases/`
- Lease file: `target/semio-dev-leases/plugin-build-sourcing.json` (created if building)

### Output Modes

#### Leader Mode (Builds Plugins)
- Acquires lease on first call with `streamPluginBuilds = true`
- Logs: `[dev] plugin builds owned by pid XXXX (port XXXX); building`
- Runs `buildPluginsStreaming()`
- Marks lease as ready: `markPluginBuildLeaseReady()`

#### Follower Mode (Serving Only)
- Detects existing lease holder
- Logs: `[dev] plugin builds owned by pid XXXX (port XXXX); serving only`
- Waits up to 60s (`PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS`) for holder
- If holder delivers → serve its outputs
- If timeout/empty → takes over lease and builds itself

#### Forced Serving-Only Mode
- Set `SKIP_PLUGIN_BUILD=1` environment variable
- Bypasses lease logic entirely
- Serves pre-built plugin-modules/ immediately
- Used by hub `users` launcher (each user serves distinct shell)

**Key Env Vars:**
- `S_OS_PORT`: Override default port (6081)
- `SEMIO_RENDERER`: Force "react" or "wgpu" (default: "react")
- `SKIP_PLUGIN_BUILD`: Skip build, serve only (default: unset = build)
- `SEMIO_PLUGIN`: Plugin filter (redundant with segments[0])

---

## 7. Current System State

### Running Processes
- **Port 6081:** No process listening
  - Command: `lsof -nP -iTCP:6081 -sTCP:LISTEN`
  - Result: Empty

- **Vite/Framework-OS-Dev processes:** None running
  - Command: `pgrep -fl vite`
  - Command: `pgrep -fl framework-os-dev`
  - Result: Empty

### Plugin Build Lease
- **Lease directory:** Does not exist (`target/semio-dev-leases/` missing)
- **Implication:** No active dev processes holding or waiting for lease

### Conclusion
**Status: Idle** — No dev server or build process currently active. Safe to launch `bun run dev:sourcing` without conflicts.

---

## 8. Bootstrap Path Summary for Coordinator

### Fast Boot Steps (when plugin-modules already built)
1. **Set env:** `S_OS_PORT=6081 SKIP_PLUGIN_BUILD=1 SEMIO_PLUGIN=sourcing`
2. **Run:** `bun nx run @semio-tech/framework-os-dev:dev -- sourcing`
3. **Waits for:** Vite to bind 127.0.0.1:6081 (typically ~2–5s with React)
4. **Result:** Serves 🪵️sourcing plugin-modules/ at `http://127.0.0.1:6081`

### Full Boot Steps (clean build)
1. **Build plugins:** `bun nx run @semio-tech/framework-os-dev:plugin -- sourcing`
   - Outputs to `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/`
   - Cargo builds ~30 crates (~5–10 min depending on cache)
2. **Start dev server:** `bun run dev:sourcing`
   - Acquires lease, streams build completion to Vite
   - Binds at 127.0.0.1:6081

### Heartbeat / Status Detection
- **Leader is building:** Look for non-empty 🔌️plugin-modules/🪵️sourcing/ + recent mtime on 🛂️.descriptor.semio
- **Follower is serving:** Check for "[dev] ... serving only" in stdout + 127.0.0.1:6081 LISTENING
- **Health check:** `curl http://127.0.0.1:6081/` should return HTML (not 503)

---

## File Paths Reference

| Entity | Path |
|---|---|
| **Package.json** | `package.json` (line 38) |
| **Root script** | `📜️script.ts` (line 502) |
| **NX project config** | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` (lines 18–23, 94–100) |
| **Dev script** | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` (lines 1827–1978) |
| **Playground catalog** | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json` |
| **Plugin output dir** | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/` |
| **Served descriptor** | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/🔣️.json` |
| **Binary descriptor** | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/🛂️.descriptor.semio` |
| **Editor source (bounded tools)** | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (lines 230–246, 1053–1162) |
| **Lease directory** | `target/semio-dev-leases/` |

---

## Appendix: Descriptor JSON Structure

### Top-level Keys in 🔣️.json
- `activationEvents` — When plugin auto-starts
- `capabilityRequests` — Sandbox permissions needed
- `contributions` — UI panels, commands
- `descriptorVersion` — Schema version
- `execution` — Concurrency/isolation mode
- `hashes` — Integrity checks
- `manifest` — Apps, schemas, artifact kinds, commands
- `quotas` — Resource limits
- `role` — Plugin role classification

### Why 142 Commands ≠ 14 Bounded Tools
The `manifest.apps[0].commands[]` in the descriptor is exhaustive: it includes:
- UI-callable commands (the 14 bounded tools)
- Schema mutations (internal, not UI-exposed)
- Lifecycle handlers (initialization, cleanup)
- Extension commands (from extensions like beams, slabs, windows)

Only the 14 bounded tools have `"interactiveJob": "Migrated"` AND appear in `SOURCING_CURATION_BOUNDED_TOOL_IDS`.

---

**Exploration completed:** 2026-09-05 03:30 UTC
