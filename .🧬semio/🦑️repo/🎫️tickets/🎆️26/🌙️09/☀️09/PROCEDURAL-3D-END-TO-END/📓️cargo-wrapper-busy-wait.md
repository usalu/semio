# Cargo rustc wrapper busy-wait

## Findings
1. Jobs: the launcher (`🏃️process/🦀️cargo/🚪️launcher`) already sets `CARGO_BUILD_JOBS=tokens`, `SEMIO_COMPILER_PARALLELISM=tokens` and a jobserver of `tokens`; the cargo-test path (`🧪️testing/📦️executable`) does the same from the capacity lease. Cargo's own fan-out is bounded per run (existing fixtures assert `CARGO_BUILD_JOBS`). The wrapper count seen (107) is the sum over every concurrent Cargo run on the host (all agents), not one run.
2. Root cause: the wrapper does not wait on the jobserver. It waits in `acquireSlotV1` (`⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts`), a host-wide slot pool of `SEMIO_UNIT_CACHE_SLOTS` (= ncpu) that every run shares. Each waiter polled every 25 ms, and each poll listed all tickets and read/`kill -0`-checked each one: O(waiters) file work per poll per waiter (the ~3 % CPU each, R state). `acquireUnitClaimV1` polled at 10-200 ms.
3. Fix: new `watchChangesV1` (fs.watch on the slot and ticket directories, `rename` events only so heartbeat touches do not wake anyone, jittered 500 ms fallback for dead-holder reaping, watcher created before inspecting state so no wake-up is lost). Slot waiting and unit-claim waiting use it. Schema `slot.pollMilliseconds` became `slot.fallbackMilliseconds` (500).

## Tests (unit-cache tests file, worker mode `slot-cpu`)
- "waiters of a full slot pool block on events and use almost no CPU": 12 waiters on a 1-slot pool, CPU over 2.5 s below 2 % each. Fails with the 25 ms poll restored (verified), passes now.
- FIFO and SIGKILL-holder replacement tests still pass.
- Other tests in that file fail only because rustup has no default toolchain in this shell (environment); no end-to-end cargo count was run for that reason.
