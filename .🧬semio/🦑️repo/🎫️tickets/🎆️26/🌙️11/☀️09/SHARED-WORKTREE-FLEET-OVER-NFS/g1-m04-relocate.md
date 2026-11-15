# g1-m04 Relocation to ~/semio (Linux Worker, SteamOS)

Machine `ueli-workstation-lin-1`, account `deck`, origin `g1-m04` (unchanged). Old worktree `/srv/semio`, new canonical worktree `/home/deck/semio` (the account's existing repository). State of this record: 2026-11-14 02:35 CET, relocation step `finished`. Work ticket: `26/11/14/FINISH-CANONICAL-LINUX-WORKER-SETUP-ON-WORKSTATION-LIN-1`.

Exports for every run: `SEMIO_LANGUAGE=en`, `SEMIO_FLEET_GROUP=g1`, `SEMIO_FLEET_NFS_HOST=192.168.178.70`, `SEMIO_FLEET_NETWORK=192.168.178.0/24`, `SEMIO_FLEET_CACHE_ENDPOINT` unset.

## Before the Run: Store Unreachable for Seven Hours

The replica in `/srv/semio` had reported `store-unreachable … stalled` since 2026-11-13 17:46. The group server answered ping, portmap, mountd and a userspace NFSv3 `GETATTR`/`FSINFO` on a fresh connection within milliseconds (`🧰️scripts/nfs_probe.py` in the work ticket), while the kernel's one TCP connection to port 2049 had received nothing for 6.9 h although its segments were still acknowledged. A `hard` mount never reconnects in that state. `ss -K dst 192.168.178.70 dport = 2049` destroyed the socket; the kernel reconnected, retransmitted the pending calls (`nfs: server 192.168.178.70 OK`) and the daemon resumed by itself. No remount, no data lost.

## Preparation of ~/semio

- HEAD `5ececaa78b9`, working tree clean, same branch and tags as the fleet.
- Nine stashes existed only there. Lifted into loose refs `refs/old-repo/g1-m04/stash-0` … `stash-8` (`346c0d03ec8f`, `baa37988b33f`, `cff4f40c5c9f`, `07d9b7f6ab2f`, `f29e4b04a746`, `1d0baeff5768`, `7fa9110474e9`, `a266267110d7`, `01f2c114b15c`); they are present after the adoption, `refs/stash` is the fleet's.
- Copied the current `🤝️shared-worktree` module and `📚️library/🏗️build/🔣️policy.json` from `/srv/semio`, because the relocation runs the code of the new tree (the old checkout held 36 of 121 module files).

## Dry Run

`🥾️provision/🐚️.sh g1 g1-m04 --user deck --plan`, exit 0: 269344 identical, 1876 missing here (taken from the fleet), 1994 stale (the fleet wins, bytes kept), conflicts 0, new local work 0. Git: fleet tip and local tip `5ececaa78b92`, relation equal, 0 staged paths.

## Real Run

`🥾️provision/🐚️.sh g1 g1-m04 --user deck`, 00:55 to 01:25 CET, exit 1 at the `finish` phase only.

- Step 6a: `boot program: {"action":"current","version":3,"changed":true}`; `~/.local/share/semio/fleet/entry/worktree` names `/home/deck/semio`. The fault of g1-m02 and g1-m03 (entry kept the old tree) did not occur.
- `relocate` prepare: `prepared`. Move: `applied 5438`, `applied 3793`, `applied git 2275`, `published 36 (4135795515 bytes)`, step `adopted`.
- Adoption report `~/semio/.cache/f/lost/adopt-2026-11-13T23-56-30-523Z.md`: same 269341, materialise 1876, publish 0, edit 0, stale 1997, conflict 0, nested 0.
- Services started from the new path. `finish` inside the script: `The new worktree or the fleet does not agree yet`.
- `relocate --phase finish --wait 0`, second attempt (01:36): `The new worktree is verified (273181 paths checked). Relocation step reached: finished`. `/srv/semio/.cache/f/relocated.json` exists; the old tree is retired with synchronisation off.

## After the Run

- `verify --json` (01:40): `ok: true`, 273181 checked, drift 0, `refusedTotal` 87: the same 87 absolute symbolic links as on g1-m02 and g1-m03 (targets under `/Users/ueli/Documents/semio`, dangling here). Only the links were removed; the list is `removed-links.md` in the work ticket.
- `.git/info/exclude` held ten private rules written by Claude Code (`guards.infoExclude: 10 instead of 0` in `fleet`). Emptied to its comments; the old file is `~/semio/.cache/f/lost/manual/info-exclude-2026-11-14`.
- `preflight` (01:41), with the fix below: store mount, mount options, store writable, cache endpoint, clock, limits, disk (168.8 GiB free), daemon, compaction and access `ok`; `WARN native build: pending`; the seven git guard checks `skip`; replica status, file watcher and event visibility timed out after 15 s while the daemon's event loop was busy.
- 01:32 to 01:45: the daemon's event loop did not answer (one core at 94 %, 1300 reads of `/proc/self/statm` per second, no store I/O) and `systemctl restart` had to kill it (status 137). After the restart it attested within a minute and has stayed responsive. Cause not found.
- `fleet` (02:29): g1-m01, m02, m03, m06, m07 `ok`; g1-m04 `diverged` in `guards.infoExclude` (fixed above), one manifest bucket and `indexTree`. At that time a repack of the shared git objects was in transit (`.git/objects/pack` here: index of `pack-afe8dcf2…` without its pack, `git-index.json` `degraded: write-tree-failed`), so HEAD could not be read on this machine and g1-server was the only origin reporting a head commit.

## Fix in the Fleet Code

`🗺️topology/⌨️command/🟦️.ts`: the Linux store-mount check compared the literal store path with `/proc/self/mountinfo`, which names `/var/mnt/semio-store` on SteamOS (`/mnt` is a symbolic link). The check now also looks up the resolved path (probe `resolved`, `realpath`). One test case added; `🧪️tests/🟦️.ts`: 67 pass.

## Open

- Native build: pending after five provisioning runs (exit 3). The machine-side causes are solved (npm mirror of the group server bypassed with `SEMIO_FLEET_MIRROR=off`, graph memory limit); the last run fails on four `workspace:setup` tasks of the shared source. Details in `report.md` of the work ticket.
- `/srv/semio` (35 GB) is retired and holds no unsynchronised work: its ignored files outside `.cache` are generated outputs (330 entries). It was not deleted but moved to the Trash of the account (`~/.local/share/Trash/files/semio-srv-retired-g1-m04-20261114`), so that `/home/deck/semio` is the only repository path; the owner empties the Trash.
- Final checks (09:35): `preflight` 24 checks, none failed, one warning (native build); `verify` refused 0, drift 52 loose git objects in transit fleet-wide.
- Background scripts of sessions from 2026-11-02 to 2026-11-10 still run in `~/semio` (`cache-guard.sh`, `owners-sampler.sh` three times, `nice-others.sh`, `queue-sequencer.sh`, `phase-watch.sh`, two `until` loops); they write below `🗑️generated` only and were not stopped.
- Leftovers of the first join, unchanged: `/etc/sudoers.d/zz-semio-provision`, `steamos-readonly disable`, `/home/semio-swap` (not in fstab). The GitKraken Flatpak override now names `/home/deck/semio`.
