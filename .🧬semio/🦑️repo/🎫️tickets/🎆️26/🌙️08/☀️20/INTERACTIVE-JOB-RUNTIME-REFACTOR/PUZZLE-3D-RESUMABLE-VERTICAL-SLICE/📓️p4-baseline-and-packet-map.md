# Phase 4 Baseline and Packet Map

## Scope

Phase 4 is the Puzzle 3D vertical slice of the interactivity-first runtime refactor. It owns the fill planner, collision sampling, preview publication, the fill-tick action, and the Puzzle plugin manifest/test gates. It does not own Puzzle 2D, WFC, FEM, Energy, renderer internals, or `compose`.

## Current execution path

1. `fill-build-tick/🦀️component.rs` runs as an ordinary view action and directly calls `Puzzle3dPrecomputeSession::precompute_step_lane(Fill, 8)`.
2. `⏳️precompute/🦀️component.rs` gives a call a 12 ms soft budget, but always executes at least one whole task.
3. `fill_step_one` rebuilds free-target and weighted-order vectors, scans targets and candidates, constructs transforms, scans every placed body, and can run all 512 Monte-Carlo samples before returning.
4. `solid_overlap_volume` performs broad-phase bounds, pairwise part intersection, then a monolithic seeded sample loop.
5. Progress exposes only accepted plan prefixes/counts. The currently tested candidate, broad-phase set, pair, sample cursor, collisions, and rejection reason are not modeled or published.

The code therefore has a time check around indivisible work rather than bounded work. The existing `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS = 12.0` cannot enforce the global 8 ms ceiling.

## Persistent state that must be made explicit

- operation identity: operation id, base document revision, input generation, preview sequence, deterministic seed;
- fill stage: select target, enumerate candidate, construct transform, query broad phase, test pair, sample batch, accept/reject, publish prefix;
- target order and cursor;
- candidate order and cursor;
- candidate preview and world transform;
- spatial-index query cursor and deterministic ordered candidate ids;
- collision pair cursor;
- sample state: intersection bounds, RNG state, sample cursor, inside count, overlap estimate, early-rejection threshold;
- accepted sequence, appended objects/attractions, rejection counts and last rejection reason;
- checkpoint/replay bytes sufficient to resume without re-enumerating prior decisions.

## Packet ownership

### P4a — Fill job

Own:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/**/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs`
- fill-specific portions of `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/**/✏️editor/⏳️precompute/🦀️component.rs`
- Puzzle Cargo dependency on `semio-framework-job`

Deliver a synchronous `InteractiveJob` with one bounded transition per drive, generation/base-revision validation, lossless checkpoint/commit candidate, deterministic target/candidate sequencing, cancellation, and a batch test driver using the same job.

### P4b — Collision job

Own:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/**/✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`

Replace `solid_overlap_volume` with an owned sample-batched state machine. Preserve the deterministic RNG stream and early rejection. Add an incrementally maintained spatial hash or BVH with stable query ordering; no full placed-body scan in the fill step. Tests must compare batched and reference outcomes over adversarial/coincident/disjoint cases and multiple batch sizes.

### P4c — Preview/action integration

Own:

- Puzzle 3D precompute progress schema and owned pack codecs;
- `fill-build-tick/🦀️component.rs` and its registry/action wiring;
- Puzzle 3D fill tool/window presentation paths and focused tests.

Publish coalesced preview data for target slot, translucent candidate ghost, broad-phase ids, current pair, collision/contact samples, rejection reason, accepted prefix, and search counters. `fillBuildTick` becomes enqueue/poll only; it must not execute solver work on the UI thread. Commit remains lossless and authoritative; previews remain replaceable, revisioned operation state.

## Compiler and dependency prerequisites

- The Puzzle crate currently has no `semio-framework-job` dependency.
- The owned paths contain widespread decorative `async fn` and calls without `.await`; phase-owned pure functions must be de-asynced rather than bridged.
- The Puzzle crate depends on stdio and is not yet reachable through a clean full build while the Phase 1.5 stdio sweep is active.
- Existing external `nalgebra`/`parry3d` use remains until the scoped Phase 9 math/collision replacement; Phase 4 must put it behind the existing owned geometry types and expose no external type.

## Required gates

- `bun nx run @semio-tech/puzzle-plugin:test-quick` in debug and release-equivalent project routing;
- Puzzle native library/test compilation and both repository wasm targets after the stdio wall clears;
- focused adversarial step watchdog: no fill/collision step reaches 8 ms;
- cancellation observed by the next bounded step;
- continuously increasing preview sequence with no stale-generation publication;
- byte-identical accepted placement sequence and final commit at worker counts 1, 2, 4, and host default;
- checkpoint/resume and replay byte-identical to uninterrupted execution;
- static census: no private thread/pool and no interactive `block_on`/`run_blocking` in owned paths.

## Baseline validation status

The source and call-site audit is complete. Runtime gates are intentionally pending until the Phase 1.5 stdio diagnostics reach zero; no Phase 4 functionality is claimed yet.
