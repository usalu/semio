# Unit-cache suite repair

## Priority finding
No class (a) defect. None of the 11 reported failures points at a unit-cache defect that could corrupt or break the running release. All are test drift (b), environment/harness needs (c), or non-test helper files that the sweep ran as tests. No production code was changed.

## Per file
| File (under `unit-cache/` unless noted) | Class | Cause | Result |
|---|---|---|---|
| `🏛️seal/🧪️tests` | c | Needs managed binding `SEMIO_TEST_PYTHON_BINARY` (canonical suite tool preparation). | 9/9 pass with the pinned python bound. |
| `🧹️gc/🧱️units/🧪️tests` (1 case, explicit reclaim) | b | Test made its store with a bare `mkdtemp(tmpdir())`. The scratch preload puts `tmpdir()` inside the checkout, so `profileLeasePortV1` resolved the lease database of the live checkout and the lease fence refused. A second, latent error: `capacityBytes:1` gives budget 0, so the planner (correctly) removed the young units too. | Fixed: uses the file's `privateWorkspace` (own store and lease dir); capacity derived from `unitBudgetBytesV1` as 2.5 units. 13/13 pass. |
| `🧪️tests/🏖️scratch/🧪️tests` (9 cases) | b | Test root was `mkdtemp(tmpdir())`; the scratch preload (`🧹️scratch`) moved `tmpdir()` to `<checkout>/.cache/scratch`, so Cargo reached the checkout `.cargo/config.toml` and workspace. Also needs `SEMIO_TEST_CARGO` and the pinned toolchain env (c). | Fixed: root below `cargoScratchBaseV1()`. 9/9 pass. |
| `🧪️tests/🕰️freshness` (2 cases) | b | Same cause: `base=resolve(tmpdir())` inside the checkout, so the checkout wrapper config reached the scratch Cargo build and no deps dir appeared. | Fixed: `base=cargoScratchBaseV1()` (as churn, complete, incremental already do). 2/2 pass. |
| `🧵️lane/📤️retention/🧪️tests/🔐️retention-io` | flaky under load | Passes 10/10 on three consecutive runs here. Likely timing-sensitive under the sweep's host load (load average 11 with the release running). Not reproduced; no change. | pass |
| `🛰️fleet/🧪️tests/🔢️count/🟦️.ts` | c (not a test) | A compiler shim script (`bun count.ts <log> <crate> <delay> -- <compiler...>`), spawned by the fleet tests; run as a test it throws on missing arguments. Exclude from the test sweep. | n/a |
| `...retention-io/📥️worker/🟦️.ts` (worker helper; also `🧪️tests/📥️worker`, `🏊️pool/.../📥️worker`, `🪦️dead-holder/📥️worker`) | c (not a test) | Child-process helpers of the retention, pool and dead-holder tests. Not test files. | n/a |
| `remote-execution/🧪️tests/🔧️bazel` (real Bazel) | c | The test is `skipIf(!SEMIO_TEST_BAZEL_BINARY)`; it needs the pinned Bazel (`.cache/b/t/m/bazel/bin/*/bazel`) and a JVM. Not run: host load average 11 with a release running, and a real Bazel run is heavy. Here it only skips. | not run (skip) |

## Edited files
- `unit-cache/🧪️tests/🕰️freshness/🟦️.ts`
- `unit-cache/🧪️tests/🏖️scratch/🧪️tests/🟦️.ts`
- `unit-cache/🧹️gc/🧱️units/🧪️tests/🟦️.ts`

## How the runs were made
Pinned bun, one file at a time, `SEMIO_TEST_ARTIFACT_DIR=/private/tmp/claude-501/art`, `SEMIO_TEST_PYTHON_BINARY` (pinned cpython 3.12), `SEMIO_TEST_CARGO`/`SEMIO_TEST_RUSTC` and `RUSTUP_HOME`/`RUSTUP_TOOLCHAIN` set to the pinned nightly. The seal and scratch files need these bindings; the sweep should run them through the repo envelope.

## Follow-up for the sweep
- Exclude non-test helpers (`count`, every `📥️worker`) from the file glob of sweeps.
- The scratch preload makes `tmpdir()` unsuitable as a Cargo scratch base for any test; other files still using `tmpdir()` for Cargo builds (not failing in the sweep) should move to `cargoScratchBaseV1()` as they are touched.
