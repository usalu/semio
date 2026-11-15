# Package P: the 15-minute daemon stalls of g1-server (2026-11-14 00:49 to 02:21)

## What the evidence says
- The health field `{"kind":"blocked","count":N}` is NOT the loop watchdog. It is the per-path event `blocked ... EACCES` for `.git/objects/pack/pack-afe8dcf2....pack` (now 3.96 GB, read-only 0444), retried about 6 times a second. The watchdog's own record is `loop-blocked` and is not in the live health.
- Stall windows, from sidecar logs and gaps in the server's own journal (events published by the daemon): 00:49-01:04, 01:06-01:21, 01:31-01:47, 01:48-02:04, 02:06-02:21. Each lasts 15 to 16 minutes = `KILL_MS` 900 s of the boot program. Nothing was published in them, then the new daemon published the backlog in one batch (02:03:59 for the 01:48 stall). Journal volume is tiny (about 30 events per 10 minutes): no burst, nothing from ignored build paths is published.
- The kill was real but SILENT: the installed boot program (`~/Library/Application Support/semio/fleet/entry/boot.mjs`, version 3, not the in-module supervisor) killed the ready daemon at lag >= 900 s and, for a ready daemon, wrote no notice, no state attempt, no record. That is why the log has no trace. The stalls began 1-2 minutes after a start in four consecutive daemons and ended with 54054 (02:21), which has run 40+ minutes with lag 200 ms.
- Other machines: attest.json of m01..m07 show no failures (only m04 `refused`/`round`); their journal gaps are idle periods. Nothing indicates a fleet-wide block; the stall was this machine's daemon.

## Cause: NOT proven
Tested and rejected (numbers on the real 273 133-file state, 124 MB checkpoint, copied):
- open of the real state 1.0 s (lag 0.87 s); checkpoint 0.31 s (lag 38 ms); manifest/bucket hex 0 ms.
- closure hash / import closure of the module: 23 ms for 24 sources + 21 closure files.
- burst of 60 000 file events in ignored paths with a live daemon: max lag 195 ms (`.cache/b/...`: 59 ms). Ignored build output does not block.
- a read-only/unreadable large file retry loop in a small tree: 5 % CPU, lag 8 ms.
- A scratch daemon on a clone (APFS clonefile) of the real tree and state, empty store: 100 % CPU for the first minutes (profile: 70 % in `observe` -> `sharedV1` -> `ignores` -> `decide` regexes, 40 % `lstatSync` in `hotChanged`), RSS 0.9-1.26 GB, loop lag stays < 1 s. So a real daemon at this scale is CPU-hot for many minutes after start, but cooperative; it does not block. The real block was not reproduced.
Most likely remaining candidates (untested, need the new capture): a synchronous section inside the first full tiers or the git/view reading after start while 3-4 `git` processes read the new 4 GB pack and the machine ran at load 25-30, i.e. memory-pressure paging (swap used 2.5 GB). Whatever it is, the next occurrence now records itself.

## Real defect found and fixed
`copyHashedV1`: a read-only (0444) source of 64 MiB or more on the store's device is cloned with its mode, then reopened `r+` -> EACCES, forever. The 4 GB git pack was never published and retried 6 times a second (measured 76 retries in 13 s). Fix: `chmod 0600` after the clone. Test: sparse read-only file above the threshold is stored with the correct sha256.

## Survivable now
1. Hard limit: `KILL_MS` default 900 s -> 120 s, only in phase `run` (boot/open stay bounded by `READY_MS`), both in the in-module supervisor and in the installed boot program (`bootVersionV1` 3 -> 4, the daemon installs it on the next healthy start; running boot programs keep the old limit until the service restarts).
2. Before the kill: `.cache/f/boot/loop-blocked-restart.json` (count, pid, lag, phase, reason, declared activity, sample file) and `loop-blocked-<pid>.sample.txt` (macOS `sample <pid> 3`, 10 s cap, newest 2 kept; other platforms: sample null). Bun frames are not symbolicated, so the sample shows native phases only.
3. Activity slot (`activityV1`, 14 names, third double of the shared buffer): the main thread declares idle/follow/publish/hot/sweep/file/attest/sidecars/health/tend/maintain/compact; the beat and the record carry the last one, the notice says `blocked for N s in <activity>`.
4. The boot program says "The daemon was ended (...)" in both languages and the entry state attempt shows the reason. The next daemon reports `loop-blocked-restart` (count, last time) in health.json failures.
5. Sidecars (real-process test, `🧷️sidecars/🧪️tests`): a daemon with a blocked loop leaves its sidecar launcher running and the launcher restarts the role itself on exit 75 while the daemon is blocked; a killed daemon takes the launcher and role down within seconds (parent watch, by design); the next daemon starts the sidecar again. So after the 120 s kill the gap is restart time plus `startAfterSeconds`, not 15 minutes.

Tests added: boot.ts (kill at 120 s default, record, activity, sample, next daemon shows restart, second kill count 2), entry test (installed boot program ends a blocked daemon, record, both-language notice), resilience (activity names equal supervisor list, `loop-blocked-restart` failure), journal (read-only clone), sidecars (real processes). Entry tests adjusted to bootVersion 4/5. All 33 test files pass on the live tree after landing (one unrelated timing failure of `🔁️replica/🧪️tests/🟦️.ts` once on the copy under load, passes on rerun and on live).

## Landed
`1113-landings.md` 03:20, under the lock, one three-way merge (`🔁️replica/🟦️.ts`, one trivial conflict). Live daemon restarted onto the new code (pid 4304, health failures [], attest.json advancing, beat carries `activity`).

## Unverified
- The real cause of the block; the 120 s kill has not fired live (no block since 02:21).
- Boot program v4 on this machine: installed by the daemon on a healthy start, not confirmed in `entry.json`.
- Other machines' boot programs keep 900 s until they restart onto v4.
- The retry storm for persistently blocked paths (no backoff) remains; with the clone fix the 4 GB pack should now publish (about 4 GB copy into the store, disk 94 % full: watch it).
