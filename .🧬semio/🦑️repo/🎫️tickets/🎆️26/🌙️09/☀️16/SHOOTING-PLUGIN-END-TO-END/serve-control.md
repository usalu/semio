# Shooting Serve Admission Investigation

Date: 2026-11-04. Build skill applied; source, cache state, envelopes and peer processes preserved.

## Confirmed Boundaries

The sole browser owner remains the React agents session76166 for `repo run @semio-tech/framework-os-dev:serve-shooting-react-prototype --lang=en --output=stream`. No duplicate server or cancellation was performed by this investigation.

The registry entry `repo tools exec` runs before `runAdmittedBuildCliV1`, which is the Script envelope issuer. The tools command passes its original environment unchanged to `runToolV1` with trusted tool selection and capacity admission. It does not create a Script process invocation. Consequently `repo tools exec repo|bun -- <owner script>` does not supply the missing Original parent envelope of a direct owner router. No artificial envelope was introduced and the known refusal was not repeated.

The existing `--graph-policy=current` switch is real, but README section Graph Generations explicitly confines it to graph/show queries and requires fresh for run/run-many/affected/exec/watch. An older build-guide example conflicts with that authority. No current-policy task was launched.

Changing backend cannot skip neutral graph admission: command/runtime loads the baseline before backend preparation. Excluding task dependencies likewise does not narrow the source owner closure.

## Fresh Scope Behavior

`buildInvocationGraphSelectionV1` converts this exact run into one selected task; `loadBuildGraphOnceV1` dispatches it to `loadBuildGraphSliceV1`. `sliceClosureV1` admits every transitive dependency edge of every selected owner, then follows active target inputs, dependencies, orchestration children and catalog providers. Project edge traversal is unconditional, so a general development owner can reach large portions of the repository even when only shooting is selected.

`refreshClosure` initially loads the selected closure; a missing owner or incomplete fragment can produce an unscoped first refresh. Selected verification additionally observes environment facts unless a valid coverage stamp excludes them. In the observed process, all2024 closure checks and1037 project refreshes repeated after unrelated changes. The available logs do not distinguish an incomplete initial slice from the transitive general-owner closure being intrinsically large. No speculative weakening of these checks was made.

## Reviewable Result

No production changes have been made by this investigation. No safe existing alternative owner entry was found. A proper repair must preserve fresh selected source facts and issued Script provenance; allowing stale task graphs or borrowing another invocation is excluded. Further diagnosis should capture exact selected owner names and the first selection fallback reason at the graph owner, then fix that proven boundary with schema-first neutral cases and an independent oracle.

## Read-Only Graph Export

Started the documented `repo graph --graph-policy=current --file=<ticket>/🗑️generated/serve-graph.json --lang=en` query (session18071) to inspect the exact sealed dependency closure without a diagnostic edit. Its own capacity admission requests5120MB and is queued behind current work; no graph has been read or report produced yet. The query is intentionally current only because it is read-only. The active serve remains fresh.

The read-only graph query was cancelled through its own session with SIGINT before admission after the React owner observed that query ahead of its inference child in the capacity queue. No export was produced and no graph evidence is claimed. The retained inspection script passes node syntax validation and is ready for an admitted read-only export.

Static package manifests do not directly declare the1037 owners: dev has12 workspace runtime dependencies and6 workspace development dependencies; repo-lib has6 workspace runtime dependencies and5 workspace development dependencies. These counts cannot prove the inferred transitive closure. The missing/incomplete selected-generation fallback and broad source-edge closure remain alternative diagnoses.
