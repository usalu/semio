# F88: type errors in importers around build, graph, caching and process routing

Lane F88, 2026-11-26. Method: `repo typecheck --all-errors --no-importers` per folder (the JSON form lists at most 200 diagnostics, so each folder was re-run after every batch), fix at the root, rerun the changed test files with `repo test-file`, one file at a time.

## Counts (compiler errors)

| Scope | Before | After |
|---|---|---|
| Whole library (`repo typecheck --all-errors --no-importers <library>`) | 579 | 381 |
| `🏗️build` folder alone | 659 | 441 |
| `🏗️build/🏡️resident` | about 87 (79 after the first d.mts batch) | 0 |
| `🏗️build/🕸️graph` | 283 | 154 |
| `⚡️caching` | 84 | 84 (not touched) |
| `🎛️commands` | 21 | 21 (not touched) |

The library figure at the start (579) is lower than the `build` figure (659) because the whole-library run reads a different program than a per-folder run; use each scope against itself. `repo typecheck --declarations`: 0 drifts. `dangling-imports.py` exit 0 for every touched folder.

## Root fixes (types follow what the runtime reads)

Declarations (`.d.mts`, no behaviour change):
- `🔭️witness`: `verifyWorldWitnessV1` takes `Pick<WorldWitnessV1,"observations">` (the runtime reads only `observations`).
- resident `⌨️command/🌐️host`, `🚪️client/🛰️plan`: `policy` is `Partial<typeof HOST_POLICY_V1>` (the runtime merges it over the host policy), `probe` and `launchctl` ports declared; `PlanResultV1.welcome.service.instance`.
- resident `🏗️composition/🌐️host`: `launchd.run` is `LaunchdRunV1` (a number or `{status,stdout}`, as `answer()` accepts); `spawnHostServiceV1` request needs only `directory`/`log`.
- resident `🏗️composition/🟨️`: `executorSize` and `environment` dependencies; `👷️slice-pool`: `createWorker` data is the real request plus `heartbeatMilliseconds`, `reserve` always gets `onWait`, `shareKeyV1` accepts the extra request fields, `ProcessRowV1.service`, `errorOfMessageV1` message `type`.
- resident `🏛️service`: `ports.settle`, `ports.resident.rss`, `ports.workers.active`, `PlanTaskV1.key`, planner `plan` index needs `value` and optional `key`; `mirrorFailureV1` option `root`; `📨️protocol`: `HostFrameV1` for `onFrame`.
- graph: `📈️wave-row` (`phases`, `maximumPhases`, progress `phases`), `🧮️derived-nodes/🧩️kinds` (`verifyObservedV1` has the ref/options shape its consumer passes), `🕰️generations/🧭️plan` (`workspaceRoot: string|undefined`, the runtime handles undefined), `🔱️warm` (`index` needs only `has`), `🏊️project-farm` (`describe()`).
- JS files without a declaration: JSDoc `@param` of `📦️project-nodes/🟨️.mjs` (setAsideOverrunV1, askOwnerRefsV1, inferenceLanesV1, ownerJobV1, orderOwnerJobsV1, graphOwnerEnforcementV1 optional arguments) and `🔖️stamps/🧮️conformance/🟨️.mjs` (resolveStampV1, runConformanceAuditV1). The old tags described the whole destructured parameter as one function.

Tests (type-level only, assertions unchanged): fixtures typed like the real API (receipts, workers, `waves` of a pointer that may be `"fault"`, `as const` literals, optional chaining where the runtime may omit a field, `expect<unknown>` where an oracle value has a wider type than the actual). No `@ts-ignore`, no new `any` except the test helper `infer` of four provider-closure tests whose modules are already loaded `as any`.

## Test counts (`repo test-file`, one file at a time, all after the edit)

resident: queue-progress 5/0, slice-priority 1/0, restart 8/0, cancel 37/0, booking 6/0, restart-quiet 7/0, restart-cause 14/0, reload 12/0, consent 16/0, supersession-drain 5/0, reservation 8/0, boot-phase 2/0, host 39/0, failure 20/0, source 1/0, fixture-server environment 2/0, installed-watcher 3/0, state 15/0, start-identity 3/0, status-deadline 8/0, slot 2/0, provision 17/0, silence 1/0, consent-service 31/0.
graph: owner-isolation 25/0, membership-follow 6/0, vanished-directory 2/0, settle-churn 4/0, idle-builder 7/0, settle-skip 4/0, waves 8/0, renamed-owner 2/0, whole-gate 13/0, source-failure 3/0, early-wave 8/0, one-budget 10/0, single-writer 4/0, owner-pool 34/0, warm 11/0, fetch-lanes 3/0, look-bound 4/0, before-infer 5/0, adopt-counters 5/0, take 9/0, cargo-stand-in 1/0, warm-jobs 5/0, retention 5/0, tool-boundary 6/0, preflight 2/0, whole-walk-boundary 4/0, tool-cover 6/0, conformance 11/0, children 3/0, shared-catalog 1/0, contract-rows 3/0, source-matches 3/0, authority-round 1/0, wasm-outputs 3/0, projection 8/0, phases 19/0, wave-row phases 4/0.
Not green, not caused by this lane: `🔖️stamps/🧪️tests` 21 pass / 1 fail (source-text assertion "no record, parser identity or generation fact is keyed by the bytes of the implementation"; this lane only changed an `expect<unknown>` in that file).

## Still open (next lane)

- graph 154: `🕰️generations` (remote-exchange, load, custody, claims, stages: about 60), `🧭️script-inputs` (closure-graph layered-witness, entries `why`, closure-bundle, producer-path, answer-validity, derive-claim, heavy-questions), `🔌️providers`, `🔗️workspace-closure`, `🧭️walk-edits`, `🪫️owner-release`, `♻️owner-reuse`, `🦀️cargo-go`, `✂️cutoff`.
- build outside graph (about 290): `👁️observation/🧪️tests`, `📥️source/🔏️closure`, `🌐️remote-execution/🛰️sidecar`, `🌐️remote-cache/🏷️retention`, `🌡️host-capacity`, `🧠️memory/🦶️footprint/🧪️tests`, `🧭️invocation/🧪️tests/🛰️plan`.
- `⚡️caching` 84 and `🎛️commands` 21 untouched. `whatwg-url` has no declaration file (`🕸️wasm-outputs` test): needs an ambient declaration under the taxonomy, not added.
- The importers check of the touched declarations (`repo typecheck <touched>`) reports 21 errors, all in files that are not touched and whose messages do not involve a changed declaration (`🧭️invocation/🧪️tests/🛰️plan`, `🧠️memory/🦶️footprint/🧪️tests`).

No area lock was held at the end. Locks taken briefly (and released): witness, resident, graph.
