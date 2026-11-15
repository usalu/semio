# g1-m05 Origin Collision (mac-mini-m4-02)

Interim record. The final `<origin>-join.md` follows once the origin of this machine is decided. No tokens in this file.

`repo` MCP was not registered in this session (the `repo` binary did not exist before provisioning), so the existing fleet ticket was used in place of `ticket_reopen`.

## Machine

- Host `ueli-workstation-mac-mini-m4-16gb-256gb-02-lan`, addresses `192.168.178.81` and `192.168.178.82`, account `ueli`, macOS 27.2.1.
- Group server `192.168.178.70`. `showmount -e` lists `/Users/Shared/semio-store` for `192.168.178.0`; `http://192.168.178.70:6090/healthz` answers `ok`.
- Exports: `SEMIO_LANGUAGE=en`, `SEMIO_FLEET_GROUP=g1`, `SEMIO_FLEET_NFS_HOST=192.168.178.70`, `SEMIO_FLEET_NETWORK=192.168.178.0/24`, `SEMIO_FLEET_CACHE_ENDPOINT` unset. No `--server`, `--first-machine`, `seed`. Topology not edited.
- Worktree `/Users/Shared/semio` (was absent, not a symlink). Store mount `/Volumes/semio-store` from `192.168.178.70:/Users/Shared/semio-store`.

## What Happened

| Time (CET, 13 Nov) | Event |
| --- | --- |
| 03:53 | `v1/origins` listed `g1-m01` … `g1-m04`, `g1-server`. `g1-m05` was free and was chosen. |
| 03:53 – 04:12 | Bootstrap archive copied as the account (12 GB over NFS), sha256 `8451c1c0…213d51` equal to `manifest.json`, 272934 files. |
| 04:15 | **Another machine created `v1/origins/g1-m05`** (`home.json` 04:15:46, first events 04:33, attestation 04:44:58 with 272426 files and `sync: false`). `g1-m06` also appeared later. |
| 04:47 | Provision run 1 (administrator prompt, `--archive` of the local copy): archive extracted, launcher built, then exit **1** at `mount_nfs … Operation not permitted`. Cause: the `store-mount` LaunchDaemon (RunAtLoad) mounted `/Volumes/semio-store` between the script's own `mount \| grep` check and its `mount_nfs`. |
| 04:52 | Provision run 2: `bootstrap-settle` moved 591 differing files aside, then `join` exited **1**: `Cannot find module '../../🎣️tiers/🟦️.ts'`. The archive of 03:37 predates the new `🎣️tiers` module; settle only replaces files that differ, new files arrive with join, and join could not load. |
| 04:55 – 05:15 | Missing files only (`tar -xk`) taken from the refreshed store archive (sha256 `043bbeae…e08bb5` equal to its manifest, 272462 files). |
| 05:19 | Provision run 3: settle moved 33 more files aside; **`Joined: 1 paths, 0 already matched, 0 still queued`**; services loaded; `fleet ready`. The origin was not re-checked before this run. |
| 05:19 – 05:27 | The replica daemon of this machine adopted the existing `g1-m05` state (1 file) and started publishing the whole tree (`publish … total 269898`) into the journal of the other machine's origin. About 6400 `put` events were appended (segment 0 from 1086 bytes to full, segment 1 to about 0.9 MB). The contents are the settled store versions, so no file content should have changed, but the events are in a journal that belongs to another machine. |
| 05:24 | `sync --off` on this machine (`.cache/f/sync.json` `enabled: false`). The running publish loop did not stop. |
| 05:27 | Replica daemon sent SIGTERM (`Replica stopped; state saved.`); launchd restarted it; no publish lines since. |
| 05:28 on | Reads of `/Volumes/semio-store` on this machine stall (hard mount). The server answers ping, `showmount`, `rpcinfo nfs 3` and the cache health check. Remounting needs root. |

## State Now

- `.cache/f/machine.json`: origin `g1-m05`, group `g1`. Sync is **off**. Daemon `tech.semio.fleet.replica` runs, silent.
- `/Users/Shared/semio/.git/HEAD` exists (from the archive). `status` last printed `Git state: waiting for the base stage of the group server`.
- Preflight (before the stall): exit 1. 10 ok, 2 warnings (native build pending, time synchronisation needs root), 1 failure: `replica status: replica timed out after 15000 ms` while the daemon was publishing.
- `verify` and `fleet --wait 300` not run.
- `repair` not run.
- Native build: pending. Provision run 3 is still in step 9. The bootstrap waits at `door.capacity`: `not enough free memory (need 6144 MB; about 5160 MB spare above the 2048 MB floor)`. Homebrew and `pkg-config` are missing on this machine (`Homebrew not available; install pkg-config, Cairo, Pango and DejaVu fonts`). No `.cache/f/native-bootstrap.done`.
- Write token: `~/.zprofile`, mode 0600, one line; a login zsh sees a 64-character value. Entered by the account, not generated here, `/etc/semio/remote-cache.env` not copied.
- Moved-aside files: `.cache/f/lost/bootstrap-1794541821616` (591) and `.cache/f/lost/bootstrap-1794543487874` (33).

## Open Decisions For The Coordinator

1. Which machine keeps `g1-m05`. The other machine joined first (04:15), so this machine should most likely become the next unused id (`g1-m07` at the time of writing; list `v1/origins` immediately before joining). That means removing this machine's `.cache/f` identity and joining again.
2. What happens to the events this machine appended to the `g1-m05` journal between 05:19 and 05:27 (hlc `1794543582365-…` to about `1794543878335-…`), and whether the other `g1-m05` machine may turn sync on before that is settled.
3. The stalled `/Volumes/semio-store` mount on this machine needs root to unmount and mount again.

## Findings For The Fleet Code

- `join` accepts an origin id that another machine already holds (an existing `origins/<id>` with a foreign attestation) and adopts its state. A check there would have stopped this.
- The provision script races its own `store-mount` LaunchDaemon on the first run (exit 1, second run passes).
- An archive older than a newly added module that the replica command imports makes `join` unloadable after `bootstrap-settle`.
- `sync --off` does not interrupt a publish pass that is already running.

## Update 13 Nov 19:41

- Decision of the account owner: this machine rejoins as the next unused id. `v1/origins` at 19:41: `g1-m01` … `g1-m06`, `g1-server`, so `g1-m07` unless taken at join time.
- 19:41:21: the abandoned provision run 3 (exit **3**, `Waiting for host capacity was cancelled by SIGTERM` after 51559 s at `door.capacity`) and the system daemons `tech.semio.fleet.replica` and `tech.semio.fleet.compact` of `/Users/Shared/semio` were stopped (`launchctl bootout`, rc 0). Their plists were moved out of `/Library/LaunchDaemons` (kept outside the worktree). `tech.semio.fleet.store-mount` stays. Last `g1-m05/attest.json` written from this machine: 19:41:08. From here on this machine no longer touches `origins/g1-m05`.
- The store mount is readable again. `/Users/Shared/semio` still carries `.cache/f/machine.json` with `g1-m05` and two build volumes mounted under `.cache/b`; it is kept until the new join is verified.
- Not done: the rejoin. The current topology and `1113-fleet-setup-existing-repo.md` put the macOS worktree at `~/Documents/semio` and adopt the account's repository. That repository on this machine is at commit `5ececaa78b9`, clean, but carries an older fleet module (other topology file, no boot program), which the current provision script refuses. Waiting for the account owner to choose between moving it aside and bootstrapping from the archive, or bringing it up to date for adoption.
