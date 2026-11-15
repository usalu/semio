# Poisoned Per-Owner Inference Estimates

## Where
- Store: `owner-costs.json` (`.cache/f/graph-costs`) per machine, plus the table the serving machine publishes to the fleet; code in `build/🪪️graph-state/🙋️wanted/💰️cost`. Learned by the memory governor (`🧠️memory/🟨️.mjs`, `costs.record(takenBytes)`) and by the sidecar's wave evidence. The "about 39020 MB" (= 19510 managed x 2) was not in this machine's file: it came from the fleet table (and tree/attributed overcounts of thrashing workers).
- Two refusal paths: `classifyOwnersV1` marked owners heavy on any recorded/seed value, and `planSoloV1`/`fleetReadV1` refused a heavy owner locally when the learned number exceeded the solo grant.

## Fix
- Record: `recordOwnerCostV1`, `recordOwnerCostFileV1` and the recorder clamp every measurement to `ownerCostCeilingV1(host) = host - floorMegabytes`.
- Read: `readOwnerCostFileV1`, `readOwnerCostsV1` (fleet) and `estimateOwnerCostV1` lower every sample to the reader's own ceiling (`clampOwnerCostTableV1`); the recorder's next flush writes the healed values back. No file deletion needed.
- Decay: samples halve per `halfLifeMilliseconds` (3 days, like the estimate store) in `estimateOwnerCostV1` (context with `nowMs`); newest 5 kept as before.
- Saturation: an estimate (recorded or seed) that reaches the ceiling is `saturated` and light: attempted, never heavy.
- No local refusal by a learned number: `planSoloV1` fits every heavy owner when the solo grant is above 0; the governor then stops it only on real growth past the limit (typed `graph.owner-too-heavy` with measured numbers). Refusal remains only when the machine can give one owner nothing.
- Callers pass `ownerCostContextV1()` (host ceiling, now): prefetch, graph host, project-nodes enforcement, sidecar.
- Not done: learning from worker RSS attributable to the owner (the governor gives parent-managed growth; the clamp bounds it), and no new yield/resume mechanism (existing solo + governor stop + retry used).

## Tests
- `💰️cost/🧪️tests`: ceiling/clamp/saturation/heal/decay/recorder write-back, solo plan. `📖️read`: attempt under tight solo grant, refusal at 0. `🕸️graph/🧪️tests/🙋️fleet-read`: updated to the attempt behavior. No oracle in this suite (Ajv validates the schema).
- Run: `SEMIO_HOST_CAPACITY=off repo tools exec bun -- test` over wanted, fleet-read, sidecar graph: all pass (79).

## Fleet
Coordinator: workers were gated on a native-bootstrap marker (see fleet-bootstrap.md). Note the 623 light projects over 44 local would still refuse by count if the fleet does not serve.

## Restart
The service host loads this code at start: a restart is needed for the resident host/sidecar to pick it up; a plain `repo run` client (prefetch path) takes it on the next run.

## Follow-up: host memory injection and the "44 of the stale projects"
- Test determinism: `fleetReadStaleGenerationV1` takes `hostMegabytes?:()=>number` (default the real host); the fleet-read test injects the declared value, so it no longer depends on the real RAM (a declared value could only lower the host, never raise it to 32 GB).
- Why 44: `inferableProjectsV1` = wave size x `maximumLocalWaves` (4). Admitted memory (half of 16 GB, less the 2048 MB floor of the available memory) gave a wave of 11 projects ((admitted - 2200 base) / 104 MB per project, at most 48); 11 x 4 = 44. It is a time-budget design number (waves one wait allows), not a memory limit of the machine.
- It was wrong as a refusal: `fleetReadV1` refused (exit 75) whenever the light projects the fleet did not deliver exceeded that number, after waiting 10 minutes. Now it refuses only when the machine cannot run the smallest wave (`waveProjects===0`, fewer than 8 projects fit); otherwise `localProjects` only decides when to ask the fleet first, and the remainder (missing) is inferred here by the normal inference loop, which the memory governor batches by memory, with the heavy ones solo afterwards. Also the unreachable-fleet branch.
- Tests updated: refusal tests use a machine too small for a wave (2048 MB admitted); new/changed: 5000 stale on 16 GB infers itself, 4096 MB host refuses; 517 projects on a big Linux worker with no fleet are served locally.

## Follow-up 2: describe14 ("can infer only 0 of 674", 464 MB spare)
- Root cause: `fleetAdmittedMegabytesV1` lowered the admitted memory to the memory free at that moment less a 2048 MB floor, so a worker whose test cargo runs held memory had a wave of 0 and `waveProjects===0` refused with exit 75.
- Fix: `fleetAdmittedMegabytesV1`/`fleetGrantMegabytesV1` no longer take the free memory at all: admitted = min(half the host, host less the floor), the same for idle and starved moments; the prefetch reading and its solo grant carry only the host memory. A machine is refused only when it can never fit the smallest wave (4 GB host: wave 0; 8 GB: infers). The inference itself waits on the host capacity gate and the memory governor ("waiting"/"serial" lines with progress), which already queue instead of exiting 75; the governor's in-run solo limit still looks at the memory available when the solo owner starts.
- Tests: `🍏️idle-grant` oracle (python) now computes from the host only; every row (idle and starved, macOS and Linux) infers 48 per wave; new law for the 4 GB / 8 GB boundary; `two-platforms` and fleet-read updated. 85 pass, 0 fail (wanted, fleet-read, sidecar graph).
- Fleet side: "Graph: no builder; 0 computer(s)" in `repo fleet status` is the local graph rebuild claim registry (queues.graph: no graph build is running), not the fleet graph server, so it is no sign that nobody serves. The server is the sidecar `runGraph` (`superviseGraphServerV1`), which claims requests only when the host has `serveTotalMegabytes` (was 24000, so none of the 16 GB machines ever served; the policy now says 15000) and starts a request read with `requestHeadroomMegabytes` (6144) free under normal pressure. Every machine's service host must run this code: g1-m07's `.cache/f/graph-server/serve-state.json` shows its server loop alive; machines still on the old policy stay silent (the requester's "delivered 0 within 301 s" is the silence bound).
- Restart: yes, a service host restart is needed on this machine for the admitted-memory change (the requester path runs in the `repo` client and takes it on the next run, but the host judges stale reads too), and the other fleet machines must restart theirs to serve with the new thresholds.
