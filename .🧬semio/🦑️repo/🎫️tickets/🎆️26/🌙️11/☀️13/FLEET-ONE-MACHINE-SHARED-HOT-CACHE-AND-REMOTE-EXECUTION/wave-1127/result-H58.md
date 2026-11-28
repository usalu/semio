# H58: g1-m05 replica daemon: no synchronous filesystem work on the event loop

Evidence: `S/origins/g1-m05/attest.json` (08:08, still code 0830ff81, i.e. before H51/H58; failures `loop-blocked-restart` 93, hot passes 0; no `loop-blocked-in-*` yet, it appears only after m05 runs newer code). Everything below is from reading the code; nothing ran on Windows.

## Root causes found in the daemon
1. Hot, directory-sweep and file-sweep passes called `lstatSync` per path and checked the time budget only every 64 paths. At 25-40 ms per stat on Windows (Defender) one slice blocked for 1.6-2.5 s, and the three tiers plus follow keep the loop saturated.
2. Watchdog thread of the boot stage (`🛟️supervisor`) called `parentLives` every second, i.e. `spawnSync powershell Get-CimInstance` (15 s timeout) on Windows and `ps` elsewhere: back-to-back PowerShell launches on a machine where a stat costs 40 ms; a CPU and Defender drain that starves the daemon (the worker thread, not the loop, but the same cores).
3. `writeJson` fsync was synchronous (`fsyncSync` on the loop).
Not changed: `ignoreWatchEventV1` does one `lstatSync` per watch event but only for dir-form-ignored ambiguous paths (rare); the 767 s view pass is a `git status` child, not loop work.

## Fix
- New `🔁️replica/🐌️slow-fs` (`StatBudgetV1`): stat results as values; synchronous while a stat averages < 1 ms (cheapest on macOS/Linux, same as before), parallel asynchronous chunks of 64 on the libuv pool once slow, probing back to sync every 32 chunks. `chunk(budget)` sizes a slice from the measured cost.
- Hot, sweep and file passes take chunks from it and await them (yield point per chunk); `hotChanged`, `directoryMoved`, `sweptDriftV1` consume the fetched stat (no second stat of a changed file).
- `syncPath` is async (`FileHandle.sync`); directory fsync stays skipped on Windows.
- Watchdog: `process.kill(pid,0)` every tick; creation-time identity only every 30 s.
- Change journal: not added. A USN/ReadDirectoryChangesW cache needs native bindings (no runtime dependency allowed); the existing recursive `fs.watch` already feeds the hot set (ReadDirectoryChangesW on Windows), so the passes only verify.

## Tests
- `🐌️slow-fs/🧪️tests/🟦️.ts` 4/0: fake fs with 5 ms per stat, 600 paths: longest synchronous section between loop turns < 100 ms (the old loop: 320 ms), results ordered and complete, switches to async; real files equal a Python `os.lstat` oracle; mode returns to sync when stats are fast again.
- boot.ts 24/0 (fixed the H51 `activity` field in the restart expectation), turns, trust, sweep, heal, replica main pass. `repo typecheck` 4 files + importers 0 errors; dangling-imports clean.

## Landing
Landings entry `Wave 1127 H58` at 08:25, lock `landing.lock.replica`. Caveat: the edits were made in place (shared worktree) at about 08:20 and not staged for a slot; the area is not an input of any workspace stage, so only the replica daemons restart once.

## Watch / open
- m05 attest at 08:08 was 16 min stale; it must restart its replica on the new code. Next check: `code` field of attest.json changes from 0830ff81, `tiers.hot.passes` > 0, `loop-blocked-in-<activity>` (H51) names anything left, then ready.json.
- USER action: Defender exclusion for `C:\semio` (provisioning cannot elevate); expected to be the largest remaining factor in the 25 files/s.
- Not verified: Windows behaviour of the async chunks (threadpool size 4 default; `UV_THREADPOOL_SIZE` not raised).
