# H4 regression audit of wave-1121 landings (wave 1122)

Scope: every wave-1121 or 2026-11-21/22 landing section in `T/landings.md` (W1, R2, R2 follow-up, R1, S1, L, G2, G1, C1, D1). The test files were taken from the paths each section names, and for directory mentions (for example "host-capacity/tests all files", "remote-cache all 26 files") from every `🟦️.ts` test file in that directory. 226 test files were run, each with `cd <its folder> && env -u ... BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 <bun> test ./<file> --timeout 180000`. Logs were kept in `🗑️generated/H4/logs` and removed when this report was written. No source file was changed. Totals: 1746 pass, 37 fail across the run.

Not covered (named in landings but location not found, so not re-run): G1 "idle-grant 12" and "fleet read 18"; C1 "bazel sessions concurrency"; C1 "nx single-flight native client"; L "startup tests" and "doctor tests 17+14+9" beyond the doctor folder; R1 "restart 2" and "sidecar 4" (both could be read two ways, so the whole sidecar test folder was run instead). Recorded counts are per file where the landing gives them, otherwise "n/s" (not stated per file).

## Regressions (new failures)

1. **D1 `🛰️sidecar/🚦️ready/🧪️tests/🟦️.ts`**: 54 pass, 1 fail (recorded 55 pass). First failing test: `the Ready workspace roster follows the unique patch selected by the Nx tool recipe`. First error: `ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/.../🧰️framework/🛍️products/...'` (a path under `🧪️tests/🧰️framework`, so a path built from a relative root). Test file mtime 2026-11-21 21:09:53, git last commit Sat Nov 21 18:13:29 2026 +0100.
2. **G2 `🕸️graph/🕰️generations/🔌️load/🧪️tests/🎫️claim-progress/🟦️.ts`**: 0 pass, 1 fail (recorded 114 for the load folder, claim-progress n/s). The first run without `SEMIO_TEST_ARTIFACT_DIR` was not the cause: the rerun with it set to `🗑️generated/H4/artifacts` fails with `error: Closed host capacity Policy required: /swap/activeBasisPointsPerSecond is required` (the build `🔣️policy.json` has the key at `/hostCapacity/swap`; policy file mtime 2026-11-22 01:11:26, G1 host-capacity module mtime 00:56:00). Test mtime 2026-11-20 07:53:09, git Fri Nov 20 17:40:05 2026 +0100. Cause not diagnosed further.
3. **W1 `🧪️tests/🕸️graph/🌐️source-discovery/🟦️.ts`**: 0 pass, 1 fail (recorded 5 pass). Same error as item 2 with the artifact dir set. Test mtime 2026-11-10 03:14:42, git Wed Nov 18 13:37:51 2026 +0100. Source `🕸️graph/🚪️admission/🟦️.ts` mtime 2026-11-21 18:15:32.
4. **D1 `🚢️fleet-status/🧪️tests/🛡️guard/🟦️.ts`**: 3 pass, 1 fail (recorded fleet-status 12 pass 2 fail, not per file). First failing test: `the command never loads the graph: no module of the graph group, the inputs module or a build invocation is loaded, the schema of the generation store is read as plain JSON`. First error: `error: expect(received).toEqual(expected)` with `- []` and `+ [ ".../🕸️graph/🕰️generations/🔌️load/🟨️.mjs"` (graph module loaded). Test mtime 2026-10-28 15:10:57, git Wed Nov 18 13:37:51 2026 +0100. Source `🚢️fleet-status/🟦️.ts` mtime 2026-11-22 00:49:23.
5. **D1 `🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/🧪️tests/🟦️.ts`**: 14 pass, 1 fail (not recorded per file; disk-recovery recorded 22 pass). First failing test: `a real synthetic fixed image (hdiutil, the product's own creation recipe) > freed blocks inside never return to the host; the retirement does, a held image keeps its bytes (digest law), a killed retirement leaves a usable image or none (kill law)`. First error: `error: Expected and actual values must be numbers or bigints`. Test mtime 2026-11-18 05:35:45. Source `🌡️host-capacity/🧹️disk-recovery/🟦️.ts` mtime 2026-11-22 01:10:58.
6. **R `🚀️offload/🧪️tests/🌐️cluster/☁️fleet-store/🟦️.ts`**: 0 pass, 1 fail (not recorded). First failing test: `a unit a worker compiled is stored once in the fleet cache under the requester's key with its slot, a third checkout takes it without any compiler, and no claim stays held`. First error: `error: the requester compiled nothing itself`. Test mtime 2026-11-14 01:48:10, git Wed Nov 18 13:37:51 2026 +0100. Sources `🗄️cas/🧬️schema/🟦️.ts` mtime 2026-11-21 16:06:29, `⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts` mtime 2026-11-22 01:00:33.
7. **R `🚀️offload/🧪️tests/🌐️cluster/🔏️signed-fleet/🟦️.ts`**: 0 pass, 1 fail (not recorded). First failing test: `signed end to end: an unsigned caller is refused, a unit is offloaded to an agent, uploaded once under the requester's key with its slot through a claim, and a third machine takes it with no compiler`. First error: `error: the requester compiled nothing itself: [{"version":1,...,"kind":"remote-miss",...}]`. Test mtime 2026-11-15 08:14:24, git Wed Nov 18 13:37:51 2026 +0100. Source `🏃️run/🟦️.ts` mtime 2026-11-22 00:56:08.
8. **R `🛰️sidecar/🧪️tests/🕸️graph/🔬️stall/🟦️.ts`**: 8 pass, 1 fail (not recorded). First failing test: `refresh suppression needs live held structural custody in the exact generation root`. First error: `error: structural`. Test mtime 2026-11-20 05:37:49, git Fri Nov 20 17:40:05 2026 +0100.
9. **R `🛰️sidecar/🧪️tests/🧮️closure/🟦️.ts`**: 4 pass, 1 fail (not recorded). First failing test: `a real sidecar restarts for a change in the code it runs and for nothing else: ... an edit of its own closure restarts the role inside the same supervised process, with one log line`. First error: `error: the agent's own module changed`. Test mtime 2026-11-22 01:01:44, git Wed Nov 18 13:37:51 2026 +0100. Source `🛰️sidecar/🟦️.ts` mtime 2026-11-22 01:08:05.
10. **R `🧪️tests/🌐️fleet/🟦️.ts`**: 7 pass, 2 fail (not recorded). First failing test: `every machine appears in the live table with its platform key, capacity, pressure, headroom and worker state, and a machine that falls silent expires` (40182 ms). First error: `error: every worker must register and report its load`. Second failing test: `a worker whose machine reports critical pressure receives nothing, the others share the work and the least loaded one is preferred`. Test mtime 2026-11-13 23:24:59, git Wed Nov 18 13:37:51 2026 +0100.
11. **L `🧭️order/🧪️tests/🧭️sites/🟦️.ts`**: 2 pass, 3 fail (recorded "sites and component-driver 2 source-drift fails", so one extra). First failing test: `the census finds the acquisition sites of the whole build tooling, and the mapping of a native image is one of them with a resolved name`. First error: `error: expect(received).toBe(expected)`.

## Known or baseline failures (matching the landing text)

- C1 `unit-cache/🛰️fleet/🧪️tests/🟦️.ts`: 16 pass, 2 fail (recorded "2 fail, counters schema test"). Source of `error: expect(received).toBe(expected)`, first failing test `the counters are one byte per decision...`.
- C1 `unit-cache/🛰️fleet/🧪️tests/🔢️count/🟦️.ts`: a helper script, not a test (recorded so): no tests, error `TypeError: The "file" argument must be of type string`.
- D1 budget `🧪️tests/🟦️.ts`: 16 pass, 3 fail (recorded 16/3). Error `Original parent Script process invocation required`.
- D1 `🛰️sidecar/🚦️ready/🧪️tests/🔁️resume/🟦️.ts`: 6 pass, 2 fail (recorded 6/2).
- L `🧭️order/🧪️tests/📥️processes/🟦️.ts`: 27 pass, 2 fail (recorded 26-27 pass, 2-3 fail).
- L `🧭️order/🧪️tests/🧩️component-driver/🟦️.ts`: 22 pass, 2 fail (recorded 2 source-drift fails).
- G1 `host-capacity/🧪️tests/🧮️arguments/🟦️.ts`: 0 pass, 1 fail, `Managed test binding SEMIO_TEST_PYTHON_BINARY for python is required` (recorded as not reproducible here).
- G1 `host-capacity/🧪️tests/🔓️sealed-read/🟦️.ts`: 0 pass, 1 fail, same python binding error. Not recorded; environment, not a regression.

## Helper files run as tests (not test files, fail when run directly)

- D1 `🚢️fleet-status/🧪️tests/🛡️guard/📥️worker/🟦️.ts` and `.../🧲️holder/🟦️.ts`: read `process.argv[2]`, so `JSON Parse error`.
- L `🧭️order/🧪️tests/📥️worker/🟦️.ts`: same.
- G1 `host-capacity/🧪️tests/🔁️restart/📜️child/🟦️.ts`: `paths[0]` undefined.

These are subprocess helpers; they should not be in the manifest.

## Per-file results

Status: "now" is the pass/fail count of this run. "recorded" is taken from the landing text; "n/s" means the landing gives no per-file count. Rows for G2 claim-progress and W1 source-discovery are from the artifact-dir rerun (see regressions 2 and 3). Last git commit and file mtime are read-only, taken for the test file.

| landing | file (last 3 path parts) | pass | fail | rc | now status |
|---|---|---|---|---|---|
| C1 | ☁️stage-fleet/🧪️tests/🟦️.ts | 13 | 0 | 0 | ok |
| C1 | 📊️counters/🧪️tests/🟦️.ts | 3 | 0 | 0 | ok |
| C1 | 📤️uploader/🧪️tests/🟦️.ts | 2 | 0 | 0 | ok |
| C1 | 🧪️tests/⚡️pipeline/🟦️.ts | 7 | 0 | 0 | ok |
| C1 | 🧪️tests/🌳️rooted/🟦️.ts | 3 | 0 | 0 | ok |
| C1 | 🧪️tests/🔢️count/🟦️.ts | 0 | 1 | 1 | FAIL |
| C1 | 🛰️fleet/🧪️tests/🟦️.ts | 16 | 2 | 1 | FAIL |
| C1 | 🧪️tests/🧯️fallback/🟦️.ts | 4 | 0 | 0 | ok |
| C1 | 🧪️tests/🌍️machine-law/🟦️.ts | 4 | 0 | 0 | ok |
| C1 | 🧪️tests/🏠️local-key/🟦️.ts | 4 | 0 | 0 | ok |
| C1 | 🪪️identity/🧪️tests/🟦️.ts | 8 | 0 | 0 | ok |
| C1 | 🧪️tests/💽️journal/🟦️.ts | 3 | 0 | 0 | ok |
| C1 | 🎟️claims/🧪️tests/🟦️.ts | 8 | 0 | 0 | ok |
| C1 | 🧪️tests/🔗️restorable/🟦️.ts | 10 | 0 | 0 | ok |
| C1 | 🏷️retention/🧪️tests/🟦️.ts | 15 | 0 | 0 | ok |
| C1 | 📂️entry/🧪️tests/🟦️.ts | 1 | 0 | 0 | ok |
| C1 | 🔏️signing-proxy/🧪️tests/🟦️.ts | 5 | 0 | 0 | ok |
| C1 | 🔐️sealed/🧪️tests/🟦️.ts | 7 | 0 | 0 | ok |
| C1 | 🔬️action-result/🧪️tests/🟦️.ts | 4 | 0 | 0 | ok |
| C1 | 🧪️tests/🔁️restart/🟦️.ts | 3 | 0 | 0 | ok |
| C1 | 🧪️tests/🔐️access/🟦️.ts | 8 | 0 | 0 | ok |
| C1 | 🖥️server/🧪️tests/🟦️.ts | 6 | 0 | 0 | ok |
| C1 | 🧪️tests/🧭️heads/🟦️.ts | 5 | 0 | 0 | ok |
| C1 | 🛰️fleet/🧪️tests/🟦️.ts | 12 | 0 | 0 | ok |
| C1 | 🪪️identity/🧪️tests/🟦️.ts | 9 | 0 | 0 | ok |
| C1 | 🌐️remote-cache/🧪️tests/🟦️.ts | 21 | 0 | 0 | ok |
| C1 | 🧭️peers/🧪️tests/🟦️.ts | 15 | 0 | 0 | ok |
| C1 | 🧪️tests/🕰️long-holder/🟦️.ts | 2 | 0 | 0 | ok |
| C1 | 🚦️single-flight/🧪️tests/🟦️.ts | 14 | 0 | 0 | ok |
| C1 | 🧪️tests/🧰️nx-e2e/🟦️.ts | 2 | 0 | 0 | ok |
| C1 | 🪢️slot-settle/🧪️tests/🟦️.ts | 2 | 0 | 0 | ok |
| C1 | 🧹️collector/🧪️tests/🟦️.ts | 7 | 0 | 0 | ok |
| C1 | 🧪️tests/🔀️nx/🟦️.ts | 1 | 0 | 0 | ok |
| C1 | 🧾️provenance/🧪️tests/🟦️.ts | 2 | 0 | 0 | ok |
| C1 | 🧪️tests/🧮️by-class/🟦️.ts | 1 | 0 | 0 | ok |
| C1 | 🪞️mirror/🧪️tests/🟦️.ts | 11 | 0 | 0 | ok |
| C1 | 🪢️nx-slots/🧪️tests/🟦️.ts | 4 | 0 | 0 | ok |
| C1 | 🗑️reset/🧪️tests/🟦️.ts | 1 | 0 | 0 | ok |
| C1 | 🧪️tests/📥️runtime/🟦️.ts | 0 | 0 | 0 | ok |
| C1 | 💾️storage/🧪️tests/🟦️.ts | 10 | 0 | 0 | ok |
| C1 | 🐙️semio/🧪️tests/🟦️.ts | 36 | 0 | 0 | ok |
| D1 | 📦️budget/🧪️tests/🟦️.ts | 16 | 3 | 1 | FAIL |
| D1 | 🎚️machine/🧪️tests/🟦️.ts | 11 | 0 | 0 | ok |
| D1 | 🧪️tests/📐️model/🟦️.ts | 3 | 0 | 0 | ok |
| D1 | 🧪️tests/🔮️sqlite/🟦️.ts | 10 | 0 | 0 | ok |
| D1 | 🧾️ledger/🧪️tests/🟦️.ts | 13 | 0 | 0 | ok |
| D1 | 🧪️tests/💽️disk-recovery/🟦️.ts | 7 | 0 | 0 | ok |
| D1 | 🧪️tests/🔁️resume/🟦️.ts | 6 | 2 | 1 | FAIL |
| D1 | 🧪️tests/🕳️disk-unknown/🟦️.ts | 6 | 0 | 0 | ok |
| D1 | 🚦️ready/🧪️tests/🟦️.ts | 54 | 1 | 1 | FAIL |
| D1 | 🧪️tests/♻️reclaim/🟦️.ts | 13 | 0 | 0 | ok |
| D1 | 🖼️image-slack/🧪️tests/🟦️.ts | 14 | 1 | 1 | FAIL |
| D1 | 🗃️replica-store/🧪️tests/🟦️.ts | 10 | 0 | 0 | ok |
| D1 | 🧪️tests/📥️need/🟦️.ts | 7 | 0 | 0 | ok |
| D1 | 🧪️tests/🗂️task-views/🟦️.ts | 2 | 0 | 0 | ok |
| D1 | 🧹️disk-recovery/🧪️tests/🟦️.ts | 22 | 0 | 0 | ok |
| D1 | 🛡️guard/📥️worker/🟦️.ts | 0 | 1 | 1 | FAIL |
| D1 | 🧪️tests/🛡️guard/🟦️.ts | 3 | 1 | 1 | FAIL |
| D1 | 🛡️guard/🧲️holder/🟦️.ts | 0 | 1 | 1 | FAIL |
| D1 | 🛡️guard/🪤️spy/🟦️.ts | 0 | 0 | 0 | ok |
| D1 | 🚢️fleet-status/🧪️tests/🟦️.ts | 15 | 0 | 0 | ok |
| D1 | 👯️dedupe/🧪️tests/🟦️.ts | 9 | 0 | 0 | ok |
| D1 | 🧪️tests/⏳️future/🟦️.ts | 3 | 0 | 0 | ok |
| D1 | 🧪️tests/♻️quota/🟦️.ts | 3 | 0 | 0 | ok |
| D1 | 🧪️tests/💰️budget/🟦️.ts | 7 | 0 | 0 | ok |
| D1 | 🧪️tests/📁️directory/🟦️.ts | 3 | 0 | 0 | ok |
| D1 | 🧪️tests/🔬️verify/🟦️.ts | 4 | 0 | 0 | ok |
| D1 | 🧪️tests/🚀️launch/🟦️.ts | 11 | 0 | 0 | ok |
| D1 | 🚀️launch/🧶️nx/🟦️.ts | 3 | 0 | 0 | ok |
| D1 | 🧹️cache-gc/🧪️tests/🟦️.ts | 19 | 0 | 0 | ok |
| D1 | 🧪️tests/🧶️units/🟦️.ts | 6 | 0 | 0 | ok |
| D1 | 🧪️tests/🧹️scratch/🟦️.ts | 2 | 0 | 0 | ok |
| D1 | 🧮️sweep/🧪️tests/🟦️.ts | 4 | 0 | 0 | ok |
| G1 | 🧪️tests/♻️pid-reuse/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🧪️tests/⚓️boot-anchor/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🧪️tests/🌤️followup/🟦️.ts | 11 | 0 | 0 | ok |
| G1 | 🧪️tests/👻️orphan/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🧪️tests/💿️leaked-images/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🧪️tests/📂️roster/🟦️.ts | 6 | 0 | 0 | ok |
| G1 | 🧪️tests/📐️estimates/🟦️.ts | 15 | 0 | 0 | ok |
| G1 | 📡️sampling/🐧️linux/🟦️.ts | 6 | 0 | 0 | ok |
| G1 | 🧪️tests/📡️sampling/🟦️.ts | 12 | 0 | 0 | ok |
| G1 | 🧪️tests/🔋️available/🟦️.ts | 12 | 0 | 0 | ok |
| G1 | 🧪️tests/🔏️fence/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🧪️tests/🔒️acquire/🟦️.ts | 7 | 0 | 0 | ok |
| G1 | 🧪️tests/🔓️release/🟦️.ts | 5 | 0 | 0 | ok |
| G1 | 🧪️tests/🔬️footprint/🟦️.ts | 12 | 0 | 0 | ok |
| G1 | 🧪️tests/🔮️sqlite/🟦️.ts | 0 | 0 | 0 | ok |
| G1 | 🔼️upgrade/🌱️grow/🟦️.ts | 9 | 0 | 0 | ok |
| G1 | 🔼️upgrade/👥️agents/🟦️.ts | 11 | 0 | 0 | ok |
| G1 | 🔼️upgrade/🚪️gate/🟦️.ts | 11 | 0 | 0 | ok |
| G1 | 🧪️tests/🔼️upgrade/🟦️.ts | 10 | 0 | 0 | ok |
| G1 | 🔼️upgrade/🤝️mutual/🟦️.ts | 2 | 0 | 0 | ok |
| G1 | 🧪️tests/🕰️stale-sample/🟦️.ts | 4 | 0 | 0 | ok |
| G1 | 🧪️tests/🚪️gate/🟦️.ts | 33 | 0 | 0 | ok |
| G1 | 🌡️host-capacity/🧪️tests/🟦️.ts | 28 | 0 | 0 | ok |
| G1 | 🤝️handover/📜️child/🟦️.ts | 0 | 0 | 0 | ok |
| G1 | 🧪️tests/🤝️handover/🟦️.ts | 9 | 0 | 0 | ok |
| G1 | 🧪️tests/🤫️quiet/🟦️.ts | 5 | 0 | 0 | ok |
| G1 | 🤲️lend/🚪️gate/🟦️.ts | 1 | 0 | 0 | ok |
| G1 | 🧪️tests/🤲️lend/🟦️.ts | 7 | 0 | 0 | ok |
| G1 | 🧪️tests/🥇️essential/🟦️.ts | 6 | 0 | 0 | ok |
| G1 | 🧪️tests/🧭️liveness/🟦️.ts | 2 | 0 | 0 | ok |
| G1 | 🧪️tests/🧱️settled-read/🟦️.ts | 5 | 0 | 0 | ok |
| G1 | 🧪️tests/🧺️unmanaged/🟦️.ts | 4 | 0 | 0 | ok |
| G1 | 🧪️tests/🧾️blockers/🟦️.ts | 5 | 0 | 0 | ok |
| G1 | 🧪️tests/🪂️partial-start/🟦️.ts | 2 | 0 | 0 | ok |
| G1 | 🪜️phases/🚪️gate/🟦️.ts | 13 | 0 | 0 | ok |
| G1 | 🧪️tests/🪜️phases/🟦️.ts | 14 | 0 | 0 | ok |
| G1 | 🧪️tests/🪟️windows/🟦️.ts | 6 | 0 | 0 | ok |
| G1 | 🧪️tests/🌤️followup/🟦️.ts | 19 | 0 | 0 | ok |
| G1 | 🧪️tests/🌱️grow/🟦️.ts | 1 | 0 | 0 | ok |
| G1 | 🧪️tests/🏊️farm/🟦️.ts | 6 | 0 | 0 | ok |
| G1 | 🧪️tests/👥️agents/🟦️.ts | 2 | 0 | 0 | ok |
| G1 | 🧪️tests/📈️memory-fit/🟦️.ts | 4 | 0 | 0 | ok |
| G1 | 🧪️tests/📉️shrink/🟦️.ts | 6 | 0 | 0 | ok |
| G1 | 🔁️restart/📜️child/🟦️.ts | 0 | 1 | 1 | FAIL |
| G1 | 🧪️tests/🔁️restart/🟦️.ts | 2 | 0 | 0 | ok |
| G1 | 🧪️tests/🔓️sealed-read/🟦️.ts | 0 | 1 | 1 | FAIL |
| G1 | 🧪️tests/🔼️upgrade/🟦️.ts | 4 | 0 | 0 | ok |
| G1 | 🧪️tests/🖥️host-matrix/🟦️.ts | 7 | 0 | 0 | ok |
| G1 | 🧪️tests/🛟️unsatisfiable/🟦️.ts | 13 | 0 | 0 | ok |
| G1 | 🧪️tests/🛠️setup/🟦️.ts | 2 | 0 | 0 | ok |
| G1 | 🧪️tests/🛰️planned-grant/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🌡️host-capacity/🧪️tests/🟦️.ts | 17 | 0 | 0 | ok |
| G1 | 🧪️tests/🤫️quiet/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🧪️tests/🦀️cargo-scaling/🟦️.ts | 9 | 0 | 0 | ok |
| G1 | 🧪️tests/🧪️workers/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🧪️tests/🧮️arguments/🟦️.ts | 0 | 1 | 1 | FAIL |
| G1 | 🧪️tests/🧮️farm-width/🟦️.ts | 7 | 0 | 0 | ok |
| G1 | 🧪️tests/🧮️graph-plan/🟦️.ts | 12 | 0 | 0 | ok |
| G1 | 🧪️tests/🧾️label/🟦️.ts | 2 | 0 | 0 | ok |
| G1 | 🧪️tests/🪜️phases/🟦️.ts | 6 | 0 | 0 | ok |
| G1 | 🧪️tests/🪟️partial/🟦️.ts | 2 | 0 | 0 | ok |
| G1 | 🧪️tests/🧷️history/🟦️.ts | 7 | 0 | 0 | ok |
| G1 | 🧪️tests/🚦️width/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🌡️host-capacity/🧪️tests/🟦️.ts | 5 | 0 | 0 | ok |
| G1 | 🧪️tests/🧮️resources/🟦️.ts | 5 | 0 | 0 | ok |
| G1 | 🧪️tests/🧷️births/🟦️.ts | 3 | 0 | 0 | ok |
| G1 | 🧪️tests/🏋️soak/🟦️.ts | 1 | 0 | 0 | ok |
| G1 | 🗓️scheduler/🧪️tests/🟦️.ts | 26 | 0 | 0 | ok |
| G1 | 🧪️tests/📐️farm-width/🟦️.ts | 1 | 0 | 0 | ok |
| G1 | 🪜️plan-stage/🧪️tests/🟦️.ts | 12 | 0 | 0 | ok |
| G1 | 🧪️tests/🧮️tree-plan/🟦️.ts | 7 | 0 | 0 | ok |
| G1 | 🧪️tests/⌨️command/🟦️.ts | 9 | 0 | 0 | ok |
| G1 | 🧪️tests/🔎️probes/🟦️.ts | 14 | 0 | 0 | ok |
| G1 | 🩺️doctor/🧪️tests/🟦️.ts | 17 | 0 | 0 | ok |
| G2 | 🧪️tests/🌊️streamed-shape/🟦️.ts | 3 | 0 | 0 | ok |
| G2 | 🧪️tests/🎫️claim-progress/🟦️.ts | 0 | 6 | 1 | FAIL |
| G2 | 🧪️tests/🔁️carried-release/🟦️.ts | 2 | 0 | 0 | ok |
| G2 | 🧪️tests/🔒️current-never-builds/🟦️.ts | 8 | 0 | 0 | ok |
| G2 | 🧪️tests/🕊️early-release/🟦️.ts | 3 | 0 | 0 | ok |
| G2 | 🧪️tests/🗝️claim-keys/🟦️.ts | 3 | 0 | 0 | ok |
| G2 | 🧪️tests/🚧️withheld-dependency/🟦️.ts | 20 | 0 | 0 | ok |
| G2 | 🧪️tests/🛫️preflight/🟦️.ts | 2 | 0 | 0 | ok |
| G2 | 🔌️load/🧪️tests/🟦️.ts | 114 | 0 | 0 | ok |
| G2 | 🧪️tests/🧾️head-only/🟦️.ts | 3 | 0 | 0 | ok |
| G2 | 🧪️tests/🪑idle-builder/🟦️.ts | 7 | 0 | 0 | ok |
| G2 | 🧪️tests/🌍️machines/🟦️.ts | 6 | 0 | 0 | ok |
| G2 | 🧪️tests/📏️head-size/🟦️.ts | 2 | 0 | 0 | ok |
| G2 | 🤝️whole-gate/🧪️tests/🟦️.ts | 13 | 0 | 0 | ok |
| G2 | 🧪️tests/✂️after-cut/🟦️.ts | 1 | 0 | 0 | ok |
| G2 | 🧪️tests/🌊️seed-stream/🟦️.ts | 1 | 0 | 0 | ok |
| G2 | 🧪️tests/🌍️two-platforms/🟦️.ts | 10 | 0 | 0 | ok |
| G2 | 🧪️tests/🏭️fleet-machines/🟦️.ts | 5 | 0 | 0 | ok |
| G2 | 🧪️tests/🏷️owner-token/🟦️.ts | 1 | 0 | 0 | ok |
| G2 | 🧪️tests/📮️retry/🟦️.ts | 8 | 0 | 0 | ok |
| G2 | 🧪️tests/🗜️packed/🟦️.ts | 6 | 0 | 0 | ok |
| G2 | 🧪️tests/🚚️closures/🟦️.ts | 3 | 0 | 0 | ok |
| G2 | 🧪️tests/🚥️lookup-fault/🟦️.ts | 2 | 0 | 0 | ok |
| G2 | 📦️project-records/🧪️tests/🟦️.ts | 7 | 0 | 0 | ok |
| G2 | 🧪️tests/🤝️pool-reader/🟦️.ts | 3 | 0 | 0 | ok |
| L | 🧪️tests/🌍️ground/🟦️.ts | 5 | 0 | 0 | ok |
| L | 🧪️tests/📥️processes/🟦️.ts | 27 | 2 | 1 | FAIL |
| L | 🧪️tests/📥️worker/🟦️.ts | 0 | 1 | 1 | FAIL |
| L | 🧪️tests/🕸️wait-for/🟦️.ts | 6 | 0 | 0 | ok |
| L | 🧭️order/🧪️tests/🟦️.ts | 77 | 0 | 0 | ok |
| L | 🧪️tests/🧩️component-driver/🟦️.ts | 22 | 2 | 1 | FAIL |
| L | 🧪️tests/🧭️sites/🟦️.ts | 2 | 3 | 1 | FAIL |
| L | 🧪️tests/🧹️prune/🟦️.ts | 5 | 0 | 0 | ok |
| L | 🧪️tests/🪟️registry-root/🟦️.ts | 2 | 0 | 0 | ok |
| R | ⌨️command/🧪️tests/🟦️.ts | 7 | 0 | 0 | ok |
| R | 🧪️tests/🧭️fleet-endpoint/🟦️.ts | 2 | 0 | 0 | ok |
| R | 👷️worker/🧪️tests/🟦️.ts | 5 | 0 | 0 | ok |
| R | 📡️wire/🧪️tests/🟦️.ts | 8 | 0 | 0 | ok |
| R | 🔒️sandbox/🧪️tests/🟦️.ts | 6 | 0 | 0 | ok |
| R | ⚖️placement/🧪️tests/🟦️.ts | 7 | 0 | 0 | ok |
| R | 🗓️scheduler/🧪️tests/🟦️.ts | 8 | 0 | 0 | ok |
| R | 📜️action/🧪️tests/🟦️.ts | 3 | 0 | 0 | ok |
| R | 🧪️tests/⏱️timeouts/🟦️.ts | 6 | 0 | 0 | ok |
| R | 🌐️cluster/☁️fleet-store/🟦️.ts | 0 | 1 | 1 | FAIL |
| R | 🌐️cluster/🔏️signed-fleet/🟦️.ts | 0 | 1 | 1 | FAIL |
| R | 🧪️tests/🌐️cluster/🟦️.ts | 8 | 0 | 0 | ok |
| R | 🧪️tests/🎯️determinism/🟦️.ts | 6 | 0 | 0 | ok |
| R | 🚀️offload/🧪️tests/🟦️.ts | 28 | 0 | 0 | ok |
| R | 🧪️tests/🧰️toolchain/🟦️.ts | 4 | 0 | 0 | ok |
| R | 🧪️tests/🧰️workspace/🟦️.ts | 0 | 0 | 0 | ok |
| R | 🧪️tests/🩺️failure/🟦️.ts | 2 | 0 | 0 | ok |
| R | 🧪️tests/🪟️volume/🟦️.ts | 2 | 0 | 0 | ok |
| R | 🧪️tests/🏊️pool/🟦️.ts | 7 | 0 | 0 | ok |
| R | 🧪️tests/🔁️restart/🟦️.ts | 22 | 0 | 0 | ok |
| R | 🕸️graph/🌊️wave-rows/🟦️.ts | 3 | 0 | 0 | ok |
| R | 🕸️graph/🌱️fresh-policy/🟦️.ts | 2 | 0 | 0 | ok |
| R | 🕸️graph/🌳️tree/🟦️.ts | 8 | 0 | 0 | ok |
| R | 🕸️graph/💰️wave-costs/🟦️.ts | 2 | 0 | 0 | ok |
| R | 🕸️graph/📡️wave-upload/🟦️.ts | 3 | 0 | 0 | ok |
| R | 🕸️graph/🔬️stall/🟦️.ts | 8 | 1 | 1 | FAIL |
| R | 🕸️graph/🔭️read/🟦️.ts | 18 | 0 | 0 | ok |
| R | 🧪️tests/🕸️graph/🟦️.ts | 9 | 0 | 0 | ok |
| R | 🧪️tests/🚰️drain/🟦️.ts | 6 | 0 | 0 | ok |
| R | 🛰️sidecar/🧪️tests/🟦️.ts | 4 | 0 | 0 | ok |
| R | 🧪️tests/🧮️closure/🟦️.ts | 4 | 1 | 1 | FAIL |
| R | 🌐️fleet/🔁️restart/🟦️.ts | 2 | 0 | 0 | ok |
| R | 🧪️tests/🌐️fleet/🟦️.ts | 7 | 2 | 1 | FAIL |
| R | 🧪️tests/💽️disk/🟦️.ts | 6 | 0 | 0 | ok |
| R | 🧪️tests/📜️scenarios/🟦️.ts | 1 | 0 | 0 | ok |
| R | 🧪️tests/🔎️refusal-reason/🟦️.ts | 3 | 0 | 0 | ok |
| R | 🧪️tests/🔎️why/🟦️.ts | 10 | 0 | 0 | ok |
| R | 🧪️tests/🔧️bazel/🟦️.ts | 0 | 0 | 0 | ok |
| R | 🔧️bazel/🧭️flags/🟦️.ts | 4 | 0 | 0 | ok |
| R | 🌐️remote-execution/🧪️tests/🟦️.ts | 17 | 0 | 0 | ok |
| R | 🧪️tests/🧰️cluster/🟦️.ts | 0 | 0 | 0 | ok |
| R | 🧭️endpoint/🧪️tests/🟦️.ts | 2 | 0 | 0 | ok |
| R | 🧱️actions/🧪️tests/🟦️.ts | 10 | 0 | 0 | ok |
| W1 | 🧪️tests/🔁️observation/🟦️.ts | 4 | 0 | 0 | ok |
| W1 | 🕸️graph/🌐️source-discovery/🟦️.ts | 0 | 1 | 1 | FAIL |
