# audit-cache-keys-1026g (aud-keys-g, wave 1026g, read-only)

Scope: cached targets of the three product closures. Dashboard = `@semio-tech/repo-dashboard-rs` plus the 47 other `@semio-tech/repo-*-rs` crates (48 projects, 55 cached targets). Course = `@teaching/architecture-course`, `@semio-tech/course*`, `pets`, `pets-react`, `pets-rs`, the `ui-*` packages the course uses (`ui-styling`, `ui-styling-tokens`, `ui-react`, `ui-rs`, `ui-contract-rs`), the framework umbrellas (`framework`, `framework-rs`, `framework-actor-rs`) and `assets` (16 projects, 122 cached targets). Print = `print`, `print-rs`, `print-viz-inference`, `presentation`, `presentation-react`, `mit-bestand-bericht`, `mit-bestand-praesentation-projektetage` (7 projects, 34 cached targets). Total 211 cached targets.

Scratch and tools: `C/🗑️generated/aud-keys-g/` (`resolve.py`, `drive.py`, `files.txt` = 197,944 non-ignored repo files from `git ls-files -co --exclude-standard`, `touched48.txt` = files in commits of the last 48 h, `appendix.md`). Deletable.

## 1. Method and limits (read this before the numbers)

- Declared = resolved from the `📋️project.json` files only: named inputs, `inputs`, `{projectRoot}`/`{workspaceRoot}` globs, `!` negations, `reads`, and `targetDefaults` of `⚡️caching/🔣️policy.json` when a target has no `inputs`. Named-input recursion is included; `^dep` entries and `dependentTasksOutputFiles` are counted separately, not expanded; extglobs are reported as unsupported (only in `production`).
- NOT included in "declared", because the graph inference adds them (`🏗️build/🕸️graph/📥️inference/`, see `📥️inputs/🟨️.mjs` 255-300 and `🧱️project-assembly`): the owner-tree glob for Rust targets, the Cargo crate closure, the command import closure (`SEMIO_SOURCE_CLOSURE_*`), the toolchain files/env/commands of each tool, the `!generatedDirectories` negations, `testSourceExclusions` (heads `build`/`generate`) and `schedulingSourceExclusions`, and the precision rules. So for Rust targets the declared count is a lower bound of the effective key (the owner glob alone is 28,159 source files under `🧰️framework/`, section 4), and for test-excluded build targets it is an upper bound. Where the effective size was measured live or offline I quote those numbers from the result files instead.
- "Touched in 48h" = files of the declared set that appear in any commit of `git log --since=48.hours` (one batch commit per directory in this window, so this counts distinct files changed, not edits).
- Reference data from the result files: exec-f (executor identity, command closure, scheduling overlay), cinp-d (test exclusions, `typegen` keys), gen-d (precision rules for Cargo generators, measured offline on generation 363), nx4-d (print closures).
- Dependency closures (`^default` over ~100 crates/packages) are not enumerated; they are reported as a qualitative issue (#6).

## 2. What feeds each key (summary)

| Component | Source | Notes |
|---|---|---|
| Declared inputs | `namedInputs` + `inputs` of the target in `📋️project.json`; `targetDefaults` (`production` for build/package/wasm/native-build, `default` for lint/test/typecheck/check/generate) | `default` = `{projectRoot}/**/*` + `sharedGlobals`; `reads` (113 projects have one) = whole sibling trees |
| Inferred additions | owner glob for Rust (`{owner}/**/*.{rs,toml,…}`), Cargo crate closure, command import closure, toolchain `files`/`environment`/`commands`, generated-dir negations, test/scheduling exclusions, precision rules | `📥️inputs/🟨️.mjs` 255-300; exclusions in `🧱️project-assembly` |
| Global env | `sharedGlobals`: `SEMIO_BUILD_MODE` | runtime: `node -p platform+arch`, `bun --version` |
| Tool env | `toolchains.javascript.environment` (9 vars), `toolchains.cargo.environment` (35 vars), wasm, go, python, dotnet, cmake lists | appended to every target of a project that uses the tool |
| Test env | `testEnv` named input (`SEMIO_TEST_LEVEL`, `SEMIO_TEST_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS`) on test targets (copied into 278 project files) | budgets are timing knobs, not outcomes |
| Target env | `PROCTOR_URL` (course build), `PROCTOR_COURSE` (course check) | |
| Tool pins | bun version; `node --version` command; `rustc -vV`, `cargo --version`; `rustfmt.toml`, `Cargo.toml`, `.cargo/config.toml`, `rust-toolchain.toml`, workspace `Cargo.lock`; `WASM_BINDGEN_VERSION`; root `bun.lock`; `bunfig.toml`, `tsconfig.json`; `**/package.json` under framework (134 files for the course test) | no TeX/tectonic pin (#10) |
| Executor identity | `implementation.executor-command.json` (keyed snapshot, 1.9 MB) plus `implementation.executor-command.scheduling.json` (overlay, 3.0 MB) | exec-f: monolithic snapshot was 4.8 MB; the scheduling row is still missing from the declared scope table |
| Outputs | `outputs` of the target, excluded from its own inputs when they overlap sources | `generate-actor-network` writes into a tree that a sibling test declares (#9) |

## 3. Per-product picture

- Dashboard (48 projects, 55 cached targets): median declared 5 files, but 5 `test-*` targets of `repo-dashboard-rs` declare `reads` = `📚️library/**/*` (5,030 files) plus the dashboard tree: 5,372 files each, 1,965 touched in 48 h (37 %). `build`, `compile`, `run`, `install` and the umbrella `test` are `cache: false`, so the Rust binary itself has no key (not in scope of this audit, noted for the 2-min target).
- Course (16 projects, 122 cached targets, 81 of them with ≥ 1,000 declared files): `framework-rs` 31 targets (3,535 declared, 768 touched), `framework` TS 12 targets (3,313–4,556), `ui-rs` 18 targets (2,396–2,701), `ui-styling` 5 targets (3,568), `pets-react` 2 targets (3,747), `course-react` 2 targets (5,303; 1,903 touched), `architecture-course` `build` 2,199 declared (403 tests), `test` 1,246.
- Print (7 projects, 34 cached targets): `@semio-tech/print` 11 targets at 951 declared files each (498 tests, 52 %; 195 touched in 48 h, 21 %); `mit-bestand-bericht` `test` 1,914 files (1,719 under `bericht/**/*`); `presentation` targets 7–44 files.

Churn by directory, last 48 h (distinct files in commits): `🦑️repo` 2,504; `🦑️repo/📚️library` 1,848 (build library); `🖱️ui` 529; `🏃️process` 432; `♻️mit-bestand/🎤️präsentation` 45; `📓️print` 195; `📚️course` 81; `🐾️pets` 56; `🎓️teaching` 69; `🎤️presentation` 68; `♻️mit-bestand/📋️bericht` 4; `🎛️dashboard` 114; `🧰️framework/📦️packages` 18.

## 4. Top 15 key-widening problems (ranked by declared-size × churn × reach)

1. **Framework owner glob on every Rust target.** The inferred owner input for `framework-rs`/`framework-actor-rs` is `🧰️framework/**/*.{rs,toml,json,…,ts,css,…}`: 28,159 source files, 5,044 touched in 48 h. Measured: `framework-actor-rs:typegen` 17,716 → 10,658 and `framework-rs:generate` 18,761 → 11,352 (cinp-d), then 623 and 1,095 offline with gen-d's precision rules (live proof pending). Fix: land the gen-d precision rules (`actor-typegen`, `framework-generate`, `ui-contract-generate`) and make the owner glob apply only to test targets that name it.
2. **Dashboard tests key on the whole build library.** `reads: 📚️library/**/*` (5,030 files, 1,965 touched) on `test-dispatch`, `test-language`, `test-source-freshness`, `test-unit`, `test-windows`. Fix: replace `reads` by the per-command crate closure and exclude `🏗️build/**` unless a test imports it.
3. **Course React keys on the repo library.** `course-react` `test` and `typecheck` declare `reads: 📚️library/**/*` (5,303 files, 1,903 touched, 36 %): a cross-product coupling of a web target to the build tool. Fix: `reads` → import closure of the target (router/precision engine), no product-level sibling trees.
4. **UI tree as `reads` in 15 projects.** `🖱️ui/**/*` (2,351 files, 528 touched) is `reads` for `framework-rs` (31 targets), `framework` TS (12), `ui-rs` (18), `ui-styling` TS (5), `pets-react` (2): 2.3k–3.7k files per target. Fix: per-package `reads` narrowed to the imported modules (the `test-io-ports` pattern), keep `reads` only for generated or non-imported inputs.
5. **Framework TS umbrella reads seven module trees.** `@semio-tech/framework` `reads` = `🖱️ui`, `🚪️io`, `🏗️mesh-engine`, `📄️document`, `🛂️manifest`, `🧬️schema`, `📏️script-boundary` (3,313–4,556 files per target, 662–1,092 touched). Fix: give each `test-*` target the module it tests as its only `reads`.
6. **`^default` in test targets.** `nx.json`/policy `targetDefaults.test.inputs` = `default, ^default`: the test files of every dependency key every dependent test (`framework-rs` test alone keys 538 test files of its own closure plus its dependencies'). Fix: `^production` for test targets; `^default` only where a dependency generates an input.
7. **Course `build` keys its tests, fixtures and e2e.** `default` lists `🧪️tests/**/*`, `🧫️fixtures/**/*`, `🎭️e2e/**/*`, `🧪️tests/🏋️capacity/**/*`, plus all of `🐾️pets/**/*` (371) and the framework TS umbrella (294): 403 test files of 2,199 declared. `testSourceExclusions` is void because explicit test globs keep the target full (cinp-d rule). Fix: `build` inputs = `production`; move those globs to the `test` target.
8. **Print `reads` = the whole product.** `reads: 📓️print/**/*` (951 files, 498 tests, 195 touched) on all 11 print targets, including `check`, `preview-generated`, `test-viz*`, `test-measurement`. Fix: `reads` → `production` of print with `🧪️tests` and `📊️visualization-gallery` removed; test-only globs on `test*` targets.
9. **`bericht` test keys its own generated output.** `test` inputs `♻️mit-bestand/📋️bericht/**/*` (1,719 files) include the `📎️anhang/*.tex` files written by `generate-actor-network` (cache: true, outputs inside the same tree), so `test` re-keys after each regeneration even when nothing else changed. Fix: exclude generated `📎️anhang/**/*.tex` from `test` inputs and declare them as dependent outputs of `generate-actor-network`.
10. **MISSING, TeX/tectonic identity.** `deps-tectonic` and `deps-tex` are `cache: false` with empty `outputs`; print `build`, `test-native-grammar` and the three bericht `build-*` targets depend on them; `tectonic`/`tex` appear in no `toolchains` entry (only `serialTargets` names). A TeX upgrade therefore reuses stale PDFs (wrong hit). Fix: add a `tex` toolchain (binary path, `tectonic --version`, bundle digest) and make `deps-tex` write a fingerprint output.
11. **Javascript env on every JS key, including builds.** `toolchains.javascript.environment` = `NODE_ENV, BUN_ENV, CI, SEMIO_BUILD_MODE, SEMIO_RENDERER, SEMIO_PLUGIN, SEMIO_TEST_LEVEL, SEMIO_TEST_BUDGET_MS, SEMIO_BUILD_BUDGET_MS` is appended to every target of every JS project (`📥️inputs/🟨️.mjs` 265-270): CI vs local and budget tuning split builds that do not read them. Fix: shared key keeps `SEMIO_BUILD_MODE` only; budgets move to runner options; per-target env only where read.
12. **Cargo env contains machine paths.** `toolchains.cargo.environment` (35 names) includes `CC, CXX, SDKROOT, DEVELOPER_DIR, LDFLAGS, CFLAGS, MACOSX_DEPLOYMENT_TARGET, SEMIO_WASM_BINDGEN_BIN, SEMIO_WASM_OPT_BIN` (absolute tool locations) and per-profile `CARGO_PROFILE_*`: keys differ between macOS, devcontainer and Linux and between shells that export different values. Fix: key compiler and tool identity by `rustc -vV` and binary digest, drop path variables from keys.
13. **Executor identity still monolithic in the declared scope table.** exec-f open item 1: the `executor-command` row of `🏗️bazel/🎬️scopes/🔣️.json` has no `scheduling` domain, so `course:build` keys 354 executor files (217 scheduling/custody). Pair 1 of exec-f: a scheduling probe re-executed 6 actions. Fix: add `"scheduling": ["leases","capacity","store-custody","process-supervision","retry-observation","progress-reporting"]` to that row (laws already green).
14. **Cargo native fallback closure.** `nativeCommandInputs` (1.9k files) keys the four Rust generators and `course:build` through the cargo closure because the cargo router cannot tell which cargo command the target runs (exec-f open item 2). Fix: name the cargo command in each target (`--command`) so the router prunes like the TS routers.
15. **Router closure = union of commands, and conflicting measurements.** `@semio-tech/ui-styling-tokens:generate` (a print dependency) has a 2,444-file closure with 413 tests (nx4-d, 14:08), because the package router imports `vitest` → `repository-tooling`. exec-f reports the router pruning of `ui-styling` `generate` from 2,369 to 51 files. Fix: verify the pruned closure on the live graph for the print path (nx4-d surrogate vs graph), then route the Nx closure through the same per-command pruning.

## 5. MISSING inputs (undeclared reads, wrong-hit risk)

- M1. Dashboard Rust reads `SEMIO_REPO_IMPLEMENTATION` (`🏛️repo-domain`), `SEMIO_DASHBOARD_GRAPH` (`🎮️registry`), `SEMIO_CATALOG_PUBLISHER_PROJECT` (`🔌️plugin-registry`), `SEMIO_DASHBOARD_URL_OPENER` (`🧭️cli/🌐️open`), and `SEMIO_DASHBOARD_INSTANCE` (`🌀️daemon/✉️ipc`). The `test-*` targets declare only `SEMIO_TEST_LEVEL` and the budgets. Verify per test whether an outcome changes with these; if so they must be in the key.
- M2. `task-routes` sets `SEMIO_TEST_PATH ??= process.env.PATH`: `PATH` is per shell and per agent. If it reaches a cached test, it is both undeclared and unstable (wrong hits and churn). Verify.
- M3. `deps-tectonic`/`deps-tex` are `cache: false` with empty `outputs`; print `test` and `test-viz*` carry `dependentTasksOutputFiles: **/*` on those uncached producers: either it reads nothing (MISSING) or the whole output tree (TOO WIDE). Undefined until the producers declare outputs (see #10).
- M4. `generatedDirectories` (`build`, `out`, `bin`, `obj`, `pkg`, `coverage`, `dist`, `target`, `__pycache__`, …) are negated from every key by name. Currently no tracked source uses such a name in the product trees (checked against `files.txt`), so this is latent: a future `pkg/` source would be invisible to keys.

## 6. Per-target estimates

All 211 cached targets: declared files (own patterns, no `^` closure, no inferred owner glob), tests among them, files touched in the last 48 h, number of patterns, dependent-output globs, and the largest single pattern. The `Grp` column is D = dashboard, C = course/framework/ui, P = print/bericht.

| Grp | Project | Target | declared files | of which tests | touched in 48h | patterns | dep-output globs | top pattern (files) |
|---|---|---|---:|---:|---:|---:|---:|---|
| D | @semio-tech/repo-events-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-events-rs | test | 7 | 2 | 3 | 4 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-coordinator-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-coordinator-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-move-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-move-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-tree-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-tree-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-dashboard-rs | test-ci-source | 9 | 3 | 6 | 9 | 0 | 1 fw/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️ |
| D | @semio-tech/repo-dashboard-rs | test-dispatch | 5372 | 3234 | 1965 | 6 | 0 | 5030 fw/🛍️products/🦑️repo/🔨️modules/📚️library/**/* |
| D | @semio-tech/repo-dashboard-rs | test-language | 5371 | 3234 | 1965 | 5 | 0 | 5030 fw/🛍️products/🦑️repo/🔨️modules/📚️library/**/* |
| D | @semio-tech/repo-dashboard-rs | test-source-freshness | 5371 | 3234 | 1965 | 6 | 0 | 5030 fw/🛍️products/🦑️repo/🔨️modules/📚️library/**/* |
| D | @semio-tech/repo-dashboard-rs | test-unit | 5372 | 3234 | 1965 | 6 | 0 | 5030 fw/🛍️products/🦑️repo/🔨️modules/📚️library/**/* |
| D | @semio-tech/repo-dashboard-rs | test-windows | 5372 | 3234 | 1965 | 9 | 0 | 5030 fw/🛍️products/🦑️repo/🔨️modules/📚️library/**/* |
| D | @semio-tech/repo-providers-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-providers-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-model-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-model-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-mcp-rs | build | 5 | 0 | 2 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-mcp-rs | test | 163 | 75 | 30 | 4 | 0 | 158 fw/🛍️products/🦑️repo/🔨️modules/⌨️cli/**/* |
| D | @semio-tech/repo-identity-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-identity-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-workspace-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-workspace-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-statutes-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-statutes-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-codebase-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-codebase-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-tickets-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-tickets-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-hooks-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-hooks-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-search-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-search-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-graphql-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-graphql-rs | test | 145 | 64 | 27 | 3 | 0 | 143 fw/🛍️products/🦑️repo/🔨️modules/🔗️graphql/**/* |
| D | @semio-tech/repo-graphql-rs | check-types | 9 | 1 | 0 | 4 | 0 | 7 fw/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️ty |
| D | @semio-tech/repo-graphql-rs | preview-types | 9 | 1 | 0 | 3 | 0 | 7 fw/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️ty |
| D | @semio-tech/repo-graphql-rs | test-types-source | 10 | 2 | 0 | 5 | 0 | 7 fw/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️ty |
| D | @semio-tech/repo-cli-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-cli-rs | build-cli | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-contributors-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-contributors-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-goals-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-goals-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-languages-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-languages-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-todos-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-todos-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-test-runner-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-test-runner-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-metrics-rs | build | 5 | 0 | 4 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-metrics-rs | test | 5 | 0 | 4 | 3 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-yaml-rs | build | 5 | 0 | 3 | 8 | 0 | 3 {projectRoot}/**/* |
| D | @semio-tech/repo-yaml-rs | test | 5 | 0 | 3 | 3 | 0 | 3 {projectRoot}/**/* |
| C | @semio-tech/assets | build | 23 | 2 | 0 | 6 | 0 | 23 {projectRoot}/**/* |
| C | @semio-tech/assets | preview-generated | 1223 | 18 | 5 | 3 | 0 | 1217 fw/🔨️modules/🖼️assets/**/* |
| C | @semio-tech/assets | check-generated | 0 | 0 | 0 | 0 | 0 | 0  |
| C | @semio-tech/assets | generate-logo | 23 | 2 | 0 | 6 | 0 | 23 {projectRoot}/**/* |
| C | @semio-tech/assets | test-mesh-contract | 1223 | 18 | 5 | 3 | 0 | 1217 fw/🔨️modules/🖼️assets/**/* |
| C | @semio-tech/assets | test-tile-proxy-contract | 1223 | 18 | 5 | 3 | 0 | 1217 fw/🔨️modules/🖼️assets/**/* |
| C | @semio-tech/assets | test-dispatch-contract | 1223 | 18 | 5 | 3 | 0 | 1217 fw/🔨️modules/🖼️assets/**/* |
| C | @semio-tech/assets | test-svg-video-contract | 1223 | 18 | 5 | 3 | 0 | 1217 fw/🔨️modules/🖼️assets/**/* |
| C | @semio-tech/framework-actor-rs | test-original-receiving-source | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | test-original-receiving-native | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | test | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | typegen | 0 | 0 | 0 | 0 | 0 | 0  |
| C | @semio-tech/framework-actor-rs | preview-generated | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | wasm | 6 | 1 | 1 | 6 | 0 | 6 {projectRoot}/**/* |
| C | @semio-tech/framework-actor-rs | test-operation-diagnostics-source | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | test-operation-diagnostics-native | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | test-original-owned-async-source | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | test-original-owned-async-native | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | test-original-context-native | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/framework-actor-rs | test-ownership | 1119 | 538 | 203 | 19 | 0 | 404 fw/🔨️modules/🎒️pack/**/* |
| C | @semio-tech/ui-contract-rs | built-tree-retirement-check | 535 | 339 | 71 | 4 | 0 | 531 fw/🔨️modules/🖱️ui/🧬️contract/**/* |
| C | @semio-tech/ui-contract-rs | test | 538 | 342 | 71 | 13 | 0 | 531 fw/🔨️modules/🖱️ui/🧬️contract/**/* |
| C | @semio-tech/ui-contract-rs | conformance | 535 | 339 | 71 | 4 | 0 | 531 fw/🔨️modules/🖱️ui/🧬️contract/**/* |
| C | @semio-tech/ui-contract-rs | check-wasm | 535 | 339 | 71 | 4 | 0 | 531 fw/🔨️modules/🖱️ui/🧬️contract/**/* |
| C | @semio-tech/ui-contract-rs | generate | 531 | 337 | 70 | 1 | 0 | 531 fw/🔨️modules/🖱️ui/🧬️contract/**/* |
| C | @semio-tech/ui-contract-rs | preview-generated | 535 | 339 | 71 | 4 | 0 | 531 fw/🔨️modules/🖱️ui/🧬️contract/**/* |
| C | @semio-tech/ui-contract-rs | check | 6 | 3 | 2 | 9 | 0 | 3 {projectRoot}/**/* |
| C | @semio-tech/ui-contract-rs | test-ownership | 535 | 339 | 71 | 4 | 0 | 531 fw/🔨️modules/🖱️ui/🧬️contract/**/* |
| C | @semio-tech/ui-styling-tokens | generate | 140 | 43 | 33 | 2 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling-tokens | preview-generated | 142 | 43 | 34 | 4 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling-tokens | check-generated | 0 | 0 | 0 | 0 | 0 | 0  |
| C | @semio-tech/ui-styling-tokens | test | 142 | 43 | 34 | 4 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling-tokens | test-rs | 142 | 43 | 34 | 4 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling-tokens | test-py | 142 | 43 | 34 | 4 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling-tokens | test-relative-sizing | 142 | 43 | 34 | 4 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling-tokens | test-verification-contract | 142 | 43 | 34 | 4 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling-tokens | test-color-primitives | 142 | 43 | 34 | 4 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling | generate | 140 | 43 | 33 | 2 | 0 | 140 fw/🔨️modules/🖱️ui/🎨️styling/**/* |
| C | @semio-tech/ui-styling | test | 3568 | 1127 | 533 | 4 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-styling | twin | 3568 | 1127 | 533 | 4 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-styling | test-geometry | 3568 | 1127 | 533 | 4 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-styling | test-asset-transport | 3568 | 1127 | 533 | 4 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-react | lint | 513 | 236 | 138 | 10 | 0 | 306 fw/🔨️modules/🖱️ui/🧱️elements/**/* |
| C | @semio-tech/ui-react | test | 513 | 236 | 138 | 10 | 0 | 306 fw/🔨️modules/🖱️ui/🧱️elements/**/* |
| C | @semio-tech/ui-react | typecheck | 513 | 236 | 138 | 10 | 0 | 306 fw/🔨️modules/🖱️ui/🧱️elements/**/* |
| C | @semio-tech/ui-react | canonical-architecture | 513 | 236 | 138 | 10 | 0 | 306 fw/🔨️modules/🖱️ui/🧱️elements/**/* |
| C | @semio-tech/ui-rs | check-command-types | 2626 | 1309 | 648 | 26 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-command-dispatch | 2626 | 1310 | 648 | 30 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test | 2397 | 1114 | 564 | 30 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-wgpu-engine | 2397 | 1114 | 564 | 30 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | check-wgpu-engine-wasm | 2396 | 1113 | 564 | 21 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | check-wasm | 2396 | 1113 | 564 | 21 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | generate | 64 | 1 | 47 | 17 | 0 | 35 fw/🔨️modules/🎠️kernel/*/🟦️.ts |
| C | @semio-tech/ui-rs | preview-generated | 2396 | 1113 | 564 | 21 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | check | 8 | 3 | 1 | 9 | 0 | 5 {projectRoot}/**/* |
| C | @semio-tech/ui-rs | test-control-commit | 2429 | 1135 | 564 | 28 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-physical-close | 2396 | 1113 | 564 | 24 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-physical-close-native | 2396 | 1113 | 564 | 28 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-mounted-admission | 2396 | 1113 | 564 | 22 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-mounted-admission-native | 2396 | 1113 | 564 | 22 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-original-presented-source-native | 2397 | 1114 | 564 | 30 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-crate-boundaries | 2701 | 1126 | 570 | 41 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-tui | 2396 | 1113 | 564 | 23 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/ui-rs | test-text-editor-source | 64 | 1 | 47 | 17 | 0 | 35 fw/🔨️modules/🎠️kernel/*/🟦️.ts |
| C | @semio-tech/pets-react | test | 3747 | 1203 | 551 | 17 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/pets-react | typecheck | 3747 | 1203 | 551 | 17 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/pets-rs | build | 5 | 0 | 2 | 8 | 0 | 3 {projectRoot}/**/* |
| C | @semio-tech/pets-rs | test | 140 | 86 | 19 | 6 | 0 | 108 fw/🛍️products/🐾️pets/**/*.rs |
| C | @semio-tech/pets | test | 145 | 92 | 16 | 6 | 0 | 73 fw/🛍️products/🐾️pets/🔨️modules/**/🟦️.ts |
| C | @semio-tech/pets | typecheck | 145 | 92 | 16 | 6 | 0 | 73 fw/🛍️products/🐾️pets/🔨️modules/**/🟦️.ts |
| C | @semio-tech/course-react | test | 5303 | 3184 | 1903 | 20 | 0 | 5030 fw/🛍️products/🦑️repo/🔨️modules/📚️library/**/* |
| C | @semio-tech/course-react | typecheck | 5303 | 3184 | 1903 | 20 | 1 | 5030 fw/🛍️products/🦑️repo/🔨️modules/📚️library/**/* |
| C | @semio-tech/course-rs | build | 6 | 0 | 1 | 8 | 0 | 4 {projectRoot}/**/* |
| C | @semio-tech/course-rs | test | 102 | 80 | 8 | 6 | 0 | 59 fw/🛍️products/📚️course/**/*.rs |
| C | @semio-tech/course | test | 106 | 76 | 13 | 7 | 0 | 36 fw/🛍️products/📚️course/🧪️tests/**/🟦️.ts |
| C | @semio-tech/framework-rs | test-staged-payload-funding | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-retained-pack-encoding | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-window-utility-native | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-source-projection | 4778 | 2355 | 1198 | 18 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-directory-discovery | 4778 | 2355 | 1198 | 18 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-action-choices | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-tool-run-actions | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-history-edit-actions | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-mutation-inputs | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-host-effect-invocation | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-snapshot-sqlite | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-snapshot-sqlite-native | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-core-modules | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-deflate-encoding | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-wire-retirement-source | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-wire-retirement-native | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-package-descriptor-value-codec | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | lint | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | generate | 5 | 0 | 2 | 1 | 0 | 5 {projectRoot}/**/* |
| C | @semio-tech/framework-rs | preview-generated | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | check | 5 | 0 | 2 | 1 | 0 | 5 {projectRoot}/**/* |
| C | @semio-tech/framework-rs | test-fixture-ownership-source | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-snapshot-sqlite-source | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-artifact-kind-source | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-command-ingress-consumer | 3538 | 1675 | 770 | 22 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-history-progress | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-snapshot-receiving-source | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-argument-retirement-native | 3535 | 1674 | 768 | 16 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-original-factory-admission | 3540 | 1679 | 768 | 18 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework-rs | test-original-factory-admission-source | 3540 | 1679 | 768 | 18 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | window-utility-check | 3313 | 1501 | 662 | 9 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test-artifact-reference | 4556 | 2182 | 1092 | 12 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test-io-ports | 3313 | 1501 | 662 | 11 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test-script-boundary | 4556 | 2182 | 1092 | 12 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test | 3313 | 1501 | 662 | 9 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | typecheck | 3313 | 1501 | 662 | 9 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | installation-identity-check | 3313 | 1501 | 662 | 9 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test-polygon-mesh | 3313 | 1501 | 662 | 9 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test-sdk-surface | 3313 | 1501 | 662 | 9 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test-module-package-surface | 3313 | 1501 | 662 | 9 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test-document-scope | 3313 | 1501 | 662 | 11 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @semio-tech/framework | test-source-text | 3316 | 1504 | 662 | 11 | 0 | 2351 fw/🔨️modules/🖱️ui/**/* |
| C | @teaching/architecture-course | build | 2199 | 403 | 178 | 37 | 0 | 1217 fw/🔨️modules/🖼️assets/**/* |
| C | @teaching/architecture-course | test | 1246 | 417 | 202 | 71 | 1 | 371 fw/🛍️products/🐾️pets/**/* |
| C | @teaching/architecture-course | typecheck | 977 | 387 | 174 | 35 | 1 | 371 fw/🛍️products/🐾️pets/**/* |
| C | @teaching/architecture-course | check | 977 | 387 | 174 | 35 | 1 | 371 fw/🛍️products/🐾️pets/**/* |
| P | @semio-tech/presentation-react | test | 44 | 5 | 31 | 6 | 0 | 41 fw/🛍️products/🎤️presentation/🎯️targets/⚛️reac |
| P | @semio-tech/presentation | test | 7 | 3 | 3 | 4 | 0 | 4 {projectRoot}/**/* |
| P | @semio-tech/print-viz-inference | build | 951 | 498 | 195 | 7 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print-viz-inference | test | 951 | 498 | 195 | 8 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print-rs | test | 953 | 498 | 196 | 4 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print-rs | build | 5 | 0 | 1 | 8 | 0 | 3 {projectRoot}/**/* |
| P | @semio-tech/print-rs | test-snapshot-sqlite | 953 | 498 | 196 | 4 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print-rs | test-snapshot-sqlite-native | 953 | 498 | 196 | 4 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print-rs | test-snapshot-sqlite-source | 953 | 498 | 196 | 4 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print-rs | test-logical-owner | 953 | 498 | 196 | 4 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print-rs | test-native-emission | 953 | 498 | 196 | 4 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print | check | 951 | 498 | 195 | 3 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print | generate | 8 | 0 | 7 | 5 | 0 | 3 fw/🛍️products/📓️print/🔨️modules/🎨print-design |
| P | @semio-tech/print | preview-generated | 956 | 498 | 199 | 8 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print | test | 951 | 498 | 195 | 3 | 1 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print | test-macro-staging | 449 | 104 | 111 | 10 | 0 | 172 fw/🛍️products/📓️print/🖋️latex/**/* |
| P | @semio-tech/print | test-viz | 951 | 498 | 195 | 3 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print | test-viz-full | 26 | 8 | 1 | 1 | 1 | 26 {projectRoot}/**/* |
| P | @semio-tech/print | test-toolchain | 7 | 0 | 5 | 4 | 0 | 4 fw/🛍️products/📓️print/🔨️modules/🖨️tectonic-te |
| P | @semio-tech/print | test-viz-fixtures | 951 | 498 | 195 | 3 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print | test-native-grammar | 951 | 498 | 195 | 3 | 1 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/print | test-measurement | 951 | 498 | 195 | 3 | 0 | 951 fw/🛍️products/📓️print/**/* |
| P | @semio-tech/mit-bestand-bericht | build | 34 | 16 | 7 | 2 | 1 | 29 fw/🛍️products/📓️print/📦️artifacts/**/* |
| P | @semio-tech/mit-bestand-bericht | build-zwischenbericht | 1059 | 14 | 11 | 27 | 0 | 849 ♻️mit-bestand/📋️bericht/📋️zwischenbericht/**/* |
| P | @semio-tech/mit-bestand-bericht | build-forschungsbericht | 1043 | 14 | 11 | 27 | 0 | 833 ♻️mit-bestand/📋️bericht/📑️forschungsbericht/**/* |
| P | @semio-tech/mit-bestand-bericht | build-kompaktbericht | 211 | 1 | 11 | 24 | 0 | 172 fw/🛍️products/📓️print/🖋️latex/**/* |
| P | @semio-tech/mit-bestand-bericht | akteursnetz-validate | 838 | 13 | 2 | 10 | 0 | 830 ♻️mit-bestand/📋️bericht/📑️forschungsbericht/🖼️asset/** |
| P | @semio-tech/mit-bestand-bericht | test | 1914 | 30 | 16 | 27 | 0 | 1719 ♻️mit-bestand/📋️bericht/**/* |
| P | @semio-tech/mit-bestand-bericht | generate-actor-network | 4 | 0 | 2 | 5 | 0 | 1 fw/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️wo |
| P | @semio-tech/mit-bestand-bericht | preview-generated | 12 | 0 | 2 | 6 | 0 | 8 ♻️mit-bestand/📋️bericht/📎️anhang/*.tex |
| P | @semio-tech/mit-bestand-bericht | check-generated | 12 | 0 | 2 | 6 | 0 | 8 ♻️mit-bestand/📋️bericht/📎️anhang/*.tex |
| P | @semio-tech/mit-bestand-praesentation-projektetage | build | 6 | 0 | 5 | 6 | 0 | 6 {projectRoot}/**/* |
| P | @semio-tech/mit-bestand-praesentation-projektetage | test | 90 | 1 | 37 | 6 | 0 | 54 ♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️publi |
| P | @semio-tech/mit-bestand-praesentation-projektetage | canonical-architecture | 90 | 1 | 37 | 6 | 0 | 54 ♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️publi |

## 7. Verification status

- Read-only: no build, no graph run, no source edit, no modifying git command (git ls-files, git log only). No sub-agents.
- Verified by reading the files: all named project.json entries quoted above; `nx.json` named inputs; `⚡️caching/🔣️policy.json` (toolchains, targetDefaults, targetClasses); `📥️inputs/🟨️.mjs` 255-300 (toolchain contract appended to every key, owner glob, generated-dir negations); `drive.py` output per target.
- Not verified here (open): the live effect of M1/M2 on test outcomes; the router conflict in #15; whether `^default` closures differ from the declared set by the factor I assume in #6; the effective size of the owner glob after the task-wide negations (cinp-d measured 18.8k → 11.4k for `framework-rs:generate`, the current code state was not re-measured).
