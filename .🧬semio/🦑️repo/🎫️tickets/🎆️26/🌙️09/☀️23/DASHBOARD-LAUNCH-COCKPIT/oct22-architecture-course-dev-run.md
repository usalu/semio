# Architecture Course Dev Through the Dashboard

Session on 2026-10-22. Goal: the dashboard runs end to end, and `@teaching/architecture-course:dev` becomes ready through it.

## Association

`repo ticket reopen 26/09/23/DASHBOARD-LAUNCH-COCKPIT` returned `ticketReopen: ticket is already open`. This session stays on that ticket. Goal on the ticket is `🎯runningframework🎯runningproducts🎯runningrepo` (`RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-REPO`, "Running Repo"). `repo goal list` currently fails with `goal file not found: A/I/-/O/P/T/I/M/I/Z/E/D/-/R/E/P/O`.

## Launch path

`repo dashboard` is the Go launcher route to `@semio-tech/repo-dashboard-rs:run`. The native binary is `semio`. A bare invocation is the TUI. Headless course start is:

```bash
SEMIO_DASHBOARD_INSTANCE=course-dev-check repo dashboard run @teaching/architecture-course:dev --refresh --detach --wait-ready --timeout 900 --json
```

`dev` is declared on `@teaching/architecture-course` with ready port 6061. The stack starts the proctor, then the Vite site, and prints the site URL when it answers.

## State before this run

- Ports 6061 and 8791 refused connections.
- `semio` is not on `PATH`.
- The installation record at `.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json` is version 2 and points at `.🧬semio/🦑️repo/⚡️cache/tools/dashboard-cli/713c31…/semio`. Current installation code requires version 3 under `.cache/b/t/m/dashboard-cli/<mode>/<digest>/semio`. That directory does not exist, so the next `repo dashboard` rebuilds before it launches.
- Cached `commands.json` has no `architecture-course` entry, so the run passes `--refresh`.
- Default `daemon.pid` 99855 is not alive. This run uses instance `course-dev-check` so it does not take the default daemon directory.

## Evidence

Command log: `🗑️generated/course-dev-run.log`.

## Queue

The official run is waiting on host capacity. It has not been admitted, so no dashboard rebuild or course process has started. Last lines of `🗑️generated/course-dev-run.log`:

```
[capacity] waiting 323s for run: not enough free memory (need 3584 MB; -3074 MB spare above the 6553 MB floor, load 24.01; 3 running, 0 ahead)
[capacity] waiting 328s for run: not enough free memory (need 3584 MB; -1559 MB spare above the 6553 MB floor, load 24.41; 3 running, 0 ahead)
[capacity] waiting 334s for run: not enough free memory (need 3584 MB; -888 MB spare above the 6553 MB floor, load 23.02; 3 running, 0 ahead)
```

Other agents hold the running slots. This run stays queued behind them.

## Admitted

Host capacity admitted the run. It passed source admission and dependency verification, then stopped on an exclusive lease for `repository-graph-facts` shard 7. Holder pid 93195 is a live Bazel run of `@semio-tech/mit-bestand-demonstrator:activate-prototype` (about 97% CPU, sqlite leases open). This run is position 3 of 3 and is still waiting. The course server has not started.

## Installed dashboard

The installed `semio` (version 2 record) launches and resolves `@teaching/architecture-course:dev` to `bun nx run @teaching/architecture-course:dev` with ready port 6061. The daemon accepted the task and the process exited 1 before readiness. Task log: `error: Cannot run nx.json` / `Bun cannot run json files directly`. There is no `nx` executable. Current registry source launches `bun repo run` instead. A current `semio` still has to be built. That build is the queued `repo dashboard` run, still behind pid 93195 on graph shard 7. The isolated daemon for this failed attempt was stopped.

## Authorizer cap

`loadSemioGraphAuthorizersV1` returned null because the published Nx loader refused an authorizer source census above 1024. The live census is 1214. The authorizer schema already allows 2048 sources and rejects 2049. The Nx patch and the published `handle-import.js` for generation `98e9ea6b…` now use 2048, and the tooling receipt hash matches the file. A direct load then returns both `withOriginalSelectedNativeSnapshotV1` and `withOriginalNxGraphNativeStorageV1`.

## Restart

The first dashboard run was stopped after the authorizer refusal. The second run is `🗑️generated/course-dev-run-2.log`. It was admitted, republished the dependency generation after the Nx patch bytes changed (`Switched 🏢️semio-tech to its new dependency generation (epoch 6)`), then failed during graph discovery with `TOML Parse error: Cannot redefine key 'semio-framework-pack-json'` (`EXIT:1`). Ports 6061 and 8791 stayed closed. A later scan of every manifest that names that key parses with Bun's TOML parser, so the duplicate is gone from the tree. The third run is `🗑️generated/course-dev-run-3.log`. It was admitted, inferred and assembled all 65 projects, waited on the shared rust image, then failed with `Authorizer DATA directory changed` (`EXIT:1`). That check treated any ancestor directory mtime change as a torn read, so a sibling write elsewhere in the tree aborted discovery. Ancestor rows now have to keep the same device and inode; only the directory being enumerated still has to keep its full stamp. The fourth run is `🗑️generated/course-dev-run-4.log`. It assembled the graph, then the Nx daemon child exited because startup held `nx:daemon-startup` (graph, rank 4) while requesting `nx-tooling` (rank 2). `nx:daemon-startup:` is now a tool-singleton, the same rank as the tooling lease it takes, so that request is in order. The fifth run is `🗑️generated/course-dev-run-5.log`. It reached `nx:daemon-startup` and the daemon child still exited before acquisition, this time without a lock-order line, then the command only heartbeated. The Nx patch now puts the child exit code and the tail of `daemon.log` on that error. The sixth run is `🗑️generated/course-dev-run-6.log`. It died while republishing dependencies: Bun rejected the Nx patch (`failed to parse patchfile`) because the daemon-log hunk declared 31 new lines and contained 33. The hunk count is now 33. The seventh run is `🗑️generated/course-dev-run-7.log`. The corrected hunk parsed and the run was admitted. The fifth run's child died because it holds `nx:daemon-slot:` (graph rank 4) and then requests `nx-tooling:` (rank 2). `nx:daemon-slot:` is now tool-singleton, same rank as tooling. The seventh run published dependency epoch 7, assembled all 65 projects, then failed `Tooling patch cumulative coordinates changed` (`EXIT:1`): the daemon-log hunk grew by 2 lines and the next hunk in `dist/src/daemon/client/client.js` still started at the old new-file line. That next hunk now starts two lines later, and the coordinate check passes. The eighth run is `🗑️generated/course-dev-run-8.log`. It published dependency epoch 7, assembled the graph, then the daemon child exited `code=1` with `Original native caller capabilities required`. The parent kept heartbeating until the process was terminated (`EXIT:1`). That child checked the caller with the authorizer bundle's schema, which now matches the source schema; the parent had loaded an older copy. The ninth run is `🗑️generated/course-dev-run-9.log`. The course server is not up yet.
