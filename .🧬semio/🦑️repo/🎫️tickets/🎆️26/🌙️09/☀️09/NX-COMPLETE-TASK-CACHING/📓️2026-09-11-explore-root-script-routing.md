# Root Script Routing & NX Caching Architecture

## Overview
The monorepo uses a **bun-based script router** (`📜️script.ts`) as the sole entry point for all Nx targets. The root `📋️project.json` delegates every `build`, `test`, `lint`, `verify` target to this script, which further dispatches to specialized Script classes (BuildScript, TestScript, LintScript, VerifyScript, etc.). Actual build/test/check commands (cargo, bun, vite, go, dotnet, cmake) are spawned from TypeScript helpers in the repo library.

---

## Root `📋️project.json` Targets Summary

**File:** `/Users/ueli/Documents/semio/📋️project.json` (1,613 lines)

### Key Targets (cache status, command, dependsOn):

| Target | Executor | Cache | Command | Inputs | Outputs |
|--------|----------|-------|---------|--------|---------|
| **lint** | `nx:run-commands` | `true` | `bun ./📜️script.ts lint` | (default) | `[]` |
| **lint-repo** | `nx:run-commands` | `true` | `bun ./📜️script.ts lint repo` | (default) | `[]` |
| **test** | `nx:run-commands` | `true` | `bun ./📜️script.ts test` | (default) | `[]` |
| **test-discover** | `nx:run-commands` | `true` | `bun ./📜️script.ts test discover` | (default) | (none) |
| **test-doctor** | `nx:run-commands` | `true` | `bun ./📜️script.ts test doctor` | (default) | (none) |
| **test-contract** | `nx:run-commands` | `false` | `bun ./📜️script.ts test contract` | (default) | (none) |
| **test-oracle** | `nx:run-commands` | `false` | `bun ./📜️script.ts test oracle` | (default) | (none) |
| **build** | `nx:run-commands` | `true` | `bun ./📜️script.ts build` | (default) | `[]` |
| **build-storybook** | `nx:run-commands` | `true` | `bun ./📜️script.ts build storybook` | `production`, `^production`, `.storybook/**/*`, `STORYBOOK_*` env | `storybook-static` |
| **cpp** | `nx:run-commands` | `false` | `bun ./📜️script.ts cpp` | (default) | `[]` |
| **cpp-setup** | `nx:run-commands` | `false` | `bun ./📜️script.ts cpp setup` | (default) | `[]` |
| **cpp-configure** | `nx:run-commands` | `true` | `bun ./📜️script.ts cpp configure` | (default) | `[]` |
| **cpp-build** | `nx:run-commands` | `true` | `bun ./📜️script.ts cpp build` | (default) | `[]` |
| **cpp-test** | `nx:run-commands` | `true` | `bun ./📜️script.ts cpp test` | (default) | `[]` |
| **verify** (workspace-level) | `nx:run-commands` | `false` | `bun ./📜️script.ts verify` | (default) | (varies: 800+ verify-* targets defined) |

**Pattern:** All root targets use `forwardAllArgs: true` except a few verify targets. Most `verify-*` targets have `cache: false`.

**DependsOn structure:**
- `lint`, `test`, `build` have `dependsOn: [{target: "<same>", projects: ["*", "!workspace"]}]` — orchestrate per-project targets
- `build` also depends on `build-storybook`
- `cpp` depends on `cpp-test`; `cpp-build` depends on `cpp-configure`; `cpp-test` depends on `cpp-build`

---

## Root `📜️script.ts` Command Router

**File:** `/Users/ueli/Documents/semio/📜️script.ts` (31,035 lines)

### Router Setup (line 18004):

```typescript
const router = new ScriptRouter(WORKSPACE_ROOT, WORKSPACE_ROOT)
  .register("os", OsScript)
  .register("semio", SemioScript)
  .register("examples", ExamplesScript)
  .register("setup", SetupScript)
  .register("start", StartScript)
  .register("dev", DevScript)
  .register("generate", GenerateScript)
  .register("scale-fixture", <inline class>)
  .register("new", CleanMechanismNewScript)
  .register("schema", SchemaScript)
  .register("lint", LintScript)           // line 18030
  .register("verify", VerifyScript)        // line 18031
  .register("format", FormatScript)
  .register("test", TestScript)            // line 18033
  .register("bench", BenchScript)
  .register("stdio", StdioScript)
  .register("build", BuildScript)          // line 18036
  .register("cpp", CppScript)
  .register("publish", PublishScript)
  .register("purge", PurgeScript)
  .register("clean", CleanScript)
  .register("micro-commit", MicroCommitScript)
  .register("commit", CommitScript);
```

### Main Entry Point (line 29-32):

```typescript
if (import.meta.main) {
  if (!(await dispatchPolicyArgv(process.argv.slice(2), import.meta.url))) {
    await runWorkspaceScriptMain(router);
  }
}
```

---

### Script Classes & Their Command Routing

#### **1. LintScript** (lines 1022–1032)

```typescript
export class LintScript extends Script {
  run(segments: string[]): void {
    if (segments[0] === "repo") {
      console.log("[lint] Nx completed the repository lint graph.");
      return;
    }
    runCmd("bunx", ["dependency-cruiser", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", 
      "--config", ".dependency-cruiser.cjs", "--output-type", "err"], { cwd: this.root, shell: true });
  }
}
```

**Behavior:** 
- `lint repo`: No-op (Nx orchestrated)
- `lint` (default): Calls `bunx dependency-cruiser` on framework/plugins/hub/mit-bestand

---

#### **2. TestScript** (lines 14203–14244)

```typescript
export class TestScript extends Script {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    if (rest[0] === "storybook") { ... runStorybookPlaywright(); ... }
    if (rest[0] === "repo-client") { await this.runRepoGoTest(`./${REPO_CLIENT_GO}`, level, rest.slice(1)); ... }
    if (rest[0] === "repo-mcp") { 
      await this.runRepoGoTest(`./${REPO_MCP_GO}`, level, rest.slice(1));
      runCmd("go", ["build", "-o", mcpOut, `./${REPO_MCP_GO}`], { ... budgetMs: buildBudgetMs(), ... });
      ...
    }
    // Taxonomy-driven phase routing
    const taxonomy = repoTaxonomy(this.root);
    const domain = String(taxonomy.testDomainPath ?? "");
    const phases = (taxonomy.testPhases ?? []) as string[];
    if (domain !== "" && phases.includes(phase)) {
      runCmd("bun", [join(this.root, domain, "📜️script.ts"), phase, ..., ...phaseArgs], 
        { cwd: join(this.root, domain), ...orchestratorBudgetOpts() });
      return;
    }
    ...
  }
  
  private enforceCoverageGate(): void {
    // Walks .🧬semio/🦑️repo/📊️metrics/coverage/{rust|js|py|dotnet|go}
    // Merges LCOV/coverage.info/goProf, writes summary.json, enforces 95% threshold
  }
}
```

**Routed Subcommands:**
- `test storybook` → Run Playwright tests against Storybook
- `test repo-client` / `test repo-mcp` → Go tests + `go build -o <mcp-bin>`
- `test <phase>` (e.g., `test contract`, `test oracle`) → Routed to testing domain (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) via taxonomy
- Coverage merging: LCOV (rust/js/py), coverage.info (dotnet), go profile → single lcov.info, enforces 95% for exhaustive level

---

#### **3. BuildScript** (lines 14970–15000)

```typescript
export class BuildScript extends Script {
  run(segments: string[]): void {
    const slice = segments[0];
    const single: Record<string, string> = {
      assets: "@semio-tech/assets:build",
      storybook: "workspace:build-storybook",
      "coda-desktop": "@semio-tech/coda-desktop:build",
      "repo-cli": "@semio-tech/repo-client:build",
      "repo-server": "@semio-tech/repo-coordinator:build",
      "repo-vscode": "@semio-tech/repo-vscode:build-vsix",
    };
    
    if (!slice) {
      console.log("[build] Nx completed the workspace build graph.");
      return;
    }
    if (slice === "storybook") {
      assertNoOwnedStorybookMdx(this.root);
      runCmd("bunx", ["storybook", "build", "-c", ".storybook", 
        "--output-dir", process.env.STORYBOOK_OUTPUT_DIR ?? "storybook-static"], { cwd: this.root });
      assertUiStorybookDiscovery(this.root);
      return;
    }
    const target = single[slice];
    if (!target) { ... process.exit(1); }
    runCmd("bun", ["nx", "run", target], { cwd: this.root, ...orchestratorBudgetOpts() });
  }
}
```

**Behavior:**
- Empty/no args: No-op (Nx orchestrated each per-project build)
- `build storybook` → `storybook build` via `bunx`
- `build <slice>` (assets/coda-desktop/repo-cli/repo-server/repo-vscode) → `nx run <target>`

---

#### **4. VerifyScript** (lines 7091–7650+, massive router)

~550+ lines routing to verification tasks across:
- `verify taxonomy`, `verify mutation-outcome-law`, `verify semantic-vocabulary`, `verify package-purity`
- `verify rust-warnings` → Calls `runCargo(["clippy", "-p", pkg, ...scope...], ...)` per package (line 7641)
- `verify host-ownership` → `cargo test -p semio-framework-os-renderer-wgpu --lib <filter>`
- `verify wires-document-contract` → TypeScript oracle + `cargo test --lib committed_diff`
- `verify shared-artifact-addressing` → TypeScript oracle + `cargo test -p semio-framework-os-kernel --lib`
- `verify artifact-contract-ownership` → Cargo `check --tests` & `test --lib --no-fail-fast`
- `verify flow-window-ownership` → TypeScript oracle + `cargo check --tests` or `cargo test --lib`
- `verify stdio-document-contract` → TypeScript + Cargo
- 50+ other verify-* paths, many calling TypeScript imports + `runCargo(...)`

**Cargo spawn pattern** (line 7641):
```typescript
runCmd("cargo", ["clippy", "-p", pkg, ...scope.scopeArgs, ...scope.targetArgs, 
  ...(scope.packageArgs[pkg] ?? []), "--", "-D", "warnings"], 
  { cwd: this.root, budgetMs: buildBudgetMs() });
```

---

## Cache Root Resolution

**File locations:**

1. **`getRepoMetaDir(root)` function** — defined in `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`
   - Imported into root script at line 92
   - Returns: `<repo-root>/.🧬semio/🦑️repo/` (the metadata directory)

2. **Cache directory references:**
   - Neo4j cache: `getRepoMetaDir(root) + "/⚡️cache/neo4j/neo4j-community-<VERSION>/bin/cypher-shell"`  (line 18110)
   - CMake cache: `getRepoMetaDir(root) + "/⚡️cache/cmake/<preset>"` (line 15108)
   - sccache: `getRepoMetaDir(root) + "/⚡️cache/sccache"` (line 316)
   - Manifest export: `getRepoMetaDir(root) + "/🛂️manifest"` (line 18226)

3. **Coverage directory references** (via `coverageDir` import):
   - `coverageDir(root, "rust")` → walk for `.lcov` files
   - `coverageDir(root, "js")` → walk for `lcov.info`
   - `coverageDir(root, "py")` → walk for `.lcov`
   - `coverageDir(root, "dotnet")` → walk for `coverage.info`
   - `coverageDir(root, "go")` → walk for `.cover` (Go profile)

---

## Existing NX Integration Points

### **1. At Root Level**

**No `nx.json` or `nxw.json` file exists.** All NX configuration is implicit:

- `📋️project.json` uses `nx:run-commands` executor for every target
- Root script dispatcher is the **ONLY entry point** — even `bun nx run workspace:build` ultimately calls `bun ./📜️script.ts build` via project.json

### **2. Within Script Classes**

Scripts call `nx run` for orchestration:

- **BuildScript** (line 14998): `runCmd("bun", ["nx", "run", target], { cwd: this.root, ...orchestratorBudgetOpts() })`
- **TestScript** (line 14237): `runCmd("bun", [join(this.root, domain, "📜️script.ts"), phase, ...], { ...orchestratorBudgetOpts() })`

**`orchestratorBudgetOpts()`** (imported, defined in repo library):
- Returns budget options (timeout, etc.) suitable for orchestrator-level calls (Nx spans all projects)
- Different from `buildBudgetMs()` which budgets individual package builds

### **3. Calls to `bun nx run`**

Examples across script.ts:
- Line 21: `runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:scale-fixture-check"], ...)`
- Line 14998: `runCmd("bun", ["nx", "run", target], { cwd: this.root, ...orchestratorBudgetOpts() })`
- Multiple Dev/Setup/Gen scripts call `nx run` subproject targets

---

## Per-Package Build/Test/Lint Routing

### **At Root Level:**
- Root targets depend on per-project targets: `dependsOn: [{target: "build|test|lint", projects: ["*", "!workspace"]}]`
- Nx orchestrates parallel per-package runs

### **Per-Package Level (e.g., `@semio-tech/framework-library`):**
- Each package's own `📋️project.json` (in its directory) defines `build`, `test`, `lint` targets
- These call the **package-level script.ts** (e.g., `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`)
- Package scripts use the **same design**: route segments to build/test/lint classes

**Example pattern** (package-level target):
```json
{
  "build": {
    "executor": "nx:run-commands",
    "cache": true,
    "options": { "command": "bun ./📜️script.ts build", "forwardAllArgs": true },
    "inputs": [...],
    "outputs": [...]
  }
}
```

---

## Root `package.json` Scripts

**File:** `/Users/ueli/Documents/semio/package.json`

### Nx Bootstrap & Direct Entry Points:

```json
"scripts": {
  "nx": "bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx",
  "setup": "bun nx run workspace:setup",
  "start": "bun nx run workspace:start",
  "dev": "bun nx run workspace:dev",
  "dev:storybook": "bun nx run workspace:dev-storybook",
  "dev:puzzle:2d": "bun nx run workspace:dev -- 2d",
  "test": "bun nx run workspace:test",
  "build": "bun nx run workspace:build",
  "lint": "bun nx run workspace:lint",
  ...
}
```

**Pattern:** All scripts call `bun nx run workspace:<target> [--args]`, which resolves to `📋️project.json` → `bun ./📜️script.ts <verb>`

---

## Gaps for Per-Package NX Caching

1. **No `NX_*` environment variables set in script.ts** — scripts do not configure:
   - `NX_CACHE_DIRECTORY` — could point to `.🧬semio/🦑️repo/⚡️cache/nx`
   - `NX_SKIP_NX_CACHE` — always false (default)
   - `NX_CACHE_ALGORITHM` — defaults to content-hash

2. **No `createNodes` plugin in nx.json** — project.json files are fully static:
   - Cannot dynamically infer per-package build/test/lint targets
   - Cannot apply cache rules (inputs/outputs/env) at generation time
   - Every package must hand-write its own target definitions

3. **No shared NX cache across lanes** — concurrent developers/agents:
   - Each gets `NX_CACHE_DIRECTORY=<private-target-dir>` or none
   - No `.git/nxcache` or `.🧬semio/.../nx-cache` for shared builds
   - Builds re-run even if peers completed identical targets

4. **Cargo/Vite/Go not cache-aware** — script.ts spawns without hooking cache:
   - `CARGO_TARGET_DIR` not set (defaults to `target/`)
   - Vite builds never check Nx cache before invoking
   - Go builds never set `GOBIN` or cache dir
   - Each language manages its own cache independently

5. **Budget isolation not enforced** — `buildBudgetMs()` is a timeout, not a gate:
   - Orchestrator budget (`orchestratorBudgetOpts()`) and leaf budgets are unlinked
   - A slow package can block root-level orchestration without fallback
   - No `NX_CACHE_MISSES_THRESHOLD` to fast-fail on unexpected cache misses

6. **No per-command cache config** — all `bun ./📜️script.ts <cmd>` runs use identical cache logic:
   - `cargo build --release` and `cargo clippy` share cache but have different inputs (release vs debug, warnings)
   - `vitest` and `playwright` may share node_modules cache but are not cache-isolated
   - Per-crate cargo features (`--features`) are not part of cache key

---

