# Source Graph Admission and Retained Store Review

## Current Proven Selection Boundary

The active Source26 invocation uses run-many. The physical build/🕸️graph/🚪️admission/🟦️.ts buildInvocationGraphSelectionV1 returns kind all whenever args[0] is not literal run. A single repo run project:target produces one closed task selection, including exact configuration. This is an authored behavior difference, not an inference from queue PIDs.

loadBuildGraphV1 derives a selection scope only for kind tasks. withBuildGraphStateV1 uses withRepositoryBuildGraphSelectionV1 and a bounded selected shard for that scope; otherwise it calls withRepositoryBuildGraphFactStoreV1, which acquires repository-graph-facts:<workspace>. Therefore the current aggregate intentionally enters the global writer queue. A single-owner task can avoid this global writer at this boundary. This does not prove that its subsequent dependencies are small or that no other preliminary global writer is used.

## Admission Remains Exact

closeBuildOwnerAdmissionV1 admits selected tasks, actual dependsOn/generated prerequisites, imported Source owners and authored catalog providers. It repeats until the closed observations converge. loadBuildGraphV1 subsequently assembles selected files and retains complete Source-import checks. No generic declared-Source execution fast path was found. Existing early native recovery and current-domain maintenance routes are deliberately fixed semantic owner exceptions, not reusable bypasses.

A correct future run-many selection must resolve the actual project/target intersection: missing targets are skipped by actual run-many semantics. A Cartesian list with nonexistent targets would change behavior. This review does not implement a new selection or weaken generated-self-cycle guards.

## Retained Shared Store Lifetime

The ordinary build root opens diagnostics through withRepositoryCacheStoreV1 around runOwnedBuildInLeasesV1. The latter opens dependencies around graph preparation and actual Nx/Bazel execution. Graph baseline selection occurs before any selected task child executes. Thus the present global graph queue alone is not proof that a Source child is already participating in a parent/child reader cycle.

The store owner keeps originalCacheStores in AsyncLocalStorage. Same-process authenticated nested consumers reuse its exact retained frame through enrollConsumer. A separate Bun child does not inherit that JavaScript object or AsyncLocalStorage; forwarding the genuine Original envelope is not the same as borrowing an existing physical image lease. The physical image opener first requests shared custody even for an already mounted image. Fair queuing can place that request behind an exclusive waiter. A parent holding shared custody while awaiting a child that opens a second shared lease could therefore participate in a cycle, but no live PID ownership or actual descendant proof establishes that explanation for session90750. No foreign signals, store edits or new IPC capabilities were used.

The graph input owner similarly reuses a current same-workspace/same-Original writer when the namespace is absent or identical. Different selected namespaces acquire their own bounded shard while the cache-store AsyncLocalStorage remains authenticated. Its synchronous ledger transactions roll back on errors; this source trace does not establish a persistent SQL transaction deadlock.

## Independently Reported Execution Frontier

The parent retained a separate architecture test failure after653s: discoverBuildOwnerAdmissionFactsV1 rejected Generated task cannot consume its own output for @semio-tech/framework-os-shell-rs:typegen, importer framework/modules/🐚shell/🟦️.ts and generated shell/🤖️generated, phase compile. That is evidence that the peer invocation reached an unrelated producer, not proof of the edge that admitted it. The exact selected dependency/catalog/input closure must explain that edge before any fix. The self-cycle guard remains intact.

## Verification Status

This is a read-only source trace. No graph producer, Source test, native compile or dashboard was launched. The parent owns cancellation and replacement of its aggregate with sequential exact Source owner targets.

## Styling Generator Runtime-Entry Repair

Canonical attempt 20254 settled exit1 before native compilation with a generated self-cycle whose generic message omitted its identity. After the bounded identity diagnostic, attempt41072 settled exit1 identifying @semio-tech/ui-styling-tokens:generate. Its generator contract includes the public browser TypeScript facade as an exact freshness input. Graph assembly passed that data input into targetScriptSources, which unconditionally traversed literal module inputs; the facade imports the theme barrel and the generated tokens, creating a false own-output edge.

The correction gives actual executable inputs an explicit schema-authored task metadata.semio.runtimeSourceEntries contract. Entries are finite, unique, physical literal JS/TS modules under project/workspace placeholders, confined to the repository and the256-character path bound; declaration-only, missing, glob, traversal and control paths refuse. Ordinary matched literal inputs obey the existing compile/freshness role. Command imports and explicit runtime entries always retain their real transitive runtime imports, and genuine self-cycles still refuse. Freshness files remain ordinary task hash inputs.

Neutral cases and actual extracted targetScriptSources/relativeScriptInputs laws were authored before the production edit. They cover freshness-only generated facades, default and explicit compilation, explicit runtime despite freshness, real command imports, invalid and oversized entries, and strict generated self-cycle refusal. SQLite independently chooses demanded roots and esbuild independently resolves their actual import graph. The Source law has not run yet. Parent and Controls own the corresponding exact Source route declarations before the next canonical retry.
