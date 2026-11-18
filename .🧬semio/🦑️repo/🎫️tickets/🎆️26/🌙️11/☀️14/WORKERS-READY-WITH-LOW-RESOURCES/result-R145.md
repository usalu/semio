# R145: walls 18 and 19, the two "too heavy" owners and the doomed hand-off (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R145/` (`patches.py` the anchored edits, `land.py`/`restore.py`/`window.sh`/`battery.py` the landing, `new/` the three new tests, `stagetests.py` the staged red/green runs). Landing entry: `landings.md`, "R145 landing 1" (2026-11-17 20:46:27 to 20:47:02 UTC). Measurement scripts (scratch, not kept): the farm child probe of `☀️16/SHOOTING-PLUGIN-END-TO-END/farm-child-probe.mjs` with a heap-type sampler and a physical-footprint driver.

## 1. Why the two owners were refused (measured)

The refusal was never a measurement of this machine. `graph.owner-too-heavy ... s-dev-composition (about 27952 MB), repo-lib (about 9652 MB)` is the declared SEED of the cost policy (`💰️cost/🧬️schema`: 13976 and 4826 managed MB, measured once on a large serving host, times the footprint factor 2). `classifyOwnersV1` called them heavy (seed is a heavy source), `neverHere` compared the seed, in managed bytes, with the managed solo limit of the parent (8612 MB footprint on 16 GB; 1024 MB managed on 8 GB), and the owners phase then waited for a fleet machine and ended with the refusal. No inference of either owner ever started on the lab machine.

Real cost, each owner alone in its own process, cold closure store, this Mac at load 41 to 66:

| owner | declared heap of the child | peak physical footprint | peak managed (JS heap + external) | peak RSS | wall |
|---|---|---|---|---|---|
| s-dev-composition | none (lax GC) | n/a | 2517 MB (body phase) | 3096 MB | 182 s |
| s-dev-composition | 512 MB (`BUN_JSC_forceRAMSize`, what a farm child gets) | **858 MB** | 1087 MB | 1620 MB | 328 s |
| repo-lib | none | 1415 MB (`/usr/bin/time`) | 1796 MB | 2551 MB | 1007 s (210 targets) |
| repo-lib | 512 MB | not finished (stopped by me after 25 min at load 60; managed 1028 MB at the last sample, still in the body) | >1028 MB | n/a | >1500 s |

The seed says 27952 MB, the measurement 858 MB: 32 times too much. Both fit a child on 8 GB (alone cap `farmAloneCapV1(8192)` = 3615 MB of footprint) and on 16 GB (4200 MB).

## 2. The 81256 files

Not inference input. The owner root `✏️s/🧑‍💻dev` holds 479 files (450 tracked); its own discovery facts are ONE file (`📋️project.json`) without targets (measured with `discoverBuildOwnerAdmissionFactsV1`; `repo-lib`: 2 files). The 81256 counted by R140 are the files the ADMISSION CLOSURE attributes to the owner (catalog and plugin inputs found through the provider, `closeBuildOwnerAdmissionV1`), a different stage than inference and not what the child reads. What the inference of `s-dev-composition` reads is the TypeScript program of `✏️s/tsconfig.json`: 17527 files, 14898 analysed (the `input-precision` log), 30 to 41 s cold; that program, not rows, is the peak (heap types at the peak: strings 450k to 680k, objects 210k to 320k, 136k to 190k WeakRefs). Rows are cheap: 2662 observation rows for `s-dev-composition`, 5393 for `repo-lib`.

Scale law (task 3): 100000 file facts recorded through the real record path (`observedFs`, `captureObservedSourceInputsV1`) in a child charged 512 MB grow it by **83 MB** (`👁️observation/🧪️tests/🌊️owner-scale`). This law was GREEN before any change: the row table (R-earlier `🔢️row-cost`, budget 900 bytes per row) already bounds rows regardless of files, so there was no streaming to add at that layer. The files of an owner are bounded; the cost of an owner is its program.

## 3. What changed

Red first (staged copies against the live tree), then landed:
- `📦️project-nodes/🧪️tests/🐘️never-here` (3): a seed never refuses (16 GB and 8 GB, in process and in a child); a recorded cost is judged as footprint against the alone cap of a child (1500 MB-managed fits, 3000 does not, on both hosts) and in process still by the managed solo limit; `ownerMeasuredManagedBytesV1`. Red: `ownerMeasuredManagedBytesV1` missing and the seeds refused both owners. Green after.
- `🏡️resident/🧪️tests/🚫️doomed-handoff` (2): a refusal with `producers` 0 ends the read at once with one load and no hand-off; with machines that delivered, or without the number, the hand-off is as before. Red: the old pool started a second load (test timed out at 180 s). Green after.
- `neverHere(names, childFootprintMegabytes)`: refuses up front only a RECORDED cost of this machine (never a seed); with a farm it compares `megabytes x factor` with `farmAloneCapV1(host)`, the cap the farm enforces itself (`graph.farm-overrun` still sets an owner aside when a child really passes it). The farm child's reported peak (physical footprint) was recorded as if it were managed megabytes and then doubled on read; it is now divided by the factor at the governor (`ownerMeasuredManagedBytesV1`).
- The owners-phase refusal carries `producers` (machines that delivered records); the pool hands off only when it is not 0. With no fleet at all (the lab) a doomed read costs one load plus the silence window of the wait (5 minutes), not three loads of 15 minutes.

Not changed on purpose: the seed values (13976, 4826, ...). They are fixtures in about ten tests (`two-platforms`, `📖️read`, `priority`, `wave-costs`, the small-booking tests) and are now only ordering and offer hints; the right follow-up is to replace both by the measured footprint above divided by 2 (429 for s-dev-composition; repo-lib needs its 512 MB run to finish first) and rewrite those tests together.

## 4. Targets and what is NOT verified

- "Both owners inferred on declared 8 GB / 4 cores": proven only at the level of the numbers (858 MB footprint and, for repo-lib, the 1415 MB of the lax-GC run (the 512 MB run did not finish) fit the 3615 MB alone cap of 8 GB) and of the new laws. A real run of the farm with the declared host (`--memory=8192 --cores=4` slice lab with `--slice=` for these owners) was NOT done: the lab was not started (load 41 to 66, whole-workspace rule); the footprints above come from single-owner child probes with the 512 MB heap declaration, cold closure store, not from the farm itself (a farm child also carries its session).
- A recorded sample from a larger fleet host (the fleet table) can still refuse an owner up front on a small machine: `estimateOwnerCostV1` prefers samples from hosts no bigger than this one only when it has any. Not changed.
- Wall 19 (round 1 says "4453 present" and then infers everything). NOT fixed at the root and not reproduced: reuse is decided by `look` (take pass) and `observeProjectInferenceV1` per owner, not by the file census (`ownerRecordsPresentV1` lists file names only). I did not find the reason the records of round 0 were not valid in round 1; the leads: the record stamp/implementation digest or the registry digest/`commandRevision` of the globals differ between the two worker processes, or a peer edit invalidated the observed rows on the replicated tree (R140 wall 22: a read of 25 minutes loses records to ordinary edits). The only behaviour changed is that a hand-off that cannot succeed (`producers` 0) is no longer started. No test proves "a round after a failure re-infers only the missing owners"; that law needs a run with two real rounds and is for R142 (worker side).
- For R142: the freeze at 4371 to 4389 of 4455 is NOT explained by these two owners being heavy in the farm (they were never started: they were refused up front by the seed); the owners that stay open at the tail are the `alone`/`never` set plus whatever the take pass still looks at. After this landing the two are inferred (alone, one at a time, in a child) where before the read ended with the refusal; the same tail owners will now take their 3 to 17 minutes (cold) each on a loaded machine, which may look like a stall at 4453 of 4455.
- No whole-workspace read, no slice lab, no lab run after the landing.
