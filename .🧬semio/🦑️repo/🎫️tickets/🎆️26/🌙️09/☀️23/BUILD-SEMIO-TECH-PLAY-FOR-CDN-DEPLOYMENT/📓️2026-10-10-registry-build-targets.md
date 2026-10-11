# 2026-10-10 Registry Build Targets From Discovered Crates

Base: `PLUGIN/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`, `REG/` = `PLUGIN/📇️registry/`.

## Rule

Build and describe targets come from the discovered plugin crates (manifests with a component kind and a deployment directory), whatever the state of their descriptor. Deployment (`generatePluginRegistryReport`, `🔌️plugins.json`, sessions, host loading) is unchanged and still refuses or withholds stale-channel and stale-dependency rows. No flag, no fallback.

## Change

- Schema first: `REG/🔎️discovery/🧬️schema/🔣️.json` gains `PluginBuildTargetV1` (source facts plus a mandatory `directoryName`, no descriptor fields). Binding `parsePluginBuildTargetV1` in `🧬️schema/🟦️.ts`. Language-agnostic vectors `target-from-source-declaration`, `target-missing-directory`, `target-cannot-fabricate-hashes` in `🔎️discovery/🧫️fixtures/🔣️.json` (AJV oracle and independent Node run both consume them).
- `REG/🔎️discovery/🟦️.ts`: new `discoverPluginBuildTargets(repoRoot, {packages, view})` (same crate set as the deployed registry, no descriptor read). `resolveRegistryPluginIdsForFilter` defaulted to the refusing `generatePluginRegistry`, so any filtered default call threw on a stale tree; it now defaults to `generateComponentSourceRegistry`.
- `REG/📖️catalog-view/🟦️.ts`: `CatalogSelection<T>`; `projectedHostPluginFilter` and `filterProjectedPluginRegistry` accept any source-shaped rows (deployed rows still fit), so host detection and closure no longer need a deployed row (a withheld host crate is still a host).
- `PLUGIN/🏗️build/📋️plan/🟦️.ts`: `discoverPluginBuildCatalog()` (discovered crates plus generated playground rows) and `resolvePluginBuildTargets(filterPlugin?, catalog?)` now selects from it (filter closure, `SEMIO_PLUGIN_ONLY`, empty-filter error). `resolveCatalogFilterPluginId` takes the catalog.
- `PLUGIN/🏗️build/🏃️execution/🟦️.ts` `preparePluginBuildTargets` and `PLUGIN/🏗️build/👁️watch/🟦️.ts` use it; the execution, watch, materialization, descriptor, installation and refresh signatures are retyped from `DeployedRegistryEntryV1` to `PluginBuildTargetV1`.
- `ensurePluginRegistry` still syncs staged descriptors only for admitted rows. That is deployment-side (it copies a descriptor into an already staged module) and should keep refusing a stale one; the build's own `materializePlugin` stages the fresh descriptor.
- `assertExtensionOutputsFresh` already exempts the extensions being rebuilt, so including stale extension crates in the targets also removes the same dead end for retained stale-shim outputs.

## Nx graph (read-only)

The Nx graph never read the deployed registry. `componentTargets` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`, ~1042) creates `describe`, `component-<profile>` and `materialize-<profile>` per Cargo manifest, and `playgroundSessionTargets` / `playgroundPreparationTargets` derive each variant's closure from manifests (`runtimeComponentClosure`). So the deadlock was only in the `buildPlugins` script path (dev runner, hub collab prebuild, demonstrator pipeline, watch).

`nx run @semio-tech/semio-tech-play:build --graph=` fails with `Invalid string length` in nx's `JSON.stringify` of the full project graph (independent of this change). To still count tasks without executing anything, a scratch `PATH`-wrapped `node --require` preload (kept in the cache folder, not in the repo) replaced nx's final `writeJsonFile` with the task ids and dependencies only:

- 375 tasks. 70 `describe`, 70 `component-dev`, 70 `component-release`, 70 `materialize-release`, 3 `build`, 5 `generate`, 9 `graph-generate`.
- `@semio-tech/cad-plugin` and `@semio-tech/stdio-image-plugin` both have `describe`, `component-dev`, `component-release`, `materialize-release`; the 4 cad extensions have `describe` and `component-dev`.
- 70 equals the discovered crates (3 deployed plus 67 withheld diagnostics rows).
- `session-<variant>` re-runs live registry generation (`SessionScript` uses `staleChannel: "exclude"` on current descriptors) after its `describe` dependencies, so the Nx path admits re-described plugins by itself.

Observation, not changed: `materialize-release` depends on `component-release`, not on the plugin's own `describe`, and `describe` (dev wasm) depends on `generate` (which runs before it, with stale rows withheld). Ordering of the staged descriptor against `describe` therefore depends on the session/prepare chain, not on a direct edge.

## Verification

- `bun test ./PLUGIN/📇️registry/🔎️discovery/🧪️tests/🟦️.ts` (with `SEMIO_TEST_ARTIFACT_DIR` in the play-fleet cache): 10 pass, 0 fail, 106 expects. New test "a withheld stale-channel row is still a build target while deployment keeps refusing it": deployed `["fresh"]`, targets `["dependent","fresh","stale"]` with directory names, filter closure `dependent -> [dependent, stale]`, `SEMIO_PLUGIN_ONLY`, unknown filter error, `generatePluginRegistry` still throws `stale-channel descriptors refused`.
- `REG/🧪️tests/📖️generated-projection/🟦️.ts`: 6 pass. `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/📖️projection/🟦️.ts`: 1 pass.
- Real repo probe: `discoverPluginBuildCatalog()` yields 70 targets = 3 deployed plus 67 diagnostics rows; `cad` and `stdio-image` are among them.
- Every changed build/refresh module imports under bun.
- Throwaway `tsc` over the changed files: no new errors from this change (remaining ones are the existing missing `frameworkOsPlaygroundDefaultPort`-style exports from `📚️library/📦️packages/🟦️typescript/🟦️.ts`, the `.mjs` declaration, and branded-string vector typings that existed before).

## Not green, not caused here

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts`: 2 failures, both on the empty untracked `REG/🧫️fixtures/🎮️playground-session/` and `REG/🧪️tests/🎮️playground-session/` folders (peer churn).
- `🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`: cannot load (`frameworkOsPlaygroundDefaultPort` export missing from the repo-lib module, a peer's in-flight change); its `buildPluginCatalog` fixtures were retyped to `PluginBuildTargetV1` but not run.
- `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎮️session/🟦️.ts`: `factory.runIf is not a function` (test harness).

## Expected next step for the coordinator

A `build-fresh` (or any `buildPlugins`) run now targets all 70 crates, so the 67 stale ones are cargo-built and re-described (rewriting their tracked owner-root `🔣️.json` and `🛂️.descriptor.semio` at channel 23). Whether those crates compile against the current SDK is untested. The deployed registry is regenerated by the next `generate`; `catalog-release` (refusing) must run after all describes.
