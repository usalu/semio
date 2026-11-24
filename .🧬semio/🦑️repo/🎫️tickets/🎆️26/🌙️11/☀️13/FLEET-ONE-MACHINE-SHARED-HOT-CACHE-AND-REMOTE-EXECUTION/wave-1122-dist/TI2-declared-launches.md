# TI2: declared nested Repo launches for suites and `repo test-file`

## Result
The "Undeclared original Repo build selection" class is gone. A suite row now declares the nested `repo` launches its tests may spawn; the suite runner and `repo test-file` issue exactly those, and `readOwnedBuildInvocationV1` (unchanged) admits only an exact match.

## Design (schema-first)
- Suite schema `P/🧪️testing/🗂️suites/🧬️schema/🔣️.json`: `Launch` = `{command: "cargo" (closed enum), arguments: fixed tokens, paths: table-root-relative}`; suite property `launches` (max 8). Bun-files `Selection` gets `launches`.
- `processTestSuiteLaunchesV1(suite, root)` yields `["repo", command, ...arguments, ...absolute paths]`; `runProcessTestSuiteV1` passes them to `runOwnedBunFilesV1`, which appends them to the child's inherited launches.
- test-file runs the same `runProcessTestSuiteV1` with a row narrowed by the new `testFileNarrowedSuiteV1` (keeps `launches`, capabilities, scratch), so there is one source of truth.
- Declared in rows `native-artifacts`, `component-reclaim`, `native-queued-cancellation`: `repo cargo --store=shared --configuration=<prototype|release> -- build --bins --examples --message-format=json --manifest-path <admission crate Cargo.toml>`.
- `schedulerBinaries()` builds in the Original's build mode: a release build under a prototype Original is refused ("Conflicting build modes"), hence two declared shapes.
- Not done: suite-level prebuilt scheduler. The shared store makes repeat builds warm (jobserver file 26 s total); each file builds once per process.
- Not declared: `component-feature-unification` runs `repo cargo` with a per-run manifest and unit-graph arguments (no fixed shape); it needs its own row design.

## Law test (test-file's own file, 16 pass / 0 fail)
Schema mutation refusals, Ajv acceptance of the real table, declared paths exist, launches identical via the narrowed row; fixture `🧫️fixtures/🚀️launches` run under the suite runner and under test-file: declared launch admitted when the row declares it, refused when not; other manifest, other cargo verb, extra argument, bare `cargo -- check` and another `run` all refused.

## Runs
| file | before | after |
|---|---|---|
| jobserver | 7/3 | 9/0 |
| component-pass, native-artifacts | 41/29 (O10: 46/24) | 46/24, zero "Undeclared"; failures are a different set |
| component-reclaim / native-queued-cancellation | 0/2, 2/5 | 0/2, 2/5 (see below) |
| O10 "component jobs" (3 wasm workspaces overlap with jobs 3, sequential at jobs 1/none) | unverified | PASS alone (1/0, 34 s); no product bug found |

Caveat: in a full file run one failed callback closes the shared Original of the process, so later tests fail with "Finite job-only prerequisite graph required" as a cascade. Per-test isolation (`test.only`, restored byte-identical) gives:
- passes alone: "lane job runs inside its profile's Cargo lane" and "shared restart status" (their full-run failures were cascade), "component jobs".
- real failures alone, none from this change:
  - "jobs start as soon as...": `after:["job:tool","components:check"]` violates `📋️prerequisites` schema `^job:` (stale test vs schema from 10-21; pre-existing).
  - "real build over wasm32 components" and "queued exclusive compiler cancellation": "Live Original resource control issuance required" in `withCreatedNativeImageV1` (test uses a control not issued from its Original; pre-existing test-side).
  - "reserved nested compiler processes": `SEMIO_COMPILER_CAPACITY_HELD` is undefined in the job env (`capacityHeld` expectation); not attributed (candidate: capacity-held env of reserved jobs, possibly this wave's driver changes; owners O10/N1 to check).
  - "unit graph compilation cancellation": `metadata cancellation`; "pending stages and finalizers", "original finite caller budget": `toThrow` expectation mismatch; "unlaned same-profile Cargo jobs": fails without message. Not attributed to this wave's lanes by evidence.
  - Reclaim cluster (`comp-b: cannot find crate`, isolated-test primary rejection): fixture issue already in TI1.
- Isolated registered tests (reclaim, missing source witness, failed cargo exit, closure-scoped provenance) cannot be run with `test.only`; not isolated.

## Files
See `T/landings.md` "TI2". Probe/scratch removed; lock released.
