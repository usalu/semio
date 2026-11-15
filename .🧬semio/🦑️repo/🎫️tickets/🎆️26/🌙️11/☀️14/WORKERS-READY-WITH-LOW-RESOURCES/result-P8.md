# Package P8: projects are weighed by memory, one owner at a time (2026-11-14, landed 22:32 to 22:37 CET)

T = `…/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. Landings in `T/landings.md` ("Package P8 landing" 22:32, "landing 2" 22:35, "landing 3" 22:37); scripts `T/🧰️scripts/P8`, `P8b`, `P8c` (`land.py --rollback <dir>`; originals in `T/🗑️generated/P8-landing-223251`, `P8b-landing-223539`, `P8c-landing-223703`). No `repo`, graph, build or cargo command was run, nothing was killed; the server's state was only read.

## Verdicts

1. **Heavy owners (cost = managed MB the owner added above the surviving floor of its wave; physical footprint is about 2x, P4):**

   | # | Owner | Cost | Basis |
   |---|---|---|---|
   | 1 | `@semio-tech/s-dev-composition` | 13 976 MB | measured: wave 154, the ONLY project at the peak, peak 18 814 MB, floor 4 838 MB (m02 learned 17 182 MB for its kind the same evening) |
   | 2 | `@semio-tech/repo-lib` | 4 826 MB | measured: wave 153, sole project at the peak, 9 649 - 4 823 |
   | 3 | `@semio-tech/framework-os-dev` | about 3 840 MB | wave 168 with os-kernel: 11 580 - 3 903 = 7 677 MB together, split evenly; 3 634 rows, 185 s on the workers |
   | 4 | `@semio-tech/framework-os-kernel` | about 3 840 MB | same wave; 423 rows, 70 s |
   | 5 | `@semio-tech/framework-rs` | 3 259 MB | measured: wave 172, sole project, 7 616 - 4 357; 36 979 rows, 46 s |
   | 6 | `workspace` (root, `root:language:cargo`) | 3 100 MB | `owner-peaks.json` of the server: 122, 78, 295, 1 449, 2 636, 3 084 MB (six samples, growing); 18 748 rows; wave 144 (group of 4): 4 033 MB |
   | 7-8 | `framework-renderer-wgpu`, `framework-plugin` | about 1 435 MB each | wave 171 together 2 870 MB; 1 784 / 1 365 rows |
   | 9-14 | `framework-pack-rs` (505 rows), `framework-replication-rs` (188), `framework-process` (166), `framework-os` (151), `framework-job-rs` (99 rows, 116 s), `framework` (72 rows, 68 s) | not measured | slow or many rows on the workers |
   | 15-30 | `repo` (231 targets, 221 KB project.json), `repo-lib` (210), `os-hub` (119), `value-rs` (76), `repo-policy` (43 targets, 12 broad globs), `stdio-plugin`, `repo-test-domain`, `ui-rs`, `energy-model-rs`, `framework-renderer-react`, `repo-dashboard-rs`, `print`, `os-flow-browser-package`, `@teaching/architecture-course`, `@teaching/proctor`, `framework-async(-rs)` | not measured | largest by target count / broad globs (script `P8/top30.py`) |

   Only rows 1 to 8 carry a memory number; 9 to 30 are a ranking by what was observed or by structure, not a cost. Structure predicts badly: `s-dev-composition` has a 10 KB project.json and 17 targets.
   **What makes them heavy:** not their file lists. `s-dev-composition` declares input globs `✏️s/**/*` (125 594 files) and the plugin tree (6 071 files) for 17 targets; 131 665 paths are about 30 MB, the owner took 14 000 MB, so the cost is the work done over those inputs (source closures, TypeScript programs), which I cannot profile without running the graph. `workspace` reads every Cargo manifest of the repository (18 748 rows). The dominant cost of a 16 GB machine is not an owner at all: the parent holds **3.0 to 5.9 GB managed after a full collection** on the server (waves 141 to 182) and the largest process of m02 was 3.14 to 3.74 GB resident in the four tries that only waited; every inference comes on top of that.
2. **Why the recorded peaks are 0 to 25 MB (and 1 to 3 GB for the root):** (a) they are learned per **kind** with a window of the last 8 samples (`np/owner-peaks.json`: `project` [236, 813, 6, 0, 0, 220, 0, 0] MB), so the 14 GB sample of `s-dev-composition` left the window after 8 light owners of the same kind; (b) a sample is the owner's **attributed growth of the parent's managed bytes** (`heapUsed + external`, shared by the concurrent owners in proportion, bounded by the parent's own growth, a collection mid-run refunds it): what a farm worker, a child process or native memory takes is never in it; (c) the unit of the planner is a count (`planStaleReadV1`: 192 projects, 48 per wave), so no recorded number was ever consulted. m02's "17182 of 1024 MB reserved" is that kind estimate applied to every `project` owner.
3. **Policy now (landed, tested): by memory, per owner.** A reader weighs every stale project (own recorded samples, else measured seed, else kind, else 820 MB), compares it with what its admitted memory can give ONE project, and asks the fleet for every project that does not fit and WAITS for it however few remain. "2 projects left" with `workspace` among them is refused after the bound (or at once when the fleet is unreachable), naming the project.
4. **Serving order now: heavy first, by cost.** The priority file lists the projects the requesters call heavy first, then by cost (own table of the server, which it also publishes into the fleet every 60 s on change), equal cost in the order of the older request.
5. **Enforcement now: one project that outgrows the grant is stopped** by the governor that measures it, before the host runs short, released through its own scope, recorded under its own name, and the read ends with a typed refusal (exit 75) naming it. Proven with synthetic inferences and with real 32 MB buffers.
6. **Reduction at the source: not done.** No source change can be shown to bring `s-dev-composition` or `workspace` under 2 GB without a heap profile, and I may not run the graph. The fleet path (3, 4) is the answer; numbers above.

## What was built (all schema first, EN and DE)

- `🪪️graph-state/🙋️wanted/💰️cost/` (new): `🧬️schema/🔣️.json` (Policy: factor 2, margin 25 %, 5 samples, unknown 820 MB, kinds, seeds, floor 3 800 MB; Table; Texts), `🟦️.ts`: `recordOwnerCostV1`, `mergeOwnerCostTablesV1`, `estimateOwnerCostV1`, `ownerGrantV1` (admitted memory less `max(base 2 200, floor 3 800)`, / 2 / 1.25: **16 GB machine = 1 756 managed MB = 3 512 MB physical per project**, 8 GB machine 118 (so its parent budget of 1 024 MB applies), a serving machine none), `classifyOwnersV1`, `orderByCostV1`, `waveEvidenceV1`, `createOwnerCostRecorderV1` (only notable owners, written merged at most every 2 s, at once when an owner was stopped), file `<root>/.cache/f/graph-costs/owner-costs.json`, publish/read through the fleet exchange.
- Read policy: `📖️read` takes `estimate`, `🟦️.ts` `planStaleReadV1` takes `heavy`, `awaitWantedV1.settled` gets the missing NAMES, the request carries `heavy`; texts in cost schema; the refusal `build.graph.wanted-expired` gets a `heavy` sentence (catalog, resident message, failure parser). `🤝️prefetch` merges the local file and the fleet's table.
- Server: `runWantedServerV1(costs)` orders the steering, publishes the table; sidecar `inferGraphReadV1(costPath)` records the cost a wave proves when exactly one project was at its peak.
- Enforcement: new `🧠️memory/🎯️owner-scope/` (an `AsyncLocalStorage` scope per owner, `throwIfOwnerExceededV1`); the governor gets `ownerLimitBytes`, `measure`, `collect`, `costs`, `onStop`, per-owner `key`, message `stopped`; `script-inputs` `yieldInferenceV1` calls `throwIfOwnerExceededV1()` first; `project-nodes` runs every owner in its scope under its name, records, and after the phase throws `graph.owner-too-heavy` (catalog, resident message, parser, EN/DE). A stop is first confirmed by a full collection (garbage is no growth; at most one per second), alone the owner's own growth counts, with others its attributed share; a stopped owner is not learned into its kind.

## How it is proven

| Test | What |
|---|---|
| `💰️cost/🧪️tests` 11 (Ajv cross-check of the table schema) | grant numbers, recorded vs seed vs kind, measured owners split light/heavy on 16 GB and all light on 32 GB, 300 light + 1 heavy, order, wave evidence, file/merge/damage, texts EN+DE, fleet publish/read, recorder |
| `🙋️wanted/🧪️tests/📖️read` 10 | two projects left of which one is `workspace`: published with `heavy`, refused naming it (EN, DE); heavy arrives then the light rest goes local; served heavy not asked again; unreachable fleet refuses a single heavy at once; serving machine and no-estimate behave as before |
| `🛠️server/💰️priority` 3 | priority `["d","z","c","b","e","a"]` by heavy flag then cost; no table; table reaches the fleet once per change |
| `🧠️memory/🧪️tests/🛑️overrun` 6 | synthetic growth 100 MB/10 ms stopped long before 5 GB; garbage freed by the collection is not a stop; light owner beside completes and is learned; several owners; real `ArrayBuffer`s past 192 MB stopped before 1 GB |
| `📦️project-nodes/🧪️tests/🛑️too-heavy` 4 | wiring (limit, recorder, scope, refusal after the phase), the real `yieldInferenceV1` throws only in the stopped owner's scope, 16 GB = 1 756 MB / 8 GB 1 024 / serving none, recorder file |
| `sidecar/…/💰️wave-costs` 2, `🙋️fleet-read` 5, `🧯️failure` 6 | sole-project wave recorded (`s-dev-composition` 13 976), root alone refused at once, refusal fields |
Neighbours as before (reds identical to the untouched tree): memory main 1 (`psProcessRowsV1`), governed-dispatch 1, project-nodes `🚪️away` 1 (P7 in flight), resident `🌐️host` 1 (timing). All 30 files of the landing ran green apart from those.

## What a 16 GB worker's log shows next

Plan (EN; DE `Graph: … zu schwer für diese Maschine …`): `graph: 3 stale projects, 0 taken from the fleet, 2 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), workspace (about 6200 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 3`, then `graph: waiting for the fleet: x of 3 projects arrived`; the wait ends as soon as the heavy ones arrived and the rest fits, never earlier. If the heavy ones do not come: `… 3 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), workspace (about 6200 MB), which this machine must never infer itself …`, exit 75 (`unserved`). If an unknown owner outgrows the grant while inferring: `graph: project @semio-tech/x (project) grew by 1790 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here`, the read ends with `The project graph cannot be read on this machine yet: @semio-tech/x (about 3580 MB) need(s) more memory than this machine can give one project (1756 MB) … run the command again, the fleet is asked for them first`, exit 75; the next try asks the fleet first. On the server: `graph-server: wave …` lines unchanged, plus `.cache/f/graph-costs/owner-costs.json` growing and the table in the fleet (`readOwnerCostsV1`).

## Not verified

- Everything live: no read has run since the landing; the first wave after the restart (21:33, 21:35, 21:37 UTC) will show whether the server's read starts (the agent log showed the restarts and a new refresh at 21:33:12, no failure line yet). The owner-costs file does not exist yet.
- The costs in rows 3, 4, 7, 8 are halves of pair peaks; the seeds are wrong in the safe direction (a wrongly heavy owner is inferred by the server anyway, a wrongly light one is stopped).
- The floor of 3 800 MB is the resident size of m02's waiting process (3.14 to 3.74 GB), not a footprint of an inferring run; the 1 756 MB grant may still be too high or too low on a real 16 GB Mac. `SEMIO_GRAPH_OWNER_LIMIT=off` disables the stop.
- Stopping works at the yield points of the inference (every 200 ms of async work); synchronous work longer than a few seconds or code that swallows the error cannot be stopped (the scope stays exceeded, so the next yield throws again).
- The stop ends the read (refusal 75) instead of re-asking the fleet inside the same process; the ask happens at the start of the next try.
- A stop is judged by the managed counters of the process (`heapUsed + external`), which count a buffer twice; native memory and farm workers are not in it.
- `tsc`, Windows, Linux.

## For others

- Landing changed `script-inputs/🟨️.mjs` (one line), `project-nodes/🟨️.mjs` (by an anchored edit applied to the live text at landing time, `P8/pn_transform.py`, because package P7 has `[DEBUG]` trace lines and `queuedAt` on the same loop line; my edit keeps them), the memory governor, the wanted modules, the sidecar graph role (the agent restarts) and the refusal texts.
- P3/P4/N8 read-path: `fleetReadV1` is unchanged for a serving machine and for callers without `estimate`.
- Rollback: `python3 T/🧰️scripts/P8*/land.py --rollback <dir>` (P8c first, then P8b, then P8).
