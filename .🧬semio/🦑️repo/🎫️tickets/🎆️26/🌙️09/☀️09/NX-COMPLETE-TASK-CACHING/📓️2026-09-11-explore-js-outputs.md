# Exploration: JavaScript/TypeScript Output Directories and Cache Configuration

## Summary

The repository has significant output and cache directories scattered outside the nx unified cache:
- **2.8GB** storybook-static (root)
- **2.4GB** plugin-modules (development runtime outputs)
- **1.8GB** .vite-os-dev cache
- **414MB** Playwright browsers cache
- **117MB** .vite-mit-bestand-demonstrator cache
- **562MB** plugin dist/dev and dist/release outputs

Most JS/TS targets lack `cache: true` and `outputs` declarations in project.json, meaning their outputs are not captured by nx.

---

## 1. Output Directories from JS Tooling

### Root-Level Outputs (outside workspace packages)

| Directory | Size | Producer | Status | Notes |
|-----------|------|----------|--------|-------|
| `./storybook-static/` | 2.8GB | Storybook build | **UNDECLARED** | Built by root storybook build, served by `.storybook/main.ts`; no `outputs` in any target |
| `./test-results/` | 40K | Vitest/Playwright | **UNDECLARED** | Playwright test reports (trace on-first-retry) |

### Framework OS Dev Outputs

| Directory | Size | Producer | Declared | Target | Cache |
|-----------|------|----------|----------|--------|-------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/` | 2.4GB | jco transpile + plugin assembly | **NO** | `@semio-tech/framework-os-dev:plugin` | false |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/` | — | Vite build (multiple profiles) | **NO** | `@semio-tech/framework-os-dev:build` | false |

### Plugin Registry Outputs

| Directory | Size | Notes |
|-----------|------|-------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/` | Part of 562M | jco output for dev profile; also in nx cache |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules/` | Part of 562M | jco output for release profile |

### Framework Editor Outputs

| Directory | Notes |
|-----------|-------|
| `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/` | wasm-pack output (Rust → WASM) |

---

## 2. Vite Cache Directories (in `node_modules/`)

**Current locations (NOT routed to unified cache):**

| Path | Size | Used By | Configurable |
|------|------|---------|--------------|
| `node_modules/.vite-os-dev/` | 1.8GB | `@semio-tech/framework-os-dev` dev serves | **YES** — `⚙️vite.config.ts:67` |
| `node_modules/.vite-mit-bestand-demonstrator/` | 117MB | mit-bestand demonstrator dev | **YES** — per-project vite config |
| `node_modules/.vite-temp/` | 4.2M | Temporary vite optimization | **YES** — if configured |
| `node_modules/.vite/` | 28K | Default Vite cache | **YES** — if configured |

**Config File Locations:**

| File | Line | Config |
|------|------|--------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts` | 67, 96 | `playgroundCacheDir = path.join(repoRoot, "node_modules/.vite-os-dev", \`${plugin}-${renderer}\`)` → `cacheDir: playgroundCacheDir` |
| `.storybook/main.ts` | — | No `cacheDir` set (uses Vite default) |
| Root `vitest.config.ts` | — | Empty root config (no cache settings) |

---

## 3. Playwright Browsers Cache

**Current location:** `node_modules/.cache/ms-playwright/` (414MB)

**Configured in scripts:**
- `📜️script.ts:439` — bootstrap command (line 439)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:3088` (line 3088)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:4251` (line 4251)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:5145` (line 5145)
- `.devcontainer/devcontainer.json:107, 127`

**Pattern:** All scripts use `process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(repoRoot, "node_modules/.cache/ms-playwright")`

**No env var BUN_INSTALL_CACHE_DIR configured anywhere.**

---

## 4. Project Configuration Analysis

### Package.json Scripts
✅ **Correct pattern:** All scripts in `./package.json` call `bun nx …` through the bootstrap script:
```json
"nx": "bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx"
```

Example:
```json
"dev:storybook": "bun nx run workspace:dev-storybook"
```

### Project.json Targets
⚠️ **Majority pattern:** Most targets lack both `cache: true` and `outputs` declaration.

**Example from `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`:**

| Target | Cache | Outputs | Issue |
|--------|-------|---------|-------|
| `dev` | false | — | Non-cacheable (continuous:true), correct |
| `build` | **NOT SET** (defaults false) | **NO** | **BUILD OUTPUTS UNDECLARED** |
| `test` | **NOT SET** | **NO** | Test results not captured |
| `test-quick` | **NOT SET** | **NO** | — |
| `canonical-bootstrap-folder-mirror-check` | true | **NO** | Cache enabled but outputs missing |
| `catalog-smoke` | true | **YES** (empty) | Correctly disabled outputs |

**Pattern:** Cached targets (`cache: true`) exist but rarely declare `outputs`. Non-cached targets (majority) produce files that leave the workspace and are not reused across CI runs.

---

## 5. Bun and Environment Cache Configuration

### Bun Install Cache
**bunfig.toml** (only setting):
```toml
[install]
linker = "hoisted"
```
✅ Uses default Bun cache location (`~/.bun/install/cache`) — not routed to nx shared cache.

### Playwright Browsers Path
⚠️ **Not routed to nx cache:**
- Current: `node_modules/.cache/ms-playwright/`
- Desired: `.🧬semio/🦑️repo/⚡️cache/ms-playwright/` (or similar under unified cache root)

### No Vitest Cache Configuration
- Root `vitest.config.ts` is minimal (no coverage, cache, or output settings)
- Each package keeps its own `vitest.config.ts` with no explicit cacheDir

---

## 6. Nx Cache Configuration

**File:** `./nx.json` (lines 63–64)

```json
"cacheDirectory": ".nx/cache",
"maxCacheSize": "8GB"
```

✅ Nx uses `.nx/cache/` as the unified cache root.
✅ Max size: 8GB.

**Current cache contents:**
- Hash-based subdirectories (numeric hashes) containing symlinked or cached dist/ directories
- Many stale entries from intermediate build variants

---

## Gaps for Single Shared Cache

### 1. **Storybook Output Not Declared**
- **What:** `storybook-static/` (2.8GB)
- **Where:** Built by root-level storybook build (no specific package owns it)
- **Fix:** Declare `outputs: ["storybook-static/"]` in workspace root `📋️project.json` on the `build-storybook` target (if it exists) or create one; enable `cache: true` once validated

### 2. **Plugin-Modules Directory Not Cached**
- **What:** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/` (2.4GB)
- **Produced by:** `@semio-tech/framework-os-dev:plugin` target (calls `📜️script.ts plugin`)
- **Status:** `cache: false` (does not appear in project.json, defaults to false)
- **Fix:** Add `cache: true` and declare `outputs: ["🔌️plugin-modules/"]` in the `plugin` target of `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`

### 3. **Vite Caches in node_modules/**
- **What:** `.vite-os-dev/` (1.8GB), `.vite-mit-bestand-demonstrator/` (117MB), `.vite-temp/`, `.vite/`
- **Status:** Caches live in `node_modules/`, not under `.🧬semio/🦑️repo/⚡️cache/`
- **Fix:** Route all vite caches to `.🧬semio/🦑️repo/⚡️cache/vite/` by:
  - Updating `⚙️vite.config.ts:67` to use `.🧬semio/🦑️repo/⚡️cache/vite/{plugin}-{renderer}` instead of `node_modules/.vite-os-dev/{plugin}-{renderer}`
  - Adding environment variable or config setting for `.storybook/main.ts` to route storybook vite cache
  - Scanning other `vite.config.*` files in temp/ and package dirs for similar hardcoded paths

### 4. **Playwright Browsers Cache Not Routed**
- **What:** `node_modules/.cache/ms-playwright/` (414MB)
- **Status:** Hardcoded in multiple `📜️script.ts` files
- **Fix:** Route to `.🧬semio/🦑️repo/⚡️cache/ms-playwright/` by updating all `PLAYWRIGHT_BROWSERS_PATH` assignments in:
  - `📜️script.ts:439`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:3088, 4251, 5145`
  - `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts:934`
  - `.devcontainer/devcontainer.json:107, 127`

### 5. **Undeclared Build Outputs on Major Targets**
- **Targets:** `@semio-tech/framework-os-dev:build`, `test`, `test-quick`, `test-long`, `test-exhaustive`
- **Status:** No `outputs:` field, outputs live in workspace but are not cached or validated by nx
- **Fix:** Audit each target to determine its true output directory, then add to project.json:
  ```json
  {
    "executor": "nx:run-commands",
    "cache": true,
    "outputs": ["{projectRoot}/dist/**/*"],
    "options": { … }
  }
  ```

### 6. **Root Vitest and Storybook Configs**
- **Root `vitest.config.ts`:** Intentionally empty (all tests run per-package)
- **`.storybook/main.ts`:** No cacheDir set (relies on Vite default, which is `.storybook-static/.storybook`)
- **Fix:** Explicitly set cacheDir in `.storybook/main.ts` viteFinal hook:
  ```typescript
  config.cacheDir = path.join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/storybook-vite");
  ```

### 7. **Distributed Build Caches (Temporary)**
- **What:** Multiple temp dist directories in cache entries for each variant (puzzle, forms, raster, draw, etc.)
- **Status:** Nx stores these in `.nx/cache/` by hash, but they are large and per-variant
- **Fix:** No action required (nx handles this), but ensure targets declare outputs so nx knows what to cache

---

## Config File Locations for Update

### Must Update (Hardcoded Paths)
1. `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts` (line 67)
   - Change: `path.join(repoRoot, "node_modules/.vite-os-dev", ...)` → `.🧬semio/🦑️repo/⚡️cache/vite/...`

2. `📜️script.ts` (multiple lines: 439, 3088, 4251, 5145, etc.)
   - Change: `"node_modules/.cache/ms-playwright"` → `.🧬semio/🦑️repo/⚡️cache/ms-playwright`

3. `.devcontainer/devcontainer.json` (lines 107, 127)
   - Change: `"/workspaces/semio/node_modules/.cache/ms-playwright"` → `/workspaces/semio/.🧬semio/🦑️repo/⚡️cache/ms-playwright`

### Should Update (Missing Configs)
1. `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
   - Add `cache: true` and `outputs: ["🔌️plugin-modules/"]` to `plugin` target
   - Add `outputs` to `build` and test targets

2. `.storybook/main.ts`
   - Add `cacheDir` to `viteFinal` hook configuration

---

## Next Steps (Outside This Exploration)

1. **Centralize vite caches:** `.vite-os-dev/`, `.vite-mit-bestand-demonstrator/` → `.🧬semio/🦑️repo/⚡️cache/vite/`
2. **Centralize playwright cache:** `node_modules/.cache/ms-playwright/` → `.🧬semio/🦑️repo/⚡️cache/ms-playwright/`
3. **Declare missing outputs:** Add `outputs` and `cache: true` to major build targets
4. **Validate storybook-static path:** Determine which target builds it and declare its output
5. **Test with .gitignore:** Ensure all cache paths are ignored in git but available for CI runners
6. **Measure gains:** Before and after total size of cached vs. uncached outputs

---

**Report generated:** 2026-09-11  
**Explored areas:** vite configs (4 files), project.json targets (3 major), script.ts PLAYWRIGHT_BROWSERS_PATH (5 locations), bun/nx configs, storybook cache, playwright cache
