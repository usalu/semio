# E40 result: cost per crate-only owner

## Verdict
The goal (5x owners/min for crate-only owners) is NOT reachable by yields or a lean path on a warm process, because the measured warm cost is not there. Measured (script `T/🧰️scripts/E40/measure.test.ts`, 20 crate-only + 10 full owners, load ~22, one warm process, via `repo test-file`):
- Owner probe (`discoverBuildOwnerAdmissionFactsV1`, no targets): 5 ms wall median, crate-only and full alike; 5 forced progress advances per owner (E33's ~8). Cold first owner 400 ms.
- Selected inference with no targets (what a Cargo path-dependency owner costs): 121 ms wall / 95 ms CPU median (about 500 owners/min).
- Full inference of a crate-only owner: 1.6 s wall / 0.88 s CPU median; of an ordinary owner 1.5 s / 0.95 s. First owner of a process 17-22 s (cold walk, E1).
So crate-only owners are already as cheap as they can be where they are dependencies; the full record (native source hashing, build/check/test targets) costs the same for every owner kind and is the record content, so no lean path with identical output exists for it. The live 14-41 owners/min (E33) is therefore not probe or inference CPU on a warm process: it is memo row verification, memory thrash and queueing (not measured here).

## Changed (lock graph, T/landings.md "Wave 1122 E40")
- `🧭️script-inputs/🟨️.mjs` `yieldInferenceV1`: the 200 ms throttle was keyed on the per-owner context object (new for every owner, so every owner's first yield published); now keyed on the shared invocation: progress publishes on a time budget across owners.
- `🔎️owner-discovery/🟨️.mjs` `stage()`: no longer forces the advance.
- Effect measured: advances per probe 5 -> 0 (median); probe wall unchanged (5 ms), so the gain is fewer publishes (matters under load, where each waits for a continuation), not warm speed.
- Differential: probe facts (digest of files, dependencies, project) before vs after equal for all 30 sampled owners (20 crate-only, 10 full).
- Test `🕸️graph/📥️inference/🧪️tests/⏳️yield-budget` 1/0: advances equal an independent model of the time budget (forced calls always advance). Dependants: admission, load inventory, owner-harness pass.

## Not done / not verified
- (2) lean crate-only inference path and (3) batched filesystem probes: not built, no measured cost to remove (probe 5 ms; selected path 0.12 s). Full-record differential not run: records were not persisted in my runs (racy-file rule, rows 0), so no byte-equal record oracle; the only differential is for probe facts.
- The harness gap: 5 of 20 crate-only owners failed in my script with "Native dependency has no build project owner" (script projectsByRoot lacks native roots); the harness itself (`🔬️owner-harness`) still reads only 📋️project.json owners.
- Not measured: row verification (`buildGraphSourceFactsHoldV1`) per owner on the live read, which is the likely real cost; next lane should measure it with a real selected read under the 0.5 load.
- No live read; no loaded-host numbers for the publish saving.
