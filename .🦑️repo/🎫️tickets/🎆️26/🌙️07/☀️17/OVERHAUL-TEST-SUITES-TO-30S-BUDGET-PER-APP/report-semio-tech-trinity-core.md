# @semio-tech/trinity-core — Test Suite Overhaul Report

- Root: `trinity/rewrite/engine` (Rust crate `trinity_rewrite` at `trinity/rewrite/engine/rs`)
- Nx project: `@semio-tech/trinity-core`

## Before

- Runner: `trinity/rewrite/engine/script.ts` `TestScript` called `runCargo(["test", "-p", "trinity_rewrite", ...segments], this.repoRoot, playPollingEnv())` directly — not budget-enforced (no hard wall-clock kill).
- Test file: `trinity/rewrite/engine/rs/lib.rs`, `mod tests` (lines 1313-1455), 11 `#[test]` fns.
- Baseline `cargo test -p trinity_rewrite` execution (warm build, isolated `CARGO_TARGET_DIR` + `RUSTC_WRAPPER=""` to sidestep unrelated machine-wide sccache/build-lock contention from other concurrent sessions): 11 passed in 0.02s.

## Test classification (all 11 kept — none trivial)

Every existing test exercises real branching/algorithmic logic against the nakagin-capsule-tower fixture, not export-exists/getter/serde-padding boilerplate:

- nakagin_fixture_loads, nakagin_flat_position_derived — JSON graph parsing + derived flat-position geometry computation.
- jack_query_on_nakagin — Jack query-language execution (MATCH ... WHERE ... RETURN).
- rewrite_rule_labels_core, rewrite_rule_parameter_substitution — graph-rewrite rule application incl. parameter binding/substitution and generated-query-string assertion.
- rewrite_labeled_fixture_reloads — rule application + fixture serialize/reload round-trip (kept: exercises rule engine + serializer together, not a plain struct round-trip).
- trinity_host_rebuilds_engine, trinity_host_reorganize_moves_nodes — host/engine/board assembly and the reorganize layout algorithm (position-mutation assertion).
- trinity_host_tokenize_jack_json, trinity_host_complete_jack_json — Jack tokenizer and autocomplete logic.
- trinity_host_jack_create_undo — CREATE + undo state-machine behavior.

Removed: none. No export-exists, getter/identity, CSS-substring, plain serde round-trip, enumerated-list, or loop-duplicate cases were present to delete.

## Runner migration

`trinity/rewrite/engine/script.ts`:
- Import swapped `runCargo` -> `runCargoTestBudgeted` from `../../../repo/lib/js/index.ts`.
- `TestScript.run()` changed from `runCargo(["test", "-p", "trinity_rewrite", ...segments], this.repoRoot, playPollingEnv())` to `runCargoTestBudgeted(["trinity_rewrite"], this.repoRoot, segments, playPollingEnv())` — mechanical swap, `this.repoRoot`, `segments`, and `playPollingEnv()` env untouched. This now does an un-budgeted `cargo build --tests -p trinity_rewrite` followed by a budgeted (`SEMIO_TEST_BUDGET_MS` / 30s default, hard SIGKILL) `cargo test -p trinity_rewrite`.
- `project.json` already only called `bun ./📜️script.ts test`; unchanged.

## After

Re-ran through the real entry point (`bun ./📜️script.ts test`, same isolated target dir / no-sccache override used only to avoid an unrelated environment-wide sccache-server queueing stall other concurrent sessions were causing — not a code change):
- `Finished test profile ... in 2.84s` (build check, uncounted), then `running 11 tests ... test result: ok. 11 passed; ... finished in 0.02s`.
- Total wall time for the whole `bun ./📜️script.ts test` invocation: ~6.5s, dominated by bun/cargo startup, not test execution.

## Result

- Before (execution only): 0.02s (already fast; the project just lacked budget enforcement).
- After (execution only): 0.02s, now hard-capped at 30s via runCargoTestBudgeted.
- Tests before/after: 11 / 11 (no deletions — all exercise genuine logic).
- Within budget: yes, by a wide margin.

## Note on measurement environment

While baselining, the shared workspace target/ dir and its .cargo-build-lock, plus the repo's .cargo/config.toml rustc-wrapper = "sccache", were both saturated by many other concurrent sessions' simultaneous cargo builds (dozens of queued sccache-wrapped rustc invocations, sccache --show-stats showing ~83s average compiler time with most work stalled at 0% CPU for 25+ minutes). This is the known "concurrent cargo workspace churn" pattern — not a bug in this unit. Worked around for measurement purposes only by building in an isolated CARGO_TARGET_DIR (this session's scratchpad) with RUSTC_WRAPPER="", no repo files (.cargo/config.toml, etc.) were modified.
