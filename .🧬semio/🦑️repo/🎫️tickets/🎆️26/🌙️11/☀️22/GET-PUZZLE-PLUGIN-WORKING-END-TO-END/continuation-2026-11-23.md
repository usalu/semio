# Continuation after the Wave 1 fleet stopped

## Already green

- TypeScript, source laws, graph generation, and hub retained ownership: `lane1-ts-and-laws.md`.
- Board host: 54/54, including the closed lasso, in `🗑️generated/lane2-board-host.log`.
- Puzzle root workspace (`jobs`, `retained`, `board-scene`): `PUZZLE_ROOT_EXIT=0` in `🗑️generated/lane2-puzzle-root.log`.
- Puzzle 5D workspace `cargo check` and the host-admission package check both exited 0. The later per-package `cargo test` runs died inside `rustc` with no `error[` diagnostic while `codegen-units=1` was forced. That is a compiler kill, not a puzzle assertion.

## Still open

- Puzzle 2D package loop was aborted during the first crate's compile. `🗑️generated/lane2-2d-summary.log` only contains `LANE2_BEGIN semio-s-artifact-puzzle-2d`.
- Puzzle 3D `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` compiled and failed. One translate costs about 88 publication turns and 1531 store units on both the 1-object and 180-object fixtures, so the ladder is size-independent and still above the 8-turn ceiling. Each host turn stopped after about 28 preparing steps because the interactive item grant was 64 and the worker wall was 4 ms.
- `semio-hub-puzzle`, the wasm package, Storybook, and the live `en`/`de` editors are not started.

## Change now under test

`MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` now grants 512 retained items per turn, and `INTERACTIVE_TURN_WORKER_WALL_US` is 50 ms, so one host turn can carry the publication census. The temporary pump-stop log and publication-bucket counters are removed. The 3D latency law is rerunning through `native-cargo.sh`.
