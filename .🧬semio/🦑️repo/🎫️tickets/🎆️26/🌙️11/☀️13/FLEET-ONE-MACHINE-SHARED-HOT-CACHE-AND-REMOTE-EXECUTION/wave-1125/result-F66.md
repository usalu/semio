# F66 result: test-file type errors by cause

Invocation (as F63): `NODE_OPTIONS=--max-old-space-size=12000 node_modules/.bin/tsc --noEmit -p 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json` (about 2-3 min; the default 4 GB heap now runs out, so the heap flag is needed; F65's `repo typecheck` should set it). Product code stays at 0.

Total: 1264 (start of F66, includes F63 leftovers) -> 817 (end). Of the 447 drop, about 150 are F65's `.d.mts` work running at the same time (TS2305 46 -> 0, several TS2339/TS2345), about 300 are F66 groups below. Remaining list: `T/🧰️scripts/F66/tsc-remaining-errors-after.txt`. Scripts: `T/🧰️scripts/F66/` (lib.py, drop_extra_arg.py, bodyinit.py, batch2-5.py, classify.py).

## Groups fixed (errors before -> after)
| # | Cause | Fix | Errors |
|---|-------|-----|--------|
| 1 | bun-types pins `toBe/toEqual/toStrictEqual(expected: T)` to the literal type of the actual value (literal vs widened `string`, plain object vs branded row) | one shared module augmentation `📦️packages/🟦️typescript/bun-test-matchers.d.ts` with the Jest contract `<X = unknown>(expected: X)` (keeps explicit `toEqual<typeof x>(x)` calls valid) | TS2769 226 -> 32 (-194) |
| 2 | child scripts with top-level `await` and no import/export are scripts, not modules (3 files) | `export{};` appended | TS1375 13 + TS2451 8 -> 0 (-21) |
| 3 | stale trailing arguments: `server.stop(true)` (Bun API of old; ours takes none) x56, `buildGraphScopeKeyV1(sel,registry)` x13 | arguments dropped (position-verified script) | TS2554 -69 |
| 4 | corrupted test text (real runtime defects, the tests could not run): `repository.rooty.rooty.rooty.rooty.rooty.root` (units test, 13), `constrouting=` (retention negative test, lost space, 6), missing `linuxHostLibcV1` import (2 SDK tests) | repaired | TS2551 13 + TS2304 8 + ... -> 0 (-21) |
| 5 | `import X from "m.ts" with {type:"text"}` is typed as the module, not its text (16 files) | `const X=await Bun.file(new URL(rel,import.meta.url)).text()` | TS1192 16 -> 0 |
| 6 | defaults the typed `...options` spread always overwrites (`signal`, `jobs`, `onLine`, `LANG`, `SEMIO_WORKSPACE_ROOT`) | dead defaults dropped | TS2783 14 -> 0 |
| 7 | `Uint8Array<ArrayBufferLike>` is not a fetch `BodyInit` (TS 5.9 + bun-types) | `encodeProtobufV1` and its `view()` return `Uint8Array<ArrayBuffer>` (product, type-only), test `bytes:Uint8Array` params narrowed | about -16 (TS2769/TS2345/TS2322) |
| 8 | typed harnesses: selected-exchange `world(count,names)` returned `M[]` and 19 call sites cast to hand-written tuples; host `checkPlan(tasks:any[])`; reservation SQLite oracles were `unknown` | generic tuple-returning `world<const Names>`, readonly plan-row type, `db.query<Row,Params>(...)!` | -15, -8, -13 |

## Remaining 817 by cause (all in test files)
- Declaration-backed 616: the test imports a `.mjs` whose hand-written `.d.mts` is incomplete or stale (e.g. `INFERENCE_LIMITS_V1.closureAnswerAttempts`, host `status.plans/memory`, plan request `graphText`, `GraphFarmV1.snapshot`, `settled.vouched`). The runtime is right: resident host test 39/39 passes with 48 type errors. Fix belongs to the declarations (F65's `repo typecheck` declaration generator and law), not to the tests; do not cast in tests.
- Test-local 201: fixtures from JSON imports normalised into unions with `x?: undefined` that a `Record<string,string>` parameter refuses (fleet-fetch setup, 10), fake ports stubbed narrower than the port (slice-lab 7, buildscript offload 6, graph-state conformance), `Readonly` rows mutated in place (`.length=0`, `$defs=`), `string|null` from `Bun.spawnSync(...).stdout` etc. A typed fixture loader (validating against the fixture's schema with `validateJsonSchemaSubset`) is the next shared helper; not started because a new `🧪️testing` member needs taxonomy registration.
- By code now: TS2339 195, TS2345 134, TS2353 102, TS2322 91, TS7006 76 (mostly children of excess-property errors: the callback has no contextual type because the options type lacks the property), TS18046 38, TS2352 37, TS18048 37.

## Tests run (repo test-file, one at a time)
passed: resident host 39/39, memory reservation 8/8, selected-exchange, remote-cache server access, ready failure-log, graph warm progress 2/2. Not caused here: remote-cache `🧪️tests` 24/27 (3 status/schema content failures, `stop(true)` is ignored at runtime); cargo units test now runs past the `rooty` TypeError and fails on a message catalogue mismatch (`Original complete Rust compiler generation required`); retention negative test fails on "Exact private retention workspace required" (environment).
dangling-imports exit 0 over graph, remote-cache, remote-execution, resident, memory, caching, typescript package.

## Notes for others
- Per-area locks: F65 held 14 areas for 15+ minutes, so test edits waited; two earlier edits in `caching-bootstrap` (SDK derivative tests, test-only) landed before F65's lock was visible to my script.
- Two groups wait for nothing but are not done: `registry` stale args in `graph/🚪️admission` landed after F64 released its lock.
