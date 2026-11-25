# Compiler temporaries collector

Defect: rustc leaves its `*.rcgu.o` objects in the lane's `-Z temps-dir` (`<lane>/z`), nothing collected them (one lane: 9.6 GiB / 151,537 files) and storage admission was refused while they sat there.

## Design (schema-first)

- Module `unit-cache/🧹️gc/🗑️temporaries/` with schema `🧬️schema/🔣️.json` (policy `graceMilliseconds` 600000, `batchFiles` 256, `scanYieldEntries` 512; en/de messages; Holder/File/Lane/Input/Removal/Vector).
- Pure rule `planTemporariesV1(input{nowMs,lanes})`: lane without running holder -> files older than `nowMs - idleMilliseconds` (schema, 1800000; a compile that holds no marker keeps its fresh files); live lane -> only files with mtime < (oldest holder start - grace); any holder of unknown start (unreadable marker/directory, marker without `startedMs`) -> nothing. Deterministic order lane, path.
- Holders are read read-only (`temporariesHoldersV1`, `processAliveV1`); a dead holder's marker is no holder, a vanished marker was released. `enterLaneV1` now writes `startedMs` into its live marker.
- `collectTemporariesV1(root,{dryRun,needBytes,satisfied,...})`: batches of 256, liveness re-read before each batch and each file re-`lstat`ed against the fresh cutoff right before removal, ENOENT fine, `AbortSignal` and progress per batch, yields between batches, empty sub directories pruned (the `z` directory itself stays), stops when `needBytes()>0` and `satisfied()` after at least one batch; `now` is injectable and read per batch.
- Order: `reclaimUnitStorageV1` temporaries -> incremental (skipped only when temporaries removed files and the need is met, its need reduced by what is gained) -> units; `collectGarbageSingleFlightV1` and `collectRustImageV1` (`repo cache gc`) run the step only when need > 0 (macOS unpacked debuginfo objects stay useful while space is plentiful); a `--dry-run` always reports what would go, counts and bytes en/de; `reclaimUnitStorageV1` always has need. Gates, water marks, admission limits and unit recency rules untouched.
- Lane measurement: `listLanesV1` returns `temporariesBytes` (counted once, inside `bytes`), recorded in `lane.size` (`LaneSize.temporariesBytes`, optional, schema updated).

## Files changed

- `🧹️gc/🗑️temporaries/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🐍️.py`, `🧪️tests/🧫️fixtures/🔣️.json` (new)
- `🧹️gc/🟦️.ts`, `🧹️gc/♻️auto-reclaim/🟦️.ts`, `🧵️lane/🟦️.ts`, `🧬️schema/🔣️.json` (LaneSize), ticket script `measure-temporaries.ts`

## Tests (bundled bun 1.4.2)

- New: 12 pass / 0 fail (vectors + schema + independent Python/sqlite oracle, 300 seeded random states, idle lane, live lane protection, unreadable/start-less holder, re-check + ENOENT, cancellation, early satisfaction, dry run en/de, lane size, reclaim order, single-flight).
- Neighbours: gc root 14, headroom 20, admissions 2, supersession 8, auto-reclaim 13, units 13, lane 2, pool-model 6, pool-claims 7, pool sessions 1, cache-gc suites all green. `gc/🌱️incremental` passed 8/8 on re-run (one run timed out in its flock subprocess test under load, 7/8). `unit-cache/🧪️tests/🟦️.ts` has 9 failures from `rustup could not choose a version of cargo` (environment, would start a compile; not run further).

## Which compile paths hold a `live` marker

The only producer of `-Z temps-dir` is `compilerTemporaryArgumentsV1` in `📜️script.ts` (line 137): directory `SEMIO_UNIT_CACHE_LANE_ROOT/z` when that variable is set, else `<root>/l/<SEMIO_UNIT_CACHE_LANE>/z`.
- `p` lanes: `acquirePooledLaneV1` (pool) calls `enterLaneV1` (marker with pid and `startedMs`, heartbeat) and holds it for the whole compile body (`withCompilerWorkspaceViewV1`), exporting `LANE_ROOT = lane.base`.
- `q` lanes (prefix `policy.pool.product.prefix`, launcher native builds, e.g. q0c9bae): `acquireProductLaneV1` also calls `enterLaneV1` under the lane claim, same body, same `LANE_ROOT`. Both are registered.
- Without a marker: a process that has `SEMIO_UNIT_CACHE_ROOT` but no `SEMIO_UNIT_CACHE_LANE_ROOT` compiles into `<root>/l/<SEMIO_UNIT_CACHE_LANE>/z` (default lane id `d`, reserved and never collected; the fleet bench runner uses lane `t`, which is not reserved but lives in its own temporary root). Nothing registers a marker there; the 30 minute idle grace protects such files while compiling. No path hands out the temps directory of a `p`/`q` lane without its marker.

## Tests (bundled bun 1.4.2)

- New suite: 14 pass / 0 fail (vectors with `nowMs`, schema, Python/sqlite oracle, 300 random states, idle lane, fresh files of an idle lane, live lane, unknown holder, re-check + ENOENT, cancellation, early satisfaction, dry run en/de, lane size, reclaim, single-flight under need and without need, repository collector without need / dry run / under need).
- Neighbours all green: incremental 8, headroom 20, admissions 2, gc root 14, supersession 8, auto-reclaim 13, units 13, lane 2, pool sessions 1, pool-model 6, all cache-gc suites.
- `unit-cache/🧪️tests/🟦️.ts`: 19 pass / 9 fail. All nine fail at the first `cargo` spawn in a scratch workspace (`rustup could not choose a version of cargo`; seven print that text, two fail the exit-code / output assertion that follows it). No lane, live, size or GC assertion is reached. Cause: the toolchain is pinned by `rust-toolchain.toml` at the repo root, a scratch directory has no default. With `RUSTUP_TOOLCHAIN=nightly-2026-07-20 HOME=/Users/ueli` the suite is 28 pass / 0 fail with my changes, including the cleanup test that keeps a lane with a live marker. Unrelated to this change.

## Read-only measurement of the real store (`.cache/b/r/m/u`, dry run, new idle rule)

15 lanes, 1 with a running holder. The step would select **164,414 of 164,785 files = 11,806,915,747 bytes (11.0 GiB)**: q0c9bae 151,537 files 9.6 GiB; qa7b96d 12,106 files 1.4 GiB; p76ca71 420; p6f1ac5 298; p69d765 53. Lane p8f8954 (371 files) is kept entirely: its files are younger than the idle grace. Nothing was removed.
