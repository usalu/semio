# Shared worktree fleet on real Linux with kernel NFS: verification report

Date 2026-11-10. Scope: the Linux server and worker side of the fleet (provisioning script, topology module, replica daemon,
relay, cache server) on a real kernel, a real `nfsd` and a real NFS 4.2 client. Before this report the subsystem had only run
on macOS APFS with a local directory as the store. Raw logs: `$SCRATCH/lima/logs` (scratchpad of the session); reusable scripts:
`🧰️scripts/nfs-*.py` next to this file.

## Verdict

The Linux path works end to end: both provisioning flows ran from empty disks to running systemd services, the replicas
converge over the NFS mount, an NFS outage of 30 s and 11 `kill -9` of a daemon lost nothing, and the independent Python oracles agree
with the trees. Verification found **five defects, all fixed** (below); the most important is the NFS lookup cache, which made every
remote save appear about 0.4 s later than necessary (median 696 ms to 310 ms after the fix). The host ran another job at load averages of 3 to 130 on 12 cores; every figure names its load, and the headline figures were taken at load 3 to 8. Under load 30 or more the VMs
became 5 to 20 times slower, which says how a starved machine behaves, not what the design costs.

## Environment

| | |
|---|---|
| Host | macOS 27.2 arm64, 16 GB, 12 cores, no root. The host ran another agent's 50-daemon rehearsal throughout: load average 3 to 130 (noted per measurement). |
| Runner | Lima 2.3.0 (`lima-2.3.0-Darwin-arm64.tar.gz`, SHA-256 from the release `SHA256SUMS` checked), `vz` driver, `LIMA_HOME=$HOME/.semio-fleet-vm`. |
| Guest | Ubuntu 24.04.5 LTS arm64 (Lima template `ubuntu-24.04`, image `release-20261017`, digest pinned by the template), kernel 6.8.0-147-generic. |
| VMs | `srv` (group server): 3 vCPU, 2 GiB, 60 GiB ext4, 192.168.104.1. `wrk` (worker): 4 vCPU, 3 GiB, 70 GiB ext4, 192.168.104.3. Total 5 GiB. |
| Network | `vzNAT` gave each VM its own NAT: the VMs could not reach each other (ARP `FAILED`). Lima `user-v2` (user-space network, no root) connected them: RTT 0.05 to 0.2 ms. So the NFS traffic is real TCP between a real kernel server and client, but it crosses a user-space switch, not a NIC. |
| Software | Bun 1.4.2 linux-arm64 (`bun-linux-aarch64.zip`, SHA-256 from `SHASUMS256.txt` checked), Python 3.12.3, shellcheck 0.9.0, `nfs-kernel-server` and `nfs-common` from apt, Go 1.26.9 and the pinned Bun for the `repo` launcher (both fetched by the repository installer with their pinned SHA-256). |
| Trees | The shared files of the host repo (the output of `scanSharedV1` run on the host, 269,335 entries, 87 refused links) copied with `tar --null -T list` from the read-only Lima mount onto the VM's ext4 (77 s). The host repo was never written; no `.cache/f` appeared there. `ajv` from `$SCRATCH/deps/node_modules` was copied to `/srv/node_modules` of each VM. |
| Topology | A temporary edit of the topology JSON inside the VM tree only: `g1` host `192.168.104.1`, cache `http://192.168.104.1:6090`, network `192.168.104.0/24`. Groups `g2` to `g5` kept their placeholder hosts, so peer mounts failed on purpose (see 3). The committed JSON was not changed. |
| Accounts | `semio` uid 1500 on the server (the `prepare` part: `useradd --system --uid 1500 ... --home-dir /var/lib/semio`); `semio` uid 1000 on the worker, so the export's `all_squash,anonuid=1500` was exercised with a client uid that differs from the owner. |

## Commands (as run; emoji path parts are resolved by globs, `M=$(ls -d [!n]*/*/*/*/*shared-worktree)`)

```sh
# host
curl -sSL -O https://github.com/lima-vm/lima/releases/download/v2.3.0/{lima-2.3.0-Darwin-arm64.tar.gz,SHA256SUMS}; shasum -a 256 -c ...
export LIMA_HOME=$HOME/.semio-fleet-vm
limactl create --name=srv --vm-type=vz --cpus=3 --memory=2 --disk=60 --network=vzNAT --containerd=none template:ubuntu-24.04   # wrk: 4 CPUs, 3 GiB, 70 GiB
limactl edit --set '.networks = [{"lima":"user-v2"}]' srv wrk     # vzNAT VMs cannot see each other
bun $S/lima/listshared.ts shared.list0                            # scanSharedV1 on the host repo: the file list
# in the VMs (root): apt-get install nfs-kernel-server (srv) nfs-common shellcheck unzip python3 time; bun 1.4.2 from the release zip (SHA-256 checked)
tar --null -T /tmp/shared.list0 -cf - | tar -xf - -C /srv/semio                 # copy of the shared files onto ext4 (77 s)
bun test "./$M/"*journal/*tests/*.ts    # likewise boundary, topology, replica, replica/bootstrap; python3 -I <oracle> ...
shellcheck -s bash "$M/"*topology/*provision/*.sh
bash "$M/"*topology/*provision/*.sh g1 g1-server --user semio --first-machine --server    # srv
bash /tmp/boot/*topology/*provision/*.sh g1 w1 --user semio                              # wrk (only the topology directory copied there)
repo ./*script.ts join --worktree /srv/w2 --store /mnt/semio-store --group g1 --origin w2  # likewise w3; run as systemd-run units
python3 nfs-xwrite.py / nfs-xread.py / nfs-clock.py / nfs-attr-probe.py / nfs-rpc-cost.py / nfs-bulk-run.py / nfs-kill-when.py / nfs-store-check.py   # scripts next to this file
systemctl stop nfs-server; sleep 30; systemctl start nfs-server                         # the outage
```

## 1. Test suites on Linux

`bun test` with the root `ajv` resolved from `/srv/node_modules`; the Python oracles with `python3 -I`.
Final run (log `tests-linux-final.txt`, module as of the end of this work):

| Suite | Result |
|---|---|
| journal | 122 pass, 1 fail: `a big document streams member by member...` hit the 5,000 ms test timeout at host load 78 (4,645 ms in the first run, which passed 122/122). |
| boundary | 37 pass, 1 fail: `the matcher handles the whole repository file and stays fast on many paths` took 7,458 ms at host load 80 (first run 32/32 pass). A timing threshold, not a logic failure. |
| topology | with the committed JSON: 26 pass, 13 skip (the PowerShell tests, no `pwsh`), 0 fail, oracle `check` rc 0 (`tests-linux-topology-committed.txt`). With the VM's edited JSON the two tests about the committed file fail, as they must. |
| replica | 52 pass, 0 fail (302 s at host load 80; 5 s in the first run). |
| replica bootstrap | 11 pass, 0 fail. |
| Python oracles | topology `check` (committed JSON) ok; journal 166 checks ok; boundary 71 vectors, 0 mismatches (the gitignore vectors are skipped: `pathspec` is not installed); replica `fold` and `conflicts` ok. |

First run (host load about 5, `tests-linux-1.txt`): journal 122/122, boundary 32/32, topology 22/22, replica 48/48, bootstrap 11/11, all pass.
The two timing failures of the final run were not repeated on a quiet host.

## 2. shellcheck

`shellcheck -s bash` 0.9.0 on the provisioning script: 8 × SC2015 (`A && B || fail`, benign because `fail` exits) and 2 × SC2086 (unquoted
`$STARTED`, a space-separated unit list). Fixed: the two-condition checks are grouped (`{ A && B; } || fail`), `STARTED` is an array.
Now clean (`logs/shellcheck-2.txt`); `bash -n` and `zsh -n` clean too.

## 3. Provisioning executed on Linux

Server: `bash 🥾️provision/🐚️.sh g1 g1-server --user semio --first-machine --server` (log `provision-server-2.txt`).

| Step | Observed |
|---|---|
| Sysctl | `/etc/sysctl.d/90-semio-fleet.conf` written, `sysctl -p` applied, `max_user_watches` 1,048,576 and `max_user_instances` 1,024 read back. |
| Exports | `/etc/exports.d/semio-fleet.exports` written; `exportfs -v` shows `/srv/semio-store 192.168.104.0/24(sync,wdelay,hide,no_subtree_check,anonuid=1500,anongid=1500,sec=sys,rw,secure,root_squash,all_squash)`. The four placeholder peers print `exportfs: Failed to resolve nfs.gN.semio.internal` and are skipped; the real line is exported. |
| fstab | Block `# >>> semio fleet` with the bind mount (`/srv/semio-store /mnt/semio-store none bind 0 0`) and four `nfs4` peer lines; the bind mount is up (`ext4`), the four peer mounts fail at once with `Failed to resolve server` (no 120 s hang) and the script says `WARNING ... retried at boot (nofail)`. |
| Units | Written: replica `@`, compact `@` service and timer, cache, relay. `systemd-analyze verify` on all of them: no output. `semio-fleet-compact@semio.timer` listed (next run 01:45 UTC). |
| Seed | `Seeded 269337 files (4.51 GiB)`; the whole script took 10 min 28 s (user 1:48, sys 2:10), of which about 9 min were the seed while the host was loaded; a dedicated seed on a calm host took 3 min 57 s (see 4). |
| Relay | Started with the replica; emitted one `store-unreachable` event per placeholder peer, stayed up; built the first bootstrap archive on start (5,335,480,320 bytes, 269,337 files, 3 min). |
| Cache server | **First attempt crash-looped** (defect 2). After the fix it listens on `0.0.0.0:6090`; from the worker `GET /healthz` returns `200 {"status":"ok"}` and an unknown path 404. `repo remote-cache serve` therefore works from the standalone launcher on Linux; the binary that the bootstrap would install was not needed. |
| Re-run | Idempotent: second run 3.2 s (`unchanged` for every file, `already mounted`, `join is skipped`), third run after the fixes 24 s; units that changed were restarted (`try-restart`), the others not. |

Worker: `bash 🥾️provision/🐚️.sh g1 w1 --user semio` with only the `🗺️topology` directory copied to `/tmp/boot` (log `provision-worker-1.txt`, `-2`, `-3-rerun`).

| Step | Observed |
|---|---|
| Archive | The store mounted read-only at a temporary directory (`mount -t nfs4 -o ro,vers=4.2`), `worktree.tar` copied over NFS, `sha256 ff9f6016... 269337 files` verified, extracted with the system GNU tar under `C.UTF-8`; the file count matched the manifest. Fetch + verify + extract together took under two minutes. |
| Native bootstrap | Ran as the unprivileged `semio`: `apt-get` refused (no sudo), so `make` and the C toolchain were missing (defect 5); Node 24.15.0, rustup 1.29.0 with the pinned toolchain, .NET SDK 8.0.421, uv 0.11.6 and the `repo` launcher installed fine. The last step `repo run repo-mcp:install` failed after 4 min 32 s (`cargo failed (101)`, the graph service never came up). Not diagnosed further (see Unverified). |
| After installing `build-essential` as root | The second run saw `repo` and went on: sysctl (limits applied), `/etc/fstab` line appended, `mount` of `/mnt/semio-store` (`nfsstat -m`: `vers=4.2,rsize=262144,wsize=262144,hard,proto=tcp,nconnect=4,timeo=600,retrans=2,sec=sys,noatime,acregmin=1,acregmax=1,acdirmin=1,acdirmax=1,lookupcache=pos,local_lock=none`), units, `bootstrap-settle` (`269337 files already match the store, 0 differing`), `join`, service start. 4 min 42 s, mostly settle. |
| Changed mount plan | After defect 3 the script was run again on the live worker: it replaced its own fstab line (`set in /etc/fstab: ... lookupcache=none ...`) and warned that the mounted store keeps the old options. |

## 4. Replica over real NFS

Setup: server replica `g1-server` (local store through the bind mount) and relay, worker replicas `w1` (`/srv/semio`, the systemd unit), `w2` (`/srv/w2`), `w3`
(`/srv/w3`), all three through the one NFS mount, as `semio`. `w2` and `w3` joined into empty directories with
`join --worktree` (no archive); they run as transient systemd units with `Restart=always`. Clock offset between the VMs
measured with `nfs-clock.py` (NTP-style, RTT 0.05 ms; the offset drifted by up to 70 ms during a run, so cross-VM latencies carry about ±40 ms).

**Seed, join, verify (peak RSS from `/usr/bin/time -v`/`-f`).**

| Operation | Wall | Peak RSS | Note |
|---|---|---|---|
| `provision --server --first-machine` (includes the seed to the local store, the runtime download and all service setup) | 10 min 28 s (user 1:48, sys 2:10) | not measured | host load 4 to 80, the VM was starved; the seed row below is the clean figure |
| `seed` alone, 269,335 files, 4.51 GiB, into a fresh local store on the server VM (`/usr/bin/time -v`) | 3 min 57 s (user 36 s, sys 40 s) | 279 MiB | host load 1 to 3; macOS APFS rehearsal: 46.8 s, 480 MiB. Store 4.9 GB. `snapshot`: 2.25 s, 443 MiB |
| `bootstrap-settle` + `join` of `w1` from the extracted archive over NFS (inside the 4 min 42 s run) | about 4 min | not measured | 269,337 files matched |
| `join` of `w2` into an empty directory over NFS | 5 min 35.8 s | 410 MiB | tree 5.9 GB written; macOS APFS: 33 s |
| `join` of `w3` into an empty directory over NFS | 2 min 6.0 s | 342 MiB | page cache warm |
| `verify --json` `w1` / `w2` / `w3` / server | 42.9 / 104.4 / 54.1 / 144.8 s | 365 / 296 / 310 / 319 MiB | all `ok:true`, 269,337 checked, 0 drift |
| daemon start scan | `w1` 53 s, `w2` 25 s, `w3` 16 s, server 40 s | 207 to 246 MiB | no `published` event (no ping-pong) |

Independent oracle (`python3 -I` replica oracle `compare /mnt/semio-store <tree> node_modules`, 288,000 store entries, 39 to 45 s, 690 MiB): `w2` and `w3`
`ok:true`, no drift. `w1` differs only by 20,920 `extra-local` files under `.cache/b/` (the build cache that the bootstrap made; ignored by the root `.gitignore`, never in the store).
The oracle of the bootstrap archive (`check STORE` on the 5 GB archive) was killed by the kernel OOM killer at 0.92 and 1.13 GiB RSS on the two VMs (2 and 3 GiB, daemons running): not run to the end.

**Sync latency** (one file edited, `nfs-xwrite.py`/`nfs-xread.py`: the reader polls every 10 ms; edits 1.3 s apart; 200 edits; mount options `actimeo=1,lookupcache=none`, i.e. the final plan):

| Path | Host load | median | p95 | p99 | max | missed |
|---|---|---|---|---|---|---|
| server tree (local disk, `g1-server`) to `w1` (NFS client) | 3 | **305 ms** | 424 ms | 442 ms | 466 ms | 0 |
| `w1` (NFS client) to the server tree | 3 | **361 ms** | 477 ms | 502 ms | 506 ms | 0 |
| `w1` to `w2` and `w3` (same machine, same mount), `live-latency.py` | 3 to 8 | **388 / 388 ms** | 535 / 529 ms | n/a | 3,978 ms (one spike, both readers) | 0 |
| same, first attempt | 20 rising to 89 | 676 / 693 ms | 2,445 / 2,469 ms | n/a | 4,280 / 4,201 ms | 0 |

The last row ran while the host load rose to 89: it shows what a starved machine does. macOS APFS (rehearsal, same code): 397 ms median, 460 ms p95 (20 edits).
Under load the VMs got slower by the same factor as everything else: two directions at once with the host at load 49 gave median 1,356 and 574 ms, p95 3,155 and 4,058 ms, and 30 to 40 % of the edits were
overwritten by the next one before they shipped (the reader never saw those versions; nothing is lost, the last one always arrives).

**NFS attribute and lookup cache** (the question of the rehearsal report). `head.json` is replaced by rename; the replica `stat`s it every 250 ms and re-reads it only when
the signature changed. Probe `nfs-attr-probe.py`: server replaces `h.json` atomically every 1.37 s (100 times), the worker polls every 2 ms on four mounts of the same export:

| Mount options | `stat` sees the new file after (median / p95 / max) | `open`+`read` sees it after |
|---|---|---|
| defaults (`vers=4.2`) | only 46 of 100 versions seen at all; 689 / 1,317 / 2,337 ms for those | 4 ms median |
| `actimeo=1,lookupcache=positive` (the committed plan before this work) | 505 / 960 / 1,006 ms | 4 ms |
| `actimeo=0` | 11 / 16 / 18 ms | 2 ms |
| `actimeo=1,lookupcache=none` | 10 / 15 / 19 ms | 3 ms |
| `actimeo=1,acdirmin=0,acdirmax=0` | 10 / 15 / 18 ms | 2 ms |

An `open` is always fresh (close-to-open); a `stat` is not: a cached name keeps pointing at the replaced file for up to `acdirmin`. RPC cost of the replica's
poll (2 origins × 4 polls/s, `nfs-rpc-cost.py`): `actimeo=0` 71 RPC/s, `acdirmin=0` 64 RPC/s, `lookupcache=none` 32 RPC/s (all `LOOKUP`), `lookupcache=positive` 20 RPC/s (not comparable: the three daemons shared that mount). End to end, server to worker, 100 edits 1.6 s apart, host load 6 / 11: `lookupcache=positive` median 696 ms, p95 1,173 ms, max 1,558 ms;
`lookupcache=none` median 310 ms, p95 426 ms, max 462 ms. **The mount plan now uses `lookupcache=none`** (Linux client options and Linux peer options, NFS 4.2 and 3; the README
explains the measurement). Unmeasured: macOS `mount_nfs` has no such option (`actimeo=1` only), Windows Client for NFS.

**Bursts and structure changes** (`nfs-bulk-run.py`; the writer is `w1` and the readers `w2`, `w3` and the server tree, or the writer is the server tree and the readers `w1`, `w2`, `w3`).
Calm host (load 3 to 4; the server VM was seeding another tree during the first run):

| Operation | `w1` → `w2` / `w3` / server | server → `w1` / `w2` / `w3` | macOS APFS rehearsal |
|---|---|---|---|
| 2,000 small files created, all present with the right bytes | 2.80 / 2.80 / 2.80 s | 1.19 / 1.18 / 1.18 s | 2.0 s |
| the 2,000 files deleted (directory gone) | 0.68 / 0.70 / 0.78 s | 0.64 / 0.63 / 0.64 s | 0.8 s |
| directory with 500 files renamed (`src` gone, `dst` complete) | 0.85 / 0.86 / 0.88 s | 0.49 / 0.49 / 0.49 s | 1.3 to 1.5 s |
| exec bit set | 355 / 354 / 325 ms | 243 / 244 / 200 ms | 274 to 620 ms |
| path of 256 code points, all 4-byte emoji (1,003 bytes relative, 1,014 absolute, three directories and a file name, components of 252 bytes) | 388 / 388 / 556 ms | 397 / 397 / 366 ms | not tried |

The same operations with the host at load 7 to 60 (`bulk-w2s*.txt`, `bulk-s2w*.txt`): burst 9.1 s (`w1` → others, load 7) and 4.4 s (server → others, load 15); delete 1.2 to 1.4 s; the 500-file rename 1.3 to 1.5 s at load 7 but 18 to 31 s
at load 30 to 60; the long path 0.3 to 10 s. With the host starved every number grows by the same factor, the 500-file rename most (two events of 500 each per follower, three followers on 4 vCPUs). At load 78 a 2,000-file burst
was not complete after 180 s on the first attempt. A path of 320 code points was silently not shared (policy limit 256 code points; no journal message, `status` does not list it).

**Idle, 2 min** (`nfs-sample.py`, `/proc`, host load 4): `w1` 0.43 %, `w2` 0.46 % of one core, `w3` 11.6 % because its 900 s full scan fell into the window (13.9 CPU s, about
1.5 % averaged over 900 s); RSS 207 to 229 MiB. Server replica 0.37 % / 218 MiB, relay 0.08 % / 82 MiB, cache server 0.12 % / 117 MiB.

**inotify.** The tree has 154,899 directories (including empty ones and the `node_modules`/`.cache` directories that `repo` makes). Watches in use per daemon (counted in
`/proc/<pid>/fdinfo`): 154,895 (`w2`, `w3`), 157,843 (`w1`), 154,914 (server), one inotify instance each; limit 1,048,576 (per user: three daemons hold 468k together).
The kernel memory behind them was visible: with three full-tree daemons `Slab` was 996 MB of 2.9 GB (`ext4_inode_cache` 736 MiB, 468,010 `fsnotify_mark_connector`).

**NFS server stopped for 30 s** (`outage.sh`; `systemctl stop nfs-server`, 30 s, `start`; during the outage `w1` made 30 edits, `w2` 10 and the server tree 30; `hard` mounts):
no daemon exited or restarted (PIDs unchanged, `NRestarts=0`), no `error` event, no `server not responding` in `dmesg`; the worker daemons simply blocked on the mount.
After the restart all four trees held the same 70 files with identical digests **67 s after the restart** (97 s after the stop). The NFSv4 grace period of this kernel is 90 s
(`/proc/fs/nfsd/nfsv4gracetime`), which explains most of the wait.

## 5. `kill -9` in the middle of a publish

`w2` (a transient unit with `Restart=always`, `RestartSec=2`).
(a) Six kills at 0.15 to 3 s after the start of a burst of 1,500 small and 8 large (2 MiB) files; the head had not moved yet, so these hit the scan/hash phase.
(b) Five kills triggered by `nfs-kill-when.py` the moment `head.json` of `w2` had grown by 1, 30, 200, 700 and 1,200 events (a sixth, +1,600, never fired), i.e. inside the publish. The daemon came back after about 2 s each time.
After both series: 9,048 + 9,036 files, identical digests on `w1`, `w2`, `w3` and the server tree (converged 113 s and 5 s after the last restart);
`nfs-store-check.py` (independent): 3 origins, 295,173 events, 292,969 `put`s, every blob present with the right size, the 10,436 blobs written during the test re-hashed and all equal their names,
every segment complete up to `head.bytes`, **0 problems**. Left over: 23 files of 62 to 212 bytes in `v1/tmp` from the killed writes (a stale-temp sweep does not exist; cosmetic) and no corrupt line.
`verify --json` after the kills: `w2` and `w1` `ok:true`, 288,002 files checked, 0 drift, 30.7 s / 31.1 s, 360 / 358 MiB. `compact` (the systemd service, `--collect-blobs` on the server) ran next to the running daemons afterwards and removed nothing (nothing is older than 14 days); the 23 stale `v1/tmp` files stayed, and the store check after it was clean.

## 6. Defects found and fixed

All edits are under `🗺️topology/` or the README; nothing in `🔁️replica/` was touched.

1. **The topology script could not run at all.** `📜️script.ts` called `receiveScriptProcessInvocation`, which needs the invocation envelope of the build door; the provisioning script
   runs `repo ./📜️script.ts facts|env|mount-plan|server-plan|worker-service` on a machine with only the launcher, so the very first call died with `Original parent Script process invocation required`
   (also reproducible on macOS with `bun ./📜️script.ts validate`). Rewritten like the replica script (`isMainModuleV1`, `runTopologyScriptV1` exported, exit code 1 and a message on a refusal). Test added.
2. **No language on a fresh Ubuntu (`LANG=C.UTF-8`).** The launcher refuses to start without a language (`No language selected for texts`): the provisioning stopped at its first `repo` call, and the cache unit crash-looped
   (exit 2, restart counter 17 in a minute). `run_as` now passes `SEMIO_LANGUAGE` (default `en`), the Linux cache unit has `Environment=SEMIO_LANGUAGE=en`. Tests added. (The replica daemon needs nothing: it prints both languages.)
3. **`lookupcache=positive` delayed every remote save** by up to `acdirmin` (505 ms median, 960 ms p95 for a `stat` of a renamed head; with the NFS defaults a third of the versions were never seen). All four Linux option strings now use `lookupcache=none`; test and comment updated. Median
   end-to-end 696 ms → 310 ms.
4. **Re-provisioning with a changed mount plan failed** (`/etc/fstab already has a different entry`). The worker now replaces its own line (same source and mount point) and refuses only a foreign source; it warns that a mounted store keeps its old options (`remount` cannot change `lookupcache`). Tested live.
5. **A worker account without sudo failed silently and late**: the bootstrap's `apt-get` was refused, the toolchain was incomplete, and the run ended 4.5 min later in `cargo failed (101)`. The script now stops before the bootstrap when the account has no passwordless sudo and `cc c++ make pkg-config curl git unzip` are missing, with the install command. Test added.
6. shellcheck findings (2).

Files: `🗺️topology/📜️script.ts`, `🗺️topology/🟦️.ts` (options, cache unit, doc comment), `🗺️topology/🥾️provision/🐚️.sh`, `🗺️topology/🧪️tests/🟦️.ts` (four tests, one expectation), module `README.md` (Linux/NFS parts), `🧰️scripts/nfs-*.py` (new).
Schema, fixture, Python oracle, JSON and the PowerShell script are unchanged (they do not mention mount options). Host topology suite after the change: 26 pass (`🟦️.ts`), 35 pass with the PowerShell file.

## 7. Other observations (not changed)

- `useradd --system --uid 1500` (the `prepare` part) prints `uid 1500 is greater than SYS_UID_MAX`; harmless.
- `exportfs` prints one `Failed to resolve` line per unknown peer host; the rest of the file still takes effect.
- Every replica tree gets a `node_modules` of symlinks from the launcher's `Host.Prepare` (ignored by the root `.gitignore`, so never shared).
- `repo remote-cache status` runs a source admission of the whole tree before it prints; do not call it from a health check (`/healthz` is the cheap probe).
- The stale files in `v1/tmp` after a kill (above) and the silent refusal of over-long paths are replica-core behaviours; they are listed, not fixed (another agent owns `🔁️replica/🟦️.ts`).
- Seed and join are 5 to 10 times slower than on macOS APFS in these VMs (2 GiB and 3 GiB, ext4 on a sparse image, host swapping, user-space network); do not read them as NFS cost.

## 8. Unverified

- The native bootstrap to its end: `repo run repo-mcp:install` failed (`cargo failed (101)`) after the toolchain was installed without root; it was not rerun with `build-essential` present. A second run of the script skips a failed bootstrap because `repo` exists.
- Peers: real `g2`..`g5` servers (the relay's cross-group copy and the `--archive` peer flow were not run; their hosts did not exist).
- More than one NFS client machine: the worker's three replicas share one mount; the second client was the server's own local disk. Attribute-cache effects between two NFS clients are not measured (they follow from the probe).
- Quiet-host latencies for the structure operations, and the two timing tests (journal, boundary) under a normal load; `compact` with something to remove next to a running daemon (it ran next to the daemons and removed nothing; the timer is installed and listed, it did not fire).
- Other distributions (dnf, zypper, pacman branches), NFS v3 (macOS or Windows servers), SELinux, a firewall (none was active), `nconnect` behaviour on a real NIC, ext4 vs xfs on the server, a non-root worker account with passwordless sudo.
- Windows and macOS clients of the new `lookupcache` finding.

## Cleanup

Both VMs stopped and deleted (`limactl delete`), `$HOME/.semio-fleet-vm` and `$HOME/Library/Caches/lima` removed, and the Lima and Bun downloads, the file list and the tar under `$SCRATCH/lima` deleted; only the small logs and helper scripts remain there. Only processes of Lima were started and all of them ended with the VMs. `/Users/ueli/Documents/node_modules` (symlink to the scratch dependencies) was left in place. The host repo has no `.cache` directory.
