# Result H24: metrics Cargo workspace error

## Root cause
`🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/📦️packages/🦀️rust/Cargo.toml` (added by H5) declared its own empty `[workspace]` and had a nested `Cargo.lock`. The repo library (`🗂️workspaces/🦀️cargo/🟦️.ts`, `selectPlan`) then took that manifest as its own owner with default members `["."]`; the walk admits none, hence "no current source-bound members" for every crate depending on it. `cargo check --manifest-path` alone worked because cargo accepts a standalone workspace.

## Fix (landed immediately under the 01:40 exception; no landing lock, no landings.md entry, no repo typecheck run)
- Removed `[workspace]`; added `workspace = "../../../../../../.."` (root) like every other framework crate; root `Cargo.lock` already lists the crate.
- Moved the nested `Cargo.lock` to scratch (`H24-metrics-Cargo.lock`).

## Workers
No `ready-log-*` under `S/origins` that I grepped contains "source-bound"; only g1-m01 files were listed, other origins not checked.

## Dashboard test (H5 Rust headless Fleet window)
- Tests live in `semio-framework-repo-dashboard-terminal-laws` (not the terminal lib). `repo cargo` needs `--store=shared --configuration=prototype`.
- First run: 2 of 3 `a_drill_down*` passed; the new EN/DE skip+decline test failed in German: the assertion expected the full label as row prefix and `/min`, but the label column truncates with an ellipsis and German is `/Min.`.
- Test-only fix in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/📊️fleet/🦀️.rs` (prefix of 30 chars, case-insensitive `/min`).
- Second run `-p semio-framework-repo-dashboard-terminal-laws fleet_window`: 14 passed, 0 failed, 1 ignored (live store).
- An earlier run with a wrong cwd/package gave "did not match any packages" after 24 min; not reproduced afterwards.
