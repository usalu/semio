# E33 result: closure size of the layout activation read

## Verdict
1. The closure is NOT > 2,000 owners by declared dependencies. Declared edges give ~74 owners; the walk admits far more through dynamic edges. I could NOT identify the bridge edge (see Not verified); I landed a provenance trace so the next run names it.
2. The falling rate is memory thrash plus bursty probe cost, not quadratic growth.
3. "boot-recovery:observe:0/0, held 96 min" is a stale label, not recovery work and not a lease problem.

## 1. Closure evidence
- Static script (scratchpad, Bun TOML over all 4,829 registry owners; Cargo path deps incl. workspace inheritance, package.json deps, implicitDependencies, nativeRoot): closure of the real seeds (framework-os-dev with its 57 admitted dependencies, renderer-wgpu, plugin-registry, plugin-web, os-infinite, ui-rs, hub-layout-component) = 73-74 owners; layout component alone 7-8.
- Sealed facts of framework-os-dev with `activate-layout-wgpu-prototype` (selected scope b34e30d0): 57 dependencies, 3,074 files in 60 owners. Its declared dependsOn: prepare-layout-wgpu-prototype (8 entries) plus ui-rs:generate. Earlier sealed activation/test closures: 60-73 owners.
- The layout read's own checkpoint (`.cache/b/g/m/inference/selections/1485623892.../oc`, 1,086 records, 1,054 distinct owners at 08:13): every record is a probe (manifest only, no dependencies). Of those 1,054, only 58 are in the static closure; 996 are not statically reachable from the seeds (families: semio-s-artifact-* 198+, os 357, ui, value, job, pack, replication). Those 1,054 are a declared-dependency component of 1,080 owners with no declared edge from the seed closure into it (BRIDGE list empty). So the walk enters it through a dynamic edge: a TS import resolved to another owner's file (`addFile` then `sourceOwner` then `admit`, plus `addDependency` and continued import walk), a generated-output producer, or a catalog file. Candidate: the plugin catalog (`session-layout` depends on `s-dev-composition:generate`, the whole catalog), but not proven.
- No `projects:"*"` dependsOn exists in any project.json. Catalog providers: 1.
- Not an edge-semantics bug I could prove, so I did not change semantics. Rule of thumb for the fix once the hub is named: a file-derived admission (`file` kind) of an owner that is neither a dependency nor needed by a task should not expand it.

## 2. Rate
- Owner checks over time (log activate-8): 14, 41, 26, 12, 19, 14, 41, 28, 1.7, 26, 3 per minute per 10-minute window; no monotone decay, no quadratic shape. New source observations stop at minute 7 (219,231 to 219,285), so after that the checks are probes and memo/checkpoint answers.
- Host: swap 26.8 of 27.6 GB, 992 free pages, compressor 16 GB, load 35-45; the worker's CPU fell to 10-16 percent (page-fault stalls). Competing: `show projects` all-owner read (pid 67632, background, holds graph-custody exclusive 6,800 s), admission grant of graph-plan observed 3,973 MB against 2,048 MB reserved.
- Per-owner fixed cost: `discoverBuildOwnerAdmissionFactsV1` makes about 8 forced `stage()` calls, each an `advanceScriptInvocation` publish plus `yieldContinuation` (`yieldInferenceV1(...,force=true)`), even for a manifest-only probe. Under load each yield waits; candidate lever (not changed, unmeasured): do not force the stage yields when `targets.length===0 && inputNames.length===0`.
- The checkpoint got no new pack between 07:12 and 08:13 though owner checks rose by about 1,100: those checks were memo answers (row verification), consistent with "(34816 verified memo rows)".

## 3. boot-recovery label
`healNativeProcessBootV1` runs once per process; `recoverNativeBootV1` publishes one `observe` event with total 0 and returns "clean" (`!total` and ledger committed) without another event. `publisher` writes that as `activity.resource` of the worker's lease control; nothing later overwrites it. "held 96 min" is simply the age of the worker's shared image lease and the `build-graph-state` exclusive lease it legitimately holds for the selected read; the entry shows state progressing, silent 8 s. No recovery work runs. Cosmetic fix for the process lane (not mine, not done): publish a terminal label after the heal completes.

## Landed
Area lock graph, released. `🚪️admission/🟦️.ts`: `closeBuildOwnerAdmissionV1` takes a 6th optional parameter `trace`, records for every admitted owner the edge kind (`selection`, `task`, `caret`, `dependency`, `file`, `input`) and the admitting owner, and reports `{owners,pass,kinds,top}` every 250 owners and at the end (`OwnerClosureTraceV1`). `📥️load/🟦️.ts` prints it as `[graph-closure] {...}` on stderr. Test: closure-first (trace counts equal the oracle). Dependants run: admission 9/0, frontier 2/0, task-level-failure 6/0, owner-isolation 11/0, inventory 10/0. Record in `T/landings.md`.

## Not verified
- The bridge edge itself. Two attempts to run the real discovery for `s-dev-composition:generate` in a scratch process failed (first ended silently after about 6 minutes, second hit my 100 s alarm; load 40, swap full). The next layout read prints `[graph-closure]`; its `top` and `kinds` will show the hub.
- No live read with the trace; the stage-yield and memory hypotheses are unmeasured.
- The scope 1485623892 is assumed to be the layout read (start 06:17, matches).

## Other lanes
- The layout session and `show projects` (pid 67632) compete for memory on the coordinator host; stop one before measuring.
- Scratch: scratchpad/E33 only; nothing under generated.

# Round 2: the `file` edges are the real Cargo closure

## Correction of Round 1
My Round 1 figure of about 74 owners was wrong. The static script resolved `workspace = true` dependencies against the repository root `Cargo.toml`; every crate resolves them against its own Cargo workspace (`package.workspace`, e.g. `🌎️hub/Cargo.toml` for the layout component). With the same resolution the code uses (`nativeDependencyRoots`: dependencies and build-dependencies, dev-dependencies only at the root, `workspace=true` paths relative to the crate's workspace), an independent Bun script (not the repo code) gives:
- `semio-hub-layout-component` native closure 861 crates (identical with and without dev-dependencies).
- `@semio-tech/framework-os-node-graph-rs` 450 crates; overlap of the two 443; union 868.
- trace (service log, pass 2): node-graph 439 admitted, hub-layout 417. Static: 450 minus 11 already admitted = 439; 861 minus 443 = 418 (trace 417). Two independent computations agree to within one crate.
- `@semio-tech/framework-renderer-wgpu` native closure 717; union of the three 995.

## Verdict
There is no over-admission in the file edge. The 943 `file` owners are the `Cargo.toml` of the real `[dependencies]` closure of the wasm and component builds (`observeNative`), each a crate the compiler reads. The edge semantics are right; the layout closure is legitimately about 2,300 owners (about 1,000 Rust crates of the wgpu renderer and the layout component, plus the TypeScript and tooling owners). I therefore changed no edge semantics. The existing `🛂️admission` native-closure test (real `nativeDependencyRoots` against an independent oracle, 212 in that file) already covers the path-dependency rule; I added no duplicate.
What stays expensive is the cost per admitted owner (probe, then inference of each admitted owner), not the edges; see Round 1 section 2.

## Round 2 landing: the run shows the closure size
- The trace still goes to the service log (`[graph-closure]`). The run's progress line now also carries `(N owners admitted)` / `(N zugelassene Projekte)` from the same trace (updated every 250 owners and at the end): `📥️load/🟦️.ts` calls `report.count({admittedOwners})`; `📋️slice-report` schema (`admittedOwners`, text `inventoryAdmittedOwners`) and `🟨️.mjs` render it and count it as progress. Test: `🧭️inventory-progress` (949 admitted appears in both languages). Dependants: slice-report 14/0, final-line 4/0, closure-first 4/0, inventory 10/0. `🪜️read-steps` still has the one failure E20 already recorded (fixture expects the old step line); not caused by this change.
- Not verified: the live line on a layout read, and the next-read check of the trace (needs the coordinator to arrange a read; I did not touch that session).
- Other lanes: the closure of a wgpu or layout activation is about 1,000 Rust crates by design; to make such reads cheaper, work on the per-owner cost (probe and inference of crate-only owners, forced stage yields), not on edges.
