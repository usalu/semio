# Phase 4 Closure Audit — 2026-08-22

## Decision

**Do not close Phase 4.** The source still executes fill work directly from the `fillBuildTick` view action and several fill transitions remain unbounded. The required Nx gate also has a recorded failure, and the release/wasip2/worker-count/first-preview gates lack successful evidence.

## Verified Passes

| Gate portion | Result | Evidence |
| --- | --- | --- |
| Authoritative Puzzle test binary | Pass | `📝️p4-full-j4-final-4.txt` records 1,124 passed, 0 failed in 15.53 s. It includes the 3D fill checkpoint/resume, drive-batch determinism, cancellation, typed preview, collision checkpoint/cancellation/spatial-index, and sample-watchdog tests. |
| Suggestion interaction slice | Pass | `📝️p4-suggestion-timing-focused-final-3.txt` records 10 passed, 0 failed. |
| Focused fill action watchdog | Pass, narrow only | `📝️p4-fill-watchdog-final.txt` records 1 passed. The source test measures only the `fill_build_tick_only_plans_available_slider_range` scenario. |
| Persistent fill/collision structures | Pass | `⏳️precompute/🪣️fill/🦀️component.rs` has `FillBuilder: InteractiveJob`, serialized `FillJobCheckpoint`, operation/generation checks, and a checkpoint/commit outcome; `⏳️precompute/📐️geometry/🦀️component.rs` has persistent collision stages, RNG/cursors, cancellation/yield checks, sample batches, and ordered spatial-index queries. |
| Scoped forbidden-call census | Pass | A fresh read-only census found no `block_on`, `run_blocking`, private pool/thread creation, rayon, or `spawn_blocking` in the 3D precompute subtree. |
| wasm32-unknown check | Pass | `📝️p4-wasm32-unknown-final-2.txt` ends with `Finished dev profile` after checking `semio-s-plugin-puzzle`. |

## Closure Blockers

1. **P4c worker-route contract fails in the current source.** `🎮️commands/🪣️fill-build-tick/🦀️component.rs:14-20` directly calls `precompute_step_lane(PrecomputeLane::Fill, 1)` from a view action. The required route is poll/enqueue-only followed by `Effect::SpawnJob`; `enqueue_fill_job` and `poll_fill_job` exist in `⏳️precompute/🦀️component.rs:755-772`, and the plugin registers the job, but no use of `enqueue_fill_job` or `Effect::SpawnJob` reaches the action. This contradicts both the Phase 4 baseline and `📓️p4c-preview-action-integration.md`.

2. **The hard 8 ms fill bound is neither implemented nor established.** `FillBuilder::step` calls whole-stage functions without an item cursor or yield boundary. In particular `prepare_targets` enumerates, partitions, orders, collects, and rotates the complete target set (`🪣️fill/🦀️component.rs:344-357`); `prepare_candidates` computes/orders full candidate sets (`:371-386`); `query_broad_phase` materializes the full result (`:441-460`); and `accept_candidate` applies/clones a fixture and updates structures (`:517-557`). The only FillBuilder timing test is an empty-fill transition (`:787-795`); the collision test covers only 32 one-sample steps. This does not meet the plan's adversarial whole-fill-step requirement.

3. **Worker-count parity is missing for Puzzle 3D.** The 3D test at `⏳️precompute/🦀️component.rs:1388-1393` compares drive budgets `[1, 2, 4, 8]`, not runs on worker counts `1, 2, 4, default`; no 3D worker-pool test was found. The similarly named worker-count test is under Puzzle 2D and cannot discharge P4.

4. **First substantive preview within 50 ms has no test/evidence.** No 3D fill test or report measures it.

5. **The required Nx quick gate is red.** `📝️p4-nx-test-quick-final-2.txt` records the command exceeding its 15,000 ms budget and exiting non-zero.

6. **Release and wasip2 evidence is incomplete.** No successful P4 native-release result is present. `📝️p4-wasm32-wasip2-final.txt` stops while compiling `semio-s-plugin-puzzle` and contains no `Finished` marker or exit status.

## Required Follow-up

- Rewire `fillBuildTick` to poll the latest observation and emit a single isolated `SpawnJob` using `enqueue_fill_job`; only `fill_job` may drive the fill state machine.
- Split target enumeration/order, candidate enumeration/order, broad-phase query/materialization, and acceptance/application into cursor-resumable, fuel/deadline-checked slices. Add an adversarial end-to-end fill watchdog test.
- Add 3D tests for worker counts 1/2/4/default, accepted placement sequence/final commit byte identity, and first substantive preview below 50 ms.
- Re-run and retain green Nx quick, native debug/release, wasm32-unknown, and wasm32-wasip2 evidence after the source route is corrected.

The ticket remains correctly open; no repo-ticket lifecycle operation was attempted because the repository MCP is unavailable.
