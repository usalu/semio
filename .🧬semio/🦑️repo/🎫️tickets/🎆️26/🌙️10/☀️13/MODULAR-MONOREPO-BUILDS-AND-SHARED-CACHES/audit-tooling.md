# Tooling Audit

Read-only source inspection on 2026-10-13. No builds or tests were run and no runtime success is claimed. Root AGENTS.md was read. No modifying Git commands, worktrees, or production edits were made.

## Existing Architecture

Use `L` below for `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` and `P` for `🧰️framework/🔨️modules/🏃️process`.

| Authority | Source | Observed behavior |
| --- | --- | --- |
| Nx discovery/inference | `L/🟨️.mjs` | Loads taxonomy, cache policy, dependency facts, output contracts; derives target command source closures and Cargo leaves. |
| Plugin wrapper | `L/🔌️nx-plugin/🟨️.mjs` | Wraps inference; should remain thin. |
| Nx bootstrap | `L/⚡️caching/🚀️bootstrap/📜️script.ts` | Pins runtime, admits manifests incrementally, maintains graph seals, configures watcher-specific workspace data/socket paths. |
| Nx policy | `L/⚡️caching/🔣️policy.json`, `nx.json` | Explicit cache families and serial setup targets; hashes SEMIO_BUILD_MODE in selected target defaults. Global default input includes entire project; native inputs use production/native source closures. |
| Process mode | `L/🏃️process/🟦️.ts:234` | Current mode is `ship` only for SEMIO_BUILD_MODE=ship, otherwise `dev`. Bootstrap stamps it for selected product targets rather than all builds. |
| Native route | `L/⚡️caching/🦀️cargo/📜️script.ts`, `L/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts` | NativeScript forwards build arguments to repository artifact driver; check/test run direct Cargo. |
| Native repository policy | `L/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts` | Uses shared Cargo build cache, shared resource lease directory, per-owner dist capture directory. |
| Neutral compiler protocol | `P/📦️artifacts/🏗️native-build/🟦️.ts` | Exclusive compiler profile lease; per-invocation target capture; provenance receipts; releases compiler before receipt rechecks; stages final artifacts. |
| Resource leases | `P/🔒️leases/🟦️.ts`, `P/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts` | Cargo identity canonicalizes actual build directory and profile alias; profile-specific exclusive queue. Already supports wait progress and cancellation. |
| Publication | `P/📦️artifacts/📤️publication/🟦️.ts` | Shared/exclusive artifact leases, temporary directory replacement, rollback and Windows rename retries. |
| Jobserver | `P/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts` | Existing GNU make compatible token pool; Cargo/rustc/stages can share tokens. |
| Launch | `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | Huge explicit command catalog, existing separate release entries, numerous historical ticket-specific environment paths. Preserve unrelated entries. |
| Devcontainer | `.devcontainer/devcontainer.json`, `.devcontainer/Dockerfile` | Root setup through Bun/Nx; persistent repository cache volume scoped by devcontainerId. Native hosts do not share this volume. |
| CI | `.github/workflows/architecture-quiz.yml` | Hand-triggered quiz release, Ubuntu only, frozen Bun install, Bun download cache. This does not establish universal monorepo build matrix. |

## Highest Impact Implementation Units

1. **Parent-owned central mode protocol:** schema-first strict prototype/release selection; remove implicit dev/ship naming throughout all callers rather than adding aliases. Mode must enter arguments, environment, tool options, output path and receipt identity consistently. The Nx patch `L/⚡️caching/🩹️patches/nx@23.2.0.patch` currently passes SEMIO_BUILD_MODE through cache hashing and needs coordinated update only if environment naming changes.

2. **Nx mode projection:** own `L/🟨️.mjs`, its Nx contract fixtures/tests, and bootstrap invocation mapping. `cargoTargets` at lines 1048-1061 emits one build target, fixed `dist/build` output, and no configurations. Inferred component target at lines 1067-1082 is also one target with fixed dist/components and plugin-module destination. Add explicit prototype/release configurations or explicit targets from the central protocol, with distinct declared outputs and inherited dependency mode. Avoid merely stamping env on the outer task: nested dependencies and cache restorations need identical mode selection. Product preparation loops currently use dev/release at lines 1227 onward; coordinate their callers and fixtures.

3. **Native routing and capture identity:** own `L/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`, `L/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`, and neutral compiler receipt/publication tests. Select strict requested mode before constructing Cargo argv, resolve prototype/release profile through central policy, and stage each mode into distinct owner output directories. Compiler cache can remain shared across processes with profile leases; invocation capture stays unique. Audit check/test direct Cargo routing to ensure it acquires the existing applicable resource lease rather than relying on Nx parallelism, which only constrains one task graph.

4. **Bootstrap concurrent authority:** own `L/⚡️caching/🚀️bootstrap/📜️script.ts` and bootstrap fixtures/tests. Admission snapshot writes are already unique temporary plus rename. `.nxignore` publication at line 181 is direct write after an authority recheck; simultaneous writers deserve a shared lease or atomic publish with recheck. Ordinary invocations share `.nx/workspace-data`; watchers are individually isolated. Validate simultaneous ordinary graph generation and graph-seal publication before adding broad isolation that would discard warm graph reuse.

5. **Launch/zero-touch entrypoint projection:** derive mode entries from central mode declaration using the existing launch generator, update seed plus generated catalog together, register any new test targets in established order. Root package.json routes Nx through bootstrap and setup through workspace:setup; no new script filenames are needed. Devcontainer and native setup must pick the same profiles/cache policy. CI needs targeted mode and concurrency contract tests before adding costly global builds.

## Required Focused Tests

- Language-agnostic JSON cases for missing, invalid, duplicate and conflicting mode selection; compare JSON-schema validation using existing Ajv.
- Nx contract snapshot: every build family exposes both modes, explicit output paths differ, mode is hashed, and mode-specific dependencies remain in the same graph configuration. Existing suite: `L/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`; fixtures in `L/⚡️caching/🧫️fixtures/nx-contract/🔣️.json`.
- Native small fixture: compile each mode, rerun unchanged, change a shared source, confirm Cargo freshness/provenance and mode-separated final outputs. Existing suites under `P/📦️artifacts/🏗️native-build/🧪️tests` and lease/jobserver/stage-cache subdirectories; use real Cargo as third-party oracle.
- Two concurrent processes same profile: queue, cancellation, interrupted owner recovery and complete artifact publication. Different profiles may proceed independently. Existing lease cases should be extended, not replaced.
- Two simultaneous Nx invocations: same checkout/workspace data, unchanged warm graph, edit while graph admission occurs, no torn ignore file or seal. Existing bootstrap, graph-revision and daemon cases are suitable.
- Warm build evidence: record first/second wall times and exact compiler work counts; Nx cache hit alone cannot prove incremental compiler performance. Do not prescribe success thresholds without running the representative build.

## Architectural Guidance

Keep mode schema and pure resolution in neutral process code; let repository policy choose cache roots/tool profiles; let Nx project that policy; keep each product extension responsible for its own build commands. Reuse existing leases, receipt schemas, stage cache, graph seals and jobserver rather than creating parallel infrastructure. Do not use a checkout-wide exclusive lock: compiler/profile and publication/resource locks already express finer ownership. Multi-process shared budget should be verified because independent driver-created jobservers can otherwise each consume the full machine budget.
