# Fresh Collision Follow-up

Six fresh findings were repaired by explicit moves and exact reference patches:

- Block 2D IO fixtures now use `⬅️hexagonal-cut-concrete-forest-left.json` and `➡️hexagonal-cut-concrete-forest-right.json`.
- Block 5D IO fixtures now use `⬅️hexagonal-cut-concrete-forest-left.json` and `🏢️nakagin-capsule.json`. The Rust include literals use those names; payloads were not rewritten.
- OS directory fixture `⚡️events.json` is now distinct from `🧾️command-receipt-v1.json`. Its TypeScript fixture consumer and Rust include are updated. Final path resolution retains the original two-parent traversal.
- Repo-library test `📍️draw-destination-observation` is distinct from `🎯️cargo-target-discovery-skip`. Its Nx runner source, registration source/schema, local fixture/schema reads, and actual `🖍️draw-source-scenario/🔣️.json` reference now resolve. Historical launch names and semantic contract IDs remain unchanged.

`bun nx run @semio-tech/framework-os:test-quick` ran 249 tests: 248 passed and one backbone-worker unreachable-hub queue assertion failed (`🧵️backbone-worker.ts:3196`, expected a value greater than zero). The renamed directory event fixture was consumed successfully; no unrelated worker behavior was changed.

`bun nx run @semio-tech/repo-lib:test-draw-destination-observation` resolves its source and fixtures but stops at the input-authority fence. Read-only comparison confirmed that catalog and authored-source byte hashes differ from pinned SHA256 values, while the projection contract ID and mapping digest still agree. Those authority hashes were not weakened or replaced merely to pass the test. This remaining authority mismatch is reported to the coordinator.

The attempted Block fixture-verifier selectors `s.block.2d` and `s.block.5d` select zero fixture records and therefore are not used as positive fixture-verification evidence.

Read-only audits confirm all eight structural counts zero for Block (3,419 governed), OS directory fixtures (15 governed), and Repo-library tests (364 governed).
