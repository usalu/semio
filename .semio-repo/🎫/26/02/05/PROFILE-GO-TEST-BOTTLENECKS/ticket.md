---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Profiled per-test runtimes in semio-repo/cli, identified dominant bottlenecks (tree/markdown/nodes/lifecycle/list/e2e paths), and documented a fast-lane + slow-sharded strategy that preserves full behavioral coverage while reducing wall-clock feedback time.

## Changes

- Measured module-level test runtimes for `semio/go` and `semio-repo/cli`.
- Ran each `Test*` in `semio-repo/cli` individually via compiled test binary and ranked by elapsed time.
- Identified top bottlenecks:
  - `TestTreeCommands` ~33.0s
  - `TestMarkdownOutput` ~19.4s
  - `TestNodesAndEdges` ~19.4s
  - `TestNodesAndEdgesQuick` ~18.5s
  - `TestLifecycleCommands` ~17.8s
  - `TestCodebaseCommand` ~15.2s
  - `TestListCommands` ~13.8s
  - `TestCliE2E_MiscCommands_NoSideEffects` ~13.5s
  - `TestBreachsNonEmpty` ~10.0s
  - `TestGraphQLAnalyzeQuery` ~9.1s
  - `TestPolicyBreachListCommand` ~9.1s
- Confirmed isolated total across 70 tests is ~241.3s.
- Confirmed a slow-only shard (top heavy tests) runs in ~184.5s.
- Found `semio/go` tests fail quickly due missing fixtures (`../../assets/semio/*.json|*.zip`) in this workspace.
- Documented the execution strategy in `README.md` and `AGENTS.md`.

## Log

- Listed current goal tree using `./semio-repo/cli/cli --md goal tree`.
- Opened ticket `2026/02/05/PROFILE-GO-TEST-BOTTLENECKS`.
- Attempted ticket reopen on follow-up prompt; command returned `ticket is already open`.
- Enumerated Go modules and test files.
- Timed `go test ./...` in `semio/go` and `semio-repo/cli`.
- Generated full test list with `go test -list .`.
- Built test binary once with `go test -c`.
- Executed each test individually with `/tmp/semio-test-prof/repo_go.test -test.run '^TestName$'`.
- Sorted elapsed timings from `/tmp/semio-test-prof/per_test.tsv`.
- Ran a focused slow-test shard with `go test -run 'Test(...)' -count=1`.
- Inspected slow test implementations in `semio-repo/cli/main_test.go`.

## Todos

- None.

## Plan

- Keep full behavioral coverage but split execution into:
  1. Fast lane for local/dev and PR checks (`go test -short ./...` + explicitly maintained fast subset).
  2. Slow lane for heavy graph/tree/e2e/lifecycle tests.
  3. Parallel shard execution of slow lane by test groups to reduce wall time.
- For a follow-up implementation ticket:
  - Add a stable shard script (or make targets) that runs named regex groups in parallel jobs.
  - Ensure every currently measured slow test belongs to exactly one shard.
  - Keep one nightly/merge-gate full run to preserve end-to-end confidence.
