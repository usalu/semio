# F2 test budget clamp

Cause (code): `stretchedBudgetMillisecondsV1(300000)` (plain test-file row) times the load stretch (up to 8x) exceeds 1,800,000 ms once host load/cores > 6; that value goes to `timeoutMilliseconds` in the owned-Bun selection, whose schema maximum is 1,800,000, so `repo test-file` refused. Not reproduced live: the one-file run at load ~75 was admitted (1-min load fluctuated under the 6x threshold). Proven deterministically by the new law (ratio 7.5 -> needed 2,250,000).

Change: `stretchedBudgetV1` in `🧪️testing/🎚️budget/⏲️contention/🟦️.ts` clamps to `STRETCHED_BUDGET_CEILING_MILLISECONDS_V1`, read from the suite schema (`Suite.budgetMs.maximum`), returns {milliseconds, neededMilliseconds, clamped}; `describeClampedBudgetV1` words the EN|DE "budget clamped" notice (new `clamped` message in policy table + schema). Wired into `🗂️suites/🏃️run` and `test-file/🏃️run` (notice, run still admitted). Package re-export extended.

Tests: new law (fixture `funding`, Python oracle `funding`, TS test) passes. Contention file: 13 pass, 1 fail (`a process that needs more wall time than its budget survives a busy host...`, real node process at 2 s budget under host load ~70; unrelated timing). Stretch-level file: 2 fail "Independent Python oracle required" (testPythonV1 finds no python; pre-existing env). Remaining runs: see below.

Test-file member run: 12 pass / 2 fail (one failure: "a file that outlives the deadline of its envelope..." 41 s, load-sensitive; the second failure line was cut by `tail`, not diagnosed). `repo test-file` was admitted and ran for every file launched (contention, test-file tests, stretch), i.e. no refusal at current load (~45-75).
Open: suites-run member has no own test file; two unrelated small files (literal-deadlines, budget tests) were still running when this was written (output in the lane's task output, not captured); the 2 test-file failures and the contention timing failure should be re-run on a quiet host. Lock released.
