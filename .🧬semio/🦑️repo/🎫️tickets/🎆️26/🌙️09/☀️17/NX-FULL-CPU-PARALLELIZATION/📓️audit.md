# Nx Full CPU Parallelization Audit

## Bottlenecks found

- `nx.json` capped concurrent Nx tasks at `3`.
- `devToolingEnv()` did not set `NX_PARALLEL`, so CLI `--parallel` defaulted to that cap unless overridden.
- The inference plugin (`🟨️.mjs`) forced `parallelism: false` on native Cargo targets, component producers, and react/wgpu activation targets even though cached native builds use isolated `CARGO_TARGET_DIR` staging.
- `buildPluginCatalog` ran the Cargo stage strictly serially while materialize used a fixed cap of `4`.
- Authoring duplicated `parallelism: false` on many `📋️project.json` targets with isolated outputs.

## Singleton targets (stay serial at Nx layer)

- `deps-cargo`, `deps-wasm`, `deps-trunk`, `deps-tools` (workspace)
- `deps-tectonic`, `deps-tex` (print toolchain sync)

## Mechanism

- `semioNxParallel()` → `availableParallelism()` with optional `SEMIO_NX_PARALLEL` override.
- `semioNxParallelFlag()` → explicit `run-many --parallel=N` for workspace orchestrators.
- `devToolingEnv()` sets `NX_PARALLEL` for every bootstrap `bun nx` invocation.
- `orchestratorBudgetOpts()` / `daemonBudgetOpts()` merge `devToolingEnv()` so root `📜️script.ts` nx fan-outs inherit full CPU width.
- Nx patch defaults runner `parallel` to `availableParallelism()` when `nx.json` omits a cap.
- `policy.json` `nxSerialTargets` drives plugin-level `parallelism: false` only for singleton deps.
- `buildPluginCatalog` bounds both cargo and materialize stages with `semioNxParallel()`.
- Demonstrator `activate-dev` no longer blocks Nx pool slots (`parallelism: false` removed).

## Verification (2026-09-17)

- `cache-contracts` passes end-to-end (parallelism assertions, trunk lockfile, wasm outputs, runtime components, production browser artifacts, …).
- `semioNxParallel()` / `NX_PARALLEL` / `semioNxParallelFlag()` align with `availableParallelism()` on this host (10).
- `nx.json` has no `parallel` cap; runner default comes from the Nx patch when unset.
- Authored `parallelism: false` remains only on workspace `deps-trunk` and print `deps-tectonic` / `deps-tex`.
- Demonstrator `activate-dev` has no `parallelism: false`.
- Restored missing `.vscode/🧩️launch.seed.jsonc` Nx launcher entries required by cache contracts (WGPU wasm/lockfile, OS dev production builds).
- `cacheInternals.runtimeComponentClosure` uses a live getter plus `libraryBootstrap` export so contract tests see the async-loaded closure.
- Print multi-document `run-many` via bootstrap now spreads `semioNxParallelFlag()` explicitly (in addition to `NX_PARALLEL` from `devToolingEnv()`).

## Closeout (2026-09-17)

- Final `bun ./📜️script.ts test cache-contracts` exit 0 on this host.
- Repo MCP ticket tools were unavailable in this session; ticket status set in `🎫️ticket.json` manually.
