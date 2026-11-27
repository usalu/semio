# F103: product typecheck 93 -> 0

Lane F103, 2026-11-26. `repo typecheck --all-errors --no-importers 🧰️framework/🛍️products/🦑️repo` (5178 files): **93 errors before, 0 after, 0 declaration drifts** (status passed). Method as F88-F95: fix at the root, no `@ts-ignore`, no new casts or `any`.

## Fixes
- 4 tests outside the replica: `🔎️search/🧪️tests/🧵️shards` (`event:{seq?:number}`), `🔩️native/🥾️bootstrap/🧪️tests/🧰️toolchain` (2 `expect<unknown>`), `🚀️launcher/🧪️tests/🏷️managed-workspaces` (`const claimed:boolean=shape(claim)` so the guard does not narrow `any` to `unknown`), `🧪️test/🧪️tests/🧬️schema-invariants` (`expect<unknown>`).
- 34 `expect(...)` calls rewritten to `expect<unknown>(...)` where the actual value and the expectation differ in a literal union (script `scratchpad/F103/fixexpect.py`, same method as F95).
- `python!==undefined` / `=== undefined` (9 sites, `testPythonV1()` returns `null`, so the skip guards never held): now `null`; `🩹️blobs` guards the Python oracle.
- Real API: `HEALTHY` status report of topology now carries the required `retired`, `refusedPaths`, `git`, `latency`, `retired`, `health`, `store`; canary `publishRefusalV1` takes no `wallMs`; `canary` decisions go through `asserts value is CanaryInputV1` (validates the "Input" schema) instead of a cast; `judgeFleetV1` heads through a `headsOf` helper (JSON union artefact `d?: undefined`); attest `silent`/`idle` typed `LatencyReportV1`; trust steps typed `Step` with an `isPart` guard; `renameSync` import in `🗺️topology/🧪️tests/🔵️.ts`; resilience `fs.stat` mock with a real overload set; `Object.assign` for the readonly `say`; `packgate.prepareStagePackedObjectsV1` imported instead of `Reflect.get` plus a cast; git test `options?.index`; `after??null`; `event.message?.includes`; `fold` script reads `process.versions.unicode|icu` through a checked helper (Bun reports both).
- Product code (one atomic landing 15:47:08, lock `replica`): `🧷️sidecars/🟦️.ts` option `protocol` is `Partial<Record<keyof typeof sidecarProtocolV1,number>>` (the frozen literal types rejected `leaseMilliseconds:number`, the runtime spreads numbers), `🧲️adopt/🟦️.ts` `child.stdout!`, `🥾️bootstrap/🟦️.ts` `wanted` typed as manifest entries (the runtime never reads `mtimeMs`), new `🥾️entry/🟨️.d.mts` (8 exports, no drift; the installer only copies `.mjs/.sh/.ps1`). Bun's transpiler output of the three `.ts` files is identical before and after.
- Daemon after the landing: boot beat and `health.json` fresh, one process, restarted once ~35 s later (pid 51216, 13 min up when checked), no revert.

## Counts
| Scope | Before | After |
|---|---|---|
| product level | 93 | **0** |
| `🤝️shared-worktree` folder | 89 | 0 |
| launcher / native / search / test | 4 | 0 |
| declaration drifts | 0 | 0 |

## Tests (`repo test-file`, one at a time)
Green: search shards 10/0, native toolchain 35/0, launcher managed-workspaces 2/0, trust 7/0, canary 11/0, shared-worktree resilience 26/0, boundary 73/0, git 69/0, clash 24/0, state 8/0, entry 24/0, restart-in-place 3/0, sidecars 13/0, blobs 2/0, replica git 40/0, race 52/0, store 8/0, boot 24/0, replica resilience 25/0, stage-packed 2/0, adopt 25/0, provision 22/0, topology 74/0, attest 146/0, attest separation 3/0.
Not green: `🧪️test/🧪️tests/🧬️schema-invariants` 163/1: "the test platform's own subtree carries no schema-contract finding" (`schema-catalog-stale` for four `🧪️test/**/🧬️schema` folders); my edit is one `expect<unknown>`, the failing line is not edited, the cause is concurrent test-domain schema work.
`dangling-imports.py` exit 0 on shared-worktree, launcher, native bootstrap, search tests, schema-invariants.

## Notes
- The long entry test (140 s) was moved to the background by the shell harness; the second test batch was started in one detached shell loop, not by the lane.
- The first edits to test files were written before the `replica` lock (tests only); the lock was held only for the product batch.
