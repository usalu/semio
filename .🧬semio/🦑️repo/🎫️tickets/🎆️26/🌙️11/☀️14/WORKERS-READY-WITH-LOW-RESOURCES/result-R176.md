# R176: code-stale no longer stops a stage that executes planned tasks

Landed 2026-11-18 03:02:12 to 03:03:06 UTC under `landing.lock` (freeze exception from the coordinator), entry in `landings.md`. Scripts: `FLEET-.../🧰️scripts/R176/` (`patch.py`, `preload.ts`, `land.sh`).

## Where
Supervisor `🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts` (`runReadyStageV1`, tick every 2 s), rules in `🌲️process-tree/🟦️.ts` (+ schema `Policy`): `readyStaleRestartV1`, `readyCodeChangedSinceV1`, `staleStageMilliseconds` 30 min x R135 factor, `codeRoots` = process, caching, build library.

## Rule now
- `readyStagePhaseV1(lines, previous)`: `executing` after `Running target ... for project|N projects`, `> nx run`, `graph: plan answered` (EN/DE); `reading` on `Reading the project graph`, `Loading the project graph`, `Planning every target`, `Lese/Lade den Projektgraph`. Latest marker wins, none keeps the previous phase, a stage that never says stays unknown.
- `readyStaleStopV1(phase, phaseAge)`: never in `executing`; reading or unknown: the old rule, but age and the changed-file comparison count from the phase start (a second read after tasks is a new read on current code).
- The supervisor now reads every new log line (`logSinceV1`, offset, last 4 MiB), not the 60-line tail: the flood test (600 lines after the marker) is red with the tail window.
- Closure: the process does not report loaded-module digests, so the closure is the build-library roots minus tests: `codeSkipDirectories` gains `🧪️testing` (`🧪️tests`, texts, target, node_modules already skipped). Lab, ticket and tooling files are outside the roots.
- 30 minutes: not a fixed number, it is `staleStageMilliseconds` x the machine speed factor (R135, `readyScaledTreePolicyV1`), unchanged and proven by a test.
- Not touched: the holder-wait `code-changed` rule (10 min waiting for a named holder), and R154's unrefreshed-facts restart.

## Proof
`🧪️tests/🕰️stale-stage` 14 pass (9 new): phase table, law table (executing never stops; reading/unknown stop at the age; scaled age), executing + change runs to exit 0, reading + loaded-file change stops `code-stale`, reading + test/testing change no stop, never-phase falls back (stops), second read counts from its own start, flood. On the old code the new behaviours fail (6 of 6 integration/law tests red). Process-tree 15/0; staged runs of ready tests 40/1 (the same 1 on the unchanged tree), resume 8/0, stage-plan 2/0.

## Workers (03:05 UTC, `/Users/Shared/semio-store/v1/origins/g1-m*/ready.json`)
No worker shows `code-stale` now (reasons: adopted x5, previous-failed m01, stalled m06, m02 backoff `stage-failed`). Nothing stopped an executing stage since the landing; the running supervisors still hold the old code until their next restart, so the new rule applies only after each worker restarts its readiness loop.
## Not verified
Real-run phase detection on a worker log (no log with the marker lines was observed live; the Nx marker text is from Nx output and the failure-tail fixtures); the 1 baseline ready-test failure is not mine.
