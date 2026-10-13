# 2026-10-12 cargo provenance capture: profile, correction and fix

Executor: prov-perf (Sonnet). Scratch, harnesses and logs: `.🧬semio/🦑️repo/⚡️cache/play-fleet/prov-perf/` (private `CARGO_TARGET_DIR`/build dir deleted after the proof). All cargo ran through `🚦️cargo-slot.sh` with `SLOTS=2`.

## Correction to the brief: provenance is about 20 s of a 570 s task, not 9.5 minutes

The 9.5 minutes of `@semio-tech/process-plugin:component-release` in build #9 (`coordinator/fresh-9.log`) are the **cargo compile**, not the provenance stage. The nx parent prints a `[process:owner-command] running elapsedMs=N` heartbeat every 10 s, so the log is its own clock:

| marker in `fresh-9.log` | line | clock |
| --- | --- | --- |
| `> nx run @semio-tech/process-plugin:component-release` | 5435766 | 16 583 324 ms |
| `Finished wasm-release profile ... in 8m 46s`, then `[nx-native] staged 1 deliverables` | 5498832 | about 17 143 400 ms |
| last `[cargo-provenance]` line of the task (77 613 lines in this task, `work` 216 629, 4 252 429 757 bytes) | 5576449 | before 17 163 403 ms |
| `> nx run @semio-tech/process-plugin:materialize-release` | 5576453 | 17 163 403 ms |

So staging plus provenance of the process plugin took at most 20 s of the 580 s task (3.5 percent); the remaining 526 s is `Finished ... in 8m 46s`. The "4.5 MB/s" figure divided the byte counter by the whole task. The same workload (216 459 work units, 4.05 GB) finishes the whole task in 60 to 80 s for the process extensions (metal, robotic, wood) whose cargo build is fresh. The control's own physical deadline (`CARGO_PROVENANCE_PHYSICAL_BUDGET_V1.deadlineMs` = 600 000 ms) starts when the owner is constructed, after the compile, so it was never at risk.

What is real: the log. `fresh-9.log` now holds 3 228 814 `[cargo-provenance]` lines = 411 608 992 bytes, 52 percent of the log's bytes, because every filesystem operation of the physical authority is one `onProgress` row and the callback `console.log`ged each one (about 77 000 lines per large plugin, about 4 000 lines per second through nx). That floods the nx output path and the log file, and it is pure noise.

## Profile (dag plugin, `cargo rustc -p semio-hub-dag --lib --crate-type cdylib --target wasm32-wasip2 --profile wasm-release`, real cold build 11.9 min, then replayed)

The real invocation's `compiler-artifact` / `build-script-executed` stream was tee'd (137 units, 15 build scripts, 11 979 interned inputs, 410 compiler resources, receipt 4.58 MB) and replayed through `writeCompletedCargoInvocationProvenanceV1` against the same on-disk state with `Date.now` frozen, so old and new code see identical inputs. Observed: 3 123 694 778 bytes (every file hashed once, then once more by the mandatory recheck), 169 046 work units, about 60 600 progress rows.

| question | measurement | verdict |
| --- | --- | --- |
| Is SHA-256 the cost? | micro-benchmark on a 21.5 MiB rlib: 64 KiB chunks 1.37 GB/s, 1 MiB 1.91, 8 MiB 1.78, `Bun.CryptoHasher` 1.87/1.80, `readFileSync` 1.66, `Bun.file().arrayBuffer()` 1.70, `Bun.file().stream()` 1.70 | no. Hashing is CPU bound at about 1.8 GB/s whatever the chunk size or API; 3.1 GB costs about 1.7 s. Larger chunks and `Bun.CryptoHasher` buy nothing, so neither was changed (the 1 MiB cap stays) |
| Is the log the cost standalone? | old code, 60 594 rows to a file vs a no-op sink: 5.07 to 5.15 s vs 5.07 s; console.log to `/dev/null`/file/pipe/line-prefixer: 2 to 4 microseconds per line | no standalone (60 k lines = 0.2 s); under nx the same rows cost far more (about 4 000 lines/s observed), and they are 52 percent of the build log |
| Per-chunk checkpoint/recheck syscalls? | `checkpoint()` is a counter; progress runs every 128 work units | no |
| Same rlibs re-read for every unit? | the writer's `digests`/`inputs`/`lengths` maps and the owner's `files` map key by path: 8 185 digests + 1 345 reads for 137 units, each file hashed once per pass | no double reads; `read` then `digest` of one file is served from the map |
| Where does the time go? | old wall 5.1 s: first pass `digest` 8 185 calls 1.9 to 2.5 s + `read` 1 345 calls 0.3 s, `recheck` 1.7 to 2.1 s, rest in `kind`/`lstat`/writer | serialised, latency bound: about 19 000 file observations x about 9 awaited descriptor round trips (lstat, lstat, open, fstat, read, fstat, lstat, close) each hop-waiting on a thread pool, with the SHA-256 (about 1.7 s floor) idle during every hop |

## Fix (root cause: strictly serial file observations and one console line per operation)

1. `cargoProvenanceProgressSinkV1(sink, intervalMs = 1000, now)` in `🧾️receipt/🟦️.ts` (exported from native-build): admits the first row and then at most one row per second per invocation. The filter sits beside the sink, never in the authority, so cancellation, deadline and budget checks still run on every row. Used at the three `console.log("[cargo-provenance] ...")` sites: `native-build/🟦️.ts` (`captureCargoArtifacts`), `repo/.../📚️library/🏃️process/🟦️.ts`, and the wgpu compiler `🌐️wasm/📜️script.ts`. Log lines per large plugin: about 77 000 to about 4 (3 to 4 in the dag run).
2. `mapBoundedV1(items, limit, run)` and `CURRENT_PHYSICAL_CONCURRENCY_V1 = 8` in `📁️current/🟦️.ts`: ordered results, at most 8 in flight (at most 8 chunk buffers of 1 MiB = bounded memory), no item starts after the first failure, in-flight calls settle, the first failure is thrown.
3. `CurrentPhysicalOwnerV1.recheck()` rechecks its files through `mapBoundedV1`. Every file is still re-observed completely (same-descriptor hash plus before/after identity), the budget (`admit`), cancellation and deadline checks run per operation exactly as before; the directory roster and absent-input rechecks stay sequential.
4. `writeCompletedCargoInvocationProvenanceV1` warms the writer's own digest maps through `mapBoundedV1` before consuming them sequentially: all artifact filenames of all units once before the unit loop, and each unit's source inputs before its `rows` loop. The sequential loops stay as they were and now hit the maps, so receipt content and order are unchanged. No mtime or size shortcut was added anywhere; every observed input is physically re-observed by `recheck()` before the atomic rename.

## Proof

Throughput, replay of the real dag invocation, idle machine (load 3 to 7), old = byte copy of the previous files, new = landed files:

| | wall | throughput | progress lines |
| --- | --- | --- | --- |
| before (old code, 10 runs) | 5.06 to 6.02 s (median 5.26 s) | about 570 MiB/s | 60 594 |
| after (9 runs: 6 scratch copies, 3 landed files) | 2.79 to 3.50 s (median 2.95 s; landed 3.50, 2.97, 2.95) | about 1 010 MiB/s | 3 to 4 |
| before, 12 busy loops on 10 cores (+ peers, load 40) | 14.4, 18.4 s | 162 to 207 MiB/s | |
| after, same load | 7.9, 11.4 s | 261 to 375 MiB/s | |
| concurrency sweep (scratch) | 2: 3.8 s, 4: 3.1 s, 8: 2.8 s, 16: 2.6 s | | |

The remaining 2.8 s is the 1.7 s SHA-256 floor plus single-thread JS. Loaded builds (build #9 ran the process plugin at 20 s) scale the gain: hop latency, not hashing, is what the load stretches.

Identical receipts:

- Replay, same tee'd messages and disk state: `cmp` of old and new receipts is **byte-identical** (4 575 397 bytes) in all 6 comparisons (3 scratch, 3 landed; `Date.now` frozen because the receipt stores `builtAtMs`/`observedAtMs`).
- Real end-to-end, `buildCargoArtifacts` with the old code and with the new code (warm second runs, real cargo, real staging): decoded with `decodeCargoProvenanceV1` and compared after zeroing the timestamps, normalising the random capture directory, and sorting units/build scripts/compiler resources, all 16 parts are identical (137 units, 11 979 inputs, 410 compiler resources). A raw byte comparison of two real runs differs only because cargo emits the fresh units in a different order each run (checked: first difference is a unit ordering); the replay is the byte-level proof.

Tests (run through nx with `NX_FORCE_REUSE_CACHED_GRAPH=true`, `SEMIO_TEST_ARTIFACT_DIR` set):

- `@semio-tech/framework-process:test-native-artifacts` (native-build + receipt suites): before 32 pass / 0 fail, after **35 pass / 0 fail**. Three new laws in `🧾️receipt/🧪️tests/🟦️.ts` with vectors in `🧾️receipt/🧫️fixtures/🔣️.json` (`progress`, `bounded`): the sink admits exactly the language-neutral vector and agrees with `lodash/throttle` (leading, no trailing) on it; a throttled sink still raises `Current observation cancelled` and yields byte-identical receipts to a no-op sink; `mapBoundedV1` matches `p-map` on order and peak concurrency (peak = limit), starts nothing after a refusal, and refuses non-positive limits.
- `@semio-tech/filesystem-observation:test-typescript`: green (21 physical-file laws).
- `@semio-tech/repo-lib:test-native-owner-command-policy`: 9 pass.
- `@semio-tech/repo-lib:test-runtime-fixture-graph`: 104 pass, 1 fail. The failure (`actual Cargo proc-macro receipts bind current original resources and exact caller units`, `runtime-input-mismatch: Original compiler resource has no current tracked input digest`) is **pre-existing**: it fails identically with the pristine files served through a Bun `onLoad` overlay (unthrottled 226 progress rows prove the overlay was active), so it is not caused by this change and is not fixed here.
- Strict `tsc --noEmit` (`--strict --noUncheckedIndexedAccess`) of the three changed modules: clean. `bun build --no-bundle` of the two other edited call sites: ok.
- Real `buildCargoArtifacts` capture with the landed files: 4 `[cargo-provenance]` lines, receipt semantically identical to the old one.

## Files

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts`
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🧪️tests/🟦️.ts`, `🧾️receipt/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts`

The three runtime modules were developed as scratch copies (imports rewritten to absolute paths, round-trip checked to be byte-identical to the originals), tested and replayed there, then landed one file at a time by `mv` of a verified temp file in the order receipt, current, native-build, so the running build never saw a half-edited module. Build #9 processes that had already imported the old modules keep running the old code; every new component task picks up the new code.

## What was not done, and why

- Chunk size and hasher: measured, no gain (hashing is CPU bound at 1.8 GB/s); the 1 MiB cap in `observation/🟦️.ts`, its schema and test is untouched.
- Hashing on worker threads would cut the 1.7 s floor but adds a thread boundary to the exact-custody path; not worth it at 2.8 s per plugin.
- mtime/size caching of digests: deliberately not added; the recheck is a full second hash of every observed file.
- `directoryEntries` (`opendir` with `bufferSize: 1`, 77 calls, 13 ms) and the per-file lstat redundancy in the owner: not worth touching at this size.
