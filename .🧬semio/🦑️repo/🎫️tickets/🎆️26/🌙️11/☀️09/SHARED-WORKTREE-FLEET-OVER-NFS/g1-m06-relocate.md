# g1-m06 Relocate to ~/Documents/semio

Machine `g1-m06` (group `g1`, macOS, account `ueli`, 16 GB), 2026-11-13 21:00 to 2026-11-14 00:20 local time.

## Result

| Item | State |
|---|---|
| Worktree | `/Users/ueli/Documents/semio` (the canonical macOS path of the topology), the only semio tree on the machine |
| Origin | `g1-m06`, unchanged |
| Services | LaunchAgents `tech.semio.fleet.replica` and `tech.semio.fleet.compact` of `ueli`; LaunchDaemon `tech.semio.fleet.store-mount`; the system daemons of replica and compact are removed |
| Boot program | version 3, entry directory names `/Users/ueli/Documents/semio` |
| Consent | no prompt appeared; preflight `worktree access: the replica can read /Users/ueli/Documents/semio` |
| `status` | daemon running, synchronisation on, 0 incoming queued, git state shared, stage tree `47eebd16b9bf…` |
| `fleet` | row `ok` at 23:05 (`152/0/208`, code `2d6054177087`, the same as `g1-server` and `g1-m07`); at 00:19 `168/2/262` on code `68d4c0439449`, the same view as `g1-server`, verdict `lagging (not settled)` because the tree is being edited |
| `verify` | 273075 shared paths checked, drift on 1 path that another machine was rewriting (`26/11/13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landings.md`); exit 1 for that path |
| `preflight` | 23 checks, none failed, 2 warnings (time synchronisation needs root, native build pending) |
| Old tree `/Users/Shared/semio` | emptied after the audit below; the empty directory is removed at the end of the session |
| Native build | **not complete**: `.cache/f/native-bootstrap.done` absent, see "Native Build" |
| `relocate --phase finish` | never passed; `.cache/f/relocate.json` in the new tree stays at step `adopted`, see "Finish" |

## State Before

- `/Users/Shared/semio`: joined worktree of `g1-m06`, system LaunchDaemons, no native marker.
- `~/Documents/semio`: plain clone of 2026-11-12 at `5ececaa78b9` (the fleet's HEAD, equal to `origin`), `git status -uall` empty, no stash, no ignored files, no `.cache`, no fleet module. It held no local work.

## Dry Runs

1. 21:10, code of the old tree: 269439 identical, 1879 missing here, 17 only here, 1733 differ (fleet wins), 79 deleted by the fleet; git relation equal. 10 of the 17 "only here" paths were a letter-case difference (`🏛️bestest-600FF`, `🏛️bestest-900FF` under `…/📚️examples` and `…/🖼️assets` of the energy plugin; the old tree had `…ff`). The count of fleet-wins decisions is above the 25 of the relocate prompt; every losing file was the committed version of a clone without local changes, so the run continued.
2. 21:22, code of the new tree after the preparation: 272783 identical, 77 missing here, 7 only here (git packs of the clone), 6 differ (fleet wins), 80 deleted by the fleet; git relation equal.

## Preparation of ~/Documents/semio (no git command)

- The four `…FF` folders were renamed to `…ff` with `mv`.
- The shared file set of the old tree (`git ls-files --cached --others --exclude-standard`, 272782 paths, 4.6 GiB) was copied over the clone with `rsync -a --from0 --files-from`. This brought the fleet module into the clone, which the provision script needs there.

Finding: the fleet holds both spellings. After the move the daemon logged `lost the structural conflict (case-collision)` for the `…ff` paths against `…FF` (events of `g1-m01` and `g1-m02`) and settled it by itself; the rename was not needed and not harmful.

## Provision Runs

Exports as in the join. Command `bash <topology copy>/🥾️provision/🐚️.sh g1 g1-m06 --user ueli`.

1. 21:24 through an administrator dialog (`osascript … with administrator privileges`): exit 1 after `step 6a`, no error line, nothing changed. A root process started that way cannot list `~/Documents`, so `ls …/🥾️entry/*.mjs` failed under `set -euo pipefail`. Finding: the script ends silently there.
2. 22:07 as `sudo` in the terminal of the Claude app (root could list the folder): system daemons removed, LaunchAgents written, `relocate` prepare and move (`Relocation step reached: adopted`, 7 files and 4.05 GB published: the git packs of the clone), services loaded, **exit 3** (native build pending).

Two defects met in run 2:

- `boot program: {"action":"refused-downgrade","version":2,"installed":3}`: the copy in the new tree was older than the installed boot program, so `--take-over` did not repoint the entry directory. The new LaunchAgent then started the replica in the old tree (`Replicating /Users/Shared/semio … sync off`) and the script waited 90 s and reported the consent as pending. Remedy: after the adopt had brought boot version 3 into the new tree, `repo …/🥾️entry/🟨️.mjs install --worktree ~/Documents/semio --store /Volumes/semio-store --take-over` as `ueli` (`{"action":"current","version":3,"changed":true}`), then `launchctl kickstart -k gui/501/tech.semio.fleet.replica`.
- `hdiutil resize -size 36863m …/.cache/b/d/fixed.dmg` failed with `Invalid argument (22)`: 19 GB were free while the old tree held 144 GB of cache images.

## After the Move

- The stage tree was attested as `-` (the daemon cached a failed `git write-tree` from the time the 3.9 GB pack was still arriving; `write-tree` on a copy of the index gave `47eebd16b9bf…`). One restart of the agent fixed it.
- 87 untracked absolute symlinks under `26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror` (86) and `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` (1), refused as `unsafe-link`, were removed with `unlink` (the set `g1-m01-cleanup.md` removed). The view went from `…/296` to `…/208` untracked.

## Merge Audit of the Old Tree

Every shared path of `/Users/Shared/semio` (272853) was compared with the new tree by size, time and sha256, twice (22:55 and 23:00):

- 272734 the same, 117 newer in the new tree (fleet edits after the move), **0 newer in the old tree**.
- 2 only in the old tree, both deleted by the fleet: an empty `📌️important/📝️.md` of `26/11/13/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02` and a `landing.lock/owner`.
- Unshared content of the old tree: `node_modules`, `.nx`, `.cache`, generated sources, build output. Nothing else.
- `.cache/f/lost` of the old tree (58 MB) and its `native-bootstrap.log` are kept in `~/Documents/semio/.cache/f/lost/old-worktree-shared/`.

## Removal of the Old Tree

- Still running there: two `repo service host run` processes and an orphaned `repo run workspace:components -- describe norm` of 19:53 (parent 1, from the ticket `26/11/10/GET-NORM-PLUGIN-WORKING-END-TO-END`; no agent session owned it). `repo service host stop` stopped the graph service; the run (pid 43068) and the last service process (pid 35803) were ended with `kill -TERM` on those two pids.
- Ten cache volumes under `.cache/b` were detached with `hdiutil detach`.
- The contents were removed with `rm -rf`; free space went from 19 GB to 139 GB.

## Finish

`relocate --phase finish` answered `The new worktree or the fleet does not agree yet` on every try (one path in `verify` was rewritten by another machine every few seconds). After the removal it answers `The old worktree has no machine identity`. The marker `relocated.json` was therefore never written; there is no old tree left to mark. `.cache/f/relocate.json` in the new tree still says `adopted`.

## Native Build

`native-bootstrap.sh` of `26/11/13/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02` (step 9 as `ueli`, marker only after exit 0).

| Run | Stopped at |
|---|---|
| 1 (in the provision run, 22:25) | `hdiutil resize … fixed.dmg` status 22: disk full. Gone after the old tree was removed. |
| 2, 3 (22:58, 23:00) | `go:build`: `📥️runtime-provision/🐹️.go:331: undefined: fleetStoreDirectory`, `fleetFetchInto`, `fleetPublish`. The member `🚀️launcher/📦️packages/🐹️go/🛰️fleet-store` (package H of `26/11/13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`) was not a registered member name, so the canonical Go overlay left it out. **Fixed**: `"🛰️fleet-store"` added to `memberNames` of `members-of-members-of-members-of-modules` in `📚️library/🔣️taxonomy.json`. |
| 4 (23:03) | `Internal error: Exact privately produced preservation capsule required` in the dependency scopes (as on `g1-m07`). `repo dependencies sync --all` afterwards exited 0. |
| 5 (23:11) | 55 minutes of `Reading the project graph slice: reservation 0 of 8192; running: (none); waiting for memory headroom (8192 of 6624 – 8875 MB reserved)`. Stopped. |
| 6 (00:10, fresh graph service) | the same wait for 17 minutes: `8192 of 6434 – 7307 MB`. Stopped. |
| 7 (00:28) | detached retry loop `🧰️scripts/g1-m06-native-retry.sh` (parent 1, up to 24 attempts, 120 s apart, ends when the marker exists). Record: `.cache/f/native-retry.log`; output of the current attempt: `.cache/f/native-attempt.log`. |

Measured at 00:27: the floor is 2048 MB (`floorBasisPoints` 1250 of 16 GB), so the slice starts only with about 10.2 GB available (free plus inactive). With the Claude app and its session (about 2 GB), the replica (1.3 GB), the graph service of the run itself (1 GB) and 2 GB wired the host offers 8.1 to 9.3 GB. The run can pass only while the Claude app is closed.

Open: a cold graph slice without samples reserves `fallbackCeilingMegabytes` 8192 MB (`🏗️build/🔣️policy.json`, `estimates`), and this 16 GB host offers 6.6 to 8.9 GB above the floor with the Claude app (1.9 GB) and the replica (1.4 GB) running. The policy was not changed: it is shared and was edited by another machine at 23:47. `g1-m07` (same hardware) waits at the same step (`6627 of 2638 – 5020 MB`). The slice itself is the open problem of `26/11/13/GET-MIT-BESTAND-DEMONSTRATOR-RUNNING-END-TO-END/🕸️graph-slice-slowness.md`.

To retry: `bash …/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02/native-bootstrap.sh <log>`.

### Later attempts (00:30 to 04:15)

- **Cause of the 8192 MB wait found**: `$TMPDIR/semio-slice-peak-501.json` held `{"megabytes":21625}`, written 21:40 by the orphaned `workspace:components` run of the old tree. `sliceLearnedPeakV1` (`🏗️build/⌨️command/🌐️host/🟦️.ts`) only ever raises that value, and the slice pool books `min(maximumMegabytes 8192, learned)`. A peak of 21.6 GB on a 16 GB host cannot be a real resident set. The file was removed; the next load booked the unlearned 2048 MB, ran, and stored 5897 MB. Finding for the owner of the slice pool: the learned peak has no upper bound by the host's memory and no way down.
- Loop attempt 1 (00:28): graph slice read, then `ENOTDIR: not a directory, lstat '…/semio/CLAUDE.md/package.json'`. A root symlink `CLAUDE.md -> AGENTS.md` appeared in the shared tree at 00:58; the Bun workspace ownership scan probes `<entry>/package.json` for every root directory or symlink and only treated `ENOENT` as missing. **Fixed** in `📚️library/🗂️workspaces/🟦️bun/🟨️.cjs`: `ENOTDIR` is missing too, in `observe` and in the two `read` helpers of the replay and the authentication (attempts 3 and 4 failed with `Bun current ownership changed after refreshed continuation` while only `observe` was fixed).
- Loop attempt 5 (01:29): past the dependency scopes, graph slice of 702 owners read, `workspace:setup` running at 04:13.

## Replica Down and Stage Not Shared (02:30 to 04:12)

Reported by the owner: the staging of this machine never reached the other workers.

- 02:30:57: `git-index.json` `waiting / objects-missing`. Three packs of 3.7 to 3.9 GB each were arriving (`unstable` events); stage sharing waits in both directions meanwhile.
- 03:00:10: the daemon ended with 0 to start new code (`The code on disk changed`). launchd did not start it again: `state = not running`, `pended nondemand spawn = inefficient`, `last exit code = 0`, for 70 minutes. The first 17 restarts of the agent since 22:25 had come back by themselves; at 03:00 the native build had the host at 15 GB used with 7.4 GB in the compressor.
- 04:10: `launchctl kickstart gui/501/tech.semio.fleet.replica`. The packs completed at 04:11 (a check of all 2409 object ids of the 2898 stage registers after the base found none missing), and at 04:12:45 the replica published 710 stage registers. `g1-server`, `g1-m01`, `g1-m02`, `g1-m07` and `g1-m06` attest stage tree `c39a6bad5633…` since 04:13.
- 04:13: the agent was down again in the same way after the next code change and was started again.

Finding for the service entry: on macOS the self-restart depends on launchd starting a `KeepAlive` LaunchAgent again after exit 0, and launchd defers that on a host under memory pressure without a limit in time. The shim (`🥾️entry/🐚️.sh`) ends on 0 and 75 by design. Not changed here: the boot program was edited by another machine at 04:10 (boot version 4).

Temporary measure on this machine: `🧰️scripts/g1-m06-agent-guard.sh` (detached) starts the agent when it stays down for 20 s, only while the native retry loop runs; record `.cache/f/agent-guard.log`.

## Left As It Is

- `/var/log/semio-fleet-replica.log` and `semio-fleet-compact.log` of the removed system daemons (root).
- `.cache/f/lost` in the new tree: 181 overwritten files (the clone's committed versions that the fleet replaced, the old index, the old tree's lost folder).
- AppleDouble files `._attest.json`, `._head.json`, `._home.json`, `._view.txt` in `origins/g1-m06` of the store (extended attributes of files written from this Mac over NFS).
- Claude Code sessions of this Mac were rooted in `/Users/Shared/semio`; new sessions must open `~/Documents/semio`.

## Files Changed in the Shared Tree

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (one member name).
- This file.
