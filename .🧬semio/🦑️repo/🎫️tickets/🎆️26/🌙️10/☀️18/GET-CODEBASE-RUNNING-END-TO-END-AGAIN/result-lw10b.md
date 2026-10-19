# Result lw10b: WP10 part B and WP11, repo product (non-process) and other products

Executor `lw10b`. Scope: `🧰️framework/🛍️products/🦑️repo/**` imports into framework modules OTHER than `process` (process = lw10a), repo-internal deep imports and cycles, repo to os/hub; WP11 print, quiz, pets, presentation, server. No `package.json`/`bun.lock` edit, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`, no cargo, no git-modifying command, no compat barrel or re-export added.

## 1. Totals (dependency-cruiser, config of `result-lay0.md`)

Command (`lw10b-tools/cruise.sh`, about 6 min per run, whole repo product plus the five other products, targets outside are leaves via `--do-not-follow`):
`node node_modules/dependency-cruiser/bin/dependency-cruise.mjs <repo product + quiz + presentation + pets + print + server> --exclude '(^|/)(dist|target|pkg|node_modules)/' --do-not-follow '^(?!<products>/(repo|quiz|presentation|pets|print|server)/)' --config …/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type json`

| rule | before | after |
|---|---:|---:|
| no-cross-package-relative, all targets | 2,381 | 1,095 |
| of which target `process` (lw10a, repo and print) | 1,203 + 7 | 427 + 7 |
| of which MINE (non-process targets) | 1,171 | **661** |
| no-circular | 39 | **6** (repo product 35 to 6, print 3 to 0, quiz 1 to 0) |
| framework-no-implementation / no-cross-technology (repo test reads hub script text) | 1 / 1 | **0 / 0** |
| no-state-outside-os | 5 | **0** |
| no-core-path | 1 | 0 (a peer renamed the fixture) |
| **total, whole scope** | **2,428** | **1,101** |

All 661 remaining non-process `no-cross-package-relative` edges wait for manifest `exports` entries (section 3): the fixer finds 0 fixable edges (2 more were fixed in the last pass). The 6 remaining cycle records are 3 SCCs with design-bound dynamic cycle breakers (section 4).

## 2. What changed

### 2.1 Deep imports through existing public entries (tool: `lw10b-tools/ncpr-b.ts`, TypeScript checker driven)
298 specifiers in 253 files re-pointed (`lw10b-tools/ncpr-changed.txt`). An entry is used only when an EXISTING `exports` key exposes every imported symbol with the identical declaration (alias-resolved; namespace/dynamic uses need a superset). Two forms, as in lw10a:
- files OUTSIDE the build CLI closure: bare workspace specifier (`@semio-tech/schema-validator-ts`, `@semio-tech/framework/identity/path`, `@semio-tech/compiler-syntax-rust-ts`, `@semio-tech/framework-compiler-ts/syntax/module-tree`, `@semio-tech/repo-test/storage`, `@semio-tech/framework-os/modules/store/testing/…`, `@semio-tech/ui-styling/…`, …); 204 specifiers in 161 files.
- files INSIDE the CLI closure (entry `🏗️build/📜️script.ts`): RELATIVE path to the export-target glue file (92 edges, 90 files), never bare. No policy exemption is needed or added: a relative import landing exactly on an authored export target conforms (same finding as lw10a).
- skipped on purpose: frozen paths (`📦️dependencies/💾️storage/`, `🏗️build/📥️inputs/🔏️revision/`), process targets (lw10a), closure files.

### 2.2 Cycles (repo product 35 to 6, print 3 to 0, quiz 1 to 0), by leaf extraction or port
- `⚡️caching/🦀️cargo/📦️units`: `RustUnitPlanV1` moved to the plan owner (`📋️plan`), `RustUnitV1`/`RustUnitInputV1`/`readRustUnitV1` importers point at `🧬️definition`; the units root keeps only `planRustUnitsV1`; identity takes structural dependency-digest types. 26 importer files re-pointed by codemod.
- `🏃️process/🦀️cargo/📥️dependencies`: new leaf `📋️contract` (`RepositoryCargoInputsV1`, `CargoDependencyNetworkV1`); readiness storage and 4 importers (incl. `dashboard/📦️installation`, `native-build`, `verification/💾️storage`, os `plugin/🏗️build`) use it.
- `🗂️workspaces/🦀️cargo`: control contract (`CargoPreparationControlV1`, `CargoWorkspaceControlV1`, `CargoWorkspaceProgressV1`) moved into the empty `🛠️preparation/🎛️control` leaf; custody, preparation and native orchestration import it.
- `⚡️caching/🦀️cargo`: `cargoDirectories`/`cargoTargetDirectory`/`cargoBuildDirectory`/`CargoDirectories` importers re-pointed to the existing `🗂️directories` leaf (the root's re-export line deleted), `wasmBindgenVersion` moved to new leaf `🕸️wasm-bindgen`. Breaks the native-build / orchestration environment / wasm policy / process 5-node SCC. 14 importers in repo, os (mcp, wgpu scripts, plugin build) and hub re-pointed.
- `⚡️caching/🧹️pruning`: the root no longer re-exports the pressure planner (only its own test used it; the test imports `📉️pressure` directly).
- `📦️units/🏗️compiler`: `RustUnitCompilerWorkspaceV1` moved into `📍️workspace`.
- `🏗️authoring/📦️crate-tree/🧬️mutation-families`: `AggregateName`/`AggregateScope` moved into `🧰️support-hoist`.
- `⚡️caching/🕸️graph/🏁️startup`: `graphResourceLeaseDirectoryV1` moved to new leaf `🗃️lease-directory` (publication transaction no longer reaches the startup service and the nx storage).
- `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk` (7-module SCC, all gone): new contract leaf `📥️source/🔗️derivative/📋️projection` (`NxSdkDerivativeProjectionV1`); the two lazy `import()` cycle breakers became ports: `sealNxSdkSourceIndexV1<P>(…, admission: NxSdkSealAdmissionV1<P>)` (index no longer imports the projection owner or capacity; `derivative/🟦️.ts` passes `{loadProjection, loadPublication}`) and `withNxSdkDerivativePublicationV1(…, derivative: NxSdkDerivativePortV1)` (capacity no longer imports the projection owner; the script composition root and 6 test files pass it). The phase-law test was updated to the port shape (it asserts `wait(admission.loadProjection)`, no `import(`).
- print `tectonic-template-compilation`: `runPrintCompilerRefusalInspectorV1(environment, args, withCompiledPrintProbe)` receives the compiled-probe runner from the package script (composition root); test text assertion updated.
- print `schema/💡️inferences`: `inferVizLayerTable` and its 370-line layer-inference region moved out of `🧮transform` into `🧮transform/🎛️layer-inference`; transform no longer imports hierarchy/network/flow/spatial/shape/geo, so spatial and shape may import its primitives. Render, probes, package glue and the package barrel (`export *`) re-pointed.
- quiz pets: `PetPlayer`, `PetPlay`, `QuizPetsStage` and a new `StagedPets` (the stage-side part of `ShownPets`) moved into the lazily loaded stage module; `ShownPets extends StagedPets` adds `half`. The package entry exports them from the stage.
- repo test of the hub: the single test that reads the hub script source (`neutral consumed Cargo Hub actor fixtures…`) moved into the hub cargo provenance tests; this clears `framework-no-implementation`, `no-cross-technology` and the ncpr to hub.

### 2.3 New test (language-agnostic, with a third-party oracle)
`📚️library/🕸️dependencies/🧭️direction/🔗️relative-closure/{🟦️.ts, 🧪️tests/🟦️.ts, 🧪️tests/🧫️fixtures/🔣️.json}`: `readRelativeClosureV1(root, entry)` reads the files a bootstrap entry reaches through relative specifiers and the `@semio-tech/` specifiers among them. The JSON corpus (6 cases: static, re-export, dynamic, require, cycle/diamond, JSON and type-only, external/builtin) is checked against the expected sets AND against TypeScript's `preProcessFile`; a second test holds the real build CLI entry to zero workspace package specifiers (zero-touch before `node_modules`). `bun test` 2 pass, 20 expects; tsc-clean.

## 3. Not fixable without manifest change: `lw10b-tools/exports-needed.json` (lay6 format)
Per target file: proposed subpath, symbols, `importers` (outside the closure: re-run gives bare specifiers) and `closureImporters` (inside the CLI closure: re-run gives the relative glue path). 661 importer edges, of which 76 closure edges.

| package | new entries | importer edges (closure) |
|---|---:|---:|
| `@semio-tech/repo-lib` (has only `./package.json`) | 101 | 422 |
| `@semio-tech/nx-bootstrap-tools` | 29 | 48 (11) |
| `@semio-tech/framework` | 11 | 36 (3) |
| `@semio-tech/framework-plugin` | 13 | 14 (1) |
| `@semio-tech/compiler-syntax-rust-ts` | 4 | 13 |
| `@semio-tech/quiz-react` | 2 | 14 |
| `@semio-tech/plugin-registry` | 3 | 9 (3) |
| `@semio-tech/process` (print only) | 1 | 7 |
| `@semio-tech/framework-kernel` | 6 | 6 |
| `@semio-tech/repo-coordinator`, `repo-test`, `ui-styling`, `framework-os`, `framework-renderer-wgpu`, `compiler-syntax-ecma-ts`, `framework-canvas-ts`, `framework-compiler-ts`, `framework-replication`, `ui-react`, `repo-vscode` | 3 to 9 each | 1 to 12 each |

Observations for the manifest batch: 73 of the repo-lib edges are `nx-bootstrap-tools` (nested package `🚀️bootstrap/🛠️tools`) importing library `💾️storage/🔒️enforcement/**` internals, i.e. a bootstrap package depending on the library that contains it: the cleaner fix is to export that storage surface from repo-lib (or move it into the bootstrap package). `repo-lib` consumers outside the library are the server coordinator (about 60), vscode client, dashboard tests, print and quiz scripts. Re-run after the batch: `bun lw10b-tools/ncpr-b.ts <fresh cruise json> lw10b-tools/cli-closure-union.txt apply <out>` then `… apply-closure …`.

## 4. Not done (design-bound), with the exact remaining cycle edges
All three are lazy `import()` cycle breakers the maintainers paired with source-text-transpiling laws that bind ports by replacing the import text; converting them needs the same port migration as section 2.2 plus those laws, and the laws only run under the `bun repo` envelope, which is blocked (below).
1. `🏗️build` 4-module SCC (selection, caller, caller/child, bazel script; in the CLI closure): `watchBuildInvocationV1` (selection) lazily imports caller and child; `withBuildOrchestrationTaskEnvironmentV1` (caller) lazily imports the bazel planner; bazel imports caller statically. Design: pass `{read, run}` child ports in `watchBuildInvocationV1`'s `options` from `📜️script.ts` and pass the bazel planner into `withBuildOrchestrationTaskEnvironmentV1`; the `watch/🧪️tests/🎟️phase` and `📜️script.ts`, `projection` and `intent` tests bind those ports already (they replace the import text).
2. nx graph storage to publication transaction (`🔌️nx/📦️graph` `bytes()` lazily asks `graphPublicationFileAllocationV1`): move the journal reader (`journalRead`, `paths`, `regular`, `identity`, `checks`) into a leaf under `🔌️nx/📦️graph` that both import.
3. nx plugin `🟨️.mjs` root and `📤️arguments/🟨️.mjs`: the executor lazily reads the private graph through the root while the root hashes the executor source (`nxTaskArgumentsSourceIdentityV1`); needs `graphPublication` and the implementation-identity check split from the root.

## 5. Verification (exact commands, outcomes)
All from `/Users/ueli/Documents/semio`, foreground.
- Cruise before (`before.json`, 2,428) and after (`after3.json`, 1,101): numbers in section 1; SCC listing by `lw10b-tools/scc.ts`.
- tsc, repo product: `NODE_OPTIONS=--max-old-space-size=12000 node node_modules/typescript/bin/tsc --noEmit -p 🧰️framework/🛍️products/🦑️repo/tsconfig.json --pretty false`: last run 11 errors, none in a file or symbol I touched (missing `strip-ansi`/`smart-buffer`, `store/🧪️tests` arity, `nx/📦️graph/🧪️tests` null, discovery/runtime tests argument shape, `vendored-exclusion`, `launch-catalog`, os `artifact-addressing` missing child schema: peers). Intermediate runs found and I fixed: moved-symbol importers in os/hub (cargo directories, cargo inputs), 4 test type references, `PetDeed` in the pets glue, two files garbled by the codemod (below).
- tsc, other products (`lw10b-tools/tsconfig.products.json`, 12 changed files + closure): only missing-d3-types noise (`d3-*`, `dagre`, `@asamuzakjp/css-color`) and the consequent implicit-any errors; nothing about my edits. Hub + consumption + new law test (`tsconfig.hub.json`): 0 errors.
- Resolution: `Bun.resolveSync(replacement, dirname(importer))` for all 204 bare replacements: 0 unresolved (`lw10b-tools/resolvecheck.ts`).
- `bun repo --help` booted after every batch (usage printed, stage `complete`). CLI closure scan (`lw10b-tools/closure.ts`, 407 files; lw10a's list has 607): 0 bare `@semio-tech/*` specifiers; none of my bare rewrites is in lw10a's closure list (checked).
- New law: `bun test ./…/🔗️relative-closure/🧪️tests/🟦️.ts` 2 pass.
- Seal port: ran the transpiled `sealNxSdkSourceIndexV1` with stub ports: order `check, custody, check, projection admission, check, check, publication admission, check, lstat` (same order as the old dynamic imports).
- NOT verified at runtime: the harness-bound suites (`Original parent Script process invocation required` under plain `bun test`; `bun repo exec` stops in the dependency inspect with `Locked workspace package lacks original admitted Source`, the same state lw10a reports). So the phase-law test I changed, the print refusal test, the quiz tests and all moved-symbol suites are verified by tsc and module resolution only, not by execution.

## 6. Incidents and handover notes
- My codemod (`lay9-tools/movesym.ts`) corrupts a file when one import declaration has names moving to TWO destinations in one run (clause edits overlap). It garbled 3 files (`📋️plan/🕸️graph`, `📥️descriptor/🕸️nodes`, `📋️plan/🧪️tests/🔗️binding`); lw10a noticed 2 of them in flight. All repaired and checked with a repo-wide scan for `import {…} .ts"` lines (0 hits). Rule: run one destination per invocation.
- New files contain relative process imports copied from their parents (`📋️contract`, `🎛️control`, `📋️projection`); a lw10a re-run converts them like the rest.
- The empty directory `🗂️workspaces/🦀️cargo/🛠️preparation/🎛️control` (from 10-14) now holds the control contract.

## Tools kept (`lw10b-tools/`)
`cruise.sh`, `quick.sh` (include-only cruise of a few directories), `ncpr-b.ts` (plan, apply, plan-closure, apply-closure), `exports-needed-b.ts`, `exports-needed.json`, `closure.ts`, `cli-closure-union.txt`, `scc.ts`, `agg.ts`, `pairs.ts`, `targets.ts`, `split.ts`, `cycles.ts`, `resolvecheck.ts`, `showplan.ts`, `showdirs.ts`, `showfix.ts`, `pk.ts`, codemod plans `mv-*.json` (run with `lay9-tools/movesym.ts`), `tsconfig.products.json`, `tsconfig.hub.json`, `ncpr-changed.txt`. Generated output (`🗑️generated/lw10b-run`: cruise and tsc reports) deleted.
