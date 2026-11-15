# 🧾️ Worker Prompts for the Shared Worktree Fleet (2026-11-13)

Paste one block to the agent on the machine. Same facts as `mac-worker-prompt.md` and `fleet-setup.md`. Addresses come only from the ticket notes: group server host `192.168.178.70` (`g1-m01-join.md:13`), worker network `192.168.178.0/24` (`mac-worker-prompt.md:12`), store export `/Users/Shared/semio-store` (`mac-worker-prompt.md:25`). The topology keeps its placeholder hosts.

Origin ids are never reused (`fleet-setup.md:13`). The store already holds `g1-server`, `g1-m01`, `g1-m02` and `g1-m03`. Before a join, the agent lists `v1/origins` in the store and picks an unused id. A machine that was already `g1-m02` or `g1-m03` keeps its own id.

## (a) Upgrade an already joined worker (any OS)

```
Upgrade the semio fleet replica on this machine so that it runs the new replica code. You are a worker that already joined: the machine has `.cache/f/machine.json` in the worktree. Finish end to end. Read `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-git-state-contract.md` (section Self-restart and Rollout) and follow it. Do not edit `fleet-setup.md` in that ticket folder.

Fill these in before you start:
- origin: <origin, for example g1-m02>
- account: <worker login account>
- OS: <macOS | Linux | Windows>

Worktree: macOS `/Users/Shared/semio`, Linux `/srv/semio`, Windows `C:\semio`. Do not move it and do not symlink it.

Rules:
- Do not run any git command that changes the shared worktree or its `.git`. The only exception is staging or committing when the coordinator asks for it. No checkout, switch, reset, restore, clean, stash, merge, rebase or pull.
- Do not use `pkill` or `killall`. Stop only the one replica pid named in `.cache/f/daemon.lock`, once, as the worker account. Do not stop the relay, the cache or the compaction job.
- Use the same exports as the join: `SEMIO_LANGUAGE=en`, `SEMIO_FLEET_GROUP=g1`, `SEMIO_FLEET_NFS_HOST=192.168.178.70`, `SEMIO_FLEET_NETWORK=192.168.178.0/24`, `SEMIO_FLEET_CACHE_ENDPOINT` unset, `SEMIO_WORKSPACE_ROOT` set to the worktree, and `PATH` including `~/.local/bin`.

Steps:
1. Verify the worktree received the new module code: the directories `🌿️git` and `🩺️attest` exist beside `🔁️replica`. Take the sha256 of `🔁️replica/🟦️.ts` and of `🌿️git/🟦️.ts` twice, 60 seconds apart; continue only when both are unchanged (the sources are settled), otherwise wait and repeat. Do NOT run `verify` before the restart: the old daemon's `verify` reports drift on `.git/index`, `.git/logs`, `.git/config`, hooks and IDE files, which is expected and only clears once the new code runs.
2. Read the pid of the running replica from `.cache/f/daemon.lock` (JSON, field `pid`). Note it in the record.
3. Stop that pid once, as the worker account, so the service manager starts the new code:
   - macOS: `kill -TERM <pid>` from the worker account. launchd (`tech.semio.fleet.replica`, KeepAlive) restarts it.
   - Linux: `kill -TERM <pid>` from the worker account. systemd (`semio-fleet-replica@<account>.service`, Restart=always) restarts it.
   - Windows, in PowerShell as the worker account: `Stop-Process -Id <pid>`. Wait 30 seconds. If `.cache\f\daemon.lock` still names the old pid or no new pid appears, start the task with `schtasks /Run /TN semio-fleet-replica`. Record which path was taken.
4. Wait up to 60 seconds for `.cache/f/daemon.lock` to name a new pid. Record the new pid.
5. Run `repo ./📜️script.ts status --lang en` (replica directory). Record its exit code and output. It must print `Git state: shared`; `waiting for the base stage` is allowed for the first minutes. Then run `repo ./📜️script.ts verify --lang en` and record it; drift must be 0 now.
6. Run `repo ./📜️script.ts fleet --wait 120 --lang en` (replica directory). Record exit code and output. If the command reports an unknown operation, the fleet command is not in this build yet: record the exact message and stop. Do not work around it.
7. Write the record in the ticket folder as `<origin>-upgrade.md`. Include: origin, account, OS, the old pid, the new pid, the sha256 from step 1, the verify exit code, the status exit code, the fleet exit code and output, and any error text. Do not put tokens in the record.

Do not delete the ticket folder files. Do not delete `.cache/f/daemon.lock` by hand.
```

## (b) Join a Linux worker

```
Stand this Linux machine up as a worker of the semio fleet. Finish end to end: the machine joins, syncs, and the native build completes. Read `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/fleet-setup.md` and follow it. Do not edit that file.

Fill these in before you start:
- group: g1
- origin: <next unused origin, for example g1-m04; check v1/origins in the store>
- account: <login account>
- network: 192.168.178.0/24

Worktree is `/srv/semio`. It must be empty, or already this origin's joined checkout. Do not symlink it. Do not put the topology copy inside it.

Copy the topology directory (provision shell script and the json beside it) to a temporary directory outside the worktree. Run that copy. Do not pass `--server`, `--first-machine`, or `seed`. Do not edit topology hosts, networks, or cache names. Do not generate a cache token. Do not copy `/etc/semio/remote-cache.env` onto this worker.

Exports, set together:
- SEMIO_LANGUAGE=en
- SEMIO_FLEET_GROUP=g1
- SEMIO_FLEET_NFS_HOST=192.168.178.70
- SEMIO_FLEET_NETWORK=192.168.178.0/24
- unset SEMIO_FLEET_CACHE_ENDPOINT

Command shape: `bash <temporary copy>/provision.sh g1 <origin> --user <account>`. The script calls `sudo -E` itself. A bare sudo drops the exports. Install nothing by hand unless the script says the NFS client is missing. If it reports missing build tools, install them with the exact line the script prints. Re-running the same origin refreshes mounts and services and skips join.

The store is an NFS export of the group server (macOS). The script mounts it read-only for the archive and then through fstab. Check `showmount -e 192.168.178.70` lists `/Users/Shared/semio-store` for the worker network before you run the script.

Write token, after join, as the account: `~/.profile` mode 0600, exactly one line, `export SEMIO_REMOTE_CACHE_TOKEN='<existing write token>'`. The token is the one already on the group server. Do not print it. Do not invent one. Then, with the same exports: `preflight --lang en`, then `repo ./📜️script.ts status --lang en` (replica directory), then `repo ./📜️script.ts verify --lang en` (replica directory). Verify exits 0.

Native build is part of done. Provision exit 3 means fleet ready and native pending: fix the cause and run the same command again. Exit 0 writes `.cache/f/native-bootstrap.done`. `repo` must run with `SEMIO_WORKSPACE_ROOT=/srv/semio` and `PATH` including `~/.local/bin`.

Record the result in this ticket folder as `<origin>-join.md`: host used, origin, account, mount, join and verify counts, provision exit, marker present or the native error. Do not use git commands that change the shared worktree or its `.git`, except staging or committing when the coordinator asks for it.
```

## (c) Join a Windows worker

```
Stand this Windows machine up as a worker of the semio fleet. Finish end to end: the machine joins, syncs, and the native build completes. Read `.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️09\SHARED-WORKTREE-FLEET-OVER-NFS\fleet-setup.md` and follow it. Do not edit that file.

Fill these in before you start:
- group: g1
- origin: <next unused origin, for example g1-m04; check v1/origins in the store>
- account: <DOMAIN\user, the account of this elevated shell>
- network: 192.168.178.0/24

Worktree is `C:\semio`. It must be empty, or already this origin's joined checkout. Do not put the topology copy inside it. Copy the topology directory (provision.ps1 and the json beside it) to a temporary directory outside `C:\semio`.

Run in an elevated PowerShell, with these exports set in the same session:
```
$env:SEMIO_LANGUAGE = 'en'
$env:SEMIO_FLEET_GROUP = 'g1'
$env:SEMIO_FLEET_NFS_HOST = '192.168.178.70'
$env:SEMIO_FLEET_NETWORK = '192.168.178.0/24'
Remove-Item Env:SEMIO_FLEET_CACHE_ENDPOINT -ErrorAction SilentlyContinue
powershell -NoProfile -ExecutionPolicy Bypass -File <temporary copy>\provision.ps1 g1 <origin> -User "$env:USERDOMAIN\$env:USERNAME"
```
Do not pass `-Server`, `-FirstMachine`, or `seed`. Do not edit topology hosts, networks, or cache names. Do not generate a cache token. Do not copy `/etc/semio/remote-cache.env` onto this worker. Re-running the same origin refreshes mounts and tasks and skips join.

Store: the group server is macOS and exports NFS. Client for NFS mounts it on `S:\`. Check the export with `showmount -e 192.168.178.70` where available. Turn on Developer Mode if the account cannot create symlinks; without it, symlinks are skipped and recorded.

After join, as that account, store the existing write token (user scope, do not print it):
```
[Environment]::SetEnvironmentVariable('SEMIO_REMOTE_CACHE_TOKEN','<the write token already on the group server>','User')
```
Do not invent a token. Keep the account logged on: the replica runs as a scheduled task with a logon trigger and an interactive token, so it stops when the account logs off.

Then, with the same exports: `repo preflight --lang en`, then `repo ./📜️script.ts status --lang en` (replica directory), then `repo ./📜️script.ts verify --lang en` (replica directory). Verify exits 0.

Native build is part of done. Provision exit 3 means fleet ready and native pending: fix the cause and run the same command again. Exit 0 writes `.cache\f\native-bootstrap.done`. The native bootstrap installs Git through winget. Until then the replica runs without git on PATH.

Record the result in this ticket folder as `<origin>-join.md`: host used, origin, account, mount, join and verify counts, provision exit, marker present or the native error. Do not use git commands that change the shared worktree or its `.git`, except staging or committing when the coordinator asks for it.
```
