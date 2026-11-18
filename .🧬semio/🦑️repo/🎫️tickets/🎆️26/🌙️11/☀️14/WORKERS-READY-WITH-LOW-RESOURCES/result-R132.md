# R132: read farm children starved at niceness 5

## Where the lowered priority comes from
Ancestor chain of the read (13:20 UTC): launchd (1, nice 0) -> pid 77832 `bun .../door/script.ts repo show projects` (ppid 1, nice 5, started 12:55 by a detached shell job) -> 77892 (nice 5) -> nine `graph-farm-worker` children (all nice 5, states RN/UN/SN).
No farm code sets a priority (grep for `setPriority`, `nice`, `taskpolicy`, `PRIO_PROCESS` over the build library: only the fleet pool, the readiness loop and the cargo unit-cache launch lower themselves to 10, and the sidecar graph-server runner is nice 0). The 5 is inherited from the shell that started the read: zsh `BG_NICE` gives background jobs (`nohup ... &`) nice +5, and the read, its farm and every child inherit it. The other orphans at nice 5 (repo-watch, pid 60087) have the same origin; the nice 10 processes are the fleet pool and readiness loop (`setPriority(10)` in the sidecar).

## Can a process raise its priority?
No. Measured on this Mac with the repo bun: a process at nice 5 calling `os.setPriority(0)` gets `EACCES`; there is no `taskpolicy` binary here. macOS (and Linux without CAP_SYS_NICE) only let the superuser lower a niceness value. So neither the read nor its children can undo an inherited nice; the only ways to nice 0 are a start from a process that is itself at 0 (a foreground shell, the graph server's runner, launchd).

## What changed
- Schema (project-farm pool): `priority: { readFarm: 0, fleetPool: 10 }` (priority class per job kind), message `priority` in English and German.
- `claimFarmChildPriorityV1(pid, {get,set}, wanted)`: reads the child's priority, tries to set the declared class when it is lower, reads it again, returns `{before, after, raised, inherited}`. Works where raising is allowed (Windows, root); on macOS/Linux it is refused and `inherited` is true.
- The farm calls it for each child at launch and, once per farm, reports `priority` (niceness, wanted class) in both languages with the advice to start the read in the foreground or through the graph server.
- Test `project-farm/🧪️tests/⏫️child-priority` (4): schema classes; real child at 0 untouched; real child at nice 5 (via `nice -n 5`) claimed once and the refusal named; fake raisable platform ends at 0; a farm whose scripted child is a real nice-5 process reports `{nice:5, wanted:0}` once. Red before (export missing), green after. Landed 14:30 UTC; battery of 8 farm files and both standing preflights green.

## Not done / unverified
- The live read (77832) still runs at nice 5 and cannot be fixed in place; it must be restarted from a foreground shell or through the graph server runner (nice 0). The new message appears only in reads started with the new code.
- The pool and readiness loop still call `setPriority(10)` literally; `fleetPool` in the schema is declared but the sidecar does not read it yet, and "lowered only while an own read or interactive command runs" is not implemented.
- Not done: the door starting a read at niceness > 0 does not hand over to a nice-0 starter (for example a launchd job or the graph server); that would be the real cure for background-shell starts.
