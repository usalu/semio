# Graph service diagnosis (opus1122, 2026-11-22 ~15:40)

## Findings
- Endpoint dir on this host is `$XDG_RUNTIME_DIR/semio-repo/1000` (`/run/user/1000/...`), not `/tmp/semio-repo`; the /tmp files are stale leftovers. One service per sealed implementation tag (`h-<tag>.sock`); every source edit changes the tag, so many `service host run` processes pile up.
- No service listened because every `service host run` (log `host-<tag>.log`: "remains active on nx-tooling-provision (exclusive, N s)") was blocked acquiring the exclusive lease `nx-tooling-provision` at phase `source.served`. `repo leases status` showed WAITING with no HELD row.
- The lease is an OS (fcntl) lock on `.cache/b/l/m/data/603cc338....sqlite`. Holder found via `/proc/locks` + `/proc/*/fd`: orphaned pid 4001827 (parent systemd, started Nov 21 11:20, a `repo exec ... rg --files` whose child 4129675 ran `.semio-authorizer-compiler.mjs` in `.cache/b/t/m/nx/.stage-*`), asleep for 28 h holding the lock. Likely hung when /home was 100% full.
- Disk was the trigger (now 98 %, 14 GB free); the code never times out an orphaned holder whose process is alive but idle.

## Done
- `kill 4001827` (single proven-stale PID; child 4129675 exited with it), no pkill, no git changes, no other agent processes touched.
- Live service afterwards: pid 147467 (parent 146711), tag 2bbe90300b8a, socket `/run/user/1000/semio-repo/1000/h-2bbe90300b8a.sock`; `repo service host status` reports it running. The plan phase then progressed (`selected-source-inputs`).
- Left alone: older-implementation `service host run` waiters (27167, 46280, 1841105, 1878492, 1947059, 2144037, 2190016 and children) -- superseded tags that retire themselves; stale `/run/user/1000` and `/tmp` sockets/descriptors of dead pids are replaced on demand.

## Source bug (not fixed)
An alive-but-idle lease holder is never reclaimed and waiters have no upper bound besides the 10-min no-progress watchdog, which only kills the waiter (exit 124) and leaves services without a listener. A fix (holder liveness/progress lease TTL for `nx-tooling-provision`) was not attempted given concurrent builds; see result section below for proof outcome.

## Outcome (15:58)
- After the kill, service 147467 (tag 2bbe90300b8a) came up, answered "graph: plan answered" for my `repo show targets`, then retired "superseded" because the sealed implementation changed again (other agents edit sources). A fresh service (pid 526191) started for the new tag on demand.
- `repo show targets` is now only gated by machine admission: "[capacity] waiting for graph-plan: memory pressure is critical (need 6339 MB, 845 MB available, 1981 MB floor)". Output file: `🗑️generated/opus1122/graph-show-targets.txt` (empty until admitted). Not a graph-service fault.
- Caveat: while many agents edit sources, each tag change forces a service restart; expect repeated cold starts (~5-10 min each).
