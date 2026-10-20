# result-uncached-outputs: explicit outputs for the 809 cacheable-without-outputs targets (2026-10-20)

Executor `uncached-outputs`. Static only: no `bun repo` run, no nx graph load, no cargo, no Bazel action, no test run. Inputs: `brief-1019.md`, `bzl-stage-uncached-report.md` (809 targets in 278 projects, reason `unbounded`), `result-store-out.md`, `result-cache-{fw,s,rest}.md`. `C` = the coordinator ticket folder.

## Result: counts by resolution (all 809 accounted for)

| Resolution | Targets | Manifest edit |
|---|---|---|
| verdict-only, explicit `outputs: []` | 795 | `"outputs": []` added |
| - artifact-rust standard runner (`test`: 112, `test-snapshot-sqlite[-native|-source]`: 275) | 387 | |
| - TypeScript artifact standard runner (`test`) | 77 | |
| - custom handler, statically traced (registered class or lifted program, 0 workspace-anchored writes) | 284 | |
| - custom handler, hand-read (generators' check twins, stdout-only inventories, repo laws, flagged by the analysis) | 47 | |
| writes undeclared files / not a function of declared inputs: `cache: false` | 9 | `"cache": false` set |
| outputs are injected by project-assembly from a taxonomy generator contract: manifest unchanged | 3 | none (see below) |
| target no longer exists (report is stale) | 2 | none |
| exact non-empty `outputs` list | 0 | none found: every file-writing target is a generator (contract-injected), a store publisher (already receipted by store-out) or argument-dependent |
| **Total** | **809** | 795 + 9 edits in 278 manifests; dangling `dependsOn` fixed (1 removal) |

Static recount (`bun C/bzl-stage-tools/graph.ts` reconstruction with the harness' own `effective` cache rule and policy defaults, `cache === true && !Array.isArray(outputs)`): 809 unbounded before, **4 after**: the 3 contract-injected generators below plus `@leutwiler/realparts-of-powers-z-n:build`, a `repo:noop` aggregate the real adapter always caches (`noop ? {cache:true}`), not in the 809.

## How each target was resolved (handler-read evidence)

Script: `C/uncached-outputs-tools/` (`run.sh [--write]` runs it end to end; every step re-runnable and deterministic, two `decide.ts` runs are byte-identical, a second `apply.ts` is a no-op).

1. `targets.ts` parses the report; each target is resolved to its manifest, `options.cwd` and `bun ./📜️script.ts <command>`.
2. `walk.ts`/`module.ts` read the handler: the command word selects the router registration (`register`, `registerLazy`, inline class) or, for lifted programs and standard runners, the script body; the call graph is followed by name through owner code (barrel re-exports, dynamic imports, namespace imports), foreign command branches (`segments[0] === "<other>"`, `command === "<other>"`) are stripped, `bun test <file>` arguments are followed into the test file. Framework infrastructure is not descended; its calls are recorded.
3. `flow2.ts` finds every fs write primitive in the reached functions and taints its path argument by data flow: scratch (`mkdtemp`, tmpdir, temp/scratch names) versus workspace-anchored (`repoRoot`, `import.meta.dir`, `process.cwd()`, ...). A target with any workspace-anchored write site, a suspect infra call (build/publish/install/retire/pin/store) or an unclassified program is **not** auto-resolved: it is hand-read (`overrides.ts`, one cited reason per row).
4. Standard runners were read once (`⚡️caching/📦️artifacts/🦀️rust/🟦️.ts` `runArtifactRustPackageMain`: `test` = cargo test in the leased compiler store, `check` = cargo check, `test-snapshot-sqlite` = cargo `sqlite_snapshot_` tests plus `bun test` of the listed suites; `🟦️typescript/🟦️.ts`: `test` = bundle into scratch + products store, tsc, bun suites). The 209 existing bun files they run (29 snapshot-sqlite suites, 180 TypeScript artifact suites; 1 listed file is missing, open item 2) were traced too: 5,980 reachable owner functions, 0 workspace-anchored write sites, 5 other sites (4 `Bun.write(Bun.stdout, ...)` stdout writes and one `writeFileSync` of a suite-local temporary file).
5. Rust test sources (144 artifact directories, `rustscan.ts`/`rustgate.ts`): 91 fs-write sites next to `CARGO_MANIFEST_DIR`/`current_dir`; 68 are gated by an env var or write/bless flag, the 23 ungated ones are `#[ignore]` blessing tests (puzzle-5d fill-run, bmp/tiff fixture derivation) or belong to the repo-dashboard journeys (uncached below). No default-run Rust test writes a tracked file.
6. `apply.ts` re-reads every manifest right before writing, edits by span (untouched text, inline objects and concurrent edits stay intact), proves `parse(after) == parse(before) + intended keys` and re-compares the file before an atomic rename. 0 targets had changed since the report (all 804 edits applied; 0 `already-declared`).

Per-target evidence (handler, reachable functions, write sites, spawned processes, reason): `C/uncached-outputs-tools/evidence.md` (809 rows).

## Targets set to `cache: false` (9; the schema has no reason field, none was invented)

| Target | Reason |
|---|---|
| `@semio-tech/repo-dashboard-rs:test` | TestScript installs the native dashboard binary (installDashboard: temp write + rename into the tooling store), spawns daemon/pty journeys and bun suites against that binary, and writes test artifacts under .🧬semio/🦑️repo/⚡️cache/tests/dashboard-*; host state outside the declared inputs |
| `@semio-tech/repo-dashboard-rs:test-exhaustive` | same TestScript as repo-dashboard-rs:test (level exhaustive): installed native binary, daemon and pty journeys, undeclared cache-directory artifacts |
| `@semio-tech/repo-dashboard-rs:test-long` | same TestScript as repo-dashboard-rs:test (level long): installed native binary, daemon and pty journeys, undeclared cache-directory artifacts |
| `@semio-tech/repo-dashboard-rs:test-quick` | same TestScript as repo-dashboard-rs:test (level quick): installed native binary, daemon and pty journeys, undeclared cache-directory artifacts |
| `@semio-tech/s-dev-composition:preview-generated` | PreviewGeneratedScript emits its product as a JSON protocol on stdout after reading the projected-input payload from stdin (SEMIO_GENERATOR_PREVIEW_PROTOCOL) and a cancel file: neither is a declared input, no file output exists to restore |
| `workspace:test-report` | ReportScript reads the results of the latest local run from the test cache directory and writes reports/latest/* there: depends on host state and writes undeclared files |
| `workspace:verify-gate` | runGate re-enters the build tool (bun repo run / run-many --skip-cache for canonical-architecture, plugin-registry:check, lint and check targets): an orchestrator, not a pure function of its declared inputs |
| `workspace:verify-layering` | runLayering re-enters the build tool (bun repo run-many -t lint-dependency-direction ... --skip-cache): an orchestrator |
| `workspace:verify-typescript-project` | writeTypescriptProject writes the generated tsconfig (default .cache/semio/typescript/🔣️tsconfig.json or the --out argument that forwardAllArgs passes through) from the live checkout; its only product is that argument-dependent file |

## Contract-injected (manifest unchanged, 3)

`project-assembly` (`generatorOutputOwners`, assembler line 162) sets `outputs` of every `generatorContracts[*].target` from the contract's `outputRoots`, overriding any manifest list; the static harness in `C/bzl-stage-tools/graph.ts` does not run that inference, hence the report. A manifest list would be a second authority that the assembler replaces anyway.

| Target | Contract |
|---|---|
| `@semio-tech/framework-os-dev:generate-distribution` | publishes the distribution bundle; outputs are injected from generatorContracts[dev-distribution-bundle].outputRoots |
| `@semio-tech/framework-os-dev:generate-scale-fixture` | writes the scale fixture; outputs are injected from generatorContracts[scale-fixture].outputRoots |
| `@semio-tech/framework-os:generate-wgpu` | writes the six owned wgpu leaves; outputs are injected by project-assembly from taxonomy generatorContracts[wgpu-frame-worker].outputRoots (generatorOutputOwners), a manifest list would be a second authority |

Their check twins (`check-wgpu`, `check-distribution`, `check-generated`) are compare-only and got `outputs: []`.

## Stale

`@semio-tech/layout-js:test-renderer-contract` and `@semio-tech/puzzle-js:test-renderer-contract` no longer exist in the manifests (`procedural-js` and `raster-js` still define theirs and were resolved).

## Dangling `dependsOn` fixed

`✏️s/🧑‍💻dev/📋️project.json` `typecheck.dependsOn`: removed `@semio-tech/trinity-jack-lsp:wasm`. It is not a rename: `@semio-tech/trinity-jack-lsp-worker` defines only `test`, so repointing it would still be refused by `requireBazelDependencyV1`. The Rust crate that produced the `wasm` target (`✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust`) is deleted at HEAD (auto-commit fd14da55f18; only an untracked `pkg/` remains), so there is no producer left to depend on. See open item 1. The three `semio-hub-gis-component:component` references in the report are harness artifacts: the crate exists (`🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml`), its project and `component` target are inferred from Cargo.

## Hand-read verdicts (47)

| Target | Evidence |
|---|---|
| `@semio-tech/architect-program-rs:check-register-catalog` | generateCollectionCatalogV1(schema, check): renders, compares with disk and returns the stale list; the write loop is gated on mode write |
| `@semio-tech/assets:check-generated` | CheckGeneratedScript calls checkAssetArtifacts (compare only) |
| `@semio-tech/draw-js:test` | bare test falls through the segments.length branch: bun tests, typecheckPathRaster and proveDrawPublicationAuthority (reads); the Bun.write sites belong to the separate test font-shape unicode provisioning command |
| `@semio-tech/framework-graph:check-generated` | CheckGeneratedScript renders, compares inventory and file bytes, throws when stale, then runs the graph suite (scratch test artifact root) |
| `@semio-tech/framework-os-dev:check-distribution` | DistributionBundleScript check branch calls checkDistributionBundle (byte comparison) and throws when stale; publishDistributionBundle is the generate branch |
| `@semio-tech/framework-os:check-wgpu` | runWgpuPackageGenerator mode check throws on stale output before the write loop; the write loop is reached only in generate mode |
| `@semio-tech/gis-gismap-js:tiles-check` | proveGisMapTilesV1: coordinate/plan oracles and a loopback HTTP server; tile writes go to a mkdtemp sandbox under the artifact root that is removed in finally |
| `@semio-tech/gis-gismap-js:tiles-check-native` | tiles-check --native adds the native lane to the same sandboxed proof |
| `@semio-tech/repo-lib:test-inventory-artifact-shards` | bun test of 🧪️tests/💠️inventory-artifact-shards; every write goes to mkdtemp roots under SEMIO_TEST_ARTIFACT_DIR (scratch) and the suite removes its shards |
| `@semio-tech/s-dev-composition:check-generated` | catalog-publication check selects CheckGeneratedScript: byte comparison of the catalog output root |
| `@semio-tech/s-dev-composition:verify-catalog` | catalog-publication verify selects CheckScript: byte comparison of the catalog output root |
| `@semio-tech/s-fem-mesh:test-neutral-surface` | bun test of the surface suite; repoTestArtifactEnvironment only creates the ignored test-output sink under .🧬semio/🦑️repo/⚡️cache/🧪️tests |
| `@semio-tech/s-fem-mesh:test-surface-contract` | bun test of the surface contract; ignored test-output sink only |
| `@semio-tech/s-gis-services-native:mcp-check` | native cargo check --manifest ... --bin: cargo check in the leased compiler store, no artifact is consumed |
| `@semio-tech/s-native-ownership:test` | executeCommandV1 runs bun test of 🟦️.ts; artifacts (cache, leases, retained output) go to SEMIO_TEST_ARTIFACT_DIR or the ignored 🤖️generated folder next to the script, never to tracked sources |
| `@semio-tech/space-home-rs:mutation-inventory` | list-mutations prints the validated mutation inventory of the bridge binary on stdout (delivered through writeOwnedCommandOutputV1); no file is written, a cache hit replays stdout |
| `@semio-tech/space-home-rs:mutation-inventory-check` | bridge check branch: runAdmittedCargoCommandV1 cargo test --bin of the mutation bridge in the leased compiler store; the stdout-inventory branch is the separate list-mutations command |
| `@semio-tech/space-home-rs:test-complete-app-document` | bridge program with forwardAllArgs false; no write path (open defect: argv test reaches the exact-coordinates branch, see report) |
| `@semio-tech/space-space-rs:mutation-inventory` | list-mutations: stdout inventory only, like space-home-rs:mutation-inventory |
| `@semio-tech/space-space-rs:mutation-inventory-check` | bridge check branch: runAdmittedCargoCommandV1 cargo test --bin of the mutation bridge in the leased compiler store; the stdout-inventory branch is the separate list-mutations command |
| `@semio-tech/space-space-rs:test-complete-app-document` | same bridge program as space-home-rs:test-complete-app-document (same open defect) |
| `@semio-tech/stdio-gltf-rs:native-schema-check` | cargo schema lane through prepareCargoWorkspaceInvocation in the preparation store; mkdirSync only creates the ignored test artifact directory |
| `@semio-tech/stdio-js:package-contract` | verifyStdioCommandOwnership and testStdioArtifactPackageContract read and compare; the rust toolchain and tooling store helpers write only into the managed tooling store |
| `@semio-tech/stdio-js:test` | StdioCompositionTestScript: no-output checkArtifactTypeScriptSourceV1 plus bun suites; the same shape as every TypeScript artifact test (project-assembly sets outputs = [] for those) |
| `@semio-tech/stdio-json-rs:native-schema-check` | same lane as stdio-gltf-rs:native-schema-check |
| `@semio-tech/stdio-semio-rs:conversion-definition-check` | composition check runs runSemioConversionDefinitionChecks (checks only); prepareSemioConversionDefinitionV1 writes only under composition prepare |
| `@semio-tech/ui-styling-tokens:check-generated` | CheckGeneratedScript calls checkStylingArtifacts (compare only); fonts download is the separate fonts command |
| `@semio-tech/wfc-2d-rs:mutation-inventory` | list-mutations: stdout inventory only |
| `@semio-tech/wfc-2d-rs:mutation-inventory-check` | bridge check branch: runAdmittedCargoCommandV1 cargo test --bin of the mutation bridge in the leased compiler store; the stdout-inventory branch is the separate list-mutations command |
| `@semio-tech/wfc-3d-rs:mutation-inventory` | list-mutations: stdout inventory only |
| `@semio-tech/wfc-3d-rs:mutation-inventory-check` | bridge check branch: runAdmittedCargoCommandV1 cargo test --bin of the mutation bridge in the leased compiler store; the stdout-inventory branch is the separate list-mutations command |
| `@semio-tech/wfc-bitmap-rs:mutation-inventory` | list-mutations: stdout inventory only |
| `@semio-tech/wfc-bitmap-rs:mutation-inventory-check` | bridge check branch: runAdmittedCargoCommandV1 cargo test --bin of the mutation bridge in the leased compiler store; the stdout-inventory branch is the separate list-mutations command |
| `@semio-tech/wfc-grid2d-rs:mutation-inventory` | list-mutations: stdout inventory only |
| `@semio-tech/wfc-grid2d-rs:mutation-inventory-check` | bridge check branch: runAdmittedCargoCommandV1 cargo test --bin of the mutation bridge in the leased compiler store; the stdout-inventory branch is the separate list-mutations command |
| `@semio-tech/wfc-grid3d-rs:mutation-inventory` | list-mutations: stdout inventory only |
| `@semio-tech/wfc-grid3d-rs:mutation-inventory-check` | bridge check branch: runAdmittedCargoCommandV1 cargo test --bin of the mutation bridge in the leased compiler store; the stdout-inventory branch is the separate list-mutations command |
| `workspace:artifact-field-parity-test` | VerifyScript artifact-field-parity test runs testArtifactFieldParityOracle only and returns |
| `workspace:test-discover` | DiscoverScript lists discovered test cases on stdout (discoverTestCases is a read-only scan) |
| `workspace:verify-dependencies-freeze` | runDependencyVerification: dependencyFreezeCheck against the committed baseline, read-only (write-baseline is a separate mode) |
| `workspace:verify-dependencies-literal-external` | runDependencyVerification literal-external: dependencyTruthReport, read-only |
| `workspace:verify-interactivity` | runInteractivityAudit: scans and prints findings, no writes |
| `workspace:verify-taxonomy-enforce` | runTaxonomy enforce: verifyTaxonomy reads the checkout and prints findings |
| `workspace:verify-taxonomy-implementation-enforce` | taxonomy implementation lane: findings printed, throws in enforce mode |
| `workspace:verify-taxonomy-implementation-report` | taxonomy implementation lane in report mode: prints findings |
| `workspace:verify-taxonomy-report` | runTaxonomy report: same read-only verification, report mode only prints |
| `workspace:verify-typescript` | runTypescriptVerification check: the generated project and tsc logs go to the ignored .cache/semio/typescript; the verdict is the exit status (open: project file is a byproduct) |

## Changes

- Edited: 278 `📋️project.json` manifests (795 `"outputs": []`, 9 `"cache": false`, 1 `dependsOn` removal). New tool folder `C/uncached-outputs-tools/` (targets, closure, module, walk, flow2, writes, files, classify, decide, overrides, jsonspan, apply, validate, report, result, rustscan, rustgate, flagged, run.sh, evidence.md). No edits to `package.json`/`bun.lock`, library code, `🔣️taxonomy.json`, `policy.json` or `.vscode/`.

<details><summary>Edited manifests (278)</summary>

- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/⚙️engine/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/📋️project.json`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/🗂️catalog/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🪐️space/🫀️catalog-documents/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🧑‍💻dev/🌊️flow/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧑‍💻dev/📋️forms/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧑‍💻dev/📋️project.json`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🧑‍💻dev/📕️norm/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧑‍💻dev/🗒️note/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧑‍💻dev/🪐️space/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧪️tests/🏘️plugin-composition/📋️project.json`
- `✏️s/🧪️tests/🗂️native-ownership/📋️project.json`
- `📋️project.json`
- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`

</details>

## Verification

| Command | Result |
|---|---|
| `bun C/uncached-outputs-tools/decide.ts` (twice, `cmp`) | 809 decisions, byte-identical |
| `bun C/uncached-outputs-tools/apply.ts` (dry run), then `apply.ts --write` | 795 `outputs-added`, 9 `cache-disabled`, 1 `dependsOn-removed`, 278 manifests; per-manifest semantic equality proof held |
| `bun C/uncached-outputs-tools/apply.ts` again | 795 + 9 `already-declared/uncached`, 0 manifests (idempotent) |
| `bun C/uncached-outputs-tools/validate.ts` (repo `validateJsonSchemaSubset` against `🏗️build/🧬️schema/📋️project/🔣️.json`, plus Ajv as third-party cross-check) | 278 manifests, 0 repo-validator failures, 0 Ajv failures; 804 of 804 editable targets now declare `outputs` or `cache: false`, 0 unresolved |
| `bun C/bzl-stage-tools/graph.ts` static recount of `cache && !outputs` | 809 to 4 (3 contract generators + 1 noop aggregate outside the 809) |
| `bun build --target=bun` of the tool entry points | ok |
| NOT run | `bun repo show projects` / nx graph, real Bazel `bazelUncachedReportV1` on the inferred graph (expect the same 3 generators resolved by the assembler), any repo test suite, any target |

## Open items

1. **LSP worker without producer.** `@semio-tech/trinity-jack-lsp-worker` (`🧠️lsp/🟦️.ts`) still imports `./📦️packages/🦀️rust/pkg/trinity_jack_lsp.js`, which only exists as untracked residue because the Rust crate is absent at HEAD; `📇️catalog.json` (vite engines) still lists `@semio-tech/trinity-jack-lsp`. Owner decision: restore a producer (then re-add its `wasm` dependency under the new name) or retire the worker, the engines catalog entry and the `s-dev` typecheck import.
2. **Remodel snapshot suite path.** `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/📜️script.ts` lists `…/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️schema/🟦️.ts`, which does not exist (the schema test lives at `…/📸️snapshot/🗄️schema/🟦️.ts`); `remodel-remodeling-rs:test-snapshot-sqlite` and `-source` fail at the bun run.
3. **Bridge `test` dispatch.** `space-home-rs`/`space-space-rs:test-complete-app-document` run `bun ./📜️script.ts test` in the mutation-bridge script whose program only handles `check`; `test` falls to the exact-coordinates branch and throws. Resolved as verdict (no write path) but it cannot pass until the command is wired.
4. **Central policy alternative.** `⚡️caching/🔣️policy.json` `targetDefaults.test` has no `outputs`, and `targetDefaults` match exact names, so `test-*`, `verify-*` and `check-*` variants never inherit `outputs: []`; adding `"outputs": []` to `test` would cover only the 255 targets named exactly `test`, not the 142 `verify-*` or the `test-*` variants; manifest-level declarations (this change) stay explicit and tool-neutral.
5. **TypeScript artifact `test`** (77 targets) runs `context.build()`, which also publishes the package into the products store and binds the `dist` links; declared `outputs: []` like project-assembly does (`policy.outputs = []` for artifact `test`), because `build` owns the receipt and publication is content-addressed and idempotent. A cache hit does not re-bind the links; consumers depend on `build`.
6. **Stdout products.** `list-mutations` (7), `workspace:test-discover` and the taxonomy/dependency verifiers print their result; nx replays terminal output on a hit, the Bazel adapter's replay of stdout for cached actions was not checked.
7. **Test sinks.** Several tests write scratch under `SEMIO_TEST_ARTIFACT_DIR` or the ignored `.🧬semio/🦑️repo/⚡️cache/🧪️tests/<route>` and `🤖️generated` folders (`repoTestArtifactEnvironment`, `s-native-ownership`, `repo-lib:test-inventory-artifact-shards` whose default points at a ticket `🗑️generated` folder). They are discarded sandbox output, not products; the Bazel adapter must provide the variable (`test-catalog-publication` throws without it).
8. **`workspace:verify-typescript`** keeps `outputs: []` although it writes the generated tsconfig and tsc logs under `.cache/semio/typescript` as byproducts; its sibling `verify-typescript-project`, whose only product is that file, is uncached.
