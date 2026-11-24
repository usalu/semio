# TI1: does `repo test-file` give a file a different envelope than its suite?

## Verdict
No. For the "Undeclared original Repo build selection" class, `repo test-file` already builds the same declared envelope as the suite runner, so a file's nested-`repo` behaviour is identical in both. The 24 driver failures are not an envelope gap and are NOT removed by any test-file change. I landed the single-source function and a law that pins the equality; I did not and could not "fix" the failures through test-file.

## What the refusal is (file:line)
- Thrown in `readOwnedBuildInvocationV1`, `LIB/🏗️build/📥️caller/🟦️.ts:86` (LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`). It strips the host capacity grant (`--parallel`, `--parallelism`, `--workers`, `--graph-policy`) from a nested `repo <cmd>` request and demands the request equal one of `capabilities.launches` of the inherited Original. Called by `⌨️command/🦀️cargo`, `📦️dependencies`, `🩺️preflight`, `🏃️task`, `🧭️selection:136`.
- `launches` are fixed when the Original is born: `createNativeBuildCallerCapabilitiesV1` (`launches:[["repo",...args]]`), expanded in `📤️child/🟦️.ts:expandBuildBootstrapCapabilitiesV1` (Nx/storybook/dashboard descendants), or, for graph tasks, from the target's `delegation` (`📤️orchestration/🟨️.mjs:33`).
- The failing nested call is `schedulerBinaries()` (`LIB/…/native-build/🧪️tests/🦀️cargo/🟦️.ts:70`): `repo cargo --store=shared --configuration=release -- build --bins --examples --message-format=json --manifest-path <scheduler crate>`. Nothing declares that launch anywhere.

## Same envelope, measured
- Printed `createBuildBootstrapCapabilitiesV1` for `run @semio-tech/framework-process:test-native-artifacts` (what test-file builds): selection `["repo","run",…]`, launches `[["repo","run",…],["nx","run",…]]`. No cargo launch. The suite runner's target (`repo ./📜️script.ts test native-artifacts`, `🏃️process/📋️project.json:529`) is born under the same selection; `declareLibraryBuildTestEnvelopeV1` uses the same formula for library suites; the process-table suite row grants only `artifacts`/`command` (identical in both: test-file copies the row via `…rest`).
- What test-file already does like the runner: the suite row narrowed to the file, genuine-birth launchers (build/caching tables), managed Python, hermetic artifact sandbox, preparation provider, checkout cwd, budget stretch.
- Ground truth of the real runner was NOT obtainable: `repo run @semio-tech/framework-process:test-native-artifacts` sat 18 min in "Reading the project graph slice" (load 50) and was stopped; `repo ./📜️script.ts test native-artifacts` is refused ("Original parent Script process invocation required": scripts are only launched by the Nx executor). So "passes under suite" is argued from code, not observed.
- E48/E57/E69 reached the same conclusion for the "Original ... required" bucket: remaining blockers are test-side (private repository workspace, product scope wrappers, birth rows that cannot pass), not envelope.

## Landed (see `T/landings.md`, "TI1")
- `testFileEnvelopeArgumentsV1(project,suite)` in `LIB/🏗️build/🔬️test-file/🟦️.ts`; `🏃️run/🟦️.ts` uses it (was inline `["run",project+":test-"+id]`).
- Law test in `🔬️test-file/🧪️tests/🟦️.ts`: for a build, a caching and a process file, the arguments are `run <table project>:test-<suite>`, `createBuildBootstrapCapabilitiesV1` yields selection = arguments = `launches[0]` = `["repo",…args]`, Ajv (third party) admits the capabilities against `🧬️schema/📥️caller.json`, and the process `📋️project.json` declares `test-<suite>`. Result: test-file's own file 15 pass / 0 fail (18.9 s).
- Tried and reverted: removing `SEMIO_SCRIPT_*` from the env of the helper's `repo cargo` child. Still "Undeclared" (the child gets the Original through the control). Helper sha restored (`beb27108bae5`).

## Runs (one at a time, `repo test-file`, load 36-52)
| file | result | blocker |
|---|---|---|
| `native-build/🧩️component-pass/🧪️tests` (routes to 3 suites) | component-reclaim 0/2, native-queued-cancellation 2/5, native-artifacts 41 pass / 29 fail (O10 saw 46/24; a peer moved 5) | 24+ "Undeclared original Repo build selection" (nested `repo cargo` scheduler build), "Live Original resource control issuance required", "comp-b: cannot find crate" in reclaim (separate fixture issue) |
| `native-build/🎰️jobserver/🧪️tests` | 7/3 | Undeclared (scheduler build), 3 tests |
| `⚡️caching/…/📥️invocation/🧪️tests` | 2/2 | "Original Repo build caller required" (test authors its own caller; E47 drift) |
| `⚡️caching/🧪️tests/🗿️artifact-source` | 0/1 | "Exact Original workspace required" (test-side) |
| `⚡️caching/🚀️bootstrap/📥️admission/💾️storage` | 0/3 | live repository store: needs `privateRepositoryWorkspaceV1` (test-side) |
| `⚡️caching/🕸️graph/🏇️waiter` | 0/1 | "Original installed tooling identity …" (needs authorizers loaded, E48/E57) |
| `native-build/🔒️lease/🧪️tests/🏎️overlap` | 1/1 | live repository store (test-side) |
None changed against T2a, as expected.

## O10 stub driver test
Still UNVERIFIED through test-file: the "component jobs" stub test builds the scheduler via `schedulerBinaries()` and dies on the undeclared `repo cargo`. The job-slot runner itself stays covered by `job-slots/🧪️tests` 5/0.

## Open items / recommendation
1. Real fix for the 5 nested-cargo files (admission, jobserver, telemetry, frontend-threads, units/capture memo) and the 24 driver tests: the scheduler build must be a declared launch of the suite's Original. Clean option: a suite-row field (e.g. `capabilities.launches`, schema-first) read by BOTH the suite runner and test-file, and issued into the parent envelope; or have the helper take a prebuilt scheduler (`SEMIO_*` path from the registry-ready receipt) instead of a nested `repo cargo`. Not done: it changes what a suite may spawn (custody decision for the repo owners).
2. Confirm on a quiet host that `repo run @semio-tech/framework-process:test-native-artifacts` fails the same way (needs the graph read to finish).
3. Nx-namespace class (task-child, service, waiter, nx-plugin/arguments, import-edges/authority) needs `fileLaunchers` rows once E49's birth passes (E48).
