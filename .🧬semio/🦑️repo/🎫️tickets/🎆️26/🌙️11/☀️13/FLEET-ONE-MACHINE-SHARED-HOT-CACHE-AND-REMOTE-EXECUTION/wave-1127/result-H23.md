# H23: m04 disk runaway, on-demand scopes, reds

## m04 disk (evidence S/origins/g1-m04)
- Free bytes were flat 15.96 GB for 24 min before the stage started; the dependencies install itself used about 2 GB; m04 passed `dependencies` (about 01:13) and runs `tools` (4/5), 17 GB free, 299k free inodes of 5,966,400.
- H7's census was blind: one shared 400k-entry budget, so after `d` every partition was empty and partial. Fixed (per-row budget). First true census: `.cache/b` partitions all small (d 7.3, g 5.8, t 6.0, r 2.2 GB), `root/.git` 11.4 GB, `root/.🧬semio/🦑️repo/🎫️tickets` 91 then 106 GB (partial, logical bytes: hard links/sparse may inflate). The Mac's replicated tickets are 5.3 GB. Holder is NOT yet named below the tickets level; census depth 4 landed and arrives with m04's next refresh. Coordinator is investigating the tickets folder.
- Replica daemon (`🤝️shared-worktree/🔁️replica`) has no free-space check on apply: open, not changed.

## Landed
1. Disk guard on running stages (`🚦️ready`): floor max(2048 MB, 1 % of volume) or max(15000, 0.25 %) inodes, checked every 5 s; stage stopped with code `disk-guard` (bilingual), short back-off, restart only when the stage need is free. First version used 3 % and would have stopped m04 (13 GB of 475); corrected to 1 % before it mattered. Does not cover non-stage writers (pool, offloaded cargo, replica).
2. Census: per-row budgets, rows `cache/*` and `root/*`, `volume` (free/total bytes and inodes), holder depth 4.
3. Zero-touch scopes: `ensureBuildScopesV1` (any install mode; receipt quick path costs one receipt read when the sealed receipt already has the owners) called by the Nx backend before Nx for run/run-many/affected/exec with the owners of the transitive project closure; Bazel uses the same function.
4. Offline help: `repo <op> --help` through `📜️script.ts` answered before admission (132 s -> 4 s).
5. Ready-suite timing tests: stall limits, loop length and test timeouts scale with load (`slow`), idle fixture sleeps 400 s.

## Proof
disk-guard 4/0, census 5/0, nx on-demand-scope 2/0 (installer mocked: closure owners of `app` = `✏️s/app`,`✏️s/lib`,`🧰️x/base`, ensure runs before nx; Python BFS cross-check), help-offline 1/0 (111 s at load 65), operations suite 16/0, ready suite earlier 55/3 (the 3 timing tests), dangling imports 0.

## Unverified
- Full 92-file typecheck not re-run after the last test fix.
- No real install of an uninstalled scope; no macOS worker re-ran `dependencies` after a lockfile change yet.
- The guard has not fired live; the 115 GB writer is not proven.

## Update 02:0x
m04 census at depth 4: `tickets/🎆️26/🌙️11` = 107.4 GB in 212,156 files (partial) on m04, while the same month is about 1.5 GB on the coordinator Mac. The 115 GB is therefore in the November ticket folders of m04 only. holderDepth raised to 7 so the next census (about 10 min after m04's restart) names the ticket folder (`tickets/🎆️26/🌙️11/☀️dd/<TICKET>`).

## Final status (typecheck, timing tests)
- Typecheck of the 10 touched files (92 files checked, 255 s): 1 error, in my own `🛑️disk-guard` test (TS2769), fixed; re-check of that file alone: passed, 0 errors. No error in product files.
- Ready-suite timing tests rerun (foreground, load about 56-65, bounds scaled by `slow`): "elapsedMs floats" PASS, "spends processor time" PASS, "detached grandchild ... reaps by pid" FAIL. A single rerun of the grandchild test alone was still running when this was written; reason not yet read. It is the test where `launch.js` helper processes must register within the stage's stall window under load; the earlier failure was "found pids undefined", i.e. the helpers had not started in time. Not fixed; open.
- Census depth: m04 depth-4 census says `tickets/🎆️26/🌙️11` = 107 GB / 212k files; H28 owns it.

## Grandchild test, final
Alone, with stall limit 5000*slow and timeout 120000*slow (360 s): FAILS by timeout at 360 s ("killed 1 dangling process"); it is a hang, not a tight bound (first load run: assertion "pids undefined" at 5.6 s with the 5 s limit; with the scaled limit it hangs instead). Not diagnosed; candidates: the second `runReadyStageV1` (tools, 2.5 s limit, not scaled) or `gone(orphan)` waiting on helpers that never register under load. The bound change did not fix it; treat as a real open fault (owner: ready/orphan reaping), not as a load flake. The other two timing tests pass.
