# R209: "Unknown closure reference publication" on a row log (2026-11-18)

## Cause (verified in code and on disk)
- Throw site: `role()` in `LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts` (the closure role whitelist, last line `throw Error("Unknown closure reference publication: "+name)`). Untouched since Nov 14.
- Writer: `discoverBuildGraphV1` in `LIB/🏗️build/🕸️graph/🧭️discovery/🟦️.ts` (row log of R130/R200, `rowLogDirectoryV1`): spills the rows of a graph slice to `<graph writer inference dir>/rl/rl-<pid>-<time36>-<serial>.rows` (a disk, deliberately inside the graph store image). File kind: process-private temporary, NOT a publication. It removes itself on process exit and a sweep removes logs older than 1 h when the next log is opened in that directory. A killed process (SIGKILL, the build budget kill) therefore leaves it behind.
- Why it reaches the check: the closure proof (`repo reset`, headroom recovery, and the graph read's maintenance pass) enumerates every file under the image root, calls `role()` on each, and would then read and JSON-parse the file. An orphan row log from a killed read has no role, so the whole store is refused.
- Leak count on this machine (read-only): exactly 1 file, `.cache/b/g/m/inference/selections/8d8f21d7.../rl/rl-38258-mwm347au-0.rows`, 25,166,354 bytes (24 MiB), pid 38258 dead, last written 13:03 UTC. No other `rl-*.rows` in `.cache` or the store. I did not delete it (not mine); the new sweep removes it at the next row log (dead pid).

## Decision
Row logs are process-private temporaries: never a closure role, member, reference, nor read, nor retired. Registering a role would make the proof read a 25 MB JSON-lines spill and retire another process' live temp. Fix:
1. closure proof: `spill(name)` (shape `inference/[selections|contexts/<64hex>/]rl/rl-<pid>-<time36>-<serial>.rows`) is skipped by `directories()` (must be a regular file, else refusal).
2. one name factory `rowLogFileNameV1(pid,time,serial)` in the row-log module; discovery uses it (no second spelling of the shape).
3. leak: the sweep also removes a log whose owner pid (from the name) is dead and not this process, not only after 1 h.

## Laws (new `closures/🧪️tests/🌀️row-log-spill`, 4 tests)
- exact refusal: the writer's name throws `Unknown closure reference publication: ...` from `role()` and is a spill in all three writer directories (red on the base: `spill` does not exist);
- near misses are no spill (.tmp, wrong dir, short hash, uppercase, subdir);
- general registry law: every store file shape is exactly one of known closure role or spill;
- source law: no source of the build spells `"rl-"` except the row-log module.

## Landing
- 14:00:00 try: law 4, row-log 8, flat-memory 1, stream 6, fleet-marker 3 green, then `closures/🧪️tests/📁️census` red (1 pass 2 fail: `Original parent Script process invocation required`; fails identically on the unchanged base, needs a repo-launched test process; `♻️groups` also 10 fail on the base). The script restored the base exactly. Those two (and `🔏️admission`, 0 tests) were dropped from the battery.
- 14:01:11 to 14:01:46 UTC, still inside the 14:00 window, owner claude-R209, released by exit trap: law 4, row-log 8, flat-memory 1, stream 6, fleet-marker 3, generation 1, inference 2, all green, import smoke ok. Script `🧰️scripts/R209/land.sh` (base compare after the lock, restore rehearsed on a scratch mirror with forced red, staged first with the bun transpiler cache disabled), patch `patch.py`, base copies `*.base.*`, law `law/row-log-spill`.
- Edited: closures `🟦️.ts` (spill), row-log `🟨️.mjs` + `🟨️.d.mts` (`rowLogFileNameV1`, dead-pid sweep), discovery `🟦️.ts`. Effect from the next process that loads them; running reads keep old code.

## Verification on this machine
- The orphan `rl-38258-mwm347au-0.rows` (24 MiB) was swept (dead pid) by the first row log opened after the landing: gone at 14:13.
- `nice -n 10 repo show project repo-mcp`, foreground, started 14:02 UTC: NOT finished. It re-inferred the stale graph ("Inferring projects: 89 of 4785", 11 running, longest 775 s) after the owner pass (shared closures primed in 241 s, `build:closure-entry-rows` ran), load average 40, ran 1,300 s without any error text and without "Unknown closure" before I stopped my wrapper (TaskStop at 14:22). So: the previous failure point (~110 s into the read) is passed, but a complete green read is NOT proven. The `repo show` read is itself a full graph inference when the stored graph is stale; there is no smaller selected read offered by the binary. The row log of my killed run (pid 39126, 20 MB) stays as a leak until the next row log opens; with the fix it neither blocks the closure proof nor lives longer than one run.
- Not verified: the closure proof against a real store with a row log present (the law covers the classification and the source, not a kernel-exclusive image capture); whether other unknown file kinds exist in the store (the source law only guards the row log shape).

## Task 4: did a real machine hit it
`grep` over `/Users/Shared/semio-store/v1/origins/*` (all files, g1-m01 to g1-m10 incl. g1-m05) for "Unknown closure reference" and `rl-*.rows`: 0 hits since 11:00 UTC (those folders hold pool/view/ready-log files, not full read logs; the six ready workers do not re-read). Only this machine (R206, two attempts 13:00 to 13:25) is known to have hit it.
