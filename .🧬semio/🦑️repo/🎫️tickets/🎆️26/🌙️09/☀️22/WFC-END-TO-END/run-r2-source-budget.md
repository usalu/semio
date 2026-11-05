# WFC Authoritative Rust Source Budget Entry

Read-only discovery2026-11-04. No compiler invocation, admission-policy change or replacement source counter.

## Public Analyzer

Public entry @semio-tech/process/artifacts/native-build/source is physically:
🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts.

It reexports readRustSourceClosureV1 from:
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts.

Exact signature:
readRustSourceClosureV1(entries:readonly string[],root:string,cfg:readonly string[],read:(path:string)=>Promise<Uint8Array|null>,owner?:{manifestDirectory:string;generated:(file:string,output:RustGeneratedTokenOutput)=>void}):Promise<readonly RustSourceInputV1[]>.

Returned rows contain path,digest,bytes,activeBytes,activeLines,kind(source|embedded). It resolves actual cfg-selected module mounts, path attributes and authored includes. Manifest-relative references require owner.manifestDirectory. Generated compiler-owned inputs cannot simply be treated as authored source.

activeLines is0 for embedded inputs or empty selected text; otherwise selected.split(newline).length. Blank lines retained by cfg selection count. A line-count script or comment-removing heuristic is not equivalent. The diagnostic is per selected crate closure, not per directory/file.

## Authoritative Gate

Public @semio-tech/process/artifacts/native-build/source/admission exports admitCargoSourceBoundariesV1. Implementation:
🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts.

Signature:
admitCargoSourceBoundariesV1(graph:CargoCompilationGraphV1,workspaceRoot:string,policy:CargoSourceBoundaryPolicyV1,control:ResourceLeaseControl,port:CargoSourceAdmissionPortV1,memo?:CargoSourceAdmissionMemoV1):Promise<void>.

Policy requires version1,maxSourceBytes,maxSourceFiles,maxActiveLines. Each workspace-provenance unit uses intersection with its optional unit.boundary. Per-unit source closure sums activeLines; refusal string is Rust crate Source boundary exceeded before compiler: PACKAGE (active lines ACTUAL/LIMIT). Registry sources are excluded from this authored-source gate.

Port owns cfg(unit,control),read(path,control,request),recheck(control,phase). An already captured actual graph+cfg can be supplied to the same analyzer with bounded read-only source callback, avoiding a build. Exact test target/platform/features/compiler flags must match failed unit; cfg=test changes closure and cannot be guessed. Newly split component must be evaluated as its own current declared crate entry and manifest owner, while parent closure must no longer mount moved implementation as an ordinary module.

Default actualCargoSourceAdmissionPortV1 probes rustc --print cfg (not compile) and requires NativeStorageReservation. Do not call that default as a lightweight workaround outside its storage owner. Reusing captured cfg is the low-cost path.

## Secondary Unit API

Repo cache units/📥️inputs/🟦️.ts exports readRustUnitSourceClosureV1(root,unit,boundary,control). It uses unit.cfgSnapshot,target.entry and unit.limits. This is appropriate only for a captured RustUnitV1, not a guessed substitute for the ordinary CargoCompilationGraph gate.

No graph/config-named captures were found under current ticket generated tree throughdepth3. Failed owner69406 may retain captures elsewhere under its owned compiler output; parent/native owner must identify exact receipt. This discovery does not assert all five editors pass the4000 limit.

## Fleet Diagnostic Cfg-Test Measurement

Retained input fleet-source-preflight.ts reuses bitmap-pin-source-preflight.ts's exact public analyzer/read callback and cfg=["test"]. Command repo tools exec --light bun -- "<ticket>/fleet-source-preflight.ts" completed exit0. Detailed output is generated/fleet-source-preflight.json under the actual 🗑️generated directory.

| Actual Owner Entry | Files | Bytes | Active Lines |
| --- | ---: | ---: | ---: |
| graph2d standard-v1 package |17|175059|3165|
| grid2d standard-v1 package |13|149368|2200|
| graph3d editor package |21|219415|3714|
| grid3d editor package |25|243326|3988|

These are exact analyzer results for this diagnostic cfg, NOT authoritative native admission and NOT an upper bound. The API has no undefined/conservative mode: cfg predicates are evaluated against the provided known Set; absent predicates are false. Actual platform/features/test/build-unit cfg can differ. Grid3d's12line sample headroom is a risk indicator only; no production refactor was made or justified solely by these counts. Root and both medium owners received the results.


## Engine Job Before Publication Owner Helper

Public readRustSourceClosureV1 diagnostic rerun through repo tools exec --light bun using retained engine-job-source-preflight.ts:3382 active lines,158848bytes,15files,cfg=[test]. Generated detail under 🗑️generated/engine-job-source-preflight.json. This is neither authoritative native admission nor an upper bound. Concurrent migration may change it.

Nominal150line helper/test addition would total3532 in this same diagnostic, leaving468 below4000; there is no measured reason for a component split now. Largest inputs: engine job unit tests465,restore state407,propagate316,initialize287,headless272,run271,persistent263,job struct257,restore run243,publication190+unit145. A file split under the same component does not lower closure cost. If later actual admission approaches4000, publication ownership is already a domain-specific frontier suitable for a genuine separate component with its own manifest and tests; do not perform arbitrary source-file shuffling or claim cfg=test admits real native cfg.

## Post-Migration Diagnostic Refresh

Engine job public cfg=[test] diagnostic now3620active lines,180114bytes,19files;380 nominal headroom. Same retained script/output refreshed. Non-authoritative and not upper bound. No source-budget split is justified by this measurement alone. A cohesive separate generic headless component remains a sound future boundary if actual native closure exceeds4000; it depends on framework job/worker/value/async, not WFC solving internals, so dependency direction need not cycle. No queues.

## Final R3 Wait Diagnostic

At 2026-11-05T02:07:23.846Z, reran the retained public-analyzer inputs with cfg=[test]. Neutral headless1283 lines/10files, engine-job3194/17, graph2d3187/18, grid2d2259/14, graph3d3791/22, grid3d3969/26, bitmap inferences2273/24, editor3814/26, public tests1328/9, pin-solution455/6. All invocations exited0. This is the same explicitly non-authoritative diagnostic, not native admission or an upper bound. Grid3d has only31 nominal lines of headroom; preserve domain ownership and respond to actual admission if its real cfg crosses policy. No source-policy change or source minification was performed.
