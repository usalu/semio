# R89: the warm pass (parallel settle of the stored owners in farm children, durable rows, resume)

Status: LANDED 19:03 UTC (window 19:00-19:05; R90 held the lock until about 19:03, my lock was short). Seven test files green inside the lock (warm, warm-jobs, farm main, slow-last, away, both standing preflights). Read on the workers until 19:10 UTC. Scripts: `T/🧰️scripts/R89/` (`land.py`, `real-warm.ts`, `real-warm.py`, `poll.py`).

## 1. What was built, and what was NOT (read this first)

NOT built: the original ask (the parent holds no per-owner object and merges the shards' products). I measured why it cannot be done in one landing: the look of a stored owner is not a pure function of the record. `observeProjectInferenceV1` inherits the record's observation rows into the validation scope of the round (`retainObservationsV1(parent, rows)`), sets `originalInferencePublications`/`usedInference` for the generation, and `resolveClosureRefsV1` writes the rows into the parent's row log (ids per log). A child can hand back none of this without rebuilding those structures in the parent. So the parent still takes each owner itself: its clean take stays one thread, 11 owners/s at best (R87). What I built is the part that dominates the hours: everything that is NOT the clean take is moved out of the parent into short-lived children, and it is durable.

Built (member `project-nodes/🔱️warm`, schema first `🧬️schema/🔣️.json`):
1. `planWarmLanesV1`: stable lanes (sorted by recorded cost, ties by sha1 of the record key, greedy to the lightest lane); `planWarmWidthV1`: K = min(cores - 1, farm children granted, (plan limit - 1 500 MB parent) / child budget, owners).
2. `warmProjectRecordV1` (project-nodes, runs in a farm child via `service.run` when `job.warm`): reads the stored record with plain file access (`readStoredRecordV1`: the farm child has no custody of the graph store), brings the closure entries named by the record up to date with `settleClosureRowsV1` inside one closure pass, deriving the stale ones in the child's capped process, SAVING them to the closure store (the child binds the store writable; read-only by default is why a first harness run silently lost all 1 264 derived entries), and appends one durable row `{v,n,k,d,o,t,c,m}` to `.cache/f/warm/<pid>.part` (outside the graph store). The row is written after the settle, so a killed child leaves no row for an owner it did not finish.
3. `warmWaveV1` / `runWarmLanesV1` (parent): engages the farm (`farm.engage`), waits at most 90 s for a first child (`farm.whenReady`), runs K lanes side by side, `farm.warm(job)` per owner; summed progress `warmed N of M owners in K children (a now, b durable from an earlier try, c left to the parent; child peak P MB)` in English and German every 5 s, and `sources.owners.warm` in `slice-memory.json`. The parent holds a Set of strings for the durable rows and lanes of names. A failed or lost child leaves the owner to the parent (old path). Off: `SEMIO_GRAPH_WARM=off`.
4. Resume: a restarted try reads the part rows (younger than 20 minutes, same owner name, key digest of [globals, project json] and record file stamp size:mtime) and skips those owners. A torn last line is ignored.
5. Pool: a reply may carry `weight`; a warm job weighs 0.01 for the recycle-by-projects rule (lean children are replaced after 4 owners, which would have restarted a 30-45 s child boot every 4 warm jobs). The memory rule is unchanged.

Files: new `🔱️warm/{🟨️.mjs,🟨️.d.mts,🧬️schema/🔣️.json,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🧮️child.mjs}`, `project-farm/🧪️tests/🔱️warm-jobs/{🟦️.ts,🧫️fixtures/🟨️.mjs}`. Edited (merged with the live text inside the lock): `project-nodes/🟨️.mjs`, `project-farm/🟨️.mjs` + `🟨️.d.mts`, `project-farm/👷️service/🟨️.mjs`, `project-farm/🏊️pool/🟨️.mjs`.

## 2. Laws (red before, green after) with real processes

| law | test | result |
|---|---|---|
| K lanes >= 0.7 K of one lane | `🔱️warm/🧪️tests` "real children: K lanes ..." (real bun children, stdin/stdout protocol, 48 owners x 120 ms spin, K = 4) | 1 lane 5.86 s, 4 lanes 1.47 s, speed-up 3.98 (>= 2.8) |
| the same on the real store, stale path | `real-warm.ts` (R80 session, real `.cache/b` store, 4 processes x 10 owners) vs 1 process x 10 owners | 1 process: 173 s for 10 owners (101 CPU s, 1 264 entries derived); 4 processes: 150/159/56/57 s for 10 each, 219 CPU s in all: 40 owners in 159 s wall = about 4 times the throughput of one process |
| a killed try resumes without re-taking durable owners | "a try killed midway leaves durable rows and the next try warms only the owners without one" (real child killed after 15 of 40) | 15 durable, second try warms exactly the other 25, none twice, 40 distinct rows |
| parent flat for 1 000 owners | "the parent stays flat for 1000 owners ..." | rss growth < 100 MB, absolute < 1.5 GB, summed progress monotone |
| tree memory inside the plan of a 16 GB machine | "the width is one less than the cores ..." | 6 553 MB limit: 2 children (1 500 + 2 x 1 700 = 4 900 MB); every (cores, limit) pair keeps parent + K x child <= limit |
| recycle weight | `project-farm/🧪️tests/🔱️warm-jobs` | BEFORE (old pool, measured before the landing): 120 warm jobs ran on 30 processes; AFTER: 1 process, memory ceiling still recycles |
| persistence of a child's derivation | harness with a writable scratch store: 3 owners derived 63 entries (63 MB written); the same 3 owners again: derived 0, loaded 63, 2.8 s instead of 11.1 s per owner | proved |
| non-writable store (the trap) | harness without the writable binding: 1 264 entries derived twice, 0 files written (`find -mmin -30` on `ce/e`: 0) | why the child binds writable |

## 3. Not verified / what the next round must know

- The parent's own clean take is NOT parallel and NOT faster: 11 owners/s ceiling (R87), about 7 minutes for 4 774 clean owners per try, still reset by every restart. The warm pass makes the tail (stale closure entries) disappear from the parent and makes it durable, so a try that survives the warm pass has a clean take of at most that.
- The design that finishes the job (not built): persist the take product per owner (the resolved project, the observation rows by id over the persisted row dictionary `ce/x`) so a restart redoes only changed owners and children only fill that product store.
- A durable row is a hint: if a source file changed after the row, the parent re-derives that entry in-thread (old path). The row counts 20 minutes.
- Clean-owner throughput of shards on the real store was not measured (the R80 session of 13:26 UTC made every entry stale; measured: stale path, and a clean re-run of 3 heavy owners at 2.8 s each, load average 40).
- `.cache/f/warm` is assumed machine-local (not replicated). Linux/Windows not run.
- Scratch: removed at the end. `pkill`: not used; I killed one hung harness of mine by pid.

## 4. After landing (19:04 to 19:10 UTC, `poll.py` over `slice-memory.json`)

- Only g1-m07 and g1-m08 had restarted on the new code by 19:10 (m01, m02, m03 kept their old tries: their taken counters ran 0.03 to 0.97 owners/s on the old code, m01 460 of 4 774).
- g1-m07: the warm pass runs: `warmed 6 of 4520`, 2 lanes (the memory plan at 16 GB: 2 children), child peak 1 344 MB, tree footprint 0.8 to 1.1 GB, 10 processes with 5.0 GB resident in all (limit of the plan 6 553 MB), 0.02 owners/s. `taken` stays 0 during the pass.
- g1-m08: 5 children are up (1.1 to 1.2 GB each), no owner finished in 6 minutes (`warm: null`), `taken 0`.
- No Mac is `ready: true` (`R54/ready-table.py` 19:09 UTC: m01, m02, m03, m06, m07, m08, m10 `workspace 5/5`, m04, m05 `tools 4/5`, m09 no pool). No try passed the take, so the next phase was not seen.
- Resume after a restart on the new code: not observed (no new-code try was restarted yet); proven only by the law with a killed real child.

## 5. A defect found right after the landing (not fixed: one landing only)

`planWarmLanesV1` places the costliest owners first and the lanes keep that order, so every lane starts with the heaviest Rust crates (30 to 60 s of derivation each, minutes under load 40). The first completed owner takes minutes, `warmed` and `taken` look stuck at 0 for a long time, and the pass blocks the parent's own take meanwhile. The total work is the derivation work the parent used to do in-thread or the main loop gave the farm, now on 2 children instead of 1 thread, but the progress that readiness watches is worse in the first minutes than before.

Fix for the next window (two lines, tests exist): (1) in `planWarmLanesV1` return each lane in ASCENDING cost (`names.reverse()` after the greedy assignment, lane totals do not change); (2) start `warmOwners()` and `resolveOwners()` together (`const warming = warmOwners(); await resolveOwners(); await warming;`) so the parent takes the clean owners at once while the children settle the stale ones and the stage log shows `taken` rising. Until then `SEMIO_GRAPH_WARM=off` restores the old path on a worker.
