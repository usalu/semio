# L16 Graph Service Diagnosis (read-only, 2026-11-22 ~14:43)

## Verdict

The graph service is not wedged. The newest service host is alive and logging every second, but it has not published its socket yet. It is parked in the `build:source-admission` / `source.served` phase, waiting for the exclusive tool singleton `nx-tooling-provision`. That singleton is held or queued by stale service hosts from this agent session (`12974ae06f6b`) that have been waiting or holding for 10 to 20 hours. The scheduler is idle and capacity has headroom, so this is not starvation by other sessions' builds.

## Client code (what the message waits on)

- Message key `host-wait` in `🧬️schema/🔣️.json` (line 1976), printed by `connectHostV1` in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🚪️client/🛰️plan/🟨️.mjs` (around lines 120-170).
- The loop waits on three things:
  1. A connect to the socket endpoint `/run/user/1000/semio-repo/1000/h-<tag>.sock`, derived in `📨️protocol/📍️endpoint/🏠️host/🟨️.mjs`.
  2. The startup owner lock `start-<tag>.lock` (`tryStartupOwnerV1`). It is free.
  3. The spawned service handle. While `alive()` is true, the client waits up to `startCeilingMilliseconds` = 900000 ms (15 min). It fails with `timeout` (`mode: live-ceiling`) after that. If the process dies, it fails with `lost`. The silence timeout (`startTimeoutMilliseconds` = 60 s) applies only when the process is not alive.
- So the client waits on a socket that appears only after the service finishes startup. It is not a lease or a sealed-graph wait on the client side.
- The 15-minute observation matches `startCeilingMilliseconds` = 900000, which is why `l4-activate.sh` ended with exit 124 at its own 900 s cap.

## Live state (read-only)

- Client-side tag for the current activation: `b21c30021da7`.
  - Service host pid `65034` (spawned 14:41:46, `exit.log`), child `65152` (`bun --eval`, CPU active).
  - Log `/run/user/1000/semio-repo/1000/host-b21c30021da7.log` is written every second (last write 14:42:59).
  - No `h-b21c30021da7.sock` and no `host-b21c30021da7.json` yet, so the client cannot attach.
  - The log shows only `repo` / `build:source-admission` / `source.served` plus `repository:command remains active on nx-tooling-provision (exclusive, N s)`.
  - It also repeats `The lease store is 97 % full ... Lease store swept: ... (0.0 MiB freed, 3-4 % free)`.
- Stale or other service hosts, all `repo service host` processes of this checkout, still alive:
  - `2871858` (10 h): log `host-2d692d5a2071.log`. Logs `repository:command still running ... nx-tooling-provision (exclusive, 38685 s)`.
  - `1483031`, `1841105`, `1878492`, `1947059`, `2144037`, `2190016`: all have been alive 12 to 20 h, each has child processes.
  - Log `host-53b8c42fd532.log` (about 101 MB) reports `nx-tooling-provision` exclusive for 71408 s (about 20 h). Its descriptor pid `3567919` is already dead, so this log is orphaned but still written.
- Descriptors: `host-4aae3e160bc3.json` (pid 3438032) and `host-53b8c42fd532.json` (pid 3567919) point to dead pids, so they are stale.
- Lease table (`mcp__repo__leases_status`, `data.waiters`):
  - Five waiters on `nx-tooling-provision` (exclusive, `tool-singleton`, rank 2), all from session `12974ae06f6b`:
    - `2190160` (waited 53133 s)
    - `2872038` (waited 38710 s)
    - `27289` (1956 s)
    - `46411` (1030 s)
    - `65152` (81 s, the current activation)
  - `blockers: []` for every waiter, and no `holders` entry on `nx-tooling-provision`. The holder of that singleton is therefore not visible in the lease table. The only holders are shared `native-fixed-image:/home/deck/semio/.cache/b/t` leases, which the same session's service hosts hold while waiting.
  - `waitFor` and `cycles` are empty, so there is no detected deadlock.
  - Recent order violations are `native-fixed-image:.../b/d` held while `javascript-dependency-bindings` is requested (not enforced). These are a hint of lock-order risk, not the current cause.
- Scheduler (`mcp__repo__scheduler_status`): `running []`, `queued []`, `reservedMegabytes 0`. Idle.
- Capacity: `leases_status` admission shows `floorMegabytes 1981`, `headroomMegabytes 3073`, no admission waiters. `capacity_status` output was too large to read here (saved to `/home/deck/.claude/projects/-home-deck-semio/855c849e-fe7f-4925-87a2-406f81917127/tool-results/mcp-repo-capacity_status-1795354997823-0nr7uj.txt`); not needed for the verdict.
- Disk: `/home` is 97 % full, 16 GB free.
- Load average 7 to 9 on 12 cores, 5.2 GB available RAM.

## Conclusion

- Busy, not wedged: the new service is working (log advancing, CPU active in `65152`). It is blocked on the exclusive `nx-tooling-provision` singleton. That is a serialization wait, not a socket or lock-file fault.
- Starved by stale holders: the singleton was taken by earlier service hosts of the same session (`2871858` and the `1483031` chain). Those hosts have been alive 10 to 20 h, their logs show exclusive activity since early morning, and they are not released. The lease table shows no current holder, so the stale holder is a leftover from an earlier generation.
- Not the cause: scheduler queue (empty), capacity (headroom), disk gate (18 GB free on the last check), the socket path (valid), the client code itself (correct behaviour for a live but silent start).

## What would unblock it

1. Release the `nx-tooling-provision` exclusive hold. This means the stale hosts `2871858` (tree with `2872038`) and `1483031` (tree with `1483188`) must end, or their nx-tooling-provision activity must finish. This is a user or coordinator decision: I did not kill anything.
2. Once the singleton is free, `b21c` should bind its socket and write `host-b21c30021da7.json`, and the waiting activations attach. The `live-ceiling` timeout is 15 minutes from spawn (14:41:46 for `65034`), so the activation may still time out at about 14:57 unless the singleton is released first. A restart with the same tag would re-enter the same queue.
3. Secondary: the lease store is 97 % full and the sweep frees 0 MiB. Compacting or clearing idle lease databases under `/home/deck/semio/.cache/b/l/m/data` would reduce risk, but it is not the direct blocker.

Uncertainty: I did not confirm which of the two queued activations (`64937` 3d, `64949` 5d) attaches to `b21c`. The socket publication and the exact exclusive holder are inferred from logs and the lease table, not from a live probe.
