# One plugin-module staging root + loud freshness — 2026-09-12

Fixes the dev-tooling defect this ticket lost several hours to: **two (in fact three) plugin-module
staging trees existed and drifted silently**. After this change there is exactly ONE staging root per
profile, every producer writes it, every consumer reads it, and a served module that is behind its own
crate prints a `[stale]` line instead of looking like "the rebuild didn't take".

Sources for the defect: `📓️audit-build-restage-gates-2026-09-12.md` §1,
`📓️wgpu-playground-boot-2026-09-12.md` §2.1, `📓️extension-invoke-door-2026-09-12.md` §6.3.

---

## 1. What the trees actually were (corrected map)

The two prior audits disagreed about which tree the react serve mounts. Reading the code settles it —
and there were **three** roots, not two:

| # | path | written by | read by | fate |
|---|---|---|---|---|
| A | `🔌️plugin/📦️packages/🟦️typescript/dist/<profile>/🔌️plugin-modules/` (`browserModuleRoot`) | `@semio-tech/framework-plugin-web:support-<profile>`, every crate's `<project>:materialize-<profile>` | react Vite `pluginModulesDir` (**this is what 6018/6019 serve** — `⚙️vite.config.ts:33`), `PreparationScript`, `ActivationScript`, production distribution copy | **kept — now THE root** |
| B | `🧑‍💻dev/🔌️plugin-modules/` (`PLUGIN_MODULES_ROOT` / `pluginOutRoot`) | `@semio-tech/framework-os-dev:plugin` (`buildPluginCatalog`→`materializePlugin`), `publishShardWorker`, vendor/font ensure, size reports | trunk `copy-dir` in `🌐️.html:14` + `Trunk.toml [watch]`, the wgpu native entrypoint's **compiled-in fallback**, `scanBuiltPluginModules` default, registry `developmentCache` guard | **removed from every code path** |
| C | `🧑‍💻dev/📦️packages/🟦️typescript/dist/wgpu-modules/<profile>/<variant>/` (`wgpuActivatedModulesRoot`) | `stageWgpuPluginModules` (`prepare-<variant>-wgpu-<profile>`) | wgpu native runner via `SEMIO_PLUGIN_MODULES` | **removed — a third tree to drift** |

`📓️extension-invoke-door-2026-09-12.md` §6.3's claim that "the 6018 dev server serves
`🧑‍💻dev/🔌️plugin-modules/`" is wrong; `⚙️vite.config.ts` resolves `pluginModulesDir` under
`🔌️plugin/📦️packages/🟦️typescript/dist/<profile>/`, which matches
`📓️audit-build-restage-gates-2026-09-12.md` §1. The *symptom* it reported was real, though, and had two
causes: (a) trunk/native read tree B, which only `framework-os-dev:plugin` ever wrote, and (b) for
extensions, `materialize-*` never publishes into the runtime install root.

## 2. The one root

`🧑‍💻dev/♻️activation/🟦️.ts` now owns the answer, beside the two other "where do activated bytes live"
functions (`developmentRuntimeRoot`, and formerly `wgpuActivatedModulesRoot`):

```ts
export function pluginModulesRoot(profile: "dev" | "release"): string
export function pluginModulesRootIn(workspace: string, profile: "dev" | "release"): string
```

* `pluginModulesRoot` derives the repository root from `import.meta.url` — no workspace walk, so Vite's
  config bundler resolves it without pulling repository discovery in, and no caller can pick a root.
* `pluginModulesRootIn` is the SAME relative path against an explicit workspace, for the one sandboxed
  consumer that legitimately needs it (`productionBrowserSources`, driven against a throwaway workspace
  in its own tests). The repository-relative path is spelled exactly once, in `pluginModulesRootIn`.
* An unknown profile throws (`Select a plugin staging profile: dev or release`) rather than inventing a
  third root. `join`-based, so Windows separators come out of `node:path`, not a literal.

`browserModuleRoot`, `PLUGIN_MODULES_ROOT`, `wgpuActivatedModulesRoot` and `stageWgpuPluginModules` are
**deleted**. No symlink, no sync step.

### Consumers, all repointed

| consumer | before | after |
|---|---|---|
| react Vite mount (`⚙️vite.config.ts`) | inline `path.resolve(configDir, "../../../🔌️plugin/…/dist", profile, …)` | `pluginModulesRoot(profile)` |
| `PreparationScript` / `ActivationScript` | `browserModuleRoot(profile)` | `pluginModulesRoot(profile)` |
| wgpu trunk `copy-dir` (`🌐️.html`) + `Trunk.toml [watch]` | `../../../../../../🧑‍💻dev/🔌️plugin-modules` | `../../../../../../🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules` |
| wgpu native runner (`NativeRunScript`) | `wgpuActivatedModulesRoot(devPackageRoot, variant, profile)` | `pluginModulesRoot(profile)` |
| wgpu native binary (`⌨️native-entrypoint/🦀️.rs`) | `SEMIO_PLUGIN_MODULES` **or a compiled-in default** pointing at tree B | no fallback — refuses with exit 2 and the exact command to use |
| `scanBuiltPluginModules` / `semioPluginHotSwapVitePlugin` | defaulted to `PLUGIN_MODULES_ROOT` | `root` / `{ moduleRoot }` is **required** |
| production distribution copy (`🚚️distribution/🔌️components`) | second inline literal of the same path | `pluginModulesRootIn(workspace, profile)` |
| registry `createFreshCatalogBuildVerifier` "development cache" guard | tree B | the plugin-web `dist` tree |
| `pluginBuildOutputsPresent` (follower readiness) | inline tree-B literal | `pluginOutRoot` |

### Producers, all writing it

* `framework-plugin-web:support-<profile>` and every `<project>:materialize-<profile>` already wrote A.
* `@semio-tech/framework-os-dev:plugin` (`pluginOutRoot = pluginModulesRoot(devStagingProfile())`, where
  `devStagingProfile()` is `ship → release`, else `dev`, paired with `pluginWasmProfile()`).
* `materializePlugin` was rewritten to transpile/describe into a `mkdtemp` sibling and publish with
  **`stageArtifacts` under the same ownership key `MaterializeScript` uses** —
  `<cratePath>/Cargo.toml:browser:<profile>`, forward-slashed. Verified against disk: the staged
  `🌀️procedural/.nx-artifact.json` owner is `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:browser:dev`,
  byte-identical to what `componentArtifactOwner()` computes. Two producers of ONE tree, either able to
  replace the other's module directory, and no half-written module ever visible to a running server.
  `assertPluginOutputChildren` now admits `.nx-artifact.json`.
* `prepare-<variant>-wgpu-<profile>` no longer mirrors anything: `publishWgpuRuntimePrerequisites`
  publishes that lane's extensions out of the one root, ensures the guest typst font asset and writes
  the hot-swap marker. Its Nx `outputs` dropped from `{projectRoot}/dist/wgpu-modules/<profile>/<variant>`
  to `[]` (`🦑️repo/📚️library/🟨️.mjs` `playgroundPreparationTargets`), matching the react `prepare`.
* `framework-os-dev:plugin`'s Nx `outputs` dropped to `[]`: per-module ownership of the one root belongs
  to plugin-web's `support-*`/`materialize-*` targets, and two projects must not declare one cached tree.

## 3. Freshness is loud

Pure, fixture-drivable rule in `♻️activation/🟦️.ts` — ONE rule shared by the serve-start pass and the
activation-receipt watcher:

```ts
stagedModuleVerdict(facts) -> { pluginId, kind: "fresh" | "unstaged" | "unactivated" | "unpublished" | "source-newer", detail? }
stagedModuleReportLines(verdicts, command) -> readonly string[]   // one "[stale] …" line each
newestComponentSourceMtime(sourceRoot, maximumEntries?)           // bounded walk, output dirs skipped
stagedModuleMtime(moduleDirectory)                                // ignores .nx-artifact.json
```

Precedence is most-fundamental-first: `unstaged` → `unactivated` → `unpublished` (extensions) →
`source-newer` → `fresh`. `facts.activationTracked` says whether the lane is receipt-governed (react) or
reads the staging root directly (wgpu trunk/native); a receipt-less lane is only asked the questions it
can answer, instead of reporting every extension as unpublished against an invented empty receipt.

The source walk skips `dist`, `target`, `node_modules`, `pkg`, `.git` (declared in
`UNWATCHED_COMPONENT_SOURCE_DIRECTORIES`; walking them would make every crate permanently stale the
moment its own `dist/component-dev/*.wasm` lands) and is bounded by
`COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES = 20 000` per component. `stagedModuleMtime` ignores
`.nx-artifact.json` — it is written *after* every payload file, so counting it would hide the drift.

Wired into:

* **serve start** — `ServeScript.run` (react) and `DevScript`'s wgpu branch, via
  `reportServeStagedModuleFreshness` (never throws: a dev server that refuses to start over a stale
  module is worse than one that says which module is stale). Prints `[fresh] N staged components …`
  when clean.
* **the activation-receipt watcher** — `reportActivationFreshness` inside `semioActivationVitePlugin`,
  re-run on every receipt change, so a restage that lands while the server runs retires its own warning
  and one that never lands keeps saying so. `⚙️vite.config.ts` supplies `moduleRoot`, `installRoot` and
  the component list (every `PLUGIN_BUILD_TARGETS`/`EXTENSION_TARGETS` row with its owner tree
  `<cratePath>/../..`).

### `materialize-*` is no longer a silent no-op

Structurally: for **plugins** `materialize-<profile>` now writes the directory the dev server actually
serves, so the old "clean materialize, still two days stale, no warning" cannot happen.

For **extensions** the remaining gap is publication into the runtime install root, which only
`activate-*` does. Two answers, both loud:

1. `MaterializeScript` prints, on every extension materialize:
   `Extension <id> is staged but NOT published to any runtime install root — run: bun nx run @semio-tech/framework-os-dev:activate-<variant>-<react|wgpu>-<profile>`
   (and now names the output directory in its success line).
2. The freshness rule's `unpublished` verdict compares the installed `📥️install.json` `packageHash`
   against the receipt's `artifactSha256` and reports the mismatch with the same activate command.

### Live evidence (read-only, no server touched)

`🔍️staging-root-freshness-probe.ts` (this ticket folder) runs the real pass over the real
`generation3d` dev receipt:

```
[DEBUG] root …/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules
[DEBUG] receipt generation3d dev: 11 activated components, 11 checked
[DEBUG] freshness pass took 183 ms
[stale] flow: source-newer — staged 2026-09-12T06:22:47.414Z < ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🔣️.json 2026-09-12T06:22:51.574Z — run: bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev
[stale] procedural: source-newer — staged 2026-09-12T06:05:50.962Z < ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/🧪️tests/🔬️unit/🦀️.rs 2026-09-12T06:06:53.429Z — run: bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev
```

The dev-script half (`collectStagedModuleFacts` + `reportStagedModuleFreshness`) produces the identical
two lines. 183 ms for 11 components — cheap enough to run unconditionally at serve start.

Known conservative edge: a plugin whose owner root contains sibling component crates (e.g. `🌊️flow`,
whose owner tree holds `🧩️extensions/*`) is flagged when a sibling changes. It never misses, and it
matches `pluginWatchRoot`'s equally wide rebuild-watch root.

## 4. Launch seed

**No change.** No Nx target name changed (`plugin`, `prepare-*`, `activate-*`, `serve-*`, `dev-*`,
`native*`, `wasm`); only a target's `outputs` and internal paths moved. `.vscode/launch.json` and
`🧩️launch.seed.jsonc` name no module root, so nothing to regenerate.

## 5. Tests

New, fixture-driven, language-agnostic:

* `🧑‍💻dev/🧫️fixtures/🔌️staging-root.json` — the root contract (package-relative path, both profiles,
  rejected profiles, seven `{consumer, renderer, profile}` rows, retired roots/symbols, the source files
  that must not name them) and ten freshness cases with their expected verdict AND expected `[stale]`
  line, plus the declared output directories.
* `🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts` — 36 cases. Third-party oracles per the repo rule:
  **`whatwg-url`** (spec-complete, independent relative-reference resolution) rebuilds the expected root
  from the repository root for both profiles, and **`picomatch`** (chokidar's own glob engine) decides
  the output-directory exclusions a second time. Also: dev/release × react/wgpu × every declared consumer
  must resolve to exactly ONE path per profile; derived and workspace-explicit forms must agree; a
  sandboxed workspace must stay out of the repository; and a real temp tree exercises
  `stagedModuleMtime`/`newestComponentSourceMtime` including the `.nx-artifact.json` exclusion and the
  entry bound.

Updated: `🧩️wgpu-module-routes` (+2 cases: the plugin `copy-dir` must resolve to `pluginModulesRoot("dev")`;
a bundle pointing the plugin route anywhere else must be refused), `⚡️cache-contracts` (`prepare-*-wgpu-*`
outputs `[]`), `✅️catalog-complete` (dev-cache guard path), `🧪️ticket-owned-browser-host-staging`
(hot-swap plugin's `{ moduleRoot }` option).

### Results — every command run, verbatim

```
# new suite
bunx vitest run --config vitest.config.ts ../../🧪️tests/🔌️staging-root/🟦️.ts
  Test Files  1 passed (1)
        Tests  36 passed (36)

# dev package quick level (includes the new suite + 🧹️config + in-source 📜️script.ts)
bun ./📜️script.ts test quick
  Test Files  2 failed | 1 passed (3)
        Tests  5 failed | 120 passed | 28 skipped (153)

# wgpu bundle module routes
bunx vitest run --config vitest.config.ts 🧪️tests/🧩️wgpu-module-routes/🟦️.ts
  Test Files  1 passed (1)
        Tests  7 passed (7)

# repo caching / Nx target contracts (includes prepare-*-wgpu-* outputs)
SEMIO_TICKET_DIR=<this ticket> bun ./📜️script.ts test          # ⚡️caching
  [cache-contract] schema, graph ownership, source-byte discovery, native dependency oracles and
  materializer cancellation passed
  ("Component and activation contracts passed", "Editor and playground contracts passed")

# wgpu native binary (the fail-closed SEMIO_PLUGIN_MODULES change)
CARGO_INCREMENTAL=0 cargo check -p semio-framework-os-renderer-wgpu --bin semio-wgpu-native --features native-bin
  exit=0 — Finished `dev` profile [unoptimized] target(s) in 53.93s
  (24 warnings from the lib — real expansion, not an aborted check)

# long-level adapter harness (the hot-swap plugin option change)
SEMIO_TEST_LEVEL=long SEMIO_BUILD_INSPECTION_OUTPUT=1 bunx vitest run -t "routes encoded OS watcher and installation requests"
  Test Files  1 passed | 4 skipped (5)
        Tests  1 passed | 152 skipped (153)
```

### The 5 `test quick` failures are pre-existing — none touch this change

1–3. `🧹️config > vite config module graph` (denied module / module+byte bounds / Bun-oracle set).
The config graph is **57 modules, 1 518 928 bytes** against fixture bounds of 40 / 700 000, and
`🔍️discovery` is reachable via `⚙️vite.config.ts → 🖱️ui/🎨️styling/🟦️.ts → 🦑️repo/📚️library/🎮️playground/🟦️.ts`.
Both of those files are **unmodified vs HEAD** (`git diff --stat HEAD` empty) and neither is mine. My
config-graph delta is provably **zero modules**: the only imports I added to `🔌️vite-plugins.ts` are
`♻️activation/🟦️.ts` and `🔌️plugin/🏪️store/📥️store.ts`, and `git show HEAD:⚙️vite.config.ts` already
imports both directly (lines 14–15), so they were already in the closure.

4. `rewriteJcoComponentAssetUrls` — the production function lives in
`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`, which a **peer is editing right now** (+41 lines vs HEAD,
mtime 2026-09-12 07:29, before this session's edits). I did not touch that file.

5. `catalog smoke aggregation` — `🧫️fixtures/🔬️catalog-smoke.json` is internally inconsistent
(`input.shellPluginId: "space"`, `markdown` header `` `s` ``). Untouched since 2026-09-05 and unmodified
vs HEAD; `summarizeCatalogSmoke`/`catalogSmokeMarkdown` are far from every hunk of this change.

`✅️catalog-complete` also has 2 pre-existing failures (a missing
`🧫️fixtures/🧬️catalog-complete/🧬️schema/🔣️.json`, and a 17-vs-19 extension count drift from a peer's
registry work). The first aborts before reaching the dev-cache guard I edited, so that guard was verified
directly instead:
`createFreshCatalogBuildVerifier(root, <dist/dev/🔌️plugin-modules>)` →
`fresh catalog verification cannot use the development cache`. ✅

### Type checking

`bunx tsc` over the edited modules (scratch project at
`🗑️generated/staging-root/tsconfig.check.json`) reports **no new errors**. The one `defineConfig`
TS2769 on `⚙️vite.config.ts:25` was reproduced identically against `git show HEAD:⚙️vite.config.ts`, so
it is pre-existing. One real error this change introduced (`Dirent<string>[]` vs
`Dirent<NonSharedBuffer>[]` in the source walk) was fixed by routing both walks through a structurally
typed `readableDirectoryEntries` helper, which also keeps `♻️activation/🟦️.ts` on node builtins only —
it is bundled into `⚙️vite.config.ts` on every dev-server boot.

## 6. Running serves — what needs a restart

**Nothing was restarted and no staging tree was deleted.**

| serve | mount correctness | restart needed? |
|---|---|---|
| **6018 react** (screen `g3dreact`) | unchanged — it already served `dist/dev/🔌️plugin-modules`, which is now THE root | **Not for correctness.** Restart only to gain the serve-start `[stale]` report and the receipt-watcher freshness lines (both live in code Vite loads at config time). Restaging still reaches it without a restart, exactly as before. |
| **6019 react viewer** (`semio-g3d-viewer`) | same as 6018 | same as 6018 |
| **6118 wgpu trunk** (screen `g3dwgpu`) | **stale mount.** Its dist was built by `copy-dir` from the old `🧑‍💻dev/🔌️plugin-modules`; the bundle document and `Trunk.toml [watch]` now name the canonical root | **Yes — a trunk restart/rebuild is required** for it to copy the one staging root. Until then it keeps serving the bytes a previous session hand-synced into tree B. |
| wgpu **native** runner | `NativeRunScript` now exports `SEMIO_PLUGIN_MODULES=pluginModulesRoot(profile)` | n/a (per-run). A hand-launched `semio-wgpu-native` without that variable now exits 2 with the command to use, instead of silently reading a stale tree. |

## 7. Leftover, deliberately not touched

`🧑‍💻dev/🔌️plugin-modules/` (2.4 GB) still exists on disk. No code path reads or writes it any more, but
it is what the currently-running trunk dist was copied from, so deleting it is a separate step — and 178
`fixedFilenameContracts` entries in `🦑️repo/📚️library/🔣️taxonomy.json` still describe the JCO artifacts
under it (the canonical root lives under `dist/`, a policy-declared generated directory the taxonomy
walker never visits, so those contracts have no replacement there). **Follow-up: delete the directory and
those 178 contracts together, after 6118 has been restarted onto the new root.**

## 8. Files

Changed:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🌐️.html`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Trunk.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/laws.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️04/☀️27/NX-MONOREPO-SETUP/🔬️wasm-optimizer/🧩️descriptor/📜️script.ts` (old ticket probe kept runnable)

Created:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🔌️staging-root.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts`
- `<ticket>/🔍️staging-root-freshness-probe.ts`
- `<ticket>/🗑️generated/staging-root/` (tsc scratch project, cargo-check log)
