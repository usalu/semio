# Service Logs

The group server joined and the store was seeded. The replica, cache and relay LaunchDaemons then exited 78 (`EX_CONFIG`) on every retry. `/var/log` is writable only by root, and each daemon runs as the worker account, so launchd cannot create `StandardOutPath` or `StandardErrorPath`. No log file appears.

`load_daemon` now creates each of those paths, owned by the worker account, before the job is bootstrapped. When the job is already loaded and a log file was missing, it is kickstarted. The same paths are what the Python oracle reads out of the plist.

After those files existed, the jobs still exited. The canonical checkout's `node_modules` was a symlink into another checkout's dependency store. The launcher treats that symlink as a managed namespace and stats a generation directory that exists only in the other checkout. `release_foreign_namespace` removes a `node_modules` symlink whose physical target is outside the worktree and leaves a link or directory inside the worktree in place. The launcher then publishes the workspace links itself. The shell and the Python oracle (`namespace`) make the same keep or remove decision. Both sides resolve the worktree root physically, because `/var` and `/private/var` are the same directory on macOS.

## Verification

The snapshot for origin `g1-server` recorded 271490 paths. Replica status shows the daemon running, an empty incoming queue, and that origin caught up. Preflight ran 15 checks and failed none. Three warnings remain: the store volume is below the recommended free space, the native bootstrap marker is absent, and reading the time-synchronisation state needs root. Cache health answered ok on the loopback and on the group address. `showmount` lists the store export for the worker network. A later provisioning run retries the native bootstrap and omits `--first-machine`.
