# E80 heavy-owner-tail: result (16:50)

## Verdict
The hypothesis (".lake / 144k generated files are walked by the inputs expansion or closure walk of @leutwiler/realparts-of-powers-z-n") is REFUTED by measurement. No product code needed a change for it; I landed a law test that pins the exclusion and report what the tail really is.

## Measured (isolated, private staging, live tree read-only, load 47-55 on 10 cores)
Tool: `bun 🔬️owner-harness/📜️cli.mjs <workspace> --late 👴️leutwiler/💤️realparts-of-powers-z-n` (owner-harness, no store custody, no graph read touched).
- Run 1: wall 279.5 s, cpu 57.6 s (user 39.8 + sys 19.9), 226 rows, record 14 041 bytes, digest 67a5d478...
- Run 2 (`--cpu-prof`): wall 146.6 s, cpu 52.0 s, same digest and rows.
- CPU profile (158 s, 84 k samples): `readdirSync` total 0.33 s (0.2 %) - a 144 511-file tree is never listed. The time is readFileSync 17.9 % incl., TypeScript parse (parseListElement 19 % incl.), JSON.stringify 7.4 %, gc 5.7 %, existsSync/lstatSync ~8 %, observation row verification (observationLayerHoldsAsyncV1 14.5 %), closure layer save/zlib (7 %), wasmCompileOutputIndexV1 5.5 % (owner-discovery wasm index, 8.8 s), moveRest/portable bundles 4 %. i.e. the cold import-closure walk and verification of the owner's command script (📜️script.ts -> routing, print execution, process packages), not any glob expansion.
- Code reading agrees: `readBuildSourceInputMatchesV1` (owner-discovery) never descends into `POLICY.generatedDirectories` (the `ignored` set; `.lake` is one), `sourceTreeFilterV1` (source-names) drops generated/machine-local directories for every observed listing and for `declaredModuleGlobsV1`, owner-discovery walks (lines 48/65) skip them, `inferProjectInputs` appends `!{projectRoot}/**/.lake/**/*` and `!{workspaceRoot}/<owner>/**/.lake/**/*` negations. The line E54 quoted (`isOpaque... || owned(path) && generated part`, project-assembly/inputs line ~341) only decides which OUTPUT negations are emitted, it is not a walk.

## What the 10-33 min tail really is (log evidence, read 15:45 `read.1795358713888.log`)
- The owner was 2105 s: ~1700 s with the generic stage text "inferring in this process" (this label is the DEFAULT stage; the parent computes nothing, the work is in the owner pool / farm child, so the text misleads) then ~400 s "resolving closure references in the parent".
- Its fleet record "was found in the fleet cache and taken, but its record does not hold against this tree" (log line 1635, 5th stale owner) so it was re-inferred from cold; the cold cost in isolation is ~150-280 s wall / 52-58 s CPU, so ~1700 s means a 6-10x slowdown from load (load 40-55 on 10 cores), memory-headroom waits ("waiting for memory headroom 4136 of 4154 MB reserved" in the 14:18 read) and repeated closure derivation rounds. It is last in every wave only because its closure (print execution stack) is the heaviest cold derivation and the farm schedules it behind the 27 cargo owners.
- In the current read 11561 (16:45) owners are 110/121 and the lean owner is NOT the tail: the two longest owners are cargo plugins (animate-plugin 1070 s, norm-energy-plugin 1065 s); the log reports `1 stale in the fleet`.

## Landed
- New law test `🧰️framework/.../🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🧪️tests/🚫️generated-trees/🟦️.ts`: a project with a 4 000-file `🧘️lean/.lake/packages/mathlib` tree expands to the SAME matched inputs and reads the SAME number of directories as the same project without it; oracle = independent `fs.readdirSync(recursive)` enumeration filtered by `picomatch` (third-party); policy list read from the real cache policy (`generatedDirectories` contains `.lake`). Passes 1/0 via `repo test-file`; negative control with an emptied generated list fails (the `.lake` files are matched). 100k scale is the same code path (cost is O(authored entries)); not run at 100k to keep the Mac quiet.
- landings.md section "Wave 1122 E80" appended. No product code, no locks, no shared-file edits.

## Task 3: which recorded rows moved (13 of 53 re-inferred)
NOT determined per owner; honest gap:
- The read prints no row for fleet-fetched stale records: adoption summary says `111 stale (-)` / `116 stale (-)` (the kind tally is empty), and the only per-owner line is `[owners] owner X was found in the fleet cache and taken, but its record does not hold against this tree` (2 lines in the current read: @semio-tech/architect-js, @leutwiler/realparts-of-powers-z-n). The first-moved-row line (`ledger.staleRowLine`, "the first row of the record ... does not hold here is {kind} {path}") exists but is only emitted on the lookQueue and farm-verdict paths (project-nodes line ~711/720), never on the take-through-`takeAway` path that produces `BUILD_TAKE_MISS` (project-nodes line 767/776, `recordTakeLookFailureV1`); no such line appears in any of the 5 kept read logs.
- Plan-stage evidence in the current read: `16712 stale facts (analysis 7314, readdir 4421, file 3289, typescriptText 875, readdirNames 359, exists 126, field 87, lstat 74, digest 55, ...) leave 5097 projects stale`, plus roster adds of `units/sealed/a9809de7a4e3d601/...` (sealed copies of runtime sources). Files edited in the last hour on the lean script's import path: `🧰️framework/🔨️modules/🏃️process/🔒️leases/🎛️control`, `.../🧭️routing/📥️invocation/🏃️process/🟦️.ts`, `.../🐕️watchdog`, `.../📖️texts/🔣️.json`, `.../🏗️native-build/💾️storage/...` - all of these prefixes are already declared output-neutral scheduling domains (`📥️source/🔏️closure/🗓️scheduling/🔣️.json`: leases, process-supervision, progress-reporting, store-custody), so the closure LIST must not change; whether the observation ROWS for them still invalidate the owner records (derived walk fact views: `WALK_VIEW_POLICY_V1`, script-inputs line ~459) is the thing to test next: a record that carries the unpruned walk answer instead of the scheduling-pruned view would be recomputed after every landing in these folders = the "computed twice" key bug you suspect. Not proven either way.
- Decisive next step (small, needs a quiet Mac): make the take path report the first moved row (carry `staleRow` of the verifying child on `BUILD_TAKE_MISS`, call `adopt.staleRow(kind)`), rerun one read slice, then read the `stale (kind n)` tally; or run `verifyStoredProjectRecordV1` offline for the 13 names. See W/findings-E80.md.

## NOT verified
- Real parent-side cost of `resolving closure references` (400 s) was not reproduced: the harness resolves in the child session with a private closure store.
- 100k-file scale of the new test; the effect of load on the 1700 s (no controlled load experiment).
- Which rows moved per owner (above).

## Others must know
- Do not spend time on `.lake` for the tail; it is not read. Keep the generated-trees test as the guard.
- The "inferring in this process (one thread...)" parked text is the default label and is wrong for farm/pool owners: it hides whether an owner waits for a child, the memory governor or compute.
