# R232: software walls of g1-m04 (Linux) and g1-m06 (macOS), the digest mismatch

Status: fix staged and tested; the 01:00 UTC window was lost (`landing.lock` held by R230 from 01:00:14, my script waited 150 s and wrote nothing); landing retried at the 01:30 window, final state in section 6.

## 1. What `digest` in `ready.json` is, and why the three differ

`digest` is `readyDigestsV1` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts`): per stage `sha256(name, platform, 64 zeros, "<input path>\0<content hash>\n" for every input)`; the machine digest is the sha256 over the five stage digests. Inputs: `native` = native bootstrap schema + `global.json` + the platform bootstrap script, `repo-mcp` = none, `dependencies` = `bun.lock` + `package.json`, `tools` = `🧰️tool-setup/🔣️.json` + `package.json`, `workspace` = none. The platform name is part of every stage digest, so a Linux, a Windows and a macOS machine have different digests by construction. It is not stale code.

Proof: the three digests computed from the live tree with the real function (platform argument only; script `scratchpad/R232/dig.ts`):

| platform | digest from the live tree (00:35 UTC) | what the machine reports |
|---|---|---|
| linux | `304d5d98` (native b627dd, repo-mcp 62ba10, dependencies c805fc, tools 4d5f05, workspace 7ac685) | g1-m04 `304d5d98` |
| windows | `ce0ca918` | g1-m05 `ce0ca918` |
| macos | `70ffaf34` | ready majority `70ffaf34` |

All three machines see exactly the inputs of the live tree: no replica lag, no stale code, no platform-specific input beyond the platform name. The `tools` digest of g1-m04 (`4d5f051bd075`) has not changed since 2026-11-16 03:47 (`ready-timings.json`): the 49 failed attempts were never caused by newer inputs. Nothing to fix in the digest; `repo fleet-status` could print the per-platform expected digest next to the reported one (not done).

## 2. g1-m04 (Linux, `ueli-workstation-lin-1`): exact failing step and cause

Evidence: `origins/g1-m04/ready-log-tools-failed.txt`, `ready.json`, `ready-timings.json`.

- Attempt 48 (23:38:32 to 23:55:27 UTC, 1014 s): `Retained dependency generation eda9de17b6b269e988a1 cannot be read with the current field set (missing ["singletons"] ... found 10 of 11) ... retired as obsolete`, then `Internal error: Every current dependency scope must be qualified before inactivity (reference a8c34143)`.
- Attempt 49 (00:15:28 to 00:39:46 UTC, 1458 s): `Internal error: Current dependency binding observation changed (reference e0f740ad)`.
- Both are the same step: `tools setup all` retires obsolete dependency generations (`retireObsoleteRepositoryDependencyGenerationsV1`, plan, observe, retire; each phase re-reads the current bindings with `readJavascriptDependencyCurrentBindingsV1`, 275k to 290k path stamps, 60 s and more per read on the Deck, at least five reads per generation). That read pins, for every ancestor directory of every package with a `node_modules` link, the whole stamp `dev:ino:mode:nlink:size:mtimeNs:ctimeNs`, checks them again during the read (`Current dependency binding observation changed`) and hashes them into `sha256`, which the selected retirement compares with the sha of the plan (`current.sha256 !== selection.currentBindingsSha256`, thrown with the wrong text "must be qualified before inactivity"). The size, link count and times of a directory change with every file a peer, a tool or the replica creates, renames or removes in it; a checkout that is edited by 50 machines moves some of 590 package directories within any 1 to 20 minutes. So the read or the plan goes stale on a slow machine, always, and the whole 17 to 24 minute attempt is lost; 49 attempts, 20 minute back-off.
- Why only g1-m04: it holds a retained generation written by older code (`eda9...`, 10 of 11 fields) and is the only machine that needs the selected-retirement path; machines without an obsolete generation get "no candidate" from the first plan and never reach the compare. Earlier walls of the same machine (all gone or machine side): tries 1 to 9 `Exact selected dependency fields required` (fixed by R73), tries 10 to 35 `Authorizer source closure capacity exceeded 2049/2048` (ceiling is 65536 files now), tries 36 to 37 `unable to open database file` (EDQUOT/ENOSPC of `/home`, 11-16), 38 to 41 stalls and timeouts.
- Classification: CODE fault (shared tree), confirmed by the attempt-49 text; the machine faults of 11-16 (inode quota) are not visible in attempts 48 and 49.

### Fix (by construction, landing pending, see section 6)

1. `javascriptDependencyBindingStampV1` (`🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`): a directory is identified by `dev:ino:mode` only; links and files keep the whole stamp; a missing path has none. The read uses it for both the in-read check and the hash.
2. `requireRepositoryDependencySelectionCurrentV1` (`💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts`, exported): an unqualified scope stays the plain refusal; a moved sha is the typed bilingual refusal `caching.dependency-store.bindings-moved` (planned and current 8 hex, scope count).
3. `retireObsoleteRepositoryDependencyGenerationsV1` (`⚙️setup/♻️retirement/🟦️.ts`): `bindingsMovedV1` finds the refusal at the top or inside an `AggregateError`; the plan is made again at most `maximumBindingsReplans` = 3 times in a row (schema `🧬️schema/🔣️.json`, count restarts after each retirement, warn line `dependency-store.bindings-replanned`); after that the rest is deferred with the warn line `dependency-store.retirement-moved` and the stage goes on (housekeeping never fails the tools stage, same as the busy case). Every other failure still fails.
4. Texts: three keys in `⚡️caching/📖️texts/🔣️.json`, EN and DE.

Laws (new, red on the old tree: import of the new exports fails): `💾️storage/🧪️tests/🔖️stable-stamp` (5: directory stamp survives add, write, remove while nlink/mtime/ctime really move; directory vs other directory, link and file; link and file keep the whole stamp; independent Python `os.lstat` reading agrees; the read uses the function) and `♻️retirement/🧪️tests/🌀️bindings-moved` (7: typed bilingual refusal and plain unqualified refusal, aggregate detection, two moves then success, endless moves end in a warning, bound counts in a row, aggregate replan, other failures unchanged). Staged with a bun preload that maps the five changed live files to the staged copies: 5/5 and 7/7 pass; busy-driver 12, retirement main 5, roster-bound 9 pass staged. Changed tests: `👓️reading` (port of the new function, `refuseCachingV1` for the admission slice) and `🚰️deferred-flush` (source-text assertion of the new throw).

NOT verified: the real Deck run (needs the landing and one more attempt); the real `readJavascriptDependencyCurrentBindingsV1` on a real store (the `👓️reading` file is red at baseline in plain bun: 57 pass 38 fail, it needs the `repo test` harness; the landing compares the pass/fail set with that baseline); the sha stays sensitive to links, receipts and file stamps by design. Open performance point: the retained-path check is quadratic (`check()` re-stats every retained path at each progress batch), 60 s per read on the Deck.

## 3. g1-m06 (macOS, `mac-mini-m4-04`): exact blocked step

Evidence: `origins/g1-m06/ready.json`, `ready-log-workspace*.txt`, `slice-memory.json`, `pool.json`, `ready-timings.json`.

- Stage `workspace`, run 1: 22:54:56 to 00:03:58 (4140 s), log ends at `Planning the selected targets and what they need`, ended by the supervisor as `stage-timeout` (soft cap 3600 s, then 10 minutes without a moving counter). Run 2 (slice process pid 93830, started 00:12:43) is at `Reading the project graph slice (2910 s): taken 4 of 591` at 01:02 UTC: 4 of 591 owners in 48 minutes, the counter has not moved since the first line seen at 00:28 UTC.
- The single `slice-memory.json` snapshot (761 s uptime, 1.2 GB, not rewritten since) says: ledger present 4, missing 587, cursor 8, limit 4, `farm: null` (no child processes on this machine), fleet fetched 0, published 0, computed 0; six looks active for 134 to 235 s (`@semio-tech/2d-text-rs`, `semio-framework-2d`, `@semio-tech/2d-booleans-rs`, `@semio-tech/2d-path-flatten-rs`, `@semio-tech/framework`, `@semio-tech/time-travel-actions-rs`), equal in number to `alone: 6`; the four finished looks took 1.59 s each. So the first owner pass of the project-node read is blocked on six looks of heavy (`alone`) Rust owners that stand for more than 40 minutes; the dispatch cursor does not pass them. Peers finished the same 591 owners in 2058 to 3917 s (m03 2058, m07 2014, m08 2600, m10 2482, m01 3917) when the fleet had records; today the fleet has nothing to give (fetched 0) because the whole read of the group server (pid 18668) has not sealed.
- Competing load on m06 at the same time: the pool puller (`pool.json`: 16384 MB total, grant 4915 MB, children stopped by the memory limit with exit 137, 26 claimed, 14 published, 1.2 owners per minute) and three foreign cargo builds plus a `pack` command (`ready-log-workspace-containment.txt` 00:05 UTC, 84 foreign registry entries).
- Classification: NOT proven code-side from the store evidence. A code-side contribution is certain only in the visibility: a look that stands 40 minutes prints no step line and no reason, the ready supervisor sees an unchanged counter, and the stall rule counts it as no progress only after the soft cap (first run) while the adopted run is not stopped at all. Which resource the six looks wait for (memory grant, CPU, a lease) cannot be read from the store; the heartbeat `[repo] still working after N s; last phase started: door.script` carries no information. I did not change this area: the graph read and the farm belong to other packages (R177, R192, R194, R203), and a landing below the graph tree is forbidden until the read sealed. Recommendation for them: a look that stands more than 60 s writes its resource and holder into the step line (EN/DE), and an `alone` owner on a machine without a farm is set aside at once (`setAsideOverrunV1` exists) instead of being looked at in the main process.

## 4. What a human must check on the machines

g1-m04 (Deck), in a shell on the machine:
1. `df -h /home /tmp` and `df -i /home`: on 11-16 `/home` hit 5,966,399 of 5,966,400 inodes and `/tmp` has a per-user quota (EDQUOT); both must show at least 20 percent free. The old Trash worktree (about 1.02 million inodes) is yours to delete; also `du -sh ~/.cache/semio ~/semio/.cache` and `ls ~/.cache/f/owner-pool-staging`.
2. The Deck must not suspend: Settings, Power, never sleep; `systemctl --user status` of the replica daemon and of the supervisor (pid 280538 per `ready.json`); `uptime` (the lease registry listed 78 foreign entries).
3. `ls ~/semio/.cache/b/d/m/j/` and `cat ~/semio/.cache/b/d/m/dependencies.json`: after the landing, attempt 50 or 51 should print `Retiring obsolete dependency generation ...` lines; if it prints `bindings-replanned` three times and `retirement-moved`, something outside the checkout rewrites the package directories: run `inotifywait -m -r -e create,delete,move ~/semio --exclude '(\.git|\.cache|node_modules)'` for one minute.
4. If it still fails with a new text, send `~/semio/.cache/f/ready/tools.log` (the last 100 lines without the `Checking declared package providers` lines).

g1-m06 (Mac mini):
1. `vm_stat 1 5`, `memory_pressure`, `sysctl vm.swapusage`, `uptime`: swap above 2 GB or free memory below 1 GB explains the stand.
2. `ps -Ao pid,etime,rss,%cpu,command | sort -k3 -nr | head -15`: look for the three cargo builds (`a2d81a06...`), the `pack` command and pid 93830 (the slice read); `sample 93830 5` shows whether the process computes or waits.
3. `df -h /` (256 GB disk): below 25 GB free the build budget refuses.
4. Stop competing agents or the puller on this machine for the time of the read (`repo fleet` puller pause, if you want it, is yours; I did not touch any process).
5. Is the checkout in sync? `repo` build digest `440cffa5563d` in the log `Loading the project graph` must equal the one of m02/m03.
