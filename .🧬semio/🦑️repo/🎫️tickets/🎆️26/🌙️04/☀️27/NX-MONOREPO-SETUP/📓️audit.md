# Nx Orchestration and Cache Audit

Scope: the current shared checkout, with the attached Nx-only plan retained in 📋️plan.md. No worktree or modifying Git commands. Existing ticket reopened via repository MCP after reading repo://goals.

Installed/pinned Nx: 21.6.11. Initial findings: global cross-language inputs; cacheable setup/publish/update defaults; blanket ^build; mutable shared test report outputs; package aliases and script routers conceal task selection. These require resolved-graph and runtime verification.

Evidence references: [Nx inputs](https://nx.dev/docs/reference/inputs), [Nx configuration](https://nx.dev/docs/reference/nx-json), [Nx graph plugins](https://nx.dev/docs/kb/add-language-support). Installed schemas and source take precedence for version-specific behavior.

Execution plan: inventories and failing contracts; graph/input/output ownership; Nx entry-point integration; bounded storage diagnostics; runtime restoration/invalidation/cancellation verification; final report and ticket closure.

## Artifact and Go Graph Checkpoint

- Native hash library: complete cold/warm/deleted-output restoration and actual Rust consumer passed. See 📓️native-artifacts.md. The initial consumer failure identified the required external rmeta and was fixed; do not treat an rlib alone as complete.
- Seven wasm-bindgen project contracts now name their outputs, retain mandatory toolchain inputs even with custom named inputs, and exclude owned outputs from their own hashes. The actor WASM build completed after substantial contention on Cargo's shared target lock. Warm/restoration validation follows separately.
- Twenty-six extension packaging targets now own one exact SXT file and reject output overrides. The helper no longer permits undeclared destinations.
- Assets, VS Code bundles/VSIX, Go client/coordinator binaries and three existing Rust build targets have explicit output contracts. Native staging replaces obsolete owned outputs atomically and preserves executable mode.
- Go parser matches Go's own `go mod edit -json` oracle. The actual Nx graph contains client→repo-go-lib, coordinator→repo-go-lib, and MCP→client/repo-go-lib edges. The previously empty MCP package script/metadata now implement build, continuous dev and tests.
- CI test-protocol commands now invoke existing Nx targets.

Remaining work: hidden OS dev/build/plugin scheduling; setup/install and devcontainer lifecycle separation; remaining report/app/native renderer output contracts; full polyglot ownership and graph validation; real disk-prune/retention (current placeholder must be replaced); CI/editor/command and artifact overlap enforcement; repeatable cache verification plus cancellation/storage tests; root shared graph cache validation; final cross-platform configuration review. This checkpoint is not completion of the monorepo refactor.

## Continued Nx Integration

- Added a Rust module/input scanner with language-neutral fixtures. Its fixture input set exactly matches independent `rustc --emit=dep-info` output. The OS kernel scanner covered all 67 browser and 71 native Rust paths from existing compiler dependency files. Dynamic includes, conditional path attributes, generated module macros, and build scripts retain conservative ownership fallback.
- The previous actor WASM misses were traced with Nx native hash details to the over-broad OS kernel owner glob. Rust source mounts now scope that input instead of the entire OS product. Declared local outputs are excluded from both project and workspace filesets.
- Real actor WASM warm reuse, complete output restoration, byte identity and independent Node consumer execution now pass; see `📓️wasm-artifacts.md`. A subsequent build also passed with Cargo locked and wasm-pack installation disabled.
- Binding generator resolution is now cross-platform and requires the exact version in Cargo.lock. Explicit binary overrides participate in the toolchain fingerprint. Browser compiler state remains shared across crates, independently of native compiler state.
- Workspace setup now has explicit uncached dependency-environment prerequisites, with a separate prepare graph for deterministic generators/assets. Setup no longer compiles repo Go/VSCode binaries or launches nested Nx builds. Dependency installers themselves have not all been executed; native bootstrap and devcontainer lifecycle still require refactoring.
- Workspace test levels now use explicit outer Nx task dependencies instead of nested run-many and global coverage-directory deletion. Fundamental, quick, long and exhaustive are supported. The quick task graph was generated successfully; this is graph validation, not a full test-suite pass.
- Added staged build contracts for the Rust repo CLI and descriptor emitter, and fixed Vite outputs for the admin SPA/presentation. CLI run/daemon/workflow and descriptor execution now consume staged build prerequisites. Real Rust CLI build verification is ongoing.
- Restored editor commands through `.vscode/🧩️launch.seed.jsonc` and regenerated launch.json using the canonical renderer. Earlier direct launch edits had been overwritten by concurrent generation. Remaining direct commands throughout the seed still need conversion.
- Test-owner disk pruning now calls scoped, marker/lease-aware GC. It preserves referenced blobs, active publishers, and malformed manifests; full native/renderer store retention is still outstanding.

Latest audit before four additional output contracts: 303 projects, 2002 commands, 128 artifacts, 18 findings. The audit currently covers target/package contracts, not all orchestration surfaces.

References: [Nx 21 output contracts](https://21.nx.dev/docs/guides/tasks--caching/configure-outputs), [wasm-pack 0.15 installer behavior](https://github.com/wasm-bindgen/wasm-pack/blob/v0.15.0/src/install/mod.rs).

## Effective Nx Precedence Correction

The first quick-test graph export contained only the root task. Source-map inspection proved this was not a stale cache: Nx target defaults override custom-plugin inferred target fields, including authored `dependsOn` and `cache: false`. Defaults now live in the repository policy and are merged before each project declaration inside the plugin. The corrected quick graph contains 191 tasks and 188 direct root prerequisites. `repo:graph-check` now compares actual Nx cache/output/dependency fields against normalized metadata.

That check also found real missing discovery: global TeX numeric-suffix Git ignores hid the PDF 1.4 standards directory from Nx. The three overly broad numeric ignores were removed; document compiler intermediates already belong in ignored output directories. The new prepare graph's obsolete graph-manifest identity was corrected to `@semio-tech/framework-graph`. Effective graph verification is continuing.

The Rust CLI release build completed in 34 seconds and staged 48 deliverables. Its target has since been narrowed to the `semio` executable so normal CLI users do not cache unused link libraries; verification of that narrowed output is ongoing.

## Generator Authority And Resolved Graph

The resolved graph now passes `repo:graph-check` after correcting a package-level Nx name override for Infinite World. The CLI `run catalog` consumer also passes using its staged executable; `--help` was an invalid probe because this CLI delegates unknown commands to the root router.

The existing schema-owned `generatorContracts` registry is now projected into Nx inputs and complete outputs, rather than maintaining a second hand-authored output inventory. Four deterministic generators (entity catalog, styling tokens, graph catalog, plugin registry) have cache enabled. Their real cold invocation passed. The regression test failed before implementation and passes after it. Generator freshness checks remain uncached so ignored-output drift is inspected. The TypeScript styling entry point now depends on the canonical Rust-package generator, eliminating two writers.

Plugin registry hashing uses its existing exact catalog-input discovery, including membership witnesses, transitive implementation imports and ignored descriptor bytes. Rust token facts are shared only within one graph construction, avoiding repeated lexical analysis without keeping stale cross-run source snapshots. Warm cache and restoration verification for generators are underway.

## Editor Entry Points

Converted 563 seed commands to canonical Nx calls while retaining command arguments and launch metadata. Removed unconditional `--skip-nx-cache` and obsolete per-command Nx environment toggles; the root Nx launcher already supplies the workspace process policy. Updated generated playground defaults to Nx and corrected the styling and Go MCP launchers. Added generator-input and WASM-toolchain diagnostics. The existing launch regression expectation now uses the canonical command. Remaining stale Animate video, Coda, Trinity shell, inline policy and temporary ticket entry points still require review; this is not a completed editor audit.

The latest generated inventory contains 303 projects, 2,017 commands, 160 artifact declarations and 14 remaining output-contract findings. This ledger covers target/package contracts; it does not yet prove the full control-surface taxonomy in the requested plan.

## Registry Hash Failure Investigation

Actual restoration passed for entity catalog, styling and graph. Registry restoration initially rebuilt. Its `^default` closure pulled its own output back through the framework package's broad source scope. The registry now uses its exact discovered implementation/content closure plus local/default tooling inputs, without that redundant transitive default.

The independent fingerprint invocation exposed a missing framework-replication binding in the registry's schema-owned import map; the binding is now declared using the verified package export. Nx 21.6.11's [native runtime hasher](https://raw.githubusercontent.com/nrwl/nx/21.6.11/packages/nx/src/native/tasks/hashers/hash_runtime.rs) hashes stdout/stderr without checking exit status. Therefore the generator now also has an explicit uncached `repo:generator-inputs` prerequisite, so a failed fingerprint cannot silently allow a cached generator result. This prerequisite is a validation operation visible to Nx.

The first restoration attempt's generated files were all restored or regenerated successfully, including on the failing assertion. Registry restoration is being rerun with the corrected contract.

## Registry Input Investigation

Moving only the registry's declared outputs out and back produced no change across 46,872 discovered input records. The earlier restoration cache miss had different runtime fingerprints before/after, so it cannot establish an output-exclusion failure. A subsequent verification records exact per-path input differences across generation and restoration to distinguish source changes in this shared workspace from output-induced invalidation. No registry restoration success is claimed until that evidence passes.

## Registry Restoration Proven

The final controlled registry run restored all nine declared generated files (586,053 bytes) from Nx cache. SHA-256 contents and filesystem modes matched before/after. Exact input snapshots were unchanged both across generation and across restoration; the uncached input-validation prerequisite ran successfully. Earlier misses were accompanied by changed fingerprints and are not treated as successful reuse. The successful evidence is retained in `📓️generator-artifacts.md`.

## Remaining Direct Project Commands

A full authored `📋️project.json` command scan found three remaining script-boundary violations: coordinator dev/start execute `go run .` from the TypeScript package directory, and assets logo chains generator/export commands in the shell. These are the next bounded command corrections; the broader OS development and native artifact work remains open.

## Continuous Task Cancellation Failure

The coordinator built and reached `/healthz` through `bun nx run @semio-tech/repo-coordinator:dev`. Sending SIGTERM to the public Bun wrapper left the Node Nx process orphaned (PPID 1), with the coordinator still running. Sending SIGTERM directly to that known test Nx process shut down both it and the coordinator; both PIDs were absent afterward. The root NxScript used blocking spawnSync and could not forward termination. This is the runtime red case for the asynchronous wrapper correction. The verification used a ticket-local database and an isolated ephemeral localhost port. No external notification configuration was enabled.

## Entrypoint and Launcher Verification

The authored-project command audit now enforces a single explicit script command and rejects chained shell pipelines. Its neutral fixture failed on the coordinator `go run` commands and chained logo invocation, then passed after those commands became Nx prerequisites plus script leaves. One existing actor-import fixture already implements `runtime-check` as its default; its metadata now spells that command explicitly. The final test/graph/audit invocation passed with 306 resolved projects / 1,222 edges. The authored inventory contains 304 projects, 2,022 commands, 163 artifacts, and 14 still-open output contracts; this does not claim the broader orchestration audit is complete.

The launcher regression initially found that importing the new caching script through Vite left `import.meta.dir` undefined. The script now derives its directory with Node's standard fileURLToPath/dirname APIs. All five targeted launcher tests passed afterward.

The coordinator executable built successfully and served its isolated health endpoint; cancellation initially failed as recorded above and its correction is being verified separately. The logo SVG and MP4 both restored from Nx after removal with identical contents/modes; independent FFprobe confirmed the restored H.264 video. See `📓️logo-artifacts.md`.

## Checkpoint After Bootstrap and Entrypoint Corrections

Coordinator cancellation is now proven on macOS through the public Bun entrypoint, including no surviving health endpoint or child processes; see `📓️coordinator-cancellation.md`. Root NxScript retains optional budgets but now awaits Node asynchronously so cancellation can be forwarded.

Fourteen explicit build-output contracts remain unresolved, plus the broader nested OS development/build orchestration, full CI/environment lifecycle migration, complete polyglot dependency provenance, and bounded owner-specific native/renderer storage retention. The goal and ticket remain active. Logo export's existing shared implementation still needs its cross-platform file URL and detailed progress/backpressure lifecycle reviewed alongside broader process handling; actual current-platform build and restoration are proven.

The final post-cancellation `repo:test` invocation passed, and the launch file was regenerated again after the coordinator/logo entrypoints were added. `📓️coordinator-cancellation.md`, `📓️logo-artifacts.md`, `📓️dotnet-artifacts.md`, and `📓️generator-artifacts.md` retain the successful runtime evidence.

## Scale and MCP Follow-Up Contracts

The remaining scale WASI target compiles with `cargo rustc --crate-type cdylib --target wasm32-wasip2 --profile wasm-dev --features component-guest`, verifies the component header, and leaves its only deliverable in Cargo's mutable target tree. Two consumers still build/read that tree: plugin-host UI patch native verification and the OS development native benchmark. The native verifier starts a nested Nx invocation with `--skip-nx-cache`; the benchmark invokes Cargo and the wgpu script directly. These consumers must move together with the staged scale artifact and outer prerequisite graph.

The OS MCP build produces its debug executable through `buildMcpBinary`, which is also called by dev and explicit native conformance checks. Its public `resolveMcpBinaryPath`/`requireMcpBinary` helpers currently resolve Cargo target/debug by default. Any staged MCP build must update public consumers together; fresh conformance probes still require a distinct exact native-compiler path. The Hub build also duplicates its already-declared admin build prerequisite inside BuildScript.

## Development Graph Planning Constraint

The installed Nx 21.6.11 watch implementation (`node_modules/nx/src/command-line/watch/watch.js`) explicitly rejects operation when its daemon is disabled. The workspace currently disables the daemon and separately disables plugin isolation to avoid emoji-path IPC corruption. Moving plugin watch rebuilds to Nx therefore requires a real daemon/watch verification before enabling that path; silently wrapping the existing custom scheduler would not satisfy the plan. The native watcher supports project selection, dependent-project inclusion, and initial execution, which can own subsequent affected rebuilds once qualified.

OS development currently has explicit separable operations for one-plugin Cargo compilation (`buildPluginCargo`), one-plugin JCO/descriptor/materialization (`materializePlugin`), shared shard/vendor assets, and WASM engines selected by committed Cargo playground rows plus composition `browserSessionFactories`. Those are the intended task boundaries. Current DevScript still combines generation, lease acquisition, engine builds, streaming plugin compilation, Vite/trunk startup, and its own rebuild watcher. BuildScript still performs a nested plugin pipeline and renderer selection. No completion is claimed for those remaining operations.

## OS MCP and Daemon Follow-through

The MCP binary contract first failed because its build had no restorable output. The build now stages the selected executable in the Rust project's `dist/build`; development depends on that Nx build. TypeScript process suites consume the artifact without recursively forcing another uncached Nx build. The root OS MCP selection resolves before graph creation; Hub's shared executable resolver now consumes the staged binary. Cross-platform path vectors and the repository contract tests passed. The actual Cargo build exposed an existing stale `crate::os_spr` reference in its descriptor fixture; it was corrected to the current kernel module. Runtime artifact restoration remains pending until this real compile succeeds.

The Nx daemon passed an isolated actual-plugin Unicode graph/cache/watch exercise and a full repository graph check. The graph checker now reads the outer invocation's resolved graph and terminates cleanly. Details are retained in `📓️daemon-verification.md`.

The asset encoder now uses file URLs, bounded frame writes, cancellation, and staging before publication. The real FFmpeg/FFprobe test passed exact frame counts and cancellation preservation; see `📓️svg-export.md`. Windows/Linux execution remains unclaimed.

The corrected OS MCP build completed and staged one executable. Deleting its staged output and rerunning restored an Nx cache hit with identical file hashes/modes; the installed MCP SDK completed a real handshake and observed 26 tools consistently. The Cargo compilation emitted existing source warnings, which this artifact proof does not claim to resolve. The combined repository contract test also passed with daemon, MCP and SVG fixture changes.

## Compact Graph Discovery

See [graph discovery verification](📓️graph-discovery.md) for the passing compiler-oracle mutation test and measured memory/CPU reduction. The development materializer and variant graph are still outstanding.
