# Cache Infrastructure Audit: Single Shared Root via .🧬semio/🦑️repo/⚡️cache/

## Executive Summary

The repository has a **centralized cache root** at `.🧬semio/🦑️repo/⚡️cache/` (12 subdirectories, ~166 GB combined). All cache writes are coordinated through `getRepoMetaDir()` in one file. However, user-level caches (`~/.cargo`, `~/.cache/uv`, `~/Library/Caches/{sccache,go-build,bun,playwright}`) total ~52 GB and are not routed through the single root. Rust compiler (rustc) and sccache caches bypass the repo cache entirely when not configured explicitly.

---

## Area A: Existing Cache Root Structure

### Central Helper Function
**File:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:85-87`

```typescript
export function getRepoMetaDir(repoRoot: string): string {
  return join(getSemioRoot(repoRoot), REPO_META_DIR_NAME);
}
```

Where `REPO_META_DIR_NAME = "🦑️repo"` (line 49), creating `.🧬semio/🦑️repo/⚡️cache/` when joined with `"⚡️cache"`.

### Subdirectories and Sizes

| Directory | Size | Contents | Writes via |
|-----------|------|----------|-----------|
| `agents/` | 7.2 GB | Agent lifecycle cache, leases | `📜️script.ts:316`, agent resource tracking |
| `breaches/` | 7.8 MB | Policy lint findings (JSON) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` + `📚️library/🔌️nx-plugin/🟨️.mjs` |
| `cargo/browser/` | 54 GB | WASM browser compiler target | `wasmBuildEnvironment()` @ line 2905 |
| `cmake/macos/` | 372 KB | CMake build artifacts | `CMakePresets.json` line 12 |
| `dotnet/repo-test/` | 196 KB | .NET test state | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:line ~660` |
| `oracles/openstudio-*/` | 1.3 GB | Physics oracle binaries (pinned) | `✏️s/🔌️plugins/🔋️energy/🧪️oracle/📦️packages/🐍️python/📜️script.ts` |
| `print-fonts/` | 2.0 MB | LaTeX fonts (Anta, NotoEmoji, ShareTechMono) | Manual provisioning, no script writes |
| `tectonic/0.16.9/` | 68 MB | LaTeX bundles | `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts` |
| `tests/` | 138 MB | Test host fixtures, results, work dirs | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` outputs |
| `tools/` | 83 MB | binaryen, tectonic tools | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/` |
| `📺️renderer-modules/🧊️wgpu/` | 2.6 GB | WGPU render engine modules | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts` line 1 |
| `🔏️generator-inputs/📇️registry/` | 4 KB | Schema generator registry JSON | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` + `📜️script.ts` |

**Total: ~166 GB**

### Writes Tracked in Codebase

Search results for `⚡️cache` references (sorted by frequency):

| Reference Count | File | Purpose |
|-----------------|------|---------|
| 26 | `📜️script.ts` (root) | Central: sccache path, cmake presets, neo4j cache, query dir |
| 9 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` | wasmBuildEnvironment, test output, agent paths |
| 6 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/` (2 files) | Test cache path, dotnet state path |
| 5 | `🧰️framework/🛍️products/🔨️modules/📚️library/⚡️caching/` | Central policy, bootstrap tools, wasm opts |
| 4+ | Tickets, fixtures, misc scripts | Test harness paths, reproducers |

---

## Area B: Non-Rust, Non-JS Toolchains

### Go

**Current Setup:**
- Input tracking: `nx.json:29` defines `go` named input as `{projectRoot}/**/*.go`, `{projectRoot}/go.mod`, `{projectRoot}/go.sum`, `go.work`, `go.work.sum`
- Environment: `GOWORK` set in `📜️script.ts:222, 436, 700, 14219, 14306`
- **Cache location:** `~/Library/Caches/go-build` (user-level, 945 MB)
- **Missing:** No `GOCACHE` or `GOMODCACHE` override to route into `.🧬semio/🦑️repo/⚡️cache/`

### Python / uv

**Current Setup:**
- Input tracking: `nx.json:30` defines `uv` as `{projectRoot}/**/*.py`, `{projectRoot}/pyproject.toml`, `uv.lock`
- Interpreter: `.venv/bin/python3` (local; `.devcontainer/devcontainer.json:183`, `.vscode/settings.json:65`)
- **Cache locations:**
  - `~/.cache/uv` (user-level, 22 GB) — no `UV_CACHE_DIR` override
  - `.venv` (workspace-local)
  - `__pycache__` (excluded from policy: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json:60-61`)
- **pytest:** Configured but no `pytest` cache routing defined
- **Missing:** `UV_CACHE_DIR`, `PYTHONPYCACHEPREFIX`, `.pytest_cache` coordination

### .NET

**Current Setup:**
- Input tracking: `nx.json:31` defines `dotnet` as `{projectRoot}/**/*.cs`, `{projectRoot}/**/*.csproj`, `Monorepo.sln`
- Cache path: `.🧬semio/🦑️repo/⚡️cache/dotnet/repo-test` (196 KB, hardcoded in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/📜️script.ts` line ~25)
- Build outputs: `bin/`, `obj/` (workspace-scoped, not centralized)
- Environment: `DOTNET_CLI_TELEMETRY_OPTOUT=1`, `DOTNET_NOLOGO=1` (`.devcontainer/devcontainer.json:106`)
- **Missing:** No override for `bin/`/`obj/` build directories or NuGet cache routing

### CMake

**Current Setup:**
- Preset: `CMakePresets.json:12` sets `binaryDir: "${sourceDir}/.🧬semio/🦑️repo/⚡️cache/cmake/${presetName}"`
- VSCode integration: `.devcontainer/devcontainer.json:214` configures `cmake.buildDirectory` to the same path
- **Subdirectories:** `macos/` (372 KB currently)
- **Fully routed into cache root** ✓

### Tectonic / LaTeX

**Current Setup:**
- Provisioned: `.🧬semio/🦑️repo/⚡️cache/tectonic/0.16.9/` (68 MB)
- Script: `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts` manages bundles
- Fonts: `.🧬semio/🦑️repo/⚡️cache/print-fonts/` (2.0 MB; manual provisioning via `Anta-Regular.ttf`, `NotoEmoji-Regular.ttf`, `ShareTechMono-Regular.ttf`)
- **Fully routed into cache root** ✓

### Playwright

**Current Setup:**
- Devcontainer: `.devcontainer/devcontainer.json:98` mounts `ms-playwright` volume; line 107 sets `PLAYWRIGHT_BROWSERS_PATH=/workspaceFolder/node_modules/.cache/ms-playwright`
- User cache: `~/Library/Caches/ms-playwright` (554 MB, built on demand)
- **Missing:** No route to `.🧬semio/🦑️repo/⚡️cache/playwright/`; relies on `node_modules/.cache/` which is reset on reinstalls

### Matplotlib / PyNite

**Current Setup:**
- Font cache: Built on import (via `matplotlib` in `.venv`)
- **Locations:** `~/.cache/matplotlib/`, `~/.cache/pip-tools/`
- **Missing:** No coordinate with `.🧬semio/🦑️repo/⚡️cache/`

---

## Area C: User-Level Caches Growing with Builds

| Cache | Size | Path | Controlled by | Can Route to Repo? |
|-------|------|------|---------------|-------------------|
| sccache | 10 GB | `~/Library/Caches/Mozilla.sccache` | (hardcoded by sccache binary) | ❌ Needs `SCCACHE_DIR` env |
| uv | 22 GB | `~/.cache/uv` | `UV_CACHE_DIR` env | ✓ Yes, set in `📜️script.ts` |
| Cargo registry | 2.1 GB | `~/.cargo/registry` | `CARGO_HOME` env | ✓ Yes, but not yet done |
| Cargo git | 14 MB | `~/.cargo/git` | `CARGO_HOME` env | ✓ Yes, but not yet done |
| Go build | 945 MB | `~/Library/Caches/go-build` | `GOCACHE` env | ✓ Yes, set in `📜️script.ts` |
| Bun install | 7.3 GB | `~/Library/Caches/bun` | `BUN_INSTALL` env | ✓ Yes, configurable |
| Playwright | 554 MB | `~/Library/Caches/ms-playwright` | `PLAYWRIGHT_BROWSERS_PATH` env | ✓ Yes, already set in devcontainer |
| cargo-component | 0 B | `~/Library/Caches/cargo-component` | (plugin cache) | ❌ Hardcoded path |

**Total user-level: ~52 GB**

---

## Nx Configuration

**File:** `nx.json:63-64`

```json
"cacheDirectory": ".nx/cache",
"maxCacheSize": "8GB"
```

- Nx's own cache is separate from the repo cache root
- All task outputs marked `"cache": true` go here, not `.🧬semio/🦑️repo/⚡️cache/`
- Named inputs track `sharedGlobals` (line 10) which includes `⚡️caching/**/*`

---

## Environment Variable Routing

**Central point:** `devToolingEnv()` @ `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts:8-20`

Currently sets:
- `NODE_OPTIONS`: deleted (line 10)
- `VSCODE_INSPECTOR_OPTIONS`: deleted (line 11)
- Nx logging flags (lines 12–17)
- `RUSTC_WRAPPER`: defaulted to `""` (line 18)

**Does NOT set:** `CARGO_TARGET_DIR`, `SCCACHE_DIR`, `UV_CACHE_DIR`, `GOCACHE`, `GOMODCACHE`, `CARGO_HOME`, `BUN_INSTALL`, `DOTNET_*`, `CMAKE_*`, `PLAYWRIGHT_BROWSERS_PATH`

These are set individually in calling scripts or hardcoded.

---

## Gaps for Single Shared Cache

### Routed Into `.🧬semio/🦑️repo/⚡️cache/` ✓
1. Cargo (WASM browser): `.🧬semio/🦑️repo/⚡️cache/cargo/browser/` (54 GB)
2. CMake: `.🧬semio/🦑️repo/⚡️cache/cmake/` (372 KB)
3. .NET test state: `.🧬semio/🦑️repo/⚡️cache/dotnet/repo-test/` (196 KB)
4. Tectonic/LaTeX: `.🧬semio/🦑️repo/⚡️cache/tectonic/` (68 MB)
5. Print fonts: `.🧬semio/🦑️repo/⚡️cache/print-fonts/` (2.0 MB)
6. Test fixtures: `.🧬semio/🦑️repo/⚡️cache/tests/` (138 MB)
7. Tools (binaryen, tectonic): `.🧬semio/🦑️repo/⚡️cache/tools/` (83 MB)
8. Renderer modules: `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/` (2.6 GB)
9. Generator inputs: `.🧬semio/🦑️repo/⚡️cache/🔏️generator-inputs/` (4 KB)
10. Oracle binaries: `.🧬semio/🦑️repo/⚡️cache/oracles/` (1.3 GB)

### NOT Routed (Bypass User Home) ❌
1. **sccache** (rustc compiler cache, 10 GB)
   - Lives in `~/Library/Caches/Mozilla.sccache`
   - **Fix:** Set `SCCACHE_DIR` in `devToolingEnv()` or `.cargo/config.toml`
   - **Impact:** 10 GB user-level leak; shared across all projects

2. **Go cache** (go build, 945 MB)
   - Lives in `~/Library/Caches/go-build`
   - **Fix:** Set `GOCACHE` env (already referenced in `📜️script.ts` but not globally applied)
   - **Impact:** 945 MB user-level leak

3. **uv cache** (Python, 22 GB)
   - Lives in `~/.cache/uv`
   - **Fix:** Set `UV_CACHE_DIR` globally in `devToolingEnv()`
   - **Impact:** 22 GB user-level leak; largest Python cache

4. **Cargo registry/git** (Rust deps, 2.1 GB + 14 MB)
   - Lives in `~/.cargo/`
   - **Fix:** Set `CARGO_HOME` to `.🧬semio/🦑️repo/⚡️cache/cargo/` (but separate from browser target dir)
   - **Impact:** 2.1 GB leak

5. **Bun install cache** (7.3 GB)
   - Lives in `~/Library/Caches/bun`
   - **Fix:** Set `BUN_INSTALL` in `devToolingEnv()`
   - **Impact:** 7.3 GB leak

6. **Playwright** (554 MB)
   - Devcontainer routes to `node_modules/.cache/` but local dev uses `~/Library/Caches/ms-playwright`
   - **Fix:** Ensure `PLAYWRIGHT_BROWSERS_PATH` is set globally in `devToolingEnv()`
   - **Impact:** 554 MB leak on local dev; already handled in devcontainer

7. **.NET build dirs** (`bin/`, `obj/`)
   - Not centralized; scattered across codebase
   - **Fix:** Could set `DOTNET_CLI_SDK_TELEMETRY_OPTOUT` but build dir coordination is .NET convention
   - **Impact:** Unknown total; varies per project

8. **pytest cache** (`.pytest_cache`)
   - Excluded from policy (`policy.json:60`) but not routed to shared cache
   - **Fix:** Set `PYTEST_CACHE_DIR` or pytest.ini configuration
   - **Impact:** Minimal (each test run recreates)

9. **rustc incremental** (if enabled)
   - Stored in `CARGO_TARGET_DIR/release/deps/` (already in browser cache)
   - But native Rust builds use root `target/` not routed to cache
   - **Fix:** Ensure all Rust builds set `CARGO_TARGET_DIR` to `.🧬semio/🦑️repo/⚡️cache/cargo/*` with appropriate profile

10. **Node.js/Bun disk cache** (turbo, next, vite)
    - Stored in `node_modules/.cache/` (reset on `bun install`)
    - **Already handled** via `node_modules` volume in devcontainer

---

## Observations

### What Works Well
- **Central resolution:** All writes to `.🧬semio/🦑️repo/⚡️cache/` flow through one function (`getRepoMetaDir()`)
- **Cross-platform CMake:** Preset path is templated for any preset name
- **Test orchestration:** Outputs clearly marked in `.mjs` plugin
- **Zero-touch provisioning:** Oracle binaries, fonts, tools download on demand into cache

### What Leaks
- **52 GB in user home:** `~/.cargo/`, `~/.cache/uv/`, `~/Library/Caches/{sccache,bun,go-build,playwright}`
- **No global env setup:** Individual scripts hardcode `CARGO_TARGET_DIR`, `GOCACHE`, etc. instead of centralizing in `devToolingEnv()`
- **Devcontainer divergence:** Playwright is hardcoded in devcontainer but not in local dev scripts
- **No .NET output routing:** `bin/`/`obj/` dirs bypass cache root entirely
- **Incremental compilation:** If enabled globally, rustc could grow cache; currently disabled but not enforced

### Recommendations for Closure
1. **Move `devToolingEnv()`** to accept an object of all cache paths and return one complete env dict
2. **Route all 8 user-level caches** into `.🧬semio/🦑️repo/⚡️cache/` subdirectories
3. **Update Nx task outputs** to reference cache paths instead of project-local paths
4. **Consolidate .NET builds:** Either use .NET's `bin/obj` override via `csproj` `<IntermediateOutputPath>` or move to cache
5. **Enforce `CARGO_INCREMENTAL=0`** in `devToolingEnv()` to prevent unbounded cache growth
6. **Add `SCCACHE_DIR`** to `.cargo/config.toml` or env setup
7. **Document the cache policy** in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`
