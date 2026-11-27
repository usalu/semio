# fseventsd memory growth (pid 103, 7.7 GB)

## Root cause
`fseventsd` leaks about 12 KiB for every FSEvents client stream that is created, and never frees it after the stream is closed. Measured on this Mac with a throwaway bun script (watch and close one directory in a loop):

| streams created | fseventsd footprint change |
| --- | --- |
| 408 | +4 to 5 MB |
| 1200 | +15 MB |

Mount and unmount cycles of an APFS image (6 cycles) and a restart of the replica daemon (`launchctl kickstart -k gui/501/tech.semio.fleet.replica`) changed nothing. The leak is per stream creation only, not per client process and not per volume.

The unified log (`log show --predicate 'process == "fseventsd"'`) shows 12969 `fsevent_add_client` lines in 6 h, about 2160 an hour, about 26 MB an hour, about 620 MB a day. Over 14 days of uptime that is the observed 7.7 GB (5.6 GB compressed). 8000 distinct pids created 1 to 4 streams each; a few long-lived pids created 60 to 377.

## Who creates the streams
Bun implements `fs.watch(directory)` on macOS as an FSEvents stream (a directory is watched through FSEvents; only single files use kqueue). It also tears the shared stream down and builds a new one for every watcher that is added or removed (3 watchers produced 5 `add_client`).

Live capture (`log stream` joined with `ps` at the moment of each `add_client`): every one of the 116 captured pids was a unit-cache lane process (one bun process per rustc unit, `cargo/unit-cache/script.ts`). Each one calls `watchChangesV1` (`unit-cache/lock/ts`) in `acquireUnitClaimV1` (1 stream per unit, even when nobody else holds the claim), `acquireLightSlotV1` (1) and `acquireSlotV1` (2). That is the 1 to 4 streams per short-lived pid. The GIS verification lanes compile thousands of units, so they created the leak fastest.

Other, much smaller sources (a few hundred streams in 6 h, not changed):
- the resident `semio-repo-watch serve` after `USER DROPPED` events replays with a new stream per resync (pid 5618: 377 streams, 509 drops; pid 28276: 64 streams, 86 drops);
- `startNativeSourceWatchV1` (`os/plugin/build/watch/plan`) in the dev MCP bun process: one `fs.watch` per source directory, rebuilt on each addition (60 to 100 streams a second while a dev server starts, 202 in 6 h);
- the replica daemon (1 recursive watch of the worktree per start).

## Ruled out
- Spotlight: `mdfind -onlyin .cache` finds nothing (hidden directory, not indexed); the cache image volumes under `.cache/b/*/m` already carry `.metadata_never_index` and `.fseventsd/no_log` (the `silence` module), and the log confirms `no_log` for them.
- Build churn on the cache volumes: they are separate APFS images with logging off.
- Restarting the replica daemon or mounting images does not change the footprint.

## Fix
`watchChangesV1` no longer opens any directory watch on macOS. On darwin the waiter sleeps at most `policy.lock.watchlessPollMilliseconds` (100 ms, jittered) and the caller re-inspects the claim directory, exactly as the existing jittered fallback already did. Linux and Windows keep the native watches (inotify and ReadDirectoryChangesW do not leak in a daemon).

Files:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts` (`watchChangesV1`, platform and watch injectable for tests)
- `.../unit-cache/🧬️schema/🔣️.json` (`lock.watchlessPollMilliseconds: 100`)
- `.../unit-cache/🧪️tests/👂️watchless/🟦️.ts` and `🔣️.json` (language-agnostic case table: darwin opens 0 watches and sleeps at most the poll, linux and win32 open one per directory; plus a real inotify-style wake test, skipped on darwin)

The sealed lane scripts under `.cache/b/r/m/u/sealed/` re-seal on change; the new seal (`4e5b6565956005c8`) contains the fix, so every lane started from now on creates no stream. `lightslot` tests still pass (4/4), new tests 3 pass, 1 skipped.

## Memory
| step | footprint (`top -stats mem`) | compressed |
| --- | --- | --- |
| start | 7760 M (resident 2.16 GB) | 5.6 GB, fluctuating 4.6 to 6.8 GB |
| after replica kickstart | 7779 M | unchanged |
| after leak experiments (about 1600 test streams) | 7812 M | unchanged |

The leaked memory cannot be reclaimed without restarting fseventsd (root only, not done). The growth stops with the fix; the swap pressure ends when the box is rebooted or an administrator runs `sudo launchctl kickstart -k system/com.apple.fseventsd`. fseventsd rebuilds its state at once; clients (`semio-repo-watch`, replica) replay from the volume history.

## Follow-ups not done
- `semio-repo-watch` resync storms after `USER DROPPED` (one stream per replay, up to 8 in a row).
- `startNativeSourceWatchV1` should hold one stream, not one per directory.
