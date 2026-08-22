# Phase 4 Closure Blocker Repair — 2026-08-22

## Scope

This repair addresses the four source/runtime blockers in `📓️p4-closure-audit-20260822.md`: the inline `fillBuildTick` route, monolithic fill transitions, Puzzle 3D worker-count parity, and first-substantive-preview timing.

## Source Result

- `fillBuildTick` only polls the current fill observation and enqueues one isolated `Effect::SpawnJob`; it performs no solver transition inline and suppresses duplicate live-job requests.
- The registered shared plugin job exclusively drives fill resumes, coalesces internal cursor yields, publishes only visible changes, and checkpoints only stable accepted/done changes.
- Target blocking/enumeration/weight construction/weighted selection, candidate enumeration/classification/draining/weighted selection, broad-phase materialization, attraction validation, vortex construction, and placement commit are persistent cursor phases.
- Broad phase checks one placed entry per resume instead of materializing a full spatial query.
- Checkpoints persist cursor state plus append deltas and a stable seeded-base fingerprint. They no longer serialize duplicate full base and working fixtures inside an accepted-placement resume; restore reconstructs the working fixture from the already-seeded base and append deltas.
- Exhausted target cursors terminate instead of wrapping modulo the target vector.

## Focused Native Debug Evidence

The current debug test binary is `target/debug/deps/semio_s_plugin_puzzle-dc22081ae00757c2`.

| Gate | Result | Runtime evidence |
| --- | --- | --- |
| Poll/enqueue-only isolated route | Pass | `fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job`, 0.09 s |
| Checkpoint byte restore | Pass | `checkpoint_restore_is_byte_identical`, <0.01 s |
| Resume equivalence | Pass | `fill_job_checkpoint_resume_matches_uninterrupted_execution`, 0.01 s |
| Drive-batch determinism | Pass | `fill_job_is_deterministic_across_drive_batch_sizes`, 0.01 s |
| First substantive preview | Pass | final isolated sample: first preview 1,238 µs; maximum resume 422 µs |
| Worker-count commit parity | Pass | worker counts 1, 2, 4, and default 10 each produced a byte-identical 2,691-byte commit containing one accepted placement; final isolated run 0.56 s |
| Adversarial whole-fill watchdog | Pass in isolated timing run | 1,024 placed objects; final isolated sample: first preview 650 µs; maximum resume 181 µs; 3,122 transitions; 0.03 s |
| Native debug compile | Pass | `cargo check -p semio-s-plugin-puzzle --message-format=short`; finished dev profile in 4m 09s after shared-lock wait |
| Native release library compile | Pass | `cargo check --release -p semio-s-plugin-puzzle --lib --message-format=short`; exit 0, finished release profile in 7m 06s |
| `wasm32-unknown-unknown` compile | Pass | `cargo check -p semio-s-plugin-puzzle --target wasm32-unknown-unknown --message-format=short`; exit 0, finished dev profile in 9m 12s |
| `wasm32-wasip2` compile | Pass | `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2 --message-format=short`; exit 0, finished dev profile in 4m 47s |
| Collision resumability | Pass | eight overlap checkpoint/cancellation/batch/watchdog/reference tests and two spatial-index exact-order/adversarial-span tests passed in isolated current-binary runs |
| Owned-source formatting | Pass | `rustfmt --edition 2021 --check` over the action, precompute bridge, fill builder, brush helpers, and collision geometry; exit 0 |

The seven-test grouped nextest run completed six gates and recorded one adversarial wall-clock sample above 8 ms while the four-pool parity test was running concurrently in another nextest process. The same current binary passes the adversarial test in isolation with a 183 µs maximum. The isolated result is the work-ceiling measurement; the grouped failure is retained here rather than misreported as green.

## Static Census

The scoped production route/precompute source contains no `block_on`, `run_blocking`, `spawn_blocking`, Rayon use, private thread spawn, or private `WorkerPool::new`. The sole `WorkerPool::new` and `available_parallelism` matches are below the `#[cfg(test)]` boundary at line 890 and implement the required 1/2/4/default parity gate. The action has zero direct-step/executor matches and positively contains `poll_fill_job`, `enqueue_fill_job`, and one isolated `Effect::SpawnJob`. The plugin registers `FILL_JOB_KIND` to `fill_job`. `semio-framework-job` is a direct Puzzle Cargo dependency and is resolved by `cargo tree -p semio-s-plugin-puzzle --edges normal -i semio-framework-job`.

## Files

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`

## Historical Required-Gate Failure — Superseded

`bun ./📜️script.ts nx run @semio-tech/puzzle-plugin:test-quick --skip-nx-cache` remains red after repeated attempts, including a final frozen exclusive-Cargo window. The target's preliminary `cargo build --tests` completes, but its following `cargo nextest run --no-tests warn --profile fundamental ...` recompiles the same UI/plugin/Puzzle dependency chain and is killed by the wrapper's fixed 15,000 ms total command budget before assertions.

A direct, unbounded invocation of that exact nextest payload was used to finish the compile and diagnose the test boundary. It twice reached the 1,128-test suite and failed the unrelated Puzzle 2D test `editor::puzzle2d::component::tests::repeated_actions_do_not_duplicate_edges`: `setActiveExample` sampled 8,636 µs and then 10,011 µs against its 8 ms ceiling under suite concurrency. The final direct run reported 750 passed, one failed, 377 not run after fail-fast. An immediate exact Nx retry still recompiled and timed out before assertions. No P4 assertion failed in these runs, but the repository-required Nx gate is not green, so `📌️important.md` remains untouched and Phase 4 closure is not claimed.

See `📝️p4-platform-static-runtime-evidence-20260822.txt` and `📝️p4-nx-quick-current-20260822.txt` for the concise command/result ledger.

## Source Repair Implemented

The two repository-gate blockers diagnosed above now have source repairs, but Phase 4 remains open until fresh runtime evidence is recorded.

- Puzzle 2D `setActiveExample` now resets only session config and enqueues a generation-tagged `setActiveExampleStep` continuation. Each continuation dispatch emits at most one semantic document mutation, persists progress in the document/config checkpoint, coalesces the document edit, and stops without re-enqueueing when a newer generation supersedes it.
- Concrete Forest and Nakagin typed snapshots are warmed once during app initialization, outside the interaction step. No example DSL/JSON materialization or whole-document delta remains on the initiating action path.
- The continuation deterministically clears edges, nodes, and compatibility rows before rebuilding manifest metadata, compatibility, catalogs, nodes, and edges in canonical order. Tests drive the production `DispatchAction` chain, assert the one-mutation ceiling, and cover stale-generation supersession.
- `runCargoTestBudgeted` now invokes the exact nextest selection once with `--no-run` under `BUILD_BUDGET_MS`, then starts a separate assertion run under `testLevelBudgetMs(level)`. The cargo-test fallback retains its own build-budgeted `cargo build --tests` prewarm. This prevents nextest compile/prewarm time from consuming the 15-second fundamental assertion budget.
- Source-only checks completed while another phase owned Cargo: Rust formatting over the owned Puzzle 2D/config files, Bun syntax validation of the shared TypeScript wrapper, JSON-schema parsing, and schema optionality assertions all exit zero.

The source repairs above are validated by the final closure packet below.

## Final Closure Evidence

This section supersedes the historical failures retained above. All commands ran against the same final source after the continuation, cache, typed-snapshot, and checkpoint-authority repairs.

| Gate | Final result |
| --- | --- |
| Exact repository quick target | **Pass** — `bun ./📜️script.ts nx run @semio-tech/puzzle-plugin:test-quick --skip-nx-cache`; nextest passed 1,130/1,130 with 0 skipped. The assertion-only phase completed in 12.500 s under the unchanged 15 s fundamental ceiling. The normal wrapper selected 7 assertion threads from the cross-platform `availableParallelism()` reserve formula. |
| Native debug | **Pass** — `cargo check -p semio-s-plugin-puzzle --message-format=short`; exit 0, 41.22 s, warnings only. |
| Native release | **Pass** — `cargo check --release -p semio-s-plugin-puzzle --lib --message-format=short`; exit 0, 1m 02s, warnings only. |
| Browser Wasm | **Pass** — `cargo check -p semio-s-plugin-puzzle --target wasm32-unknown-unknown --message-format=short`; exit 0, 53.64 s, warnings only. |
| WASI Preview 2 | **Pass** — `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2 --message-format=short`; exit 0, 41.51 s, warnings only. |
| Framework transaction/cache focus | **Pass** — `cargo test -p semio-framework-plugin --lib 'app::testkit::transaction_testkit_tests::' --quiet`; 10 passed, 0 failed, including no-foreign preflight bypass, immutable Arc sharing, and incremental amended-history cache extension. |
| Fill route and reveal | **Pass** — poll/enqueue-only isolated worker route, direct render/reveal, and cross-pane synchronization focused tests passed; render/reveal completed in 0.14 s and cross-pane synchronization in 0.13 s. |
| Maximum fill-count continuation | **Pass** — the maximum slider delta materialized through 69 generation-tagged continuations; maximum measured interaction step was 1.979125 ms, below 8 ms, with cancellation/supersession and final prefix-stable document assertions. |
| Puzzle 2D continuation | **Pass** — repeated-action behavior completed in 0.06 s and cancellation/supersession in 1.58 s with a four-mutation maximum continuation slice. |
| Worker-count determinism | **Pass** — worker counts 1, 2, 4, and default 10 each produced the same accepted sequence and byte-identical 2,691-byte commit. |
| Preview ceiling | **Pass** — first substantive preview 245 µs and maximum fill resume 158 µs, both below their 50 ms and 8 ms ceilings. |
| Adversarial whole-fill ceiling | **Pass** — 1,024 placed objects, first substantive preview 587 µs, maximum resume 167 µs, 3,122 transitions. |

The typed Puzzle 3D play snapshot now owns an `Arc<Puzzle3dSnapshot>` and lazily projects JSON only for callers that require the value bridge. Command jobs share immutable snapshot/config/history inputs through `Arc`. Mutations that declare that they cannot emit foreign steps bypass the snapshot/diff/apply preflight. Coalesced `AmendLast` history refresh extends the cached tail incrementally instead of rebuilding the accumulated edit. These changes remove document-size-dependent work from each bounded continuation without weakening the `InteractiveJob` deadline, fuel, generation, checkpoint, or one-bounded-step semantics.

The live fill worker's latest checkpoint is authoritative while an engine exists, so a stale persisted checkpoint cannot rewind worker progress between polls. Persistence still restores the engine when no live checkpoint exists, preserving resumability across reconstruction.

## Final Static And Dependency Evidence

- `cargo tree -p semio-s-plugin-puzzle --edges normal -i semio-framework-job` exits 0 and resolves Puzzle through the shared job runtime.
- The production `fillBuildTick` action positively contains `poll_fill_job`, `enqueue_fill_job`, and isolated `Effect::SpawnJob`, with zero solver-step/executor/blocking matches.
- Production Puzzle 3D precompute, before `#[cfg(test)]`, has zero `WorkerPool::new`, `available_parallelism`, private thread/Rayon, `block_on`, `run_to_completion`, `spawn_blocking`, or `run_blocking` matches.
- `.config/nextest.toml` has no fixed `test-threads = 7`. The repository TypeScript wrapper contains the dynamic `availableParallelism()` formula, passes `--test-threads` to nextest, and retains `--status-level fail --final-status-level fail`.
- `bun --check` passes for the repository TypeScript wrapper.
- `rustfmt --edition 2021 --check` passes for the Puzzle 2D example continuation and Puzzle 3D fill action, fill-count continuation, precompute bridge, and resumable fill builder.
- The scoped changed production/test sources contain no temporary `[DEBUG]` instrumentation.

All four blockers identified by `📓️p4-closure-audit-20260822.md` now have source and runtime evidence: the inline route is removed, fill work is cursor-resumable below 8 ms, 1/2/4/default worker output is deterministic, and substantive preview is below 50 ms. The exact required Nx, native debug/release, browser Wasm, WASI Preview 2, static, dependency, and formatting gates are green.
