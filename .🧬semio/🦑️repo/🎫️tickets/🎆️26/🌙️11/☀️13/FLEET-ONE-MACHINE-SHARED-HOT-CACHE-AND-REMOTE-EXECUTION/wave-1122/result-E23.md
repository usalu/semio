# E23 result: the graph service key vanished at 03:38 (root cause, fix, tests)

## Root cause (proven, not a repo process)
The key is `$TMPDIR/semio-repo/<uid>/key` (here `/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/semio-repo/501/key`), written once with `wx` by the first service of the directory and read by every client. It is NOT in `.cache/f`, in any ledger area or in the replica, so no cache gc, retirement, CAS collector, ledger carve-out or replica delete can see it.

What removed it: **macOS `dirhelper`**. Unified log (`/usr/bin/log show`):
- 03:35:05 `UserEventAgent: Running StartCalendarInterval: com.apple.bsd.dirhelper`; `/System/Library/LaunchDaemons/com.apple.bsd.dirhelper.plist` has `StartCalendarInterval 03:35` and `CLEAN_FILES_OLDER_THAN_DAYS=3`.
- 03:35:05 `dirhelper: Cleaning T/ older than 3 days` and `Cleaning /var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T` (this user's `$TMPDIR`); the directory mtime of `semio-repo/501` moved to 03:38, the end of that sweep.
- Key age: the previous key was created 2026-11-19 ~01:36 (sibling `host-acac0579a983.log` birth 11-19 01:36, the host that wrote it) and never rewritten, so at 03:35 on 11-22 it was 3.07 days old. Nothing ever touched its mtime.
- Same incident earlier: `plans-89ee733bc283` has mtime 11-18 03:38:09, the night the 11-14 key (4 days old) was swept; a new host recreated it on 11-19. It repeats every ~3 days for any host that runs that long.
- Excluded: exit log shows no exit of any host between 03:24 and 03:49:38 (E22's `service host stop`); the running host (pid 87229, started 03:23:45) held the key in memory and kept serving, which is why clients failed with `(key)` while the service was healthy. The key written at 03:49:48 is E22's restart (`wx`, birth 03:49:48 proves it was absent). No `landings.md` entry 03:20 to 03:45 touches resident key handling; nothing in `🏡️resident` or the build library removes `key` (grep of every `rm/unlink/rename` over `key` paths: none); the disk-recovery temporary-directory cleanup only selects directories named `semio-<tag>-<pid>-<random>`.
Why only the key and not the logs: socket files, directories and recently written logs survive dirhelper; a regular file written once and never modified does not.

## Fix (root, owner side + client side)
1. New `🏡️resident/📨️protocol/🔑️capability/🛟️keeper/🟨️.mjs` (+ `.d.mts`): `keepRootKeyV1({path,key,touch,refreshMilliseconds,now}).tick()`: touches key and `touch` files (descriptor) once older than `refreshMilliseconds` (set both atime and mtime), writes a MISSING key back byte for byte with `wx` and mode 0600, never overwrites a different key of a newer generation, returns `intact|refreshed|restored|failed`.
2. Host service (`🏛️service/🌐️host/🟨️.mjs`): builds the keeper after reading/creating the key; `sweep` (resident timer, 5 s) ticks it, logs `key-restored`, and rewrites a missing descriptor. Workspace service (`🏛️service/🟨️.mjs`): same on its resident timer (its key lives in `semio-repo-<uid>/<id>/`, equally exposed).
3. Policy `keyRefreshMilliseconds: 3600000` in both resident policies (`🧬️schema/🔣️.json`, `🎚️policy/🟨️.d.mts`): 72x finer than the 3-day cleanup age.
4. Client (`🚪️client/🛰️plan/🟨️.mjs`): `planKeyActionV1(error,waited,restarted)` -> `wait` (up to 30 x 1 s, a healed key appears within 5 s), then `restart` ONCE (`retireHostServiceV1` then the normal `connectHostV1` start writes a fresh key; covers services of an older generation without the keeper, which is the state of every running host today), then `fail`. Damaged/unreadable keys (not ENOENT) still fail at once. Worst case a command now waits ~35 s instead of failing for 10 minutes.
Ledger: no ledger declaration is needed or possible: the path is outside every cache root; the claim "a collector/retirement run never touches service state" is covered by the test below instead of a ledger entry.

## Proof
New `🧪️tests/🛟️key-keeper/🟦️.ts` (8 pass): missing key restored with identical bytes and 0600, different key left alone; aged key/descriptor touched before the cleanup, fresh one not rewritten; independent oracle: the system `find -type f -mtime +3 -atime +3 -delete` takes an unkept key and spares a kept one; `planKeyActionV1` decisions; in-process service: a removed key is written back and the SAME `requestPlanV1` then succeeds with no restart; an aged key+descriptor are refreshed by `sweep` and `find` finds nothing; policy bound; `selectTemporaryV1` never selects `semio-repo`, `sr501`, `semio-repo-501`, key/socket/descriptor files. Updated pin in `🧪️tests/🚪️client/🗝️key/🟦️.ts` (3 pass).
Regression runs: host 39/0, service 32/0, consent-service 31/0, handover 2/0, protocol 23/0, keepalive 8/0, reload 12/0, launchd 9/0, same-cause 5/0, late-attach 1/0, suites 5/0. Landing recorded in `T/landings.md` (lock `resident` taken over from E19, older than 40 min; released).

## NOT verified / for others
- The restart branch (`restart`) is proven only at decision level (`planKeyActionV1`) and by reading `retireHostServiceV1`; no end-to-end test with a host that never heals (needs a stub host that answers `host-status` with a killable pid).
- The live hosts (pid 17673 tag c17863051425, started 04:30) run the old code until the supervisor reloads them; their key is young (03:49:48), so the next dirhelper deadline is 03:35 on 11-26 or later; the keeper is active after their next generation.
- dirhelper's exact age rule (atime vs mtime) is not documented; the keeper refreshes when either is older than an hour, so it holds under both.
- Windows/Linux: not swept by dirhelper (Windows uses LOCALAPPDATA; Linux XDG_RUNTIME_DIR), but systemd-tmpfiles can clean `/tmp`-based fallbacks; the same keeper covers it.
- Other files of that directory older than 3 days could also be swept (start locks are recreated with `wx`; plan spills expire after 6 h); not changed.
