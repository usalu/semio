# Hold-and-wait between the product lane and shared-store leases (1126)

## Root cause
1. `withRepositoryCargoArtifactPlanV1` (native-build) enters `withRepositoryCompilerStoreV1` first. That scope takes the shared leases (native-fixed-image, rustup:home, rustup:generation-shard, cargo:registry, cargo:configuration-view, store/build leases and more; about 18 per process in the registry, about 45 claim files across processes).
2. Inside that scope `withRepositoryCompilerStoreProductV1` calls `acquireProductLaneV1`, which polls the exclusive lane claim (`lane-q<digest>.lock`) in an unbounded loop.
3. The lane claim is a plain claim file, not a lease. The lease order policy (hold-and-wait, strictWaits, wait-for cycle detection) never sees it, so the wait holds all leases for hours, is unbounded, and prints the holder once and never again. The `repo cargo --store=shared ... test` wrapper starved because it needs exclusive access to resources the queued builds hold shared; its only output was the generic "remains active on ..." activity line, which names no holder.
4. The claims sqlite files (`semio_resource_lease`) are closed lock carriers: one row with the resource name, a validated 2-page layout and exactly one sqlite_schema row. They cannot carry per-holder rows (shared holders are many). The holder record is the per-process lease registry (pid, session, command, since per hold) plus, for lanes, the claim file.

## Fix
1. One lock order, lane first, scopes second: `acquireProductLaneV1` gets `onHeld: "refuse"` and throws `ProductLaneBusyErrorV1`; `retryWhileProductLaneBusyV1` (used by `withRepositoryCargoArtifactPlanV1`) lets the scope unwind (releasing every lease), waits with `awaitProductLaneFreeV1` holding nothing, then re-enters. Inside an outer retained store (`compilerStoreRetainedV1`) unwinding releases nothing, so that path keeps waiting in place but is now bounded and reported.
2. Bounded, reported waits: `pool.product.waitBoundMilliseconds` (1800000) refuses with `ProductLaneWaitBoundErrorV1` (EN+DE, names lane, holders, bound); `waitNoticeMilliseconds` (60000) repeats "waiting N s for lane X held by ..." (messages `productLaneWaiting`, `productLaneBound`).
3. Holder identity: lane claim files record `command`; `laneHoldersV1` prints pid, agent, session, command, age. Lease registry `holderOf` returns the holder session; lease waits print a periodic EN+DE notice (`holdAndWait.noticeMilliseconds`, 30000) naming process, command, session and held seconds (`formatLeaseWaitNoticeV1`); blocker text in violations includes the session.

## Tests (all pass)
- New `lane/pool/product/tests/hold-and-wait` (fixture JSON, holder child process, Python oracle of the scope depth): the waiter in a second process holds nothing while queuing; the legacy in-scope wait keeps its scope open (the reproduction); refusal names the holder; bound refusal is bilingual.
- Lease order test: wait notice format plus authored policy still valid against the schema.
- Regression: lease order (80), lease waits (15), status, compiler lane and compiler tests, existing product lane tests (9).

## Files
- cargo/unit-cache/lock, lane/pool (laneHoldersV1), lane/pool/product, schema JSON (unit-cache)
- storage/enforcement/compiler (index, lane), artifacts/native-build
- leases/order (index, registry, contract, schema, policy.json)
- new tests under lane/pool/product/tests/hold-and-wait

## Not done
- Retained-store sessions cannot yield; a true fix needs the product key before store entry (key uses the toolchain digest).
- No live release build was run.

## Follow-up (coordinator)
1. The queue wait that holds nothing is now unbounded; it only repeats the holder notice (`productLaneQueued`). `waitBoundMilliseconds` (30 min) applies only to the in-scope wait inside a retained store, where leases are held (`productLaneWaiting`).
2. Retained stores: the product key no longer depends on the store. `compilerProductToolchainDigestV1(workspace)` hashes the pinned `rust-toolchain.toml` plus platform and arch (test with an independent hash). `awaitCompilerProductLaneV1` waits for the lane before any store, image, toolchain or lease is opened; it runs in `withRepositoryCargoArtifactPlanV1` (non-retained) and in the artifact issuer before `withRetainedRepositoryCompilerStoreV1`.
   Residual: lane-first is a gate, not an atomic take. The lane directory lives in the image behind `native-fixed-image` and the toolchain generation is entered before the image, so taking the lane itself before the store needs a split of `withRepositoryCompilerStoreOwnedV1` (image open as medium, then lane, then the other leases). A caller that loses the race inside a retained store still waits in place, bounded and reported.
