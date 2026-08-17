# 🧪️ Test Infrastructure Audit for New OS Module + Rust Crate + TS Package

## 1. Exemplar Module Anatomy

### 1.1 `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run` (lib + [[bin]])

**File Structure:**
- `🦀️component.rs` — main library implementation (131KB)
- `📦️bin.rs` — binary entrypoint (17KB)
- `📦️packages/🦀️rust/Cargo.toml` — workspace member manifest
- `📦️packages/🦀️rust/📦️glue.rs` — glue module (720B)

**Cargo.toml** (`📦️packages/🦀️rust/Cargo.toml`):
```toml
[package]
name = "semio-framework-os-kernel"
version = "0.1.0"
edition = "2021"
rust-version = "1.88"
description = "Semio framework OS kernel — wasm-safe store/spr/dsl/pack document model"

[package.metadata.semio]
role = "framework"
id = "os-kernel"

[lints]
workspace = true

[lib]
name = "semio_framework_os_kernel"
crate-type = ["rlib", "cdylib"]
path = "📦️glue.rs"

[features]
default = ["deflate"]
deflate = ["dep:miniz_oxide"]
dsl-fixture-sweep-full = []
ureq = ["dep:ureq"]
sync = ["dep:tokio-tungstenite", "dep:notify", "dep:rusqlite", "tokio/rt", "tokio/time"]
worker = ["sync"]
typegen = ["dep:ts-rs"]

[dependencies]
dsl_derive = { path = "../../🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust", package = "semio-framework-os-kernel-dsl-derive" }
async-trait = "0.1"
base64 = "0.22.1"
blake3 = "1"
futures-lite = "2"
futures-util = { version = "0.3", features = ["sink"] }
miniz_oxide = { version = "0.8", optional = true }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
thiserror = "2.0.12"
tokio = { version = "1", features = ["sync", "macros"], default-features = false }
ts-rs = { workspace = true, optional = true }
ureq = { version = "2", optional = true }
semio-framework-hash = { path = "../../../../🔨️modules/#⃣hash/📦️packages/🦀️rust", package = "semio-framework-hash" }
zip = { version = "2.4", default-features = false, features = ["deflate"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
js-sys = "0.3.83"
serde-wasm-bindgen = "0.6.5"
wasm-bindgen = "0.2.106"
wasm-bindgen-futures = "0.4.71"
console_error_panic_hook = "0.1"
web-sys = { version = "0.3.98", features = ["Window", "Storage", "console", "WebSocket", "MessageEvent", "BinaryType", "Request", "RequestInit", "Response", "Headers"] }

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tokio-tungstenite = { version = "0.26", optional = true }
notify = { version = "8", optional = true }
rusqlite = { version = "0.38.0", optional = true, features = ["bundled"] }
tokio = { version = "1", default-features = false, features = ["net"] }

[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]
tempfile = "3.20.0"

[[bin]]
name = "pack"
path = "../../🔨️modules/🎒️pack/⌨️cli/📦️main.rs"

[[bin]]
name = "spr"
path = "../../🔨️modules/📡️spr/⌨️cli/📦️main.rs"

[[bin]]
name = "semio"
path = "../../🔨️modules/🧬️semio/📦️bin.rs"
```

**📜️script.ts**:
```typescript
#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  run(): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "--lib"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
```

**📦️glue.rs** (summary — mount pattern using `#[path]`):
```rust
//! 💻️ Semio framework OS kernel — wasm-safe document model (store, spr, dsl, pack).

#![allow(unused_extern_crates, ambiguous_glob_reexports, unused_imports)]

extern crate self as dsl;
extern crate self as dsl_grammar;
extern crate self as dsl_notation;
extern crate self as store;
extern crate self as protocol;
extern crate self as pack;
extern crate self as spr;
extern crate self as vcs;
pub extern crate self as semio_format;

// #[path] mount examples:
#[path = "."]
pub mod os_dsl {
  #[path = "../../🔨️modules/🗣️dsl/🦀️component.rs"]
  mod component;
  pub use component::*;

  #[path = "../../🔨️modules/🗣️dsl/📍️span/🦀️component.rs"]
  pub mod span;
  
  // ... many more submodules mounted via #[path]
}

#[path = "."]
pub mod os_pack {
  #[path = "../../🔨️modules/🎒️pack/🦀️component.rs"]
  mod component;
  pub use component::*;
  // ... submodules
}
```

---

### 1.2 `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory` (has 🧬️schema/ facet)

**File Structure:**
- `🦀️component.rs` — main implementation (9.7KB)
- `🟦️component.ts` — TS counterpart (6.8KB)
- `🔌️client/🦀️component.rs` — client submodule (30KB)
- `🪪️identity/🦀️component.rs` — identity submodule (13KB)
- `🧬️schema/` — schema facet:
  - `🔣️component.json` — normative JSON Schema (14KB)
  - `🟦️component.ts` — TS mirror (5.7KB)
  - `🦀️component.rs` — Rust mirror (12KB)

**Note:** Unlike `run`, `directory` has NO `📦️packages/` subdir because `directory` itself is NOT a standalone crate — it lives under the OS product's shared Rust crate (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`), mounted via `#[path]` in that crate's glue.rs. The `🧬️schema/` subdirectory is a structural **facet** (part of the artifact shape taxonomy), not a Cargo workspace member.

---

## 2. TS Package Anatomy (os 📦️packages)

**Location:** `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/`

**package.json**:
```json
{
  "$schema": "../../../../node_modules/nx/schemas/project-schema.json",
  "name": "@semio-tech/framework-os",
  "version": "0.1.0",
  "description": "framework · os — composable operating system shell (CQRS, workflow, plugin registry)",
  "type": "module",
  "private": true,
  "exports": {
    ".": "./🟦️glue.ts",
    "./backbone-worker": "./🟦️glue.backbone-worker.ts"
  },
  "dependencies": {
    "@semio-tech/framework": "workspace:*"
  },
  "scripts": {
    "test": "bun nx run @semio-tech/framework-os:test"
  },
  "devDependencies": {
    "typescript": "^5.9.3",
    "vitest": "^4.0.17"
  },
  "license": "LGPL-3.0-or-later",
  "repository": {
    "type": "git",
    "url": "https://github.com/usalu/semio.git",
    "directory": "framework/product/os"
  },
  "bundleKind": "library"
}
```

**📋️project.json**:
```json
{
  "name": "@semio-tech/framework-os",
  "$schema": "../../../../../node_modules/nx/schemas/project-schema.json",
  "namedInputs": {
    "default": ["{workspaceRoot}/🧰️framework/🛍️products/💻️os/🟦️component.ts", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧫️fixtures/**/*", "{projectRoot}/**/*"]
  },
  "targets": {
    "test": {
      "executor": "nx:run-commands",
      "dependsOn": [],
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test",
        "forwardAllArgs": true
      }
    },
    "test-quick": {
      "executor": "nx:run-commands",
      "dependsOn": [],
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test quick",
        "forwardAllArgs": true
      }
    },
    "test-long": {
      "executor": "nx:run-commands",
      "dependsOn": [],
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test long",
        "forwardAllArgs": true
      }
    },
    "test-exhaustive": {
      "executor": "nx:run-commands",
      "dependsOn": [],
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test exhaustive",
        "forwardAllArgs": true
      }
    }
  }
}
```

**📜️script.ts**:
```typescript
#!/usr/bin/env bun
/** 🖥️ `@semio-tech/framework-os` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
```

**🧪️vitest.config.ts**:
```typescript
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework-os` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-os": resolve(root, "🟦️glue.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-os",
    mode: "test",
    environment: "node",
    include: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts"],
    coverage: { include: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts"] },
    includeSource: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts"],
    passWithNoTests: false,
  },
});
```

**Vitest Invocation:** Via `runVitest()` helper from library package; **tests are in-source** using `import.meta.vitest` pattern (tests in the main component files, not separate `*.test.ts` files).

---

## 3. Test Levels Routing

**Root script.ts** → `TestScript` class (line 1108+):

```typescript
export class TestScript extends Script {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    // ... routing logic
    runCmd("bun", ["nx", "run-many", "-t", testTargetForLevel(level), "--all", "--exclude", "workspace"], { cwd: this.root, ...orchestratorBudgetOpts() });
    if (TEST_LEVELS.indexOf(level) >= TEST_LEVELS.indexOf("long")) {
      await this.runStorybookPlaywright();
    }
    // ... coverage handling
  }
}

function testTargetForLevel(level: TestLevel): string {
  return level === "fundamental" ? "test" : `test-${level}`;
}
```

**TEST_LEVELS**: `["fundamental", "quick", "long", "exhaustive"]` (from library)

**Routing:**
- `bun ./📜️script.ts test` → level="fundamental" → nx target=`test`
- `bun ./📜️script.ts test quick` → level="quick" → nx target=`test-quick`
- `bun ./📜️script.ts test long` → level="long" → nx target=`test-long`
- `bun ./📜️script.ts test exhaustive` → level="exhaustive" → nx target=`test-exhaustive`

**Verification Gate** (`verify` line 919-949):
```typescript
private checkLeveledTestTargets(): void {
  // Every project.json with a "test" target MUST also declare "test-quick", "test-long", "test-exhaustive"
  // Otherwise `nx run-many -t test-exhaustive` silently skips that project and exhaustive coverage under-counts it.
  // This check enforces that every 📋️project.json has all four levels declared.
}
```

**Cite** (script.ts:919-921): "Every `project.json` with a `test` target must also declare `test-quick`/`test-long`/`test-exhaustive`"

---

## 4. Root Workspace Wiring

### 4.1 Cargo.toml Workspace Members & Dependencies

**Members** (Cargo.toml:4-106, sorted by facet):

```toml
[workspace]
members = [
    "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
    # ... ~100 entries total
]

resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.88"

[workspace.dependencies]
semio-framework-math = { path = "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust" }
semio-framework-number = { path = "🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust" }
semio-framework-geometry = { path = "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust" }
semio-framework-graph = { path = "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust" }
semio-framework-actor = { path = "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust" }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.149"
flate2 = { version = "=1.1.9", default-features = false, features = ["rust_backend"] }
libz-sys = "=1.1.29"
wasm-bindgen = "0.2.106"
thiserror = "2.0.18"
tokio = { version = "1" }
ts-rs = "10"
semio-framework = { path = "🧰️framework/📦️packages/🦀️rust" }
semio-framework-os = { path = "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust" }
semio-framework-os-kernel = { path = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }
# ... more path aliases
```

**To insert a new entry:** Add to `members` array in sorted position, add alias to `[workspace.dependencies]` if it will have >5 downstream consumers.

---

### 4.2 TS Workspace Entries

**Root package.json** (workspaces, lines 6-67):

```json
"workspaces": [
  "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript",
  "♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript",
  "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript",
  "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript",
  "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript",
  "🧰️framework/🔨️modules/◻2d/📦️packages/🟦️typescript",
  "🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react",
  "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript",
  # ... ~50 entries total
]
```

**To insert:** Maintain alphabetical grouping by root (`♻️`,`✏️s`,`🧰️`,`🌎️`,`compose`), use exact path to `📦️packages/🟦️typescript` or specific target (e.g., `🎯️targets/⚛️react`).

---

### 4.3 Root project.json Target Shape

**Root 📋️project.json** (targets):

```json
{
  "targets": {
    "setup": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts setup"
      }
    },
    "dev": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts dev",
        "forwardAllArgs": true
      }
    },
    "dev-mcp": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts dev mcp",
        "forwardAllArgs": true
      }
    },
    "generate": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts generate",
        "forwardAllArgs": true
      }
    },
    "test": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts test",
        "forwardAllArgs": true
      }
    }
  }
}
```

---

## 5. Dev Servers + Collab E2E

**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`

### 5.1 collab-e2e Implementation (L2273–2860)

**Port Leasing:**
```typescript
// Port pool: COLLAB_E2E_PORT_MIN to COLLAB_E2E_PORT_MAX
function collabScanPort(envVar: string, taken: Set<number>): number {
  const override = process.env[envVar];
  if (override) {
    const port = Number(override);
    taken.add(port);
    return port;
  }
  for (let port = COLLAB_E2E_PORT_MIN; port <= COLLAB_E2E_PORT_MAX; port++) {
    if (taken.has(port) || isDevPortInUse("127.0.0.1", port)) continue;
    taken.add(port);
    return port;
  }
  throw new Error(`collab e2e: no free port in ${COLLAB_E2E_PORT_MIN}-${COLLAB_E2E_PORT_MAX} for ${envVar}`);
}
```

**Hub Startup** (L2293-2317):
```typescript
async function collabStartHub(port: number, dataDir: string, logPath: string): Promise<SpawnDaemonHandle> {
  const hubScript = join(repoRoot, "./🌎️hub/📦️packages/🦀️rust/📜️script.ts");
  const daemon = spawnDaemon("bun", [hubScript, "dev"], {
    cwd: join(repoRoot, "./🌎️hub/📦️packages/🦀️rust"),
    env: { ...process.env, OS_HUB_PORT: String(port), OS_HUB_DATA: dataDir, OS_HUB_ADMIN_TOKEN: COLLAB_E2E_ADMIN_TOKEN },
    stdio: "pipe",
  });
  daemon.child.stdout?.pipe(logStream);
  daemon.child.stderr?.pipe(logStream);
  const baseUrl = `http://127.0.0.1:${port}`;
  const deadline = Date.now() + COLLAB_E2E_HUB_BOOT_BUDGET_MS;
  while (Date.now() < deadline) {
    if (daemon.child.exitCode !== null) throw new Error(`hub exited early (code ${daemon.child.exitCode}) — see ${logPath}`);
    try {
      await fetch(`${baseUrl}/admin/api/overview`, { headers: { authorization: `Bearer ${COLLAB_E2E_ADMIN_TOKEN}` } });
      return daemon;
    } catch {
      await Bun.sleep(500);
    }
  }
  daemon.kill();
  throw new Error(`hub did not become ready on port ${port} within ${COLLAB_E2E_HUB_BOOT_BUDGET_MS}ms — see ${logPath}`);
}
```

**Plugin Prebuild** (L2383-2421):
- Builds ONLY `COLLAB_E2E_REQUIRED_PLUGIN_IDS = ["s", "writer"]` (not full ~58-crate catalog)
- Reuses shared `PluginBuildLease` for mutual exclusion vs. peer `bun dev s`
- Hard gate: asserts both required artifacts exist after build; throws if `"s"` missing (scenario-fatal)
- Sets `FLOW_CORE_SKIP_WASM_BUILD=1` to skip pre-existing, unrelated defects

**Dev Server Startup** (L2425-2444):
```typescript
async function collabStartUserDevServer(opts: { readonly port: number; readonly hubUrl: string; readonly user: string; readonly dataDir: string; readonly logPath: string }): Promise<SpawnDaemonHandle> {
  const devScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts");
  const daemon = spawnDaemon("bun", [devScript, "dev"], {
    cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript"),
    env: { ...process.env, SKIP_PLUGIN_BUILD: "1", SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react", S_OS_PORT: String(opts.port), S_HUB_URL: opts.hubUrl, S_USER: opts.user, S_DATA_DIR: opts.dataDir },
    stdio: "pipe",
  });
  // ... wait for port to open
}
```

**Playwright Invocation** (L2847-2861):
```typescript
page.on("console", (msg) => console.log(`[collab-e2e:console] ${label} [${msg.type()}] ${msg.text()}`));
page.on("requestfailed", (request) => console.log(`[collab-e2e:network] ${label} requestfailed: ...`));
page.on("response", (response) => console.log(`[collab-e2e:network] ${label} response: ...`));
page.on("websocket", (ws) => {
  console.log(`[collab-e2e:ws] ${label} opened: ${ws.url()}`);
  ws.on("framereceived", (frame) => console.log(`[collab-e2e:ws] ${label} recv: ...`));
  ws.on("close", () => console.log(`[collab-e2e:ws] ${label} closed: ...`));
});
```

**Scenario Execution** (L2516-2887):
- 8 steps: space creation, sharing, artifact creation, replication, user navigation, etc.
- Each step wrapped in try/catch; failures recorded but run continues
- Asserts via DOM `.locator()`, `.waitFor()`, row ID diffing
- Screenshots on failure
- Final pass/fail count reported

**Result Reporting** (L2887):
```typescript
console.log(`[collab-e2e] summary: ${passed}/${results.length} steps passed`);
```

---

## 6. wgpu Native Smoke Test

**Target Crate Name** (Cargo.toml:2): `semio-framework-os-renderer-wgpu`

**Script Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts`

**Flags** (native run, L174-200):
```typescript
class NativeRunScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || "s";
    const ship = segments.includes("--dist") || segments.includes("--release");
    // ... build plugins ...
    const smokeArgs = segments.includes("--smoke") ? ["--smoke"] : [];
    // `--smoke`: boots headless, dumps widget tree as JSON, exits
    const cargoArgs = ["run"];
    if (ship) cargoArgs.push("--release");
    cargoArgs.push("-p", crateName, "--bin", "semio-wgpu-native", "--features", "native-bin", "--", "--plugin", filterPlugin, ...appArgs, ...smokeArgs);
    if (runCmdStatus("cargo", cargoArgs, { cwd: repoRoot, env: nativeEnv, ...daemonBudgetOpts() }) !== 0) {
      throw new Error("native wgpu renderer run failed");
    }
  }
}
```

**Command shape:**
- `bun ./📜️script.ts native [plugin] [--release|--dist] [--smoke]`
- Example: `bun ./📜️script.ts native s --smoke` → boots S plugin headless, dumps JSON, exits

---

## 7. Verify Gate

**What `bun ./📜️script.ts verify gate` runs** (line 792+):

1. **dependency-cruiser boundaries** — checks cross-module imports
2. **generated catalog freshness** — runs `@semio-tech/plugin-registry:check`
3. **region/host-contract lints** — runs react renderer lint, os dev plugin lint, UI tokens check
4. **framework ts-rs binding freshness** — `@semio-tech/framework-rs:check`
5. **ui locale/terminology axes freshness** — `@semio-tech/ui-rs:check`
6. **chrome i18n literal scan** — `@semio-tech/ui-react:check-chrome-i18n`
7. **leveled test target coverage** — calls `checkLeveledTestTargets()` (every `project.json` with `test` MUST have `test-quick/long/exhaustive`)
8. **storybook scope freshness** — `checkStorybookFreshness()`
9. **OS exclusive state authority policies** — `policyOsStateAuthorityBreaches()`, `policyDocumentAppShapeBreaches()`
10. **standards/subsets vocabulary** — `policyStandardsCoverageBreaches()`, `policyStandardSubsetVocabularyBreaches()`
11. **handcrafted grammar P3/M4 policies** — `policyHandcraftedSpecP3Breaches()`
12. **artifact-schema facet policies** — `policyArtifactSchemaBreaches()`
13. **app-schema facet policies** — `policyAppSchemaBreaches()`
14. **dissolve-core / plugin-root policies** — banned name stem, emoji prefix, plugin root shape, builder, apa, inference family
15. **window capability taxonomy** — `policyWindowCompletenessBreaches()`, `policyModeCompletenessBreaches()`
16. **mutation-outcome / merge-policy law** — `policyMutationOutcomeMergePolicyBreaches()` (also standalone via `verify mutation-outcome-law`)
17. **dsl fixture laws** — runs `test-quick` on DSL crates and fixture-sweep

---

### 7.1 Verify Taxonomy Check

**What `verify taxonomy enforce` checks** (line 767-774):
```typescript
private runTaxonomy(args: string[]): void {
  const mode = args[0]; // "report" or "enforce"
  const scope = taxonomyOption(args, "--scope");
  const census = buildSemanticCensus(this.root, { scope });
  // Reports emoji-prefix collisions, facet naming violations, etc.
  if (mode === "enforce" && census.problems.some((problem) => problem.severity === "error")) {
    throw new Error(`[verify taxonomy enforce] ${census.problems.length} error finding(s).`);
  }
}
```

**Taxonomy Rules** (from discovery logic):

A NEW module directory named `🌉️mcp` with sub-facet directories like:
- `🗂️catalog/🦀️component.rs`
- `🧵️bridge/{🦀️,🟦️}component.*`

**Legality TODAY:** ✅ YES. Facet subdirectories ARE legal under the current taxonomy. The repo uses this pattern extensively:
- `📇️directory/🧬️schema/` (schema facet with sub-facets)
- `📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/` (nested facets under targets)

**Taxonomy Rules Applied:**
- Emoji-prefixed directory names MUST be unique repo-wide (enforced by `policyEmojiPrefixBreaches()`)
- Facet subdirectories under known parents (plugin roots, OS product, etc.) must match declared `artifactComponentDirs` or `pluginFacetDirs` keys
- Window/mode completeness: if a crate declares a "window" facet, it must carry all required child facets (schema snapshot/diff/mutations)

**Citation:** Verify check at script.ts:922-949; taxonomy rules built from `buildSemanticCensus()` (library/discovery/component.ts)

---

## 8. MCP SDK Availability

**Path:** `node_modules/@modelcontextprotocol/sdk/`

**Version:** 1.30.0

**Exports from `dist/esm/client/`:**
- ✅ `Client` — exported from `dist/esm/client/index.d.ts` (via `export declare class Client`)
- ✅ `StdioClientTransport` — exported from `dist/esm/client/stdio.d.ts` (implements Transport)
- ✅ `StreamableHTTPClientTransport` — exported from `dist/esm/client/streamableHttp.d.ts` (implements Transport)

**Import paths:**
```typescript
import { Client } from "@modelcontextprotocol/sdk/client";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client";
import { StreamableHTTPClientTransport } from "@modelcontextprotocol/sdk/client";
```

**Files:**
- `dist/esm/client/index.d.ts` — main client exports
- `dist/esm/client/stdio.d.ts` — `StdioClientTransport` (class)
- `dist/esm/client/streamableHttp.d.ts` — `StreamableHTTPClientTransport` (class)
- `dist/esm/shared/transport.d.ts` — `Transport` interface

---

## 9. Cookbook for the New `🌉️mcp` Module

### File Structure Template

```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/
├── 🦀️component.rs              (main Rust lib implementation)
├── 🗂️catalog/                  (facet)
│   └── 🦀️component.rs
├── 🧵️bridge/                   (facet)
│   ├── 🦀️component.rs
│   └── 🟦️component.ts
├── 🧬️schema/                   (facet — if artifacts with state)
│   ├── 🔣️component.json        (normative JSON Schema)
│   ├── 🟦️component.ts          (TS mirror)
│   └── 🦀️component.rs          (Rust mirror)
├── 📦️packages/
│   ├── 🦀️rust/
│   │   ├── Cargo.toml
│   │   ├── 📋️project.json
│   │   ├── 📜️script.ts
│   │   └── 📦️glue.rs
│   └── 🟦️typescript/
│       ├── package.json
│       ├── 📋️project.json
│       ├── 📜️script.ts
│       ├── 🧪️vitest.config.ts
│       ├── 🟦️glue.ts
│       └── 🟦️component.ts
└── 🧫️fixtures/                (optional)
    └── ...fixture assets
```

---

### Cargo.toml Skeleton

```toml
[package]
name = "semio-framework-os-mcp"
version = "0.1.0"
edition = "2021"
rust-version = "1.88"
description = "Semio framework OS MCP gateway — LLM-first protocol adapter"

[package.metadata.semio]
role = "framework"
id = "os-mcp"

[lints]
workspace = true

[lib]
name = "semio_framework_os_mcp"
crate-type = ["rlib", "cdylib"]
path = "📦️glue.rs"

[features]
default = []
typegen = ["dep:ts-rs"]

[dependencies]
semio-framework-os-kernel = { path = "../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
@modelcontextprotocol/sdk = "1.30.0"  # Or use workspace dependency pattern
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
thiserror = "2.0.18"
tokio = { version = "1", features = ["sync", "macros"], default-features = false }
ts-rs = { workspace = true, optional = true }

[target.'cfg(target_arch = "wasm32")'.dependencies]
# WASM variant deps if applicable
js-sys = "0.3.83"
wasm-bindgen = "0.2.106"
```

---

### 📋️project.json Skeleton

```json
{
  "name": "@semio-tech/framework-os-mcp",
  "root": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust",
  "sourceRoot": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp",
  "projectType": "library",
  "tags": [
    "lang:rust",
    "role:framework",
    "family:os-mcp"
  ],
  "targets": {
    "check": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun 📜️script.ts check",
        "cwd": "{projectRoot}"
      }
    },
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun 📜️script.ts test",
        "cwd": "{projectRoot}"
      }
    },
    "test-quick": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun 📜️script.ts test quick",
        "cwd": "{projectRoot}"
      }
    },
    "test-long": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun 📜️script.ts test long",
        "cwd": "{projectRoot}"
      }
    },
    "test-exhaustive": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bun 📜️script.ts test exhaustive",
        "cwd": "{projectRoot}"
      }
    }
  }
}
```

---

### 📜️script.ts Skeleton

```typescript
#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp` task router. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, resolveTestLevel, runCargoTestBudgeted, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    // Rust tests
    await runCargoTestBudgeted(["semio-framework-os-mcp"], this.repoRoot, rest);
    // TS tests (if applicable)
    // runVitest(this.tsRoot, rest, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
```

---

### TS package.json + 📋️project.json + 📜️script.ts

**package.json:**
```json
{
  "name": "@semio-tech/framework-os-mcp",
  "version": "0.1.0",
  "description": "framework · os-mcp — TypeScript client bindings and playground",
  "type": "module",
  "private": true,
  "exports": {
    ".": "./🟦️glue.ts"
  },
  "dependencies": {
    "@semio-tech/framework": "workspace:*",
    "@modelcontextprotocol/sdk": "^1.30.0"
  },
  "devDependencies": {
    "typescript": "^5.9.3",
    "vitest": "^4.0.17"
  }
}
```

**📋️project.json:**
```json
{
  "name": "@semio-tech/framework-os-mcp",
  "$schema": "../../../../../node_modules/nx/schemas/project-schema.json",
  "namedInputs": {
    "default": ["{projectRoot}/**/*"]
  },
  "targets": {
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test",
        "forwardAllArgs": true
      }
    },
    "test-quick": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test quick"
      }
    },
    "test-long": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test long"
      }
    },
    "test-exhaustive": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test exhaustive"
      }
    }
  }
}
```

**📜️script.ts:**
```typescript
#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp` TS task router. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
```

---

### Root Wiring Additions

**Add to Cargo.toml `[workspace]` members** (sorted):
```toml
"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust",
```

**Add to Cargo.toml `[workspace.dependencies]`** (if >5 downstream consumers):
```toml
semio-framework-os-mcp = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" }
```

**Add to root `package.json` `workspaces`** (sorted within framework section):
```json
"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript",
```

---

**End of Audit**

sha256: [will be generated on write]
git log --date=iso --oneline -3 -- 📓️luna-testinfra-audit.md: [will be generated on first commit]
