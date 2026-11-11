# Shared worktree fleet: macOS client, two group servers and the native bootstrap, on real machines

Date 2026-11-10. Follows `linux-nfs-report.md` (one Linux server, one Linux worker). Three gaps are closed here with real execution:
(A) two Linux group servers with real peer mounts, (B) the macOS host as a worker of a Linux group server over real NFS,
(C) the native `repo` bootstrap on a fresh Linux worker. Scripts of this run: `🧰️scripts/mixed-*.py|sh` next to this file
(the `nfs-*.py` helpers of the first report were reused).

## Verdict

* **A works.** Relay both ways over real NFS: group 1 to a worker of group 2 median 839 ms (p95 1,318 ms), group 2 to group 1 median 1,000 ms
  (p95 1,475 ms), 200 saves each, none overtaken. A 30 s stop of one NFS server, five `kill -9` of the relay in the middle of a copy and the `--archive`
  peer flow lost nothing; `verify` is clean on all three trees and the independent store check finds 0 problems.
* **B: an unprivileged NFS mount works on macOS 27**, and the macOS client is fine with the plan's options. Two defects were found and fixed:
  (1) **NFSv4 delegations made every second save of a macOS machine 4 to 8 s late** (median 4,270 ms, p95 8,252 ms, 49 of 200 saves overtaken): Linux
  group servers now set `fs.leases-enable=0` (median 347 ms, p95 477 ms, none overtaken after); (2) **a case-only rename (`readme.txt` to `README.txt`) was lost**: a
  Linux machine deleted the file on every other machine for up to 15 minutes, a macOS machine never published the rename at all (`🔁️replica`, regression test added).
  The macOS mount options of the plan are now **measured**, and unchanged.
* **C: `cargo` no longer fails; the bootstrap now stops one step later, in the repository's resident graph service.** The old exit 101 is `error: linker \`cc\` not found`
  (reproduced, gone with the toolchain that the script lists). `repo run workspace:setup` then fails on every fresh machine (three tried), in two layers: first
  `Cannot find module 'typescript'` (the planner needs what `workspace:setup` installs), then `Original captured runtime entry provider required` (native build storage,
  stack below). Not fixed (build code of the repository, not a minimal change); the completion marker was written by hand to finish the worker provisioning, which then passed
  all 12 preflight checks.

## Environment

| | |
|---|---|
| Host | macOS 27.2 arm64, 16 GB, 12 cores, no root. Nothing else ran (another job at load 130 in the first report); load average 1.8 to 3.8 during every latency run, 2.6 to 7.4 during the stat probes (noted per block). |
| Runner | Lima 2.3.0 (`lima-2.3.0-Darwin-arm64.tar.gz`, SHA-256 checked against the release `SHA256SUMS`), `vz` driver, `LIMA_HOME=$HOME/.semio-fleet-vm`. Image Ubuntu 24.04 arm64 `release-20261017`, kernel 6.8.0-147. |
| Network | Every VM got **two** networks: `vzNAT` (the host reaches the VM directly: 192.168.64.x, RTT 0.3 to 0.7 ms, no port forwarding needed) and `lima:user-v2` (VM to VM: 192.168.104.x, 0.2 to 0.6 ms). The group servers talk over user-v2, the macOS host over vzNAT. |
| VMs | `g1` group server 1: 4 vCPU, 4 GiB (3 GiB at first, see defect 8), 50 GiB. `g2` group server 2: the same. `w3` fresh worker: 4 vCPU, 4 GiB, 40 GiB. Never more than 8 GiB of VMs at once (`g2` was deleted before `w3` existed). |
| Tree | The 269,382 shared files of the repo (`scanSharedV1` on the host), copied with `tar --null -T` from the read-only Lima mount onto the ext4 of `g1`; the host repo was never written (no `.cache` there, confirmed at the end). |
| Topology | Copy of the topology directory with `g1` host `192.168.104.1`, `g2` host `192.168.104.3`, network `192.168.104.0/24`, caches on `:6090`; `g3` to `g5` keep their placeholders (their peer mounts fail on purpose). The committed JSON was not changed. |
| Accounts | `semio` uid 1500 (system account, home `/var/lib/semio`) on the servers, as in the runbook; `semio` uid 1000 (normal account, home `/home/semio`) on `w3`. |
| macOS worktree | APFS clone (`cp -c`, per file of the same list) under `$S/rehearsal/mac1`; joined to `g1` as origin `mac1` with `bootstrap-settle` and `join` (settle: 2 of the files differed from the store and went to `lost`: the topology JSON with the VM addresses and the debug patch of defect 9); the daemon ran from that clone. |

## A. Two group servers

Commands (in the VMs as root; emoji paths by glob): `apt-get install nfs-kernel-server nfs-common build-essential pkg-config curl git unzip python3 strace`,
`useradd --system --uid 1500 semio`, the `prepare` and token steps of the runbook, then

```sh
bash ./*.sh g1 g1-server --user semio --first-machine --server                                    # g1
mount -t nfs4 -o ro,vers=4.2 192.168.104.1:/srv/semio-store /mnt/semio-peers/g1                   # g2: peer mount before the archive step
bash /tmp/boot/*topology/*provision/*.sh g2 g2-server --user semio --archive /mnt/semio-peers/g1/v1/bootstrap --server
mount /mnt/semio-peers/g2                                                                         # g1: the peer that did not exist at its provisioning
fr join --group g2 --origin g2-w1 --worktree /srv/w2g --store /mnt/g2nfs                          # a worker of g2 over its own NFS export (loopback mount, plan options)
```

| Step | Observed |
|---|---|
| `g1` provisioning with seed | 2 min 1 s (user 30 s, sys 35 s): 269,382 files, 4.52 GiB seeded, units started; first archive built by the relay 3 min later (5.3 GB). |
| `g2` with `--archive` of the peer mount | 2 min 23 s: archive fetched over NFS (sha256 and file count checked), extracted, `relay --once` copied 94 MiB of the `g1` journal, `bootstrap-settle` (269,382 files matched, 0 differing), `join`, services started. The relay of `g2` then built its own archive (5.4 GB). |
| `g2` worker `g2-w1` | `join` into an empty directory over NFS 3 min; daemon start scan 20 s, no `published` event (no ping-pong). |

**Cross-group latency over real NFS** (`mixed-xlat.py`: one file edited every 2.7 s, the reader polls it every 10 ms; clock offsets between the machines measured before and after, below 5 ms):

| Path | edits | median | p95 | max | overtaken | note |
|---|---|---|---|---|---|---|
| `g1` tree to `g2` worker (relay of `g2` reads the peer mount of `g1`) | 200 | **839 ms** | 1,318 ms | 1,421 ms | 0 | host load 2.2 |
| `g2` worker to `g1` tree | 200 | **1,000 ms** | 1,475 ms | 1,556 ms | 0 | host load 2.2 |
| same two, after `fs.leases-enable=0` on the servers | 100 + 100 | 883 / 981 ms | 1,318 / 1,398 ms | 1,391 / 1,470 ms | 0 / 0 | no change for Linux clients |
| first `g1` to `g2` run: the peer mount of `g1` on `g2` had the NFS **defaults** (I mounted it by hand before the script; the script kept it) | 200 | 889 ms | **2,944 ms** | 3,674 ms | **91** | see defect 3 |

The relay polls once a second (`pollMilliseconds` times 4) and copies in its own cycle, so a copy waits 0 to 1 s plus 0.17 s for the publish and about 0.1 s for the apply: 0.8 to 1.0 s median is what the design predicts.
With the defaults on the peer mount the relay's `stat` of a replaced head came up to 3.6 s late (the Linux defaults finding of the first report, now seen on the relay path: `strace` of the relay shows
`statx` of the head unchanged for 3 cycles); the strace proof is in the session log, the script now warns about such a mount (defect 3).

**NFS server of `g1` stopped for 30 s** (`mixed-outage.sh`, `systemctl stop nfs-server`; during the outage `g1` and `g2` server trees made 30 edits each and the `g2` worker 10; hard mounts):
no daemon or relay restarted (PIDs unchanged, `NRestarts=0`), no `error` event. `g1` received the 40 files of `g2` during the outage (its relay reads `g2`), `g2` received the 30 of `g1` after the restart:
all three trees identical (70 files, same digest) **106 s after the restart** (136 s after the stop; the NFSv4 grace period of this kernel is 90 s).

**`kill -9` of the relay of `g2` in the middle of a copy** (`mixed-kill9.sh`, `mixed-kill-tmp.py`): `g1` wrote five bursts of 1,500 small and 6 large (2 MiB) files; the PID of the relay (from `systemctl show`, never by name) was killed
when `v1/tmp` of the `g2` store held +1, +1, +2, +3 and +4 in-flight writes (4 to 30 entries seen). Restarted by systemd each time (`NRestarts` 5), all three trees converged with 7,530 identical files; `nfs-store-check.py` on both stores:
277,683 events and puts, every blob present with the right size, the 7,560 blobs of the test re-hashed, **0 problems**; 31 stale temp files in `v1/tmp` of `g2` (cosmetic, as in the first report). `verify --json`: `g1` 22 s, `g2` server 38 s, `g2` worker 39 s,
all `ok:true`, 276,993 files checked, 0 drift.

## B. macOS host as a worker of `g1`

### B1. Can macOS mount NFS without root?

**Yes.** `mount -t nfs -o vers=4,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache,noresvport 192.168.64.4:/srv/semio-store "$S/mnt/store"` on a directory owned by the user
succeeded at the first attempt (`mount` shows `nfs, nodev, nosuid, nobrowse, mounted by ueli`); the Linux export needed `insecure` (no reserved port) and a line for the vzNAT network
(`/etc/exports.d/semio-host.exports`: `192.168.64.0/24(rw,sync,no_subtree_check,all_squash,anonuid=1500,anongid=1500,insecure)`); no port forwarding, no `mountd`, NFSv4 needs only port 2049.
Several mounts of the same export at different mount points with different options work too (used for the option comparison). No attempt to gain root was made.
`nfsstat -m` shows what the client really negotiated: **NFS 4.0** for `vers=4` (4.1 for `vers=4.1`; `vers=4.2` is not offered, the man page says minor 0 or 1), TCP, `rsize=wsize=32768`,
`readahead=16`, no `nconnect`, `callback` on, `acregmin/max=1`, `acdirmin/max=0`, `nonegnamecache` as given.

### B2. Join, daemon, cost

`bootstrap-settle` 405 MB peak, `join` 336 MB peak (`/usr/bin/time -l`), together about 1.5 min on the clone; the daemon scans 269,382 files on start (20 to 40 s). Idle (daemon with the plan's options, 100 s):
**1.9 % of one core (1.9 CPU s in 100 s), RSS 299 MiB** (363 MiB after the latency runs); the plan costs 65 NFS calls per second for four origins (see B3). The first report's Linux figure is 0.4 %: the macOS client revalidates
every name (`acdirmin=0`) where the Linux client with `lookupcache=none` revalidates the same names, at 4 calls per `stat` on both.

### B3. How late does a follower see a new head? (mount options, measured)

`mixed-attr-run.py`: `g1` replaces `h.json` by rename 100 times, 1.37 s apart; the macOS host polls it every 2 ms **with `stat` only** (what the replica does: it opens a head only when the stat changed; an `open` revalidates
the directory and hides a stale name cache: a first run in `both` mode showed 5 ms even for the defaults and was discarded). One mount per run, one after the other; load 2.6 to 7.4.

| macOS mount options | heads seen (of 100) | median | p95 | max | NFS calls per second for 4 heads polled 4 times a second |
|---|---|---|---|---|---|
| **plan**: `vers=4,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache` | **100** | **2 ms** | 4 ms | 4 ms | 65.4 |
| plan without `nonegnamecache` | 100 | 2 ms | 3 ms | 3 ms | not measured |
| `noac` | 100 | 2 ms | 4 ms | 4 ms | 79.1 |
| `actimeo=1` | 100 | 446 ms | 917 ms | 944 ms | 11.5 |
| `vers=4.1` plus the plan | 100 (both-mode run) | 2 ms | 4 ms | 5 ms | 67.4 |
| macOS defaults (`vers=4`) | **28** | 672 ms | 1,292 ms | 1,306 ms | 0.0 (cached) |

Conclusion: the options of the plan are right and stay (`mount_nfs` has no `lookupcache`; `acdirmin=acdirmax=0` is what revalidates the name; `noac` is as fresh and costs 20 % more calls; `actimeo=1` is cheap and 0.4 to 0.9 s late;
the defaults miss two thirds of the heads, the Linux finding of the first report). `nonegnamecache` did not matter for the head and stays for blobs that appear later. The README table and the doc comment in `🗺️topology/🟦️.ts`
now say "measured on macOS 27" with these figures. **`preflight` on the real macOS output**: for the plan mount `ok mount options: the correctness options are in effect: hard,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache`
(the `mount` line `... (nfs, nodev, nosuid, nobrowse, mounted by ueli)` and the block of `nfsstat -m` parse); for a default mount `FAIL mount options: wrong options in effect: acregmin=1 (found acregmin=5), acregmax=1 (found acregmax=60), acdirmin=0 (...`.
The other checks failed as they must on that host (topology addresses of the VM network, no LaunchDaemon, no cache reachable): 11 checks, 5 failed, 2 warnings.

### B4. End to end, macOS to Linux and back

`mixed-xlat.py`, one file edited every 2.7 s, 200 edits each way, plan mount, host load 1.9 to 3.8 (clocks: the macOS host and `g1` differ by 1 to 5 ms, measured with `nfs-clock.py`):

| Path | run | median | p95 | max | overtaken |
|---|---|---|---|---|---|
| `g1` tree to macOS tree | with delegations | 309 ms | 436 ms | 996 ms | 1 |
| macOS tree to `g1` tree | with delegations | **4,270 ms** | **8,252 ms** | 9,965 ms | **49** |
| `g1` tree to macOS tree | `fs.leases-enable=0` | **324 ms** | 436 ms | 449 ms | 0 |
| macOS tree to `g1` tree | `fs.leases-enable=0` | **347 ms** | 477 ms | 1,397 ms | 0 |
| `g1` to macOS, macOS **defaults** mount, 60 edits | `fs.leases-enable=0` | 1,470 ms | 2,813 ms | 2,842 ms | 25 of 60 |
| macOS to `g1`, macOS defaults mount, 60 edits | `fs.leases-enable=0` | 327 ms | 489 ms | 949 ms | 0 |

Linux to Linux in the first report: 305 to 388 ms median; the macOS client is within 40 ms of it.

**Defect 1: the 4 s stall.** The daemon's own `published` event came 0.2 s or 4.2 s after the edit (bimodal, never in between); `fs.watch` delivered the event after 12 ms; the main thread was idle in `kevent` (`sample`); a
bun script that `stat`ed `head.json` in a loop on the same mount showed `stat head mac1 3,939 ms` the moment the daemon replaced it; the same stall with `nocallback` on the macOS mount; **none while the relay of `g2` was stopped**
(published 205 to 219 ms) and none after `sysctl fs.leases-enable=0` plus a restart of `nfs-server`. `/proc/fs/nfsd/clients/*/states` showed read delegations on the heads for the other clients (the relay of `g2` polls them).
The Linux server answers `NFS4ERR_DELAY` to the macOS client's rename-over while it recalls those delegations, and the macOS client retries after about 4 s (a Linux client retries after 100 ms, which is why the first report never saw it;
a micro-benchmark with only one client touching the file shows no stall either). Fix: `server-plan --part sysctl` (`fs.leases-enable=0`, `/etc/sysctl.d/91-semio-fleet-server.conf`), applied and read back by the provisioning script **before** the NFS server starts.
Linux latencies did not change (table in A).

### B5. Bursts and structure changes (`mixed-bulk.py`, plan mount, `fs.leases-enable=0`, host load 4 to 5)

| Operation | `g1` to macOS | macOS to `g1` |
|---|---|---|
| 2,000 small files created, all present with the right bytes | 1,509 ms | 3,101 ms |
| the 2,000 files deleted (directory gone) | 692 ms | 553 ms |
| directory with 500 files renamed (`src` gone, `dst` complete) | 496 ms | 1,016 ms |
| exec bit set | 229 ms | 376 ms |
| path of 256 emoji code points (1,003 bytes) | not run: with the 190-byte path of the rehearsal worktree the absolute path exceeds the 1,024 bytes of macOS `PATH_MAX` (at `/Users/Shared/semio` it is 1,023 and fits by 1 byte; not tried) | |

(The first macOS-to-`g1` rename run failed because the previous direction had left `nfs-rename/dst` behind: a test artefact, repeated after cleaning.)

### B6. Names and metadata (`mixed-names.py`, mac and `g1` trees, replicas on the same store; the first version of the script forgot `mkdir -p` on the Linux side and was fixed)

| Case | Result |
|---|---|
| file created on macOS with a decomposed name (`cafe` + U+0301, which APFS stores as given) | arrives on Linux as the composed name (`636166c3a9...`), after 0.7 to 4.5 s; the macOS file keeps its decomposed name locally; the store holds exactly **one** `put` for it, no ping-pong (head counters of both origins quiet after the test) |
| file created on Linux with a composed name | arrives on macOS composed |
| `refused ... not-nfc` for the decomposed file | printed by the **full scan** of the macOS daemon (the walk lists the real, decomposed name; the watcher normalises), then the state of the path is dropped silently; observed, not changed |
| exec bit both ways | Linux `755` shows as `0o755` on macOS, macOS `755` as `755` on Linux |
| symlinks both ways | `l-link -> target.txt` and `m-link -> target.txt` arrive intact |
| case-only rename, Linux to macOS (`readme.txt` to `README.txt`) | **before the fix: the file disappeared on macOS** (delete published, put withheld) and came back 15 min later with the next full scan |
| case-only rename on macOS (`notes.txt` to `NOTES.txt`) | **before the fix: never published**, Linux kept `notes.txt` for more than an hour and three full scans |
| `Dup.txt` and `dup.txt` created on Linux | both kept on Linux, only one on macOS (queued as `case-collision`, as documented); after the fix both sides converge on one `Dup.txt` (last writer's content), quietly |

**Defect 2: case-only renames.** In `observe` the scan skips a touched path whose case-folded twin is live in the state (`replica.folds.find(path)`), even when the twin is the old name that has just been renamed: on a case-sensitive
machine the delete of the old name was published and the put of the new name was dropped until the next full scan (`scanSeconds`, 900 s, so up to 15 minutes in which every other machine had deleted the file); on a case-insensitive machine
the old name still answers `lstat` (it is an alias of the file), so the old entry was neither deleted nor the new one published (full scan: `forget` without an event, and the put queued as a collision on every receiver). Fix in `🔁️replica/🟦️.ts`: a helper `listed`
(the exact name is in the directory listing, NFC-folded); `observe` treats the twin as renamed when it is not listed and reports it as missing, and the deletion check treats a path that answers `lstat` but is not listed as deleted. Regression test
`a case-only rename is published as one put and one delete, from a watcher event and from a full scan...` (fails on the old code, runs on APFS here, on a case-sensitive file system the same assertions hold); live: after the fix both case renames settled in 0.5 to 0.6 s on both systems.

### B7. NFS server of `g1` stopped for 30 s with the macOS replica running (`mixed-outage-mac.sh`)

The macOS daemon and the `g1` replica each created 30 files during the outage. The daemon did not exit or restart (same PID, no `store-unreachable` event: the hard mount blocks), the `g1` replica had `NRestarts=0`.
After the restart both trees held the same 60 files with the same digest **96 s after the restart** (126 s after the stop; NFS grace period 90 s).

## C. The native bootstrap on a fresh Linux worker

Fresh Ubuntu 24.04 VM `w3` (4 vCPU, 4 GiB). First run of `bash ./*.sh g1 w3 --user semio` without the build tools: stopped at once with the designed message (`semio has no passwordless sudo ... these tools are missing: cc c++ make pkg-config unzip:
install them as root first (apt-get install -y build-essential pkg-config curl git unzip)`). After exactly that command the same call ran 4 min 16 s (archive over NFS, extraction, launcher, Node 24.15.0, Rust nightly-2026-07-20, .NET 8.0.421, uv 0.11.6,
Go 1.26.9) and failed in the last step:

| Step | Result |
|---|---|
| `repo run repo-mcp:build` / `:install` | **passes**: `Compiling semio-repo-watch ... Finished release profile` (cargo, 10 `unexpected cfg` warnings), `Installed native repository command: /home/semio/.local/bin/repo` |
| `repo run workspace:setup` | **fails**: `PlanRefusedErrorV1: The resident service met an unexpected failure (reference 36779741)` |

**The old `cargo failed (101)`:** with the pinned nightly and no C compiler, `cargo build` of a trivial crate prints `error: linker \`cc\` not found` and exits 101; with `cc` on the PATH it builds. So the first report's failure was the missing toolchain
(`apt-get` was refused for the unprivileged account), which the script now stops on before the bootstrap. The packages the script lists (`build-essential pkg-config curl git unzip`) are enough for the bootstrap up to the failure below; the repository's own
longer list (Pango, Cairo, ICU, fonts, LLDB) is for later native builds, not for the bootstrap, and was not needed.

**Why `workspace:setup` fails** (the log of the resident service drops its reason: `runHostForegroundV1` is started without a `log`, so `plan-failed` is never written; I patched the log into the host of the VM tree, outside any joined tree, and reverted it):

1. On `w3` (nothing above `/srv/semio` has a `node_modules`): `plan-failed 36779741 Error: Slice worker exited with code 75`, and in the host log `Cannot find module 'typescript'`. The slice worker of the planner imports `typescript`; `workspace:setup`
   is the step that installs it. `/srv/semio/node_modules` holds only the four symlinks that the launcher makes (`@semio-tech`, `semio-framework-*`).
2. With `typescript` resolvable (on `g1` and `g2` it was, through the `/srv/node_modules` that I had copied there for the tests; on `w3` after copying it): `plan-failed f6270450 Error: Original captured runtime entry provider required`, three machines, same reference, every time:
   ```
   at captureRuntimeEntryV1  (🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🚪️entry/🟦️.ts:34:266)
   at <anonymous>            (🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:178:3353)   (prepare, branch `current !== "mounted"`)
   at async <anonymous>      (.../🧱️fixed/🟦️.ts:178:8115)
   at async <anonymous>      (🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:449:202)
   ```
   `captureRuntimeEntryV1` needs the `AsyncLocalStorage` provider that `withCapturedRuntimeEntryProviderV1` sets, and the only caller outside tests is `🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts:24` (a command admitted through the door). The plan request that the
   resident host serves reaches the preparation of the fixed native-build storage outside that wrapper. The first `repo run` of a fresh machine works because no host is running yet (it runs in the process of the command and then leaves the host running);
   every later `repo run` goes through the host and fails. `SEMIO_BUILD_SERVICE=off` and `--service=off` do not help (the plan client always asks the host); a 4 GiB VM with the group-server daemons running does not even get that far: `[capacity] waiting ... not enough free
   memory (need 1183 MB; 1044 MB spare above the 1536 MB floor)` (the capacity gate of `repo`; defect 8).

Not fixed: this is the build framework of the repository, there is no minimal and clearly correct change, and I could not tell whether a macOS machine reaches the same state (it has a working `repo` already). **The marker `.cache/f/native-bootstrap.done` was therefore written by hand** to run the rest of the
provisioning on `w3`, which then ran as designed: `bootstrap-settle` (269,477 files matched), `join` (1 min 31 s; see defect 6), units and timer started, and `preflight` on the real Linux output: **12 checks, 11 ok, 1 warning (25.9 GiB free below the recommended 30), none failed** (`mount options: hard,lookupcache=none,...`,
cache `/healthz` in 5 ms, clock +0.4 s of the server, inotify limits, daemon, compaction timer, status 269,385 files and 4 origins). `repo --help` and `repo --version` run (launcher 1.0.0, bun 1.4.2) from the worktree; `node`, `cargo`, `rustc`, `dotnet`, `uv` run.

## Defects and what changed

Files: `🔁️replica/🟦️.ts` and its test (defect 2), `🗺️topology/🟦️.ts`, `🗺️topology/🥾️provision/🐚️.sh`, `🗺️topology/🧪️tests/🟦️.ts` (defects 1 and 3 and the macOS comments), `README.md` of the module, `🧰️scripts/mixed-*` (new). Schema, fixture, Python oracles, JSON and the PowerShell script are unchanged (they do not mention these keys).

| # | Defect | Fix | Test |
|---|---|---|---|
| 1 | NFSv4 delegations: a macOS writer stalls 4 s whenever another client holds a delegation on the head (median 4,270 ms, p95 8,252 ms, 49 of 200 saves overtaken) | Linux server plan: `sysctl-target` and `sysctl` parts (`fs.leases-enable=0`); the script installs the file, applies it and reads it back before it starts `nfs-server` | topology: key list of the server plan, new test on the script order |
| 2 | Case-only rename lost (Linux) or never published (macOS) | `listed` helper, `observe` and the deletion check in `🔁️replica/🟦️.ts` | replica: `a case-only rename is published as one put and one delete...` (fails without the fix) |
| 3 | An already mounted peer keeps the options it was mounted with (here the defaults: relay 3 s late, half the saves of the other group skipped) and the script said only `already mounted` | the script warns when an fstab peer line has `lookupcache=none` and the mount does not (`findmnt`) | topology: script test |
| 4 | macOS options of the plan were by reasoning | measured (B3); options unchanged; README table, doc comment and test title say "measured" | topology test title |

Suites on the host after the changes (cwd the repo, `bun test`): journal 126 pass, boundary 38 pass, topology 62 pass (61 plus one), replica 73 pass (72 plus one), replica bootstrap 11 pass; `shellcheck -s bash` on the provisioning script clean.

Observed and **not** changed:

5. `repo run workspace:setup` fails on fresh machines (C above) and the resident service never logs the reason of a `plan-failed`.
6. `join` exits 1 when entries stay queued (here three case-colliding paths of my own tests in the store); the provisioning script has `set -e` and stopped **silently** after `Joined: ... 3 still queued` (step 7). Cleaning the store made the rerun pass.
7. The macOS full scan reports `refused ... not-nfc` for a decomposed name that the watcher published (above).
8. `repo`'s capacity gate needs 1,183 MB above a 1,536 MB floor: on a 3 GiB VM (and on a 4 GiB group server with its daemons) a `repo run` waits for ever. Worker minimum in the README is 8 GB, which is fine; a small group server is not.
9. A temporary debug patch of the host service (a `log` function) that I made on `g1` was published by its replica into the store (shared source file) and reached the macOS clone; it was reverted the same way. The topology JSON with the VM addresses is likewise a shared file in that store (and in the macOS clone, never in the host repo).

## Unverified

* A macOS group server, NFS 3 on any client, the Windows clients (no machine).
* The macOS mount made by the LaunchDaemon as root (here: by hand, unprivileged, `noresvport`, against `insecure`); a macOS replica as a service under launchd; the real canonical path `/Users/Shared/semio` (the clone lived under a 190-byte scratch path, which also kept the 256-emoji path test from running).
* The native bootstrap to its end (C): it needs a change in the repository's build framework; the marker was written by hand. Whether a fresh macOS machine hits the same `workspace:setup` failure was not tried.
* Quiet-production latencies with 10 to 50 machines per group; only two machines per group were involved. The Linux server ran with `fs.leases-enable=0` only after the macOS test; its effect on 50 Linux clients is not measured (the poll of Linux clients showed no change at two).
* The other distributions, SELinux, a firewall, a real NIC (user-space switch and vzNAT only), `nconnect` on a real network, ext4 versus xfs.
* `fs.leases-enable=0` also switches off file leases for Samba on the same machine (a group server runs none).
* The case-only rename fix on a case-sensitive file system rests on the Linux VM run (the two live cases) and on the reasoning in the test; the unit test itself ran on APFS only.

## Cleanup

macOS daemon stopped by its recorded PID, store unmounted, VMs `g1` and `w3` stopped and deleted (`g2` was deleted earlier), `$HOME/.semio-fleet-vm` and `$HOME/Library/Caches/lima` removed, the Lima download, `$S/rehearsal`,
`$S/mnt`, the file lists, tars and logs under `$S/fv`, the helper files, and the `/Users/ueli/Documents/node_modules` symlink deleted. The host repo has no `.cache` directory.
