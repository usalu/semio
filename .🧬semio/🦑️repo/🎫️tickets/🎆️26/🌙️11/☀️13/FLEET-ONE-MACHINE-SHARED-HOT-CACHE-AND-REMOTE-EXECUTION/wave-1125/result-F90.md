# F90: type errors in the caching library and the commands library

Lane F90, 2026-11-26. Method as F88: `repo typecheck --all-errors --no-importers <folder>` before and after, fix at the root, rerun each changed test file with `repo test-file`, one file at a time.

## Counts (compiler errors)

| Scope | Before | After |
|---|---|---|
| `library/⚡️caching` | 84 | 0 |
| `library/🎛️commands` | 21 (22 lines incl. 1 caching error shared with the caching run) | 0 |

`repo typecheck --declarations`: 0 drifts. `repo typecheck <🏗️build/📥️source/🔏️closure/🟢️node/🟨️.d.mts>` (7 importers): passed. `dangling-imports.py` on caching, commands, closure/node: exit 0.

## Root fixes (no behaviour change, no cast or any to silence)

Declaration outside scope (the only one): `🏗️build/📥️source/🔏️closure/🟢️node/🟨️.d.mts`: `buildSourceClosureNodeArgumentsV1(snapshot: unknown, ...)`; the runtime only `JSON.stringify`s the snapshot, so `BuildSourceClosureV1` over-constrained it (the capacity test passes a `{payload}` padding object). Unused type import removed. Lock `source` held briefly.

Tests that were out of date with the real API (the compiler found stale call shapes):
- `📦️budget/🧪️tests`: `cleanRemovePath` has 6 parameters (the 7th was dead; the incremental loop variable now reaches `allowCargoIncremental`); `requireImageFitsHostV1` helper called with a stray 4th argument.
- `🎛️commands/🛠️dev/🌱️birth`: `transpileStageRequestV1` is `(component, materialization, base, native)`, the test passed an extra `"jco"` and the hash landed in `maps`; `captureOwnedCommandV1` options are now built from `env/signal/output/control` instead of spreading the full `OwnedCommandOptions`; descriptor mode narrowed with a real guard.
- `⚡️caching/🧪️tests/🔬️script-commands`: dynamic import of `readNativeCallerAuthorityV1` pointed at `📦️artifacts/🟦️.ts`, which does not export it (fixed to the `📋️native-orchestration/📥️invocation` module). Same options fix in `📦️native-dependencies/🧪️tests`.
- `🌍️machine-law`: `testPythonV1()` returns `string|null`; the guard tested `=== undefined`, now `=== null`.
- `🧰️setup`: fleet remote requires `authenticated`; tests now state it (`false` for the mirror case, which is what the runtime treated a missing flag as). `decision` row type carries `version`.
- Duplicate object key `observeSelectedToolingPlanServiceV1` in the reading test (the first, dead, one removed; the runtime already took the last).
- Typed fixtures: `Uint8Array<ArrayBuffer>` for `Response`/`digest` bodies, `expect<unknown>` where a JSON fixture string is compared to a literal-union result, optional fixture fields (`extents`, `records`, `directories`, `raw`, `document`, `routes`, `available`) narrowed with `??`/`in`, readonly `length`/`$defs`/`configFile` mutated through `Reflect.set`/`Object.assign`/a copied object.

## Test counts (`repo test-file`, one file at a time, after the edits)

Green: vite admission 3/0, forwarding 2/0, caller 1/0, wait 2/0, busy-funding 23/0, foreign-names 5/0, opening admission 6/0, authority-seal 4/0, shards 4/0, nested-custody 14/0, sqlite 10/0, ignored 2/0, nx-cache 2/0, read-context 2/0, recheck 1/0, work 3/0, fence 8/0, javascript-storage 47/0, retirement 14/0, progress 7/0, continuous-exit 2/0, engine-digest 12/0, stamps 9/0, authorizers loading 2/0, capacity 10/0, data 5/0, sites 7/0, activation 4/0, rustup 20/0, wiring 5/0, install 27/0, fleet-fetch 8/0, mirror 2/0, components 4/0, machine-law 4/0, identity 8/0, cutoff 4/0, restart-hold 5/0, temporaries 14/0, native-dependencies module 0/0, script-commands module 0/0, registry 70/0, split-leaves 9/0.

Red, all from causes outside the edited lines (peer product state), not caused by this lane:
- profile-recheck 7/2 (lines 54, 90: drift verdict "accepted"; only line 35 was edited, types only).
- budget `🧪️tests` 23/2 ("Actual Native platform image required", test line 70).
- artifacts physical 1/15 ("Actual artifact repository authority required").
- dependencies reading 49/36, all `ReferenceError: descriptorProtocolV1 is not defined` in `readRepositoryGraphMaintenanceFileV1` (nx graph journal, product).
- incremental 18/1 ("hardlink control").
- derivative 17/7 (`openNxSdkStampMemoV1 is not defined`, private-repository reach-through; `records`/`directories` are always defined in the fixture so the `?? 0` does not change the rows).
- fleet-fetch setup 10/1 (generate task outputs in the framework-schema project file differ: product).
- native-dependencies tests 2/6 (admission closed after a first failure in `🔍️discovery/📂️source`).
- birth 14/2 (60 s playground timeout; Vite builder `🌐️vite/🟦️.ts:46`).
- closure/node tests 2/1 (peer-edited file; not touched here).

## Not verified
Whether the red files were red before this lane: no baseline run was possible without writing copies into the shared tree; each failure site was compared with the edited lines instead.
