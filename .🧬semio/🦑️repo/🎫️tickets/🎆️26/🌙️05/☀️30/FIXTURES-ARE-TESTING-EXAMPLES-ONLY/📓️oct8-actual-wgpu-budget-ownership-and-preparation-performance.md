# Actual WGPU Budget Ownership and Duplicate Preparation Review

Read-only during active actual WGPU session 58744; no settings, source or budgets changed. This direct registered Nx command is launched by exec without a temporary budget wrapper. Whitelisted live environment observation of actual Nx32370/native owner32581/native dependency32622 finds no `SEMIO_ORCHESTRATOR_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS` or `SEMIO_CARGO_BUDGET_MS` overrides. No other environment values were emitted.

Current process budget owner defines BUILD/CMD/ORCHESTRATOR/DAEMON defaults as zero and validates explicit overrides. Native orchestration's actual `native:owner-command` delegates `runOwnedCommand(...,0,...)`. Native dependency synchronization supplies a timer only when `orchestratorBudgetMs()` is positive; it is zero in this observed current process. The actual Trunk owner later uses its own existing `buildBudgetMs()` and cancellation signal, never a ticket timer wrapping the summed Nx prerequisites.

Previous48039 raw failure identifies Cargo fetch101 refusing the current S lock after composition preparation; its stack names `runTool`, `prepareDependencies`, and Nx marks the wasm target NOT RUN because workspace:deps-cargo failed. The raw output contains no budget-expiration exception. Its observed overall Nx130/10m3s does not establish a10-minute compiler or DAG budget. Retain that actual numeric terminal separately from the explicit lock refusal; do not infer timeout merely from elapsed time or exit code.

Duplicate preparation is independently real: each normal update and locked fetch enters the generic Cargo preparer separately, which recreates a per-call recipe dedup set. Current active job continues through actual standalone authored workspaces. Parent directs preserving it through authoritative terminal. Safe later optimization requires exact owner/preparation source/recipe input/manifest/member/output custody and currentness before fetch with the expected lock update separated. No arbitrary skip flag, unknown-input waiver, source mutation during active build or weakened locked verification is justified.

Current source SHA identities:

- `🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts`: `01afaba090403b6de22715d69f82d303ef665f6ba2bd633e4eef5ffdb1fc2c37`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`: `da09924ddbc385cee21242e315ebf79701e2842d0e98ab8f6d1cf95a3628feba`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts`: `0588e7f1fe4ba2c40430ffcb653f7fdf75554a7d9e3193c22037fd0417122730`
