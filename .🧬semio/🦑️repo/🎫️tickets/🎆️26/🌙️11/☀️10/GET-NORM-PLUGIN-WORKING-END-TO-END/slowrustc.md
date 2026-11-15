# Slow rustc under `repo run workspace:components -- describe norm`

## Wrapper and scheduler are not the cost
- Unit-cache event log `.cache/b/r/m/u/x/q91aad72f06e.jsonl`: 473 misses in 4546 s. Sum of rustc wall (`miss` ms) = 3887 s (85 % of the wall clock); publish adds only ~0.35 s per unit; lock waits 2.7 s in total; no `slot-wait` events.
- Workspace crates: median 5.8 s, mean 8.4 s, p90 19.8 s; trend worsened from 7.9 s to 11 s mean per unit over 75 min. Current rate 10.5 units/min (not 4), so the remaining ~1500 units are ~2.4 h, not 6 h.

## Replay of the live rustc (args + env read from the live process by `sysctl KERN_PROCARGS2`)
- `semio_framework_onboarding`: 34.9 s in the build; 1.6 s standalone (6.6 s CPU across 10 threads, `-Z threads=0`).
- Same command on an EMPTY jobserver pipe (Cargo `-j1`, `CARGO_BUILD_JOBS=1`, what the build gives every rustc): 4.2-5 s serial (user 4 s).
- 3 tokens: 1.7-1.9 s; 2 tokens 2-3 s; 1 token 3-4 s; no pool 1.7 s; 9 tokens 1.6 s.
- Intermittent stalls of 15-76 s at ~0.05 s CPU: about 15 % of runs on the empty pool, and still ~2/14 with 3 or 9 tokens, at nice 0 and nice 5. Mean 0.85 x 4.5 s + 0.15 x 35 s = 9 s matches the 8 s mean in the log. The stall is therefore the dominant cost and is not a token problem.
- A sample during a stall: rustc main thread in `semaphore_wait`, worker threads idle, one thread in `locator::get_metadata_section` / `CrateMetadata::new` (page faults reading dependency rmeta files). The `diskimages-helper` for `.cache/b/r/fixed.dmg` (pid 36174, nice 5, user-space I/O server of the compile volume) sits in `pwrite` (1305 samples) and `pread` (645) while rustc waits; the main disk shows 3.8-5k tps at 20-100 MB/s, swap 2.4 of 4 GB used, 23 M swapouts since boot.

## Fix delivered (jobserver starvation, the reproducible 2.5x part)
- `jobserverPlanV1` now gives a child of a one-job admission (`SEMIO_COMPILER_PARALLELISM=1`, a reachable or closed pool, POSIX) a private fifo pool of `STARVED_POOL_TOKENS_V1` = 3 tokens, removed on exit (`privatePoolV1`, `release`).
- Files: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🎰️jobserver/🟦️.ts`, `…/🗃️unit-cache/📜️script.ts` (release on close and spawn error), test in `…/🎰️jobserver/🧪️tests/🟦️.ts` (plan swap, no swap for wider admission, no pool and Windows; the `jobserver` crate probe counts exactly 3 tokens in the private pool; release removes it).
- Effect: typical unit 4.5 s to 1.8 s; stall tail unchanged. Expected 10.5 to about 14 units/min.

## Not fixed
The 15-76 s stalls come from I/O on the compile volume (user-space disk-image helper at nice 5 plus a saturated main disk and swap), which the build tooling does not own. Candidates: attach the disk images from a nice-0 parent, reduce concurrent writers to the main disk, free memory.
