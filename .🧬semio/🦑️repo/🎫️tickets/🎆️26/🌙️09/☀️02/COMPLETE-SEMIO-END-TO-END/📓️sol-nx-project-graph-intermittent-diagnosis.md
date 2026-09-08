# Nx Project Graph Intermittent Diagnosis

## Scope

This is a bounded read-only diagnosis of the intermittent root-runner failure rendered as `NX Failed to process project graph. undefined`. No Nx cache, daemon, process, or peer output was changed or removed.

## Observed state

- The isolated `🗑️generated/nx-root-opening` workspace-data directory has no open-file owner.
- Its `project-graph.json`, `file-map.json`, `source-maps.json`, and `nx_files.nxt` are readable. The JSON parses, the graph has 306 nodes and no retained errors, and its `computedAt` is `1788840749335`.
- Its Nx database is version `21.6.11`; an immutable read reports `integrity_check=ok`, 177 task-detail rows, 221 history rows, no running task, and no cache-output row.
- The empty, old `project-graph.lock` file is normal: Nx's native `FileLock` owns locking state, while the file itself persists.
- Commands routed through the repository `bun nx` wrapper already force `NX_ISOLATE_PLUGINS=false`; isolated plugin workers are therefore not the source of these specific routed failures.
- `NX_DAEMON=false` is present on the registered native runners, so their graph is computed in-process.

## Concrete rendering cause

Nx 21.6.11 writes a failed graph to `project-graph.json` with the raw JavaScript `Error` subclasses in its `errors` field (`nx/src/project-graph/nx-deps-cache.js:135-166`). `writeJsonFile` uses JSON serialization, but an `Error`'s `message` and `stack` are non-enumerable. For example, local read-only evaluation of the installed code gives:

```text
JSON.stringify(new AggregateCreateNodesError(...))
=> {"errors":[[null,{}]],"partialResults":[],"name":"AggregateCreateNodesError"}

JSON.stringify(new ProcessDependenciesError(...))
=> {"pluginName":"p","name":"ProcessDependenciesError"}
```

When a second Nx process finds `project-graph.lock` held, it waits, then reads the just-written graph using its own start time as `minimumComputedAt` (`nx/src/project-graph/project-graph.js:229-272`). If that graph contains the serialized failed `errors`, `readProjectGraphCache` reconstructs a `ProjectGraphError` from objects whose `message` was erased (`nx/src/project-graph/nx-deps-cache.js:63-83`). The CLI's non-verbose handler then renders `projectGraphError.getErrors().map(error => error.message)` (`nx/src/utils/handle-errors.js:18-34`), producing the literal body `undefined`.

Therefore `undefined` is a follower-process symptom of a prior graph producer's real error. It is not the originating cause and cannot identify the failing plugin. This also explains the observed intermittent sequence: one failing producer can poison waiting processes, while a later successful producer overwrites the graph with `errors=[]` and passes.

## Remaining unknown

The current isolated graph has already been overwritten by a successful build and contains no error. The retained sessions supplied only the non-verbose follower output, so the original graph producer and its real nested error are not recoverable from current cache state. The custom library/test plugins throw `Error` values at their explicit refusal sites, and the healthy graph does not prove which plugin or native workspace scan produced the earlier transient source error.

One later registered launch-generation attempt supplied a concrete producer-side failure before the task body ran. `repo:generator-inputs` terminated with `readCachedProjectGraph()`: `No cached ProjectGraph is available`, from `nx/src/project-graph/project-graph.js:44`, while Nx was dispatching `run-executor.js`. An immediate retry with `NX_DAEMON=false` completed generation. This proves at least one intermittent pre-task class is a graph cache availability race at nested Nx task dispatch; it does not prove that every prior `undefined` occurrence had that same originating error.

## Exact next capture

Keep `NX_DAEMON=false`, the isolated `NX_WORKSPACE_DATA_DIRECTORY`, and graph persistence required by the child dispatch. Add `--verbose`/`NX_VERBOSE_LOGGING=true` to every process sharing that isolated directory, not only the retry. The first process that owns graph construction will then retain the real stack before JSON erases it for waiters. Record the producer PID/start time and hash or copy the failed `project-graph.json` before any subsequent successful run overwrites it.

Do not use `nx reset` or delete the isolated directory: current files are structurally healthy, and deletion would erase the only chance to correlate a poisoned graph. `NX_CACHE_PROJECT_GRAPH=false` avoids persisting the malformed error but also defeats the existing forked-task graph handoff; it is not appropriate for this runner. Giving each independent root runner a different workspace-data directory prevents unrelated graph producers from poisoning one another, while source/native stages that intentionally share a produced graph must remain serialized within one directory.

One repository-specific caveat is independent of this failure: the top-level `bun nx` wrapper currently replaces an inherited `NX_WORKSPACE_DATA_DIRECTORY` with `.nx/workspace-data` in `📜️script.ts:657-661`. A caller that relies only on a launch environment variable does not actually get its requested ticket-local directory when routed through that wrapper. Exact isolated runners must either make the wrapper honor the supplied path or invoke the underlying Nx process through the existing controlled runner that installs the path after wrapper resolution. The environment of a live routed process confirmed `.nx/workspace-data` despite launch-level cache overrides.
