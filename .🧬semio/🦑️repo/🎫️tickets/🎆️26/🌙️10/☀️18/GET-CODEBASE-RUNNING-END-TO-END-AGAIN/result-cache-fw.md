# result-cache-fw: cache coverage for 🧰️framework project manifests (2026-10-19)

Executor `cache-fw`. Static only: no `bun repo` run, no nx graph load, no cargo, no activation. Inputs: `brief-1019.md`, `audit-cache-coverage-1019.md`, `result-bzl-man.md`, `audit-bazel-state-1019.md`, plus the plugin source (`🏗️build/🕸️graph/📥️inference/🧱️project-assembly`, `🧭️script-inputs`, `🕸️dependency-edges`) to learn what the assembled graph adds on its own.

## Result

Framework scope (1673 targets, 259 manifests with targets, census replica `cache-fw-tools/census.ts`):

| Class | Before | After |
|---|---|---|
| deterministic-cacheable | 702 | 1511 |
| deterministic-but-cache-off | 794 | 50 |
| unclassified-cache-off | 127 | 62 |
| continuous | 16 | 16 |
| side-effecting | 19 | 19 |
| test-against-live | 4 | 4 |
| diagnostic-uncached | 11 | 11 |
| aggregate | 0 | 0 |
| Total | 1673 | 1673 |

Raw manifest flags (framework): `cache: true` 237 to 1512; `cache: false` 964 to 156; `cache` absent 472 to 5. Flag `no-inputs` 806 to 51, `no-outputs` 787 to 23, `cache-off-deterministic` 794 to 50, `needs-decision` 127 to 62.

The 50 deterministic-but-cache-off and 62 unclassified-cache-off that remain are deliberate and listed below with reasons (host-dependent infrastructure tests, live journeys, launchers, writers, orchestration drivers, policy-pinned). The remaining `no-inputs` are generator-contract targets whose inputs/outputs the assembly derives from `taxonomy.json` `generatorContracts`, and a few orchestrators.

## Method

1. What the assembly already does (read from `🧱️project-assembly`): for every target with an effective `cache: true`, the assembler appends the literal import closure of the command script (`commandSourcesN` groups); Cargo targets get `nativeSources`/`nativeTestSources` plus lock inputs and the explicit `inputs` are replaced; nx dependency edges are inferred from TS imports, Cargo path dependencies (not dev) and package.json. So manifests must declare what is NOT reachable that way: project/data reads outside the project default, test files spawned by path, tool/toolchain files, env.
2. Static analyzer `cache-fw-tools/analyze.ts` (string-literal relative paths in each project's scripts, their import closure, minus the script closure and minus what `default` plus the dependency closure already cover) gave per project the extra read roots; they are collapsed to module roots and written once per project as the project-level named input `reads` (not into `default`, so they do not leak into every dependent through `^default`; for Cargo projects whose `nativeTestSources` is not owner-defined it is extended with `reads` because the plugin replaces explicit inputs there). Rust path dev-dependencies (not edges) are added to `reads` too. Print product targets read the whole print product.
3. Target decisions `cache-fw-tools/decide.ts` + `manual.ts`; edits are span-based (`jsonspan.ts`), so untouched text, inline objects and concurrent edits stay intact. Every file was re-read and compared right before writing (0 stale).
4. Inputs written per flipped or implicit target: policy default by name (`production`,`^production` for build families, else `default`,`^default`), plus project groups `testEnv` (the three `SEMIO_*` test env inputs for every `test*`), `cargoToolchain` (workspace `Cargo.toml`, `Cargo.lock`, `.cargo/config.toml`, `rust-toolchain.toml`, `rustfmt.toml`) for Rust projects, `goToolchain` (`go.work`, `go.work.sum`) for Go projects, `reads`, and `{dependentTasksOutputFiles}` where the target consumes a dependency (not tool installers). Owner-authored input lists on formerly cache-off targets are kept and unioned with these (never narrowed). Targets that were already `cache: true` with their own inputs were not touched.
5. Outputs: `[]` for tests/checks/lint/verify/contract/source/native/oracle families and for Rust `build` (artifacts live in the leased compiler store, same accepted boundary as proctor in `result-bzl-man.md`); existing declared outputs were kept. Generator-contract targets and their check targets only got `cache: true`; the assembly injects their inputs and outputs. In-tree generators (`generate*`, `typegen`, `channel-version-generate`) use `outputs: []` following the policy default for `generate`; store-publishing builders (`print:build`, Rust builds) use `outputs: []` for the same reason as the Vite/cargo boundary.
6. Executor conversion requested by the coordinator (message during the run): the 32 framework targets of `rules-fix-tools/non-repo-script-targets.json` now carry `"executor": "repo:script"` (10 manifests). Only `semio-framework-os-infinite:{build,check,test}` called a foreign driver directly; they now call `bun ./📜️script.ts build|check|test` and `♾️infinite/📦️packages/🦀️rust/📜️script.ts` implements `BuildScript` (`buildAdmittedCargoArtifactsV1`, as `pets-rs`), `CheckScript` (`runAdmittedCargoCommandV1`, as `tool-run-rs`) and `TestScript` (`runAdmittedCargoPackageTestsV1`, as `test-wgpu-world-terrain` there). `ui-styling-py:deps` keeps `bun ../../🏗️builder/🐍️python/📜️script.ts deps` (already a `📜️script.ts <command>` call). Framework now has 0 targets without `repo:script`/`repo:noop`.

## Targets kept at cache false (schema has no reason field; `Task` has no `additionalProperties: false` and no neutral reason key, so none was invented)

### Deliberate, explicit `cache: false` written or confirmed (103)

| Target | Reason |
|---|---|
| @semio-tech/framework-async:twin | measures live event-loop deadlines and chained scheduling on the current host |
| @semio-tech/framework-process:test-process-tree-termination | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-budget | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-execution-budget | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-resource-leases | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-native-storage-platform | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-cargo-test-leases | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-runtime-environment | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-cargo-build-lease | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-compiler-capacity | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-compiler-capacity-inheritance | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-compiler-capacity-custodian | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-exact-cargo-storage | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-process:test-resource-lease-waits | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/ui-styling-tokens:fonts | downloads fonts over the network |
| @semio-tech/framework-os-kernel:test-directory-lease-fixture | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/framework-renderer-wgpu:browser-dock-acceptance | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs (browser or GPU) |
| @semio-tech/framework-renderer-wgpu:browser-media-acceptance | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs (browser or GPU) |
| @semio-tech/framework-renderer-wgpu:native-scale | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs (browser or GPU) |
| @semio-tech/framework-renderer-wgpu:browser-embedded-acceptance | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs (browser or GPU) |
| @semio-tech/plugin-registry:rebuild-all | rebuilds and republishes every generated catalog artifact in place (mutating) |
| @semio-tech/plugin-registry:surface-schema | tree-wide projection writer: rewrites about 2600 generated schema leaves across the workspace, outputs unbounded |
| @semio-tech/os-plugin-describe-rs:describe | emits descriptors into the leased products store on every run; consumers re-run it |
| @semio-tech/framework-os-dev:plugin-size | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:program-matrix | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:tool-run-matrix | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:two-human | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:connection-budget | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:idle-budget | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:interaction-latency | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:memory-soak | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:cold-boot-check-s | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:s-host-foreign-kind-s | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:two-human-viewer | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:two-human-cross-undo | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:boot-budget | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:time-travel | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/framework-os-dev:parity-journey | live host, browser or server run: measures or drives a running system, result depends on runtime state outside declared inputs |
| @semio-tech/print-rs:produce-logical-owner | publishes a produced logical owner into the artifact store (mutating) |
| @semio-tech/repo-cli-rs:repo | launcher: starts a long-lived tool or UI, no cacheable result |
| @semio-tech/repo-cli-rs:mcp | launcher: starts a long-lived tool or UI, no cacheable result |
| @semio-tech/repo-dashboard-rs:test-journeys | dashboard UI journeys against a running dashboard |
| @semio-tech/repo-dashboard-rs:test-smoke | dashboard UI journeys against a running dashboard |
| @semio-tech/repo-dashboard-rs:run | launcher: starts a long-lived tool or UI, no cacheable result |
| @semio-tech/repo-dashboard-rs:preferences | launcher: starts a long-lived tool or UI, no cacheable result |
| @semio-tech/repo-developer-tools:git-show | reads live git state or opens the editor; not a function of declared inputs |
| @semio-tech/repo-developer-tools:git-blame | reads live git state or opens the editor; not a function of declared inputs |
| @semio-tech/repo-developer-tools:git-diff | reads live git state or opens the editor; not a function of declared inputs |
| @semio-tech/repo-developer-tools:git-history | reads live git state or opens the editor; not a function of declared inputs |
| @semio-tech/repo-developer-tools:open-source | reads live git state or opens the editor; not a function of declared inputs |
| @semio-tech/repo-developer-tools:git-status | reads live git state or opens the editor; not a function of declared inputs |
| @semio-tech/repo-developer-tools:debuggers | reads live git state or opens the editor; not a function of declared inputs |
| repo:debug-bun-bundle | debug bundle inspection of the live bundle store |
| repo:digest-source-inventory | prints digests of the whole workspace source inventory; unbounded input set |
| repo:digest-source-closure | prints digests of the whole workspace source inventory; unbounded input set |
| repo:digest-cargo-lock | prints digests of the whole workspace source inventory; unbounded input set |
| repo:emit-typescript-declarations | writes declaration files into the leased artifact store |
| repo:cargo-provenance-repair | mutates Cargo workspace state |
| repo:test-cargo-build-lease | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-next-runtime | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-next-dev-runtime | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-graph-cache-waiter | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-bazel-storage-runtime | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-bazel-storage | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-tools-partition | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-daemon-journal | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-daemon-journal-workspace | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:cargo-prepare | mutates Cargo workspace state |
| repo:cargo-synchronize | mutates Cargo workspace state |
| repo:test-storage-budget | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-leases-partition | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-lease-bootstrap | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-lease-queue | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-javascript-storage | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-runtime-verification-storage | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-runtime-verification-storage-runtime | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-rustup-toolchain | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-bootstrap-original-invocation-source | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| repo:test-runtime-bootstrap | host-dependent: exercises real storage, lease, process, toolchain or runtime state of the machine outside the declared inputs |
| @semio-tech/repo-lib:go-test | runs the Go toolchain over Go modules outside the declared inputs |
| @semio-tech/repo-lib:test-cross-platform-bootstrap | bootstrap acceptance: installs and probes tooling on the host |
| @semio-tech/repo-lib:workspaces-write | mutating writer: rewrites tracked repository state in place |
| @semio-tech/repo-lib:test-dependency-policy-bootstrap | bootstrap acceptance: installs and probes tooling on the host |
| @semio-tech/repo-lib:probe-nx-owner-arguments | probes the live nx owner-argument path |
| @semio-tech/repo-lib:schema-read-inventory | reads schema trees across the whole workspace; unbounded input set |
| @semio-tech/repo-lib:test-go-executable-acceptance | builds and runs Go executables outside the declared inputs |
| @semio-tech/repo-lib:owner-command | wrapper that spawns the owner command; result is the child's |
| @semio-tech/cargo-workspaces:members-write | mutating writer: rewrites tracked repository state in place |
| @semio-tech/repo-mcp-rs:run | launcher: starts a long-lived tool or UI, no cacheable result |
| repo-next-storage:test-nx-cache | exercises the nx cache directory itself |
| @semio-tech/repo-coordinator-rs:run | launcher: starts a long-lived tool or UI, no cacheable result |
| @semio-tech/repo-test-domain:test-probe | probe of the live test domain |
| @semio-tech/framework-actor:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| @semio-tech/ui-react:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| semio-framework-3d:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| @semio-tech/framework-job-rs:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| @semio-tech/framework-os:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| @semio-tech/framework-os-kernel:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| @semio-tech/framework-renderer-wgpu:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| @semio-tech/framework-plugin:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| @semio-tech/framework-os-dev:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |
| @semio-tech/repo-lib:canonical-architecture | policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs |

### Orchestration drivers left as authored (12; children are cached)

| Target | Reason |
|---|---|
| @semio-tech/framework-process:test-native-artifact-publication | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:build | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:test | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:test-authority | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:test-execution | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:test-quick | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:test-long | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:test-exhaustive | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:test-coverage | orchestration driver; its child tasks are cached |
| @semio-tech/repo-dashboard-rs:test-load | orchestration driver; its child tasks are cached |
| @semio-tech/repo-lib:test-fixture-ownership | orchestration driver; its child tasks are cached |
| @semio-tech/repo-lib:test-original-default-preparation-control | orchestration driver; its child tasks are cached |

### Census classes continuous / side-effecting / live / diagnostic left untouched (50; policy owns them)

`@semio-tech/ui-styling-py:deps` (side-effecting), `@semio-tech/ui-styling-dotnet:deps` (side-effecting), `@semio-tech/assets:logo-dev` (continuous), `semio-framework-3d:bench` (diagnostic-uncached), `@semio-tech/pets-react:dev` (continuous), `@semio-tech/framework-os-kernel:wal-writer-fence-live` (test-against-live), `@semio-tech/framework-os-mcp-rs:publish` (side-effecting), `@semio-tech/framework-os-mcp-rs:hub-live-catalog-oracle` (test-against-live), `@semio-tech/framework-os-mcp-rs:hub-live-catalog-check` (test-against-live), `@semio-tech/framework-os-mcp-rs:hub-live-catalog-native-check` (test-against-live), `@semio-tech/framework-os-mcp-rs:dev` (continuous), `@semio-tech/plugin-registry:new` (side-effecting), `@semio-tech/framework-os-dev:bench` (diagnostic-uncached), `@semio-tech/framework-os-dev:parity` (diagnostic-uncached), `@semio-tech/framework-os-dev:bench-plugins-native` (side-effecting), `@semio-tech/framework-os-dev:bench-plugins-react` (side-effecting), `@semio-tech/framework-os-dev:bench-plugins-wgpu` (side-effecting), `@semio-tech/framework-os-dev:serve-hold` (continuous), `@semio-tech/print:watch` (continuous), `@semio-tech/print:watch-viz` (continuous), `@semio-tech/print:deps-tectonic` (side-effecting), `@semio-tech/print:deps-tex` (side-effecting), `@semio-tech/repo-dashboard-rs:daemon` (continuous), `@semio-tech/repo-dashboard-rs:install` (side-effecting), `@semio-tech/repo-client:dev` (continuous), `@semio-tech/repo-developer-tools:terminal` (continuous), `@semio-tech/repo-developer-tools:debug` (continuous), `@semio-tech/repo-vscode:dev` (continuous), `@semio-tech/repo-vscode:watch-extension` (continuous), `repo:ci-baseline` (diagnostic-uncached), `repo:audit` (diagnostic-uncached), `repo:doctor` (diagnostic-uncached), `repo:disk-report` (diagnostic-uncached), `repo:cache-report` (diagnostic-uncached), `repo:cache-prune` (side-effecting), `repo:cache-prune-stale-incremental` (side-effecting), `repo:cache-verify` (diagnostic-uncached), `@semio-tech/repo-lib:test-root-clean-scaffold-source` (side-effecting), `@semio-tech/repo-lib:test-inventory-artifact-shards` (side-effecting), `@semio-tech/repo-lib:ui-storybook-dev` (continuous), `@semio-tech/repo-coordinator-go:dev` (continuous), `@semio-tech/repo-coordinator:dev` (continuous), `@semio-tech/repo-coordinator:start` (continuous), `@semio-tech/repo-test-domain:test-clean` (side-effecting), `@semio-tech/repo-test-domain:test-doctor` (diagnostic-uncached), `@semio-tech/repo-test-domain:test-inventory` (diagnostic-uncached), `@semio-tech/repo-test-domain:test-fixture-reproduce` (side-effecting), `@semio-tech/repo-test-domain:test-fixture-generate` (side-effecting), `@semio-tech/repo-test-domain:test-gc` (side-effecting), `@semio-tech/repo-test-dotnet:deps` (side-effecting)

### Skipped, plugin- or owner-controlled (7)

| Target | Reason |
|---|---|
| @semio-tech/framework-editor-rs:wasm | plugin owns wasm target (cache=false, mode-scoped outputs) |
| @semio-tech/framework-actor-rs:wasm | plugin owns wasm target (cache=false, mode-scoped outputs) |
| @semio-tech/framework-surface-rs:wasm | plugin owns wasm target (cache=false, mode-scoped outputs) |
| semio-framework-os-flow-browser-package:wasm | plugin owns wasm package output |
| @semio-tech/framework-os-node-graph-rs:wasm | plugin owns wasm target (cache=false, mode-scoped outputs) |
| @semio-tech/framework-os-host-rs:wasm | plugin owns wasm target (cache=false, mode-scoped outputs) |
| repo:toolchain | owner-declared fingerprint of the live toolchain; leave to its owner |

Note: the plugin forces `cache=false` for Rust `wasm`/`wasm-release` targets (except renderer-wgpu) and for `role:artifact` TypeScript `build`/`test`; raw manifests with `cache: true` on those names (e.g. `framework-editor-rs:wasm`) are therefore not effective until the plugin changes. `node-graph-rs:wasm` was the only one with `cache` absent; it now says `false`.

## Verification

| Check | Command | Result |
|---|---|---|
| Neutral schema, repo validator | `bun cache-fw-tools/validate.ts` (calls `validateJsonSchemaSubset` with `🏗️build/🧬️schema/📋️project/🔣️.json` on every framework manifest with targets) | 259 manifests, 0 errors |
| Neutral schema, third party | same script, Ajv draft-07 (`allErrors`) | 259 manifests, 0 errors |
| JSON well-formedness and no stale writes | `decide.ts --apply` parses every result before writing | 256 files written, 0 stale |
| Census replica before/after | `cache-fw-tools/census.ts` (copy of `aud-cachecov-tools/census.ts`, tolerant of files deleted mid-run) and `counts.ts` | table above |
| Infinite script | `bun build --target=bun ♾️infinite/📦️packages/🦀️rust/📜️script.ts` | bundles (652 modules); not executed |
| Executor conformance | `cache-fw-tools/execcount.ts` | 1673 framework targets, 0 without repo:script/repo:noop |
| Not run | `bun repo show projects`, nx graph load, any task execution, cargo, `--tool=bazel`, the cache-contract test suite | not run per brief; the assembled inputs (command-source groups, native replacement) are inferred from reading the plugin, not observed |

## Open items

1. First real graph load must confirm: project-level named inputs `reads`, `testEnv`, `cargoToolchain`, `goToolchain` survive `projectInputs` (the plugin generates its own `commandSourcesN` groups; no collisions were found by grep), and no target trips the output/input overlap negation.
2. Projects whose owners define `nativeTestSources`/`default` without `{projectRoot}` (for example `semio-framework-os-infinite`) do not consume `reads` on native tests; their Rust sources come from compiler dep-info. TS-side test files reached only by path stay unhashed there until the owner includes `reads`.
3. `reads` is a static string-literal analysis (module-root granularity). Tests that scan directories at run time (source-text laws, workspace walkers; 43 flagged projects in `aud` output) are covered only as far as their referenced roots; shared infra scanners (workspaces, native-build, leases) are execution environment, not inputs.
4. Store-published builders (Rust builds, `print:build`) and in-tree generators use `outputs: []`. A pruned store or deleted untracked generated file is not repaired by a cache hit; consumers re-run the producing step themselves today. Declaring store-aware outputs is the same open item as in `result-bzl-man.md` (item 2).
5. Orchestration drivers and the host-dependent infra tests listed above stay uncached; if the Bazel exemption list should be tool-readable, the schema needs a reason field (not added).
6. Bazel adapter still defaults absent `inputs` to `{projectRoot}/**/*`; all flipped framework targets now have explicit inputs, the 170 raw-absent remainder are contract targets, orchestrators, uncached classes.

## Files changed

256 framework `📋️project.json` manifests (cache, inputs, outputs, project-level `namedInputs`; list: `git diff --name-only -- 🧰️framework | grep project.json`, other fleets also edit there), plus 10 of them for the executor conversion, and `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts` (build/check/test commands). Tools: `C/cache-fw-tools/` (`census.ts`, `analyze.ts`, `decide.ts`, `manual.ts`, `jsonspan.ts`, `validate.ts`, `executor.ts`, helpers).
