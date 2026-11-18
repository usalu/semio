# Direct target run

## Gap

`repo ./📜️script.ts <cmd>` refuses with `Original parent Script process invocation required` (`receiveScriptProcessInvocation`), because only the build driver issues the script envelope, caller capabilities (native owner), provider designation (`SEMIO_SCRIPT_PROVIDERS`), task identity and store receipt. `repo run <project:target> --exclude-dependencies` already issued all of these, but its graph selection still admitted the whole owner closure (every `dependsOn`, project dependency and imported source owner), so on a starved graph the single task could not be resolved.

## Design

The route is `repo run <project:target> --exclude-dependencies` (alias `--exclude-task-dependencies`), tool-neutral like every `run`. No new verb, no second launcher:

1. `buildInvocationGraphSelectionV1` turns the flag into the closed selection `{kind:"tasks",tasks:[...],excludeDependencies:true}` (a flag after `--` stays a task argument; `=false` keeps the closure).
2. `closeBuildOwnerAdmissionV1` admits exactly the owner of the named task. Its manifest and command source are read; `dependsOn`, project dependencies, dependency inputs, imported-source owners and generated producers admit nothing. Catalog providers keep their own rule.
3. `discoverBuildGraphV1` drops edges whose endpoint is not in the one-owner graph; the selected read never uses the sealed generation (default `SEMIO_BUILD_GRAPH_SLICE_READ`).
4. `planBuildTasksV1` (semio/Nx path) and `planBazelTasksV1` (Bazel adapter) skip a dependency on an absent project when `excludeDependencies` is set, instead of refusing it.
5. The semio backend then runs the unchanged path: `issueBuildTaskEnvironmentV1` mints the envelope, native-owner capabilities and providers from the caller envelope, the task keeps its id, and `withStoreReceiptsV1` settles the receipt (`excludeDependencies` already keeps prerequisites out of the receipt closure). Provenance is therefore identical to a graph-wide run; only the cache key differs because dependency inputs are not part of it.

The selection scope key differs from the full one, so the one-owner graph is stored apart.

## Invocation

```text
repo run @semio-tech/semio-tech-play:build --exclude-dependencies
repo capacity run --memory=6144 -- repo run <project:target> --exclude-dependencies
```

Help (`repo run --help --lang=en|de`) carries a new example and a note in English and German.

## Files

- `🕸️graph/🚪️admission/🟦️.ts`, `🧬️schema/🚪️selection.json`, `🧬️schema/🔣️.json` (selection schema, graph admission copy)
- `🕸️graph/🧭️discovery/🟦️.ts`, `🧮️task-model/🟦️.ts`, `🔧️tools/🏗️bazel/📜️script.ts`
- `⚙️operations/🔣️catalog.json`, help snapshots `⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json`, `🧭️scoping/🔣️.json` (note count 48)
- Tests: `🕸️graph/🚪️admission/🧪️tests/🟦️.ts` plus `🔣️.json` case `directTaskAdmission` (language-agnostic cases; Ajv schema oracle and an independent SQLite recursive closure); planner cases in `🔧️tools/🐙️semio/🧪️tests/🟦️.ts` (semio planner equals the Bazel adapter).

All paths are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`.

## Not covered

No live run: the end-to-end behaviour (graph read, envelope, receipt for a real project) was not executed here. Under bare `bun test`, tests that need an envelope still fail with the original refusal (pre-existing, same gap); the Git inventory tests of the semio suite also fail in this checkout independent of this change. The staged `SEMIO_BUILD_GRAPH_SLICE_READ=generation` path ignores the flag and keeps reading the closure.
