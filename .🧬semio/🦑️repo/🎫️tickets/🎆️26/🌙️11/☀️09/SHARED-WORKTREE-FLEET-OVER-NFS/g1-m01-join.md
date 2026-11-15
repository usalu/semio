# g1-m01 Join

This Mac (`ueli-workstation-mac-mini-m6-16gb-256gb-01-lan`, account `ueli`, addresses `192.168.178.77` and `192.168.178.78`) joined group `g1` as origin `g1-m01`.

`repo` MCP was not registered in this session, so the existing fleet ticket was used in place of `ticket_reopen`.

## Host

`192.168.178.71` does not answer ARP. `showmount -e 192.168.178.70` lists `/Users/Shared/semio-store` for `192.168.178.0`. `http://192.168.178.70:6090/healthz` returns `status: ok`. The topology file was not edited. Provisioning used:

- `SEMIO_LANGUAGE=en`
- `SEMIO_FLEET_GROUP=g1`
- `SEMIO_FLEET_NFS_HOST=192.168.178.70`
- `SEMIO_FLEET_NETWORK=192.168.178.0/24`
- `SEMIO_FLEET_CACHE_ENDPOINT` unset

No `--server`, `--first-machine`, or `seed`.

## Archive

Root cannot read the NFS export (`sed` and `cp` return operation not permitted; the account `ueli` can read it). The bootstrap archive was copied as `ueli` and checked against `manifest.json` (`sha256` `522798507c215cdfac1ad9e8f59bd4dc320a20e57ea29881a2b631938917ba48`, 271491 files). The provision script then received `--archive` of that local copy. The temporary read-only mount under `/var/tmp` was removed with `diskutil unmount force` (`umount` itself is refused).

The launcher already in `~/.local/bin/repo` (10 Nov) could not publish the workspace links. It was set aside and the script built the launcher from the checkout.

## Result

- Worktree `/Users/Shared/semio`, store `/Volumes/semio-store` from `192.168.178.70:/Users/Shared/semio-store`.
- `.cache/f/machine.json`: origin `g1-m01`, group `g1`.
- Joined 271491 paths, all already matched. Replica daemon `tech.semio.fleet.replica` is running. Daily compact is scheduled.
- Provisioning exited **3**. The native bootstrap stops at `native/bootstrap` line 1180: `DOTNET_ROOT` is an unbound variable. `launchctl setenv` is refused while System Integrity Protection is on; the cache variables are in `/etc/zshenv` and `/etc/profile`.
- Preflight `--lang en` exited 0. Warnings: native build pending, and time synchronisation cannot be read without root. No check failed.
- Replica `status` exited 0: store reachable, daemon pid running, 0 events behind `g1-server`.
- Replica `verify` exited 0: no drift, 271491 paths agree with the state and the store.

## Write token

The account added the existing write token to `~/.zprofile`. The file also contained a gitconfig `[user]` block, which zsh tried to glob, so the token never reached the shell. That block was removed (the same name and email are already in `~/.gitconfig`). The token line is now `export SEMIO_REMOTE_CACHE_TOKEN=…`, mode `0600`. A login zsh sees a 64-character value. `repo remote-cache status` reports the token present and both cache probes authorized. No new pair was generated. `/etc/semio/remote-cache.env` was not copied onto this worker.
