# Audit: g1-server graph read and pool state (read-only)

Snapshot 2026-11-17 00:51 UTC (local clock UTC+1). Times are UTC. No process was signalled, no git command run.

## 1. Process 58106

Command (python3 print of `ps -o command= -p 58106`):

    /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a…/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🕸️graph/🏃️runner/🟦️.ts /Users/ueli/Documents/semio/.cache/f/graph-server/read-record.json /Users/ueli/Documents/semio/.cache/f/graph-server/read.log /Users/ueli/Documents/semio/.cache/f/graph-server/read-watch 1800000 2000 /Users/ueli/Documents/semio request /Users/ueli/.local/bin/repo show projects

Parent chain (`ps -o pid,ppid,etime,command`):

    58106  ppid 1  etime 02:24:21  (runner, command above)
    1      ppid 0  etime 03-08:08:39  /sbin/launchd

- Started 22:27:02 UTC. ppid is 1, so the spawning shell is gone and the process was reparented to launchd.
- Started by the `repo` CLI (`/Users/ueli/.local/bin/repo show projects`, verb `request`). The originating agent shell is not recoverable. `read-record.json` (beatMs 00:51:27 UTC) lists runner 58106, read 58107, reason `request`.
- RSS 21.8 MB, CPU 0.0 %.

`lsof -p 58106 | tail -30` (11 rows):

    bun 58106 cwd  DIR  /Users/ueli/Documents/semio
    bun 58106 3u   KQUEUE count=0, state=0x12
    bun 58106 4w   REG  /Users/ueli/Documents/semio/.cache/f/graph-server/read.log
    bun 58106 5u   unix 0x1dd130615e66eab6 ->0x60521eeebb165b78

Child 58107 holds the other end of that socket, so 58106 is an IPC waiter.

Worker tree:

- 58107 (ppid 58106, 72 MB, 0.1 %): read worker, waits on 58206.
- 58206 (ppid 58107, `bun --eval`, 67.1 % lifetime CPU, RSS 6.8 GB at 02:24:10, 4.6 GB at 02:24:43): does the graph read. Nine `bun --eval` children (30-130 MB, 0-0.7 %) sit under it.

Other bun processes whose argv mentions `repo` (83 bun processes in total, 39 match):

- Resident services: 75460 `script.ts relay --peers ...` (ppid 1, 01-09:50, 0 %, 56 MB). 8961 `fleet/entry/boot.mjs run` (ppid 1, 01-07:04, 0 %, 9.9 MB).
- Clients waiting: 11485 puller (0 %, 73 MB), 86473 (0 %, 59 MB), 42711 (0 %, 89 MB), 80859-80861 (0 %, 19-20 MB), 85320 (0.3 %, 50 MB).
- Doing work (46-96 % CPU, 0.7-1.0 GB each): 57943, 49914, 8032, 9102, 10034, 34985, 39138, 94560, 96365, 8967 (worker of 8961), plus 58206.
- About 20 `ready-tools/grandchild.js` helpers (13-15 MB, 0 %, 7-8 h old), idle.

## 2. Server read log and progress

- No read.log exists under `/Users/Shared/semio-store/v1/origins/g1-server/`. The find for `host-*.log` under `/private/var/folders` returned nothing.
- The graph-server log is `/Users/ueli/Documents/semio/.cache/f/graph-server/read.log` (6355 lines, mtime 00:51:27 UTC, i.e. now).
- Last 15 lines (distinct patterns, ~10 s heartbeats, newest last). Newest line is about 0 s old:

      [repository:command] still running after 8545 s
      [repository:command] still running after 8555 s
      [repo] still working after 8575 s; last phase started: source.served
      ... (heartbeats 8565-8645 s, same two patterns)
      [repo] still working after 8665 s; last phase started: source.served

  The phase has been `source.served` throughout.
- Owner progress (150 `[owners] owners N of 4775 done` lines): the last one, at line 4062, reads `owners 22 of 4775 done: 0 present, 0 fetched from the fleet, 22 inferred here (27 running), 4753 wanted from the fleet, 10 stale in the fleet`. No owner-done line follows in the ~2300 later lines (heartbeats only). Rate over the tail: 0 owners per minute. `pool.json` also says `ownersPerMinute: 0`.
- `serve-state.json` holds the request with `requester: "g1-m10"`.

## 3. pool.json and memory

`/Users/Shared/semio-store/v1/origins/g1-server/pool.json` does NOT say `idle, slots 2, target 2, nothing-fits`. Its current content (written 00:51:27 UTC) is:

    "state": "idle", "role": "puller", "pid": 11485, "slots": 3, "target": 3,
    "reason": "yielding",
    "reasonText": "waiting: the machine's own work holds the capacity; the pool keeps one child and takes what is left",
    "running": 0, "children": 0, "held": 0, "failed": 0, "rssMegabytes": 72, "childMegabytes": 800, "ownersPerMinute": 0

The reason is `yielding`, not `nothing-fits`. The quoted values may come from an earlier write I cannot see.

Current numbers:

- `hw.memsize` 34359738368 (32 GiB), `hw.ncpu` 10.
- `vm_stat` (16 KiB pages): free 4007 (0.07 GB), inactive 654,460 (10.7 GB), active 657,966 (10.8 GB), wired 172,157 (2.8 GB). Free plus inactive is about 10.8 GB.
- `memory_pressure`: "System-wide memory free percentage: 64%".
- Capacity log: `admitted show with 12899 of 14336 MB ... above the 4096 MB floor`.
- One pool child costs 800 MB, so it fits in the 10.8 GB free plus inactive. Memory does not explain the stall. CPU does: about 13 bun build workers run at 46-96 % on 10 cores.
- Top RSS: 58206 at 6.3 GB, then about 13 bun build workers at 0.7-1.0 GB each, plus one git process at 1.0 GB.

## 4. Counts and memory

- `bun test` runners: 0. The 12 regex hits for `bun.*test` are `bun --eval` children of 58206 whose script text contains "test".
- Agent shells: one `claude` (pid 69123, claude-code 2.1.325, Claude desktop, etime 12:38:06) with about five zsh children. `codex`: 0.
- Total RSS of 83 bun processes: 15,622.8 MB (about 15.3 GiB).

## Findings

1. 58106 is a `repo show projects` graph-read runner reparented to launchd, started 22:27:02 UTC by the `repo` CLI. It waits on 58107, whose `bun --eval` child 58206 holds 4.6-6.8 GB and does the work.
2. The server read is alive (heartbeat now) but not progressing: `owners 22 of 4775 done`, flat for ~2300 log lines, 0 per minute, 4753 wanted and 0 fetched.
3. pool.json says `idle, slots 3, target 3, yielding`, not `nothing-fits`. About 10.8 GB is free plus inactive, far above the 4 GB floor and 800 MB child, so the wait is CPU contention from the machine's own builds, not memory.
4. No `bun test` runners are running. One Claude desktop agent (pid 69123), no codex. Bun total RSS is 15.6 GB, mostly the graph-read tree and build workers.
