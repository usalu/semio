# Plugin Component Bundle Inventory

**Date:** 2026-09-04  
**Scope:** Determine exactly which plugin component WASM bundles exist on disk and their staleness.

---

## 1. WASM Artifact Build Location

**Target Directory:** `target/wasm32-wasip2/{debug,wasm-release}/`

Per `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (lines 1958–1959):
```typescript
const WASM_TARGET_DIR = ["target", "wasm32-wasip2"];
const WASM_PROFILE_DIRS = ["debug", "wasm-release"];
```

---

## 2. Currently Built WASM Artifacts

**Only 3 WASM files exist on disk** (framework components, not plugins):

| File | Path | Size | Mtime |
|------|------|------|-------|
| semio_framework_actor.wasm | target/wasm32-wasip2/debug/deps/semio_framework_actor.wasm | 14361 bytes | 2026-09-03 22:00 |
| semio_framework_math.wasm | target/wasm32-wasip2/debug/deps/semio_framework_math.wasm | 14360 bytes | 2026-09-03 21:59 |
| semio_framework_os_kernel.wasm | target/wasm32-wasip2/debug/deps/semio_framework_os_kernel.wasm | 14365 bytes | 2026-09-03 22:01 |

**All three are older than 2026-09-01.** Last built: **2026-09-03 22:01** (oldest of the three).

---

## 3. Plugin Component Status: Six Demonstrator Plugins + Utilities

### Demonstrator Plugins

All 8 target plugin component WASM files are **MISSING**:

| Plugin | Expected wasmOut Filename | Built Artifact | JSON Descriptor | Notes |
|--------|---------------------------|-----------------|-----------------|-------|
| **cad** (s.cad.cad) | semio_s_plugin_cad.wasm | NOT FOUND | Exists (Sep 4 00:46) | Registry knows v1 hashes |
| **gis** (s.gis.gismap) | semio_s_plugin_gis.wasm | NOT FOUND | Exists (Sep 4 00:46) | Registry knows v1 hashes |
| **procedural** (s.procedural.procedural3d) | semio_s_plugin_procedural.wasm | NOT FOUND | Exists (Sep 4 00:46) | Registry knows v1 hashes |
| **process** (s.process.process3d) | semio_s_plugin_process.wasm | NOT FOUND | Exists (Sep 4 00:46) | Registry knows v1 hashes |
| **puzzle** (s.puzzle.puzzle3d) | semio_s_plugin_puzzle.wasm | NOT FOUND | Exists (Sep 4 00:46) | Registry knows v1 hashes |
| **sourcing** (s.sourcing.curate) | semio_s_plugin_sourcing.wasm | NOT FOUND | Exists (Sep 4 00:46) | Registry knows v1 hashes |
| **stdio** (stdlib) | semio_s_plugin_stdio.wasm | NOT FOUND | Exists (Sep 4 00:46) | No hashes in registry (null) |
| **demonstrator** | semio_s_plugin_demonstrator.wasm | NOT FOUND | Exists (Sep 4 00:46) | Registry knows v1 hashes |

### Registry Descriptor Files (Generated)

Location: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/`

All three registry descriptors refreshed **today at 2026-09-04 00:46**:

| File | Size | Mtime |
|------|------|-------|
| 🔣️plugins.json | 46083 bytes | Sep 4 00:46 |
| 🔣️framework.json | 12898 bytes | Sep 4 00:46 |
| 🔣️playgrounds.json | 26135 bytes | Sep 4 00:46 |

---

## 4. Build Command for One Plugin Component

**Source file:** `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📜️script.ts`

```typescript
/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-cad", join(this.root, "..", "..")));
  }
}
```

**Actual build logic** (from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts`, lines 92–96):

```typescript
export function buildPluginComponent(repoRoot: string, packageName: string, rootCdylib = false, budgetMs = buildBudgetMs()): string {
  const buildArgs = rootCdylib
    ? ["rustc", "-p", packageName, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2"]
    : ["build", "-p", packageName, "--target", "wasm32-wasip2"];
  runCmd("cargo", buildArgs, { cwd: repoRoot, env: devToolingEnv(), budgetMs });
  const component = pluginWasmArtifactPath(repoRoot, packageName);
  if (!existsSync(component)) throw new Error(`cargo did not produce ${component}`);
  return component;
}
```

**Key command:** `cargo build -p <packageName> --target wasm32-wasip2`

**Environment variable:** When building only one plugin, `SEMIO_PLUGIN_ONLY=<pluginId>` is set by the caller.  
**Example from demonstrator script** (`♻️mit-bestand/🧺️demonstrator/📜️script.ts`, lines 20–29):
```typescript
async function buildRuntimePlugin(variant: string): Promise<void> {
  const pluginId = runtimePluginId(variant);
  const previousPluginOnly = process.env.SEMIO_PLUGIN_ONLY;
  process.env.SEMIO_PLUGIN_ONLY = pluginId;
  try {
    await buildPlugins(variant);
  } finally {
    if (previousPluginOnly === undefined) delete process.env.SEMIO_PLUGIN_ONLY;
    else process.env.SEMIO_PLUGIN_ONLY = previousPluginOnly;
  }
}
```

---

## Summary of Findings

1. **No plugin WASM bundles built:** 0 of 8 expected plugin components exist on disk.
2. **Registry metadata is fresh:** Descriptors regenerated today (Sep 4 00:46), containing expected v1 hashes for 7 of 8 plugins.
3. **Framework components only:** Three WASM files built Sep 3 21:59–22:01, all framework support, not plugins.
4. **Build command is standardized:** Each plugin calls `cargo build -p <packageName> --target wasm32-wasip2` via the shared `describePluginComponent()` function; demonstrator app uses `SEMIO_PLUGIN_ONLY=<pluginId>` to isolate builds.
