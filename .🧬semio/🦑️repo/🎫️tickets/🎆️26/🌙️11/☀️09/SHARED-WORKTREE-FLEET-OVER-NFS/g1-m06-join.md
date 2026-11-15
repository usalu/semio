# g1-m06 Join

This Mac (`ueli-workstation-mac-mini-m4-16gb-256gb-04-lan`, account `ueli`, address `192.168.178.85`, macOS 26.6.2, 16 GB) is a member of group `g1` as origin `g1-m06`. It syncs and shares git state. **The native build is not complete** (provision exit 3, no marker).

`repo` MCP was not registered in this session, so the existing fleet ticket was used in place of `ticket_reopen`.

## Summary

| Item | Value |
|---|---|
| Host used | `192.168.178.70` (NFS and cache), network `192.168.178.0/24` |
| Origin | `g1-m06` (first joined as `g1-m05`, see "Origin collision") |
| Account | `ueli` |
| Mount | `/Volumes/semio-store` from `192.168.178.70:/Users/Shared/semio-store` (nfs, nobrowse) |
| Worktree | `/Users/Shared/semio`, not a symlink |
| Join count | `Joined: 275399 paths, 272431 already matched, 0 still queued.` |
| Provision exit | 3 (fleet ready, native build pending) on the last run |
| `repair` | not run |
| Native marker | `.cache/f/native-bootstrap.done` absent |
| Write token | `~/.zprofile` mode 0600, one token line, a login zsh sees 64 hex characters |

Exports on every run: `SEMIO_LANGUAGE=en`, `SEMIO_FLEET_GROUP=g1`, `SEMIO_FLEET_NFS_HOST=192.168.178.70`, `SEMIO_FLEET_NETWORK=192.168.178.0/24`, `SEMIO_FLEET_CACHE_ENDPOINT` unset. No `--server`, `--first-machine` or `seed`. The topology file was not edited. No token was generated and `/etc/semio/remote-cache.env` was not copied.

## Before the run

- `showmount -e 192.168.178.70` listed `/Users/Shared/semio-store 192.168.178.0`.
- `http://192.168.178.70:6090/healthz` answered `{"status":"ok"}`.
- `/Users/Shared/semio` did not exist. `~/.local/bin/repo` did not exist.
- `sudo` needs a password, so every provision run went through an administrator prompt (`osascript … with administrator privileges`) around a wrapper that sets the exports and then calls `bash <temporary copy>/🥾️provision/🐚️.sh g1 <origin> --user ueli`.

## Archive and topology copy

The store was mounted read-only as `ueli` in a temporary directory, `manifest.json` and `worktree.tar` were copied out, and the mount was removed. The tar matched the manifest: sha256 `8451c1c04a0994b9506e672cbc479dcf2a05237c8cf1592cb1547d15e4213d51`, 272934 files. The topology directory was extracted from that tar into a temporary directory outside the worktree and that copy was run with `--archive` of the local copy.

## Origin collision

1. `v1/origins` was listed at 04:01: `g1-m01` to `g1-m04` and `g1-server`. `g1-m05` was unused and was chosen.
2. At 04:15:46, while the 12 GB archive was copying, a Windows machine created `origins/g1-m05` (`attest.json` platform `win32`).
3. At 04:33 this Mac joined as `g1-m05` (`Joined: 275377 paths, 272345 already matched, 0 still queued.`). Neither `provision` nor `join` refuses an origin directory that another machine created.
4. `status` showed a `g1-m05 win32` row. Sync was turned off here at 04:38 (`sync --off`); the services were booted out at 04:45.
5. The dev decided: this Mac re-joins as `g1-m06`. The local state `.cache/f` of `g1-m05` was moved out of the worktree and `join` ran as `g1-m06`.

What this Mac wrote into `origins/g1-m05` while it held that id, for the coordinator:

- Three `put` events in `segments/000000000000.jsonl` (hlc `1794540822572`, `1794540823955`, `1794540825432`), all for `🔩️native/🥾️bootstrap/🐚️.sh` (the bash 3.2 fix below).
- `head.json` at 04:33:45.
- `attest.json` (last at 04:44) and `view.txt` (04:36), which replaced the Windows machine's files of 04:15:46.

The Windows machine wrote `head.json` and `segments` again at 05:23 to 05:24. Its `g1-m05` row is `missing (no attestation)` in `fleet` since about 05:05. Whether its journal is consistent after the three foreign events was not checked from here.

## Second join

The first `g1-m06` provision run exited **1** in step 7: `bootstrap-settle` moved 26 files aside that had changed in the store while sync was off, among them the replica command, which now imports the new module `🤝️shared-worktree/🎣️tiers`. That module did not exist in the tree yet and `join` (which would have created it) could not start: `Cannot find module '../../🎣️tiers/🟦️.ts'`.

The five files of `🎣️tiers` were copied from the store blobs named by the `g1-server` journal (sha256 checked for each). `bootstrap-settle` (1 more file moved aside) and `join --group g1 --origin g1-m06` then ran as `ueli` and exited 0. The next provision run skipped `join`, loaded `tech.semio.fleet.replica` and `tech.semio.fleet.compact`, and went on to the native bootstrap.

Files moved aside by settle are in `.cache/f/lost/bootstrap-1794541541504` and `bootstrap-1794541694128` (27 files, all superseded by the store versions).

## Checks

`preflight --lang en` (topology script), last run 08:32: **exit 0**. 13 checks, none failed, 2 warnings:

- `native build: pending`
- `time synchronisation: the synchronisation state cannot be read here (… needs root)`; the provision run itself printed `network time is on`.

`status`, last run 08:32: **exit 0**.

```
Replica g1-m06 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 71035)
272777 files tracked; queued: 0 incoming, ? outgoing; 27 overwritten files kept in .cache/f/lost
Git state: shared (stage and commits replicate); stage tree f3fc6c8568636a95769723c9e6d357f53250a1c4.
g1-m01 … g1-server: 0 events (0 B) behind (all seven origins)
```

Between about 05:12 and 07:00 `status` printed `Git state: waiting for the base stage`: the replica code changed in the worktree twice, the daemon restarted itself onto the new code and walked all 272544 files again, as every other worker did. The daemon logged `repaired … waiting-for-base` on its own. `.git/HEAD` was present throughout. The `repair` command was not run.

`verify`: **exit 0 once** (05:13, `No drift: 272544 shared paths agree with the state and the store.`). Every other run exited 1 with drift on one to fourteen paths, different each time, each a file that another machine or another session was editing at that moment. The last three runs (08:33 to 08:35):

```
Drift on 1 paths (272776 shared paths checked).  content differs: .git/index
Drift on 1 paths (272776 shared paths checked).  state differs: ✏️s/🔌️plugins/📕️norm/Cargo.lock
Drift on 1 paths (272776 shared paths checked).  state differs: .🧬semio/…/GET-NORM-PLUGIN-WORKING-END-TO-END/hitcost.md
```

`fleet --wait 300`: **exit 1** on all four runs. The fleet did not agree at any time.

- 05:11: `g1-m05 missing (no attestation)`; `g1-m01`, `g1-m04`, `g1-server` `diverged` in `view.digest`; `g1-m06 ok`.
- 05:18 and 05:40: all six workers `missing (no attestation)`, `g1-server` `lagging (not settled)`, then `ESTALE … origins/g1-server/attest.json`.
- 08:35: `ESTALE: unknown error, open '/Volumes/semio-store/v1/origins/g1-m01/attest.json'`. The `status` table just before it:

```
  g1-m01 darwin 3s 0/0 on 8e0acd9c25af 47/0/109 diverged
  g1-m02 darwin 2s 0/0 on 8e0acd9c25af 54/0/124 ok
  g1-m03 darwin 2s 0/0 on 8e0acd9c25af 52/0/122 diverged
  g1-m04 linux 2s 0/0 on 8e0acd9c25af 54/0/124 ok
  g1-m05 - missing (no attestation)
  g1-m06 darwin 1s 0/0 on 8e0acd9c25af 37/0/83 diverged
  g1-server - missing (no attestation)
```

`fleet --diff`: **exit 1**. First run (05:11):

```
g1-m01 differs from this machine: 0 lines only there, 1 lines only here.
  only on g1-m06 (here): ?? …/SHARED-WORKTREE-FLEET-OVER-NFS/1113-result-warm-restart.md
g1-m04 differs from this machine: 0 lines only there, 1 lines only here.
  only on g1-m06 (here): ?? …/SHARED-WORKTREE-FLEET-OVER-NFS/1113-result-warm-restart.md
g1-server differs from this machine: 2 lines only there, 0 lines only here.
  only on g1-server:  M 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts
  only on g1-server: ?? …/SHARED-WORKTREE-FLEET-OVER-NFS/🧰️scripts/1113-chaos-sweep.sh
The working-tree views differ or are unavailable for 3 origins.
```

Last run (08:35), 198 lines:

```
g1-m01 differs from this machine: 43 lines only there, 7 lines only here.
g1-m02 differs from this machine: 65 lines only there, 7 lines only here.
g1-m03 differs from this machine: 61 lines only there, 7 lines only here.
g1-m04 differs from this machine: 65 lines only there, 7 lines only here.
The working-tree views differ or are unavailable for 4 origins.
```

The files agree (0 events behind, same `code`); the working-tree view of this Mac is old. `status` reports `git view 896953 ms` for the last pass here: one `git status` of the tree takes about 15 minutes on this machine under its current load, so `view.txt` trails the other machines by that much.

## Native build

Not complete. What was fixed and what is left:

1. **Xcode Command Line Tools were missing** (`xcode-select -p` failed, `/usr/bin/git` was the shim). Installed through `softwareupdate` (Command Line Tools for Xcode 26.6) in the first provision run.
2. **bash 3.2 `unbound variable`** at `🔩️native/🥾️bootstrap/🐚️.sh` line 891 (`$root…`). Three log lines now brace the variable before the ellipsis: `${version}…` (uv), `${root}…` (Node.js), `${DOTNET_ROOT}…` (.NET).
3. **Capacity gate.** `dependencies` wants 6144 MB above the 2048 MB floor and waited 13 minutes. GitKraken and Firefox were closed by the dev; it was then admitted.
4. **Plan cancelled for memory that was not short.** `The plan was cancelled: the host has had only 143 MB of free memory (floor 256 MB) for 30 s`, while the capacity gate had just admitted the run with 7.5 GB available. The slice pool's pressure watch and `hostAvailableBytesV1` read `os.freemem()`, which on macOS is the free pages alone. Both now read `availableMemoryBytesV1` of `🏗️build/🧠️memory` (free plus inactive pages from `vm_stat`, `MemAvailable` on Linux), which already documents this exact trap. Files: `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs`, `…/🧱️limit/🟨️.mjs`, test line in `🏡️resident/🧪️tests/🛑️cancel/🟦️.ts` (36 pass, 0 fail). The graph service that predated the change was stopped with `repo service host stop`.
5. **Open: `Slice worker exited with code 75`.** The last three attempts (06:15, 07:07, 07:31 UTC) ended in `The resident service met an unexpected failure (reference a0db6966)`. The service log shows the slice worker waiting for `the graph build of a peer` and the host short of memory: `not enough free memory (need 3584 MB; 282 MB spare …); behind workspace:components (running, 7 job(s), 7546 MB)`.

The cause of 5 is load from a second session on this Mac, not started by this one: since 05:46 scripts of the ticket `26/11/10/GET-NORM-PLUGIN-WORKING-END-TO-END` run here (`normcomp-assets-retry.sh`, `hitcost-driver.ts` at 4.5 GB resident, `repo cargo … check`, `workspace:components` with 7 jobs). On 16 GB the cold project graph of the native bootstrap does not fit beside them. They were left running.

Also open: Homebrew is not installed, so the bootstrap printed `Homebrew not available; install pkg-config, Cairo, Pango and DejaVu fonts before running native C++ builds`. No run got far enough to show whether that fails the build.

Provision exits in order: 3 (`g1-m05`, unbound variable), 1 (`g1-m06`, missing `🎣️tiers` before join), 3 (`g1-m06`, plan cancelled). The later attempts ran `bash …/🔩️native/🥾️bootstrap/🐚️.sh setup` directly as `ueli` with the login environment, which does not write the marker.

To finish: when the other session's builds are idle, run the provision command for `g1-m06` again with the same exports. It skips `join`, refreshes the services and retries the native bootstrap; exit 0 writes `.cache/f/native-bootstrap.done`.

## Other notes

- `launchctl setenv` is refused under System Integrity Protection; the variables are in `/etc/zshenv`, `/etc/profile` and the LaunchDaemon environment.
- `~/.zprofile` holds the native bootstrap's own `semio native repo PATH` block (11 lines) plus the one token line, not the token line alone: the bootstrap wrote its block first and writes it again on every run.
- The Claude desktop app runs `git diff`/`git status` in `/Users/Shared/semio` and in `/Users/ueli/Documents/semio` every few seconds since the Command Line Tools gave the Mac a real `git`; each takes 1 to 2 GB and the index lock. The daemon logged `blocked … write-tree-failed` followed by `repaired` twice.
- One `git write-tree` was run by hand in `/Users/Shared/semio` during diagnosis; it failed on `index.lock` and changed nothing.
