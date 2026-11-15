# g1-m04 Join (Linux Worker, SteamOS)

State on 2026-11-13 10:20 CET: the machine is a member of g1 and syncs. The native build is **not** complete: `.cache/f/native-bootstrap.done` is absent.

## Identity

| | |
|---|---|
| Host used | `192.168.178.70` (group server, macOS), export `/Users/Shared/semio-store` |
| Origin | `g1-m04` (store held `g1-server`, `g1-m01`, `g1-m02`, `g1-m03` before the join) |
| Machine | `ueli-workstation-lin-1`, SteamOS (Arch based, read-only root), `192.168.178.76`, 16 GB RAM |
| Account | `deck` |
| Worktree | `/srv/semio` (real directory, created by the script) |
| Mount | `192.168.178.70:/Users/Shared/semio-store` at `/mnt/semio-store`, `nfs vers=3,hard,noatime,actimeo=1,lookupcache=none,nconnect=4,nolock,_netdev,nofail` (fstab). The kernel lists it as `/var/mnt/semio-store` because `/mnt` is a symlink to `/var/mnt` on SteamOS |
| Write token | one `export SEMIO_REMOTE_CACHE_TOKEN=…` line in `~/.profile` (mode 0600), added by the operator; the bootstrap's PATH block and the cargo line in that file were kept |

Exports for every run: `SEMIO_LANGUAGE=en`, `SEMIO_FLEET_GROUP=g1`, `SEMIO_FLEET_NFS_HOST=192.168.178.70`, `SEMIO_FLEET_NETWORK=192.168.178.0/24`, `SEMIO_FLEET_CACHE_ENDPOINT` unset. Command: `bash <temporary copy>/🥾️provision/🐚️.sh g1 g1-m04 --user deck`. No `--server`, `--first-machine` or `seed`.

## Join and Verify Counts

- Archive: sha256 `5bcd4dd7…fc79`, 272356 files, verified and extracted.
- Settle: 272312 files already matched the store, 23 differing files moved to `/srv/semio/.cache/f/lost/bootstrap-1794529430211`.
- Join: `Joined: 273132 paths, 272320 already matched, 0 still queued.`
- Status (01:50 and 10:10): `Synchronization: on`, daemon running, 0 events behind every origin, `Git state: shared (stage and commits replicate)`.
- Verify: exit 0 at 01:52 (`No drift: 272417 shared paths`), exit 1 at 10:12 (one file, `🧰️scripts/1113-measure/samples.csv`, being written by another machine), exit 0 at 10:15 (`No drift: 272956 shared paths agree with the state and the store.`).

## Provision Runs

| Run | Exit | Stopped at | Cause | Fix |
|---|---|---|---|---|
| 1 | 1 | step 7, before join | The bootstrap archive did not contain the replica's own source (`🔁️replica/🟦️.ts`, `⌨️command`, `🧬️schema`, `📖️texts`): they were being edited on g1-server while the archive was built, and the archive skips files that do not match the manifest | Latest content of the `🤝️shared-worktree` module written from the store's blobs (`🧰️scripts/restore-module-from-store.py`); topology copy staged again from `/srv/semio` |
| 2 | 3 | native bootstrap | `pacman` could not verify packages (keyring never initialised on SteamOS), so `make` stayed missing | `pacman-key --init`, `pacman-key --populate archlinux holo` |
| 3 | 3 | launcher build | An empty leftover `/tmp/.git` made Go's VCS stamping fail (`error obtaining VCS status: exit status 128`) | `rmdir /tmp/.git` |
| 4 | 3 | dependency scopes | `bun.lock` out of date: root `overrides` declare `diff 8.0.4`, the lock said `^8.0.3` | `repo dependencies lock --scope .` (one line changed; shared file) |
| 5 | 3 | project graph | Duplicate key `semio-framework-value-testing-admission` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧰️fixtures/📦️packages/🦀️rust/Cargo.toml` | Duplicate line removed (shared file); all 5267 `Cargo.toml` files parse |
| 6 | 3 | project graph, after 8 h | `The plan cannot continue: the service holds 5700 MB of its 5700 MB memory limit (safety margin 256 MB), so it was cancelled to keep the service up` | open, see below |

Join ran in run 2; runs 3 to 6 skipped it.

## Native Error (Open)

`repo doctor` reports `No project graph generation is published`, so this machine has to build the graph cold once. The service log shows the graph plan at about 10 GB while `MemAvailable` is about 9.3 GB. The 9 GB of swap that made `ensure_swap` report enough memory is 7.7 GB zram (compressed RAM) plus a 1 GB file, both full; about 5 GB of RAM is held by files in the tmpfs `/tmp`.

A 16 GB disk swap file `/home/semio-swap` is active (priority 200, not in fstab). A direct `repo show projects` after that was stopped by the agent harness after 200 s because the system was low on memory. It was not started again.

Next step: free memory (close other sessions, clear old data in `/tmp`), run `repo show projects` in `/srv/semio`, then the provision command once more.

## Fleet

- 01:55, `fleet --wait 120`: exit 1. `g1-m01`, `g1-m02`, `g1-m04`, `g1-server` ok on code `a6bd6778a398`; `g1-m03 diverged` (manifest, head.commit, indexTree).
- 08:41 to 10:09: the replica daemon of this machine crash-looped (`Export named 'gitGuardsV1' not found in module …/🌿️git/🟦️.ts`), the failure described in `1113-worker-recovery-0841.md`. Repaired at 10:09 by writing the module's latest content from the store's blobs; the daemon then started by itself.
- 10:14, `fleet --wait 120`: exit 1.

```
  g1-m01 - - - - - - - missing (no attestation)
  g1-m02 - - - - - - - missing (no attestation)
  g1-m03 - - - - - - - missing (no attestation)
  g1-m04 linux 4s 0/0 on 37dde6043d2e 121/0/302 -/-|1478/2786 diverged
  g1-m05 - - - - - - - missing (no attestation)
  g1-m06 - - - - - - - missing (no attestation)
  g1-server darwin 0s 0/0 on 37dde6043d2e 120/0/189 336/1661|4064/4064 ok
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  g1-m04 differs in view.digest: 774835227706 instead of 5d368ddc2ec5
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

g1-m04 runs the same code as g1-server and is 0 events behind; it differs in the git view digest (121 modified and 302 untracked here, 120 and 189 on g1-server).

## Preflight

- 01:51: 14 checks, 1 failed, 1 warning. `FAIL store mount: nothing is mounted at /mnt/semio-store` although the store is mounted and writable: the check compares the literal path with `/proc/self/mountinfo`, which names `/var/mnt/semio-store`. `WARN native build: pending`.
- 10:16, with the fleet's current code: `No group: pass --group <id> or run the preflight on a machine that has joined (.cache/f/machine.json)`, although `machine.json` names `g1-m04` and `g1`. With `--group g1` it inspects `/home/deck/semio` (an old git checkout of this machine) instead of `SEMIO_WORKSPACE_ROOT=/srv/semio`.

## Findings for the Fleet Code

- The bootstrap archive can lack the replica's own source, and then no machine can join from it.
- `ensure_swap` counts zram as swap, and its swap file path `/var/lib/semio-swap` lies on a 230 MB partition on SteamOS.
- The launcher installer builds in `mktemp -d` without `-buildvcs=false`.
- The preflight mount check does not resolve a symlinked mount point.
- `/usr/local/bin/repo` and the pacman packages live on the SteamOS root image: `steamos-readonly disable` was needed, and an OS update removes them, so provisioning has to run again after one.

## Local Changes Outside the Script

- `/etc/sudoers.d/zz-semio-provision` (passwordless sudo for `deck`, by the operator): remove after the native build.
- `steamos-readonly disable`, pacman keyring initialised.
- `/home/semio-swap` (16 GB, active until reboot).
- Flatpak override so GitKraken can open `/srv/semio`.
