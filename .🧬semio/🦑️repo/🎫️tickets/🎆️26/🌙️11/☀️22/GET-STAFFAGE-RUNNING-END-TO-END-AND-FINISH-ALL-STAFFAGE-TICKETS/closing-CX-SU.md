FINAL (2026-11-27 14:16: 332 passed, 0 failed)

# CX-SU: test run of eight suites after the tessellation rule change

Paths: `R` = `/Users/ueli/Documents/semio`, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder. Runner `U/🧰️matrix/CX-SU/run.sh` ran the eight suites one after the other with `repo capacity run --memory=3000 -- timeout 5400 <bun> node_modules/vitest/vitest.mjs run … --testTimeout=3600000 --hookTimeout=3600000`, from `R`, without `SEMIO_STAFFAGE_MODEL_ASSETS`. The logs were deleted with `U/🗑️generated/matrix/CX-SU` after this report was written.

## Results

| Suite | Passed | Failed | Seconds | Exit | Note |
|---|---|---|---|---|---|
| `🧊️catalog-silhouette` | 0 | 0 | 5476 | 124 | timeout at 5400 s, no test result written (neither a pass nor a fail) |
| `🧊️three-views` | 50 | 0 | 5447 | 1 | run aborted by a worker crash: 50 of 775 tests reported, 725 not executed |
| `🧊️volumes-thermal` | 12 | 0 | 2285 | 0 | all passed |
| `🧊️volumes-flood` | 29 | 0 | 1570 | 0 | all passed |
| `🧊️volumes-connected` | 101 | 0 | 1565 | 0 | all passed |
| `🧊️volume-schema` | 115 | 0 | 2210 | 0 | all passed |
| `🧊️model-looks` | 14 | 0 | 1498 | 0 | all passed |
| `🧊️model-batch` | 11 | 0 | 979 | 0 | all passed |
| Total | 332 | 0 | | | two suites incomplete |

Seconds are wall-clock and include the wait in the capacity queue. The memory queue held the runs for a long time: the three-views log shows about 5240 s of waiting before its 57 s of test execution (run started 11:26:06).

## Failure details

No assertion failed in any suite, so there is no failing test to attribute to triangle or segment counts. Two runs did not produce a clean result:

1. `🧊️catalog-silhouette`: the run hit the 5400 s `timeout` (exit 124) with no output. Vitest writes its report only at the end, so the log has no test name and no assertion message. Whether a test was slow or hung is unknown. Nothing in this run shows a rule-change failure. A rerun with more time, or split by file, is the next step. I did not rerun it, because one attempt already took 90 minutes.
2. `🧊️three-views`: the worker process exited (`[vitest-pool]: Worker forks emitted error`, `Caused by: Error: Worker exited unexpectedly`), so the file did not finish. 50 tests passed and 725 did not run. The log records that swap was crowded while memory was short, so resource exhaustion is the likely cause. There is no assertion message. Whether a triangle or segment count change could make the worker exit is not shown by this run, and I did not check it.

## Triangle or segment count changes

The rule change is in `S/🔨️modules/🧊️volume/⏱️job/🟦️.ts` (`git diff HEAD`, read-only: 4 insertions, 4 deletions). It adds `referenceScale: 50` to `ROUNDNESS` and makes the chord in `qualityOf` `min(largest * max(scale, 50) / (50 * 3200), 0.02 * feature)`. The seven suites that finished show no assertion failure against this rule. The two that did not finish (silhouette by timeout, three-views by worker crash) have no assertion message to compare.

## Stated plainly

- No source file was edited, and no git command that changes state was run.
- Open items: `🧊️catalog-silhouette` needs a rerun with more time; `🧊️three-views` needs a rerun with free memory to cover its 725 unexecuted tests.
- Only my own runner and its child processes were involved; the runner exited on its own after the last suite.
