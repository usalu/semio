# F21: blind worker failure logs, native-storage test drift

## 1. Failed task output in the stage failure log
Cause: the Nx failed-task output was in the stage log, but (a) coloured `> nx run X` headers were not recognised, so attribution (and the one-line reason, empty on m02/m03) failed, and (b) the failure mirror keeps the last 200 non-noise lines plus 8 early error lines, so the text of every failed task except the last was elided.
Change: `failure-tail` (schema + `nxFailedTaskTailsV1`, ANSI stripped, shared section parser); `🚦️ready/🟦️.ts` `failedTaskTailLinesV1` appends the last 60 own-output lines per failed task (max 16 tasks) to `ready-log-<stage>-failed.txt` and the last 10 per task to `ready.json` tail (schema `failedTaskSummaryLines`). Landing details in `T/landings.md` (Wave 1125 F21).
Tests: failure-log 8/0 (new, Python cut oracle), ready main 58/0, refresh 10/0, source-transient 4/0, stale-stage 15/0, nx summary 5/0, failure-tail 9/0.
Not verified: live on a worker (reaches fleet after supervisors restart); window is the last 1 MiB of the stage log, a failed task whose block is older is not found.

## 2. Native-storage tests
`📥️release`: ReferenceError in the TEST: it evals the callback of `openPersistentNativeImageBodyV1` which now closes over the parameter `existingOnly`; binding added. `📥️persistent`: product refactored the opener into `openPersistentNativeImageModeV1`; assertions updated. Product untouched. Both pass.
