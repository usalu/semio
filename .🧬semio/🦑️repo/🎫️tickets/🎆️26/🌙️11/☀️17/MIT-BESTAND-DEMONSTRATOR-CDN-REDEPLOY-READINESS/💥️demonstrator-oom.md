# Demonstrator Component Final Killed With Exit 137

## Killer

macOS jetsam (host memory exhaustion), not the driver.

- No driver, broker or wrapper code sends SIGKILL to rustc. `frozen` only stops new admissions (`🎟️admission/🟦️.ts:131`), the rustc wrapper only forwards signals (`unit-cache/📜️script.ts:58`), and owned-execution kills write a kill-record line; the log has none.
- `runs/w22ewhn/r/timeline.jsonl`: the unit ran alone (`rustcRunning` 1, `admissionWaiting` 0). Swap climbed to 10.7 GB, available memory fell to 1.1 GB, then swap dropped 10.7 to 8.4 to 4.2 to 1.7 GB within 10 s of rustc ending (about 9 GB released).
- `log show` at 15:58:53, the second the unit died: launchd reports `OS_REASON_JETSAM` for homed, siriinferenced and mediaanalysisd, a system-wide swap exhaustion; rustc was the largest process.
- Peak resident was 3.76 GB at t=74 s (front-end phase, 12 front-end threads); the rest of its footprint sat in swap. The other graph workers (about 4 GB) plus desktop apps left no room.

## Why policy cannot fix it

A sole unit is already admitted regardless of its prediction (`🎟️admission/🟦️.ts:121`). Waiting for 6.95 GB to be free would only stall behind processes that cannot be killed. The footprint must shrink.

## Fix

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs`: new `frontend_tokens`, used at the call of `bounded_argfiles`. A `--crate-type cdylib` final (a plugin component: one entry point, all work is codegen and LTO of its closure) gets one front-end thread instead of the admitted parallelism. The measured doc of `bounded_threads` states 1 thread 2.9 GB versus 4 threads 5.4 GB and 10 threads 5.2 GB; the final's CPU was about 100 % (one core) in the front-end phase, so no speed is lost. The rewrite happens at process start only: Cargo fingerprints, unit cache keys and the artifact are unchanged (the existing law test proves thread count does not change output). No profile change, so no cache invalidation.

Tests: unit test `grants_one_front_end_thread_to_a_cdylib_final` (passes, run standalone), four new corpus cases in `🧪️tests/🧵️frontend-threads/🔣️.json` with the Python oracle `🐍️.py` updated and agreeing on all 15 cases.

## Caveat

The scheduler binary is rebuilt by the next driver start. Expected peak roughly 2.5 to 3 GB lower during the front end; the LTO phase is unchanged and unmeasured. If it still dies, the next lever is `release-wasm` `lto` in `🧬️schema/🔣️.json` of `📚️library/🏗️build/🎚️mode/🦀️profile`, which re-keys all 1438 units.
