# F77: the 22 type errors found by F76's typecheck, fixed at the root

## Before
`repo typecheck` on F76's 4 files (closures/🟦️.ts, window test, resolve test, build/📥️inputs/💾️storage/🟨️.mjs): exit 1, 22 errors in 8 files, 0 declaration drifts.

## Causes and fixes (no casts added)
- 5x TS2339 `control` (closures/🧪️tests/♻️groups): the fixture's `withMaintenance` declared custody as the union `NativeImageMaintenanceCustodyV1`, but its `.mjs` throws unless `kind==="kernel-ex"`. The `.d.mts` now declares `Extract<NativeImageMaintenanceCustodyV1,{kind:"kernel-ex"}>` (build/📥️inputs/💾️storage/🟨️.d.mts). Test unchanged.
- 2x `string | number` (storage/🧪️tests/🪣️refusal): `bytes=typeof row.bytes==="number"?row.bytes:free+1` replaces the `=== "physical"` test, which did not narrow a JSON-typed value.
- 5x TS2322 `projectInputs:0` (seal/🎯️selection/📥️scope/🧪️tests): the planner contract is an array that production iterates with `for...of`; all 6 stubs (5 typed, 1 in the child-process code string) now return `[]`.
- 2x (observation/🧪️tests/📥️reuse): `holds(row.initializer.body)` got a Block; now guarded with `!ts.isBlock(...)`. `data.message` (unknown) is checked with `typeof ... === "string"` before `.startsWith`.
- 2x (observation/🧪️tests/🪪️presence): the sqlite oracle queries are now typed with `db.query<Row,[number]>` and `.get(...)!`.
- 1x (inputs/🧪️tests/📤️publication/📏️boundary): `withBuildGraphStagingV1` was declared `Promise<T>|T`, but the result type equals the body's result type. `.d.mts` is now `withBuildGraphStagingV1<R>(staging, body:()=>R):R`.
- 5x (graph/📥️load/💾️owner-checkpoint/🧪️tests): stubs were untyped (`observe` implicit any, `as never`). A typed helper `observed(rows,retained)` returns `Pick<typeof observationV1,"captureObservedSourceInputsV1"|"retainObservedRowsV1">`; row `kind` is `"exists" as const` so rows are real `SourceObservationV1`. No production type change (a narrowing of `observeOwnerFactsMemoizedV1` was tried and reverted: it broke importers).
- 1x (build/🧪️tests/🕸️graph/🧊️optional-cache): `walk` annotated `:void` with `child:import("typescript").Node`.

## After
- `repo typecheck` on the 12 touched files: exit 0, 54 files, 0 drifts.
- The original 4-file command with `--all-errors`: 1 error left, not in my 22: `🌐️remote-execution/🛰️agent/🟦️.ts:74` passes `reservationOwner` to `buildRemoteExecutionLoadReportV1` (a peer's half landing, also named in result-F70). Not touched.

## Tests (one run each, `repo test-file`)
- presence 41 pass / 0 fail; reuse 12/0; refusal 1/0; publication boundary 2/0; owner-checkpoint 7/0.
- scope (seal/🎯️selection/📥️scope): 6 pass / 12 fail, all "Required original cache consumer unavailable" thrown in the invocation admission before any planner code runs (suite graph-processes; harness/routing issue, not from `projectInputs`). Not fixed.
- optional-cache: 11 pass / 3 fail: `ReferenceError: sameClosureReadDescriptorV1 is not defined` and `Invalid build graph ... /graph is not allowed` in production paths (peer in-flight changes); my edit there is a type annotation only. Not fixed.
- groups test: not re-run (only a `.d.mts` type changed; F76 ran it 13/0).
- dangling-imports on the 5 touched folders: exit 0.

## Process note
All edits were atomic (temp file + rename) and compile-atomic, but I took no area lock: no `landing.lock.*` existed and the changes were type-only, and I realised after the fact. No sub-agents; claims line added to claims-tests.md.
