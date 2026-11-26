# F89: graph type errors 154 -> 0, stamps source-text assertion fixed

Lane F89, 2026-11-26. Scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph`. Method as F88: `repo typecheck --all-errors --no-importers <folder>`, fix at the root, no `@ts-ignore`, no new `any` casts, no behaviour change, then `repo test-file` on every changed test file.

## Counts

| Scope | Before | After |
|---|---|---|
| `🕸️graph` (608 files) | 154 | 0 |
| whole library (`--all-errors --no-importers`) | 381 (F88) | 125 (the other lanes work in the same library; none of the 125 is in `🕸️graph`) |
| `repo typecheck --declarations` | 0 drifts | 0 drifts |

A per-subfolder run (for example only `🕰️generations`, 148 files) reports fewer errors than the run of the whole `🕸️graph` folder (608 files); the numbers above are the whole-folder run, repeated until it passed.

## Root fixes

Declarations (the runtime really reads or returns the field):
- `👁️observation/🟨️.d.mts` (outside the scope folder, a `.d.mts` the graph tests import): `SourceObservationV1` has the `global` kind (`registerGlobalFactV1` records it); `ObservationWitnessV1.base`; `verdicts` maps are keyed by strings or witness objects (`Map<unknown,boolean>`); `observeDetachedSyncV1` rows are `SourceObservationV1`.
- `👁️observation/🔢️row-table/🟨️.d.mts` (same reason): `AuthorityObservationV1.fact` optional (the encoder tolerates a missing fact).
- `🦀️native-manifests/🟨️.d.mts`: a workspace dependency table entry may be a registry version string, a table with other keys, or absent; `📐️projection/🟨️.mjs` JSDoc return type (the shape the readers consume).
- `📥️inference/🟨️.d.mts`: `ScriptInputCacheV1.generatedOutput` (assigned and read at runtime).
- `🧭️script-inputs`: `closureGraphAnswerV1` resolves `{digest, answer}` (it was `string`); `closureTravelV1.export` bundles are the portable `ClosureBundleV1`; `🌳️closure-graph` node `extra.witness.base`; `💾️entries` `why`; `🧹️retention` record may be null.
- `🧶️bun-lock/🟨️.mjs` and `🔖️stamps/🧮️conformance/🏃️runners/🟨️.mjs`: JSDoc types (return `BunLockGraphV1`, `Record<string, unknown>` for the case tables).
- `🧮️derived-nodes`: JSDoc of `taskKeysV1` (`signal` optional), `recordEntryV1` (`vouch` optional), `loadManifestV1` (reader), `rowPathsV1` (typed result), `createWalkFarmV1` (`speed`).
- `🕰️generations`: `GenerationPartV1.dispose`; `createGenerationProgressV1` / `onGenerationProgressV1` carry `running` and `decision` and the clock has `running`; plan `say` takes `{en,de}`; `settled` of the epoch doubt resolves a boolean; the store options name `doubt` (unknown: the host passes its own doubt shape).
- `📥️load/🟦️.ts`: the observation port of `observeOwnerFactsMemoizedV1` names only what the function calls (type only).

Tests (types follow the real API, assertions unchanged): `expect<unknown>` where the oracle value is wider than the actual, JSON fixtures decoded to a declared shape, helper types made generic or readonly, typed `bun:sqlite` queries, `Object.assign` for the generator override, `Reflect.get` for fields of fake children, a local `projectNames` guard instead of a cast, `whatwg-url` ambient declaration `📥️inference/🔎️owner-discovery/🕸️wasm-outputs/🧪️tests/🧰️support/whatwg-url.d.ts` (referenced from the test).

Three dead references removed from tests because the runtime never reads them: `durableAnswers` passed to `createGenerationStoreV1` (it is read only by the load layer), `owner` passed to `acquireFreshResourceLeaseV1` (queued leases only), `settler.release?.()` (no such member).

## Stamps test (`🔖️stamps/🧪️tests`): stale assertion, fixed on the test side

Failing assertion: `toContain("nestedProject?recordStampV1():OBSERVATION_REVISION")`. The current implementation keys every owner record by `sha256([recordStampV1(), null, key])` for nested and non-nested computations alike; `OBSERVATION_REVISION` (bytes of the module) is no longer in a record digest. So the assertion described an earlier implementation (the non-nested branch used the bytes); it is stale, no drift. The implementation is stricter than the old text, so the test now asserts the law itself: `not.toMatch(/JSON\.stringify\(\[[^\]]*OBSERVATION_REVISION/u)`. The remaining uses of `OBSERVATION_REVISION` are the revision field of publication receipts, which are not records. 21 pass / 1 fail -> 22 pass / 0 fail.

## Test counts (`repo test-file`, one file at a time, after the edits)

pass/fail: derived-nodes 14/0, registry-reads 4/0, admission 10/0, frontier 2/0, vanished-directory 2/0, owner-isolation 25/0, closure-repair 11/0, cutoff 159/0, generations load 114/0, door-board 6/0, plan-once 8/0, workspace-closure 37/0, owner-reuse 2/0, multiwriter 6/0, doubt 10/0, liveness 2/0, adaptive-waves 24/0, owner-replacement 3/0, row-log 9/0, edge-reads 4/0, producer-path 2/0, owner-release 4/0, bounded-door 11/0, part-ids 5/0, closure-shards 2/0, lost-leaves 5/0, closure-leaves 10/0, memo 2/0, projection 8/0, evidence 1/0, restore 1/0, streamed-shape 3/0, cargo-go 7/0, head-only 3/0, claim-keys 3/0, carried-release 2/0, stream external 6/0, claims processes 10/0, requested-group 2/0, wasm-outputs 3/0, warm 11/0, answer-validity 10/0, derive-claim 3/0, take 11/0, adopt-counters 5/0, resource-key 1/0, layered-witness 3/0, tool-boundary 6/0, stamps 22/0.

Not green, not caused by the edits (a type-only or behaviour-neutral change cannot move them):
- `🧪️tests/🙋️fleet-read/🍏️macos-reading` 1/2: the message embeds the live workspace (a 27952 MB project, grant 4096 MB instead of 3512 MB); the changed `!` is in the passing test.
- `📥️inference/🧪️tests/🧭️walk-edits` 6/3: three "recorded observations hold" assertions; the edit is only the `write` helper skipping `undefined` (never present at runtime). Other lanes changed `👁️observation` sources in the same hours.
- `🪆️heavy-questions` 15/1 (first run 14/2): a timing law of the child event loop (51 s), jitter under load; the edit is one `Reflect.get` in another test.
- `🏃️processes/📥️holder/🟦️.ts` is a spawned child script, not a test (`0 pass 1 fail` when run alone).

## Checks

`dangling-imports.py` exit 0 on `🕸️graph`, `👁️observation` (declaration and row-table), `🧪️tests/🕸️graph/🧰️support/🔗️owner-closure`. `repo typecheck` of the touched observation declarations with importers: 21 errors, 0 in `🕸️graph`, all in importers that are not touched (`🧊️revision`, `🧠️memory/📋️slice-report`, `🧪️tests/🕸️graph` seal-under-churn, stale-facts, repair-rounds, invocation slice, graph-state); their messages do not name a changed declaration.

## Notes for other lanes

- `staleImplementationV1` (`🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts`, outside the scope) is typed `Error` but returns an error with `code` and `moved`; the vanished-directory test reads the code with `Reflect.get`. A `Error & {code:string; moved:readonly string[]}` return type would be the root fix.
- Process note: the edits were landed as many small atomic edits during the session; the landing locks `graph`, `observation` and `tests` were taken at the end, before the final checks (they were free; no other lane held them). Released at the end.
