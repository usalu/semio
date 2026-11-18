# Why the native component build runs one rustc unit at a time

Evidence: the logs print `[compile-units] component build: 1 of 12 cores planned (...; the free memory carries fewer compile units)` and `the build uses -j1`.

Root cause (driver defect, not the launcher): `🧩️component-pass/🟦️.ts:398` (default `withCapacity`) called `compileUnitsForHostV1` without `history`. `compileUnitMegabytesV1` (`🏗️build/🎟️capacity/🧮️units/🟦️.ts:~55`) then falls back to the authored `megabytesPerUnit` = 2048 MB. Free memory is ~4.8 GB, so `floor((4800 - 2048 reserve) / 2048)` = 1 unit. The measured store (`.cache/b/r/m/components/build-telemetry/units.jsonl`, 1654 units, q75 peak = 416 MB; rustc really sits at 0.3-0.8 GiB) was only read later (line ~414) for the broker, so the plan never saw it. The jobserver tokens, the broker memory share (the `0.7 GiB` budget) and `-j` all follow that 1.

Not the cause: no `repo capacity run`/`--jobs` grant in the environment (`grantedJobs` null), `frozen` is only the broker reacting to the 0.7 GiB budget, `native-launch.ts` envelope.

Fix: `🧩️component-pass/🟦️.ts` reads the unit history once (lazy `unitHistory()`), passes it to `compileUnitsForHostV1` and reuses it for the broker.

Expected next run: floor((4800 - 2048) / 416) = 6 units, capped at the half-of-12-cores share = 6; minimum start 3. Memory-bound, so it varies with free memory (more free memory still caps at 6). Two builds at once share the same free memory. Running processes were not touched; the fix takes effect on the next launch.
