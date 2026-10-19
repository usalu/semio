# Result lw1: WP1 plugins to framework (neutral SDK only)

Executor `lw1`. No `package.json` / `bun.lock` edit, nothing under the frozen storage/revision paths, no cargo, no git-modifying command, no sub-agents. `AGENTS.md` untouched.

## 1. Totals (dependency-cruiser, plugin sources only)

Scoped run, same config as `result-lay0.md`: `lw1-tools/cruise.sh <out.json>` cruises `✏️s/🔌️plugins` with `--do-not-follow` on `🧰️framework|🌎️hub|♻️mit-bestand`, so every plugin-to-framework edge is judged but the framework is not walked (about 9 min under the current load). `lw1-tools/agg.ts` buckets the plugin-to-framework violations by target.

| WP1 scope (target outside `💻️os`) | before | after |
|---|---:|---:|
| plugins to `framework/products/repo` | **1,098** (ncpr 513, cross-technology 33, sdk-only 552) | **25** (ncpr 7, cross-technology 7, sdk-only 11) |
| plugins to `framework/modules` | **41** (ncpr 6, cross-technology 2, sdk-only 33) | **0** |
| plugins to other framework products and the ui build tooling | **1** | **7** |
| **WP1 total** | **1,140** (374 source files; the audit said 1,157) | **32** (13 source files) |

`plugins-framework-sdk-only` for the whole `✏️s/🔌️plugins` source area went 733 to 48 (the 34 left toward `💻️os` belong to WP2, executor lw2 is moving them concurrently). Nothing else in the plugin area changed rule counts because of this work.

## 2. Design (schema-first dependency inversion, no compat layer)

The neutral SDK `@semio-tech/framework` already re-exported the framework process modules; plugins reached around it into `🛍️products/🦑️repo` for process, cargo, inventory and test-platform APIs. The repository product now implements framework-owned ports and the plugins call the SDK only. This extends the pattern of `result-inv.md` (`defineScriptProviderV1`, designation `SEMIO_SCRIPT_PROVIDERS`, composition root `📚️library/🏃️process/🔌️providers`).

### 2.1 Loader
`🧰️framework/🔨️modules/🏃️process/🔌️providers/🟦️.ts`: new `loadScriptProviderSyncV1` (same designation, same checks, same module cache as the asynchronous loader, `createRequire`, the provider module must not use top-level await). Needed because the plugin scripts call synchronous helpers (`runCmd`, `runCargo`, `cargoTargetDirectory`, `devToolingEnv`, ...) at module top level and in sync `run()` methods; making them async would have meant rewriting about 250 call sites and every constant initializer. Exported through the process package facade.

### 2.2 Three new framework ports (all under `🧰️framework/🔨️modules/🏃️process/`)

| port (designation name) | framework module | product implementation | what plugins get |
|---|---|---|---|
| `repository-tooling` | `🛠️repository-tooling/🟦️.ts` | `📚️library/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts` | `runCmd`, `runProbe`, `runBun`, `runCargo`, `orchestratorBudgetOpts`, `devToolingEnv`, `repoCacheDirectory`, `cargoTargetDirectory`, `getRepoMetaDir`, `repoTestArtifactEnvironment`, `runVitest`, `runRepositoryCommand`, `runRepositoryTestCommand`, `runRepositoryMutationBridgeScriptV1`, `dispatchOwnedScriptRoute`, `generateCollectionCatalogV1` and the types `RunCmdOpts`, `RunProbeResult`, `RepositoryCommandPolicyV1`, `RepositoryCommandOptionsV1`, `RepositoryTestCommandOptionsV1`, `RepositoryMutationCollectorV1` |
| `artifact-package` | `🧩️artifact-package/🟦️.ts` | `📚️library/🏃️process/🧩️artifact-package/🔌️provider/🟦️.ts` | `runArtifactRustPackageMain`, `runArtifactRustPackagesTests`, `runArtifactRustTests`, `runArtifactTypeScriptPackageMain` and the types `ArtifactTwinSelfTest`, `ArtifactRustPackageOptions`, `ArtifactTypeScriptPackageOptions` |
| `cargo-workspace` | `🦀️cargo-workspace/🟦️.ts` | `📚️library/🏃️process/🦀️cargo-workspace/🔌️provider/🟦️.ts` | `observeCargoPreparationSourceV1/InputV1/OutputV1`, `completeCargoPreparationObservationV1` (inert when no repository is designated, like before when no capture was active), `prepareCargoCapabilityLinksV1`, `runCargoCapabilityContributionChecks`, `cargoRepositoryPackages`, `cargoWorkspaceForManifest`, `prepareCargoWorkspaceInvocation` (the provider supplies the preparation storage) |

The types that crossed the boundary moved to the framework (the product modules now import them): `RunCmdOpts` (was in `🏃️process/🟦️.ts`), `RunProbeResult`, `RepositoryCommandPolicyV1/OptionsV1`, `RepositoryTestCommandOptionsV1`, `RepositoryMutationCollectorV1`, `ArtifactTwinSelfTest`, the two package option types. `🎓️teaching/…/❓️quiz/🚀️deploy` imports the policy type from the SDK.

Designation: `📚️library/🏃️process/🔌️providers/🔣️.json` now lists 8 ports (was 5). The build graph already derives provider inputs from that declaration for every script whose closure reaches the loader (the wrappers import it), so the plugin targets keep their cache inputs.

### 2.3 No new manifest key: every new function rides an existing SDK entry
The public entries are glue files with an authored list; the manifest is frozen, so each new function was added to the glue of an entry that already exists (both the process package facade and the SDK glue):
`process/execution` (tooling, paths, command, route dispatch, catalog generation), `process/vitest` (`runVitest`), `process/test-execution` (`runRepositoryTestCommand`), `process/cargo-test` (artifact Rust package main plus the existing admission API `runAdmittedCargoPackageTestsV1`, `runAdmittedCargoCommandV1`, `runAdmittedExactCargoLawsV1`), `process/typescript-check` (`runArtifactTypeScriptPackageMain`), `process/native-build` (cargo workspace port), `process/wasm-build` (`buildAdmittedWasmWebV1`), `test/mutation/inventory/execution` (mutation bridge script), `compiler/syntax/module-tree` (`readSourceFamilySource`, `sliceModuleTreeSource` joined `readModuleTreeSource`), `test/adapter` (see 2.4).

### 2.4 Moved to the layer that owns it
`committedArtifact`, `currentPlatform` and the fixture URI pattern `TEST_INPUT_URI_RE` were pure helpers over the framework's `AdapterContext`/`PlatformId` that lived in the repo test platform (`@semio-tech/repo-test`). They now live in `🧰️framework/🔨️modules/🧪️test/🔌️adapter/🟦️.ts` and are exported by the existing `test/adapter` entry. The repo-test platform imports them from there (`fixture-inputs`, `adapter-context`, `subset-policy`, `execution-plan`, `reporting`, `host`, package index). 34 plugin mutation tests import them from `@semio-tech/framework/test/adapter`.

### 2.5 Call-site changes with a behaviour decision (all deliberate)
* `getWorkspaceRoot()` to `findWorkspaceRoot()` (framework routing, pure, same env precedence then marker walk).
* `runRepositoryCargoTests` to `runAdmittedCargoPackageTestsV1`, `runRepositoryCargoCommandV1` to `runAdmittedCargoCommandV1`, `runRepositoryExactCargoLaws` to `runAdmittedExactCargoLawsV1` (the provider now supplies `preparation`; the `preparation: configuredRepositoryCargoPreparationPolicyV1()` argument is gone from 8 scripts), `buildRepositoryWasmWebV1` to `buildAdmittedWasmWebV1`.
* `📦️packages/🦀️rust/📜️script.ts` of stdio semio no longer self-heals missing JavaScript dependencies (`prepareJavascriptDependencies` from the frozen storage partition). Dependency installation belongs to `dependencies sync`; a missing `protobufjs` now fails the test with the module error instead of installing inside a plugin script.
* `📜️script.ts` of note lost its inline check that the OS worker pumps exactly once per driven turn (`toolJobMountedDispatchOneTurnExact` on the OS worker source): the OS suite `🔬️tool-job-latest-wins` already runs that law on the unchanged source (`none` case) and on the `duplicate-pump` mutation.
* `flow … 🔬️source-contract` no longer calls the framework fixture `testBuiltTreeRetirementFixture()` (it is owned and already run by the ui contract/runtime/native-ownership suites, six call sites).
* stdio semio mesh mutation test: local `digest` over the framework hash entry `@semio-tech/framework/hash` (`sha256Hex(...).slice(0, 32)`, identical value).
* Self-tests of product laws that lived in plugins (`🧩️puzzle` 4, `🏗️fem` 2) receive the law as their first parameter (dependency injection); the callers that own the laws pass them: `✏️s/🧑‍💻dev/🏗️fem/…/📜️script.ts`, `✏️s/🧑‍💻dev/🧩️puzzle/…/📜️script.ts`, `🌎️hub/🧩️compositions/🧩️puzzle/…/📜️script.ts`. Verified by running the puzzle self-tests with the real laws (73 and 37 checks, same as before the signature change).

### 2.6 Specifier style
`📜️script.ts` and `🧪️tests/🎚️config` files import the SDK glue by relative path (existing convention, zero-touch, exempt from `no-cross-technology`); every other plugin file uses the bare `@semio-tech/framework/<key>` (resolves through the workspace link in `node_modules`, `Bun.resolveSync` checked).

## 3. Files
* Framework: 3 new port modules, loader + its facade, 8 process package facade files, 10 SDK glue files, 1 new glue file (`🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts`), `🧪️test/🔌️adapter` module.
* Product repo: 3 new providers, designation JSON, 5 modules that now import the moved types (`🏃️process/🟦️.ts`, `🔍️probe`, `🎛️owned-execution`, `🧪️testing/🎛️repository-command`, `🦀️cargo/🏭️inventory`, `⚡️caching/📦️artifacts/🦀️rust`, `…/🟦️typescript/📜️script.ts`), the repo test platform files of 2.4.
* Plugins: 339 files rewritten by `lw1-tools/rewrite.ts` (AST based, symbol to SDK entry table, renames applied), plus the hand edits of 2.5. Full lists: `lw1-tools/touched-plugins.txt` (339 files), `lw1-tools/touched-framework.txt` (31 files).

## 4. Verification (exact commands, outcomes)
All from `/Users/ueli/Documents/semio`, foreground.
* Scoped cruise before/after: `lw1-tools/cruise.sh <out>` then `bun lw1-tools/agg.ts <out>` (section 1). The final run is the third (after1 1,140 to 391 with the first rewrite, after2, after3 final).
* Strict tsc: `NODE_OPTIONS=--max-old-space-size=8192 bun lw1-tools/tc.ts lw1-tools/touched-framework.txt lw1-tools/touched-plugins.txt` over those 370 files (339 plugin + 31 framework/product/dev): 1 diagnostic, `Cannot find module 'brepjs'` in `🗄️stdio/…/🧊️brep/🏭️generator/📜️script.ts` (the package is not installed; present before the change).
* `bun test ./🧰️framework/🔨️modules/🏃️process/🔌️providers/🧪️tests/🟦️.ts`: 4 pass (new: synchronous loader shares the cache with the asynchronous loader, refuses missing function, missing export, undesignated port, missing designation).
* `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🧪️tests/🟦️.ts`: 4 pass (the repository designates exactly the 8 ports; every provider loads and implements every declared function; new: sync wrappers reach the repository implementation, `getRepoMetaDir`/`repoCacheDirectory`/`cargoTargetDirectory`/`devToolingEnv`/`runCmd` agree with the product functions, `runCmd` exit status 3 throws, observations are inert without designation).
* Wrapper to product wiring probes (ticket scratch, deleted): `runArtifactRustPackageMain`, `runArtifactTypeScriptPackageMain`, `runRepositoryMutationBridgeScriptV1` reach the product and stop at its own `Original parent Script process invocation required`; `committedArtifact` equals the previous digest and refusal text.
* Boundary laws: `SEMIO_TEST_ARTIFACT_DIR=<dir> bun test --timeout 300000 ./🧰️framework/📏️script-boundary/🧪️tests/🟦️.ts` 3 pass and `…/🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts` 3 pass; `inv-tools/inventory.ts` framework files importing a product module: 0.
* `SEMIO_TEST_ARTIFACT_DIR=<dir> bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧭️fixture-resolution/🟦️.ts`: 10 pass (uses `TEST_INPUT_URI_RE` from the SDK).
* Plugin suites that import the rewritten entries: layout export (16 pass), draw native-owner (22 pass), stdio registry editing unit (55 pass).
* Naming law `lay0-tools/law-run.ts` (`policyBannedNameStemBreaches`): 0 breaches.
* Not done: any plugin target through `bun repo run`. `bun repo run @semio-tech/vcs-js:test` stops in the dependency partition ("Locked workspace package lacks original admitted Source", `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:152`), independent of this change. End-to-end execution of a plugin script with the new ports is therefore verified only through the provider tests and wiring probes above.

## 5. Still open (32 violations, 13 files)

| files | edges | why not fixed here | recommended owner action |
|---|---:|---|---|
| `🎞️animate/🧪️tests/🎚️config`, `📐️cad/🧩️extensions/🏛️aec-building-structure/🧪️tests/🎚️config` | 6 (ncpr 4, sdk-only 2) | need a new SDK key | add `./ui/targets/react/build-tooling` to `@semio-tech/framework` (`lw1-tools/exports-needed.json`; the glue file exists already); no import change is needed after that, the two configs already point at the glue file |
| `📐️cad/…/👥️presence/🧪️tests/🔬️cad-presence-retirement` | 4 targets (12 violations) | the plugin twin applies product tool-job laws (`toolJob*Exact`, `toolJobRustBlock`) and `WORKSPACE_ROOT` to cad and OS sources; the caller is the plugin's own script | move the self-test into the product `tool-job-coverage` laws (plugin fixtures read by path) or inject the laws; needs a decision on where twins get product laws |
| `📕️norm/📇️registry/🧬️contract/🧪️tests/🧱️definition-ownership` | 1 (3 violations) | applies the Rust source direction rule engine (`rustSourceDirectionEdges`) to the norm contract | declare the rule `norm-contract-no-artifact-input` in the dependency direction taxonomy and drop the test, or move it to the product laws |
| `📕️norm/🧪️tests/🔮️oracle-source-ownership` | 6 targets (taxonomy authority, file-kind resolution, four `@semio-tech/repo-test` entries) | a repo-test-platform law about the norm oracle adapters | move to the product (`🧪️test` module tests), reading the plugin fixture by path |
| `🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🧩️index` | 1 | the plugin's only vitest case is a core test of the framework product `🎤️presentation`; the SDK may not depend on a product | move the case into `🎤️presentation/🧪️tests` and retire the animate vitest project (project.json, script, config, launch entries) |

Other notes for the batch/coordinator:
* The manifests of the plugin packages do not list `@semio-tech/framework` as a dependency in most places (bare specifiers resolve through the root workspace link, as the stdio tests already did); add it when the freeze lifts.
* The process package facade `@semio-tech/process` and the SDK are consistent; no entry points to a path that is not an export target except the one build-tooling glue above.
* The synchronous provider load relies on `require` of ESM (Bun and Node 22.12+); the provider modules contain no top-level await (the provider test would fail otherwise).
* `lay1` (intra-plugin cycles) edits the same plugin files; the rewrite re-read each file immediately before writing and only touched import statements, specifier strings and the renamed identifiers.

Tools (inputs, kept): `lw1-tools/{scan,symbols,rewrite,tc,agg,rest}.ts`, `lw1-tools/cruise.sh`, `lw1-tools/exports-needed.json`, `lw1-tools/touched-{plugins,framework}.txt`. Generated output (`🗑️generated/lw1-run`, 97 MB of cruise reports) deleted.
