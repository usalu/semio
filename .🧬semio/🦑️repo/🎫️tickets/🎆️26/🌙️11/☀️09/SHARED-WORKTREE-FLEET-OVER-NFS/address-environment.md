# Address Environment

The committed topology keeps placeholders. This machine's LAN address, DNS name and worker network are not stored in the repository.

`SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST` and `SEMIO_FLEET_NETWORK` are set together. `SEMIO_FLEET_CACHE_ENDPOINT` is optional. When it is absent, the group's cache endpoint keeps its scheme and port and takes the host from `SEMIO_FLEET_NFS_HOST`. An empty value counts as absent. A group id that is not in the topology leaves that file unchanged, so a shell that already exports `g1` does not break fixture groups. After the overlay the topology is validated again. The file on disk is not written.

The provisioning script applies the same host override before it compares the NFS source, forwards the four variables through `sudo -u`, and writes them into `/etc/zshenv` and `/etc/profile`. It also tries `launchctl setenv`. On this Mac that call returns 150 because System Integrity Protection refuses it, and the script used to stop there, before the daemons, the seed and the services. A refusal is now a warning. The same variables, plus `SEMIO_WORKSPACE_ROOT` and `SEMIO_LANGUAGE`, are inserted into the `EnvironmentVariables` dict of the replica, compact, cache and relay LaunchDaemons, which do not inherit `launchctl setenv`. LaunchDaemons and `/etc/exports` are generated at provision time from the overlaid topology, so the machine configuration carries the real network and the repository does not.

`nfsd checkexports` refuses an export line whose peer host does not resolve. On the first macOS group server the other groups do not exist yet. `filter_unresolved_exports` drops those hosts, leaves `-network` and `-mask` values untouched, and omits a line that has neither a network nor a remaining host. The same filter is the Python oracle in the topology tests.

For this machine the operator exports `SEMIO_FLEET_GROUP=g1` with the LAN address and its `/24` network. The cache endpoint is then `http://<that host>:6090`.

The canonical checkout is `/Users/Shared/semio` (no `.git`). `/Users/Shared/semio-store` exists and is empty. Writing `/etc/semio/remote-cache.env`, enabling `nfsd` and running the provision script need an administrator password, which this session cannot enter.

A fleet checkout has no `.git`. The installed launcher then treats the topology directory as the repository and looks for the runtime registry under that directory. The provision scripts run `repo` from the canonical worktree with an absolute script path, and they also set `SEMIO_WORKSPACE_ROOT` to that worktree.
