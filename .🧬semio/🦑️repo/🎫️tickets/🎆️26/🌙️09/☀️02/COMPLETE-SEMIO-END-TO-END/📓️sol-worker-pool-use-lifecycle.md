# Worker Pool Use Lifecycle

## Outcome

`WorkerPool` now has a cold, linearized lifecycle: `Open { retained_uses }`, `Closing`, and `Stopped`. `acquire_use` increments one fixed counter and returns an `Arc<WorkerPoolUse>` cell; cloning that cell does not mint another use. Shutdown with any retained use returns typed `Busy` and leaves ingress and workers executable. The final cell drop permits the exact later `Open -> Closing -> Stopped` transition. A racing acquire and shutdown linearize under the same lifecycle mutex.

`Database::open_with` acquires its use before the first storage capability probe. The same cell is cloned through a document-mount future and moved into the resulting `ArtifactAuthority`; no second pool lifecycle admission occurs after catalog publication. `Database::shutdown_step` drops the root cell only at terminal acknowledgement, after all mount owners, authorities, the version graph, and emit closure have completed. Later document mounts reject `Closed`. Direct `ArtifactAuthority::spawn` acquires its own cell, while Database uses the crate-private `spawn_with_pool_use` boundary.

The public post-terminal surface is fenced at the same retained-use boundary. Mount, catalog creation, sync hello, and checkpoint reject `Closed` after terminal database acknowledgement even when an unrelated pool use keeps the executor live. Catalog and hello construction distinguish a pre-admission `Closed` result from an already-retained rejection, so cleanup authority is never flattened into a `DbError`. Escaped capability, catalog-read, catalog-bootstrap, catalog-create, and sync-hello operations retain the use cell until their own state retires.

Each registered DB I/O backend now owns one additional `WorkerPoolUse` from registration through exact terminal backend close. Registration refusal transfers the cell with the executor and reserved credit into the lost/rejected retirement owner; neither facade drop nor deferred cleanup permits early pool shutdown. `submit_db_io_task` no longer accepts a caller-selected pool: it resolves and clones the pool from the atomically validated backend kind, slot, and generation before any page admission. Standalone compaction likewise acquires its use before its admission slot.

Hard document-mount scheduler refusals are fail-stop. `Shutdown` and `Poisoned` retain the exact rejected job in a `NonRunnable` mount owner, arm no timer, accept no false wake/resume progress, and surface `DatabaseShutdownBlock::Executor`. Only `Contended` and `Saturated` use the bounded retry timer.

## Neutral and exact laws

The strict JSON Schema fixture contains five pool lifecycle traces and five mounted traces. Its independent Bun/AJV oracle derives use count from use-cell ownership and checks Database, authority, terminal-release, pre-probe refusal, post-terminal API fencing, and hard scheduler markers.

Pure async exact selectors:

1. `native_pool::tests::worker_pool_use_native_busy_keeps_executor_running_until_final_release`
2. `native_pool::tests::worker_pool_use_acquire_and_shutdown_linearize_exactly_once`
3. `wasm_pool::cooperative_tests::worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release`

Database exact selectors are laws 12–14 in `📓️sol-database-document-mount-single-flight.md`.

## Verification

- Registered source gate GREEN: `@semio-tech/framework-async-rs:worker-pool-use-check`, `AJV=1 cases=5 mounted=5 native=1 cooperative=1`; the oracle also requires all four retained Database capability/catalog owners plus ArtifactAuthority handoff and sync-hello use cells.
- Registered DB writer/source gate GREEN: `@semio-tech/framework-os-kernel:wal-writer-authority-check`, `AJV=6 ... backend-pool-use=5`. Its strict backend-pool fixture and independent oracle require exact registered-pool derivation, retained drop/close ownership, pre-admission kind fencing, and compaction use-before-admission.
- Registered document-mount source gate GREEN: `AJV=1 cases=10 waiters=32 owner-futures=1` with three strict driver interleavings.
- `rustfmt --emit stdout` parser checks are GREEN for async, artifact, and engine sources.
- Launch seeds 411.0793/.0794 register the source/native pool-use gates with the shared projection target and 24-hour build budget.
- Five backend-pool native selectors are registered at the tail of the existing `@semio-tech/framework-os-kernel:wal-writer-authority-native-check` group, which now contains 35 selectors. Exact native qualification is pending the root-owned shared cache; no Cargo command was launched for this source boundary.

## Nonclaims

- Raw standalone sync futures other than the retained hello surface are not yet covered here. Registered DB I/O storage backends and standalone compaction are covered by source laws but remain native-unqualified after this source change.
- A hard non-runnable mount retains its exact Database-owned job for diagnosis/recovery; this slice does not fabricate a replacement scheduler or discard its WAL/open owner.
- This lifecycle does not qualify durable-group journal recovery or GIS fixed-three publication.
