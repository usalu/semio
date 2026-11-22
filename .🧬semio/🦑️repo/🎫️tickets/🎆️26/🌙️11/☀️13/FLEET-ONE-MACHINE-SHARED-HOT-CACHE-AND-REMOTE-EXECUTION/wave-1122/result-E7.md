# Result E7: memory thresholds as shares of RAM, swap signal, cargo reserve, fseventsd churn

Landing records: `../landings.md` ("Wave 1122 E7" and "E7 (addendum)"). Scripts: `../🧰️scripts/E7/`. `HC` = `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity`, `B` = build library.

## 1. Swap signal (the coordinator carries 20 of 21.5 GB swap)
Cause: the hard stop `swap` was `swapUsed > 25% of RAM AND swap traffic >= 1 MB/s` (absolute), `swap-activity` was 64 MB/s absolute. On a host that parks cold pages in swap for days, any traffic above 1 MB/s stopped every admission, and a hard stop has no override before the G1 deadline. This also feeds `headroom:false` of the load report (E5: `no-headroom (swap)`).
Now (`hostSwapStopV1`, HC decision): `swap` needs all of: occupied swap above `ceilingBasisPoints` (25%), traffic above `activeBasisPointsPerSecond` (0.6 bp of RAM per s, 1 MB/s at 16 GB) and memory the host can give out below `shortBasisPoints` (2500 = 25% of RAM). `swap-activity` is `outBasisPointsPerSecond` 40 (64 MB/s at 16 GB, 131 MB/s at 32 GB). Occupied swap alone never stops. Memory pressure level (macOS `kern.memorystatus_vm_pressure_level`, Linux PSI, compressor share, available share) and Windows commit/available were already in `pressure`; unchanged.
Live: this host has pressure normal, `memory_pressure` free 82%, swap traffic 0 MB/s in a 10 s sample; coordinator case in test: 32 GB, swap 20352 MB used, 15227 MB available, 30 MB/s traffic -> admit; same with 4000 MB available -> wait `swap`; 400 MB/s -> `swap-activity`.
`owner-pool` (graph farm) had its own absolute copy of the thresholds: converted to the same shares (E2 held the lock, done at 01:25).

## 2. Absolute memory thresholds as fractions
- Swap thresholds: shares (above). Schema `Policy.swap`, `B/🔣️policy.json`, texts en/de updated.
- Cold request sizes (`sizeHostRequestV1(..., host)`, callers `build/🌡️host-capacity` and `tools/🏃️run`): fixed and per-job megabytes clamped to `estimates.maximumShareBasisPoints` (50%) of the room above the floor, never below an operation's own `minimumMegabytes`; the tool still gets its declared per-job size as `envelopePerJobMegabytes`. Before, an end-to-end job on 16 GB asked 7680 MB (and 4 jobs the whole machine), on 8 GB 5632+ of 6656. Learned estimates already used that share.
- Cargo admission reserve: `reserveBytes` 6 GiB became `maximumReserveBytes` 6 GiB bound plus `reserveBasisPoints` 2500 (`admissionReserveBytesV1`: 8 GB host 2 GiB, 16 GB 4 GiB, 32 and 64 GB 6 GiB); share math, broker option `reserveBytes`, callers component-pass and cargo launcher.
- Kept absolute as bounds (documented in the oracle): floor minimum 1536 / maximum 4096 MB, per-kind and fixed MB of operations (tool sizes), queue 256.
- Proof: new `HC/🧪️tests/📐️scaling` (4 tests, Python oracle `🐍️oracle/🐍️.py`, `--check` equal) over 8, 16, 32, 64 GB: floors, caps, 8 operations x 4 hosts request sizes, 28 swap vectors, and a decision test for the coordinator case. Admission memory test uses an SQLite oracle with the fractional reserve and asserts 2/4/6/6 GiB reserves.
Tests passed: HC main 28/0, upgrade 10, followup 11, sampling 12, phases 14, lend 7, blockers 5, unmanaged 4, gate 33; B host-capacity all files except runner-bound `sealed-read` and `arguments` (need the managed python binding); agent-sessions adapters 7; admission memory test pass (3 others need the repo runner, replay 4/0); owner-pool see landings; G2 claim-progress 6, W1 source-discovery 5. Stored fixtures: 10 corpus rows of `HC/🧪️tests/🧫️fixtures` regenerated from the SQLite oracle because hosts of 65536 MB now use the share threshold.

## 3. fseventsd churn (fseventsd 9.6 GB RSS)
Method: `find -mmin` counts per tree (no fs_usage, no sudo), mount and `.fseventsd/no_log` checks.
- All 15 image volumes under `.cache/b/**` (including the native partitions) already carry `.fseventsd/no_log` and `.metadata_never_index`. Nothing to fix there.
- The churn is on the Data volume. Per 5 to 10 minutes: `.cache/f/cache-server` (the fleet remote-cache server store, 2 GB, 197k files) **6103 events per 5 min**, two new paths per entry (`v1/main/XX/<digest>` and `v1/.meta/main/<digest>`) plus a unique `.tmp/<pid>/<uuid>` file; `.cache/f/remote-execution` 610; `.git` 6560 per 10 min (bursts of about 9700 loose objects at once, 105k loose objects, `gc.auto=0`, 82 MB index rewritten); `/private/var/folders/.../T` about 5700 per 3 min (tests run with bare `bun test` use `tmpdir()` on the Data volume: 68 leftover `fl-*` dirs of the unit-cache fleet test with about 220 files each, `semio-read-verdicts-501`, `chaos`, `ready-*`); `/Users/Shared/semio-store` about 740 new blobs per 10 min (content addressed, inherent); the rest of the worktree about 460 per 10 min.
- No repo watcher or sqlite journal of ours is the source. The replica daemon has no filesystem watch of the tree; `git` stage work uses `.cache/f/tmp/index-observe-<token>` (reflink copy of the 82 MB index, removed at once).
- Not fixed in code (reason): moving the cache-server store onto a no_log image needs a new storage partition (the 24 GiB `products` image is mounted, empty and no_log, but writing there outside the storage API is unsafe because an unmounted image would hide the data and E4 owns the server); `.git` loose objects need `git gc` (forbidden here). See `findings-E7.md` for the concrete proposals.
- Mitigation available now without code: lanes should run bare `bun test` with `TMPDIR` on a no_log volume (`.cache/b/o/m/tmp`, 2 GiB, empty); `repo test` already routes scratch into the diagnostics image. Not applied: cargo scratch tests refuse a scratch under a checkout.

## Not verified
- No run on real 16 GB hardware; the 16 GB numbers are model results (oracle) and the live 32 GB sample.
- Cargo broker end to end (`🔓️tokens`, "real Cargo through the wrapper") could not run here (cargo manifest parse error in the scratch, runner-bound tests).
- The effect of any change on fseventsd RSS is not measured (restart of fseventsd needs sudo).
- Typecheck not run (bun test and `bun build --no-bundle` only).

## Other lanes must know
- `B/🔣️policy.json` was reverted three times by a stale whole-file write; anyone editing it must re-read and use anchored edits. The swap keys are now `activeBasisPointsPerSecond`, `outBasisPointsPerSecond`, `shortBasisPoints`; any consumer or fixture with the old MB keys fails `readHostCapacityPolicyV1`.
- E5: `no-headroom (swap)` is produced by `hostSwapStopV1` through the load report `headroom`; it now clears on this coordinator while memory is plentiful.

# Round 2

## 1. Fleet cache index on a declared no_log partition area (landed 01:51)
- Ledger (`B/../⚡️caching/💾️storage/🧾️ledger`): new nested area `fleet-cache-index` in the `products` image (`.cache/b/p/m/fleet/cache-server`, no_log and never_index like all images), budget 1000 bp of the partition (2.4 GiB, a host share: it scales with the image). The partition's soft-reclaim rule left room for 100 bp only, so `semio-actions` went from 2000 to 1000 bp (Bazel's 4000 bp is tied to its own schema by a law test and was not touched). `cacheAreaHomeV1` resolves directory and budget and, on macOS, refuses while the image is not attached (a write below an unattached mount point would be hidden when the image attaches).
- Server: `remoteCache.server.fleetDirectory` was replaced by `fleetArea: "fleet-cache-index"` (schema `🏃️execution/🧬️schema/🌐️remote-cache.json`, policy, sidecar `🗄️cache/🟦️.ts`): directory and `maximumBytes = min(policy 2.5 GiB, area budget)` come from the ledger. No migration: the server starts empty on the new home.
- Live: the successor started 01:51 on `.cache/b/p/m/fleet/cache-server` (1000 to 1500 files per minute there), the old server wrote 0 files into `.cache/f/cache-server` at 01:54; the restart was the normal supervised one (old server kept serving until the new one bound the shared port). Cache facts still named the old directory at 01:53 (published every 60 s). Tests: ledger 13, law 8, model 3, needs 8, sqlite 10, sidecar 4, remote-cache 22 pass.
- Retirement of the old directory: the ledger cannot retire `f/cache-server` because it lies inside the control area `f` (`fleet-state`; a retirement there is refused as overlapping homes). It holds 2 GB and nothing writes it any more. Removing it is a one-off by a human or the coordinator (`rm -r .cache/f/cache-server`, outside my allowed paths); until then it only costs disk.
- Coordination with E5: no extra restart beyond the single code-change restart; the successor mode (`SEMIO_SIDECAR_SUCCESSOR`, `reusePort`) was used as designed.

## 2. Other high-rate writers (measured by mtime sampling, no fs_usage)
- `.cache/f/remote-execution/work/<worker>/<task>`: 1186 new paths per 2 min on this host (private action roots of offloaded units), 1.9 GB. Same cure prepared: nested area `remote-execution-worker` in the `dependencies` image (1000 bp, 3.6 GiB) and `👷️agent/🟦️.ts` taking its directory from `cacheAreaHomeV1` (scripts `../🧰️scripts/E7/patch-worker-home.py`, tested only by reading). NOT landed: `caching-storage` lock was held by E13 for 12 minutes; run the script when it is free (re-read the files; it asserts every anchor).
- Heartbeats (fixed paths, about 15 events/s together): `boot/beat.json` 1 Hz, `graph-server/read-watch` 1 Hz, sidecar status files and owner-pool lease 0.5 Hz. Left alone: they belong to the replica watchdog and sidecars with liveness thresholds of other lanes (`BEAT_MS` could go to 3000 against a 20 s grace).
- Test scratch from bare `bun test` under `/private/var/folders/.../T`: 7193 entries, `fl-*` fleet unit-cache test dirs not cleaned after a killed run, `semio-read-verdicts-501` (graph read verdict files, 543 events per 3 min). `repo test` already uses the diagnostics image. Not changed (graph read is E1/E2 code).
- `.git` loose objects: see findings (needs `git gc`, forbidden here).

## fseventsd before and after
| time | probe `hostDeliversEventsV1` (3 s x 6) | fseventsd RSS | CPU |
|---|---|---|---|
| 01:35 before | 1 of 6 delivered | 8.6 GB | 113 % |
| 01:54 after cache index moved (3 min) | 0 of 6 | 9.4 GB | 124 % |
| 01:55 | 0 of 6 | 9.4 GB | 103 % |
| 02:01 | not re-run | 8.1 GB | 100 % |
Event delivery has NOT recovered yet and fseventsd memory did not drop: it must first drain its backlog (it holds events of 8 days of uptime), and it cannot be restarted without sudo. The source is reduced (about 20 events per second less on the Data volume since 01:54), the effect will show only after the daemon catches up. Re-measure in 30 to 60 minutes with the same probe (`scratchpad/E7/probe.ts` calls the E13 helper).

## 3. Tests that need the runner or the managed Python binding
- `mcp__repo__test` takes a whole project and the whole graph is not available on this machine (`show projects` refuses: no sealed generation; `repo:test` dry run died after 8 min with an unrelated resident-service failure `Unclosed source input alternatives` in `🏗️build/📥️inputs/🟨️.mjs:47` during graph slice reading). It cannot run one file.
- Workaround proven: bind the managed interpreter by hand (`SEMIO_TEST_PYTHON_BINARY=.cache/b/t/m/python/interpreters/cpython-3.12.13-macos-aarch64-none/bin/python3.12`): `HC` B `sealed-read` 1 pass and `arguments` 6 pass.
- The admission tests that say "Original parent Script process invocation required" need the runner's capability objects and cannot be run by hand; nothing in my area fails for another reason (admission memory test and replay pass). Three failures of the native law test (`machine-state` schema `to`, `source-freshness` `node_modules`, `cache-gc` `none`, stray `framework/.cache/`) are not caused by this lane.

## 4. Who overwrites `B/🔣️policy.json`
Not proven; evidence and candidates in `findings-E7.md` (no cross-machine conflict in the replica `lost` folder, so a local process; stable since 01:14; the schema now fails loudly on the old keys).

## Not verified (round 2)
Whether event delivery recovers; the cache hit rate after the empty start; the worker home change (not landed).

# Round 3

## 1. Worker action roots on a declared partition area (landed 02:30)
Ledger area `remote-execution-worker` (nested in `dependencies`, `remote-execution`, 1000 bp = 3.6 GiB, owner remote-execution) and `🛰️sidecar/👷️agent/🟦️.ts` takes its worker directory from `cacheAreaHomeV1` (anchored edits, applied after a fresh read; the first version of the German purpose text exceeded the 240 character schema bound and made the ledger unloadable for about 1 minute until I shortened it, the live fleet reads this ledger: lesson, run the ledger test before releasing, not after). Tests: sidecar 4, cache-gc 19, ledger 15 pass. NOT yet live: the agent sidecar (pid 84634 from 01:19) had not restarted onto the new code at 02:45 (supervised code-change restart waits for a quiet period that other landings keep resetting); `.cache/f/remote-execution` (about 500 events per 3 min and 1.9 GB) is still the active worker home until then. Nothing to do, it switches by itself; verify with `ls .cache/b/d/m/remote-execution`.

## 2. Carved-out retirements in a control area (zero-touch retirement) and the old cache dir
- Schema: a ledger retirement may carry `within` (a control area id). `readCacheLedgerV1` accepts it only strictly inside a control area of the same root, carves it out of the area's overlap check (the area keeps owning the rest) and refuses overlapping carved-out retirements; `classifyCachePathV1` answers `retired` for such paths before the area. `repo cache gc` needed no change: it already collects every `ledger.retirements` row with the quiet custody (a unit is removed only when nothing touched it for `quietMs`, here 15 min; the new server never opens the old path).
- Policy: `native f/cache-server within fleet-state quietMs 900000`.
- Test: ledger "a retired path inside a control area is carved out of it" (accepts, classifies `retired`, `f/boot/beat.json` stays area `fleet-state`; refuses without `within`, `within` naming a partition, the whole area, and a duplicate).
- Executed: `repo cache gc --area=retired --dry-run` listed exactly `f/cache-server/{fleet 97.65 MiB, mirror 1.03 GiB, v1 794.52 MiB}`, 0 kept; then `repo cache gc --area=retired` (02:31 to 02:33): "Removed 3 entries, 1.91 GiB freed, 0 kept", `.cache/f/cache-server` is gone. Cache facts now publish `.cache/b/p/m/fleet/cache-server` and the new store holds about 265 files per minute. The host volume was 96.1% used before (dry-run line), so the 1.9 GB also matters for disk.

## 3. Event delivery and fseventsd trend
| time | probe 3 s x 6 | fseventsd RSS | CPU |
|---|---|---|---|
| 01:35 (before) | 1 of 6 | 8.6 GB | 113 % |
| 01:54 (cache index moved) | 0 of 6 | 9.4 GB | 124 % |
| 02:29 | 0 of 6 | 7.9 GB | 122 % |
| 02:45 (old dir removed) | 0 of 6 | 7.5 GB | 100 to 158 % |
Events on the `.cache` Data-volume paths per 3 minutes: about 6100 per 5 min at 01:25 (cache server alone) down to about 500 (worker roots only) at 02:45; but a new source appeared at 02:29: `.cache/f/source-analyses/XX/<digest>.json.br` (graph source-analysis cache, 4862 events per 3 min, 8000 files, 22 MB) written during the graph slice reads of the workers' warm-up. RSS is trending down slowly (8.6 to 7.5 GB), CPU did not drop, delivery has not recovered after one hour. Honest reading: fseventsd is still saturated by the remaining Data-volume sources (`.git` bursts, `/var/folders` test scratch, the graph slice caches, heartbeats, the worker roots until it restarts) and by its own backlog; the two moves removed the largest single steady source, which is not enough alone. Next candidates in order: worker roots (lands by itself), `f/source-analyses` into the `graph` partition (E1/E2 code), `git gc`, test scratch to the diagnostics image.

## Not verified (round 3)
That the worker agent switches home (not restarted at 02:45); whether delivery recovers once the worker roots and graph caches also leave the Data volume; Windows and Linux behaviour of `cacheAreaHomeV1` (no attach check there by design, directories).
