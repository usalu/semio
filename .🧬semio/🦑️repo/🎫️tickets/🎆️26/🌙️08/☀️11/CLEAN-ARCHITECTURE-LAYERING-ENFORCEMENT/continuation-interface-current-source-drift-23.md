# Interface Current Source Drift 23

Observed at 2026-10-08T14:33:27.382425+00:00.

The retained stage21 authority is historical. This audit preserves current files and does not restore proposed bytes. Existing source-cut changes require a fresh reconciled authority before whole execution.

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts

Current SHA256 `54b2818ea52de6b1e11599352ff6e789a354836eb6d3daaa14edb7ffa078dd1b`. Retained before `9b7e6ecd5ecd1e92cd72c2f5d8618e735c56c05f4ff11456f2c5e470b12b4899`. Retained proposal `072eb44532df9bb5b262687ddf70714ce2f153be6a13bde3a67e5b4600d03a2c`.

Current changes: 1 ranges; retained before 306 lines; current 306 lines.

```diff
--- retained-before

+++ current

@@ -271,5 +271,5 @@

  const result=Bun.spawnSync([process.execPath,script,...recipe.command],{cwd:resolve(root,pkg.directory),env:{...process.env,NX_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_ACTIVE:script,...(observation?{SEMIO_CARGO_PREPARATION_OBSERVATION:observation}:{})},stdout:"pipe",stderr:"inherit",timeout:30_000});
  if(result.stdout.byteLength)process.stderr.write(result.stdout);if(result.exitCode!==0)throw Error(`Cargo owner preparation failed: ${manifest} (${result.exitCode})`);
- if(observation){const captured=admitCargoPreparationObservationV1(JSON.parse(readFileSync(observation,"utf8")));if(captured.root!==root||captured.script!==script||JSON.stringify(captured.command)!==JSON.stringify(recipe.command)||captured.sources.find(input=>input.path===script)?.sha256!==before)throw Error("Preparation observation does not bind its original producer");assertCargoPreparationObservationCurrentV1(captured);assertCargoPreparationObservationCurrentV1({...captured,sources:program!.sources,inputs:program!.inputs,outputs:[]});const merged={...captured,sources:[...new Map([...program!.sources,...captured.sources].map(input=>[input.path,input])).values()],inputs:[...new Map([...program!.inputs,...captured.inputs].map(input=>[JSON.stringify([input.path,input.kind]),input])).values()]};assertCargoPreparationObservationCurrentV1(merged);recipes.push(merged);}
+ if(observation){const captured=admitCargoPreparationObservationV1(JSON.parse(readFileSync(observation,"utf8")));if(captured.root!==root||captured.script!==script||JSON.stringify(captured.command)!==JSON.stringify(recipe.command)||captured.sources.find(input=>input.path===script)?.sha256!==before)throw Error("Preparation observation does not bind its original producer");assertCargoPreparationObservationCurrentV1(captured);assertCargoPreparationObservationCurrentV1({...captured,sources:program!.sources,inputs:program!.inputs,outputs:[],resolutions:program!.resolutions});const merged={...captured,resolutions:program!.resolutions,sources:[...new Map([...program!.sources,...captured.sources].map(input=>[input.path,input])).values()],inputs:[...new Map([...program!.inputs,...captured.inputs].map(input=>[JSON.stringify([input.path,input.kind]),input])).values()]};assertCargoPreparationObservationCurrentV1(merged);recipes.push(merged);}
  diagnostic("recipe",manifest,recipeStart,1);
  }
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts

Current SHA256 `2925e404aa333f1f84fab97a4fdce9c831d692ccac9b150d51d520c7bf145cbc`. Retained before `1cbc9cecc774d2cd46f2a1f3021af1daa91b1a5ceaa4de6aff3eb909610097a5`. Retained proposal `1a667ea5b6684f5523dd077375fab98b5bf9cfcdd5fab6829478bb094df34c21`.

Current changes: 1 ranges; retained before 167 lines; current 176 lines.

```diff
--- retained-before

+++ current

@@ -123,4 +123,13 @@

     assert.equal(scope.hashes["🔣️.json"],new Bun.CryptoHasher("sha256").update(readFileSync(join(workspace,custodySchemaPath))).digest("hex"));
     assert.deepEqual(inventory.diagnostics.filter(row=>row.path.startsWith(dirname(custodySchemaPath))),[]);
+    const {cargoPreparationProgramSourcesV1}=await import("../../../🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts"),ts=require("typescript");
+    const importRoot=join(root,"program-imports");mkdirSync(importRoot,{recursive:true});
+    for(const row of fixture.programImports){
+      const entry=join(importRoot,"entry.ts"),source=`import value from ${row.literal};export default value;\n`;
+      writeFileSync(entry,source);writeFileSync(join(importRoot,row.path),"export default 7;\n");
+      const parsed=ts.createSourceFile(entry,source,ts.ScriptTarget.Latest,true),specifier=parsed.statements[0].moduleSpecifier.text;
+      assert.equal(specifier,row.path);assert.equal(Bun.resolveSync(specifier,importRoot),join(importRoot,row.path));
+      const program=cargoPreparationProgramSourcesV1(root,entry);assert.deepEqual(program.sources.map((input:any)=>input.path).sort(),[entry,join(importRoot,row.path)].sort());
+    }
     const pairPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts";
     const {withPreparedCargoDependencyPairV1}=await import(pathToFileURL(join(root,pairPath)).href);
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json

Current SHA256 `e04b27ae54cdd34d679a71e95c4668fb7a5222ebb3d02bcece3e431f4de17325`. Retained before `d22df546cc058149c426ae5137009eb1236b19ac318431371aa5ddc26641e8a6`. Retained proposal `d22df546cc058149c426ae5137009eb1236b19ac318431371aa5ddc26641e8a6`.

Current changes: 20 ranges; retained before 30139 lines; current 30317 lines.

```diff
--- retained-before

+++ current

@@ -8774,4 +8774,5 @@

         "🎞️animate",
         "🏛️architect",
+        "🏬️bim",
         "🧱️block",
         "📐️cad",
@@ -9622,4 +9623,5 @@

         "📼️avi",
         "🔊️wav",
+        "🏢️model",
         "🔋️model",
         "🔌️jack",
@@ -9746,5 +9748,8 @@

         "🚪️two-room-corridor",
         "🧱️wall-roof-facade-strip",
-        "🪠️pipes-3d"
+        "🪠️pipes-3d",
+        "🎬️demo",
+        "🏡️house",
+        "🏢️office"
       ],
       "source": "registry"
@@ -10609,5 +10614,12 @@

         "💡️inference",
         "🗃️apply-local-catalog-document",
-        "✂️text-splice"
+        "✂️text-splice",
+        "🌐️world-pointer-move",
+        "🏗️create-entity",
+        "🏷️rename-entity",
+        "👆️canvas-double-click",
+        "🛠️arm-utility",
+        "🩹️set-field",
+        "🪟️set-view"
       ],
       "source": "registry"
@@ -10677,5 +10689,6 @@

         "🔺️euler",
         "🧱️primitives",
-        "🧵️sew"
+        "🧵️sew",
+        "🩹️patches"
       ],
       "source": "registry"
@@ -10700,5 +10713,11 @@

         "🧵️session",
         "🧵️simulation-session",
-        "🧵️reconstruction-session"
+        "🧵️reconstruction-session",
+        "🎛️chrome",
+        "🔮️inference",
+        "🕹️interaction",
+        "🧩️entities",
+        "🧰️kit",
+        "🧵️gestures"
       ],
       "source": "registry"
@@ -11012,5 +11031,8 @@

         "🚰️pipes",
         "🧱️wall-roof-facade-strip",
-        "🪠️pipes-3d"
+        "🪠️pipes-3d",
+        "🏡️house",
+        "🏢️office",
+        "🧰️checks"
       ],
       "source": "registry"
@@ -11275,5 +11297,7 @@

         "🚦️throttle",
         "🎯️point-publication",
-        "🎯️selection"
+        "🎯️selection",
+        "🚪️svg",
+        "🛠️gestures"
       ],
       "source": "registry"
@@ -11358,5 +11382,20 @@

         "🧾️outline",
         "🪞️symmetry",
-        "🪟️window"
+        "🪟️window",
+        "⚠️diagnostics",
+        "🏠️house",
+        "🔗️wall-joins",
+        "🗺️plan-linework",
+        "🛋️spaces",
+        "🧊️element-solids",
+        "🧮️quantities",
+        "🧱️wall-layout",
+        "🪜️stair-runs",
+        "🪟️opening-frames",
+        "🏠️spaces",
+        "🏢️storey-levels",
+        "📦️bodies",
+        "🕸️model-graph",
+        "🪞️curtain-layout"
       ],
       "source": "registry"
@@ -11393,5 +11432,29 @@

       "memberNames": [
         "🎬️interaction-spec",
-        "📦️opc"
+        "📦️opc",
+        "🏠️house",
+        "✒️path",
+        "🎚️style",
+        "📏️projection",
+        "📐️sheet",
+        "🧱️codec",
+        "⬜️horizontal",
+        "🏗️frame",
+        "🏠️spaces",
+        "🏰️walls",
+        "📏️grids",
+        "🔬️projection",
+        "🧊️brep",
+        "🧬️data",
+        "🧭️frames",
... 192 additional diff lines remain in the current file and retained authority.
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts

Current SHA256 `dae7aed7dbdab51c7fa869c6dbfe3e9294ffa036f4180a2e5935b347dca78a4a`. Retained before `f7c0611a0b12a42d67a39e65f556da0f29146fac20ecb278917de0c63d284e53`. Retained proposal `885a8214f4c58c4aef36e8addbb66a6987fe3d20905c326e50247dee3d840538`.

Current changes: 4 ranges; retained before 5655 lines; current 5618 lines.

```diff
--- retained-before

+++ current

@@ -36,5 +36,4 @@

 //#endregion 🔌️Adapters
 
-import type { PlaygroundSelection as PlaygroundVariant } from "./🎮️playground/🧭️selection/🟦️.ts";
 
 import { loadFrameworkOsPlaygroundCatalog } from "./🎮️playground/🟦️.ts";
@@ -936,4 +935,5 @@

   "semio-hub-fem": { quick: 1_800_000 },
   "semio-hub-architect": { quick: 1_800_000 },
+  "semio-hub-bim": { quick: 1_800_000 },
   "semio-hub-process": { quick: 1_200_000 },
   "semio-hub-lowpoly": { quick: 900_000 },
@@ -1977,40 +1977,4 @@

 /** 🔌️ Process env for the absolute asset server URL base (native-bin wgpu route-relative fetches). */
 export const SEMIO_ASSET_BASE_URL_ENV = "SEMIO_ASSET_BASE_URL";
-
-/** 🔌️ Resolves the default dev port for a given catalog variant and renderer. */
-export function frameworkOsPlaygroundDefaultPort(catalog: readonly PlaygroundVariant[], variant: string, renderer: string): number {
-  const row = catalog.find((r) => r.variant === variant);
-  if (!row) return 6066;
-  return renderer === "wgpu" ? row.ports.wgpu : row.ports.react;
-}
-
-/** 🎯️ Resolves `bun ./📜️script.ts dev …` segments to a framework OS plugin filter via the catalog. */
-export function resolveFrameworkOsPlaygroundPlugin(catalog: readonly PlaygroundVariant[], segments: readonly string[]): { readonly plugin: string; readonly rest: readonly string[] } | null {
-  if (segments.length === 0) return null;
-  for (let len = segments.length; len >= 1; len--) {
-    const alias = segments.slice(0, len).join(" ");
-    const row = catalog.find((r) => r.variant === alias || r.aliases.includes(alias));
-    if (row) {
-      return { plugin: row.variant, rest: segments.slice(len) };
-    }
-  }
-  return null;
-}
-
-/** 🧊️ Env for `@semio-tech/framework-os-dev:dev` with a plugin filter and the renderer an
- * explicit `SEMIO_RENDERER` (launch row, `extra`) selects — wgpu only as the unset default. The value
- * picks the `dev-<variant>-<renderer>-<profile>` target in `resolveNxInvocation`, so a react launch
- * row reaches Vite and never the wgpu browser server. */
-export function frameworkOsPlaygroundDevEnv(catalog: readonly PlaygroundVariant[], plugin: string, extra: Partial<NodeJS.ProcessEnv> = {}, env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
-  const renderer = extra.SEMIO_RENDERER ?? env.SEMIO_RENDERER ?? "wgpu";
-  const defaultPort = frameworkOsPlaygroundDefaultPort(catalog, plugin, renderer);
-  const portVal = extra.S_OS_PORT || env.S_OS_PORT || String(defaultPort);
-  return devToolingEnv({
-    SEMIO_PLUGIN: plugin,
-    ...extra,
-    SEMIO_RENDERER: renderer,
-    S_OS_PORT: portVal,
-  });
-}
 //#endregion 🖥️FrameworkOsPlaygroundDev
 
@@ -5651,5 +5615,4 @@

  * `📚️library/🎮️playground/🟦️.ts` so port consumers need no taxonomy walk. */
 export * from "./🎮️playground/🟦️.ts";
-export { loadFrameworkOsPlaygroundSelections } from "./🎮️playground/🧭️selection/🟦️.ts";
 export { playgroundVariantsFromCrateManifest, registerPlaygroundSiteBuildCommands } from "./🎮️playground/🌐️site/🟦️.ts";
 //#endregion 🎮️Playground
```

## 📜️script.ts

Current SHA256 `8a503abec3694f7578a2b9db450aee24625b99d36d973e649b2c8e1c4f3a9cb1`. Retained before `55042b2441a8adb1531f47f55dba31202423358c8ef961378d7ece61ba9d260c`. Retained proposal `9e6c24125b32eb2254aa1fe1e3bb82f6148960f3d91a38f22fa9ef204eb6d5bc`.

Current changes: 56 ranges; retained before 26272 lines; current 26156 lines.

```diff
--- retained-before

+++ current

@@ -45,8 +45,9 @@

 import { microsecondsFromMilliseconds } from "./🧰️framework/🔨️modules/🧵️job/⏱️budget/🟨️.js";
 /**
- * 🧭️ Monorepo command router: `bun ./📜️script.ts <verb> [segments…]` (e.g. `📜️script.ts dev`, `📜️script.ts dev mcp`).
+ * 🧭️ Monorepo command router: `bun ./📜️script.ts <verb> [segments…]` (e.g. `📜️script.ts dev storybook`, `📜️script.ts dev mcp stdio os`).
  */
-import { dispatchOwnedScriptRoute, canonicalFilenameForKind, canonicalFilenamesForKind, canonicalPrimaryFilenameForKind, createTaxonomyPathMatcher, createFixedContractResolver, coverageDir, coverageEnabled, daemonBudgetOpts, devToolingEnv, discoverOwners, discoverPackages, discoverPackageProblems, dispatchPolicyArgv, dispatchSubcommand, defineLint, fixedContractFilename, fixedDirectoryContractIdsForPath, fixedFilenameContractIdsForPath, loadTaxonomy, resolveSchemaFacetKind, schemaFacetFormatEntries, semanticDirectoryKindId, enforceCoverageThreshold, frameworkOsPlaygroundDevEnv, getWorkspaceRoot, getRepoMetaDir, getMapCacheDir, getSemioRoot, HUB_DATA_DIR_NAME, MAP_CACHE_DIR_NAME, REPO_META_DIR_NAME, SPACE_DATA_DIR_NAME, goCoverageArgs, goLevelTestArgs, goProfileToLcov, loadFrameworkOsPlaygroundSelections, mergeLcov, orchestratorBudgetOpts, parseLcov, renderLcov, resolveCliBin, resolveMcpBin, resolveFrameworkOsPlaygroundPlugin, runCmd, runCmdStatus, runCanonicalGoBuild, runCanonicalGoTests, runProbe, runRepositoryTestCommand, spawnDaemon, summarizeCoverage, semioShipEnv, semioNxParallelFlag, installMicroCommitGitHooks, runCommit, runMicroCommit, runWorkspaceScriptMain, TechnologyLinter, tryRun, type BreachRecord, type PackageRole, type LcovFileRecord } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
+import { dispatchOwnedScriptRoute, canonicalFilenameForKind, canonicalFilenamesForKind, canonicalPrimaryFilenameForKind, createTaxonomyPathMatcher, createFixedContractResolver, coverageDir, coverageEnabled, daemonBudgetOpts, devToolingEnv, discoverOwners, discoverPackages, discoverPackageProblems, dispatchPolicyArgv, dispatchSubcommand, defineLint, fixedContractFilename, fixedDirectoryContractIdsForPath, fixedFilenameContractIdsForPath, loadTaxonomy, resolveSchemaFacetKind, schemaFacetFormatEntries, semanticDirectoryKindId, enforceCoverageThreshold, getWorkspaceRoot, getRepoMetaDir, getMapCacheDir, getSemioRoot, HUB_DATA_DIR_NAME, MAP_CACHE_DIR_NAME, REPO_META_DIR_NAME, SPACE_DATA_DIR_NAME, goCoverageArgs, goLevelTestArgs, goProfileToLcov, mergeLcov, orchestratorBudgetOpts, parseLcov, renderLcov, resolveCliBin, resolveMcpBin, runCmd, runCmdStatus, runCanonicalGoBuild, runCanonicalGoTests, runProbe, runRepositoryTestCommand, spawnDaemon, summarizeCoverage, semioShipEnv, semioNxParallelFlag, installMicroCommitGitHooks, runCommit, runMicroCommit, runWorkspaceScriptMain, TechnologyLinter, tryRun, type BreachRecord, type PackageRole, type LcovFileRecord } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
 import { Script, ScriptRouter } from "./🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
+import { checkDerivedConfig, writeDerivedConfig } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts";
 import { repoCacheDirectory } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
 import { canonicalArchitectureEnvironment } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
@@ -67,4 +68,6 @@

 import { policyListMutationDirs } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts";
 import { CleanScript } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts";
+import { AgentsScript } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🎮️command/🟦️.ts";
+import { AGENT_CLIENT_DECLARATION_MANIFEST, checkAgentClients, mcpServersOf, readDeclaredTools, type McpServer } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🟦️.ts";
 import { CleanMechanismNewScript } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts";
 import { policySnakeToCamel, type PolicySchemaLeafExtract } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🧱️contract/🟦️.ts";
@@ -112,50 +115,4 @@

 export { Script };
 
-function ensureFrameworkOsPlaygroundCatalog() {
-  const catalog = loadFrameworkOsPlaygroundSelections();
-  if (catalog.length === 0) throw new Error("No authored playground declarations were discovered");
-  return catalog;
-}
-
-function resolvePlaygroundDevApp(segments: string[]): { readonly app: string; readonly rest: string[] } | null {
-  const resolved = resolveFrameworkOsPlaygroundPlugin(ensureFrameworkOsPlaygroundCatalog(), segments);
-  if (!resolved) return null;
-  return { app: resolved.plugin, rest: [...resolved.rest] };
-}
-
-/** 🍽️ A bare `dev <variant>` under `SEMIO_RENDERER=react` runs the variant's whole Nx activation
- * chain — every selected plugin's `component-<profile>`/`materialize-<profile>`, the browser support
- * bundle, the guest fonts, the engine `wasm` producers, the generated playground session, then
- * `prepare` and `activate` — before Vite serves the receipt (os-dev `DevScript`).
- *
- * `served` opts out of that chain and serves whatever `dist/<profile>/🔌️plugin-modules/` and
- * `dist/runtime/<profile>/<variant>/activation/` already hold, for a boot that must not wait on (or
- * contend for) the shared Cargo lock. It also forces react, because
- * `frameworkOsPlaygroundDevEnv` defaults `SEMIO_RENDERER` to `wgpu` — a bare `dev s` otherwise builds
- * the whole catalog and hands off to `trunk serve`, never to Vite on `S_OS_PORT`.
- *
- * It is a command segment rather than an env var so it stays reachable from `launch.json`, which is
- * how every dev here starts things and which carries no `env` field.
- *
- * The renderer is selected by the *target*, never forced by a server script: `@semio-tech/framework-
- * os-dev:dev` is an alias that `resolveNxInvocation` (root `package.json`'s `nx` wrapper) rewrites to
- * `<dev|serve>-<variant>-<react|wgpu>-<dev|release>` from `SEMIO_PLUGIN`/`SEMIO_RENDERER`/
- * `SEMIO_BUILD_MODE`. Only the resolved wgpu target reaches the wgpu browser server, so its
- * `SEMIO_RENDERER = "wgpu"` assignment can never overrule a react launch row.
- *
- * `dev s` IS the all-plugins hub: `space` declares `[package.metadata.semio].host`, and every layer
- * keys off that one declaration — `runtimeComponentClosure` fans the Nx closure out to every
- * registered component, `buildPlaygroundSession`/`expandPluginRegistry` return the whole registry
- * unfiltered, and both renderers boot from that list. There is no separate multi-plugin variant to
- * select; a `dev multi` segment existed once and never resolved past this file. */
-function runFrameworkOsPlaygroundDev(plugin: string, rest: string[] = []): void {
-  const served = rest.includes("served");
-  runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:dev", "--", plugin, ...rest], {
-    cwd: WORKSPACE_ROOT,
-    env: frameworkOsPlaygroundDevEnv(ensureFrameworkOsPlaygroundCatalog(), plugin, served ? { SEMIO_RENDERER: "react" } : {}),
-    ...daemonBudgetOpts(),
-  });
-}
-
 //#region 🔖️NativeOsScript
 /** 🖥️Runs the native bootstrap under `repo/native/bootstrap`. */
@@ -288,35 +245,4 @@

 //#endregion 🔖️SetupScript
 
-//#region 🔖️StartScript
-export class StartScript extends Script {
-  run(_segments: string[]): void {
-    process.chdir(this.root);
-    if (!existsSync(join(this.root, "node_modules", "nx", "package.json"))) {
-      console.log("[start] node_modules incomplete — run `bun install` and `bun ./📜️script.ts setup`.");
-      return;
-    }
-
-    if (process.env.DEVCONTAINER === "true") {
-      console.log("[start] Devcontainer session ready.");
-      return;
-    }
-
-    if (process.env.S_LOCAL_ONLY !== "1" && process.env.S_LOCAL_ONLY !== "true") {
-      process.env.S_HUB_URL = process.env.S_HUB_URL || "http://127.0.0.1:8787";
-      process.env.OS_HUB_DATA = process.env.OS_HUB_DATA || join(this.root, ".🧬semio", "🌐hub", "hub-dev");
-      spawnDaemon("bun", ["nx", "run", "@semio-tech/framework-os-dev:local-hub"], {
-        cwd: this.root,
-        env: process.env,
-        stdio: "ignore",
-      });
-      console.log(`[start] local development hub owner launching at ${process.env.S_HUB_URL} (data ${process.env.OS_HUB_DATA}, log local-hub.log there); every \`dev s\` row signs in through its session broker`);
-    }
-    if (process.platform !== "win32" && process.platform !== "darwin" && process.platform !== "linux") {
-      console.log(`[start] Unsupported platform ${process.platform}.`);
-    }
-  }
-}
-//#endregion 🔖️StartScript
-
 //#region 🔖️DevScript
 export class DevScript extends Script {
@@ -330,22 +256,10 @@

       return;
     }
-    if (segments[0] === "s") {
-      runFrameworkOsPlaygroundDev("s", segments.slice(1));
-      return;
-    }
     if (segments[0] === "mcp") {
       this.runMcp(segments.slice(1));
       return;
     }
-    const playgroundApp = resolvePlaygroundDevApp(segments);
... 601 additional diff lines remain in the current file and retained authority.
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts

Current SHA256 `e600fb4ac078afbe3f2d4814d85b4b9b9389e511f005d8b9fdccb7575248109a`. Retained before `11a3afdb2d96a60db259bc42e0306682b60cc9913a68d8169855dfc4a394da81`. Retained proposal `f79fc13036f06b236c3f9098e24239e5fc0a4e95262ad6a13b357f350e6e8c3c`.

Current changes: 1 ranges; retained before 136 lines; current 136 lines.

```diff
--- retained-before

+++ current

@@ -114,5 +114,5 @@

   /** 🧟️Reaps stray bun/zsh/cargo/node/esbuild/rustc processes left over from crashed or abandoned sessions. */
   private runKillStrayProcesses(dry: boolean): void {
-    const removals = cleanKillStrayProcesses(dry);
+    const removals = cleanKillStrayProcesses(dry, repoCacheDirectory(this.root, "🎛️dashboard"));
     console.log(`[clean] stray-processes ${dry ? "dry-run" : "applied"} removals=${removals.length}`);
     for (const row of removals) console.log(`[clean] ${dry ? "would-kill" : "killed"} ${row.action} pid=${row.pid} ppid=${row.ppid} ${row.name}`);
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts

Current SHA256 `2a3b039b52482860c1c649632ec8185b811cb74118d856cb5341f95676db8237`. Retained before `790e5b7a38910c5d6da72a4286bef42dc367e4d5740f657fd159c3952da8a993`. Retained proposal `33dd5d2119bf3cc207a629ab1b7669bf82114686c571866aa326da55f3dda158`.

Current changes: 3 ranges; retained before 816 lines; current 809 lines.

```diff
--- retained-before

+++ current

@@ -791,17 +791,10 @@

   });
 
-  test("the focused policy command is registered from package through both launch authorities", () => {
+  test("the focused policy command is registered from package through its Nx target", () => {
     const libraryPackageRoot = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript");
     const project = JSON.parse(readFileSync(join(libraryPackageRoot, "📋️project.json"), "utf8"));
     const libraryPackage = JSON.parse(readFileSync(join(libraryPackageRoot, "package.json"), "utf8"));
-    const workspacePackage = JSON.parse(readFileSync(join(repoRoot, "package.json"), "utf8"));
-    const command = "bun nx run @semio-tech/repo-lib:test-package-body-policy";
     expect(project.targets["test-package-body-policy"].options.command).toBe("bun ./📜️script.ts test package-body-policy");
     expect(libraryPackage.scripts["test-package-body-policy"]).toBe("nx run @semio-tech/repo-lib:test-package-body-policy");
-    expect(workspacePackage.scripts["test:repo-lib:package-body-policy"]).toBe(command);
-    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
-      const launch = Bun.JSONC.parse(readFileSync(join(repoRoot, path), "utf8")) as { configurations: readonly { command?: string }[] };
-      expect(launch.configurations.filter((entry) => entry.command === command)).toHaveLength(1);
-    }
   });
 
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts

Current SHA256 `421106d066c999347dd09e9dc457cc78c9dc43a0e8de5115bba52a00a00c1ec3`. Retained before `a8d36ebcbff730565b6c8b5e351b99740236f103f3b9742895bf945cab6517cb`. Retained proposal `930a1e70d8f191b50a072cfcb39c09dc612ecd9c5edee5c3d315df69799abb11`.

Current changes: 2 ranges; retained before 7552 lines; current 7511 lines.

```diff
--- retained-before

+++ current

@@ -21,5 +21,5 @@

 import { playgroundStaticSiteBuildOptions } from "../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
 import { newScaffoldMutationTree } from "../../🏗️authoring/🧬️mutation-tree/🟦️.ts";
-import { PLAYGROUND_LOCKED_EXAMPLE_ENV, PLAYGROUND_PORTS, allPlaygroundReservedPorts, areaOf, budgetTimeoutHint, canReuseDevPort, capturedTestFailureDiagnostics, clearDiscoveryCache, computeWorkspaces, daemonBudgetOpts, defineLint, dependencyBoundaryBreachesForBundleDir, dependencyBoundaryBreachesForFile, describeDevPortOccupant, devServerUrl, devToolingEnv, diffWorkspaces, discoverBurndown, discoverOwners, discoverPackageProblems, discoverPackages, dispatchSubcommand, frameworkOsPlaygroundDevEnv, getWorkspaceRoot, gitSpawnEnv, goLevelTestArgs, isAdapterBoundaryFile, isDevPortInUse, loadFrameworkOsPlaygroundCatalog, loadTaxonomy, nextestArtifactLocation, orchestratorBudgetOpts, parseTsImportSpecs, playgroundDevPort, playgroundPlayViteDefine, policyDiscoveredAllowlist, readSemioMarker, resolveCargoPackageName, resolveCargoPackageNames, resolveDevPort, resolveFrameworkOsPlaygroundPlugin, resolveWorkspaceTaxonomyAuthority, resolveWorkspaceTaxonomyAuthorityFromDirectory, runCmd, runCmdStatus, runProbe, validateTaxonomy, vitestLevelArgs, wgpuDevPlayUrl, type FileLinter } from "../../📦️packages/🟦️typescript/🟦️.ts";
+import { PLAYGROUND_LOCKED_EXAMPLE_ENV, PLAYGROUND_PORTS, allPlaygroundReservedPorts, areaOf, budgetTimeoutHint, canReuseDevPort, capturedTestFailureDiagnostics, clearDiscoveryCache, computeWorkspaces, daemonBudgetOpts, defineLint, dependencyBoundaryBreachesForBundleDir, dependencyBoundaryBreachesForFile, describeDevPortOccupant, devServerUrl, devToolingEnv, diffWorkspaces, discoverBurndown, discoverOwners, discoverPackageProblems, discoverPackages, dispatchSubcommand, getWorkspaceRoot, gitSpawnEnv, goLevelTestArgs, isAdapterBoundaryFile, isDevPortInUse, loadTaxonomy, nextestArtifactLocation, orchestratorBudgetOpts, parseTsImportSpecs, playgroundDevPort, playgroundPlayViteDefine, policyDiscoveredAllowlist, readSemioMarker, resolveCargoPackageName, resolveCargoPackageNames, resolveDevPort, resolveWorkspaceTaxonomyAuthority, resolveWorkspaceTaxonomyAuthorityFromDirectory, runCmd, runCmdStatus, runProbe, validateTaxonomy, vitestLevelArgs, wgpuDevPlayUrl, type FileLinter } from "../../📦️packages/🟦️typescript/🟦️.ts";
 import { partitionNextestExecutionFilters } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
 import { BundleScript, ScriptRouter, findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
@@ -2159,45 +2159,4 @@

     expect(playgroundDevPort("dag")).toBe(6017);
     expect(allPlaygroundReservedPorts().size).toBeGreaterThanOrEqual(Object.keys(PLAYGROUND_PORTS).length);
-  });
-
-  test("resolveFrameworkOsPlaygroundPlugin maps CLI segments to OS plugin ids", () => {
-    const catalog = loadFrameworkOsPlaygroundCatalog();
-    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["dag"])).toEqual({ plugin: "dag", rest: [] });
-    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["gis", "2d"])).toEqual({ plugin: "gis2d", rest: [] });
-    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["procedural", "3d", "fixture", "hexagonal-column"])).toEqual({
-      plugin: "generation3d",
-      rest: ["fixture", "hexagonal-column"],
-    });
-    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["trinity", "jack"])).toEqual({ plugin: "trinity-jack", rest: [] });
-    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["unknown"])).toBeNull();
-    const resolvableSegments = catalog.reduce((sum, row) => sum + 1 + row.aliases.length, 0);
-    expect(resolvableSegments).toBeGreaterThan(20);
-  });
-
-  test("frameworkOsPlaygroundDevEnv defaults wgpu renderer and resolves catalog dev port", () => {
-    const catalog = loadFrameworkOsPlaygroundCatalog();
-    const dagEnv = frameworkOsPlaygroundDevEnv(catalog, "dag", {}, {});
-    expect(dagEnv.SEMIO_RENDERER).toBe("wgpu");
-    expect(dagEnv.SEMIO_PLUGIN).toBe("dag");
-    expect(dagEnv.S_OS_PORT).toBe("6117");
-
-    const cadEnv = frameworkOsPlaygroundDevEnv(catalog, "cad", {}, { S_OS_PORT: "6020" });
-    expect(cadEnv.SEMIO_RENDERER).toBe("wgpu");
-    expect(cadEnv.SEMIO_PLUGIN).toBe("cad");
-    expect(cadEnv.S_OS_PORT).toBe("6020");
-  });
-
-  test("frameworkOsPlaygroundDevEnv derives the port from an explicit renderer override", () => {
-    const catalog = loadFrameworkOsPlaygroundCatalog();
-    const row = catalog.find((entry) => entry.variant === "s")!;
-    const servedEnv = frameworkOsPlaygroundDevEnv(catalog, "s", { SEMIO_RENDERER: "react", SKIP_ENGINE_BUILD: "1" }, {});
-    expect(servedEnv.SEMIO_RENDERER).toBe("react");
-    expect(servedEnv.SKIP_ENGINE_BUILD).toBe("1");
-    expect(servedEnv.S_OS_PORT).toBe(String(row.ports.react));
-    expect(servedEnv.S_OS_PORT).not.toBe(String(row.ports.wgpu));
-    const pinnedEnv = frameworkOsPlaygroundDevEnv(catalog, "s", { SEMIO_RENDERER: "react", S_OS_PORT: "6074" }, { S_OS_PORT: "6099" });
-    expect(pinnedEnv.S_OS_PORT).toBe("6074");
-    const inheritedEnv = frameworkOsPlaygroundDevEnv(catalog, "s", { SEMIO_RENDERER: "react" }, { S_OS_PORT: "6099" });
-    expect(inheritedEnv.S_OS_PORT).toBe("6099");
   });
 
```

## Retirement and Laws Observation

Retained retirement rows: 8; current before mismatches: 0.
Original law paths: 41; currently existing: 41. Runtime acceptance has not been rerun.
