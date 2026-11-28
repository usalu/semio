# Coordinator status, 2026-11-27 11:20 CET

## Goal checklist

| Item | State |
|---|---|
| 5 signed components | **Eurocode published (10:55).** Structural is running in the chain; Materials is next. Base and Energy are rerun automatically after the chain (`publish-base-after-chain.sh`), followed by the sweep. |
| Strict 15-route sweep | Not started; runs after all 5 components. |
| EN 1990 physical replay | `ColdHeapQuote` in `ColdPreparationBounds` is implemented, allocation-free. Run 37 is queued (2,300 MB). |
| EN 1995 | **Done.** Peak 925 MiB (< 1,536); bridges 4/4, schema and edit-rules rc 0. |
| Job-context and job-receiving | Carve done, all receiving tests green. One target, `original-return-and-retirement-contracts`, is red only because of another session's live `[DEBUG]` `eprintln!` lines in the job crates; it is rerun in final verification. |
| Norm Rust suites | Hub norm package green (15). Plugin workspace: fixes applied to 8 failing targets, confirmation rerun in progress. Nested workspaces: bridge and en1999 compiling, 13 queued. |

## Problems found and fixed since the last status

- **Host starvation:** `fseventsd` held 15 GB. The administrator restarted it, and swap went from 8.8 to 1.3 GB.
- **Head-of-line blocking:** the publication chain's 4.6 GB request blocked the shared queue. The chain now pre-gates on spare memory for 5 minutes.
- **Repo infrastructure:**
  - `repo run` graph read failed because member packages were treated as scopes (`javascriptDeclaredScopesV1`).
  - The artifact reader failed on macOS lazily bumping directory ctime through its xattrs. The fingerprint no longer includes directory ctime; there is a test.
  - The component publication waited about an hour for a 20-hour-old dev MCP shared lease. I killed the holder; a structural fix (content-addressed component generations) is in progress.
  - The publication deadline was a fixed cutoff that lease waits could not extend. It is now the 6 h maximum, supervised by the idle window; there is a test.
- **EN 1990:**
  - The fixture copy budget was too small.
  - Leftover `dbg_add` code allocated on every denied advance.
  - Cold edits had an unquoted heap (fixed with `ColdHeapQuote`).
- **Job crates:**
  - The return-budget quote clamp was restored.
  - `close_step` performed what it settled.
  - Test caller fuel clamp, test lifecycles, deprecated `fetch_update`.
- **Norm plugin tests:**
  - oracle paths after the `📦️packages/🦀️rust` move;
  - missing EN/DE fault notices and fixtures.

## Anticipated

- Structural and Materials may hit the same reader lease until content-addressed generations land. The chain retries deadline and identity errors.
- Nested workspaces take 15–75 min each in a fresh lane, and 13 remain. This is the longest pole.
- Further breakage from concurrent edits by other sessions, such as debug lines and refactors.

## Expected finish

My estimate, not a measurement: about 4–6 h. Components: Structural and Materials about 1–2 h, then Base and Energy about 1–2 h, then the sweep. The nested suites run in parallel, about 4–6 h. Final verification and closing the ticket come after.

## Resource use

- **Memory:** it is the limit. 4 jobs are running and 4 waiting, with 606 MB spare. Swap is 2.3 GB, which is healthy since the `fseventsd` restart.
- **CPU:** load 5–7 on 10 cores. It is never the limit, because each heavy compile is pinned to one worker so it fits in memory.
- **Agents:** 6 are working:
  - EN 1990;
  - plugin workspace suites;
  - nested workspace suites;
  - component lease redesign;
  - job-receiving (done);
  - deadline fix (done).
- **Why not more agents:** every remaining step needs a compile slot from the same memory pool. More agents would only lengthen the queue.
- **Fleet:** used for final links. It cannot run the browser sweep (no network) or tests that need local state.
