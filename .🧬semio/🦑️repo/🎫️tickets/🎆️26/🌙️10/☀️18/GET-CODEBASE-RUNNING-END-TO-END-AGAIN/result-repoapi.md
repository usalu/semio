# Result repoapi: repo-lib public API, framework moves, final exports spec, apply script

Executor `repoapi`. Freeze honored: no `package.json` / `bun.lock` edit, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**` (the storage glue only re-exports from the frozen file, read-only), no cargo, no modifying git command, no sub-agents. `bun repo --help` boots after the work (checked at the end). Paths: `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `C` = this ticket folder. All tools are inputs under `C/repoapi-tools/`; generated output is deleted.

## 1. Outcome

| item | result |
|---|---|
| specs aggregated | 13 files `*-tools/exports-needed.json` (lay6, lay7, lay9, lw1, lw2, lw2b, lw4, lw5, lw5b, lw8, lw10a, lw10b and one more as they appeared), 1,160 rows, 382 of them for repo-lib, plus a fresh scan of 15,001 source files (the scan is the truth for repo-lib, the specs only widen it) |
| symbols classified (repo-lib) | 555 distinct (target, symbol) pairs over 1,733 import specifiers: **(a) framework 43 symbols / 530 specifiers, (b) repo-lib public API 478 symbols / 1,157 specifiers, (c) deleted coupling 34 symbols / 46 specifiers** (`repoapi-tools/classification.json` lists every pair with its destination and importer counts) |
| repo-lib public API | **15 entry modules** (11 TypeScript, 3 JavaScript, 1 CommonJS) instead of 208 per-file entries, all code written (section 4) |
| framework moves | destinations are the existing provider ports; the `repository-tooling` port got 9 more methods with their repo-side implementation (section 3) |
| consolidated spec | `C/repoapi-tools/exports-final.json`: 13 spec files, 1,160 rows read, 44 packages, **657 manifest lines in 41 package.json files** (15 for repo-lib, the rest deduplicated from the other specs). The deps executor applied them in the meantime (`result-deps.md`, batch 3); 3 lines are still missing in the live manifests (`exportsMissing`) |
| glue of the other packages | the manifests reference 586 glue files that did not exist; **created 503 files (94 JSON wrappers) and extended 17 existing glue files** with `glue-others.ts --write`; the 583 TypeScript glue files type-check with 0 diagnostics |
| apply script | `C/repoapi-tools/apply.ts` (`dry` / `apply`), dry-run tested against the live tree and type-checked with an overlay (sections 7 and 8) |

The brief's 180+ entries into repo internals collapse to the 15 entries below because the specs asked for every internal file that some importer touched; the classification shows that 530 of the 1,733 specifiers do not belong to the repository product at all (they are execution, cache and test-harness plumbing that the framework already exposes through ports), 46 are dead or illegitimate, and the rest group by what a consumer wants to do.

## 2. Method

1. `scan.ts`: one pass over the 15,001 tracked/untracked source files (`git ls-files -co`), TypeScript AST per file, every import-like edge (static, `import type`, `export from`, dynamic, `require`, `import()` types) that resolves into `LIB`, with imported names, namespace members, unused names. Importers inside `LIB` (including the nested `nx-bootstrap-tools`) stay relative and are not rewritten. 1,346 edges, 552 importer files, 212 target files.
2. `rules.ts` is the whole policy: the domain table (library path prefix to entry), the framework table (library symbol to framework port function) and the rewrite constraints. `engine.ts` applies it per file and is the single code path for classification, glue generation and rewriting.
3. `selftest.ts` pins the engine on 15 synthetic cases (rename, alias, shorthand property, merge, dead import, JSON default static and dynamic, type import, mixed destinations, cargo argument dropping and its fallback, repo-product importers, scripts).

Policy that decides a symbol:

* **(a) framework** only for importers outside the repository product, and only when the destination glue exports the name (checked on the glue source, a missing name falls back to (b)). The repository product itself keeps using its own entry. Rewrites that change a call (cargo test/exact/artifact functions) drop the arguments the provider supplies (`preparation:`, `network:`) with an AST edit inside calls of the renamed functions only; any leftover use of a dropped name makes the whole file fall back to (b).
* **Configuration files** (`vitest.config`, `vite.config`, `🧪️tests/🎚️config`, `🏗️builder/🌐️vite`, `.storybook`) never take a provider-port destination: they are evaluated when an editor or runner loads them, outside the script envelope that issues `SEMIO_SCRIPT_PROVIDERS` (a port call would throw there). Only the pure `findWorkspaceRoot` is allowed for them.
* **(c)** unused imports (42, dead code), and imports of another package's test fixtures (4 edges in 2 files, section 6). Query imports (`?raw`, `?url`, `?worker`) are never rewritten: they read the file itself and a glue file would change what they read.
* **(b)** everything else, assigned to a domain by library path.

Specifier form (same convention as lw10a): bare `@semio-tech/...` outside the build CLI closure; relative to the glue file for the closure (607 files, recomputed), `📜️script.ts`, configuration files, `.mjs`/`.cjs` files and the bootstrap tooling under `LIB/⚡️caching/🚀️bootstrap/`. A relative import that lands exactly on a manifest export target is conformant for the cruiser, so nothing needs an exemption.

## 3. (a) What belongs to the framework

All destinations are `@semio-tech/process` entries that already exist in the manifest (no new key):

| framework destination | symbols | specifiers |
|---|---:|---:|
| `execution` (port `repository-tooling`: `runCmd`, `runProbe`, `runBun`, `runCargo`, `orchestratorBudgetOpts`, `devToolingEnv`, `repoCacheDirectory`, `repoTestArtifactEnvironment`, `cargoTargetDirectory`, `getRepoMetaDir`, `runRepositoryCommand`, `runRepositoryMutationBridgeScriptV1`, `dispatchOwnedScriptRoute` and the nine below) | 22 | 170 |
| `cargo-test` (port `cargo-admission`, `runRepositoryCargoTests` becomes `runAdmittedCargoPackageTestsV1`, `...ExactCargoLaws` becomes `runAdmittedExactCargoLawsV1`, `buildRepositoryCargoArtifacts` becomes `buildAdmittedCargoArtifactsV1`, test policy, command, artifact-package functions) | 8 | 164 |
| arguments dropped because the provider supplies them (`configuredRepositoryCargoPreparationPolicyV1`, `repositoryCargoNetworkV1` inside port calls) | 2 | 65 |
| `routing` (`getWorkspaceRoot` becomes `findWorkspaceRoot`, pure) | 1 | 62 |
| `vitest` (`runVitest`) | 1 | 32 |
| `test-execution` (`runRepositoryTestCommand`) | 1 | 16 |
| `typescript-check` (`typecheckRepositoryProjectV1` becomes `typecheckAdmittedProjectV1`, artifact TypeScript package main) | 2 | 10 |
| `wasm-build` (`buildRepositoryWasmWebV1` becomes `buildAdmittedWasmWebV1`) | 1 | 6 |
| `native-build` (cargo-workspace port: observation, workspace for manifest, repository packages) | 5 | 5 |

Implemented here (code): the `repository-tooling` port (`🧰️framework/🔨️modules/🏃️process/🛠️repository-tooling/🟦️.ts`, written by lw1) lacked the process helpers that os/hub/s/play scripts still imported from the repository. Added, additive only, nine methods with the same names and signatures as the repository functions: `runCmdStatus`, `runNodeBinStatus`, `resolveWorkspaceBin`, `daemonBudgetOpts`, `cargoProfileDir`, `cargoBuildDirectory`, `cargoDirectories`, `semioBuildParallel`, `repoToolCacheEnv`. Touched: the port, its re-export in `process/execution` (`📦️packages/🟦️typescript/🎛️execution/🟦️.ts`) and the repo-side provider `LIB/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts`. Both provider suites pass (`bun test`: framework 4/4, repository 4/4, 91 expectations instead of 82 because the new methods are checked to exist), the three files type-check clean.

`prepareCargoWorkspaceInvocation` is deliberately not mapped: the port drops its first (`preparation`) parameter, so it is not a rename (the first overlay run caught this as `TS2554`). Its two importers stay on `./cargo`.

Not moved (still (b), consumed outside the repo product by production code or scripts): `spawnDaemon`/`SpawnDaemonHandle`, `protectOwnerOnly` (own Rust twin and fixture), `runBunxStatus`, `viteConfigLoader`, `semioBuildMode`, `runRepoScriptMain`, the cargo dependency-input cluster (`repositoryCargoNetworkV1` outside port calls, `withRepositoryCargoInputsV1`, `repositoryCargoConsumptionV1`, ...), `launchRepositoryCargoV1`, `nextestArtifactLocation`, the cargo CLI policy parser and toml reader. Each is a candidate for the next port method; none needs a new mechanism.

## 4. (b) The repo-lib public API

Fifteen glue modules under `LIB/📦️packages/🟦️typescript/<dir>/`, generated by `glue.ts --write` from the live classification: explicit `export { ... } from` (values) and `export type { ... } from` (types) of the library file that owns each member, JSON documents as named exports (`export { default as x } from "...json" with { type: "json" }`), nothing defined in them. Each starts with an emoji docstring naming the entry; the taxonomy glue grammar classifies re-export-only files as `declaration`, which is allowed without a statement limit. Directory names reuse names that already exist in the library (`⚖️laws`, `📇️inventory`, `🏗️authoring`, `🧪️testing`, `🏃️process`, `🦀️cargo`, `📦️artifacts`, `💾️storage`, `🏗️build`, `🎮️playground`, `🚨️policy`).

| entry | glue file (below `LIB/📦️packages/🟦️typescript/`) | library files | names | specifiers repo product / other products | only used by tests outside the repo product |
|---|---|---:|---:|---|---:|
| `./laws` | `⚖️laws/🟦️.ts` | 22 | 127 | 0 / 151 | 107 |
| `./inventory` | `📇️inventory/🟦️.ts` | 49 | 103 | 30 / 159 | 29 |
| `./inventory/tooling` | `📇️inventory/🟨️.mjs` + `.d.mts` | 2 | 3 | 0 / 10 | 0 |
| `./inventory/commonjs` | `📇️inventory/🟨️.cjs` | 1 | 1 | 1 / 0 | 0 |
| `./authoring` | `🏗️authoring/🟦️.ts` | 10 | 21 | 1 / 21 | 7 |
| `./testing` | `🧪️testing/🟦️.ts` | 9 | 14 | 83 / 10 | 5 |
| `./process` | `🏃️process/🟦️.ts` | 10 | 17 | 25 / 22 | 4 |
| `./cargo` | `🦀️cargo/🟦️.ts` | 27 | 62 | 58 / 81 | 9 |
| `./artifacts` | `📦️artifacts/🟦️.ts` | 15 | 34 | 20 / 225 | 0 |
| `./storage` | `💾️storage/🟦️.ts` | 22 | 32 | 28 / 85 | 3 |
| `./build` | `🏗️build/🟦️.ts` | 8 | 13 | 12 / 13 | 1 |
| `./build/tooling` | `🏗️build/🟨️.mjs` + `.d.mts` | 3 | 7 | 8 / 1 | 0 |
| `./build/inference` | `🏗️build/🕸️graph/📥️inference/🟨️.mjs` + `.d.mts` | 3 | 3 | 12 / 4 | 3 |
| `./dev` | `🎮️playground/🟦️.ts` | 16 | 37 | 0 / 75 | 4 |
| `./policy` | `🚨️policy/🟦️.ts` | 6 | 11 | 22 / 4 | 1 |

Design points:

* The JavaScript entries exist because Nx plugins and dependency-cruiser load `.mjs`/`.cjs` facets under node; they cannot go through a TS glue. `./build/tooling` (build mode, observation, graph fact storage) and `./build/inference` (inference plugin, arguments, Nx dependency implementation) are split so the Nx plugin does not load the inference closure. The `.d.mts` siblings follow the existing `🟨️.d.mts` precedent so TypeScript importers keep their types.
* Tests: of the 403 repo-lib symbols that products other than the repository use, 173 are only used by tests there (107 of 127 law symbols are the OS tool-job aggregator suite binding repository laws to its own rows). They stay public because the law engine is meant to be bound by product suites; the table shows where a test move into the owner would shrink the API.
* Cost: importing a domain loads every member (cold, Bun, under load): laws 1.2 s, inventory 1.0 s, storage 0.7 s, build/tooling 0.35 s, others below 0.6 s. Typical consumers import several members of the same domain, and the `build/*` JS split protects the Nx plugin load.

Checks run on the glue (`tc-glue.ts`, `load-glue.ts`): TypeScript program over the 14 TS/JS glue files (1,948 files): **0 diagnostics**, with a control (`TS2305` on a bogus export) proving the checker sees the files; every glue module loads at runtime under Bun and exports every requested name; the CommonJS glue loads under node and exposes `loadDependencyDirectionPolicy`.

## 5. Final spec `repoapi-tools/exports-final.json`

Structure: `exportsToAdd` (package.json path to `{subpath: target}`, **584 lines in 38 manifests**, ready to merge), `repoLib` (the 15 domains with members, importer counts and `specCoverage`: all 382 repo-lib rows with their disposition, 25 of them settled because no importer outside the library is left), `packages` (44 packages, per entry: chosen subpath, target, glue file, symbols, importers, source specs, `existing` when a key in another package already exports the file), `byPackage` (lay6-compatible view), `glueToCreate` (586 files), `classification` (counters, the open fixture couplings, failures: none).

Rules of the merge: rows are grouped by (package, target file); the subpath is the existing key for that target if there is one, else the most frequent proposal, else the shortest (JSON documents keep a `.json` suffix; 4 colliding proposals resolved); glue mirrors the owner-relative path (`🔨️modules/` dropped for `@semio-tech/framework`, `🧪️tests` written `🧪️testing`, JSON documents get a TS wrapper `<dir>/🔣️json/🟦️.ts` because a JSON file cannot be glue); packages whose manifest sits in the source owner export the file itself; stylesheet assets (2 rows) are left to the owner; rows whose target is already an export target of another package point at that entry (9). The other packages keep the existing per-file convention; consolidating `framework-os` (125), `framework` (112), `process` (89), `framework-plugin` (76) the way repo-lib was consolidated is possible with the same tooling but was not asked.

State at the end of this work: the deps executor merged the lines (651 of 657 identical to the spec, 2 keys exist with another target and are skipped by the apply script with a message: `repo-test` `.` and `ui-react` `./i18n`; 3 are still missing in the manifests) and added the workspace dependencies of `manifests-needed.json`. The glue files its lines point at are now written (`bun C/repoapi-tools/glue-others.ts --write [--force]`; `--force` regenerates only files that still start with the generated docstring). Entries whose importers only use `?raw`/side effects get an `export *` glue. Rows whose target vanished between spec and now (one stale `r3f` entry) are skipped.

## 6. (c) Coupling that was not made public

* 42 unused imports across 30 files (for example `runVitest` unused in 5 files, `runCmd` in 4) are removed by the apply script.
* 4 edges in 2 files import another package's **test fixtures**: `🔌️plugin/📇️registry/🧪️tests/🧩️composition/🟦️.ts` (three preview-contract tests that load `LIB/🏭️generator/👁️preview/{🔤️arguments,📈️progress}/🧪️tests/🧫️fixtures`) and `🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/🧪️tests/🟦️.ts` (selection fixture). They are repository contract tests that sit in the OS/hub tree; fixtures are not API. The edges stay untouched and are listed as `OPEN`; the fix is to move those tests into `LIB/🏭️generator/👁️preview/🧪️tests` (same package, relative fixtures) or to give them their own fixture. Not done here: the tests need `yargs-parser`, `string-argv`, `ajv` resolution that I could not run.

## 7. Apply script `repoapi-tools/apply.ts`

```
bun C/repoapi-tools/exports-final.ts            # regenerate the spec from all exports-needed.json
bun C/repoapi-tools/apply.ts dry   [--relative] [--only=<substring>] [--skip-others] [--skip-repo-lib] [--no-scan] [--show=<file substring>]
bun C/repoapi-tools/apply.ts apply [same flags]
```

* Scans the tree (refreshing the file list in `apply`), plans every importer, and rewrites in one pass per file: other packages' deep imports to their new entries (entries whose manifest key exists; `?query` imports and glue-less targets are skipped with a reason), then repo-lib imports through the engine (framework ports, domain entries, dead imports removed, imports of the same entry merged, JSON default imports turned into named imports, dynamic JSON imports rewritten).
* `dry` writes only `🗑️generated/repoapi/{apply-plan,overlay,apply-edges}.json` and `manifests-needed.json`, treating the manifest keys as present. `apply` refuses (exit 2) while a repo-lib manifest key is missing, unless `--relative` targets the glue files directly; it re-reads each file before writing and skips it when it changed since the plan, resolves every new bare specifier with `Bun.resolveSync`, and never touches frozen paths.
* `apply` first writes the missing glue (`--no-glue` skips it), then rewrites. Last dry run on the live tree (manifest keys live, glue written): 3,999 importer files scanned, **1,059 files would change** (556 through the repo-lib engine, 449 framework-port names, 34 dead imports removed, 1,805 edges to other packages' entries), 0 engine failures, 0 unresolved specifiers, 1 manifest still lacking a dependency; skipped with a reason and left for a second run: whole-module uses that need an `export *` glue, names an existing glue still lacks, files edited by peers after the scan, `?query` imports, the two key collisions. The count of skipped edges (194) shrinks once the 17 glue extensions are in, which they now are.
* Write-path test: a synthetic importer (default JSON import, a domain function, `getWorkspaceRoot`) was created in the hub test tree, `apply apply --relative --only=...` rewrote it to `findWorkspaceRoot` from the process routing glue, `withRepositoryCacheStoreV1` from the `storage` glue and the storage schema as `artifactsStorageSchema` from the `artifacts` glue, and Bun executed the result with the expected value types; the file was deleted afterwards.

## 8. Verification

* Engine: `selftest.ts`, 15 synthetic cases, all pass (rerun at the end).
* **Overlay type-check of the whole plan** (`verify-overlay.ts`, TypeScript API, root `tsconfig.json` options): the 1,032 files the dry run would change are checked once as they are on disk and once with their rewritten text overlaid (new glue overlaid as virtual files before it existed, real files afterwards). Final run, glue on disk: baseline 69 diagnostics, after 68, **0 regressions**, 1 fixed. The run found and drove out these defects, in order: positional arguments of `prepareCargoWorkspaceInvocation` (not a rename), a `network:` argument dropped outside a port call, `?raw` query imports losing the query, JSON wrappers and `.d.mts` companions, `🧪️tests` versus `🧪️testing` glue directories, glue exporting names that the module does not export, namespace uses with missing members, existing-glue extensions (319 then 203 then 22 then 0 regressions across the runs).
* Glue of the other packages: 583 TypeScript glue files, TypeScript program of 5,196 files, 0 diagnostics (`tc-files.ts`); repo-lib glue: 14 files, 0 diagnostics, a bogus export is reported as `TS2305` (control); all repo-lib glue loads at runtime and exports every requested name.
* Provider suites: `bun test` `LIB/🏃️process/🔌️providers/🧪️tests` 4/4 (91 expectations) and `🧰️framework/🔨️modules/🏃️process/🔌️providers/🧪️tests` 4/4 (52), after the port extension.
* Write path: exercised on a synthetic file (section 7), refusal path exercised (`apply` exits 2 with the missing-key list while the manifest lacked the keys).
* `bun repo --help` boots (checked before and after the glue was written; the CLI closure was never edited, closure files get relative glue paths only).
* Not run: the cruiser, the taxonomy/package-body laws, any `bun repo run` target (shared lease queue), vitest suites of the 1,059 files (the type-check is the evidence, not behavior), the Rust side.

## 9. Open items

1. Run `bun C/repoapi-tools/apply.ts apply` (writes glue if missing, then rewrites 1,059 files; re-run `exports-final.ts` first if new `exports-needed.json` files appeared, the script warns). Review the diff by package; the rewrite is mechanical and type-checked, not executed.
2. The taxonomy/package-body laws were not run: new directories and `🟨️.d.mts`/`🟨️.cjs` files under `LIB/📦️packages/🟦️typescript` follow existing precedents (process package, `🟨️.d.mts` companions) but are unverified; `bun repo run @semio-tech/repo-lib:test-package-body-policy` should be run once the CLI queue is free.
3. Packages whose manifest sits in a Rust crate directory (`framework-plugin`, `framework-os-node-graph-rs`, the `-rs` packages named by lay9) receive TS glue next to the Rust sources in the spec because that is where their manifest is; confirm with the dependency owner.
4. Provider semantics: the `repository-tooling` functions throw `Script provider designation required` when `SEMIO_SCRIPT_PROVIDERS` is absent; scripts and tests run under `bun repo` have it, vitest/vite configs do not, hence the configuration-file rule.
5. `./build/inference` points at `.mjs` files that have no declaration companion; TypeScript importers get `any`-class errors there as before, now reported at the new entry.
6. Fixture couplings (section 6) and the framework candidates (end of section 3).

## Files

Code (new): the 15 glue modules and the two `.d.mts` companions listed in section 4. Code (edited, additive): `🧰️framework/🔨️modules/🏃️process/🛠️repository-tooling/🟦️.ts`, `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`, `LIB/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts`.
Ticket inputs `C/repoapi-tools/`: `lib.ts rules.ts engine.ts exports.ts packages.ts specs.ts scan.ts classify.ts classification.ts glue.ts glue-others.ts exports-final.ts apply.ts verify-overlay.ts tc-glue.ts tc-files.ts load-glue.ts selftest.ts report-tables.ts`, outputs worth keeping `exports-final.json classification.json manifests-needed.json`.
