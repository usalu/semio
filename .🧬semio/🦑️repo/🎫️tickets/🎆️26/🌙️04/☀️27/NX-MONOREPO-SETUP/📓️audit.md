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
