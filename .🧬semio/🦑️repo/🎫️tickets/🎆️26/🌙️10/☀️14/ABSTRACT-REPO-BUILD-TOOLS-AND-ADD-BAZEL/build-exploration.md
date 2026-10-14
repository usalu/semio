# Build Infrastructure Audit

Read-only exploration on 2026-10-14. No runtime tests were run and no source was modified.

All relative paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` unless explicitly identified as workspace files.

## Existing Authorities

- `🟨️.mjs` is the primary Nx project inference owner. `emojiProjectJsonNodes` at line 1440 assembles authored `📋️project.json` projects and inferred Cargo projects. `projectWithDefaults` at line 935 adds policy, inputs, outputs and generated targets; `withNativePreparation`, `withReleasePipelinePreparation` and `targetBuildModes` finish the graph. `createDependenciesImplementation` at line 1615 supplies dependency edges. `dependencyResolutionAuthority` is separately public.
- `🔌️nx-plugin/🟨️.mjs` is a smaller policy script plugin, not the main project plugin. It discovers exported policy scripts and creates cacheable lint tasks.
- Workspace `nx.json` declares named inputs, environment hashing, plugins, catalog providers, and the Nx cache path. It directly loads `🟨️.mjs` and the separate test plugin.
- `🏗️build/📜️script.ts` owns `BuildScript`; its nested target route calls `runOwnedNxInvocationV1`. Its default build branch assumes the graph was already run and only reports completion.
- `⚡️caching/🚀️bootstrap/📜️script.ts` owns `NxScript`, `resolveNxInvocation`, `createNxBootstrapCapabilitiesV1`, and `runOwnedNxInvocationV1`. It provisions immutable tooling, publishes bootstrap sources and collection admission, preserves process capability/deadline envelopes, supports cancellation and watcher readiness, and activates optional dashboard routing.
- `🔌️nx-plugin/🏃️execution/🧬️schema/🔣️.json` already defines source/native/orchestration execution kinds with a native production/test scope. This semantic contract should remain common rather than becoming duplicated per backend.
- Workspace `📜️script.ts` has many direct nested `bun nx run` and `run-many` calls, including lines 7814, 8340 onward, 8507, and 14274 onward. Backend selection must propagate through these nested calls for end-to-end support.

## Recommended Seam

Extract a backend-neutral discovered project/task graph from the primary inference owner. Preserve existing project discovery, source input inference, generator ownership, native prerequisite graphs, dependency authority and build modes. Keep Nx executor names and Nx-shaped named inputs in its projection. Bazel must obtain this common graph without launching Nx or requiring its installed runtime.

Use a neutral selected-backend invocation owner for bootstrap, run, run-many, graph inspection and watching. Nx remains the default implementation. A Bazel implementation must translate every task from the common graph, preserving task dependencies, selected configuration, forwarded arguments, environment and working directory. Generated Bazel artifacts belong in backend cache, with a single current graph authority and bounded cancellation/progress. Backend selection must be inherited by nested orchestration scripts.

`rootCommandTargets` at primary inference line 1374 currently excludes only the `nx` route. Any new neutral orchestration route must also be excluded to prevent recursive task generation.

The extraction should include the policy-script and test-plugin project contributions, not only authored project files and Cargo inference. Catalog plugin contributions currently contain Nx-specific names and are read from `nx.json`; give their common task contribution an independent declaration before backend projection.

## Verification Targets

Create language-neutral graph/invocation fixtures, validate both backends against the same expected task/dependency/output surface, and compare Nx projection against the installed Nx library. Exercise real Nx and real Bazel leaf and dependency tasks, argument forwarding, configurations, failure status, cancellation, and nested backend propagation. Static generated-file checks alone cannot establish end-to-end Bazel execution.

No Bazel implementation was found in the library during this audit.
