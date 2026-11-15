# Package R2: what the graph service keeps alive between plans (2026-11-15)

T = `.../☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.
**PREPARED, NOT LANDED (coordinator lands, per the changed rule).** `python3 -I T/🧰️scripts/R2/land.py --dry-run` is clean (4 edited files, 6 new, no drift). No graph command ran; only code reading and unit-level scripts on an overlay in scratchpad (deleted).

## Reading of the evidence (verified in code)
- Each slice load runs in its OWN worker process (`LIB/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` runWorker, terminated in `finish`). The three "plans" in round-13 (managed 1040, 1831, 3213 MB) are therefore WAVES of one read inside one worker (`build()` loop, `LIB/🕸️graph/🕰️generations/🟨️.mjs:341-350`, each wave calls `discoverPart` then `host.release({more:true})`). The service process itself only receives the finished slice graph over IPC.
- Between waves `host.release` (`LIB/🕸️graph/🕰️host/🟦️.ts:147`) drops the row table, verified blocks, project images, input memos and collects, and `releaseCutoff` does it again before the next wave. Managed still starts 790 and 1380 MB higher, so what stays is reachable, not garbage.

## Retainers found (code reading; sizes NOT measured on a real worker)
1. `walkFactsByDigestV1` / `walkMembersByDigestV1`, `LIB/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:1032` (used 1039-1041): plain `Map`s keyed by layer digest, one `Set` of walk questions per digest, filled each time a stored closure entry is consumed, never evicted and not cleared by `resetClosureEntriesV1`. The server holds 32 078 layers; a Set per layer fits the 160 k Sets in the heap statistics. UNBOUNDED. Fixed: named bounded caches (65 536 entries) and cleared by the reset.
2. `build()` wave loop keeps `last` (with `plan: found`, its `refresh` map and `current`) alive through the next wave (`generations/🟨️.mjs:341-350`). Not changed (the data is what `hint.plan` reuses; modest).
3. Caches NOT cleared by `releaseInferenceMemoryV1`: `closureEntriesV1` (96 MiB bound), `knownClosureRecipesV1`/`knownClosureInputsV1` (64 MiB each) are bounded and deliberately carried (`more:true`) - at most ~230 MB, they do not explain 1.4 GB. `inferenceStamps`, `loadObservations`, `projectComputations`, `sourceAnalyses` (observation module, lines 41-75) are small keyed maps; not shown to grow.
4. Service side (`LIB/🏡️resident/🏛️service/🌐️host/🟨️.mjs` `run()`): the `answer.slice` graph that arrives from the worker is garbage after the reply, but the service runs with the large collector target (`BUN_JSC_forceRAMSize` of the service) and nothing collected or shrank after the answer, so the pages stayed resident (the ~5 GB at pressure level 2). Plan memo keeps only tasks and a spill reference (bounded by `planMemoEntries`); `workspace.handle` is empty in slice mode. Fixed by the settling below.
- The 1.4 GB inference growth per wave is NOT pinned to a line. The new retained-caches line (below) names the bounded caches after each wave so the next round can.

## What changed (schema first, EN/DE, tests first)
- New member `LIB/🧠️memory/🧽️settle/` (`🧬️schema/🔣️.json` with Policy, bilingual Messages, Result schema; `🟨️.mjs`; `.d.mts`; tests). `settleGraphMemoryV1`: release hook, full GC, `Bun.shrink()` (returns pages to the OS), GC, then ONE line per language: `graph: plan answered, managed X MB, resident Y MB (released Z MB)` / `Graph: Plan beantwortet, verwaltet X MB, resident Y MB (Z MB freigegeben)`. Only on a machine with a plan limit or `SEMIO_GRAPH_PHASES=1` (serving machines untouched). `reportRetainedGraphMemoryV1`: after each wave (`more:true`) one line with managed, resident and the 5 largest bounded caches.
- Service (`🏛️service/🌐️host/🟨️.mjs`): after a job's clients are answered and no other job is running or queued, awaits `ports.settle`. Composition (`🏗️composition/🌐️host/🟨️.mjs`) passes `settleGraphMemoryV1()`.
- Script inputs: the two walk maps are bounded, named caches and emptied by the reset.
- Graph host release: logs the retained caches between waves.

## Measured (overlay, bun 1.4.2, this machine, small synthetic fixture)
- `🏡️resident/🧪️tests/🧽️plan-answered`: two plans in one service process, each with a transient 3 M-object slice graph: start 3.3 MB, plan footprint 113.7 MB, after plan 1 2.6 MB, after plan 2 2.6 MB (after Bun.gc); asserts footprint > 80 MB, after-1 within 40 MB of start, |after-2 - after-1| < 25 MB, exactly two `plan answered` lines matching the pattern.
- `🧠️memory/🧽️settle/🧪️tests`: 5 pass (schema, EN/DE placeholders, result schema, serving machine silent, retained line, REAL 3 M-object heap: managed falls by > 80 MB through a settling).
- `🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🧽️walk-maps`: 1 pass (two named caches, finite bounds, empty after reset).
- Neighbours on the overlay: resident `🧭️plan` 2 pass; resident host test 34 pass / 1 fail where the one fail (`host service held by a long run`) fails identically on the live tree before this change; tests that spawn workers or read live schema files (plan-stage, plan-refusal, slice pool) cannot run on an overlay and are in the landing's test list.
- "Before" number for the real worker (managed 1040 -> 1831 -> 3213 MB, resident 5.4 GB) cannot be reproduced without a graph command; the "after" real number is what the worker's log must show.

## NOT verified
- The line that holds the 1.4 GB per wave (retainer 1 is unbounded but its size is unmeasured); on the real service whether `Bun.shrink` lowers resident on macOS (unit proof is managed heap only); `tsc` of `🕰️host/🟦️.ts`; Windows/Linux.

## What the worker's log must show next
- After each wave: `graph: after the wave managed N MB, resident M MB; largest bounded caches: ...` (names `inference:closure-walk-*`, closure recipes/inputs, with bytes). If managed at the next `plan stage inference starts` is still ~800 MB above the first and none of the named caches is large, the holder is outside the bounded caches (candidate: the `last`/`hint.plan` chain, the generation engine `answers`, or the farm message buffers) and a heap snapshot at that line is the next step.
- On the service after each answered plan: `graph: plan answered, managed X MB, resident Y MB (released Z MB)`; Z should be gigabytes after a big slice, and the resident size should stay near the first plan's.
- Landing restarts the service on every machine (edited files are in the host service revision); coordinator batches it with R1b.

## R2b: the red `host service held by a long run` test (2026-11-15)
- Verdict: a timing/environment-sensitive TEST, not a defect of the hold rule. The first half passes (a held connection keeps the service for 1.2 s). The red half is the retire after release: the idle timer (`LIB/🏡️resident/🏛️service/🌐️host/🟨️.mjs:371`) also asks `runInFlightV1()` (line 18), which reads the REAL `host-capacity/grants` directory of the machine and treats any grant touched in the last 120 s as a live build ("the service outlives a silent run"). On a fleet machine a build always runs, so the service rightly never retires and the test waits 5 s for nothing. That rule is intended (it keeps the cache endpoint alive for a quiet test suite); it is also why the test failed before R2.
- Fix (cause, no sleeps): the probe is a port, `(ports.runInFlight??runInFlightV1)()`; the held-connection law injects `runInFlight:async()=>false`, and a new law injects `async()=>true` and waits (poll, no fixed sleep) until the probe was asked 3 times, asserting the service is still alive: this pins the "in-flight run keeps the service" rule, which had no test. `inProcess` takes optional extra ports.
- Prepared: `T/🧰️scripts/R2b/land.py` (2 files edited by anchors, no new files, dry-run clean, baseline taken after the R2 landing). Overlay run of the host test file: 30 pass, 6 fail, all six being the worker-spawning tests that cannot run on an overlay; both the held-connection law and the new in-flight law pass. NOT landed (coordinator). Landing restarts the service (service file is in the host revision).
