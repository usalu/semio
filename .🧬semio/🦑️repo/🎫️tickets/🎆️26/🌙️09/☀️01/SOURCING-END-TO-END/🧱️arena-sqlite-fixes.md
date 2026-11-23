# 🧱️ Arena cascade, sqlite encode refusals, pane smoke (2026-11-23)

## A. UI arena cascade
Root causes (two, independent):
1. `UiArenaFixture::drop` only closed the arena when the test was already panicking; a refused close (or a close that panicked in a passing test) leaked the lease and left the process-wide registry pointer set, so every later `UiArenaFixture::new()` failed with `UI arena already has an actual lifetime owner` (10 failures in the lib run).
   Fix: `🧪️fixture/🦀️.rs` wraps every close in `catch_unwind`; on failure `abandon()` forgets the unclosable Source and calls the new testkit-only `UiArenaRegistry::abandon` (`📌️registration/🦀️.rs`), which resets pointer/closing/poison and bumps the epoch, plus `UiArenaHandbacks::abandon` (`♻️retirement/📮️handback/🦀️.rs`) so stale handbacks never reach the next arena. A passing test whose close fails still reports its failure (`resume_unwind`) but can no longer poison others.
2. The 4 `hoisted_tests_8` pool tests that failed at close (`UI arena descendants remain borrowed at lifetime closure`): a dropped `BuiltNode` tree does not release its values; `BuiltChildren::drop` queues its pages in the process-wide built-child retire authority, which only `close_built_node_page_one()` drains (the reactor pump in production). Live debug of the registry showed 8 occupied Map collections (the toggle `{moduleId,enabled}` args of the filter bar) owned by those queued pages.
   Fix: curation test support `context::arena()` / `ArenaScope` drains the built-node pages before returning the arena; the four pool tests use it.

## B. SQLite native encode "quoted allocation effect"
The refusal text no longer exists anywhere in the source: the framework encode allocation migration (value `🛫️encode`, `🧪️test/🛬️original-snapshot-owners`) landed concurrently. With the current tree all 19 curation sqlite snapshot tests pass.

## C. aussuchen_pane_smoke
Fails inside `pane_smoke_law` at the archive door: `document archive parent Pack and SPR hydration was rejected at Fold: Replay: native construction exceeds original remaining Job grant`. This is the job-grant archive-load family owned by the process agent, not arena or sqlite related.

## D. bun oracle tests
The three sqlite tests spawning `bun` (distinct_logical_and_addressed_children, independent_queries_and_complete_surrogate_edits, independent_malformed_sql) pass with bun on PATH.

## Results
- Full curation lib before: 63 passed / 33 failed (10 `already has an actual lifetime owner`, 4 sqlite encode, 4 arena close). After the registry abandon fix alone (`🗑️generated/af1.txt`): 65 / 31, zero lifetime-owner failures, all 19 sqlite tests green. After the `ArenaScope` drain (`af5.txt`, `hoisted_tests_8`): the four close failures are green, 9 pass / 7 fail; projected lib 69 / 27 (not rerun in full).
- Every remaining failure is the typed-operation settlement stall (`complete original typed operation settlement`), the archive-load job-grant refusal, `captured shared original changed its Work or destination` (`renders_pool_table_scene` at paid app close), or the history acceptance law: all owned by the process agent.
