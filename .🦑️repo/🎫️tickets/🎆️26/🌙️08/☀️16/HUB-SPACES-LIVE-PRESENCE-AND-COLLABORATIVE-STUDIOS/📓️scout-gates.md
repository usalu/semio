# Scout Report: HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS
**Lane: 0-S3 | Date: 2026-08-16**

---

## 1. Dev Script (`📜️script.ts`)

### ScriptRouter Registration (line 3014-3072)
```typescript
const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("verify", VerifyScript)
  .register("layer-lint", CapabilityLayeringLintScript)
  .register("index-lint", PluginIndexExportPathLintScript)
  .register("host-handle-lint", HostHandleReachLintScript)
  .register("parity", ParitySubcommandRouter)
  .register("plugin", PluginSubcommandRouter);
```

### DevScript.run Method (line 1237-1343)
- **Location**: Line 1237 - class definition starts
- **Signature**: `async run(segments: string[]): Promise<void>`
- **Key flow**:
  1. Handles "multi" variant → calls `runViteBunxDev` with fixed port 6071
  2. Parses variant/renderer/filter from segments and env
  3. For streaming builds (react only): calls `ensurePluginRegistry` fast path
  4. Calls `buildEngineWasm` 
  5. For wgpu renderer: spawns trunk dev server directly
  6. For react: starts Vite with `runViteBunxDev` (non-blocking)
  7. If streaming: calls `buildPluginsStreaming`, then `watchPluginRebuilds`
  8. Awaits Vite completion

### Function Signatures & Behaviors

#### `ensurePluginRegistry(filterPlugin?: string): Promise<void>` (line 809)
```typescript
export async function ensurePluginRegistry(filterPlugin?: string): Promise<void> {
  const registryScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts");
  if (runCmdStatus("bun", [registryScript, "generate"], { cwd: repoRoot }) !== 0) throw new Error("plugin registry generation failed");
  const variant = filterPlugin ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT;
  writePlaygroundSession(variant, playgroundSessionPath, repoRoot);
}
```
- Regenerates plugin registry via `registry/script.ts generate`
- Writes playground session file for the active variant
- Used in fast path (no cargo) before Vite starts

#### `buildPluginsStreaming(filterPlugin?: string): Promise<void>` (line 880)
```typescript
export async function buildPluginsStreaming(filterPlugin?: string): Promise<void> {
  const targets = await preparePluginBuildTargets(filterPlugin);
  const hostPluginId = resolvePlaygroundFilter(filterPlugin ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT).pluginId;
  const ordered = [...targets].sort((a, b) => (a.pluginId === hostPluginId ? -1 : b.pluginId === hostPluginId ? 1 : 0));
  for (const target of ordered) {
    try {
      await buildPlugin(target);
    } catch (error) {
      console.error(`[DEBUG] program build failed, continuing with remaining targets: ${target.pluginId}`, error);
    }
  }
}
```
- Prioritizes host plugin (builds first), then continues with others on failure
- Non-blocking from Vite's perspective (runs after server starts)

#### `watchPluginRebuilds(targets: readonly PluginRegistryEntry[]): void` (line 1125)
```typescript
function watchPluginRebuilds(targets: readonly PluginRegistryEntry[]): void {
  const byPluginId = new Map(targets.map((target) => [target.pluginId, target] as const));
  const dirty = new Set<string>();
  let draining = false;

  async function drain(): Promise<void> {
    if (draining) return;
    draining = true;
    try {
      while (dirty.size > 0) {
        const [pluginId] = dirty;
        dirty.delete(pluginId!);
        const target = byPluginId.get(pluginId!);
        if (!target) continue;
        try {
          await buildPlugin(target);
        } catch (error) {
          console.error("[DEBUG] program watch rebuild failed", error);
        }
      }
    } finally {
      draining = false;
    }
  }

  for (const target of targets) {
    watch(pluginWatchRoot(target), { recursive: true }, () => {
      dirty.add(target.pluginId);
      void drain();
    });
  }
  console.log("[DEBUG] watching plugin crates for hot-swap rebuilds");
}
```
- Sets up file watchers on each plugin crate root
- On change: marks dirty, drains queue serially (no concurrent cargo builds)
- Logs when watches are active

#### `buildEngineWasm(variant: string, renderer: string): Promise<void>` (line 1197)
```typescript
export async function buildEngineWasm(variant: string, renderer: string): Promise<void> {
  ensureAppleDeveloperDir();
  if (renderer !== "react" || process.env.SKIP_ENGINE_BUILD === "1") return;
  if (process.env.FORCE_ENGINE_BUILD !== "1") {
    // Checks existence of framework-surface.js, framework-editor_bg.wasm, flow_core_bg.wasm
    // Returns early if all exist (reuse path)
  }
  // Runs wasm build for: framework-surface, framework-editor, framework-surface-board-2d
  // Then conditionally runs flow-core wasm build
  // Finally builds any `engines` crates declared in variant's [[package.metadata.semio.playground]]
}
```
- No-op for wgpu renderer
- Reuses existing `pkg/` files unless FORCE_ENGINE_BUILD=1
- Builds unconditional framework engines first, then variant-specific engines
- Budgeted at `buildBudgetMs()` per inner crate (not shared with dev default)

### Imported Utility Functions
Imported from `../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`:

- **`runViteBunxDev`**: Spawns Vite dev server with port/env mapping
- **`spawnDaemon`**: Starts child process, returns handle for manual kill
- **`isDevPortInUse(host, port)`**: Checks if port is listening (returns boolean)
- **`describeDevPortOccupant(port)`**: Returns process description string
- **`daemonBudgetOpts()`**: Returns `{ budgetMs }` for child processes

### ParityScript - mkdir-lock Pattern (line 2726-2768)

**Function**: `prebuildParityPlugin(variant: string): Promise<void>`

**mkdir-lock pattern** (line 2731-2742):
```typescript
const lockRoot = resolve(process.env.PARITY_CARGO_TARGET_DIR ?? parityOutDir());
const lockPath = join(lockRoot, ".semio-parity-prebuild-lock");
mkdirSync(lockRoot, { recursive: true });
const lockDeadline = Date.now() + PARITY_DEV_SERVER_BOOT_BUDGET_MS;
while (true) {
  try {
    mkdirSync(lockPath);  // Atomic: fails with EEXIST if already held
    break;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
    if (Date.now() >= lockDeadline) throw new Error(`plugin prebuild lock for ${variant} exceeded ${PARITY_DEV_SERVER_BOOT_BUDGET_MS}ms (${lockPath})`);
    await Bun.sleep(500);
  }
}
try {
  // ... run prebuild ...
} finally {
  rmSync(lockPath, { recursive: true, force: true });
}
```
- Uses atomic `mkdir` to hold lock directory
- Polling loop with 500ms sleep
- Deadline enforced at `PARITY_DEV_SERVER_BOOT_BUDGET_MS` (config-dependent)
- Cleanup in finally block

### SpaceE2eVerify Region (line 1861-2034)

**Playwright imports and browser launch** (line 1947-1949):
```typescript
async function runStudioE2eVerify(baseUrl: string, timeoutMs: number): Promise<void> {
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
```
- Dynamic import of `playwright` (only when e2e runs)
- Launches with `headless: true` (default)

**Selector conventions** (10 real assertions):
```typescript
// Line 1956: Loaded state check
await page.waitForFunction(() => /home/i.test(document.body.innerText) && /Demo Studio|New Studio/i.test(document.body.innerText) && document.querySelectorAll("#root *").length > 150, { timeout: 120_000 });

// Line 1961: Home studios vfs assertion
spaceE2eAssert(/Demo Studio|Studios/i.test(booted.text), "home studios vfs should list seeded studio");

// Line 1966: Studio URI assertion
spaceE2eAssert(pathAfterCreate.startsWith("/spaces/"), "studio uri should be under /spaces/");

// Line 1972: Node graph host assertion
spaceE2eAssert((await page.locator(".semio-node-graph-host").count()) > 0, "node graph host should render");

// Line 1973: Text editor host assertion
spaceE2eAssert((await page.locator(".semio-text-editor-host").count()) > 0, "compiled dag editor should render");

// Line 1971: Missing window assertion
spaceE2eAssert(!/Missing window:/i.test(bodyText), "all studio windows should render");

// Line 1994: Undo command palette assertion
spaceE2eAssert((await page.locator("[cmdk-item]").filter({ hasText: "Undo" }).count()) > 0, "undo should be in command palette");

// Line 2002: Checkpoint command assertion
spaceE2eAssert((await page.locator("[cmdk-item]").filter({ hasText: /checkpoint/i }).count()) > 0, "checkpoint command should be in command palette");

// Line 2009: Find palette assertion
spaceE2eAssert((await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0, "find palette should open");

// Line 2029: Page errors assertion
spaceE2eAssert(criticalErrors.length === 0, `page errors: ${criticalErrors.join(" | ")}`);
```

**Helper functions**:
- `spaceE2eAssert(condition, message)`: Throws if condition false (line 1865)
- `waitForStudioE2eCondition(page, predicate, label, deadline)`: Polls until predicate true (line 1873)
- `openStudioE2e(page, deadline)`: Triggers Meta+N, waits for `/spaces/` URI (line 1886)
- `expandStudioE2eWorkflowEngagement(page)`: Opens workflow panel (line 1908)
- `spawnStudioE2eDrawFromEngagement(page)`: Tests engagement rail spawn (line 1914)
- `openStudioE2eCommandPalette(page)`: Triggers Meta+P (line 1923)
- `spawnStudioE2eDrawFromPalette(page)`: Tests palette-based spawn (line 1930)

**PASS reporting** (line 2032):
```typescript
console.log("PASS: S studio end-to-end workflows verified");
```

### VerifyScript (line 2036-2054)
- **e2e subcommand** (line 2041): Runs `runStudioE2eVerify` only
- **Default flow**: Runs cargo test on all plugins, then react vitest, then e2e, then plugin capability lint
- **Output**: `[DEBUG] s studio verify passed (${studioUrl})`

---

## 2. Vite Config (`⚙️vite.config.ts`)

**Location**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts`

### Define/Env Passthrough (line 143-147)
```typescript
define: {
  "import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? DEFAULT_HOST_VARIANT),
  "import.meta.env.VITE_SEMIO_RENDERER": JSON.stringify(renderer),
  "import.meta.env.VITE_SEMIO_BRAND": JSON.stringify(brand?.id ?? ""),
},
```
- Passes runtime env vars as compile-time constants
- Only `VITE_SEMIO_*` vars are explicitly defined (hardcoded list)
- Source: process.env at config eval time (not runtime)
- Renderers uses `process.env.SEMIO_RENDERER ?? "react"` (line 20)
- Plugin uses `process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT` (line 21)

### Middleware Plugins (line 105-137)
```typescript
plugins: [
  ...semioHostHtmlVitePlugin(repoRoot, { title: "semio · os", entry: "./🟦️component.ts" }),
  semioEmojiIndexHtmlVitePlugin(playDir),
  playgroundFlowWasmDevStubPlugin(repoRoot),
  semioBackboneVitePlugin(),           // /backbone endpoint
  semioBlobVitePlugin(),               // /blob endpoint
  semioPluginHotSwapVitePlugin(),       // /plugin-modules hot-swap SSE
  semioExtensionStoreVitePlugin({ installRoot: installedExtensionsDir, repoRoot }),
  ...semioAssetsVitePlugin(repoRoot),
  // Plugin modules static-dir copy (production build)
  ...(pluginModuleDirNames ? pluginModuleDirNames.flatMap(...) : staticDirVitePlugin(...)),
  staticDirVitePlugin(repoRoot, { kind: "static-dir", route: "/extensions", ... }),
  // Brand-specific static assets
  ...(brand?.assetsDir ? staticDirVitePlugin(...) : []),
  ...semioBrandHtmlVitePlugins(repoRoot, brand),
  ...playgroundAssetVitePlugins(repoRoot, resolvedPlaygroundAssets, ...),
  ...(renderer === "wgpu" ? [tailwindcss()] : [react(), tailwindcss()]),
],
```

### Server Config (line 96-104)
```typescript
server: {
  port: Number(process.env.S_OS_PORT ?? 6066),
  strictPort: true,  // Fails if port taken
  fs: { allow: [repoRoot, pluginModulesDir, installedExtensionsDir, rendererModulesDir] },
  watch: {
    ignored: ["**/📇️registry/🤖️generated/**", "**/🤖️generated/**", "**/.vscode/launch.json"],
  },
},
```
- Host: **not explicitly set** (defaults to 127.0.0.1)
- Port: Read from `S_OS_PORT` env (default 6066)
- strictPort: **true** (no fallback to next available)
- FS allow list includes plugin-modules, extensions, renderer-modules dirs
- Ignores generated registry and launch.json to prevent watch loops

---

## 3. Plugin Registry (`📇️registry/`)

**Location**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/`

### PlaygroundEntry Type (line 177-191)
```typescript
export type PlaygroundEntry = {
  readonly variant: string;
  readonly pluginId: string;
  readonly cratePath: string;
  readonly app?: string;
  /** @emoji 🏷️ Shell brand id (see `framework/os/dev/brand`) this variant ships as. */
  readonly brand?: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
  readonly examples: readonly string[];
  /** @emoji 🔌️ Crate paths whose `wasm` build target must run for this playground variant. */
  readonly engines: readonly string[];
  /** @emoji 🗂️ Dev-time asset-serving needs for this variant. */
  readonly assets: readonly AssetSpecRow[];
};
```

### parsePlaygroundBlock Function (line 236-248)
Parses one `[[package.metadata.semio.playground]]` TOML block:
```typescript
function parsePlaygroundBlock(block: string, pluginId: string, cratePath: string): PlaygroundEntry | undefined {
  const variant = block.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
  if (!variant) return undefined;
  const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
  const brand = block.match(/^brand\s*=\s*"([^"]+)"/m)?.[1];
  const aliases = parseTomlStringArray(block, "aliases");
  const portsBlock = block.match(/^ports\s*=\s*\{([^}]*)\}/m)?.[1];
  const react = portsBlock?.match(/react\s*=\s*(\d+)/)?.[1];
  const wgpu = portsBlock?.match(/wgpu\s*=\s*(\d+)/)?.[1];
  if (!react || !wgpu) return undefined;
  const engines = parseTomlStringArray(block, "engines");
  return { variant, pluginId, cratePath, app, brand, aliases, ports: { react: Number(react), wgpu: Number(wgpu) }, examples: [], engines, assets: [] };
}
```
- Returns `undefined` if variant or ports missing
- Example TOML block:
  ```toml
  [[package.metadata.semio.playground]]
  variant = "cad"
  app = "cad"
  brand = "aggregator"
  aliases = ["drafting", "design"]
  ports = { react = 6000, wgpu = 6001 }
  engines = ["./crate/path"]
  ```

### emitPlaygroundsTypeScript Function (line 556-592)
```typescript
function emitPlaygroundsTypeScript(playgrounds: PlaygroundEntry[], defaultHostVariant: string): string {
  // Builds PLAYGROUND_BUILD_TARGETS array literal with each playground's props
  // Exports DefaultHostVariant constant
  // Includes PlaygroundAssetSpec type union and PlaygroundBuildTarget type
}
```
- Output file: `🤖️generated/🟦️playgrounds.ts`
- Contains: `PLAYGROUND_BUILD_TARGETS[]` array, `DEFAULT_HOST_VARIANT` constant
- Consumed by: dev script, vite.config.ts, launch.ts

### GenerateScript vs CheckScript (line 1685-1763)

**GenerateScript.run** (line 1685-1700):
- Calls `renderCatalogFiles(repoRoot)` to generate catalog in memory
- Writes all files to `🤖️generated/` directory
- Also calls `generateLaunchJson()` and writes `.vscode/launch.json`
- Logs plugin/playground/framework package counts

**CheckScript.run** (line 1706-1763):
- Renders catalog in memory (same as Generate)
- **Never writes** (verify-only)
- Byte-compares each file against on-disk artifacts
- Throws if stale; list stale files
- Validates playground registry (playground validation errors)
- Audits taxonomy tree per plugin area (warn if `legacy`/`mixed`, fail if `clean`)
- Checks package discovery problems
- Output: logs freshness status or exits with violations

---

## 4. Launch JSON Generation (`🖥️launch.ts`)

**Location**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🖥️launch.ts`

### DevLauncherEntry Type (line 33-42)
```typescript
type DevLauncherEntry = {
  readonly namePrefix: string;           // e.g., "🖥️s", "📐️cad"
  readonly order: number;                // Presentation order in 3_dev group
  readonly command: string;              // Launch command
  readonly reactEnv: Readonly<Record<string, string>>;
  readonly reactServerReadyAction: ServerReadyTemplate;
  readonly wgpuOrder?: number;           // Optional separate wgpu order
  readonly wgpuEnv?: Readonly<Record<string, string>>;
  readonly wgpuServerReadyAction?: ServerReadyTemplate;
};
```

### readSeed Function (line 49-58)
```typescript
function readSeed(repoRoot: string): { readonly skeleton: string; readonly devLaunchers: Readonly<Record<string, DevLauncherEntry>> } {
  const seedPath = join(repoRoot, SEED_REL_PATH);
  const raw = readFileSync(seedPath, "utf8");
  const markerIndex = raw.indexOf(DEV_LAUNCHERS_MARKER);
  if (markerIndex === -1) throw new Error(`🖥️launch.ts: seed file ${seedPath} is missing the devLaunchers marker`);
  const skeleton = `${raw.slice(0, markerIndex)}}\n`;
  const devLaunchersJsonText = raw.slice(markerIndex + DEV_LAUNCHERS_MARKER.length, raw.length - "\n}\n".length);
  const devLaunchers = JSON.parse(devLaunchersJsonText) as Record<string, DevLauncherEntry>;
  return { skeleton, devLaunchers };
}
```
**Marker contract** (line 23-24):
```
  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated
  // output); see 🖥️launch.ts readSeed() for the exact split contract this marker line supports.
  "devLaunchers": 
```
- Seed file split at this exact marker line
- Everything before marker = skeleton (configurations array)
- Everything after marker = devLaunchers JSON table

### renderEntry Function (line 74-89)
```typescript
function renderEntry(name: string, launcher: DevLauncherEntry, renderer: "react" | "wgpu", port: number): object {
  const env = renderer === "react" ? launcher.reactEnv : launcher.wgpuEnv;
  const sra = renderer === "react" ? launcher.reactServerReadyAction : launcher.wgpuServerReadyAction;
  const order = renderer === "react" ? launcher.order : launcher.wgpuOrder;
  if (!env || !sra || order === undefined) throw new Error(`🖥️launch.ts: devLauncher "${name}" is missing ${renderer} fields`);
  return {
    name,
    type: "node-terminal",
    request: "launch",
    command: launcher.command,
    cwd: "${workspaceFolder}",
    env: renderEnv(env, port),
    presentation: { group: "3_dev", order },
    serverReadyAction: renderServerReadyAction(sra, port),
  };
}
```

### generateLaunchJson Function (line 105-125)
```typescript
export function generateLaunchJson(repoRoot: string, playgrounds: readonly PlaygroundEntry[]): string {
  const { skeleton, devLaunchers } = readSeed(repoRoot);
  const byVariant = new Map(playgrounds.map((entry) => [entry.variant, entry]));
  let out = skeleton;
  for (const [variant, launcher] of Object.entries(devLaunchers)) {
    const playground = byVariant.get(variant);
    if (!playground) throw new Error(`🖥️launch.ts: devLaunchers["${variant}"] has no matching playground registry entry (renamed or removed plugin — update the seed)`);
    const reactPlaceholder = JSON.stringify(`@generated:${variant}:react`);
    if (!out.includes(reactPlaceholder)) throw new Error(`🖥️launch.ts: seed is missing placeholder ${reactPlaceholder}`);
    const reactName = `🛠️dev${launcher.namePrefix}⚛️react`;
    out = out.replace(reactPlaceholder, reindent(JSON.stringify(renderEntry(reactName, launcher, "react", playground.ports.react), null, 2), 4));
    if (launcher.wgpuOrder !== undefined) {
      const wgpuPlaceholder = JSON.stringify(`@generated:${variant}:wgpu`);
      if (!out.includes(wgpuPlaceholder)) throw new Error(`🖥️launch.ts: seed is missing placeholder ${wgpuPlaceholder}`);
      const wgpuName = `🛠️dev${launcher.namePrefix}🧊️wgpu🌐️wasm`;
      out = out.replace(wgpuPlaceholder, reindent(JSON.stringify(renderEntry(wgpuName, launcher, "wgpu", playground.ports.wgpu), null, 2), 4));
    }
  }
  if (out.includes("@generated:")) throw new Error("🖥️launch.ts: an @generated placeholder was not resolved (devLaunchers table is missing an entry)");
  return out;
}
```
- Replaces `"@generated:<variant>:<renderer>"` placeholders with full config objects
- Ports come from playground registry (from Cargo.toml)
- Names are built as `🛠️dev + namePrefix + renderer emoji`
- Reindents JSON to 4-space depth (seed's `configurations` array level)

---

## 5. Launch Seed File (`.vscode/🧩️launch.seed.jsonc`)

**Location**: `/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc`

### os-hub Entry (line 1509-1523)
```jsonc
{
  "name": "🛠️dev🗄️os-hub",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run os-hub:dev",
  "cwd": "${workspaceFolder}",
  "env": {
    "OS_HUB_PORT": "8787",
    "OS_HUB_DATA": "${workspaceFolder}/.semio/hub-dev/"
  },
  "presentation": {
    "group": "3_dev",
    "order": 387
  }
}
```

### Generated Placeholder Examples
- Line 114: `"@generated:cad:react",`
- Line 115: `"@generated:cad:wgpu",`
- Line 403-404: `"@generated:dag:react",` / `"@generated:dag:wgpu",`

### Compounds Array (line 2264-2273)
```jsonc
"compounds": [
  {
    "name": "🧭️compound🖥️s⚛️react🗄️os-hub",
    "configurations": ["🛠️dev🗄️os-hub", "🛠️dev🖥️s⚛️react"],
    "stopAll": true,
    "presentation": {
      "group": "3_dev",
      "order": 386.15
    }
  }
]
```

### devLaunchers Entry Example (line 2867-2900)
```jsonc
"s": {
  "namePrefix": "🖥️s",
  "order": 386.2,
  "command": "bun nx run @semio-tech/framework-os-dev:dev",
  "reactEnv": {
    "S_OS_PORT": "{PORT}",
    "SEMIO_PLUGIN": "s",
    "SEMIO_RENDERER": "react"
  },
  "reactServerReadyAction": {
    "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):{PORT})",
    "uriFormat": "%s"
  },
  "wgpuOrder": 386,
  "wgpuEnv": {
    "S_OS_PORT": "{PORT}",
    "SEMIO_PLUGIN": "s",
    "SEMIO_RENDERER": "wgpu"
  },
  "wgpuServerReadyAction": {
    "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):{PORT})",
    "uriFormat": "%s"
  }
}
```

### Presentation Group & Order Conventions
**Groups** (by numerical prefix):
- `1_keyboard`: order 10-20 (gemini, kiro)
- `2_mouse`: order 10-30 (f3d, gitkraken, mcpinspector)
- `3_dev`: order 1-387.x (dashboard, variants, os-hub, etc.)

**Order ranges** (3_dev group):
- 1-1.3: Dashboard & daemon (order 1, 1.1, 1.2, 1.3)
- 10-20: cad native/concrete fixture variants
- 170-170.1: animate (react/wgpu pair)
- 386-387.x: studio shell (`s`) and os-hub compound

---

## 6. Root Script (`📜️script.ts`)

**Location**: `/Users/ueli/Documents/semio/📜️script.ts`

### Verify Gate Steps (line 772-886)
Executed in order by `bun ./📜️script.ts verify gate`:

1. **Line 777-778**: `dependency-cruiser boundaries` — Composes framework, s, hub, mit-bestand
2. **Line 779-781**: `generated catalog freshness` — `nx run @semio-tech/plugin-registry:check`
3. **Line 782-785**: `region/host-contract script lints` — framework-renderer-react:lint, plugin lint, ui-styling-tokens:check-no-px
4. **Line 786-787**: `framework ts-rs binding freshness` — `@semio-tech/framework-rs:check`
5. **Line 788-789**: `ui locale/terminology axes freshness` — `@semio-tech/ui-rs:check`
6. **Line 790-791**: `chrome i18n literal scan` — `@semio-tech/ui-react:check-chrome-i18n`
7. **Line 792-793**: `leveled test target coverage` — `checkLeveledTestTargets()` (all project.json have test-quick/long/exhaustive)
8. **Line 794-795**: `storybook scope freshness` — `checkStorybookFreshness()` (all StoryScope paths exist)
9. **Line 796-808**: `OS exclusive state authority policies` — Breach checks for state authority + document app shape
10. **Line 809-820**: `standards/subsets vocabulary` — Checks coverage + vocabulary breaches (high priority only)
11. **Line 822-830**: `handcrafted grammar P3/M4 policies` — Grammar spec breaches
12. **Line 832-840**: `artifact-schema facet policies` — Schema breaches
13. **Line 842-850**: `app-schema facet policies` — App schema breaches
14. **Line 852-867**: `dissolve-core / plugin-root policies` — Banned names, emoji prefix, shape, builder, APA, inference family (high priority)
15. **Line 869-876**: `window capability taxonomy` — Window completeness, mode completeness
16. **Line 878-885**: `dsl fixture laws` — `test-quick` for DSL crates
17. **Line 886**: Logs `[verify] gate passed.`

### Policy Functions
Multiple `policy*Breaches()` functions exist but are NOT part of public CLI — they're internal to VerifyScript. Example: `policyOsStateAuthorityBreaches(root)` returns `BreachRecord[]`.

### Setup Command (SetupScript, line 215-370)
Playwright browser installation occurs in `SetupScript.run()`:
```typescript
const browsersPath = join(this.root, "node_modules", ".cache", "ms-playwright");
mkdirSync(browsersPath, { recursive: true });
console.log("[setup] Playwright browsers…");
tryRun("bunx", ["playwright", "install", "--with-deps", "chromium"], {
  env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: browsersPath },
});
```
- Sets `PLAYWRIGHT_BROWSERS_PATH` env before install
- Installs chromium with deps (`--with-deps` flag)
- Stores in `node_modules/.cache/ms-playwright` by default

---

## 7. Playwright & Browser Setup

### Playwright Version
`package.json` (root):
```json
"@playwright/test": "^1.57.0",
"playwright": "^1.57.0",
```

### PLAYWRIGHT_BROWSERS_PATH Environment Variable
- **Setup location**: `/Users/ueli/Documents/semio/📜️script.ts` line ~340
- **Path set to**: `${root}/node_modules/.cache/ms-playwright`
- **Used in**: Both SetupScript (browser install) and test execution
- **Zero-touch**: Automatically downloaded on first run if missing

### Browser Launch Default
**In dev script** (line 1949):
```typescript
const browser = await chromium.launch({ headless: true });
```
- Launches with `headless: true` (no UI, automated)
- Uses installed browsers from `PLAYWRIGHT_BROWSERS_PATH`

---

## 8. NX Project Configuration

| Project Name | Directory | Targets |
|---|---|---|
| `@semio-tech/framework-os-dev` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev` | dev, build, test, test-quick, test-long, test-exhaustive, verify, plugin, parity, layer-lint, index-lint, host-handle-lint |
| `@semio-tech/framework-os` | `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript` | test, test-quick, test-long, test-exhaustive |
| `@semio-tech/framework-renderer-react` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react` | lint, test, test-quick, test-long, test-exhaustive |
| `@semio-tech/plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry` | check, generate, new |
| `os-hub` | `🌎️hub/📦️packages/🦀️rust` | build, dev, setup, test, test-quick, test-long, test-exhaustive |
| `@semio-tech/ui-react` | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react` | build, check-chrome-i18n, check-ui-primitives, dev, lint, test, test-quick, test-long, test-exhaustive, typecheck |
| `@semio-tech/space-plugin` | `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust` | test, test-quick, test-long, test-exhaustive |

---

## 9. Verify Gate Baseline (W0)

**Run command**: `bun ./📜️script.ts verify gate`
**Baseline file**: `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/🧪️w0-gate-baseline.txt`

### Gate Failure Summary
**Status**: FAILED at step 1 (dependency-cruiser)
- **Violations**: 828 total (651 errors, 177 warnings)
- **Exit code**: 139 (segmentation fault or out-of-memory)
- **Error**: `bunx dependency-cruiser compose 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .dependency-cruiser.cjs --output-type err exited with status 139`

**No subsequent gate steps executed** — gate short-circuits on first failure.

---

## Report Metadata
- **Prepared**: 2026-08-16
- **Baseline captured**: Yes (🧪️w0-gate-baseline.txt)
- **Implementation readiness**: Full context available for workers to add `users` dimension or other modifications without re-reading this file
