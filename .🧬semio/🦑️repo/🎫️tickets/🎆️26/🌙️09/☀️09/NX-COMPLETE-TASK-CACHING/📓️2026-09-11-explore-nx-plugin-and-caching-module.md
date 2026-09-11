# NX Plugin and Caching Module Exploration

## Overview

The Semio monorepo uses a custom Nx plugin to infer projects from `📋️project.json`, `Cargo.toml`, and `bun.lock` files, combined with a comprehensive caching module (`⚡️caching/`) that manages inputs, outputs, and cache policies for deterministic builds.

**Current cache location**: `.nx/cache` (max size: 8GB, nx.json:63-64)

---

## Part 1: NX Plugin Architecture

### Plugin Location & Registration
- **Main plugin**: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` (986 lines)
- **Registered in**: `nx.json:53-55` as `"plugin": "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"`
- **Includes filters**: `["**/📋️project.json", "**/Cargo.toml", "bun.lock", "**/*.patch"]`
- **Test plugin**: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs`

### Project Inference

#### From `📋️project.json` Files
- Direct parsing at `🟨️.mjs:838-847`
- Must have a `name` field (🟨️.mjs:856)
- Discards files with U+FFFD emoji replacements and generated directories
- Loads and applies caching policy via `targetPolicy()` function

#### From `Cargo.toml` Files  
- Discovered at `🟨️.mjs:869-882`
- Only if `[package]` exists (🟨️.mjs:874)
- Generates implicit `build`, `check`, `test` targets if no explicit `📋️project.json`
- Creates `@nativeDependencies` edges from path-dependencies

#### From `bun.lock`
- Loaded at `🟨️.mjs:886` when `analyzeLockfile: true`
- Creates npm external nodes via `readBunLockGraph()`
- Records patch bytes inline for deterministic hashing

### Target Generation

#### Cargo Targets (`cargoTargets()`, lines 688–702)
**Exposed for every Cargo package:**
- `build`: outputs `["{projectRoot}/dist/build"]`, cache: true
- `check`: **outputs: `[]`** (CACHE-06 GAP: no outputs declared), cache: true
- `test`: **outputs: `[]`** (CACHE-06 GAP: no outputs declared), cache: true

**Inputs for all three:**
- `nativeSources` or `nativeTestSources` (for test)
- `^nativeSources` or `^nativeTestSources` (transitive deps)
- `nativeCommandInputs()` output (toolchain, environment, runtime checks)

**Example** (line 699):
```javascript
inputs: [target === "test" ? "nativeTestSources" : "nativeSources", 
         target === "test" ? "^nativeTestSources" : "^nativeSources", 
         ...commandInputs]
```

#### Component Targets (`componentTargets()`, lines 704–733)
**For Cargo crates with `[package.metadata.semio.component]`:**
- `component-dev` & `component-release`: outputs `["{projectRoot}/dist/component-{profile}"]`, cache: true
- `materialize-dev` & `materialize-release`: outputs `["{workspaceRoot}/${webRoot}/dist/{profile}/🔌️plugin-modules/${moduleDirectory}"]`, cache: true

**Inputs:**
- Line 722: `["nativeSources", "^nativeSources", ...commandInputs, { env: "SEMIO_PLUGIN_SYMBOLS" }]`
- Line 729: `["production", "^production", { dependentTasksOutputFiles: "**/*" }, ...]`

#### Print Document Targets (`printDocumentTargets()`, lines 664–686)
**For each document in `project.metadata.printCatalog`:**
- `build-{id}`: outputs `["{projectRoot}/dist/documents/{id}"]`, cache: true
- `watch-{id}`: outputs `[]`, cache: false, continuous: true

#### Playground Targets (`playgroundPreparationTargets()`, lines 757–804)
**For browser playgrounds (Cargo metadata):**
- Deployment-specific: outputs vary by profile (dev/release)
- `build-{variant}-react-release`: outputs `["{projectRoot}/dist/build-{variant}-react-release"]`, cache: true

### Policy Application

#### `targetPolicy()` Function (lines 379–384)
**Applies caching rules based on target name:**

```javascript
function targetPolicy(name, target, policy = POLICY) {
  if (matchesCommand(name, policy.continuous)) return { ...target, cache: false, continuous: true };
  if (matchesUncached(name, policy.uncached) || mutatingName(name) || liveName(name)) 
    return { ...target, cache: false };
  if (cacheableFamily(name) || verifyCommand(target)) 
    return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  return { ...target, cache: target.cache !== false };
}
```

**Cacheable families** (line 30, `cacheableFamily()` regex):
- `test(-quick|-long|-exhaustive)?`
- `check`, `lint`, `typecheck`, `format-check`
- `generate`, `schema`, `verify`, `stdio`, `build`, `wasm`, `native-build`
- `package`, `extension-package`, `cpp-*`, `graph-check`, `policy-check`, `artifact-*`
- Targets matching `contract` or `generator` pattern

**Uncached targets** (policy.json:4–37):
- `activate`, `audit`, `clean`, `deploy`, `doctor`, `format`, `fuzz`, `install`
- `migrate`, `setup`, `test-discover`, `test-e2e`, `test-fixture-*`, `publish`
- etc. (~35 patterns)

**Continuous targets** (policy.json:39–45):
- `dev`, `serve`, `watch`, `start`, `test-watch`, `daemon`

**Mutating patterns** (line 27): targets matching `/(?:^|-)(?:reset|clean|gc|prune|report|setup|fuzz)(?:-|$)/`

**Live patterns** (line 28): targets matching `/(?:^|-)(?:e2e|live)(?:-|$)/`

---

## Part 2: Caching Module Structure

### Root: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/`

**Policy & Configuration:**
- `🔣️policy.json` (492 lines): master caching policy
  - Uncached/continuous commands
  - Toolchain contract (files, env, commands) for JS, Cargo, Go, Python, .NET, CMake, WASM
  - Target defaults (test, build, lint, check, etc.)
  - Generator input fingerprints
- `📋️project.json` (100 lines): defines repo-level tasks (ci-baseline, test, policy-check, artifact-check, etc.)
- `📜️script.ts` (main entry): orchestrates caching operations

**Submodules with their functions:**

#### `🦀️cargo/📜️script.ts`
- **Purpose**: Rust/Cargo build orchestration
- **Exports**: 
  - `startNativeProgress()` — periodic progress output during long-running cargo waits
  - `runOwnedCommand()` — bounded command execution with SIGTERM/SIGKILL handling
  - `validateNativeCargoArguments()` — enforce contract (no `--workspace`, `--package`, etc.)
  - `artifactRustCargoArguments()` — normalize caller args + extract test level
  - `buildCargoArtifacts()` — capture Cargo deliverables to staging dir
- **Input data**: Cargo.toml manifest, optional test level, optional custom output dir
- **Output data**: Staged artifact files in `dist/build`
- **Commands wired**: Used by cargo build/check/test targets (line 700: `"bun ${script} native cargo ${target} --manifest ..."`)

#### `🚀️bootstrap/`
**Bootstrap & toolchain initialization**
- `📜️script.ts` — workspace root entrypoint (bun entry for dev/staging/CI)
- `📦️dependencies/📜️script.ts` — dependency fetch & caching
- `🛠️tools/📜️script.ts` — tool version management
- `🛠️tools/🕸️wasm/📜️script.ts` — WASM toolchain fingerprint + wasm-opt/wasm-bindgen setup
  - **Used by**: policy.json:215 — `WASM_* toolchain environment fingerprint`

#### `📦️artifacts/`
**Artifact registry & staging**
- `📜️script.ts` — main artifact coordinator
- `🟦️typescript/📜️script.ts` — TypeScript/Vite artifact preparation (vite.config.ts resolution, output layout)
- `🦀️rust/📜️script.ts` — Rust artifact staging (bin/lib output collection)
- `📇️registry/🟦️.ts` — `ArtifactRegistry`: keeps track of all cacheable deliverables
  - `createArtifactRegistry()` — build in-memory registry from all project targets
  - `measureArtifactRegistry()` — compute sizes and budget impact
  - `artifactBudgets()` — enforce limits per category (cache size limits)
- `🐳️containers/` — Docker build context staging (dev/test/release profiles)
- `🗂️files/` — file operations & staging coordination

#### `🌐️vite/`
**Vite dev server & cache session management**
- `📜️script.ts` — Vite proxy + HMR orchestration
- `🧾️session/` — browser session cache (dev server state, HMR sockets, etc.)
  - `🧬️schema/` — session state schema definition

#### `📇️inventory/🟦️.ts`
**Nx graph inventory & source mapping**
- `readInventoryGraph()` — full Nx project graph with resolved inputs/outputs
- Tracks: projects, targets, inputs, outputs, dependsOn relationships
- Used by main cache audit to detect CACHE-* violations

#### `🔏️inputs/📜️script.ts`
**Input discovery & source contracts**
- `sourceInputContract` resolution for explicit named inputs
- Registry catalog generation (`registry-catalog` command at line 88)
- Output: `.🧬semio/🦑️repo/⚡️cache/🔏️generator-inputs/📇️registry/🔣️.json`

#### `🧪️tests/⚡️cache-contracts/`
**Cache policy test suite**
- Validates that caching rules match expected behavior
- Tests inputs/outputs correctness

#### `🚦️ci/📜️script.ts`
**CI cache baseline & metrics**
- Tracks cache hit/miss rates
- CI-specific cache strategy (pull/push to remote, etc.)

---

## Part 3: Named Inputs in nx.json

### Declared at `nx.json:8–31`

**`default`** (line 9):
```json
["{projectRoot}/**/*", "sharedGlobals"]
```
- All project sources + workspace-wide globals

**`sharedGlobals`** (lines 10–19):
```json
[
  "{workspaceRoot}/📜️script.ts",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/**/*",
  { "runtime": "node -p \"process.platform.concat(process.arch)\"" },
  { "runtime": "bun --version" }
]
```
- Plugin, caching module, root script, platform/runtime

**`schemaSources`** (lines 21–25):
```json
[
  "sharedGlobals",
  "{workspaceRoot}/**/🧬️schema/**/*",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"
]
```
- For schema generation targets

**`production`** (line 27):
```json
["default", "!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*", ...]
```
- Excludes test/fixture code

**`cargo`** (line 28):
```json
["{projectRoot}/**/*.rs", "{projectRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock"]
```
- Cargo-specific: .rs files + manifest + lock

**Language-specific**: `go`, `uv` (Python), `dotnet` — similar structure

### Dynamic Named Inputs

Computed in `projectInputs()` (lines 518–578 of 🟨️.mjs):

**`nativeSources`** (native.rs files + production excludes)
**`nativeTestSources`** (includes test files)
**`artifactSources`** (artifact TypeScript projects)
**`artifactCommandSources`** (artifact 📜️script.ts imports)

---

## Part 4: Gaps for Single Shared Cache

### CACHE-06: Missing Output Declarations

**Severity**: HIGH — Prevents cache reuse across lanes

#### Cargo `check` and `test` targets
- **Location**: `🟨️.mjs:698`
- **Issue**: `outputs: target === "build" ? [...] : []`
- **Impact**: Nx cannot track what these targets produce; cache invalidates on any code change
- **Fix needed**: Declare outputs for:
  - `check`: Likely `[]` if read-only, OR capture compiler metadata if needed
  - `test`: Likely `[]` if read-only, OR capture test-results (JUnit XML, coverage)

**Affected targets in repo:**
- Any Cargo crate `build:build` ✓ (has outputs)
- Any Cargo crate `check:check` ✗ (no outputs)
- Any Cargo crate `test:test` ✗ (no outputs)

#### Other cacheable targets missing outputs
- `policy-check` (repo:📋️project.json:24): cache: true, **outputs: `[]` ✗**
- `graph-check` (repo:📋️project.json:30): cache: true, **outputs: `[]` ✗**
- `artifact-check` (repo:📋️project.json:37): cache: true, **outputs: `[]` ✗**
- `artifact-package-contract` (repo:📋️project.json:44): cache: true, **outputs: `[]` ✗**
- `toolchain` (repo:📋️project.json:91): cache: true, **outputs: `[]` ✗**
- `format-check` (policy.json:414–421): cache: true, **outputs: `[]` ✗**
- `typecheck` (policy.json:398–405): cache: true, **outputs: `[]` ✗**
- `lint` (policy.json:223–230): cache: true, **outputs: `[]` ✗**
- `schema-generate` (policy.json:478–483): cache: true, **outputs: `[]` ✗**
- `generate` (policy.json:430–437): cache: true, **outputs: `[]` ✗**
- `verify` (policy.json:438–445): cache: true, **outputs: `[]` ✗**

**Root cause**: Audit rule CACHE-06 at `📜️script.ts:70` checks for this:
```typescript
if ((target.cache || /^(build(?:-|$)|wasm$|native-build$|package$|extension-package$)/.test(name)) 
    && !Object.hasOwn(target, "outputs")) 
  report("CACHE-06", path, identity, "Target still needs an explicit output contract", ...);
```

---

### CACHE-04: Overly Broad Output Paths

**Severity**: MEDIUM — Cache will rebuild on unrelated changes

**Location**: `📜️script.ts:74`
- Artifacts cannot output to `node_modules`, `target`, `.venv`, `.nx`, or workspace root
- Any violation is flagged as CACHE-04

**Current known violations**: (none found in standard targets, but check generated components)

---

### Missing `^` (Dependency) Inputs on Some Targets

**Severity**: MEDIUM — Cache breaks when transitive deps change

**Pattern**: Most cacheable targets DO use `^` correctly:
- Cargo targets: `^nativeSources` / `^nativeTestSources` ✓ (line 699)
- Policy defaults (test, build, lint, check): `^default` ✓ (policy.json)
- Production builds: `^production` ✓ (policy.json:236)

**But some specialized targets lack it:**
- `lint` (policy.json:224): has `^default` ✓
- `typecheck` (policy.json:399): has `^default` ✓
- `schema-generate` (policy.json:479): uses `schemaSources` only ✗ — might need `^schemaSources`
- `artifact-*` (inferred): check whether component deps are included

**Audit rule** at 🟨️.mjs:462 detects missing `^nativeSources` inputs in native targets:
```javascript
if (projectsByRoot.size && target.inputs?.some((input) => input === "^nativeSources" || input === "^nativeTestSources")) {
  // Replace with project-specific list
}
```

---

### Uncached but Deterministic Targets

**Severity**: LOW — Missed optimization opportunity

**Targets set cache: false that could be cached if outputs declared:**
- `setup` (policy.json:220–222): `cache: false` — actually deterministic if input deps stable
- `deps` / `install` — explicitly marked uncached (policy.json:349–353)
- `clean` / `purge` — intentionally mutating (policy.json:361–366)
- `format`, `format-fix` — formatting is deterministic but typically run pre-commit

---

### No Output Staging for Test Results

**Severity**: MEDIUM — CI cannot reuse test outputs

**Issue**: `test` and `test-*` targets cache: true but outputs: []

**Missing outputs**:
- JUnit XML: typically `test-results/*.xml`
- Coverage reports: `coverage/`
- Test logs: `.test-output/`

**Impact**: Each CI run re-executes tests even if inputs unchanged

**Fix**: Decide retention policy:
- `outputs: ["test-results/**/*"]` — cache test results
- `outputs: []` — discard (acceptable if tests are fast enough)

---

### Duplicate Cache Directories Not Unified

**Current state**:
- Nx cache: `.nx/cache` (max 8GB, nx.json:63–64)
- Cargo output: `target/` (shared workspace, unbounded)
- npm/bun: `node_modules/` (lockfile-driven, ~1–2GB typical)
- Vite: `dist/` + `.vite/` cache (unbounded per project)
- Build artifacts: `dist/build/`, `dist/component-*`, etc. (per project)
- Python: `.venv/`, `.cache/`, `__pycache__/` (unbounded)

**Problem**: No single shared cache location for all build outputs

**Solution approach**:
1. Route all Cargo `CARGO_TARGET_DIR` to `{workspaceRoot}/.⚡️build/cargo`
2. Route all Vite cache to `{workspaceRoot}/.⚡️build/vite`
3. Route test/coverage to `{workspaceRoot}/.⚡️build/test-results`
4. Declare outputs for each tool's target

---

## Summary Table: Cache Readiness by Target Type

| Target Family | Cache | Inputs | Outputs | `^` Deps | Status |
|---|---|---|---|---|---|
| `cargo build` | ✓ | nativeSources | dist/build | ✓ | Ready |
| `cargo check` | ✓ | nativeSources | **[] ✗** | ✓ | Needs outputs |
| `cargo test` | ✓ | nativeTestSources | **[] ✗** | ✓ | Needs outputs |
| `component-*` | ✓ | nativeSources | dist/component-* | ✓ | Ready |
| `test` (TypeScript) | ✓ | default | **[] ✗** | ✓ | Needs outputs (optional) |
| `lint` | ✓ | default | **[] ✗** | ✓ | OK (read-only) |
| `typecheck` | ✓ | default | **[] ✗** | ✓ | OK (read-only) |
| `format-check` | ✓ | default | **[] ✗** | ✓ | OK (read-only) |
| `build-*` (print) | ✓ | production | dist/documents/* | ✓ | Ready |
| `materialize-*` | ✓ | production | dist/plugin-modules/* | ✓ | Ready |
| `dev` | ✗ | — | — | N/A | Continuous (no cache) |
| `serve` | ✗ | — | — | N/A | Continuous (no cache) |

---

## Recommendations

1. **Immediate (blocking single cache)**:
   - Declare outputs for cargo `check` and `test` (even if `[]`)
   - Verify audit (`repo:policy-check`) catches all CACHE-06 violations

2. **Short-term (optimizing cache hit)**:
   - Add outputs to test-result targets (if capturing JUnit/coverage)
   - Consolidate tool cache directories under `.⚡️build/`
   - Set `CARGO_TARGET_DIR` to shared location in nx.json named inputs

3. **Monitoring**:
   - Run `repo:cache-verify` in CI to catch output regressions
   - Track cache hit rate with `repo:ci-baseline` metrics
   - Audit with `repo:inventory` when policies change
