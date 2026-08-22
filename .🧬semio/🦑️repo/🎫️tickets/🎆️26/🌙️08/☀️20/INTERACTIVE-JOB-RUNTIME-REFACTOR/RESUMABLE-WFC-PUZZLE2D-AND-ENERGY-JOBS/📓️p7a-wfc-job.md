# P7a Persistent WFC Job

## Scope

- Removed 691 decorative `async fn` declarations and converted 270 non-suspending async tests throughout the assembly WFC engine. The final engine census has zero `async fn`, `.await`, or `async_test` hits.
- Added the universal `WfcJob<T>` interactive state machine and mounted it through the procedural Rust package.
- Kept the ordinary `solve` and `solve_cancellable` APIs as synchronous batch drivers. Restart/nogood advanced modes and a bounded hard-instance fallback continue to use the proven synchronous solver internally; the interactive API never exposes a monolithic run-to-completion step.

## Persistent State Machine

`WfcJob<T>` owns the compiled model, topology, optional initial domains, revision generations, stale-entry-safe entropy heap, FIFO propagation queue, compatibility-edge cursor, removal trail, decision/backtrack frames, seeded xoshiro state, deterministic counters, preview deltas, and contradiction state.

Its stages are `InitializeDomains`, `FindMinimumEntropySlot`, `ChooseCandidate`, `PropagateCompatibilityEdge`, `DetectContradiction`, `BacktrackTrailEntry`, `CommitSlot`, and `Complete`. A work unit initializes one slot, pops one entropy entry, chooses one candidate, checks one compatibility edge, or undoes one trail entry. Cancellation, generation freshness, deadline, and fuel are checked before mutation and after each unit.

Typed previews expose the active slot, live candidates, tested tile, propagation wave, changed domains, contradiction, backtrack path, partial assignment, sequence, and counters. The encoded preview uses the same data. Only `Complete` produces a commit candidate.

Checkpoints contain every mutable cursor and RNG word. Restore rejects operation, model fingerprint, or topology-size mismatch and resumes with byte-identical checkpoint state and deterministic result parity.

## Verification

- Formatting: final changed WFC leaves formatted with `rustfmt --edition 2021`.
- Source census: `rg -n 'async fn|async_test|\.await' <wfc-engine>` — zero hits.
- Full focused debug: `CARGO_TARGET_DIR=<ticket>/🧪️target-p7-wfc cargo test --manifest-path <ticket>/🧪️wfc-focused/Cargo.toml` — exit 0; **275 passed, 0 failed** in 0.13 s (`📝️p7a-full-debug-9.txt`).
- Full focused release: the same command with `--release` — exit 0; **275 passed, 0 failed** in 0.05 s (`📝️p7a-full-release-2.txt`).
- Wasm: `cargo check --target wasm32-unknown-unknown --manifest-path <ticket>/🧪️wfc-focused/Cargo.toml` in the same ticket-local target — exit 0 (`📝️p7a-full-wasm-2.txt`).
- Product structured checks reached concurrent framework/plugin and then stdio migration walls before procedural; the latest preserved product log has zero WFC-local diagnostics (`📝️p7a-procedural-check-3.json`). The focused harness mounts all 41 production WFC modules rather than copied implementations.

## Test Coverage

- deterministic results across fuel batch sizes;
- byte-identical checkpoint restore and resumed-result parity;
- pre-cancellation and stale-generation no-mutation;
- disjoint, empty, and adversarial 4,096-slot graphs;
- compatibility-edge-bounded propagation/backtracking;
- a 65,536-slot one-work-unit `<8 ms` watchdog;
- the pre-existing full solver, property, constraint, grid, sparse, symmetry, repair, chunk, and topology suites.

The ordinary slice-boundary yield is zero-copy; callers request full checkpoint bytes explicitly, so a large domain cannot force synchronous serialization on every fuel exhaustion.

## 2026-08-21 current-tree reverification

- Focused debug: **275/275 passed**.
- Focused release: **275/275 passed**.
- `wasm32-unknown-unknown`: passed.
- `wasm32-wasip2`: passed.
- The production constraint-enum comment was changed from a rustdoc comment to a regular contract
  comment because rustdoc cannot attach documentation to the closing macro invocation.

The ticket remains open as required.

## 2026-08-22 live inference integration

- Removed the assembly inference path's direct `GraphSolver::solve` call.
- `AssemblySolve` and `AssemblyContradiction` now compile a `WfcJob<GraphTopology>` with the snapshot seed and pinned modules, then use the shared `run_to_completion` headless adapter with one fuel unit per step, a 2 ms per-step budget, cancellation, trace correlation, and the global watchdog.
- Added a crate-visible immutable operation accessor to `WfcJob` so the adapter cannot manufacture an identity that differs from the job it drives.
- Converted the stale assembly `InferredField` implementations, helpers, and ten inference tests to the store's synchronous trait contract.
- Source validation: `rustfmt --edition 2021` completed; the inference leaf has zero `async fn`, `async_test`, `compile_and_solve`, or direct `GraphSolver` hits; `git diff --check` passed for the integration leaves.
- Current-tree Cargo validation is pending the repository's serialized Cargo window and is not claimed by this entry.
