# Mac Worker Setup Prompt

Paste the block below to the agent on the new Mac. Fill the four blanks. The sequence is `fleet-setup.md` in this ticket. This file adds the macOS cases that sequence does not spell out.

```
Stand this Mac up as a worker of the semio fleet. Finish end to end: the machine joins, syncs, and the native build completes. Read `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/fleet-setup.md` and follow it. Do not edit that file.

Fill these in before you start:
- group: g1
- origin: g1-mNN
- account: <login account>
- network: 192.168.178.0/24

Worktree is `/Users/Shared/semio`. It must be empty, or already this origin's joined checkout. Do not symlink it. Do not put the topology copy inside it.

Copy the topology directory (provision shell script and the json beside it) to a temporary directory outside the worktree. Run that copy. Do not pass `--server`, `--first-machine`, or `seed`. Do not edit topology hosts, networks, or cache names. Do not generate a cache token. Do not copy `/etc/semio/remote-cache.env` onto this worker.

Exports, set together:
- SEMIO_LANGUAGE=en
- SEMIO_FLEET_GROUP=<group>
- SEMIO_FLEET_NFS_HOST=<host that actually exports the store>
- SEMIO_FLEET_NETWORK=<network>
- unset SEMIO_FLEET_CACHE_ENDPOINT

Discover the host. The topology name `nfs.g1.semio.internal` is a placeholder. From this Mac, `showmount -e <host>` must list `/Users/Shared/semio-store` for the worker network. On the g1 LAN that host is 192.168.178.70, not 192.168.178.71 (`.71` does not answer ARP). Cache health is `http://<host>:6090/healthz`. Leave the cache endpoint unset so it is derived from the NFS host.

Command shape: `bash <temporary copy> <group> <origin> --user <account>`. The script calls `sudo -E` itself. A bare sudo drops the exports and the machine tries the placeholder host. If sudo needs a password, run the same command as root through an administrator prompt and keep the exports. Re-running the same origin refreshes services and skips join.

macOS root cannot read the NFS export (`sed` and `cp` return operation not permitted; the account can read it). When that happens, mount or read the store as the account, copy `v1/bootstrap/manifest.json` and `worktree.tar` to the temp directory, check the tar sha256 against the manifest, and pass `--archive` of that local copy. Remove a leftover read-only mount with `diskutil unmount force`. `umount` is refused. The real store mount is `/Volumes/semio-store` via `tech.semio.fleet.store-mount`.

If `~/.local/bin/repo` already exists and fails with `link …/.repo-link-… …/framework: operation not permitted`, move that binary aside and let provisioning rebuild the launcher from the checkout. Do not delete the replica daemon `tech.semio.fleet.replica`.

Write token, after join, as the account: `~/.zprofile` mode `0600`, exactly one line, `export SEMIO_REMOTE_CACHE_TOKEN='<existing write token>'`. The token is the one already on the group server. Do not print it. Do not invent one. A gitconfig fragment in that file makes zsh abort before the token loads. `launchctl setenv` failing under System Integrity Protection is expected; `/etc/zshenv` and `/etc/profile` still receive the cache variables. A login zsh must see a 64-character token, and `repo remote-cache status` must show the token present and the probes authorized.

Native build is part of done, not optional. Provision exit 3 means fleet ready and native pending: fix the cause and run it again. Exit 0 writes `.cache/f/native-bootstrap.done`. macOS `/bin/bash` is 3.2: `$NAME…` (ellipsis) is one unset variable; brace it as `${NAME}…` in the bootstrap log lines if setup dies with `unbound variable`. `repo` must run with `SEMIO_WORKSPACE_ROOT=/Users/Shared/semio` and `PATH` including `~/.local/bin`.

A cold graph is one Bun process, `slice-pool/worker.ts`, several GB, one core, tens of minutes, during `workspace:setup` / `door.script` ("Reading the project graph slice"). That is the memory and the time. Cursor is the next resident, about 1 GB. The replica is not. On macOS `os.freemem()` is only the free-page list and sits near 100 MB while inactive memory is still gigabytes. The no-cgroup pressure measure must be the memory the host can still give out (free plus inactive pages, `availableMemoryBytesV1`), not `os.freemem()`. If a plan cancels at about 100 MB free while inactive is large, that measure has regressed. Do not lower `freeFloorMegabytes`. `SEMIO_GRAPH_MEMORY_LIMIT_MB` does not cap that reader. Do not kill the replica to free RAM.

Then, with the same exports: `preflight --lang en`, replica `status`, replica `verify`. Verify exits 0. A time-sync warning that needs root may remain. A missing Homebrew warning for pkg-config, cairo, and pango is non-fatal. Record the result in this ticket as `<origin>-join.md`: host used, origin, account, mount, join and verify counts, provision exit, marker present or the native error. Do not use git commands that change the shared worktree.
```
