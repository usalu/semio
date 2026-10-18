# Result bzl: Bazel route for the products (executor `bzl`)

Goal: `bun repo --tool=bazel run <target>` for `@teaching/architecture-quiz:{test,publish,dev}`, `@teaching/proctor:build`, `@semio-tech/semio-tech-play:{activate-prototype,serve}`, `@semio-tech/mit-bestand-demonstrator:{activate-prototype,serve}`.
Exact names verified from the manifests: the demonstrator has no `activate`; the target is `activate-prototype`. All eight names exist (`dev`, `publish`, `test`, `serve`, `build`, `activate-prototype`).

Status: STATIC PHASE DONE. RUNTIME PHASE BLOCKED: `node_modules` is still a dangling symlink after ~2 h of polling (deps `sync --all` attempts 9-14 failed one after another: `Exact current Store Source fence changed`, `Closed declared resolution profile required`, `Compatible declared provider version required`; none relates to this work). `bun repo show projects` cannot load the graph without it (`Cannot find module '@semio-tech/process/wasm-build'`). Resume with SendMessage once `ls node_modules/nx` works.

Additional test edit: `🔧️tools/🏗️bazel/🧪️tests/🟦️.ts`, phase `services` (SEMIO_BAZEL_INTEGRATION=1) now also runs a `ready`-declared listener (starts listening after 400 ms and announces its address) with a dependent `waiter` that fails unless the listener already answers when it starts. Transpiles; not run.

Runtime phase checklist: (1) `bun test` of `🟢️readiness/🧪️tests` (2 oracle tests need strip-ansi/ajv); (2) re-measure the adapter closure with `🗑️generated/bzl-closure/measure.ts <entry> <root>` (static estimate 507/512); (3) `bun repo tools setup bazel`; (4) `--dry-run` for the 8 targets; (5) real `quiz:test`, `proctor:build` (only if no other `bun repo cargo` runs); (6) Bazel suite incl. SEMIO_BAZEL_INTEGRATION services phase; (7) Nx `readyWhen` check on a continuous root.

## Static findings (from the audits plus reading the adapter)

1. Continuous readiness was only the stdout marker `[bazel] task-started <id>` printed before the command started. `metadata.semio.dashboard.ready` (33 declarations, all schema-valid) was ignored.
2. `{options.env.SEMIO_BUILD_MODE}` in `outputs` is used by `quiz:publish`, `quiz:build` and by every inferred `framework-os-dev:prepare-*/activate-*` target. None of them carries `options.env`, so `captureBazelSnapshotV1` threw `Build task requires option env.SEMIO_BUILD_MODE` after the command had run (audit 5.3 was wider than the audit thought: `publish` is a requested goal target, and the prototype targets of play and demonstrator hit it too).
3. Adapter closure budget: the captured Bazel implementation closure may hold 512 files (`maximumFiles`). Measured with the closure walker (relative imports plus sibling schema directories, one bare `@semio-tech/process/routing` import unresolved until node_modules exists): 506 files before this work, 507 after. Margin is 5 files; re-measure after the dependency install.

## Changes

Neutral, schema-first readiness contract (shared by Nx and Bazel, same semantics as the dashboard daemon):
- `🏗️build/🧬️schema/🔣️.json` (graph schema): new `$defs` `ReadyDeclaration`, `ReadyPrinted`, `Ready`, `ReadinessPolicy`; target property `metadata.semio.dashboard.ready` now validated by `$ref`.
- `🏗️build/🧬️schema/📋️project/🔣️.json` (project schema): `ReadyDeclaration`, `ReadyPrinted`; `Task.metadata.semio.dashboard.ready` validated by `$ref`.
- `🏗️build/🟢️readiness/🟦️.ts` (new): `readReadinessDeclarationV1`, `admitReadinessDeclarationV1`, `resolveReadinessV1` (portEnv wins over port), `ReadinessMatcherV1` (streaming, ANSI-stripping, byte-for-byte same decisions as the Rust tracker), `probeReadinessV1` (bounded GET on 127.0.0.1 / ::1, 2xx-3xx, redirects not followed), `watchReadinessV1` (probe loop 50 ms doubling to 1 s), `readinessMarkerV1`.
- `🏗️build/🟢️readiness/🧪️tests/{🟦️.ts,🔣️.json,🥒️.feature}` (new tests, see Verification).

Bazel adapter `🔧️tools/🏗️bazel/📜️script.ts`:
- Continuous task readiness: explicit `readyWhen` markers keep their meaning; else a declared `metadata.semio.dashboard.ready` is awaited as address announced on the output plus an answering HTTP probe (prints `[bazel] task-ready <id> <address>`); else the old started marker. A continuous task that exits before readiness still fails the run.
- `interpolate`: an undeclared `{options.env.<NAME>}` now resolves to the environment the task process receives (`issueBuildTaskEnvironmentV1`, i.e. the invocation build mode for `SEMIO_BUILD_MODE`), lazily; declared `options.env` still wins.

Nx route (mirror, same contract):
- `⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs` (`executeLiteralBuildArgumentsV1`): a `continuous` target that declares `ready` and no `readyWhen` gets `readyWhen` = the shared marker, and the private command source carries the declaration.
- `…/📤️arguments/📜️script.ts` (command bridge): resolves the port from the task environment, watches the command output, and prints `<marker> <address>` once the address answers. Nx's own `readyWhen` handling then marks the task ready.
- `…/📤️arguments/🧬️schema/📥️source.json`: optional `ready` member.

## Verification (static phase)

- `bun test` in `🟢️readiness/🧪️tests`: 16 pass, 2 fail only because `strip-ansi` and `ajv` are not installed yet (the two third-party oracle tests; they run in the runtime phase). Passing: shared 22-case table of the dashboard decided identically in chunks and character by character, one-address-once, pending-until-next-character, schema vectors, resolution vectors, real local HTTP server statuses (200/204/302/399 ready; 400/404/503, closed port, foreign host, cancelled probe not ready), watch waits for a late server, watch cancellation, schema parity of graph/project/dashboard declarations, and the Nx bridge end to end (marker printed after the server line, `readyWhen` only for continuous + declared + no explicit `readyWhen`).
- `bun build --no-bundle` of the changed `.ts`/`.mjs`: OK; `node --check` of the `.mjs`: OK.
- Declaration scan of all 1,519 tracked `project.json`: 33 `ready` declarations, 0 invalid against the new schema.

## Open

- Runtime phase (needs node_modules): re-measure the closure, run the Bazel suite, `tools setup bazel`, dry-runs, real runs of `quiz:test` and `proctor:build`.
- Verify at runtime that Nx honours `readyWhen` of the bridge for a continuous root, and that all inferred `framework-os-dev:(prepare|activate)-<variant>-react-prototype` names are in the loaded graph.
- Extend the Bazel service integration phase (`SEMIO_BAZEL_INTEGRATION`) with a `ready`-declared service.
