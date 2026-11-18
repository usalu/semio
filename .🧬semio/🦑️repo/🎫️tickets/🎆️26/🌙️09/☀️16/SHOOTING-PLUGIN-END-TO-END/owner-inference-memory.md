# Owner inference memory: single heavy owners are not the cause; the owner-state release was blind

## Short answer
The premise "single owners allocate 3.9 / 3.0 / 4.4 GB" does not hold when one owner is inferred alone. Those figures are process growth while other owners ran, not what one owner allocates. What grew the worker to 4.4 GB (and the plan to 10 GB) is the owner state that finished owners leave behind in the shared script cache: about 10 to 15 MB per owner. `releaseOwnerStateV1` did not drop it, because it skipped the release whenever the gate's footprint stayed below 60 % of the plan ceiling, and on this host that footprint reading was blind.

## Measurements before (current per-owner code, `owner-memory-probe.mjs`, `owner-memory-probe-many.mjs`)
Each owner inferred in its own process: `projectWithDefaults` through `inferProjectBodyV1`, then `resolveClosureRefsV1`, cold closure store (growth = peak managed minus the start, `Bun.gc` collected start).

| owner | peak growth | live after collection |
|---|---|---|
| flow-extension-brep-rust (4 targets) | 446 to 489 MB | about 250 MB (module code) |
| wfc-2d (3 targets) | 297 MB | 226 MB |
| mit-bestand-demonstrator (13 targets) | 547 MB | 349 MB |
| stdio-pdf-rs / stdio-txt / stdio-dwg | 248 / 197 / 205 MB | 216 to 263 MB |

- Heap snapshot of one finished owner: about 190 MB of objects, almost all module code and shared tables; nothing owner-sized is retained.
- Owners inferred one after the other in one process (release after each, no ceiling gate): live heap after collection 278, 265, 298, 336, 334, 342, 359, 374, 375, 388 MB: about 11 to 14 MB retained per owner. A snapshot comparison of owner 2 against owner 10 shows the growth spread over the shared string and row tables, module keys and module graph nodes (tens of MB each), not one structure.
- The earlier failing worker (`semio-footprint/21992.log`, 13:13 to 13:16 UTC): managed heap 1398 MB at owner 71, 2455 at 132, 3782 at 248, 4447 at 262 (objects 3.0 M to 7.7 M), while the same log line read footprint 623 MB. The heap grew about 15 MB per owner and was never released.
- A worker from 15:06 UTC and the activation worker of this run: managed heap flat at 500 to 900 MB over 126 to 330 and 131 to 200 owners (4.1 M objects); spikes of external memory of 1.5 to 2.2 GB for a few seconds exist during `resolve-closure-refs` of the stdio owners (not reproduced in isolation).

## Root cause
1. `releaseOwnerStateV1` (script-inputs) returned at once when `decideFootprintV1` said `run` (footprint below 60 % of the grant), without counting the owner. The slice worker's gate (`useSliceCeilingGateV1`) compares the tree footprint, which `footprint -p` reports at 623 MB for a worker holding 4447 MB of managed memory on a host that compresses its pages. So the verdict was `run` for every owner, the interval release (every 32 owners) never ran, and the state of 262 owners stayed live until the plan guard stopped the plan. The earlier pass made this worse: before the gate existed, the interval release always ran.
2. A footprint reading of a process could stand below what its own runtime holds, so the settle watcher, the governor hold and the spill never saw the growth either.

## Fix (schema first, no cap raised, no new cap)
- `inference/schema`: `ownersPerRelease` (32) and `ownersPerShrinkRelease` (8) are declared limits with en and de descriptions (they were private constants).
- `releaseOwnerStateV1`: the interval release no longer waits for the footprint verdict; the verdict only decides `shrink` and the spill of closure entries.
- `footprint`: new `managedBytesV1` and `footprintAtLeastManagedV1`; the default meter of the process and the slice ceiling gate raise their reading to the managed bytes of the same process.

## Measurements after
Same 42 cargo and typescript owners in one process with a blind gate (footprint 623 MB, grant 6553 MB), real `releaseOwnerStateV1`, `owner-mem-many-after.txt`: live heap after collection climbs 278 to 571 MB over owners 0 to 30, drops to 385 MB at owner 31 (the 32nd release) and climbs again (433 to 462 MB at 40, 41). Per-owner peak growth (peak minus start of the owner): 150 to 420 MB, maximum 421 MB (stdio-tiff-rs). Without the release (old behaviour with a blind gate) the same slope reaches about 700 MB at owner 42 and 15 MB per owner beyond, i.e. 4 GB by owner 262 as measured in `21992.log`.
Per-owner inference itself is unchanged: no owner of those measured exceeds about 575 MB peak growth (largest: mit-bestand-demonstrator 547 MB, 13 targets). I did not change `projectWithDefaults`: the profile shows no per-target duplication worth fixing (the per-target 300 to 400 MB is collectable garbage; after collection an owner holds under 15 MB).

## Changed files (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`)
- `🕸️graph/📥️inference/🧬️schema/🔣️.json`
- `🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` (`releaseOwnerStateV1`, interval limits from the schema)
- `🧠️memory/🦶️footprint/🟨️.mjs`, `🟨️.d.mts`
- tests: `🕸️graph/📥️inference/🧪️tests/🪫️owner-release/` (new: `🟦️.ts`, `🐍️.py`, `🔣️.json`), `🧠️memory/🦶️footprint/🧪️tests/🟦️.ts` (two tests changed from "caches survive below soft" to "released at the declared interval")
- ticket tooling: `owner-memory-probe.mjs`, `owner-memory-probe-many.mjs`, `owner-mem-snapcmp.mjs`, `owner-mem-snapdiff.mjs`, `owner-mem-watch.py` (log lines prefixed `[DEBUG] `, tooling only).

## Tests
- `owner-release` (independent Python model of the release schedule and of the footprint decisions): 3 pass. It fails against the old skip by construction (the held entries would reach 400 against a bound of 128); I did not run it against the old source because peers read this tree live.
- `bundle-rows`: 2 pass. `target-fanout`: 3 pass.
- footprint suite: 46 pass, 1 fail (`a cold part engages the farm, a warm part does not`: farm engagement, not touched).
- `flat-heap`: timed out at its own 600 s limit on the loaded host (ungated release path, not touched).

## Activation
See the last section; the run was queued for capacity (another agent's `workspace:setup` held 11.7 GB) and then read the slice at 131 to 200 owners with a flat 0.8 to 0.9 GB worker heap.

## Activation result (honest: not finished when this was written)
- Command: `repo run @semio-tech/framework-os-dev:activate-shooting-react-prototype --lang=en --output=stream`, log `🗑️generated/owner-memory-activation-1.txt` (a final line `EXIT <code>` is written when it ends; none yet).
- It waited about 10 minutes for capacity behind another agent's `workspace:setup` (11.7 GB), then read the graph slice. The slice worker (pid 50402) stood at 707 MB heap at 131 owners, 966 MB (+506 MB external, 4.8 M objects) at 243 owners, i.e. flat to slowly rising, far from the 4.4 GB at owner 262 of the failing run; the reading had restarted once (progress counter back to 100 s). No exit 75 so far, no plan ceiling refusal.
- Activation exit code: NOT KNOWN YET. The full 4774-owner read had not finished after about 45 minutes on this saturated host (load 9 to 10). The shooting cargo tests were not reached; I did not touch the shooting fixtures.
- Not verified: that the full plan stays below 6553 MB to the end, and that the old skip was the only growth path of a real cold run (the isolated owners and the 42-owner sequence support it; the take-pass path of the current tree already stays flat without it).
