# quietfc-g: the hot no-op that never got the quiet grant

Status: STAGED, not applied. Batch: `quietfc-g-tools` (apply with `sh quietfc-g-tools/apply.sh apply`, laws with `sh quietfc-g-tools/apply.sh check`; it rebuilds a scratch overlay from the live tree and runs every law one test file at a time).

## Findings (gate-g row 4c, replayed from the recorded classes in `new/.../🧪️tests/🌤️followup/🧫️fixtures/📼️recorded.json`)

1. The quiet forecast needed at least `quietMinimumSamples` weighted quiet runs. After the runs of 16:24 and 16:53 executed the planner (busy), the class of `run @teaching/architecture-course:build` had no quiet history, so the next hot no-op was sized from the busy class: 200% of the default from 12 earlier runs, 7168 MB, and waited 165+ s in the machine-wide queue.
2. Even a quiet grant was held by the soft stops: on the 16 GB profile of the bench2-g log (5686 MB spare, raised pressure, load 52) the 2048 MB quiet request waited with `pressure-warn` exactly like the full request.
3. Multi-worker targets (end-to-end) were admitted as one job of the default size, so the worker policy planned one stack and sharding was inert on an admitted run.

## Change

- Quiet follow-up: a run that follows a finished run of its class within `quietFollowUpMilliseconds` (1 800 000) is forecast quiet because the earlier run left its results cached; the forecast carries `followUpMilliseconds`, the log line is `[capacity] run: forecast quiet because the earlier run of its class finished Ns ago ...` (EN + DE). Outside the window, and without quiet history, the door asks for the full grant again.
- `memory.quietAvoidsSoftStops`: a quiet grant ignores raised pressure, load, open files and attached images; critical pressure, swap and memory still hold it. Both knobs are policy, off in every stored vector (flags off reproduce them byte for byte).
- Envelopes: `policy.envelopes` (`end-to-end`: 4 cores, 6144 MB for `^test-e2e(?:-|$)`) size a target by what its workers hold; the ask now carries `targets`, the plan carries the envelope to the worker policy.
- Declared machine, reconciled with bench2-g's LIVE host-total patch (18:02): no second env. `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` stays the memory declaration (same semantics: only shrinks, malformed means the real host); one new sibling `SEMIO_HOST_CAPACITY_CORES` declares cores. A new `🪞️profile` module (`hostProfileV1`, `hostDeclarationKeyV1`) is the single reader used by the sampler (replacing the direct use of `simulatedHostTotalV1`'s result), the library door, the worker policy, the Bazel server plan and the graph farm. My earlier `SEMIO_HOST_CAPACITY_PROFILE=16g8c` shorthand and its refusal messages were removed.
- Shared sample file: `sample.json` held one sample for all processes, so a per-process declaration was ignored whenever another process had sampled within `sampleMaximumAgeMilliseconds`, and a declaring process would have poisoned the sample of everyone else. The record now names the machine it describes (`declared`, empty when undeclared) and `currentSample` reuses it only for the same declaration; the status reader takes the environment too. A private `SEMIO_HOST_CAPACITY_DIRECTORY` is no longer needed.

## Evidence (scratch overlay of the live tree, all single-file runs)

- Replay of 4c: before, the ask is 7168 MB and the forecast is not quiet; after, quiet with `followUpMilliseconds` 90000, a 2048 MB reservation admitted beside raised pressure and attached-image stops, critical pressure still waits; the next four runs of the chain stay quiet, 1800 s later the door asks for the full grant again.
- Independent Python oracle (`🌤️followup/🐍️oracle`, built on the repo oracles of the decision, forecast and estimates laws; weighted quantile cross-checked against numpy there): 220 decision worlds, 70 forecasts, 8 envelope cases, 12 declaration cases; the flag changes 9 decisions and only for quiet requests. The Ajv schema agreement test covers the new knobs and `Envelope`.
- Test files green: framework followup 9, upgrade 9 + gate 9, estimates 14, sampling 10 + linux 6, gate 23, phases 10 + 4, handover 9, root 21; library followup 6 (the follow-up line EN/DE under a held machine, window expiry, raised pressure, envelope + e2e stacks > 1, declared machine, profile shrinks the reservation), upgrade 4 (updated: after a busy run the next run now follows it and answers at once), arguments 6 (ask fixtures gained `targets`), farm 4, memory-fit 4, restart 2, partial 2, root 16, bazel 4 + 5, workers 11.
- Known red, same on the unmodified base: `🧭️liveness` (2, needs the original parent Script process), `🧪️workers` Cargo case (symlink farm of the overlay), `🔁️restart/📜️child` is a helper.

## Not done / caveats

- Not applied to the live tree and no official run (quiet window rule). The live number (hot no-op end to end) is for gate-g after apply.
- The follow-up window is a policy number (30 min), not tuned; a class whose finished run executed a lot is still forecast quiet inside the window, the full request is asked for on demand as before, so the cost of a wrong forecast is a short upgrade wait.
- `apply` is anchor-based and idempotent; it refuses with the anchor name when a live file moved.

---

# FIX-OF-RED quietfc2 (21:5x, CHECKPOINT state) — g5 upgrade size, quiet-limit, soft-stop grants

State of the live tree: quietfc was REVERTED at 21:48 (revert-quietfc.sh); the live files are the pre-quietfc ones. Nothing of mine is applied, nothing runs, no live file was touched by this lane. The batch below is the WHOLE quietfc change plus the fixes (it replaces the earlier batch; do not apply the old one).

## Root cause of the 35840 MB (verified by replaying the g5 store through the live code, `scratchpad/red/probe.ts`)

- The course build asks for 8 jobs: default = 1536 + 8 x 2048 = 17920 MB. The class ratio is clamped at `maximumRatioBasisPoints` 20000 (200%), so the full request is 3072 + 8 x 4096 = 35840 MB; the share bound (50% of 57344) clamps the wait to 28672 MB.
- The ratios (6177 .. 25000 bp, 90th percentile + 25% margin = clamp) were learned against the default of the jobs the run was GRANTED (one job = 3584 MB: a 5 GB peak is 140%) and are applied to the default of the 8 jobs it ASKS for (17920 MB): about 5x. A Bazel build holds its memory in the server, not per job, so the ratio model does not apply; the class measures 4.4-6.3 GiB.
- The upgrade needs all preferred jobs (`minimumJobs = preferredJobs`), so the inflated request is waited for in full.

## Change (all in `quietfc-g-tools`, anchored edits + oracles; flat batch built from them)

1. Declared ceiling: `policy.classCeilings` (`Ceiling`: name, pattern on the class key, megabytes); production declares `course-build` = 12288 MB for `^run:build:@teaching/architecture-course:build$`. `estimateHostPhasesV1` bounds the ratio so that the request at its preferred jobs never exceeds it (so the door, the upgrade and the phase resizer all see the bound).
2. Measured bound: samples can keep the absolute peak (`peakMegabytes`, written by the library door via `keepPeak`; other callers and all stored vectors unchanged). Once 3 effective runs of the most specific class kept peaks, the request is bounded by their 90th percentile + margin, independent of the granted jobs (`measuredHostPeakV1`). Old samples have no peak, so until then the declared ceiling holds.
3. Quiet limit: `memory.maximumQuiet` is now a real cap on outstanding quiet grants (new reason `quiet-limit`, EN/DE text); before it only widened the queue barrier.
4. Soft-stop grants: `memory.quietSoftStopGrants` (production 1; replaces the boolean `quietAvoidsSoftStops`) = how many quiet grants may be outstanding while a soft stop holds; the upgrade (full request) honours the soft stop like any request. 0 = soft stops hold every quiet grant (the old behaviour; every stored fixture policy says 0).

## Verified (scratch overlay of the live tree, one test file at a time)

- `sh quietfc-g-tools/verify.sh` after `sh rebuild.sh`: both oracles `vectors equal the oracle` (upgrade: 320 decisions, `quietLimit` reached in 32; followup: 220 decisions, 70 forecasts, 8 envelopes, 12 declarations); framework followup 10 (adds the 3-agent law: cap 2 admits 2 of 3 with `quiet-limit`, under raised pressure with 1 soft-stop grant only the first starts, the upgrade waits `pressure-warn`), upgrade 9, estimates 14, sampling 10, gate 23, phases 10, handover 9, root 21; library followup 8 (adds the two g5 laws), upgrade 4, arguments 6, farm 4, memory-fit 4, restart 2, partial 2, phases 3, root 16. 0 failures.
- g5 laws (library `🌤️followup`): with `classCeilings: []` the recorded g5 store reproduces 35840 MB (the red); with the production policy the full request is <= 12288 MB; with 5 measured peaks of 4.4-6.3 GiB and no ceiling it is 5000-8000 MB.
- `stage.py check --root <overlay>`: 31 applied; against the live tree: 31 pending (nothing base-changed at 21:57).

## NOT verified / open

- NOT run on a `mirror-tools/mirror.py` mirror and NOT run live (no official build); the numbers above are from the overlay in the scratchpad.
- The 12288 MB ceiling is an estimate (2x the measured maximum), not a measurement; the first runs after landing teach the measured bound.
- The upgrade still needs all preferred jobs; on a machine that stays full it can wait for 12 GB (not 35 GB). Reducing jobs on upgrade would change the tool's job contract: not done.
- The jobs-dependent ratio denominator is unchanged (only bounded); `~busy`/`~quiet` class ratios keep their meaning.
- `sample.json` keyed by declaration, envelopes, follow-up window: unchanged from the first batch (all inside the same flat batch).

## Exact next commands

```sh
cd /Users/ueli/Documents/semio
C=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
python3 -I $C/quietfc-g-tools/stage.py check            # 31 pending expected
python3 -I $C/quietfc-g-tools/stage.py apply            # all-or-nothing; backups in quietfc-g-tools/staged/backup
sh $C/quietfc-g-tools/rebuild.sh && sh $C/quietfc-g-tools/verify.sh   # laws (overlay), if a file moved: sh $C/quietfc-g-tools/apply.sh apply (anchor route)
python3 -I $C/quietfc-g-tools/stage.py revert           # byte-exact revert
```
If the batch is re-staged after a peer edit: `sh rebuild.sh && python3 -I stage.py build`.
