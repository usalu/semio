# Fable Hub Native Qualification

Lane `fable-hub-native-qualification` (Claude Fable 5.1, Opus 5). Read-only verification lane: **no production code was edited**, no git-modifying command was run, no ticket lifecycle call was made, no other lane's `🗑️generated` subfolder was touched.

All receipts, stdout/stderr captures and the lane-local Cargo target live under
`🗑️generated/fable-hub-native-qualification/`.

**Status: IN PROGRESS — this file is written incrementally so evidence survives a session cut. See the per-gate table for what is currently credited.**

## Method

Each gate was run exactly as registered, with `--skip-nx-cache`, through the root Nx router.
`runExactCargoLaws` refuses to run unless `SEMIO_TEST_ARTIFACT_DIR` and `CARGO_TARGET_DIR` are absolute paths under a `🗑️generated` ancestor, so this lane supplied:

- `SEMIO_TEST_ARTIFACT_DIR=🗑️generated/fable-hub-native-qualification/<gate>-exact`
- `CARGO_TARGET_DIR=🗑️generated/fable-hub-native-qualification/cargo-target` (one lane-local target shared across all gates, so only the first gate pays the cold build)
- `CARGO_BUILD_JOBS=4`, `NX_ISOLATE_PLUGINS=false`
- `SEMIO_BUILD_BUDGET_MS=10800000` after the default 20-min/`BUILD_BUDGET_MS` 1,200,000 ms ceiling killed the first cold build (see gate 1 attempt 1).

The gate scripts themselves choose `--all-features` for `semio-hub`; that is the registered target body, not a lane choice.

One cargo/nx process at a time throughout. Because the tool call ceiling is 600 s and a cold `semio-hub --all-features` test build exceeds two hours on this host, long gates were launched detached with `nohup … & disown` and polled; at no point was more than one lane process running.

## Per-gate results

(filled in as gates terminate — see below)

## Host conditions

The host was under a sustained load average between 36 and 377 for the whole window, with four sibling Fable lanes plus the Sol/Terra fleet compiling concurrently. `semio-s-plugin-stdio` alone took over 100 minutes of wall time inside this lane's build.
