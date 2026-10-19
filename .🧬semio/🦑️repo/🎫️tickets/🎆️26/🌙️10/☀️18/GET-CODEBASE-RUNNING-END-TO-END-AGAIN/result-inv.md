# Result inv: framework -> products dependency inversion (follow-up of `followup-law1-script-product-imports.md`)

Executor `inv` (Sonnet 5.5), foreground only, no sub-agents, no cargo, no modifying git command, no `package.json` / `bun.lock` edit. `AGENTS.md` not edited.

## Result

| law (acceptance gate) | before | after |
|---|---|---|
| `framework/📏️script-boundary` (router bundle, 3 tests) | red | **green** (3/3) |
| `tool-run/🏛️ownership` (2 tests) | red | **green** (2/2) |
| `actor/🧪️tests/🏛️owner-boundary` (1 test) | red | **green** (1/1) |
| `actor/🎟️retained-turn/🎟️context/🧪️tests/🏛️boundary` (2 tests) | red | **blocked by a missing devDependency, not by a product import** (see Blockers 2); all its assertions pass when `tree-sitter-wasms` is resolved from the one package that declares it |
| `process/🧭️routing/🎛️command/🧪️tests/🚧️boundary` (3 tests) | red | tests 1 and 2 **green**; test 3 fails only on 6 dangling `package.json` export keys (see Blockers 1); with those 6 keys ignored it is green (3/3) |

`bun repo --help` boots. No exemption was added anywhere. Framework files that import a `🛍️products` module: 196 -> 0 (`inv-tools/inventory.ts` walks every `.ts/.tsx/.mjs/.js` under `🧰️framework` except `🛍️products`, `🛒️products`, `node_modules`, `target`, `dist`, generated; resolves relative and `@semio-tech/*` specifiers).

## Design (schema-first, interface + registry in the framework, implementation in the product)

* `🧰️framework/🔨️modules/🏃️process/🔌️providers/` (new): neutral loader. `🧬️schema/🔣️.json` is the closed wire contract `{version:1, providers:{<port-name>: <absolute module path>}}`; `defineScriptProviderV1(name, methods)` declares a port, `loadScriptProviderV1(token)` reads the map from env `SEMIO_SCRIPT_PROVIDERS`, imports the designated module once (`pathToFileURL`, so paths with spaces are fine on all OSes), requires its `provider` export and checks every declared function. The framework holds no static or literal reference to any product.
* Ports (types + thin delegating functions, no product logic), each re-exported through the existing `📦️packages/🟦️typescript/**` facade that the scripts already import:
  * `cargo-admission` (`🧪️testing/🦀️cargo/🔌️provider`): `runAdmittedCargoTestsV1`, `…LintV1`, `…CommandV1`, `…PackageTestsV1`, `admittedCargoTestPolicyV1`, `runAdmittedExactCargoLawsV1` (the provider supplies the preparation policy, so scripts no longer pass `preparation`), `buildAdmittedCargoArtifactsV1` (the provider supplies the network grant), `withAdmittedCompilerStoreV1`.
  * `wasm-build` (`📦️artifacts/🕸️wasm-build/🔌️provider`): `buildAdmittedWasmWebV1`.
  * `typescript-check` (`🏗️build/🟦️typescript/🔌️provider`): `typecheckAdmittedProjectV1`.
  * `artifact-staging` (`📦️artifacts/📤️publication/🔌️provider`): `stageAdmittedArtifactTreesV1` (canvas runtime assets; mode check moved into the provider).
  * `styling-python` (`🖱️ui/🎨️styling/🏗️builder/🐍️python/🔌️provider`): run / withWheel.
* Product implementations (each a `provider` export typed by the framework interface): `📚️library/🏃️process/🦀️cargo/🔌️provider`, `🏃️process/🕸️wasm/🔌️provider`, `🏗️build/🟦️typescript/🔌️provider`, `⚡️caching/📦️artifacts/🔌️provider`, `🎨️styling/🏗️builder/🐍️python/🔌️provider`. They wrap the unchanged functions (`runRepositoryCargoTestsV1`, `runRepositoryExactCargoLaws`, `buildRepositoryWasmWebV1`, ...), so execution stays in-process in the script process with identical leases and cancellation.
* Composition root: `📚️library/🏃️process/🔌️providers/🟦️.ts` lists the five designations; `runBuildCliV1` (`🏗️build/📜️script.ts`) calls `issueRepositoryScriptProvidersV1(environment)` first, so every descendant (nx runner, bridge command, script, nested `bun repo`) inherits `SEMIO_SCRIPT_PROVIDERS`. Rejected alternatives: spawning `bun repo cargo-*` per script (exact laws take structured options, process nesting would change lease/admission semantics, cannot be verified without cargo); `BUN_OPTIONS=--preload` (cannot carry paths with spaces, verified); static imports (the law).

## Batches

1. Cargo admission/exact/policy/network/artifact (61 scripts + `🕸️graph/🛂️manifest/🏃️execution`): AST codemod `inv-tools/codemod-cargo.ts` (aliases kept, `await` added for the policy call, `preparation:` and `network:` arguments dropped). 69 files, `tc.ts` (strict tsc over all of them): 0 diagnostics.
2. Remaining 8 scripts by hand: 3 wasm (`actor`, `surface`, `editor`), `canvas` stage-runtime-assets, `ui/styling/builder/python`, `ui/targets/react` typecheck, framework root `🦀️rust` (`withAdmittedCompilerStoreV1`), framework root `🟦️typescript` typecheck.
3. Tests/libraries (45 files):
   * Neutral harness moved into the framework: `registerBuildTestLawV1` / `withBuildTestInvocationV1` / fixtures (`🏗️build/🧪️tests/📥️invocation` -> `🏃️process/🧪️testing/📥️invocation`, 6 files); 140 importers repointed by `inv-tools/mv-dir.ts` (also products, `🌎️hub`, ticket scripts).
   * Product self-tests moved to the product laws tree (importer: root `📜️script.ts`, rewritten): tool-job fixed-operation-registry, history-closure-policy, mounted-layout-text, mounted-prepared-render, mounted-surface-lane, prepared-raster-producer. `tool-job-microsecond-budget` split: arithmetic oracles stay in the framework, the worker-binding half moved to `⚖️laws/🎯️tool-job-coverage/🏭️factory/🧪️tests/🔬️tool-job-microsecond-worker-binding` and is added to the os `tool-job-coverage` sum. `WORKSPACE_ROOT` replaced by `findWorkspaceRoot(import.meta.dir)` in 3 tests.
   * Native-build tests that exercise the product stores moved to the product: units, units/profile, publication, platform/parent, configuration/runtime, exact `🎟️phase` and `📥️invocation`, canonical SDK helper, persistent-state nx output case (new product test, passes 1/1), canvas bundle-staging and nx-output case, adapter-invocation (2 tests), native bootstrap workspace-resolution (2 tests), python builder implementation + tests (framework keeps contract/inventory/schema). Shared fixtures that framework tests also read (`📤️publication/🧪️tests/🔣️.json`, `🕸️units/🧪️tests/🧫️fixtures`) stayed in the framework. `import.meta.dir`-relative reads in moved tests were re-pointed (`verify-paths.ts`: all literal paths of every moved file exist).
   * Typecheck-state test now calls the `typescript-check` port.
   * Six `📦️packages/🟦️typescript/🛍️products/🦑️repo/**` re-export stubs deleted; their 26 consumers (`🖥️server/🎛️coordinator`) import the product library directly.
   * Run sites: moved tests that framework suites used to run are still listed in the framework suite tables by full product path (`process/📜️script.ts`: `native-storage-platform`, `workspace`; `🧪️test/📜️script.ts`: `adapter-ownership`); the exact invocation test moved to the product script list. New tests ride existing targets (no new command, so no `project.json` / `launch.json` change): framework provider test via the `routing` suite, product provider test via `original-*-control`.

## New tests (language-agnostic corpus + third-party oracle)

* `🏃️process/🔌️providers/🧪️tests` (3 tests, pass): designation corpus `🧫️fixtures/🔣️.json` validated by Ajv and by our `validateJsonSchemaSubset` (must agree), port-name grammar, env round trip, relative paths refused, single load, missing function / missing export / undesignated port refused.
* `📚️library/🏃️process/🔌️providers/🧪️tests` (2 tests, pass): the repository designates exactly the five ports, every designated module exists, and each one loads (full import closure resolves) and implements every declared function.

## Verification actually run

* The 5 law suites (table above), `bun repo --help`, both provider suites, the moved persistent-state test.
* `tc.ts` strict tsc over 195 changed/moved/new files and over the 144 importers of the moved harness: 0 diagnostics from this change (one TS2345 in `🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts:10` is in a concurrently edited line, `store.parent` -> `store.root`).
* The microsecond worker binding fails with `microsecond exact worker binding: none` at HEAD as well (ran the HEAD file): pre-existing, behaviour preserved by the split (framework half returns 30).
* Not run: cargo, any `bun repo run` target. A real `bun repo run @semio-tech/framework-process:test-routing` sat in the shared lease queue (0 CPU for 10+ min, behind `dependencies sync` / cargo checks) and was stopped; end-to-end provider loading inside a nx-launched script is therefore verified only by the provider test (same loader, real modules) and statically.

## Blockers needing the coordinator (all outside the freeze)

1. `🧰️framework/📦️packages/🟦️typescript/package.json`: delete the 6 `"./products/repo/modules/..."` export keys (lines 237-241 and 258: native bootstrap fixtures, server library authentication/event/persistence/source-index, dashboard installation). Their targets were framework re-export stubs of product modules and are gone; consumers are repointed. Routing-boundary test 3 turns green with exactly this edit (verified by a copy of the law that ignores those 6 keys).
2. `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/package.json`: declare `"tree-sitter-wasms": "0.1.13"` (devDependency; today only `🛍️products/🦑️repo/🔨️modules/🧪️test` declares it, the dependency image already contains it). With it resolvable from the product package the context-boundary law passes (2/2, verified by a copy).

## Residual debt (documented, not hidden)

* `process/📜️script.ts`, `🧪️test/📜️script.ts` and `🧪️tests` fixtures still name product test files as path strings (laws check imports only). Same pattern existed before (`compiler-capacity-inheritance` etc.).
* `tool-job-microsecond-budget` (framework test) and the `process` suites read product `.rs` / test paths as data.
* New framework modules are reachable by relative path only (no new `exports` entries because of the freeze); add `./providers`, `./cargo-test` additions if external packages need them.
* Build-graph script-input inference follows literal imports, so product provider closures are no longer inputs of the 15 `cache:true` framework targets that run these scripts; add explicit inputs if those caches must invalidate on product admission changes.
* Bazel actions build their own child environment; `SEMIO_SCRIPT_PROVIDERS` is issued for the Nx path (default). The nx daemon env exclusion list (`nx@23.2.0.patch`, `🌍️environment/📋️policy`) was not touched: the value is constant per checkout, so no graph churn expected.
* Moved tests were checked for existing relative targets and strict types only; none could be executed under the harness here.

Tools (inputs, kept): `inv-tools/{inventory,symbols,codemod-cargo,mv-dir,tc,verify-paths}.ts`. Generated output removed.
