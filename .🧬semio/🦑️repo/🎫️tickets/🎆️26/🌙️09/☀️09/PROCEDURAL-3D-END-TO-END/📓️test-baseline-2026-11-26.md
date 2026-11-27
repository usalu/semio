# Test baseline (2026-11-26)

Raw outputs: `🗑️generated/test-baseline/` (dry-run plans: `rust-project-dry-run.json`, `geometry-dry-run.json`). Failing real-run output was not persisted to files; first failure messages are quoted below.

## 1. Rust: `the_preview_reissues_working_evaluations_as_compact_resumes`

- Location: `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️budget/🦀️.rs:213`
- Owning project: `@semio-tech/s-composition-laws-rs`. The budget file is compiled via `#[path]` into the `generation3d-app-laws-editor-commands` target (`🧪️tests/🧊️generation3d/🎮️editor-command-laws/🦀️.rs:76`), run by project target `test-generation3d-app-laws`.
- `mcp__repo__test_file` refuses `.rs` files (TS/JS only), so the project target was used.
- Attempt 1: `test` dry-run failed after 38m53s with `BUILD_SOURCE_CONFLICT` (conflicting inference observations during graph read). Not a test failure.
- Attempt 2: `test` dry-run succeeded (exit 0, 1005 s).
- Attempt 3, `test-generation3d-app-laws`: **FAIL, cargo never ran.** First failure:
  `AssertionError: generation3d-app-laws-viewer` at `🧪️tests/🔬️ownership/🟦️.ts:45`, sha256 actual `50d78fc6…1077` vs expected `eefa2953…1d98`.
- Result: **not executed (blocked by fixture hash mismatch)**. Whole-target filter to the single test is not available through the repo MCP tools.

## 2. Gherkin contract `example-geometry-3d-1`

Contract: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️example-geometry-3d-1/🥒️.feature`

- Dry run: `mcp__repo__run` target `@semio-tech/s-composition-laws-rs:test-generation3d-geometry`, `dry_run`, `en`: **PASS** (exit 0). Plan saved to `geometry-dry-run.json`. `test_file` cannot dry-run the `.py` lane.
- Rust lane (`generation3d-example-geometry`, via `test-generation3d-geometry`): **FAIL, cargo never ran.** First failure: `AssertionError: generation3d-example-geometry` at `🧪️tests/🔬️ownership/🟦️.ts:45`, sha256 actual `ca4c154d…77d5` vs expected `6621461f…2952`.
- TypeScript lane: **NOT RUN, no file.** `example-geometry-3d-1/` contains only `🥒️.feature` and `🐍️.py`. The runbook's TS lane (`🧩️example/🟦️.ts`) is not in that folder.
- Python lane (`🐍️.py`, run via `repo capacity run` with `/usr/bin/python3 -I`): **FAIL.** First failure: `ModuleNotFoundError: No module named 'numpy'` (line 41). Environment issue. Dependencies were not installed.

## Summary

| Test | Lane | Result | First failure |
|---|---|---|---|
| `the_preview_reissues_working_evaluations_as_compact_resumes` | Rust | not executed | ownership hash mismatch, `generation3d-app-laws-viewer` |
| `example-geometry-3d-1` dry run | repo plan | pass | none |
| `example-geometry-3d-1` | Rust | not executed | ownership hash mismatch, `generation3d-example-geometry` |
| `example-geometry-3d-1` | TypeScript | not run | no TS file in contract folder |
| `example-geometry-3d-1` | Python | fail | `ModuleNotFoundError: No module named 'numpy'` |
