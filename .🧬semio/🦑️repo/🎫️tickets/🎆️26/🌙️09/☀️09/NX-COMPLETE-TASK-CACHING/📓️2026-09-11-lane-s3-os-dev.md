# Lane S3 — os/dev Nx Caching (2026-09-11)

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/**`, `.storybook/**`, and only
`playgroundPreparationTargets()` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`.

## 1. WGPU dev path now consumes Nx-cached plugin targets

**Design**: `playgroundPreparationTargets()` (`🟨️.mjs`) now generates a wgpu counterpart to every react
target it already generated, reusing the exact same dependency closure (session, web support, fonts,
selected plugins' `materialize-<profile>`) plus the wgpu renderer's own `wasm` producer looked up
dynamically (mirrors how the react `engines` set resolves crate paths to `<project>:wasm`, not a
hardcoded string):

- `prepare-<variant>-wgpu-<profile>` — `dependsOn`: `plugin-registry:session-<variant>`,
  `framework-plugin-web:support-<profile>`, `framework-os-infinite:fonts`,
  `@semio-tech/framework-renderer-wgpu:wasm`, every selected plugin's `<project>:materialize-<profile>`.
- `activate-<variant>-wgpu-<profile>` — `dependsOn: [prepare-...]`.
- `serve-<variant>-wgpu-<profile>` / `dev-<variant>-wgpu-<profile>` — continuous, `dependsOn: [activate-...]`,
  command `bun ./📜️script.ts dev <variant> served` with `env.SEMIO_RENDERER=wgpu` (there is no
  wgpu-specific `serve` CLI command — `served` skips the redundant re-activation the nx `dependsOn`
  chain already did).

`DevScript.run`'s wgpu branch (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`) no longer calls
`buildPlugins(plugin)` (raw serial `cargo rustc` + jco catalog build). It now calls
`activatePlaygroundRuntime(plugin, profile, "wgpu")`, which runs
`bun nx run @semio-tech/framework-os-dev:activate-<variant>-wgpu-<profile>` — the same delegation pattern
the react path already used (`activatePlaygroundRuntime` was generalized to take a `renderer` param,
default `"react"`, so the sole existing react call site is unchanged).

**The one asymmetry vs. react, and why**: react's Vite dev server serves plugin modules directly from
`browserModuleRoot(profile)` (`🔌️plugin/📦️packages/🟦️typescript/dist/<profile>/🔌️plugin-modules/`, the
Nx `materialize-<profile>` output), via a `MODULE_PLUGIN_ROUTE` alias in `⚙️vite.config.ts` (owned by S3,
unchanged). The wgpu native runner reads a *fixed filesystem path* instead —
`🧑‍💻dev/🔌️plugin-modules/` via `SEMIO_PLUGIN_MODULES` — a hardcoded constant in
`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`, which is **S4's file, not
touched**. So `PreparationScript` (`prepare <variant> <react|wgpu> <profile>`, extended to accept both
renderers) now calls a new `stageWgpuPluginModules()` helper only on the wgpu branch: it copies the
already-Nx-verified `browserModuleRoot(profile)` content (preview2 vendor, shard worker, each plugin's
transpiled+described module dir) into the live `pluginOutRoot`, and runs `publishBuiltExtension` for
extension-role targets — i.e. it reads from cache and republishes, it never runs cargo/jco itself.
`ActivationScript` branches early for `renderer === "wgpu"` (preparation IS activation for this renderer —
there is no HTTP activation receipt to publish, unlike react's Vite runtime root).

Cancellation/progress: unchanged — `runCmdStatus("bun", ["nx", "run", target], { budgetMs: buildBudgetMs() })`
is the same mechanism the react path already used; `nx run` itself streams cargo/jco progress.

**Verified** (`NX_DAEMON=false bunx nx show project @semio-tech/framework-os-dev --json`, before the repo-wide
graph outage described in §5 below):
- `prepare-generation3d-wgpu-dev.dependsOn` = `[session-generation3d, support-dev, fonts,
  framework-renderer-wgpu:wasm, flow-plugin:materialize-dev, …8 extension crates…,
  procedural-plugin:materialize-dev]` — exact selected-plugin closure, matching the react target's own
  list minus the react-only engine wasm producers (surface/editor/flow-core), plus the wgpu producer.
- `activate-generation3d-wgpu-dev.dependsOn = [prepare-generation3d-wgpu-dev]`.
- `serve-generation3d-wgpu-dev` / `dev-generation3d-wgpu-dev`: `continuous: true`, `dependsOn:
  [activate-generation3d-wgpu-dev]`, `options.command = "bun ./📜️script.ts dev generation3d served"`,
  `options.env = { SEMIO_RENDERER: "wgpu" }`. Confirmed for every generated variant (aggregator, animate,
  architect, … — the full playground catalog), not just generation3d.

`buildPlugins`/`buildPluginCatalog`/`materializePlugin` themselves are **not dead** — `buildPlugins` is
still the implementation behind the `plugin` CLI/nx target (raw catalog build, still needed for CI/build
verification) and `PluginWatchScript`'s hot-swap rebuild loop, so they were kept, not deleted.
`buildEngineWasm(plugin, "wgpu")` was already a no-op (`if (renderer !== "react" ...) return;`) — its
now-redundant call in the wgpu branch was removed as dead code.

## 2. Private cargo target roots removed

- `buildPluginCargo(target: PluginRegistryEntry)` — dropped the `ownedTargetRoot` parameter entirely.
  Always builds into `cargoTargetDirectory(repoRoot)` (the ONE shared, fine-grain-locked target dir from
  Wave A). No more `CARGO_BUILD_JOBS: "1"`, `CARGO_INCREMENTAL: "0"`, `RUSTC_WRAPPER: ""`,
  `RUSTC_WORKSPACE_WRAPPER: ""` overrides.
- `stageTestBrowserHostV1`'s only caller of the old `ownedTargetRoot` (`join(artifactRoot,
  "browser-host-wasi-target")`) required care: `ownedTestBrowserHostInput()` is a **security check** —
  it demands the fresh Space component be a bounded regular file physically inside the ticket-owned
  `artifactRoot`, not a symlink, so a build artifact from a shared/adversarial location can't be
  substituted. Since the artifact now lands in the shared cache dir (outside `artifactRoot`), the fix is
  to build once into the shared dir, then make one ticket-owned byte-for-byte copy
  (`writeFileSync(stagedSpacePath, readFileSync(builtSpace.artifact), { mode: 0o600 })`) and run the
  ownership check against *that* copy — preserving the exact security invariant with the private-target-dir
  build isolation removed. `materializeTestBrowserPluginV1` for `space` now transpiles from the staged
  copy (`spaceComponent.path`), not the shared-dir path, so the whole chain after the copy is exclusively
  ticket-owned, matching the pre-existing design intent.
- `prebuildParityPlugin` / `startParityDevServer` (parity harness, ~line 4130-4210): removed
  `PARITY_CARGO_TARGET_DIR` env passthrough from both `spawnDaemon` calls; the prebuild's own mkdir-mutex
  lock file now lives under `parityOutDir()` (the parity report dir) unconditionally instead of a private
  `CARGO_TARGET_DIR`-or-`parityOutDir()` fallback — the lock only dedupes the ~30-crate prebuild across
  concurrent parity runs, cargo's own output location is the one shared dir regardless.
- `rg -n "CARGO_TARGET_DIR|target-dir|RUSTC_WRAPPER|SCCACHE|CARGO_INCREMENTAL|ownedTargetRoot"` over the
  dev script now returns **zero matches** (was ~12 before).

## 3. Vite/Storybook cache dirs routed to the shared cache root

- `⚙️vite.config.ts`: `playgroundCacheDir` — `path.join(repoRoot, "node_modules/.vite-os-dev",
  "<plugin>-<renderer>")` → `repoCacheDirectory(repoRoot, "vite", "os-dev", "<plugin>-<renderer>")`.
- `.storybook/main.ts`: added `config.cacheDir = repoCacheDirectory(repoRootPath, "vite", "storybook");`
  inside `viteFinal` (previously unset, defaulting to Vite's own `node_modules/.vite`).
- Both import `repoCacheDirectory` from `…/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts` (only imports
  `node:path`, per the memory note about framework-core imports breaking vite config loading — this file
  has no such transitive risk, confirmed below).
- No other `.vite-*`/`node_modules/.vite` hits remain under `🧑‍💻dev/**` or `.storybook/**`
  (mit-bestand's own `.vite-mit-bestand-demonstrator` is out of S3's file scope — S5 owns
  `♻️mit-bestand/**`).

**Verified the config still loads**: `bun build --target=bun --no-bundle ⚙️vite.config.ts` → exit 0, and
`esbuild`'s own bundle of the config graph (see §5) resolves `⚡️caching/🟦️.ts` as a single leaf module
with no new transitive imports beyond `node:path` — the import does not change the config's load-time
behavior.

## 4. Playwright browser cache routed to the shared cache root

All three `PLAYWRIGHT_BROWSERS_PATH` assignments in `📜️script.ts` (`~3129`, `~4294`, `~5188`) changed
from `join(repoRoot, "node_modules", ".cache", "ms-playwright")` to
`repoCacheDirectory(repoRoot, "tools", "ms-playwright")`. `rg -n "ms-playwright"` confirms no remaining
`node_modules/.cache` literal in the file.

## 5. os-dev `📋️project.json` outputs

- `plugin` target: added `outputs: ["{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules"]`.
  **Deliberately left `cache` unset (still not cached)**: `🔌️plugin-modules/` is a shared, live,
  concurrently-mutated directory — a running `dev` session (react or wgpu) and hot-swap watch rebuilds all
  write into it directly, and different invocations of `plugin`/`SEMIO_PLUGIN_ONLY` write different
  subsets of the same path. Declaring `outputs` without `cache: true` is informational only (no nx-managed
  restore/wipe of that directory); enabling caching on it risks nx overwriting a peer's live dev output on
  a cache hit/miss and needs its own design (out of this lane's asked-for change).
- `build` target: **intentionally left undeclared.** Its actual output directory is not static —
  `⚙️vite.config.ts`'s `outDir` is `path.join(playDir, DISTRIBUTION_LAYOUT.directory)` normally but
  `path.join(repoRoot, brand.distDir)` for a brand with a custom `distDir` (e.g. the Aggregator brand
  builds to `♻️/aggregator/dist`, a location outside the project entirely). A single `outputs` entry can't
  represent both without either being wrong for the brand case or requiring inventing a brand-parametrized
  target family, which is a bigger change than "declare outputs" — deferred.
- Added explicit `"outputs": []` to the four already-`cache: true` check-style targets that were missing
  it (`canonical-bootstrap-folder-mirror-check`, `canonical-bootstrap-folder-mirror-process-check`,
  `browser-host-staging-check`, `closed-browser-component-factory-check`) — matches the existing
  `catalog-smoke` convention (`cache: true` + explicit empty `outputs` for a check that writes no files).
- `dev` target: already `cache: false, continuous: true` — no change needed (continuous targets stay
  uncached, per the task).
- `test`/`test-quick`/`test-long`/`test-exhaustive`/`verify`/`bench*`: left alone — enabling caching on
  vitest suites correctly requires verifying hermeticity per-suite, out of this lane's explicit scope
  (`plugin → outputs`, `build → its dist` were the two named examples; `build`'s case turned out
  non-static, see above).

## Verification commands run (with real results)

1. `node --check "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"` → OK (both before and
   after the `playgroundPreparationTargets` edit).
2. `bun build --target=bun --no-bundle` on every edited `.ts`: dev `📜️script.ts`, `⚙️vite.config.ts`,
   `.storybook/main.ts`, the `🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` test file — all exit 0.
3. `python3 -c "json.load(...)"` on `📋️project.json` — valid JSON before and after the edits.
4. `NX_DAEMON=false bunx nx show project @semio-tech/framework-os-dev --json` — ran successfully twice
   early in the session (captured the full generated target set, including every `prepare/activate/serve/
   dev-<variant>-wgpu-<profile>` target with correct `dependsOn`, shown in §1). **Later in the session this
   command started failing repo-wide**, unrelated to any file in this lane's scope:
   ```
   An error occurred while processing files for the @repo/emoji-project-json plugin
   Cannot find module '../../../🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts'
   Required from: 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts
   Source project does not exist: npm:@asamuzakjp/css-color
   ```
   That file (`📓️print/…/🖨️pipeline/🟦️.ts`) has an inconsistent relative import at its own line 273
   (`../../../🔨️modules/…`, 3 levels, vs. the correct 4 levels used at its own lines 1/12) — a pre-existing
   defect outside every Wave B lane's file ownership (not print, not touched by this session). Confirmed
   this is unrelated to my edits: it reproduced 3 times in a row, persisted across retries, and the print
   module was never touched by any of my edits (verified via `git diff --stat` against HEAD — only
   `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` and the files listed above differ). This blocks any
   *repo-wide* nx graph command (`nx show project`, `nx run <anything>`) for the remainder of the session —
   noted here rather than worked around, since fixing an unrelated file outside my scope risks
   contradicting whichever lane/peer owns it.
5. `bun ./📜️script.ts test quick --testNamePattern='ticket-owned browser host staging'` (direct command,
   bypassing nx since nx itself is broken repo-wide per #4) — **initially failed**: the suite has a
   source-text assertion pinning the exact `stageTestBrowserHostV1` call shape
   (`buildPluginCargo(space, join(artifactRoot, "browser-host-wasi-target"))` and the
   `CARGO_TARGET_DIR/CARGO_INCREMENTAL/RUSTC_WRAPPER` env literal), which my §2 change legitimately
   changed. Updated the test's assertions in
   `🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` to match the new staged-copy shape. Re-ran:
   **1 passed, 95 skipped** — clean.
6. `bun ./📜️script.ts test quick` (full quick suite) — **5 failures, all pre-existing and unrelated to
   this lane**, confirmed by isolation:
   - 3 failures in `../../🧪️tests/🧹️config/🟦️.ts` ("vite config module graph"): the fixture
     (`🧫️fixtures/⚙️config-graph.json`) bounds `⚙️vite.config.ts`'s real esbuild-bundled import graph at
     `maxModules: 40` and denies `🔍️discovery/🟦️.ts` from being reachable. **Isolated with a temporary
     revert**: with my `repoCacheDirectory` import removed, the graph is already 56 modules (vs. the
     40 max) and already includes the denied `🔍️discovery/🟦️.ts` — traced to
     `📚️library/🎮️playground/🟦️.ts` (a required module, not owned by this lane) itself importing
     `../🔍️discovery/🟦️.ts` directly. My one-file addition brings the count to 57 — a 1-module marginal
     effect on an already-broken fixture. Not caused by this lane; `📚️library/🎮️playground/**` is outside
     every Wave B lane's declared ownership.
   - 1 failure: `rewriteJcoComponentAssetUrls` version-propagation test — that function lives in
     `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` (S4's file), never touched by this lane.
   - 1 failure: catalog-smoke markdown golden mismatch (`` `s` `` vs `` `space` ``) — `catalogSmokeMarkdown`
     and the fixture (`🧫️fixtures/🔬️catalog-smoke.json`) are both untouched by this lane's diff
     (`git diff --stat` shows only the dev `📜️script.ts` hunks listed in §1/§2, none near
     `catalogSmokeMarkdown`'s definition at line ~2318, and the fixture file's git history predates this
     session). Confirmed present before and after every edit in this lane.
7. `PLAYWRIGHT_BROWSERS_PATH`/`repoCacheDirectory`/`CARGO_TARGET_DIR` `rg` sweeps — all clean (§2, §4).

## Left undone / handed off

- `build` target's `outputs` (brand-dependent path, see §5) — needs either per-brand target generation or
  an accepted "no cache on `build`" decision from the ticket owner.
- `plugin` target caching (shared live-directory semantics, see §5) — needs a design for how a cache
  hit/restore interacts with a concurrently-running `dev` session's hot-swap writes before it's safe to
  flip `cache: true`.
- The pre-existing print-pipeline import bug and the three pre-existing `test quick` failures (§ Verification
  #4/#6) are outside this lane's file ownership and were left as found — flagged here for whichever lane
  covers `📓️print/**`, `🔌️plugin/**`, and `📚️library/🎮️playground/**`/the catalog-smoke fixture.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`
- `.storybook/main.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` (`playgroundPreparationTargets()` only)
