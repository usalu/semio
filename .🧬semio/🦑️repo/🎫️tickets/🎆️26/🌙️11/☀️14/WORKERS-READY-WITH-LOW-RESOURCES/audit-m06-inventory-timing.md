# m06 Inventory Reader Timing

Slice workers now report inventory batch timing in `slice-memory.json` under `sources.owners.presence.timings`.

| Phase | Measured interval |
| --- | --- |
| `reader` | From requesting the retained Graph consumer until its first Store verify begins |
| `preVerify` | The first Store verify, which includes Native storage revalidation |
| `body` | The inventory callback, including rechecks and retained-file observations |
| `postVerify` | The final Store verify, which includes Native storage revalidation |

Each phase reports the number of batches that reached it (`started`), the number currently in it (`active`), and cumulative `milliseconds` including active time at the sample. Milliseconds are summed across batches, so overlapping work contributes once per batch. An early withdrawal or refusal settles its current phase and leaves later phases untouched. The counters use the process monotonic clock and saturate at the maximum safe integer. The counters are scoped to the existing workspace and issued control and use the existing report cadence.

The change places observation points around the existing authority enrollment, verify calls, and callback. It introduces no extra checks or retries and keeps the receiver and verification ordering.

## Verification

The neutral vector covers overlapping batches, a batch ending after pre-verify, active samples, final settlement, and counter saturation. SQLite independently computes the interval totals and counts from the same vector. The coalescing and changed-inventory suites passed together with 12 tests and zero failures.

A genuine private Graph fixture made a managed inventory request and wrote the counters to the real slice-memory file. It observed one completed start in each of the four phases, with zero active counters. The summed milliseconds fell inside the outer interval measured with the process high-resolution clock. The fixture also passed alongside the existing physical inventory revalidation test (2 passing tests, 34 filtered, 44 assertions). Verification used the private fixture on the current host. Live m06 timing will be available from a worker running this revision.
