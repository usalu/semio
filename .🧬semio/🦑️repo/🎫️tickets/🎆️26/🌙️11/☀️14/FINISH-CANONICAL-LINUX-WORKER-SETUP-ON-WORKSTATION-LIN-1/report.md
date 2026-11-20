# Canonical Linux Worker g1-m04 on ueli-workstation-lin-1

State: 2026-11-14 09:40 CET. Machine `ueli-workstation-lin-1` (SteamOS, account `deck`, 16 GB RAM), group `g1`, origin `g1-m04`.

## Result

- One repository only: `/home/deck/semio` is the canonical, synchronised worktree. The replica service `semio-fleet-replica@deck` and the timer `semio-fleet-compact@deck` run from it.
- The former replica `/srv/semio` is retired (`relocated.json`, synchronisation off) and no longer at its path. It was not deleted: it lies intact in the Trash as `~/.local/share/Trash/files/semio-srv-retired-g1-m04-20261114` (35 GB, restorable from the file manager). Emptying the Trash is the owner's decision.
- The relocation itself, the NFS reconnect, the adoption numbers and the fix in the fleet code are recorded in `☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m04-relocate.md`.
- Native build: not complete. The machine syncs and is provisioned, but the bootstrap's last step fails on four tasks of the shared source; see "Native Bootstrap" below.

## What Was Changed on the Machine

| Item | Change |
| --- | --- |
| NFS transport | `ss -K dst 192.168.178.70 dport = 2049` once, to make the kernel reconnect a dead connection of the `hard` mount |
| `~/.local/share/semio/fleet/entry/worktree` | names `/home/deck/semio` (written by provisioning) |
| `~/semio/.git/info/exclude` | ten private rules removed; old file in `~/semio/.cache/f/lost/manual/info-exclude-2026-11-14` |
| `refs/old-repo/g1-m04/stash-0` … `stash-8` | the nine stashes that existed only in `~/semio`, kept as loose refs |
| 87 absolute symbolic links | removed (dangling, targets under `/Users/ueli/Documents/semio`); list in `removed-links.md` |
| `/srv/semio` | moved to the Trash (rename on the same file system, `/srv` is a bind of `/home/.steamos/offload/srv`) |
| GitKraken Flatpak override | `filesystems=/home/deck/semio;` instead of `/srv/semio` |

## What Was Changed in the Repository

- `🤝️shared-worktree/🗺️topology/⌨️command/🟦️.ts`: the Linux store-mount check of `preflight` also looks up the resolved mount point (`/mnt` is a symbolic link to `/var/mnt` on SteamOS). Before, `preflight` reported that nothing was mounted.
- `🤝️shared-worktree/🗺️topology/🧪️tests/🟦️.ts`: one case for the linked mount point; 67 pass.

## Native Bootstrap

Four provisioning runs were needed; each failure had a different cause outside this machine's setup.

1. Run 1 (relocation run): the script's own `finish` refused once; the native step was not reached.
2. Run 2: a fleet-wide repack of the shared git objects by `g1-server` (about 02:04) removed packs before their replacement arrived. `git` could not read HEAD for 45 minutes, the Go builds failed at VCS stamping, exit 3.
3. Run 3: watchdog, `repository:build made no progress for 11 min and has no live child`, exit 124, during the dependency step.
4. `repo dependencies ensure` on its own showed the cause of run 3: the installer of scope `♻️mit-bestand` failed three times with `GET … - 502`. The npm mirror of the group server is broken: `http://192.168.178.70:6090/mirror/healthz` answers `Not found`, `…/mirror/npm/react/-/react-18.3.1.tgz` answers 404. With `SEMIO_FLEET_MIRROR=off` the install of all scopes finished with exit 0 (03:42 to 04:31).
5. Run 4 (04:31 to 06:41), exit 3: steps 1 to 8 unchanged and `fleet ready`; step 9 built the cold project graph for 126 minutes (703 owners) and was cancelled in the last phase, the assembly: `the service holds 7125 MB of its 7125 MB memory limit (safety margin 256 MB)`. The limit of one slice load is lent from the memory available at its start (75 % of it less 1536 MB, at most 8192 MB; `sliceHostElasticity`), without swap.
6. Run 5 (06:42 to 09:26), exit 3: the graph was built (the learned peak raised the limit), `nx run workspace:setup --parallel=4` ran 86 minutes, 13 of 17 executed tasks succeeded and four failed. None of the four is a fault of this machine's setup:
   - `@semio-tech/framework-os-mcp-rs:build`: `could not compile semio-os-mcp-plugin-runtime (lib) due to 10 previous errors` (E0433 `serde_json` not found, E0107/E0061 changed signatures, E0308, E0599 `map_err` on `RetainedWorkOutcome`): source that is being changed in the shared worktree.
   - `workspace:deps-cargo`: `Invalid capability output region: Capability Features` in `cargo:owner-preparation`.
   - `workspace:deps-build-tools`: `Internal error: Original Repo build caller required`.
   - `@semio-tech/energy-oracle-py:deps`: `uv python install 3.12` refused, `Python downloads are not allowed (python-downloads = "never")`; CPython 3.12.15 is already installed under `~/.local/share/uv/python`, the script still calls the install under a policy that forbids it.

The completion marker `.cache/f/native-bootstrap.done` is therefore not written. Everything this machine has to provide is in place (tools, pinned runtimes, all dependency scopes installed, project graph built and learned). The marker is written by the next run of `🥾️provision/🐚️.sh g1 g1-m04 --user deck` (with `SEMIO_FLEET_MIRROR=off` as long as the mirror is down) once those four tasks pass in the shared source; a run takes about 90 minutes here now that the graph is warm.

### After Run 5: Why No Sixth Run

- Single runs of the four failing targets (`repo run <project>:<target>`, 09:32 to 11:38) all ended after 31 minutes with `The graph service could not answer (timeout)`. The replica had promoted new build code in the meantime; a promoted implementation starts its own graph service, which has to read the cold graph again (126 minutes on this machine), while the service of run 4 (pid 569455) still idles. Every change of the shared build code therefore costs this 16 GB machine a two-hour graph load before any task starts.
- The causes of the four tasks, as far as they could be read from the source:
  - `workspace:deps-build-tools`: the nested `repo 📜️script.ts setup git` inside the owner command is refused by `📚️library/🏗️build/📥️caller/🟦️.ts` (`Original Repo build caller required`).
  - `@semio-tech/energy-oracle-py:deps`: follows from it. The tool policy of `uv` (`🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json`) sets `UV_PYTHON_DOWNLOADS=never`, the interpreter is expected in the repository's tools partition, and the tools target that installs it there failed.
  - `workspace:deps-cargo`: `📚️library/🗂️workspaces/🦀️cargo/🧩️capabilities/🟦️.ts` expects the regions `# 🧩️ Capability Features` and `# 🧩️ Capability Dependencies` in a manifest that does not carry them.
  - `@semio-tech/framework-os-mcp-rs:build`: compile errors in `semio-os-mcp-plugin-runtime`.
- The same targets fail on every worker. `g1-m01-relocate.md` (sections "Native build retry" and "Native Build: Who Works On It") lists them for g1-m01 and g1-m07 and states that g1-m06 has no marker either; the fixes are being made from the g1-m07 ticket (`☀️13/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02/worker-setup-and-merge-report.md`), and the other workers left the build code alone to avoid two writers on the same files.
- The owner's current worker prompt (`1114-new-mac-worker-prompt.md`, 2026-11-14) says of exit 3: "this is EXPECTED tonight … the fix is being landed and arrives by itself through the fleet. Do not retry the native build in a loop; note it and continue."

For these reasons no build code was changed from this machine and no sixth run was started. g1-m04 is in the same state as every other worker of the group: fleet ready, native build pending.

### Third Pass (11:40 to 12:00): What a Fix Needs

The ticket was reopened to fix the build code from here. Read against the source and the g1-m07 report (sections "Verification Of The Fixes" to "Why The Standalone Tools Setup Stops"), the four tasks reduce to two decisions about protocols of the build, which gate the tools setup on every platform:

1. **Authority for the tools setup.** `workspace:deps-build-tools` runs `repo ./📜️script.ts repo tools setup all`, a nested repo operation. `readOwnedBuildInvocationV1` (`📚️library/🏗️build/📥️caller/🟦️.ts`) admits a nested operation only if the envelope's `arguments[0]` is `repo` and the request is one of its declared `launches`; an orchestration target receives a repo caller only for declared child targets. Either the target gets a script entry of its own (as `dependencies/🏗️native/📜️script.ts sync <kind>` of its siblings) or the execution metadata gains a declared launch for an operation that is not a task.
2. **Roles of the dependencies partition.** The standalone `repo tools setup all` checks the root of `.cache/b/d/m` against the declared roles of the selected dependency plan (`…/💾️storage/🧬️schema/📥️install.json`). This machine holds the same undeclared entries as g1-m07 (`dotnet`, `nuget`, `python`, `generation`, `pointer`; 15 entries), which that check refuses (`Closed current dependency image roles required`).

`@semio-tech/energy-oracle-py:deps` follows from them (the interpreter is installed into the tools partition by the tools setup, and `UV_PYTHON_DOWNLOADS=never` of the tool registry then holds). `workspace:deps-cargo` (capability regions) and the compile errors of `semio-os-mcp-plugin-runtime` are separate.

Not changed from this machine, for three reasons: both points decide how the authority model and the dependency plan are meant to work, not how to repair a slip; the files are the ones the g1-m07 session is changing; and a change cannot be verified here in reasonable time (two hours of graph load after every promotion of build code, then about 90 minutes of setup), while a wrong change to this code reaches every machine of the fleet within a second.

### Fourth Pass (from 12:35): Worker of the Readiness Protocol

At 11:45 the owner opened `☀️14/WORKERS-READY-WITH-LOW-RESOURCES`: a coordinator on g1-server fixes the build code in one place, one agent per worker runs rounds and reports (`PROTOCOL.md`: no file outside `workers/<origin>/`, no build code, no sudo). g1-m04 had no folder there; it has one now (`workers/g1-m04/`: `run-round.sh`, `wait-round.sh`, `round-1.md`, `state.json`), and the native build of this machine is completed through that protocol.

- Before the protocol was found, one build file was changed from here: `🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts` (`faultMessageV1`, an aggregated fault names the faults it holds) with one test (22 pass). It is disclosed in `round-1.md`.
- Standalone `repo tools setup all` (11:42 to 12:34): 52 minutes, then `Internal error: Original physical body, drain and cleanup settlement failed`.
- Round 1 (12:36 to 13:12 CET): `native_rc=1`; `workspace:setup` waited 1883 s for the graph service and timed out. A graph service of an older implementation (pid 569455, started by run 4) is alive without serving, and the service of the current implementation never got past `booting`.

## Findings for the Fleet Owner

- **npm mirror on the group server is down** (404/502 on `:6090/mirror/…`). Every machine that installs dependencies through it fails the same way; the watchdog then reports the failure as "no progress" instead of the HTTP error. Workaround used here: `SEMIO_FLEET_MIRROR=off`.
- **Repack in transit breaks git on the other origins**: packs disappear before the new pack has been replicated; `git-index.json` reports `degraded: write-tree-failed` and HEAD is unreadable until the pack arrives.
- **Case variant in the shared git index**: the index names `🏛️bestest-600ff` and `🏛️bestest-900ff` under `✏️s/🔌️plugins/🔋️energy/…`, the files on disk are `…FF`. The case-insensitive Macs see deletions only, this case-sensitive machine sees added and deleted entries and untracked files, so the `view.digest` of g1-m04 differs although manifest, guards and index tree agree. It has to be corrected once in the shared index (archivist), not on this machine.
- **Daemon event loop blocked for 13 minutes** after the adoption (one core at 94 %, 1300 reads of `/proc/self/statm` per second, no store I/O). A restart was needed; cause not found. A `fleet --wait 150` call spun for 40 minutes in the same period.
- **Receipt check of scope `.`**: `repo dependencies ensure` ends with `ENOTDIR … lstat '/home/deck/semio/CLAUDE.md/package.json'`, i.e. a root-level file is taken for a scope directory. The exit code is 0.
- **`finish` inside the provisioning script** runs before the new daemon has attested and therefore refuses; `relocate --phase finish --wait 0` a few minutes later succeeds.

## Left as It Was

- Background scripts of sessions from 2026-11-02 to 2026-11-10 still run in `~/semio` (`cache-guard.sh`, three `owners-sampler.sh`, `nice-others.sh`, `queue-sequencer.sh`, `phase-watch.sh`, two `until` loops). They write below `🗑️generated` only; the fleet rules forbid `pkill`, and they belong to other tickets.
- From the first join: `/etc/sudoers.d/zz-semio-provision`, `steamos-readonly disable`, `/home/semio-swap` (active, not in fstab).

## Final Checks

From `/home/deck/semio`, 2026-11-14 09:27 to 09:35 CET.

- `status`: store reachable, synchronisation on, daemon running, 279313 files, 0 events behind all eight origins, git state `shared`; apply p50 306 ms, p95 659 ms.
- `preflight --lang en`: 24 checks, none failed, one warning (`native build: pending`). Identity, store mount (the fixed check), mount options, store writable, cache endpoint, clock (-0.4 s), inotify limits, disk (121.0 GiB free), daemon, compaction, worktree access, replica status, file watcher and event visibility (220 ms median, 345 ms p95) are `ok`. Seven git guard checks are `skip` (`no git, no repository or the guards could not be read`), as on the first run.
- `verify --json`: 275399 checked, refused 0 (the 87 links are gone), drift 52, all `extra-local` loose objects under `.git/objects`. Every origin shows the same 52 queued in `fleet`, so they are in transit in the fleet, not a fault of this replica.
- `fleet`: g1-m04 runs the same code `1c863927612f` as the Macs and reports the view 347/12/500 against 347/14/500 (the case variant above). All origins are `lagging (not settled … still moving)` because the fleet is publishing; g1-m05 and g1-m07 are `stale`. At that moment the worktree code did not start for anyone (`🔁️replica/🥾️bootstrap/🧺️packs/🧬️schema/🔣️.json: schema without definitions`) and the sealed code ran instead.
- Paths: `/srv/semio` does not exist; `/home/deck/semio` is the only semio repository of the account.

## Status on 2026-11-16, 22:50 UTC

- Rounds 2 to 31 of the coordinator ticket `☀️14/WORKERS-READY-WITH-LOW-RESOURCES` are reported in its folder `workers/g1-m04/` (`round-N.md`). Since round 31 the machine drives its own setup; stages `native`, `repo-mcp` and `dependencies` passed, `tools` (4 of 5) was running in try 38, `workspace` was not reached. `.cache/f/native-bootstrap.done` is absent.
- 14:10 to 18:27 UTC: out of the fleet because the home volume had no free file slots (leftover owner-pool staging folders); cleared, see `round-31.md`, section "Standstill".
- 22:07 UTC: a SteamOS update switched the system slot at reboot. The store line in `/etc/fstab`, the mount point, the sysctl file, the compact timer unit and the passwordless sudo rule are gone; the store is not mounted and the machine is out of the fleet. See `round-31.md`, section "Out of the Fleet Since 22:07 UTC".
- Open, needs root (the owner): activate `/home/semio-swap`, then run `🥾️provision/🐚️.sh g1 g1-m04 --user deck` and stop it with Ctrl+C when it prints `step 9: native bootstrap` (round 31: the machine runs that stage itself).

## Git Stage Identical to the Fleet Since 13:15 UTC on 2026-11-18 (Was `objects-missing` From 12:04 UTC)

- After the return the files were identical within minutes (manifest digest and file count equal to g1-m01, same commit `3d62b2ef22`), but the git stage was not: `Git state: waiting for objects of the shared stage`, `queue.incoming` 7206, git view 5067 modified / 13 deleted / 3570 untracked against 99 / 11 / 92 elsewhere. It did not heal in 70 minutes.
- Cause, local: `.git/objects/pack/multi-pack-index`, written by git on this machine at 03:38 UTC on 2026-11-18 (during the outage, a git client ran here), named three packs; the daemon removed two of them after the return (`repaired git local-packs 448`). `git multi-pack-index verify`: `failed to load pack in position 0`, `… position 2`. With it git could not read objects that the fleet pack `pack-2678b326…` holds: 162 folder trees of the stage base `a4d0762fc86b` (published by g1-server, hlc `1794923253279`) read as missing; `git -c core.multiPackIndex=false cat-file -t` found them.
- Done at 13:14 UTC: the stale `multi-pack-index` was moved out of the repository (to `~/.cache/semio-stale-multi-pack-index-20261118`; the file is never shared, README "Pack gate"). 30 s later: `repaired git objects-missing`, `Git state: shared`, stage tree `da285029bd71`.
- Before that, at 12:56 UTC, four objects of the stage tree were written from byte-identical local files (`git hash-object -w` for three blobs, `git mktree` for the tree of `.claude`); only objects, no file, stage or ref was changed. They were absent for the same reason or were never published; not determined.
- Check at 13:18 UTC (attestations): g1-m04, g1-m01, g1-m07 and g1-server all `indexTree da285029bd71`, manifest `fc8eb0a99486`, 318 549 files, commit `3d62b2ef22`, view 0 staged / 68 modified / 10 deleted / 40 untracked with the same digest `e1f3b499…`, `queue.incoming` 0; `status` verdict for g1-m04 `ok`.
- What this machine published after the return, outside its own folders: git objects, the ref file `.git/refs/remotes/origin/🐙ueli/⛳wip`, and two files of the ticket `☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END` that was opened on this machine during the outage. No stage register, no other path.

For the coordinator: a `multi-pack-index` that names packs the pack rescue then deletes leaves git unable to read objects of the remaining pack; the daemon reports `objects-missing` and waits without end. Removing the `multi-pack-index` together with the rescued packs would avoid it.
