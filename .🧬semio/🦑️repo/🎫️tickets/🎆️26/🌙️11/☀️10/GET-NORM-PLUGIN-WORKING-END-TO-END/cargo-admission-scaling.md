# Cargo admission starvation

## Root cause
The 6932 MB figure was not per-job times jobs. The queued request of every waiting cargo run was `fixedMegabytes 6570 + 1 x perJobMegabytes 362`. The 6570 MB fixed block was the learned `phase:plan` floor (5 samples of the graph planner, about 6.4 GB) that `estimateHostPhasesV1` adds to the fixed block of every request (`scaleHostRequestV1`, `floorMegabytes`). `repo cargo` never plans a graph, but `withBuildCapacityV1` filtered the planner phase classes only for runs planned by the host service, so cargo inherited the constant floor. One job needed 6932 MB, more than the 5.3 GB the host could spare, so no job count helped.

## Already right (unchanged)
- The decision already admits `max jobs` between `minimumJobs 1` and `preferredJobs` that fit (`reduced`), waits only when one job does not fit, and passes later waiters while they fit; a head becomes a barrier only when draining our own grants would admit it.
- The granted jobs already reach cargo (`--parallelism=<jobs>`, `SEMIO_BUILD_PARALLEL`).

## Fix
`PHASELESS_COMMANDS_V1` (`cargo`, `command`) in `...library/🏗️build/🌡️host-capacity/🎯️planned-commands/🟦️.ts`; `withBuildCapacityV1` now drops the planner phase classes from the estimate and from learning for these commands as for service-planned runs. A cargo request is its tool alone: fixed block plus learned per-job footprint.

## Tests (managed Bun, direct)
- Hand cases in `...process/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json` (checked by evaluator and SQLite oracle): `cargo-oversized-request-runs-with-the-jobs-that-fit` (4 of 8 jobs), `cargo-waits-only-when-one-job-does-not-fit`, `cargo-waiters-are-admitted-side-by-side-while-they-fit` (4, 4, reduced 4).
- New `...library/🏗️build/🌡️host-capacity/🧪️tests/🦀️cargo-scaling` (JSON fixture + runner): store with a large learned plan floor, cargo `--parallelism=8` is granted fewer jobs and the argument follows; fails without the fix (waits), passes with it.
- 41 files, 280 pass; the only error is `🧮️arguments` needing `SEMIO_TEST_PYTHON_BINARY` (environmental). Host-capacity and estimates suites pass with it set. Log: `🗑️generated/cargo-admission-suites.log`; generator `cargo-admission-scaling/make-cases.ts`.

## Effects on waiting runs
Not re-evaluated. The request is sized once at start (old code, floor inside the queued request); polling re-reads only the decision. The 8 waiting `repo cargo` processes must be restarted.
