# F67 result: real types instead of the `any` stubs

## Count of `any`
- Before: about 268 `any` in the stub blocks F65 appended to 35 `.d.mts` files (151 exports, plus the 4 arity fixes), plus about 60 hand-written `any` in project-assembly and native-manifests.
- After: 0 in the stub blocks; 17 `any` remain in those 36 files, all hand-written before F65 (`run?:(...args:any[])=>any` probe, `ScriptInputsValueV1 = any`, `closureGraphPassV1(cache:any)`, `workerData:any`, `KIND_RULES_V1 (ctx:any)`, take CPU runners, command `runtime:any`, planner port graph). Listed by `python3 T/🧰️scripts/F67/anycount.py v`-style scan; no `unknown` was used as an escape (only at validation boundaries: errors, `canonicalKeyV1` value, graph payloads).
- Types come from the `.mjs` (parameters, returns), the callers, and the JSON schemas: `INFERENCE_LIMITS_V1`, `GraphMemoryPolicyV1`, `GraphFarmPolicyV1` are now `typeof import("…/🔣️.json")[…]["const"]`.

## `repo typecheck` heap
`compiler.heap` in `B/🔬️typecheck/🔣️.json` + schema: `maximumMegabytes` 12000, `minimumMegabytes` 4096, `hostShare` 0.6. `typecheckHeapMegabytesV1` = clamp(host memory * hostShare) and is passed as `NODE_OPTIONS=--max-old-space-size=N` to the compiler unless the caller already set one. Test added; typecheck tests 19/0.

## Library error count (whole-library `tsc -p`, same command as F63/F66)
817 -> 721 errors. Product (non-test) errors: 0 caused by declarations; 2 remain in `🏃️process/🎛️owned-execution` (`ChildProcess` union, not mine). `repo typecheck --declarations` 0 drifts. Per-area landings in `T/landings.md`.

## For F66 (test-file errors)
- Declarations tightened, so test expectations may now show real mismatches: `GraphMemoryPolicyV1` and `GraphFarmPolicyV1` follow the schema (tests using `peakSamples`, `workerMegabytes` now compile; tests using non-schema fields will not), `HostStatusV1` (`plans`, `memory` fields), `access`/`launchd` options, `SliceLoadRequestV1` (extra worker fields allowed), `GraphFarmV1.snapshot`.
- Still test-side (declaration is correct, the fixture is looser): `Iterable<SourceObservationV1>` observations cast to `{kind;path}[]`, `GenerationMembershipV1` partials, `IndexHandleV1` without key/release.
- Remaining ~720 errors are in test files; the largest groups: graph-hooks tests (25), cutoff (18), resident host (13), invocation plan (13), closure-graph layered-witness (12).
