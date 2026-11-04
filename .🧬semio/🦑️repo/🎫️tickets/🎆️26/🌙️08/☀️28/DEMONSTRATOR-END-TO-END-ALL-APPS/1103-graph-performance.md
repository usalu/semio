# Graph Performance Audit — 2026-11-03

Read-only static audit; no builds, tests, Source changes, cache changes or process actions. Samples supplied by parent (1650-project rebuild, 35 done at ~900s, parent RSS 4.3GiB against ~1.6GiB reservation, ~1.4GiB spare) describe the earlier run, not current liveness.

## Conclusion

No justified Source fix identified. The optional farm already accepts fewer than nine funded workers. Its 3584MiB wait is the minimum ONE-worker cost, not a nine-worker all-or-nothing request. At 1.4GiB spare, even removing its generic fixed overhead would leave the current 2048MiB worker estimate unfunded. Asking for fewer workers cannot resolve that sample. Do not raise budgets, bypass admission or start duplicate rebuilds.

## Exact Capacity Chain

Paths relative to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/:

- 🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs: open() requests preferredJobs only; it does not set minimumJobs to the preference. Own-grant pool starts separately if qualified live allocation funds it. Parent inference proceeds while supplementary capacity waits.
- 🌡️host-capacity/🟦️.ts:94–106 requestBuildFarmCapacityV1 sizes operation exec and sets elastic:true; grant answer exposes handle.jobs, not preferredJobs.
- 🔣️policy.json: exec fixedMegabytes=1536, build per-job estimate=2048. Thus minimum 1536+2048=3584MiB.
- 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts: sizeHostRequestV1 sets minimumJobs:1. evaluateHostCapacityV1 scans preferredJobs down to minimumJobs and admits a reduced count that fits. Elastic waiters follow ordinary requests and do not erect a queue barrier.
- farm/🏊️pool/🟨️.mjs: grantedPoolV1 accepts any answer at least minimumGrantedWorkers (schema=1), starts exactly granted.jobs; ownGrantPoolV1 partitions the qualified live reservation through graphMemoryAllocationV1. No unsupported extra worker is needed for forward progress.

The fixed 1536MiB exec cost might eventually merit a measured farm-specific resource class, because this request supplements an already running parent, but removing it is not a demonstrated necessary fix and would not fund a 2048MiB worker under the reported 1.4GiB spare. Do not make this policy change speculatively.

## Parent Retention

No concrete obsolete-graph retention leak established. Current graph/🟦️.ts already has releaseBuildGraphV1 (around196): it invalidates graph content fields, clears expanded-closure and string-interner caches, yields the event loop and collects. Generation seam release() (around451) releases inference/input memory and removes setup shares when carry is false; stale-project prefetch is chunked and receives this release hook. During discoverBuildGraphPartV1 (around603), stubs/environment, observation rows and newly inferred project results remain required for construction/publication. project-nodes/🟨️.mjs retains setup/caches for reuse and collects results until it returns. These are active-build data; static presence does not prove obsolete ownership.

A single RSS sample cannot distinguish stale graph objects, required setup/results, module baseline, allocator-retained pages, and current project work. Calling releaseBuildGraphV1 on a graph still needed by a caller deliberately causes graph.released-read; it is not a safe speculative performance patch. No heap/retainer evidence supplied demonstrates a safe earlier lifetime boundary.

Memory governor is intentionally a progress governor, not a strict RSS ceiling: 🧠️memory/🟨️.mjs measures startup baseline and charges subsequent growth; decideMemoryAdmissionV1 permits one forced serial project after collection cannot release retained data. Therefore observed total RSS above a reservation does not by itself prove a new farm deadlock or an obsolete graph leak. The baseline subtraction/accounting relationship warrants a separate measured ownership review only if the next run reproduces excess RSS; it is not evidence to alter limits here.

## Existing Test Seams If a Reproduction Requires Change

- requestBuildFarmCapacityV1 accepts HostCapacityPortsV1 (sample/clock/control ports); test with preferred9 and capacity sufficient for one or two workers, asserting reduced jobs and per-worker budget. Independent Python decision oracle exists under host-capacity/🧪️tests; use the same language-neutral capacity facts to validate an implementation change.
- openProjectFarmV1 exposes request, spawnProcess, adopt and cores; inject a capacity answer of jobs1 to prove exactly one hosted pool starts while prepare falls back before grant and close releases pending capacity. Its pool policy already declares minimumGrantedWorkers1.
- releaseBuildGraphV1 has an explicit observable fail-closed seam: released reads throw. Any earlier release change needs a bounded lifecycle test proving no legitimate consumer reads afterward, plus retained-reference evidence before proposing it.

Recommended immediate work: continue the single correctly owned demonstrator baseline and classify its actual terminal result. Do not pause end-to-end app work for an unproved graph redesign.
