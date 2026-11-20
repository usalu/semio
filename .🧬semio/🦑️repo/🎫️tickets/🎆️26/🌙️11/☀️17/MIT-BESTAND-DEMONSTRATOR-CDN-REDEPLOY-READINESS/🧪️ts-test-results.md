# TypeScript suites of the 2026-11-17/18 changes

Run natively: `repo capacity run --memory=1024 -- repo tools exec bun -- test ./<file>` (operand needs `./`), one file at a time, `SEMIO_TEST_ARTIFACT_DIR` under `.cache` (short path; the ticket path exceeds the 256-character bound), envelope via `native-launch.ts`. Runner: `ts-run.sh`, `ts-run2.sh`, list `ts-suites.txt`.

| Suite | Pass | Fail | Note |
|---|---|---|---|
| describe emitter sealed recheck | 1 | 0 | |
| describe emitter request | 2 | 0 | |
| native-transpile | 2 | 0 | |
| image reclaim | 16 | 0 | |
| image owners/ index file | 0 | 0 | index has no tests |
| image sparse | 6 | 0 | |
| image silence | 7 | 0 | with envelope and short artifact dir (6 of 7 without) |
| fixed image livelock | 6 | 0 | |
| lane retention dead-holder | 7 | 0 | needs `--timeout 120000` under load (one pre-existing 5 s test timed out at 16 s) |
| lane retention-io | 10 | 0 | short artifact dir under `.cache` |
| project-farm (26 files) | all but 2 | 2 | see below |
| budget | 16 | 2 | "Actual Native platform image required": needs the runner's real native image, not a scratch dir |
| component-pass | 10 | 19 | needs the repository launcher, enforcing store and cargo ingress (`Original compiler ingress requires the repository launcher`); not runnable natively without cargo |

## Project-farm failures (not fixed)
- `🚧️no-inline`: `open("silent",...)` returns undefined; `🪶️lean`: "serving host at two cores" yields a farm. Both depend on host memory and the real capacity lease (`reserveFarmV1`, `leanSliceV1`); `plan-stage`/lean code was not changed today (only slice-report, pool queue/refuse, host-capacity settled rule). Likely host-state dependent; unconfirmed without a stash-free baseline.

## Fixes
None needed in TypeScript.
