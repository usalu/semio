# P7b Puzzle 2D Fill Job

## Scope

Puzzle 2D fill now uses the repository InteractiveJob protocol over an immutable Send snapshot. BoardHost and its RefCell/render caches never enter the job. The Puzzle 3D, renderer, stdio, and plugin-host paths were not edited by this packet.

## Runtime design

BoardFillJob persists the following deterministic stages and cursors:

1. PrepareSources: one host or accepted-prefix virtual handle per step.
2. SelectTarget: one seeded weighted rank per source per step.
3. PrepareCandidates: one node-kind/handle-template pair per step.
4. SelectCandidate: one seeded weighted rank per candidate per step.
5. ConstructPreview: one candidate transform and AABB.
6. ScanHostCollision: one immutable host node per step.
7. ScanVirtualCollision: one accepted-prefix virtual node per step.
8. AcceptCandidate: append one accepted node/edge plan and publish a lossless checkpoint.
9. PublishPlanPrefix: publish the accepted prefix and restart the frontier cursors.
10. Complete: expose the final commit candidate.

The state checkpoint contains the snapshot, seeded RNG state, all cursors, rejected target/candidate sets, accepted virtual frontier, placements, search count, generation, and monotonic preview sequence. Restore resumes from the exact next deterministic unit.

Every step checks cancellation, operation identity, generation freshness, fuel, and the absolute deadline before work. The Send assertions cover BoardFillSnapshot and BoardFillJob. Structured previews expose stage, accepted count, target handle, candidate kind, collision cursor and tested ID, rejection reason, and search count.

## Product routing

set-fill-count creates the snapshot/job, stores its operation/generation/checkpoint, performs one instrumented step with one fuel unit and a seven-millisecond deadline, then enqueues brushFillSessionStep through DispatchAction. Each continuation carries the expected generation and a fresh request ID. Stale continuations return without mutation. Accepted checkpoint prefixes are applied in deterministic order; cancellation and faults do not apply uncommitted placements.

Changing utility or clearing fill increments the generation and clears checkpoint/preview state. The fill tool reports live accepted-count progress while a checkpoint is active. The former public brushFillJson and WASM fill-session run-to-completion adapters were removed.

## Tests

The Puzzle brush suite now covers:

- byte-identical same-seed replay;
- monotonic structured preview sequencing;
- checkpoint interruption, restore, and final-result equivalence;
- cancellation and generation supersession without checkpoint mutation;
- byte-identical completion through shared WorkerPool configurations requesting one, two, and four workers;
- a 1,024-node/handle adversarial host with cursor-visible progress and an assertion that no individual step reaches eight milliseconds.

## Verification

- `cargo check -p semio-framework-os-infinite --lib`: passed on the ticket-local target after the geometry RNG repair. This compiles the Send snapshot/job implementation and its production integration.
- `rustfmt --edition 2024 --check` over the owned Puzzle command and brush-test sources: passed. The shared Infinite component parses; whole-file formatting is not claimed because concurrently owned surrounding code differs from standalone rustfmt output.
- The unrelated directed-DAG compile wall is cleared: `cargo test -p semio-framework-os-infinite --lib --quiet` and its release equivalent each passed 309/309, while `cargo check -p semio-framework-os-infinite --target wasm32-unknown-unknown --lib` and the `wasm32-wasip2` equivalent both exited 0. The earlier `semio-framework-os-infinite ... board_fill` command targeted the framework crate, while the behavior suite above is compiled by `semio-s-plugin-puzzle`; its authoritative `cargo test -p semio-s-plugin-puzzle --lib board_fill` rerun remains pending the active Puzzle crate compile gate and is not claimed here.
- `bun nx run @semio-tech/puzzle-plugin:test-quick`: repeatedly rerun on the ticket-local target as upstream repairs landed. The newest owned run advanced through the repaired plugin framework and stdio PDF path, then stopped on exactly two unrelated plugin-host stale awaits at `plugin/🖥️host/🦀️component.rs:4751` and `:5003` (`role.as_str().await`). The plugin-host owner removed both immediately afterward, but the coordinator accepted and froze this P7 source packet and reassigned its owner before a later authoritative Puzzle rerun; therefore this report does not claim a passing Puzzle Nx gate.
- `cargo check -p semio-framework-os-infinite --lib --release`: passed on the ticket-local target after the unrelated SPR history repair.
- `cargo tree -p semio-framework-os-infinite -d --depth 1` and `cargo tree -p semio-s-plugin-puzzle -d --depth 1`: completed successfully as dependency-graph diagnostics; the workspace contains pre-existing duplicate transitive versions, and P7b added only the existing internal Job and Async workspace crates needed by the implementation and tests.
- Puzzle release, wasm, and clippy gates remain pending the plugin/plugin-host and stdio compile repairs.
