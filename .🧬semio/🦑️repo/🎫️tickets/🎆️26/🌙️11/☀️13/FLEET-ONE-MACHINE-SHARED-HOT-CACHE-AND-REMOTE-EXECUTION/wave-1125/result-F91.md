# F91: type errors in observation, source closure, sidecar, remote-cache retention, host-capacity, footprint, invocation plan

Lane F91, 2026-11-26 12:2x-12:50. Method as F88: `repo typecheck --all-errors --no-importers <folder>` per folder before and after, fix at the root, no behaviour change, no `@ts-ignore`, no `any`.

## Counts (compiler errors)

| Folder (under library/🏗️build) | Before | After |
|---|---|---|
| 👁️observation | 36 | 0 |
| 📥️source/🔏️closure | 27 | 0 |
| 🌐️remote-execution/🛰️sidecar | 18 | 0 |
| 🌐️remote-cache/🏷️retention | 11 | 0 |
| 🌡️host-capacity | 11 | 0 |
| 🧠️memory/🦶️footprint | 4 | 0 |
| 🧭️invocation/🧪️tests/🛰️plan | 9 | 0 |
| Seven folders together | 116 | 0 |
| Whole library (`--all-errors --no-importers`) | 381 (F88 end) | 124 (includes F89 and F90 work in parallel) |

The brief said about 290; the per-folder runs find 116 in the named folders (the whole-library program reads the same files differently). `repo typecheck --declarations`: 0 drifts. `dangling-imports.py`: exit 0 on every touched folder. Importers check of the touched declarations: 18 errors, all in untouched files (🧊️revision tests, 🧠️memory/📋️slice-report tests, graph repair-rounds, graph-state, slice-lab, invocation/memo slice), none involving a declaration of this lane.

## Root fixes (types follow what the runtime reads)

Declarations and JSDoc (no behaviour change):
- `🏡️resident/🚪️client/🛰️plan/🟨️.d.mts`: `PlanRefusedErrorV1` constructor takes the frame it really reads (`code`, `retry`, `message` per language, optional `t`).
- `🏡️resident/🏊️delegate/🟨️.d.mts`: `createWarmPoolV1` option `snapshot` (the runtime writes it to the fifth pipe of the child).
- `👁️observation/🟨️.d.mts`: `OBSERVATION_LIMITS_V1` declares all 14 limits the schema carries and the declaration lacked (e.g. `consistencyMemoRows`, `rowStoreRows`, `yieldMilliseconds`); `observeDetachedV1` accepts a synchronous or asynchronous computation (the runtime awaits it) and returns `SourceObservationV1[]` rows.
- JSDoc types for JS without a declaration: `📂️source-names/🟨️.mjs` `sourceTreeFilterV1` (generic entries, `isHollow` optional), `📥️reuse/🎫️verified/🟨️.mjs` `holds` (`recheck` receives the ancestry), `🧩️field/🟨️.mjs` `fieldFactV1` (`readOther(path)`).
- `📥️source/🔏️closure/🟢️node/🟨️.d.mts`: `buildSourceClosureNodeArgumentsV1` takes `policy: unknown` (the runtime validates it and refuses forged ones; the test feeds forged ones on purpose).
- `📥️source/🔏️closure/🚪️command/🟦️.ts`: global `ImportMeta` fields `semioCapturedImplementationIdentity` and `semioCapturedLoaderIdentity` next to `requireCapturedBuildCommandV1`, the function every captured script passes them to (same two-line declaration the bazel executor and the memo command already carry).
- `🌐️remote-execution/🛰️sidecar/🟦️.ts`: `sidecarRestartPolicyForV1` return type includes the agent's optional `drainMaximumMilliseconds` and `drainPollMilliseconds`.
- `🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts`: `inflight?()` may return `null` (the runtime reads `inflight?.()??null`).

Tests (type-level; one test input changed to the real API): fixtures typed like the real API (`as const` literal arrays, `SidecarWorkV1`, `ConsentStateV1`, `Record<string,string>` environments, `Uint8Array<ArrayBuffer>`, a `Duplex` guard for the extra child pipes, an explicit row type for the staleness JSON), `expect<unknown>` or `expect<string>` where an oracle JSON value is wider than the actual, optional chaining or `!` where the fixture guarantees a field. Typed fakes of the overloaded `statSync`/`lstatSync` go through two small helpers (`statFake`, `lstatFake`, one assertion each, the same idiom the file already used for its spies). One real input fix: `🚦️ready/🧪️tests` passed `free:` where the runtime reads `available:` (disk-short row), now `available:`.

## Tests (`repo test-file`, one file at a time, after the edits)

Green: staleness 5/0, retention 18/0, process-tree 16/0, holder-custody 6/0, silence child 4/0, watchdog 10/0, ready 58/0, stage-observers 4/0, stage-completion 8/0, restart 23/0, fresh-policy 2/0, followup 19/0, cargo-scaling 16/0, graph-plan 13/0, reclaim 14/0, disk-recovery 23/0, row-table 12/0, hollow 4/0, read-set 2/0, listed-children 4/0, stale-row 3/0, discovery-reads 3/0, observation main 69/0, field 3/0, witness 5/0, conflict 2/0, verified-presence 4/0, dependencies admission 10/0, dependencies main 5/0, readmission 13/0, stream 20/0, warm 2/0, warm pool 1/0, mismatch 2/0, invocation plan 17/0, footprint 53/0.

Not green, runtime behaviour my type-only edits do not touch (the assertions differ on values, other lanes are editing those modules):
- 🌡️host-capacity `📉️shrink` 4/2 (oracle says "starved", the policy says "memory"; one test times out at 60 s) and `🚦️native-cargo-grant` 2/2 (expected 5 jobs, received 4): the host-capacity policy and code have 16 modified files in the tree.
- 👁️observation `📂️source-names/🧪️tests` 9/1 (law "every fact kind that reads a directory is declared": `readdir` declared, not found in the source block) and `👥️analysis-pair` 3/1 (the exact file observation is not recorded): runtime of `👁️observation/🟨️.mjs`, not the JSDoc added in this lane.
- 📥️source/🔏️closure `🔁️dependencies/🧪️tests/📥️process` 0/3 (issued-envelope text differs from the expected child wire), `🟢️node/🧪️tests` 2/1 ("Original process birth already received"), `🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan` stopped at the 3072 MB process ceiling: the process routing module (F85) changed under them.

## Notes

- `whatwg-url` in `🕸️wasm-outputs` (graph, lane F89): the test already carries a private ambient declaration `🧰️support/whatwg-url.d.ts` and the package is the independent third-party oracle for the WHATWG URL parser that CLAUDE.md requires, so replacing it by the built-in `URL` would remove the oracle; I did not touch it. `source-demand` loads it with `require`.
- Locks: `resident` taken and released for the two resident declarations. The `source` lock was held by F90 (12:20) during my edits under `📥️source/🔏️closure` (type-only, I did not take it); observation, sidecar, retention, host-capacity, footprint and invocation edits were type-only and made without an area lock, as in F88 for tests. No file of 🕸️graph or ⚡️caching was edited.
