# repo graph reading 2: os-hub inference and cache-decision refusal


## Defect (b): "Repository target cache decision refused: side-effecting/writes-build-output"

Root cause: `🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/📋️project.json`, target `export-models` (line 47, added 21:59 today by another agent, uncommitted) declares `metadata.semio.cache = {class: side-effecting, reason: writes-build-output}`. The closed schema `🏗️build/🏃️execution/🧬️schema/💾️cache.json` admits only `publishes-live-state, publishes-remote, mutates-workspace, mutates-host-store` for that class (139 other declarations use `mutates-workspace`). It is not caused by the `📜️script.ts` edit (that diff only adds one test source to the `graph-memory` list). Read-only inference was classified correctly; the refusal was thrown as a plain `Error`, so the CLI printed "Internal error" and the whole graph read died, and because `discoverBuildProjectsV1` awaits all in-flight workers after the first rejection (`await Promise.allSettled(workers)` in `📦️project-nodes/🟨️.mjs`), the command also waited for the project that was still running (os-hub).

Fix: `🕸️graph/📥️inference/🎯️target-policy/🟨️.mjs` now throws a named `invalid` refusal `build.target.cache-decision-refused` carrying `{target, decision, admitted}` (project:target identity passed from `🧱️project-assembly/🟨️.mjs`), text in `📖️texts/🔣️.json` (en, de). Tests: `🧪️tests/💾️cache/🟦️.ts` (new test of code, key, values) and `🎯️target-policy/🧪️tests/🟦️.ts` (error projection back to the language-agnostic corpus text; 22/22 pass).
The project.json value itself must be changed to `mutates-workspace` by its owner (not touched: constraint).

## Defect (a): os-hub inference

Measured in isolation (`repo-graph-2-infer-one.mjs`, farm-service path, warm shared closures): 31 s CPU before, 13 s after. Profile (`--cpu-prof`): 55.7 % self time in `withoutSchedulingV1` (`🧭️script-inputs/🟨️.mjs`), which re-filtered the whole command-closure walk against the scheduling prefixes for every provider port x view (memoized only for the standard prefix set). os-hub is the pathological project because it names every provider port. Fix: memoize per (closure, prefix set); regression test `🧭️script-inputs/🧪️tests/without-scheduling.test.ts`; `.d.mts` signature corrected.
The 605 s in the full run is not reproducible in isolation (13 s); see Verification for what was observed in the full run.

## Verification (one full `repo show projects`, 23:14:38 to 23:39:04)

- Wall 1465.6 s, user 1000.8 s, sys 299.2 s, peak RSS 11.76 GB (`/usr/bin/time -l`, includes the inference child; the child showed 6.8 to 7.9 GB in flight). Exit 1.
- Inference reached 451 of 4754 projects (about 18 to 30 projects/min, cold, one parent process; the host capacity plan of the graph child was jobs 1, cpu 2, 5120 MB fixed, no worker farm started).
- Failure is now the named refusal, not "Internal error": `Target @teaching/staffage:export-models declares the cache decision side-effecting/writes-build-output, which the closed cache decision schema refuses; admitted: publishes-live-state, publishes-remote, mutates-workspace, mutates-host-store` then `repository:command failed with exit code 1`.
- os-hub observation in the real run: it started at 23:31 and showed 60 rows at 0 s, 263 rows at 105 s, 532 at 280 s, then 3463 rows from 315 s until the end; "1 running" from project 451 on. The 451st project is the staffage project that raised the refusal; after the first rejection `discoverBuildProjectsV1` drains all running loops (`await Promise.allSettled(workers)`), so the remaining ~8 minutes were os-hub finishing alone (closure-entry row validation heartbeats `build:closure-entry-rows`, process 9 % CPU, no child process, no lock). So defect (a) in the observed run was os-hub's own slow inference (about 480 s under a cold parent) plus the unbounded drain after the failure of defect (b); the dominant CPU term of os-hub is fixed, the drain and the cold parent throughput are open.

## `repo run` results

NOT RUN. `repo run @semio-tech/framework-3d-js:test` and `repo run @teaching/staffage:test` both need the complete graph, which cannot be built while `📋️project.json` of staffage carries `writes-build-output` (file under `🎓️teaching`, off limits for this task; still unchanged at 23:40, mtime 21:59). One-word fix by its owner: `"reason": "mutates-workspace"` at line 47. After that, `repo show projects` needs again about 1 to 3 hours cold (see Open), because no generation is published by a failed run.

## Open

- Cold graph throughput (about 20 projects/min, parent RSS up to 8 GB, farm not engaged under a jobs:1 host plan). Needs a JS profile of the parent loop; warm per-project cost in a single worker-like process is only 0.3 s, so the parent-side per-project overhead (observation capture, `consistentObservationsV1`, record store, late closure resolution) is the next target.
- On the first failing project the graph drain should cancel in-flight project inference instead of waiting for it (`stopped=true; await Promise.allSettled(workers)` in `📦️project-nodes/🟨️.mjs`); not changed (needs a per-job abort signal).
- Root `package.json`/`bun.lock` unchanged since 21:48; no dependency sync was needed.
- Ticket helpers: `repo-graph-2-infer-one.mjs` (farm-service path, `INFER_F_FAITHFUL=1`), `repo-graph-2-late-one.mjs` (body + `resolveClosureRefsV1`, late mode); logs in `🗑️generated/repo-graph-2/`.
