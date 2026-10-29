# Supported Native Backend Alternative

The current backend registry explicitly registers semio as an in-house backend with native run, exec, parallelism, cancellation, progress and single-flight support. The neutral CLI selects ids through this registry, so the supported command is:

```text
SEMIO_LANGUAGE=en repo --tool=semio run @semio-tech/print:build-thesis --parallel=1
```

This is a legitimate backend selection, not an admission bypass. No run, dry run, reset or build was launched during this exploration.

## Source Evidence

- Build 🔌️backends/📋️registry/🔣️.json registers semio and its backend module; nx remains the default. The older build guide example lists only nx/bazel, but authoritative runtime selection derives accepted names from the live registry.
- Build 🔌️backends/🟦️.ts validates registry ids and exposes them to CLI help/selection.
- Build 🔧️tools/🐙️semio/🔌️backend/🟦️.ts plans the existing portable graph, issues original task authority, binds shared cache stores and calls the native scheduler. Task environments propagate SEMIO_BUILD_TOOL=semio to nested repo commands. It does not launch Nx.
- Build 🔧️tools/🐙️semio/🏁️scheduler/🟦️.ts bounds regular task slots with request.parallel, so parallel=1 serializes regular tasks. Continuous task handling remains governed by its normal scheduler semantics.
- Build 🔧️tools/🐙️semio/🚀️exec/🟦️.ts spawns declared programs or Bun portable-shell commands, retaining cancellation of their full process trees. It does not remove declared nested repo commands.

## Scheduling Implications

The semio selection removes the Nx backend process/plugin layer and propagates its choice into nested repo work. It cannot be claimed to eliminate the 3584 MB admission request: declarations may still invoke nested repo exec, whose resource admission remains active. It also uses the same portable graph, with no compact-graph declaration in its registry row; its backend retains the graph in context and offers no source evidence of lower graph-worker peak memory or fewer stale-source restarts. Parallel=1 caps task concurrency, not graph verification/inference memory.

Actionable conclusion: this is an authorized alternate scheduler worth trying once if the current run is deliberately replaced, but it is not a proven fix for graph-worker restarts or nested-exec resource contention. Do not launch it alongside the existing thesis producer, because that would add another graph/build request. No infrastructure edits or resource-limit changes are justified by this review.
