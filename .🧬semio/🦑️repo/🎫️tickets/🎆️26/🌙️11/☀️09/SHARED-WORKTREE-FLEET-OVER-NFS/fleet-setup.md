# Fleet Setup After The First Group Server

`g1` is the first group server. Its store already contains origin `g1-server`, a snapshot, and `v1/bootstrap` (`manifest.json` and `worktree.tar`). Every later machine joins that store. None of them runs `seed` or `--first-machine`.

Addresses stay out of this file. On each machine, `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST` and `SEMIO_FLEET_NETWORK` are set together to that machine's own group. Leave `SEMIO_FLEET_CACHE_ENDPOINT` unset: the endpoint keeps the file's scheme and port and takes the host from `SEMIO_FLEET_NFS_HOST`. `SEMIO_LANGUAGE` is `en`. The topology file keeps its placeholder hosts, networks and cache names.

## Every machine

- The worktree path is the one in the topology: `/Users/Shared/semio`, `/srv/semio`, or `C:\semio`. Do not move it and do not symlink it.
- The worktree is empty, or it is already this machine's joined checkout. Anything else is refused.
- Copy the topology directory (the provision script and the topology json beside it) to a temporary directory. Do not put that copy inside the canonical worktree.
- Export the three variables, then start the script as the editing account. The script re-executes through `sudo -E`. A bare `sudo` without `-E` drops the variables and the machine tries the placeholder host.
- Origin ids match `^[a-z0-9][a-z0-9-]{0,39}$`, are unique in the fleet, and are never reused. The group server is `<group>-server`. Workers are `<group>-m01`, `<group>-m02`, and so on.
- The account that edits receives the existing write token as `SEMIO_REMOTE_CACHE_TOKEN` (mode 0600). Do not generate a new pair. Do not copy `/etc/semio/remote-cache.env` onto a worker.
- Exit code 3 means the machine is in the fleet and the native build is still pending. Run the same command again later. Exit code 0 means the native build finished too.
- After provisioning, with the same three variables: `preflight --lang en`, then replica `status`, then replica `verify`. `verify` exits 0.

## 1. Workers of g1

Do these before any other group. They must be on the worker network already exported by the g1 server. From the worker, `showmount -e <g1 host>` lists the store export. The g1 server's disk is below the recommended free space for a store; workers do not add a second copy of the blobs.

macOS worker, origin `g1-m01`, account `<account>`:

```
export SEMIO_LANGUAGE=en
export SEMIO_FLEET_GROUP=g1
export SEMIO_FLEET_NFS_HOST=<g1 host>
export SEMIO_FLEET_NETWORK=<g1 worker CIDR>
unset SEMIO_FLEET_CACHE_ENDPOINT
bash <temporary copy>/provision.sh g1 g1-m01 --user <account>
```

Then, as `<account>`, mode 0600, in `~/.zprofile`:

```
export SEMIO_REMOTE_CACHE_TOKEN=<the write token already on the g1 server>
```

Linux worker: same exports and the same script. The canonical path is `/srv/semio`. `--user` is the login account. Install nothing by hand unless the script says the NFS client is missing.

Windows worker, elevated PowerShell, topology directory copied outside `C:\semio`:

```
$env:SEMIO_LANGUAGE = 'en'
$env:SEMIO_FLEET_GROUP = 'g1'
$env:SEMIO_FLEET_NFS_HOST = '<g1 host>'
$env:SEMIO_FLEET_NETWORK = '<g1 worker CIDR>'
Remove-Item Env:SEMIO_FLEET_CACHE_ENDPOINT -ErrorAction SilentlyContinue
powershell -NoProfile -ExecutionPolicy Bypass -File <temporary copy>\provision.ps1 g1 g1-m01 -User "$env:USERDOMAIN\$env:USERNAME"
```

Then, as that account:

```
[Environment]::SetEnvironmentVariable('SEMIO_REMOTE_CACHE_TOKEN','<the write token>','User')
```

Turn on Developer Mode if the account cannot create symlinks. The next origin is `g1-m02`, then `g1-m03`. Re-running with the same origin refreshes mounts and services and skips `join`.

## 2. Another group server

Add a group only when its server has a large local disk (the sizing note asks for about 2 TB). Every group store ends up with every group's blobs, so a small g1 disk also grows when g2 starts publishing.

The committed g2–g5 servers are Linux, NFS, export `/srv/semio-store`. A server that is not that shape needs the structural fields changed on g1 first (`server.platform`, `server.protocol`, `nfs.export`). Leave host, network and cache name as placeholders. Let that edit replicate, then refresh g1's bootstrap archive (`bootstrap-archive` on the g1 server) before the new server extracts it.

On every group server, make `nfs.gN.semio.internal` resolve to that group's server (local DNS or `/etc/hosts`). Those names stay out of the topology file. Until a name resolves, g1 omits that peer from its export.

On the new server, same token pair in `/etc/semio/remote-cache.env` (mode 0600, owner the editing account on macOS). Apply `server-plan --group g2 --part prepare`, then the `provision` line that `server-plan` prints. It already contains `--server` and `--archive` of a peer bootstrap. Do not add `--first-machine`. macOS peers mount under `/Volumes/semio-peers/<group>/v1/bootstrap`. Linux peers mount under `/mnt/semio-peers/<group>/v1/bootstrap`.

Wait until this server's replica `status` shows the origins copied from the peer. Then add its workers with the same prompts as section 1, `SEMIO_FLEET_GROUP` set to the new group and `SEMIO_FLEET_NFS_HOST` set to the new server.

When the new name resolves, run the g1 provision script again on the g1 server with origin `g1-server` and `--server`, without `--first-machine`. That refreshes the peer mount and the export. `join` is skipped.

## 3. Acceptance

On each new machine, preflight fails nothing. Expected warnings on a small disk, a pending native build, or a time-synchronisation state that needs root are the same three already seen on g1. A save on one machine of a group appears on another machine of that group in about a second. A save reaches another group after the relay copies it, about a second longer.

The archivist is one machine with its own `.git`, which is not replicated. Do not create `.git` in a fleet worktree.
